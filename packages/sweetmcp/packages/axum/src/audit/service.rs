use crate::db::{Dao, DatabaseClient};
use crate::db::dao::entities::AuditLog;
use chrono::Utc;
use futures::StreamExt;

#[derive(Clone)]
pub struct AuditService {
    dao: Dao<AuditLog>,
}

impl AuditService {
    pub fn new(client: DatabaseClient) -> Self {
        Self {
            dao: Dao::new(client),
        }
    }
    
    /// Record operation (non-blocking)
    pub fn record(
        &self,
        operation: &str,
        entity_type: &str,
        entity_id: &str,
        actor: &str,
        old_values: Option<serde_json::Value>,
        new_values: Option<serde_json::Value>,
    ) {
        let mut log = AuditLog {
            id: None,
            operation: operation.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            actor: actor.to_string(),
            old_values,
            new_values,
            created_at: Utc::now(),
        };
        
        let dao = self.dao.clone();
        tokio::spawn(async move {
            if let Err(e) = dao.create(&mut log).await {
                log::error!("Failed to create audit log: {}", e);
            }
        });
    }
    
    /// Get entity history
    pub async fn get_entity_history(&self, entity_id: &str) -> Vec<AuditLog> {
        let stream = self.dao.find_by_field("entity_id", entity_id).await;
        futures::pin_mut!(stream);
        stream.collect().await
    }
}
