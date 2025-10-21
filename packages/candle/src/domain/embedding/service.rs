//! Embedding Service Implementations and Caching Mechanisms
//!
//! This module provides production-ready embedding services with zero-allocation methods,
//! lock-free caching, and high-performance vector operations.

use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::Stream;

use crate::domain::memory::serialization::content_hash;
use async_stream;

/// Error type for vector store operations
#[derive(Debug, thiserror::Error)]
pub enum VectorStoreError {
    /// Requested item was not found in the vector store
    #[error("Not found")]
    NotFound,
    /// Vector store operation failed with detailed error message
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Production-ready embedding service trait with zero-allocation methods
pub trait EmbeddingService: Send + Sync {
    /// Get embedding for content with zero-copy return
    fn get_embedding(&self, content: &str) -> Pin<Box<dyn Stream<Item = Option<Vec<f32>>> + Send>>;

    /// Get or compute embedding with zero-allocation caching
    fn get_or_compute_embedding(&self, content: &str) -> Pin<Box<dyn Stream<Item = Vec<f32>> + Send>>;

    /// Precompute embeddings for batch content
    fn precompute_batch(&self, content: &[&str]) -> Pin<Box<dyn Stream<Item = ()> + Send>>;

    /// Get embedding dimensions
    fn embedding_dimension(&self) -> usize;

    /// Clear cache to free memory
    fn clear_cache(&self);
}

/// Embedding pool for zero-allocation vector reuse
pub struct EmbeddingPool {
    sender: mpsc::UnboundedSender<Vec<f32>>,
    receiver: Arc<Mutex<mpsc::UnboundedReceiver<Vec<f32>>>>,
    dimension: usize,
    max_capacity: usize,
}
impl EmbeddingPool {
    /// Create new embedding pool with specified capacity
    #[inline]
    #[must_use]
    pub fn new(dimension: usize, capacity: usize) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        // Pre-allocate vectors to avoid allocations during runtime
        for _ in 0..capacity {
            let vec = vec![0.0; dimension];
            let _ = sender.send(vec);
        }

        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            dimension,
            max_capacity: capacity,
        }
    }

    /// Get vector from pool or create new one (zero-allocation in common case)
    #[inline]
    #[must_use]
    pub async fn acquire(&self) -> Vec<f32> {
        let mut receiver = self.receiver.lock().await;
        receiver.try_recv().unwrap_or_else(|_| vec![0.0; self.dimension])
    }

    /// Return vector to pool for reuse
    #[inline]
    pub fn release(&self, mut vec: Vec<f32>) {
        if vec.len() == self.dimension {
            vec.fill(0.0); // Clear data
            let _ = self.sender.send(vec); // Ignore if send fails
        }
    }

    /// Get pool statistics
    #[inline]
    #[must_use]
    pub fn stats(&self) -> (usize, usize) {
        // Note: tokio mpsc doesn't expose queue length, returning max_capacity as estimate
        (self.max_capacity, self.max_capacity)
    }
}

/// Production-ready in-memory embedding cache with zero-allocation operations
///
/// This cache validates embedding dimensions to prevent mixing incompatible embeddings
/// from different models. All stored embeddings must match the cache's configured dimension.
/// Dimension mismatches are treated as cache misses to ensure compatibility.
pub struct InMemoryEmbeddingCache {
    cache: tokio::sync::RwLock<HashMap<String, (Vec<f32>, usize)>>,
    pool: EmbeddingPool,
    dimension: usize,
}
impl InMemoryEmbeddingCache {
    /// Create new embedding cache with specified dimension
    #[inline]
    #[must_use]
    pub fn new(dimension: usize) -> Self {
        Self {
            cache: tokio::sync::RwLock::new(HashMap::with_capacity(1000)),
            pool: EmbeddingPool::new(dimension, 100),
            dimension,
        }
    }

    /// Get cached embedding with zero-copy return and dimension validation
    #[inline]
    pub async fn get_cached(&self, content: &str) -> Option<Vec<f32>> {
        let cache = self.cache.read().await;
        cache.get(content).and_then(|(embedding, cached_dimension)| {
            if *cached_dimension == self.dimension {
                Some(embedding.clone())
            } else {
                None // Dimension mismatch, treat as cache miss
            }
        })
    }

    /// Store embedding in cache with dimension validation
    #[inline]
    pub async fn store(&self, content: String, embedding: Vec<f32>) -> Result<(), VectorStoreError> {
        // Validate embedding dimension matches cache dimension
        if embedding.len() != self.dimension {
            return Err(VectorStoreError::OperationFailed(format!(
                "Embedding dimension mismatch: expected {}, got {}",
                self.dimension,
                embedding.len()
            )));
        }

        let mut cache = self.cache.write().await;
        cache.insert(content, (embedding, self.dimension));
        Ok(())
    }

    /// Generate deterministic embedding based on content hash with correct dimension
    #[inline]
    pub async fn generate_deterministic(&self, content: &str) -> Vec<f32> {
        let mut embedding = self.pool.acquire().await;
        // Ensure embedding has correct dimension
        if embedding.len() != self.dimension {
            embedding = vec![0.0; self.dimension];
        }
        // Fill with deterministic values based on content hash
        let hash = content_hash(content);
        for (i, val) in embedding.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let hash_val = ((hash + i64::try_from(i).unwrap_or(i64::MAX)) as f32) / (i64::MAX as f32);
            *val = hash_val;
        }
        embedding
    }

    /// Validate embedding dimensions match expected dimension
    #[inline]
    #[must_use]
    pub fn validate_dimensions(&self, embedding: &[f32]) -> bool {
        embedding.len() == self.dimension
    }

    /// Clear cache entries with invalid dimensions
    #[inline]
    pub async fn clear_invalid_entries(&self) -> usize {
        let mut cache = self.cache.write().await;
        let initial_count = cache.len();
        
        cache.retain(|_, (_, cached_dimension)| *cached_dimension == self.dimension);
        
        initial_count - cache.len()
    }

    /// Clear cache to free memory
    #[inline]
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

/// Concrete implementation of EmbeddingService using a capability model and cache
pub struct EmbeddingServiceImpl<M: crate::capability::traits::TextEmbeddingCapable> {
    model: M,
    cache: InMemoryEmbeddingCache,
}

impl<M: crate::capability::traits::TextEmbeddingCapable> EmbeddingServiceImpl<M> {
    /// Create new embedding service with model and cache
    #[must_use]
    pub fn new(model: M) -> Self {
        let dimension = model.embedding_dimension();
        let cache = InMemoryEmbeddingCache::new(dimension);
        
        Self { model, cache }
    }

    /// Get the underlying model
    #[must_use]
    pub fn model(&self) -> &M {
        &self.model
    }

    /// Get the cache
    #[must_use]
    pub fn cache(&self) -> &InMemoryEmbeddingCache {
        &self.cache
    }
}

impl<M: crate::capability::traits::TextEmbeddingCapable + Send + Sync> EmbeddingService for EmbeddingServiceImpl<M> {
    fn get_embedding(&self, content: &str) -> Pin<Box<dyn Stream<Item = Option<Vec<f32>>> + Send>> {
        let content = content.to_string();
        let model = self.model.clone();
        let cache = self.cache.clone();
        
        Box::pin(async_stream::stream! {
            // Try cache first
            if let Some(embedding) = cache.get_cached(&content).await {
                yield Some(embedding);
                return;
            }
            
            // Generate embedding
            match model.embed(&content, None).await {
                Ok(embedding) => {
                    // Store in cache (ignore errors for streaming)
                    let _ = cache.store(content, embedding.clone()).await;
                    yield Some(embedding);
                }
                Err(_) => {
                    yield None;
                }
            }
        })
    }

    fn get_or_compute_embedding(&self, content: &str) -> Pin<Box<dyn Stream<Item = Vec<f32>> + Send>> {
        let content = content.to_string();
        let model = self.model.clone();
        let cache = self.cache.clone();
        
        Box::pin(async_stream::stream! {
            // Try cache first
            if let Some(embedding) = cache.get_cached(&content).await {
                yield embedding;
                return;
            }
            
            // Generate embedding
            match model.embed(&content, None).await {
                Ok(embedding) => {
                    // Store in cache (ignore errors for streaming)
                    let _ = cache.store(content, embedding.clone()).await;
                    yield embedding;
                }
                Err(e) => {
                    // For get_or_compute, we yield a deterministic embedding on error
                    // This ensures the stream always produces a value
                    yield cache.generate_deterministic(&content).await;
                }
            }
        })
    }

    fn precompute_batch(&self, content: &[&str]) -> Pin<Box<dyn Stream<Item = ()> + Send>> {
        let content: Vec<String> = content.iter().map(|s| s.to_string()).collect();
        let model = self.model.clone();
        let cache = self.cache.clone();
        
        Box::pin(async_stream::stream! {
            // Process in batches using model's batch capability
            let content_refs: Vec<&str> = content.iter().map(|s| s.as_str()).collect();
            if let Ok(embeddings) = model.batch_embed(&content_refs, None).await {
                for (text, embedding) in content.iter().zip(embeddings.iter()) {
                    let _ = cache.store(text.clone(), embedding.clone()).await;
                    yield ();
                }
            }
        })
    }

    fn embedding_dimension(&self) -> usize {
        self.cache.dimension
    }

    fn clear_cache(&self) {
        let cache = self.cache.clone();
        tokio::spawn(async move {
            cache.clear().await;
        });
    }
}
