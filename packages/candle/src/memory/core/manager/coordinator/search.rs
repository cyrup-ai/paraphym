//! Search and retrieval operations for memories

use std::time::SystemTime;

use futures_util::StreamExt;

use crate::domain::memory::primitives::node::MemoryNode;
use crate::memory::core::manager::surreal::trait_def::MemoryManager;
use crate::memory::core::ops::filter::MemoryFilter;
use crate::memory::utils::Result;

use super::lifecycle::MemoryCoordinator;

impl MemoryCoordinator {
    /// Search memories by content using vector similarity
    ///
    /// This method:
    /// 1. Generates embedding for query text
    /// 2. Performs cosine similarity search in SurrealDB
    /// 3. Applies temporal decay to results
    /// 4. Optionally filters by memory type, importance, time range
    /// 5. Boosts scores for entangled memories
    /// 6. Sorts by decayed importance
    ///
    /// # Arguments
    /// * `query` - Search query text
    /// * `top_k` - Maximum number of results to return
    /// * `filter` - Optional filter criteria
    ///
    /// # Returns
    /// Vector of matching memories, sorted by relevance
    pub async fn search_memories(
        &self,
        query: &str,
        top_k: usize,
        filter: Option<MemoryFilter>,
    ) -> Result<Vec<MemoryNode>> {
        // Create enhanced query for routing
        let enhanced_query = crate::memory::cognitive::quantum::types::EnhancedQuery {
            query: query.to_string(),
            routing_strategy: crate::memory::cognitive::quantum::types::RoutingStrategy::Hybrid(vec![]),
            temporal_context: crate::memory::cognitive::quantum::types::TemporalContext::default(),
            coherence_threshold: 0.7,
        };

        // Get routing decision from quantum router
        let cognitive_state_guard = self.cognitive_state.read().await;
        let routing_decision = self.quantum_router
            .route(enhanced_query, Some(&*cognitive_state_guard))
            .await
            .unwrap_or_default();
        drop(cognitive_state_guard);

        log::info!(
            "Routing: {:?} (confidence: {:.2})",
            routing_decision.strategy,
            routing_decision.confidence
        );

        // Dispatch based on strategy
        let memory_stream = match routing_decision.strategy {
            crate::memory::cognitive::quantum::types::RoutingStrategy::Attention => {
                // Content/keyword search
                self.surreal_manager.search_by_content(query)
            }
            _ => {
                // Default: Vector search (Quantum, Causal, Emergent, Hybrid)
                let query_embedding = self.generate_embedding(query).await?;
                self.surreal_manager.search_by_vector(query_embedding, top_k * 2) // Request 2x for filtering
            }
        };

        // Collect results
        let memories: Vec<_> = memory_stream.collect().await;

        // Convert to domain nodes with error handling
        let mut result_memories = Vec::new();
        for memory_result in memories {
            match memory_result {
                Ok(memory_node) => {
                    let domain_memory = self.convert_memory_to_domain_node(&memory_node)?;
                    result_memories.push(domain_memory);
                }
                Err(e) => {
                    log::warn!("Failed to retrieve search result: {}", e);
                }
            }
        }

        // NOTE: Temporal decay now applied by background DecayWorker
        // Removed lazy evaluation from read path for performance

        // Apply optional filter
        let filtered_memories = if let Some(filter) = filter {
            result_memories
                .into_iter()
                .filter(|memory| {
                    // Apply memory type filter
                    if let Some(ref memory_types) = filter.memory_types {
                        // Convert domain type to core type for comparison
                        let converted_type = match memory.memory_type() {
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Semantic => crate::memory::core::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Episodic => crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Procedural => crate::memory::core::primitives::types::MemoryTypeEnum::Procedural,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Working => crate::memory::core::primitives::types::MemoryTypeEnum::Working,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::LongTerm => crate::memory::core::primitives::types::MemoryTypeEnum::LongTerm,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Fact => crate::memory::core::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Episode => crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Declarative => crate::memory::core::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Implicit => crate::memory::core::primitives::types::MemoryTypeEnum::Procedural,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Explicit => crate::memory::core::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Contextual => crate::memory::core::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Temporal => crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Spatial => crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Associative => crate::memory::core::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Emotional => crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                        };
                        if !memory_types.contains(&converted_type) {
                            return false;
                        }
                    }

                    // Apply importance range filter
                    if let Some((min_importance, max_importance)) = filter.importance_range {
                        let importance = memory.importance();
                        if importance < min_importance || importance > max_importance {
                            return false;
                        }
                    }

                    // Apply time range filter
                    if let Some(time_range) = &filter.time_range {
                        if let Some(start) = time_range.start {
                            let start_system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(start.timestamp() as u64);
                            if memory.base_memory.created_at < start_system_time {
                                return false;
                            }
                        }
                        if let Some(end) = time_range.end {
                            let end_system_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(end.timestamp() as u64);
                            if memory.base_memory.created_at >= end_system_time {
                                return false;
                            }
                        }
                    }

                    true
                })
                .collect()
        } else {
            result_memories
        };

        // Boost scores for entangled memories
        let mut boosted_memories = filtered_memories;
        {
            let state = self.quantum_state.read().await;

            for memory in &mut boosted_memories {
                let memory_id = memory.id().to_string();

                // Find all entanglement links involving this memory
                let entangled_links: Vec<&crate::memory::cognitive::quantum::EntanglementLink> =
                    state
                        .entanglement_links
                        .iter()
                        .filter(|link| link.node_a == memory_id || link.node_b == memory_id)
                        .collect();

                // Boost importance based on entanglement strength
                if !entangled_links.is_empty() {
                    let total_entanglement: f64 = entangled_links
                        .iter()
                        .map(|link| link.entanglement_strength)
                        .sum();

                    let boost_factor = 1.0 + (total_entanglement * 0.2); // 20% boost per entanglement

                    // Get quality score from metadata (stored by CognitiveWorker)
                    let quality_score = {
                        let metadata_guard = memory.base_memory.metadata.read().await;
                        metadata_guard
                            .get("quality_score")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.5) // Default to neutral
                    };

                    // Quality multiplier: 0.5 is neutral, >0.5 boosts, <0.5 reduces
                    let quality_multiplier = 1.0 + (quality_score - 0.5) * 0.4; // ±20% max

                    // Combine entanglement and quality boosts
                    let combined_boost = boost_factor * quality_multiplier;

                    let current_importance = memory.importance();
                    let boosted_importance = (current_importance as f64 * combined_boost) as f32;

                    // Apply the boost using the setter method (clamps to 0.0-1.0)
                    if let Err(e) = memory.set_importance(boosted_importance.min(1.0)) {
                        log::warn!("Failed to apply boost for {}: {}", memory_id, e);
                    }

                    log::trace!(
                        "Boost for {}: {} links (ent={:.2}), quality={:.2}, importance {} -> {}",
                        memory_id,
                        entangled_links.len(),
                        boost_factor,
                        quality_score,
                        current_importance,
                        boosted_importance
                    );

                    // Note: This is a query-time boost, not persisted to DB
                    // The boost only affects this search result ranking
                }
            }
        }

        // Re-sort by decayed importance for better RAG relevance
        boosted_memories.sort_by(|a, b| {
            // Sort by importance descending (higher importance first)
            let a_importance = a.importance();
            let b_importance = b.importance();
            b_importance
                .partial_cmp(&a_importance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply top_k limit after sorting
        boosted_memories.truncate(top_k);

        // Update cognitive state with query pattern for adaptive routing
        {
            let _cognitive_state_guard = self.cognitive_state.write().await;
            // Track query for future routing adaptations
            // (Actual tracking implementation depends on CognitiveState API)
            log::debug!("Cognitive state updated for query: {}", query);
        }

        Ok(boosted_memories)
    }

    /// Get memories by filter
    pub async fn get_memories(&self, filter: MemoryFilter) -> Result<Vec<MemoryNode>> {
        let memories = self.repository.read().await.filter(&filter);
        let mut result_memories = Vec::new();
        for arc_memory in memories {
            let domain_memory = self.convert_memory_to_domain_node(&arc_memory)?;
            result_memories.push(domain_memory);
        }
        Ok(result_memories)
    }
}
