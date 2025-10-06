//! Edge service operations and utility methods
//!
//! This module provides utility operations for EdgeService such as health checks
//! and statistics. HTTP request handling is done via ProxyHttp trait implementation
//! in proxy_impl.rs and authentication functions in auth/validation.rs.



#![allow(dead_code)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::time::{Duration, interval};

use arc_swap::ArcSwap;
use pingora_load_balancing::Backend;
use pingora_load_balancing::health_check::{Health, TcpHealthCheck, HealthCheck};
use log::{info, warn};

use super::service::{EdgeService, EdgeServiceError, HealthCheckConfig};

impl EdgeService {
    /// Generate unique request ID for tracking
    pub fn generate_request_id(&self) -> String {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("req_{}", id)
    }

    /// Health check for the service
    pub async fn health_check(&self) -> Result<HealthStatus, EdgeServiceError> {
        let start_time = tokio::time::Instant::now();

        // Check backend health
        let healthy_backends = self.count_healthy_backends().await;
        let total_backends = self.backend_count();

        // Check component health
        let auth_healthy = self.auth.is_healthy();
        let rate_limiter_healthy = self.rate_limit_manager.is_healthy().await;
        let peer_registry_healthy = self.peer_registry.is_healthy();
        if !peer_registry_healthy {
            warn!("Peer registry health check failed: no healthy peers available");
        }

        let overall_healthy =
            healthy_backends > 0 && auth_healthy && rate_limiter_healthy && peer_registry_healthy;

        let duration = start_time.elapsed();

        Ok(HealthStatus {
            overall_healthy,
            healthy_backends,
            total_backends,
            auth_healthy,
            rate_limiter_healthy,
            peer_registry_healthy,
            check_duration: duration,
        })
    }

    /// Count healthy backends by checking Health status
    async fn count_healthy_backends(&self) -> usize {
        let health_map = self.backend_health.load();
        
        // Count backends that are ready (healthy AND enabled)
        // Health.ready() checks both health status and enabled flag
        health_map
            .values()
            .filter(|health| health.ready())
            .count()
    }

    /// Check if a specific backend is healthy
    pub fn is_backend_ready(&self, backend: &Backend) -> bool {
        let health_map = self.backend_health.load();
        health_map
            .get(&backend.hash_key())
            .map(|h| h.ready())
            .unwrap_or(false)
    }
    
    /// Get detailed health status for monitoring
    pub fn get_backend_health_status(&self) -> Vec<BackendHealthStatus> {
        let health_map = self.backend_health.load();
        let picker = self.picker.load();
        
        picker.backends.iter()
            .filter_map(|backend| {
                let is_ready = health_map
                    .get(&backend.hash_key())
                    .map(|h| h.ready())
                    .unwrap_or(false);
                    
                // Convert pingora_core::SocketAddr to std::net::SocketAddr
                // HTTP/HTTPS upstreams are always Inet addresses (not Unix domain sockets)
                backend.addr.as_inet().copied().map(|addr| BackendHealthStatus {
                    addr,
                    healthy: is_ready,
                })
            })
            .collect()
    }

    /// Get service statistics with real metric data
    pub async fn get_statistics(&self) -> ServiceStatistics {
        // Load atomic counters (Ordering::Relaxed is sufficient for statistics)
        let total = self.metrics.total_requests.load(Ordering::Relaxed);
        let successful = self.metrics.successful_requests.load(Ordering::Relaxed);
        let failed = self.metrics.failed_requests.load(Ordering::Relaxed);
        let active = self.metrics.active_connections.load(Ordering::Relaxed);
        let total_time_us = self.metrics.total_response_time_us.load(Ordering::Relaxed);

        // Calculate average response time (future: will be tracked per-request)
        let average_response_time = if total > 0 {
            Duration::from_micros(total_time_us / total)
        } else {
            Duration::from_millis(0)
        };

        ServiceStatistics {
            total_requests: total,
            successful_requests: successful,
            failed_requests: failed,
            average_response_time,
            active_connections: active,
            backend_count: self.backend_count(),
            uptime: self.start_time.elapsed(),
        }
    }
}

/// Health status information
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub overall_healthy: bool,
    pub healthy_backends: usize,
    pub total_backends: usize,
    pub auth_healthy: bool,
    pub rate_limiter_healthy: bool,
    pub peer_registry_healthy: bool,
    pub check_duration: Duration,
}

/// Service statistics for monitoring
#[derive(Debug, Clone)]
pub struct ServiceStatistics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: Duration,
    pub active_connections: u64,
    pub backend_count: usize,
    pub uptime: Duration,
}

impl ServiceStatistics {
    /// Calculate success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.successful_requests as f64 / self.total_requests as f64) * 100.0
        }
    }

    /// Calculate error rate as percentage
    pub fn error_rate(&self) -> f64 {
        100.0 - self.success_rate()
    }

    /// Check if service is performing well
    pub fn is_healthy(&self) -> bool {
        self.success_rate() >= 95.0 && self.average_response_time < Duration::from_millis(1000)
    }
}

/// Backend health status for monitoring
#[derive(Debug, Clone)]
pub struct BackendHealthStatus {
    pub addr: SocketAddr,
    pub healthy: bool,
}

/// Background health check task - implements Pingora's health check pattern
/// Reference: forks/pingora/pingora-load-balancing/src/lib.rs:249-298
pub async fn run_health_checks(
    backends: Vec<Backend>,
    health_checker: Arc<TcpHealthCheck>,
    backend_health: Arc<ArcSwap<HashMap<u64, Health>>>,
    config: HealthCheckConfig,
) {
    let mut check_interval = interval(Duration::from_millis(config.interval_ms));
    
    loop {
        check_interval.tick().await;
        
        // Pingora's check_and_report pattern (lib.rs:259-277)
        async fn check_and_report(
            backend: &Backend,
            checker: &Arc<TcpHealthCheck>,
            health_map: &HashMap<u64, Health>,
        ) {
            let check_result = checker.check(backend).await;
            let is_healthy = check_result.is_ok();
            
            if let Some(health) = health_map.get(&backend.hash_key()) {
                let threshold = checker.health_threshold(is_healthy);
                let flipped = health.observe_health(is_healthy, threshold);
                
                if flipped {
                    checker.health_status_change(backend, is_healthy).await;
                    let summary = checker.backend_summary(backend);
                    if let Err(e) = check_result {
                        warn!("{} became unhealthy: {}", summary, e);
                    } else {
                        info!("{} became healthy", summary);
                    }
                }
            }
        }
        
        let health_map = backend_health.load_full();
        
        if config.parallel {
            // Parallel health checks (more efficient)
            let mut tasks = vec![];
            for backend in backends.iter() {
                let backend = backend.clone();
                let checker = health_checker.clone();
                let hm = health_map.clone();
                
                tasks.push(tokio::spawn(async move {
                    check_and_report(&backend, &checker, &hm).await;
                }));
            }
            
            // Wait for all checks to complete
            for task in tasks {
                let _ = task.await;
            }
        } else {
            // Sequential health checks
            for backend in backends.iter() {
                check_and_report(backend, &health_checker, &health_map).await;
            }
        }
    }
}
