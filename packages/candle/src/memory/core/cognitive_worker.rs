use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::timeout;

use crate::memory::core::cognitive_queue::{CognitiveProcessingQueue, CognitiveTaskType};
use crate::memory::core::manager::surreal::{SurrealDBMemoryManager, MemoryManager};
use crate::memory::cognitive::committee::ProviderCommitteeEvaluator;
use crate::memory::cognitive::quantum::state::QuantumState;
use crate::memory::monitoring::operations::{OperationTracker, OperationType};

/// Background worker for processing cognitive tasks
pub struct CognitiveWorker {
    queue: Arc<CognitiveProcessingQueue>,
    memory_manager: Arc<SurrealDBMemoryManager>,
    committee_evaluator: Arc<ProviderCommitteeEvaluator>,
    /// Quantum state for entanglement tracking
    quantum_state: Arc<RwLock<QuantumState>>,
    /// Operation tracker for metrics
    operation_tracker: Arc<OperationTracker>,
}

impl CognitiveWorker {
    /// Create new cognitive worker
    pub fn new(
        queue: Arc<CognitiveProcessingQueue>,
        memory_manager: Arc<SurrealDBMemoryManager>,
        committee_evaluator: Arc<ProviderCommitteeEvaluator>,
        quantum_state: Arc<RwLock<QuantumState>>,
    ) -> Self {
        Self {
            queue,
            memory_manager,
            committee_evaluator,
            quantum_state,
            operation_tracker: Arc::new(OperationTracker::new(10_000)),
        }
    }

    /// Main worker loop - processes tasks from queue
    pub fn run(&self) {
        log::info!("Cognitive worker started, waiting for tasks...");
        
        loop {
            // Blocking dequeue - thread efficiently parks until work arrives
            match self.queue.dequeue() {
                Ok(task) => {
                    log::debug!(
                        "Task dequeued: type={:?}, memory_id={}, priority={}",
                        task.task_type,
                        task.memory_id,
                        task.priority
                    );
                    
                    match task.task_type {
                        CognitiveTaskType::CommitteeEvaluation => {
                            self.process_committee_evaluation(&task.memory_id);
                        }
                        CognitiveTaskType::EntanglementDiscovery => {
                            self.process_entanglement_discovery(&task.memory_id);
                        }
                        CognitiveTaskType::QuantumRouting => {
                            log::debug!("QuantumRouting task deferred to COGMEM_6");
                        }
                        CognitiveTaskType::BatchProcessing(memory_ids) => {
                            self.process_batch_evaluation(memory_ids);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Worker dequeue error: {}", e);
                    // Channel disconnected - exit worker
                    break;
                }
            }
        }
        
        log::info!("Cognitive worker stopped");
    }

    /// Get current operation metrics for monitoring
    pub fn metrics(&self) -> Arc<OperationTracker> {
        self.operation_tracker.clone()
    }

    /// Get queue depth for monitoring
    pub fn queue_depth(&self) -> usize {
        self.queue.get_depth()
    }

    /// Process committee evaluation using real LLM with timeout, retry, and metrics
    fn process_committee_evaluation(&self, memory_id: &str) {
        let memory_id = memory_id.to_string();
        let manager = self.memory_manager.clone();
        let evaluator = self.committee_evaluator.clone();
        let tracker = self.operation_tracker.clone();
        
        // Start operation tracking
        let op_id = tracker.start_operation(OperationType::CommitteeEvaluation, None);
        
        // Spawn non-blocking task
        tokio::task::spawn_blocking(move || {
            if let Some(runtime) = crate::runtime::shared_runtime() {
                runtime.block_on(async move {
                    let start_time = Instant::now();
                    
                    // Get memory from database
                    let mut memory = match manager.get_memory(&memory_id).await {
                        Ok(Some(mem)) => mem,
                        Ok(None) => {
                            log::warn!("Memory {} not found", memory_id);
                            tracker.fail_operation(op_id, "Memory not found".to_string());
                            return;
                        }
                        Err(e) => {
                            log::error!("Failed to fetch memory {}: {:?}", memory_id, e);
                            tracker.fail_operation(op_id, format!("Fetch error: {:?}", e));
                            return;
                        }
                    };
                    
                    // Evaluate with timeout + retry (3 attempts, 10s timeout each)
                    match Self::evaluate_with_timeout_and_retry(
                        evaluator,
                        &memory.content.text,
                        2, // Max 2 retries (3 total attempts)
                    ).await {
                        Ok(score) => {
                            // Store quality score
                            memory.metadata.set_custom("quality_score", score).ok();
                            memory.metadata.set_custom("evaluation_status", "Success").ok();
                            
                            // Update memory in database
                            if let Err(e) = manager.update_memory(memory).await {
                                log::error!("Failed to update memory {}: {:?}", memory_id, e);
                                tracker.fail_operation(op_id, format!("Update error: {:?}", e));
                            } else {
                                log::info!("Committee evaluation completed: {} (score: {:.2})", memory_id, score);
                                tracker.complete_operation(op_id);
                            }
                        }
                        Err(e) => {
                            // All retries failed - use default score
                            log::error!("Committee evaluation exhausted retries for {}: {}", memory_id, e);
                            
                            memory.metadata.set_custom("quality_score", 0.5).ok(); // Default fallback
                            memory.metadata.set_custom("evaluation_status", "Failed").ok();
                            memory.metadata.set_custom("error_message", e.clone()).ok();
                            
                            manager.update_memory(memory).await.ok();
                            tracker.fail_operation(op_id, e);
                        }
                    }
                    
                    let duration = start_time.elapsed();
                    log::debug!("Committee evaluation took {:?} for {}", duration, memory_id);
                });
            } else {
                log::error!("Shared runtime not available");
                tracker.fail_operation(op_id, "Runtime unavailable".to_string());
            }
        });
    }

    /// Process entanglement discovery for a memory
    fn process_entanglement_discovery(&self, memory_id: &str) {
        let memory_id = memory_id.to_string();
        let manager = self.memory_manager.clone();
        let quantum_state = self.quantum_state.clone();
        let tracker = self.operation_tracker.clone();
        
        // Start operation tracking
        let op_id = tracker.start_operation(OperationType::EntanglementDiscovery, None);

        // Spawn non-blocking task - worker continues immediately
        tokio::task::spawn_blocking(move || {
            if let Some(runtime) = crate::runtime::shared_runtime() {
                runtime.block_on(async move {
                    use futures_util::StreamExt;
                    use crate::memory::cognitive::quantum::entanglement::EntanglementLink;
                    use crate::memory::core::primitives::relationship::MemoryRelationship;
                    
                    // Get source memory
                    let memory = match manager.get_memory(&memory_id).await {
                        Ok(Some(mem)) => mem,
                        Ok(None) => {
                            log::warn!("Memory {} not found for entanglement discovery", memory_id);
                            tracker.fail_operation(op_id, "Memory not found".to_string());
                            return;
                        }
                        Err(e) => {
                            log::error!("Failed to retrieve memory {}: {:?}", memory_id, e);
                            tracker.fail_operation(op_id, format!("Failed to retrieve: {:?}", e));
                            return;
                        }
                    };

                    // Get embedding for similarity search
                    let embedding_vec = match &memory.embedding {
                        Some(emb) => emb.clone(),
                        None => {
                            log::warn!("Memory {} has no embedding, cannot discover entanglement", memory_id);
                            tracker.fail_operation(op_id, "No embedding".to_string());
                            return;
                        }
                    };

                    // Search for similar memories
                    let memory_stream = manager.search_by_vector(embedding_vec.clone(), 10);
                    let memories: Vec<_> = memory_stream.collect().await;
                    
                    let mut entangled_count = 0;

                    // Process each similar memory
                    for memory_result in memories {
                        let related_memory = match memory_result {
                            Ok(mem) => mem,
                            Err(e) => {
                                log::error!("Error in similarity search: {:?}", e);
                                continue;
                            }
                        };

                        // Skip self
                        if related_memory.id == memory_id {
                            continue;
                        }

                        // Calculate cosine similarity
                        let similarity = match (&memory.embedding, &related_memory.embedding) {
                            (Some(emb1), Some(emb2)) => {
                                // Dot product
                                let dot: f32 = emb1.iter()
                                    .zip(emb2.iter())
                                    .map(|(a, b)| a * b)
                                    .sum();
                                
                                // Norms
                                let norm1: f32 = emb1.iter().map(|x| x * x).sum::<f32>().sqrt();
                                let norm2: f32 = emb2.iter().map(|x| x * x).sum::<f32>().sqrt();
                                
                                if norm1 == 0.0 || norm2 == 0.0 {
                                    0.0
                                } else {
                                    dot / (norm1 * norm2)
                                }
                            }
                            _ => continue,
                        };

                        // Create entanglement if similarity > 0.7
                        if similarity > 0.7 {
                            // Create entanglement link
                            let link = EntanglementLink::new(
                                memory_id.clone(),
                                related_memory.id.clone(),
                                similarity as f64,
                            );

                            // Store in quantum state
                            {
                                let mut state = quantum_state.write().await;
                                state.add_entanglement(link);
                            }

                            // Create SurrealDB relationship
                            let relationship = MemoryRelationship::new(
                                memory_id.clone(),
                                related_memory.id.clone(),
                                "entangled".to_string(),
                            )
                            .with_metadata(serde_json::json!({
                                "strength": similarity,
                                "discovered_at": chrono::Utc::now().to_rfc3339(),
                            }));

                            // Store relationship
                            if let Err(e) = manager.create_relationship(relationship).await {
                                log::error!("Failed to create relationship: {:?}", e);
                            } else {
                                entangled_count += 1;
                                log::debug!(
                                    "Created entanglement: {} <-> {} (strength: {:.3})",
                                    memory_id, related_memory.id, similarity
                                );
                            }
                        }
                    }

                    log::info!(
                        "Entanglement discovery complete for {}: {} links created",
                        memory_id, entangled_count
                    );
                    tracker.complete_operation(op_id);
                });
            } else {
                log::error!("Shared runtime not available for entanglement discovery");
                tracker.fail_operation(op_id, "Runtime unavailable".to_string());
            }
        });
    }

    /// Evaluate with timeout and retry for transient failures
    async fn evaluate_with_timeout_and_retry(
        evaluator: Arc<ProviderCommitteeEvaluator>,
        content: &str,
        max_retries: u32,
    ) -> Result<f64, String> {
        let mut attempt = 0;
        let mut backoff_ms = 100u64;
        
        loop {
            // Wrap evaluation in 10-second timeout
            let eval_future = evaluator.evaluate(content);
            match timeout(Duration::from_secs(10), eval_future).await {
                Ok(Ok(score)) => {
                    log::debug!("Evaluation succeeded on attempt {}", attempt + 1);
                    return Ok(score);
                }
                Ok(Err(e)) if attempt < max_retries => {
                    // Cognitive error - retry with backoff
                    attempt += 1;
                    log::warn!(
                        "Evaluation attempt {}/{} failed: {:?}, retrying in {}ms",
                        attempt, max_retries + 1, e, backoff_ms
                    );
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    backoff_ms *= 2; // Exponential backoff
                }
                Ok(Err(e)) => {
                    // Max retries exceeded
                    return Err(format!("Evaluation failed after {} attempts: {:?}", attempt + 1, e));
                }
                Err(_) if attempt < max_retries => {
                    // Timeout - retry
                    attempt += 1;
                    log::warn!(
                        "Evaluation timeout on attempt {}/{}, retrying in {}ms",
                        attempt, max_retries + 1, backoff_ms
                    );
                    tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    backoff_ms *= 2;
                }
                Err(_) => {
                    return Err(format!("Evaluation timed out after {} attempts", attempt + 1));
                }
            }
        }
    }

    /// Process batch of memories for committee evaluation
    fn process_batch_evaluation(&self, memory_ids: Vec<String>) {
        let manager = self.memory_manager.clone();
        let evaluator = self.committee_evaluator.clone();

        // Use tokio::task::spawn_blocking as shown in existing methods
        tokio::task::spawn_blocking(move || {
            if let Some(runtime) = crate::runtime::shared_runtime() {
                runtime.block_on(async move {
                    log::info!("Processing batch evaluation for {} memories", memory_ids.len());

                    // Collect memory contents
                    let mut memories = Vec::new();
                    for id in &memory_ids {
                        match manager.get_memory(id).await {
                            Ok(Some(memory)) => {
                                memories.push((id.clone(), memory.content.text.clone()));
                            }
                            Ok(None) => {
                                log::warn!("Memory {} not found for batch evaluation", id);
                            }
                            Err(e) => {
                                log::error!("Failed to retrieve memory {}: {:?}", id, e);
                            }
                        }
                    }

                    if memories.is_empty() {
                        log::warn!("No memories to evaluate in batch");
                        return;
                    }

                    // Evaluate batch with single LLM call
                    match evaluator.evaluate_batch(&memories).await {
                        Ok(results) => {
                            log::info!("Batch evaluation successful: {} scores returned", results.len());

                            // Update each memory with its score
                            for (id, score) in results {
                                match manager.get_memory(&id).await {
                                    Ok(Some(mut memory)) => {
                                        // Update quality score in metadata
                                        memory.metadata.set_custom("quality_score", score).ok();
                                        memory.metadata.set_custom("evaluation_status", "Success").ok();
                                        memory.metadata.set_custom("evaluated_at", chrono::Utc::now().to_rfc3339()).ok();
                                        memory.metadata.set_custom("evaluation_method", "batch_committee").ok();

                                        // Update memory
                                        match manager.update_memory(memory).await {
                                            Ok(_) => {
                                                log::debug!("Updated memory {} with score {:.3}", id, score);
                                            }
                                            Err(e) => {
                                                log::error!("Failed to update memory {}: {:?}", id, e);
                                            }
                                        }
                                    }
                                    Ok(None) => {
                                        log::warn!("Memory {} disappeared during batch processing", id);
                                    }
                                    Err(e) => {
                                        log::error!("Failed to retrieve memory {} for update: {:?}", id, e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Batch evaluation failed: {:?}", e);
                            
                            // Mark all memories as failed
                            for id in &memory_ids {
                                if let Ok(Some(mut memory)) = manager.get_memory(id).await {
                                    memory.metadata.set_custom("evaluation_status", "Failed").ok();
                                    memory.metadata.set_custom("error_message", e.to_string()).ok();
                                    let _ = manager.update_memory(memory).await;
                                }
                            }
                        }
                    }
                });
            } else {
                log::error!("Shared runtime not available for batch processing");
            }
        });
    }

}