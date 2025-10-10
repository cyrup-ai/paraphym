# MPOOL_5: Integrate Pool into Registry Enum Dispatch

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

Modify `capability/registry.rs` enum implementations to route trait method calls through pool instead of calling models directly. This makes pool integration transparent to users.

## CONTEXT

User code calls:
```rust
let model = registry::get<TextEmbeddingModel>("dunzhang/stella_en_1.5B_v5")?;
let embedding = model.embed("hello world", None)?;  // POOL INTERCEPTS HERE
```

Registry enum dispatch is where pool slides in. For TextEmbedding match arms, check if workers exist, spawn if needed, route through pool.

## SUBTASK 1: Integrate Pool into TextEmbeddingModel Enum

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs`

**Find**: `impl TextEmbeddingCapable for TextEmbeddingModel` block

**Modify `embed()` method**:
```rust
impl TextEmbeddingCapable for TextEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>)
        -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>
    {
        match self {
            Self::GteQwen(m) => {
                let registry_key = m.info().registry_key;
                let pool = crate::pool::text_embedding_pool();

                // Lazy worker spawn if needed
                if !pool.has_workers(registry_key) {
                    let m_clone = m.clone();
                    pool.spawn_text_embedding_worker(
                        registry_key,
                        move || {
                            use crate::capability::text_embedding::gte_qwen::LoadedGteQwenModel;
                            LoadedGteQwenModel::load(&m_clone)
                                .map_err(|e| crate::pool::PoolError::SpawnFailed(e.to_string()))
                        },
                        m.info().est_memory_allocation_mb,
                    )?;
                }

                // Route through pool
                pool.embed_text(registry_key, text, task)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
            Self::JinaBert(m) => {
                // Same pattern with LoadedJinaBertModel
                todo!()
            }
            Self::NvEmbed(m) => {
                // Same pattern with LoadedNvEmbedModel
                todo!()
            }
            Self::Stella(m) => {
                // Same pattern with LoadedStellaModel
                todo!()
            }
            Self::Bert(m) => {
                // Same pattern with LoadedBertModel
                todo!()
            }
        }
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>)
        -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>>
    {
        match self {
            // Same pattern for all 5 models using pool.batch_embed_text()
            todo!()
        }
    }

    // embedding_dimension, supported_dimensions, etc. remain unchanged (delegate to model)
}
```

**Why**: Pool integration happens at registry dispatch, transparent to users.

## SUBTASK 2: Handle Cold Start (0→2 Workers)

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs`

**Pattern for cold start** (Scenario 5):
```rust
// First request for this model
if !pool.has_workers(registry_key) {
    // Spawn 2 workers if memory allows
    let per_worker_mb = m.info().est_memory_allocation_mb;
    let current_mb = pool.total_memory_mb();

    // Check memory for 2 workers
    if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
        // Spawn 2 workers (asymmetric cold start)
        pool.spawn_text_embedding_worker(registry_key, model_loader.clone(), per_worker_mb)?;
        pool.spawn_text_embedding_worker(registry_key, model_loader, per_worker_mb)?;
    } else {
        // Degraded: spawn only 1 worker
        pool.spawn_text_embedding_worker(registry_key, model_loader, per_worker_mb)?;
    }
}
```

**Why**: Cold start spawns 2 workers immediately per Scenario 5 requirements.

## SUBTASK 3: Add Pool Imports

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs`

**Add to imports**:
```rust
use crate::pool::{text_embedding_pool, PoolError};
use crate::capability::text_embedding::{
    gte_qwen::LoadedGteQwenModel,
    jina_bert::LoadedJinaBertModel,
    nvembed::LoadedNvEmbedModel,
    stella::LoadedStellaModel,
    bert::LoadedBertModel,
};
```

**Why**: Need pool accessor and LoadedModel types.

## SUBTASK 4: Integrate Pool into TextToTextModel Enum (Optional)

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs`

**Note**: TextToText models (Phi4, KimiK2, Qwen3Coder) already store state in their structs. They don't have the "reload per call" problem.

**Decision**: Skip TextToText pool integration for now. TextToText models already keep state and don't reload per call. Pool integration is ONLY needed for TextEmbedding models that currently reload.

**Why**: TextToText models don't need fixing (no performance problem).

## SUBTASK 5: Document Integration Pattern

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry.rs`

**Add module-level comment**:
```rust
//! # Pool Integration
//!
//! TextEmbedding models route through pool for performance:
//! - First request: Spawn 2 workers (0→2 cold start)
//! - Subsequent requests: Route to least-busy worker
//! - Workers keep models loaded in memory (no disk reload)
//!
//! TextToText, Vision, ImageEmbedding, TextToImage models call directly:
//! - These models already store state in structs
//! - No reload-per-call performance problem
//! - Pool integration not needed (yet)
//!
//! # User Transparency
//!
//! Users call:
//! ```rust
//! let model = registry::get<TextEmbeddingModel>("registry_key")?;
//! let embedding = model.embed("text", None)?;  // Pool intercepts here
//! ```
//!
//! Pool integration is invisible - user code unchanged.
```

**Why**: Clear documentation for maintainers.

## DEFINITION OF DONE

- [ ] TextEmbeddingModel enum `embed()` method routes through pool for all 5 models
- [ ] TextEmbeddingModel enum `batch_embed()` method routes through pool for all 5 models
- [ ] Cold start logic spawns 2 workers (or 1 if memory insufficient)
- [ ] Pool accessor imports added
- [ ] LoadedModel imports added
- [ ] Module documentation added
- [ ] Code compiles with `cargo check`
- [ ] User code requires NO changes (transparent integration)

## DEPENDENCIES

**Requires**: MPOOL_3A (text_embedding_pool API), MPOOL_4 (LoadedModel types)

**Blocks**: None (pool is now fully integrated and functional)

## RESEARCH NOTES

**Integration Point** (from MODEL_POOL.md):
```
User → registry::get<TextEmbeddingModel>()
     → TextEmbeddingModel enum
     → match Self::GteQwen(m) {
           // POOL INTEGRATION POINT
           pool.embed_text(...)
       }
     → Worker processes request
     → Result back to user
```

**Cold Start Policy** (Scenario 5):
- First request for model: 0 workers exist
- Spawn 2 workers immediately (asymmetric)
- Memory check: `current + (2 * per_worker) <= 0.80 * total`
- If insufficient memory: degrade to 1 worker

**Lazy Activation** (Scenario 2):
- Registry has 500+ models registered as metadata
- Pool has 0 workers for all models at startup
- Workers only spawn when model actually used
- Unused models stay at 0 workers forever

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **USER TRANSPARENCY**: Zero changes to user code. Pool integration must be invisible.
- **BACKWARD COMPATIBILITY**: Non-pooled models (TextToText, etc.) continue working unchanged.

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
