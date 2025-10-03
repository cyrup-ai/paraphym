# VISION_2: Multimodal Embedding Service Integration

## OBJECTIVE

Create a unified multimodal embedding service that bridges ClipVisionProvider (vision) and EmbeddingModel trait (text), enabling cross-modal similarity search between text and images.

---

## BACKGROUND

**Current Architecture (ACTUAL)**:
- ✅ **ClipVisionProvider** exists in `packages/candle/src/providers/clip_vision.rs` (from VISION_1)
  - Returns: `Tensor` (async methods)
  - Methods: `encode_image()`, `encode_url()`, `encode_base64()`, `encode_batch()`
  
- ✅ **EmbeddingModel trait** exists in `packages/candle/src/memory/vector/embedding_model.rs`
  - Returns: `Vec<f32>` (sync methods)
  - Methods: `embed()`, `batch_embed()`, `embedding_dimension()`
  - Implementations: BERT, Stella, GTE-Qwen, Jina-BERT, NVEmbed
  
- ✅ **EmbeddingModelFactory** creates `Arc<dyn EmbeddingModel>` instances
  - File: `packages/candle/src/memory/vector/embedding_factory.rs`
  - Pattern: `EmbeddingModelFactory::create_from_model_info()`

- ✅ **cosine_similarity** already exists in `paraphym_simd` package
  - No need to reimplement similarity functions

**Architecture Mismatch**:
- EmbeddingModel trait: **sync**, text-based, returns `Vec<f32>`
- ClipVisionProvider: **async**, vision-based, returns `Tensor`
- **Cannot directly unify** → Need wrapper service

**What This Task Accomplishes**:
- Creates `MultimodalEmbeddingService` to bridge text and vision systems
- Wraps `Arc<dyn EmbeddingModel>` and `Arc<ClipVisionProvider>`
- Provides unified API for text embeddings, image embeddings, and cross-modal similarity
- Maintains backward compatibility (no modifications to existing systems)

---

## SUBTASK 1: Create MultimodalEmbeddingService Struct

**File to CREATE**: `packages/candle/src/memory/vector/multimodal_service.rs`

**What to Write**:

```rust
use std::sync::Arc;
use candle_core::Tensor;
use crate::memory::vector::embedding_model::EmbeddingModel;
use crate::providers::ClipVisionProvider;

/// Multimodal embedding service bridging text and vision embeddings
///
/// Wraps both a text embedding model (EmbeddingModel trait) and a vision
/// provider (ClipVisionProvider) to enable unified multimodal operations.
pub struct MultimodalEmbeddingService {
    /// Text embedding model (sync, trait-based)
    text_model: Arc<dyn EmbeddingModel>,
    
    /// Vision embedding provider (async, ClipVisionProvider)
    vision_provider: Arc<ClipVisionProvider>,
}

impl MultimodalEmbeddingService {
    /// Create new multimodal service with text and vision providers
    pub fn new(
        text_model: Arc<dyn EmbeddingModel>,
        vision_provider: Arc<ClipVisionProvider>,
    ) -> Self {
        Self {
            text_model,
            vision_provider,
        }
    }
    
    /// Get text embedding dimension
    pub fn text_embedding_dimension(&self) -> usize {
        self.text_model.embedding_dimension()
    }
    
    /// Get vision embedding dimension (512 for ViT-Base, 768 for ViT-Large)
    pub fn vision_embedding_dimension(&self) -> usize {
        // ClipVisionProvider doesn't expose config publicly, so we infer from first embedding
        // For now, return common CLIP dimensions (512 or 768)
        // TODO: Make ClipVisionProvider expose embedding_dimension() method
        512  // Default for ViT-Base-Patch32
    }
}
```

**Why**: Creates the foundational wrapper service that holds both text and vision providers.

**Definition of Done**:
- ✅ File created at correct path
- ✅ Struct holds Arc<dyn EmbeddingModel> and Arc<ClipVisionProvider>
- ✅ Constructor `new()` method implemented
- ✅ Helper methods for embedding dimensions
- ✅ Documentation comments explain purpose

---

## SUBTASK 2: Implement Text Embedding Methods

**File**: `packages/candle/src/memory/vector/multimodal_service.rs` (continue editing)

**What to Add** (in `impl MultimodalEmbeddingService` block):

```rust
impl MultimodalEmbeddingService {
    /// Embed text using the text model
    /// 
    /// Delegates to the wrapped EmbeddingModel trait implementation.
    pub fn embed_text(&self, text: &str, task: Option<String>) -> Result<Vec<f32>, String> {
        self.text_model
            .embed(text, task)
            .map_err(|e| format!("Text embedding failed: {}", e))
    }
    
    /// Batch embed multiple texts
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

**Why**: Provides text embedding interface by delegating to the existing EmbeddingModel trait.

**Definition of Done**:
- ✅ `embed_text()` method delegates to `text_model.embed()`
- ✅ `batch_embed_text()` method delegates to `text_model.batch_embed()`
- ✅ Error messages are descriptive
- ✅ Methods compile without errors

---

## SUBTASK 3: Implement Image Embedding Methods with Tensor Conversion

**File**: `packages/candle/src/memory/vector/multimodal_service.rs` (continue editing)

**What to Add** (in `impl MultimodalEmbeddingService` block):

```rust
impl MultimodalEmbeddingService {
    /// Embed image from file path
    /// 
    /// Uses ClipVisionProvider to encode image, then converts Tensor to Vec<f32>
    /// for compatibility with text embeddings and similarity functions.
    pub async fn embed_image(&self, image_path: &str) -> Result<Vec<f32>, String> {
        // Get tensor from vision provider (async)
        let tensor = self.vision_provider.encode_image(image_path).await?;
        
        // Convert Tensor to Vec<f32>
        // CRITICAL: Use flatten_all() → to_vec1::<f32>() pattern
        tensor
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| format!("Failed to convert image tensor to vector: {}", e))
    }
    
    /// Embed image from URL
    pub async fn embed_image_url(&self, url: &str) -> Result<Vec<f32>, String> {
        let tensor = self.vision_provider.encode_url(url).await?;
        tensor
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| format!("Failed to convert image tensor to vector: {}", e))
    }
    
    /// Embed image from base64 data (for API usage)
    pub async fn embed_image_base64(&self, base64_data: &str) -> Result<Vec<f32>, String> {
        let tensor = self.vision_provider.encode_base64(base64_data).await?;
        tensor
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
            .map_err(|e| format!("Failed to convert image tensor to vector: {}", e))
    }
    
    /// Batch embed multiple images from file paths
    pub async fn batch_embed_images(&self, image_paths: Vec<&str>) -> Result<Vec<Vec<f32>>, String> {
        // Use ClipVisionProvider's batch encoding for efficiency
        let batch_tensor = self.vision_provider.encode_batch(image_paths).await?;
        
        // Convert batch tensor (N, D) to Vec<Vec<f32>>
        let embedding_dim = batch_tensor.dim(1)
            .map_err(|e| format!("Failed to get embedding dimension: {}", e))?;
        
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

**Tensor to Vec Conversion Pattern**:
```rust
// ALWAYS use this pattern for Tensor → Vec<f32>:
let vec: Vec<f32> = tensor
    .flatten_all()?           // Flatten to 1D tensor
    .to_vec1::<f32>()?;       // Convert to Vec<f32>
```

**Why**: Enables image embedding with proper tensor-to-vector conversion for compatibility with similarity functions.

**Definition of Done**:
- ✅ `embed_image()` method implemented (async)
- ✅ `embed_image_url()` method implemented (async)
- ✅ `embed_image_base64()` method implemented (async)
- ✅ `batch_embed_images()` method implemented (async)
- ✅ Tensor conversion uses correct `flatten_all()` → `to_vec1::<f32>()` pattern
- ✅ Error handling for tensor operations
- ✅ Methods compile without errors

---

## SUBTASK 4: Implement Cross-Modal Similarity Methods

**File**: `packages/candle/src/memory/vector/multimodal_service.rs` (continue editing)

**What to Add**:

**First, add the import at top of file**:
```rust
use paraphym_simd::cosine_similarity;  // Already exists in paraphym_simd!
```

**Then add methods** (in `impl MultimodalEmbeddingService` block):

```rust
impl MultimodalEmbeddingService {
    /// Compute cross-modal similarity between text and image
    /// 
    /// Embeds text using text model and image using vision provider,
    /// then computes cosine similarity using paraphym_simd::cosine_similarity.
    pub async fn text_image_similarity(
        &self,
        text: &str,
        image_path: &str,
    ) -> Result<f32, String> {
        // Embed text (sync)
        let text_emb = self.embed_text(text, None)?;
        
        // Embed image (async)
        let img_emb = self.embed_image(image_path).await?;
        
        // Dimension check
        if text_emb.len() != img_emb.len() {
            return Err(format!(
                "Embedding dimension mismatch: text={}, image={}. Use models with matching dimensions.",
                text_emb.len(),
                img_emb.len()
            ));
        }
        
        // Compute cosine similarity using paraphym_simd (SIMD-optimized)
        Ok(cosine_similarity(&text_emb, &img_emb))
    }
    
    /// Compute image-text similarity (alias for symmetry)
    pub async fn image_text_similarity(
        &self,
        image_path: &str,
        text: &str,
    ) -> Result<f32, String> {
        self.text_image_similarity(text, image_path).await
    }
    
    /// Compute similarity between text and image URL
    pub async fn text_image_url_similarity(
        &self,
        text: &str,
        image_url: &str,
    ) -> Result<f32, String> {
        let text_emb = self.embed_text(text, None)?;
        let img_emb = self.embed_image_url(image_url).await?;
        
        if text_emb.len() != img_emb.len() {
            return Err(format!(
                "Embedding dimension mismatch: text={}, image={}",
                text_emb.len(),
                img_emb.len()
            ));
        }
        
        Ok(cosine_similarity(&text_emb, &img_emb))
    }
    
    /// Batch compute cross-modal similarities
    /// 
    /// For each text, compute similarity with corresponding image.
    /// Requires texts.len() == image_paths.len()
    pub async fn batch_text_image_similarity(
        &self,
        texts: &[String],
        image_paths: Vec<&str>,
    ) -> Result<Vec<f32>, String> {
        if texts.len() != image_paths.len() {
            return Err(format!(
                "Batch size mismatch: texts={}, images={}",
                texts.len(),
                image_paths.len()
            ));
        }
        
        // Batch embed texts
        let text_embs = self.batch_embed_text(texts, None)?;
        
        // Batch embed images
        let img_embs = self.batch_embed_images(image_paths).await?;
        
        // Compute similarities
        let mut similarities = Vec::with_capacity(texts.len());
        for (text_emb, img_emb) in text_embs.iter().zip(img_embs.iter()) {
            if text_emb.len() != img_emb.len() {
                return Err(format!(
                    "Embedding dimension mismatch in batch: text={}, image={}",
                    text_emb.len(),
                    img_emb.len()
                ));
            }
            similarities.push(cosine_similarity(text_emb, img_emb));
        }
        
        Ok(similarities)
    }
}
```

**CRITICAL**: Use `paraphym_simd::cosine_similarity` (already exists, SIMD-optimized).

**Why**: Enables cross-modal similarity search using existing high-performance similarity function.

**Definition of Done**:
- ✅ Import `paraphym_simd::cosine_similarity` at top of file
- ✅ `text_image_similarity()` method implemented
- ✅ `image_text_similarity()` alias method added
- ✅ `text_image_url_similarity()` method for URLs
- ✅ `batch_text_image_similarity()` for batch operations
- ✅ Dimension mismatch detection and clear errors
- ✅ Uses existing `paraphym_simd::cosine_similarity` (no reimplementation)
- ✅ Methods compile without errors

---

## SUBTASK 5: Export MultimodalEmbeddingService from Module

**File to MODIFY**: `packages/candle/src/memory/vector/mod.rs`

**What to Add**:

Find the module exports section and add:

```rust
// Multimodal embedding service (text + vision)
pub mod multimodal_service;
pub use multimodal_service::MultimodalEmbeddingService;
```

**Why**: Makes MultimodalEmbeddingService publicly accessible from memory::vector module.

**Definition of Done**:
- ✅ `pub mod multimodal_service;` added
- ✅ `pub use multimodal_service::MultimodalEmbeddingService;` added
- ✅ File compiles without errors
- ✅ Service accessible via `use crate::memory::vector::MultimodalEmbeddingService;`

---

## RESEARCH REFERENCES

### ClipVisionProvider (VISION_1)
- **File**: `packages/candle/src/providers/clip_vision.rs`
- **Async Methods**: 
  - `encode_image(image_path: &str) -> Result<Tensor, String>`
  - `encode_url(url: &str) -> Result<Tensor, String>`
  - `encode_base64(base64_data: &str) -> Result<Tensor, String>`
  - `encode_batch(image_paths: Vec<&str>) -> Result<Tensor, String>`
- **Returns**: Candle `Tensor` (needs conversion to `Vec<f32>`)

### EmbeddingModel Trait (Existing)
- **File**: `packages/candle/src/memory/vector/embedding_model.rs`
- **Trait Definition**:
  ```rust
  pub trait EmbeddingModel: Send + Sync + std::fmt::Debug {
      fn embed(&self, text: &str, task: Option<String>) -> Result<Vec<f32>>;
      fn batch_embed(&self, texts: &[String], task: Option<String>) -> Result<Vec<Vec<f32>>>;
      fn embedding_dimension(&self) -> usize;
  }
  ```
- **Sync Methods** (no async)
- **Returns**: `Vec<f32>` (ready for similarity)

### EmbeddingModelFactory (Existing)
- **File**: `packages/candle/src/memory/vector/embedding_factory.rs`
- **Pattern**: Creates `Arc<dyn EmbeddingModel>` instances
- **Supported Models**: BERT, Stella, GTE-Qwen, Jina-BERT, NVEmbed
- **Usage**: `EmbeddingModelFactory::create_from_model_info(model_info)`

### Similarity Function (Existing)
- **Package**: `paraphym_simd`
- **Function**: `cosine_similarity(a: &[f32], b: &[f32]) -> f32`
- **SIMD-optimized**: Uses platform-specific SIMD instructions
- **DO NOT REIMPLEMENT**: Just import and use

### Tensor Conversion Pattern
```rust
// ALWAYS use this pattern for Tensor → Vec<f32>:
let vec: Vec<f32> = tensor
    .flatten_all()?           // Flatten tensor to 1D
    .to_vec1::<f32>()?;       // Convert to Vec<f32>
```

---

## CRITICAL REQUIREMENTS

### ✅ Architecture Correctness
- **DO NOT modify** existing EmbeddingModel trait or implementations
- **DO NOT modify** existing ClipVisionProvider
- **CREATE NEW** MultimodalEmbeddingService as wrapper/bridge
- Use `Arc<dyn EmbeddingModel>` for text model (matches factory pattern)
- Use `Arc<ClipVisionProvider>` for vision provider

### ✅ Tensor Conversion
- Use `flatten_all()` → `to_vec1::<f32>()` pattern consistently
- Handle conversion errors with descriptive messages
- Ensure embeddings are 1D vectors for similarity

### ✅ Similarity Functions
- **MUST USE** `paraphym_simd::cosine_similarity` (already exists)
- **DO NOT** reimplement similarity functions
- Handle dimension mismatches with clear error messages
- Check for zero vectors if needed

### ✅ API Consistency
- Text methods: sync (delegate to EmbeddingModel)
- Image methods: async (delegate to ClipVisionProvider)
- Similarity methods: async (combine text + image)
- Error messages follow existing patterns

### ✅ Batch Operations
- Leverage `ClipVisionProvider::encode_batch()` for efficiency
- Leverage `EmbeddingModel::batch_embed()` for efficiency
- Handle batch dimension mismatches gracefully

---

## DEFINITION OF DONE

### File Creation
1. ✅ `/packages/candle/src/memory/vector/multimodal_service.rs` created

### Struct and Constructor
2. ✅ `MultimodalEmbeddingService` struct defined
3. ✅ Holds `Arc<dyn EmbeddingModel>` and `Arc<ClipVisionProvider>`
4. ✅ `new()` constructor implemented
5. ✅ Dimension helper methods added

### Text Embedding Methods
6. ✅ `embed_text()` method delegates to text_model
7. ✅ `batch_embed_text()` method delegates to text_model
8. ✅ Error handling for text embedding

### Image Embedding Methods
9. ✅ `embed_image()` method with Tensor conversion
10. ✅ `embed_image_url()` method with Tensor conversion
11. ✅ `embed_image_base64()` method with Tensor conversion
12. ✅ `batch_embed_images()` method with batch Tensor conversion
13. ✅ Tensor conversion uses `flatten_all()` → `to_vec1::<f32>()` pattern

### Cross-Modal Similarity
14. ✅ Import `paraphym_simd::cosine_similarity` at top
15. ✅ `text_image_similarity()` method implemented
16. ✅ `image_text_similarity()` alias method
17. ✅ `text_image_url_similarity()` for URLs
18. ✅ `batch_text_image_similarity()` for batch ops
19. ✅ Dimension mismatch detection and errors
20. ✅ Uses existing `paraphym_simd::cosine_similarity`

### Module Export
21. ✅ `pub mod multimodal_service;` added to memory/vector/mod.rs
22. ✅ `pub use multimodal_service::MultimodalEmbeddingService;` added
23. ✅ Service publicly accessible

### Compilation
24. ✅ All files compile without errors
25. ✅ No warnings from Rust analyzer
26. ✅ Imports resolve correctly

---

## NO TESTS OR BENCHMARKS

**Do NOT create**:
- Unit tests for MultimodalEmbeddingService
- Integration tests for cross-modal search
- Benchmark comparisons
- Example usage files
- Test fixtures or sample images

**Reason**: Separate team handles testing. Focus on implementation only.

---

## USAGE EXAMPLE (For Reference Only - Do Not Implement)

```rust
use std::sync::Arc;
use crate::memory::vector::{EmbeddingModelFactory, MultimodalEmbeddingService};
use crate::providers::ClipVisionProvider;
use candle_core::Device;

// Create text model using factory
let text_model = EmbeddingModelFactory::create_from_model_info(model_info)?;

// Create vision provider
let vision_provider = Arc::new(
    ClipVisionProvider::from_pretrained("path/to/clip", Device::Cpu)?
);

// Create multimodal service
let service = MultimodalEmbeddingService::new(text_model, vision_provider);

// Embed text
let text_emb = service.embed_text("a photo of a cat", None)?;

// Embed image (async)
let img_emb = service.embed_image("cat.jpg").await?;

// Cross-modal similarity (async)
let similarity = service.text_image_similarity("a photo of a cat", "cat.jpg").await?;
println!("Similarity: {}", similarity);
```

**Note**: This example is for understanding only. Do not create example files.
