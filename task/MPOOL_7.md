# MPOOL_7: Final Integration and Initialization

**PREFIX**: MPOOL (Model Pool)

## CRITICAL DESIGN PRINCIPLE

**pool.rs CONTAINS ZERO MODEL-SPECIFIC LOGIC**

This cannot be emphasized enough:

- ❌ **NO** GteQwen-specific code in pool.rs
- ❌ **NO** JinaBert-specific code in pool.rs  
- ❌ **NO** NvEmbed-specific code in pool.rs
- ❌ **NO** Phi4-specific code in pool.rs
- ❌ **NO** KimiK2-specific code in pool.rs
- ❌ **NO** Qwen3Coder-specific code in pool.rs
- ❌ **NO** ClipVision-specific code in pool.rs
- ❌ **NO** LLaVA-specific code in pool.rs
- ❌ **NO** FLUX-specific code in pool.rs
- ❌ **NO** StableDiffusion-specific code in pool.rs
- ❌ **NO** knowledge of any specific model's existence

## OBJECTIVE

Wire up final pieces to ensure pool initialization happens at correct time, module exports are clean, and all components integrate properly.

## CONTEXT

Final integration checklist:
- Ensure maintenance thread starts when first pool accessed
- Verify all module exports are correct
- Add pool initialization call in CLI runner
- Document pool usage for library users
- Verify compilation with all pieces integrated

## SUBTASK 1: Add Pool Initialization to CLI Runner

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/runner.rs`

**Find**: `pub async fn run(&mut self) -> Result<()>` method

**Add initialization at start of method** (before any pool usage):
```rust
pub async fn run(&mut self) -> Result<()> {
    // Initialize pool maintenance thread (lazy init)
    crate::pool::init_maintenance();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // ... rest of method
}
```

**Why**: Ensures maintenance thread starts when CLI begins.

## SUBTASK 2: Verify All Module Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`

**Ensure complete**:
```rust
//! Worker pool infrastructure for keeping models loaded in memory
//!
//! # Architecture
//!
//! 5 global pool instances (one per capability trait):
//! - TEXT_EMBEDDING_POOL: TextEmbeddingCapable models
//! - TEXT_TO_TEXT_POOL: TextToTextCapable models
//! - IMAGE_EMBEDDING_POOL: ImageEmbeddingCapable models
//! - VISION_POOL: VisionCapable models
//! - TEXT_TO_IMAGE_POOL: TextToImageCapable models
//!
//! # Usage
//!
//! Pool integration is transparent. Users call registry methods:
//! ```rust
//! let model = registry::get<TextEmbeddingModel>("dunzhang/stella_en_1.5B_v5")?;
//! let embedding = model.embed("hello world", None)?;  // Pool intercepts
//! ```
//!
//! # Initialization
//!
//! Call `init_maintenance()` once at application startup:
//! ```rust
//! crate::pool::init_maintenance();
//! ```
//!
//! # Architecture Details
//!
//! See task/MODEL_POOL.md for complete design documentation.

pub mod core;
pub mod capabilities;
pub mod maintenance;
pub mod shutdown;

pub use core::{Pool, PoolConfig, PoolError, WorkerHandle};
pub use capabilities::{
    text_embedding_pool,
    text_to_text_pool,
    image_embedding_pool,
    vision_pool,
    text_to_image_pool,
};
pub use maintenance::start_maintenance_thread;
pub use shutdown::begin_shutdown;

use once_cell::sync::Lazy;

/// Global maintenance thread handle
static MAINTENANCE_THREAD: Lazy<std::thread::JoinHandle<()>> = Lazy::new(|| {
    start_maintenance_thread()
});

/// Initialize pool maintenance thread
///
/// Call once at application startup. Subsequent calls are no-ops.
/// Thread starts immediately and runs until shutdown.
pub fn init_maintenance() {
    // Force lazy initialization
    let _ = &*MAINTENANCE_THREAD;
    log::info!("Pool maintenance thread initialized");
}
```

**Why**: Complete public API and documentation.

## SUBTASK 3: Add Pool Module to Main Library Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/lib.rs`

**Verify pool is exported**:
```rust
pub mod builders;
pub mod capability;
pub mod cli;
pub mod core;
pub mod domain;
pub mod memory;
pub mod pool;       // Ensure this exists
pub mod util;
pub mod workflow;

// Re-export key types
pub use pool::{Pool, PoolError, init_maintenance};  // NEW
```

**Why**: Makes pool accessible to library users.

## SUBTASK 4: Add Logging Configuration Note

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`

**Add to module docs**:
```rust
//! # Logging
//!
//! Pool logs important events:
//! - Worker spawn: `log::info!("Spawned worker X for Y")`
//! - Worker eviction: `log::info!("Evicted worker X (idle cooldown)")`
//! - Shutdown: `log::info!("Graceful shutdown complete")`
//! - Errors: `log::error!("Worker loading failed: {}")`
//!
//! Enable pool logging:
//! ```bash
//! RUST_LOG=paraphym_candle::pool=info cargo run
//! ```
```

**Why**: Users know how to enable pool logging for debugging.

## SUBTASK 5: Verify Compilation

**Commands to run**:
```bash
# Check compilation
cargo check

# Check specific packages
cargo check -p paraphym_candle

# Check with all features
cargo check --all-features

# Check warnings
cargo clippy --all-targets -- -D warnings
```

**Expected result**: Clean compilation with no errors, minimal warnings.

**Why**: Ensures all pieces integrate correctly.

## SUBTASK 6: Document Known Limitations

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`

**Add to module docs**:
```rust
//! # Known Limitations (Phase 1)
//!
//! Current implementation:
//! - TextEmbedding models use pool (5 models)
//! - TextToText models call directly (already have state)
//! - ImageEmbedding, Vision, TextToImage call directly (not prioritized)
//!
//! Future enhancements:
//! - Pool integration for TextToText models
//! - Pool integration for Vision models
//! - Metrics dashboard (request counts, latencies, queue depths)
//! - Dynamic timeout adjustment based on load
//! - Worker health monitoring
//!
//! See task/MODEL_POOL.md Implementation Phases for roadmap.
```

**Why**: Clear expectations for users and maintainers.

## SUBTASK 7: Verify Memory Field Dependencies

**Ensure all MODEL_INFO declarations have est_memory_allocation_mb**:
```bash
# Search for MODEL_INFO without est_memory_allocation_mb
rg "CandleModelInfo \{" packages/candle/src/capability/ | \
  xargs -I {} grep -L "est_memory_allocation_mb" {}
```

**Expected result**: Empty (all MODEL_INFO declarations have the field)

**Why**: Compilation will fail if any MODEL_INFO missing required field.

## DEFINITION OF DONE

- [ ] `init_maintenance()` called in cli/runner.rs
- [ ] Pool module exports verified and complete
- [ ] Module documentation complete with usage examples
- [ ] Pool exported from main lib.rs
- [ ] Logging configuration documented
- [ ] Known limitations documented
- [ ] Compilation verified with `cargo check`
- [ ] All MODEL_INFO declarations have est_memory_allocation_mb field
- [ ] No compilation errors
- [ ] Minimal warnings (clippy clean)

## DEPENDENCIES

**Requires**: All previous MPOOL tasks (1, 2A, 2B, 3A, 3B, 3C, 4, 5, 6A, 6B)

**Blocks**: None (final integration step)

## RESEARCH NOTES

**Initialization Order**:
1. User starts CLI: `cargo run --bin candle-chat`
2. CLI runner calls `pool::init_maintenance()`
3. Maintenance thread starts (lazy)
4. User makes first request
5. Registry enum dispatch checks pool
6. Pool spawns workers (lazy, 0→2)
7. Workers process requests
8. Maintenance thread evicts idle workers

**Module Structure** (final):
```
pool/
  mod.rs                    - Public API, exports, init_maintenance()
  core/
    mod.rs                  - Core exports
    pool.rs                 - Generic Pool<T> implementation
    worker.rs               - Generic worker helpers
    types.rs                - WorkerHandle, PoolConfig, PoolMetrics
    error.rs                - PoolError enum
  capabilities/
    mod.rs                  - Capability exports
    text_embedding.rs       - TextEmbedding pool
    text_to_text.rs         - TextToText pool
    image_embedding.rs      - ImageEmbedding pool
    vision.rs               - Vision pool
    text_to_image.rs        - TextToImage pool
  maintenance.rs            - Background eviction thread
  shutdown.rs               - Graceful shutdown logic
```

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **CLEAN COMPILATION**: Must compile without errors, minimal warnings.
- **DOCUMENTATION COMPLETE**: All public APIs documented with examples.

   =>  

# CODE GENERATION GUIDELINES

## No Potential for Improvement

Write code with these goals in mind: 

  - zero allocation
  - blazing-fast
  - no unsafe
  - no unchecked 
  - *no locking*
  - elegant ergonomic code

DO NOT WRITE TESTS IN THE SAME FILE
ANOTHER AGENT will write those in ./tests/ (sister to src)


Do not include areas for potential future improvement. If you identify them, think through them with ultrathink, step by step sequential reasoning and roll them into your source code. Do this iteratively and recursively until there is zero need for a "future enhancements" section.

think sequentially. step by step. ULTRATHINK.

Check all your work twice to ensure no symbol, method, trait bounds or other detail is missed, misaligned or omitted.

Review the architecture and requirements ... Focus keenly on the USER OBJECTIVE. it is your "guiding light" and ultimate "source of truth". Ensure all delivered items incrementally lead to this end state and ALL "the pieces fit.

Check all of your work a third time. Think sequentially, step by step. ULTRATHINK. Focus on performance. Are you using channels properly. are you optimizing allocations and inlining all the happy paths where it wi matter. Are all errors handled fully and semantically? think sequentially. step by step. ULTRATHINK.

Check all of your work a fourth time. think sequentially. step by step. ULTRATHINK. "Have I provided ALL the code, full and complete with all details handled and no "future enhancements", todos, "in a real situation", "for now", "in production". All such work will be rejected. Revise it recursively until it is perfected. 

Check all your work a fifth time. Are all the third party libraries using the very latest api signatures and "best in class idioms"? Revise your work recursively until all such issues are handled. Be a software artisan. Complex, feature rich, elegant, ergonomic source code is your requirement.

## All Issues Handle. NOTHING simplified. NOTHING stubbed. NOTHING "miminal"

Do not include areas for potential future improvement. If you identify them, think through them with ultrathink, step by step sequential reasoning and roll them into your source code. Do this interactively until there is zero need for a "future enhancements" section.

=========================================

- express all source code fully
- certify that the code is complete and every potential optimization is included.


==== MANIFEST WITH THESE CONSTRAINTS =====

## No Potential for Improvement

Do not include areas for potential future improvement. If you identify them, think through them with ultrathink, step by step sequential reasoning and roll them into your source code. Do this iteratively and recursively until there is zero need for a "future enhancements" section.

ADDITIONAL CONSTRAINTS:

- never use unwrap() (period!)
- never use expect() (in src/* or in examples)
- DO USE expect() in ./tests/*
- DO NOT use unwrap in ./tests/*

## MAKE ONLY NECESSARY CHANGES

- Focus on the User's objective
- Be useful, not thorough
- Make surgical, targeted changes vs sweeping changes

## DO NOT STUB CODE TO COME BACK LATER

- You will forget! 
- Write the full and correct code right now!
- if you don't know how and need to research, pause and research

## CLARIFICATIONS 

I DO NOT WANT YOU TO REWRITE WORKING CODE UNLESS REQUESTED (Bad)
I DO WANT YOU TO WRITE ALL NEW AND MODIFIED CODE WITH THESE CONSTRAINTS 
