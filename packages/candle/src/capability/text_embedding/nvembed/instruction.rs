//! Instruction formatting and masking for NVEmbed v2
//!
//! NVEmbed v2 requires task-specific instruction prefixes for optimal performance.
//! This module handles instruction formatting, mask generation, and instruction-aware inference.

use crate::memory::utils::error::{Error as MemoryError, Result};
use candle_core::{Device, Tensor};
use candle_transformers::models::nvembed_v2::model::Model as NvEmbedModel;
use tokenizers::Tokenizer;

/// Format text with task-specific instruction prefix for NVEmbed v2
#[inline]
pub(crate) fn format_with_instruction(text: &str, task: Option<&str>) -> String {
    match task {
        Some("search_query") => format!(
            "Instruct: Given a web search query, retrieve relevant passages that answer the query.\nQuery: {}",
            text
        ),
        Some("search_document") => format!(
            "Instruct: Given a web search query, retrieve relevant passages that answer the query.\nPassage: {}",
            text
        ),
        Some("classification") => format!(
            "Instruct: Retrieve semantically similar text.\nText: {}",
            text
        ),
        Some("clustering") => {
            format!("Instruct: Identify and group similar text.\nText: {}", text)
        }
        Some("retrieval") => format!(
            "Instruct: Given a question, retrieve passages that answer the question.\nPassage: {}",
            text
        ),
        _ => text.to_string(), // No instruction for default case
    }
}

/// Create instruction mask that excludes instruction tokens from pooling
/// Returns a mask where 1.0 indicates content tokens and 0.0 indicates instruction tokens
#[inline]
pub(crate) fn create_instruction_mask(
    tokenizer: &Tokenizer,
    token_ids: &Tensor,
    formatted_texts: &[String],
    original_texts: &[&str],
    device: &Device,
) -> Result<Tensor> {
    let (batch_size, seq_len) = token_ids
        .dims2()
        .map_err(|e| MemoryError::ModelError(format!("Invalid token_ids shape: {}", e)))?;

    let mut instruction_mask_data = vec![vec![1.0f32; seq_len]; batch_size];

    for (batch_idx, (formatted_text, original_text)) in formatted_texts
        .iter()
        .zip(original_texts.iter())
        .enumerate()
    {
        // If text was formatted with instruction, find where original content starts
        if formatted_text != *original_text {
            // Find the last occurrence of original text to correctly identify instruction boundary
            if let Some(content_start_pos) = formatted_text.rfind(original_text) {
                // Tokenize both full text and content-only to find instruction token boundary
                let full_tokens =
                    tokenizer
                        .encode(formatted_text.as_str(), false)
                        .map_err(|e| {
                            MemoryError::ModelError(format!(
                                "Failed to tokenize full text: {}",
                                e
                            ))
                        })?;

                let content_only = &formatted_text[content_start_pos..];
                let content_tokens = tokenizer.encode(content_only, false).map_err(|e| {
                    MemoryError::ModelError(format!("Failed to tokenize content: {}", e))
                })?;

                let full_token_count = full_tokens.get_ids().len();
                let content_token_count = content_tokens.get_ids().len();

                // Calculate instruction token count by difference
                let instruction_token_count = if full_token_count >= content_token_count {
                    full_token_count - content_token_count
                } else {
                    // Fallback: use character-based estimation if tokenization is inconsistent
                    let instruction_char_ratio =
                        content_start_pos as f32 / formatted_text.len() as f32;
                    (instruction_char_ratio * full_token_count as f32).ceil() as usize
                };

                // Mark instruction tokens as 0.0 (exclude from pooling)
                for item in instruction_mask_data[batch_idx]
                    .iter_mut()
                    .take(instruction_token_count.min(seq_len))
                {
                    *item = 0.0;
                }
            }
        }
    }

    // Convert to tensor
    let flat_data: Vec<f32> = instruction_mask_data.into_iter().flatten().collect();
    Tensor::from_vec(flat_data, (batch_size, seq_len), device).map_err(|e| {
        MemoryError::ModelError(format!("Failed to create instruction mask tensor: {}", e))
    })
}

/// Execute forward pass with instruction-aware pooling
#[inline]
pub(crate) fn forward_pass_with_instruction(
    tokenizer: &Tokenizer,
    model: &mut NvEmbedModel,
    device: &Device,
    texts: &[&str],
    task: Option<&str>,
) -> Result<Vec<Vec<f32>>> {
    // Format texts with task-specific instructions
    let formatted_texts: Vec<String> = texts
        .iter()
        .map(|text| format_with_instruction(text, task))
        .collect();

    // Tokenize formatted texts
    let tokens = tokenizer
        .encode_batch(formatted_texts.clone(), true)
        .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))?;

    let token_ids = tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.get_ids().to_vec();
            Tensor::new(tokens.as_slice(), device)
                .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
        })
        .collect::<Result<Vec<_>>>()?;

    let attention_mask = tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.get_attention_mask().to_vec();
            Tensor::new(tokens.as_slice(), device)
                .map_err(|e| MemoryError::ModelError(format!("Tensor creation failed: {}", e)))
        })
        .collect::<Result<Vec<_>>>()?;

    let token_ids = Tensor::stack(&token_ids, 0).map_err(|e| {
        MemoryError::ModelError(format!("Token IDs tensor stack failed: {}", e))
    })?;
    let attention_mask = Tensor::stack(&attention_mask, 0).map_err(|e| {
        MemoryError::ModelError(format!("Attention mask tensor stack failed: {}", e))
    })?;

    // Create instruction-aware pool_mask that excludes instruction tokens
    let instruction_mask =
        create_instruction_mask(tokenizer, &token_ids, &formatted_texts, texts, device)?;
    let pool_mask = (&attention_mask * &instruction_mask).map_err(|e| {
        MemoryError::ModelError(format!("Failed to apply instruction mask: {}", e))
    })?;

    // Forward pass using real NVEmbed API
    let embeddings = model
        .forward(&token_ids, &attention_mask, &pool_mask)
        .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)))?;

    let embeddings_data = embeddings
        .to_vec2::<f32>()
        .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))?;

    Ok(embeddings_data)
}
