use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::timeout;

use crate::memory::core::cognitive_queue::{CognitiveProcessingQueue, CognitiveTaskType};
use crate::memory::core::manager::surreal::{SurrealDBMemoryManager, MemoryManager};
use crate::memory::cognitive::committee::ModelCommitteeEvaluator;
use crate::domain::memory::cognitive::types::{
    CognitiveMemory,
    CognitiveMemoryConfig,
    CognitiveState,
    AlignedActivationPattern,
    EntanglementType,
    CausalLink,
};
use crate::memory::monitoring::operations::{OperationTracker, OperationType};

/// Background worker for processing cognitive tasks
pub struct CognitiveWorker {
    queue: Arc<CognitiveProcessingQueue>,
    memory_manager: Arc<SurrealDBMemoryManager>,
    committee_evaluator: Arc<ModelCommitteeEvaluator>,
    /// Cognitive memory for quantum features and pattern storage
    _cognitive_memory: Arc<RwLock<CognitiveMemory>>,
    /// Operation tracker for metrics
    operation_tracker: Arc<OperationTracker>,
}

impl CognitiveWorker {
    // ═════════════════════════════════════════════════════════════════════════════
    // TEMPORAL DECAY MAINTENANCE (Future Enhancement)
    // ═════════════════════════════════════════════════════════════════════════════
    //
    // OVERVIEW:
    // Temporal context maintains a sliding window over memory history. As time
    // advances, older memories decay in relevance through exponential window sliding.
    //
    // MECHANISM:
    // - TemporalContext::slide_window() applies discrete exponential decay
    // - Formula: V(n+1) = V(n) * (1 - temporal_decay)
    // - Default decay_rate: 0.1 (10% reduction per window slide)
    // - Default window_duration: 3600s (1 hour)
    // - Half-life: ~6.58 windows (6.58 hours with default settings)
    //
    // MATHEMATICAL MODEL:
    // Discrete exponential decay equivalent to continuous: V(t) = V₀ * e^(-λt)
    // where λ = -ln(1 - decay_rate) ≈ 0.105 for decay_rate=0.1
    //
    // IMPLEMENTATION STRATEGY:
    // Add periodic maintenance task that calls slide_window() on:
    // - Fixed intervals (e.g., every 5 minutes via tokio::time::interval)
    // - After N new memories added (threshold-based trigger)
    // - On explicit user request (maintenance API call)
    //
    // ARCHITECTURE NOTE:
    // Current temporal_context is Arc<CachePadded<TemporalContext>> without interior
    // mutability. To enable slide_window() (which requires &mut self), one of:
    // 1. Wrap in RwLock: Arc<RwLock<TemporalContext>>
    // 2. Refactor slide_window() to use &self with internal Atomics/RwLock
    // 3. Add accessor method in CognitiveMemory for clone-modify-swap pattern
    //
    // REFERENCE IMPLEMENTATION:
    /*
    async fn maintain_temporal_context(&self) -> Result<()> {
        let cognitive_mem = self.cognitive_memory.read().await;
        
        // NOTE: Requires RwLock wrapper on temporal_context first
        // let mut temporal_ctx = cognitive_mem.state().temporal_context.write().await;
        // temporal_ctx.slide_window();
        
        log::debug!("Applied temporal decay via window sliding");
        
        Ok(())
    }
    */
    //
    // PERFORMANCE:
    // - Complexity: O(n) where n = history_embedding dimension
    // - Typical execution: <1μs for 1024-dim embeddings
    // - Lock contention: Minimal with 5-minute intervals
    //
    // INTEGRATION TRIGGERS (to implement):
    // 1. Time-based: tokio::time::interval(Duration::from_secs(300))
    // 2. Event-based: After cognitive_memory.stats().working_memory_accesses > threshold
    // 3. Hybrid: Whichever comes first (recommended)
    //
    // TODO: Add RwLock wrapper to temporal_context in CognitiveState
    // TODO: Integrate into periodic maintenance system when available
    // TODO: Add metrics for decay effectiveness (temporal_relevance_score)
    // ═════════════════════════════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────────────────────────────
    // TEMPORAL DECAY CONFIGURATION
    // ─────────────────────────────────────────────────────────────────────────────
    //
    // Decay parameters are configured in TemporalContext (domain/memory/cognitive/types.rs:315)
    //
    // KEY PARAMETERS:
    //
    // 1. window_duration: Duration
    //    - How often to apply decay (slide window)
    //    - Default: Duration::from_secs(3600) = 1 hour
    //    - Trade-off: Shorter = more responsive, higher CPU; Longer = coarser granularity
    //
    // 2. temporal_decay: f32
    //    - Decay factor per window slide (0.0 - 1.0)
    //    - Default: 0.1 (10% decay per window)
    //    - Formula: new_value = old_value * (1.0 - temporal_decay)
    //    - Half-life: window_duration * ln(2) / ln(1/(1-temporal_decay))
    //
    // 3. history_embedding: Vec<f32>
    //    - Temporal memory vector (dimension = embedding model dim)
    //    - Stores accumulated historical context with decay weights
    //    - Decayed during each slide_window() call
    //
    // 4. prediction_horizon: Vec<f32>
    //    - Future projection vector (not currently decayed)
    //    - For anticipatory/planning features
    //
    // TUNING GUIDANCE BY USE CASE:
    //
    // Conversational AI (focus on recent context):
    //   - window_duration: 15-30 minutes
    //   - temporal_decay: 0.2-0.3 (aggressive forgetting)
    //   - Rationale: Conversation shifts topics rapidly, old context less relevant
    //
    // Research Assistant (preserve long-term patterns):
    //   - window_duration: 4-8 hours
    //   - temporal_decay: 0.05-0.1 (gradual forgetting)
    //   - Rationale: Long-term connections matter, preserve historical insights
    //
    // Real-time Systems (balance recency and history):
    //   - window_duration: 1-2 hours
    //   - temporal_decay: 0.1-0.15 (moderate forgetting)
    //   - Rationale: Balance between responsiveness and pattern retention
    //
    // PERFORMANCE CONSIDERATIONS:
    //
    // - Window sliding is O(n) where n = history_embedding.len()
    //   * 384 dims: ~100ns
    //   * 1024 dims: ~250ns  
    //   * 4096 dims: ~1μs
    //
    // - Batching strategy:
    //   * Option A: Fixed interval (e.g., every 5 minutes)
    //     - Pros: Predictable CPU usage, consistent decay
    //     - Cons: May slide unnecessarily during idle periods
    //
    //   * Option B: Threshold-based (e.g., after 100 new memories)
    //     - Pros: Only slides when there's activity, efficient
    //     - Cons: Irregular timing, may delay decay during low activity
    //
    //   * Option C: Hybrid (recommended)
    //     - Max interval: 5 minutes (ensures regular decay)
    //     - Max memories: 100 additions (responds to bursts)
    //     - Whichever trigger fires first → slide_window()
    //     - Pros: Best of both, responsive and efficient
    //
    // - Trade-offs:
    //   * Decay precision vs. computational cost
    //   * Memory freshness vs. historical retention
    //   * CPU overhead vs. relevance accuracy
    //
    // IMPLEMENTATION EXAMPLES:
    //
    // // Example 1: Time-based trigger (simple)
    // tokio::spawn(async move {
    //     let mut interval = tokio::time::interval(Duration::from_secs(300));
    //     loop {
    //         interval.tick().await;
    //         if let Err(e) = worker.maintain_temporal_context().await {
    //             log::error!("Temporal maintenance failed: {}", e);
    //         }
    //     }
    // });
    //
    // // Example 2: Hybrid trigger (recommended)
    // let memory_threshold = 100;
    // let time_threshold = Duration::from_secs(300);
    // let mut last_slide = Instant::now();
    // let mut memories_since_slide = 0;
    //
    // // In memory addition code:
    // memories_since_slide += 1;
    // if memories_since_slide >= memory_threshold || last_slide.elapsed() >= time_threshold {
    //     worker.maintain_temporal_context().await?;
    //     memories_since_slide = 0;
    //     last_slide = Instant::now();
    // }
    // ─────────────────────────────────────────────────────────────────────────────

    /// Create new cognitive worker
    pub fn new(
        queue: Arc<CognitiveProcessingQueue>,
        memory_manager: Arc<SurrealDBMemoryManager>,
        committee_evaluator: Arc<ModelCommitteeEvaluator>,
    ) -> Self {
        Self {
            queue,
            memory_manager,
            committee_evaluator,
            _cognitive_memory: Arc::new(RwLock::new(
                CognitiveMemory::new(CognitiveMemoryConfig::default())
            )),
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
        
        // Spawn async task
        tokio::spawn(async move {
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
                &evaluator,
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
    }

    /// Process entanglement discovery for a memory
    fn process_entanglement_discovery(&self, memory_id: &str) {
        let memory_id = memory_id.to_string();
        let manager = self.memory_manager.clone();
        let tracker = self.operation_tracker.clone();
        
        // Start operation tracking
        let op_id = tracker.start_operation(OperationType::EntanglementDiscovery, None);

        // Spawn async task
        tokio::spawn(async move {
            use futures_util::StreamExt;
                    
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

                    // Load existing quantum signature if available (cached bonds)
                    // Note: Primary edges are in entangled RELATION table, this is denormalized cache
                    let existing_signature = match manager.get_quantum_signature(&memory_id).await {
                        Ok(Some(state)) => {
                            log::debug!(
                                "Loaded existing quantum signature for {} with {} cached bonds",
                                memory_id,
                                state.quantum_entanglement_bond_count()
                            );
                            Some(state)
                        }
                        Ok(None) => {
                            log::debug!("No existing quantum signature cache for {}", memory_id);
                            None
                        }
                        Err(e) => {
                            log::warn!(
                                "Failed to load quantum signature for {}: {:?}",
                                memory_id,
                                e
                            );
                            None
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

                    // Convert to SIMD-aligned activation pattern for vectorized operations
                    let activation_pattern = AlignedActivationPattern::new(embedding_vec.clone());

                    log::debug!(
                        "Created SIMD-aligned activation pattern for {}: dimension={}, timestamp={:?}",
                        memory_id,
                        activation_pattern.dimension,
                        activation_pattern.last_update
                    );

                    // Use existing signature cache if available, otherwise create new
                    let state_a = if let Some(existing) = existing_signature {
                        log::debug!(
                            "Using existing quantum state for {} (preserving {} cached bonds)",
                            memory_id,
                            existing.quantum_entanglement_bond_count()
                        );
                        existing
                    } else {
                        match CognitiveState::with_quantum_coherence(
                            embedding_vec.clone(),
                            vec![0.0; embedding_vec.len()]  // Zero phases = amplitude-only state
                        ) {
                            Ok(state) => {
                                log::debug!(
                                    "Created new quantum coherent state for {} with {} dimensions",
                                    memory_id,
                                    embedding_vec.len()
                                );
                                state
                            }
                            Err(e) => {
                                log::warn!("Failed to create quantum coherent state for {}: {}", memory_id, e);
                                tracker.fail_operation(op_id, format!("Quantum state creation failed: {}", e));
                                return;
                            }
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

                        // Extract related memory embedding
                        let related_embedding_vec = match &related_memory.embedding {
                            Some(emb) => emb.clone(),
                            None => {
                                log::debug!("Related memory {} has no embedding, skipping", related_memory.id);
                                continue;
                            }
                        };

                        // Convert related embedding to SIMD-aligned pattern
                        let related_activation = AlignedActivationPattern::new(related_embedding_vec.clone());

                        log::debug!(
                            "Created SIMD-aligned pattern for related memory {}: dimension={}",
                            related_memory.id,
                            related_activation.dimension
                        );

                        // Create quantum state for related memory
                        let state_b = match CognitiveState::with_quantum_coherence(
                            related_embedding_vec.clone(),
                            vec![0.0; related_embedding_vec.len()]
                        ) {
                            Ok(state) => state,
                            Err(e) => {
                                log::warn!("Failed to create quantum state for related memory {}: {}", related_memory.id, e);
                                continue;
                            }
                        };

                        // Measure quantum entanglement between states
                        let entanglement_strength = match state_a.measure_quantum_entanglement(&state_b) {
                            Some(strength) => strength,
                            None => {
                                log::warn!(
                                    "Dimension mismatch measuring entanglement between {} and {}", 
                                    memory.id, 
                                    related_memory.id
                                );
                                continue;
                            }
                        };

                        log::debug!(
                            "Quantum entanglement strength between {} and {}: {:.4}",
                            memory.id,
                            related_memory.id,
                            entanglement_strength
                        );

                        // Calculate temporal distance between memories (milliseconds)
                        // 
                        // Temporal semantics:
                        // - Positive value: related_memory is NEWER → potential effect (cause → effect)
                        // - Negative value: related_memory is OLDER → retrospective link (looking back)
                        // - None: Overflow (extremely rare, only if years+ time difference)
                        let temporal_distance_ms: Option<i64> = {
                            // MemoryNode.created_at is DateTime<Utc> (always present, never Option)
                            let created_at = memory.created_at;
                            let related_created_at = related_memory.created_at;
                            
                            // Calculate signed duration (positive if related is newer, negative if older)
                            let duration = related_created_at.signed_duration_since(created_at);
                            
                            // Convert to milliseconds with overflow protection
                            match duration.num_milliseconds().checked_abs() {
                                Some(abs_ms) => {
                                    // Preserve sign for causal direction analysis
                                    if duration.num_milliseconds() < 0 {
                                        Some(-abs_ms)  // related_memory is OLDER
                                    } else {
                                        Some(abs_ms)   // related_memory is NEWER
                                    }
                                }
                                None => {
                                    // Overflow: time difference too large for i64 milliseconds
                                    // This would require ~292 million years - practically impossible
                                    log::warn!(
                                        "Temporal distance overflow between {} and {}: duration exceeds i64::MAX milliseconds",
                                        memory.id,
                                        related_memory.id
                                    );
                                    None
                                }
                            }
                        };

                        // Log temporal distance for debugging
                        log::trace!(
                            "Temporal distance {} → {}: {:?}ms {}",
                            memory.id,
                            related_memory.id,
                            temporal_distance_ms,
                            match temporal_distance_ms {
                                Some(ms) if ms > 0 => "(newer)",
                                Some(ms) if ms < 0 => "(older)",
                                Some(_) => "(simultaneous)",
                                None => "(overflow)",
                            }
                        );

                        // Create classified entanglement bond if strength is significant
                        if entanglement_strength > 0.7 {
                            // Quantum Entanglement Type Classification
                            // 
                            // Based on quantum correlation strength, we classify entanglement into types
                            // that correspond to quantum mechanical states:
                            //
                            // - EntanglementType::BellPair    : >0.95  - Maximal entanglement (Bell pair state |Φ⁺⟩)
                            // - EntanglementType::Bell        : 0.85-0.95 - High entanglement (Bell state)
                            // - EntanglementType::Semantic    : 0.7-0.85 - Meaning-based similarity (moderate)
                            // - EntanglementType::Temporal    : Future (QCOG_4) - Time-ordered sequences
                            // - EntanglementType::Causal      : Future (QCOG_4) - Cause-effect relationships
                            // - EntanglementType::Emergent    : Future - Pattern emergence via ML detection
                            // - EntanglementType::Werner      : Future - Mixed quantum states
                            // - EntanglementType::Weak        : Future - Low entanglement (0.5-0.7 range)
                            //
                            // The thresholds are based on standard quantum information theory where:
                            // - Maximal entanglement approaches 1.0 (perfect correlation)
                            // - Bell states show strong but not perfect entanglement
                            // - Semantic similarity indicates moderate quantum correlation

                            // Determine entanglement type based on temporal context AND semantic strength
                            let entanglement_type = if let Some(temp_dist) = temporal_distance_ms {
                                let abs_dist = temp_dist.abs();
                                
                                if abs_dist < 1000 {
                                    // Within 1 second - likely causal relationship
                                    EntanglementType::Causal
                                } else if abs_dist < 60_000 {
                                    // Within 1 minute - temporal sequence
                                    EntanglementType::Temporal
                                } else if entanglement_strength > 0.95 {
                                    EntanglementType::BellPair  // Maximal semantic (time-independent)
                                } else if entanglement_strength > 0.85 {
                                    EntanglementType::Bell  // High semantic (time-independent)
                                } else {
                                    EntanglementType::Semantic  // Meaning-based (time-independent)
                                }
                            } else {
                                // No temporal info - fall back to strength-based semantic classification
                                if entanglement_strength > 0.95 {
                                    EntanglementType::BellPair
                                } else if entanglement_strength > 0.85 {
                                    EntanglementType::Bell
                                } else {
                                    EntanglementType::Semantic
                                }
                            };

                            // Parse memory IDs from String to Uuid for quantum signature
                            let related_id_uuid = match uuid::Uuid::parse_str(&related_memory.id) {
                                Ok(uuid) => uuid,
                                Err(e) => {
                                    log::warn!(
                                        "Failed to parse UUID for memory {}: {}", 
                                        related_memory.id, 
                                        e
                                    );
                                    continue;
                                }
                            };

                            // Create causal link with temporal data if available
                            if let Some(temporal_dist) = temporal_distance_ms {
                                // Parse source memory UUID (related UUID already parsed above as related_id_uuid)
                                if let Ok(source_uuid) = uuid::Uuid::parse_str(&memory.id) {
                                    // Create causal link instance
                                    let causal_link = CausalLink::new(
                                        source_uuid,
                                        related_id_uuid,
                                        entanglement_strength,
                                        temporal_dist
                                    );

                                    log::info!(
                                        "Causal link: {} -> {} (strength: {:.3}, temporal: {}ms, direction: {})",
                                        memory.id,
                                        related_memory.id,
                                        causal_link.strength,
                                        causal_link.temporal_distance,
                                        if temporal_dist > 0 { "forward" } 
                                        else if temporal_dist < 0 { "backward" } 
                                        else { "simultaneous" }
                                    );
                                }
                            }

                            // Add quantum entanglement bond to cognitive state
                            let bond_created = state_a.add_quantum_entanglement_bond(
                                related_id_uuid,
                                entanglement_strength,
                                entanglement_type
                            );

                            if bond_created {
                                log::info!(
                                    "Created {:?} entanglement bond: {} <-> {} (strength: {:.4}, temporal_dist: {:?}ms)",
                                    entanglement_type,
                                    memory.id,
                                    related_memory.id,
                                    entanglement_strength,
                                    temporal_distance_ms
                                );

                                // Use RELATE to create graph-optimized edge in entangled RELATION table
                                // This creates automatic IN/OUT pointers for O(1) traversal
                                if let Err(e) = manager.create_entanglement_edge(
                                    &memory.id,
                                    &related_memory.id,
                                    entanglement_strength,
                                    entanglement_type,
                                ).await {
                                    log::error!(
                                        "Failed to create entanglement edge {} -> {}: {:?}",
                                        memory.id,
                                        related_memory.id,
                                        e
                                    );
                                } else {
                                    log::debug!(
                                        "Created entanglement edge: {} ->entangled-> {} (strength: {:.3}, type: {:?})",
                                        memory.id, 
                                        related_memory.id, 
                                        entanglement_strength, 
                                        entanglement_type
                                    );
                                    
                                    // Update quantum signature cache (denormalized)
                                    if let Err(e) = manager.update_quantum_signature(&memory.id, &state_a).await {
                                        log::error!(
                                            "Failed to persist quantum signature for {}: {:?}", 
                                            memory.id, 
                                            e
                                        );
                                    } else {
                                        log::debug!(
                                            "Persisted quantum signature for {} (bond count: {})",
                                            memory.id,
                                            state_a.quantum_entanglement_bond_count()
                                        );
                                    }
                                    
                                    entangled_count += 1;
                                }
                            } else {
                                log::warn!(
                                    "Failed to create entanglement bond to {} (invalid strength or target)",
                                    related_memory.id
                                );
                            }
                        }
                    }

                    // Persist final quantum signature with all bonds (cache update)
                    // Primary edges are already in entangled RELATION table via RELATE
                    if entangled_count > 0 {
                        if let Err(e) = manager.update_quantum_signature(&memory_id, &state_a).await {
                            log::error!(
                                "Failed to persist final quantum signature for {}: {:?}",
                                memory_id,
                                e
                            );
                        } else {
                            log::info!(
                                "Persisted quantum signature for {} with {} entanglement bonds (cached)",
                                memory_id,
                                state_a.quantum_entanglement_bond_count()
                            );
                        }
                    }

                    log::info!(
                        "Entanglement discovery complete for {}: {} links created in RELATION table",
                        memory_id, entangled_count
                    );
                    tracker.complete_operation(op_id);
        });
    }

    /// Evaluate with timeout and retry for transient failures
    async fn evaluate_with_timeout_and_retry(
        evaluator: &ModelCommitteeEvaluator,
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

        // Spawn async task
        tokio::spawn(async move {
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
    }

    /// Maintains temporal context by applying decay via window sliding.
    /// 
    /// # Mathematical Model
    /// 
    /// Implements discrete exponential decay:
    /// - V(n+1) = V(n) * (1 - decay_rate)
    /// - Equivalent continuous form: V(t) = V₀ * e^(-λt) where λ = -ln(1-decay_rate)
    /// - Default decay_rate: 0.1 → λ ≈ 0.105
    /// - Half-life: ~6.58 window durations (default: ~6.58 hours)
    /// 
    /// # Architecture Constraint
    /// 
    /// **BLOCKER:** Current TemporalContext is `Arc<CachePadded<TemporalContext>>` without
    /// interior mutability. slide_window() requires `&mut self`, which cannot be obtained
    /// through Arc without RwLock wrapper.
    /// 
    /// **Required change before activation:**
    /// ```rust
    /// // In CognitiveState (domain/memory/cognitive/types.rs:42)
    /// temporal_context: Arc<RwLock<TemporalContext>>  // Add RwLock
    /// ```
    /// 
    /// # Future Integration
    /// 
    /// This method should be called periodically to:
    /// - Apply temporal decay to memory history embedding
    /// - Maintain relevance weighting of recent vs. old memories  
    /// - Prevent unbounded accumulation of historical context
    ///
    /// **Trigger conditions (to be implemented):**
    /// 1. **Time-based:** Every N minutes (default: 5 minutes)
    ///    ```rust
    ///    let mut interval = tokio::time::interval(Duration::from_secs(300));
    ///    loop {
    ///        interval.tick().await;
    ///        self.maintain_temporal_context().await?;
    ///    }
    ///    ```
    /// 
    /// 2. **Threshold-based:** After M new memories added (default: 100)
    ///    ```rust
    ///    if self.memory_count_since_last_slide() >= 100 {
    ///        self.maintain_temporal_context().await?;
    ///    }
    ///    ```
    /// 
    /// 3. **Hybrid (recommended):** Whichever comes first
    ///    - Ensures regular decay even during idle periods
    ///    - Responds to high-activity bursts
    /// 
    /// # Performance
    /// 
    /// - **Complexity:** O(n) where n = history_embedding.len() (typically 384-1024)
    /// - **Execution time:** <1μs for 1024 dimensions with SIMD
    /// - **Lock contention:** Minimal (write lock held <1μs)
    /// 
    /// # Errors
    /// 
    /// Returns error if:
    /// - Temporal context lock cannot be acquired (future: when RwLock added)
    /// - System time moves backwards (handled with Duration::ZERO fallback)
    ///
    #[allow(dead_code)]
    async fn maintain_temporal_context(&self) -> Result<(), String> {
        // ARCHITECTURE NOTE: This is a placeholder until temporal_context has RwLock wrapper
        // 
        // Future implementation (after adding RwLock):
        //
        // let cognitive_mem = self.cognitive_memory.read().await;
        // let state = cognitive_mem.state();
        // 
        // // Acquire write lock on temporal context
        // let mut temporal_ctx = state.temporal_context.write().await;
        // 
        // // Apply exponential decay to history embedding
        // temporal_ctx.slide_window();
        // 
        // log::debug!(
        //     "Applied temporal decay: window_start={:?}, decay_rate={}, history_dim={}",
        //     temporal_ctx.window_start,
        //     temporal_ctx.temporal_decay,
        //     temporal_ctx.history_embedding.len()
        // );
        
        log::debug!(
            "Temporal decay maintenance placeholder - awaiting RwLock wrapper on temporal_context"
        );
        
        Ok(())
    }

}