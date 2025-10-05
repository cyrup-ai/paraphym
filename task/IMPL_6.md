# IMPL_6: Database Live Query Subscriptions

## OBJECTIVE
Implement SurrealDB LIVE queries for real-time database change notifications to support resource subscriptions in the MCP resource system.

## CONTEXT
**Target File:** [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs)  
**Current State:** Lines 299-301 contain TODO comment: "Set up real-time subscription for database changes"  
**Function:** `stream_resources_realtime()` - loads initial data but lacks live subscription mechanism  
**Severity:** HIGH - Real-time updates are broken; clients cannot receive live database change notifications

## BACKGROUND RESEARCH

### SurrealDB 3.0 LIVE Query API
Researched from [`forks/surrealdb/crates/sdk/src/api/method/live.rs`](../forks/surrealdb/crates/sdk/src/api/method/live.rs)

The SurrealDB SDK provides LIVE queries through the following API:

```rust
use surrealdb::{Surreal, Notification, Action};
use futures::StreamExt;

// Start a live query on a table
let mut stream = db
    .select("table_name")
    .live()
    .await?;

// Stream yields Notification<T> items
while let Some(result) = stream.next().await {
    match result {
        Ok(notification) => {
            // notification.query_id: Uuid
            // notification.action: Action (Create, Update, Delete)
            // notification.data: T (the record)
        }
        Err(e) => { /* handle error */ }
    }
}
```

**Type Definitions** from [`forks/surrealdb/crates/sdk/src/api/value/mod.rs:450-486`](../forks/surrealdb/crates/sdk/src/api/value/mod.rs):

```rust
/// The action performed on a record
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Action {
    Create,
    Update,
    Delete,
}

/// A live query notification
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct Notification<R> {
    pub query_id: Uuid,
    pub action: Action,
    pub data: R,
}
```

### Current Codebase Structure

**Database Table:** `node` (confirmed from streaming.rs line 72: "SELECT * FROM node")

**Record Type:** [`NodeRow`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/core.rs#L106-L138)
```rust
#[derive(serde::Deserialize, Debug)]
pub struct NodeRow {
    pub type_: String,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub slug: Option<String>,
    pub tags: Option<Vec<String>>,
    pub mime_type: Option<String>,
    pub parent: Option<surrealdb::sql::Thing>,
    pub children: Option<Vec<surrealdb::sql::Thing>>,
    pub links: Option<Vec<surrealdb::sql::Thing>>,
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

**DatabaseClient:** [`packages/sweetmcp/packages/axum/src/db/client.rs`](../packages/sweetmcp/packages/axum/src/db/client.rs#L18-L36)
```rust
pub enum DatabaseClient {
    SurrealKv(Surreal<Db>),
    RemoteHttp(Surreal<http::Client>),
}
```

**Stream Type:** [`ResourceStream`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/core.rs#L27-L40)
- Wraps `tokio::sync::mpsc::Receiver<HandlerResult<Resource>>`
- Implements `futures::Stream`

## IMPLEMENTATION PLAN

### SUBTASK 1: Add LIVE Query Support to DatabaseClient

**File:** [`packages/sweetmcp/packages/axum/src/db/client.rs`](../packages/sweetmcp/packages/axum/src/db/client.rs)  
**Location:** Add after existing methods (around line 300+)

**Required Imports (add to top of file):**
```rust
use futures::stream::Stream;
use surrealdb::method::Live;
use std::pin::Pin;
```

**Implementation:**
```rust
impl DatabaseClient {
    /// Create a live query stream for a table
    /// Returns a Stream of Notifications for real-time updates
    pub async fn select_live<T>(&self, table: &str) -> Result<Pin<Box<dyn Stream<Item = surrealdb::Result<surrealdb::Notification<T>>> + Send>>>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let stream = match self {
            DatabaseClient::SurrealKv(db) => {
                Box::pin(db.select(table).live().await?)
                    as Pin<Box<dyn Stream<Item = surrealdb::Result<surrealdb::Notification<T>>> + Send>>
            }
            DatabaseClient::RemoteHttp(db) => {
                Box::pin(db.select(table).live().await?)
                    as Pin<Box<dyn Stream<Item = surrealdb::Result<surrealdb::Notification<T>>> + Send>>
            }
        };
        Ok(stream)
    }
}
```

**Why:** The DatabaseClient wrapper currently doesn't expose `.select().live()` functionality. This method provides access to SurrealDB's LIVE query stream while maintaining the enum abstraction.

### SUBTASK 2: Create ResourceSubscriptionManager

**File:** [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs)  
**Location:** Add after existing imports (create new module section before `resources_list_stream` function)

**Required Imports (add to top of file):**
```rust
use tokio::sync::broadcast;
use dashmap::DashMap;
use std::sync::Arc;
use futures::StreamExt;
use uuid::Uuid;
```

**Data Structures:**
```rust
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
}
```

**Why:** Centralizes subscription management and provides multi-subscriber broadcast capabilities using DashMap for concurrent access.

### SUBTASK 3: Implement subscribe_to_table() Method

**File:** [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs)  
**Location:** Add to `ResourceSubscriptionManager` impl block

**Implementation:**
```rust
impl ResourceSubscriptionManager {
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
}
```

**Why:** Sets up the actual SurrealDB LIVE query, spawns an async task to process notifications, and broadcasts updates to all subscribers using tokio broadcast channels.

### SUBTASK 4: Add unsubscribe() Method

**File:** [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs)  
**Location:** Add to `ResourceSubscriptionManager` impl block

**Implementation:**
```rust
impl ResourceSubscriptionManager {
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
```

**Why:** Provides subscription lifecycle management. The spawned task automatically stops when all receivers are dropped (broadcast channel semantics).

### SUBTASK 5: Integrate into stream_resources_realtime()

**File:** [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs)  
**Location:** Replace TODO comment at lines 299-301

**Implementation:**
```rust
/// Stream resources with real-time updates using SurrealDB LIVE queries
pub fn stream_resources_realtime(request: Option<ListResourcesRequest>) -> ResourceStream {
    let (tx, rx) = mpsc::channel(32);

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
```

**Why:** Combines initial query load with continuous live updates, providing seamless real-time streaming to clients.

### SUBTASK 6: Add Required Dependencies

**File:** [`packages/sweetmcp/packages/axum/Cargo.toml`](../packages/sweetmcp/packages/axum/Cargo.toml)  
**Location:** In `[dependencies]` section

**Check and add if missing:**
```toml
[dependencies]
uuid = { version = "1.0", features = ["v4", "serde"] }
dashmap = "6.1"
futures = "0.3"
```

**Action:** Run `cargo check` to verify these dependencies are available. They likely already exist in the workspace.

### SUBTASK 7: Update Exports

**File:** [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/mod.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/mod.rs)

**Add exports:**
```rust
pub use streaming::{
    ResourceSubscriptionManager,
    ResourceUpdate,
    stream_resources_realtime,
    // ... existing exports
};
```

**Why:** Makes the subscription manager and types available to other modules.

## DEFINITION OF DONE

- [ ] `DatabaseClient::select_live()` method implemented and compiles
- [ ] `ResourceSubscriptionManager` struct created with DashMap-backed subscription tracking
- [ ] `subscribe_to_table()` successfully creates SurrealDB LIVE query and spawns broadcast task
- [ ] `ResourceUpdate` properly converts from `Notification<NodeRow>` with all fields mapped
- [ ] `unsubscribe()` and helper methods (`get_receiver`, `is_subscribed`) implemented
- [ ] `stream_resources_realtime()` function updated to use actual LIVE queries (TODO removed)
- [ ] Code compiles without errors: `cargo check -p sweetmcp-axum`
- [ ] Exports updated in mod.rs
- [ ] Live subscription automatically terminates when all receivers drop

## IMPLEMENTATION NOTES

### Critical Considerations

1. **Thread Safety**: DashMap provides lock-free concurrent access to subscriptions
2. **Resource Cleanup**: Broadcast channel semantics ensure spawned tasks terminate when receivers drop
3. **Error Handling**: Live query errors are logged and terminate the subscription gracefully
4. **URI Format**: Uses existing `create_uri_from_node()` function for consistency
5. **Action Mapping**: SurrealDB Action enum maps directly to string constants

### Table Name Extraction

The current implementation targets the "node" table specifically. For URI-based table extraction:

```rust
fn extract_table_from_uri(uri: &str) -> Result<String, ResourceDaoError> {
    // Parse URI format: cms://node/id or resource://node/id
    uri.strip_prefix("cms://")
        .or_else(|| uri.strip_prefix("resource://"))
        .and_then(|s| s.split('/').next())
        .map(String::from)
        .ok_or_else(|| ResourceDaoError::InvalidUri(format!("Invalid resource URI format: {}", uri)))
}
```

### Performance Characteristics

- **Initial Load**: Standard query execution time
- **Live Updates**: Near-instant notification (SurrealDB native LIVE query)
- **Memory**: ~100 events buffered per subscription (broadcast channel capacity)
- **Scalability**: One live query per table, broadcast to N subscribers

### Alternative: Per-Resource Subscriptions

For finer-grained subscriptions (individual resources), modify the LIVE query with conditions:

```rust
// In select_live implementation, add WHERE clause support
let stream = db.query(format!("LIVE SELECT * FROM {} WHERE id = $id", table))
    .bind(("id", record_id))
    .await?;
```

This would require extending the API but follows the same pattern.

## DEPENDENCIES

- **Upstream**: Requires `DatabaseClient` from [`packages/sweetmcp/packages/axum/src/db/client.rs`](../packages/sweetmcp/packages/axum/src/db/client.rs)
- **Downstream**: Required by `IMPL_7` (MCP Resource Subscriptions) - provides the backend for SSE streaming
- **References**: SurrealDB SDK in [`forks/surrealdb/crates/sdk`](../forks/surrealdb/crates/sdk)

## RESEARCH ARTIFACTS

- **SurrealDB LIVE Query Implementation**: [`forks/surrealdb/crates/sdk/src/api/method/live.rs`](../forks/surrealdb/crates/sdk/src/api/method/live.rs)
- **Action/Notification Types**: [`forks/surrealdb/crates/sdk/src/api/value/mod.rs:450-486`](../forks/surrealdb/crates/sdk/src/api/value/mod.rs)
- **Current Streaming Implementation**: [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs)
- **DatabaseClient Wrapper**: [`packages/sweetmcp/packages/axum/src/db/client.rs`](../packages/sweetmcp/packages/axum/src/db/client.rs)
- **Resource Data Types**: [`packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/core.rs`](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/core.rs)
