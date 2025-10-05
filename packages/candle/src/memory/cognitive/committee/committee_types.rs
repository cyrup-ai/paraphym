//! Committee Types with Direct Provider Usage
//!
//! Committee evaluation using CandleKimiK2Provider and CandleQwen3CoderProvider directly.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

// StreamExt not currently used but may be needed for future async operations

use crate::memory::cognitive::types::CognitiveError;
use crate::capability::text_to_text::{CandleKimiK2Provider, CandleQwen3CoderProvider};
use crate::domain::{
    completion::{CandleCompletionModel, CandleCompletionParams},
    context::chunk::CandleCompletionChunk,
    prompt::CandlePrompt,
};

/// Committee configuration for provider-based evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeConfig {
    pub member_count: usize,
    pub consensus_threshold: f64,
    pub use_multiple_providers: bool,
}

impl Default for CommitteeConfig {
    fn default() -> Self {
        Self {
            member_count: 3,
            consensus_threshold: 0.7,
            use_multiple_providers: true,
        }
    }
}

// ModelType enum removed - use provider instances directly

// CommitteeProviderPool removed - use providers directly

/// Committee for memory evaluation using providers directly
#[derive(Debug)]
pub struct Committee {
    pub config: CommitteeConfig,
    pub kimi_provider: Arc<CandleKimiK2Provider>,
    pub qwen_provider: Arc<CandleQwen3CoderProvider>,
}

impl Committee {
    pub async fn new(config: CommitteeConfig) -> Result<Self, CognitiveError> {
        let kimi_provider = Arc::new(CandleKimiK2Provider::new().await
            .map_err(|e| CognitiveError::InitializationError(e.to_string()))?);
        let qwen_provider = Arc::new(CandleQwen3CoderProvider::new().await
            .map_err(|e| CognitiveError::InitializationError(e.to_string()))?);

        Ok(Self { config, kimi_provider, qwen_provider })
    }


    /// Evaluate content using KimiK2 provider directly
    pub async fn evaluate(&self, content: &str) -> Result<f64, CognitiveError> {
        let evaluation_prompt = format!(
            "Evaluate the quality of this content on a scale from 0.0 to 1.0, considering factors like clarity, relevance, and completeness. Return only a decimal number between 0.0 and 1.0.\n\nContent:\n{}",
            content
        );

        let prompt = CandlePrompt::new(&evaluation_prompt);
        let params = CandleCompletionParams::default();

        // Use real provider.prompt() method - no abstraction layer
        let mut response = String::new();
        let stream = self.kimi_provider.prompt(prompt, &params);
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

        let score = parse_score_from_response(&response).unwrap_or(0.5);
        Ok(score)
    }

}

/// Parse numerical score from AI response
pub fn parse_score_from_response(response: &str) -> Option<f64> {
    // Look for decimal numbers in the response
    use regex::Regex;
    let re = Regex::new(r"([01]?\.\d+|[01]\.?0*)").ok()?;

    for cap in re.captures_iter(response) {
        if let Ok(score) = cap[1].parse::<f64>()
            && (0.0..=1.0).contains(&score) {
                return Some(score);
            }
    }
    None
}