use crossbeam_channel::{unbounded, Sender, Receiver};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Task types for cognitive processing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CognitiveTaskType {
    /// Evaluate memory quality using Committee (LLM-based)
    CommitteeEvaluation,
    /// Determine routing strategy using QuantumRouter
    QuantumRouting,
    /// Discover entangled/related memories
    EntanglementDiscovery,
    /// Batch processing of multiple memory IDs
    BatchProcessing(Vec<String>),
}

/// Cognitive task with priority and metadata
#[derive(Debug, Clone)]
pub struct CognitiveTask {
    pub memory_id: String,
    pub task_type: CognitiveTaskType,
    pub priority: u8,  // 0=lowest, 255=highest
    pub created_at: DateTime<Utc>,
}

impl CognitiveTask {
    pub fn new(memory_id: String, task_type: CognitiveTaskType, priority: u8) -> Self {
        Self {
            memory_id,
            task_type,
            priority,
            created_at: Utc::now(),
        }
    }
}

/// Accumulates tasks for efficient batch processing
pub struct BatchAccumulator {
    tasks: Vec<CognitiveTask>,
    max_batch_size: usize,
    max_wait: Duration,
    last_batch_time: Instant,
}

impl BatchAccumulator {
    pub fn new(max_batch_size: usize, max_wait_ms: u64) -> Self {
        Self {
            tasks: Vec::with_capacity(max_batch_size),
            max_batch_size,
            max_wait: Duration::from_millis(max_wait_ms),
            last_batch_time: Instant::now(),
        }
    }

    /// Add task and return batch if ready (size or timeout reached)
    pub fn add(&mut self, task: CognitiveTask) -> Option<Vec<CognitiveTask>> {
        self.tasks.push(task);

        // Check if batch is ready
        let size_reached = self.tasks.len() >= self.max_batch_size;
        let timeout_reached = self.last_batch_time.elapsed() >= self.max_wait;

        if size_reached || (timeout_reached && !self.tasks.is_empty()) {
            let batch = std::mem::take(&mut self.tasks);
            self.last_batch_time = Instant::now();
            Some(batch)
        } else {
            None
        }
    }

    /// Force flush any pending tasks
    pub fn flush(&mut self) -> Option<Vec<CognitiveTask>> {
        if self.tasks.is_empty() {
            None
        } else {
            let batch = std::mem::take(&mut self.tasks);
            self.last_batch_time = Instant::now();
            Some(batch)
        }
    }
}

/// Lock-free queue for cognitive processing
#[derive(Clone)]
pub struct CognitiveProcessingQueue {
    sender: Sender<CognitiveTask>,
    receiver: Receiver<CognitiveTask>,
    // Batch accumulator for CommitteeEvaluation tasks
    batch_accumulator: Arc<Mutex<BatchAccumulator>>,
}

impl CognitiveProcessingQueue {
    /// Create new unbounded queue using crossbeam
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self { 
            sender, 
            receiver,
            // Default: batch_size=5, max_wait=2000ms
            batch_accumulator: Arc::new(Mutex::new(BatchAccumulator::new(5, 2000))),
        }
    }
    
    /// Enqueue a task for processing
    pub fn enqueue(&self, task: CognitiveTask) -> Result<(), String> {
        self.sender.send(task)
            .map_err(|e| format!("Failed to enqueue task: {}", e))
    }
    
    /// Enqueue task with automatic batching for CommitteeEvaluation
    pub fn enqueue_with_batching(&self, task: CognitiveTask) -> Result<(), String> {
        // Only batch CommitteeEvaluation tasks
        if matches!(task.task_type, CognitiveTaskType::CommitteeEvaluation) {
            let mut accumulator = self.batch_accumulator.lock()
                .map_err(|e| format!("Lock failed: {}", e))?;

            if let Some(batch) = accumulator.add(task) {
                // Convert to BatchProcessing task
                let memory_ids: Vec<String> = batch.into_iter()
                    .map(|t| t.memory_id)
                    .collect();

                let batch_task = CognitiveTask::new(
                    format!("batch_{}", memory_ids.len()),
                    CognitiveTaskType::BatchProcessing(memory_ids),
                    5  // Medium priority
                );

                self.sender.send(batch_task)
                    .map_err(|e| format!("Failed to enqueue batch: {}", e))?;
            }
            Ok(())
        } else {
            // Non-batchable tasks go directly to queue
            self.enqueue(task)
        }
    }

    /// Flush any pending batches (call before shutdown)
    pub fn flush_batches(&self) -> Result<(), String> {
        let mut accumulator = self.batch_accumulator.lock()
            .map_err(|e| format!("Lock failed: {}", e))?;

        if let Some(batch) = accumulator.flush() {
            let memory_ids: Vec<String> = batch.into_iter()
                .map(|t| t.memory_id)
                .collect();

            let batch_task = CognitiveTask::new(
                format!("batch_flush_{}", memory_ids.len()),
                CognitiveTaskType::BatchProcessing(memory_ids),
                5
            );

            self.sender.send(batch_task)
                .map_err(|e| format!("Failed to enqueue flush batch: {}", e))?;
        }
        Ok(())
    }
    
    /// Dequeue a single task (blocks if empty)
    pub fn dequeue(&self) -> Result<CognitiveTask, String> {
        self.receiver.recv()
            .map_err(|e| format!("Failed to dequeue task: {}", e))
    }
    
    /// Try to dequeue without blocking
    pub fn try_dequeue(&self) -> Option<CognitiveTask> {
        self.receiver.try_recv().ok()
    }
    
    /// Batch dequeue up to `size` tasks
    pub fn batch_dequeue(&self, size: usize) -> Vec<CognitiveTask> {
        let mut batch = Vec::with_capacity(size);
        for _ in 0..size {
            match self.receiver.try_recv() {
                Ok(task) => batch.push(task),
                Err(_) => break,
            }
        }
        batch
    }
    
    /// Get approximate queue depth
    pub fn get_depth(&self) -> usize {
        self.receiver.len()
    }
}

impl Default for CognitiveProcessingQueue {
    fn default() -> Self {
        Self::new()
    }
}