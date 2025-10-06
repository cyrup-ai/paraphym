# STUB_4: Configuration Factory Implementation

**Priority:** 🟡 MEDIUM  
**Severity:** Incomplete Feature  
**Estimated Effort:** 1 session

## OBJECTIVE

Replace stub `create_default_config()` function with production-quality configuration factory that reads from environment variables, validates settings, and provides sensible production defaults.

## BACKGROUND

Current implementation is explicitly marked as stub:
```rust
/// Create default configuration for the domain (stub)
fn create_default_config() -> MemoryConfig {
    MemoryConfig::default()
}
```

Just returns `MemoryConfig::default()` without any production configuration, environment variable support, or validation.

## LOCATION

**File:** `packages/candle/src/init/globals.rs`  
**Line:** 54-56

## SUBTASK 1: Implement Environment Variable Reading

**What:** Read configuration from environment with production defaults  
**Where:** `create_default_config()` function

**Why:** Production deployments need configurable database paths, timeouts, and resource limits

**Implementation:**
```rust
use std::env;
use std::path::PathBuf;
use std::time::Duration;

fn create_default_config() -> MemoryConfig {
    // Database path with fallback to user data directory
    let db_path = env::var("PARAPHYM_DB_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = dirs::data_local_dir()
                .expect("Failed to determine local data directory");
            path.push("paraphym");
            path.push("memory.db");
            path
        });
    
    // Connection pool size
    let connection_pool_size = env::var("PARAPHYM_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(10);
    
    // Connection timeout in milliseconds
    let connection_timeout_ms = env::var("PARAPHYM_CONNECTION_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(5000);
    
    // Query timeout in milliseconds
    let query_timeout_ms = env::var("PARAPHYM_QUERY_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(30000);
    
    // WAL (Write-Ahead Logging) mode
    let enable_wal = env::var("PARAPHYM_ENABLE_WAL")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true);
    
    // Cache size in megabytes
    let cache_size_mb = env::var("PARAPHYM_CACHE_SIZE_MB")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(256);
    
    // Ensure database directory exists
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .expect("Failed to create database directory");
        }
    }
    
    // Build configuration
    MemoryConfig {
        database_path: db_path,
        connection_pool_size,
        connection_timeout: Duration::from_millis(connection_timeout_ms),
        query_timeout: Duration::from_millis(query_timeout_ms),
        enable_write_ahead_log: enable_wal,
        cache_size_bytes: cache_size_mb * 1024 * 1024,
        auto_vacuum: true,
        synchronous: SynchronousMode::Normal,
        journal_mode: if enable_wal { JournalMode::Wal } else { JournalMode::Delete },
        temp_store: TempStore::Memory,
        mmap_size: cache_size_mb * 1024 * 1024 / 2,
    }
}
```

## SUBTASK 2: Add Configuration Validation

**What:** Implement validation method for MemoryConfig  
**Where:** New `impl MemoryConfig` block

**Why:** Catch configuration errors early, prevent runtime failures

**Implementation:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid connection pool size: {0} (must be 1-1000)")]
    InvalidPoolSize(usize),
    
    #[error("Cache size too small: {0} bytes (minimum 1 MB)")]
    CacheTooSmall(usize),
    
    #[error("Relative database path not allowed: {0:?}")]
    RelativePath(PathBuf),
    
    #[error("Database path parent directory does not exist: {0:?}")]
    InvalidDatabasePath(PathBuf),
    
    #[error("Timeout too short: {0:?} (minimum 100ms)")]
    TimeoutTooShort(Duration),
}

impl MemoryConfig {
    /// Validate configuration values
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate pool size
        if self.connection_pool_size == 0 {
            return Err(ConfigError::InvalidPoolSize(0));
        }
        if self.connection_pool_size > 1000 {
            return Err(ConfigError::InvalidPoolSize(self.connection_pool_size));
        }
        
        // Validate cache size
        if self.cache_size_bytes < 1024 * 1024 {
            return Err(ConfigError::CacheTooSmall(self.cache_size_bytes));
        }
        
        // Database path must be absolute
        if !self.database_path.is_absolute() {
            return Err(ConfigError::RelativePath(self.database_path.clone()));
        }
        
        // Parent directory must exist (or be creatable)
        if let Some(parent) = self.database_path.parent() {
            if !parent.exists() && std::fs::create_dir_all(parent).is_err() {
                return Err(ConfigError::InvalidDatabasePath(parent.to_path_buf()));
            }
        }
        
        // Validate timeouts
        if self.connection_timeout < Duration::from_millis(100) {
            return Err(ConfigError::TimeoutTooShort(self.connection_timeout));
        }
        if self.query_timeout < Duration::from_millis(100) {
            return Err(ConfigError::TimeoutTooShort(self.query_timeout));
        }
        
        Ok(())
    }
}
```

## SUBTASK 3: Add Validation to Global Initialization

**What:** Call validation when creating CONFIG_CACHE  
**Where:** Update `CONFIG_CACHE` lazy static initialization

**Why:** Fail fast with clear error if configuration is invalid

**Implementation:**
```rust
pub static CONFIG_CACHE: Lazy<ArcSwap<MemoryConfig>> = Lazy::new(|| {
    let config = create_default_config();
    config.validate().expect("Invalid memory configuration");
    ArcSwap::new(Arc::new(config))
});
```

## SUBTASK 4: Add Configuration Logging

**What:** Log configuration on initialization for debugging  
**Where:** In `create_default_config()` before return

**Why:** Production deployments need visibility into active configuration

**Implementation:**
```rust
// At end of create_default_config(), before constructing MemoryConfig:
tracing::info!(
    "Initializing memory configuration: \
     db_path={:?}, \
     pool_size={}, \
     connection_timeout={:?}, \
     query_timeout={:?}, \
     wal={}, \
     cache_mb={}",
    db_path,
    connection_pool_size,
    Duration::from_millis(connection_timeout_ms),
    Duration::from_millis(query_timeout_ms),
    enable_wal,
    cache_size_mb
);
```

## SUBTASK 5: Update Function Documentation

**What:** Remove "(stub)" from function documentation  
**Where:** Line 53 docstring

**Old:**
```rust
/// Create default configuration for the domain (stub)
```

**New:**
```rust
/// Create production configuration from environment variables with validated defaults
///
/// # Environment Variables
///
/// - `PARAPHYM_DB_PATH`: Database file path (default: user data dir)
/// - `PARAPHYM_POOL_SIZE`: Connection pool size (default: 10, range: 1-1000)
/// - `PARAPHYM_CONNECTION_TIMEOUT_MS`: Connection timeout (default: 5000ms)
/// - `PARAPHYM_QUERY_TIMEOUT_MS`: Query timeout (default: 30000ms)
/// - `PARAPHYM_ENABLE_WAL`: Enable write-ahead logging (default: true)
/// - `PARAPHYM_CACHE_SIZE_MB`: Cache size in megabytes (default: 256)
///
/// # Panics
///
/// Panics if:
/// - Local data directory cannot be determined
/// - Database directory cannot be created
/// - Configuration validation fails
```

## SUBTASK 6: Add Dependencies

**What:** Ensure required dependencies are available  
**Where:** Check `Cargo.toml` for `packages/candle`

**Required dependencies:**
```toml
[dependencies]
dirs = "5"  # For dirs::data_local_dir()
tracing = "0.1"  # For logging
thiserror = "1"  # For ConfigError
```

## DEFINITION OF DONE

- [ ] Environment variable reading implemented for all config parameters
- [ ] Production-quality defaults set (10 connections, 5s timeout, etc.)
- [ ] Database directory creation with existence check
- [ ] Configuration validation implemented (ConfigError enum)
- [ ] Validation called in CONFIG_CACHE initialization
- [ ] Configuration logging for production debugging
- [ ] Function documentation updated with env var list
- [ ] "(stub)" marker removed from comments
- [ ] Dependencies added to Cargo.toml if needed
- [ ] Code compiles without warnings
- [ ] Validation prevents invalid configurations

## REQUIREMENTS

- ❌ **NO TESTS** - Testing team handles test coverage
- ❌ **NO BENCHMARKS** - Performance team handles benchmarking
- ✅ **PRODUCTION CODE ONLY** - Complete implementation, no stubs

## RESEARCH NOTES

### Production Default Values

| Parameter | Default | Rationale |
|-----------|---------|-----------|
| Pool Size | 10 | Balances concurrency and resource usage |
| Connection Timeout | 5000ms | Allows time for initial connection without hanging |
| Query Timeout | 30000ms | Complex queries need time, but prevent infinite waits |
| WAL Mode | true | Better concurrency, corruption resistance |
| Cache Size | 256MB | Reasonable for most workloads |

### Database Path Selection

Priority order:
1. `PARAPHYM_DB_PATH` environment variable (explicit override)
2. User data directory + `paraphym/memory.db` (standard location)
3. Current directory (fallback, not recommended)

Using `dirs::data_local_dir()`:
- Linux: `$XDG_DATA_HOME` or `~/.local/share`
- macOS: `~/Library/Application Support`
- Windows: `{FOLDERID_LocalAppData}`

### SQLite Configuration

- **WAL mode**: Write-Ahead Logging for better concurrency
- **Synchronous Normal**: Balance safety and performance
- **Auto Vacuum**: Prevent database bloat
- **Temp Store Memory**: Faster temporary tables
- **mmap_size**: Memory-mapped I/O for performance (half of cache)

### Validation Ranges

- Pool size: 1-1000 (prevent resource exhaustion)
- Cache: >= 1MB (below this is ineffective)
- Timeouts: >= 100ms (below this is unrealistic)
- Path: Must be absolute (relative paths are deployment-specific)

## VERIFICATION

After implementation, verify:
1. Default configuration loads without panics
2. Environment variables override defaults correctly
3. Invalid values rejected with clear errors
4. Database directory created if missing
5. Configuration logged on startup
6. Documentation accurately reflects environment variables
7. Validation catches all invalid configurations
