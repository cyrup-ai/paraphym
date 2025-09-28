//! Global state management for the domain
//!
//! This module contains all global static variables, initialization counters,
//! and shared state that needs to be accessed across the domain.

use std::sync::{atomic::AtomicUsize, Arc, LazyLock};

use arc_swap::ArcSwap;
use atomic_counter::RelaxedCounter;
use crossbeam::queue::SegQueue;
use crossbeam_utils::CachePadded;


use crate::domain::error::SimpleCircuitBreaker;
// Temporarily disabled to break circular dependency
// use crate::memory::{MemoryConfig, SurrealDBMemoryManager};
// use crate::memory::memory::MemoryMetadata;

// Use stub types from memory::manager
use crate::domain::memory::MemoryConfig;
use crate::memory::memory::manager::surreal::SurrealDBMemoryManager;

/// Global configuration cache with copy-on-write semantics for zero-allocation access
pub static CONFIG_CACHE: LazyLock<ArcSwap<MemoryConfig>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(create_default_config())));

/// Lock-free connection pool with ring buffer for zero-allocation connection management
pub static CONNECTION_POOL: LazyLock<SegQueue<Arc<SurrealDBMemoryManager>>> =
    LazyLock::new(|| SegQueue::new());

/// Circuit breaker for error recovery with exponential backoff
pub static CIRCUIT_BREAKER: LazyLock<SimpleCircuitBreaker> =
    LazyLock::new(|| SimpleCircuitBreaker::new(5, 30000)); // 30 seconds in milliseconds

/// Global initialization statistics for monitoring
pub static INIT_STATS: LazyLock<CachePadded<RelaxedCounter>> =
    LazyLock::new(|| CachePadded::new(RelaxedCounter::new(0)));

/// Pool statistics for monitoring
pub static POOL_STATS: LazyLock<CachePadded<AtomicUsize>> =
    LazyLock::new(|| CachePadded::new(AtomicUsize::new(0)));

/// Circuit breaker reset statistics
pub static CIRCUIT_BREAKER_RESET_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static CIRCUIT_BREAKER_LAST_RESET: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

// Thread-local storage for configuration caching
thread_local! {
    pub static LOCAL_CONFIG: std::cell::RefCell<Option<Arc<MemoryConfig>>> =
        std::cell::RefCell::new(None);
}

/// Create default configuration for the domain (stub)
fn create_default_config() -> MemoryConfig {
    MemoryConfig::default()
}
