//! Rate limiting module decomposition
//!
//! This module provides the decomposed rate limiting functionality split into
//! logical modules for better maintainability and adherence to the 300-line limit.


#![allow(dead_code)]

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

    /// Check if the rate limiter is healthy
    pub async fn is_healthy(&self) -> bool {
        match self {
            RateLimiter::Distributed(mgr) => mgr.is_healthy(),
            RateLimiter::Advanced(mgr) => mgr.is_healthy().await,
        }
    }

    /// Get the distributed rate limit manager if this is a Distributed variant
    pub fn as_distributed(&self) -> Option<Arc<DistributedRateLimitManager>> {
        match self {
            RateLimiter::Distributed(mgr) => Some(Arc::clone(mgr)),
            RateLimiter::Advanced(_) => None,
        }
    }
}
