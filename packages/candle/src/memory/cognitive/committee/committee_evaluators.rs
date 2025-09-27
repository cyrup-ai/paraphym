//! Committee Evaluators Using Existing Providers
//!
//! Committee evaluation implementation using existing CandleKimiK2Provider and CandleQwen3CoderProvider.

use std::sync::Arc;
// StreamExt not currently used but may be needed for future async operations
use crate::memory::cognitive::types::CognitiveError;
use crate::domain::completion::traits::CandleCompletionModel;
use crate::memory::cognitive::committee::committee_types::{
    Committee, CommitteeConfig
};
use crate::domain::{
    completion::CandleCompletionParams,
    context::chunk::CandleCompletionChunk,
    prompt::CandlePrompt,
};

/// Committee evaluator using existing providers (CandleKimiK2Provider, CandleQwen3CoderProvider)
#[derive(Debug)]
pub struct ProviderCommitteeEvaluator {
    committee: Arc<Committee>,
}

impl ProviderCommitteeEvaluator {
    /// Create new committee evaluator using existing providers
    pub async fn new() -> Result<Self, CognitiveError> {
        let config = CommitteeConfig::default();
        let committee = Arc::new(Committee::new(config).await?);

        Ok(Self { committee })
    }

    /// Create evaluator with custom configuration
    pub async fn with_config(config: CommitteeConfig) -> Result<Self, CognitiveError> {
        let committee = Arc::new(Committee::new(config).await?);

        Ok(Self { committee })
    }

    /// Evaluate content using existing providers for real AI evaluation
    pub async fn evaluate(&self, content: &str) -> Result<f64, CognitiveError> {
        self.committee.evaluate(content).await
    }

    /// Generate evaluation report using AI
    pub async fn generate_report(&self, content: &str) -> Result<String, CognitiveError> {
        let score = self.evaluate(content).await?;
        Ok(format!("AI evaluation score: {:.2} (using local CandleKimiK2Provider)", score))
    }

    /// Evaluate with KimiK2 provider
    pub async fn evaluate_with_kimi(&self, content: &str) -> Result<String, CognitiveError> {
        let evaluation_prompt = format!(
            "Provide a detailed evaluation of this content including strengths, weaknesses, and an overall quality assessment:\n\nContent:\n{}",
            content
        );

        let prompt = CandlePrompt::new(&evaluation_prompt);
        let params = CandleCompletionParams::default();
        
        let mut response = String::new();
        let stream = self.committee.kimi_provider.prompt(prompt, &params);
        let stream = Box::pin(stream);

        while let Some(chunk) = stream.next().await {
            match chunk {
                CandleCompletionChunk::Text(text) => response.push_str(&text),
                CandleCompletionChunk::Complete { text, .. } => {
                    response.push_str(&text);
                    break;
                }
                _ => {}
            }
        }

        Ok(response)
    }

    /// Evaluate with Qwen provider
    pub async fn evaluate_with_qwen(&self, content: &str) -> Result<String, CognitiveError> {
        let evaluation_prompt = format!(
            "Provide a detailed evaluation of this content including strengths, weaknesses, and an overall quality assessment:\n\nContent:\n{}",
            content
        );

        let prompt = CandlePrompt::new(&evaluation_prompt);
        let params = CandleCompletionParams::default();
        
        let mut response = String::new();
        let stream = self.committee.qwen_provider.prompt(prompt, &params);
        let stream = Box::pin(stream);

        while let Some(chunk) = stream.next().await {
            match chunk {
                CandleCompletionChunk::Text(text) => response.push_str(&text),
                CandleCompletionChunk::Complete { text, .. } => {
                    response.push_str(&text);
                    break;
                }
                _ => {}
            }
        }

        Ok(response)
    }

    /// Multi-provider consensus evaluation using both providers directly
    pub async fn consensus_evaluate(&self, content: &str) -> Result<f64, CognitiveError> {
        let evaluation_prompt = format!(
            "Rate this content quality from 0.0 to 1.0. Return only the number:\n\n{}",
            content
        );

        let prompt = CandlePrompt::new(&evaluation_prompt);
        let params = CandleCompletionParams::default();
        let mut scores = Vec::new();

        // Evaluate with KimiK2 provider
        let mut kimi_response = String::new();
        let kimi_stream = self.committee.kimi_provider.prompt(prompt.clone(), &params);
        let kimi_stream = Box::pin(kimi_stream);

        while let Some(chunk) = kimi_stream.next().await {
            match chunk {
                CandleCompletionChunk::Text(text) => kimi_response.push_str(&text),
                CandleCompletionChunk::Complete { text, .. } => {
                    kimi_response.push_str(&text);
                    break;
                }
                _ => {}
            }
        }
        if let Some(score) = super::committee_types::parse_score_from_response(&kimi_response) {
            scores.push(score);
        }

        // Evaluate with Qwen provider
        let mut qwen_response = String::new();
        let qwen_stream = self.committee.qwen_provider.prompt(prompt, &params);
        let qwen_stream = Box::pin(qwen_stream);

        while let Some(chunk) = qwen_stream.next().await {
            match chunk {
                CandleCompletionChunk::Text(text) => qwen_response.push_str(&text),
                CandleCompletionChunk::Complete { text, .. } => {
                    qwen_response.push_str(&text);
                    break;
                }
                _ => {}
            }
        }
        if let Some(score) = super::committee_types::parse_score_from_response(&qwen_response) {
            scores.push(score);
        }

        if scores.is_empty() {
            return Err(CognitiveError::EvaluationError("No valid scores from providers".to_string()));
        }

        let consensus = scores.iter().sum::<f64>() / scores.len() as f64;
        Ok(consensus)
    }
}