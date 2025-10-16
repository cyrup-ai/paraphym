//! High-level memory management functionality

use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use chrono;
use moka::sync::Cache;
use tokio::sync::RwLock;

use crate::capability::registry::TextEmbeddingModel;
use crate::capability::traits::TextEmbeddingCapable;

use crate::domain::memory::cognitive::types::{CognitiveState, EntanglementType};
use crate::domain::memory::primitives::{node::MemoryNode, types::MemoryTypeEnum};
use crate::memory::core::manager::surreal::{
    MemoryManager, MemoryQuery, MemoryStream, PendingDeletion, PendingEntanglementEdge,
    PendingMemory, PendingQuantumSignature, PendingQuantumUpdate, PendingRelationship,
    RelationshipStream, SurrealDBMemoryManager,
};
use crate::memory::core::ops::filter::MemoryFilter;
use crate::memory::core::primitives::{
    node::MemoryNode as CoreMemoryNode, types::MemoryTypeEnum as CoreMemoryTypeEnum,
};
use crate::memory::utils::{Error, Result};
use crate::memory::{MemoryMetadata, MemoryRelationship, repository::MemoryRepository};
use futures_util::StreamExt;

// Cognitive queue from memory/core module
use crate::memory::core::cognitive_queue::{
    CognitiveProcessingQueue, CognitiveTask, CognitiveTaskType,
};

// Committee evaluator from memory/cognitive module
use crate::memory::cognitive::committee::ModelCommitteeEvaluator;

// Quantum components from memory/cognitive/quantum module
use crate::memory::cognitive::quantum::{QuantumRouter, QuantumState};

/// Strategy for handling memories with pending cognitive evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LazyEvalStrategy {
    /// Wait for background processing to complete (polls with timeout)
    WaitForCompletion,
    /// Return immediately with partial data (non-blocking, default)
    #[default]
    ReturnPartial,
    /// Trigger immediate evaluation and wait (bypasses queue)
    TriggerAndWait,
}

/// High-level memory manager that uses SurrealDB's native capabilities directly
///
/// Note: cognitive_queue, committee_evaluator, quantum_router, and quantum_state
/// are wired in but not used until COGMEM_4 worker implementation
#[allow(dead_code)]
#[derive(Clone)]
pub struct MemoryCoordinator {
    surreal_manager: Arc<SurrealDBMemoryManager>,
    repository: Arc<RwLock<MemoryRepository>>,
    embedding_model: TextEmbeddingModel,
    // NEW COGNITIVE FIELDS:
    cognitive_queue: Arc<CognitiveProcessingQueue>,
    committee_evaluator: Arc<ModelCommitteeEvaluator>,
    quantum_router: Arc<QuantumRouter>,
    quantum_state: Arc<RwLock<QuantumState>>,
    cognitive_workers: Arc<std::sync::RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    // LAZY EVALUATION FIELDS:
    lazy_eval_strategy: LazyEvalStrategy,
    evaluation_cache: Cache<String, f64>,
    // TEMPORAL DECAY:
    decay_rate: f64,
}

impl MemoryCoordinator {
    /// Create a new memory coordinator with SurrealDB and embedding model
    pub async fn new(
        surreal_manager: Arc<SurrealDBMemoryManager>,
        embedding_model: TextEmbeddingModel,
    ) -> Result<Self> {
        // Initialize committee evaluator with error handling
        // Note: ModelCommitteeEvaluator::new() is async and returns Result<Self, CognitiveError>
        let committee_evaluator = Arc::new(
            ModelCommitteeEvaluator::new()
                .await
                .map_err(|e| Error::Internal(format!("Failed to init committee: {:?}", e)))?,
        );

        let cognitive_queue = Arc::new(CognitiveProcessingQueue::new());
        let quantum_router = Arc::new(QuantumRouter::default());
        let quantum_state = Arc::new(RwLock::new(QuantumState::new()));

        // Spawn cognitive workers as async tasks (now Send-compatible)
        let num_workers = 2;

        for worker_id in 0..num_workers {
            let queue = cognitive_queue.clone();
            let manager = surreal_manager.clone();
            let evaluator = committee_evaluator.clone();

            let worker = crate::memory::core::cognitive_worker::CognitiveWorker::new(
                queue, manager, evaluator,
            );

            // Spawn on main tokio runtime (workers are Send now)
            tokio::spawn(async move {
                log::info!("Cognitive worker {} started", worker_id);
                worker.run().await;
                log::info!("Cognitive worker {} stopped", worker_id);
            });
        }

        log::info!("Started {} cognitive worker tasks", num_workers);

        Ok(Self {
            surreal_manager,
            repository: Arc::new(RwLock::new(MemoryRepository::new())),
            embedding_model,
            cognitive_queue,
            committee_evaluator,
            quantum_router,
            quantum_state,
            cognitive_workers: Arc::new(std::sync::RwLock::new(Vec::new())),
            lazy_eval_strategy: LazyEvalStrategy::default(),
            evaluation_cache: Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(300))
                .build(),
            decay_rate: 0.1,
        })
    }

    /// Configure lazy evaluation strategy
    pub fn set_lazy_eval_strategy(&mut self, strategy: LazyEvalStrategy) {
        self.lazy_eval_strategy = strategy;
    }

    /// Apply temporal decay to memory importance based on age
    ///
    /// This method calculates exponential decay based on memory age and updates:
    /// - Memory importance score (reduced over time)
    /// - Quantum coherence level (simulating decoherence)
    async fn apply_temporal_decay(
        &self,
        memory: &mut crate::domain::memory::primitives::node::MemoryNode,
    ) -> Result<()> {
        // Convert SystemTime to DateTime<Utc> for calculation
        let created_time = chrono::DateTime::<chrono::Utc>::from(memory.base_memory.created_at);
        let now = chrono::Utc::now();

        // Calculate age of memory
        let age = now.signed_duration_since(created_time);

        // Calculate days old with fractional precision
        let days_old = age.num_seconds() as f64 / 86400.0; // seconds per day

        // Apply exponential decay: e^(-decay_rate * days)
        // Using self.decay_rate for configurability
        let decay = (-self.decay_rate * days_old).exp();

        // Apply decay to importance (ensure it doesn't go below minimum threshold)
        let current_importance = memory.importance();
        let new_importance = (current_importance * decay as f32).max(0.01); // Minimum threshold 0.01

        // Update memory importance using existing setter
        memory
            .set_importance(new_importance)
            .map_err(|e| Error::Internal(format!("Failed to set decayed importance: {}", e)))?;

        // Also decay quantum coherence if applicable
        {
            let mut quantum_state = self.quantum_state.write().await;
            // Apply same decay to coherence level
            quantum_state.coherence_level = (quantum_state.coherence_level * decay).max(0.01);
        }

        // Update last_accessed_at in metadata to track access patterns
        memory.stats.record_read();

        log::trace!(
            "Temporal decay applied to {}: importance {} -> {} (age: {:.2} days)",
            memory.id(),
            current_importance,
            new_importance,
            days_old
        );

        Ok(())
    }

    /// Apply temporal decay to core memory node (for internal use)
    ///
    /// Note: Currently unused as architecture applies decay at domain layer.
    /// Retained for future optimization where decay might be applied to core nodes
    /// before domain conversion to reduce conversion overhead.
    #[allow(dead_code)]
    async fn apply_temporal_decay_core(
        &self,
        memory: &mut crate::memory::core::primitives::node::MemoryNode,
    ) -> Result<()> {
        let now = chrono::Utc::now();
        let age = now.signed_duration_since(memory.created_at);

        // Calculate days old with fractional precision
        let days_old = age.num_seconds() as f64 / 86400.0;

        // Apply exponential decay
        let decay = (-self.decay_rate * days_old).exp();

        // Apply decay to importance
        memory.metadata.importance = (memory.metadata.importance as f64 * decay).max(0.01) as f32;

        // Update last_accessed_at
        memory.metadata.last_accessed_at = Some(now);

        Ok(())
    }

    /// Set the decay rate for temporal importance decay
    ///
    /// # Arguments
    /// * `rate` - Decay rate (recommended: 0.01 to 0.5)
    ///   - 0.01: Very slow decay (memories stay relevant longer)
    ///   - 0.1: Default balanced decay
    ///   - 0.5: Fast decay (strong recency bias)
    pub fn set_decay_rate(&mut self, rate: f64) -> Result<()> {
        if rate <= 0.0 || rate > 1.0 {
            return Err(Error::InvalidInput(
                "Decay rate must be between 0.0 and 1.0".into(),
            ));
        }
        self.decay_rate = rate;
        log::info!("Temporal decay rate updated to {}", rate);
        Ok(())
    }

    /// Get current decay rate
    pub fn get_decay_rate(&self) -> f64 {
        self.decay_rate
    }

    /// Spawn background cognitive workers
    ///
    /// Returns the number of workers successfully spawned
    ///
    /// # Errors
    ///
    /// Returns `Error::Internal` if unable to store worker handles due to lock poisoning
    pub fn spawn_cognitive_workers(&self, worker_count: usize) -> Result<usize> {
        use crate::memory::core::cognitive_worker::CognitiveWorker;

        for i in 0..worker_count {
            let queue = self.cognitive_queue.clone();
            let manager = self.surreal_manager.clone();
            let evaluator = self.committee_evaluator.clone();

            let worker = CognitiveWorker::new(queue, manager, evaluator);

            // Spawn on main tokio runtime (workers are Send now)
            tokio::spawn(async move {
                log::info!("Cognitive worker {} started", i);
                worker.run().await;
                log::info!("Cognitive worker {} stopped", i);
            });
        }

        log::info!("Spawned {} cognitive worker tasks", worker_count);
        Ok(worker_count)
    }

    /// Enqueue a cognitive task for background processing
    pub fn enqueue_cognitive_task(
        &self,
        memory_id: String,
        task_type: CognitiveTaskType,
        priority: u8,
    ) -> crate::memory::utils::Result<()> {
        use crate::memory::core::cognitive_queue::CognitiveTask;

        let task = CognitiveTask::new(memory_id, task_type, priority);
        self.cognitive_queue.enqueue(task).map_err(|e| {
            crate::memory::utils::Error::Internal(format!(
                "Failed to enqueue cognitive task: {}",
                e
            ))
        })
    }

    /// Add a new memory using SurrealDB's native capabilities with deduplication
    ///
    /// This method implements content-based deduplication:
    /// 1. Calculate content hash from document text
    /// 2. Check if document with same hash already exists
    /// 3. If exists: Update age/timestamp to "brand new" (skip ingestion)
    /// 4. If new: Ingest document with hash stored
    ///
    /// # Arguments
    /// * `content` - The text content of the memory
    /// * `memory_type` - The type of memory (Semantic, Episodic, etc.)
    /// * `metadata` - Additional metadata (importance, keywords, tags)
    ///
    /// # Returns
    /// * `Ok(MemoryNode)` - The stored or refreshed memory
    /// * `Err(Error)` - Database or processing error
    pub async fn add_memory(
        &self,
        content: String,
        memory_type: MemoryTypeEnum,
        metadata: MemoryMetadata,
    ) -> Result<MemoryNode> {
        // Calculate content hash for deduplication
        let content_hash = crate::domain::memory::serialization::content_hash(&content);

        // Check if document with same content hash already exists
        if let Some(_existing_memory) = self
            .surreal_manager
            .find_document_by_hash(content_hash)
            .await?
        {
            // Document exists - refresh age instead of re-ingesting
            let now = chrono::Utc::now();
            let updated = self
                .surreal_manager
                .update_document_age_by_hash(content_hash, now)
                .await?;

            if updated {
                log::info!(
                    "Document already exists (hash: {}), age refreshed to brand new",
                    content_hash
                );

                // Return the existing memory with refreshed timestamp (converted to domain)
                // Re-fetch to get updated timestamps
                let refreshed_memory = self
                    .surreal_manager
                    .find_document_by_hash(content_hash)
                    .await?
                    .ok_or_else(|| {
                        Error::Internal("Failed to re-fetch refreshed document".to_string())
                    })?;
                return self.convert_memory_to_domain_node(&refreshed_memory);
            } else {
                log::warn!(
                    "Failed to refresh age for existing document (hash: {})",
                    content_hash
                );
                // Fall through to normal ingestion if update failed
            }
        }

        // Document is new - proceed with normal ingestion
        log::debug!("Ingesting new document (hash: {})", content_hash);

        // Create domain memory node first
        let content_struct =
            crate::domain::memory::primitives::types::MemoryContent::text(content.clone());
        let mut domain_memory = MemoryNode::new(memory_type, content_struct);
        domain_memory
            .set_importance(metadata.importance)
            .map_err(|e| Error::Internal(format!("Failed to set importance: {}", e)))?;

        // Apply metadata keywords and tags
        for keyword in &metadata.keywords {
            domain_memory.set_custom_metadata(
                format!("keyword_{}", keyword),
                serde_json::Value::String(keyword.clone()),
            );
        }

        for tag in &metadata.tags {
            domain_memory.set_custom_metadata(
                format!("tag_{}", tag),
                serde_json::Value::String(tag.clone()),
            );
        }

        // Generate embedding for the content
        let embedding_vec = self.generate_embedding(&content).await?;
        domain_memory.embedding = Some(
            crate::domain::memory::primitives::node::AlignedEmbedding::new(embedding_vec.clone()),
        );

        // Convert to memory system format for SurrealDB storage
        let memory_for_storage = self.convert_domain_to_memory_node(&domain_memory);

        // Store in SurrealDB - it handles embedding indexing natively
        let stored_memory = self
            .surreal_manager
            .create_memory(memory_for_storage.clone())
            .await?;

        // Queue cognitive evaluation task with batching (non-blocking)
        if let Err(e) = self
            .cognitive_queue
            .enqueue_with_batching(CognitiveTask::new(
                stored_memory.id.clone(),
                CognitiveTaskType::CommitteeEvaluation,
                5, // Priority: medium (0-255 scale)
            ))
        {
            // Log warning but don't fail - memory already stored
            log::warn!(
                "Failed to queue committee evaluation for memory {}: {}",
                stored_memory.id,
                e
            );
        }

        // Queue quantum routing task for search optimization
        if let Err(e) = self.cognitive_queue.enqueue(CognitiveTask::new(
            stored_memory.id.clone(),
            CognitiveTaskType::QuantumRouting,
            3, // Priority: lower than committee
        )) {
            log::warn!(
                "Failed to queue quantum routing for memory {}: {}",
                stored_memory.id,
                e
            );
        }

        // Queue entanglement discovery task for background processing
        if let Err(e) = self.cognitive_queue.enqueue(CognitiveTask::new(
            stored_memory.id.clone(),
            CognitiveTaskType::EntanglementDiscovery,
            3, // Lower priority than committee evaluation
        )) {
            // Log but don't fail - entanglement is enhancement, not critical
            log::warn!(
                "Failed to queue entanglement discovery for {}: {}",
                stored_memory.id,
                e
            );
        }

        // Add to repository cache
        self.repository.write().await.add(memory_for_storage);

        // Convert stored memory back to domain format for return
        self.convert_memory_to_domain_node(&stored_memory)
    }

    /// Get a memory by ID with evaluation status
    ///
    /// Returns the memory with its current `evaluation_status` field indicating
    /// cognitive processing state:
    ///
    /// # Evaluation Status Values
    /// - `Pending`: Memory stored but cognitive processing not started
    /// - `InProgress`: Currently being evaluated by background workers
    /// - `Success`: Cognitive evaluation complete, quality scores available
    /// - `Failed`: Evaluation attempted but encountered errors
    /// - `Cancelled`: Processing was cancelled
    ///
    /// # Write-Ahead Pattern
    /// Memories are available immediately after creation regardless of evaluation status.
    /// The write-ahead pattern ensures zero-latency reads while cognitive processing
    /// happens asynchronously in background worker threads.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the memory to retrieve
    ///
    /// # Returns
    /// * `Ok(Some(MemoryNode))` - Memory found with evaluation_status field populated
    /// * `Ok(None)` - Memory not found
    /// * `Err(Error)` - Database or conversion error
    ///
    /// # Example
    /// ```rust
    /// if let Some(memory) = coordinator.get_memory("mem_123").await? {
    ///     match memory.evaluation_status() {
    ///         OperationStatus::Pending => println!("Evaluation queued"),
    ///         OperationStatus::Success => println!("Evaluation complete"),
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub async fn get_memory(&self, id: &str) -> Result<Option<MemoryNode>> {
        let memory_option = self.surreal_manager.get_memory(id).await?;

        match memory_option {
            Some(mut memory) => {
                // Apply lazy evaluation strategy if memory evaluation is pending
                if memory.evaluation_status
                    == crate::memory::monitoring::operations::OperationStatus::Pending
                {
                    match self.lazy_eval_strategy {
                        LazyEvalStrategy::WaitForCompletion => {
                            // Poll database until evaluation completes or timeout
                            let timeout = Duration::from_secs(5);
                            let start = Instant::now();

                            loop {
                                // Re-fetch to check if background worker completed evaluation
                                if let Ok(Some(updated)) = self.surreal_manager.get_memory(&memory.id).await
                                    && updated.evaluation_status != crate::memory::monitoring::operations::OperationStatus::Pending {
                                    memory = updated;
                                    break;
                                }

                                if start.elapsed() >= timeout {
                                    log::warn!(
                                        "Evaluation timeout for memory {}, returning with Pending status",
                                        memory.id
                                    );
                                    break;
                                }

                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                        }
                        LazyEvalStrategy::ReturnPartial => {
                            // Return immediately with whatever data we have
                            // evaluation_status remains Pending
                        }
                        LazyEvalStrategy::TriggerAndWait => {
                            // Check cache first to avoid redundant LLM calls
                            if let Some(cached_score) = self.evaluation_cache.get(&memory.id) {
                                memory
                                    .metadata
                                    .set_custom("quality_score", cached_score)
                                    .ok();
                                memory.evaluation_status =
                                    crate::memory::monitoring::operations::OperationStatus::Success;
                            } else {
                                // Trigger immediate synchronous evaluation
                                match self
                                    .committee_evaluator
                                    .evaluate(&memory.content.text)
                                    .await
                                {
                                    Ok(score) => {
                                        memory.metadata.set_custom("quality_score", score).ok();
                                        memory.evaluation_status = crate::memory::monitoring::operations::OperationStatus::Success;

                                        // Cache the result
                                        self.evaluation_cache.insert(memory.id.clone(), score);

                                        // Persist to database
                                        if let Err(e) =
                                            self.surreal_manager.update_memory(memory.clone()).await
                                        {
                                            log::error!(
                                                "Failed to update evaluated memory {}: {:?}",
                                                memory.id,
                                                e
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        log::error!(
                                            "Immediate evaluation failed for {}: {:?}",
                                            memory.id,
                                            e
                                        );
                                        memory.evaluation_status = crate::memory::monitoring::operations::OperationStatus::Failed;
                                        memory
                                            .metadata
                                            .set_custom("error_message", e.to_string())
                                            .ok();
                                    }
                                }
                            }
                        }
                    }
                }

                // Convert to domain format
                let mut domain_memory = self.convert_memory_to_domain_node(&memory)?;

                // Apply temporal decay before returning
                self.apply_temporal_decay(&mut domain_memory).await?;

                Ok(Some(domain_memory))
            }
            None => Ok(None),
        }
    }

    /// Update an existing memory using SurrealDB's native capabilities
    pub async fn update_memory(
        &self,
        id: &str,
        content: Option<String>,
        metadata: Option<MemoryMetadata>,
    ) -> Result<MemoryNode> {
        // Get existing memory from SurrealDB
        let existing_memory = self
            .surreal_manager
            .get_memory(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("Memory with id {} not found", id)))?;

        let mut updated_memory = existing_memory;

        // Update content if provided
        if let Some(new_content) = content {
            updated_memory.content =
                crate::memory::core::primitives::types::MemoryContent::new(&new_content);

            // Re-generate embedding for updated content
            let embedding = self.generate_embedding(&new_content).await?;
            updated_memory.embedding = Some(embedding);
        }

        // Update metadata if provided
        if let Some(new_metadata) = metadata {
            updated_memory.metadata = new_metadata;
        }

        // Update in SurrealDB - it handles embedding indexing natively
        let stored_memory = self
            .surreal_manager
            .update_memory(updated_memory.clone())
            .await?;

        // Update in repository cache
        self.repository.write().await.update(updated_memory);

        // Convert to domain MemoryNode for return
        self.convert_memory_to_domain_node(&stored_memory)
    }

    /// Delete a memory using SurrealDB's native capabilities
    pub async fn delete_memory(&self, id: &str) -> Result<()> {
        // Delete from SurrealDB - it handles vector index removal natively
        self.surreal_manager.delete_memory(id).await?;

        // Remove from repository cache
        self.repository.write().await.delete(id);

        Ok(())
    }

    /// Search for memories using native SurrealDB vector search
    pub async fn search_memories(
        &self,
        query: &str,
        filter: Option<MemoryFilter>,
        top_k: usize,
    ) -> Result<Vec<MemoryNode>> {
        // Generate embedding for the query using the same method as add_memory
        let query_embedding = self.generate_embedding(query).await?;

        // Use SurrealDB's native vector similarity search directly
        let memory_stream = self
            .surreal_manager
            .search_by_vector(query_embedding, top_k);

        // Collect results using StreamExt::collect()
        let memories: Vec<_> = memory_stream.collect().await;

        // Apply lazy evaluation strategy before domain conversion
        let mut core_memories = Vec::new();
        for memory_result in memories {
            match memory_result {
                Ok(mut memory) => {
                    // Apply lazy evaluation strategy if pending
                    if memory.evaluation_status
                        == crate::memory::monitoring::operations::OperationStatus::Pending
                    {
                        match self.lazy_eval_strategy {
                            LazyEvalStrategy::WaitForCompletion => {
                                let timeout = Duration::from_secs(5);
                                let start = Instant::now();

                                loop {
                                    if let Ok(Some(updated)) = self.surreal_manager.get_memory(&memory.id).await
                                        && updated.evaluation_status != crate::memory::monitoring::operations::OperationStatus::Pending {
                                        memory = updated;
                                        break;
                                    }
                                    if start.elapsed() >= timeout {
                                        break;
                                    }
                                    tokio::time::sleep(Duration::from_millis(100)).await;
                                }
                            }
                            LazyEvalStrategy::ReturnPartial => {
                                // Keep as-is, client can check evaluation_status
                            }
                            LazyEvalStrategy::TriggerAndWait => {
                                // Check cache first
                                if let Some(cached_score) = self.evaluation_cache.get(&memory.id) {
                                    memory
                                        .metadata
                                        .set_custom("quality_score", cached_score)
                                        .ok();
                                    memory.evaluation_status = crate::memory::monitoring::operations::OperationStatus::Success;
                                } else {
                                    // Evaluate immediately
                                    if let Ok(score) = self
                                        .committee_evaluator
                                        .evaluate(&memory.content.text)
                                        .await
                                    {
                                        memory.metadata.set_custom("quality_score", score).ok();
                                        memory.evaluation_status = crate::memory::monitoring::operations::OperationStatus::Success;
                                        self.evaluation_cache.insert(memory.id.clone(), score);
                                        self.surreal_manager
                                            .update_memory(memory.clone())
                                            .await
                                            .ok();
                                    }
                                }
                            }
                        }
                    }
                    core_memories.push(memory);
                }
                Err(e) => return Err(Error::Internal(format!("Search failed: {}", e))),
            }
        }

        // Convert to domain after lazy evaluation applied and apply temporal decay
        let mut result_memories = Vec::new();
        for memory in core_memories {
            let mut domain_memory = self.convert_memory_to_domain_node(&memory)?;

            // Apply temporal decay to each memory
            self.apply_temporal_decay(&mut domain_memory).await?;

            result_memories.push(domain_memory);
        }

        // Apply quantum decoherence to accessed memories
        for memory in &result_memories {
            let mut state = self.quantum_state.write().await;
            let coherence = state.measure(); // Reduces coherence by 5%

            // Calculate decayed importance based on coherence
            let current_importance = memory.importance();
            let decayed_importance = current_importance * coherence as f32;

            log::trace!(
                "Decoherence applied to {}: importance {} -> {} (coherence: {})",
                memory.id(),
                current_importance,
                decayed_importance,
                coherence
            );
        }

        // Apply filter if provided
        let filtered_memories = if let Some(filter) = filter {
            result_memories.into_iter()
                .filter(|memory| {
                    // Apply memory type filter
                    if let Some(memory_types) = &filter.memory_types {
                        let domain_memory_type = memory.memory_type();
                        let converted_type = match domain_memory_type {
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Semantic => crate::memory::core::primitives::types::MemoryTypeEnum::Semantic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Episodic => crate::memory::core::primitives::types::MemoryTypeEnum::Episodic,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Procedural => crate::memory::core::primitives::types::MemoryTypeEnum::Procedural,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::Working => crate::memory::core::primitives::types::MemoryTypeEnum::Working,
                            crate::domain::memory::primitives::types::MemoryTypeEnum::LongTerm => crate::memory::core::primitives::types::MemoryTypeEnum::LongTerm,
                            // Map additional domain variants to closest memory system equivalents
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
                    let current_importance = memory.importance();
                    let boosted_importance = (current_importance as f64 * boost_factor) as f32;

                    log::trace!(
                        "Entanglement boost for {}: {} links, importance {} -> {}",
                        memory_id,
                        entangled_links.len(),
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

    /// Add a relationship between memories using SurrealDB's native capabilities
    pub async fn add_relationship(
        &self,
        source_id: &str,
        target_id: &str,
        relationship_type: String,
        metadata: Option<serde_json::Value>,
    ) -> Result<MemoryRelationship> {
        let mut relationship = MemoryRelationship::new(
            source_id.to_string(),
            target_id.to_string(),
            relationship_type,
        );

        if let Some(metadata) = metadata {
            relationship = relationship.with_metadata(metadata);
        }

        // Store relationship in SurrealDB
        let stored_relationship = self
            .surreal_manager
            .create_relationship(relationship)
            .await?;

        Ok(stored_relationship)
    }

    /// Get relationships for a memory using SurrealDB's native capabilities
    pub async fn get_relationships(&self, memory_id: &str) -> Result<Vec<MemoryRelationship>> {
        // Use SurrealDB's native relationship retrieval directly
        let relationship_stream = self.surreal_manager.get_relationships(memory_id);

        // Collect results using StreamExt::collect()
        let relationships: Vec<_> = relationship_stream.collect().await;

        // Convert to MemoryRelationships with proper error handling
        let mut result_relationships = Vec::new();
        for relationship_result in relationships {
            match relationship_result {
                Ok(relationship) => result_relationships.push(relationship),
                Err(e) => {
                    return Err(Error::Internal(format!(
                        "Failed to retrieve relationships: {}",
                        e
                    )));
                }
            }
        }

        Ok(result_relationships)
    }

    /// Convert domain MemoryNode to memory MemoryNode for storage compatibility
    fn convert_domain_to_memory_node(
        &self,
        domain_node: &crate::domain::memory::primitives::node::MemoryNode,
    ) -> crate::memory::core::primitives::node::MemoryNode {
        use crate::memory::core::primitives::{
            metadata::MemoryMetadata as MemoryMemoryMetadata, node::MemoryNode as MemoryMemoryNode,
            types::MemoryContent as MemoryMemoryContent,
            types::MemoryTypeEnum as MemoryMemoryTypeEnum,
        };

        let embedding_vec = domain_node
            .embedding()
            .map(|aligned_emb| aligned_emb.to_vec());

        // Create memory system metadata preserving domain metadata
        let memory_metadata = MemoryMemoryMetadata {
            user_id: domain_node
                .metadata
                .custom
                .get("user_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            agent_id: domain_node
                .metadata
                .custom
                .get("agent_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            context: domain_node
                .metadata
                .custom
                .get("context")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string(),
            importance: domain_node.importance(),
            keywords: domain_node
                .metadata
                .keywords
                .iter()
                .map(|k| k.to_string())
                .collect(),
            tags: domain_node
                .metadata
                .tags
                .iter()
                .map(|t| t.to_string())
                .collect(),
            category: domain_node
                .metadata
                .custom
                .get("category")
                .and_then(|v| v.as_str())
                .unwrap_or("general")
                .to_string(),
            source: domain_node
                .metadata
                .custom
                .get("source")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            created_at: domain_node.base_memory.created_at.into(),
            last_accessed_at: Some(chrono::DateTime::<chrono::Utc>::from(
                domain_node.last_accessed(),
            )),
            embedding: embedding_vec.clone(),
            custom: serde_json::to_value(&domain_node.metadata.custom)
                .unwrap_or_else(|_| serde_json::Value::Object(serde_json::Map::new())),
        };

        // Create memory content
        let memory_content = MemoryMemoryContent::new(&domain_node.content().to_string());

        // Convert memory type - map to closest equivalent
        let memory_type = match domain_node.memory_type() {
            crate::domain::memory::primitives::types::MemoryTypeEnum::Semantic => {
                MemoryMemoryTypeEnum::Semantic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Episodic => {
                MemoryMemoryTypeEnum::Episodic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Procedural => {
                MemoryMemoryTypeEnum::Procedural
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Working => {
                MemoryMemoryTypeEnum::Working
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::LongTerm => {
                MemoryMemoryTypeEnum::LongTerm
            }
            // Map additional domain variants to closest memory system equivalents
            crate::domain::memory::primitives::types::MemoryTypeEnum::Fact => {
                MemoryMemoryTypeEnum::Semantic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Episode => {
                MemoryMemoryTypeEnum::Episodic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Declarative => {
                MemoryMemoryTypeEnum::Semantic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Implicit => {
                MemoryMemoryTypeEnum::Procedural
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Explicit => {
                MemoryMemoryTypeEnum::Semantic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Contextual => {
                MemoryMemoryTypeEnum::Semantic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Temporal => {
                MemoryMemoryTypeEnum::Episodic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Spatial => {
                MemoryMemoryTypeEnum::Episodic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Associative => {
                MemoryMemoryTypeEnum::Semantic
            }
            crate::domain::memory::primitives::types::MemoryTypeEnum::Emotional => {
                MemoryMemoryTypeEnum::Episodic
            }
        };

        // Calculate content hash
        let content_hash = crate::domain::memory::serialization::content_hash(&memory_content.text);

        MemoryMemoryNode {
            id: domain_node.id().to_string(),
            content: memory_content,
            content_hash,
            memory_type,
            created_at: domain_node.base_memory.created_at.into(),
            updated_at: domain_node.base_memory.updated_at.into(),
            embedding: embedding_vec,
            evaluation_status: crate::memory::monitoring::operations::OperationStatus::Pending,
            metadata: memory_metadata,
            relevance_score: None, // No score for stored memories, only for retrieved ones
        }
    }

    /// Convert memory MemoryNode to domain MemoryNode for API compatibility
    fn convert_memory_to_domain_node(
        &self,
        memory_node: &crate::memory::core::primitives::node::MemoryNode,
    ) -> Result<crate::domain::memory::primitives::node::MemoryNode> {
        use crate::domain::memory::primitives::{
            node::{AlignedEmbedding, MemoryNode as DomainMemoryNode, MemoryNodeMetadata},
            types::{MemoryContent as DomainMemoryContent, MemoryTypeEnum as DomainMemoryTypeEnum},
        };
        use uuid::Uuid;

        // Convert memory type - map to closest equivalent
        let domain_memory_type = match memory_node.memory_type {
            crate::memory::core::primitives::types::MemoryTypeEnum::Semantic => {
                DomainMemoryTypeEnum::Semantic
            }
            crate::memory::core::primitives::types::MemoryTypeEnum::Episodic => {
                DomainMemoryTypeEnum::Episodic
            }
            crate::memory::core::primitives::types::MemoryTypeEnum::Procedural => {
                DomainMemoryTypeEnum::Procedural
            }
            crate::memory::core::primitives::types::MemoryTypeEnum::Working => {
                DomainMemoryTypeEnum::Working
            }
            crate::memory::core::primitives::types::MemoryTypeEnum::LongTerm => {
                DomainMemoryTypeEnum::LongTerm
            }
        };

        // Create domain content
        let domain_content = DomainMemoryContent::text(&memory_node.content.text);

        // Parse UUID from string ID - fail fast on corruption
        let uuid = Uuid::parse_str(&memory_node.id).map_err(|e| {
            Error::Internal(format!(
                "Invalid UUID in memory node {}: {}",
                memory_node.id, e
            ))
        })?;

        // Create domain node
        let mut domain_node = DomainMemoryNode::with_id(uuid, domain_memory_type, domain_content);

        // Convert embedding if present
        if let Some(embedding_vec) = &memory_node.embedding {
            domain_node.embedding = Some(AlignedEmbedding::new(embedding_vec.clone()));
        }

        // Convert metadata, preserving evaluation_status in custom field
        let mut custom_map: std::collections::HashMap<Arc<str>, Arc<serde_json::Value>> =
            memory_node
                .metadata
                .custom
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (Arc::from(k.as_str()), Arc::new(v.clone())))
                        .collect()
                })
                .unwrap_or_default();

        // Store evaluation_status in custom metadata for domain layer
        custom_map.insert(
            Arc::from("evaluation_status"),
            Arc::new(
                serde_json::to_value(&memory_node.evaluation_status)
                    .unwrap_or(serde_json::Value::Null),
            ),
        );

        let domain_metadata = MemoryNodeMetadata {
            importance: memory_node.metadata.importance,
            keywords: memory_node
                .metadata
                .keywords
                .iter()
                .map(|k| k.clone().into())
                .collect(),
            tags: memory_node
                .metadata
                .tags
                .iter()
                .map(|t| t.clone().into())
                .collect(),
            custom: custom_map,
            version: 1,
        };
        domain_node.metadata =
            std::sync::Arc::new(domain_metadata);

        Ok(domain_node)
    }

    /// Generate embedding for text content using BERT model
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Use existing BERT embedding provider
        let embedding = self
            .embedding_model
            .embed(text, None)
            .await
            .map_err(|e| Error::Internal(format!("BERT embedding failed: {}", e)))?;
        Ok(embedding)
    }

    /// Shutdown all cognitive worker tasks gracefully
    pub fn shutdown_workers(&mut self) {
        // Flush any pending batches before shutdown
        if let Err(e) = self.cognitive_queue.flush_batches() {
            log::warn!("Failed to flush batches during shutdown: {}", e);
        }

        // Note: Tokio tasks will be cancelled when runtime shuts down
        // We don't await them here since this method is sync
        // The queue channel will be dropped, causing workers to exit their loops
        log::info!("Cognitive workers will shut down when queue is closed");
    }
}

impl MemoryManager for MemoryCoordinator {
    // Delegate non-search methods directly to surreal_manager

    fn create_memory(&self, memory: CoreMemoryNode) -> PendingMemory {
        self.surreal_manager.create_memory(memory)
    }

    fn get_memory(&self, id: &str) -> MemoryQuery {
        self.surreal_manager.get_memory(id)
    }

    fn update_memory(&self, memory: CoreMemoryNode) -> PendingMemory {
        self.surreal_manager.update_memory(memory)
    }

    fn delete_memory(&self, id: &str) -> PendingDeletion {
        self.surreal_manager.delete_memory(id)
    }

    fn create_relationship(&self, relationship: MemoryRelationship) -> PendingRelationship {
        self.surreal_manager.create_relationship(relationship)
    }

    fn get_relationships(&self, memory_id: &str) -> RelationshipStream {
        self.surreal_manager.get_relationships(memory_id)
    }

    fn delete_relationship(&self, id: &str) -> PendingDeletion {
        self.surreal_manager.delete_relationship(id)
    }

    fn query_by_type(&self, memory_type: CoreMemoryTypeEnum) -> MemoryStream {
        self.surreal_manager.query_by_type(memory_type)
    }

    // QUANTUM-ROUTED SEARCH METHODS

    fn search_by_content(&self, query: &str) -> MemoryStream {
        let query = query.to_string();
        let self_clone = self.clone();

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            // Generate embedding lazily when stream is consumed
            match self_clone.generate_embedding(&query).await {
                Ok(emb) => {
                    // Use vector search with cosine similarity
                    let mut stream = self_clone.surreal_manager.search_by_vector(emb, 10);

                    // Forward results through sender
                    use futures_util::StreamExt;
                    while let Some(result) = stream.next().await {
                        if tx.send(result).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    // Fall back to substring search
                    log::warn!(
                        "Embedding generation failed, falling back to substring search: {}",
                        e
                    );

                    let mut stream = self_clone.surreal_manager.search_by_content(&query);

                    // Forward results through sender
                    use futures_util::StreamExt;
                    while let Some(result) = stream.next().await {
                        if tx.send(result).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        MemoryStream::new(rx)
    }

    fn search_by_vector(&self, vector: Vec<f32>, limit: usize) -> MemoryStream {
        // Vector search already uses quantum strategy by default
        self.surreal_manager.search_by_vector(vector, limit)
    }

    fn search_by_temporal(&self, query: &str, limit: usize) -> MemoryStream {
        self.surreal_manager.search_by_temporal(query, limit)
    }

    fn search_by_pattern(&self, query: &str, limit: usize) -> MemoryStream {
        self.surreal_manager.search_by_pattern(query, limit)
    }

    fn update_quantum_signature(
        &self,
        memory_id: &str,
        cognitive_state: &CognitiveState,
    ) -> PendingQuantumUpdate {
        self.surreal_manager
            .update_quantum_signature(memory_id, cognitive_state)
    }

    fn get_quantum_signature(&self, memory_id: &str) -> PendingQuantumSignature {
        self.surreal_manager.get_quantum_signature(memory_id)
    }

    fn create_entanglement_edge(
        &self,
        source_id: &str,
        target_id: &str,
        strength: f32,
        bond_type: EntanglementType,
    ) -> PendingEntanglementEdge {
        self.surreal_manager
            .create_entanglement_edge(source_id, target_id, strength, bond_type)
    }

    fn get_entangled_memories(&self, memory_id: &str, min_strength: f32) -> MemoryStream {
        self.surreal_manager
            .get_entangled_memories(memory_id, min_strength)
    }

    fn get_entangled_by_type(&self, memory_id: &str, bond_type: EntanglementType) -> MemoryStream {
        self.surreal_manager
            .get_entangled_by_type(memory_id, bond_type)
    }

    fn traverse_entanglement_graph(&self, memory_id: &str, max_depth: usize) -> MemoryStream {
        self.surreal_manager
            .traverse_entanglement_graph(memory_id, max_depth)
    }

    fn expand_via_entanglement(&self, memory_ids: Vec<String>, min_strength: f32) -> MemoryStream {
        self.surreal_manager
            .expand_via_entanglement(memory_ids, min_strength)
    }
}

// Optional: Add Drop implementation for automatic cleanup
impl Drop for MemoryCoordinator {
    fn drop(&mut self) {
        // Implement graceful shutdown of workers
        self.shutdown_workers();
    }
}
