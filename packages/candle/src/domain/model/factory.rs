//! Model factory for creating completion providers from HuggingFace registry keys

use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::capability::text_to_text::{
    CandlePhi4ReasoningProvider,
    CandleKimiK2Provider,
    CandleQwen3CoderProvider,
};
use crate::domain::agent::role::CandleCompletionProviderType;

/// Model creation result type
type ModelResult = Result<CandleCompletionProviderType, Box<dyn std::error::Error + Send + Sync>>;

/// Model creation function type - async function that creates a provider
type ModelCreatorFn = fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ModelResult> + Send>>;

/// Registry of known GGUF models by their HuggingFace identifiers
static MODEL_CREATORS: Lazy<HashMap<&'static str, ModelCreatorFn>> = Lazy::new(|| {
    let mut map = HashMap::new();
    
    // Register Phi-4-Reasoning
    map.insert("unsloth/Phi-4-reasoning-GGUF", (|| {
        Box::pin(async {
            let provider = CandlePhi4ReasoningProvider::default_for_builder()?;
            Ok(CandleCompletionProviderType::Phi4Reasoning(provider))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = ModelResult> + Send>>
    }) as ModelCreatorFn);
    
    // Register Kimi-K2
    map.insert("unsloth/Kimi-K2-Instruct-GGUF", (|| {
        Box::pin(async {
            let provider = CandleKimiK2Provider::default_for_builder()?;
            Ok(CandleCompletionProviderType::KimiK2(provider))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = ModelResult> + Send>>
    }) as ModelCreatorFn);
    
    // Register Qwen3-Coder
    map.insert("Qwen/Qwen2.5-Coder-32B-Instruct-GGUF", (|| {
        Box::pin(async {
            let provider = CandleQwen3CoderProvider::new().await?;
            Ok(CandleCompletionProviderType::Qwen3Coder(provider))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = ModelResult> + Send>>
    }) as ModelCreatorFn);
    
    map
});

/// Model factory for creating models from registry keys
pub struct ModelFactory;

impl ModelFactory {
    /// Create a model from a HuggingFace-style registry key
    ///
    /// # Arguments
    /// * `registry_key` - HuggingFace repository identifier (e.g., "unsloth/Phi-4-reasoning-GGUF")
    ///
    /// # Returns
    /// Result containing the completion provider type
    ///
    /// # Errors
    /// Returns error if:
    /// - Registry key is not found and cannot be loaded generically
    /// - Model download fails
    /// - Model initialization fails
    ///
    /// # Examples
    /// ```ignore
    /// // Load known model
    /// let provider = ModelFactory::create_from_registry_key("unsloth/Phi-4-reasoning-GGUF").await?;
    ///
    /// // Load any HuggingFace GGUF model (future extension)
    /// let provider = ModelFactory::create_from_registry_key("TheBloke/Mistral-7B-Instruct-v0.2-GGUF").await?;
    /// ```
    pub async fn create_from_registry_key(
        registry_key: &str
    ) -> Result<CandleCompletionProviderType, Box<dyn std::error::Error + Send + Sync>> {
        // Check if it's a known model with registered creator
        if let Some(creator) = MODEL_CREATORS.get(registry_key) {
            return creator().await;
        }
        
        // For unknown models, return error for now
        // Future: could implement generic GGUF loading using DownloadProviderFactory
        Err(format!(
            "Model '{}' not yet supported. Supported models: {}",
            registry_key,
            Self::list_available_models().join(", ")
        ).into())
    }
    
    /// List all registered model registry keys
    ///
    /// # Returns
    /// Vector of supported HuggingFace repository identifiers
    pub fn list_available_models() -> Vec<&'static str> {
        MODEL_CREATORS.keys().copied().collect()
    }
    
    /// Create a model from a short alias
    ///
    /// # Arguments
    /// * `alias` - Short model name (e.g., "phi4", "kimi", "qwen")
    ///
    /// # Returns
    /// Result containing the completion provider type
    ///
    /// # Examples
    /// ```ignore
    /// let provider = ModelFactory::create_from_alias("phi4").await?;
    /// ```
    pub async fn create_from_alias(
        alias: &str
    ) -> Result<CandleCompletionProviderType, Box<dyn std::error::Error + Send + Sync>> {
        let registry_key = match alias.to_lowercase().as_str() {
            "phi4" | "phi-4" | "phi4-reasoning" => "unsloth/Phi-4-reasoning-GGUF",
            "kimi" | "kimi-k2" => "unsloth/Kimi-K2-Instruct-GGUF",
            "qwen" | "qwen-coder" | "qwen3-coder" => "Qwen/Qwen2.5-Coder-32B-Instruct-GGUF",
            _ => return Err(format!("Unknown model alias: {}", alias).into()),
        };
        
        Self::create_from_registry_key(registry_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_available_models() {
        let models = ModelFactory::list_available_models();
        assert!(models.contains(&"unsloth/Phi-4-reasoning-GGUF"));
        assert!(models.contains(&"unsloth/Kimi-K2-Instruct-GGUF"));
        assert!(models.contains(&"Qwen/Qwen2.5-Coder-32B-Instruct-GGUF"));
    }
}
