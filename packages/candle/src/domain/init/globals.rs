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
// use crate::memory::core::MemoryMetadata;

// Production memory configuration management
use crate::domain::memory::MemoryConfig;
use crate::memory::core::manager::surreal::SurrealDBMemoryManager;

/// Global configuration cache with copy-on-write semantics for zero-allocation access
pub static CONFIG_CACHE: LazyLock<ArcSwap<MemoryConfig>> =
    LazyLock::new(|| ArcSwap::new(Arc::new(create_default_config())));

/// Lock-free connection pool with ring buffer for zero-allocation connection management
pub static CONNECTION_POOL: LazyLock<SegQueue<Arc<SurrealDBMemoryManager>>> =
    LazyLock::new(SegQueue::new);

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
        const { std::cell::RefCell::new(None) };
}

/// Create default configuration for the domain
fn create_default_config() -> MemoryConfig {
    // 1. Load preset based on profile
    let profile = std::env::var("PARAPHYM_MEMORY_PROFILE")
        .ok()
        .unwrap_or_else(|| "default".to_string());
    
    let mut config = match profile.as_str() {
        "development" | "dev" => {
            log::info!("Loading development memory configuration preset");
            MemoryConfig::development()
        }
        "production" | "prod" => {
            log::info!("Loading production memory configuration preset");
            MemoryConfig::production()
        }
        _ => {
            log::info!("Loading default memory configuration preset");
            MemoryConfig::default()
        }
    };
    
    // 2. Load from config file if specified (overrides preset)
    if let Some(config_from_file) = load_config_file() {
        config = config_from_file;
        log::info!("Memory configuration loaded from file");
    }
    
    // 3. Apply environment variable overrides (highest priority)
    apply_env_overrides(&mut config);
    
    // 4. Validate
    if let Err(e) = config.validate() {
        log::error!("Invalid memory configuration: {e}. Falling back to safe defaults.");
        return MemoryConfig::default();
    }
    
    log::info!("Memory configuration initialized: profile={}, max_ops={}, cache_size={}, timeout_ms={}",
        profile,
        config.performance.max_concurrent_operations,
        config.performance.cache_size,
        config.performance.operation_timeout_ms
    );
    config
}

/// Apply environment variable overrides to memory configuration
fn apply_env_overrides(config: &mut MemoryConfig) {
    // Performance overrides
    if let Some(max_ops) = std::env::var("PARAPHYM_MEMORY_MAX_OPERATIONS")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.performance.max_concurrent_operations = max_ops;
        log::debug!("Override: max_concurrent_operations={max_ops}");
    }
    
    if let Some(timeout) = std::env::var("PARAPHYM_MEMORY_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.performance.operation_timeout_ms = timeout;
        log::debug!("Override: operation_timeout_ms={timeout}");
    }
    
    if let Some(cache_size) = std::env::var("PARAPHYM_MEMORY_CACHE_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.performance.cache_size = cache_size;
        log::debug!("Override: cache_size={cache_size}");
    }
    
    if let Some(batch_size) = std::env::var("PARAPHYM_MEMORY_BATCH_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.performance.batch_size = batch_size;
        log::debug!("Override: batch_size={batch_size}");
    }
    
    // Retention overrides
    if let Some(retention) = std::env::var("PARAPHYM_MEMORY_RETENTION_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.retention.default_retention_seconds = retention;
        log::debug!("Override: default_retention_seconds={retention}");
    }
    
    if let Some(max_age) = std::env::var("PARAPHYM_MEMORY_MAX_AGE_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.retention.max_age_seconds = max_age;
        log::debug!("Override: max_age_seconds={max_age}");
    }
    
    if let Some(max_memories) = std::env::var("PARAPHYM_MEMORY_MAX_ACTIVE")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.retention.max_active_memories = max_memories;
        log::debug!("Override: max_active_memories={max_memories}");
    }
    
    // Security overrides
    if let Some(enable_encryption) = std::env::var("PARAPHYM_MEMORY_ENABLE_ENCRYPTION")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.security.enable_encryption = enable_encryption;
        log::debug!("Override: enable_encryption={enable_encryption}");
    }
    
    if let Some(max_attempts) = std::env::var("PARAPHYM_MEMORY_MAX_FAILED_ATTEMPTS")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.security.max_failed_attempts = max_attempts;
        log::debug!("Override: max_failed_attempts={max_attempts}");
    }
    
    // Monitoring overrides
    if let Some(interval) = std::env::var("PARAPHYM_MEMORY_METRICS_INTERVAL_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        config.monitoring.metrics_interval_seconds = interval;
        log::debug!("Override: metrics_interval_seconds={interval}");
    }
}

/// Load configuration from TOML file if specified
fn load_config_file() -> Option<MemoryConfig> {
    let config_path = std::env::var("PARAPHYM_CONFIG_PATH").ok()?;
    
    match std::fs::read_to_string(&config_path) {
        Ok(content) => match toml::from_str::<MemoryConfig>(&content) {
            Ok(config) => {
                log::info!("Configuration loaded from: {config_path}");
                Some(config)
            }
            Err(e) => {
                log::error!("Failed to parse config file {config_path}: {e}");
                None
            }
        },
        Err(e) => {
            log::error!("Failed to read config file {config_path}: {e}");
            None
        }
    }
}
