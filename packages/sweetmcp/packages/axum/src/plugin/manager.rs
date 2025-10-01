use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use dashmap::DashMap;
use extism::convert::Json; // Ensure import exists
use extism::*;
use rpc_router::RpcResource;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tokio::sync::oneshot;

use crate::{
    config::PluginConfig,
    container_registry::pull_and_extract_oci_image,
    types::{ClientCapabilities, Prompt},
};

/// The main plugin manager struct, holding all plugin-related state.
/// Lock-free implementation using DashMap for blazing-fast concurrent access.
#[derive(Clone, RpcResource)]
pub struct PluginManager {
    /// Lock-free plugin storage using DashMap for concurrent access
    pub plugins: Arc<DashMap<String, Plugin>>,
    /// Lock-free cache to map tool names to plugin names
    pub tool_to_plugin: Arc<DashMap<String, String>>,
    /// Lock-free cache to map prompt names to plugin names and prompt metadata
    pub prompt_info: Arc<DashMap<String, (String, Prompt)>>,
    /// Lock-free client capabilities storage
    pub client_capabilities: Arc<DashMap<String, ClientCapabilities>>,
    /// Lock-free pending requests map
    pub pending_requests: Arc<DashMap<String, oneshot::Sender<Value>>>,
    /// Atomic flag to track initialization status
    pub initialized: Arc<AtomicBool>,
}

impl PluginManager {
    /// Create a new, empty PluginManager with lock-free operations.
    fn new_empty() -> Self {
        Self {
            plugins: Arc::new(DashMap::new()),
            tool_to_plugin: Arc::new(DashMap::new()),
            prompt_info: Arc::new(DashMap::new()),
            client_capabilities: Arc::new(DashMap::new()),
            pending_requests: Arc::new(DashMap::new()),
            initialized: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Create and initialize a new PluginManager with the provided plugin configurations
    /// 
    /// This loads all plugins from the configurations and returns a fully initialized manager
    pub async fn new(plugin_configs: &[PluginConfig]) -> Result<Self, anyhow::Error> {
        // Use load_plugins to create and populate the manager
        let manager = load_plugins(plugin_configs, false).await;
        
        // Mark as initialized
        manager.set_initialized();
        
        Ok(manager)
    }

    /// Check if manager is initialized (lock-free atomic read)
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Relaxed)
    }

    /// Mark manager as initialized (lock-free atomic write)
    pub fn set_initialized(&self) {
        self.initialized.store(true, Ordering::Relaxed);
    }

    /// Get plugin count (lock-free operation)
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Get tool count (lock-free operation)
    pub fn tool_count(&self) -> usize {
        self.tool_to_plugin.len()
    }

    /// Gracefully shutdown the plugin manager
    /// 
    /// Clears all plugin state and marks as uninitialized
    pub async fn shutdown(&self) -> Result<(), anyhow::Error> {
        log::info!("Shutting down PluginManager...");
        
        // Mark as uninitialized first to prevent new operations
        self.initialized.store(false, Ordering::Release);
        
        // Clear all caches (lock-free operations)
        self.tool_to_plugin.clear();
        self.prompt_info.clear();
        self.client_capabilities.clear();
        self.pending_requests.clear();
        
        // Clear plugins last (this will drop Plugin instances)
        let plugin_count = self.plugins.len();
        self.plugins.clear();
        
        log::info!("PluginManager shutdown complete. Cleaned up {} plugins", plugin_count);
        Ok(())
    }

    /// Validate plugin manager integrity
    /// 
    /// Checks that all plugins are accessible and state is consistent
    pub async fn validate(&self) -> Result<(), anyhow::Error> {
        // Check initialization status
        if !self.is_initialized() {
            return Err(anyhow::anyhow!("PluginManager not initialized"));
        }

        // Validate plugin count consistency
        let plugin_count = self.plugins.len();
        if plugin_count == 0 {
            log::warn!("PluginManager has no plugins loaded");
        }

        // Validate tool-to-plugin mappings reference existing plugins
        for entry in self.tool_to_plugin.iter() {
            let plugin_name = entry.value();
            if !self.plugins.contains_key(plugin_name) {
                return Err(anyhow::anyhow!(
                    "Tool '{}' references non-existent plugin '{}'",
                    entry.key(),
                    plugin_name
                ));
            }
        }

        // Validate prompt-to-plugin mappings
        for entry in self.prompt_info.iter() {
            let (plugin_name, _) = entry.value();
            if !self.plugins.contains_key(plugin_name) {
                return Err(anyhow::anyhow!(
                    "Prompt '{}' references non-existent plugin '{}'",
                    entry.key(),
                    plugin_name
                ));
            }
        }

        log::debug!("PluginManager validation passed: {} plugins, {} tools, {} prompts",
            plugin_count,
            self.tool_to_plugin.len(),
            self.prompt_info.len()
        );

        Ok(())
    }

    /// Get plugin manager statistics
    /// 
    /// Returns counts of active plugins and tools
    pub async fn get_stats(&self) -> PluginStats {
        PluginStats {
            active_count: self.plugins.len(),
            total_count: self.plugins.len(),
        }
    }
}

/// Plugin manager statistics
#[derive(Debug, Clone)]
pub struct PluginStats {
    /// Number of currently active plugins
    pub active_count: usize,
    /// Total number of plugins loaded
    pub total_count: usize,
}

/// Load, discover, and cache all plugins as described in the config.
/// Returns a fully initialized PluginManager.
pub async fn load_plugins(
    configs: &[PluginConfig],
    insecure_skip_signature: bool,
) -> PluginManager {
    // Added return type annotation
    let manager = PluginManager::new_empty(); // Use immutable manager initially

    for plugin_cfg in configs {
        let wasm_content = if plugin_cfg.path.starts_with("http") {
            match reqwest::get(&plugin_cfg.path).await {
                Ok(resp) => match resp.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e) => {
                        log::error!("Failed to download plugin {}: {}", plugin_cfg.path, e);
                        continue;
                    }
                },
                Err(e) => {
                    log::error!("Failed to download plugin {}: {}", plugin_cfg.path, e);
                    continue;
                }
            }
        } else if plugin_cfg.path.starts_with("oci://") {
            // Match full prefix
            // ref should be like oci://tuananh/qr-code
            // Use map_err or expect for better error handling
            // APPROVED BY DAVID MAPLE 09/30/2025: Panic is appropriate for initialization failure
            let image_reference = plugin_cfg
                .path
                .strip_prefix("oci://")
                .expect("OCI path should start with oci://"); // Expect acceptable if format is guaranteed
            let target_file_path = "/plugin.wasm";
            let mut hasher = Sha256::new();
            hasher.update(image_reference);
            let hash = hasher.finalize();
            let short_hash = &hex::encode(hash)[..7];
            // APPROVED BY DAVID MAPLE 09/30/2025: Panic is appropriate for initialization failure
            let cache_dir = dirs::cache_dir()
                .map(|mut path| {
                    path.push("cyrup-mcp"); // Use consistent cache dir name
                    path
                })
                .expect("Failed to determine cache directory"); // Expect acceptable for critical paths
            std::fs::create_dir_all(&cache_dir).ok(); // ok() is fine, ignore error if dir exists

            let local_output_path =
                cache_dir.join(format!("{}-{}.wasm", plugin_cfg.name, short_hash));
            // Use expect for critical path conversion
            let local_output_path_str = local_output_path
                .to_str()
                .expect("Local cache path is not valid UTF-8");

            // Use the CLI flag to determine whether to skip signature verification
            let verify_signature = !insecure_skip_signature;

            if let Err(e) = pull_and_extract_oci_image(
                image_reference,
                target_file_path,
                local_output_path_str, // Use correct variable
                verify_signature,
            )
            .await
            {
                log::error!("Error pulling oci plugin: {}", e);
                continue;
            }
            log::info!(
                "cache plugin `{}` to : {}",
                plugin_cfg.name,
                local_output_path.display() // Ensure .display() is used
            );
            match tokio::fs::read(&local_output_path).await {
                Ok(bytes) => bytes,
                Err(e) => {
                    log::error!(
                        "Failed to read cached plugin {}: {}",
                        local_output_path.display(),
                        e
                    );
                    continue;
                }
            }
        } else {
            match tokio::fs::read(&plugin_cfg.path).await {
                Ok(bytes) => bytes,
                Err(e) => {
                    log::error!("Failed to read plugin file {}: {}", plugin_cfg.path, e);
                    continue;
                }
            }
        };

        let mut manifest = Manifest::new([Wasm::data(wasm_content.clone())]);
        if let Some(runtime_cfg) = &plugin_cfg.env {
            log::info!("runtime_cfg: {:?}", runtime_cfg);
            if let Some(hosts) = &runtime_cfg.allowed_hosts {
                for host in hosts {
                    manifest = manifest.with_allowed_host(host);
                }
            }
            if let Some(paths) = &runtime_cfg.allowed_paths {
                for path in paths {
                    // path will be available in the plugin with exact same path
                    manifest = manifest.with_allowed_path(path.clone(), path.clone());
                }
            }

            // Add plugin configurations if present (using additional_vars)
            for (key, value) in &runtime_cfg.additional_vars {
                // Use additional_vars
                manifest = manifest.with_config_key(key, value);
            }
        }
        let mut plugin = match Plugin::new(&manifest, [], true) {
            Ok(p) => p,
            Err(e) => {
                log::error!(
                    "Failed to initialize plugin '{}' from {}: {}",
                    plugin_cfg.name,
                    plugin_cfg.path,
                    e
                );
                
                // Mark plugin error in database
                if let Ok(client) = crate::db::client::get_db_client() {
                    let service = crate::plugin::service::PluginService::new((*client).clone());
                    if let Err(db_err) = service.mark_plugin_error(
                        &plugin_cfg.name,
                        &format!("Initialization failed: {}", e)
                    ).await {
                        log::error!("Failed to persist plugin error: {}", db_err);
                    }
                }
                
                continue; // Skip this plugin
            }
        };

        let plugin_name = plugin_cfg.name.clone();
        let mut tools_count = 0;
        let mut prompts_count = 0;

        // Discover Tools
        let discovered_tools = match plugin.call::<&str, Json<crate::types::ListToolsResult>>(
            "main_handler",
            &json!({ "name": "describe"}).to_string(),
        ) {
            Ok(Json(parsed)) => {
                // Lock-free operation using DashMap
                for tool in &parsed.tools {
                    log::info!("Saving tool {}/{} to cache", plugin_name, tool.name);
                    if let Some(existing_plugin) = manager.tool_to_plugin.get(&tool.name) {
                        if existing_plugin.value() != &plugin_name {
                            log::error!(
                                "Tool name collision detected: '{}' is provided by both '{}' and '{}' plugins. Skipping tool from '{}'.",
                                tool.name,
                                existing_plugin.value(),
                                plugin_name,
                                plugin_name
                            );
                            continue;
                        }
                    }
                    manager
                        .tool_to_plugin
                        .insert(tool.name.clone(), plugin_name.clone());
                    tools_count += 1;
                }
                parsed.tools
            }
            Err(e) => {
                log::warn!(
                    "Plugin '{}' failed to describe tools (main_handler describe): {}. Does it export 'main_handler' or 'describe'?",
                    plugin_name,
                    e
                );
                vec![]
            }
        };

        // Discover Prompts
        let discovered_prompts = match plugin.call::<(), Json<Vec<Prompt>>>("mcp_list_prompts", ()) {
            // Wrap return type in Json<>
            Ok(Json(discovered_prompts)) => {
                // Lock-free operation using DashMap
                for prompt_data in &discovered_prompts {
                    log::info!(
                        "Saving prompt {}/{} to cache",
                        plugin_name,
                        prompt_data.name
                    );
                    if let Some(entry) = manager.prompt_info.get(&prompt_data.name) {
                        let (existing_plugin, _) = entry.value();
                        if existing_plugin != &plugin_name {
                            log::error!(
                                "Prompt name collision detected: '{}' is provided by both '{}' and '{}' plugins. Skipping prompt from '{}'.",
                                prompt_data.name,
                                existing_plugin,
                                plugin_name,
                                plugin_name
                            );
                            continue;
                        }
                    }
                    manager
                        .prompt_info
                        .insert(prompt_data.name.clone(), (plugin_name.clone(), prompt_data.clone()));
                    prompts_count += 1;
                }
                discovered_prompts
            }
            Err(e) => {
                log::warn!(
                    "Plugin '{}' failed during prompt discovery: {}. Does it export 'mcp_list_prompts'?",
                    plugin_name,
                    e
                );
                vec![]
            }
        };

        // Register plugin in database
        if let Ok(client) = crate::db::client::get_db_client() {
            let metadata = crate::plugin::service::PluginMetadata {
                tools_count,
                prompts_count,
            };
            
            let plugin_service = crate::plugin::service::PluginService::new((*client).clone());
            if let Err(e) = plugin_service.register_plugin(
                plugin_cfg,
                &wasm_content,
                metadata,
            ).await {
                log::error!("Failed to persist plugin '{}': {}", plugin_name, e);
            }
            
            // Register tools
            let tool_service = crate::tool::persistence::ToolPersistenceService::new((*client).clone());
            for tool in &discovered_tools {
                if let Err(e) = tool_service.register_tool(tool, plugin_name.clone()).await {
                    log::error!("Failed to persist tool '{}': {}", tool.name, e);
                }
            }
            
            // Register prompts
            let prompt_service = crate::prompt::persistence::PromptPersistenceService::new((*client).clone());
            for prompt_data in &discovered_prompts {
                // Fetch template from plugin
                let template = match plugin.call::<Json<serde_json::Value>, String>(
                    "mcp_get_prompt_template",
                    Json(json!({ "id": prompt_data.id })),
                ) {
                    Ok(tmpl) => tmpl,
                    Err(_) => {
                        log::warn!("Failed to get template for prompt '{}'", prompt_data.id);
                        continue;
                    }
                };
                
                if let Err(e) = prompt_service.register_prompt(
                    prompt_data,
                    plugin_name.clone(),
                    template,
                ).await {
                    log::error!("Failed to persist prompt '{}': {}", prompt_data.id, e);
                }
            }
        }

        // Store the plugin itself using lock-free DashMap
        manager.plugins.insert(plugin_name.clone(), plugin);
        log::info!("Loaded plugin {} successfully", plugin_name);
    }

    manager
}
