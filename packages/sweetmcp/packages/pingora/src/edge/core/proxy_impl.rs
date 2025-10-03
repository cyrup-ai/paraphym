//! ProxyHttp trait implementation for EdgeService
//!
//! This module implements the pingora ProxyHttp trait to enable EdgeService
//! to function as a proxy server with full request lifecycle management.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use pingora::http::ResponseHeader;
use pingora::prelude::*;
use pingora_proxy::{ProxyHttp, Session};
use serde_json;
use tracing::{warn, info};

use crate::edge::auth::AuthHandler;
use crate::api::peers::handle_peers_request;
use super::service::EdgeService;

/// Per-request context with protocol conversion support
pub struct EdgeContext {
    pub peer_id: Option<String>,
    
    // Protocol normalization fields
    pub protocol_context: Option<crate::normalize::ProtocolContext>,
    pub request_buffer: Vec<u8>,
    pub response_buffer: Vec<u8>,
    
    // HTTP metrics tracking
    pub request_start: std::time::Instant,
    pub method: String,
    pub endpoint: String,
    pub request_size: usize,
    pub response_size: usize,
    pub status_code: u16,
}

#[async_trait]
impl ProxyHttp for EdgeService {
    /// Per-request context type
    type CTX = EdgeContext;

    /// Create new request context
    fn new_ctx(&self) -> Self::CTX {
        EdgeContext { 
            peer_id: None,
            protocol_context: None,
            request_buffer: Vec::new(),
            response_buffer: Vec::new(),
            request_start: std::time::Instant::now(),
            method: String::new(),
            endpoint: String::new(),
            request_size: 0,
            response_size: 0,
            status_code: 200,
        }
    }

    /// Select upstream peer for the request
    ///
    /// This is the core routing logic that determines which backend
    /// should handle this request based on circuit breaker state and metrics.
    fn upstream_peer<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        _session: &'life1 mut Session,
        ctx: &'life2 mut Self::CTX,
    ) -> Pin<Box<dyn Future<Output = Result<Box<HttpPeer>>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            use pingora::protocols::l4::socket::SocketAddr as PingoraSocketAddr;

            let current_picker = self.picker.load();
            
            // Try each backend until we find one with closed/half-open circuit
            let mut candidate_backend = None;
            
            for backend in &current_picker.backends {
                // Get peer_id for circuit breaker lookup
                let peer_id = match &backend.addr {
                    PingoraSocketAddr::Inet(addr) => format!("{}:{}", addr.ip(), addr.port()),
                    PingoraSocketAddr::Unix(_) => continue, // Skip unix sockets
                };
                
                // Check circuit breaker state
                let breaker = self.circuit_breaker_manager.get_breaker(&peer_id).await;
                if breaker.should_allow_request().await {
                    candidate_backend = Some((backend, peer_id));
                    break;
                }
                tracing::debug!("Skipping backend {} - circuit open", peer_id);
            }
            
            // If all circuits open, fall back to round-robin
            let (backend, peer_id) = candidate_backend.or_else(|| {
                tracing::warn!("All circuits open - using fallback backend");
                current_picker.backends.first().map(|b| {
                    let id = match &b.addr {
                        PingoraSocketAddr::Inet(addr) => format!("{}:{}", addr.ip(), addr.port()),
                        PingoraSocketAddr::Unix(_) => "unix".to_string(),
                    };
                    (b, id)
                })
            }).ok_or_else(|| Error::new(ConnectNoRoute))?;
            
            // Store peer_id in context for later tracking
            ctx.peer_id = Some(peer_id);
            
            // Get TLS config and create peer
            let (use_tls, sni) = match &backend.addr {
                PingoraSocketAddr::Inet(addr) => self.get_tls_config(&addr),
                _ => (false, String::new()),
            };
            
            let peer = Box::new(HttpPeer::new(backend.clone(), use_tls, sni));
            Ok(peer)
        })
    }

    /// Filter incoming requests for authentication and rate limiting
    ///
    /// This callback is invoked for every request BEFORE upstream_peer() is called.
    /// It performs:
    /// 1. Local API endpoint handling (/api/peers with crypto token verification)
    /// 2. JWT authentication for proxied requests
    /// 3. Rate limiting checks
    ///
    /// Returns Ok(true) if response was sent (auth failed), Ok(false) to continue
    fn request_filter<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        session: &'life1 mut Session,
        _ctx: &'life2 mut Self::CTX,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
        Self::CTX: Send + Sync,
    {
        Box::pin(async move {
            // Track active connection (request started)
            self.metrics.active_connections.fetch_add(1, Ordering::Relaxed);

            let req_header = session.req_header();
            let path = req_header.uri.path().to_string(); // Clone path to avoid borrow conflict
            let method = req_header.method.clone();

            // PHASE 1: Handle local API endpoints (bypass normal auth/proxy flow)
            if path == "/api/peers" && method == pingora::http::Method::GET {
                // Peer discovery endpoint - verify encrypted discovery token
                let headers = &req_header.headers;
                
                match handle_peers_request(
                    Arc::clone(&self.token_manager),
                    Arc::new(self.peer_registry.clone()),
                    &self.cfg.auth.discovery_token,
                    headers,
                ).await {
                    Ok(peers_response) => {
                        // Serialize peer response to JSON
                        let json_body = match serde_json::to_string(&peers_response) {
                            Ok(json) => json,
                            Err(e) => {
                                warn!("Failed to serialize peers response: {}", e);
                                session.respond_error(500).await?;
                                return Ok(true);
                            }
                        };
                        
                        // Build HTTP 200 response with JSON body
                        let mut response_header = ResponseHeader::build(200, None)?;
                        response_header.insert_header("Content-Type", "application/json")?;
                        response_header.insert_header("Content-Length", json_body.len().to_string())?;
                        
                        // Write response header
                        session.as_mut()
                            .write_response_header(Box::new(response_header))
                            .await?;
                        
                        // Write response body
                        session.as_mut()
                            .write_response_body(bytes::Bytes::from(json_body), true)
                            .await?;
                        
                        info!("Served peer list to {}", session.client_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string()));
                        
                        return Ok(true); // Response sent, stop processing
                    }
                    Err(e) => {
                        // Token verification failed - log and return 401
                        warn!("Peer discovery token verification failed from {}: {}", 
                            session.client_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".to_string()),
                            e
                        );
                        session.respond_error(401).await?;
                        return Ok(true); // Response sent, stop processing
                    }
                }
            }

            // PHASE 2: JWT Authentication (for non-peer endpoints)
            match AuthHandler::authenticate_request(self, session).await {
                Ok(auth_context) if auth_context.is_authenticated => {
                    // Authentication successful - continue to rate limiting
                }
                Ok(_) | Err(_) => {
                    // Authentication failed - send 401 and stop processing
                    warn!("Authentication required - no valid credentials provided");
                    session.respond_error(401).await?;
                    return Ok(true); // Response sent, stop here
                }
            }

            // PHASE 3: Rate Limiting
            let client_addr = session.client_addr()
                .ok_or_else(|| Error::new(InternalError))?;
            let client_id = client_addr
                .as_inet()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // Check rate limit
            if !self.rate_limit_manager.check_request(&path, Some(&client_id), 1) {
                warn!("Rate limit exceeded for client: {} on endpoint: {}", client_id, path);
                session.respond_error(429).await?;
                return Ok(true); // Response sent, stop here
            }

            // All checks passed - continue to upstream_peer()
            Ok(false)
        })
    }

    /// Buffer and convert request body chunks
    ///
    /// Accumulates request body chunks and performs protocol conversion
    /// when the full body is received (end_of_stream = true).
    async fn request_body_filter(
        &self,
        session: &mut Session,
        body: &mut Option<bytes::Bytes>,
        end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        use crate::normalize::{detect_protocol, to_json_rpc_with_headers, Proto};
        
        // Buffer incoming chunks
        if let Some(b) = body {
            ctx.request_buffer.extend_from_slice(&b[..]);
            b.clear(); // Don't forward chunks until conversion
        }
        
        if end_of_stream && !ctx.request_buffer.is_empty() {
            // Get request headers for protocol detection
            let req_header = session.req_header();
            
            // Detect and convert protocol
            match detect_protocol(&ctx.request_buffer, Some(req_header)) {
                Ok(detection) => {
                    tracing::debug!(
                        "Detected protocol: {:?} with confidence {}",
                        detection.protocol,
                        detection.confidence
                    );
                    
                    // Convert non-JSON-RPC protocols
                    if detection.protocol != Proto::JsonRpc {
                        match to_json_rpc_with_headers("user", &ctx.request_buffer, Some(req_header)) {
                            Ok((proto_ctx, jsonrpc_value)) => {
                                // Store protocol context for response conversion
                                ctx.protocol_context = Some(proto_ctx);
                                
                                // Serialize to bytes
                                match serde_json::to_vec(&jsonrpc_value) {
                                    Ok(jsonrpc_bytes) => {
                                        *body = Some(bytes::Bytes::from(jsonrpc_bytes));
                                        tracing::info!(
                                            "Converted {:?} to JSON-RPC ({} bytes)",
                                            detection.protocol,
                                            body.as_ref().map(|b| b.len()).unwrap_or(0)
                                        );
                                    }
                                    Err(e) => {
                                        tracing::error!("JSON serialization failed: {}", e);
                                        return Err(Error::because(
                                            ErrorType::InternalError,
                                            "Protocol conversion serialization failed",
                                            e,
                                        ));
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Protocol conversion failed: {}", e);
                                // Forward original body on conversion failure
                                *body = Some(bytes::Bytes::from(ctx.request_buffer.clone()));
                            }
                        }
                    } else {
                        // Already JSON-RPC, forward as-is
                        *body = Some(bytes::Bytes::from(ctx.request_buffer.clone()));
                        tracing::debug!("Request already JSON-RPC, no conversion needed");
                    }
                }
                Err(e) => {
                    // Detection failed, assume JSON-RPC (backward compatible)
                    tracing::debug!("Protocol detection failed ({}), assuming JSON-RPC", e);
                    *body = Some(bytes::Bytes::from(ctx.request_buffer.clone()));
                }
            }
            
            // Clear buffer after processing
            ctx.request_buffer.clear();
        }
        
        Ok(())
    }

    /// Modify response headers for protocol conversion
    ///
    /// Removes Content-Length and adds Transfer-Encoding: chunked
    /// because converted response size differs from original.
    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        use crate::normalize::Proto;
        
        // Only modify headers if we converted the request
        if let Some(proto_ctx) = &ctx.protocol_context {
            if proto_ctx.protocol != Proto::JsonRpc {
                // Remove Content-Length (size will change after conversion)
                upstream_response.remove_header("Content-Length");
                
                // Add chunked encoding
                upstream_response
                    .insert_header("Transfer-Encoding", "chunked")
                    .map_err(|e| {
                        tracing::error!("Failed to set Transfer-Encoding: {}", e);
                        Error::because(
                            ErrorType::InternalError,
                            "Header modification failed",
                            e,
                        )
                    })?;
                
                tracing::debug!(
                    "Modified response headers for {:?} back-conversion",
                    proto_ctx.protocol
                );
            }
        }
        
        Ok(())
    }

    /// Buffer and convert response body chunks
    ///
    /// Accumulates response body chunks from upstream and converts
    /// back to original protocol when full response received.
    fn response_body_filter(
        &self,
        _session: &mut Session,
        body: &mut Option<bytes::Bytes>,
        end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> Result<Option<std::time::Duration>>
    where
        Self::CTX: Send + Sync,
    {
        use crate::normalize::{from_json_rpc, Proto};
        
        // Buffer incoming response chunks
        if let Some(b) = body {
            ctx.response_buffer.extend_from_slice(&b[..]);
            b.clear(); // Don't forward until conversion
        }
        
        if end_of_stream && !ctx.response_buffer.is_empty() {
            // Only convert if we converted the request
            if let Some(proto_ctx) = &ctx.protocol_context {
                if proto_ctx.protocol != Proto::JsonRpc {
                    // Parse JSON-RPC response
                    match serde_json::from_slice::<serde_json::Value>(&ctx.response_buffer) {
                        Ok(jsonrpc_response) => {
                            // Convert back to original protocol
                            match from_json_rpc(proto_ctx, &jsonrpc_response) {
                                Ok(converted_bytes) => {
                                    *body = Some(bytes::Bytes::from(converted_bytes));
                                    ctx.response_size = body.as_ref().map(|b| b.len()).unwrap_or(0);
                                    tracing::info!(
                                        "Converted JSON-RPC back to {:?} ({} bytes)",
                                        proto_ctx.protocol,
                                        ctx.response_size
                                    );
                                }
                                Err(e) => {
                                    tracing::warn!(
                                        "Response back-conversion failed ({}), sending JSON-RPC",
                                        e
                                    );
                                    // Send JSON-RPC on conversion failure
                                    *body = Some(bytes::Bytes::from(ctx.response_buffer.clone()));
                                    ctx.response_size = body.as_ref().map(|b| b.len()).unwrap_or(0);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Failed to parse JSON-RPC response ({}), forwarding as-is",
                                e
                            );
                            // Forward unparseable response as-is
                            *body = Some(bytes::Bytes::from(ctx.response_buffer.clone()));
                            ctx.response_size = body.as_ref().map(|b| b.len()).unwrap_or(0);
                        }
                    }
                } else {
                    // Was already JSON-RPC, forward as-is
                    *body = Some(bytes::Bytes::from(ctx.response_buffer.clone()));
                    ctx.response_size = body.as_ref().map(|b| b.len()).unwrap_or(0);
                }
            } else {
                // No protocol conversion, forward as-is
                *body = Some(bytes::Bytes::from(ctx.response_buffer.clone()));
                ctx.response_size = body.as_ref().map(|b| b.len()).unwrap_or(0);
            }
            
            // Clear buffer after processing
            ctx.response_buffer.clear();
        }
        
        // No delay needed
        Ok(None)
    }

    /// Collect metrics for completed requests
    /// Called at END of request lifecycle by pingora framework
    async fn logging(&self, _session: &mut Session, _e: Option<&Error>, _ctx: &mut Self::CTX)
    where
        Self::CTX: Send + Sync,
    {
        // Calculate final duration
        let duration_secs = _ctx.request_start.elapsed().as_secs_f64();
        
        // Record comprehensive HTTP metrics
        crate::metrics::record_http_request(
            &_ctx.method,
            &_ctx.endpoint,
            _ctx.status_code,
            duration_secs,
            _ctx.request_size,
            _ctx.response_size,
        );
        
        // Decrement active requests
        crate::metrics::decrement_active_requests(&_ctx.method, &_ctx.endpoint);
        
        // Track total requests
        self.metrics.total_requests.fetch_add(1, Ordering::Relaxed);

        // Track success vs failure based on error parameter
        // _e.is_some() = request failed, _e.is_none() = request succeeded
        if _e.is_some() {
            self.metrics.failed_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            self.metrics.successful_requests.fetch_add(1, Ordering::Relaxed);
        }

        // Request completed - decrement active connections
        self.metrics.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Record circuit breaker success/failure based on upstream response
    fn upstream_response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        // Store status code for metrics
        ctx.status_code = upstream_response.status.as_u16();
        
        // Get peer_id from context and spawn async task to record result
        if let Some(peer_id) = ctx.peer_id.clone() {
            let breaker_manager = self.circuit_breaker_manager.clone();
            let status = upstream_response.status;
            
            tokio::spawn(async move {
                let breaker = breaker_manager.get_breaker(&peer_id).await;
                
                // Record based on HTTP status
                if status.is_success() || status.is_redirection() {
                    breaker.record_success().await;
                    tracing::debug!("Circuit breaker recorded success for {}", peer_id);
                } else if status.is_client_error() || status.is_server_error() {
                    breaker.record_failure().await;
                    tracing::warn!("Circuit breaker recorded failure for {} (status: {})", peer_id, status);
                }
            });
        }
        
        Ok(())
    }

    /// Record circuit breaker failure on connection errors
    fn fail_to_connect(
        &self,
        _session: &mut Session,
        _peer: &HttpPeer,
        ctx: &mut Self::CTX,
        e: Box<Error>,
    ) -> Box<Error>
    where
        Self::CTX: Send + Sync,
    {
        // Record circuit breaker failure on connection errors
        if let Some(peer_id) = ctx.peer_id.clone() {
            let breaker_manager = self.circuit_breaker_manager.clone();
            
            tokio::spawn(async move {
                let breaker = breaker_manager.get_breaker(&peer_id).await;
                breaker.record_failure().await;
                tracing::error!("Circuit breaker recorded connection failure for {}", peer_id);
            });
        }
        
        e
    }
}
