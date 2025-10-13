//! Entity model for Rust-mem0 - THREAD-SAFE SYNCHRONOUS OPERATIONS
//!
//! This module provides a comprehensive entity model that maps domain objects
//! to graph nodes, with support for attributes, validation, and serialization.
//! All operations are synchronous and thread-safe for maximum performance.

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::Value;
use tokio::sync::oneshot;

use crate::memory::graph::graph_db::{GraphDatabase, GraphError, Node, Result};

/// Type alias for entity validation functions
pub type EntityValidatorFn = Box<dyn Fn(&dyn Entity) -> Result<()> + Send + Sync>;

/// Future wrapper for entity creation/update operations
pub struct PendingEntity {
    rx: oneshot::Receiver<Result<Box<dyn Entity>>>,
}

impl PendingEntity {
    pub fn new(rx: oneshot::Receiver<Result<Box<dyn Entity>>>) -> Self {
        Self { rx }
    }
}

impl std::future::Future for PendingEntity {
    type Output = Result<Box<dyn Entity>>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match std::pin::Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => std::task::Poll::Ready(Err(GraphError::Other("Channel closed".to_string()))),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Future wrapper for entity retrieval operations
pub struct PendingEntityOption {
    rx: oneshot::Receiver<Result<Option<Box<dyn Entity>>>>,
}

impl PendingEntityOption {
    pub fn new(rx: oneshot::Receiver<Result<Option<Box<dyn Entity>>>>) -> Self {
        Self { rx }
    }
}

impl std::future::Future for PendingEntityOption {
    type Output = Result<Option<Box<dyn Entity>>>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match std::pin::Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => std::task::Poll::Ready(Err(GraphError::Other("Channel closed".to_string()))),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Future wrapper for entity list operations
pub struct PendingEntityList {
    rx: oneshot::Receiver<Result<Vec<Box<dyn Entity>>>>,
}

impl PendingEntityList {
    pub fn new(rx: oneshot::Receiver<Result<Vec<Box<dyn Entity>>>>) -> Self {
        Self { rx }
    }
}

impl std::future::Future for PendingEntityList {
    type Output = Result<Vec<Box<dyn Entity>>>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match std::pin::Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => std::task::Poll::Ready(Err(GraphError::Other("Channel closed".to_string()))),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Future wrapper for entity count operations
pub struct PendingEntityCount {
    rx: oneshot::Receiver<Result<usize>>,
}

impl PendingEntityCount {
    pub fn new(rx: oneshot::Receiver<Result<usize>>) -> Self {
        Self { rx }
    }
}

impl std::future::Future for PendingEntityCount {
    type Output = Result<usize>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match std::pin::Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => std::task::Poll::Ready(Err(GraphError::Other("Channel closed".to_string()))),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Future wrapper for unit operations (delete, etc.)
pub struct PendingUnit {
    rx: oneshot::Receiver<Result<()>>,
}

impl PendingUnit {
    pub fn new(rx: oneshot::Receiver<Result<()>>) -> Self {
        Self { rx }
    }
}

impl std::future::Future for PendingUnit {
    type Output = Result<()>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match std::pin::Pin::new(&mut self.rx).poll(cx) {
            std::task::Poll::Ready(Ok(result)) => std::task::Poll::Ready(result),
            std::task::Poll::Ready(Err(_)) => std::task::Poll::Ready(Err(GraphError::Other("Channel closed".to_string()))),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/// Entity trait for domain objects
pub trait Entity: Send + Sync + Debug {
    /// Get the entity ID
    fn id(&self) -> &str;

    /// Get the entity type
    fn entity_type(&self) -> &str;

    /// Get an attribute value
    fn get_attribute(&self, name: &str) -> Option<&Value>;

    /// Set an attribute value
    fn set_attribute(&mut self, name: &str, value: Value);

    /// Get all attributes
    fn attributes(&self) -> &HashMap<String, Value>;

    /// Validate the entity
    fn validate(&self) -> Result<()>;

    /// Convert to a graph node
    fn to_node(&self) -> Node;

    /// Create from a graph node
    fn from_node(node: Node) -> Result<Self>
    where
        Self: Sized;
}

/// Base entity implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    id: String,
    entity_type: String,
    attributes: HashMap<String, Value>,
}

impl BaseEntity {
    /// Create a new base entity
    pub fn new(id: String, entity_type: String) -> Self {
        Self {
            id,
            entity_type,
            attributes: HashMap::new(),
        }
    }

    /// Create with pre-allocated capacity for attributes
    pub fn with_capacity(id: String, entity_type: String, capacity: usize) -> Self {
        Self {
            id,
            entity_type,
            attributes: HashMap::with_capacity(capacity),
        }
    }

    /// Get memory usage in bytes (approximate)
    pub fn memory_usage(&self) -> usize {
        self.id.len() + self.entity_type.len() + (self.attributes.len() * 64) // Approximate
    }

    /// Bulk set attributes with validation
    pub fn set_attributes_bulk(&mut self, attributes: HashMap<String, Value>) -> Result<()> {
        // Validate all attributes first
        for (key, value) in &attributes {
            self.validate_attribute(key, value)?;
        }

        // If all valid, set them
        self.attributes.extend(attributes);
        Ok(())
    }

    /// Validate a single attribute (can be overridden)
    pub fn validate_attribute(&self, _key: &str, _value: &Value) -> Result<()> {
        // Default validation - accept all
        Ok(())
    }

    /// Remove multiple attributes
    pub fn remove_attributes(&mut self, keys: &[&str]) {
        for key in keys {
            self.attributes.remove(*key);
        }
    }

    /// Check if entity has all required attributes
    pub fn has_required_attributes(&self, required: &[&str]) -> bool {
        required
            .iter()
            .all(|key| self.attributes.contains_key(*key))
    }
}

impl Entity for BaseEntity {
    fn id(&self) -> &str {
        &self.id
    }

    fn entity_type(&self) -> &str {
        &self.entity_type
    }

    fn get_attribute(&self, name: &str) -> Option<&Value> {
        self.attributes.get(name)
    }

    fn set_attribute(&mut self, name: &str, value: Value) {
        self.attributes.insert(name.to_string(), value);
    }

    fn attributes(&self) -> &HashMap<String, Value> {
        &self.attributes
    }

    fn validate(&self) -> Result<()> {
        // Basic validation - ensure ID and type are not empty
        if self.id.is_empty() {
            return Err(GraphError::ValidationError(
                "Entity ID cannot be empty".to_string(),
            ));
        }

        if self.entity_type.is_empty() {
            return Err(GraphError::ValidationError(
                "Entity type cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    fn to_node(&self) -> Node {
        // Convert surrealdb::Value attributes to serde_json::Value for Node compatibility
        let mut properties = std::collections::HashMap::new();
        for (key, value) in &self.attributes {
            // Convert surrealdb::Value to serde_json::Value via serialization
            let json_value = serde_json::to_value(value)
                .unwrap_or_else(|_| serde_json::Value::String(format!("{:?}", value)));
            properties.insert(key.clone(), json_value);
        }
        
        properties.insert("id".to_string(), serde_json::Value::String(self.id.clone()));
        properties.insert(
            "entity_type".to_string(),
            serde_json::Value::String(self.entity_type.clone()),
        );

        Node {
            id: self.id.clone(),
            properties,
            labels: vec![self.entity_type.clone()],
        }
    }

    fn from_node(node: Node) -> Result<Self> {
        let id = node
            .properties
            .get("id")
            .and_then(|v| {
                // Try to deserialize as String from SurrealDB Value
                v.as_str().map(|s| s.to_string())
            })
            .ok_or_else(|| GraphError::ValidationError("Node missing ID".to_string()))?;

        let entity_type = node
            .properties
            .get("entity_type")
            .and_then(|v| {
                // Try to deserialize as String from SurrealDB Value
                v.as_str().map(|s| s.to_string())
            })
            .ok_or_else(|| GraphError::ValidationError("Node missing entity_type".to_string()))?;

        // Convert serde_json::Value to surrealdb::Value for attributes
        let mut attributes = HashMap::new();
        for (key, value) in node.properties {
            if key != "id" && key != "entity_type" {
                // Convert serde_json::Value to surrealdb::Value using proper conversion
                let surreal_value = surrealdb::value::to_value(value).unwrap_or_default();

                attributes.insert(key, surreal_value);
            }
        }

        Ok(Self {
            id,
            entity_type,
            attributes,
        })
    }
}

impl BaseEntity {
    /// Builder pattern method to set an attribute and return self
    #[must_use]
    pub fn with_attribute(mut self, name: &str, value: Value) -> Self {
        self.attributes.insert(name.to_string(), value);
        self
    }
}

/// Thread-safe entity repository trait - Returns Futures
///
/// This trait provides an interface for entity CRUD operations that return futures.
/// All methods are thread-safe and return pending wrappers that can be awaited.
/// This pattern matches the rest of the memory system.
pub trait EntityRepository: Send + Sync {
    /// Create a new entity
    ///
    /// # Arguments
    /// * `entity` - Entity to create
    ///
    /// # Returns
    /// PendingEntity that resolves to the created entity with database-assigned ID
    fn create_entity(&self, entity: Box<dyn Entity>) -> PendingEntity;

    /// Get an entity by ID
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the entity
    ///
    /// # Returns
    /// PendingEntityOption that resolves to the entity if found
    fn get_entity(&self, id: &str) -> PendingEntityOption;

    /// Update an entity
    ///
    /// # Arguments
    /// * `entity` - Entity with updated data
    ///
    /// # Returns
    /// PendingEntity that resolves to the updated entity
    fn update_entity(&self, entity: Box<dyn Entity>) -> PendingEntity;

    /// Delete an entity
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the entity to delete
    ///
    /// # Returns
    /// PendingUnit that resolves when deletion is complete
    fn delete_entity(&self, id: &str) -> PendingUnit;

    /// Find entities by type with pagination
    ///
    /// # Arguments
    /// * `entity_type` - Type of entities to find
    /// * `limit` - Maximum number of results
    /// * `offset` - Number of results to skip
    ///
    /// # Returns
    /// PendingEntityList that resolves to list of matching entities
    fn find_entities_by_type(
        &self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> PendingEntityList;

    /// Find entities by attribute with pagination
    ///
    /// # Arguments
    /// * `attribute_name` - Name of the attribute to filter by
    /// * `attribute_value` - Value to match
    /// * `limit` - Maximum number of results
    /// * `offset` - Number of results to skip
    ///
    /// # Returns
    /// PendingEntityList that resolves to list of matching entities
    fn find_entities_by_attribute(
        &self,
        attribute_name: &str,
        attribute_value: &Value,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> PendingEntityList;

    /// Count entities by type
    ///
    /// # Arguments
    /// * `entity_type` - Type of entities to count
    ///
    /// # Returns
    /// PendingEntityCount that resolves to the count
    fn count_entities_by_type(&self, entity_type: &str) -> PendingEntityCount;

    /// Batch create entities (optimized for bulk operations)
    ///
    /// # Arguments
    /// * `entities` - Vector of entities to create
    ///
    /// # Returns
    /// PendingEntityList that resolves to vector of created entities
    fn batch_create_entities(&self, entities: Vec<Box<dyn Entity>>) -> PendingEntityList;

    /// Batch update entities
    ///
    /// # Arguments
    /// * `entities` - Vector of entities to update
    ///
    /// # Returns
    /// PendingEntityList that resolves to vector of updated entities
    fn batch_update_entities(&self, entities: Vec<Box<dyn Entity>>) -> PendingEntityList;

    /// Batch delete entities
    ///
    /// # Arguments
    /// * `ids` - Vector of entity IDs to delete
    ///
    /// # Returns
    /// PendingUnit that resolves when deletion is complete
    fn batch_delete_entities(&self, ids: Vec<&str>) -> PendingUnit;
}

/// SurrealDB-backed entity repository implementation
pub struct SurrealEntityRepository<E: Entity + Clone + 'static> {
    db: Arc<dyn GraphDatabase>,
    table_name: String,
    validator: Option<EntityValidatorFn>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E: Entity + Clone + 'static> SurrealEntityRepository<E> {
    /// Create a new SurrealDB entity repository
    ///
    /// # Arguments
    /// * `db` - Graph database connection
    /// * `table_name` - Name of the table/collection for this entity type
    ///
    /// # Returns
    /// New repository instance
    pub fn new(db: Arc<dyn GraphDatabase>, table_name: String) -> Self {
        Self {
            db,
            table_name,
            validator: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create with custom validation
    ///
    /// # Arguments
    /// * `db` - Graph database connection
    /// * `table_name` - Name of the table/collection
    /// * `validator` - Custom validation function
    ///
    /// # Returns
    /// New repository instance with validation
    pub fn with_validator(
        db: Arc<dyn GraphDatabase>,
        table_name: String,
        validator: EntityValidatorFn,
    ) -> Self {
        Self {
            db,
            table_name,
            validator: Some(validator),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set or update the validator function
    pub fn set_validator(&mut self, validator: EntityValidatorFn) {
        self.validator = Some(validator);
    }

    /// Remove the validator function
    pub fn remove_validator(&mut self) {
        self.validator = None;
    }

    /// Get the table name
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}

impl<E: Entity + Clone + 'static> EntityRepository for SurrealEntityRepository<E> {
    fn create_entity(&self, entity: Box<dyn Entity>) -> PendingEntity {
        // Validate synchronously
        if let Some(validator) = &self.validator {
            if let Err(e) = validator(entity.as_ref()) {
                let (tx, rx) = oneshot::channel();
                let _ = tx.send(Err(e));
                return PendingEntity::new(rx);
            }
        }

        if let Err(e) = entity.validate() {
            let (tx, rx) = oneshot::channel();
            let _ = tx.send(Err(e));
            return PendingEntity::new(rx);
        }

        let node = entity.to_node();
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = db.create_node(node.properties).await
                .map_err(|e| GraphError::DatabaseError(e.to_string()))
                .map(|_| entity);
            let _ = tx.send(result);
        });

        PendingEntity::new(rx)
    }

    fn get_entity(&self, id: &str) -> PendingEntityOption {
        let id = id.to_string();
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = db.get_node(&id).await
                .map_err(|e| GraphError::DatabaseError(e.to_string()))
                .and_then(|opt| {
                    Ok(opt.and_then(|node| {
                        E::from_node(node).ok().map(|entity| Box::new(entity) as Box<dyn Entity>)
                    }))
                });
            let _ = tx.send(result);
        });

        PendingEntityOption::new(rx)
    }

    fn update_entity(&self, entity: Box<dyn Entity>) -> PendingEntity {
        // Validate synchronously
        if let Some(validator) = &self.validator {
            if let Err(e) = validator(entity.as_ref()) {
                let (tx, rx) = oneshot::channel();
                let _ = tx.send(Err(e));
                return PendingEntity::new(rx);
            }
        }

        if let Err(e) = entity.validate() {
            let (tx, rx) = oneshot::channel();
            let _ = tx.send(Err(e));
            return PendingEntity::new(rx);
        }

        let node = entity.to_node();
        let entity_id = entity.id().to_string();
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = db.update_node(&entity_id, node.properties).await
                .map_err(|e| GraphError::DatabaseError(e.to_string()))
                .map(|_| entity);
            let _ = tx.send(result);
        });

        PendingEntity::new(rx)
    }

    fn delete_entity(&self, id: &str) -> PendingUnit {
        let id = id.to_string();
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = db.delete_node(&id).await
                .map_err(|e| GraphError::DatabaseError(e.to_string()));
            let _ = tx.send(result);
        });

        PendingUnit::new(rx)
    }

    fn find_entities_by_type(
        &self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> PendingEntityList {
        let entity_type = entity_type.to_string();
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            use futures_util::StreamExt;

            let node_stream = db.get_nodes_by_type(&entity_type);
            let mut entities = Vec::new();
            let mut count = 0;
            let start_offset = offset.unwrap_or(0);
            let max_limit = limit.unwrap_or(usize::MAX);

            let mut stream = node_stream;
            while let Some(node_result) = stream.next().await {
                match node_result {
                    Ok(node) => {
                        if count < start_offset {
                            count += 1;
                            continue;
                        }

                        if entities.len() >= max_limit {
                            break;
                        }

                        if let Ok(entity) = E::from_node(node) {
                            entities.push(Box::new(entity) as Box<dyn Entity>);
                        }
                        count += 1;
                    }
                    Err(_) => break,
                }
            }

            let _ = tx.send(Ok(entities));
        });

        PendingEntityList::new(rx)
    }

    fn find_entities_by_attribute(
        &self,
        attribute_name: &str,
        attribute_value: &Value,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> PendingEntityList {
        let attribute_name = attribute_name.to_string();
        let attribute_value = attribute_value.clone();
        let table_name = self.table_name.clone();
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        // Convert attribute_value to JSON value
        let json_value = match serde_json::to_value(&attribute_value) {
            Ok(v) => v,
            Err(e) => {
                let _ = tx.send(Err(GraphError::ConversionError(format!("Failed to convert attribute value: {}", e))));
                return PendingEntityList::new(rx);
            }
        };

        tokio::spawn(async move {
            use crate::memory::graph::graph_db::GraphQueryOptions;
            use futures_util::StreamExt;

            let query = format!("SELECT * FROM {} WHERE {} = $value", table_name, attribute_name);
            let options = GraphQueryOptions {
                limit,
                offset,
                filters: {
                    let mut filters = std::collections::HashMap::new();
                    filters.insert("value".to_string(), json_value);
                    filters
                }
            };

            let node_stream = db.query(&query, Some(options));
            let mut entities = Vec::new();
            let mut stream = node_stream;

            while let Some(node_result) = stream.next().await {
                if let Ok(node) = node_result
                    && let Ok(entity) = E::from_node(node) {
                        entities.push(Box::new(entity) as Box<dyn Entity>);
                    }
            }

            let _ = tx.send(Ok(entities));
        });

        PendingEntityList::new(rx)
    }

    fn count_entities_by_type(&self, entity_type: &str) -> PendingEntityCount {
        let entity_type = entity_type.to_string();
        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            use crate::memory::graph::graph_db::GraphQueryOptions;
            use futures_util::StreamExt;

            let query = format!("SELECT COUNT(*) FROM {}", entity_type);
            let node_stream = db.query(&query, Some(GraphQueryOptions::default()));
            let mut stream = node_stream;
            let mut count = 0;

            if let Some(Ok(node)) = stream.next().await
                && let Some(count_value) = node.properties.get("count") {
                    count = count_value.as_u64().unwrap_or(0) as usize;
                }

            let _ = tx.send(Ok(count));
        });

        PendingEntityCount::new(rx)
    }

    fn batch_create_entities(
        &self,
        entities: Vec<Box<dyn Entity>>,
    ) -> PendingEntityList {
        // Validate all entities first
        for entity in &entities {
            if let Some(validator) = &self.validator {
                if let Err(e) = validator(entity.as_ref()) {
                    let (tx, rx) = oneshot::channel();
                    let _ = tx.send(Err(e));
                    return PendingEntityList::new(rx);
                }
            }
            if let Err(e) = entity.validate() {
                let (tx, rx) = oneshot::channel();
                let _ = tx.send(Err(e));
                return PendingEntityList::new(rx);
            }
        }

        // Convert entities to JSON array for SurrealQL
        let create_items: Vec<serde_json::Value> = entities
            .iter()
            .map(|entity| {
                let node = entity.to_node();
                serde_json::json!({
                    "properties": node.properties
                })
            })
            .collect();

        let query = format!(
            "BEGIN TRANSACTION; FOR $item IN $items {{ CREATE {}:ulid() CONTENT $item.properties; }}; COMMIT TRANSACTION;",
            self.table_name
        );

        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = db.batch_query(&query, serde_json::json!({ "items": create_items })).await
                .map_err(|e| GraphError::DatabaseError(e.to_string()))
                .and_then(|nodes| {
                    let mut results = Vec::with_capacity(nodes.len());
                    for node in nodes {
                        match E::from_node(node) {
                            Ok(entity) => results.push(Box::new(entity) as Box<dyn Entity>),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(results)
                });
            let _ = tx.send(result);
        });

        PendingEntityList::new(rx)
    }

    fn batch_update_entities(
        &self,
        entities: Vec<Box<dyn Entity>>,
    ) -> PendingEntityList {
        // Validate all entities first
        for entity in &entities {
            if let Some(validator) = &self.validator {
                if let Err(e) = validator(entity.as_ref()) {
                    let (tx, rx) = oneshot::channel();
                    let _ = tx.send(Err(e));
                    return PendingEntityList::new(rx);
                }
            }
            if let Err(e) = entity.validate() {
                let (tx, rx) = oneshot::channel();
                let _ = tx.send(Err(e));
                return PendingEntityList::new(rx);
            }
        }

        // Convert entities to JSON array with IDs for SurrealQL
        let update_items: Vec<serde_json::Value> = entities
            .iter()
            .map(|entity| {
                let node = entity.to_node();
                serde_json::json!({
                    "id": entity.id(),
                    "properties": node.properties
                })
            })
            .collect();

        let query = "BEGIN TRANSACTION; FOR $item IN $items { UPDATE $item.id CONTENT $item.properties; }; COMMIT TRANSACTION;".to_string();

        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = db.batch_query(&query, serde_json::json!({ "items": update_items })).await
                .map_err(|e| GraphError::DatabaseError(e.to_string()))
                .and_then(|nodes| {
                    let mut results = Vec::with_capacity(nodes.len());
                    for node in nodes {
                        match E::from_node(node) {
                            Ok(entity) => results.push(Box::new(entity) as Box<dyn Entity>),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(results)
                });
            let _ = tx.send(result);
        });

        PendingEntityList::new(rx)
    }

    fn batch_delete_entities(&self, ids: Vec<&str>) -> PendingUnit {
        let id_array: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let query = "BEGIN TRANSACTION; FOR $id IN $ids { DELETE $id; }; COMMIT TRANSACTION;".to_string();

        let db = self.db.clone();
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let result = db.batch_query(&query, serde_json::json!({ "ids": id_array })).await
                .map_err(|e| GraphError::DatabaseError(e.to_string()))
                .map(|_| ());
            let _ = tx.send(result);
        });

        PendingUnit::new(rx)
    }
}
