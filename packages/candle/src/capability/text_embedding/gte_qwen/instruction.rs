//! GTE-Qwen2 instruction formatting and forward pass logic
//!
//! This module handles task-specific instruction prefixes and the complete forward pass
//! including tokenization, model inference, and attention-masked last token pooling.

use crate::memory::utils::error::{Error as MemoryError, Result};
use candle_core::{Device, Tensor};
use candle_transformers::models::qwen2::Model;
use tokenizers::Tokenizer;

/// Forward pass with task-specific instruction formatting
///
/// Formats input texts with task-specific prefixes (search_query, search_document, etc.),
/// tokenizes, runs model inference, and applies attention-masked last token pooling.
///
/// Returns (Model, embeddings) to allow model reuse in LoadedGteQwenModel.
#[inline]
pub(crate) async fn forward_pass_with_task(
    tokenizer: Tokenizer,
    model: Model,
    device: Device,
    texts: Vec<String>,
    task: Option<String>,
) -> Result<(Model, Vec<Vec<f32>>)> {
    // Format input with task-specific instruction prefix
    let formatted_texts: Vec<String> = match task.as_deref() {
        Some("search_query") => texts.iter()
            .map(|text| format!("Instruct: Given a web search query, retrieve relevant passages that answer the query.\nQuery: {}", text))
            .collect(),
        Some("search_document") | None => texts.iter()
            .map(|text| format!("Instruct: Given a web search query, retrieve relevant passages that answer the query.\nPassage: {}", text))
            .collect(),
        Some(custom_task) => texts.iter()
            .map(|text| format!("Instruct: {}.\nText: {}", custom_task, text))
            .collect(),
    };

    // Tokenize - wrap in spawn_blocking for CPU-intensive operation
    let tokenizer_clone = tokenizer.clone();
    let tokens = tokio::task::spawn_blocking(move || {
        tokenizer_clone
            .encode_batch(formatted_texts, true)
            .map_err(|e| MemoryError::ModelError(format!("Tokenization failed: {}", e)))
    })
    .await
    .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))??;

    // Collect into 2D vectors
    let ids_vecs: Vec<Vec<u32>> = tokens
        .iter()
        .map(|t| t.get_ids().to_vec())
        .collect();

    let mask_vecs: Vec<Vec<u32>> = tokens
        .iter()
        .map(|t| t.get_attention_mask().to_vec())
        .collect();

    // Create tensors directly from 2D data
    let token_ids = Tensor::new(ids_vecs, &device)
        .map_err(|e| MemoryError::ModelError(format!("Failed to create batch input tensor: {}", e)))?;
    let attention_mask = Tensor::new(mask_vecs, &device)
        .map_err(|e| MemoryError::ModelError(format!("Failed to create batch attention mask: {}", e)))?;

    // Forward pass - wrap in spawn_blocking for CPU-intensive operation
    let token_ids_clone = token_ids.clone();
    let attention_mask_clone = attention_mask.clone();
    let (returned_model, logits) = tokio::task::spawn_blocking(move || {
        let mut model_mut = model;
        let result = model_mut
            .forward(&token_ids_clone, 0, None)
            .map_err(|e| MemoryError::ModelError(format!("Forward pass failed: {}", e)));
        (model_mut, result)
    })
    .await
    .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))?;
    let logits = logits?;

    // Apply attention-masked last token pooling
    let (_batch_size, _seq_len, _hidden_size) = logits
        .dims3()
        .map_err(|e| MemoryError::ModelError(format!("Invalid logits shape: {}", e)))?;

    // Extract embeddings - wrap in spawn_blocking for CPU-intensive operation
    let embeddings_data = tokio::task::spawn_blocking(move || {
        // Find actual last tokens using attention mask
        let last_indices = attention_mask_clone
            .sum(1)
            .map_err(|e| MemoryError::ModelError(format!("Failed to sum attention mask: {}", e)))?
            .to_vec1::<u32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert indices: {}", e)))?
            .into_iter()
            .map(|len| (len - 1) as usize)
            .collect::<Vec<_>>();

        // Extract embeddings for each sequence's actual last token
        let mut batch_embeddings = Vec::new();
        for (i, &last_idx) in last_indices.iter().enumerate() {
            let seq_embeddings = logits
                .get(i)
                .map_err(|e| {
                    MemoryError::ModelError(format!("Failed to get sequence {}: {}", i, e))
                })?
                .get(last_idx)
                .map_err(|e| {
                    MemoryError::ModelError(format!("Failed to get token {}: {}", last_idx, e))
                })?;
            batch_embeddings.push(seq_embeddings);
        }

        let embeddings = Tensor::stack(&batch_embeddings, 0)
            .map_err(|e| MemoryError::ModelError(format!("Failed to stack embeddings: {}", e)))?;

        embeddings
            .to_vec2::<f32>()
            .map_err(|e| MemoryError::ModelError(format!("Failed to convert embeddings: {}", e)))
    })
    .await
    .map_err(|e| MemoryError::ModelError(format!("Spawn blocking failed: {}", e)))??;

    Ok((returned_model, embeddings_data))
}
