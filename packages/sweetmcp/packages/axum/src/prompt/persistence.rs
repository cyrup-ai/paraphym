use crate::db::{Dao, DatabaseClient};
use crate::db::dao::entities::PromptEntity;
use crate::types::Prompt;
use futures::StreamExt;

#[derive(Clone)]
pub struct PromptPersistenceService {
    dao: Dao<PromptEntity>,
}

impl PromptPersistenceService {
    pub fn new(client: DatabaseClient) -> Self {
        Self {
            dao: Dao::new(client),
        }
    }
    
    /// Register prompt during plugin load (upsert with version tracking)
    pub async fn register_prompt(
        &self,
        prompt: &Prompt,
        plugin_name: String,
        template: String,
    ) -> Result<PromptEntity, String> {
        if let Some(mut existing) = self.find_by_prompt_id(&prompt.id).await {
            // Check if template changed
            if existing.template != template {
                existing.update_template(template);
                
                self.dao.update(&existing).await
                    .ok_or_else(|| format!("Failed to update prompt '{}'", prompt.id))
            } else {
                Ok(existing)
            }
        } else {
            // Create
            let mut entity = PromptEntity::from_prompt(prompt, plugin_name, template);
            self.dao.create(&mut entity).await
        }
    }
    
    /// Record prompt usage
    pub async fn record_prompt_use(&self, prompt_id: &str) -> Result<(), String> {
        if let Some(mut entity) = self.find_by_prompt_id(prompt_id).await {
            entity.record_use();
            
            self.dao.update(&entity).await
                .ok_or_else(|| format!("Failed to update prompt '{}' statistics", prompt_id))?;
        }
        Ok(())
    }
    
    /// Find prompt by prompt_id
    pub async fn find_by_prompt_id(&self, prompt_id: &str) -> Option<PromptEntity> {
        let stream = self.dao.find_by_field("prompt_id", prompt_id).await;
        futures::pin_mut!(stream);
        stream.next().await
    }
    
    /// Search prompts by tag
    pub async fn search_by_tag(&self, tag: &str) -> Vec<PromptEntity> {
        let stream = self.dao.find().await;
        futures::pin_mut!(stream);
        
        stream
            .filter(|prompt| {
                let contains = prompt.tags.iter().any(|t| t == tag);
                futures::future::ready(contains)
            })
            .collect()
            .await
    }
    
    /// Get total prompt count
    #[allow(dead_code)] // Public API endpoint - will be wired to HTTP handler
    pub async fn count_prompts(&self) -> usize {
        let stream = self.dao.find().await;
        futures::pin_mut!(stream);
        let prompts: Vec<PromptEntity> = stream.collect().await;
        prompts.len()
    }
    
    /// Get popular prompts (sorted by use_count)
    pub async fn get_popular_prompts(&self, limit: usize) -> Vec<PromptEntity> {
        let stream = self.dao.find().await;
        futures::pin_mut!(stream);
        
        let mut prompts: Vec<PromptEntity> = stream.collect().await;
        prompts.sort_by(|a, b| b.use_count.cmp(&a.use_count));
        prompts.truncate(limit);
        prompts
    }
}
