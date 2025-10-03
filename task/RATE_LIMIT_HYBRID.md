# Rate Limiting: HybridAlgorithm Disconnection

## Status
**DISCONNECTED** - HybridAlgorithm exists but unreachable

## Problem
HybridAlgorithm (combines TokenBucket + SlidingWindow for stricter limiting) is fully implemented but cannot be instantiated.

## Root Cause
1. **HybridAlgorithm exists**: `rate_limit/algorithms.rs:458-518`
2. **AdvancedRateLimitManager supports it**: Uses `Box<dyn RateLimitAlgorithm>`
3. **DistributedRateLimitManager cannot use it**: Uses `RateLimiter` enum which lacks Hybrid variant
4. **RateLimitAlgorithmConfig missing Hybrid variant**: `algorithms.rs:406-410`
5. **EdgeService hardcoded to Distributed**: `edge/core/service.rs:124`

## Architecture
```
RateLimitAlgorithm trait (low-level)
  ├── TokenBucket ✓
  ├── SlidingWindow ✓
  └── HybridAlgorithm ✓ (EXISTS BUT UNREACHABLE)

RateLimiter enum (mid-level)
  ├── TokenBucket(TokenBucket)
  └── SlidingWindow(SlidingWindow)
  └── Hybrid(HybridAlgorithm) ✗ MISSING

RateLimitAlgorithmConfig enum
  ├── TokenBucket(TokenBucketConfig)
  └── SlidingWindow(SlidingWindowConfig)
  └── Hybrid(TokenBucketConfig, SlidingWindowConfig) ✗ MISSING

Managers:
  AdvancedRateLimitManager (NEVER INSTANTIATED)
    - Uses Box<dyn RateLimitAlgorithm> → CAN access Hybrid
    - Has cleanup_handle for background task
    - Has operational state tracking

  DistributedRateLimitManager (HARDCODED EVERYWHERE)
    - Uses RateLimiter enum → CANNOT access Hybrid
    - Has load_multiplier for adaptive limiting
    - Has per-endpoint configs
```

## Reconnection Steps

### 1. Add Hybrid to RateLimitAlgorithmConfig
**File**: `rate_limit/algorithms.rs:406-410`
```rust
pub enum RateLimitAlgorithmConfig {
    TokenBucket(TokenBucketConfig),
    SlidingWindow(SlidingWindowConfig),
    Hybrid(TokenBucketConfig, SlidingWindowConfig), // ADD THIS
}
```

### 2. Add Hybrid to RateLimiter enum
**File**: `rate_limit/algorithms.rs:318-322`
```rust
pub enum RateLimiter {
    TokenBucket(TokenBucket),
    SlidingWindow(SlidingWindow),
    Hybrid(HybridAlgorithm), // ADD THIS
}
```

### 3. Update RateLimiter::new()
**File**: `rate_limit/algorithms.rs:326-335`
```rust
pub fn new(algorithm: &RateLimitAlgorithmConfig) -> Self {
    match algorithm {
        RateLimitAlgorithmConfig::TokenBucket(config) => {
            Self::TokenBucket(TokenBucket::new(config.clone()))
        }
        RateLimitAlgorithmConfig::SlidingWindow(config) => {
            Self::SlidingWindow(SlidingWindow::new(config.clone()))
        }
        RateLimitAlgorithmConfig::Hybrid(token_config, window_config) => {
            Self::Hybrid(HybridAlgorithm::new(token_config.clone(), window_config.clone()))
        }
    }
}
```

### 4. Update RateLimiter::check_request()
**File**: `rate_limit/algorithms.rs:338-346`
Add Hybrid case to match.

### 5. Update RateLimiter::update_config()
**File**: `rate_limit/algorithms.rs:349-370`
Add Hybrid case to match.

### 6. Update RateLimiter::get_state()
**File**: `rate_limit/algorithms.rs:373-378`
Add Hybrid case to match.

### 7. Update RateLimiter methods (is_active, reset, last_used)
Add Hybrid case to all match statements.

### 8. Create RateLimitManager Trait
**File**: `rate_limit/mod.rs` (add trait)
```rust
pub trait RateLimitManager: Send + Sync {
    fn check_request(&self, endpoint: &str, peer_id: Option<&str>, count: u32) -> bool;
    fn cleanup_unused_limiters(&self);
    fn is_healthy(&self) -> bool;
}
```

### 9. Impl trait for both managers
- `impl RateLimitManager for AdvancedRateLimitManager`
- `impl RateLimitManager for DistributedRateLimitManager`

### 10. Update EdgeService
**File**: `edge/core/service.rs:59`
```rust
pub rate_limit_manager: Arc<dyn RateLimitManager>, // was Arc<DistributedRateLimitManager>
```

### 11. Update EdgeServiceBuilder
**File**: `edge/core/builder.rs:23,65`
```rust
custom_rate_limiter: Option<Arc<dyn RateLimitManager>>, // was Arc<DistributedRateLimitManager>

pub fn with_custom_rate_limiter(mut self, rate_limiter: Arc<dyn RateLimitManager>) -> Self
```

## Testing
1. Create Hybrid config: `RateLimitAlgorithmConfig::Hybrid(token, window)`
2. Use in DistributedRateLimitManager via configure_endpoint()
3. Verify both token bucket AND sliding window enforced
4. Test AdvancedRateLimitManager instantiation
5. Test switching between managers via builder

## Files Modified
- `rate_limit/algorithms.rs` - Add Hybrid to enums and matches
- `rate_limit/mod.rs` - Add RateLimitManager trait
- `rate_limit/limiter.rs` - Impl trait for AdvancedRateLimitManager
- `rate_limit/distributed.rs` - Impl trait for DistributedRateLimitManager
- `edge/core/service.rs` - Use trait object
- `edge/core/builder.rs` - Accept trait object
