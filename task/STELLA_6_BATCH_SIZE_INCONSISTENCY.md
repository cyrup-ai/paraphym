# Issue: Inconsistent Batch Size Recommendations

## Severity: MEDIUM
**Impact**: Suboptimal performance, user confusion

## Location
- `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/base.rs:374-385`
- `/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs:192-204, 369-381`

## Problem Description

Different batch size recommendations exist across implementations:

### base.rs (Lines 374-385)
```rust
fn recommended_batch_size(&self) -> usize {
    match detect_variant(self.info().registry_key) {
        ModelVariant::Large => 8,   // 1.5B model
        ModelVariant::Small => 16,  // 400M model
    }
}

fn max_batch_size(&self) -> usize {
    match detect_variant(self.info().registry_key) {
        ModelVariant::Large => 32,  // 1.5B model
        ModelVariant::Small => 64,  // 400M model
    }
}
```

### loaded.rs (Lines 192-204) - Used in load()
```rust
pub fn recommended_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 2,   // ← DIFFERENT! (was 8)
        ModelVariant::Small => 8,   // ← DIFFERENT! (was 16)
    }
}

pub fn max_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 8,   // ← DIFFERENT! (was 32)
        ModelVariant::Small => 32,  // ← DIFFERENT! (was 64)
    }
}
```

### loaded.rs (Lines 369-381) - Used in trait impl
```rust
fn recommended_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 8,   // ← DIFFERENT AGAIN! (matches base.rs)
        ModelVariant::Small => 16,  // ← DIFFERENT AGAIN! (matches base.rs)
    }
}

fn max_batch_size(&self) -> usize {
    match self.variant {
        ModelVariant::Large => 32,  // ← DIFFERENT AGAIN! (matches base.rs)
        ModelVariant::Small => 64,  // ← DIFFERENT AGAIN! (matches base.rs)
    }
}
```

## Summary of Values

| Implementation | Model | Recommended | Max |
|---------------|-------|-------------|-----|
| base.rs trait | Large | 8 | 32 |
| base.rs trait | Small | 16 | 64 |
| loaded.rs methods | Large | 2 | 8 |
| loaded.rs methods | Small | 8 | 32 |
| loaded.rs trait | Large | 8 | 32 |
| loaded.rs trait | Small | 16 | 64 |

## Impact

1. **Confusion**: Three different sets of values for the same models
2. **Suboptimal Performance**: Conservative values (2, 8) might be too small
3. **Inconsistent Behavior**: Depending on which method is called, different limits apply

## Root Cause

The `loaded.rs` has **two different implementations**:
- Public methods (lines 192-204): Conservative values
- Trait implementation (lines 369-381): Matches base.rs

This suggests the public methods were added later with more conservative estimates.

## Recommendation

**Unify the values** based on actual benchmarking:

```rust
// For 400M model on typical GPU (8GB VRAM):
recommended_batch_size: 16
max_batch_size: 32

// For 1.5B model on typical GPU (8GB VRAM):
recommended_batch_size: 4
max_batch_size: 8
```

Remove the duplicate public methods in `loaded.rs` and only keep the trait implementation.

## Testing Needed

Run actual benchmarks to determine optimal batch sizes for:
- CPU inference
- CUDA inference (different VRAM sizes)
- Metal inference (M1/M2/M3)

Batch size should maximize throughput without OOM.
