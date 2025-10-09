# Phi-4-reasoning-GGUF - Candle Implementation Reference

> **Integration Task:** See `PHI4TXTGEN_1.md` for full implementation plan

## Model Architecture

| Parameter | Value |
|-----------|-------|
| **Parameters** | 14B dense decoder-only Transformer |
| **Context Length** | 32k tokens |
| **Vocabulary Size** | 100,352 tokens |
| **Hidden Size** | 5,120 |
| **Layers** | 40 blocks |
| **License** | MIT |

## Critical Implementation Notes

**IMPORTANT**: You must use jinja template processing to enable reasoning functionality.

The model outputs responses in two sections:
1. A reasoning chain-of-thought block (within `<think>` tags)
2. A summarization/solution block

## Quantization Details (Q4_K_M)

- **Quantization Method**: Unsloth Dynamic 2.0 with importance matrix
- **imatrix Dataset**: unsloth_calibration_phi-4-reasoning.txt
- **imatrix Entries**: 160
- **imatrix Chunks**: 81

### Tensor Precision Breakdown

| Tensor Type | Precision |
|-------------|-----------|
| `token_embd.weight` | Q4_K |
| `blk.*.attn_norm.weight` | F32 |
| `blk.*.attn_qkv.weight` | Q5_K |
| `blk.*.ffn_down.weight` | Q6_K |
| `blk.*.ffn_norm.weight` | F32 |
| `blk.*.ffn_up.weight` | Q4_K |
| `blk.*.attn_output.weight` | Q4_K |
| `output.weight` | Q6_K |
| `output_norm.weight` | F32 |

### Per-Layer Tensor Shapes

```
token_embd.weight: [5120, 100352]

For each block (40 total):
  attn_norm.weight: [5120]
  attn_qkv.weight: [5120, 7680]      # GQA: Q + K + V combined
  attn_output.weight: [5120, 5120]
  ffn_norm.weight: [5120]
  ffn_up.weight: [5120, 35840]        # 7x expansion
  ffn_down.weight: [17920, 5120]

output_norm.weight: [5120]
output.weight: [5120, 100352]
```

## Chat Template (Jinja2)

```jinja
{% if messages|length == 0 or messages[0]['role'] != 'system' %}
{{'<|im_start|>system<|im_sep|>You are Phi, a language model trained by Microsoft to help users. Your role as an assistant involves thoroughly exploring questions through a systematic thinking process before providing the final precise and accurate solutions. This requires engaging in a comprehensive cycle of analysis, summarizing, exploration, reassessment, reflection, backtracing, and iteration to develop well-considered thinking process. Please structure your response into two main sections: Thought and Solution using the specified format: <think> {Thought section} </think> {Solution section}. In the Thought section, detail your reasoning process in steps. Each step should include detailed considerations such as analysing questions, summarizing relevant findings, brainstorming new ideas, verifying the accuracy of the current steps, refining any errors, and revisiting previous steps. In the Solution section, based on various attempts, explorations, and reflections from the Thought section, systematically present the final solution that you deem correct. The Solution section should be logical, accurate, and concise and detail necessary steps needed to reach the conclusion. Now, try to solve the following question through the above guidelines:<|im_end|>'}}
{% endif %}
{% for message in messages %}
{% if messages[0]['role'] == 'system' %}
{{'<|im_start|>system<|im_sep|>You are Phi, a language model trained by Microsoft to help users. Your role as an assistant involves thoroughly exploring questions through a systematic thinking process before providing the final precise and accurate solutions. This requires engaging in a comprehensive cycle of analysis, summarizing, exploration, reassessment, reflection, backtracing, and iteration to develop well-considered thinking process. Please structure your response into two main sections: Thought and Solution using the specified format: <think> {Thought section} </think> {Solution section}. In the Thought section, detail your reasoning process in steps. Each step should include detailed considerations such as analysing questions, summarizing relevant findings, brainstorming new ideas, verifying the accuracy of the current steps, refining any errors, and revisiting previous steps. In the Solution section, based on various attempts, explorations, and reflections from the Thought section, systematically present the final solution that you deem correct. The Solution section should be logical, accurate, and concise and detail necessary steps needed to reach the conclusion. Now, try to solve the following question through the above guidelines:<|im_end|>'}}
{% elif message['role'] == 'user' %}
{{'<|im_start|>user<|im_sep|>' + message['content'] + '<|im_end|>'}}
{% elif message['role'] == 'assistant' %}
{{'<|im_start|>assistant<|im_sep|>' + message['content'] + '<|im_end|>'}}
{% endif %}
{% endfor %}
{% if add_generation_prompt %}
{{ '<|im_start|>assistant<|im_sep|>' }}
{% endif %}
```

## Special Tokens

| Token | Value |
|-------|-------|
| BOS | `<\|endoftext\|>` |
| EOS | `<\|im_end\|>` |
| Chat Template Tokens | `<\|im_start\|>`, `<\|im_sep\|>`, `<\|im_end\|>` |

## Inference Parameters

```
temperature: 0.0
top_p: 1.0
repeat_penalty: 1.0
```

## Model Capabilities

**Primary Focus**: Math reasoning with chain-of-thought
**Input**: Text prompts in chat format
**Output**: Structured response with `<think>` reasoning section followed by solution
**Training Data Cutoff**: March 2025
**Languages**: Primarily English

## Architecture Notes for Candle

- Base model same as Phi-4
- Dense decoder-only Transformer
- Uses grouped query attention (QKV combined)
- FFN uses up/down projection pattern (up: 5120→35840, down: 17920→5120)
- Normalization layers use F32 precision
- Attention and FFN output use Q4_K quantization
- QKV projection uses Q5_K (higher precision)
- FFN down projection uses Q6_K (higher precision)

## Model Download

```bash
# HuggingFace model repository
https://huggingface.co/unsloth/Phi-4-reasoning-GGUF

# Specific file
phi-4-reasoning-Q4_K_M.gguf (~8.5GB)
```

## Implementation Reference

**Candle Phi Example**: `tmp/candle/candle-examples/examples/phi/main.rs`
- TextGeneration pattern (lines 29-136)
- GGUF loading (lines 356-366)
- Quantized model handling (lines 362-366)

**Integration Task**: `task/PHI4TXTGEN_1.md`
