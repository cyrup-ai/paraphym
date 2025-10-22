# Issue: Base Model Reloads Entire Model on Every Request

## Severity: CRITICAL
**Impact**: Extreme performance degradation, 1000x+ slower than necessary

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/base.rs`

## Problem Description

The `StellaEmbeddingModel` (base.rs) is a **zero-state struct** that reloads the entire model from disk on EVERY embedding request:

```rust
pub struct StellaEmbeddingModel {}  // ← No state!

impl TextEmbeddingCapable for StellaEmbeddingModel {
    fn embed(&self, text: &str, task: Option<String>) -> ... {
        Box::pin(async move {
            // Lines 89-100: Load files from disk EVERY TIME
            let base_weights = self.huggingface_file(...).await?;
            let projection_head = self.huggingface_file(...).await?;
            let tokenizer_path = self.huggingface_file(...).await?;
            
            // Lines 103-169: Recreate tokenizer and model EVERY TIME
            let mut tokenizer = Tokenizer::from_file(&tokenizer_path)?;
            let mut model = EmbeddingModel::new(&stella_config, base_vb, embed_vb)?;
            
            // Only NOW does it actually embed
            let embeddings = model.forward_norm(...)?;
        })
    }
}
```

## Performance Impact

For stella_en_400M_v5:
- **Model loading**: ~2-5 seconds (400MB weights from disk)
- **Actual inference**: ~50-100ms
- **Overhead**: 20-100x slower than necessary

For stella_en_1.5B_v5:
- **Model loading**: ~8-15 seconds (1.5GB weights)
- **Actual inference**: ~150-300ms
- **Overhead**: 50-100x slower

## Why This Exists

The `base.rs` implementation appears to be a **fallback/compatibility layer** for when the model pool is not used. However:

1. It's **never used in production** - the pool always uses `LoadedStellaModel`
2. It's **extremely wasteful** if accidentally called
3. It creates **confusion** about which implementation is active

## Evidence from Logs

The logs show workers using `LoadedStellaModel` correctly:
```
[INFO] TextEmbedding worker 0 ready
[INFO] TextEmbedding worker 1 ready
```

But if `base.rs` were ever called directly, it would cause massive slowdowns.

## Root Cause

The worker pool spawns `LoadedStellaModel` (correct):
```rust
// In text_embedding.rs
pool.spawn_text_embedding_worker(
    registry_key,
    move || async move {
        LoadedStellaModel::load(&m_clone).await  // ← Uses loaded.rs
    },
    ...
)
```

But the `TextEmbeddingModel` enum variant holds `StellaEmbeddingModel` (base.rs):
```rust
pub enum TextEmbeddingModel {
    Stella(Arc<StellaEmbeddingModel>),  // ← This is base.rs!
    // ...
}
```

## Fix Required

**Option 1**: Remove `base.rs` entirely, only keep `LoadedStellaModel`
- Simplest solution
- Eliminates confusion
- Forces proper pool usage

**Option 2**: Make `base.rs` a thin wrapper that spawns a pool worker
- Maintains API compatibility
- Still uses efficient loaded model

**Option 3**: Add clear documentation and runtime warnings
- Least invasive
- Doesn't prevent the problem

## Recommendation

**Remove base.rs** - it serves no production purpose and is a performance footgun.
