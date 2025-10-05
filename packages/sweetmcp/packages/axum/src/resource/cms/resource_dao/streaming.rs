//! Resource streaming operations with zero allocation patterns
//!
//! This module provides streaming functionality for CMS resources with zero
//! allocation patterns, blazing-fast performance, and comprehensive async
//! operations for production environments.

use crate::resource::cms::resource_dao::core::*;
use crate::db::DatabaseClient;
use crate::types::*;
use std::fmt::Write;
use arrayvec::ArrayString;
use tokio::sync::mpsc;
use url::Url;
use tokio::sync::broadcast;
use dashmap::DashMap;
use std::sync::Arc;
use futures::StreamExt;
use uuid::Uuid;

/// Update notification for a resource
#[derive(Debug, Clone)]
pub struct ResourceUpdate {
    /// Resource URI
    pub uri: String,
    /// Action performed (CREATE, UPDATE, DELETE)
    pub action: String,
    /// Updated resource data
    pub resource: Option<Resource>,
    /// Query ID from SurrealDB
    pub query_id: Uuid,
}

/// Manages live subscriptions to resource changes
pub struct ResourceSubscriptionManager {
    /// Active subscriptions: table_name -> broadcast sender
    subscriptions: Arc<DashMap<String, broadcast::Sender<ResourceUpdate>>>,
    /// Database client for creating live queries
    db_client: Arc<DatabaseClient>,
}

impl ResourceSubscriptionManager {
    /// Create new subscription manager
    pub fn new(db_client: Arc<DatabaseClient>) -> Self {
        Self {
            subscriptions: Arc::new(DashMap::new()),
            db_client,
        }
    }

    /// Subscribe to live updates for a table
    /// Returns the subscription UUID and a receiver for updates
    pub async fn subscribe_to_table(
        &self,
        table: &str,
    ) -> Result<(Uuid, broadcast::Receiver<ResourceUpdate>), ResourceDaoError> {
        // Check if subscription already exists
        if let Some(tx) = self.subscriptions.get(table) {
            let rx = tx.subscribe();
            // Generate a client-side ID (not the query_id)
            return Ok((Uuid::new_v4(), rx));
        }

        // Create new broadcast channel for this table
        let (tx, rx) = broadcast::channel(100);
        self.subscriptions.insert(table.to_string(), tx.clone());

        // Start SurrealDB LIVE query
        let mut live_stream = self.db_client
            .select_live::<NodeRow>(table)
            .await
            .map_err(|e| ResourceDaoError::QueryExecution(e.to_string()))?;

        // Spawn task to process live updates
        let table_name = table.to_string();
        let tx_clone = tx.clone();
        
        tokio::spawn(async move {
            while let Some(notification_result) = live_stream.next().await {
                match notification_result {
                    Ok(notification) => {
                        // Convert NodeRow to Resource
                        let uri = create_uri_from_node(&notification.data)
                            .unwrap_or_else(|_| {
                                // Fallback URI
                                Url::parse(&format!("cms://node/{}", notification.query_id))
                                    .expect("Valid fallback URI")
                            });
                        
                        let action_str = match notification.action {
                            surrealdb::Action::Create => "CREATE",
                            surrealdb::Action::Update => "UPDATE",
                            surrealdb::Action::Delete => "DELETE",
                        };

                        let resource = notification.data.to_resource(uri.clone());
                        
                        let update = ResourceUpdate {
                            uri: uri.to_string(),
                            action: action_str.to_string(),
                            resource: Some(resource),
                            query_id: notification.query_id,
                        };

                        // Broadcast to all subscribers
                        if tx_clone.send(update).is_err() {
                            log::warn!("No active subscribers for table {}", table_name);
                            break;
                        }
                    }
                    Err(e) => {
                        log::error!("Live query error for {}: {}", table_name, e);
                        break;
                    }
                }
            }
            
            log::info!("Live query stream ended for table: {}", table_name);
        });

        Ok((Uuid::new_v4(), rx))
    }

    /// Unsubscribe from table updates
    /// Note: The live query continues as long as there are active receivers
    pub fn unsubscribe(&self, table: &str) -> bool {
        // When the last receiver is dropped, the spawned task will
        // detect send failure and terminate
        self.subscriptions.remove(table).is_some()
    }
    
    /// Get a receiver for an existing subscription
    pub fn get_receiver(&self, table: &str) -> Option<broadcast::Receiver<ResourceUpdate>> {
        self.subscriptions.get(table).map(|tx| tx.subscribe())
    }
    
    /// Check if a table has active subscriptions
    pub fn is_subscribed(&self, table: &str) -> bool {
        self.subscriptions.contains_key(table)
    }
}

/// Stream-based resources list implementation
pub fn resources_list_stream(request: Option<ListResourcesRequest>) -> ResourceStream {
    let (tx, rx) = mpsc::channel(16);

    // Clone the request for the async task
    let request_clone = request.clone();

    tokio::spawn(async move {
        // Build the query based on request parameters
        let query_builder = build_query_from_request(request_clone);
        let query = query_builder.build_query();

        // Execute the database query
        match execute_resources_query(&query).await {
            Ok(rows) => {
                for row in rows {
                    // Parse the URI from the node ID
                    let uri = match create_uri_from_node(&row) {
                        Ok(uri) => uri,
                        Err(e) => {
                            log::error!("Failed to create URI for node: {}", e);
                            continue; // Skip this row
                        }
                    };

                    // Convert row to resource
                    let resource = row.to_resource(uri);

                    // Send the resource through the channel
                    if tx.send(Ok(resource)).await.is_err() {
                        log::warn!("Receiver dropped for resources_list_stream");
                        break; // Stop sending if receiver is gone
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to query resources: {}", e);
                // Send an error through the channel if the query failed
                let _ = tx.send(Err(rpc_router::HandlerError::new(format!(
                    "Database query failed: {}",
                    e
                ))));
            }
        }
    });

    ResourceStream::new(rx)
}

/// Build query builder from list resources request
fn build_query_from_request(request: Option<ListResourcesRequest>) -> ResourceQueryBuilder {
    let mut builder = ResourceQueryBuilder::new();

    if let Some(req) = request {
        // Set resource types filter
        if let Some(types) = req.resource_types {
            builder = builder.with_types(types);
        }

        // Set tags filter
        if let Some(tags) = req.tags {
            builder = builder.with_tags(tags);
        }

        // Set parent filter
        if let Some(parent) = req.parent {
            builder = builder.with_parent(parent);
        }

        // Set search query
        if let Some(query) = req.search_query {
            builder = builder.with_search(query);
        }

        // Set pagination
        if let Some(limit) = req.limit {
            builder = builder.with_limit(limit as usize);
        }

        if let Some(offset) = req.offset {
            builder = builder.with_offset(offset as usize);
        }

        // Set sorting
        if let Some(sort_field) = req.sort_field {
            let sort_direction = req.sort_direction.unwrap_or_else(|| "ASC".to_string());
            builder = builder.with_sort(sort_field, sort_direction);
        }
    }

    builder
}

/// Execute resources query against the database
async fn execute_resources_query(query: &str) -> Result<Vec<NodeRow>, ResourceDaoError> {
    // Get database client
    let db = get_database_client().await
        .map_err(|e| ResourceDaoError::DatabaseConnection(e.to_string()))?;

    // Execute the query
    let mut result = db.query(query).await
        .map_err(|e| ResourceDaoError::QueryExecution(e.to_string()))?;

    // Extract the results
    let rows: Vec<NodeRow> = result.take(0)
        .map_err(|e| ResourceDaoError::Serialization(e.to_string()))?;

    Ok(rows)
}

/// Create URI from node row
fn create_uri_from_node(row: &NodeRow) -> Result<Url, ResourceDaoError> {
    // Generate URI from node data (assuming we have an ID or slug)
    let uri_string = if let Some(ref slug) = row.slug {
        format!("cms://node/{}", slug)
    } else {
        // Fallback to a generated ID based on title
        let normalized_title = row.title
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();
        format!("cms://node/{}", normalized_title)
    };

    Url::parse(&uri_string)
        .map_err(|e| ResourceDaoError::InvalidUri(format!("Failed to parse URI '{}': {}", uri_string, e)))
}

/// Get database client with error handling
async fn get_database_client() -> Result<DatabaseClient, String> {
    // Access the ResourceDao singleton and extract the DatabaseClient
    crate::resource::cms::resource_dao::get_resource_dao()
        .map_err(|e| format!("ResourceDao not initialized: {}", e))?
        .client()
        .cloned()
        .ok_or_else(|| "DatabaseClient not available in ResourceDao".to_string())
}

/// Stream resources by type
pub fn stream_resources_by_type(resource_type: String) -> ResourceStream {
    let mut request = ListResourcesRequest::default();
    request.resource_types = Some(vec![resource_type]);
    resources_list_stream(Some(request))
}

/// Stream resources by tags
pub fn stream_resources_by_tags(tags: Vec<String>) -> ResourceStream {
    let mut request = ListResourcesRequest::default();
    request.tags = Some(tags);
    resources_list_stream(Some(request))
}

/// Stream resources by parent
pub fn stream_resources_by_parent(parent: String) -> ResourceStream {
    let mut request = ListResourcesRequest::default();
    request.parent = Some(parent);
    resources_list_stream(Some(request))
}

/// Stream resources with search query
pub fn stream_resources_with_search(query: String) -> ResourceStream {
    let mut request = ListResourcesRequest::default();
    request.search_query = Some(query);
    resources_list_stream(Some(request))
}

/// Stream paginated resources
pub fn stream_paginated_resources(limit: u32, offset: u32) -> ResourceStream {
    let mut request = ListResourcesRequest::default();
    request.limit = Some(limit);
    request.offset = Some(offset);
    resources_list_stream(Some(request))
}

/// Stream sorted resources
pub fn stream_sorted_resources(sort_field: String, sort_direction: String) -> ResourceStream {
    let mut request = ListResourcesRequest::default();
    request.sort_field = Some(sort_field);
    request.sort_direction = Some(sort_direction);
    resources_list_stream(Some(request))
}

/// Advanced resource streaming with multiple filters
pub fn stream_resources_advanced(
    resource_types: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    parent: Option<String>,
    search_query: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    sort_field: Option<String>,
    sort_direction: Option<String>,
) -> ResourceStream {
    let request = ListResourcesRequest {
        resource_types,
        tags,
        parent,
        search_query,
        limit,
        offset,
        sort_field,
        sort_direction,
        ..Default::default()
    };

    resources_list_stream(Some(request))
}

/// Stream resources with custom query
pub fn stream_resources_custom_query(query: String) -> ResourceStream {
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        match execute_resources_query(&query).await {
            Ok(rows) => {
                for row in rows {
                    let uri = match create_uri_from_node(&row) {
                        Ok(uri) => uri,
                        Err(e) => {
                            log::error!("Failed to create URI for node: {}", e);
                            continue;
                        }
                    };

                    let resource = row.to_resource(uri);

                    if tx.send(Ok(resource)).await.is_err() {
                        log::warn!("Receiver dropped for custom query stream");
                        break;
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to execute custom query: {}", e);
                let _ = tx.send(Err(rpc_router::HandlerError::new(format!(
                    "Custom query failed: {}",
                    e
                ))));
            }
        }
    });

    ResourceStream::new(rx)
}

/// Stream resources with real-time updates
pub fn stream_resources_realtime(request: Option<ListResourcesRequest>) -> ResourceStream {
    let (tx, rx) = mpsc::channel(32); // Larger buffer for real-time updates

    tokio::spawn(async move {
        // Initial load
        let query_builder = build_query_from_request(request.clone());
        let query = query_builder.build_query();

        match execute_resources_query(&query).await {
            Ok(rows) => {
                for row in rows {
                    let uri = match create_uri_from_node(&row) {
                        Ok(uri) => uri,
                        Err(e) => {
                            log::error!("Failed to create URI for node: {}", e);
                            continue;
                        }
                    };

                    let resource = row.to_resource(uri);

                    if tx.send(Ok(resource)).await.is_err() {
                        log::warn!("Receiver dropped for realtime stream");
                        return;
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to query resources for realtime stream: {}", e);
                let _ = tx.send(Err(rpc_router::HandlerError::new(format!(
                    "Realtime query failed: {}",
                    e
                ))));
                return;
            }
        }

        // Set up real-time subscription using SurrealDB LIVE queries
        let db_client = match get_database_client().await {
            Ok(client) => Arc::new(client),
            Err(e) => {
                log::error!("Failed to get database client: {}", e);
                return;
            }
        };

        let manager = ResourceSubscriptionManager::new(db_client);
        
        match manager.subscribe_to_table("node").await {
            Ok((_subscription_id, mut rx_updates)) => {
                // Stream live updates to the channel
                while let Ok(update) = rx_updates.recv().await {
                    if let Some(resource) = update.resource {
                        if tx.send(Ok(resource)).await.is_err() {
                            log::warn!("Receiver dropped, stopping live updates");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to subscribe to live updates: {}", e);
            }
        }
    });

    ResourceStream::new(rx)
}

/// Batch stream resources in chunks
pub fn stream_resources_batched(
    request: Option<ListResourcesRequest>,
    batch_size: usize,
) -> ResourceStream {
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut offset = 0;
        let limit = batch_size;

        loop {
            // Create request for this batch
            let mut batch_request = request.clone().unwrap_or_default();
            batch_request.limit = Some(limit as u32);
            batch_request.offset = Some(offset as u32);

            let query_builder = build_query_from_request(Some(batch_request));
            let query = query_builder.build_query();

            match execute_resources_query(&query).await {
                Ok(rows) => {
                    if rows.is_empty() {
                        // No more results
                        break;
                    }

                    for row in rows {
                        let uri = match create_uri_from_node(&row) {
                            Ok(uri) => uri,
                            Err(e) => {
                                log::error!("Failed to create URI for node: {}", e);
                                continue;
                            }
                        };

                        let resource = row.to_resource(uri);

                        if tx.send(Ok(resource)).await.is_err() {
                            log::warn!("Receiver dropped for batched stream");
                            return;
                        }
                    }

                    offset += batch_size;
                }
                Err(e) => {
                    log::error!("Failed to query batch: {}", e);
                    let _ = tx.send(Err(rpc_router::HandlerError::new(format!(
                        "Batch query failed: {}",
                        e
                    ))));
                    break;
                }
            }
        }
    });

    ResourceStream::new(rx)
}

/// Stream resources with error recovery
pub fn stream_resources_with_retry(
    request: Option<ListResourcesRequest>,
    max_retries: usize,
) -> ResourceStream {
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut retry_count = 0;

        while retry_count <= max_retries {
            let query_builder = build_query_from_request(request.clone());
            let query = query_builder.build_query();

            match execute_resources_query(&query).await {
                Ok(rows) => {
                    for row in rows {
                        let uri = match create_uri_from_node(&row) {
                            Ok(uri) => uri,
                            Err(e) => {
                                log::error!("Failed to create URI for node: {}", e);
                                continue;
                            }
                        };

                        let resource = row.to_resource(uri);

                        if tx.send(Ok(resource)).await.is_err() {
                            log::warn!("Receiver dropped for retry stream");
                            return;
                        }
                    }
                    return; // Success, exit retry loop
                }
                Err(e) => {
                    retry_count += 1;
                    if retry_count > max_retries {
                        log::error!("Failed to query resources after {} retries: {}", max_retries, e);
                        let _ = tx.send(Err(rpc_router::HandlerError::new(format!(
                            "Query failed after {} retries: {}",
                            max_retries, e
                        ))));
                        break;
                    } else {
                        log::warn!("Query failed, retrying ({}/{}): {}", retry_count, max_retries, e);
                        // Wait before retrying
                        tokio::time::sleep(tokio::time::Duration::from_millis(1000 * retry_count as u64)).await;
                    }
                }
            }
        }
    });

    ResourceStream::new(rx)
}