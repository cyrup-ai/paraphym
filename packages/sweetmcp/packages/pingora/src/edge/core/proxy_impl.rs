//! ProxyHttp trait implementation for EdgeService
//!
//! This module implements the pingora ProxyHttp trait to enable EdgeService
//! to function as a proxy server with full request lifecycle management.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use async_trait::async_trait;
use pingora::prelude::*;
use pingora_proxy::{ProxyHttp, Session};
use tracing::warn;

use crate::edge::auth::AuthHandler;
use super::service::EdgeService;

/// Per-request context (empty for now, can be extended)
pub struct EdgeContext;

#[async_trait]
impl ProxyHttp for EdgeService {
    /// Per-request context type
    type CTX = EdgeContext;

    /// Create new request context
    fn new_ctx(&self) -> Self::CTX {
        EdgeContext
    }

    /// Select upstream peer for the request
    ///
    /// This is the core routing logic that determines which backend
    /// should handle this request based on the MetricPicker's selection.
    fn upstream_peer<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        _session: &'life1 mut Session,
        _ctx: &'life2 mut Self::CTX,
    ) -> Pin<Box<dyn Future<Output = Result<Box<HttpPeer>>> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            use pingora::protocols::l4::socket::SocketAddr as PingoraSocketAddr;

            // Use the metric picker to select a backend
            let backend = self
                .picker
                .pick_backend()
                .ok_or_else(|| {
                    Error::new(ConnectNoRoute)
                })?;

            // Extract std::net::SocketAddr from pingora SocketAddr and get TLS config
            let (use_tls, sni) = match &backend.addr {
                PingoraSocketAddr::Inet(addr) => self.get_tls_config(addr),
                _ => (false, String::new()), // Unix sockets don't use TLS
            };

            // Create HttpPeer with proper TLS configuration
            let peer = Box::new(HttpPeer::new(
                backend.clone(),
                use_tls,  // ✅ Determined from URL scheme
                sni,      // ✅ Hostname for SNI
            ));

            Ok(peer)
        })
    }

    /// Filter incoming requests for authentication and rate limiting
    ///
    /// This callback is invoked for every request BEFORE upstream_peer() is called.
    /// It performs:
    /// 1. Authentication validation (JWT/API key/Discovery token)
    /// 2. Rate limiting checks
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

            // PHASE 1: Authentication
            // Call existing authentication logic from validation.rs
            match AuthHandler::authenticate_request(self, session).await {
                Ok(auth_context) if auth_context.is_authenticated => {
                    // Authentication successful - user is authenticated
                    // auth_context contains user claims, could be stored in CTX if needed
                    // Continue to rate limiting
                }
                Ok(_) | Err(_) => {
                    // Authentication failed or unauthenticated - send 401 and stop processing
                    warn!("Authentication required - no valid credentials provided");
                    session.respond_error(401).await?;
                    return Ok(true); // true = response already sent, stop here
                }
            }

            // PHASE 2: Rate Limiting
            // Extract client IP for rate limiting
            let client_addr = session.client_addr()
                .ok_or_else(|| Error::new(InternalError))?;
            let client_id = client_addr
                .as_inet()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // Check rate limit (synchronous call)
            if !self.rate_limit_manager.check_rate_limit(&client_id, 1) {
                // Rate limit exceeded - send 429 and stop processing
                warn!("Rate limit exceeded for client: {}", client_id);
                session.respond_error(429).await?;
                return Ok(true); // true = response already sent, stop here
            }

            // Both checks passed - continue to upstream_peer()
            Ok(false) // false = continue processing
        })
    }

    /// Collect metrics for completed requests
    /// Called at END of request lifecycle by pingora framework
    async fn logging(&self, _session: &mut Session, _e: Option<&Error>, _ctx: &mut Self::CTX)
    where
        Self::CTX: Send + Sync,
    {
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
}
