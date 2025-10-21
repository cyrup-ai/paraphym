//! Decay worker implementation
//!
//! Implements continuous batch processing:
//! 1. Wake every N seconds
//! 2. Query batch of memories using cursor pagination
//! 3. Apply temporal decay to memory nodes (importance + quantum coherence)
//! 4. Query and decay associated entanglement/causal edges
//! 5. Persist changes to SurrealDB
//! 6. Repeat with next batch

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use chrono::Utc;
use futures_util::StreamExt;

use crate::memory::core::manager::coordinator::MemoryCoordinator;
use crate::memory::core::manager::surreal::trait_def::MemoryManager;
use crate::memory::utils::Result;

use super::config::DecayWorkerConfig;

/// Background worker for temporal decay processing
#[derive(Debug)]
pub struct DecayWorker {
    coordinator: Arc<MemoryCoordinator>,
    config: DecayWorkerConfig,
    cursor: Arc<AtomicUsize>,
}

impl DecayWorker {
    /// Create new decay worker
    pub fn new(coordinator: Arc<MemoryCoordinator>, config: DecayWorkerConfig) -> Self {
        Self {
            coordinator,
            config,
            cursor: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Run the decay worker loop
    pub async fn run(self) {
        let cycle_interval = Duration::from_secs(self.config.cycle_interval_secs);

        loop {
            // Sleep first to allow system to stabilize on startup
            tokio::time::sleep(cycle_interval).await;

            log::debug!("Decay worker cycle starting");

            // Process one batch
            match self.process_batch().await {
                Ok(processed_count) => {
                    log::debug!("Decay worker processed {} memories", processed_count);
                }
                Err(e) => {
                    log::error!("Decay worker batch processing failed: {}", e);
                }
            }
        }
    }

    /// Process a single batch of memories
    async fn process_batch(&self) -> Result<usize> {
        let offset = self.cursor.load(Ordering::Relaxed);
        let limit = self.config.batch_size;

        // Query batch of memories using pagination
        let memory_stream = self.coordinator.surreal_manager.list_all_memories(limit, offset);

        let memories: Vec<_> = memory_stream.collect().await;

        if memories.is_empty() {
            // Reached end of memories, reset cursor
            log::debug!("Decay worker reached end, resetting cursor");
            self.cursor.store(0, Ordering::Relaxed);
            return Ok(0);
        }

        let mut processed_count = 0;

        for memory_result in memories {
            match memory_result {
                Ok(memory_node) => {
                    // Check minimum age requirement
                    let age = Utc::now().signed_duration_since(memory_node.created_at);
                    let hours_old = age.num_seconds() as f64 / 3600.0;

                    if hours_old < self.config.min_age_hours as f64 {
                        // Skip memories that are too fresh
                        continue;
                    }

                    // Process this memory
                    if let Err(e) = self.process_memory(&memory_node).await {
                        log::warn!("Failed to process memory {}: {}", memory_node.id, e);
                    } else {
                        processed_count += 1;
                    }
                }
                Err(e) => {
                    log::warn!("Failed to retrieve memory from batch: {}", e);
                }
            }
        }

        // Advance cursor for next batch
        self.cursor.fetch_add(limit, Ordering::Relaxed);

        Ok(processed_count)
    }

    /// Process a single memory: apply decay to node and edges
    async fn process_memory(&self, memory_node: &crate::memory::core::primitives::node::MemoryNode) -> Result<()> {
        // Step 1: Apply temporal decay to memory node
        let mut domain_memory = self.coordinator.convert_memory_to_domain_node(memory_node)?;

        self.coordinator.apply_temporal_decay(&mut domain_memory).await?;

        // Convert back and persist
        let updated_memory = self.coordinator.convert_domain_to_memory_node(&domain_memory);
        self.coordinator.surreal_manager.update_memory(updated_memory).await?;

        // Step 2: Apply decay to entanglement edges
        self.decay_entanglement_edges(&memory_node.id).await?;

        // Step 3: Apply decay to causal edges
        self.decay_causal_edges(&memory_node.id).await?;

        Ok(())
    }

    /// Apply temporal decay to entanglement edge strengths
    async fn decay_entanglement_edges(&self, memory_id: &str) -> Result<()> {
        use surrealdb::sql::Thing;

        // Query all entanglement edges for this memory
        let query = format!(
            "SELECT id, in, out, strength, created_at FROM {} WHERE in = {} OR out = {}",
            "entangled", memory_id, memory_id
        );

        let db = &self.coordinator.surreal_manager.db;
        let mut response = db
            .query(&query)
            .await
            .map_err(|e| crate::memory::utils::Error::Database(format!("{:?}", e)))?;

        #[derive(serde::Deserialize)]
        struct EntanglementEdge {
            id: Thing,
            strength: f32,
            created_at: u64,
        }

        let edges: Vec<EntanglementEdge> = response.take(0).unwrap_or_default();

        // Apply exponential decay to each edge
        let now = Utc::now();
        let decay_rate = self.coordinator.get_decay_rate();

        for edge in edges {
            let created_time = chrono::DateTime::<Utc>::from_timestamp_millis(edge.created_at as i64)
                .unwrap_or_else(|| Utc::now());

            let age = now.signed_duration_since(created_time);
            let days_old = age.num_seconds() as f64 / 86400.0;

            // Calculate decayed strength: strength * e^(-decay_rate * days)
            let decay_factor = (-decay_rate * days_old).exp();
            let new_strength = (edge.strength as f64 * decay_factor).max(0.01) as f32;

            // Update edge in database
            let update_query = format!("UPDATE {} SET strength = $strength", edge.id);

            if let Err(e) = db
                .query(&update_query)
                .bind(("strength", new_strength))
                .await
            {
                log::warn!("Failed to update entanglement edge {:?}: {:?}", edge.id, e);
            }
        }

        Ok(())
    }

    /// Apply temporal decay to causal edge strengths
    async fn decay_causal_edges(&self, memory_id: &str) -> Result<()> {
        use surrealdb::sql::Thing;

        // Query all causal edges for this memory
        let query = format!(
            "SELECT id, in, out, strength, created_at FROM {} WHERE in = {} OR out = {}",
            "caused", memory_id, memory_id
        );

        let db = &self.coordinator.surreal_manager.db;
        let mut response = db
            .query(&query)
            .await
            .map_err(|e| crate::memory::utils::Error::Database(format!("{:?}", e)))?;

        #[derive(serde::Deserialize)]
        struct CausalEdge {
            id: Thing,
            strength: f32,
            created_at: u64,
        }

        let edges: Vec<CausalEdge> = response.take(0).unwrap_or_default();

        // Apply exponential decay to each edge
        let now = Utc::now();
        let decay_rate = self.coordinator.get_decay_rate();

        for edge in edges {
            let created_time = chrono::DateTime::<Utc>::from_timestamp_millis(edge.created_at as i64)
                .unwrap_or_else(|| Utc::now());

            let age = now.signed_duration_since(created_time);
            let days_old = age.num_seconds() as f64 / 86400.0;

            // Calculate decayed strength: strength * e^(-decay_rate * days)
            let decay_factor = (-decay_rate * days_old).exp();
            let new_strength = (edge.strength as f64 * decay_factor).max(0.01) as f32;

            // Update edge in database
            let update_query = format!("UPDATE {} SET strength = $strength", edge.id);

            if let Err(e) = db
                .query(&update_query)
                .bind(("strength", new_strength))
                .await
            {
                log::warn!("Failed to update causal edge {:?}: {:?}", edge.id, e);
            }
        }

        Ok(())
    }
}
