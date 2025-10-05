# IMPL_6: Database Live Query Subscriptions

You must be an expert in surrealdb 3.0 to work on this task. Start here:

- /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/sdk
- /Volumes/samsung_t9/paraphym/forks/surrealdb/crates/core

## OBJECTIVE
Implement SurrealDB LIVE queries for real-time database change notifications to support resource subscriptions.

## CONTEXT
**File:** `packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/streaming.rs`  
**Lines:** 299-301  
**Current State:** TODO comment indicates subscriptions not implemented  
**Severity:** HIGH - Real-time Updates Broken

## REQUIREMENTS
- **NO unit tests** - Testing team handles all test code
- **NO benchmarks** - Benchmarking team handles performance testing
- Focus solely on `./src` modifications

## SUBTASK1: Create ResourceSubscriptionManager Struct

**What:** Add new subscription manager with broadcast channels  
**Where:** Create in streaming.rs or new subscriptions.rs module  
**Why:** Centralize subscription management

Implementation:
```rust
use tokio::sync::broadcast;
use std::sync::Arc;
use dashmap::DashMap;

pub struct ResourceSubscriptionManager {
    /// Subscription channels: resource_uri -> broadcast channel
    subscriptions: Arc<DashMap<String, broadcast::Sender<ResourceUpdate>>>,
    /// SurrealDB client for live queries
    db: Arc<crate::db::DatabaseClient>,
}

pub struct ResourceUpdate {
    pub uri: String,
    pub action: String, // "CREATE", "UPDATE", "DELETE"
    pub data: serde_json::Value,
}
```

## SUBTASK2: Implement subscribe_to_resource() Method

**What:** Create subscription with SurrealDB LIVE query  
**Where:** ResourceSubscriptionManager impl block  
**Why:** Set up real-time notifications for resource

Implementation:
```rust
impl ResourceSubscriptionManager {
    pub async fn subscribe_to_resource(&self, uri: &str) -> Result<String> {
        let subscription_id = uuid::Uuid::new_v4().to_string();
        
        // Create broadcast channel for this resource
        let (tx, _rx) = broadcast::channel(100);
        self.subscriptions.insert(uri.to_string(), tx.clone());
        
        // Set up SurrealDB LIVE query
        let table_name = self.extract_table_from_uri(uri)?;
        let db = self.db.clone();
        let uri_clone = uri.to_string();
        
        tokio::spawn(async move {
            // SurrealDB LIVE query for real-time updates
            let mut stream = db.select_live(table_name).await?;
            
            while let Some(update) = stream.next().await {
                match update {
                    Ok(notification) => {
                        let resource_update = ResourceUpdate {
                            uri: uri_clone.clone(),
                            action: notification.action,
                            data: notification.data,
                        };
                        
                        // Broadcast to all subscribers
                        let _ = tx.send(resource_update);
                    }
                    Err(e) => {
                        error!("Live query error for {}: {}", uri_clone, e);
                        break;
                    }
                }
            }
            
            Ok::<(), anyhow::Error>(())
        });
        
        Ok(subscription_id)
    }
}
```

## SUBTASK3: Implement extract_table_from_uri() Helper

**What:** Parse SurrealDB table name from resource URI  
**Where:** ResourceSubscriptionManager impl  
**Why:** Convert URI to table name for LIVE query

Implementation:
```rust
fn extract_table_from_uri(&self, uri: &str) -> Result<String> {
    // Parse URI format: resource://table_name/id or resource://table_name
    uri.strip_prefix("resource://")
        .and_then(|s| s.split('/').next())
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("Invalid resource URI format: {}", uri))
}
```

## SUBTASK4: Implement unsubscribe() Method

**What:** Remove subscription and stop LIVE query  
**Where:** ResourceSubscriptionManager impl  
**Why:** Clean up when client unsubscribes

Implementation:
```rust
pub async fn unsubscribe(&self, subscription_id: &str) -> Result<()> {
    // Note: This is simplified. Full implementation needs to track
    // subscription_id -> uri mapping and handle LIVE query cancellation
    
    // For now, remove from subscriptions map
    // (The spawned task will stop when all senders are dropped)
    
    Ok(())
}
```

## SUBTASK5: Add get_subscription_receiver() Method

**What:** Allow clients to receive updates  
**Where:** ResourceSubscriptionManager impl  
**Why:** Provide broadcast receiver for subscription

Implementation:
```rust
pub fn get_subscription_receiver(
    &self, 
    uri: &str
) -> Option<broadcast::Receiver<ResourceUpdate>> {
    self.subscriptions.get(uri).map(|tx| tx.subscribe())
}
```

## SUBTASK6: Verify SurrealDB LIVE Query API

**What:** Check actual SurrealDB client API for LIVE queries  
**Where:** Review crate::db::DatabaseClient and SurrealDB docs  
**Why:** Ensure correct API usage

Check:
- Does DatabaseClient support select_live()?
- What does notification structure look like?
- How to properly handle LIVE query streams?
- Adjust implementation based on actual API

## SUBTASK7: Add Required Dependencies

**What:** Ensure all required crates are available  
**Where:** Cargo.toml for the package  
**Why:** Support broadcast channels and async streams

Add if missing:
```toml
[dependencies]
uuid = { version = "1.0", features = ["v4"] }
dashmap = "5.5"
```

## DEFINITION OF DONE
- [ ] ResourceSubscriptionManager struct created
- [ ] subscribe_to_resource() sets up LIVE query
- [ ] ResourceUpdate struct defined
- [ ] extract_table_from_uri() parses URIs correctly
- [ ] unsubscribe() removes subscriptions
- [ ] get_subscription_receiver() provides receivers
- [ ] Code compiles without errors
- [ ] Works with actual SurrealDB LIVE query API

## RESEARCH NOTES
### SurrealDB LIVE Queries
- Check SurrealDB Rust client docs for LIVE query syntax
- May need to use different API (e.g., `db.live()` or `db.select().live()`)
- Notification format varies by SurrealDB version

### Alternative Implementation
If SurrealDB client API differs:
```rust
// Some versions use:
let mut notifications = db.live::<ResourceData>("table_name").await?;
```

### URI Format
- Determine actual resource URI format used in the system
- May be different from resource://table/id
- Adjust parser accordingly

## DEPENDENCIES
- Requires crate::db::DatabaseClient
- May depend on IMPL_1 (ResourceDao) being complete
- IMPL_7 depends on this task being complete
