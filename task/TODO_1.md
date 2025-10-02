# TODO_1: Implement GPU Device Detection in Providers

## OBJECTIVE
Replace hardcoded `Device::Cpu` with intelligent GPU detection in language model providers.

## PRIORITY
🟡 HIGH - Performance optimization

## BACKGROUND
Both Qwen3 and Kimi K2 providers hardcode CPU device selection with TODO comments indicating GPU detection is needed. This limits performance on GPU-enabled systems.

## SUBTASK 1: Implement Device Detection Utility
**File:** Create or update device detection utility  
**Location:** `packages/candle/src/core/device_util.rs` or similar

**Requirements:**
- Detect CUDA availability
- Detect Metal availability (macOS)
- Fall back to CPU if no GPU available
- Return appropriate `Device` enum

**Example implementation:**
```rust
pub fn detect_best_device() -> Result<Device> {
    #[cfg(feature = "cuda")]
    {
        if candle_core::utils::cuda_is_available() {
            return Ok(Device::new_cuda(0)?);
        }
    }
    
    #[cfg(feature = "metal")]
    {
        if candle_core::utils::metal_is_available() {
            return Ok(Device::new_metal(0)?);
        }
    }
    
    Ok(Device::Cpu)
}
```

## SUBTASK 2: Update Qwen3 Provider
**File:** `packages/candle/src/providers/qwen3_coder.rs`  
**Line:** 512

**Current code:**
```rust
let device = Device::Cpu; // TODO: Add GPU detection
```

**Required replacement:**
```rust
let device = crate::core::device_util::detect_best_device()
    .unwrap_or_else(|e| {
        log::warn!("GPU detection failed, falling back to CPU: {}", e);
        Device::Cpu
    });
```

## SUBTASK 3: Update Kimi K2 Provider
**File:** `packages/candle/src/providers/kimi_k2.rs`  
**Line:** 416

**Current code:**
```rust
let device = Device::Cpu; // TODO: Add GPU detection
```

**Required replacement:**
```rust
let device = crate::core::device_util::detect_best_device()
    .unwrap_or_else(|e| {
        log::warn!("GPU detection failed, falling back to CPU: {}", e);
        Device::Cpu
    });
```

## SUBTASK 4: Add Device Logging
**Action:** Log which device is selected for debugging

**Requirements:**
- Log at INFO level when GPU is detected and used
- Log at WARN level when falling back to CPU due to error
- Include device details (CUDA/Metal/CPU)

## DEFINITION OF DONE
- [ ] Device detection utility implemented
- [ ] Qwen3 provider uses dynamic device selection
- [ ] Kimi K2 provider uses dynamic device selection
- [ ] TODO comments removed
- [ ] Device selection logged appropriately
- [ ] Falls back gracefully to CPU on detection failure
- [ ] Code compiles without warnings

## CONSTRAINTS
- ❌ DO NOT write unit tests
- ❌ DO NOT write integration tests
- ❌ DO NOT write benchmarks
- ✅ Focus solely on ./src modifications

## TECHNICAL NOTES
- GPU detection should be safe and never panic
- CPU fallback ensures compatibility on all systems
- Consider feature flags: `cuda`, `metal`
- Device selection impacts model load time and inference speed significantly
- Log device selection for user awareness
