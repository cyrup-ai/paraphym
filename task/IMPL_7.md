# IMPL_7: Resource Subscription Handlers

You must be an expert in [Model Context Protocol](https://modelcontextprotocol.io/docs/getting-started/intro) (MCP) to work on this task.


## OBJECTIVE
Implement resources/subscribe and resources/unsubscribe handlers to enable real-time resource updates.

## CONTEXT
**File:** `packages/sweetmcp/packages/axum/src/router.rs`  
**Lines:** 41-43  
**Current State:** Handlers commented out with TODO  
**Severity:** HIGH - Real-time Features Missing

## REQUIREMENTS
- **NO unit tests** - Testing team handles all test code
- **NO benchmarks** - Benchmarking team handles performance testing
- Focus solely on `./src` modifications

## SUBTASK1: Create SubscribeRequest and SubscribeResponse Types

**What:** Define request/response structures for subscription  
**Where:** Create in resource module types file  
**Why:** Type-safe subscription API

Implementation:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeResponse {
    pub subscription_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    pub subscription_id: String,
}
```

## SUBTASK2: Create resource_subscribe_handler()

**What:** Implement handler for resources/subscribe endpoint  
**Where:** Add to resource module (likely resource/handlers.rs or similar)  
**Why:** Enable clients to subscribe to resources

Implementation:
```rust
pub async fn resource_subscribe_handler(
    request: SubscribeRequest
) -> Result<SubscribeResponse> {
    let subscription_id = resource_dao::subscribe_to_resource(&request.uri).await
        .map_err(|e| anyhow::anyhow!("Subscription failed: {}", e))?;
    
    Ok(SubscribeResponse {
        subscription_id,
    })
}
```

## SUBTASK3: Create resource_unsubscribe_handler()

**What:** Implement handler for resources/unsubscribe endpoint  
**Where:** Same location as subscribe handler  
**Why:** Allow clients to clean up subscriptions

Implementation:
```rust
pub async fn resource_unsubscribe_handler(
    request: UnsubscribeRequest
) -> Result<()> {
    resource_dao::unsubscribe(&request.subscription_id).await
        .map_err(|e| anyhow::anyhow!("Unsubscribe failed: {}", e))?;
    
    Ok(())
}
```

## SUBTASK4: Expose Handlers from Module

**What:** Export handlers from resource module  
**Where:** resource/mod.rs  
**Why:** Make handlers accessible to router

Add to module exports:
```rust
pub use handlers::{resource_subscribe_handler, resource_unsubscribe_handler};
```

## SUBTASK5: Uncomment and Wire Up Routes

**What:** Enable subscription routes in router  
**Where:** Lines 41-43 in router.rs  
**Why:** Activate subscription endpoints

Replace:
```rust
// TODO: Add when handlers are implemented
// .append("resources/subscribe", resource_subscribe_handler)
// .append("resources/unsubscribe", resource_unsubscribe_handler)
```

With:
```rust
.append("resources/subscribe", resource_subscribe_handler)
.append("resources/unsubscribe", resource_unsubscribe_handler)
```

## SUBTASK6: Wire Handlers to ResourceSubscriptionManager

**What:** Connect handlers to subscription manager from IMPL_6  
**Where:** resource_dao module or handlers  
**Why:** Use LIVE query subscriptions

Add to resource_dao or create module-level access:
```rust
use once_cell::sync::Lazy;

static SUBSCRIPTION_MANAGER: Lazy<ResourceSubscriptionManager> = Lazy::new(|| {
    ResourceSubscriptionManager::new(/* db client */)
});

pub async fn subscribe_to_resource(uri: &str) -> Result<String> {
    SUBSCRIPTION_MANAGER.subscribe_to_resource(uri).await
}

pub async fn unsubscribe(subscription_id: &str) -> Result<()> {
    SUBSCRIPTION_MANAGER.unsubscribe(subscription_id).await
}
```

## SUBTASK7: Verify Handler Return Types Match Router

**What:** Ensure handler signatures match router expectations  
**Where:** Check router.rs append() requirements  
**Why:** Prevent type mismatches

Verify:
- Return types are compatible with router
- async/await properly handled
- Error types match expected Result<T, E>

## DEFINITION OF DONE
- [ ] SubscribeRequest/Response types defined
- [ ] UnsubscribeRequest type defined
- [ ] resource_subscribe_handler() implemented
- [ ] resource_unsubscribe_handler() implemented
- [ ] Handlers exported from resource module
- [ ] Routes uncommented and wired up in router
- [ ] Handlers connected to ResourceSubscriptionManager
- [ ] Code compiles without errors
- [ ] Subscription endpoints are functional

## RESEARCH NOTES
### Router Pattern
- Check how other handlers are structured in router.rs
- Verify .append() signature and requirements
- May need middleware or error conversion

### Resource Module Structure
- Locate where other resource handlers are defined
- Follow existing patterns for consistency
- Check module exports (mod.rs)

### Integration Points
- Handlers need access to ResourceSubscriptionManager (from IMPL_6)
- May need dependency injection or global static
- Consider using OnceLock or Lazy for manager instance

## DEPENDENCIES
- **REQUIRES IMPL_6** - ResourceSubscriptionManager must be implemented first
- Uses resource_dao or subscription manager module
- Depends on router infrastructure being in place
