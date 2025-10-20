//! GTE-Qwen2 model configuration

use crate::domain::model::CandleModelInfo;
use std::num::NonZeroU32;

// Static model info for GTE-Qwen2
pub(crate) static GTE_QWEN_MODEL_INFO: CandleModelInfo = CandleModelInfo {
    provider: crate::domain::model::CandleProvider::AlibabaNLP,
    name: "gte-Qwen2-1.5B-instruct",
    registry_key: "Alibaba-NLP/gte-Qwen2-1.5B-instruct",
    quantization_url: None,
    max_input_tokens: NonZeroU32::new(32768),
    max_output_tokens: None,
    input_price: None,
    output_price: None,
    supports_vision: false,
    supports_function_calling: false,
    supports_streaming: false,
    supports_embeddings: true,
    requires_max_tokens: false,
    supports_thinking: false,
    optimal_thinking_budget: None,
    system_prompt_prefix: None,
    real_name: None,
    model_type: None,
    model_id: "gte-qwen2-1.5b",
    quantization: "none",
    patch: None,
    embedding_dimension: Some(1536),
    vocab_size: None,
    image_size: None,
    image_mean: None,
    image_std: None,
    default_temperature: None,
    default_top_k: None,
    default_top_p: None,
    supports_kv_cache: false,
    supports_flash_attention: false,
    use_bf16: false,
    default_steps: None,
    default_guidance_scale: None,
    time_shift: None,
    est_memory_allocation_mb: 0,
};
