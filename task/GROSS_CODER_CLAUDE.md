# Cleanup: Duplicate Model/Provider Definitions

## Problem

Claude created numerous duplicate trait, enum, and struct definitions for Models and Providers throughout the codebase. These duplicates cause:
- Compilation ambiguity
- Maintenance overhead
- API confusion
- Import hell

## Scope

**Files to Modify:** 15+ files across domain, capability, memory, context, and builders

**What to Keep:**
- ✅ [`domain/model/traits.rs`](../packages/candle/src/domain/model/traits.rs) - `CandleModel` trait (canonical)
- ✅ [`domain/model/info.rs`](../packages/candle/src/domain/model/info.rs) - `CandleProvider` enum (canonical)
- ✅ [`capability/`](../packages/candle/src/capability/) - All capability trait implementations

**What to Delete:** Everything else that duplicates the above

---

## Duplicate Traits to Remove

### ✅ 1. CandleCompletionModel - COMPLETED
- ✅ Deleted `domain/completion/traits.rs` entirely
- ✅ Deleted trait from `domain/completion/core.rs`
- ✅ Deleted `CandleCompletionProvider` stub from `builders/agent_role.rs`
- ✅ Updated 4 call sites to use `TextToTextCapable + CandleModel`
- ✅ Removed `NoProvider` and `NoProviderAgent` structs

### 2. EmbeddingModel - 3 COPIES

**KEEP:** None of these. All models should use capability traits from `capability/traits.rs`

**DELETE:** [`domain/embedding/core.rs:15`](../packages/candle/src/domain/embedding/core.rs#L15)
- Delete entire file: `domain/embedding/core.rs`
- This is superseded by `TextEmbeddingCapable` and `ImageEmbeddingCapable` in capability/traits.rs

**DELETE:** [`memory/vector/embedding_model.rs:24`](../packages/candle/src/memory/vector/embedding_model.rs#L24)
- Keep the file, but refactor to use `TextEmbeddingCapable` from capability/traits.rs
- Change trait bound from `EmbeddingModel` to `TextEmbeddingCapable`

**DELETE:** [`context/provider.rs:969`](../packages/candle/src/context/provider.rs#L969)
- Already marked deprecated
- Delete lines 968-972:
```rust
#[deprecated(note = "Use CandleImmutableEmbeddingModel instead")]
pub trait EmbeddingModel: CandleImmutableEmbeddingModel {}

#[deprecated(note = "Use CandleImmutableMemoryManager instead")]
pub trait MemoryManager: CandleImmutableMemoryManager {}
```

### 4. CandleImmutableEmbeddingModel - 2 COPIES

**KEEP:** [`domain/context/provider.rs:415`](../packages/candle/src/domain/context/provider.rs#L415)

**DELETE:** [`context/provider.rs:355`](../packages/candle/src/context/provider.rs#L355)
- Delete trait definition (lines 355-379)
- Update imports in this file to use domain version

### 5. CandleModel - INCOMPATIBLE DUPLICATE

**KEEP:** [`domain/model/traits.rs:12`](../packages/candle/src/domain/model/traits.rs#L12) - The real CandleModel trait

**DELETE:** [`core/generation/models.rs:26`](../packages/candle/src/core/generation/models.rs#L26)
- This is a DIFFERENT trait with different methods (has `forward()`)
- Rename to `GenerativeModel` or `InferenceModel`
- Delete lines 26-30, replace with:
```rust
pub trait GenerativeModel: Send + Sync {
    fn forward(&mut self, input: &Tensor, position: usize) -> CandleResult<Tensor>;
    // ... rest of methods
}
```

---

## Duplicate Enums to Remove

### 6. Model Variant Enums - ENTIRE FILE DUPLICATE

**DELETE:** [`domain/model/providers.rs`](../packages/candle/src/domain/model/providers.rs) - **ENTIRE FILE**

This file contains:
- `TextToTextModel` (line 10)
- `TextEmbeddingModel` (line 18)
- `ImageEmbeddingModel` (line 28)
- `VisionModel` (line 34)
- `TextToImageModel` (line 41)
- `DomainModelType` (line 112)
- `CandleDomainModel` (line 106)

**Why delete:** These are completely redundant with:
- `CandleProvider` enum in domain/model/info.rs (provider organization)
- Capability traits in capability/traits.rs (model capabilities)
- Actual model structs in capability/ (concrete implementations)

**Migration:**
- Any code using `TextToTextModel::KimiK2` should use the actual struct `CandleKimiK2Model`
- Any code using `DomainModelType` should use capability traits instead
- Search for imports and replace with direct model references

### 7. EmbeddingModelType - 2 COPIES

**KEEP:** Neither. Use `CandleProvider` enum instead.

**DELETE:** [`domain/memory/config/shared.rs:217`](../packages/candle/src/domain/memory/config/shared.rs#L217)
- Delete enum (lines 214-224)
- Replace usages with `CandleProvider`

**DELETE:** [`memory/utils/config.rs:97`](../packages/candle/src/memory/utils/config.rs#L97)
- Delete enum (lines 96-103)
- Replace usages with `CandleProvider`

### 8. Provider Enums - UNRELATED TO CandleProvider

**DELETE:** [`memory/utils/config.rs:125`](../packages/candle/src/memory/utils/config.rs#L125)
- `CompletionProvider` enum
- This seems to be for external API providers (OpenAI, Anthropic)
- Keep if needed for external API config
- If so, rename to `ExternalAPIProvider` to avoid confusion

**DELETE:** [`domain/memory/config/llm.rs:9`](../packages/candle/src/domain/memory/config/llm.rs#L9)
- `LLMProvider` enum
- Same as above - external API providers
- Consolidate with `CompletionProvider` or rename to `ExternalLLMProvider`

---

## Duplicate Structs to Remove

### 9. CandleProviderError - 2 COPIES

**KEEP:** [`domain/context/provider.rs:86`](../packages/candle/src/domain/context/provider.rs#L86)

**DELETE:** [`context/provider.rs:76`](../packages/candle/src/context/provider.rs#L76)
- Delete error enum (lines 75-102)
- Update imports to use domain version
- Fix any match statements that reference this

### 10. Misnamed Config Struct

**RENAME:** [`domain/chat/config.rs:66`](../packages/candle/src/domain/chat/config.rs#L66)
- Current name: `CandleModelConfig`
- This is CHAT-SPECIFIC, not a general model config
- Rename to: `CandleChatModelConfig`
- Update all references (search codebase)

---

## Files to Delete Entirely

1. **[`domain/completion/traits.rs`](../packages/candle/src/domain/completion/traits.rs)**
   - Reason: Exact replica of core.rs, serves no purpose

2. **[`domain/embedding/core.rs`](../packages/candle/src/domain/embedding/core.rs)**
   - Reason: Superseded by capability traits

3. **[`domain/model/providers.rs`](../packages/candle/src/domain/model/providers.rs)**
   - Reason: Duplicates capability organization, adds no value

---

## Cleanup Steps

### Step 1: Remove Trait Duplicates

```bash
# Delete entire duplicate files
rm packages/candle/src/domain/completion/traits.rs
rm packages/candle/src/domain/embedding/core.rs
rm packages/candle/src/domain/model/providers.rs

# Update mod.rs files to remove references
# domain/completion/mod.rs - remove: mod traits;
# domain/embedding/mod.rs - remove: mod core;
# domain/model/mod.rs - remove: mod providers;
```

### Step 2: Fix Imports

Search for imports of deleted items and replace:

```rust
// BEFORE
use crate::domain::completion::traits::CandleCompletionModel;
use crate::domain::model::providers::{TextToTextModel, DomainModelType};
use crate::domain::embedding::core::EmbeddingModel;

// AFTER
use crate::domain::completion::core::CandleCompletionModel;
use crate::capability::text_to_text::kimi_k2::CandleKimiK2Model; // Use actual struct
use crate::capability::traits::TextEmbeddingCapable; // Use capability trait
```

### Step 3: Remove Inline Duplicates

**File: `builders/agent_role.rs`**
- Delete lines 34-35 (CandleCompletionProvider stub)

**File: `context/provider.rs`**
- Delete lines 355-379 (CandleImmutableEmbeddingModel duplicate)
- Delete lines 968-972 (deprecated trait aliases)

**File: `domain/context/provider.rs`**
- Keep this one (it's the canonical version)

**File: `core/generation/models.rs`**
- Rename `CandleModel` trait to `GenerativeModel` (lines 26-30)

### Step 4: Consolidate Enums

**Files: `domain/memory/config/shared.rs` and `memory/utils/config.rs`**
- Delete `EmbeddingModelType` from both
- Replace with `CandleProvider` enum

**Files: `memory/utils/config.rs` and `domain/memory/config/llm.rs`**
- Consolidate `CompletionProvider` and `LLMProvider`
- Rename to `ExternalAPIProvider` if needed for clarity

### Step 5: Rename Misnamed Struct

**File: `domain/chat/config.rs`**
- Rename `CandleModelConfig` → `CandleChatModelConfig` (line 66)
- Find and replace all usages across codebase

---

## Verification

After cleanup, run:

```bash
cd /Volumes/samsung_t9/paraphym

# Check compilation
cargo check --package paraphym_candle

# Search for any remaining duplicates
rg "trait CandleCompletionModel" packages/candle/src/
rg "enum TextToTextModel" packages/candle/src/
rg "trait EmbeddingModel" packages/candle/src/

# Expected: Only canonical versions remain
```

---

## Definition of Done

- ✅ 3 duplicate files deleted
- ✅ Zero duplicate trait definitions
- ✅ Zero duplicate enum definitions (except external API enums if needed)
- ✅ `cargo check -p paraphym_candle` passes
- ✅ All imports updated to use canonical versions
- ✅ Misnamed structs renamed appropriately
- ✅ No remaining references to deleted items

---

## Summary

**Total Items to Delete:** 
- 3 entire files
- 7 trait definitions
- 5 enum definitions
- 2 struct definitions
- 1 rename

**Estimated LOC Removed:** ~800 lines

**Files Modified:** ~15 files

**Risk:** Low (most are unused duplicates)
