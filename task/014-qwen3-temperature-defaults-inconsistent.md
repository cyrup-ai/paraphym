# Task 014: Standardize Temperature Default to 0.0 Universally

## Priority: HIGH
## Status: NOT STARTED
## Created: 2025-10-19
## Updated: 2025-10-19

---

## Core Objective

**Standardize all temperature defaults to 0.0 across the entire codebase** to ensure deterministic, predictable output from all AI models. This eliminates behavioral inconsistency and aligns with production best practices for AI systems.

---

## Why Temperature 0.0?

### Temperature Values Explained

| Temperature | Behavior | Use Case |
|-------------|----------|----------|
| **0.0** | **Greedy sampling** - Always picks highest probability token | **Deterministic, predictable, production-ready** |
| 0.7 | Creative but controlled | Casual chat applications |
| 0.8 | More creative | Brainstorming, ideation |
| 1.0+ | Highly random | Fiction writing, games |

### Production Rationale

1. **Determinism**: Temperature 0.0 produces the same output for the same input (critical for testing, debugging, reproducibility)
2. **Quality**: Picks the most confident/likely token at each step (highest quality for factual tasks)
3. **Speed**: Greedy sampling is computationally faster than probabilistic sampling
4. **Alignment**: Matches industry best practices for reasoning models (see PHI-4)
5. **Existing Pattern**: CLI already defaults to 0.0 ([./src/cli/args.rs:58](../packages/candle/src/cli/args.rs))

### Reference: PHI-4 Reasoning Model

The PHI-4 reasoning model documentation explicitly requires:

```
temperature: 0.0
top_p: 1.0
repeat_penalty: 1.0
```

Source: [./docs/models/text_to_text/PHI-4-REASONING-Q4_K_M.md:93](../docs/models/text_to_text/PHI-4-REASONING-Q4_K_M.md)

**Reasoning models require deterministic sampling** to maintain logical coherence in chain-of-thought reasoning.

---

## Current State Analysis

### Files to Modify

#### **1. Qwen3 Quantized Implementation**

**File**: [`./packages/candle/src/capability/text_to_text/qwen3_quantized.rs`](../packages/candle/src/capability/text_to_text/qwen3_quantized.rs)

**Current State**: Mixed defaults (0.7, 0.8, inconsistent fallbacks)

**Changes Required**:

##### Location 1: MODEL_INFO Struct (Line 96)
```rust
// CURRENT (WRONG):
default_temperature: Some(0.8),

// CHANGE TO:
default_temperature: Some(0.0),  // Greedy sampling for deterministic output
```

##### Location 2: CandleQwen3QuantizedModel::prompt() (Line 128)
```rust
// CURRENT (WRONG):
self.info().default_temperature.unwrap_or(0.7)

// CHANGE TO:
self.info().default_temperature.unwrap_or(0.0)
```

##### Location 3: Engine Config Initialization (Line 60)
```rust
// CURRENT (WRONG):
.with_temperature(0.8);   // From QWEN3_QUANTIZED_MODEL_INFO

// CHANGE TO:
.with_temperature(0.0);   // Greedy sampling for deterministic output
```

##### Location 4: LoadedQwen3QuantizedModel::prompt() (Line 646)
```rust
// CURRENT (CORRECT, but should document why):
QWEN3_QUANTIZED_MODEL_INFO.default_temperature.unwrap_or(0.0)

// KEEP AS-IS (already correct)
```

**Impact**: After these changes, Qwen3 will produce deterministic output by default, matching CLI behavior and PHI-4 standards.

---

#### **2. KimiK2 Model Implementation**

**File**: [`./packages/candle/src/capability/text_to_text/kimi_k2.rs`](../packages/candle/src/capability/text_to_text/kimi_k2.rs)

**Current State**: Uses 0.7 as default

##### Location 1: Static MODEL_INFO (Line 342)
```rust
// CURRENT (WRONG):
default_temperature: Some(0.7),

// CHANGE TO:
default_temperature: Some(0.0),  // Greedy sampling for deterministic output
```

##### Location 2: new() method fallback (Line 47)
```rust
// CURRENT (WRONG):
let default_temperature = KIMI_K2_MODEL_INFO.default_temperature.unwrap_or(0.7);

// CHANGE TO:
let default_temperature = KIMI_K2_MODEL_INFO.default_temperature.unwrap_or(0.0);
```

##### Location 3: prompt() method fallback (Line 550)
```rust
// CURRENT (WRONG):
KIMI_K2_MODEL_INFO.default_temperature.unwrap_or(0.7)

// CHANGE TO:
KIMI_K2_MODEL_INFO.default_temperature.unwrap_or(0.0)
```

---

#### **3. PHI-4 Reasoning Model Implementation**

**File**: [`./packages/candle/src/capability/text_to_text/phi4_reasoning.rs`](../packages/candle/src/capability/text_to_text/phi4_reasoning.rs)

**Current State**: Uses 0.7 (WRONG for reasoning model!)

##### Location 1: Static MODEL_INFO (Line 322)
```rust
// CURRENT (WRONG):
default_temperature: Some(0.7),

// CHANGE TO:
default_temperature: Some(0.0),  // REQUIRED for reasoning models
```

##### Location 2: prompt() method fallback (Line 125)
```rust
// CURRENT (WRONG):
self.info().default_temperature.unwrap_or(0.7)

// CHANGE TO:
self.info().default_temperature.unwrap_or(0.0)
```

##### Location 3: LoadedModel prompt() fallback (Line 462)
```rust
// CURRENT (WRONG):
PHI4_REASONING_MODEL_INFO.default_temperature.unwrap_or(0.7)

// CHANGE TO:
PHI4_REASONING_MODEL_INFO.default_temperature.unwrap_or(0.0)
```

**CRITICAL**: PHI-4 reasoning model documentation **explicitly requires temperature 0.0** for proper chain-of-thought reasoning. Using 0.7 breaks the reasoning capability.

---

#### **4. Global Engine Default**

**File**: [`./packages/candle/src/core/engine.rs`](../packages/candle/src/core/engine.rs)

**Current State**: Line 103 uses 0.7 as fallback

```rust
// CURRENT (WRONG):
temperature: Some(0.7),

// CHANGE TO:
temperature: Some(0.0),  // Global default: greedy sampling
```

---

#### **5. CLI Config Default**

**File**: [`./packages/candle/src/cli/config.rs`](../packages/candle/src/cli/config.rs)

**Current State**: Line 36 uses 0.7

```rust
// CURRENT (WRONG):
default_temperature: 0.7,

// CHANGE TO:
default_temperature: 0.0,  // Greedy sampling for CLI
```

---

#### **6. Domain Completion Default**

**File**: [`./packages/candle/src/domain/completion/candle.rs`](../packages/candle/src/domain/completion/candle.rs)

**Current State**: Line 151 uses 0.7

```rust
// CURRENT (WRONG):
temperature: 0.7,

// CHANGE TO:
temperature: 0.0,  // Greedy sampling for completions
```

---

#### **7. Agent Types Default**

**File**: [`./packages/candle/src/domain/agent/types.rs`](../packages/candle/src/domain/agent/types.rs)

**Current State**: Line 98 uses 0.7

```rust
// CURRENT (WRONG):
temperature: 0.7,

// CHANGE TO:
temperature: 0.0,  // Greedy sampling for agents
```

---

#### **8. Chat Config Default**

**File**: [`./packages/candle/src/domain/chat/config.rs`](../packages/candle/src/domain/chat/config.rs)

**Current State**: Lines 130 and 1387 use 0.7

```rust
// CURRENT (WRONG):
temperature: 0.7,

// CHANGE TO:
temperature: 0.0,  // Greedy sampling for chat
```

---

#### **9. Cognitive Memory Models**

**File**: [`./packages/candle/src/memory/cognitive/common/models.rs`](../packages/candle/src/memory/cognitive/common/models.rs)

**Current State**: Line 53 uses 0.7

```rust
// CURRENT (WRONG):
temperature: Some(0.7),

// CHANGE TO:
temperature: Some(0.0),  // Greedy sampling for cognitive operations
```

---

#### **10. Builder Example**

**File**: [`./packages/candle/src/lib.rs`](../packages/candle/src/lib.rs)

**Current State**: Line 138 uses 0.7

```rust
// CURRENT (WRONG):
.temperature(0.7)

// CHANGE TO:
.temperature(0.0)  // Greedy sampling example
```

---

## Files That Are Already Correct

The following files already use 0.0 and should remain unchanged:

1. **CLI Args** - `./packages/candle/src/cli/args.rs:58` ✅ (already 0.0)
2. **Agent Role Builder** - `./packages/candle/src/builders/agent_role.rs:631` ✅ (already 0.0)

---

## Implementation Strategy

### Step-by-Step Execution

For each file listed above:

1. **Open the file** in your editor
2. **Locate the line number** specified (use Ctrl+G or `:linenum` in vim)
3. **Find the exact code pattern** shown in the "CURRENT (WRONG)" section
4. **Replace with the exact code** shown in the "CHANGE TO" section
5. **Preserve all formatting, indentation, and surrounding code**
6. **Add the inline comment** for clarity (e.g., `// Greedy sampling for deterministic output`)

### Verification Method

After making changes, verify correctness by:

```bash
# Build the project to ensure no syntax errors
cargo build -p paraphym_candle

# Search for remaining non-zero temperature defaults
rg "default_temperature.*0\.[^0]" packages/candle/src/
rg "temperature:.*0\.[^0]" packages/candle/src/
rg "\.with_temperature\(0\.[^0]" packages/candle/src/

# Expected: No matches (all should be 0.0)
```

---

## Code Pattern Reference

### Temperature Parameter Extraction Pattern

This is the **standard pattern** used throughout the codebase to extract temperature from params:

```rust
// Standard pattern (used in all prompt() methods)
let temperature = if params.temperature != 1.0 {
    params.temperature
} else {
    // MODEL_INFO.default_temperature OR hardcoded fallback
    self.info().default_temperature.unwrap_or(0.0)  // ← This fallback must be 0.0
};
```

**Key Insight**: When `params.temperature == 1.0`, the code treats this as "unset" and falls back to the model's default. This is why **both** the MODEL_INFO constant AND the fallback value must be 0.0.

### Sampling Logic Pattern

```rust
// Candle's sampling logic (from candle-transformers)
let sampling = if temperature <= 0.0 {
    Sampling::ArgMax  // ← Greedy: always pick highest probability token
} else {
    // Probabilistic sampling with temperature scaling
    match (top_k, top_p) {
        (None, None) => Sampling::All { temperature },
        (Some(k), None) => Sampling::TopK { k, temperature },
        (None, Some(p)) => Sampling::TopP { p, temperature },
        (Some(k), Some(p)) => Sampling::TopKThenTopP { k, p, temperature },
    }
};
```

**When temperature = 0.0**, Candle automatically switches to `Sampling::ArgMax` (greedy sampling).

---

## Definition of Done

✅ **All temperature defaults are 0.0** across the codebase
✅ **All MODEL_INFO structs** use `default_temperature: Some(0.0)`
✅ **All fallback values** in unwrap_or() are `0.0`
✅ **All engine configs** use `.with_temperature(0.0)`
✅ **Project builds successfully** with `cargo build -p paraphym_candle`
✅ **Grep searches return no matches** for non-zero temperature defaults
✅ **Inline comments added** explaining "Greedy sampling for deterministic output"

---

## Related Tasks

- **Task 007**: Wrong model metadata (may need similar MODEL_INFO fixes)
- **Task PHI4TXTGEN_1**: PHI-4 integration (requires 0.0 temperature)

---

## Technical Context

### Temperature Scaling in Inference

Temperature affects logit scaling before softmax:

```rust
// Temperature scaling (from qwen3_quantized.rs:331-344)
let logits = if temperature != 1.0 {
    logits / temperature as f64  // Scale logits by 1/temperature
} else {
    logits  // No scaling
};
```

- **temperature = 0.0**: Special case → skip softmax, use argmax directly
- **temperature < 1.0**: Sharper distribution (more confident/deterministic)
- **temperature = 1.0**: Raw model probabilities (no scaling)
- **temperature > 1.0**: Flatter distribution (more random)

### Impact on Model Behavior

| Model | Current Temp | New Temp | Behavior Change |
|-------|-------------|----------|-----------------|
| Qwen3 | Mixed (0.7/0.8) | 0.0 | More deterministic, faster, consistent |
| KimiK2 | 0.7 | 0.0 | More deterministic, higher quality |
| PHI-4 | 0.7 (BROKEN!) | 0.0 (FIXED) | **Reasoning capability restored** |
| Llava | 0.2 (vision) | 0.2 (keep as-is) | Vision models may need lower temp |

**Note**: Vision model (Llava) already uses 0.2, which is appropriate for visual tasks. Only text-to-text models need 0.0.

---

## Architecture Alignment

This change aligns with the fluent API pattern documented in `ARCHITECTURE.md`:

```rust
// Users can still override temperature if needed
let agent = CandleFluentAi::agent_role("assistant")
    .with_completion_provider(qwen3_model)
    .with_temperature(0.8)  // ← User override still works
    .build()?;
```

**Key**: Defaults are 0.0 (deterministic), but users can override for creative tasks.

---

## Search Patterns Used

```bash
# Find all temperature defaults
rg "default_temperature" packages/candle/src/ -n

# Find temperature initializations
rg "temperature.*0\.[78]" packages/candle/src/ -n

# Find with_temperature calls
rg "\.with_temperature\(" packages/candle/src/ -n

# Find temperature parameter in structs
rg "temperature: 0\." packages/candle/src/ -n
```

---

## Additional Notes

### Why Not 0.7 or 0.8?

The original task file suggested 0.8 based on the reference implementation's CLI default. However:

1. **CLI defaults don't dictate library defaults** - they're for interactive use
2. **Production systems need determinism** - 0.0 is the industry standard
3. **PHI-4 reasoning requires 0.0** - validates our approach
4. **Existing CLI already uses 0.0** - shows intent toward determinism

### User Override Pattern

Users who want creative output can still override:

```rust
// Example: Creative writing with higher temperature
let params = CandleCompletionParams {
    temperature: 1.2,  // ← User override
    max_tokens: Some(NonZeroU32::new(500).unwrap()),
    ..Default::default()
};
```

The default being 0.0 doesn't prevent users from choosing other values.

---

## File Summary Table

| File | Lines to Change | Current Values | New Value |
|------|----------------|----------------|-----------|
| qwen3_quantized.rs | 60, 96, 128 | 0.8, 0.8, 0.7 | 0.0, 0.0, 0.0 |
| kimi_k2.rs | 47, 342, 550 | 0.7, 0.7, 0.7 | 0.0, 0.0, 0.0 |
| phi4_reasoning.rs | 125, 322, 462 | 0.7, 0.7, 0.7 | 0.0, 0.0, 0.0 |
| engine.rs | 103 | 0.7 | 0.0 |
| cli/config.rs | 36 | 0.7 | 0.0 |
| completion/candle.rs | 151 | 0.7 | 0.0 |
| agent/types.rs | 98 | 0.7 | 0.0 |
| chat/config.rs | 130, 1387 | 0.7, 0.7 | 0.0, 0.0 |
| memory/.../models.rs | 53 | 0.7 | 0.0 |
| lib.rs | 138 | 0.7 | 0.0 |

**Total Lines to Modify**: 16 lines across 10 files

---

## End of Task Specification

**Next Step**: Execute changes systematically, file by file, verifying with `cargo build` after each file.