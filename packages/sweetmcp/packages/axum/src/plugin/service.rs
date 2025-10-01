use crate::db::{Dao, DatabaseClient};
use crate::db::dao::entities::PluginEntity;
use crate::config::PluginConfig;
use sha2::{Digest, Sha256};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use futures::StreamExt;

#[derive(Clone)]
pub struct PluginService {
    dao: Dao<PluginEntity>,
}

impl PluginService {
    pub fn new(client: DatabaseClient) -> Self {
        Self {
            dao: Dao::new(client),
        }
    }
    
    /// Register plugin after successful load (upsert pattern)
    pub async fn register_plugin(
        &self,
        config: &PluginConfig,
        wasm_bytes: &[u8],
        metadata: PluginMetadata,
    ) -> Result<PluginEntity, String> {
        // Compute WASM hash
        let mut hasher = Sha256::new();
        hasher.update(wasm_bytes);
        let wasm_hash = hex::encode(hasher.finalize());
        
        // Check if plugin exists
        if let Some(existing) = self.find_by_name(&config.name).await {
            // Update existing
            let mut updated = existing;
            updated.source_path = config.path.clone();
            updated.wasm_hash = wasm_hash;
            updated.status = "active".to_string();
            updated.error_message = None;
            updated.metadata = Some(serde_json::to_value(metadata).unwrap_or_default());
            updated.updated_at = Utc::now();
            updated.last_loaded_at = Some(Utc::now());
            
            self.dao.update(&updated).await
                .ok_or_else(|| format!("Failed to update plugin '{}'", config.name))
        } else {
            // Create new
            let mut entity = PluginEntity::from_config(config, wasm_hash);
            entity.metadata = Some(serde_json::to_value(metadata).unwrap_or_default());
            
            self.dao.create(&mut entity).await
        }
    }
    
    /// Mark plugin as error
    pub async fn mark_plugin_error(&self, name: &str, error: &str) -> Result<(), String> {
        if let Some(mut entity) = self.find_by_name(name).await {
            entity.status = "error".to_string();
            entity.error_message = Some(error.to_string());
            entity.updated_at = Utc::now();
            
            self.dao.update(&entity).await
                .ok_or_else(|| format!("Failed to update plugin '{}' error status", name))?;
        }
        Ok(())
    }
    
    /// Find plugin by name
    pub async fn find_by_name(&self, name: &str) -> Option<PluginEntity> {
        let stream = self.dao.find_by_field("name", name).await;
        futures::pin_mut!(stream);
        stream.next().await
    }
    
    /// List all active plugins
    pub async fn list_active(&self) -> Vec<PluginEntity> {
        let stream = self.dao.find_by_field("status", "active").await;
        futures::pin_mut!(stream);
        stream.collect().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub tools_count: usize,
    pub prompts_count: usize,
}
