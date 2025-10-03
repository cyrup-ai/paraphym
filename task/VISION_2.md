# VISION_2: Multimodal Embedding Service Integration

## STATUS: CRITICAL - ZERO IMPLEMENTATION (0% COMPLETE)

**QA Rating: 1/10**

**Current State:**
- ❌ Main file `multimodal_service.rs` does NOT exist
- ❌ No module exports added to `mod.rs`
- ❌ Zero code implemented (0/26 requirements complete)
- ✅ Prerequisites verified: ClipVisionProvider, EmbeddingModel trait, paraphym_simd::cosine_similarity

**Objective:** Create MultimodalEmbeddingService to bridge ClipVisionProvider (async vision) with EmbeddingModel trait (sync text) for cross-modal similarity search.

---

## CRITICAL REQUIREMENTS

### Architecture Constraints
- **DO NOT modify** existing EmbeddingModel trait or ClipVisionProvider
- **CREATE NEW** wrapper service: MultimodalEmbeddingService
- Uses `Arc<dyn EmbeddingModel>` for text, `Arc<ClipVisionProvider>` for vision
- Tensor→Vec conversion: `tensor.flatten_all()?.to_vec1::<f32>()?`
- **MUST USE** `paraphym_simd::cosine_similarity` (already exists, DO NOT reimplement)

---

## IMPLEMENTATION TASKS

### TASK 1: Create Core Service File ❌
**File:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/vector/multimodal_service.rs` (MISSING)

```rust
use std::sync::Arc;
use candle_core::Tensor;
use paraphym_simd::cosine_similarity;
use crate::memory::vector::embedding_model::EmbeddingModel;
use crate::providers::ClipVisionProvider;

pub struct MultimodalEmbeddingService {
    text_model: Arc<dyn EmbeddingModel>,
    vision_provider: Arc<ClipVisionProvider>,
}

impl MultimodalEmbeddingService {
    pub fn new(
        text_model: Arc<dyn EmbeddingModel>,
        vision_provider: Arc<ClipVisionProvider>,
    ) -> Self {
        Self { text_model, vision_provider }
    }
    
    pub fn text_embedding_dimension(&self) -> usize {
        self.text_model.embedding_dimension()
    }
    
    pub fn vision_embedding_dimension(&self) -> usize {
        512  // ViT-Base-Patch32 default
    }
}
```

### TASK 2: Text Embedding Methods ❌

```rust
impl MultimodalEmbeddingService {
    pub fn embed_text(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, String> {
        self.text_model
            .embed(text, task)
            .map_err(|e| format!("Text embedding failed: {}", e))
    }
    
    pub fn batch_embed_text(
        &self,
        texts: &[String],
        task: Option<String>,
    ) -> Result<Vec<Vec<f32>>, String> {
        self.text_model
            .batch_embed(texts, task)
            .map_err(|e| format!("Batch text embedding failed: {}", e))
    }
}
```

### TASK 3: Image Embedding Methods with Tensor Conversion ❌

**CRITICAL:** Use `flatten_all()` → `to_vec1::<f32>()` for Tensor→Vec conversion

```rust
impl MultimodalEmbeddingService {
    pub async fn embed_image(&self, image_path: &str) -> Result<Vec<f32>, String> {
        let tensor = self.vision_provider.encode_image(image_path).await?;
        tensor
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| format!("Failed to convert image tensor: {}", e))
    }
    
    pub async fn embed_image_url(&self, url: &str) -> Result<Vec<f32>, String> {
        let tensor = self.vision_provider.encode_url(url).await?;
        tensor.flatten_all().and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| format!("Failed to convert image tensor: {}", e))
    }
    
    pub async fn embed_image_base64(&self, base64_data: &str) -> Result<Vec<f32>, String> {
        let tensor = self.vision_provider.encode_base64(base64_data).await?;
        tensor.flatten_all().and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| format!("Failed to convert image tensor: {}", e))
    }
    
    pub async fn batch_embed_images(&self, image_paths: Vec<&str>) -> Result<Vec<Vec<f32>>, String> {
        let batch_tensor = self.vision_provider.encode_batch(image_paths).await?;
        let batch_size = batch_tensor.dim(0)
            .map_err(|e| format!("Failed to get batch size: {}", e))?;
        
        let mut embeddings = Vec::with_capacity(batch_size);
        for i in 0..batch_size {
            let row = batch_tensor
                .get(i)
                .and_then(|t| t.flatten_all())
                .and_then(|t| t.to_vec1::<f32>())
                .map_err(|e| format!("Failed to extract embedding {}: {}", i, e))?;
            embeddings.push(row);
        }
        Ok(embeddings)
    }
}
```

### TASK 4: Cross-Modal Similarity Methods ❌

**CRITICAL:** Use existing `paraphym_simd::cosine_similarity` - DO NOT reimplement

```rust
impl MultimodalEmbeddingService {
    pub async fn text_image_similarity(
        &self,
        text: &str,
        image_path: &str,
    ) -> Result<f32, String> {
        let text_emb = self.embed_text(text, None)?;
        let img_emb = self.embed_image(image_path).await?;
        
        if text_emb.len() != img_emb.len() {
            return Err(format!(
                "Dimension mismatch: text={}, image={}",
                text_emb.len(), img_emb.len()
            ));
        }
        
        Ok(cosine_similarity(&text_emb, &img_emb))
    }
    
    pub async fn image_text_similarity(
        &self,
        image_path: &str,
        text: &str,
    ) -> Result<f32, String> {
        self.text_image_similarity(text, image_path).await
    }
    
    pub async fn text_image_url_similarity(
        &self,
        text: &str,
        image_url: &str,
    ) -> Result<f32, String> {
        let text_emb = self.embed_text(text, None)?;
        let img_emb = self.embed_image_url(image_url).await?;
        
        if text_emb.len() != img_emb.len() {
            return Err(format!("Dimension mismatch: text={}, image={}", 
                text_emb.len(), img_emb.len()));
        }
        
        Ok(cosine_similarity(&text_emb, &img_emb))
    }
    
    pub async fn batch_text_image_similarity(
        &self,
        texts: &[String],
        image_paths: Vec<&str>,
    ) -> Result<Vec<f32>, String> {
        if texts.len() != image_paths.len() {
            return Err(format!("Batch size mismatch: texts={}, images={}", 
                texts.len(), image_paths.len()));
        }
        
        let text_embs = self.batch_embed_text(texts, None)?;
        let img_embs = self.batch_embed_images(image_paths).await?;
        
        let mut similarities = Vec::with_capacity(texts.len());
        for (text_emb, img_emb) in text_embs.iter().zip(img_embs.iter()) {
            if text_emb.len() != img_emb.len() {
                return Err(format!("Dimension mismatch in batch: text={}, image={}", 
                    text_emb.len(), img_emb.len()));
            }
            similarities.push(cosine_similarity(text_emb, img_emb));
        }
        Ok(similarities)
    }
}
```

### TASK 5: Module Export Integration ❌

**File:** `/Volumes/samsung_t9/paraphym/packages/candle/src/memory/vector/mod.rs`

Add to module exports section:

```rust
// Multimodal embedding service (text + vision)
pub mod multimodal_service;
pub use multimodal_service::MultimodalEmbeddingService;
```

---

## DEFINITION OF DONE (0/26 Complete)

### File Creation (0/1)
- ❌ Create `/packages/candle/src/memory/vector/multimodal_service.rs`

### Struct & Constructor (0/4)
- ❌ MultimodalEmbeddingService struct with Arc fields
- ❌ new() constructor
- ❌ text_embedding_dimension() method
- ❌ vision_embedding_dimension() method

### Text Methods (0/2)
- ❌ embed_text() delegates to text_model.embed()
- ❌ batch_embed_text() delegates to text_model.batch_embed()

### Image Methods (0/4)
- ❌ embed_image() with Tensor conversion
- ❌ embed_image_url() with Tensor conversion
- ❌ embed_image_base64() with Tensor conversion
- ❌ batch_embed_images() with batch Tensor conversion

### Similarity Methods (0/5)
- ❌ Import paraphym_simd::cosine_similarity
- ❌ text_image_similarity()
- ❌ image_text_similarity() alias
- ❌ text_image_url_similarity()
- ❌ batch_text_image_similarity()

### Module Integration (0/2)
- ❌ Add pub mod multimodal_service to mod.rs
- ❌ Add pub use MultimodalEmbeddingService to mod.rs

### Compilation (0/8)
- ❌ multimodal_service.rs compiles
- ❌ mod.rs compiles with new exports
- ❌ All imports resolve
- ❌ No clippy warnings
- ❌ Tensor conversions correct
- ❌ Error handling complete
- ❌ Async/sync boundaries correct
- ❌ Arc usage correct

---

## KEY IMPLEMENTATION NOTES

1. **Tensor Conversion Pattern** (CRITICAL):
   ```rust
   let vec: Vec<f32> = tensor.flatten_all()?.to_vec1::<f32>()?;
   ```

2. **Similarity Function** (CRITICAL):
   - Use `paraphym_simd::cosine_similarity(&a, &b)`
   - DO NOT reimplement - already optimized with SIMD

3. **Error Handling**:
   - Dimension mismatches: Clear error messages
   - Tensor conversion: Descriptive failures
   - Batch operations: Index-specific errors

4. **Async Boundaries**:
   - Text methods: sync (EmbeddingModel trait)
   - Image methods: async (ClipVisionProvider)
   - Similarity methods: async (combines both)

---

## PREREQUISITES (Verified ✅)
- ✅ ClipVisionProvider: `/packages/candle/src/providers/clip_vision.rs`
- ✅ EmbeddingModel trait: `/packages/candle/src/memory/vector/embedding_model.rs`
- ✅ cosine_similarity: `paraphym_simd` crate
- ✅ EmbeddingModelFactory: Creates Arc<dyn EmbeddingModel> instances
