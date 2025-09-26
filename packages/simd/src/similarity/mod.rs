//! High-performance vector similarity operations with automatic SIMD acceleration
//!
//! This module provides optimized implementations of common similarity metrics
//! with automatic selection of the best available implementation at runtime.

#![allow(dead_code)] // Temporary until all implementations are added

pub mod metrics;
mod scalar;
mod simd;
mod traits;

use std::sync::Arc;

use lazy_static::lazy_static;
pub use metrics::SimilarityMetricsSnapshot;
pub use scalar::ScalarSimilarity;
pub use simd::portable::PortableSimdSimilarity;
pub use traits::{CosineSimilarity, RuntimeSelectable, SimilarityBuilder};

lazy_static! {
    /// Global instance of the best available similarity implementation
    static ref GLOBAL_SIMILARITY: Arc<dyn RuntimeSelectable> = simd::best_available();
}

/// Compute the cosine similarity between two vectors using the best available implementation
///
/// This function automatically selects the optimal implementation based on:
/// - Vector length
/// - CPU features available at runtime
/// - Cache line alignment
///
/// # Panics
/// - If input vectors have different lengths
#[inline]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(
        a.len(),
        b.len(),
        "Vectors must have the same length for cosine similarity"
    );

    // For very small vectors, use scalar implementation
    if a.len() < 8 {
        return ScalarSimilarity::new().cosine_similarity(a, b);
    }

    // Otherwise, use the best available implementation
    GLOBAL_SIMILARITY.cosine_similarity(a, b)
}

/// Smart cosine similarity that automatically selects the best implementation
///
/// This is an alias for [`cosine_similarity`] that provides automatic
/// runtime selection of the optimal implementation. The name "smart" indicates
/// the intelligent selection of SIMD vs scalar implementations based on
/// vector size and CPU capabilities.
#[inline]
pub fn smart_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    cosine_similarity(a, b)
}

/// SIMD-optimized cosine similarity (alias for compatibility)
///
/// This function is an alias for the general [`cosine_similarity`] function,
/// which already uses SIMD when appropriate. Kept for API compatibility.
#[inline]
pub fn simd_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    cosine_similarity(a, b)
}

/// Get the name of the currently active similarity implementation
#[inline]
pub fn active_implementation() -> &'static str {
    GLOBAL_SIMILARITY.name()
}

/// Get metrics about similarity operations
#[inline]
pub fn metrics() -> SimilarityMetricsSnapshot {
    GLOBAL_SIMILARITY.metrics()
}

/// Reset all similarity metrics to zero
#[inline]
pub fn reset_metrics() {
    GLOBAL_SIMILARITY.reset_metrics();
}

/// Create a new similarity builder for custom configuration
#[inline]
pub fn builder() -> SimilarityBuilder {
    SimilarityBuilder::new()
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_cosine_similarity() {
        // Test with simple vectors
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];

        let result = cosine_similarity(&a, &b);
        let expected = 0.9746318; // Precomputed value
        assert_relative_eq!(result, expected, epsilon = 1e-6);

        // Test with orthogonal vectors
        let a = [1.0, 0.0];
        let b = [0.0, 1.0];
        assert_relative_eq!(cosine_similarity(&a, &b), 0.0);

        // Test with identical vectors
        let a = [1.0, 2.0, 3.0];
        assert_relative_eq!(cosine_similarity(&a, &a), 1.0);
    }

    #[test]
    fn test_metrics() {
        reset_metrics();

        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];

        // First call
        cosine_similarity(&a, &b);
        let metrics_snapshot = metrics();
        assert_eq!(metrics_snapshot.total_calculations, 1);
        assert_eq!(metrics_snapshot.total_elements_processed, 3);

        // Second call
        cosine_similarity(&a, &b);
        let metrics_snapshot = metrics();
        assert_eq!(metrics_snapshot.total_calculations, 2);
        assert_eq!(metrics_snapshot.total_elements_processed, 6);

        // Reset and verify
        reset_metrics();
        let metrics_snapshot = metrics();
        assert_eq!(metrics_snapshot.total_calculations, 0);
        assert_eq!(metrics_snapshot.total_elements_processed, 0);
    }

    #[test]
    fn test_implementation_name() {
        // Just verify it returns something
        assert!(!active_implementation().is_empty());
    }
}
