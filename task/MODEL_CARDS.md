# Model Card Documentation Task

## Core Objective

Download HuggingFace model cards for all 12 models in the registry, convert them to markdown format, and extract **memory allocation specifications** to populate the `est_memory_allocation_mb` field in `CandleModelInfo`. This is a **CRITICAL BLOCKER** for implementing memory-aware load balancing in the model pool.

## Why This Matters

The pool implementation needs accurate memory estimates to:
- Calculate dynamic worker limits based on available VRAM/RAM
- Prevent OOM crashes by not loading too many models simultaneously
- Make intelligent scheduling decisions about which models to keep loaded
- Provide accurate resource usage reporting

**Without these memory specs, the pool cannot function safely.**

## Codebase Structure

### Where Models Are Defined

All 12 models are registered as static `CandleModelInfo` structs in capability-specific files:

```
packages/candle/src/capability/
├── text_embedding/
│   ├── bert.rs              → BERT MiniLM L6 v2
│   ├── stella.rs            → Stella EN 1.5B v5
│   ├── gte_qwen.rs          → GTE-Qwen2 1.5B Instruct
│   ├── jina_bert.rs         → Jina Embeddings v2 Base EN
│   └── nvembed.rs           → NV-Embed v2
├── text_to_text/
│   ├── phi4_reasoning.rs    → Phi-4 Reasoning Q4_K_M (✅ already documented)
│   ├── kimi_k2.rs           → Kimi K2 Instruct Q4_K_M
│   └── qwen3_coder.rs       → Qwen3-Coder 30B Q4_K_M
├── image_embedding/
│   └── clip_vision.rs       → CLIP ViT Base Patch32
├── vision/
│   └── llava.rs             → LLaVA 1.5 7B
└── text_to_image/
    ├── flux_schnell.rs      → FLUX.1 Schnell
    └── stable_diffusion_35_turbo/ → SD 3.5 Large Turbo
```

### CandleModelInfo Struct

**Location:** [`packages/candle/src/domain/model/info.rs:103`](../packages/candle/src/domain/model/info.rs)

Current struct (BEFORE adding memory field):

```rust
pub struct CandleModelInfo {
    pub provider: CandleProvider,
    pub name: &'static str,
    pub registry_key: &'static str,
    pub max_input_tokens: Option<NonZeroU32>,
    pub max_output_tokens: Option<NonZeroU32>,
    // ... many other fields ...
    pub time_shift: Option<f64>,
}
```

**AFTER adding memory field** (THIS BREAKS COMPILATION):

```rust
pub struct CandleModelInfo {
    pub provider: CandleProvider,
    pub name: &'static str,
    pub registry_key: &'static str,
    
    /// Estimated memory allocation in MB for model inference
    /// 
    /// This is the total memory (VRAM + RAM) required to load and run
    /// the model for inference. Includes:
    /// - Model weights
    /// - KV cache allocation
    /// - Activation memory
    /// - Safety margin (10-20%)
    /// 
    /// For quantized models (Q4_K_M, Q5_K, etc.), use the quantized size.
    /// For safetensor models, use FP16 or FP32 size based on actual usage.
    /// 
    /// **CRITICAL**: Round UP to nearest 100 MB for safety.
    pub est_memory_allocation_mb: usize,
    
    pub max_input_tokens: Option<NonZeroU32>,
    // ... rest of fields ...
}
```

### Static Model Info Pattern

Each model file contains a static declaration like this:

**Example from [`bert.rs:145-181`](../packages/candle/src/capability/text_embedding/bert.rs):**

```rust
static BERT_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::SentenceTransformers,
    name: "all-MiniLM-L6-v2",
    registry_key: "sentence-transformers/all-MiniLM-L6-v2",
    max_input_tokens: NonZeroU32::new(512),
    max_output_tokens: None,
    // ... all other fields ...
    time_shift: None,
};
```

**AFTER adding est_memory_allocation_mb**, each static will need:

```rust
static BERT_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::SentenceTransformers,
    name: "all-MiniLM-L6-v2",
    registry_key: "sentence-transformers/all-MiniLM-L6-v2",
    est_memory_allocation_mb: 100,  // ← ADD THIS FIELD (example value)
    max_input_tokens: NonZeroU32::new(512),
    // ... rest remains the same ...
};
```

## Models to Document

### Text Embedding (5 models)

**Location:** `docs/models/text_embedding/`

1. **Stella EN 1.5B v5**
   - HuggingFace: `dunzhang/stella_en_1.5B_v5`
   - File: `STELLA-EN-1.5B-V5.md`
   - Registry key: `dunzhang/stella_en_1.5B_v5`
   - Source: [`stella.rs`](../packages/candle/src/capability/text_embedding/stella.rs)

2. **BERT MiniLM L6 v2**
   - HuggingFace: `sentence-transformers/all-MiniLM-L6-v2`
   - File: `BERT-MINILM-L6-V2.md`
   - Registry key: `sentence-transformers/all-MiniLM-L6-v2`
   - Source: [`bert.rs`](../packages/candle/src/capability/text_embedding/bert.rs)

3. **GTE-Qwen2 1.5B Instruct**
   - HuggingFace: `Alibaba-NLP/gte-Qwen2-1.5B-instruct`
   - File: `GTE-QWEN2-1.5B-INSTRUCT.md`
   - Registry key: `Alibaba-NLP/gte-Qwen2-1.5B-instruct`
   - Source: [`gte_qwen.rs`](../packages/candle/src/capability/text_embedding/gte_qwen.rs)

4. **Jina Embeddings v2 Base EN**
   - HuggingFace: `jinaai/jina-embeddings-v2-base-en`
   - File: `JINA-EMBEDDINGS-V2-BASE-EN.md`
   - Registry key: `jinaai/jina-embeddings-v2-base-en`
   - Source: [`jina_bert.rs`](../packages/candle/src/capability/text_embedding/jina_bert.rs)

5. **NV-Embed v2**
   - HuggingFace: `nvidia/NV-Embed-v2`
   - File: `NV-EMBED-V2.md`
   - Registry key: `nvidia/NV-Embed-v2`
   - Source: [`nvembed.rs`](../packages/candle/src/capability/text_embedding/nvembed.rs)

### Text to Text (3 models)

**Location:** `docs/models/text_to_text/`

1. **Phi-4 Reasoning Q4_K_M** ✅ DONE
   - HuggingFace: `microsoft/phi-4`
   - File: `PHI-4-REASONING-Q4_K_M.md` (already exists)
   - Registry key: `microsoft/phi-4`
   - Source: [`phi4_reasoning.rs`](../packages/candle/src/capability/text_to_text/phi4_reasoning.rs)
   - Template: [`docs/models/text_to_text/PHI-4-REASONING-Q4_K_M.md`](../docs/models/text_to_text/PHI-4-REASONING-Q4_K_M.md)

2. **Kimi K2 Instruct Q4_K_M**
   - HuggingFace: `unsloth/Kimi-K2-Instruct-GGUF`
   - File: `KIMI-K2-INSTRUCT-Q4_K_M.md`
   - Registry key: `unsloth/Kimi-K2-Instruct-GGUF`
   - Note: GGUF format, check quantization specs
   - Source: [`kimi_k2.rs`](../packages/candle/src/capability/text_to_text/kimi_k2.rs)

3. **Qwen3-Coder 30B Q4_K_M**
   - HuggingFace: `unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF`
   - File: `QWEN3-CODER-30B-Q4_K_M.md`
   - Registry key: `unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF`
   - Note: GGUF format, check quantization specs
   - Source: [`qwen3_coder.rs`](../packages/candle/src/capability/text_to_text/qwen3_coder.rs)

### Image Embedding (1 model)

**Location:** `docs/models/image_embedding/`

1. **CLIP ViT Base Patch32**
   - HuggingFace: `openai/clip-vit-base-patch32`
   - File: `CLIP-VIT-BASE-PATCH32.md`
   - Registry key: `openai/clip-vit-base-patch32`
   - Source: [`clip_vision.rs`](../packages/candle/src/capability/image_embedding/clip_vision.rs)

### Vision (1 model)

**Location:** `docs/models/vision/`

1. **LLaVA 1.5 7B**
   - HuggingFace: `llava-hf/llava-1.5-7b-hf`
   - File: `LLAVA-1.5-7B.md`
   - Registry key: `llava-hf/llava-1.5-7b-hf`
   - Source: [`llava.rs`](../packages/candle/src/capability/vision/llava.rs)

### Text to Image (2 models)

**Location:** `docs/models/text_to_image/`

1. **FLUX.1 Schnell**
   - HuggingFace: `black-forest-labs/FLUX.1-schnell`
   - File: `FLUX-SCHNELL.md`
   - Registry key: `black-forest-labs/FLUX.1-schnell`
   - Source: [`flux_schnell.rs`](../packages/candle/src/capability/text_to_image/flux_schnell.rs)

2. **Stable Diffusion 3.5 Large Turbo**
   - HuggingFace: `stabilityai/stable-diffusion-3.5-large-turbo`
   - File: `SD-3.5-LARGE-TURBO.md`
   - Registry key: `stabilityai/stable-diffusion-3.5-large-turbo`
   - Source: [`stable_diffusion_35_turbo/`](../packages/candle/src/capability/text_to_image/stable_diffusion_35_turbo/)

## Step-by-Step Instructions

### Step 1: Download Model Cards and Extract Memory Requirements

For each of the 11 models (Phi-4 already done), visit the HuggingFace URL and extract:

1. **Model description** - Brief overview
2. **Architecture details** - Type, parameters, layers
3. **Memory requirements** - **THIS IS THE CRITICAL PIECE**
   - Look for "VRAM requirements"
   - Look for "RAM requirements"
   - Look for "Memory footprint"
   - Look for "System requirements"
   - For GGUF models, check the quantization row (Q4_K_M)
4. **Quantization information** - Q4_K_M, Q5_K, FP16, etc.
5. **Parameter count** - e.g., "1.5B", "7B", "30B"
6. **Input/output specifications** - Token limits, dimensions

### Step 2: Convert to Markdown

Follow the format from [`PHI-4-REASONING-Q4_K_M.md`](../docs/models/text_to_text/PHI-4-REASONING-Q4_K_M.md):

```markdown
# {MODEL_NAME}

**Registry Key**: `{registry_key}`
**HuggingFace**: https://huggingface.co/{org}/{model}

## Overview

{Brief description from model card}

## Architecture

- **Type**: {embedding/decoder-only/encoder-decoder/vision-language/diffusion}
- **Parameters**: {parameter count}
- **Hidden Size**: {hidden_size if available}
- **Layers**: {num_layers if available}
- **Quantization**: {quantization type}

## Memory Requirements

**CRITICAL FOR POOL IMPLEMENTATION**

- **Inference (FP32)**: {X} GB (if applicable)
- **Inference (FP16)**: {Y} GB (if applicable)
- **Inference (Quantized Q4_K_M)**: {Z} GB (for GGUF models)
- **Recommended VRAM**: {N} GB
- **Estimated Total Memory**: {M} GB

## Performance

{Any relevant performance metrics from model card}

## Usage Notes

{Any special considerations for this model}
```

### Step 3: Extract Memory Specification

From the markdown file, identify the memory requirement for the **actual configuration we use**:

#### For GGUF Models (Q4_K_M quantized):
- Kimi K2: Use Q4_K_M quantized memory
- Qwen3-Coder 30B: Use Q4_K_M quantized memory
- Phi-4 Reasoning: Use Q4_K_M quantized memory

#### For Safetensor Models (FP16/FP32):
- All embedding models: Use FP16 or FP32 based on what we load
- CLIP: Check actual precision used
- LLaVA: Typically FP16 for inference
- FLUX/SD: Check actual precision used

#### Estimation Rules:
1. **If model card says "3.2 GB VRAM required"** → record as `3200 MB`
2. **If model card says "~4 GB"** → record as `4000 MB`
3. **If range given "2-4 GB"** → use **upper bound**: `4000 MB`
4. **Always round UP to nearest 100 MB** for safety margin
5. **Add 10-20% buffer** for KV cache and activations

#### Memory Estimation Formulas:

**For quantized models (Q4_K_M):**
```
Memory (MB) = (Parameters × Bits per param) / 8 × 1.15
```
Example: 14B params × 4.5 bits/param / 8 × 1.15 ≈ 9000 MB

**For FP16 models:**
```
Memory (MB) = Parameters × 2 bytes × 1.20
```
Example: 1.5B params × 2 × 1.20 ≈ 3600 MB

**For embedding models:**
```
Memory (MB) = Model size on disk × 1.10
```
(Usually smaller, ~100-500 MB for BERT-based models)

### Step 4: Create Memory Specifications Summary

Create `docs/models/MEMORY_SPECS.md` with a table of all memory values:

```markdown
# Model Memory Specifications

## Summary Table

| Model | Registry Key | Memory (MB) | Quantization | Source |
|-------|--------------|-------------|--------------|--------|
| Stella EN 1.5B v5 | dunzhang/stella_en_1.5B_v5 | XXXX | none | [stella.rs](../../packages/candle/src/capability/text_embedding/stella.rs) |
| BERT MiniLM L6 v2 | sentence-transformers/all-MiniLM-L6-v2 | XXXX | none | [bert.rs](../../packages/candle/src/capability/text_embedding/bert.rs) |
| GTE-Qwen2 1.5B | Alibaba-NLP/gte-Qwen2-1.5B-instruct | XXXX | none | [gte_qwen.rs](../../packages/candle/src/capability/text_embedding/gte_qwen.rs) |
| Jina Embeddings v2 | jinaai/jina-embeddings-v2-base-en | XXXX | none | [jina_bert.rs](../../packages/candle/src/capability/text_embedding/jina_bert.rs) |
| NV-Embed v2 | nvidia/NV-Embed-v2 | XXXX | none | [nvembed.rs](../../packages/candle/src/capability/text_embedding/nvembed.rs) |
| Phi-4 Reasoning | microsoft/phi-4 | XXXX | Q4_K_M | [phi4_reasoning.rs](../../packages/candle/src/capability/text_to_text/phi4_reasoning.rs) |
| Kimi K2 Instruct | unsloth/Kimi-K2-Instruct-GGUF | XXXX | Q4_K_M | [kimi_k2.rs](../../packages/candle/src/capability/text_to_text/kimi_k2.rs) |
| Qwen3-Coder 30B | unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF | XXXX | Q4_K_M | [qwen3_coder.rs](../../packages/candle/src/capability/text_to_text/qwen3_coder.rs) |
| CLIP ViT Base | openai/clip-vit-base-patch32 | XXXX | none | [clip_vision.rs](../../packages/candle/src/capability/image_embedding/clip_vision.rs) |
| LLaVA 1.5 7B | llava-hf/llava-1.5-7b-hf | XXXX | FP16 | [llava.rs](../../packages/candle/src/capability/vision/llava.rs) |
| FLUX Schnell | black-forest-labs/FLUX.1-schnell | XXXX | none | [flux_schnell.rs](../../packages/candle/src/capability/text_to_image/flux_schnell.rs) |
| SD 3.5 Large Turbo | stabilityai/stable-diffusion-3.5-large-turbo | XXXX | none | [stable_diffusion_35_turbo/](../../packages/candle/src/capability/text_to_image/stable_diffusion_35_turbo/) |

## Implementation Notes

After gathering all memory specifications:

1. Add `est_memory_allocation_mb: usize` to `CandleModelInfo` in [`info.rs:103`](../../packages/candle/src/domain/model/info.rs)
2. Update each static MODEL_INFO declaration in the source files listed above
3. Compilation will succeed once all 12 models have the new field

## Total Memory Budget

Total estimated memory for all 12 models: TBD MB

This represents the maximum memory required if all models were loaded simultaneously (which the pool will prevent).
```

## Output Deliverables

1. **11 new markdown files** in `docs/models/{capability}/` directories
2. **Memory specifications summary** in `docs/models/MEMORY_SPECS.md`
3. **Clear action plan** for adding `est_memory_allocation_mb` field

## Definition of Done

This task is complete when:

- [ ] All 11 model markdown files are created with complete memory specifications
- [ ] `MEMORY_SPECS.md` summary table is complete with all 12 models
- [ ] Each memory spec is rounded UP to nearest 100 MB
- [ ] Memory specs include safety margin (10-20%)
- [ ] Files follow naming convention (UPPERCASE-WITH-HYPHENS.md)
- [ ] All memory values are extracted from official HuggingFace model cards
- [ ] Ready to add `est_memory_allocation_mb` field to CandleModelInfo struct

## Critical Requirements

**THIS IS A REQUIRED FIELD - NOT OPTIONAL**

When `est_memory_allocation_mb: usize` is added to CandleModelInfo:
- ❌ **COMPILATION WILL BREAK** until ALL 12 models are updated
- ❌ **NO DEFAULT VALUES** - must be explicitly specified
- ❌ **NO Option<usize>** - required field, period
- ❌ **POOL CANNOT WORK** without this data

This is intentional. We **CANNOT** do memory-aware load balancing without accurate memory specs for every model.

## Implementation Workflow

### Phase 1: Documentation (THIS TASK)

1. Download and document all 11 model cards
2. Extract memory requirements into markdown files
3. Create `MEMORY_SPECS.md` summary table
4. Verify all memory values are present and accurate

### Phase 2: Struct Modification (NEXT TASK)

1. Add `est_memory_allocation_mb: usize` to CandleModelInfo in `info.rs:103`
2. Compilation breaks with 12 errors (one per MODEL_INFO)
3. Example error:
   ```
   error: missing field `est_memory_allocation_mb` in struct `CandleModelInfo`
     --> packages/candle/src/capability/text_embedding/bert.rs:145:48
   ```

### Phase 3: Update All Static Registrations (NEXT TASK)

Update each of the 12 static MODEL_INFO structs:

```rust
// BEFORE
static BERT_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::SentenceTransformers,
    name: "all-MiniLM-L6-v2",
    registry_key: "sentence-transformers/all-MiniLM-L6-v2",
    max_input_tokens: NonZeroU32::new(512),
    // ...
};

// AFTER
static BERT_EMBEDDING_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::SentenceTransformers,
    name: "all-MiniLM-L6-v2",
    registry_key: "sentence-transformers/all-MiniLM-L6-v2",
    est_memory_allocation_mb: 100,  // ← Value from MEMORY_SPECS.md
    max_input_tokens: NonZeroU32::new(512),
    // ...
};
```

### Phase 4: Verification (FINAL STEP)

1. Run `cargo check` → should compile without errors
2. Pool implementation can proceed with memory-aware scheduling

## Estimated Time

- 20-30 minutes per model (research + documentation)
- Total: ~6-8 hours for all 11 models
- **MUST BE COMPLETED** before pool work continues

## Research Resources

### HuggingFace Model Card Locations

Direct links to model cards:
- https://huggingface.co/dunzhang/stella_en_1.5B_v5
- https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2
- https://huggingface.co/Alibaba-NLP/gte-Qwen2-1.5B-instruct
- https://huggingface.co/jinaai/jina-embeddings-v2-base-en
- https://huggingface.co/nvidia/NV-Embed-v2
- https://huggingface.co/unsloth/Kimi-K2-Instruct-GGUF
- https://huggingface.co/unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF
- https://huggingface.co/openai/clip-vit-base-patch32
- https://huggingface.co/llava-hf/llava-1.5-7b-hf
- https://huggingface.co/black-forest-labs/FLUX.1-schnell
- https://huggingface.co/stabilityai/stable-diffusion-3.5-large-turbo

### Memory Specification Search Terms

When browsing model cards, search for:
- "memory requirements"
- "VRAM"
- "RAM"
- "GPU memory"
- "system requirements"
- "inference requirements"
- "quantization" (for GGUF models)
- "model size"

### Example Memory Specs (Reference)

From Phi-4 documentation:
- Phi-4 14B Q4_K_M: ~8.5 GB → record as `8500 MB` (rounded up)

Typical ranges:
- Small embedding models (BERT-based): 100-500 MB
- Medium embedding models (1.5B): 3000-4000 MB
- Large language models (7B Q4_K_M): 4000-6000 MB
- Very large models (30B Q4_K_M): 18000-22000 MB
- Diffusion models (FLUX, SD): 15000-25000 MB

## Questions for Clarification

If memory specifications are unclear on the model card:
1. Check the "Files and versions" tab for actual .gguf or .safetensors file sizes
2. Look for community discussions in the model card comments
3. Search for benchmark reports or inference examples
4. For GGUF models, calculate from parameter count using formulas above
5. When in doubt, round UP generously (better to overestimate than OOM)

## Success Criteria

- All 11 models documented
- Memory requirements clearly specified for each model
- Files formatted consistently
- `MEMORY_SPECS.md` summary table complete
- Ready to populate CandleModelInfo structs with memory values
- Clear path forward for adding `est_memory_allocation_mb` field
