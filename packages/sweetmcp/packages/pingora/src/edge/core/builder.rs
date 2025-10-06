//! EdgeService builder pattern implementation
//!
//! This module provides the EdgeServiceBuilder for flexible construction

#![allow(dead_code)]
//! of EdgeService instances with zero allocation patterns and blazing-fast
//! performance.

use std::collections::{BTreeSet, HashMap};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use arc_swap::ArcSwap;
use pingora_load_balancing::Backend;
use tokio::sync::mpsc::Sender;
use log::{debug, error, info};

use super::service::{EdgeService, EdgeServiceError};
use crate::{
    auth::JwtAuth,
    config::Config,
    crypto::core::TokenManager,
    load::Load,
    metric_picker::MetricPicker,
    peer_discovery::{PeerDiscovery, PeerRegistry},
    rate_limit::{distributed::DistributedRateLimitManager, limiter::AdvancedRateLimitManager, RateLimiter},
    shutdown::ShutdownCoordinator,
};

/// Builder for EdgeService with flexible configuration
pub struct EdgeServiceBuilder {
    cfg: Option<Arc<Config>>,
    bridge_tx: Option<Sender<crate::mcp_bridge::BridgeMsg>>,
    peer_registry: Option<PeerRegistry>,
    custom_rate_limiter: Option<RateLimiter>,
    custom_shutdown_coordinator: Option<Arc<ShutdownCoordinator>>,
}

impl EdgeServiceBuilder {
    /// Create new EdgeServiceBuilder
    pub fn new() -> Self {
        debug!("Creating new EdgeServiceBuilder");
        Self {
            cfg: None,
            bridge_tx: None,
            peer_registry: None,
            custom_rate_limiter: None,
            custom_shutdown_coordinator: None,
        }
    }

    /// Set configuration with validation
    pub fn with_config(mut self, cfg: Arc<Config>) -> Self {
        debug!(
            "Setting configuration with {} upstreams",
            cfg.upstreams.len()
        );
        self.cfg = Some(cfg);
        self
    }

    /// Set bridge channel with optimized channel handling
    pub fn with_bridge_channel(mut self, bridge_tx: Sender<crate::mcp_bridge::BridgeMsg>) -> Self {
        debug!("Setting bridge channel");
        self.bridge_tx = Some(bridge_tx);
        self
    }

    /// Set peer registry with fast registry setup
    pub fn with_peer_registry(mut self, peer_registry: PeerRegistry) -> Self {
        debug!("Setting peer registry");
        self.peer_registry = Some(peer_registry);
        self
    }

    /// Set custom rate limiter with advanced configuration
    pub fn with_custom_rate_limiter(mut self, rate_limiter: RateLimiter) -> Self {
        debug!("Setting custom rate limiter");
        self.custom_rate_limiter = Some(rate_limiter);
        self
    }

    /// Set custom shutdown coordinator with optimized shutdown handling
    pub fn with_custom_shutdown_coordinator(
        mut self,
        coordinator: Arc<ShutdownCoordinator>,
    ) -> Self {
        debug!("Setting custom shutdown coordinator");
        self.custom_shutdown_coordinator = Some(coordinator);
        self
    }

    /// Build EdgeService with validation and optimization
    pub fn build(self) -> Result<EdgeService, EdgeServiceError> {
        info!("Building EdgeService");

        let cfg = self.cfg.ok_or_else(|| {
            EdgeServiceError::Configuration("Configuration is required".to_string())
        })?;

        let bridge_tx = self.bridge_tx.ok_or_else(|| {
            EdgeServiceError::Configuration("Bridge channel is required".to_string())
        })?;

        let peer_registry = self.peer_registry.ok_or_else(|| {
            EdgeServiceError::Configuration("Peer registry is required".to_string())
        })?;

        // Parse URLs and build backends (from EdgeService::new logic)
        let mut backends = BTreeSet::new();
        let mut url_map = HashMap::new();
        for url in &cfg.upstreams {
            match url::Url::parse(url) {
                Ok(parsed) => {
                    if let Some(host) = parsed.host_str() {
                        let port = parsed.port().unwrap_or(
                            if parsed.scheme() == "https" { 443 } else { 80 }
                        );
                        let addr_str = format!("{}:{}", host, port);
                        if let Ok(backend) = Backend::new(&addr_str) {
                            backends.insert(backend.clone());
                            if let Ok(sock_addr) = addr_str.parse::<SocketAddr>() {
                                url_map.insert(sock_addr, url.clone());
                            }
                        }
                    }
                }
                Err(e) => error!("Failed to parse upstream URL {}: {}", url, e),
            }
        }

        // Create circuit breaker manager
        let circuit_config = crate::circuit_breaker::CircuitBreakerConfig {
            error_threshold_percentage: cfg.circuit_breaker_threshold,
            request_volume_threshold: 20,
            sleep_window: std::time::Duration::from_secs(5),
            half_open_requests: 3,
            metrics_window: std::time::Duration::from_secs(10),
        };
        let circuit_breaker_manager = Arc::new(crate::circuit_breaker::CircuitBreakerManager::new(circuit_config));

        // Use custom or default rate limiter
        let rate_limit_manager = self.custom_rate_limiter
            .unwrap_or(RateLimiter::Distributed(Arc::new(DistributedRateLimitManager::new())));

        // Use custom or default shutdown coordinator
        let shutdown_coordinator = self.custom_shutdown_coordinator
            .unwrap_or_else(|| Arc::new(ShutdownCoordinator::new(std::env::temp_dir().join("sweetmcp"))));

        // Initialize crypto token manager
        let token_manager = Arc::new(TokenManager::new()
            .map_err(|e| EdgeServiceError::Internal(format!("TokenManager init failed: {}", e)))?);

        // Start token rotation
        let manager_clone = Arc::clone(&token_manager);
        tokio::spawn(async move {
            if let Err(e) = manager_clone.start_rotation_task().await {
                error!("Token rotation task failed: {}", e);
            }
        });

        // Initialize health checking (following EdgeService::new pattern)
        let health_check_config = super::service::HealthCheckConfig::default();
        
        // Create health status map for all backends
        let mut backend_health_map = std::collections::HashMap::new();
        for backend in backends.iter() {
            backend_health_map.insert(backend.hash_key(), pingora_load_balancing::health_check::Health::default());
        }
        let backend_health = Arc::new(ArcSwap::from_pointee(backend_health_map));
        
        // Create TcpHealthCheck instance
        let mut tcp_check = pingora_load_balancing::health_check::TcpHealthCheck::default();
        tcp_check.consecutive_success = health_check_config.success_threshold;
        tcp_check.consecutive_failure = health_check_config.failure_threshold;
        tcp_check.peer_template.options.connection_timeout = 
            Some(std::time::Duration::from_millis(health_check_config.timeout_ms));
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

        // Construct EdgeService DIRECTLY
        let service = EdgeService {
            cfg: cfg.clone(),
            auth: JwtAuth::new(cfg.jwt_secret.clone(), cfg.jwt_expiry),
            picker: Arc::new(ArcSwap::from_pointee(MetricPicker::from_backends(&backends))),
            load: Arc::new(Load::new()),
            bridge_tx,
            peer_registry: peer_registry.clone(),
            peer_discovery: Arc::new(PeerDiscovery::new(peer_registry)),
            rate_limit_manager,
            shutdown_coordinator,
            circuit_breaker_manager,
            upstream_urls: Arc::new(url_map),
            metrics: Arc::new(super::service::AtomicMetrics::new()),
            start_time: Instant::now(),
            token_manager,
            auth_attempt_tracker: Arc::new(dashmap::DashMap::new()),
            backend_health,
            health_checker,
            health_check_config,
        };

        // Validate the built service
        service.validate_config()?;

        info!("EdgeService built successfully");
        Ok(service)
    }

    /// Build with default components for testing
    pub fn build_for_testing(self) -> Result<EdgeService, EdgeServiceError> {
        info!("Building EdgeService for testing");

        // Create minimal configuration if not provided
        let cfg = self.cfg.unwrap_or_else(|| {
            // Create a proper 32-byte secret for testing
            let test_secret: [u8; 32] = *b"test_secret_exactly_32_bytes!!!!";
            Arc::new(Config {
                upstreams: vec!["http://localhost:8080".to_string()],
                jwt_secret: Arc::new(test_secret),
                ..Default::default()
            })
        });

        // Create test channel if not provided
        let bridge_tx = self.bridge_tx.unwrap_or_else(|| {
            let (tx, _rx) = tokio::sync::mpsc::channel(100);
            tx
        });

        // Create test peer registry if not provided
        let peer_registry = self.peer_registry.unwrap_or_else(|| {
            let circuit_config = crate::circuit_breaker::CircuitBreakerConfig {
                error_threshold_percentage: 50,
                request_volume_threshold: 20,
                sleep_window: std::time::Duration::from_secs(5),
                half_open_requests: 3,
                metrics_window: std::time::Duration::from_secs(10),
            };
            let circuit_breaker_manager = Arc::new(crate::circuit_breaker::CircuitBreakerManager::new(circuit_config));
            crate::peer_discovery::PeerRegistry::new(circuit_breaker_manager)
        });

        // Build with test configuration
        Self {
            cfg: Some(cfg),
            bridge_tx: Some(bridge_tx),
            peer_registry: Some(peer_registry),
            custom_rate_limiter: self.custom_rate_limiter,
            custom_shutdown_coordinator: self.custom_shutdown_coordinator,
        }
        .build()
    }

    /// Validate builder state before building
    pub fn validate(&self) -> Result<(), EdgeServiceError> {
        if self.cfg.is_none() {
            return Err(EdgeServiceError::Configuration(
                "Configuration must be set before building".to_string(),
            ));
        }

        if self.bridge_tx.is_none() {
            return Err(EdgeServiceError::Configuration(
                "Bridge channel must be set before building".to_string(),
            ));
        }

        if self.peer_registry.is_none() {
            return Err(EdgeServiceError::Configuration(
                "Peer registry must be set before building".to_string(),
            ));
        }

        // Validate configuration if present
        if let Some(ref cfg) = self.cfg {
            if cfg.upstreams.is_empty() {
                return Err(EdgeServiceError::Configuration(
                    "At least one upstream must be configured".to_string(),
                ));
            }

            if cfg.jwt_secret.is_empty() {
                return Err(EdgeServiceError::Configuration(
                    "JWT secret must be configured".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Get builder status for debugging
    pub fn status(&self) -> BuilderStatus {
        BuilderStatus {
            has_config: self.cfg.is_some(),
            has_bridge_channel: self.bridge_tx.is_some(),
            has_peer_registry: self.peer_registry.is_some(),
            has_custom_rate_limiter: self.custom_rate_limiter.is_some(),
            has_custom_shutdown_coordinator: self.custom_shutdown_coordinator.is_some(),
            is_ready: self.validate().is_ok(),
        }
    }

    /// Reset builder to initial state
    pub fn reset(mut self) -> Self {
        debug!("Resetting EdgeServiceBuilder");
        self.cfg = None;
        self.bridge_tx = None;
        self.peer_registry = None;
        self.custom_rate_limiter = None;
        self.custom_shutdown_coordinator = None;
        self
    }

    /// Clone builder for parallel construction
    pub fn clone_builder(&self) -> Self {
        Self {
            cfg: self.cfg.clone(),
            bridge_tx: self.bridge_tx.clone(),
            peer_registry: self.peer_registry.clone(),
            custom_rate_limiter: self.custom_rate_limiter.clone(),
            custom_shutdown_coordinator: self.custom_shutdown_coordinator.clone(),
        }
    }

    /// Build multiple services with different configurations
    pub fn build_multiple(
        base_builder: Self,
        configs: Vec<Arc<Config>>,
    ) -> Result<Vec<EdgeService>, EdgeServiceError> {
        let mut services = Vec::with_capacity(configs.len());

        for config in configs {
            let builder = base_builder.clone_builder().with_config(config);
            let service = builder.build()?;
            services.push(service);
        }

        info!("Built {} EdgeService instances", services.len());
        Ok(services)
    }

    /// Create builder from existing service configuration
    pub fn from_service(service: &EdgeService) -> Self {
        Self {
            cfg: Some(service.cfg.clone()),
            bridge_tx: Some(service.bridge_tx.clone()),
            peer_registry: Some(service.peer_registry.clone()),
            custom_rate_limiter: Some(service.rate_limit_manager.clone()),
            custom_shutdown_coordinator: Some(service.shutdown_coordinator.clone()),
        }
    }

    /// Apply configuration preset
    pub fn with_preset(self, preset: BuilderPreset) -> Self {
        match preset {
            BuilderPreset::Development => {
                debug!("Applying development preset");
                // Low rate limits for local development
                let rate_limiter = RateLimiter::Advanced(Arc::new(
                    AdvancedRateLimitManager::new(10.0, 100, 60)
                ));
                self.with_custom_rate_limiter(rate_limiter)
            }
            BuilderPreset::Production => {
                debug!("Applying production preset");
                // Use default DistributedRateLimitManager (high performance)
                let rate_limiter = RateLimiter::Distributed(Arc::new(
                    DistributedRateLimitManager::new()
                ));
                self.with_custom_rate_limiter(rate_limiter)
            }
            BuilderPreset::Testing => {
                debug!("Applying testing preset");
                // Permissive limits for test suites
                let rate_limiter = RateLimiter::Advanced(Arc::new(
                    AdvancedRateLimitManager::new(1000.0, 10000, 3600)
                ));
                self.with_custom_rate_limiter(rate_limiter)
            }
        }
    }
}

impl Default for EdgeServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder status information
#[derive(Debug, Clone)]
pub struct BuilderStatus {
    pub has_config: bool,
    pub has_bridge_channel: bool,
    pub has_peer_registry: bool,
    pub has_custom_rate_limiter: bool,
    pub has_custom_shutdown_coordinator: bool,
    pub is_ready: bool,
}

impl BuilderStatus {
    /// Check if builder is complete
    pub fn is_complete(&self) -> bool {
        self.has_config && self.has_bridge_channel && self.has_peer_registry
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        let required_components = 3.0; // config, bridge_channel, peer_registry
        let mut completed = 0.0;

        if self.has_config {
            completed += 1.0;
        }
        if self.has_bridge_channel {
            completed += 1.0;
        }
        if self.has_peer_registry {
            completed += 1.0;
        }

        (completed / required_components) * 100.0
    }

    /// Get missing components
    pub fn missing_components(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();

        if !self.has_config {
            missing.push("configuration");
        }
        if !self.has_bridge_channel {
            missing.push("bridge_channel");
        }
        if !self.has_peer_registry {
            missing.push("peer_registry");
        }

        missing
    }
}

/// Configuration presets for common use cases
#[derive(Debug, Clone)]
pub enum BuilderPreset {
    Development,
    Production,
    Testing,
}

/// Convenience functions for common builder patterns
impl EdgeServiceBuilder {
    /// Quick builder for development
    pub fn development() -> Self {
        Self::new().with_preset(BuilderPreset::Development)
    }

    /// Quick builder for production
    pub fn production() -> Self {
        Self::new().with_preset(BuilderPreset::Production)
    }

    /// Quick builder for testing
    pub fn testing() -> Self {
        Self::new().with_preset(BuilderPreset::Testing)
    }
}
