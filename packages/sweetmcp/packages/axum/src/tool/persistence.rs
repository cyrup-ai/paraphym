use crate::db::{Dao, DatabaseClient};
use crate::db::dao::entities::ToolEntity;
use crate::types::Tool;
use chrono::Utc;
use futures::StreamExt;

#[derive(Clone)]
pub struct ToolPersistenceService {
    dao: Dao<ToolEntity>,
}

impl ToolPersistenceService {
    pub fn new(client: DatabaseClient) -> Self {
        Self {
            dao: Dao::new(client),
        }
    }
    
    /// Register tool during plugin load (upsert)
    pub async fn register_tool(
        &self,
        tool: &Tool,
        plugin_name: String,
    ) -> Result<ToolEntity, String> {
        if let Some(mut existing) = self.find_by_name(&tool.name).await {
            // Update
            existing.description = tool.description.clone();
            existing.input_schema = serde_json::to_value(&tool.input_schema)
                .unwrap_or(serde_json::Value::Null);
            existing.updated_at = Utc::now();
            
            self.dao.update(&existing).await
                .ok_or_else(|| format!("Failed to update tool '{}'", tool.name))
        } else {
            // Create
            let mut entity = ToolEntity::from_tool(tool, plugin_name);
            self.dao.create(&mut entity).await
        }
    }
    
    /// Record tool call with timing (non-blocking async)
    pub async fn record_tool_call(&self, tool_name: &str, duration_ms: f64) -> Result<(), String> {
        if let Some(mut entity) = self.find_by_name(tool_name).await {
            entity.record_call(duration_ms);
            
            self.dao.update(&entity).await
                .ok_or_else(|| format!("Failed to update tool '{}' statistics", tool_name))?;
        }
        Ok(())
    }
    
    /// Find tool by name
    pub async fn find_by_name(&self, name: &str) -> Option<ToolEntity> {
        let stream = self.dao.find_by_field("name", name).await;
        futures::pin_mut!(stream);
        stream.next().await
    }
    
    /// Search tools by tag
    pub async fn search_by_tag(&self, tag: &str) -> Vec<ToolEntity> {
        let stream = self.dao.find().await;
        futures::pin_mut!(stream);
        
        stream
            .filter(|tool| {
                let contains = tool.tags.iter().any(|t| t == tag);
                futures::future::ready(contains)
            })
            .collect()
            .await
    }
    
    /// Get popular tools (sorted by call_count)
    pub async fn get_popular_tools(&self, limit: usize) -> Vec<ToolEntity> {
        let stream = self.dao.find().await;
        futures::pin_mut!(stream);
        
        let mut tools: Vec<ToolEntity> = stream.collect().await;
        tools.sort_by(|a, b| b.call_count.cmp(&a.call_count));
        tools.truncate(limit);
        tools
    }
}
