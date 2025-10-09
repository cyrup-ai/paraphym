# Model Memory Specifications

## Summary Table

| Model | Registry Key | Memory (MB) | Quantization | Source |
|-------|--------------|-------------|--------------|--------|
| Stella EN 1.5B v5 | dunzhang/stella_en_1.5B_v5 | 7200 | none | [stella.rs](../../packages/candle/src/capability/text_embedding/stella.rs) |
| BERT MiniLM L6 v2 | sentence-transformers/all-MiniLM-L6-v2 | 100 | none | [bert.rs](../../packages/candle/src/capability/text_embedding/bert.rs) |
| GTE-Qwen2 1.5B | Alibaba-NLP/gte-Qwen2-1.5B-instruct | 7600 | none | [gte_qwen.rs](../../packages/candle/src/capability/text_embedding/gte_qwen.rs) |
| Jina Embeddings v2 | jinaai/jina-embeddings-v2-base-en | 400 | none | [jina_bert.rs](../../packages/candle/src/capability/text_embedding/jina_bert.rs) |
| NV-Embed v2 | nvidia/NV-Embed-v2 | 18100 | none | [nvembed.rs](../../packages/candle/src/capability/text_embedding/nvembed.rs) |
| Phi-4 Reasoning | microsoft/phi-4 | 10400 | Q4_K_M | [phi4_reasoning.rs](../../packages/candle/src/capability/text_to_text/phi4_reasoning.rs) |
| Kimi K2 Instruct | unsloth/Kimi-K2-Instruct-GGUF | 715000 | Q4_K_M | [kimi_k2.rs](../../packages/candle/src/capability/text_to_text/kimi_k2.rs) |
| Qwen3-Coder 30B | unsloth/Qwen3-Coder-30B-A3B-Instruct-GGUF | 21400 | Q4_K_M | [qwen3_coder.rs](../../packages/candle/src/capability/text_to_text/qwen3_coder.rs) |
| CLIP ViT Base | openai/clip-vit-base-patch32 | 700 | none | [clip_vision.rs](../../packages/candle/src/capability/image_embedding/clip_vision.rs) |
| LLaVA 1.5 7B | llava-hf/llava-1.5-7b-hf | 17000 | FP16 | [llava.rs](../../packages/candle/src/capability/vision/llava.rs) |
| FLUX Schnell | black-forest-labs/FLUX.1-schnell | 27700 | none | [flux_schnell.rs](../../packages/candle/src/capability/text_to_image/flux_schnell.rs) |
| SD 3.5 Large Turbo | stabilityai/stable-diffusion-3.5-large-turbo | 25600 | none | [stable_diffusion_35_turbo/](../../packages/candle/src/capability/text_to_image/stable_diffusion_35_turbo/) |

## Implementation Notes

After gathering all memory specifications:

1. Add `est_memory_allocation_mb: usize` to `CandleModelInfo` in [`info.rs:103`](../../packages/candle/src/domain/model/info.rs)
2. Update each static MODEL_INFO declaration in the source files listed above
3. Compilation will succeed once all 12 models have the new field

## Total Memory Budget

Total estimated memory for all 12 models: **851,200 MB** (≈ **831 GB**)

This represents the maximum memory required if all models were loaded simultaneously (which the pool will prevent).

## Memory Allocation Breakdown by Capability

### Text Embedding Models (5 models)
- BERT MiniLM L6 v2: 100 MB (smallest)
- Jina Embeddings v2: 400 MB
- Stella EN 1.5B v5: 7,200 MB
- GTE-Qwen2 1.5B: 7,600 MB
- NV-Embed v2: 18,100 MB (largest embedding model)
- **Subtotal**: 33,400 MB (≈ 32.6 GB)

### Text-to-Text Models (3 models)
- Phi-4 Reasoning Q4_K_M: 10,400 MB
- Qwen3-Coder 30B Q4_K_M: 21,400 MB
- Kimi K2 Instruct Q4_K_M: 715,000 MB (extremely large MoE)
- **Subtotal**: 746,800 MB (≈ 729 GB)

### Image Embedding Models (1 model)
- CLIP ViT Base: 700 MB
- **Subtotal**: 700 MB

### Vision Models (1 model)
- LLaVA 1.5 7B: 17,000 MB
- **Subtotal**: 17,000 MB (≈ 16.6 GB)

### Text-to-Image Models (2 models)
- SD 3.5 Large Turbo: 25,600 MB
- FLUX Schnell: 27,700 MB
- **Subtotal**: 53,300 MB (≈ 52 GB)

## Notes on Memory Usage

1. **Kimi K2** is exceptionally large (715 GB) due to being a 1T parameter MoE model. This model will require dedicated high-memory systems or distributed inference.

2. **Embedding models** range from 100 MB (BERT) to 18.1 GB (NV-Embed v2), making them suitable for various resource constraints.

3. **Diffusion models** (FLUX, SD 3.5) are in the 25-28 GB range and require substantial VRAM for efficient inference.

4. **All memory allocations include 10-20% overhead** for KV cache, activations, and inference overhead, rounded up to the nearest 100 MB.

5. **Quantized models** (Q4_K_M) provide significant memory savings compared to FP16/FP32 variants while maintaining good quality.

## Practical Implications for Pool Implementation

- The pool must intelligently schedule models based on available memory
- Small models (BERT, Jina, CLIP) can be kept resident with minimal impact
- Large models (Kimi K2, diffusion models) should be loaded on-demand
- Memory-aware worker limits are critical to prevent OOM crashes
- Consider model co-location strategies (e.g., multiple small embedding models vs. one large LLM)
