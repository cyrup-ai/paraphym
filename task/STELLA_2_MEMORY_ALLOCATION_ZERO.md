# Issue: Memory Allocation Set to Zero - Critical Bug

## Severity: CRITICAL
**Impact**: Pool memory governor completely bypassed, potential OOM crashes

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/config.rs:44`

## Problem Description

The `est_memory_allocation_mb` field in `STELLA_MODEL_INFO` is hardcoded to `0`:

```rust
pub(crate) static STELLA_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    // ... other fields ...
    est_memory_allocation_mb: 0,  // ← CRITICAL BUG
};
```

## Impact

1. **Memory Governor Bypass**: The pool's memory governor uses this value to track and limit memory usage. With `0`, it thinks workers use no memory.

2. **Unlimited Worker Spawning**: The pool can spawn unlimited workers since each "costs" 0 MB.

3. **OOM Risk**: On systems with limited memory, this can cause out-of-memory crashes.

4. **Log Evidence**: From the logs:
   ```
   [INFO] Allocated 0 MB (total: 0 MB)
   [INFO] Released 0 MB via AllocationGuard (total: 0 MB)
   ```

## Expected Values

From official documentation (`/Volumes/samsung_t9/cyrup/docs/models/text_embedding/STELLA-EN-1.5B-V5.md`):

- **stella_en_1.5B_v5**: **7200 MB** (1.54B parameters, FP32, 6.17 GB weights + overhead)
- **stella_en_400M_v5**: ~1600 MB (estimated: 400M parameters × 4 bytes/param + overhead)

## Fix Required

The config currently defines only the 400M variant. We need TWO separate ModelInfo structs:

```rust
// For 400M variant (default)
pub(crate) static STELLA_400M_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: CandleProvider::Dunzhang,
    name: "stella_en_400M_v5",
    registry_key: "dunzhang/stella_en_400M_v5",
    // ... other fields ...
    est_memory_allocation_mb: 1600,  // 400M × 4 bytes + overhead
};

// For 1.5B variant
pub(crate) static STELLA_1_5B_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: CandleProvider::Dunzhang,
    name: "stella_en_1.5B_v5",
    registry_key: "dunzhang/stella_en_1.5B_v5",
    // ... other fields ...
    est_memory_allocation_mb: 7200,  // Per official docs
};
```

Then update `StellaEmbeddingModel::info()` to return the correct one based on variant.

## Related Code

The memory governor allocation code in `pool/core/spawn.rs`:
```rust
let allocation_guard = governor
    .try_allocate(per_worker_mb)  // ← per_worker_mb comes from est_memory_allocation_mb
    .await
```

With `per_worker_mb = 0`, this always succeeds regardless of actual memory usage.
