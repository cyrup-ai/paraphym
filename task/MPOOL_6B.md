# MPOOL_6B: Implement Graceful Shutdown

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

Implement graceful shutdown mechanism that drains in-flight requests for up to 5 seconds before forcing exit. Integrates with Ctrl+C signal handling.

## CONTEXT

Shutdown behavior (Scenario 7):
1. Signal received (SIGINT/SIGTERM) → set shutdown flag
2. Stop accepting new requests → return PoolError::ShuttingDown
3. Drain period (5 seconds) → workers finish in-flight requests
4. Timeout reached → force exit

## SUBTASK 1: Implement Shutdown Module

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/shutdown.rs`

**Implementation**:
```rust
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Global shutdown flag (accessed by all pools)
static SHUTDOWN_FLAG: AtomicBool = AtomicBool::new(false);

/// Begin shutdown sequence for all pools
///
/// 1. Set shutdown flag (pools reject new requests)
/// 2. Wait for drain timeout (workers finish in-flight)
/// 3. Signal all workers to shutdown
/// 4. Log completion stats
pub fn begin_shutdown(timeout_secs: u64) {
    use crate::pool::capabilities::{
        text_embedding_pool,
        text_to_text_pool,
        image_embedding_pool,
        vision_pool,
        text_to_image_pool,
    };

    log::info!("Shutdown signal received, draining pools (timeout: {}s)...", timeout_secs);

    // Set shutdown flags on all pools
    text_embedding_pool().begin_shutdown();
    text_to_text_pool().begin_shutdown();
    image_embedding_pool().begin_shutdown();
    vision_pool().begin_shutdown();
    text_to_image_pool().begin_shutdown();

    // Wait for drain timeout
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(timeout_secs);

    while start.elapsed() < timeout {
        // Check if all pools drained
        if all_pools_drained() {
            let elapsed = start.elapsed().as_secs_f64();
            log::info!("Graceful shutdown complete ({:.2}s, all queues drained)", elapsed);
            return;
        }

        // Sleep briefly before checking again
        std::thread::sleep(Duration::from_millis(100));
    }

    // Timeout reached - force shutdown
    log::warn!(
        "Shutdown timeout reached ({}s), forcing exit with {} in-flight requests",
        timeout_secs,
        count_in_flight_requests()
    );
}

/// Check if all pools have drained (no in-flight requests)
fn all_pools_drained() -> bool {
    use crate::pool::capabilities::{
        text_embedding_pool,
        text_to_text_pool,
        image_embedding_pool,
        vision_pool,
        text_to_image_pool,
    };

    // Check each pool's workers for pending requests
    let text_embedding_pending = count_pool_pending(text_embedding_pool());
    let text_to_text_pending = count_pool_pending(text_to_text_pool());
    let image_embedding_pending = count_pool_pending(image_embedding_pool());
    let vision_pending = count_pool_pending(vision_pool());
    let text_to_image_pending = count_pool_pending(text_to_image_pool());

    text_embedding_pending == 0
        && text_to_text_pending == 0
        && image_embedding_pending == 0
        && vision_pending == 0
        && text_to_image_pending == 0
}

/// Count pending requests in a pool
fn count_pool_pending<T>(pool: &Pool<T>) -> usize {
    let mut total = 0;
    for entry in pool.workers.iter() {
        for worker in entry.value().iter() {
            total += worker.pending_requests.load(Ordering::Acquire);
        }
    }
    total
}

/// Count total in-flight requests across all pools
fn count_in_flight_requests() -> usize {
    use crate::pool::capabilities::{
        text_embedding_pool,
        text_to_text_pool,
        image_embedding_pool,
        vision_pool,
        text_to_image_pool,
    };

    count_pool_pending(text_embedding_pool())
        + count_pool_pending(text_to_text_pool())
        + count_pool_pending(image_embedding_pool())
        + count_pool_pending(vision_pool())
        + count_pool_pending(text_to_image_pool())
}
```

**Why**: Centralized shutdown logic for all 5 pools (Scenario 7).

## SUBTASK 2: Integrate Shutdown Hook in CLI Runner

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/cli/runner.rs`

**Find**: `pub async fn run(&mut self) -> Result<()>` method

**Add shutdown hook before main loop**:
```rust
// Setup Ctrl+C handler
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

let shutdown_flag = Arc::new(AtomicBool::new(false));
let shutdown_flag_clone = shutdown_flag.clone();

ctrlc::set_handler(move || {
    if shutdown_flag_clone.load(Ordering::Acquire) {
        // Second Ctrl+C - force exit immediately
        eprintln!("Force exit");
        std::process::exit(1);
    } else {
        // First Ctrl+C - graceful shutdown
        shutdown_flag_clone.store(true, Ordering::Release);
        eprintln!("\nShutdown signal received, draining pools...");

        // Begin graceful shutdown
        crate::pool::shutdown::begin_shutdown(5); // 5 second timeout

        std::process::exit(0);
    }
}).expect("Error setting Ctrl-C handler");
```

**Why**: CLI integrates shutdown handling (Scenario 7).

## SUBTASK 3: Add Dependency

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/Cargo.toml`

**Add to dependencies**:
```toml
[dependencies]
# ... existing dependencies ...
ctrlc = "3.4"
```

**Why**: Need ctrlc crate for signal handling.

## SUBTASK 4: Wire Up Module Exports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/pool/mod.rs`

```rust
pub mod core;
pub mod capabilities;
pub mod maintenance;
pub mod shutdown;  // NEW

pub use core::{Pool, PoolConfig, PoolError};
pub use capabilities::{
    text_embedding_pool,
    text_to_text_pool,
    image_embedding_pool,
    vision_pool,
    text_to_image_pool,
};
pub use maintenance::start_maintenance_thread;
pub use shutdown::begin_shutdown;  // NEW
```

## SUBTASK 5: Update Pool API to Check Shutdown Flag

**Files**: All capability pool implementations (already done in MPOOL_3A/B/C)

**Pattern** (already implemented):
```rust
pub fn embed_text(&self, registry_key: &str, text: &str, task: Option<String>)
    -> Result<Vec<f32>, PoolError>
{
    // Check shutdown flag
    if self.is_shutting_down() {
        return Err(PoolError::ShuttingDown("Pool shutting down".to_string()));
    }

    // ... process request
}
```

**Why**: Pools reject new requests during shutdown (Scenario 7 behavior 2).

## DEFINITION OF DONE

- [ ] `shutdown.rs` file created
- [ ] `begin_shutdown()` function implemented with timeout logic
- [ ] `all_pools_drained()` helper implemented
- [ ] `count_pool_pending()` helper implemented
- [ ] `count_in_flight_requests()` helper implemented
- [ ] Ctrl+C handler integrated in cli/runner.rs
- [ ] `ctrlc` dependency added to Cargo.toml
- [ ] Module exports configured
- [ ] All pool methods check shutdown flag (already done in MPOOL_3A/B/C)
- [ ] Code compiles with `cargo check`

## DEPENDENCIES

**Requires**: MPOOL_2A (Pool<T>), MPOOL_3A/B/C (pool accessors)

**Blocks**: None (graceful shutdown is enhancement)

**Parallel with**: MPOOL_6A (maintenance thread)

## RESEARCH NOTES

**Shutdown Behavior** (from Scenario 7):
```
1. Shutdown Signal Received (SIGINT, SIGTERM):
   - Set shutdown flag (AtomicBool)
   - Stop accepting new requests to all 5 pools
   - Start drain timer (default: 5 seconds)

2. Drain Period (0 to N seconds):
   - In-flight requests: Workers finish processing
   - Queued requests: Workers continue pulling and processing
   - New requests: Return PoolError::ShuttingDown
   - Maintenance thread: Stops spawning new workers

3. Timeout Reached (after N seconds):
   - Force exit: Drop all remaining queued requests
   - Worker threads: Send shutdown signal via channel
   - In-flight requests: Workers interrupted
   - Main thread: Exits with status code 0

4. Clean Exit Before Timeout:
   - If all queues empty and all workers idle: exit immediately
   - Log: "Graceful shutdown complete (X.Xs, Y requests drained)"
```

**Configuration**:
```rust
// In PoolConfig
pub struct PoolConfig {
    pub shutdown_timeout_secs: u64,  // Default: 5
    // ... other fields
}
```

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **TIMEOUT PREVENTS HANG**: Shutdown must complete within timeout (default 5s).
- **COORDINATED**: Single shutdown sequence for all 5 pools.

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
