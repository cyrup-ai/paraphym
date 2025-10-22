# Issue: Device Cloned Unnecessarily

## Severity: LOW
**Impact**: Minor inefficiency, code clarity issue

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/loaded.rs:228`

## Problem Description

The `Device` is cloned before being moved into `spawn_blocking`:

```rust
fn embed(&self, text: &str, task: Option<String>) -> ... {
    let device = self.device.clone();  // ← Clone device
    
    Box::pin(async move {
        tokio::task::spawn_blocking(move || {
            let input_ids = Tensor::new(tokens.get_ids(), &device)?;
            // ...
        })
    })
}
```

## Analysis

Looking at Candle's `Device` enum:
```rust
pub enum Device {
    Cpu,
    Cuda(CudaDevice),
    Metal(MetalDevice),
}
```

- `Device::Cpu` - zero-size variant, clone is free
- `Device::Cuda(CudaDevice)` - contains `Arc<CudaDeviceInner>`, clone is cheap
- `Device::Metal(MetalDevice)` - contains `Arc<MetalDeviceInner>`, clone is cheap

## Impact

**Minimal** - Device clone is already cheap (Arc-based). However:

1. **Code clarity**: Readers might think Device is expensive to clone
2. **Consistency**: We're cloning something that's already cheap to copy
3. **Future-proofing**: If Device implementation changes, we're already safe

## Recommendation

**Keep as-is** - the clone is already cheap and makes the code's intent clear (we're moving a copy into the closure).

Alternatively, document that Device is cheap to clone:
```rust
let device = self.device.clone();  // Device is Arc-based, cheap to clone
```

## Non-Issue

This is **not a performance problem**, just a code clarity observation.
