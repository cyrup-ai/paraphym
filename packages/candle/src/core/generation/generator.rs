//! Core text generation engine with SIMD acceleration
//!
//! This module contains the TextGenerator implementation with tokio stream token streaming,
//! SIMD-optimized sampling methods, and pure SIMD delegation without scalar fallbacks.

use candle_core::{Device, Tensor};
use tokenizers::Tokenizer;
use tokio_stream::Stream;
use crate::async_stream;

use crate::domain::context::chunk::CandleStringChunk;
use paraphym_simd::logits::LogitsProcessor as LogitsProcessorTrait;
use paraphym_simd::logits::constraints::GenerationConstraint;

use super::{
    config::SamplingConfig,
    metrics::SimdMetrics,
    models::CandleModel,
    stats::GenerationStatistics,
    tokens::{SpecialTokens, TokenHistory},
    types::CandleResult,
};

// Import constraint types for schema-based generation
use paraphym_simd::logits::constraints::{JsonConstraint, json::JsonState};

/// Core text generation engine with SIMD acceleration
///
/// Provides tokio stream-based token generation with comprehensive SIMD optimization,
/// statistics tracking, and configurable sampling parameters. Uses pure SIMD
/// delegation without scalar fallbacks for maximum performance.
pub struct TextGenerator {
    /// The model for text generation
    pub model: Box<dyn CandleModel>,

    /// Tokenizer for encoding/decoding
    pub tokenizer: Tokenizer,

    /// Device for tensor operations
    pub device: Device,

    /// Sampling configuration
    pub config: SamplingConfig,

    /// Token history for repetition penalties
    pub token_history: TokenHistory,

    /// Generation statistics tracking
    pub stats: GenerationStatistics,

    /// SIMD performance metrics
    pub simd_metrics: SimdMetrics,

    /// Optional JSON constraint for structured generation
    pub constraint: Option<JsonConstraint<'static>>,

    /// Current JSON constraint state
    pub constraint_state: Option<JsonState>,
}
impl TextGenerator {
    /// Create new TextGenerator
    pub fn new(
        model: Box<dyn CandleModel>,
        tokenizer: Tokenizer,
        device: Device,
        config: SamplingConfig,
    ) -> Self {
        let max_history = config.repetition_context_length;

        Self {
            model,
            tokenizer,
            device,
            config,
            token_history: TokenHistory::new(max_history),
            stats: GenerationStatistics::new(),
            simd_metrics: SimdMetrics::new(),
            constraint: None,
            constraint_state: None,
        }
    }

    /// Generate text using tokio stream with SIMD acceleration
    pub fn generate(
        mut self,
        prompt: String,
        max_tokens: u32,
        special_tokens: SpecialTokens,
    ) -> impl Stream<Item = crate::domain::context::chunk::CandleStringChunk> {
        async_stream::spawn_stream(move |tx| async move {
            self.stats.start_generation();

            // Encode prompt to tokens using tokenizer
            let tokens = match self.tokenizer.encode(prompt.as_str(), true) {
                Ok(encoded) => encoded.get_ids().to_vec(),
                Err(e) => {
                    log::error!("Prompt encoding error: {}", e);
                    return;
                }
            };

            self.stats.set_input_tokens(tokens.len() as u64);
            let mut all_tokens = tokens.clone();
            let mut position = 0;

            // Initial forward pass
            let initial_input = match Tensor::new(tokens.as_slice(), &self.device) {
                Ok(tensor) => match tensor.unsqueeze(0) {
                    Ok(unsqueezed) => unsqueezed,
                    Err(e) => {
                        log::error!("Initial tensor creation error: {}", e);
                        return;
                    }
                },
                Err(e) => {
                    log::error!("Initial tensor from slice error: {}", e);
                    return;
                }
            };

            let initial_logits = match self.model.forward(&initial_input, position) {
                Ok(logits) => match logits.squeeze(0) {
                    Ok(squeezed) => squeezed,
                    Err(e) => {
                        log::error!("Initial logits squeeze error: {}", e);
                        return;
                    }
                },
                Err(e) => {
                    log::error!("Initial forward pass error: {}", e);
                    return;
                }
            };

            self.stats.record_forward_pass();
            let logits_vec = match initial_logits.to_vec1::<f32>() {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Converting initial logits to vector error: {}", e);
                    return;
                }
            };
            let mut next_token = match self.sample_token(&logits_vec, &tokens) {
                Ok(token) => token,
                Err(e) => {
                    log::error!("Initial SIMD sampling error: {}", e);
                    return;
                }
            };
            all_tokens.push(next_token);
            self.token_history.push(next_token);

            // Update constraint state after token sampling
            if let Err(e) = self.update_constraint_state(next_token) {
                log::warn!("Failed to update constraint state: {}", e);
            }

            position += 1;

            // Check termination before decoding
            if self.should_stop(next_token, &special_tokens) {
                self.stats.stop_generation();
                return; // Graceful EOS termination
            }

            // Decode and emit initial token
            match self.tokenizer.decode(&[next_token], false) {
                Ok(token_str) => {
                    let _ = tx.send(CandleStringChunk(token_str));
                }
                Err(e) => {
                    log::error!("Initial token decoding error: {}", e);
                    return;
                }
            };

            // Generation loop - stream each token as generated
            for _index in 1..max_tokens {
                // Prepare input tensor for next forward pass
                let input = match Tensor::new(&[next_token], &self.device) {
                    Ok(tensor) => match tensor.unsqueeze(0) {
                        Ok(unsqueezed) => unsqueezed,
                        Err(e) => {
                            log::error!("Tensor creation in loop error: {}", e);
                            break;
                        }
                    },
                    Err(e) => {
                        log::error!("Tensor from single token error: {}", e);
                        break;
                    }
                };

                // Forward pass using model
                let logits = match self.model.forward(&input, position) {
                    Ok(logits) => match logits.squeeze(0) {
                        Ok(squeezed) => squeezed,
                        Err(e) => {
                            log::error!("Logits squeeze in loop error: {}", e);
                            break;
                        }
                    },
                    Err(e) => {
                        log::error!("Forward pass in loop error: {}", e);
                        break;
                    }
                };

                self.stats.record_forward_pass();
                // SIMD sampling using existing infrastructure
                let logits_vec = match logits.to_vec1::<f32>() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Converting logits to vector in loop error: {}", e);
                        break;
                    }
                };
                next_token = match self.sample_token(&logits_vec, &all_tokens) {
                    Ok(token) => token,
                    Err(e) => {
                        log::error!("SIMD sampling in loop error: {}", e);
                        break;
                    }
                };
                all_tokens.push(next_token);
                self.token_history.push(next_token);

                // Update constraint state after token sampling
                if let Err(e) = self.update_constraint_state(next_token) {
                    log::warn!("Failed to update constraint state: {}", e);
                }

                position += 1;

                // Check termination before decoding
                if self.should_stop(next_token, &special_tokens) {
                    break; // Graceful EOS termination
                }

                // Decode and emit individual token
                match self.tokenizer.decode(&[next_token], false) {
                    Ok(token_str) => {
                        let _ = tx.send(CandleStringChunk(token_str)); // Individual token streaming
                    }
                    Err(e) => {
                        log::error!("Token decoding in loop error: {}", e);
                        break;
                    }
                };

                self.stats.add_tokens(1);
            }

            self.stats.stop_generation();
        })
    }
    /// SIMD-optimized token sampling with comprehensive acceleration
    pub fn sample_token(&mut self, logits: &[f32], _context: &[u32]) -> CandleResult<u32> {
        use paraphym_simd::{
            argmax, prepare_nucleus_sampling_simd, scale_temperature, softmax, topk_filtering_simd,
        };

        let mut logits = logits.to_vec();

        // Apply temperature scaling with SIMD
        if self.config.temperature != 1.0 {
            scale_temperature(&mut logits, self.config.temperature).map_err(|e| {
                crate::domain::model::error::CandleModelError::OperationNotSupported(
                    e.to_string().into(),
                )
            })?;
            self.simd_metrics.record_temperature_op();
        }

        // Apply top-k filtering with SIMD
        if let Some(k) = self.config.top_k {
            topk_filtering_simd(&mut logits, k).map_err(|e| {
                crate::domain::model::error::CandleModelError::OperationNotSupported(
                    e.to_string().into(),
                )
            })?;
            self.simd_metrics.record_topk_op();
        }

        // Apply nucleus sampling with SIMD
        if let Some(p) = self.config.top_p {
            prepare_nucleus_sampling_simd(&mut logits, p).map_err(|e| {
                crate::domain::model::error::CandleModelError::OperationNotSupported(
                    e.to_string().into(),
                )
            })?;
            self.simd_metrics.record_nucleus_op();
        }

        // Create processing context with all fields including constraints
        let context = paraphym_simd::context::ProcessingContext {
            temperature: self.config.temperature,
            top_k: self.config.top_k,
            top_p: self.config.top_p.map(|p| p as f32),
            token_history: self.token_history.as_slice(),
            start_time: None,
            max_new_tokens: None,
            stop_tokens: Vec::new(),
            json_constraint: self.constraint.clone(), // Add constraint from generator state
            json_constraint_state: self.constraint_state.clone(), // Add constraint state
            schema_constraint: None,
            schema_constraint_state: None,
        };

        // Use ConstrainedLogitsProcessor for all processing including constraints
        let processor_config = paraphym_simd::config::ProcessorConfig {
            temperature: self.config.temperature,
            top_k: self.config.top_k,
            top_p: self.config.top_p.map(|p| p as f32),
            repetition_penalty: self.config.repetition_penalty,
            frequency_penalty: self.config.frequency_penalty,
            presence_penalty: self.config.presence_penalty,
        };

        let mut processor =
            paraphym_simd::logits::constraints::ConstrainedLogitsProcessor::new(processor_config);
        processor.process(&mut logits, &context).map_err(|e| {
            crate::domain::model::error::CandleModelError::OperationNotSupported(
                e.to_string().into(),
            )
        })?;
        self.simd_metrics.record_penalty_op();
        // Convert to probabilities with SIMD softmax
        let probs = softmax(&logits).map_err(|e| {
            crate::domain::model::error::CandleModelError::OperationNotSupported(
                e.to_string().into(),
            )
        })?;
        self.simd_metrics.record_softmax_op();

        // Sample token with SIMD argmax (for deterministic) or weighted sampling
        let token = if self.config.is_deterministic() {
            argmax(&probs).map_err(|e| {
                crate::domain::model::error::CandleModelError::OperationNotSupported(
                    e.to_string().into(),
                )
            })?
        } else {
            // Weighted sampling implementation would go here
            argmax(&probs).map_err(|e| {
                crate::domain::model::error::CandleModelError::OperationNotSupported(
                    e.to_string().into(),
                )
            })?
        };

        self.simd_metrics.record_argmax_op();
        self.stats.record_simd_operation();

        Ok(token as u32)
    }

    /// Check if generation should stop
    pub fn should_stop(&self, token: u32, special_tokens: &SpecialTokens) -> bool {
        special_tokens.is_eos(token)
    }

    /// Get generation statistics
    pub fn stats(&self) -> &GenerationStatistics {
        &self.stats
    }

    /// Get SIMD metrics
    pub fn simd_metrics(&self) -> &SimdMetrics {
        &self.simd_metrics
    }

    /// Reset all statistics and metrics
    pub fn reset_stats(&mut self) {
        self.stats.reset();
        self.simd_metrics.reset();
        self.token_history.clear();
    }

    /// Set JSON constraint for structured generation
    pub fn set_json_constraint(
        &mut self,
        constraint: JsonConstraint<'static>,
    ) -> anyhow::Result<()> {
        let initial_state = constraint.new_state();
        self.constraint = Some(constraint);
        self.constraint_state = Some(initial_state);
        Ok(())
    }

    /// Remove JSON constraint
    pub fn clear_constraint(&mut self) {
        self.constraint = None;
        self.constraint_state = None;
    }

    /// Check if generation has active constraints
    pub fn has_constraints(&self) -> bool {
        self.constraint.is_some() && self.constraint_state.is_some()
    }

    /// Update constraint state after token generation
    pub fn update_constraint_state(&mut self, token: u32) -> anyhow::Result<bool> {
        if let (Some(constraint), Some(state)) = (&self.constraint, &mut self.constraint_state) {
            constraint.update(state, token)
        } else {
            Ok(true) // No constraints, continue generation
        }
    }

    /// Check if constraint-based generation is complete
    pub fn is_constraint_done(&self) -> bool {
        if let (Some(constraint), Some(state)) = (&self.constraint, &self.constraint_state) {
            constraint.is_done(state)
        } else {
            false // No constraints, not constraint-complete
        }
    }
}

impl std::fmt::Debug for TextGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextGenerator")
            .field("device", &self.device)
            .field("config", &self.config)
            .field("token_history", &self.token_history)
            .field("stats", &self.stats)
            .field("simd_metrics", &self.simd_metrics)
            .field("has_constraint", &self.constraint.is_some())
            .finish()
    }
}
