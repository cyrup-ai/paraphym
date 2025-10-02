//! Base entities and example implementations
//!
//! This module provides base entity structures and example implementations
//! with zero allocation patterns and blazing-fast performance.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::core::{Entity, utc_now};

/// Common fields for database entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    /// Entity ID
    pub id: Option<String>,

    /// Creation timestamp
    #[serde(default = "utc_now")]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    #[serde(default = "utc_now")]
    pub updated_at: DateTime<Utc>,
}

impl BaseEntity {
    /// Create a new entity
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a new entity with a specific ID
    pub fn with_id(id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Some(id),
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Check if the entity is new (no ID)
    pub fn is_new(&self) -> bool {
        self.id.is_none()
    }

    /// Check if the entity is persisted (has ID)
    pub fn is_persisted(&self) -> bool {
        self.id.is_some()
    }

    /// Get the age of the entity in seconds
    pub fn age_seconds(&self) -> i64 {
        (Utc::now() - self.created_at).num_seconds()
    }

    /// Get the time since last update in seconds
    pub fn seconds_since_update(&self) -> i64 {
        (Utc::now() - self.updated_at).num_seconds()
    }

    /// Check if the entity was recently created (within specified seconds)
    pub fn is_recently_created(&self, seconds: i64) -> bool {
        self.age_seconds() <= seconds
    }

    /// Check if the entity was recently updated (within specified seconds)
    pub fn is_recently_updated(&self, seconds: i64) -> bool {
        self.seconds_since_update() <= seconds
    }

    /// Clone the entity and mark it as new (remove ID and update timestamps)
    pub fn clone_as_new(&self) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Prepare for update (update the updated_at timestamp)
    pub fn prepare_for_update(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl Default for BaseEntity {
    fn default() -> Self {
        Self::new()
    }
}

/// Example user entity implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub base: BaseEntity,

    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub is_active: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub profile: UserProfile,
}

/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
}

impl Entity for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn id(&self) -> Option<String> {
        self.base.id.clone()
    }

    fn set_id(&mut self, id: String) {
        self.base.id = Some(id);
    }
}

impl User {
    /// Create a new user
    pub fn new(username: String, email: String) -> Self {
        Self {
            base: BaseEntity::new(),
            username,
            email,
            password_hash: None,
            is_active: true,
            last_login: None,
            profile: UserProfile::default(),
        }
    }

    /// Create a new user with password
    pub fn with_password(username: String, email: String, password_hash: String) -> Self {
        Self {
            base: BaseEntity::new(),
            username,
            email,
            password_hash: Some(password_hash),
            is_active: true,
            last_login: None,
            profile: UserProfile::default(),
        }
    }

    /// Set the password hash
    pub fn set_password_hash(&mut self, password_hash: String) {
        self.password_hash = Some(password_hash);
        self.base.prepare_for_update();
    }

    /// Clear the password hash
    pub fn clear_password(&mut self) {
        self.password_hash = None;
        self.base.prepare_for_update();
    }

    /// Check if user has a password set
    pub fn has_password(&self) -> bool {
        self.password_hash.is_some()
    }

    /// Activate the user
    pub fn activate(&mut self) {
        self.is_active = true;
        self.base.prepare_for_update();
    }

    /// Deactivate the user
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.base.prepare_for_update();
    }

    /// Record a login
    pub fn record_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.base.prepare_for_update();
    }

    /// Check if user has ever logged in
    pub fn has_logged_in(&self) -> bool {
        self.last_login.is_some()
    }

    /// Get days since last login
    pub fn days_since_last_login(&self) -> Option<i64> {
        self.last_login
            .map(|login_time| (Utc::now() - login_time).num_days())
    }

    /// Check if user is recently active (logged in within specified days)
    pub fn is_recently_active(&self, days: i64) -> bool {
        match self.days_since_last_login() {
            Some(days_since) => days_since <= days,
            None => false, // Never logged in
        }
    }

    /// Update user profile
    pub fn update_profile(&mut self, profile: UserProfile) {
        self.profile = profile;
        self.base.prepare_for_update();
    }

    /// Set display name
    pub fn set_display_name(&mut self, display_name: Option<String>) {
        self.profile.display_name = display_name;
        self.base.prepare_for_update();
    }

    /// Get the user's display name or username as fallback
    pub fn get_display_name(&self) -> &str {
        self.profile
            .display_name
            .as_deref()
            .unwrap_or(&self.username)
    }

    /// Get the user's full name if available
    pub fn get_full_name(&self) -> Option<String> {
        match (&self.profile.first_name, &self.profile.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            (None, None) => None,
        }
    }

    /// Check if user profile is complete
    pub fn is_profile_complete(&self) -> bool {
        self.profile.first_name.is_some()
            && self.profile.last_name.is_some()
            && self.profile.display_name.is_some()
    }

    /// Validate user data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.username.trim().is_empty() {
            errors.push("Username cannot be empty".to_string());
        }

        if self.username.len() < 3 {
            errors.push("Username must be at least 3 characters long".to_string());
        }

        if self.email.trim().is_empty() {
            errors.push("Email cannot be empty".to_string());
        }

        if !self.email.contains('@') {
            errors.push("Email must be a valid email address".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Create a sanitized version for public display (remove sensitive data)
    pub fn sanitize(&self) -> PublicUser {
        PublicUser {
            id: self.base.id.clone(),
            username: self.username.clone(),
            display_name: self.get_display_name().to_string(),
            is_active: self.is_active,
            created_at: self.base.created_at,
            last_login: self.last_login,
            profile: PublicUserProfile {
                display_name: self.profile.display_name.clone(),
                bio: self.profile.bio.clone(),
                avatar_url: self.profile.avatar_url.clone(),
            },
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            base: BaseEntity::new(),
            username: String::new(),
            email: String::new(),
            password_hash: None,
            is_active: true,
            last_login: None,
            profile: UserProfile::default(),
        }
    }
}

impl UserProfile {
    /// Create a new empty profile
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a profile with basic information
    pub fn with_names(first_name: String, last_name: String) -> Self {
        Self {
            first_name: Some(first_name.clone()),
            last_name: Some(last_name.clone()),
            display_name: Some(format!("{} {}", first_name, last_name)),
            bio: None,
            avatar_url: None,
            timezone: None,
            locale: None,
        }
    }

    /// Check if the profile has any information
    pub fn is_empty(&self) -> bool {
        self.first_name.is_none()
            && self.last_name.is_none()
            && self.display_name.is_none()
            && self.bio.is_none()
            && self.avatar_url.is_none()
            && self.timezone.is_none()
            && self.locale.is_none()
    }

    /// Get completion percentage (0.0 to 1.0)
    pub fn completion_percentage(&self) -> f32 {
        let total_fields = 7.0;
        let mut filled_fields = 0.0;

        if self.first_name.is_some() {
            filled_fields += 1.0;
        }
        if self.last_name.is_some() {
            filled_fields += 1.0;
        }
        if self.display_name.is_some() {
            filled_fields += 1.0;
        }
        if self.bio.is_some() {
            filled_fields += 1.0;
        }
        if self.avatar_url.is_some() {
            filled_fields += 1.0;
        }
        if self.timezone.is_some() {
            filled_fields += 1.0;
        }
        if self.locale.is_some() {
            filled_fields += 1.0;
        }

        filled_fields / total_fields
    }
}

/// Public user representation (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Option<String>,
    pub username: String,
    pub display_name: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub profile: PublicUserProfile,
}

/// Public user profile (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUserProfile {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

/// Plugin registry entity for persistent storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEntity {
    /// SurrealDB record ID (format: "plugin_registry:uuid")
    pub id: Option<String>,
    
    /// Plugin name (unique identifier)
    pub name: String,
    
    /// Plugin source path (file path, URL, or OCI reference)
    pub source_path: String,
    
    /// Plugin type: "file", "http", "oci"
    pub source_type: String,
    
    /// SHA256 hash of plugin WASM for integrity verification
    pub wasm_hash: String,
    
    /// Runtime environment configuration (JSON)
    pub env_config: Option<serde_json::Value>,
    
    /// Plugin status: "active", "disabled", "error"
    pub status: String,
    
    /// Last error message if status="error"
    pub error_message: Option<String>,
    
    /// Discovery metadata (tools_count, prompts_count as JSON)
    pub metadata: Option<serde_json::Value>,
    
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_loaded_at: Option<DateTime<Utc>>,
}

impl Entity for PluginEntity {
    fn table_name() -> &'static str {
        "plugin_registry"
    }
    
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    
    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

impl PluginEntity {
    /// Create from PluginConfig and WASM bytes
    pub fn from_config(config: &crate::config::PluginConfig, wasm_hash: String) -> Self {
        let source_type = if config.path.starts_with("http") {
            "http"
        } else if config.path.starts_with("oci://") {
            "oci"
        } else {
            "file"
        };
        
        let env_config = config.env.as_ref().map(|e| {
            serde_json::to_value(e).unwrap_or(serde_json::Value::Null)
        });
        
        let now = Utc::now();
        
        Self {
            id: None,
            name: config.name.clone(),
            source_path: config.path.clone(),
            source_type: source_type.to_string(),
            wasm_hash,
            env_config,
            status: "active".to_string(),
            error_message: None,
            metadata: None,
            created_at: now,
            updated_at: now,
            last_loaded_at: Some(now),
        }
    }
}

/// Tool registry entity for indexing and search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEntity {
    pub id: Option<String>,
    
    /// Tool name (unique across all plugins)
    pub name: String,
    
    /// Plugin that provides this tool
    pub plugin_name: String,
    
    /// Tool description
    pub description: Option<String>,
    
    /// Input schema (stored as JSON)
    pub input_schema: serde_json::Value,
    
    /// Tags for categorization/search (extracted from description)
    pub tags: Vec<String>,
    
    /// Usage statistics
    pub call_count: i64,
    pub last_called_at: Option<DateTime<Utc>>,
    pub average_duration_ms: Option<f64>,
    
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entity for ToolEntity {
    fn table_name() -> &'static str {
        "tool_registry"
    }
    
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    
    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

impl ToolEntity {
    /// Create from MCP Tool
    pub fn from_tool(tool: &crate::types::Tool, plugin_name: String) -> Self {
        let input_schema = serde_json::to_value(&tool.input_schema)
            .unwrap_or(serde_json::Value::Null);
        
        let tags = extract_tags_from_description(
            tool.description.as_deref().unwrap_or("")
        );
        
        let now = Utc::now();
        
        Self {
            id: None,
            name: tool.name.clone(),
            plugin_name,
            description: tool.description.clone(),
            input_schema,
            tags,
            call_count: 0,
            last_called_at: None,
            average_duration_ms: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Record tool call with duration (updates moving average)
    pub fn record_call(&mut self, duration_ms: f64) {
        self.call_count += 1;
        self.last_called_at = Some(Utc::now());
        
        // Calculate moving average
        if let Some(avg) = self.average_duration_ms {
            let n = self.call_count as f64;
            self.average_duration_ms = Some((avg * (n - 1.0) + duration_ms) / n);
        } else {
            self.average_duration_ms = Some(duration_ms);
        }
        
        self.updated_at = Utc::now();
    }
}

/// Prompt library entity for template storage and versioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptEntity {
    pub id: Option<String>,
    
    /// Prompt ID (unique across all plugins)
    pub prompt_id: String,
    
    /// Prompt name
    pub name: String,
    
    /// Plugin that provides this prompt
    pub plugin_name: String,
    
    /// Description
    pub description: Option<String>,
    
    /// Template content (Jinja2 template)
    pub template: String,
    
    /// Arguments schema (JSON array)
    pub arguments: Option<serde_json::Value>,
    
    /// Version number for template versioning
    pub version: i32,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Usage statistics
    pub use_count: i64,
    pub last_used_at: Option<DateTime<Utc>>,
    
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entity for PromptEntity {
    fn table_name() -> &'static str {
        "prompt_library"
    }
    
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    
    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

impl PromptEntity {
    /// Create from MCP Prompt with template
    pub fn from_prompt(
        prompt: &crate::types::Prompt,
        plugin_name: String,
        template: String,
    ) -> Self {
        let arguments = prompt.arguments.as_ref().map(|args| {
            serde_json::to_value(args).unwrap_or(serde_json::Value::Null)
        });
        
        let tags = extract_tags_from_description(
            prompt.description.as_deref().unwrap_or("")
        );
        
        let now = Utc::now();
        
        Self {
            id: None,
            prompt_id: prompt.id.clone(),
            name: prompt.name.clone(),
            plugin_name,
            description: prompt.description.clone(),
            template,
            arguments,
            version: 1,
            tags,
            use_count: 0,
            last_used_at: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Update template and increment version
    pub fn update_template(&mut self, new_template: String) {
        self.template = new_template;
        self.version += 1;
        self.updated_at = Utc::now();
    }
    
    /// Record prompt usage
    pub fn record_use(&mut self) {
        self.use_count += 1;
        self.last_used_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

/// Audit log for tracking all database operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Option<String>,
    
    /// Operation type: "create", "update", "delete"
    pub operation: String,
    
    /// Entity type: "plugin", "tool", "prompt"
    pub entity_type: String,
    
    /// Entity ID that was operated on
    pub entity_id: String,
    
    /// Actor (user or system) that performed operation
    pub actor: String,
    
    /// Old values (for updates/deletes, stored as JSON)
    pub old_values: Option<serde_json::Value>,
    
    /// New values (for creates/updates, stored as JSON)
    pub new_values: Option<serde_json::Value>,
    
    /// Timestamp
    pub created_at: DateTime<Utc>,
}

impl Entity for AuditLog {
    fn table_name() -> &'static str {
        "audit_log"
    }
    
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    
    fn set_id(&mut self, id: String) {
        self.id = Some(id);
    }
}

/// Extract keywords from description for tagging
fn extract_tags_from_description(desc: &str) -> Vec<String> {
    let keywords = ["authentication", "hash", "crypto", "time", "browser", 
                    "fetch", "database", "file", "network", "api"];
    let desc_lower = desc.to_lowercase();
    
    keywords.iter()
        .filter(|&kw| desc_lower.contains(kw))
        .map(|&kw| kw.to_string())
        .collect()
}
