//! Quantum-inspired router for local cognitive operations

use thiserror::Error;

use super::types::{RoutingStrategy, EnhancedQuery, RoutingDecision};

/// Quantum router error types
#[derive(Error, Debug)]
pub enum QuantumRouterError {
    #[error("Superposition error: {0}")]
    SuperpositionError(String),
    
    #[error("Entanglement error: {0}")]
    EntanglementError(String),
    
    #[error("Measurement error: {0}")]
    MeasurementError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Local quantum router
#[derive(Debug, Clone)]
pub struct QuantumRouter {
    pub routing_strategy: RoutingStrategy,
    pub coherence_threshold: f64,
}

impl QuantumRouter {
    pub fn new(strategy: RoutingStrategy) -> Self {
        Self {
            routing_strategy: strategy,
            coherence_threshold: 0.7,
        }
    }
    
    pub async fn route(&self, query: EnhancedQuery) -> Result<RoutingDecision, QuantumRouterError> {
        // Analyze query characteristics to determine optimal strategy
        let query_text = query.query.to_lowercase();
        
        // Determine strategy based on query patterns
        let (strategy, confidence, reasoning) = if query_text.contains("recent") 
            || query_text.contains("latest") 
            || query_text.contains("today")
            || query_text.contains("yesterday")
            || query_text.contains("last week") {
            // Temporal queries - use Causal strategy for time-ordered results
            (
                RoutingStrategy::Causal,
                0.9,
                "Query contains temporal keywords - using time-ordered search"
            )
        } else if query_text.contains("similar to")
            || query_text.contains("like")
            || query_text.contains("related")
            || query_text.contains("about") {
            // Semantic similarity queries - use Quantum (vector) strategy
            (
                RoutingStrategy::Quantum,
                0.85,
                "Query seeks semantic similarity - using vector search"
            )
        } else if query_text.contains("*")
            || query_text.contains("?")
            || query_text.contains("%")
            || query_text.starts_with("pattern:") {
            // Pattern matching queries - use Emergent strategy
            (
                RoutingStrategy::Emergent,
                0.8,
                "Query contains wildcards or pattern indicators"
            )
        } else if query_text.split_whitespace().count() <= 3 {
            // Short keyword queries - use Attention (content) strategy
            (
                RoutingStrategy::Attention,
                0.75,
                "Short query - using keyword-based content search"
            )
        } else {
            // Default: analyze query complexity for hybrid approach
            let word_count = query_text.split_whitespace().count();
            
            if word_count > 10 {
                // Complex queries benefit from multiple strategies
                (
                    RoutingStrategy::Hybrid(vec![
                        RoutingStrategy::Quantum,
                        RoutingStrategy::Attention
                    ]),
                    0.7,
                    "Complex query - using hybrid approach"
                )
            } else {
                // Medium-length queries: semantic search works well
                (
                    RoutingStrategy::Quantum,
                    0.8,
                    "Standard query - using semantic vector search"
                )
            }
        };
        
        // Check if override strategy was requested in enhanced query
        let final_strategy = match query.routing_strategy {
            RoutingStrategy::Hybrid(ref strategies) if !strategies.is_empty() => {
                // If caller specified preferred strategies, respect them
                // but still return our confidence assessment
                query.routing_strategy
            }
            _ => strategy
        };
        
        Ok(RoutingDecision {
            strategy: final_strategy,
            confidence,
            reasoning: reasoning.to_string(),
        })
    }
}

impl Default for QuantumRouter {
    fn default() -> Self {
        Self::new(RoutingStrategy::Attention)
    }
}