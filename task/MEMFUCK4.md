# MEMFUCK4: Using eprintln! Instead of Proper Logging Framework

## Problem
Error handling uses `eprintln!` to write directly to stderr instead of using a proper logging framework. This is not production-grade because:
- No log levels (debug, info, warn, error)
- No structured logging with metadata
- No log aggregation support
- No runtime log filtering
- Goes directly to stderr regardless of environment

## Locations
Memory storage errors:
- `/packages/candle/src/domain/agent/chat.rs:470`: `eprintln!("Failed to store user memory: {e:?}");`
- `/packages/candle/src/domain/agent/chat.rs:487`: `eprintln!("Failed to store assistant memory: {e:?}");`
- `/packages/candle/src/domain/agent/chat.rs:506`: `eprintln!("Failed to store context memory: {e:?}");`
- `/packages/candle/src/builders/agent_role.rs:1009`: `eprintln!("Failed to store user memory: {:?}", e);`
- `/packages/candle/src/builders/agent_role.rs:1014`: `eprintln!("Failed to store assistant memory: {:?}", e);`

## Current Broken Pattern
```rust
if let Err(e) = user_pending.await {
    eprintln!("Failed to store user memory: {e:?}");  // NOT PRODUCTION GRADE!
}
```

## What Should Happen
Use the `tracing` or `log` crate:
```rust
use tracing::{error, warn};

if let Err(e) = user_pending.await {
    error!(
        error = ?e,
        memory_type = "user",
        "Failed to store memory to database"
    );
}
```

Or with `log` crate:
```rust
use log::error;

if let Err(e) = user_pending.await {
    error!("Failed to store user memory: {:?}", e);
}
```

## Impact
- No structured logging for production monitoring
- Can't control log levels at runtime
- No integration with log aggregation systems
- Errors go to stderr even in production
- No context or metadata with errors
- Can't filter or route logs by severity

## Recommended Solution
1. Add `tracing` to Cargo.toml dependencies
2. Replace all `eprintln!` with appropriate log levels:
   - `error!` for failures
   - `warn!` for recoverable issues
   - `info!` for important events
   - `debug!` for development info
3. Add structured fields for better observability

## Fix Priority
**MEDIUM** - Critical for production observability and debugging