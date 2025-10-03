# Edge Service Builder Pattern Disconnection

## Status
**DISCONNECTED** - Builder pattern exists but EdgeService::new() called directly

## Problem Analysis

### Core Issue
The builder pattern infrastructure is fully implemented but completely bypassed in **two critical places**:

1. **[main.rs:163](../packages/sweetmcp/packages/pingora/src/main.rs#L163)** - Direct `EdgeService::new()` call
2. **[builder.rs:118](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs#L118)** - `Builder.build()` also calls `EdgeService::new()`, defeating its purpose

### Why This Matters

The builder exists to provide:
- **Flexibility**: Swap rate limiters (DistributedRateLimitManager ↔ AdvancedRateLimitManager)
- **Validation**: Pre-construction config checking via `validate()`
- **Presets**: Environment-specific configurations (dev/prod/test)
- **Testability**: Inject mock components for isolated testing

**But none of this works** because the builder ultimately calls the hardcoded constructor.

## Source File Reference Map

### Core Files
- **[builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)** - EdgeServiceBuilder implementation
  - Lines 18-25: Builder struct definition
  - Lines 29-80: Fluent builder methods
  - Lines 82-134: `build()` method (CALLS EdgeService::new - THE PROBLEM)
  - Lines 274-292: `with_preset()` (EMPTY - needs implementation)
  - Lines 355-378: Preset enum and convenience functions

- **[service.rs](../packages/sweetmcp/packages/pingora/src/edge/core/service.rs)** - EdgeService struct
  - Lines 66-73: EdgeService struct with public fields
  - Lines 74-165: `EdgeService::new()` - hardcoded constructor
  - Lines 260-287: `validate_config()` - validation logic

- **[main.rs](../packages/sweetmcp/packages/pingora/src/main.rs)** - Service initialization
  - Line 163: **DIRECT CALL** to `EdgeService::new()` - bypasses builder

### Rate Limiter Files
- **[distributed.rs](../packages/sweetmcp/packages/pingora/src/rate_limit/distributed.rs)** - DistributedRateLimitManager
  - Line 47: `new()` with no parameters (production default)
  
- **[limiter.rs](../packages/sweetmcp/packages/pingora/src/rate_limit/limiter.rs)** - AdvancedRateLimitManager
  - Line 59: Struct definition
  - Line 75: `new(requests_per_second, burst_size, window_size_seconds)` - configurable

## Rate Limiter Comparison

### DistributedRateLimitManager (Production Default)
```rust
// File: src/rate_limit/distributed.rs
impl DistributedRateLimitManager {
    pub fn new() -> Self {
        // No parameters - uses hardcoded defaults
        // Per-endpoint and per-peer tracking
        // Production-ready limits
    }
}
```

**Usage Pattern:**
```rust
let rate_limiter = Arc::new(DistributedRateLimitManager::new());
```

**Characteristics:**
- Zero configuration required
- Per-endpoint rate limiting with endpoint-specific rules
- Per-peer tracking with trusted peer multipliers
- Dynamic load-based adaptive limits
- Production-optimized defaults

### AdvancedRateLimitManager (Flexible Alternative)
```rust
// File: src/rate_limit/limiter.rs  
impl AdvancedRateLimitManager {
    pub fn new(
        requests_per_second: f64,
        burst_size: u32, 
        window_size_seconds: u64
    ) -> Self {
        // Configurable global settings
        // Per-endpoint and per-peer tracking
        // Customizable for dev/test environments
    }
}
```

**Usage Pattern:**
```rust
// Development: Low limits for testing
let rate_limiter = Arc::new(AdvancedRateLimitManager::new(
    10.0,   // 10 requests/second
    100,    // burst of 100
    60      // 60 second window
));

// Testing: Permissive limits
let rate_limiter = Arc::new(AdvancedRateLimitManager::new(
    1000.0, // 1000 requests/second  
    10000,  // burst of 10000
    3600    // 1 hour window
));
```

**Characteristics:**
- Fully configurable rate limits
- Supports Token Bucket and Sliding Window algorithms
- Ideal for development and testing environments
- Per-endpoint and per-peer granularity

## Architecture Pattern Flaw

### Current Flawed Implementation

**[builder.rs:118](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs#L118):**
```rust
pub fn build(self) -> Result<EdgeService, EdgeServiceError> {
    // ... validation ...
    
    // ❌ THE PROBLEM: Builder calls EdgeService::new()
    let mut service = EdgeService::new(
        cfg,
        bridge_tx, 
        peer_registry,
        circuit_breaker_manager
    );
    
    // ❌ Then tries to override fields after construction
    if let Some(custom_rate_limiter) = self.custom_rate_limiter {
        service.rate_limit_manager = custom_rate_limiter;
    }
    
    // This pattern defeats the entire purpose of the builder!
    Ok(service)
}
```

### Why This Is Wrong

1. **EdgeService::new() hardcodes components:**
   - Line 128: `DistributedRateLimitManager::new()` - always created
   - Line 129-131: `ShutdownCoordinator::new()` - always created
   - Cannot inject alternatives during construction

2. **Builder override is wasteful:**
   - Creates default components
   - Immediately throws them away
   - Replaces with builder-provided components
   - Allocates and discards unnecessarily

3. **Defeats builder pattern benefits:**
   - No true construction flexibility
   - Presets cannot avoid hardcoded defaults
   - Testing still creates production components first

### Correct Implementation Pattern

**Builder should construct EdgeService directly:**
```rust
pub fn build(self) -> Result<EdgeService, EdgeServiceError> {
    // Validate
    let cfg = self.cfg.ok_or(...)?;
    let bridge_tx = self.bridge_tx.ok_or(...)?;
    let peer_registry = self.peer_registry.ok_or(...)?;
    
    // Choose rate limiter (builder-provided or default)
    let rate_limit_manager = self.custom_rate_limiter
        .unwrap_or_else(|| Arc::new(DistributedRateLimitManager::new()));
    
    // Choose shutdown coordinator (builder-provided or default)  
    let shutdown_coordinator = self.custom_shutdown_coordinator
        .unwrap_or_else(|| Arc::new(ShutdownCoordinator::new(...)));
    
    // Construct EdgeService DIRECTLY (not via ::new())
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
        metrics: Arc::new(AtomicMetrics::new()),
        start_time: Instant::now(),
        token_manager,
    };
    
    service.validate_config()?;
    Ok(service)
}
```

## Disconnected Components (31 items)

### 1. Builder Struct (Never Constructed)
**File**: [edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)
- `EdgeServiceBuilder` never instantiated (line 18-25)
- All builder methods unused:
  - `new()` (line 29)
  - `with_config()` (line 41)
  - `with_bridge_channel()` (line 51)
  - `with_peer_registry()` (line 58)
  - `with_custom_rate_limiter()` (line 65)
  - `with_custom_shutdown_coordinator()` (line 72)
  - `build()` (line 82)
  - `build_for_testing()` (line 129)

### 2. Builder Presets (Never Used)
**File**: [edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)
- `BuilderPreset` enum exists (line 355-360)
- `development()` never called (line 365)
- `production()` never called (line 370)
- `testing()` never called (line 375)
- `with_preset()` never called (line 274)
- **CRITICAL: Preset implementations are EMPTY** (line 276-290)

### 3. Builder Utilities (Never Used)
**File**: [edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)
- `validate()` never called (line 174)
- `status()` never called (line 212)
- `reset()` never called (line 224)
- `clone_builder()` never called (line 235)
- `build_multiple()` never called (line 246)
- `from_service()` never called (line 263)

### 4. BuilderStatus (Never Constructed)
**File**: [edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)
- `BuilderStatus` struct (line 302-310)
- `is_complete()` never called (line 314)
- `completion_percentage()` never called (line 319)
- `missing_components()` never called (line 337)

## Implementation Steps

### Step 1: Fix Builder.build() Method

**File**: [packages/sweetmcp/packages/pingora/src/edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)

**Line 82-134: Replace entire `build()` method:**

```rust
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
        .unwrap_or_else(|| Arc::new(DistributedRateLimitManager::new()));

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
        metrics: Arc::new(AtomicMetrics::new()),
        start_time: Instant::now(),
        token_manager,
    };

    // Validate the built service
    service.validate_config()?;

    info!("EdgeService built successfully");
    Ok(service)
}
```

### Step 2: Implement Preset Logic

**File**: [packages/sweetmcp/packages/pingora/src/edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)

**Line 274-292: Replace `with_preset()` implementation:**

```rust
/// Apply configuration preset
pub fn with_preset(self, preset: BuilderPreset) -> Self {
    match preset {
        BuilderPreset::Development => {
            debug!("Applying development preset");
            // Low rate limits for local development
            let rate_limiter = Arc::new(AdvancedRateLimitManager::new(
                10.0,   // 10 requests/second
                100,    // burst of 100
                60      // 60 second window
            ));
            self.with_custom_rate_limiter(rate_limiter)
        }
        BuilderPreset::Production => {
            debug!("Applying production preset");
            // Use default DistributedRateLimitManager (high performance)
            let rate_limiter = Arc::new(DistributedRateLimitManager::new());
            self.with_custom_rate_limiter(rate_limiter)
        }
        BuilderPreset::Testing => {
            debug!("Applying testing preset");
            // Permissive limits for test suites
            let rate_limiter = Arc::new(AdvancedRateLimitManager::new(
                1000.0,  // 1000 requests/second
                10000,   // burst of 10000
                3600     // 1 hour window
            ));
            self.with_custom_rate_limiter(rate_limiter)
        }
    }
}
```

### Step 3: Update main.rs to Use Builder

**File**: [packages/sweetmcp/packages/pingora/src/main.rs](../packages/sweetmcp/packages/pingora/src/main.rs)

**Line 162-163: Replace direct EdgeService::new() call:**

```rust
// OLD (line 162-163):
let edge_service =
    edge::EdgeService::new(cfg.clone(), bridge_tx.clone(), peer_registry.clone(), circuit_breaker_manager.clone());

// NEW:
use crate::edge::EdgeServiceBuilder;

let edge_service = EdgeServiceBuilder::new()
    .with_config(cfg.clone())
    .with_bridge_channel(bridge_tx.clone())
    .with_peer_registry(peer_registry.clone())
    .build()
    .expect("Failed to build EdgeService");
```

### Step 4: Environment-Based Presets (Optional Enhancement)

**File**: [packages/sweetmcp/packages/pingora/src/main.rs](../packages/sweetmcp/packages/pingora/src/main.rs)

**Alternative main.rs implementation with environment detection:**

```rust
use crate::edge::{EdgeServiceBuilder, BuilderPreset};

// Detect environment from ENV var or config
let preset = match std::env::var("SWEETMCP_ENV").as_deref() {
    Ok("development") => BuilderPreset::Development,
    Ok("testing") => BuilderPreset::Testing,
    _ => BuilderPreset::Production,
};

let edge_service = EdgeServiceBuilder::new()
    .with_preset(preset)
    .with_config(cfg.clone())
    .with_bridge_channel(bridge_tx.clone())
    .with_peer_registry(peer_registry.clone())
    .build()
    .expect("Failed to build EdgeService");
```

### Step 5: Add Required Imports to builder.rs

**File**: [packages/sweetmcp/packages/pingora/src/edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)

**Add to imports (after line 16):**

```rust
use std::collections::{BTreeSet, HashMap};
use std::net::SocketAddr;
use std::time::Instant;

use arc_swap::ArcSwap;
use pingora_load_balancing::Backend;

use crate::{
    auth::JwtAuth,
    crypto::core::TokenManager,
    load::Load,
    metric_picker::MetricPicker,
    peer_discovery::PeerDiscovery,
    rate_limit::limiter::AdvancedRateLimitManager,
};
```

## Files to Modify

1. **[packages/sweetmcp/packages/pingora/src/edge/core/builder.rs](../packages/sweetmcp/packages/pingora/src/edge/core/builder.rs)**
   - Line 1-16: Add imports (Step 5)
   - Line 82-134: Replace `build()` method (Step 1)
   - Line 274-292: Replace `with_preset()` implementation (Step 2)

2. **[packages/sweetmcp/packages/pingora/src/main.rs](../packages/sweetmcp/packages/pingora/src/main.rs)**
   - Line 162-163: Replace EdgeService::new() with builder pattern (Step 3)
   - Optional: Add environment-based preset selection (Step 4)

3. **No changes needed to:**
   - [service.rs](../packages/sweetmcp/packages/pingora/src/edge/core/service.rs) - EdgeService struct already has public fields
   - Rate limiter files - both managers already support the needed APIs

## Definition of Done

### Functional Requirements
1. ✅ **Builder constructs EdgeService directly** - No call to EdgeService::new() in build()
2. ✅ **Presets configure rate limiters** - Development/Production/Testing presets work correctly
3. ✅ **main.rs uses builder** - EdgeServiceBuilder used instead of direct constructor
4. ✅ **Rate limiter swapping works** - Can choose between Distributed and Advanced managers
5. ✅ **Validation runs before construction** - validate() called in build() method

### Code Quality Requirements
1. ✅ **Zero compilation errors** - All changes compile successfully
2. ✅ **All imports resolved** - Required imports added to builder.rs
3. ✅ **Logging preserved** - debug!() and info!() calls maintained
4. ✅ **Error handling intact** - All Result types and error messages preserved

### Behavioral Requirements
1. ✅ **Service initializes correctly** - EdgeService starts with all components
2. ✅ **Presets apply different configs** - Development uses AdvancedRateLimitManager, Production uses DistributedRateLimitManager
3. ✅ **Custom components injectable** - with_custom_rate_limiter() and with_custom_shutdown_coordinator() work
4. ✅ **Configuration validation runs** - Invalid configs rejected before construction

### Verification Steps (Manual)
1. Run `cargo check -p sweetmcp_pingora` - should compile without errors
2. Run the edge service with SWEETMCP_ENV=development - should use AdvancedRateLimitManager
3. Run the edge service with SWEETMCP_ENV=production - should use DistributedRateLimitManager
4. Check logs for "Building EdgeService" and "EdgeService built successfully" messages

## Benefits Unlocked

Once reconnected, the builder pattern will provide:

1. **Environment-specific configs** via presets
   - Development: Low rate limits for local testing
   - Production: High-performance distributed rate limiting
   - Testing: Permissive limits for test suites

2. **Testability** with mock components
   - Inject custom rate limiters for testing
   - Mock shutdown coordinators
   - Test-specific configurations

3. **Flexibility** to swap rate limiters
   - Choose between Distributed and Advanced managers
   - Configure custom limits per environment
   - Runtime component injection

4. **Validation** before construction
   - Pre-build config validation
   - Clear error messages for missing components
   - Fail fast on invalid configurations

5. **Better developer experience**
   - Fluent API for service construction
   - Self-documenting builder methods
   - Status checking with builder.status()
