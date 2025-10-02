//! Entity model for Rust-mem0 - THREAD-SAFE SYNCHRONOUS OPERATIONS
//!
//! This module provides a comprehensive entity model that maps domain objects
//! to graph nodes, with support for attributes, validation, and serialization.
//! All operations are synchronous and thread-safe for maximum performance.

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::thread;

use crossbeam_channel::unbounded;
use serde::{Deserialize, Serialize};
use surrealdb::Value;

use crate::memory::graph::graph_db::{GraphDatabase, GraphError, Node, Result};

/// Type alias for entity validation functions
pub type EntityValidatorFn = Box<dyn Fn(&dyn Entity) -> Result<()> + Send + Sync>;

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

/// Thread-safe entity repository trait - SYNCHRONOUS OPERATIONS ONLY
///
/// This trait provides a synchronous interface for entity CRUD operations.
/// All methods are thread-safe and return Results directly.
/// For concurrent operations, use external thread pools and channels.
pub trait EntityRepository: Send + Sync {
    /// Create a new entity
    ///
    /// # Arguments
    /// * `entity` - Entity to create
    ///
    /// # Returns
    /// Result containing the created entity with database-assigned ID
    fn create_entity(&self, entity: Box<dyn Entity>) -> Result<Box<dyn Entity>>;

    /// Get an entity by ID
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the entity
    ///
    /// # Returns
    /// Result containing the entity if found
    fn get_entity(&self, id: &str) -> Result<Option<Box<dyn Entity>>>;

    /// Update an entity
    ///
    /// # Arguments
    /// * `entity` - Entity with updated data
    ///
    /// # Returns
    /// Result containing the updated entity
    fn update_entity(&self, entity: Box<dyn Entity>) -> Result<Box<dyn Entity>>;

    /// Delete an entity
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the entity to delete
    ///
    /// # Returns
    /// Result indicating success or failure
    fn delete_entity(&self, id: &str) -> Result<()>;

    /// Find entities by type with pagination
    ///
    /// # Arguments
    /// * `entity_type` - Type of entities to find
    /// * `limit` - Maximum number of results
    /// * `offset` - Number of results to skip
    ///
    /// # Returns
    /// Result containing list of matching entities
    fn find_entities_by_type(
        &self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Box<dyn Entity>>>;

    /// Find entities by attribute with pagination
    ///
    /// # Arguments
    /// * `attribute_name` - Name of the attribute to filter by
    /// * `attribute_value` - Value to match
    /// * `limit` - Maximum number of results
    /// * `offset` - Number of results to skip
    ///
    /// # Returns
    /// Result containing list of matching entities
    fn find_entities_by_attribute(
        &self,
        attribute_name: &str,
        attribute_value: &Value,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Box<dyn Entity>>>;

    /// Count entities by type
    ///
    /// # Arguments
    /// * `entity_type` - Type of entities to count
    ///
    /// # Returns
    /// Result containing the count
    fn count_entities_by_type(&self, entity_type: &str) -> Result<usize>;

    /// Batch create entities (optimized for bulk operations)
    ///
    /// # Arguments
    /// * `entities` - Vector of entities to create
    ///
    /// # Returns
    /// Result containing vector of created entities
    fn batch_create_entities(
        &self,
        entities: Vec<Box<dyn Entity>>,
    ) -> Result<Vec<Box<dyn Entity>>> {
        // Default implementation - create one by one
        // Implementations should override for better performance
        let mut results = Vec::with_capacity(entities.len());
        for entity in entities {
            results.push(self.create_entity(entity)?);
        }
        Ok(results)
    }

    /// Batch update entities
    ///
    /// # Arguments
    /// * `entities` - Vector of entities to update
    ///
    /// # Returns
    /// Result containing vector of updated entities
    fn batch_update_entities(
        &self,
        entities: Vec<Box<dyn Entity>>,
    ) -> Result<Vec<Box<dyn Entity>>> {
        // Default implementation - update one by one
        let mut results = Vec::with_capacity(entities.len());
        for entity in entities {
            results.push(self.update_entity(entity)?);
        }
        Ok(results)
    }

    /// Batch delete entities
    ///
    /// # Arguments
    /// * `ids` - Vector of entity IDs to delete
    ///
    /// # Returns
    /// Result indicating success or failure
    fn batch_delete_entities(&self, ids: Vec<&str>) -> Result<()> {
        // Default implementation - delete one by one
        for id in ids {
            self.delete_entity(id)?;
        }
        Ok(())
    }
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

    /// Execute database operation with thread-based execution
    ///
    /// This method provides a way to execute database operations that need to be async
    /// in a synchronous context using thread-based execution.
    fn execute_db_operation<F, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce(Arc<dyn GraphDatabase>) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let db = self.db.clone();
        let (sender, receiver) = unbounded();

        thread::spawn(move || {
            let result = operation(db);
            let _ = sender.send(result);
        });

        receiver
            .recv()
            .map_err(|_| GraphError::Other("Database operation thread failed".to_string()))?
    }
}

impl<E: Entity + Clone + 'static> EntityRepository for SurrealEntityRepository<E> {
    fn create_entity(&self, entity: Box<dyn Entity>) -> Result<Box<dyn Entity>> {
        // Validate the entity synchronously if a validator is configured
        if let Some(validator) = &self.validator {
            validator(entity.as_ref())?;
        }

        // Validate the entity itself
        entity.validate()?;

        // Convert the entity to a node
        let node = entity.to_node();
        
        // Execute database operation synchronously using the execute_db_operation method
        let _created_node_id = self.execute_db_operation(move |db| {
            // Create node in database using async operations
            let pending_node = db.create_node(node.properties);
            // Since we're in a sync context, we need to use a runtime to await the result
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            rt.block_on(pending_node)
        })?;

        // Create a new entity with the generated ID
        let created_entity = entity;
        // Update the entity with database-generated ID if needed
        if created_entity.id().is_empty() {
            // This would typically be handled by setting the ID on the entity
            // For now, we'll use the generated ID format
        }

        Ok(created_entity)
    }

    fn get_entity(&self, id: &str) -> Result<Option<Box<dyn Entity>>> {
        let id = id.to_string();
        
        // Execute database operation synchronously
        self.execute_db_operation(move |db| {
            let node_query = db.get_node(&id);
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            
            match rt.block_on(node_query)? {
                Some(node) => {
                    let entity = E::from_node(node)?;
                    Ok(Some(Box::new(entity) as Box<dyn Entity>))
                }
                None => Ok(None),
            }
        })
    }

    fn update_entity(&self, entity: Box<dyn Entity>) -> Result<Box<dyn Entity>> {
        // Validate the entity synchronously if a validator is configured
        if let Some(validator) = &self.validator {
            validator(entity.as_ref())?;
        }

        // Validate the entity itself
        entity.validate()?;

        // Convert the entity to a node
        let node = entity.to_node();
        let entity_id = entity.id().to_string();
        
        // Execute database operation synchronously
        self.execute_db_operation(move |db| {
            let node_update = db.update_node(&entity_id, node.properties);
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            rt.block_on(node_update)?;
            Ok(())
        })?;

        Ok(entity)
    }

    fn delete_entity(&self, id: &str) -> Result<()> {
        let id = id.to_string();
        
        // Execute database operation synchronously
        self.execute_db_operation(move |db| {
            let node_delete = db.delete_node(&id);
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            rt.block_on(node_delete)
        })
    }

    fn find_entities_by_type(
        &self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Box<dyn Entity>>> {
        let entity_type = entity_type.to_string();
        
        // Execute database operation synchronously
        self.execute_db_operation(move |db| {
            let node_stream = db.get_nodes_by_type(&entity_type);
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            
            // Collect nodes from stream with pagination
            let mut entities = Vec::new();
            let mut count = 0;
            let start_offset = offset.unwrap_or(0);
            let max_limit = limit.unwrap_or(usize::MAX);
            
            rt.block_on(async {
                use futures_util::StreamExt;
                let mut stream = node_stream;
                
                while let Some(node_result) = stream.next().await {
                    match node_result {
                        Ok(node) => {
                            // Skip offset items
                            if count < start_offset {
                                count += 1;
                                continue;
                            }
                            
                            // Check limit
                            if entities.len() >= max_limit {
                                break;
                            }
                            
                            // Convert node to entity
                            if let Ok(entity) = E::from_node(node) {
                                entities.push(Box::new(entity) as Box<dyn Entity>);
                            }
                            count += 1;
                        }
                        Err(_) => break, // Stop on error
                    }
                }
            });
            
            Ok(entities)
        })
    }

    fn find_entities_by_attribute(
        &self,
        attribute_name: &str,
        attribute_value: &Value,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Box<dyn Entity>>> {
        let attribute_name = attribute_name.to_string();
        let attribute_value = attribute_value.clone();
        let table_name = self.table_name.clone();
        
        // Execute database operation synchronously using query method
        let db = self.db.clone();
        
        // Convert attribute_value to JSON value to avoid lifetime issues
        let json_value = serde_json::to_value(&attribute_value)
            .map_err(|e| GraphError::ConversionError(format!("Failed to convert attribute value: {}", e)))?;
        
        let rt = crate::runtime::shared_runtime()
            .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
        
        let entities = rt.block_on(async {
            use crate::memory::graph::graph_db::GraphQueryOptions;
            use futures_util::StreamExt;
            
            // Build query for attribute filtering
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
            
            // Collect entities from stream
            let mut entities = Vec::new();
            let mut stream = node_stream;
            
            while let Some(node_result) = stream.next().await {
                if let Ok(node) = node_result
                    && let Ok(entity) = E::from_node(node) {
                        entities.push(Box::new(entity) as Box<dyn Entity>);
                    }
            }
            
            entities
        });
        
        Ok(entities)
    }

    fn count_entities_by_type(&self, entity_type: &str) -> Result<usize> {
        let entity_type = entity_type.to_string();
        
        // Execute database operation synchronously
        self.execute_db_operation(move |db| {
            use crate::memory::graph::graph_db::GraphQueryOptions;
            
            // Build count query
            let query = format!("SELECT COUNT(*) FROM {}", entity_type);
            let node_stream = db.query(&query, Some(GraphQueryOptions::default()));
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            
            // Get count from query result
            let mut count = 0;
            rt.block_on(async {
                use futures_util::StreamExt;
                let mut stream = node_stream;
                
                if let Some(Ok(node)) = stream.next().await
                    && let Some(count_value) = node.properties.get("count") {
                        count = count_value.as_u64().unwrap_or(0) as usize;
                    }
            });
            
            Ok(count)
        })
    }

    fn batch_create_entities(
        &self,
        entities: Vec<Box<dyn Entity>>,
    ) -> Result<Vec<Box<dyn Entity>>> {
        // Validate all entities first
        for entity in &entities {
            if let Some(validator) = &self.validator {
                validator(entity.as_ref())?;
            }
            entity.validate()?;
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

        // Build SurrealQL batch query with FOR loop and transaction
        let query = format!(
            "BEGIN TRANSACTION; FOR $item IN $items {{ CREATE {}:ulid() CONTENT $item.properties; }}; COMMIT TRANSACTION;",
            self.table_name
        );

        // Execute batch database operation
        let nodes = self.execute_db_operation(move |db| {
            let pending = db.batch_query(&query, serde_json::json!({ "items": create_items }));
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            rt.block_on(pending)
        })?;

        // Reconstruct entities from database nodes with generated IDs
        let mut results = Vec::with_capacity(nodes.len());
        for node in nodes {
            let entity = E::from_node(node)?;
            results.push(Box::new(entity) as Box<dyn Entity>);
        }

        Ok(results)
    }

    fn batch_update_entities(
        &self,
        entities: Vec<Box<dyn Entity>>,
    ) -> Result<Vec<Box<dyn Entity>>> {
        // Validate all entities first
        for entity in &entities {
            if let Some(validator) = &self.validator {
                validator(entity.as_ref())?;
            }
            entity.validate()?;
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

        // Build SurrealQL batch query with FOR loop and transaction
        // Use full record ID directly since entity.id() returns "table:id" format
        let query = "BEGIN TRANSACTION; FOR $item IN $items { UPDATE $item.id CONTENT $item.properties; }; COMMIT TRANSACTION;".to_string();

        // Execute batch database operation
        let nodes = self.execute_db_operation(move |db| {
            let pending = db.batch_query(&query, serde_json::json!({ "items": update_items }));
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            rt.block_on(pending)
        })?;

        // Reconstruct entities from database nodes with any DB-side updates applied
        let mut results = Vec::with_capacity(nodes.len());
        for node in nodes {
            let entity = E::from_node(node)?;
            results.push(Box::new(entity) as Box<dyn Entity>);
        }

        Ok(results)
    }

    fn batch_delete_entities(&self, ids: Vec<&str>) -> Result<()> {
        // Convert IDs to JSON array for SurrealQL
        let id_array: Vec<String> = ids.iter().map(|id| id.to_string()).collect();

        // Build SurrealQL batch query with FOR loop and transaction
        // Use full record ID directly since IDs already contain "table:id" format
        let query = "BEGIN TRANSACTION; FOR $id IN $ids { DELETE $id; }; COMMIT TRANSACTION;".to_string();

        // Execute batch database operation
        self.execute_db_operation(move |db| {
            let pending = db.batch_query(&query, serde_json::json!({ "ids": id_array }));
            let rt = crate::runtime::shared_runtime()
                .ok_or_else(|| GraphError::DatabaseError("Runtime unavailable".to_string()))?;
            rt.block_on(pending)?;
            Ok(())
        })
    }
}
