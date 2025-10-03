//! Edge service operations and utility methods
//!
//! This module provides utility operations for EdgeService such as health checks
//! and statistics. HTTP request handling is done via ProxyHttp trait implementation
//! in proxy_impl.rs and authentication functions in auth/validation.rs.


use std::sync::atomic::Ordering;
use tokio::time::Duration;

use super::service::{EdgeService, EdgeServiceError};

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
        let rate_limiter_healthy = self.rate_limit_manager.is_healthy();
        let peer_registry_healthy = true; // TODO: implement health check for peer registry

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

    /// Count healthy backends by performing TCP health checks
    async fn count_healthy_backends(&self) -> usize {
        // Get number of backends from metrics targets
        let targets = self.picker.load().get_metrics_targets();
        let backend_count = targets.len();
        
        if backend_count == 0 {
            return 0;
        }
        
        // For now, assume all backends are healthy if we have metrics targets
        // TODO: Implement actual TCP health checks
        backend_count
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
