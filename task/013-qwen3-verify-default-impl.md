# Task 013: Add Default Trait Implementation for Qwen3 Quantized Model

## Priority: MEDIUM
## Status: NOT STARTED
## Created: 2025-10-19
## Updated: 2025-10-19 (Research Complete)

---

## Executive Summary

The `CandleQwen3QuantizedModel` struct **DOES NOT** have a Default trait implementation. This implementation must be added to enable static registry initialization via `LazyLock` in storage.rs. The implementation must follow the Kimi K2 pattern (not Phi4) because both Qwen3 and Kimi K2 have fallible `new()` methods that return `Result<Self, Error>`.

**Core Objective:** Add `impl Default for CandleQwen3QuantizedModel` after line 961 in qwen3_quantized.rs, then register the model in storage.rs.

---

## Current State Analysis

### File Location
**Primary File**: [`/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/qwen3_quantized.rs`](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs)

### Current Implementation (Lines 62-73)

```rust
pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
    // Create engine configuration using ModelInfo values
    let engine_config = EngineConfig::new("qwen3-quantized", "candle-qwen")
        .with_streaming()
        .with_max_tokens(32768)  // From QWEN3_QUANTIZED_MODEL_INFO
        .with_temperature(0.8);   // From QWEN3_QUANTIZED_MODEL_INFO

    let engine = Arc::new(Engine::new(engine_config)?);

    Ok(Self { engine })
}
```

**Key Finding**: `new()` returns a `Result`, meaning it can fail during `Engine::new()`.

### Missing Implementation

After line 961 (end of `impl CandleModel for LoadedQwen3QuantizedModel`), there is **NO** Default implementation:

```rust
// ❌ MISSING - Must add here (after line 961)
impl Default for CandleQwen3QuantizedModel {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            panic!("Failed to initialize Qwen3 Quantized model: {}", e)
        })
    }
}
```

---

## Pattern Analysis: Kimi K2 vs Phi4

### Pattern 1: Kimi K2 (CORRECT PATTERN for Qwen3)

**File**: [`/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/kimi_k2.rs:390-394`](../packages/candle/src/capability/text_to_text/kimi_k2.rs)

```rust
impl Default for CandleKimiK2Model {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| panic!("Failed to initialize Kimi K2 model: {}", e))
    }
}
```

**Why this pattern?**
- Kimi K2's `new()` returns `Result<Self, Box<dyn Error>>` (fallible)
- Uses `unwrap_or_else` to panic with **descriptive error message**
- Critical for debugging initialization failures

### Pattern 2: Phi4 (WRONG PATTERN for Qwen3)

**File**: [`/Volumes/samsung_t9/paraphym/packages/candle/src/capability/text_to_text/phi4_reasoning.rs:70-74`](../packages/candle/src/capability/text_to_text/phi4_reasoning.rs)

```rust
impl Default for CandlePhi4ReasoningModel {
    fn default() -> Self {
        Self::new()
    }
}
```

**Why NOT this pattern?**
- Phi4's `new()` returns `Self` directly (infallible) - see line 78
- No Result handling needed
- Qwen3 has fallible initialization, so cannot use this pattern

---

## Error Propagation: When Can Default Panic?

### Engine::new() Failure Cases

**File**: [`/Volumes/samsung_t9/paraphym/packages/candle/src/core/engine.rs:217-228`](../packages/candle/src/core/engine.rs)

```rust
pub fn new(config: EngineConfig) -> EngineResult<Self> {
    config.validate()?;  // ⚠️ Can fail here

    Ok(Self {
        config,
        request_count: Arc::new(AtomicU64::new(0)),
        active_requests: Arc::new(AtomicU64::new(0)),
        successful_requests: Arc::new(AtomicU64::new(0)),
        failed_requests: Arc::new(AtomicU64::new(0)),
        is_healthy: Arc::new(AtomicBool::new(true)),
    })
}
```

### EngineConfig::validate() Failure Modes

**File**: [`/Volumes/samsung_t9/paraphym/packages/candle/src/core/engine.rs:184-205`](../packages/candle/src/core/engine.rs)

The validate() method can return `EngineError::ConfigurationError` when:

1. **Empty registry_key** (lines 186-190)
   ```rust
   if self.registry_key.is_empty() {
       return Err(EngineError::ConfigurationError(
           "Model name cannot be empty".to_string(),
       ));
   }
   ```

2. **Empty provider** (lines 192-196)
   ```rust
   if self.provider.is_empty() {
       return Err(EngineError::ConfigurationError(
           "Provider cannot be empty".to_string(),
       ));
   }
   ```

3. **Zero timeout** (lines 198-202)
   ```rust
   if self.timeout_seconds == 0 {
       return Err(EngineError::ConfigurationError(
           "Timeout must be greater than 0".to_string(),
       ));
   }
   ```

4. **Invalid temperature** (lines 204-208)
   ```rust
   if let Some(temp) = self.temperature
       && !(0.0..=1.0).contains(&temp)
   {
       return Err(EngineError::ConfigurationError(
           "Temperature must be between 0.0 and 1.0".to_string(),
       ));
   }
   ```

**Analysis**: Qwen3's `new()` creates a valid config, so these errors are unlikely in practice. However, the Default impl must still handle potential errors to match the established pattern.

---

## Registry Integration

### Current Registry State

**File**: [`/Volumes/samsung_t9/paraphym/packages/candle/src/capability/registry/storage.rs:26-43`](../packages/candle/src/capability/registry/storage.rs)

```rust
pub(super) static TEXT_TO_TEXT_UNIFIED: LazyLock<RwLock<HashMap<String, TextToTextModel>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        let model = Arc::new(CandleKimiK2Model::default());  // ✅ Uses Default
        let key = model.info().registry_key.to_string();
        map.insert(key, TextToTextModel::KimiK2(model));

        let model = Arc::new(CandlePhi4ReasoningModel::default());  // ✅ Uses Default
        let key = model.info().registry_key.to_string();
        map.insert(key, TextToTextModel::Phi4Reasoning(model));

        RwLock::new(map)
    });
```

**Critical Finding**: Qwen3 is **NOT** registered in the TEXT_TO_TEXT_UNIFIED registry. This must be added after implementing Default.

---

## Implementation Steps

### Step 1: Add Default Implementation to qwen3_quantized.rs

**Location**: After line 961 (end of `impl CandleModel for LoadedQwen3QuantizedModel`)

**File**: [`qwen3_quantized.rs:962`](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs)

```rust
impl Default for CandleQwen3QuantizedModel {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            panic!("Failed to initialize Qwen3 Quantized model: {}", e)
        })
    }
}
```

**Why this exact pattern:**
- Uses `unwrap_or_else` instead of plain `unwrap()` for better error messages
- Follows Kimi K2 pattern precisely (proven working pattern)
- Panic is intentional - Default trait cannot return Result
- Error propagation happens at program start (LazyLock initialization)
- Descriptive panic message aids debugging

### Step 2: Import CandleQwen3QuantizedModel in storage.rs

**File**: [`storage.rs:10-15`](../packages/candle/src/capability/registry/storage.rs)

Add to imports section:

```rust
use crate::capability::text_to_text::{
    CandleKimiK2Model, 
    CandlePhi4ReasoningModel,
    CandleQwen3QuantizedModel,  // ✅ Add this import
};
```

### Step 3: Register Qwen3 in TEXT_TO_TEXT_UNIFIED

**File**: [`storage.rs:26-43`](../packages/candle/src/capability/registry/storage.rs)

Add after Phi4 registration (line 42):

```rust
pub(super) static TEXT_TO_TEXT_UNIFIED: LazyLock<RwLock<HashMap<String, TextToTextModel>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        let model = Arc::new(CandleKimiK2Model::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextToTextModel::KimiK2(model));

        let model = Arc::new(CandlePhi4ReasoningModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextToTextModel::Phi4Reasoning(model));

        // ✅ Add Qwen3 registration
        let model = Arc::new(CandleQwen3QuantizedModel::default());
        let key = model.info().registry_key.to_string();
        map.insert(key, TextToTextModel::Qwen3Quantized(model));

        RwLock::new(map)
    });
```

### Step 4: Add Qwen3Quantized Variant to TextToTextModel Enum

**File**: [`enums.rs`](../packages/candle/src/capability/registry/enums.rs) (check current enum definition)

Add variant to the `TextToTextModel` enum:

```rust
pub enum TextToTextModel {
    KimiK2(Arc<CandleKimiK2Model>),
    Phi4Reasoning(Arc<CandlePhi4ReasoningModel>),
    Qwen3Quantized(Arc<CandleQwen3QuantizedModel>),  // ✅ Add this variant
}
```

---

## Expected Behavior After Implementation

### Successful Initialization

1. Program starts → LazyLock initializes TEXT_TO_TEXT_UNIFIED
2. Qwen3 Default::default() is called
3. CandleQwen3QuantizedModel::new() succeeds
4. Engine::new(config) creates engine successfully
5. Model registered with key "qwen-3" (from QWEN3_QUANTIZED_MODEL_INFO.registry_key)
6. Registry lookup works: `registry::get::<TextToTextModel>("qwen-3")` returns Some(model)

### Initialization Failure (Panic with Descriptive Error)

If Engine::new() fails due to config validation:

```
thread 'main' panicked at packages/candle/src/capability/text_to_text/qwen3_quantized.rs:964:
Failed to initialize Qwen3 Quantized model: Configuration error: Model name cannot be empty
```

This is **correct behavior** - the program should not continue if core models fail to initialize.

---

## Definition of Done

1. ✅ Default trait implementation added to CandleQwen3QuantizedModel after line 961 in qwen3_quantized.rs
2. ✅ Implementation follows Kimi K2 pattern exactly (unwrap_or_else with descriptive panic)
3. ✅ Import added to storage.rs
4. ✅ Qwen3 registration added to TEXT_TO_TEXT_UNIFIED LazyLock
5. ✅ Qwen3Quantized variant added to TextToTextModel enum
6. ✅ Cargo check passes with no compilation errors
7. ✅ Registry initialization succeeds without panics
8. ✅ Model accessible via `registry::get::<TextToTextModel>("qwen-3")`

---

## Files Modified

1. [`packages/candle/src/capability/text_to_text/qwen3_quantized.rs`](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs)
   - Add Default impl after line 961

2. [`packages/candle/src/capability/registry/storage.rs`](../packages/candle/src/capability/registry/storage.rs)
   - Import CandleQwen3QuantizedModel
   - Register model in TEXT_TO_TEXT_UNIFIED

3. [`packages/candle/src/capability/registry/enums.rs`](../packages/candle/src/capability/registry/enums.rs)
   - Add Qwen3Quantized variant to TextToTextModel enum

---

## Code Reference Summary

| Component | File | Lines | Purpose |
|-----------|------|-------|---------|
| Qwen3 struct | [qwen3_quantized.rs](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs) | 35-38 | Model definition |
| Qwen3 new() | [qwen3_quantized.rs](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs) | 62-73 | Fallible constructor |
| Kimi K2 Default | [kimi_k2.rs](../packages/candle/src/capability/text_to_text/kimi_k2.rs) | 390-394 | Reference pattern |
| Phi4 Default | [phi4_reasoning.rs](../packages/candle/src/capability/text_to_text/phi4_reasoning.rs) | 70-74 | Wrong pattern (infallible) |
| Engine::new() | [engine.rs](../packages/candle/src/core/engine.rs) | 217-228 | Can fail on validation |
| config.validate() | [engine.rs](../packages/candle/src/core/engine.rs) | 184-205 | 4 failure modes |
| Registry init | [storage.rs](../packages/candle/src/capability/registry/storage.rs) | 26-43 | Uses Default trait |

---

## Notes

- No third-party libraries needed - all code is in-tree
- The Default trait panic behavior is intentional - models must initialize at startup
- The unwrap_or_else pattern provides better error messages than plain unwrap()
- This task completes the refactoring started in a previous PR where Default was planned but not implemented
