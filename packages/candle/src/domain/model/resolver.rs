//! Model resolution and lookup utilities

// Removed unused import: std::borrow::Cow
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::sync::LazyLock;

// Removed unused import: std::sync::Arc
use ahash::RandomState;
use ystream::AsyncStream;
// Removed unused import: once_cell::sync::Lazy
use regex::Regex;
use serde::{Deserialize, Serialize};
use strsim;

use crate::domain::model::error::{CandleModelError as ModelError, CandleResult as Result};
use crate::domain::model::info::CandleModelInfo as ModelInfo;
use crate::domain::model::traits::CandleModel as Model;
use crate::capability::registry::{self, AnyModel};

/// Cached environment variable for default provider
/// Using `LazyLock` prevents memory leaks while maintaining &'static str return type
static ENV_DEFAULT_PROVIDER: LazyLock<Option<&'static str>> = LazyLock::new(|| {
    std::env::var("CANDLE_DEFAULT_PROVIDER")
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| Box::leak(s.into_boxed_str()) as &'static str)
});

/// A pattern that can be used to match model names
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ModelPattern {
    /// Match exact model name
    Exact(String),

    /// Match model name with a glob pattern
    Pattern(String),

    /// Match model name with a regular expression
    Regex(String),
}

impl ModelPattern {
    /// Check if the pattern matches the given model name
    #[must_use]
    pub fn matches(&self, model_name: &str) -> bool {
        match self {
            ModelPattern::Exact(pattern) => pattern == model_name,
            ModelPattern::Pattern(pattern) => {
                // Convert glob pattern to regex
                let mut regex = String::with_capacity(pattern.len() * 2);
                regex.push('^');

                for c in pattern.chars() {
                    match c {
                        '*' => regex.push_str(".*"),
                        '?' => regex.push('.'),
                        '.' | '^' | '$' | '|' | '(' | ')' | '[' | ']' | '{' | '}' | '+' | '\\' => {
                            regex.push('\\');
                            regex.push(c);
                        }
                        _ => regex.push(c),
                    }
                }

                regex.push('$');

                // Compile the regex and check for a match
                Regex::new(&regex)
                    .map(|re| re.is_match(model_name))
                    .unwrap_or(false)
            }
            ModelPattern::Regex(pattern) => Regex::new(pattern)
                .map(|re| re.is_match(model_name))
                .unwrap_or(false),
        }
    }

    /// Get the pattern as a string
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            ModelPattern::Exact(s) | ModelPattern::Pattern(s) | ModelPattern::Regex(s) => s,
        }
    }
}

impl fmt::Display for ModelPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelPattern::Exact(s) => write!(f, "{s}"),
            ModelPattern::Pattern(s) => write!(f, "pattern:{s}"),
            ModelPattern::Regex(s) => write!(f, "regex:{s}"),
        }
    }
}

/// A rule for model resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResolutionRule {
    /// The pattern to match against model names
    pub pattern: ModelPattern,

    /// The target provider to use
    pub provider: String,

    /// The target model name to use
    pub target: String,

    /// The priority of this rule (higher = more specific)
    pub priority: i32,

    /// Optional condition for when this rule applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<RuleCondition>,
}

/// A condition for when a rule should apply
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RuleCondition {
    /// The rule only applies if the model has the specified capability
    HasCapability { capability: String },

    /// The rule only applies if the model has the specified feature flag
    HasFeature { feature: String },

    /// The rule only applies if the specified environment variable is set
    EnvVarSet { name: String },

    /// The rule only applies if the specified feature flag is enabled
    FeatureEnabled { name: String },
}

/// A model resolution result
#[derive(Debug, Clone, Default)]
pub struct ModelResolution {
    /// The resolved provider name
    pub provider: String,

    /// The resolved model name
    pub model: String,

    /// The model info (if available)
    pub info: Option<ModelInfo>,

    /// The rule that was used for resolution (if any)
    pub rule: Option<ModelResolutionRule>,

    /// The score of the match (higher is better)
    pub score: f64,
}

impl cyrup_sugars::prelude::MessageChunk for ModelResolution {
    fn bad_chunk(_error: String) -> Self {
        Self::default()
    }

    fn error(&self) -> Option<&str> {
        None
    }
}

/// Wrapper for model lookup results that implements `MessageChunk`
#[derive(Debug, Clone)]
pub struct ModelResult {
    /// The model if found
    pub model: Option<AnyModel>,
}

impl ModelResult {
    /// Create a result with a found model
    #[must_use]
    pub fn found(model: AnyModel) -> Self {
        Self { model: Some(model) }
    }
    
    /// Create a result with no model found
    #[must_use]
    pub fn not_found() -> Self {
        Self { model: None }
    }
}

impl Default for ModelResult {
    fn default() -> Self {
        Self::not_found()
    }
}

impl cyrup_sugars::prelude::MessageChunk for ModelResult {
    fn bad_chunk(_error: String) -> Self {
        Self::default()
    }

    fn error(&self) -> Option<&str> {
        None
    }
}

impl ModelResolution {
    /// Create a new resolution result
    pub fn new(
        provider: impl Into<String>,
        model: impl Into<String>,
        info: Option<ModelInfo>,
        rule: Option<ModelResolutionRule>,
        score: f64,
    ) -> Self {
        Self {
            provider: provider.into(),
            model: model.into(),
            info,
            rule,
            score,
        }
    }

    /// Check if the resolution is valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.score > 0.0 && !self.provider.is_empty() && !self.model.is_empty()
    }
}

/// A resolver for model names and providers
#[derive(Clone)]
pub struct ModelResolver {
    rules: Vec<ModelResolutionRule>,
    aliases: HashMap<String, (String, String), RandomState>,
    feature_flags: HashMap<String, bool, RandomState>,
    default_provider: Option<&'static str>,
}

impl Default for ModelResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelResolver {
    /// Create a new model resolver
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            aliases: HashMap::with_hasher(RandomState::new()),
            feature_flags: HashMap::with_hasher(RandomState::new()),
            default_provider: None,
        }
    }

    /// Add a resolution rule
    pub fn add_rule(&mut self, rule: ModelResolutionRule) {
        self.rules.push(rule);
        // Sort rules by priority (highest first)
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Add multiple resolution rules
    pub fn add_rules(&mut self, rules: impl IntoIterator<Item = ModelResolutionRule>) {
        self.rules.extend(rules);
        // Sort rules by priority (highest first)
        self.rules.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Add a model alias
    pub fn add_alias(
        &mut self,
        alias: impl Into<String>,
        provider: impl Into<String>,
        model: impl Into<String>,
    ) {
        self.aliases
            .insert(alias.into(), (provider.into(), model.into()));
    }

    /// Set the default provider for model resolution
    /// 
    /// This provider will be used when no provider is explicitly specified
    /// and the `CANDLE_DEFAULT_PROVIDER` environment variable is not set.
    /// 
    /// # Example
    /// ```rust
    /// let mut resolver = ModelResolver::new();
    /// resolver.with_default_provider("candle-kimi");
    /// ```
    pub fn with_default_provider(&mut self, provider: &'static str) -> &mut Self {
        self.default_provider = Some(provider);
        self
    }

    /// Resolve a model by name and optional provider
    pub fn resolve(
        &self,
        model_name: &str,
        provider: Option<&str>,
    ) -> AsyncStream<ModelResolution> {
        let resolver = self.clone();
        let model_name = model_name.to_string();
        let provider = provider.map(str::to_string);

        AsyncStream::with_channel(move |sender| {
            if let Ok(resolution) = resolver.resolve_internal(&model_name, provider.as_deref()) {
                let _ = sender.try_send(resolution);
            } else {
                // Provide a fallback resolution
                let fallback =
                    ModelResolution::new("fallback", model_name.clone(), None, None, 0.0);
                let _ = sender.try_send(fallback);
            }
        })
    }

    /// Internal resolve implementation using new registry
    ///
    /// # Errors
    ///
    /// Returns error if no matching model is found in the registry
    fn resolve_internal(
        &self,
        model_name: &str,
        provider: Option<&str>,
    ) -> Result<ModelResolution> {
        // Check for exact match first (provider:model)
        if let Some(provider) = provider {
            if let Some(model) = registry::get_by_provider_and_name(provider, model_name) {
                return Ok(ModelResolution::new(
                    provider.to_string(),
                    model_name.to_string(),
                    Some(model.info().clone()),
                    None,
                    1.0,
                ));
            }
        }

        // Check for exact match in the default provider
        if let Some(default_provider) = self.get_default_provider() {
            if let Some(model) = registry::get_by_provider_and_name(default_provider, model_name) {
                return Ok(ModelResolution::new(
                    default_provider.to_string(),
                    model_name.to_string(),
                    Some(model.info().clone()),
                    None,
                    0.9,
                ));
            }
        }

        // Check for aliases
        if let Some(alias_entry) = self.aliases.get(model_name) {
            let (provider, model_name_alias) = alias_entry;
            if let Some(model) = registry::get_by_provider_and_name(provider, model_name_alias) {
                return Ok(ModelResolution::new(
                    provider.clone(),
                    model_name_alias.clone(),
                    Some(model.info().clone()),
                    None,
                    0.8,
                ));
            }
        }

        // Apply resolution rules
        for rule in &self.rules {
            if rule.pattern.matches(model_name) {
                if let Some(model) = registry::get_by_provider_and_name(&rule.provider, &rule.target) {
                    if let Some(condition) = &rule.condition {
                        if !self.check_condition(condition, model.info()) {
                            continue;
                        }
                    }

                    return Ok(ModelResolution::new(
                        rule.provider.clone(),
                        rule.target.clone(),
                        Some(model.info().clone()),
                        Some(rule.clone()),
                        0.7,
                    ));
                }
            }
        }

        // Try fuzzy matching
        self.fuzzy_match(model_name, provider)
    }

    /// Get a model by name and optional provider
    pub fn get_model(
        &self,
        model_name: &str,
        provider: Option<&str>,
    ) -> AsyncStream<ModelResult> {
        let resolver = self.clone();
        let model_name = model_name.to_string();
        let provider = provider.map(str::to_string);

        AsyncStream::with_channel(move |sender| {
            let resolution_stream = resolver.resolve(&model_name, provider.as_deref());

            // Use proper streams-only pattern with collect() for blocking collection
            let resolutions = resolution_stream.collect();
            if let Some(resolution) = resolutions.into_iter().next() {
                if resolution.is_valid() {
                    if let Some(model) = registry::get_by_provider_and_name(&resolution.provider, &resolution.model) {
                        let _ = sender.send(ModelResult::found(model));
                    } else {
                        let _ = sender.send(ModelResult::not_found());
                    }
                } else {
                    let _ = sender.send(ModelResult::not_found());
                }
            } else {
                let _ = sender.send(ModelResult::not_found());
            }
        })
    }

    /// Get the provider with the most registered models
    /// 
    /// This is used as a fallback when no explicit default is configured.
    /// Returns None if no providers are registered.
    fn get_most_used_provider(&self) -> Option<&'static str> {
        let provider_counts = registry::count_models_by_provider();
        
        provider_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(provider, _)| provider)
    }

    /// Get the default provider using priority-based selection
    /// 
    /// Priority order:
    /// 1. Environment variable `CANDLE_DEFAULT_PROVIDER` (highest)
    /// 2. Explicitly configured provider via `with_default_provider()`
    /// 3. Provider with most registered models (fallback)
    /// 4. None if no providers available
    /// 
    /// # Returns
    /// 
    /// The default provider name if one can be determined, or None
    #[must_use]
    pub fn get_default_provider(&self) -> Option<&'static str> {
        // Priority 1: Environment variable (highest priority)
        if let Some(provider) = *ENV_DEFAULT_PROVIDER {
            return Some(provider);
        }
        
        // Priority 2: Explicit configuration
        if let Some(provider) = self.default_provider {
            return Some(provider);
        }
        
        // Priority 3: Most-used provider (fallback)
        self.get_most_used_provider()
    }

    /// Enable a feature flag
    pub fn enable_feature(&mut self, name: impl Into<String>) {
        self.feature_flags.insert(name.into(), true);
    }
    
    /// Disable a feature flag
    pub fn disable_feature(&mut self, name: impl Into<String>) {
        self.feature_flags.insert(name.into(), false);
    }
    
    /// Check if a feature flag is enabled
    #[must_use]
    pub fn is_feature_enabled(&self, name: &str) -> bool {
        self.feature_flags.get(name).copied().unwrap_or(false)
    }

    /// Check if a rule condition is satisfied
    fn check_condition(&self, condition: &RuleCondition, model_info: &ModelInfo) -> bool {
        match condition {
            RuleCondition::HasCapability { capability } => {
                // Parse capability string and check if model has it
                use crate::domain::model::capabilities::CandleCapability;
                
                if let Some(cap) = CandleCapability::from_string(capability) {
                    let capabilities = model_info.to_capabilities();
                    capabilities.has_capability(cap)
                } else {
                    // Unknown capability string - treat as not supported
                    false
                }
            }
            
            RuleCondition::HasFeature { feature } => {
                self.is_feature_enabled(feature)
            }
            
            RuleCondition::FeatureEnabled { name } => {
                // Check global feature flags
                self.is_feature_enabled(name)
            }
            
            RuleCondition::EnvVarSet { name } => {
                // Check environment variable
                std::env::var(name).is_ok()
            }
        }
    }

    /// Find the best matching model using fuzzy matching
    fn fuzzy_match(
        &self,
        model_name: &str,
        provider: Option<&str>,
    ) -> Result<ModelResolution> {
        // Get all registry keys and fetch models
        let all_keys = registry::all_registry_keys();
        
        // Find the best match using Jaro-Winkler similarity
        let best_match = all_keys
            .iter()
            .filter_map(|key| registry::get_model(key))
            .filter(|m| provider.is_none_or(|p| m.provider() == p))
            .map(|m| {
                let info = m.info();
                (info, strsim::jaro_winkler(info.name(), model_name))
            })
            .filter(|(_, score)| *score > 0.7)
            .max_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((info, score)) = best_match {
            Ok(ModelResolution::new(
                info.provider().to_string(),
                info.name().to_string(),
                Some(info.clone()),
                None,
                score,
            ))
        } else {
            Err(ModelError::ModelNotFound {
                provider: provider.unwrap_or("any").to_string().into(),
                name: model_name.to_string().into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU32;
    use crate::domain::model::{CandleModel, CandleModelInfo};

    #[test]
    fn test_pattern_matching() {
        // Test exact matching
        let exact = ModelPattern::Exact("kimi-k2".to_string());
        assert!(exact.matches("kimi-k2"));
        assert!(!exact.matches("qwen3-coder"));

        // Test glob pattern matching
        let glob = ModelPattern::Pattern("kimi-*".to_string());
        assert!(glob.matches("kimi-k2"));
        assert!(glob.matches("kimi-k2-instruct"));
        assert!(!glob.matches("qwen3-coder"));

        // Test regex pattern matching
        let regex = ModelPattern::Regex(r"^kimi-k\d+$".to_string());
        assert!(regex.matches("kimi-k2"));
        assert!(regex.matches("kimi-k1"));
        assert!(!regex.matches("kimi-k2-instruct"));
        assert!(!regex.matches("qwen3-coder"));
    }

    // Test model infrastructure
    #[derive(Debug, Clone)]
    struct TestModel {
        info: &'static CandleModelInfo,
    }

    impl CandleModel for TestModel {
        fn info(&self) -> &'static CandleModelInfo {
            self.info
        }
    }

    static TEST_MODEL_A_INFO: CandleModelInfo = CandleModelInfo {
        provider_name: "test-provider",
        name: "test-model-a",
        max_input_tokens: NonZeroU32::new(4096),
        max_output_tokens: NonZeroU32::new(2048),
        input_price: None,
        output_price: None,
        supports_vision: false,
        supports_function_calling: false,
        supports_streaming: true,
        supports_embeddings: false,
        requires_max_tokens: false,
        supports_thinking: false,
        optimal_thinking_budget: None,
        system_prompt_prefix: None,
        real_name: None,
        model_type: None,
        model_id: "test-a",
        hf_repo_url: "test/model-a",
        quantization: "Q4_0",
        patch: None,
    };

    static TEST_MODEL_B_INFO: CandleModelInfo = CandleModelInfo {
        provider_name: "test-provider",
        name: "test-model-b",
        max_input_tokens: NonZeroU32::new(8192),
        max_output_tokens: NonZeroU32::new(4096),
        input_price: None,
        output_price: None,
        supports_vision: false,
        supports_function_calling: true,
        supports_streaming: true,
        supports_embeddings: false,
        requires_max_tokens: false,
        supports_thinking: false,
        optimal_thinking_budget: None,
        system_prompt_prefix: None,
        real_name: None,
        model_type: None,
        model_id: "test-b",
        hf_repo_url: "test/model-b",
        quantization: "Q5_0",
        patch: None,
    };

    fn create_test_model_a() -> TestModel {
        TestModel {
            info: &TEST_MODEL_A_INFO,
        }
    }

    fn create_test_model_b() -> TestModel {
        TestModel {
            info: &TEST_MODEL_B_INFO,
        }
    }

    #[test]
    #[ignore = "Needs rewrite for static registry"]
    fn test_exact_model_match() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // TODO: Rewrite using actual registered models from static registry
        Ok(())
    }

    #[test]
    #[ignore = "Needs rewrite for static registry"]
    fn test_alias_resolution() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // TODO: Rewrite using actual registered models from static registry
        Ok(())
    }

    #[test]
    fn test_rule_priority_ordering() {
        let mut resolver = ModelResolver::new();

        let rule1 = ModelResolutionRule {
            pattern: ModelPattern::Exact("test-pattern".to_string()),
            provider: "provider-a".to_string(),
            target: "model-a".to_string(),
            priority: 5,
            condition: None,
        };

        let rule2 = ModelResolutionRule {
            pattern: ModelPattern::Exact("test-pattern".to_string()),
            provider: "provider-b".to_string(),
            target: "model-b".to_string(),
            priority: 10,
            condition: None,
        };

        let rule3 = ModelResolutionRule {
            pattern: ModelPattern::Exact("test-pattern".to_string()),
            provider: "provider-c".to_string(),
            target: "model-c".to_string(),
            priority: 1,
            condition: None,
        };

        resolver.add_rules(vec![rule1, rule2, rule3]);

        // Verify rules are sorted by priority (highest first)
        assert_eq!(resolver.rules[0].priority, 10);
        assert_eq!(resolver.rules[0].provider, "provider-b");
        assert_eq!(resolver.rules[1].priority, 5);
        assert_eq!(resolver.rules[1].provider, "provider-a");
        assert_eq!(resolver.rules[2].priority, 1);
        assert_eq!(resolver.rules[2].provider, "provider-c");
    }

    #[test]
    #[ignore = "Needs rewrite for static registry"]
    fn test_fuzzy_matching() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // TODO: Rewrite using actual registered models from static registry
        Ok(())
    }

    #[test]
    fn test_feature_flags() {
        let mut resolver = ModelResolver::new();

        // Test initial state
        assert!(!resolver.is_feature_enabled("test-feature"));

        // Enable feature
        resolver.enable_feature("test-feature");
        assert!(resolver.is_feature_enabled("test-feature"));

        // Disable feature
        resolver.disable_feature("test-feature");
        assert!(!resolver.is_feature_enabled("test-feature"));

        // Test multiple features
        resolver.enable_feature("feature-1");
        resolver.enable_feature("feature-2");
        assert!(resolver.is_feature_enabled("feature-1"));
        assert!(resolver.is_feature_enabled("feature-2"));
        assert!(!resolver.is_feature_enabled("feature-3"));
    }

    #[test]
    fn test_env_var_condition() {
        // Test EnvVarSet condition evaluation
        // Note: This test uses a likely-unset env var to test the false case
        // and PATH (which should always exist) for the true case
        let resolver = ModelResolver::new();
        let test_info = &TEST_MODEL_A_INFO;

        // Test with unlikely env var (should not be set)
        let unset_condition = RuleCondition::EnvVarSet {
            name: "PARAPHYM_TEST_NONEXISTENT_VAR_XYZ123".to_string(),
        };
        assert!(!resolver.check_condition(&unset_condition, test_info));

        // Test with PATH which should exist
        let set_condition = RuleCondition::EnvVarSet {
            name: "PATH".to_string(),
        };
        assert!(resolver.check_condition(&set_condition, test_info));
    }

    #[test]
    #[ignore = "Needs rewrite for static registry"]
    fn test_rule_with_condition() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // TODO: Rewrite using actual registered models from static registry
        Ok(())
    }
}
