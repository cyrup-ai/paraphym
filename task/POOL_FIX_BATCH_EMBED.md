# POOL_FIX_BATCH_EMBED: Fix batch_embed() Cold Start Logic

## OBJECTIVE

Fix the bug in TextEmbedding pool integration where `batch_embed()` does not spawn workers on first call, causing `PoolError::NoWorkers` if batch_embed() is called before embed().

## SCOPE

**5 Models:**
- GteQwen
- JinaBert
- NvEmbed
- Stella
- Bert

**1 Method:**
- `batch_embed()` - batch text embedding

## THE BUG

**Current State**: `batch_embed()` assumes workers already exist
**Problem**: If user calls `batch_embed()` as their FIRST operation, it fails with `PoolError::NoWorkers`

## CURRENT STATE (Broken)

**File**: `packages/candle/src/capability/registry.rs` lines ~437-470

```rust
fn batch_embed(&self, texts: &[String], task: Option<String>)
    -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
    match self {
        Self::GteQwen(m) => {
            let registry_key = m.info().registry_key;
            let pool = text_embedding_pool();

            // ❌ NO COLD START CHECK - just calls pool directly
            pool.batch_embed_text(registry_key, texts, task)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        }
        Self::JinaBert(m) => {
            // Same bug for all 5 models
        }
        // ... NvEmbed, Stella, Bert
    }
}
```

## REQUIRED STATE (Fixed)

Add the SAME cold start logic that `embed()` already has:

```rust
fn batch_embed(&self, texts: &[String], task: Option<String>)
    -> std::result::Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>> {
    match self {
        Self::GteQwen(m) => {
            let registry_key = m.info().registry_key;
            let pool = text_embedding_pool();

            // ✅ ADD COLD START CHECK (copy from embed())
            if !pool.has_workers(registry_key) {
                let per_worker_mb = m.info().est_memory_allocation_mb;
                let current_mb = pool.total_memory_mb();
                let total_system_mb = query_system_memory_mb();
                let memory_limit_mb = (total_system_mb as f64 * 0.80) as usize;

                let workers_to_spawn = if current_mb + (2 * per_worker_mb) <= memory_limit_mb {
                    2
                } else if current_mb + per_worker_mb <= memory_limit_mb {
                    1
                } else {
                    return Err(Box::new(PoolError::MemoryExhausted(format!(
                        "Cannot spawn workers for {}. Need {} MB, only {} MB available",
                        registry_key, per_worker_mb, memory_limit_mb.saturating_sub(current_mb)
                    ))) as Box<dyn std::error::Error + Send + Sync>);
                };

                for _ in 0..workers_to_spawn {
                    let m_clone = m.clone();
                    pool.spawn_text_embedding_worker(
                        registry_key,
                        move || {
                            LoadedGteQwenModel::load(&m_clone)
                                .map_err(|e| PoolError::SpawnFailed(e.to_string()))
                        },
                        per_worker_mb,
                    ).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                }
            }

            // Now route through pool
            pool.batch_embed_text(registry_key, texts, task)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        }
        Self::JinaBert(m) => {
            // Same pattern with LoadedJinaBertModel
        }
        Self::NvEmbed(m) => {
            // Same pattern with LoadedNvEmbedModel
        }
        Self::Stella(m) => {
            // Same pattern with LoadedStellaModel
        }
        Self::Bert(m) => {
            // Same pattern with LoadedBertModel
        }
    }
}
```

## IMPLEMENTATION STEPS

### 1. Find batch_embed() Implementation
Located at registry.rs lines ~437-470

### 2. Copy Cold Start Block from embed()
The cold start logic is in embed() at lines ~237-278

### 3. Add Cold Start to GteQwen batch_embed()
Paste the cold start block after `let pool = text_embedding_pool();`

### 4. Update LoadedModel Type
Change `LoadedGteQwenModel::load(&m_clone)` to match model

### 5. Repeat for JinaBert
Copy cold start block, update to `LoadedJinaBertModel`

### 6. Repeat for NvEmbed
Copy cold start block, update to `LoadedNvEmbedModel`

### 7. Repeat for Stella
Copy cold start block, update to `LoadedStellaModel`

### 8. Repeat for Bert
Copy cold start block, update to `LoadedBertModel`

## VERIFICATION

### Compile Check
```bash
cargo check -p paraphym_candle
```

### Test Batch-First Workflow
```rust
let model = registry::get::<TextEmbeddingModel>("dunzhang/stella_en_1.5B_v5")?;

// This should work now (previously failed with NoWorkers)
let texts = vec!["hello".to_string(), "world".to_string()];
let embeddings = model.batch_embed(&texts, None)?;

assert_eq!(embeddings.len(), 2);  // ✅ Should pass
```

### Test That embed() Still Works
```rust
let model = registry::get::<TextEmbeddingModel>("dunzhang/stella_en_1.5B_v5")?;

// This already worked
let embedding = model.embed("test", None)?;
```

## WHY THIS BUG EXISTED

The QA review found that `embed()` has cold start logic but `batch_embed()` does not. This creates an API asymmetry where:

❌ **Broken**: `batch_embed()` as first operation → fails
✅ **Works**: `embed()` then `batch_embed()` → works
✅ **Works**: `embed()` as first operation → works

The fix makes both methods work independently.

## DEFINITION OF DONE

- [ ] `batch_embed()` for GteQwen has cold start logic
- [ ] `batch_embed()` for JinaBert has cold start logic
- [ ] `batch_embed()` for NvEmbed has cold start logic
- [ ] `batch_embed()` for Stella has cold start logic
- [ ] `batch_embed()` for Bert has cold start logic
- [ ] `cargo check -p paraphym_candle` passes
- [ ] Test case: calling batch_embed() first works without errors
- [ ] No unwrap() or expect() in implementation

## ESTIMATED TIME

15 minutes (5 models, simple copy-paste from embed())
