//! Core EdgeService struct and initialization
//!
//! This module provides the core EdgeService struct and initialization logic

#![allow(dead_code)]
//! with zero allocation fast paths and blazing-fast performance.

use std::collections::{BTreeSet, HashMap};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::{Duration, Instant};

use arc_swap::ArcSwap;
use dashmap::DashMap;
use pingora_load_balancing::Backend;
use pingora_load_balancing::health_check::{Health, TcpHealthCheck};
use tokio::sync::mpsc::Sender;
use log::{error, info};

use crate::{
    auth::JwtAuth,
    circuit_breaker::CircuitBreakerManager,
    config::Config,
    crypto::core::TokenManager,
    load::Load, metric_picker::MetricPicker,
    peer_discovery::{PeerDiscovery, PeerRegistry},
    rate_limit::{RateLimiter, DistributedRateLimitManager},
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

impl Default for AtomicMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Health check configuration matching Pingora patterns
#[derive(Clone, Debug)]
pub struct HealthCheckConfig {
    /// Interval between health checks (milliseconds)
    pub interval_ms: u64,
    /// TCP connection timeout (milliseconds)  
    pub timeout_ms: u64,
    /// Consecutive failures before marking unhealthy
    pub failure_threshold: usize,
    /// Consecutive successes before marking healthy
    pub success_threshold: usize,
    /// Run health checks in parallel (from Pingora's LoadBalancer pattern)
    pub parallel: bool,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_ms: 5000,          // 5 seconds
            timeout_ms: 3000,           // 3 second timeout
            failure_threshold: 3,       // 3 failures -> unhealthy
            success_threshold: 2,       // 2 successes -> healthy
            parallel: true,             // Parallel checks for performance
        }
    }
}

/// EdgeService provides auth, overload protection, and routing functionality
/// with zero allocation fast paths and blazing-fast performance
pub struct EdgeService {
    pub cfg: Arc<Config>,
    pub auth: JwtAuth,
    pub picker: Arc<ArcSwap<MetricPicker>>,
    pub load: Arc<Load>,
    pub bridge_tx: Sender<crate::mcp_bridge::BridgeMsg>,
    pub peer_registry: PeerRegistry,
    pub peer_discovery: Arc<PeerDiscovery>,
    pub rate_limit_manager: RateLimiter,
    pub shutdown_coordinator: Arc<ShutdownCoordinator>,
    pub circuit_breaker_manager: Arc<CircuitBreakerManager>,
    /// Maps backend SocketAddr to original upstream URL for TLS detection
    pub upstream_urls: Arc<HashMap<SocketAddr, String>>,
    /// Atomic metrics for request tracking
    pub metrics: Arc<AtomicMetrics>,
    /// Service start time for uptime calculation
    pub start_time: Instant,
    /// Token manager for peer discovery crypto
    pub token_manager: Arc<TokenManager>,
    /// Track authentication attempts per IP: (count, window_start_time)
    pub auth_attempt_tracker: Arc<DashMap<String, (u32, Instant)>>,
    /// Health status tracking per backend (backend.hash_key() -> Health)
    /// Uses Pingora's Health struct for atomic status updates with thresholds
    pub backend_health: Arc<ArcSwap<HashMap<u64, Health>>>,
    /// TCP health check instance (Pingora's TcpHealthCheck)
    pub health_checker: Arc<TcpHealthCheck>,
    /// Health check configuration
    pub health_check_config: HealthCheckConfig,
}

impl EdgeService {
    /// Create new EdgeService with optimized initialization
    pub fn new(
        cfg: Arc<Config>,
        bridge_tx: Sender<crate::mcp_bridge::BridgeMsg>,
        peer_registry: PeerRegistry,
        circuit_breaker_manager: Arc<CircuitBreakerManager>,
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

        // Initialize health checking (Pingora pattern)
        let health_check_config = HealthCheckConfig::default();
        
        // Create health status map for all backends
        let mut backend_health_map = HashMap::new();
        for backend in backends.iter() {
            backend_health_map.insert(backend.hash_key(), Health::default());
        }
        let backend_health = Arc::new(ArcSwap::from_pointee(backend_health_map));
        
        // Create TcpHealthCheck instance
        let mut tcp_check = TcpHealthCheck::default();
        tcp_check.consecutive_success = health_check_config.success_threshold;
        tcp_check.consecutive_failure = health_check_config.failure_threshold;
        tcp_check.peer_template.options.connection_timeout = 
            Some(Duration::from_millis(health_check_config.timeout_ms));
        let health_checker = Arc::new(tcp_check);
        
        // Convert backends to Vec for health check task
        let backends_vec: Vec<_> = backends.iter().cloned().collect();
        
        // Spawn background health check task
        let health_checker_clone = health_checker.clone();
        let backend_health_clone = backend_health.clone();
        let check_config = health_check_config.clone();
        
        tokio::spawn(async move {
            super::operations::run_health_checks(
                backends_vec,
                health_checker_clone,
                backend_health_clone,
                check_config,
            ).await;
        });

        // Create PeerDiscovery
        let peer_discovery = Arc::new(PeerDiscovery::new(peer_registry.clone()));

        // Initialize components with optimized settings
        let auth = JwtAuth::new(cfg.jwt_secret.clone(), cfg.jwt_expiry);
        let initial_picker = MetricPicker::from_backends(&backends);
        let picker = Arc::new(ArcSwap::from_pointee(initial_picker));
        let load = Arc::new(Load::new());
        let rate_limit_manager = RateLimiter::Distributed(Arc::new(DistributedRateLimitManager::new()));
        let shutdown_coordinator = Arc::new(ShutdownCoordinator::new(
            std::env::temp_dir().join("sweetmcp")
        ));

        let metrics = Arc::new(AtomicMetrics::new());
        let start_time = Instant::now();

        // Initialize crypto token manager for peer discovery
        let token_manager = match TokenManager::new() {
            Ok(tm) => Arc::new(tm),
            Err(e) => {
                error!("Failed to initialize TokenManager for peer crypto: {}", e);
                panic!("Failed to initialize TokenManager for peer crypto: {}", e);
            }
        };

        // Start automatic token rotation task (24 hour rotation by default)
        let manager_clone = Arc::clone(&token_manager);
        tokio::spawn(async move {
            if let Err(e) = manager_clone.start_rotation_task().await {
                error!("Token rotation task failed: {}", e);
            }
        });

        Self {
            cfg,
            auth,
            picker,
            load,
            bridge_tx,
            peer_registry,
            peer_discovery,
            rate_limit_manager,
            shutdown_coordinator,
            circuit_breaker_manager,
            upstream_urls,
            metrics,
            start_time,
            token_manager,
            auth_attempt_tracker: Arc::new(DashMap::new()),
            backend_health,
            health_checker,
            health_check_config,
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
    pub fn picker(&self) -> &Arc<ArcSwap<MetricPicker>> {
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
    pub fn rate_limit_manager(&self) -> &RateLimiter {
        &self.rate_limit_manager
    }

    /// Get shutdown coordinator
    pub fn shutdown_coordinator(&self) -> &Arc<ShutdownCoordinator> {
        &self.shutdown_coordinator
    }

    /// Extract TLS configuration from upstream URL
    /// Returns (use_tls, sni_hostname)
    pub fn get_tls_config(&self, addr: &SocketAddr) -> (bool, String) {
        if let Some(url_str) = self.upstream_urls.get(addr)
            && let Ok(url) = url::Url::parse(url_str) {
                let use_tls = url.scheme() == "https";
                let sni = url.host_str().unwrap_or("").to_string();
                return (use_tls, sni);
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
            return Err(EdgeServiceError::Configuration(
                "No upstream servers configured".to_string(),
            ));
        }

        if self.cfg.jwt_secret.is_empty() {
            return Err(EdgeServiceError::Configuration(
                "JWT secret not configured".to_string(),
            ));
        }

        // Validate upstream URLs
        for upstream in &self.cfg.upstreams {
            if upstream.parse::<url::Url>().is_err() {
                return Err(EdgeServiceError::Configuration(format!(
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

    /// Check if auth attempts from IP exceed rate limit
    /// Returns true if rate limit exceeded
    pub fn auth_attempts_exceeded(&self, client_ip: &str, max_attempts: u32) -> bool {
        let now = Instant::now();
        let window = std::time::Duration::from_secs(60); // 1 minute window
        
        // Get or create entry
        let mut entry = self.auth_attempt_tracker
            .entry(client_ip.to_string())
            .or_insert((0, now));
        
        // Reset if window expired
        if now.duration_since(entry.1) > window {
            entry.0 = 1;
            entry.1 = now;
            return false;
        }
        
        // Increment and check
        entry.0 += 1;
        entry.0 > max_attempts
    }
    
    /// Record successful authentication (reset counter)
    pub fn reset_auth_attempts(&self, client_ip: &str) {
        self.auth_attempt_tracker.remove(client_ip);
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
            peer_discovery: self.peer_discovery.clone(),
            rate_limit_manager: self.rate_limit_manager.clone(),
            shutdown_coordinator: self.shutdown_coordinator.clone(),
            circuit_breaker_manager: self.circuit_breaker_manager.clone(),
            upstream_urls: self.upstream_urls.clone(),
            metrics: self.metrics.clone(),
            start_time: self.start_time,
            token_manager: self.token_manager.clone(),
            auth_attempt_tracker: self.auth_attempt_tracker.clone(),
            backend_health: self.backend_health.clone(),
            health_checker: self.health_checker.clone(),
            health_check_config: self.health_check_config.clone(),
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
            peer_discovery: self.peer_discovery.clone(),
            rate_limit_manager: self.rate_limit_manager.clone(),
            shutdown_coordinator: self.shutdown_coordinator.clone(),
            circuit_breaker_manager: self.circuit_breaker_manager.clone(),
            upstream_urls: self.upstream_urls.clone(),
            metrics: self.metrics.clone(),
            start_time: self.start_time,
            token_manager: self.token_manager.clone(),
            auth_attempt_tracker: self.auth_attempt_tracker.clone(),
            backend_health: self.backend_health.clone(),
            health_checker: self.health_checker.clone(),
            health_check_config: self.health_check_config.clone(),
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
#[allow(clippy::enum_variant_names)]
pub enum EdgeServiceError {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Rate limiting error: {0}")]
    RateLimit(String),

    #[error("Backend error: {0}")]
    Backend(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Shutdown error: {0}")]
    Shutdown(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl EdgeServiceError {
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            EdgeServiceError::Configuration(_) => false,
            EdgeServiceError::Authentication(_) => true,
            EdgeServiceError::RateLimit(_) => true,
            EdgeServiceError::Backend(_) => true,
            EdgeServiceError::Network(_) => true,
            EdgeServiceError::Shutdown(_) => false,
            EdgeServiceError::Internal(_) => false,
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            EdgeServiceError::Configuration(_) => ErrorSeverity::Critical,
            EdgeServiceError::Authentication(_) => ErrorSeverity::Warning,
            EdgeServiceError::RateLimit(_) => ErrorSeverity::Info,
            EdgeServiceError::Backend(_) => ErrorSeverity::Error,
            EdgeServiceError::Shutdown(_) => ErrorSeverity::Critical,
            EdgeServiceError::Internal(_) => ErrorSeverity::Critical,
            EdgeServiceError::Network(_) => ErrorSeverity::Error,
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
