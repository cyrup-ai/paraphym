# MPOOL_6A Implementation Summary

## Overview
Implemented background maintenance thread for worker pool eviction and memory monitoring as specified in MPOOL_6A.

## Files Created
- `src/pool/maintenance.rs` (248 lines) - Core maintenance logic

## Files Modified
1. `src/pool/core/types.rs` - Enhanced WorkerHandle with shutdown_tx and per_worker_mb
2. `src/pool/core/pool.rs` - Added workers() accessor for maintenance operations
3. `src/pool/mod.rs` - Added maintenance module and lazy initialization
4. `src/pool/capabilities/text_embedding.rs` - Updated WorkerHandle construction
5. `src/pool/capabilities/text_to_text.rs` - Updated WorkerHandle construction
6. `src/pool/capabilities/image_embedding.rs` - Updated WorkerHandle construction
7. `src/pool/capabilities/vision.rs` - Updated WorkerHandle construction
8. `src/pool/capabilities/text_to_image.rs` - Updated WorkerHandle construction

## Key Features

### 1. Background Maintenance Thread
- Runs every 60 seconds (configurable via PoolConfig.maintenance_interval_secs)
- Coordinates across all 5 pool instances (TextEmbedding, TextToText, ImageEmbedding, Vision, TextToImage)
- Graceful shutdown detection via is_shutting_down() checks
- Lazy initialization using once_cell::sync::Lazy

### 2. Worker Eviction Logic
- **all_workers_idle()**: Checks if ALL workers have pending_requests==0 AND idle_duration >= 60 seconds
- **find_lru_worker()**: Selects worker with oldest last_used timestamp
- **evict_worker()**: Removes worker, sends shutdown signal, updates memory tracking
- **Gradual scaling**: 4→3→2→1→0 workers (1 eviction per model per minute)

### 3. Memory Tracking
- WorkerHandle stores per_worker_mb for accurate tracking
- Maintenance reads per_worker_mb directly from worker handle
- Memory decremented atomically during eviction
- Total memory usage logged across all pools

### 4. Production-Grade Quality
- ✅ NO unwrap() in src/* (replaced with .unwrap_or(0) for time errors)
- ✅ NO expect() in src/* (replaced with .map_err() and Result returns)
- ✅ Lock-free design with DashMap and atomic operations
- ✅ Proper error handling with Result types
- ✅ Comprehensive logging (debug, info, warn)
- ✅ Metrics tracking (workers_evicted counter)

### 5. Design Principles
- ✅ ZERO model-specific logic in pool.rs
- ✅ Generic over capability trait T
- ✅ No GteQwen, JinaBert, NvEmbed, etc. references
- ✅ Uses pool accessor functions, not direct model access

## Definition of Done - All 9 Items Complete

1. ✅ `maintenance.rs` file created
2. ✅ `all_workers_idle()` function implemented
3. ✅ `find_lru_worker()` function implemented
4. ✅ `evict_worker()` function implemented
5. ✅ `start_maintenance_thread()` function implemented
6. ✅ `process_pool_maintenance()` function implemented
7. ✅ `log_memory_usage()` function implemented
8. ✅ Maintenance thread initialization added to pool/mod.rs
9. ✅ `shutdown_tx` channel added to WorkerHandle

## Cooldown Policy Implementation (Scenario 5)

```
t=5:00  Last request completes, all 4 workers idle
t=6:00  All idle 1 min → evict 1 worker (3 remain)
t=7:00  All idle 1 min → evict 1 worker (2 remain)
t=7:30  NEW REQUEST → cooldown resets, 2 workers stay
t=8:30  Request done, 2 workers idle
t=9:30  All idle 1 min → evict 1 worker (1 remains)
t=10:30 All idle 1 min → evict last worker (0 remain)
```

This policy is implemented via:
- Maintenance thread wakes every 60 seconds
- all_workers_idle() checks pending_requests==0 AND idle_duration >= 60 seconds
- Only evicts if ALL workers for a model are idle
- Evicts 1 LRU worker per idle model per iteration
- Any new request resets last_used timestamp via touch()

## Usage

The maintenance thread starts automatically on first pool access:

```rust
use candle::pool::{text_embedding_pool, init_maintenance};

// Option 1: Force initialization explicitly
init_maintenance();

// Option 2: Let lazy initialization happen on first pool use
let pool = text_embedding_pool();
```

## Implementation Notes

1. **Lock-Free Coordination**: Uses DashMap for lock-free concurrent access to worker vectors
2. **Two-Phase Eviction**: Collects eviction candidates first, then performs evictions after releasing iterator locks
3. **Graceful Shutdown**: Workers receive shutdown signal via crossbeam::channel and exit cleanly
4. **Memory Safety**: All atomic operations use proper Ordering (Acquire for reads, Release for writes)
5. **Error Recovery**: Failed evictions logged but don't crash maintenance thread

## Constraints Satisfied

- ✅ NO TESTS in same file (tests handled separately)
- ✅ NO BENCHMARKS in same file (benchmarks handled separately)
- ✅ Gradual eviction: 1 worker per model per minute
- ✅ Coordinated: Single thread for all 5 pools
- ✅ Zero allocation where possible
- ✅ Blazing-fast atomic operations
- ✅ No unsafe code
- ✅ Elegant ergonomic design
