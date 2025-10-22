# Issue: Tokenizer Cloned on Every Request

## Severity: MEDIUM
**Impact**: Unnecessary allocations, ~10-20% performance overhead

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs:226`

## Problem Description

In `LoadedStellaModel::embed()`, the tokenizer is cloned on every request:

```rust
fn embed(&self, text: &str, task: Option<String>) -> ... {
    let text = text.to_string();
    let tokenizer = self.tokenizer.clone();  // ← Clone entire tokenizer
    let model = self.model.clone();          // ← Arc clone (cheap)
    let device = self.device.clone();        // ← Device clone (cheap)
    
    Box::pin(async move {
        tokio::task::spawn_blocking(move || {
            // Use cloned tokenizer
            let tokens = tokenizer.encode(formatted_text, true)?;
            // ...
        })
    })
}
```

## Why It's Cloned

The tokenizer needs to be moved into `spawn_blocking`, which requires ownership. Since `self` is borrowed, we can't move `self.tokenizer` directly.

## Performance Impact

`Tokenizer::clone()` is **not cheap**:
- Clones internal vocabulary HashMap (~50K entries)
- Clones normalizer state
- Clones pre-tokenizer configuration
- Estimated cost: **5-10ms per clone**

For a batch of 100 embeddings:
- Wasted time: 500-1000ms just cloning tokenizers
- Memory churn: 100 temporary tokenizer allocations

## Better Approach

### Option 1: Arc-Wrapped Tokenizer (Recommended)

```rust
pub struct LoadedStellaModel {
    tokenizer: Arc<Tokenizer>,  // ← Wrap in Arc
    model: Arc<Mutex<EmbeddingModel>>,
    device: Device,
    config: Config,
    variant: ModelVariant,
}

fn embed(&self, text: &str, task: Option<String>) -> ... {
    let tokenizer = self.tokenizer.clone();  // ← Now just Arc clone (cheap)
    // ...
}
```

**Pros**:
- Arc clone is just a pointer copy + atomic increment
- ~1000x faster than full tokenizer clone
- No API changes needed

**Cons**:
- Tokenizer is immutable (but we don't mutate it anyway)

### Option 2: Tokenizer Pool

```rust
pub struct LoadedStellaModel {
    tokenizer_pool: Arc<Mutex<Vec<Tokenizer>>>,  // Pool of tokenizers
    // ...
}
```

**Pros**:
- No cloning at all
- Supports parallel tokenization

**Cons**:
- More complex
- Requires pool management

## Recommendation

**Use Arc<Tokenizer>** - simple, effective, no downsides for our read-only use case.

## Related Issue

Same problem exists in `batch_embed()` at line 305:
```rust
let tokenizer = self.tokenizer.clone();  // ← Also clones
```
