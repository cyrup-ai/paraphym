# IMPL_5: Context Store Search Implementation

You must be an expert in [Model Context Protocol](https://modelcontextprotocol.io/docs/getting-started/intro) (MCP) to work on this task.

## OBJECTIVE
Implement actual context store search to replace empty stream in context_get_context_stream() function.

## CONTEXT
**File:** `packages/sweetmcp/packages/axum/src/context/stream.rs`  
**Lines:** 37-40  
**Current State:** Creates channel but never sends data, search commented out  
**Severity:** HIGH - Context Search Broken

## REQUIREMENTS
- **NO unit tests** - Testing team handles all test code
- **NO benchmarks** - Benchmarking team handles performance testing
- Focus solely on `./src` modifications

## SUBTASK1: Create search_and_stream_contexts() Helper

**What:** Implement async function to search and stream context items  
**Where:** Add to stream.rs module  
**Why:** Separate search logic from stream setup

Implementation:
```rust
async fn search_and_stream_contexts(
    request: GetContextRequest,
    tx: mpsc::Sender<Result<ContextItem, String>>
) -> Result<()> {
    let store = CONTEXT_STORE.read().await;
    
    // Filter contexts by scopes if specified
    let matching_contexts: Vec<_> = if let Some(scopes) = &request.scopes {
        store.contexts
            .iter()
            .filter(|(id, _)| scopes.iter().any(|scope| id.starts_with(scope)))
            .collect()
    } else {
        store.contexts.iter().collect()
    };
    
    // Stream matching contexts
    for (context_id, context) in matching_contexts {
        let context_item = ContextItem {
            id: context_id.clone(),
            content: context.content.clone(),
            metadata: context.metadata.clone(),
        };
        
        // Send item (with backpressure handling)
        if tx.send(Ok(context_item)).await.is_err() {
            debug!("Context stream receiver dropped");
            break;
        }
    }
    
    Ok(())
}
```

## SUBTASK2: Update context_get_context_stream() to Use Search

**What:** Replace commented TODO with actual implementation  
**Where:** Lines 37-40  
**Why:** Make context search functional

Replace:
```rust
pub fn context_get_context_stream(_request: GetContextRequest) -> ContextItemStream {
    let (_tx, rx) = mpsc::channel(16);
    // TODO: Implement actual context store search
```

With:
```rust
pub fn context_get_context_stream(request: GetContextRequest) -> ContextItemStream {
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        if let Err(e) = search_and_stream_contexts(request, tx).await {
            error!("Context search failed: {}", e);
        }
    });

    ContextItemStream { receiver: rx }
}
```

## SUBTASK3: Verify CONTEXT_STORE Access Pattern

**What:** Check how CONTEXT_STORE is defined and accessed  
**Where:** Look for static/global CONTEXT_STORE definition  
**Why:** Ensure read access pattern is correct

Expected pattern:
```rust
// Should be defined somewhere in the module
static CONTEXT_STORE: /* some type with .read() method */;
```

Verify:
- CONTEXT_STORE exists and is accessible
- It has a .read().await method (likely RwLock)
- The contexts field exists with proper structure
- Iterator pattern works as expected

## SUBTASK4: Verify ContextItem Structure

**What:** Confirm ContextItem fields match actual context structure  
**Where:** Context type definitions  
**Why:** Ensure proper field mapping

Check that context has:
- id/identifier field
- content field
- metadata field

Adjust mapping if field names differ.

## SUBTASK5: Add Error Logging Import

**What:** Ensure error! and debug! macros are available  
**Where:** Module imports  
**Why:** Support logging in implementation

Add if missing:
```rust
use tracing::{debug, error};
```

## DEFINITION OF DONE
- [ ] search_and_stream_contexts() function implemented
- [ ] context_get_context_stream() spawns search task
- [ ] Scope filtering works when scopes provided
- [ ] All contexts returned when no scopes specified
- [ ] Backpressure handling on send
- [ ] Error logging on search failure
- [ ] Code compiles without errors

## RESEARCH NOTES
### Context Store Structure
- Locate CONTEXT_STORE global definition
- Likely uses RwLock<ContextStore> pattern
- Should have contexts: HashMap or similar

### GetContextRequest
- Check if scopes field is Option<Vec<String>>
- Verify request structure matches usage

### Scope Filtering Logic
- Current impl uses starts_with() for prefix matching
- Adjust if different scope matching needed

## DEPENDENCIES
- Requires CONTEXT_STORE to be properly defined
- Needs ContextItem and GetContextRequest types
