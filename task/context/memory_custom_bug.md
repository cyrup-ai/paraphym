# Task: Fix Memory Custom Field Type Mismatch Bug

## Priority: HIGH
## Status: NOT STARTED
## Created: 2025-10-19

---

## Error Message

```
2025-10-19T18:45:52Z ERROR paraphym_candle::builders::agent_role] 
Failed to store user memory to database: Database("missing field custom")
```

---

## Root Cause Analysis

### The Bug

Type mismatch between `MemoryNodeMetadata.custom` field and database schema expectations:

**Domain struct** ([`./src/domain/memory/primitives/node.rs:123`](../../packages/candle/src/domain/memory/primitives/node.rs#L123)):
```rust
pub struct MemoryNodeMetadata {
    pub custom: HashMap<Arc<str>, Arc<serde_json::Value>>,  // ❌ HashMap
    // ...
}
```

**Database schema** ([`./src/memory/schema/memory_schema.rs:35`](../../packages/candle/src/memory/schema/memory_schema.rs#L35)):
```rust
pub struct MemoryMetadataSchema {
    pub custom: serde_json::Value,  // ❌ Expects JSON Value
    // ...
}
```

**Conversion code** ([`./src/memory/core/manager/surreal.rs:52`](../../packages/candle/src/memory/core/manager/surreal.rs#L52)):
```rust
impl From<&MemoryNode> for MemoryNodeCreateContent {
    fn from(memory: &MemoryNode) -> Self {
        Self {
            // ...
            metadata: MemoryMetadataSchema {
                // ...
                custom: memory.metadata.custom.clone(),  // ❌ WRONG: HashMap → Value
            },
        }
    }
}
```

### Why It Fails

When `MemoryNode` is converted to `MemoryNodeCreateContent` for database storage, line 52 tries to assign a `HashMap<Arc<str>, Arc<serde_json::Value>>` to a field expecting `serde_json::Value`. This causes a deserialization error: "missing field custom".

---

## Current Code Structure

### Files Involved

| File | Line | Issue |
|------|------|-------|
| [`./src/domain/memory/primitives/node.rs`](../../packages/candle/src/domain/memory/primitives/node.rs#L123) | 123 | `custom` is `HashMap<Arc<str>, Arc<serde_json::Value>>` |
| [`./src/memory/schema/memory_schema.rs`](../../packages/candle/src/memory/schema/memory_schema.rs#L35) | 35 | Schema expects `serde_json::Value` |
| [`./src/memory/core/manager/surreal.rs`](../../packages/candle/src/memory/core/manager/surreal.rs#L52) | 52 | **BUG HERE**: Direct clone without conversion |

### Error Location

[`./src/domain/agent/chat.rs:524`](../../packages/candle/src/domain/agent/chat.rs#L524) - Where error is logged:
```rust
let user_pending = memory_tool_clone.memory().create_memory(user_memory.clone());
tokio::spawn(async move {
    if let Err(e) = user_pending.await {
        log::error!("Failed to store user memory to database: {e:?}");  // ← Error appears here
    }
});
```

---

## Solution

### Convert HashMap to JSON Value

Update [`./src/memory/core/manager/surreal.rs:52`](../../packages/candle/src/memory/core/manager/surreal.rs#L52):

**Current (Broken)**:
```rust
impl From<&MemoryNode> for MemoryNodeCreateContent {
    fn from(memory: &MemoryNode) -> Self {
        Self {
            content: memory.content.text.clone(),
            content_hash: memory.content_hash,
            memory_type: memory.memory_type,
            metadata: MemoryMetadataSchema {
                created_at: memory.metadata.created_at,
                last_accessed_at: memory
                    .metadata
                    .last_accessed_at
                    .unwrap_or(memory.metadata.created_at),
                importance: memory.metadata.importance,
                embedding: memory.metadata.embedding.clone(),
                custom: memory.metadata.custom.clone(),  // ❌ WRONG TYPE
            },
        }
    }
}
```

**Fixed**:
```rust
impl From<&MemoryNode> for MemoryNodeCreateContent {
    fn from(memory: &MemoryNode) -> Self {
        Self {
            content: memory.content.text.clone(),
            content_hash: memory.content_hash,
            memory_type: memory.memory_type,
            metadata: MemoryMetadataSchema {
                created_at: memory.metadata.created_at,
                last_accessed_at: memory
                    .metadata
                    .last_accessed_at
                    .unwrap_or(memory.metadata.created_at),
                importance: memory.metadata.importance,
                embedding: memory.metadata.embedding.clone(),
                custom: convert_custom_metadata(&memory.metadata.custom),  // ✅ CONVERT
            },
        }
    }
}

/// Convert HashMap custom metadata to JSON Value for database storage
fn convert_custom_metadata(custom: &HashMap<Arc<str>, Arc<serde_json::Value>>) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> = custom
        .iter()
        .map(|(k, v)| (k.as_ref().to_string(), (**v).clone()))
        .collect();
    
    serde_json::Value::Object(map)
}
```

---

## Implementation Steps

### Step 1: Add Conversion Helper Function

**File**: [`./src/memory/core/manager/surreal.rs`](../../packages/candle/src/memory/core/manager/surreal.rs)

**Location**: After line 30 (before `impl From<&MemoryNode>`)

**Add**:
```rust
/// Convert HashMap custom metadata to JSON Value for database storage
///
/// Transforms `HashMap<Arc<str>, Arc<serde_json::Value>>` → `serde_json::Value::Object`
/// This is necessary because the database schema expects a JSON object, not a HashMap.
fn convert_custom_metadata(custom: &HashMap<Arc<str>, Arc<serde_json::Value>>) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> = custom
        .iter()
        .map(|(k, v)| (k.as_ref().to_string(), (**v).clone()))
        .collect();
    
    serde_json::Value::Object(map)
}
```

### Step 2: Update From Implementation

**File**: [`./src/memory/core/manager/surreal.rs`](../../packages/candle/src/memory/core/manager/surreal.rs)

**Location**: Line 52

**Change**:
```rust
custom: memory.metadata.custom.clone(),  // ❌ Remove this
```

**To**:
```rust
custom: convert_custom_metadata(&memory.metadata.custom),  // ✅ Use conversion function
```

### Step 3: Add Import for HashMap

**File**: [`./src/memory/core/manager/surreal.rs`](../../packages/candle/src/memory/core/manager/surreal.rs)

**Location**: Top of file (around line 1-30)

**Verify import exists**:
```rust
use std::collections::HashMap;  // Should already be there, verify
```

If not present, add it with other use statements.

---

## Definition of Done

✅ `convert_custom_metadata()` helper function added  
✅ `From<&MemoryNode>` implementation updated to use conversion  
✅ HashMap import verified  
✅ Code compiles without errors  
✅ Memory creation succeeds without "missing field custom" error  
✅ Custom metadata properly stored as JSON object in database  
✅ Memory retrieval works correctly  

---

## Verification

### Compile Check
```bash
cd /Volumes/samsung_t9/paraphym/packages/candle
cargo check --lib
```

**Expected**: No errors

### Manual Test
```bash
cargo run --example candle_chat_loop_example
```

**Expected**: No "missing field custom" errors in logs

**Before Fix**:
```
[ERROR] Failed to store user memory to database: Database("missing field custom")
```

**After Fix**:
```
[INFO] Memory stored successfully  # No errors
```

---

## Technical Details

### Type Conversion Explanation

**Original HashMap**:
```rust
HashMap<Arc<str>, Arc<serde_json::Value>>
```

**Target JSON Value**:
```rust
serde_json::Value::Object(Map<String, Value>)
```

**Conversion Steps**:
1. Iterate over HashMap entries
2. Convert `Arc<str>` keys → `String`
3. Dereference `Arc<serde_json::Value>` → `serde_json::Value`
4. Collect into `serde_json::Map`
5. Wrap in `serde_json::Value::Object`

### Why HashMap in Domain?

The domain uses `HashMap<Arc<str>, Arc<serde_json::Value>>` for performance:
- `Arc<str>`: Zero-copy key sharing
- `Arc<serde_json::Value>`: Zero-copy value sharing
- Efficient for in-memory operations

The database uses `serde_json::Value` for compatibility:
- Standard JSON representation
- Serde serialization/deserialization
- Database storage format

**The conversion bridges the two representations.**

---

## Edge Cases Handled

### Empty HashMap

**Input**: `HashMap::new()`  
**Output**: `serde_json::Value::Object(Map::new())`  
**Status**: ✅ Handled

### Nested Values

**Input**: `{"key": {"nested": "value"}}`  
**Output**: Preserves nesting structure  
**Status**: ✅ Handled by Value clone

### Special Characters in Keys

**Input**: `{"key-with-dashes": "value"}`  
**Output**: Keys preserved as-is  
**Status**: ✅ Handled

---

## Related Code

### Where Custom Metadata is Set

[`./src/domain/memory/primitives/node.rs:149`](../../packages/candle/src/domain/memory/primitives/node.rs#L149):
```rust
pub fn set_custom(&mut self, key: impl Into<Arc<str>>, value: serde_json::Value) {
    self.custom.insert(key.into(), Arc::new(value));
    self.version += 1;
}
```

### Where Schema is Defined

[`./src/memory/schema/memory_schema.rs:25-36`](../../packages/candle/src/memory/schema/memory_schema.rs#L25-L36):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadataSchema {
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub importance: f32,
    pub embedding: Option<Vec<f32>>,
    pub custom: serde_json::Value,  // ← Must be JSON Value
}
```

---

## File Summary

| File | Action | Lines Changed |
|------|--------|---------------|
| [`./src/memory/core/manager/surreal.rs`](../../packages/candle/src/memory/core/manager/surreal.rs) | **MODIFY** | ~12 lines (add function + update call) |

**Total**: 1 file modified, ~12 lines changed

---

## End of Task Specification
