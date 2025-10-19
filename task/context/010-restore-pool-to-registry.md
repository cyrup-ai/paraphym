# Task: Restore Pool to Registry (Fix Architectural Violation)

## Problem
I incorrectly treated `pool` as a separate system from `registry`, when they are one unified system. The registry IS ALWAYS POOLED.

## What Was Done

### 1. ✅ Moved Directory
```bash
mv src/pool src/capability/registry/pool
```

### 2. ✅ Updated Module Declaration
In `/src/capability/registry/mod.rs`:
```rust
// Pool is an integral part of registry - registry IS ALWAYS POOLED
pub mod pool;
```

### 3. ✅ Updated Public API
In `/src/lib.rs`:
```rust
pub use crate::capability::registry::pool::{Pool, PoolError, init_maintenance};
```

## Remaining Work: Update All Imports

### Files That Need Import Updates

All files that use `use crate::pool::` need to change to `use crate::capability::registry::pool::`:

1. **Pool internal files** (self-referential):
   - `/src/capability/registry/pool/maintenance.rs`
   - `/src/capability/registry/pool/shutdown.rs`
   - `/src/capability/registry/pool/core/pool.rs`
   - `/src/capability/registry/pool/core/orchestrator.rs`
   - `/src/capability/registry/pool/core/worker.rs`
   - `/src/capability/registry/pool/core/types.rs`
   - `/src/capability/registry/pool/capabilities/*.rs` (all 5 files)

2. **Registry files** that use pool:
   - `/src/capability/registry/text_to_text.rs`
   - `/src/capability/registry/text_embedding.rs`
   - `/src/capability/registry/image_embedding.rs`
   - `/src/capability/registry/vision.rs`
   - `/src/capability/registry/text_to_image.rs`

3. **Other files** that reference pool:
   - Check if any examples or tests import pool

## Import Pattern Changes

### Pattern 1: Absolute paths from crate root
```rust
// BEFORE:
use crate::pool::core::{Pool, PoolError};
use crate::pool::capabilities::text_to_text_pool;

// AFTER:
use crate::capability::registry::pool::core::{Pool, PoolError};
use crate::capability::registry::pool::capabilities::text_to_text_pool;
```

### Pattern 2: Within pool module (use super or relative)
```rust
// Option A: Use super (better for pool-internal files)
use super::core::{Pool, PoolError};
use super::capabilities::text_to_text_pool;

// Option B: Use crate path (more explicit)
use crate::capability::registry::pool::core::{Pool, PoolError};
```

### Pattern 3: WorkerState imports
```rust
// BEFORE:
use crate::pool::core::worker_state::WorkerState;
use crate::pool::WorkerState;  // via re-export

// AFTER:
use crate::capability::registry::pool::core::worker_state::WorkerState;
use crate::capability::registry::pool::WorkerState;  // via re-export
```

## Testing After Import Updates

```bash
cd /Volumes/samsung_t9/paraphym/packages/candle

# Check compilation
cargo check

# Run tests
cargo test

# Build examples
cargo build --example fluent_builder --release
```

## Success Criteria

- ✅ All files compile without errors
- ✅ No "unresolved import" errors
- ✅ Pool is recognized as part of registry in docs/architecture
- ✅ Public API works: `paraphym_candle::capability::registry::pool::{...}`
- ✅ All tests pass
- ✅ Examples build and run

## Documentation Updates Needed

After import fixes, update:

1. **Pool module docs** (`/src/capability/registry/pool/mod.rs`):
   - Already correctly describes pool as part of registry
   - Update any outdated examples if needed

2. **Registry docs** (`/src/capability/registry/mod.rs`):
   - Already mentions pool integration
   - Ensure consistency

3. **Root lib.rs docs**:
   - Update any architecture diagrams
   - Clarify registry-pool relationship

## Why This Matters

**Conceptual Clarity:**
- Registry and pool are NOT separate concerns
- You don't "use a registry" and "use a pool"
- You use THE REGISTRY, which happens to be pooled for performance

**Code Organization:**
- Pool implementation details are private to registry
- Users interact with registry API, not pool directly
- Pool is an internal optimization, not a public abstraction

**Maintenance:**
- Changes to pool affect registry behavior
- Can't change one without considering the other
- Single unified system is easier to reason about

This was a fundamental architectural mistake on my part. The registry IS the pool. There is no separation.
