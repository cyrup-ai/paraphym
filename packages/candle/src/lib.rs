//! Fluent AI Candle Library
#![feature(impl_trait_in_fn_trait_return)]
#![feature(impl_trait_in_assoc_type)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(fn_traits)]

//! This crate provides Candle ML framework integration for AI services.
//! All Candle-prefixed domain types, builders, and providers are defined here
//! to ensure complete independence from the main paraphym packages.

pub mod macros;

// Candle-specific modules (minimal set for core functionality)
/// Candle builders for zero-allocation construction patterns
pub mod builders;
/// Chat functionality is now available through domain::chat
/// Core components (engine, generation, etc.)
pub mod core;
/// Candle domain types (replaces paraphym_domain dependency)  
pub mod domain;
/// Candle macros for ARCHITECTURE.md syntax support

/// Candle model providers for local inference
pub mod providers;
/// Real workflow execution system with streams-only architecture
pub mod workflow;
/// Memory system with cognitive features and vector storage
pub mod memory;

// Essential Candle re-exports for public API (minimal set)
// Domain types will be added as they become available

// Prelude - All types needed for ARCHITECTURE.md syntax
pub mod prelude {
    pub use crate::builders::{CandleAgentBuilder, CandleAgentRoleBuilder, CandleFluentAi};
    // Re-export generation types from modular structure
    pub use crate::core::generation::{
        CandleLlamaModel, CandleModel, GenerationStatistics, SamplingConfig, SimdMetrics,
        SpecialTokens, TextGenerator, TokenHistory,
    };
    // Core engine types for model-agnostic inference
    pub use crate::core::{
        Engine, EngineConfig, EngineError, EngineResult, ModelArchitecture, ModelConfig,
        ModelConfigError,
    };
    pub use crate::domain::chat::message::CandleMessageChunk;
    pub use crate::domain::chat::CandleChatLoop;
    pub use crate::domain::{
        agent::CandleAgent,
        chat::message::types::CandleMessageRole,
        context::{
            provider::{CandleContext, CandleDirectory, CandleFile, CandleFiles, CandleGithub},
            FinishReason,
        },
        tool::{
            core::{CalculatorTool, CandlePerplexity},
            CandleExecToText,
        },
    };
    pub use crate::providers::{
        CandleKimiK2Config, CandleKimiK2Provider, CandleQwen3CoderConfig, CandleQwen3CoderProvider,
    };
    // Real workflow execution types - streams-only architecture
    pub use crate::workflow::{candle_workflow, CandleExecutableWorkflow, CandleWorkflowStep};

    // Model types for ARCHITECTURE.md integration with generation system
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CandleModels {
        KimiK2,
        Qwen3Coder,
        Llama,
    }

    impl CandleModels {
        /// Get the model type string for ModelFactory::create_from_type()
        pub fn model_type(&self) -> &'static str {
            match self {
                CandleModels::KimiK2 => "kimi-k2",
                CandleModels::Qwen3Coder => "qwen3-coder",
                CandleModels::Llama => "llama",
            }
        }

        /// Create model using the generation system ModelFactory
        pub fn create_model(
            &self,
            config: std::sync::Arc<crate::core::ModelConfig>,
            device: candle_core::Device,
        ) -> Result<
            Box<dyn crate::core::generation::models::CandleModel>,
            crate::domain::model::error::CandleModelError,
        > {
            crate::core::generation::models::ModelFactory::create_from_type(
                self.model_type(),
                config,
                device,
            )
        }
    }

    // Implement CandleCompletionModel trait for builder integration
    impl crate::domain::completion::CandleCompletionModel for CandleModels {
        fn prompt(
            &self,
            prompt: crate::domain::prompt::CandlePrompt,
            _params: &crate::domain::completion::types::CandleCompletionParams,
        ) -> ystream::AsyncStream<crate::domain::completion::CandleCompletionChunk>
        {
            // Delegate to the appropriate provider based on model type
            let model_type = *self; // Copy the enum value to move into closure
            ystream::AsyncStream::with_channel(move |sender| {
                match model_type {
                    CandleModels::KimiK2 => {
                        // Route to KimiK2Provider with async handling
                        ystream::spawn_task(|| async move {
                            // Create provider with default config
                            let provider_result =
                                crate::providers::kimi_k2::CandleKimiK2Provider::new().await;

                            match provider_result {
                                Ok(_provider) => {
                                    // For now, return a placeholder until full inference is implemented
                                    let chunk =
                                        crate::domain::completion::CandleCompletionChunk::Text(
                                            format!("KimiK2 completion: {}", prompt.content()),
                                        );
                                    let _ = sender.send(chunk);
                                }
                                Err(_err) => {
                                    // Send error chunk if provider creation fails
                                    let error_chunk =
                                        crate::domain::completion::CandleCompletionChunk::Error(
                                            "Failed to initialize KimiK2 provider".to_string(),
                                        );
                                    let _ = sender.send(error_chunk);
                                }
                            }
                        });
                    }
                    CandleModels::Qwen3Coder | CandleModels::Llama => {
                        // TODO: Route to other providers
                        let chunk =
                            crate::domain::completion::CandleCompletionChunk::Text(format!(
                                "Completion from {} model: {}",
                                model_type.model_type(),
                                prompt.content()
                            ));
                        let _ = sender.send(chunk);
                    }
                }
            })
        }
    }

    pub struct CandleLibrary;

    impl CandleLibrary {
        pub fn named(_name: &str) -> Self {
            Self
        }
    }

    // Re-export tool implementation that provides static methods
    pub use ystream::AsyncStream;

    pub use crate::domain::tool::core::CandleToolImpl as CandleTool;

    // Helper function for ARCHITECTURE.md example
    pub fn process_turn() -> CandleChatLoop {
        CandleChatLoop::Reprompt("continue".to_string())
    }
}

// Re-export everything from prelude at root level for convenience
// Alias for backward compatibility - people expect async_task module
pub use ystream as async_task;
pub use ystream::spawn_task as spawn_async;
// Streaming primitives from paraphym-async (kept as-is per requirements)
pub use ystream::{spawn_task, AsyncStream, AsyncStreamSender, AsyncTask};
// SIMD operations from paraphym-simd for high-performance ML workloads
pub use paraphym_simd;
pub use prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_md_syntax_works() {
        // Test that ARCHITECTURE.md builder pattern still works after all fixes
        let _agent = CandleFluentAi::agent_role("test-agent")
            .temperature(0.7)
            .max_tokens(1000)
            .system_prompt("You are a helpful assistant")
            .into_agent();

        // If this compiles, the ARCHITECTURE.md syntax is working! ✅
    }
}
