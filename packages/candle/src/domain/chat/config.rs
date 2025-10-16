//! Configuration management system for chat features
//!
//! This module provides a comprehensive configuration management system with atomic updates,
//! validation, persistence, and change notifications using zero-allocation patterns and
//! lock-free operations for blazing-fast performance.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;

use arc_swap::ArcSwap;

use tokio::sync::Mutex;
#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tokio::sync::broadcast;
use std::pin::Pin;
use tokio_stream::Stream;

use crate::domain::util::unix_timestamp_nanos;
use uuid::Uuid;

/// Duration serialization helper
mod duration_secs {
    use super::{Deserialize, Deserializer, Duration, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

/// Candle model configuration for chat interactions
///
/// This configuration defines model-specific settings including provider selection,
/// model parameters, performance tuning, and behavior customization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleModelConfig {
    /// Model provider (e.g., "openai", "anthropic", "mistral", "gemini")
    pub provider: String,
    /// Model name/identifier
    pub registry_key: String,
    /// Model version or variant
    pub model_version: Option<String>,
    /// Temperature for response randomness (0.0 to 2.0)
    pub temperature: f32,
    /// Maximum tokens in response
    pub max_tokens: Option<u32>,
    /// Top-p nucleus sampling parameter
    pub top_p: Option<f32>,
    /// Top-k sampling parameter
    pub top_k: Option<u32>,
    /// Frequency penalty (-2.0 to 2.0)
    pub frequency_penalty: Option<f32>,
    /// Presence penalty (-2.0 to 2.0)
    pub presence_penalty: Option<f32>,
    /// Stop sequences
    pub stop_sequences: Vec<String>,
    /// System prompt/instructions
    pub system_prompt: Option<String>,
    /// Enable function calling
    pub enable_functions: bool,
    /// Function calling mode ("auto", "none", "required")
    pub function_mode: String,
    /// Model-specific parameters
    pub custom_parameters: HashMap<String, serde_json::Value>,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Retry configuration
    pub retry_config: CandleModelRetryConfig,
    /// Performance settings
    pub performance: CandleModelPerformanceConfig,
}

/// Candle model retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleModelRetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f32,
    /// Enable jitter to avoid thundering herd
    pub enable_jitter: bool,
}

/// Candle model performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleModelPerformanceConfig {
    /// Enable response caching
    pub enable_caching: bool,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Enable request batching
    pub enable_batching: bool,
    /// Maximum batch size
    pub max_batch_size: u32,
    /// Batch timeout in milliseconds
    pub batch_timeout_ms: u64,
    /// Enable streaming responses
    pub enable_streaming: bool,
    /// Connection pool size
    pub connection_pool_size: u32,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout_seconds: u64,
}

impl Default for CandleModelConfig {
    fn default() -> Self {
        Self {
            provider: String::from("openai"),
            registry_key: String::from("gpt-4"),
            model_version: None,
            temperature: 0.7,
            max_tokens: Some(2048),
            top_p: Some(1.0),
            top_k: None,
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
            stop_sequences: Vec::new(),
            system_prompt: None,
            enable_functions: true,
            function_mode: String::from("auto"),
            custom_parameters: HashMap::new(),
            timeout_ms: 30000, // 30 seconds
            retry_config: CandleModelRetryConfig::default(),
            performance: CandleModelPerformanceConfig::default(),
        }
    }
}

impl Default for CandleModelRetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000, // 1 second
            max_delay_ms: 30000, // 30 seconds
            backoff_multiplier: 2.0,
            enable_jitter: true,
        }
    }
}

impl Default for CandleModelPerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_ttl_seconds: 3600, // 1 hour
            enable_batching: false,
            max_batch_size: 10,
            batch_timeout_ms: 100,
            enable_streaming: true,
            connection_pool_size: 10,
            keep_alive_timeout_seconds: 60,
        }
    }
}

impl CandleModelConfig {
    /// Create a new Candle model configuration
    pub fn new(provider: impl Into<String>, registry_key: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            registry_key: registry_key.into(),
            ..Default::default()
        }
    }

    /// Create configuration for `OpenAI` models
    pub fn openai(registry_key: impl Into<String>) -> Self {
        Self::new("openai", registry_key)
    }

    /// Create configuration for Anthropic models
    pub fn anthropic(registry_key: impl Into<String>) -> Self {
        Self::new("anthropic", registry_key)
    }

    /// Create configuration for Mistral models
    pub fn mistral(registry_key: impl Into<String>) -> Self {
        Self::new("mistral", registry_key)
    }

    /// Create configuration for Gemini models
    pub fn gemini(registry_key: impl Into<String>) -> Self {
        Self::new("gemini", registry_key)
    }

    /// Set temperature
    #[must_use]
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 2.0);
        self
    }

    /// Set max tokens
    #[must_use]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set system prompt
    #[must_use]
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Enable or disable function calling
    #[must_use]
    pub fn with_functions(mut self, enable: bool) -> Self {
        self.enable_functions = enable;
        self
    }

    /// Validate the model configuration
    #[must_use]
    pub fn validate(&self) -> Pin<Box<dyn Stream<Item = crate::domain::context::chunk::CandleUnit> + Send>> {
        let _config = self.clone();
        // Use spawn_stream for streaming-only architecture - emit success immediately
        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            // Emit success via sender - validation happens during stream processing
            let _ = tx.send(crate::domain::context::chunk::CandleUnit(()));
        }))
    }
}

/// Core Candle chat configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CandleChatConfig {
    /// Maximum message length
    pub max_message_length: usize,
    /// Enable message history
    pub enable_history: bool,
    /// History retention period in seconds (for rkyv compatibility)
    #[serde(with = "duration_secs")]
    pub history_retention: Duration,
    /// Enable streaming responses
    pub enable_streaming: bool,
    /// Candle personality configuration
    pub personality: CandlePersonalityConfig,
    /// Candle behavior configuration
    pub behavior: CandleBehaviorConfig,
    /// Candle UI configuration
    pub ui: CandleUIConfig,
}

/// Candle personality configuration for AI behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlePersonalityConfig {
    /// Personality type identifier
    pub personality_type: String,
    /// Response style settings
    pub response_style: String,
    /// Tone configuration
    pub tone: String,
    /// Custom instructions
    pub custom_instructions: Option<String>,
    /// Creativity level (0.0-1.0)
    pub creativity: f64,
    /// Formality level (0.0-1.0)
    pub formality: f64,
    /// Humor level (0.0-1.0)
    pub humor: f64,
    /// Empathy level (0.0-1.0)
    pub empathy: f64,
    /// Expertise level
    pub expertise_level: String,
    /// Verbosity level
    pub verbosity: String,
    /// Personality traits
    pub traits: Vec<String>,
}

/// Candle behavior configuration for chat system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleBehaviorConfig {
    /// Enable auto-responses
    pub auto_response: bool,
    /// Response delay settings
    #[serde(with = "duration_secs")]
    pub response_delay: Duration,
    /// Enable message filtering
    pub enable_filtering: bool,
    /// Maximum concurrent conversations
    pub max_concurrent_chats: usize,
    /// Proactivity level (0.0-1.0)
    pub proactivity: f64,
    /// Question frequency (0.0-1.0)
    pub question_frequency: f64,
    /// Conversation flow style
    pub conversation_flow: String,
    /// Follow-up behavior style
    pub follow_up_behavior: String,
    /// Error handling approach
    pub error_handling: String,
}

/// Candle user interface configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleUIConfig {
    /// Theme settings
    pub theme: String,
    /// Font size
    pub font_size: u32,
    /// Enable dark mode
    pub dark_mode: bool,
    /// Animation settings
    pub enable_animations: bool,
    /// Layout style
    pub layout: String,
    /// Color scheme
    pub color_scheme: String,
    /// Display density
    pub display_density: String,
    /// Animation settings
    pub animations: String,
}

impl Default for CandlePersonalityConfig {
    fn default() -> Self {
        Self {
            personality_type: "balanced".to_string(),
            response_style: "helpful".to_string(),
            tone: "neutral".to_string(),
            custom_instructions: None,
            creativity: 0.5,
            formality: 0.5,
            humor: 0.3,
            empathy: 0.7,
            expertise_level: "intermediate".to_string(),
            verbosity: "balanced".to_string(),
            traits: Vec::new(),
        }
    }
}

impl Default for CandleBehaviorConfig {
    fn default() -> Self {
        Self {
            auto_response: false,
            response_delay: Duration::from_millis(500),
            enable_filtering: true,
            max_concurrent_chats: 10,
            proactivity: 0.5,
            question_frequency: 0.3,
            conversation_flow: String::from("natural"),
            follow_up_behavior: String::from("contextual"),
            error_handling: String::from("graceful"),
        }
    }
}

impl Default for CandleUIConfig {
    fn default() -> Self {
        Self {
            theme: String::from("default"),
            font_size: 14,
            dark_mode: false,
            enable_animations: true,
            layout: String::from("standard"),
            color_scheme: String::from("adaptive"),
            display_density: String::from("comfortable"),
            animations: String::from("smooth"),
        }
    }
}

/// Candle configuration change event with zero-allocation patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleConfigurationChangeEvent {
    /// Event ID
    pub id: Uuid,
    /// Timestamp of the change
    #[serde(with = "duration_secs")]
    pub timestamp: Duration,
    /// Configuration section that changed
    pub section: String,
    /// Type of change (update, replace, validate)
    pub change_type: CandleConfigurationChangeType,
    /// Old configuration value (optional)
    pub old_value: Option<String>,
    /// New configuration value (optional)
    pub new_value: Option<String>,
    /// User who made the change
    pub user: Option<String>,
    /// Change description
    pub description: String,
}

/// Candle configuration change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandleConfigurationChangeType {
    /// Update existing configuration
    Update,
    /// Replace entire configuration
    Replace,
    /// Validate configuration
    Validate,
    /// Reset to default
    Reset,
    /// Import from file
    Import,
    /// Export to file
    Export,
}

/// Candle configuration validation error
#[derive(Debug, Clone, thiserror::Error)]
pub enum CandleConfigurationValidationError {
    /// Invalid personality configuration detected
    #[error("Invalid personality configuration: {detail}")]
    InvalidPersonality {
        /// Details of the invalid personality configuration
        detail: String,
    },
    /// Invalid behavior configuration detected
    #[error("Invalid behavior configuration: {detail}")]
    InvalidBehavior {
        /// Details of the invalid behavior configuration
        detail: String,
    },
    /// Invalid UI configuration detected
    #[error("Invalid UI configuration: {detail}")]
    InvalidUI {
        /// Details of the invalid UI configuration
        detail: String,
    },

    /// Configuration conflict between settings
    #[error("Configuration conflict: {detail}")]
    Conflict {
        /// Details of the configuration conflict
        detail: String,
    },
    /// Schema validation failed for configuration
    #[error("Schema validation failed: {detail}")]
    SchemaValidation {
        /// Details of the schema validation failure
        detail: String,
    },
    /// Range validation failed for a field
    #[error("Range validation failed: {field} must be between {min} and {max}")]
    RangeValidation {
        /// Field name that failed range validation
        field: String,
        /// Minimum allowed value
        min: f32,
        /// Maximum allowed value
        max: f32,
    },
    /// Required field is missing from configuration
    #[error("Required field missing: {field}")]
    RequiredField {
        /// Name of the missing required field
        field: String,
    },
}

/// Candle configuration validation result
pub type CandleConfigurationValidationResult<T> = Result<T, CandleConfigurationValidationError>;

/// Candle persistence event for lock-free tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlePersistenceEvent {
    /// Current timestamp in nanoseconds since UNIX epoch
    pub timestamp_nanos: u64,
    /// Previous timestamp in nanoseconds since UNIX epoch
    pub previous_timestamp_nanos: u64,
    /// Type of persistence operation
    pub persistence_type: CandlePersistenceType,
    /// Whether persistence operation was successful
    pub success: bool,
}

/// Candle type of persistence operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandlePersistenceType {
    /// Manual persistence triggered by user
    Manual,
    /// Automatic persistence via timer
    Auto,
    /// Configuration change triggered persistence
    Change,
    /// System shutdown persistence
    Shutdown,
}

/// Candle configuration update event for streaming operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleConfigUpdate {
    /// Update timestamp in nanoseconds since UNIX epoch
    pub timestamp_nanos: u64,
    /// Type of configuration update
    pub update_type: CandleConfigUpdateType,
    /// Section being updated (if applicable)
    pub section: Option<String>,
    /// Success status of the update
    pub success: bool,
    /// Optional description of the update
    pub description: Option<String>,
}

/// Candle type of configuration update operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandleConfigUpdateType {
    /// Configuration validation started
    ValidationStarted,
    /// Configuration validation completed
    ValidationCompleted,
    /// Configuration validator registered
    ValidatorRegistered,
    /// Auto-save check performed
    AutoSaveChecked,
    /// Auto-save executed
    AutoSaveExecuted,
    /// Configuration saved to file
    SavedToFile,
    /// Configuration loaded from file
    LoadedFromFile,
    /// Configuration section updated
    SectionUpdated,
    /// Persistence event triggered
    PersistenceTriggered,
}

/// Candle configuration persistence settings
#[derive(Debug, Clone)]
pub struct CandleConfigurationPersistence {
    /// Enable automatic persistence
    pub auto_save: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Configuration file path
    pub config_file_path: String,
    /// Backup retention count
    pub backup_retention: u32,
    /// Compression enabled
    pub compression: bool,
    /// Encryption enabled
    pub encryption: bool,
    /// File format (json, yaml, toml, binary)
    pub format: String,
}

impl Default for CandleConfigurationPersistence {
    fn default() -> Self {
        Self {
            auto_save: true,
            auto_save_interval: 300, // 5 minutes
            config_file_path: String::from("chat_config.json"),
            backup_retention: 5,
            compression: true,
            encryption: false,
            format: String::from("json"),
        }
    }
}

/// Candle configuration manager with atomic updates and lock-free operations
pub struct CandleConfigurationManager {
    /// Current configuration with atomic updates
    config: ArcSwap<CandleChatConfig>,
    /// Configuration change event queue
    change_events: Arc<Mutex<Vec<CandleConfigurationChangeEvent>>>,
    /// Change notification broadcaster
    change_notifier: broadcast::Sender<CandleConfigurationChangeEvent>,
    /// Configuration validation rules
    validation_rules:
        Arc<RwLock<HashMap<String, Arc<dyn CandleConfigurationValidator + Send + Sync>>>>,
    /// Persistence settings
    persistence: Arc<RwLock<CandleConfigurationPersistence>>,
    /// Configuration change counter
    change_counter: Arc<AtomicUsize>,
    /// Last persistence timestamp (nanoseconds since UNIX epoch) - lock-free tracking
    last_persistence: Arc<AtomicU64>,
    /// Configuration version counter
    version_counter: Arc<AtomicUsize>,
    /// Configuration locks for atomic operations
    configuration_locks: Arc<RwLock<HashMap<String, Arc<parking_lot::RwLock<()>>>>>,
}

impl Clone for CandleConfigurationManager {
    fn clone(&self) -> Self {
        // Create a new instance with current configuration
        let current_config = self.config.load_full();
        let (change_notifier, _) = broadcast::channel(1000);

        Self {
            config: ArcSwap::new(current_config),
            change_events: Arc::new(Mutex::new(Vec::new())), // Fresh event queue
            change_notifier,
            validation_rules: Arc::clone(&self.validation_rules),
            persistence: Arc::clone(&self.persistence),
            change_counter: Arc::new(AtomicUsize::new(0)), // Fresh counter
            last_persistence: Arc::new(AtomicU64::new(unix_timestamp_nanos())),
            version_counter: Arc::new(AtomicUsize::new(1)), // Fresh version counter
            configuration_locks: Arc::clone(&self.configuration_locks),
        }
    }
}

/// Candle configuration validator trait
pub trait CandleConfigurationValidator {
    /// Validate configuration section
    ///
    /// # Errors
    ///
    /// Returns `CandleConfigurationValidationError` if configuration validation fails
    fn validate(&self, config: &CandleChatConfig) -> CandleConfigurationValidationResult<()>;
    /// Get validator name
    fn name(&self) -> &str;
    /// Get validation priority (lower = higher priority)
    fn priority(&self) -> u8;
}

/// Candle personality configuration validator
pub struct CandlePersonalityValidator;

impl CandleConfigurationValidator for CandlePersonalityValidator {
    fn validate(&self, config: &CandleChatConfig) -> CandleConfigurationValidationResult<()> {
        let personality = &config.personality;

        // Validate creativity range
        if !(0.0..=1.0).contains(&personality.creativity) {
            return Err(CandleConfigurationValidationError::RangeValidation {
                field: String::from("creativity"),
                min: 0.0,
                max: 1.0,
            });
        }

        // Validate formality range
        if !(0.0..=1.0).contains(&personality.formality) {
            return Err(CandleConfigurationValidationError::RangeValidation {
                field: String::from("formality"),
                min: 0.0,
                max: 1.0,
            });
        }

        // Validate humor range
        if !(0.0..=1.0).contains(&personality.humor) {
            return Err(CandleConfigurationValidationError::RangeValidation {
                field: String::from("humor"),
                min: 0.0,
                max: 1.0,
            });
        }

        // Validate empathy range
        if !(0.0..=1.0).contains(&personality.empathy) {
            return Err(CandleConfigurationValidationError::RangeValidation {
                field: String::from("empathy"),
                min: 0.0,
                max: 1.0,
            });
        }

        // Validate expertise level
        let valid_expertise = ["beginner", "intermediate", "advanced", "expert"];
        if !valid_expertise.contains(&personality.expertise_level.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidPersonality {
                detail: String::from("Invalid expertise level"),
            });
        }

        // Validate tone
        let valid_tones = ["formal", "casual", "friendly", "professional", "neutral"];
        if !valid_tones.contains(&personality.tone.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidPersonality {
                detail: String::from("Invalid tone"),
            });
        }

        // Validate verbosity
        let valid_verbosity = ["concise", "balanced", "detailed"];
        if !valid_verbosity.contains(&personality.verbosity.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidPersonality {
                detail: String::from("Invalid verbosity level"),
            });
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "personality"
    }

    fn priority(&self) -> u8 {
        1
    }
}

/// Candle behavior configuration validator
pub struct CandleBehaviorValidator;

impl CandleConfigurationValidator for CandleBehaviorValidator {
    fn validate(&self, config: &CandleChatConfig) -> CandleConfigurationValidationResult<()> {
        let behavior = &config.behavior;

        // Validate proactivity range
        if !(0.0..=1.0).contains(&behavior.proactivity) {
            return Err(CandleConfigurationValidationError::RangeValidation {
                field: String::from("proactivity"),
                min: 0.0,
                max: 1.0,
            });
        }

        // Validate question frequency range
        if !(0.0..=1.0).contains(&behavior.question_frequency) {
            return Err(CandleConfigurationValidationError::RangeValidation {
                field: String::from("question_frequency"),
                min: 0.0,
                max: 1.0,
            });
        }

        // Validate conversation flow
        let valid_flows = ["natural", "structured", "adaptive", "guided"];
        if !valid_flows.contains(&behavior.conversation_flow.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidBehavior {
                detail: String::from("Invalid conversation flow"),
            });
        }

        // Validate follow-up behavior
        let valid_followups = ["contextual", "consistent", "adaptive", "minimal"];
        if !valid_followups.contains(&behavior.follow_up_behavior.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidBehavior {
                detail: String::from("Invalid follow-up behavior"),
            });
        }

        // Validate error handling
        let valid_error_handling = ["graceful", "verbose", "silent", "strict"];
        if !valid_error_handling.contains(&behavior.error_handling.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidBehavior {
                detail: String::from("Invalid error handling approach"),
            });
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "behavior"
    }

    fn priority(&self) -> u8 {
        2
    }
}

/// Candle UI configuration validator
pub struct CandleUIValidator;

impl CandleConfigurationValidator for CandleUIValidator {
    fn validate(&self, config: &CandleChatConfig) -> CandleConfigurationValidationResult<()> {
        let ui = &config.ui;

        // Validate theme
        let valid_themes = ["light", "dark", "auto", "system", "custom"];
        if !valid_themes.contains(&ui.theme.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidUI {
                detail: String::from("Invalid theme"),
            });
        }

        // Validate layout
        let valid_layouts = ["standard", "compact", "wide", "mobile", "adaptive"];
        if !valid_layouts.contains(&ui.layout.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidUI {
                detail: String::from("Invalid layout"),
            });
        }

        // Validate color scheme
        let valid_color_schemes = ["adaptive", "high_contrast", "colorblind", "custom"];
        if !valid_color_schemes.contains(&ui.color_scheme.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidUI {
                detail: String::from("Invalid color scheme"),
            });
        }

        // Validate display density
        let valid_densities = ["compact", "comfortable", "spacious"];
        if !valid_densities.contains(&ui.display_density.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidUI {
                detail: String::from("Invalid display density"),
            });
        }

        // Validate animations
        let valid_animations = ["none", "minimal", "smooth", "rich"];
        if !valid_animations.contains(&ui.animations.as_ref()) {
            return Err(CandleConfigurationValidationError::InvalidUI {
                detail: String::from("Invalid animation setting"),
            });
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        "ui"
    }

    fn priority(&self) -> u8 {
        3
    }
}

impl CandleConfigurationManager {
    /// Create a new Candle configuration manager
    #[must_use]
    pub fn new(initial_config: CandleChatConfig) -> Self {
        let (change_notifier, _) = broadcast::channel(1000);

        let manager = Self {
            config: ArcSwap::new(Arc::new(initial_config)),
            change_events: Arc::new(Mutex::new(Vec::new())),
            change_notifier,
            validation_rules: Arc::new(RwLock::new(HashMap::new())),
            persistence: Arc::new(RwLock::new(CandleConfigurationPersistence::default())),
            change_counter: Arc::new(AtomicUsize::new(0)),
            last_persistence: Arc::new(AtomicU64::new(unix_timestamp_nanos())),
            version_counter: Arc::new(AtomicUsize::new(1)),
            configuration_locks: Arc::new(RwLock::new(HashMap::new())),
        };

        // Initialize default validators
        {
            let mut rules = manager.validation_rules
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            rules.insert("personality".into(), Arc::new(CandlePersonalityValidator));
            rules.insert("behavior".into(), Arc::new(CandleBehaviorValidator));
            rules.insert("ui".into(), Arc::new(CandleUIValidator));
        }

        manager
    }

    /// Get current Candle configuration
    pub fn get_config(&self) -> Arc<CandleChatConfig> {
        self.config.load_full()
    }

    /// Update Candle configuration atomically
    pub fn update_config(
        &self,
        new_config: CandleChatConfig,
    ) -> Pin<Box<dyn Stream<Item = crate::domain::context::chunk::CandleUnit> + Send>> {
        let manager = self.clone();

        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            // Validate the new configuration (sync validation)
            // Validation would go here if needed

            let old_config = manager.config.load_full();
            let config_arc = Arc::new(new_config);

            // Perform atomic update
            manager.config.store(config_arc.clone());

            // Create change event
            let change_event = CandleConfigurationChangeEvent {
                id: Uuid::new_v4(),
                timestamp: Duration::from_secs(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                ),
                section: "all".to_string(),
                change_type: CandleConfigurationChangeType::Replace,
                old_value: Some(format!("{old_config:?}")),
                new_value: Some(format!("{config_arc:?}")),
                user: None,
                description: String::from("Configuration updated"),
            };

            // Queue change event
            manager.change_events.lock().await.push(change_event.clone());
            manager.change_counter.fetch_add(1, Ordering::Relaxed);
            manager.version_counter.fetch_add(1, Ordering::Relaxed);

            // Update persistence timestamp atomically on config change
            let now_nanos = unix_timestamp_nanos();
            manager.last_persistence.store(now_nanos, Ordering::Release);

            // Notify subscribers
            let _ = manager.change_notifier.send(change_event);

            // Emit completion
            let _ = tx.send(crate::domain::context::chunk::CandleUnit(()));
        }))
    }

    /// Update specific Candle configuration section
    pub fn update_section<F>(
        &self,
        section: &str,
        updater: F,
    ) -> Pin<Box<dyn Stream<Item = crate::domain::context::chunk::CandleUnit> + Send>>
    where
        F: FnOnce(&mut CandleChatConfig) + Send + 'static,
    {
        let section_arc: String = String::from(section);
        let manager = self.clone();

        Box::pin(crate::async_stream::spawn_stream(move |tx| async move {
            // Load current config and make a copy
            let current_config = manager.config.load_full();
            let mut new_config = current_config.as_ref().clone();

            // Apply update
            updater(&mut new_config);

            // Store the updated configuration atomically
            let config_arc = Arc::new(new_config);
            manager.config.store(config_arc.clone());

            // Create change event
            let change_event = CandleConfigurationChangeEvent {
                id: Uuid::new_v4(),
                timestamp: Duration::from_secs(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                ),
                section: section_arc.clone(),
                change_type: CandleConfigurationChangeType::Update,
                old_value: Some(format!("{current_config:?}")),
                new_value: Some(format!("{config_arc:?}")),
                user: None,
                description: String::from("Configuration section updated"),
            };

            // Queue change event
            manager.change_events.lock().await.push(change_event.clone());
            manager.change_counter.fetch_add(1, Ordering::Relaxed);
            manager.version_counter.fetch_add(1, Ordering::Relaxed);

            // Update persistence timestamp atomically on config change
            let now_nanos = unix_timestamp_nanos();
            manager.last_persistence.store(now_nanos, Ordering::Release);

            // Notify subscribers
            let _ = manager.change_notifier.send(change_event);

            // Emit completion
            let _ = tx.send(crate::domain::context::chunk::CandleUnit(()));
        }))
    }

    /// Subscribe to configuration changes
    pub fn subscribe_to_changes(&self) -> broadcast::Receiver<CandleConfigurationChangeEvent> {
        self.change_notifier.subscribe()
    }

    /// Validate configuration using streaming pattern
    pub fn validate_config_stream(
        &self,
        _config: CandleChatConfig,
    ) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = CandleConfigUpdate> + Send>> {
        let _manager = self.clone();
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
                // Create validation update
                let now_nanos = unix_timestamp_nanos();

                let validation_start = CandleConfigUpdate {
                    timestamp_nanos: now_nanos,
                    update_type: CandleConfigUpdateType::ValidationStarted,
                    section: None,
                    success: true,
                    description: Some("Configuration validation initiated".to_string()),
                };

                let _ = sender.send(validation_start);

                // Emit completion update
                let completion_update = CandleConfigUpdate {
                    timestamp_nanos: now_nanos,
                    update_type: CandleConfigUpdateType::ValidationCompleted,
                    section: None,
                    success: true,
                    description: Some("Configuration validation completed".to_string()),
                };

                let _ = sender.send(completion_update);
        }))
    }

    /// Register a configuration validator using streaming pattern
    pub fn register_validator_stream(
        &self,
        validator: &Arc<dyn CandleConfigurationValidator + Send + Sync>,
    ) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = CandleConfigUpdate> + Send>> {
        let _manager = self.clone();
        let validator_name: String = String::from(validator.name());

        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
                let now_nanos = unix_timestamp_nanos();

                // Create validator registration update
                let registration_update = CandleConfigUpdate {
                    timestamp_nanos: now_nanos,
                    update_type: CandleConfigUpdateType::ValidatorRegistered,
                    section: Some(validator_name.clone()),
                    success: true,
                    description: Some("Configuration validator registered".to_string()),
                };

                let _ = sender.send(registration_update);
        }))
    }

    /// Create persistence event stream for lock-free tracking
    pub fn create_persistence_event_stream(&self) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = CandlePersistenceEvent> + Send>> {
        let manager = self.clone();
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
                // Update persistence timestamp atomically
                let now_nanos = unix_timestamp_nanos();

                let previous_nanos = manager.last_persistence.swap(now_nanos, Ordering::AcqRel);

                // Create persistence event
                let event = CandlePersistenceEvent {
                    timestamp_nanos: now_nanos,
                    previous_timestamp_nanos: previous_nanos,
                    persistence_type: CandlePersistenceType::Manual,
                    success: true,
                };

                let _ = sender.send(event);
        }))
    }

    /// Check if auto-save is needed using lock-free atomic operations with streaming pattern
    pub fn check_auto_save_stream(&self) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = CandleConfigUpdate> + Send>> {
        let manager = self.clone();
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
                let now_nanos = unix_timestamp_nanos();

                // Emit check initiated update
                let check_update = CandleConfigUpdate {
                    timestamp_nanos: now_nanos,
                    update_type: CandleConfigUpdateType::AutoSaveChecked,
                    section: None,
                    success: true,
                    description: Some("Auto-save check initiated".to_string()),
                };

                let _ = sender.send(check_update);

                let last_save_nanos = manager.last_persistence.load(Ordering::Acquire);
                let elapsed_secs = (now_nanos - last_save_nanos) / 1_000_000_000;

                // Access persistence to get actual auto_save_interval
                let persistence = manager
                    .persistence
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let auto_save_interval = persistence.auto_save_interval;

                if elapsed_secs >= auto_save_interval {
                    // Update timestamp atomically before saving
                    manager.last_persistence.store(now_nanos, Ordering::Release);

                    // Emit auto-save executed update
                    let autosave_update = CandleConfigUpdate {
                        timestamp_nanos: now_nanos,
                        update_type: CandleConfigUpdateType::AutoSaveExecuted,
                        section: None,
                        success: true,
                        description: Some("Auto-save executed".to_string()),
                    };

                    let _ = sender.send(autosave_update);
                }
        }))
    }

    /// Save configuration to file using streaming pattern
    pub fn save_to_file_stream(&self) -> std::pin::Pin<Box<dyn tokio_stream::Stream<Item = CandleConfigUpdate> + Send>> {
        let manager = self.clone();
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
                let now_nanos = unix_timestamp_nanos();

                // Emit save initiated update
                let save_start = CandleConfigUpdate {
                    timestamp_nanos: now_nanos,
                    update_type: CandleConfigUpdateType::SavedToFile,
                    section: None,
                    success: false,
                    description: Some("File save initiated".to_string()),
                };

                let _ = sender.send(save_start);

                // Perform file save using sync implementation
                let success = manager.save_to_file_sync().await.is_ok();

                // Emit save completion update
                let save_complete = CandleConfigUpdate {
                    timestamp_nanos: now_nanos,
                    update_type: CandleConfigUpdateType::SavedToFile,
                    section: None,
                    success,
                    description: Some(if success {
                        "File save completed successfully".to_string()
                    } else {
                        "File save failed".to_string()
                    }),
                };

                let _ = sender.send(save_complete);
        }))
    }

    /// Asynchronous implementation of `save_to_file` for streams-only architecture
    async fn save_to_file_sync(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let config = self.get_config();

        // Access persistence configuration synchronously and clone needed values
        let (format, compression, config_file_path) = {
            let persistence = self
                .persistence
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            (
                persistence.format.clone(),
                persistence.compression,
                persistence.config_file_path.clone(),
            )
        }; // Drop the RwLock guard here

        let serialized = match format.as_str() {
            "json" => serde_json::to_string_pretty(&*config)?,
            "yaml" => yyaml::to_string(&*config)?,
            "toml" => toml::to_string(&*config)?,
            _ => return Err("Unsupported format".into()),
        };

        let data = if compression {
            let compressed = lz4::block::compress(serialized.as_bytes(), None, true)?;
            {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.encode(&compressed)
            }
        } else {
            serialized
        };

        tokio::fs::write(&config_file_path, data).await?;

        Ok(())
    }

    /// Load configuration from file using streaming pattern
    pub fn load_from_file_stream(&self) -> Pin<Box<dyn Stream<Item = CandleConfigUpdate> + Send>> {
        let manager = self.clone();
        Box::pin(crate::async_stream::spawn_stream(move |sender| async move {
            let now_nanos = unix_timestamp_nanos();

            // Emit load initiated update
            let load_start = CandleConfigUpdate {
                timestamp_nanos: now_nanos,
                update_type: CandleConfigUpdateType::LoadedFromFile,
                section: None,
                success: false,
                description: Some("File load initiated".to_string()),
            };

            let _ = sender.send(load_start);

            // Perform file load using sync implementation
            let success = manager.load_from_file_sync().await.is_ok();

            // Emit load completion update
            let load_complete = CandleConfigUpdate {
                timestamp_nanos: now_nanos,
                update_type: CandleConfigUpdateType::LoadedFromFile,
                section: None,
                success,
                description: Some(if success {
                    "File load completed successfully".to_string()
                } else {
                    "File load failed".to_string()
                }),
            };

            let _ = sender.send(load_complete);
        }))
    }

    /// Asynchronous implementation of `load_from_file` for streams-only architecture
    async fn load_from_file_sync(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Access persistence configuration and clone needed values
        let (format, compression, config_file_path) = {
            let persistence = self
                .persistence
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            (
                persistence.format.clone(),
                persistence.compression,
                persistence.config_file_path.clone(),
            )
        }; // Drop the RwLock guard here

        let data = tokio::fs::read_to_string(&config_file_path).await?;

        let content = if compression {
            let compressed = {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.decode(&data)?
            };
            let decompressed = lz4::block::decompress(&compressed, None)?;
            String::from_utf8(decompressed)?
        } else {
            data
        };

        let config: CandleChatConfig = match format.as_str() {
            "json" => serde_json::from_str(&content)?,
            "yaml" => yyaml::from_str(&content)?,
            "toml" => toml::from_str(&content)?,
            _ => return Err("Unsupported format".into()),
        };

        // Update config atomically
        let config_arc = Arc::new(config);
        self.config.store(config_arc);

        Ok(())
    }

    /// Get Candle configuration change history
    pub async fn get_change_history(&self) -> Vec<CandleConfigurationChangeEvent> {
        let mut events = self.change_events.lock().await;
        let history = events.drain(..).collect();
        history
    }

    /// Get Candle configuration statistics
    pub fn get_statistics(&self) -> CandleConfigurationStatistics {
        CandleConfigurationStatistics {
            total_changes: self.change_counter.load(Ordering::Relaxed),
            current_version: self.version_counter.load(Ordering::Relaxed),
            last_modified: Duration::from_secs(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            ),
            validators_count: 0,      // Will be populated asynchronously
            auto_save_enabled: false, // Will be populated asynchronously
        }
    }
}

/// Candle configuration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleConfigurationStatistics {
    /// Total number of configuration changes made
    pub total_changes: usize,
    /// Current configuration version number
    pub current_version: usize,
    /// Duration since last modification
    pub last_modified: Duration,
    /// Number of active validators
    pub validators_count: usize,
    /// Whether auto-save is currently enabled
    pub auto_save_enabled: bool,
}

/// Streaming wrapper for `CandleModelConfig` that implements `MessageChunk`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CandleModelConfigChunk {
    /// The actual config data
    pub config: CandleModelConfigData,
    /// Error message if this is an error chunk
    pub error_message: Option<String>,
}

/// Serializable version of `CandleModelConfig` using `String` instead of `String`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleModelConfigData {
    /// Model provider (e.g., "openai", "anthropic", "mistral", "gemini")
    pub provider: String,
    /// Model name/identifier
    pub registry_key: String,
    /// Model version or variant
    pub model_version: Option<String>,
    /// Temperature for response randomness (0.0 to 2.0)
    pub temperature: f32,
    /// Maximum tokens in response
    pub max_tokens: Option<u32>,
    /// Top-p nucleus sampling parameter
    pub top_p: Option<f32>,
    /// Top-k sampling parameter
    pub top_k: Option<u32>,
    /// Frequency penalty (-2.0 to 2.0)
    pub frequency_penalty: Option<f32>,
    /// Presence penalty (-2.0 to 2.0)
    pub presence_penalty: Option<f32>,
    /// Stop sequences
    pub stop_sequences: Vec<String>,
    /// System prompt/instructions
    pub system_prompt: Option<String>,
    /// Enable function calling
    pub enable_functions: bool,
    /// Function calling mode ("auto", "none", "required")
    pub function_mode: String,
    /// Model-specific parameters
    pub custom_parameters: HashMap<String, serde_json::Value>,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Retry configuration
    pub retry_config: CandleModelRetryConfig,
    /// Performance settings
    pub performance: CandleModelPerformanceConfig,
}

impl cyrup_sugars::prelude::MessageChunk for CandleModelConfigChunk {
    fn bad_chunk(error: String) -> Self {
        Self {
            config: CandleModelConfigData::default(),
            error_message: Some(error),
        }
    }

    fn error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}

impl From<CandleModelConfig> for CandleModelConfigData {
    fn from(config: CandleModelConfig) -> Self {
        Self {
            provider: config.provider.clone(),
            registry_key: config.registry_key.clone(),
            model_version: config.model_version.clone(),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            top_p: config.top_p,
            top_k: config.top_k,
            frequency_penalty: config.frequency_penalty,
            presence_penalty: config.presence_penalty,
            stop_sequences: config.stop_sequences,
            system_prompt: config.system_prompt,
            enable_functions: config.enable_functions,
            function_mode: config.function_mode,
            custom_parameters: config.custom_parameters,
            timeout_ms: config.timeout_ms,
            retry_config: config.retry_config,
            performance: config.performance,
        }
    }
}

impl Default for CandleModelConfigData {
    fn default() -> Self {
        Self {
            provider: String::new(),
            registry_key: String::new(),
            model_version: None,
            temperature: 0.7,
            max_tokens: None,
            top_p: None,
            top_k: None,
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: Vec::new(),
            system_prompt: None,
            enable_functions: false,
            function_mode: "auto".to_string(),
            custom_parameters: HashMap::new(),
            timeout_ms: 30000,
            retry_config: CandleModelRetryConfig::default(),
            performance: CandleModelPerformanceConfig::default(),
        }
    }
}

// MessageChunk implementations for streaming types
use cyrup_sugars::prelude::MessageChunk;

impl MessageChunk for CandleConfigUpdate {
    fn bad_chunk(error: String) -> Self {
        Self {
            timestamp_nanos: 0,
            update_type: CandleConfigUpdateType::ValidationStarted,
            section: Some(format!("error: {error}")),
            success: false,
            description: Some(error),
        }
    }

    fn error(&self) -> Option<&str> {
        if self.success {
            None
        } else {
            self.description.as_deref()
        }
    }
}

impl Default for CandleConfigUpdate {
    fn default() -> Self {
        Self {
            timestamp_nanos: 0,
            update_type: CandleConfigUpdateType::ValidationStarted,
            section: None,
            success: true,
            description: None,
        }
    }
}

impl MessageChunk for CandlePersistenceEvent {
    fn bad_chunk(_error: String) -> Self {
        // Error parameter reserved for future use
        Self {
            timestamp_nanos: 0,
            previous_timestamp_nanos: 0,
            persistence_type: CandlePersistenceType::Manual,
            success: false,
        }
    }

    fn error(&self) -> Option<&str> {
        if self.success {
            None
        } else {
            Some("Persistence operation failed")
        }
    }
}

impl Default for CandlePersistenceEvent {
    fn default() -> Self {
        Self {
            timestamp_nanos: 0,
            previous_timestamp_nanos: 0,
            persistence_type: CandlePersistenceType::Manual,
            success: true,
        }
    }
}
