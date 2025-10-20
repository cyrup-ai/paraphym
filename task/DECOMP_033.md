# DECOMP_033: Reduce base.rs and loaded.rs Line Counts

**File:** `packages/candle/src/capability/text_embedding/gte_qwen/`  
**Module Area:** capability / text_embedding  
**Status:** 90% Complete - Requires line count optimization

---

## OBJECTIVE

Reduce `base.rs` and `loaded.rs` to meet the "< 250 lines per module" requirement by extracting duplicate model-loading code into helper methods.

---

## CURRENT STATUS

✅ **COMPLETED ITEMS:**
- Original `gte_qwen.rs` (711 lines) successfully deleted
- New `gte_qwen/` directory created with 5 focused modules
- Module structure matches nvembed/stella pattern exactly
- mod.rs: 14 lines ✓
- config.rs: 44 lines ✓
- instruction.rs: 129 lines ✓
- Parent `mod.rs` updated to export both models
- Public API completely preserved
- Zero unwrap()/expect() in implementation
- All functionality preserved
- No compilation errors in gte_qwen module

❌ **REMAINING ISSUES:**

**Line Count Requirement Violation:**
- **base.rs: 306 lines** (56 lines OVER the < 250 requirement)
- **loaded.rs: 262 lines** (12 lines OVER the < 250 requirement)

---

## ROOT CAUSE ANALYSIS

Both `base.rs` and `loaded.rs` contain substantial duplicate code in their `TextEmbeddingCapable` trait implementations:

**Duplication in base.rs:**
- `embed()` method: lines 39-164 (~125 lines)
  - Model loading setup: ~80 lines
  - Forward pass call: ~5 lines
- `batch_embed()` method: lines 166-291 (~125 lines)
  - **Identical model loading setup: ~80 lines** (DUPLICATE)
  - Forward pass call: ~5 lines

**Duplication in loaded.rs:**
- Similar pattern in `embed()` and `batch_embed()` implementations

---

## SOLUTION: Extract Helper Method

### For base.rs

Extract the duplicate model-loading code into a private helper method:

```rust
impl CandleGteQwenEmbeddingModel {
    /// Load model and tokenizer from disk
    ///
    /// Helper method to eliminate duplication between embed() and batch_embed()
    async fn load_model_and_tokenizer(
        &self,
    ) -> std::result::Result<
        (Tokenizer, Model, Device),
        Box<dyn std::error::Error + Send + Sync>,
    > {
        // Get configuration from ModelInfo
        let max_length = self
            .info()
            .max_input_tokens
            .ok_or("max_input_tokens missing in ModelInfo")?
            .get() as usize;

        // Auto-detect runtime environment
        let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
            log::warn!("Device detection failed: {}. Using CPU.", e);
            Device::Cpu
        });

        // Auto-detect dtype based on device
        let dtype = if device.is_cuda() {
            DType::F16
        } else {
            DType::F32
        };

        // Get file paths via huggingface_file
        let tokenizer_path = self.huggingface_file(self.info().registry_key, "tokenizer.json").await?;
        let config_path = self.huggingface_file(self.info().registry_key, "config.json").await?;
        let index_path =
            self.huggingface_file(self.info().registry_key, "model.safetensors.index.json").await?;

        // Load tokenizer
        let mut tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

        // Configure tokenizer for left padding with EOS token
        let eos_token_id = 151643;
        if tokenizer.token_to_id("|endoftext|>") != Some(eos_token_id) {
            return Err(Box::from(format!(
                "Tokenizer EOS token mismatch: expected {}, got {:?}",
                eos_token_id,
                tokenizer.token_to_id("|endoftext|>")
            )));
        }

        let padding_params = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            direction: tokenizers::PaddingDirection::Left,
            pad_id: eos_token_id,
            pad_token: "|endoftext|>".to_string(),
            ..Default::default()
        };
        tokenizer.with_padding(Some(padding_params));

        let truncation_params = TruncationParams {
            max_length,
            ..Default::default()
        };
        tokenizer
            .with_truncation(Some(truncation_params))
            .map_err(|e| format!("Failed to set truncation: {}", e))?;

        // Load config.json
        let config_str = tokio::fs::read_to_string(&config_path).await
            .map_err(|e| format!("Failed to read config: {}", e))?;
        let qwen_config: Config = serde_json::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        // Load model weights from index
        let model_dir = index_path.parent().ok_or("Failed to get model directory")?;

        let index_content = tokio::fs::read_to_string(&index_path).await
            .map_err(|e| format!("Failed to read index: {}", e))?;
        let index: serde_json::Value = serde_json::from_str(&index_content)
            .map_err(|e| format!("Failed to parse index: {}", e))?;

        let weight_map = index["weight_map"]
            .as_object()
            .ok_or("Missing weight_map in index")?;

        let mut unique_files: std::collections::HashSet<String> = std::collections::HashSet::new();
        for filename in weight_map.values() {
            if let Some(filename_str) = filename.as_str() {
                unique_files.insert(filename_str.to_string());
            }
        }

        let weight_files: Vec<std::path::PathBuf> = unique_files
            .into_iter()
            .map(|f| model_dir.join(f))
            .collect();

        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&weight_files, dtype, &device)
                .map_err(|e| format!("Failed to load weights: {}", e))?
        };

        // Create model
        let model =
            Model::new(&qwen_config, vb).map_err(|e| format!("Failed to create model: {}", e))?;

        Ok((tokenizer, model, device))
    }
}
```

**Then update embed() to use it:**
```rust
fn embed(&self, text: &str, task: Option<String>) -> ... {
    let text = text.to_string();
    Box::pin(async move {
        self.validate_input(&text)?;
        
        let (tokenizer, model, device) = self.load_model_and_tokenizer().await?;
        
        let (_model, embeddings) =
            instruction::forward_pass_with_task(tokenizer, model, device, vec![text], task)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| "No embeddings generated".into())
    })
}
```

**And batch_embed():**
```rust
fn batch_embed(&self, texts: &[String], task: Option<String>) -> ... {
    let texts = texts.to_vec();
    Box::pin(async move {
        self.validate_batch(&texts)?;
        
        let (tokenizer, model, device) = self.load_model_and_tokenizer().await?;
        
        let (_model, embeddings) = 
            instruction::forward_pass_with_task(tokenizer, model, device, texts, task)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(embeddings)
    })
}
```

**Expected result:**
- base.rs: ~220 lines (86 lines reduction)
- loaded.rs: Already has this pattern in `load()` method, no changes needed (stays at 262, but see note below)

**Note on loaded.rs:** The `load()` method in loaded.rs already consolidates model loading, so loaded.rs only needs minor cleanup if any. The 262 lines may be acceptable if most of the code is in the single `load()` method rather than duplicated across `embed()` and `batch_embed()`.

---

## IMPLEMENTATION STEPS

1. **Add `load_model_and_tokenizer()` helper to base.rs**
   - Extract lines that are identical in both embed() and batch_embed()
   - Make it a private async method on CandleGteQwenEmbeddingModel

2. **Refactor `embed()` in base.rs**
   - Replace duplicate loading code with call to helper
   - Verify behavior is identical

3. **Refactor `batch_embed()` in base.rs**
   - Replace duplicate loading code with call to helper
   - Verify behavior is identical

4. **Check loaded.rs line count**
   - If still > 250, review if any further consolidation is possible
   - The `embed()` and `batch_embed()` methods should already be minimal

5. **Verify compilation**
   ```bash
   cargo check -p cyrup_candle
   ```

6. **Verify line counts**
   ```bash
   wc -l packages/candle/src/capability/text_embedding/gte_qwen/*.rs
   ```

---

## DEFINITION OF DONE

- [ ] base.rs is < 250 lines
- [ ] loaded.rs is < 250 lines (or justifiably minimal if slightly over)
- [ ] No unwrap() or expect() added
- [ ] All functionality preserved (behavior identical)
- [ ] Cargo check shows no new errors in gte_qwen module
- [ ] Code is production quality

---

## CONSTRAINTS

- **DO NOT** change public API
- **DO NOT** change method behavior
- **DO NOT** add unwrap() or expect()
- **DO NOT** fix unrelated compilation errors in other modules
- Extract helper methods ONLY - this is internal refactoring

---

## SUCCESS CRITERIA

Task succeeds when ALL files in `gte_qwen/` are < 250 lines while maintaining identical functionality and API.
