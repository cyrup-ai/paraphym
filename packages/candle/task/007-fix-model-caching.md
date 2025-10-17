# Task 007: Fix LoadedPhi4ReasoningModel to cache model (IN PROGRESS 🔨)

## Problem
LoadedPhi4ReasoningModel stores Arc<Model> but prompt() method still tries to load from gguf_path

## Current State
- ✅ Struct changed to store `model: Arc<CandleQuantizedPhiModel>`
- ✅ `load()` method loads model once into Arc
- 🔨 `prompt()` method updated to use cached model BUT needs compilation fix
- ❌ SharedPhiModel wrapper created but has unsafe code

## Blocking Issue
**COMPILATION ERROR**: `CandleQuantizedPhiModel` doesn't implement `Clone`

```rust
// Line 498 - FAILS
let quantized_model = (*model).clone();
```

## Real Solution

**DON'T FIX THIS YET** - Wait for Task 001 (async model.forward) to complete first.

Once CandleModel trait is async, the entire architecture changes:
1. TextGenerator will own the model differently for async access
2. SharedPhiModel wrapper may not be needed
3. The caching strategy will be clearer

This task is **BLOCKED BY** and **SUPERSEDED BY** Task 001.

## Dependencies
- **MUST WAIT FOR:** Task 001 (async forward)
- Then refactor model ownership for async access

## Estimated Effort
2 hours
