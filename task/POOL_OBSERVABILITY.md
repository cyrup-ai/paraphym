# POOL_OBSERVABILITY

**Priority**: MEDIUM
**Component**: pool/core
**Estimated Effort**: 1 day
**Risk**: Low
**Dependencies**: None

## Problem Statement

Current observability gaps:
- No structured metrics collection
- Limited debug logging
- No performance profiling hooks
- Missing health check endpoints
- No request tracing

## Solution Design

### Metrics Collection

```rust
// pool/core/metrics.rs
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use dashmap::DashMap;

pub struct PoolMetricsCollector {
    // Per-model metrics
    model_metrics: DashMap<String, ModelMetrics>,

    // Global pool metrics
    total_requests: AtomicU64,
    total_errors: AtomicU64,
    total_spawns: AtomicU64,
    total_evictions: AtomicU64,
}

pub struct ModelMetrics {
    // Request metrics
    pub requests_total: AtomicU64,
    pub requests_failed: AtomicU64,
    pub requests_timeout: AtomicU64,

    // Latency tracking
    pub latency_sum_ms: AtomicU64,
    pub latency_count: AtomicU64,
    pub latency_max_ms: AtomicU64,

    // Worker metrics
    pub workers_spawned: AtomicU64,
    pub workers_evicted: AtomicU64,
    pub workers_crashed: AtomicU64,
    pub workers_current: AtomicUsize,

    // Queue metrics
    pub queue_depth_sum: AtomicU64,
    pub queue_depth_samples: AtomicU64,
    pub queue_depth_max: AtomicUsize,

    // Memory metrics
    pub memory_mb_allocated: AtomicUsize,
    pub memory_mb_peak: AtomicUsize,
}

impl PoolMetricsCollector {
    pub fn record_request(
        &self,
        registry_key: &str,
        duration: Duration,
        status: RequestStatus,
    ) {
        let metrics = self.model_metrics
            .entry(registry_key.to_string())
            .or_insert_with(ModelMetrics::default);

        metrics.requests_total.fetch_add(1, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);

        match status {
            RequestStatus::Success => {
                let latency_ms = duration.as_millis() as u64;
                metrics.latency_sum_ms.fetch_add(latency_ms, Ordering::Relaxed);
                metrics.latency_count.fetch_add(1, Ordering::Relaxed);

                // Update max latency
                let mut current = metrics.latency_max_ms.load(Ordering::Relaxed);
                while latency_ms > current {
                    match metrics.latency_max_ms.compare_exchange_weak(
                        current,
                        latency_ms,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => break,
                        Err(x) => current = x,
                    }
                }
            }
            RequestStatus::Failed => {
                metrics.requests_failed.fetch_add(1, Ordering::Relaxed);
                self.total_errors.fetch_add(1, Ordering::Relaxed);
            }
            RequestStatus::Timeout => {
                metrics.requests_timeout.fetch_add(1, Ordering::Relaxed);
                self.total_errors.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn get_prometheus_metrics(&self) -> String {
        let mut output = String::new();

        // Global metrics
        output.push_str(&format!(
            "pool_requests_total {{}} {}\n",
            self.total_requests.load(Ordering::Relaxed)
        ));
        output.push_str(&format!(
            "pool_errors_total {{}} {}\n",
            self.total_errors.load(Ordering::Relaxed)
        ));

        // Per-model metrics
        for entry in self.model_metrics.iter() {
            let (model, metrics) = (entry.key(), entry.value());

            output.push_str(&format!(
                "pool_model_requests_total {{model=\"{}\"}} {}\n",
                model, metrics.requests_total.load(Ordering::Relaxed)
            ));

            // Calculate average latency
            let sum = metrics.latency_sum_ms.load(Ordering::Relaxed);
            let count = metrics.latency_count.load(Ordering::Relaxed);
            if count > 0 {
                output.push_str(&format!(
                    "pool_model_latency_avg_ms {{model=\"{}\"}} {:.2}\n",
                    model, sum as f64 / count as f64
                ));
            }

            output.push_str(&format!(
                "pool_model_workers {{model=\"{}\"}} {}\n",
                model, metrics.workers_current.load(Ordering::Relaxed)
            ));
        }

        output
    }
}

pub enum RequestStatus {
    Success,
    Failed,
    Timeout,
}
```

### Structured Logging

```rust
// pool/core/logging.rs
use tracing::{info, warn, error, debug, instrument, span, Level};

#[instrument(skip(pool, spawn_fn), fields(model = %registry_key, memory_mb = %per_worker_mb))]
pub fn log_worker_spawn<F>(
    registry_key: &str,
    per_worker_mb: usize,
    spawn_fn: F,
) -> Result<(), PoolError>
where
    F: FnOnce() -> Result<(), PoolError>,
{
    let span = span!(Level::INFO, "spawn_worker");
    let _enter = span.enter();

    info!("Spawning worker for model");

    match spawn_fn() {
        Ok(()) => {
            info!("Worker spawned successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to spawn worker: {}", e);
            Err(e)
        }
    }
}

// Request lifecycle logging
pub fn log_request_start(registry_key: &str, request_id: u64) {
    debug!(
        model = %registry_key,
        request_id = %request_id,
        "Request started"
    );
}

pub fn log_request_complete(
    registry_key: &str,
    request_id: u64,
    duration: Duration,
    queue_time: Duration,
) {
    info!(
        model = %registry_key,
        request_id = %request_id,
        total_ms = %duration.as_millis(),
        queue_ms = %queue_time.as_millis(),
        "Request completed"
    );
}

pub fn log_worker_health(registry_key: &str, worker_id: usize, status: &str) {
    debug!(
        model = %registry_key,
        worker_id = %worker_id,
        status = %status,
        "Worker health check"
    );
}
```

### Health Check Endpoint

```rust
// pool/core/health.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct PoolHealth {
    pub status: HealthStatus,
    pub models: Vec<ModelHealth>,
    pub memory: MemoryHealth,
    pub timestamp: u64,
}

#[derive(Serialize)]
pub struct ModelHealth {
    pub registry_key: String,
    pub status: HealthStatus,
    pub workers: WorkerHealth,
    pub queue_depth: usize,
    pub avg_latency_ms: Option<f64>,
}

#[derive(Serialize)]
pub struct WorkerHealth {
    pub total: usize,
    pub busy: usize,
    pub idle: usize,
    pub crashed: usize,
}

#[derive(Serialize)]
pub struct MemoryHealth {
    pub used_mb: usize,
    pub total_mb: usize,
    pub percentage: f32,
}

#[derive(Serialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl<T> Pool<T> {
    pub fn get_health(&self) -> PoolHealth {
        let mut models = Vec::new();

        for entry in self.workers.iter() {
            let registry_key = entry.key();
            let workers = entry.value();

            let total = workers.len();
            let busy = workers.iter()
                .filter(|w| w.pending_requests.load(Ordering::Acquire) > 0)
                .count();
            let idle = total - busy;

            let model_health = ModelHealth {
                registry_key: registry_key.clone(),
                status: if total == 0 {
                    HealthStatus::Unhealthy
                } else if busy == total {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Healthy
                },
                workers: WorkerHealth {
                    total,
                    busy,
                    idle,
                    crashed: 0, // Track this in validate_workers
                },
                queue_depth: 0, // Would need channel introspection
                avg_latency_ms: self.metrics.get_avg_latency(registry_key),
            };

            models.push(model_health);
        }

        let used_mb = self.total_memory_mb();
        let total_mb = query_system_memory_mb();

        PoolHealth {
            status: if models.iter().all(|m| matches!(m.status, HealthStatus::Healthy)) {
                HealthStatus::Healthy
            } else if models.iter().any(|m| matches!(m.status, HealthStatus::Unhealthy)) {
                HealthStatus::Unhealthy
            } else {
                HealthStatus::Degraded
            },
            models,
            memory: MemoryHealth {
                used_mb,
                total_mb,
                percentage: (used_mb as f32 / total_mb as f32) * 100.0,
            },
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }
}
```

## Implementation Steps

1. **Add metrics.rs** with PoolMetricsCollector
2. **Add logging.rs** with structured logging
3. **Add health.rs** with health check implementation
4. **Integrate metrics** into request paths
5. **Add /metrics endpoint** for Prometheus
6. **Add /health endpoint** for monitoring
7. **Update worker loops** with health reporting

## Acceptance Criteria

- [ ] Prometheus-compatible metrics endpoint
- [ ] JSON health check endpoint
- [ ] Request tracing with correlation IDs
- [ ] Worker lifecycle logging
- [ ] Memory usage tracking
- [ ] Latency percentiles (p50, p95, p99)
- [ ] Queue depth monitoring

## Success Metrics

- Metrics overhead < 1% CPU
- Health checks < 10ms response time
- All critical paths instrumented