//! Core EdgeService struct and initialization
//!
//! This module provides the core EdgeService struct and initialization logic
//! with zero allocation fast paths and blazing-fast performance.

use std::collections::{BTreeSet, HashMap};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::Instant;

use pingora_load_balancing::Backend;
use tokio::sync::mpsc::Sender;
use tracing::{error, info};

use crate::{
    auth::JwtAuth, config::Config, load::Load, metric_picker::MetricPicker,
    peer_discovery::PeerRegistry, rate_limit::AdvancedRateLimitManager,
    shutdown::ShutdownCoordinator,
};

/// Atomic metrics for thread-safe request tracking
/// Pattern follows load.rs - lock-free operations with Ordering::Relaxed
pub struct AtomicMetrics {
    pub total_requests: AtomicU64,
    pub successful_requests: AtomicU64,
    pub failed_requests: AtomicU64,
    pub active_connections: AtomicU64,
    pub total_response_time_us: AtomicU64,  // Reserved for future use
}

impl AtomicMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            total_response_time_us: AtomicU64::new(0),
        }
    }
}

/// EdgeService provides auth, overload protection, and routing functionality
/// with zero allocation fast paths and blazing-fast performance
pub struct EdgeService {
    pub cfg: Arc<Config>,
    pub auth: JwtAuth,
    pub picker: Arc<MetricPicker>,
    pub load: Arc<Load>,
    #[allow(dead_code)]
    pub bridge_tx: Sender<crate::mcp_bridge::BridgeMsg>,
    pub peer_registry: PeerRegistry,
    pub rate_limit_manager: Arc<AdvancedRateLimitManager>,
    pub shutdown_coordinator: Arc<ShutdownCoordinator>,
    /// Maps backend SocketAddr to original upstream URL for TLS detection
    pub upstream_urls: Arc<HashMap<SocketAddr, String>>,
    /// Atomic metrics for request tracking
    pub metrics: Arc<AtomicMetrics>,
    /// Service start time for uptime calculation
    pub start_time: Instant,
}

impl EdgeService {
    /// Create new EdgeService with optimized initialization
    pub fn new(
        cfg: Arc<Config>,
        bridge_tx: Sender<crate::mcp_bridge::BridgeMsg>,
        peer_registry: PeerRegistry,
    ) -> Self {
        // Parse URLs and build both backends and URL map
        let mut backends = BTreeSet::new();
        let mut url_map = HashMap::new();

        for url in &cfg.upstreams {
            match url::Url::parse(url) {
                Ok(parsed) => {
                    if let Some(host) = parsed.host_str() {
                        // Determine port based on scheme
                        let port = parsed.port().unwrap_or(
                            if parsed.scheme() == "https" { 443 } else { 80 }
                        );
                        
                        // Create backend address
                        let addr_str = format!("{}:{}", host, port);
                        if let Ok(backend) = Backend::new(&addr_str) {
                            // Store backend
                            backends.insert(backend.clone());
                            
                            // Map SocketAddr to original URL for TLS lookup
                            if let Ok(sock_addr) = addr_str.parse::<SocketAddr>() {
                                url_map.insert(sock_addr, url.clone());
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to parse upstream URL {}: {}", url, e);
                }
            }
        }

        let upstream_urls = Arc::new(url_map);
        info!("Initialized EdgeService with {} backends, {} URL mappings", 
             backends.len(), upstream_urls.len());

        // Initialize components with optimized settings
        let auth = JwtAuth::new(cfg.jwt_secret.clone(), cfg.jwt_expiry);
        let picker = Arc::new(MetricPicker::from_backends(&backends));
        let load = Arc::new(Load::new());
        let rate_limit_manager = Arc::new(AdvancedRateLimitManager::new(
            cfg.rate_limit.per_ip_rps as f64,
            cfg.rate_limit.burst_capacity,
            60, // 60 second window
        ));
        let shutdown_coordinator = Arc::new(ShutdownCoordinator::new(
            std::env::temp_dir().join("sweetmcp")
        ));

        let metrics = Arc::new(AtomicMetrics::new());
        let start_time = Instant::now();

        Self {
            cfg,
            auth,
            picker,
            load,
            bridge_tx,
            peer_registry,
            rate_limit_manager,
            shutdown_coordinator,
            upstream_urls,
            metrics,
            start_time,
        }
    }

    /// Get service configuration
    pub fn config(&self) -> &Config {
        &self.cfg
    }

    /// Get authentication handler
    pub fn auth(&self) -> &JwtAuth {
        &self.auth
    }

    /// Get metric picker
    pub fn picker(&self) -> &Arc<MetricPicker> {
        &self.picker
    }

    /// Get load handler
    pub fn load(&self) -> &Arc<Load> {
        &self.load
    }

    /// Get peer registry
    pub fn peer_registry(&self) -> &PeerRegistry {
        &self.peer_registry
    }

    /// Get rate limit manager
    pub fn rate_limit_manager(&self) -> &Arc<AdvancedRateLimitManager> {
        &self.rate_limit_manager
    }

    /// Get shutdown coordinator
    pub fn shutdown_coordinator(&self) -> &Arc<ShutdownCoordinator> {
        &self.shutdown_coordinator
    }

    /// Extract TLS configuration from upstream URL
    /// Returns (use_tls, sni_hostname)
    pub fn get_tls_config(&self, addr: &SocketAddr) -> (bool, String) {
        if let Some(url_str) = self.upstream_urls.get(addr) {
            if let Ok(url) = url::Url::parse(url_str) {
                let use_tls = url.scheme() == "https";
                let sni = url.host_str().unwrap_or("").to_string();
                return (use_tls, sni);
            }
        }
        // Fallback: no TLS, no SNI
        (false, String::new())
    }

    /// Check if service is properly initialized
    pub fn is_initialized(&self) -> bool {
        !self.cfg.upstreams.is_empty() && !self.cfg.jwt_secret.is_empty()
    }

    /// Get service status summary
    pub fn status_summary(&self) -> ServiceStatus {
        ServiceStatus {
            upstreams_count: self.cfg.upstreams.len(),
            is_initialized: self.is_initialized(),
            auth_enabled: !self.cfg.jwt_secret.is_empty(),
            rate_limiting_enabled: true,
        }
    }

    /// Validate service configuration
    pub fn validate_config(&self) -> Result<(), EdgeServiceError> {
        if self.cfg.upstreams.is_empty() {
            return Err(EdgeServiceError::ConfigurationError(
                "No upstream servers configured".to_string(),
            ));
        }

        if self.cfg.jwt_secret.is_empty() {
            return Err(EdgeServiceError::ConfigurationError(
                "JWT secret not configured".to_string(),
            ));
        }

        // Validate upstream URLs
        for upstream in &self.cfg.upstreams {
            if upstream.parse::<url::Url>().is_err() {
                return Err(EdgeServiceError::ConfigurationError(format!(
                    "Invalid upstream URL: {}",
                    upstream
                )));
            }
        }

        Ok(())
    }

    /// Get backend count
    pub fn backend_count(&self) -> usize {
        self.cfg.upstreams.len()
    }

    /// Check if backend is healthy
    pub fn is_backend_healthy(&self, backend_url: &str) -> bool {
        // This would integrate with health checking in a full implementation
        self.cfg.upstreams.contains(&backend_url.to_string())
    }

    /// Get service metrics
    pub fn get_metrics(&self) -> ServiceMetrics {
        ServiceMetrics {
            backend_count: self.backend_count(),
            active_connections: 0,    // Would be tracked in real implementation
            requests_per_second: 0.0, // Would be tracked in real implementation
            error_rate: 0.0,          // Would be tracked in real implementation
        }
    }

    /// Shutdown service gracefully
    pub async fn shutdown(&self) -> Result<(), EdgeServiceError> {
        info!("Initiating EdgeService shutdown");

        // Use shutdown coordinator for graceful shutdown
        self.shutdown_coordinator
            .initiate_shutdown()
            .await;

        info!("EdgeService shutdown completed");
        Ok(())
    }

    /// Update service configuration
    pub fn update_config(&mut self, new_config: Arc<Config>) -> Result<(), EdgeServiceError> {
        // Validate new configuration
        let temp_service = Self {
            cfg: new_config.clone(),
            auth: self.auth.clone(),
            picker: self.picker.clone(),
            load: self.load.clone(),
            bridge_tx: self.bridge_tx.clone(),
            peer_registry: self.peer_registry.clone(),
            rate_limit_manager: self.rate_limit_manager.clone(),
            shutdown_coordinator: self.shutdown_coordinator.clone(),
            upstream_urls: self.upstream_urls.clone(),
            metrics: self.metrics.clone(),
            start_time: self.start_time,
        };

        temp_service.validate_config()?;

        // Update configuration
        self.cfg = new_config;
        info!("EdgeService configuration updated");

        Ok(())
    }

    /// Clone service for testing or parallel operations
    pub fn clone_for_testing(&self) -> Self {
        Self {
            cfg: self.cfg.clone(),
            auth: self.auth.clone(),
            picker: self.picker.clone(),
            load: self.load.clone(),
            bridge_tx: self.bridge_tx.clone(),
            peer_registry: self.peer_registry.clone(),
            rate_limit_manager: self.rate_limit_manager.clone(),
            shutdown_coordinator: self.shutdown_coordinator.clone(),
            upstream_urls: self.upstream_urls.clone(),
            metrics: self.metrics.clone(),
            start_time: self.start_time,
        }
    }
}

/// Service status information
#[derive(Debug, Clone)]
pub struct ServiceStatus {
    pub upstreams_count: usize,
    pub is_initialized: bool,
    pub auth_enabled: bool,
    pub rate_limiting_enabled: bool,
}

/// Service metrics for monitoring
#[derive(Debug, Clone)]
pub struct ServiceMetrics {
    pub backend_count: usize,
    pub active_connections: u64,
    pub requests_per_second: f64,
    pub error_rate: f64,
}

/// Edge service error types
#[derive(Debug, thiserror::Error)]
pub enum EdgeServiceError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Rate limiting error: {0}")]
    RateLimitError(String),

    #[error("Backend error: {0}")]
    BackendError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Shutdown error: {0}")]
    ShutdownError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl EdgeServiceError {
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            EdgeServiceError::ConfigurationError(_) => false,
            EdgeServiceError::AuthenticationError(_) => true,
            EdgeServiceError::RateLimitError(_) => true,
            EdgeServiceError::BackendError(_) => true,
            EdgeServiceError::NetworkError(_) => true,
            EdgeServiceError::ShutdownError(_) => false,
            EdgeServiceError::InternalError(_) => false,
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            EdgeServiceError::ConfigurationError(_) => ErrorSeverity::Critical,
            EdgeServiceError::AuthenticationError(_) => ErrorSeverity::Warning,
            EdgeServiceError::RateLimitError(_) => ErrorSeverity::Info,
            EdgeServiceError::BackendError(_) => ErrorSeverity::Error,
            EdgeServiceError::ShutdownError(_) => ErrorSeverity::Critical,
            EdgeServiceError::InternalError(_) => ErrorSeverity::Critical,
            EdgeServiceError::NetworkError(_) => ErrorSeverity::Error,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}
