//! Fluent AI Candle Library
#![recursion_limit = "256"]
#![feature(impl_trait_in_fn_trait_return)]
#![feature(impl_trait_in_assoc_type)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(fn_traits)]

//! This crate provides Candle ML framework integration for AI services.
//! All Candle-prefixed domain types, builders, and providers are defined here
//! to ensure complete independence from the main paraphym packages.

// Initialize performance optimizations on library load
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize library-wide performance optimizations
pub fn init_candle() {
    INIT.call_once(|| {
        // Initialize timestamp caching for high-performance operations
        domain::memory::cache::initialize_timestamp_cache();

        // Initialize memory node pool for zero-allocation memory operations
        // Using 1000 nodes with 768-dimensional embeddings (typical for BERT-base)
        domain::memory::pool::initialize_memory_node_pool(1000, 768);
    });
}

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
/// Shared Tokio runtime for avoiding multiple runtime creation
pub mod runtime;
/// Model system for provider and model enumeration
pub mod model;
/// Async stream utilities re-exporting ystream
pub mod async_stream;

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
            SweetMcpRouter, RouterError, ToolRoute, ToolInfo,
        },
        image_generation::{
            ImageGenerationConfig,
            ImageGenerationChunk,
            ImageGenerationModel,
            tensor_to_image,
        },
    };
    pub use crate::providers::{
        CandleKimiK2Config, CandleKimiK2Provider, CandleQwen3CoderConfig, CandleQwen3CoderProvider,
        SD35TurboConfig, StableDiffusion35Turbo,
        FluxConfig, FluxSchnell,
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
            let params = _params.clone(); // Clone params to own the data
            ystream::AsyncStream::with_channel(move |sender| {
                match model_type {
                    CandleModels::KimiK2 => {
                        // Clone values before moving into async closure
                        let prompt_clone = prompt.clone();
                        let params_clone = params.clone();
                        
                        // Route to KimiK2Provider with async handling
                        ystream::spawn_task(|| async move {
                            // Create provider with default config
                            let provider_result =
                                crate::providers::kimi_k2::CandleKimiK2Provider::new().await;

                            match provider_result {
                                Ok(provider) => {
                                    // Call real provider.prompt() method for inference
                                    let completion_stream = provider.prompt(prompt_clone, &params_clone);
                                    
                                    // Stream all chunks from provider to outer sender
                                    while let Some(chunk) = completion_stream.try_next() {
                                        if sender.send(chunk).is_err() {
                                            // Receiver dropped, exit gracefully
                                            return;
                                        }
                                    }
                                }
                                Err(err) => {
                                    // Send error chunk if provider creation fails
                                    let error_chunk =
                                        crate::domain::completion::CandleCompletionChunk::Error(
                                            format!("Failed to initialize KimiK2 provider: {}", err)
                                        );
                                    let _ = sender.send(error_chunk);
                                }
                            }
                        });
                    }
                    CandleModels::Qwen3Coder => {
                        // Clone values before moving into async closure
                        let prompt_clone = prompt.clone();
                        let params_clone = params.clone();
                        
                        // Route to Qwen3CoderProvider with async handling
                        ystream::spawn_task(|| async move {
                            // Create provider with default config
                            let provider_result =
                                crate::providers::qwen3_coder::CandleQwen3CoderProvider::new().await;

                            match provider_result {
                                Ok(provider) => {
                                    // Call real provider.prompt() method for inference
                                    let completion_stream = provider.prompt(prompt_clone, &params_clone);
                                    
                                    // Stream all chunks from provider to outer sender
                                    while let Some(chunk) = completion_stream.try_next() {
                                        if sender.send(chunk).is_err() {
                                            // Receiver dropped, exit gracefully
                                            return;
                                        }
                                    }
                                }
                                Err(err) => {
                                    // Send error chunk if provider creation fails
                                    let error_chunk =
                                        crate::domain::completion::CandleCompletionChunk::Error(
                                            format!("Failed to initialize Qwen3Coder provider: {}", err)
                                        );
                                    let _ = sender.send(error_chunk);
                                }
                            }
                        });
                    }
                    CandleModels::Llama => {
                        // Llama provider not yet implemented
                        let error_chunk =
                            crate::domain::completion::CandleCompletionChunk::Error(
                                "Llama provider not yet implemented. Available models: KimiK2, Qwen3Coder".to_string()
                            );
                        let _ = sender.send(error_chunk);
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
