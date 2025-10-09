# MPOOL_4: Create LoadedModel Wrappers for TextEmbedding Models

**PREFIX**: MPOOL (Model Pool)

## OBJECTIVE

Extract model loading logic from 5 TextEmbedding models (gte_qwen, jina_bert, nvembed, stella, bert) and create LoadedModel wrapper structs that stay alive in worker threads. This eliminates disk reload on every inference call.

## CONTEXT

Current TextEmbedding models reload from disk per call:
- Load tokenizer (I/O)
- Read config.json (I/O)
- Map safetensors (I/O)
- Create model instance
- Run inference ONCE
- **Discard everything**

LoadedModel pattern:
- Load once during worker spawn
- Store (tokenizer, model, device, config) in struct
- Run inference many times
- Stay alive until worker evicted

## SUBTASK 1: Create LoadedGteQwenModel Wrapper

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_embedding/gte_qwen.rs`

**Current loading logic location**: Lines 178-249 in `embed()` method

**Add new struct**:
```rust
/// Loaded GTE-Qwen model that stays in memory
pub struct LoadedGteQwenModel {
    tokenizer: Tokenizer,
    model: Model,
    device: Device,
    config: Config,
}

impl LoadedGteQwenModel {
    /// Create loaded model from base model info
    pub fn load(base_model: &CandleGteQwenEmbeddingModel)
        -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Extract loading logic from current embed() method (lines 178-249)
        // 1. Get paths via huggingface_file
        // 2. Load tokenizer from file
        // 3. Load config.json
        // 4. Load model weights via VarBuilder
        // 5. Create Model instance
        // 6. Return LoadedGteQwenModel

        todo!("Extract loading logic from lines 178-249")
    }
}

impl TextEmbeddingCapable for LoadedGteQwenModel {
    fn embed(&self, text: &str, task: Option<String>)
        -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>>
    {
        // NO I/O - everything already loaded in self
        let embeddings = CandleGteQwenEmbeddingModel::forward_pass_with_task(
            &self.tokenizer,
            &mut self.model,  // Use loaded model
            &self.device,
            &[text],
            task.as_deref(),
        )?;

        Ok(embeddings.into_iter().next().unwrap())
    }

    fn batch_embed(&self, texts: &[String], task: Option<String>)
        -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error + Send + Sync>>
    {
        // Similar - use loaded state
        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        CandleGteQwenEmbeddingModel::forward_pass_with_task(
            &self.tokenizer,
            &mut self.model,
            &self.device,
            &text_refs,
            task.as_deref(),
        )
    }

    fn embedding_dimension(&self) -> usize {
        GTEQWEN_MODEL_INFO.embedding_dimension.unwrap_or(1536) as usize
    }

    fn supported_dimensions(&self) -> Vec<usize> {
        vec![1536]
    }

    fn recommended_batch_size(&self) -> usize {
        8
    }

    fn max_batch_size(&self) -> usize {
        32
    }
}
```

**Why**: Worker owns loaded model, processes many requests without reloading.

## SUBTASK 2: Create LoadedJinaBertModel Wrapper

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_embedding/jina_bert.rs`

**Current loading logic location**: Lines 178-242 in `embed()` method

**Follow same pattern as GTE-Qwen**:
```rust
pub struct LoadedJinaBertModel {
    tokenizer: Tokenizer,
    model: BertModel,
    device: Device,
}

impl LoadedJinaBertModel {
    pub fn load(base_model: &CandleJinaBertEmbeddingModel)
        -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Extract loading logic from lines 178-242
        todo!()
    }
}

impl TextEmbeddingCapable for LoadedJinaBertModel {
    // Implement all trait methods using loaded state
}
```

**Why**: Same pattern for JinaBERT model.

## SUBTASK 3: Create LoadedNvEmbedModel Wrapper

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_embedding/nvembed.rs`

**Current loading logic location**: Lines 188-267 in `load_model_and_tokenizer()` method

**Follow same pattern**:
```rust
pub struct LoadedNvEmbedModel {
    tokenizer: Tokenizer,
    model: NvEmbedModel,
    device: Device,
}

impl LoadedNvEmbedModel {
    pub fn load(base_model: &CandleNvEmbedEmbeddingModel)
        -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Extract loading logic from lines 188-267
        todo!()
    }
}

impl TextEmbeddingCapable for LoadedNvEmbedModel {
    // Implement all trait methods using loaded state
}
```

**Why**: Same pattern for NvEmbed model.

## SUBTASK 4: Create LoadedStellaModel Wrapper

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_embedding/stella.rs`

**Current loading logic location**: Lines 191-258 in `load_model_and_tokenizer()` method

**Follow same pattern**:
```rust
pub struct LoadedStellaModel {
    tokenizer: Tokenizer,
    model: BertModel,
    device: Device,
}

impl LoadedStellaModel {
    pub fn load(base_model: &CandleStellaEmbeddingModel)
        -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Extract loading logic from lines 191-258
        todo!()
    }
}

impl TextEmbeddingCapable for LoadedStellaModel {
    // Implement all trait methods using loaded state
}
```

**Why**: Same pattern for Stella model.

## SUBTASK 5: Create LoadedBertModel Wrapper

**File**: `/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_embedding/bert.rs`

**Current loading logic location**: Lines 165-229 in `load_model_and_tokenizer()` method

**Follow same pattern**:
```rust
pub struct LoadedBertModel {
    tokenizer: Tokenizer,
    model: BertModel,
    device: Device,
}

impl LoadedBertModel {
    pub fn load(base_model: &CandleBertEmbeddingModel)
        -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
    {
        // Extract loading logic from lines 165-229
        todo!()
    }
}

impl TextEmbeddingCapable for LoadedBertModel {
    // Implement all trait methods using loaded state
}
```

**Why**: Same pattern for BERT model.

## DEFINITION OF DONE

- [ ] LoadedGteQwenModel struct created and implements TextEmbeddingCapable
- [ ] LoadedJinaBertModel struct created and implements TextEmbeddingCapable
- [ ] LoadedNvEmbedModel struct created and implements TextEmbeddingCapable
- [ ] LoadedStellaModel struct created and implements TextEmbeddingCapable
- [ ] LoadedBertModel struct created and implements TextEmbeddingCapable
- [ ] All `.load()` factory methods extract loading logic from current implementations
- [ ] All trait implementations use loaded state (no I/O in trait methods)
- [ ] Code compiles with `cargo check`
- [ ] Original model structs remain unchanged (backward compatibility)

## DEPENDENCIES

**Requires**: MPOOL_3A (needs pool API to call), MPOOL_1 (CandleModelInfo with est_memory_allocation_mb)

**Blocks**: MPOOL_5 (registry integration uses these LoadedModel types)

## RESEARCH NOTES

**Problem Pattern** (from MODEL_POOL.md):
```rust
// Current: Load, use once, discard
fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError> {
    let tokenizer = Tokenizer::from_file(&tokenizer_path)?;  // I/O
    let config = serde_json::from_str(&config_str)?;         // I/O
    let vb = VarBuilder::from_mmaped_safetensors(...)?;      // I/O
    let model = Model::new(&config, vb)?;                    // GPU memory

    let embeddings = forward_pass(&tokenizer, &model, ...)?;  // Inference

    Ok(embeddings)  // DISCARD tokenizer, model, everything
}
```

**Solution Pattern**:
```rust
// New: Load once, use many times
struct LoadedModel {
    tokenizer: Tokenizer,  // STAYS IN MEMORY
    model: Model,          // STAYS IN MEMORY
    device: Device,        // STAYS IN MEMORY
}

fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, BoxError> {
    // NO I/O - everything already loaded
    forward_pass(&self.tokenizer, &self.model, ...)
}
```

**Worker Usage**:
```rust
// Worker thread loads model once
let loaded_model = LoadedGteQwenModel::load(&base_model)?;

// Processes many requests without reloading
loop {
    let req = recv_request();
    let result = loaded_model.embed(&req.text, req.task);  // NO I/O
    send_response(result);
}
```

## CONSTRAINTS

- **NO TESTS**: Do not write any test code. Tests are handled by separate team.
- **NO BENCHMARKS**: Do not write any benchmark code. Benchmarks are handled by separate team.
- **BACKWARD COMPATIBILITY**: Original model structs must remain unchanged. LoadedModel is NEW addition.
- **TRAIT IMPLEMENTATION**: LoadedModel must implement TextEmbeddingCapable trait for worker generics.
