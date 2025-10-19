# Task 012: Remove Dangerous Dead Code - CandleQwen3QuantizedModel::prompt()

## Priority: HIGH (blocked by Task 016)
## Status: NOT STARTED
## Created: 2025-10-19
## Updated: 2025-10-19 (Augmented with implementation details)

---

## Executive Summary

**Objective**: Delete the dangerous `impl TextToTextCapable for CandleQwen3QuantizedModel` implementation that reloads the model on every request.

**Impact**: Removes 390 lines of dead code that creates a 10,000x performance trap.

**Scope**: Single file change - delete one implementation block.

---

## Problem Statement

The `CandleQwen3QuantizedModel::prompt()` method (lines 108-498 in `qwen3_quantized.rs`) loads the model from scratch on every call:

```rust
// ❌ DANGEROUS - Loads model every request (5-10 seconds)
impl TextToTextCapable for CandleQwen3QuantizedModel {
    fn prompt(...) {
        // Downloads GGUF file
        let gguf_path = model.huggingface_file(...).await?;
        
        // Downloads tokenizer  
        let tokenizer_path = model.huggingface_file(...).await?;
        
        // Parses 1.7GB GGUF file
        let content = gguf_file::Content::read(&mut file)?;
        
        // Creates model from scratch
        let model = Qwen3Model::from_gguf(content, &mut file, &device)?;
        
        // Finally generates...
    }
}
```

While the registry correctly uses `LoadedQwen3QuantizedModel` instead, this method is:

1. **Dead code** in production (never called by registry)
2. **Dangerous** if anyone calls it directly (10,000x slower)
3. **Publicly accessible** through trait implementation

---

## Verification: Registry Uses Correct Path

**Source**: [`packages/candle/src/capability/registry/text_to_text.rs:87`](../packages/candle/src/capability/registry/text_to_text.rs)

```rust
impl_text_to_text_spawn!(
    spawn_stream_qwen3_quantized, 
    crate::capability::text_to_text::qwen3_quantized::CandleQwen3QuantizedModel, 
    LoadedQwen3QuantizedModel  // ✅ CORRECT - uses cached model
);
```

The macro expands to call `LoadedQwen3QuantizedModel::load()` once per worker:

```rust
<LoadedQwen3QuantizedModel>::load(&m_clone)
    .await
    .map_err(|e| PoolError::SpawnFailed(e.to_string()))
```

**Flow**:
1. Registry calls `LoadedQwen3QuantizedModel::load()` once per worker
2. Workers reuse loaded model for all requests  
3. `LoadedQwen3QuantizedModel::prompt()` uses cached resources (fast)

---

## Systemic Pattern Across All Models

**Research Finding**: This is NOT unique to Qwen3. All three text-to-text models have this pattern:

**File**: [`packages/candle/src/capability/text_to_text/`](../packages/candle/src/capability/text_to_text/)

```bash
# KimiK2
kimi_k2.rs:83:    impl TextToTextCapable for CandleKimiK2Model          # SLOW
kimi_k2.rs:532:   impl TextToTextCapable for LoadedKimiK2Model          # FAST

# Phi4Reasoning  
phi4_reasoning.rs:112:  impl TextToTextCapable for CandlePhi4ReasoningModel    # SLOW
phi4_reasoning.rs:441:  impl TextToTextCapable for LoadedPhi4ReasoningModel    # FAST

# Qwen3Quantized
qwen3_quantized.rs:108:  impl TextToTextCapable for CandleQwen3QuantizedModel   # SLOW ← THIS TASK
qwen3_quantized.rs:627:  impl TextToTextCapable for LoadedQwen3QuantizedModel   # FAST
```

**Implication**: After this task proves the pattern, consider applying to KimiK2 and Phi4 (see Task 017 for full audit).

---

## The Performance Trap

Anyone can accidentally use the slow path:

```rust
use paraphym_candle::capability::text_to_text::CandleQwen3QuantizedModel;
use paraphym_candle::capability::traits::TextToTextCapable;

// ❌ CATASTROPHICALLY SLOW - but compiles!
let model = CandleQwen3QuantizedModel::new()?;
model.prompt(prompt, params);  // 5-10 seconds per request
```

This is **10,000x slower** than the registry path:

| Path | Load Time | Per Request |
|------|-----------|-------------|
| Base struct `prompt()` | 5-10 sec | Downloads + parses GGUF every time |
| Loaded struct `prompt()` | <1ms | Uses cached model in memory |

---

## Implementation: What to Change

### File Location

**Path**: [`packages/candle/src/capability/text_to_text/qwen3_quantized.rs`](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs)

### Exact Change Required

**DELETE** lines 108-498 (entire `impl TextToTextCapable for CandleQwen3QuantizedModel` block):

```rust
// DELETE FROM LINE 108:
impl crate::capability::traits::TextToTextCapable for CandleQwen3QuantizedModel {
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>> {
        // ... 380 lines of dangerous code ...
    }
}
// DELETE TO LINE 498 (closing brace)
```

**KEEP** the `CandleModel` impl (lines 499-505):

```rust
// KEEP THIS - provides model metadata
impl CandleModel for CandleQwen3QuantizedModel {
    #[inline]
    fn info(&self) -> &'static CandleModelInfo {
        &QWEN3_QUANTIZED_MODEL_INFO
    }
}
```

**KEEP** all `LoadedQwen3QuantizedModel` code (lines 506+):

```rust
// KEEP THIS - the correct implementation
impl crate::capability::traits::TextToTextCapable for LoadedQwen3QuantizedModel {
    fn prompt(...) {
        // Uses cached model - FAST
    }
}
```

---

## Why This is Safe

### 1. Base Struct is Just a Factory

**Source**: [`packages/candle/src/capability/text_to_text/qwen3_quantized.rs:38-41`](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs)

```rust
#[derive(Debug, Clone)]
pub struct CandleQwen3QuantizedModel {
    /// Engine for orchestration and stream conversion
    engine: Arc<Engine>,  // ← Just configuration, no model!
}
```

The base struct is lightweight - it only holds configuration. It's a **factory** for creating loaded models.

### 2. Registry Pattern Enforces Correct Usage

**Source**: [`packages/candle/src/capability/registry/text_to_text.rs:39-76`](../packages/candle/src/capability/registry/text_to_text.rs)

```rust
macro_rules! impl_text_to_text_spawn {
    ($fn_name:ident, $model_ty:ty, $loaded_ty:ty) => {
        fn $fn_name(...) -> ... {
            // Spawns workers with LoadedModel, not base model
            pool.spawn_text_to_text_worker(
                registry_key,
                move || async move {
                    <$loaded_ty>::load(&m_clone)  // ← Uses Loaded variant
                        .await
                },
                per_worker_mb,
                allocation_guard,
            )
        }
    };
}
```

The registry **always** calls `LoadedQwen3QuantizedModel::load()`, never the base struct's prompt.

### 3. No Breaking Changes

After deletion:
- ✅ Registry still works (uses LoadedQwen3QuantizedModel)
- ✅ Workers still reuse cached models
- ✅ Public API unchanged (base struct still exists)
- ✅ The slow path becomes unreachable
- ✅ Compilation succeeds

---

## Performance Comparison

### Base Implementation (TO DELETE)

**Location**: Lines 108-498

**Overhead per request**:
```
Network I/O:     1-3 seconds   (download GGUF + tokenizer)
Disk I/O:        0.5-1 second  (read 1.7GB file)
GGUF parsing:    1-2 seconds   (parse file structure)
Model init:      2-3 seconds   (allocate + initialize weights)
Device setup:    0.5-1 second  (Metal/CUDA initialization)
─────────────────────────────────────────────────────────
TOTAL:           5-10 seconds  (BEFORE generation starts)
```

### Loaded Implementation (TO KEEP)

**Location**: Lines 627+

**Overhead per request**:
```
Arc clone:       <1μs          (pointer copy)
Mutex acquire:   <100μs        (uncontended lock)
─────────────────────────────────────────────────────────
TOTAL:           <1ms          (generation starts immediately)
```

**Speedup**: 5,000x - 10,000x faster startup

---

## Code Architecture Context

### TextToTextCapable Trait

**Source**: [`packages/candle/src/capability/traits.rs:48-59`](../packages/candle/src/capability/traits.rs)

```rust
pub trait TextToTextCapable: CandleModel {
    /// Generate completion from prompt - the actual work method
    fn prompt(
        &self,
        prompt: CandlePrompt,
        params: &CandleCompletionParams,
    ) -> Pin<Box<dyn Stream<Item = CandleCompletionChunk> + Send>>;
    
    // ... other methods ...
}
```

Both base and loaded models implement this trait, but **only the loaded version should**.

### Model Lifecycle Pattern

```
┌─────────────────────────────────────────────────────────┐
│ User Code                                               │
│ ├─ CandleFluentAi::agent_role("qwen-3")               │
│ └─ .with_completion_provider_auto()                    │
└────────────────────┬────────────────────────────────────┘
                     │
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Registry (capability/registry/text_to_text.rs)         │
│ ├─ TextToTextModel::Qwen3Quantized(Arc<Base>)         │
│ └─ spawn_stream_qwen3_quantized()                      │
└────────────────────┬────────────────────────────────────┘
                     │
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Pool Worker (spawn once)                               │
│ └─ LoadedQwen3QuantizedModel::load(&base)             │
│    ├─ Download files (once)                           │
│    ├─ Parse GGUF (once)                               │
│    ├─ Init model (once)                               │
│    └─ Cache in Arc<Mutex<Model>>                      │
└────────────────────┬────────────────────────────────────┘
                     │
                     ↓
┌─────────────────────────────────────────────────────────┐
│ Request Handling (reuses loaded model)                 │
│ └─ LoadedQwen3QuantizedModel::prompt()  ← FAST        │
│    ├─ Clone Arc (pointer copy)                        │
│    ├─ Lock mutex                                      │
│    └─ Generate with cached model                     │
└─────────────────────────────────────────────────────────┘
```

**Key Point**: The base struct is only used to **create** the loaded struct. It should never generate text itself.

---

## Implementation Steps

### Step 1: Open the File

```bash
# File location
packages/candle/src/capability/text_to_text/qwen3_quantized.rs
```

### Step 2: Locate the Block to Delete

Search for:
```rust
impl crate::capability::traits::TextToTextCapable for CandleQwen3QuantizedModel {
```

This appears at **line 108**.

### Step 3: Find the End of the Block

The closing brace is at **line 498**.

Between these lines is the entire dangerous implementation (~390 lines).

### Step 4: Delete the Block

Delete lines 108-498 inclusive.

### Step 5: Verify Compilation

```bash
cargo check -p paraphym_candle
```

Expected output:
```
    Checking paraphym_candle v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
```

---

## What Remains After Deletion

### 1. Base Struct (lines 38-72)

```rust
/// Lightweight factory for creating loaded models
#[derive(Debug, Clone)]
pub struct CandleQwen3QuantizedModel {
    engine: Arc<Engine>,
}

impl CandleQwen3QuantizedModel {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Creates lightweight factory
    }
}
```

### 2. CandleModel Impl (lines 499-505)

```rust
impl CandleModel for CandleQwen3QuantizedModel {
    fn info(&self) -> &'static CandleModelInfo {
        &QWEN3_QUANTIZED_MODEL_INFO
    }
}
```

### 3. Loaded Struct and Its Impl (lines 506+)

```rust
/// Pre-loaded model with cached resources
pub struct LoadedQwen3QuantizedModel {
    model: Arc<tokio::sync::Mutex<Qwen3Model>>,
    tokenizer: Tokenizer,
    device: Device,
    engine: Arc<Engine>,
    eos_token_id: Option<u32>,
}

impl LoadedQwen3QuantizedModel {
    pub async fn load(base: &CandleQwen3QuantizedModel) -> Result<Self, ...> {
        // Loads model once, caches in Arc<Mutex<>>
    }
}

impl TextToTextCapable for LoadedQwen3QuantizedModel {
    fn prompt(...) -> ... {
        // Uses cached model - FAST PATH
    }
}
```

---

## Definition of Done

**The task is complete when**:

1. ✅ Lines 108-498 deleted from `qwen3_quantized.rs`
2. ✅ `cargo check -p paraphym_candle` passes
3. ✅ Base struct still exists (factory pattern intact)
4. ✅ `CandleModel` impl still exists (provides metadata)
5. ✅ `LoadedQwen3QuantizedModel` and its impl untouched
6. ✅ No compilation errors

**Success criteria**:
- Code compiles without warnings
- Registry continues to work correctly
- The dangerous slow path is now unreachable

---

## Dependencies and Blockers

**BLOCKED BY**: Task 016 - Enforce registry-only access pattern

Task 016 makes model structs `pub(crate)`, which would make this change safer by preventing external code from accessing the base struct. However, this task can proceed independently since:

1. The dangerous code is dead code (registry doesn't use it)
2. Deletion removes the trap regardless of visibility
3. After Task 016, the base struct becomes truly internal

**Execution order**:
- This task can be done now (removes immediate danger)
- Task 016 adds architectural safety (prevents future misuse)

---

## Related Tasks

- **Task 016**: Make model structs `pub(crate)` for architectural enforcement
- **Task 017**: Full audit of all model implementations for similar patterns

---

## Why This Method Exists (Historical Context)

Looking at the pattern across all three models suggests:

1. **Copy-paste pattern**: Original implementation was copied without understanding
2. **Testing artifact**: Base prompt() might have been used for early testing
3. **Incomplete refactoring**: The loaded pattern was added but old code not removed
4. **API evolution**: Trait design evolved but implementations didn't clean up

The correct pattern should be:
- Base struct: Factory only (no TextToTextCapable impl)
- Loaded struct: Worker only (has TextToTextCapable impl)

---

## Future Work

After this task proves the pattern:

1. Apply same fix to `CandleKimiK2Model` (kimi_k2.rs:83-531)
2. Apply same fix to `CandlePhi4ReasoningModel` (phi4_reasoning.rs:112-440)
3. Document the correct pattern for new models

---

## Appendix: Code References

### Files Modified
- [`packages/candle/src/capability/text_to_text/qwen3_quantized.rs`](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs)

### Related Code  
- [`packages/candle/src/capability/registry/text_to_text.rs`](../packages/candle/src/capability/registry/text_to_text.rs) - Registry implementation
- [`packages/candle/src/capability/traits.rs`](../packages/candle/src/capability/traits.rs) - TextToTextCapable trait definition
- [`packages/candle/src/capability/text_to_text/kimi_k2.rs`](../packages/candle/src/capability/text_to_text/kimi_k2.rs) - Same pattern in KimiK2
- [`packages/candle/src/capability/text_to_text/phi4_reasoning.rs`](../packages/candle/src/capability/text_to_text/phi4_reasoning.rs) - Same pattern in Phi4

### Key Patterns
- **Worker pool pattern**: [`packages/candle/src/capability/registry/pool/`](../packages/candle/src/capability/registry/pool/)
- **Model lifecycle**: Factory → Load → Cache → Reuse
- **Trait design**: Separate traits for factory and worker concerns

---

**End of Task Specification**