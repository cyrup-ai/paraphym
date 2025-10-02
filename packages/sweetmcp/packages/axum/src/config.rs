// Removed unused db imports

use std::{collections::HashMap, io::Write, path::Path, str::FromStr};

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use log::LevelFilter;
use serde::{Deserialize, Serialize};

use crate::db::DatabaseConfig;

/// Initialize the logger with the specified path and level
pub fn init_logger(path: Option<&str>, level: Option<&str>) -> Result<()> {
    let log_level = LevelFilter::from_str(level.unwrap_or("info"))?;

    // If the log path is not provided, use the stderr
    let log_file = match path {
        Some(p) => Box::new(
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(p)?,
        ) as Box<dyn Write + Send + Sync + 'static>,
        _ => Box::new(std::io::stderr()) as Box<dyn Write + Send + Sync + 'static>,
    };

    // TODO: apply module filter
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}/{}:{} {} [{}] - {}",
                record.module_path().unwrap_or("unknown"),
                basename(record.file().unwrap_or("unknown")),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(log_file))
        .filter(None, log_level)
        .try_init()?;

    Ok(())
}

#[allow(dead_code)]
/// Helper function to extract the basename from a path.
/// Returns the input string if it cannot be parsed as a Path or has no filename.
///
/// Used internally by the logger implementation
#[doc(hidden)]
pub fn basename(path_str: &str) -> String {
    // Make pub
    Path::new(path_str)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or(path_str) // Fallback to the original string if no filename
        .to_string()
}

/// Supported configuration file formats
///
/// Used internally by the config parsing implementation
#[derive(Debug)]
#[doc(hidden)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

/// Detect the configuration format based on content
pub fn detect_format_from_content(content: &str) -> Result<ConfigFormat> {
    // Try to determine if it's JSON by checking for { or [ at start (after whitespace)
    let trimmed = content.trim_start();
    if (trimmed.starts_with('{') || trimmed.starts_with('['))
        && serde_json::from_str::<serde_json::Value>(content).is_ok()
    {
        return Ok(ConfigFormat::Json);
    }

    // Try YAML - Check for common YAML indicators
    if (trimmed.contains(": ") || trimmed.starts_with("---"))
        && serde_yaml::from_str::<serde_yaml::Value>(content).is_ok()
    {
        return Ok(ConfigFormat::Yaml);
    }

    // Try TOML - Look for key-value pairs with = or section headers
    if (trimmed.contains('=') || trimmed.contains('['))
        && toml::from_str::<toml::Value>(content).is_ok()
    {
        return Ok(ConfigFormat::Toml);
    }

    Err(anyhow!(
        "Unable to detect config format. Content doesn't appear to be valid JSON, YAML, or TOML"
    ))
}

/// Validate the configuration content
pub fn validate_config(content: &str) -> Result<()> {
    // First try to parse as a generic Value to check basic format
    let format = detect_format_from_content(content)?;
    let value: serde_json::Value = match format {
        ConfigFormat::Json => serde_json::from_str(content).context("Failed to parse as JSON")?,
        ConfigFormat::Yaml => {
            let yaml_value: serde_yaml::Value =
                serde_yaml::from_str(content).context("Failed to parse as YAML")?;
            serde_json::to_value(yaml_value).context("Failed to convert YAML to JSON value")?
        }
        ConfigFormat::Toml => {
            let toml_value: toml::Value =
                toml::from_str(content).context("Failed to parse as TOML")?;
            serde_json::to_value(toml_value).context("Failed to convert TOML to JSON value")?
        }
    };

    // Additional validation for file paths
    if let Some(plugins) = value
        .as_object()
        .and_then(|obj| obj.get("plugins"))
        .and_then(|v| v.as_array())
    {
        for plugin in plugins {
            if let Some(path) = plugin.get("path").and_then(|v| v.as_str()) {
                // Only validate local file paths (not http or oci)
                if !path.starts_with("http")
                    && !path.starts_with("oci://")
                    && !Path::new(path).exists()
                {
                    return Err(anyhow!("Local plugin path '{}' does not exist", path));
                }
            }
        }
    }

    Ok(())
}

/// Parse configuration from a string
pub fn parse_config_from_str<T: serde::de::DeserializeOwned>(content: &str) -> Result<T> {
    let format = detect_format_from_content(content)?;
    match format {
        ConfigFormat::Json => serde_json::from_str(content).context("Failed to parse JSON config"),
        ConfigFormat::Yaml => serde_yaml::from_str(content).context("Failed to parse YAML config"),
        ConfigFormat::Toml => toml::from_str(content).context("Failed to parse TOML config"),
    }
}

/// Parse configuration from a file path and its content string.
/// It first attempts to determine the format from the file extension.
/// If the extension is missing or unrecognized, it falls back to detecting the format from the
/// content.
pub fn parse_config<T: serde::de::DeserializeOwned>(content: &str, file_path: &Path) -> Result<T> {
    if let Some(extension) = file_path.extension().and_then(|ext| ext.to_str()) {
        // If we have a file extension, try that format first
        match extension.to_lowercase().as_str() {
            "json" => return serde_json::from_str(content).context("Failed to parse JSON config"),
            "yaml" | "yml" => {
                return serde_yaml::from_str(content).context("Failed to parse YAML config");
            }
            "toml" => return toml::from_str(content).context("Failed to parse TOML config"),
            _ => {} // Fall through to content-based detection
        }
    }

    // If no extension or unknown extension, try to detect from content
    parse_config_from_str(content)
}

/// Root configuration entry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootEntry {
    pub name: String,
    pub url: String,
}

/// Filesystem scanning configuration for roots
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootScanConfig {
    /// Paths to scan for roots
    pub paths: Vec<String>,
    
    /// Marker files that indicate a root (e.g., .git, package.json)
    pub markers: Vec<String>,
    
    /// Maximum directory depth to scan
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
}

fn default_max_depth() -> usize { 3 }

/// Root configuration
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RootsConfig {
    /// Static roots defined in config
    #[serde(default)]
    pub static_roots: Vec<RootEntry>,
    
    /// Filesystem scanning configuration
    #[serde(default)]
    pub scan: Option<RootScanConfig>,
}

/// Represents the top-level configuration structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// A list of plugin configurations.
    pub plugins: Vec<PluginConfig>,

    /// Database configuration (optional).
    #[serde(default)]
    pub database: Option<DatabaseConfig>,
    
    /// Root configuration
    #[serde(default)]
    pub roots: RootsConfig,
}

/// Represents the configuration for a single plugin.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    /// The unique name of the plugin.
    pub name: String,
    /// The path to the plugin (file path, URL, or OCI reference).
    pub path: String,
    /// Optional environment configuration for the plugin runtime.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<EnvConfig>,
}

/// Represents the environment configuration for a plugin runtime.
/// Corresponds to the "env" object in the JSON schema.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EnvConfig {
    /// Optional list of hosts the plugin is allowed to connect to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_hosts: Option<Vec<String>>,
    /// Optional list of file system paths the plugin is allowed to access.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_paths: Option<Vec<String>>,

    /// Captures any additional key-value pairs defined under the "env" object,
    /// fulfilling the "additionalProperties": true requirement in the schema.
    /// Assumes string values based on common environment variable usage.
    #[serde(flatten)]
    pub additional_vars: HashMap<String, String>,
}

impl Config {
    /// Load configuration from a file
    pub async fn load(path: &Path) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .context(format!("Failed to read config file: {:?}", path))?;
        
        let config: Config = serde_json::from_str(&content)
            .context(format!("Failed to parse config file: {:?}", path))?;
        
        Ok(config)
    }
    
    /// Get a configuration value by key using dot-notation path
    /// 
    /// # Examples
    /// - `config.get("plugins.0.name")` - Get first plugin's name
    /// - `config.get("database.namespace")` - Get database namespace
    /// - `config.get("plugins.1.env.allowed_hosts.0")` - Get first allowed host of second plugin
    /// 
    /// Returns `None` if the path is invalid or the key doesn't exist
    pub fn get(&self, key: &str) -> Option<String> {
        if key.is_empty() {
            return None;
        }
        
        // Convert config to JSON value for traversal
        let config_value = match serde_json::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        
        // Split key into path segments and traverse
        let segments: Vec<&str> = key.split('.').collect();
        let mut current = &config_value;
        
        for segment in segments {
            current = if let Ok(index) = segment.parse::<usize>() {
                // Numeric segment - try array indexing
                current.get(index)?
            } else {
                // String segment - try object key access
                current.get(segment)?
            };
        }
        
        // Convert final value to string
        match current {
            serde_json::Value::String(s) => Some(s.clone()),
            serde_json::Value::Null => None,
            other => Some(other.to_string()),
        }
    }
    
    /// Set a configuration value by key using dot-notation path
    /// 
    /// # Examples
    /// - `config.set("database.namespace", "production")` - Set database namespace
    /// - `config.set("plugins.0.name", "updated-plugin")` - Update first plugin's name
    /// 
    /// Returns an error if the path is invalid or the modification fails
    pub fn set(&mut self, key: &str, value: String) -> Result<()> {
        if key.is_empty() {
            return Err(anyhow!("Configuration key cannot be empty"));
        }
        
        // Convert config to mutable JSON value
        let mut config_value = serde_json::to_value(&*self)
            .context("Failed to serialize config for modification")?;
        
        let segments: Vec<&str> = key.split('.').collect();
        if segments.is_empty() {
            return Err(anyhow!("Invalid configuration key path"));
        }
        
        // Navigate to parent of target location
        let mut current = &mut config_value;
        for segment in &segments[..segments.len() - 1] {
            let next = if let Ok(index) = segment.parse::<usize>() {
                // Numeric segment - navigate array
                match current.get_mut(index) {
                    Some(v) => v,
                    None => return Err(anyhow!("Array index {} out of bounds in path '{}'", index, key)),
                }
            } else {
                // String segment - navigate object
                match current.as_object_mut() {
                    Some(obj) => match obj.get_mut(*segment) {
                        Some(v) => v,
                        None => return Err(anyhow!("Key '{}' not found in path '{}'", segment, key)),
                    },
                    None => return Err(anyhow!("Expected object at '{}' in path '{}'", segment, key)),
                }
            };
            current = next;
        }
        
        // Parse value - try JSON first, fallback to string
        let new_value = match serde_json::from_str::<serde_json::Value>(&value) {
            Ok(v) => v,
            Err(_) => serde_json::Value::String(value),
        };
        
        // Set the value at the final segment
        let last_segment = segments.last().context("Empty segments list")?;
        if let Ok(index) = last_segment.parse::<usize>() {
            // Set array element
            match current.as_array_mut() {
                Some(arr) => {
                    if index < arr.len() {
                        arr[index] = new_value;
                    } else {
                        return Err(anyhow!("Array index {} out of bounds in path '{}'", index, key));
                    }
                }
                None => return Err(anyhow!("Expected array at final position in path '{}'", key)),
            }
        } else {
            // Set object field
            match current.as_object_mut() {
                Some(obj) => {
                    obj.insert(last_segment.to_string(), new_value);
                }
                None => return Err(anyhow!("Expected object at final position in path '{}'", key)),
            }
        }
        
        // Convert back to Config and update self
        *self = serde_json::from_value(config_value)
            .context("Failed to deserialize modified config")?;
        
        Ok(())
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate plugins
        if self.plugins.is_empty() {
            return Err(anyhow!("Config must have at least one plugin"));
        }
        
        for plugin in &self.plugins {
            if plugin.name.is_empty() {
                return Err(anyhow!("Plugin name cannot be empty"));
            }
            if plugin.path.is_empty() {
                return Err(anyhow!("Plugin path cannot be empty"));
            }
        }
        
        Ok(())
    }
}
