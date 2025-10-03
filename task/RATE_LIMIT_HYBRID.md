# Rate Limiting: Enable HybridAlgorithm in DistributedRateLimitManager

## Status
**PARTIALLY CONNECTED** - HybridAlgorithm accessible via AdvancedRateLimitManager, not via DistributedRateLimitManager

## Problem Statement (CORRECTED)

The HybridAlgorithm (combines TokenBucket + SlidingWindow for stricter rate limiting) exists and is **already usable** via `AdvancedRateLimitManager`, but is **not accessible** via `DistributedRateLimitManager`.

### Current State
- ✅ HybridAlgorithm exists: [`src/rate_limit/algorithms.rs:458-518`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs)
- ✅ AdvancedRateLimitManager CAN use HybridAlgorithm via `RateLimitAlgorithmType::Hybrid`: [`src/rate_limit/limiter.rs:192-195, 209-212`](../packages/sweetmcp/packages/pingora/src/rate_limit/limiter.rs)
- ❌ DistributedRateLimitManager CANNOT use HybridAlgorithm (uses `algorithms::RateLimiter` enum which lacks Hybrid variant)

### Architecture (As-Is)

```
RateLimitAlgorithm trait (low-level interface)
  ├── TokenBucket ✓
  ├── SlidingWindow ✓
  └── HybridAlgorithm ✓ (EXISTS, implements trait)

algorithms.rs enums (used by DistributedRateLimitManager):
  RateLimitAlgorithmConfig (lines 406-410)
    ├── TokenBucket(TokenBucketConfig)
    └── SlidingWindow(SlidingWindowConfig)
    └── Hybrid(TokenBucketConfig, SlidingWindowConfig) ✗ MISSING

  RateLimiter (lines 318-321)
    ├── TokenBucket(TokenBucket)
    └── SlidingWindow(SlidingWindow)
    └── Hybrid(HybridAlgorithm) ✗ MISSING

limiter.rs (used by AdvancedRateLimitManager):
  RateLimitAlgorithmType (line 373)
    ├── TokenBucket
    ├── SlidingWindow
    └── Hybrid ✓ PRESENT

Managers:
  AdvancedRateLimitManager (limiter.rs:59-74)
    - Uses Box<dyn RateLimitAlgorithm> → CAN use HybridAlgorithm ✓
    - Selects algorithm via RateLimitAlgorithmType enum (has Hybrid variant)

  DistributedRateLimitManager (distributed.rs:36-45)
    - Uses algorithms::RateLimiter enum → CANNOT use HybridAlgorithm ✗
    - Needs Hybrid variant added to enums it uses

Wrapper (mod.rs:17-22):
  RateLimiter enum
    ├── Distributed(Arc<DistributedRateLimitManager>)
    └── Advanced(Arc<AdvancedRateLimitManager>) ✓ Can access Hybrid

EdgeService (edge/core/service.rs:60)
  - Uses wrapper RateLimiter enum
  - Defaults to Distributed (line 124)
  - Can use Advanced variant to access HybridAlgorithm
```

## Objective

Enable HybridAlgorithm support in `DistributedRateLimitManager` by adding Hybrid variants to the enums it uses (`RateLimitAlgorithmConfig` and `RateLimiter` in `algorithms.rs`).

## Implementation Steps

### 1. Add Hybrid to RateLimitAlgorithmConfig Enum
**File**: [`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs) (lines 406-410)

**Current**:
```rust
pub enum RateLimitAlgorithmConfig {
    TokenBucket(TokenBucketConfig),
    SlidingWindow(SlidingWindowConfig),
}
```

**Change to**:
```rust
pub enum RateLimitAlgorithmConfig {
    TokenBucket(TokenBucketConfig),
    SlidingWindow(SlidingWindowConfig),
    Hybrid(TokenBucketConfig, SlidingWindowConfig),
}
```

### 2. Add Hybrid to RateLimiter Enum
**File**: [`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs) (lines 318-321)

**Current**:
```rust
pub enum RateLimiter {
    TokenBucket(TokenBucket),
    SlidingWindow(SlidingWindow),
}
```

**Change to**:
```rust
pub enum RateLimiter {
    TokenBucket(TokenBucket),
    SlidingWindow(SlidingWindow),
    Hybrid(HybridAlgorithm),
}
```

### 3. Update RateLimiter::new()
**File**: [`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs) (lines 326-335)

Add match arm:
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
**File**: [`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs) (lines 338-346)

Add match arm:
```rust
pub fn check_request(&mut self, tokens: u32) -> bool {
    match self {
        Self::TokenBucket(bucket) => bucket.try_consume(tokens),
        Self::SlidingWindow(window) => window.try_request(),
        Self::Hybrid(hybrid) => {
            // Hybrid requires both algorithms to allow
            for _ in 0..tokens {
                if !hybrid.try_request() {
                    return false;
                }
            }
            true
        }
    }
}
```

### 5. Update RateLimiter::update_config()
**File**: [`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs) (lines 349-370)

Add match arms for Hybrid in both outer and inner matches:
```rust
pub fn update_config(&mut self, algorithm: &RateLimitAlgorithmConfig) {
    match algorithm {
        RateLimitAlgorithmConfig::TokenBucket(config) => { /* existing */ }
        RateLimitAlgorithmConfig::SlidingWindow(config) => { /* existing */ }
        RateLimitAlgorithmConfig::Hybrid(token_config, window_config) => {
            match self {
                Self::Hybrid(hybrid) => {
                    // Update both internal algorithms
                    *self = Self::Hybrid(HybridAlgorithm::new(
                        token_config.clone(),
                        window_config.clone()
                    ));
                }
                _ => {
                    // Switch to Hybrid
                    *self = Self::Hybrid(HybridAlgorithm::new(
                        token_config.clone(),
                        window_config.clone()
                    ));
                }
            }
        }
    }
}
```

### 6. Update RateLimiter::get_state()
**File**: [`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs) (lines 373-378)

Add match arm:
```rust
pub fn get_state(&self) -> AlgorithmState {
    match self {
        Self::TokenBucket(bucket) => bucket.get_state(),
        Self::SlidingWindow(window) => window.get_state(),
        Self::Hybrid(hybrid) => hybrid.get_state(),
    }
}
```

### 7. Update Remaining RateLimiter Methods
**File**: [`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs) (lines 380+)

Add Hybrid match arms to:
- `is_active()`: `Self::Hybrid(hybrid) => hybrid.is_active(),`
- `reset()`: `Self::Hybrid(hybrid) => hybrid.reset(),`
- `last_used()`: `Self::Hybrid(hybrid) => hybrid.last_used(),`

### 8. Update DistributedRateLimitManager::check_endpoint_request()
**File**: [`src/rate_limit/distributed.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs) (lines 137-157)

Add match arm:
```rust
fn check_endpoint_request(&self, endpoint: &str, config: &EndpointRateConfig, tokens: u32) -> bool {
    if !self.endpoint_limiters.contains_key(endpoint) {
        let limiter = match &config.algorithm {
            RateLimitAlgorithmConfig::TokenBucket(token_config) => {
                RateLimiter::TokenBucket(super::algorithms::TokenBucket::new(token_config.clone()))
            }
            RateLimitAlgorithmConfig::SlidingWindow(window_config) => {
                RateLimiter::SlidingWindow(super::algorithms::SlidingWindow::new(window_config.clone()))
            }
            RateLimitAlgorithmConfig::Hybrid(token_config, window_config) => {
                RateLimiter::Hybrid(super::algorithms::HybridAlgorithm::new(
                    token_config.clone(),
                    window_config.clone()
                ))
            }
        };
        self.endpoint_limiters.insert(endpoint.to_string(), limiter);
    }
    // ... rest of method
}
```

### 9. Update DistributedRateLimitManager::check_peer_request()
**File**: [`src/rate_limit/distributed.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs) (lines 172-194)

Add match arm (same pattern as step 8):
```rust
fn check_peer_request(&self, endpoint: &str, peer_ip: &str, config: &EndpointRateConfig, tokens: u32) -> bool {
    // ... setup code ...
    if !endpoint_limiters.contains_key(peer_ip) {
        let limiter = match &config.algorithm {
            RateLimitAlgorithmConfig::TokenBucket(token_config) => {
                RateLimiter::TokenBucket(super::algorithms::TokenBucket::new(token_config.clone()))
            }
            RateLimitAlgorithmConfig::SlidingWindow(window_config) => {
                RateLimiter::SlidingWindow(super::algorithms::SlidingWindow::new(window_config.clone()))
            }
            RateLimitAlgorithmConfig::Hybrid(token_config, window_config) => {
                RateLimiter::Hybrid(super::algorithms::HybridAlgorithm::new(
                    token_config.clone(),
                    window_config.clone()
                ))
            }
        };
        endpoint_limiters.insert(peer_ip.to_string(), limiter);
    }
    // ... rest of method
}
```

### 10. Update DistributedRateLimitManager::get_stats()
**File**: [`src/rate_limit/distributed.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs) (around line 289)

Update algorithm name display:
```rust
endpoint_info.insert(
    "algorithm",
    serde_json::json!(match &config.algorithm {
        RateLimitAlgorithmConfig::TokenBucket(_) => "TokenBucket",
        RateLimitAlgorithmConfig::SlidingWindow(_) => "SlidingWindow",
        RateLimitAlgorithmConfig::Hybrid(_, _) => "Hybrid",
    }),
);
```

## Files to Modify (2 files)

1. **[`src/rate_limit/algorithms.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs)** - Add Hybrid to both enums and all match statements
2. **[`src/rate_limit/distributed.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs)** - Add Hybrid match arms where configs are used

## Definition of Done

1. `RateLimitAlgorithmConfig::Hybrid` variant added with two config parameters
2. `RateLimiter::Hybrid` variant added wrapping `HybridAlgorithm`
3. All `RateLimiter` methods handle the Hybrid variant correctly
4. `DistributedRateLimitManager` can create and use Hybrid limiters
5. Hybrid algorithm enforces both token bucket AND sliding window limits (both must allow)
6. Configuration example works:

```rust
use sweetmcp::rate_limit::*;

// Create distributed manager
let manager = DistributedRateLimitManager::new();

// Configure endpoint with Hybrid algorithm
manager.configure_endpoint(
    "/api/critical".to_string(),
    EndpointRateConfig {
        algorithm: RateLimitAlgorithmConfig::Hybrid(
            TokenBucketConfig {
                capacity: 100,
                refill_rate: 10.0,
                initial_tokens: 100,
            },
            SlidingWindowConfig {
                window_size: 60,
                max_requests: 50,
                sub_windows: 6,
            }
        ),
        per_peer: true,
        trusted_multiplier: 1.0,
    }
);

// Both token bucket AND sliding window must allow request
let allowed = manager.check_request("/api/critical", Some("192.168.1.1"), 1);
```

## References

- HybridAlgorithm implementation: [`src/rate_limit/algorithms.rs:458-518`](../packages/sweetmcp/packages/pingora/src/rate_limit/algorithms.rs)
- AdvancedRateLimitManager (working example): [`src/rate_limit/limiter.rs:192-195`](../packages/sweetmcp/packages/pingora/src/rate_limit/limiter.rs)
- DistributedRateLimitManager: [`src/rate_limit/distributed.rs`](../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs)
- RateLimiter wrapper enum: [`src/rate_limit/mod.rs:17-22`](../packages/sweetmcp/packages/pingora/src/rate_limit/mod.rs)
