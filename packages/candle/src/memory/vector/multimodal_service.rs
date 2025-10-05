//! Multimodal embedding service bridging text and vision embeddings
//!
//! This module provides a unified interface for both text and image embeddings,
//! enabling cross-modal similarity operations between text and images.

use std::sync::Arc;

use paraphym_simd::cosine_similarity;
use crate::memory::vector::embedding_model::EmbeddingModel;
use crate::capability::vision::ClipVisionProvider;

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
        self.text_model.dimension()
    }
    
    /// Get vision embedding dimension (512 for ViT-Base, 768 for ViT-Large)
    pub fn vision_embedding_dimension(&self) -> usize {
        // ClipVisionProvider doesn't expose config publicly, so we use default
        // For ViT-Base-Patch32 (default configuration)
        512
    }

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
                "Dimension mismatch: text={}, image={}",
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
                "Dimension mismatch: text={}, image={}",
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
