use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::pool::capabilities::{
    image_embedding_pool, text_embedding_pool, text_to_image_pool, text_to_text_pool,
    vision_pool,
};
use crate::pool::core::types::WorkerHandle;
use crate::pool::core::Pool;

/// Check if all workers for a model are idle
///
/// A worker is considered idle if:
/// - It has no pending requests (pending_requests == 0)
/// - It hasn't been used for at least idle_threshold_secs seconds
fn all_workers_idle(workers: &[WorkerHandle], idle_threshold_secs: u64) -> bool {
    if workers.is_empty() {
        return false;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    workers.iter().all(|w| {
        let pending = w.pending_requests.load(Ordering::Acquire);
        let last_used = w.last_used.load(Ordering::Acquire);
        let idle_duration = now.saturating_sub(last_used);

        pending == 0 && idle_duration >= idle_threshold_secs
    })
}

/// Find least recently used (LRU) worker
///
/// Returns the index of the worker with the oldest last_used timestamp.
/// Returns None if workers vector is empty.
fn find_lru_worker(workers: &[WorkerHandle]) -> Option<usize> {
    workers
        .iter()
        .enumerate()
        .min_by_key(|(_, w)| w.last_used.load(Ordering::Acquire))
        .map(|(idx, _)| idx)
}

/// Evict worker from pool
///
/// Removes the worker at the specified index, sends shutdown signal,
/// and updates memory tracking.
///
/// # Arguments
/// * `pool` - The pool to evict from
/// * `registry_key` - Model registry key
/// * `worker_idx` - Index of worker to evict
/// * `per_worker_mb` - Memory footprint of worker in MB
///
/// # Returns
/// Ok(()) on success, Err with description on failure
fn evict_worker<T: ?Sized>(
    pool: &Pool<T>,
    registry_key: &str,
    worker_idx: usize,
    per_worker_mb: usize,
) -> Result<(), String> {
    // Get mutable access to workers vector
    let mut workers_guard = pool
        .workers()
        .get_mut(registry_key)
        .ok_or_else(|| format!("No workers for {}", registry_key))?;

    if worker_idx >= workers_guard.len() {
        return Err(format!(
            "Worker index {} out of bounds (len: {})",
            worker_idx,
            workers_guard.len()
        ));
    }

    // Remove worker from vector
    let worker = workers_guard.remove(worker_idx);
    let remaining_count = workers_guard.len();
    drop(workers_guard); // Release lock

    // Send shutdown signal to worker thread
    // Worker loop will receive signal and break
    if let Err(e) = worker.shutdown_tx.send(()) {
        log::warn!(
            "Failed to send shutdown signal to worker {}: {}",
            worker.worker_id,
            e
        );
    }

    // Update memory tracking
    pool.remove_memory(per_worker_mb);

    // Update metrics
    pool.metrics().workers_evicted.fetch_add(1, Ordering::Release);

    log::info!(
        "Evicted worker {} from {} (idle cooldown), {} workers remain",
        worker.worker_id,
        registry_key,
        remaining_count
    );

    Ok(())
}

/// Validate health of all workers in a pool
///
/// Checks each worker's health and removes dead workers.
/// Should be called before idle eviction to ensure accurate state.
fn validate_pool_health<T: ?Sized>(
    pool: &'static Pool<T>,
    pool_name: &str,
) {
    for entry in pool.workers().iter() {
        let registry_key = entry.key();
        let removed = pool.validate_workers(registry_key);
        
        if removed > 0 {
            log::warn!(
                "[{}] Removed {} dead workers for model '{}'",
                pool_name,
                removed,
                registry_key
            );
        }
    }
}

/// Process maintenance for one pool
///
/// Iterates over all models in the pool and evicts one LRU worker
/// per idle model.
fn process_pool_maintenance<T: ?Sized>(
    pool: &'static Pool<T>,
    idle_threshold_secs: u64,
    pool_name: &str,
) {
    // Collect models that need eviction (to avoid holding locks)
    let mut models_to_evict = Vec::new();

    // Scan all models in pool
    for entry in pool.workers().iter() {
        let registry_key = entry.key().clone();
        let workers = entry.value();

        // Check if all workers are idle and there's at least one worker
        if all_workers_idle(workers, idle_threshold_secs) && !workers.is_empty() {
            // Find LRU worker index
            if let Some(lru_idx) = find_lru_worker(workers) {
                models_to_evict.push((registry_key, lru_idx));
            }
        }
    }

    // Perform evictions (after releasing iterator locks)
    for (registry_key, lru_idx) in models_to_evict {
        // Get per_worker_mb from the worker handle
        let per_worker_mb = pool
            .workers()
            .get(&registry_key)
            .and_then(|workers| workers.get(lru_idx).map(|w| w.per_worker_mb))
            .unwrap_or(1024); // Default fallback if worker not found

        log::debug!(
            "{} pool: All workers idle for {}, evicting LRU worker at index {} ({} MB)",
            pool_name,
            registry_key,
            lru_idx,
            per_worker_mb
        );

        if let Err(e) = evict_worker(pool, &registry_key, lru_idx, per_worker_mb) {
            log::warn!(
                "Failed to evict worker from {} pool ({}): {}",
                pool_name,
                registry_key,
                e
            );
        }
    }
}

/// Log current memory usage across all pools
fn log_memory_usage() {
    let text_embedding_mb = text_embedding_pool().total_memory_mb();
    let text_to_text_mb = text_to_text_pool().total_memory_mb();
    let image_embedding_mb = image_embedding_pool().total_memory_mb();
    let vision_mb = vision_pool().total_memory_mb();
    let text_to_image_mb = text_to_image_pool().total_memory_mb();

    let total_mb = text_embedding_mb
        + text_to_text_mb
        + image_embedding_mb
        + vision_mb
        + text_to_image_mb;

    log::debug!(
        "Pool memory usage: {} MB (TextEmbedding: {}, TextToText: {}, ImageEmbedding: {}, Vision: {}, TextToImage: {})",
        total_mb,
        text_embedding_mb,
        text_to_text_mb,
        image_embedding_mb,
        vision_mb,
        text_to_image_mb
    );
}

/// Start maintenance thread for all pools
///
/// Runs every 1 minute (configurable via pool config):
/// - Check each pool for idle workers
/// - Evict 1 LRU worker per idle model
/// - Monitor system memory pressure
/// - Log eviction events
///
/// The thread continues until all pools signal shutdown.
pub fn start_maintenance_thread() -> Result<thread::JoinHandle<()>, String> {
    thread::Builder::new()
        .name("pool-maintenance".to_string())
        .spawn(move || {
            // Get interval from config (default 60 seconds)
            let config = text_embedding_pool().config();
            let interval = Duration::from_secs(config.maintenance_interval_secs);
            let idle_threshold = config.cooldown_idle_minutes * 60; // Convert minutes to seconds

            log::info!(
                "Maintenance thread started (interval: {}s, idle_threshold: {}s)",
                interval.as_secs(),
                idle_threshold
            );

            loop {
                thread::sleep(interval);

                // Check if shutting down (check all pools, exit if any shutting down)
                if text_embedding_pool().is_shutting_down()
                    || text_to_text_pool().is_shutting_down()
                    || image_embedding_pool().is_shutting_down()
                    || vision_pool().is_shutting_down()
                    || text_to_image_pool().is_shutting_down()
                {
                    log::info!("Maintenance thread shutting down");
                    break;
                }

                // Validate worker health (remove dead workers)
                validate_pool_health(text_embedding_pool(), "TextEmbedding");
                validate_pool_health(text_to_text_pool(), "TextToText");
                validate_pool_health(image_embedding_pool(), "ImageEmbedding");
                validate_pool_health(vision_pool(), "Vision");
                validate_pool_health(text_to_image_pool(), "TextToImage");

                // Process each pool (evict idle workers)
                process_pool_maintenance(text_embedding_pool(), idle_threshold, "TextEmbedding");
                process_pool_maintenance(text_to_text_pool(), idle_threshold, "TextToText");
                process_pool_maintenance(
                    image_embedding_pool(),
                    idle_threshold,
                    "ImageEmbedding",
                );
                process_pool_maintenance(vision_pool(), idle_threshold, "Vision");
                process_pool_maintenance(text_to_image_pool(), idle_threshold, "TextToImage");

                // Log memory usage
                log_memory_usage();
            }

            log::info!("Maintenance thread exited");
        })
        .map_err(|e| format!("Failed to spawn maintenance thread: {}", e))
}
