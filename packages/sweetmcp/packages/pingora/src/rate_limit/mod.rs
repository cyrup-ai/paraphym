//! Rate limiting module decomposition
//!
//! This module provides the decomposed rate limiting functionality split into
//! logical modules for better maintainability and adherence to the 300-line limit.

use std::sync::Arc;

pub mod algorithms;
pub mod distributed;
pub mod limiter;

pub use distributed::DistributedRateLimitManager;
pub use limiter::AdvancedRateLimitManager;

/// Polymorphic rate limiter wrapper supporting multiple rate limiting strategies
#[derive(Clone)]
pub enum RateLimiter {
    /// Distributed rate limiting with per-endpoint and per-peer tracking
    Distributed(Arc<DistributedRateLimitManager>),
    /// Advanced configurable rate limiting with custom parameters
    Advanced(Arc<AdvancedRateLimitManager>),
}

impl RateLimiter {
    /// Check if a request is allowed under the configured rate limit
    pub fn check_request(&self, endpoint: &str, peer_ip: Option<&str>, tokens: u32) -> bool {
        match self {
            RateLimiter::Distributed(mgr) => mgr.check_request(endpoint, peer_ip, tokens),
            RateLimiter::Advanced(mgr) => mgr.check_request(endpoint, peer_ip, tokens),
        }
    }
}
