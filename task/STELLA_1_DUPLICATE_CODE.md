# STELLA_1: Eliminate Massive Code Duplication

## Severity: HIGH  
**Impact**: Maintenance burden, inconsistency risk, binary bloat (~400 duplicated lines)

## Core Objective

Extract ~400 lines of duplicated code between `base.rs` and `loaded.rs` into shared utility functions in a new `utils.rs` file.

**Source Files**:
- [`base.rs`](../packages/candle/src/capability/text_embedding/stella/base.rs) - 390 lines
- [`loaded.rs`](../packages/candle/src/capability/text_embedding/stella/loaded.rs) - 385 lines
- **Duplication**: ~250 lines (64% overlap)

## Duplication Analysis

### 1. Device/DType Detection (4 instances)
**Locations**: base.rs:78-87, base.rs:243-251, loaded.rs:79-90

```rust
let device = crate::core::device_util::detect_best_device().unwrap_or_else(|e| {
    log::warn!("Device detection failed: {}. Using CPU.", e);
    Device::Cpu
});
let dtype = if device.is_cuda() { DType::F16 } else { DType::F32 };
```

### 2. Tokenizer Configuration (3 instances)
**Locations**: base.rs:107-144, base.rs:271-303, loaded.rs:108-145

Variant-specific padding (Large=Left with `