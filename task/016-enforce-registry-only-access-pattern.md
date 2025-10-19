# Task 016: Fix Registry Tests & Documentation (CRITICAL)

## Priority: HIGH
## Status: IN PROGRESS  
## Created: 2025-10-19
## Updated: 2025-10-19 (QA Review - Tests Broken)

---

## Executive Summary

**Core architectural changes are COMPLETE** ✅ but **integration tests are BROKEN** ❌

All model types are now `pub(crate)` and compiler enforces registry-only access. However:
- Integration tests cannot compile (trying to import private types)
- Documentation shows impossible examples
- Compiler warnings for unused imports

**Blocking Issue**: `cargo check --workspace --all-targets` fails with 6 errors

---

## Outstanding Issues

### 🔴 Issue 1: CRITICAL - Broken Integration Tests

**Problem**: `tests/registry_tests.rs` has 6 compilation errors attempting to import now-private model types.

**File**: `tests/registry_tests.rs`

**Errors**:
```
error[E0603]: struct `CandleKimiK2Model` is private
  --> tests/registry_tests.rs:16:52
  
error[E0603]: struct `CandleKimiK2Model` is private
  --> tests/registry_tests.rs:62:52

(4 more similar errors at lines 104, 151, 175, 277)
```

**Root Cause**: Integration tests verify `register_text_to_text()` by creating model instances:
```rust
use paraphym_candle::capability::text_to_text::CandleKimiK2Model;  // ❌ Private
let model = TextToTextModel::KimiK2(Arc::new(CandleKimiK2Model::default()));
```

**Why This Matters**: 
- Breaks `cargo check --workspace --all-targets`
- Blocks CI/CD pipelines  
- Prevents production deployment

**Solution Options**:

#### Option A: Move Tests to Internal Module (RECOMMENDED)
Move tests from `tests/` to `src/capability/registry/tests.rs`:

1. Create `src/capability/registry/tests.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::text_to_text::CandleKimiK2Model;  // ✅ Can access pub(crate)
    
    #[tokio::test]
    async fn test_runtime_registration() {
        let model = TextToTextModel::KimiK2(Arc::new(CandleKimiK2Model::default()));
        // ... rest of test
    }
}
```

2. Add to `src/capability/registry/mod.rs`:
```rust
#[cfg(test)]
mod tests;
```

3. Delete `tests/registry_tests.rs`

**Benefits**: Tests can access `pub(crate)` types, architecturally correct

#### Option B: Rewrite Tests for Public API Only
Remove tests that require direct model instantiation. Test only public getter APIs:
```rust
#[test]
fn test_get_registered_models() {
    // Test that statically registered models are accessible
    let model = registry::get::<TextToTextModel>("unsloth/phi-4-reasoning");
    assert!(model.is_some());
}
```

**Benefits**: Tests what external users can actually do  
**Drawback**: Cannot test runtime registration functionality

#### Option C: Make Registration Internal-Only
Change registration functions to `pub(crate)`:
```rust
pub(crate) async fn register_text_to_text(...) -> Result<...> {
    // Internal use only
}
```

**Benefits**: Aligns with architecture (external code shouldn't register)
**Drawback**: Breaks any external code using registration (but should they?)

---

### Issue 2: Misleading Documentation Examples

**Problem**: Three public registration functions have doc examples showing imports of **private types**.

**File**: `src/capability/registry/runtime.rs`

#### Fix 2.1: register_image_embedding (lines 40-47)

**Current (WRONG)**:
```rust
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
/// use paraphym_candle::capability::image_embedding::ClipVisionEmbeddingModel;  // ❌ Private
///
/// let model = ClipVisionEmbeddingModel::from_model(clip_model, 512);
/// registry::register_image_embedding("my-clip-model", model).await?;
/// ```
```

**Required Fix**:
```rust
/// # Example
/// 
/// **Note**: This function is for internal use only. External code cannot 
/// construct model instances directly.
/// 
/// ```rust,no_run
/// // Internal use only - external code cannot access model types
/// use paraphym_candle::capability::registry;
/// 
/// // Model registration happens during crate initialization
/// // External code should use: registry::get::<ImageEmbeddingModel>("model-key")
/// ```
```

#### Fix 2.2: register_text_to_image (lines 77-84)

**Current (WRONG)**:
```rust
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
/// use paraphym_candle::capability::text_to_image::FluxSchnell;  // ❌ Private
///
/// let model = FluxSchnell::from_pretrained().unwrap();
/// registry::register_text_to_image("flux-schnell", model).await?;
/// ```
```

**Required Fix**: Same pattern as 2.1

#### Fix 2.3: register_text_to_text (lines 114-125)

**Current (WRONG)**:
```rust
/// # Example
/// ```rust
/// use paraphym_candle::capability::registry;
/// use paraphym_candle::capability::text_to_text::CandleQwen3CoderModel;  // ❌ Private
/// use std::sync::Arc;
///
/// let model = CandleQwen3CoderModel::new().await?;
/// registry::register_text_to_text(
///     "unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF",
///     TextToTextModel::Qwen3Coder(Arc::new(model))  // ❌ Variant doesn't exist
/// ).await?;
/// ```
```

**Additional Issues**: References non-existent `TextToTextModel::Qwen3Coder` variant (should be `Qwen3Quantized`).

**Required Fix**: Same pattern as 2.1

---

### Issue 3: Unused Imports Cleanup

**File**: `src/capability/text_to_text/mod.rs`

**Problem**: Compiler warnings for 6 unused imports

**Current (lines 10-14)**:
```rust
pub(crate) use kimi_k2::{CandleKimiK2Model, KIMI_K2_MODEL_INFO, LoadedKimiK2Model};
pub(crate) use phi4_reasoning::{
    CandlePhi4ReasoningModel, LoadedPhi4ReasoningModel, PHI4_REASONING_MODEL_INFO,
};
pub(crate) use qwen3_quantized::{CandleQwen3QuantizedModel, LoadedQwen3QuantizedModel, QWEN3_QUANTIZED_MODEL_INFO};
```

**Required Fix**:
```rust
pub(crate) use kimi_k2::CandleKimiK2Model;
pub(crate) use phi4_reasoning::CandlePhi4ReasoningModel;
pub(crate) use qwen3_quantized::CandleQwen3QuantizedModel;
```

**Why**: The `Loaded*` variants and `*_MODEL_INFO` constants are not used within the crate.

---

## Definition of Done

### Success Criteria

1. **Tests compile and pass**
   - `cargo check --workspace --all-targets` succeeds with zero errors
   - All tests in `src/capability/registry/tests.rs` pass
   - Integration tests deleted OR rewritten for public API only

2. **Documentation is accurate**
   - No examples showing import of `pub(crate)` types
   - Clear indication that registration is internal-only
   - External users directed to use `registry::get()`

3. **No compiler warnings**
   - `cargo check --lib` produces zero warnings
   - Unused imports removed

4. **Code quality**
   - Documentation reflects actual API usage patterns
   - Tests verify functionality external users can access
   - No misleading examples that would confuse users

---

## Implementation Checklist

**Priority 1: Fix Broken Tests** 🔴
- [ ] Choose solution: Move to `src/` (Option A) OR Rewrite (Option B) OR Make internal (Option C)
- [ ] Implement chosen solution
- [ ] Verify `cargo check --workspace --all-targets` succeeds
- [ ] Verify `cargo test` passes

**Priority 2: Fix Documentation**
- [ ] Fix doc example in `register_image_embedding` (runtime.rs:40-47)
- [ ] Fix doc example in `register_text_to_image` (runtime.rs:77-84)
- [ ] Fix doc example in `register_text_to_text` (runtime.rs:114-125)

**Priority 3: Cleanup Warnings**
- [ ] Remove unused imports from `text_to_text/mod.rs` (lines 10-14)
- [ ] Verify `cargo check --lib` produces zero warnings

**Final Verification**
- [ ] `cargo check --workspace --all-targets` - zero errors
- [ ] `cargo test --workspace` - all pass
- [ ] `cargo doc --no-deps` - builds cleanly

---

## Verification Commands

```bash
# Must all succeed with zero errors/warnings
cargo check --workspace --all-targets
cargo test --workspace
cargo check --lib -p paraphym_candle
cargo doc --no-deps --lib
```

---

## Why This Matters

**Broken tests block production deployment.**

- CI/CD pipelines fail on test compilation errors
- Misleading documentation frustrates users
- Compiler warnings suggest unmaintained code
- Production-grade code requires production-grade quality

The architectural enforcement is **perfect**. The surrounding quality issues must be resolved before this can ship.

---

*This task completes the registry-only access pattern by fixing broken tests and ensuring documentation accurately reflects the enforced architecture.*
