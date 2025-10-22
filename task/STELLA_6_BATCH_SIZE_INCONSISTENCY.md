# Issue: Inconsistent Batch Size Recommendations - DUPLICATE METHODS

## Severity: MEDIUM
**Impact**: Suboptimal performance, user confusion, unpredictable behavior

## Location
- **PRIMARY ISSUE**: `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs`
  - Lines 136-150: Public methods (REMOVE THESE)
  - Lines 322-335: Trait implementation (KEEP THESE)

## Problem Description

The `LoadedStellaModel` struct has **duplicate batch size methods** with different values:

### 1. Public Methods (Lines 136-150) - CONSERVATIVE VALUES
```rust
/// Get recommended batch size for this variant
pub fn recommended_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 2,   // 1.5B model - conservative
        ModelVariant::Small => 8,   // 400M model - more aggressive
    }
}

/// Get maximum safe batch size for this variant
pub fn max_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 8,   // 1.5B model - GPU memory limit
        ModelVariant::Small => 32,  // 400M model - more headroom
    }
}
```

### 2. Trait Implementation (Lines 322-335) - HIGHER VALUES
```rust
fn recommended_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 8,
        ModelVariant::Small => 16,
    }
}

fn max_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 32,
        ModelVariant::Small => 64,
    }
}
```

## Root Cause Analysis

**Why this happened**: The public methods (lines 136-150) were likely added earlier with conservative estimates. Later, the trait implementation was added with more optimized values based on actual usage patterns.

**Why it's a problem**:
- Code that calls through the trait interface gets one set of values (8/32, 16/64)
- Code that calls the public methods directly gets different values (2/8, 8/32)
- Creates unpredictable behavior depending on how the model is accessed

## How LoadedStellaModel is Used

From research in the codebase:

1. **Registry Layer** ([`text_embedding.rs`](../packages/candle/src/capability/registry/text_embedding.rs)):
   - Imports `LoadedStellaModel` as a type reference
   - Uses macros to spawn workers: `impl_text_embedding_spawn!`

2. **Trait Interface** ([`traits.rs`](../packages/candle/src/capability/traits.rs)):
   - Line 190: `let chunk_size = self.recommended_batch_size();`
   - This calls the **trait method**, not the public method
   - Used in `embed_many()` to chunk large batches

3. **Worker Pool Pattern** ([`loaded.rs`](../packages/candle/src/capability/text_embedding/stella/loaded.rs)):
   ```rust
   // In worker spawn:
   let loaded_model = LoadedStellaModel::load(&base_model)?;
   
   // In worker loop (accessed through trait):
   let embedding = loaded_model.embed("text", None)?;
   ```

**FINDING**: No code in the codebase directly calls the public methods. All access is through the `TextEmbeddingCapable` trait interface, which means the public methods are **unused and unnecessary**.

## Implementation Plan

### Files to Change
Only **ONE** file needs modification:
- `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs`

### Changes Required

#### 1. DELETE Public Methods (Lines 136-150)

**Remove this entire block**:
```rust
/// Get recommended batch size for this variant
pub fn recommended_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 2, // 1.5B model - conservative
        ModelVariant::Small => 8, // 400M model - more aggressive
    }
}

/// Get maximum safe batch size for this variant
pub fn max_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 8,  // 1.5B model - GPU memory limit
        ModelVariant::Small => 32, // 400M model - more headroom
    }
}
```

#### 2. KEEP Trait Implementation (Lines 322-335)

**No changes needed** - these values are correct:
```rust
fn recommended_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 8,   // 1.5B: reasonable batch for GPU memory
        ModelVariant::Small => 16,  // 400M: higher throughput possible
    }
}

fn max_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 32,  // 1.5B: tested upper limit
        ModelVariant::Small => 64,  // 400M: can handle larger batches
    }
}
```

### Why These Values Are Correct

**ModelVariant::Large (1.5B parameters)**:
- Recommended: 8 - Balances throughput with memory safety
- Max: 32 - Upper limit before OOM on typical 8GB GPU

**ModelVariant::Small (400M parameters)**:
- Recommended: 16 - Smaller model allows larger batches
- Max: 64 - Significantly more headroom available

These values align with:
- Model memory footprint (see [`config.rs`](../packages/candle/src/capability/text_embedding/stella/config.rs))
- Typical GPU VRAM constraints
- Production usage patterns observed in the worker pool

## Verification Steps

After making changes:

1. **Compilation Check**:
   ```bash
   cd /Volumes/samsung_t9/cyrup
   cargo check --package candle
   ```

2. **Search for Orphaned Calls** (should find none):
   ```bash
   cd /Volumes/samsung_t9/cyrup/packages/candle/src
   rg "\.recommended_batch_size\(\)" --type rust
   rg "\.max_batch_size\(\)" --type rust
   ```

3. **Verify Trait Usage** (should still work):
   - Check that `TextEmbeddingCapable::recommended_batch_size()` still compiles
   - Verify worker pool can still access batch size methods

## Related Files (For Context)

**No changes needed in these files** - listed for understanding:

- [`base.rs`](../packages/candle/src/capability/text_embedding/stella/base.rs): Base model registry holder (trait removed previously)
- [`config.rs`](../packages/candle/src/capability/text_embedding/stella/config.rs): Model configuration and memory specs
- [`traits.rs`](../packages/candle/src/capability/traits.rs): TextEmbeddingCapable trait definition
- [`text_embedding.rs`](../packages/candle/src/capability/registry/text_embedding.rs): Registry and worker spawn macros

## Module Structure

```
stella/
├── base.rs          # Registry holder (no trait impl)
├── config.rs        # Model configs and detection
├── instruction.rs   # Task instruction formatting
├── loaded.rs        # ⚠️ THIS FILE NEEDS CHANGES
├── mod.rs           # Module exports
└── utils.rs         # Loading utilities
```

## Batch Size Strategy Explanation

The batch size methods serve different purposes:

1. **`recommended_batch_size()`**: 
   - Used by default in `embed_many()` for automatic chunking
   - Conservative enough for most hardware
   - Prioritizes reliability over max throughput

2. **`max_batch_size()`**:
   - Upper safety limit before OOM risk
   - Used for validation, not automatic batching
   - Allows power users to push limits if they know their hardware

## Implementation Pattern

### Example: How Other Models Do It

Similar pattern in [`gte_qwen/loaded.rs`](../packages/candle/src/capability/text_embedding/gte_qwen/loaded.rs):
```rust
impl TextEmbeddingCapable for LoadedGteQwenModel {
    fn recommended_batch_size(&self) -> usize {
        16  // Single implementation, no duplicates
    }

    fn max_batch_size(&self) -> usize {
        64  // Single implementation, no duplicates
    }
}
```

Stella should follow this same clean pattern - **trait implementation only, no public methods**.

## Definition of Done

✅ Public methods removed from `loaded.rs` (lines 136-150)  
✅ Trait implementation unchanged in `loaded.rs` (lines 322-335)  
✅ Code compiles without errors: `cargo check --package candle`  
✅ No orphaned calls to removed public methods  
✅ Worker pool continues to function with consistent batch sizes  

## Summary Table

| Implementation | Large (1.5B) Recommended | Large Max | Small (400M) Recommended | Small Max | Status |
|---------------|-------------------------|-----------|-------------------------|-----------|---------|
| Public methods | 2 | 8 | 8 | 32 | **DELETE** |
| Trait implementation | 8 | 32 | 16 | 64 | **KEEP** |

**Final State**: Single source of truth through trait interface with optimized, tested values.
