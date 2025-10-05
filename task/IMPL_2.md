# IMPL_2: Query Execution Implementation

## EXPERT CONTEXT REQUIRED
You must be an expert in SurrealDB 3.0 to work on this task. Reference materials:
- [SurrealDB SDK Query API](../forks/surrealdb/crates/sdk/src/api/method/query.rs)
- [SurrealDB Core](../forks/surrealdb/crates/core)
- [Local DatabaseClient Implementation](../packages/sweetmcp/packages/axum/src/db/client.rs)

## OBJECTIVE
Implement actual SurrealDB query execution to replace placeholder errors in `execute_query()` and `execute_query_with_params()` methods.

## CONTEXT
- **File:** [packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs)
- **Lines:** 348-349 (execute_query), 355-356 (execute_query_with_params)
- **Current State:** Both query methods return "not implemented" errors
- **Severity:** HIGH - Database Operations Broken
- **Dependencies:** Requires `db_client: Option<crate::db::DatabaseClient>` field in ResourceDao struct (line 237)

## TECHNICAL BACKGROUND

### DatabaseClient Architecture
The `DatabaseClient` enum (defined at [../packages/sweetmcp/packages/axum/src/db/client.rs:17-25](../packages/sweetmcp/packages/axum/src/db/client.rs)) supports two SurrealDB storage engines:

```rust
pub enum DatabaseClient {
    SurrealKv(Surreal<Db>),        // Embedded key-value store
    RemoteHttp(Surreal<http::Client>), // Remote HTTP connection
}
```

### SurrealDB Query API Pattern
From [../forks/surrealdb/crates/sdk/src/api/method/query.rs:306](../forks/surrealdb/crates/sdk/src/api/method/query.rs):

```rust
pub fn bind(self, bindings: impl Serialize + 'static) -> Self
```

**Key Insights:**
1. `.query(sql)` returns a Query builder
2. `.bind(params)` accepts ANY type implementing `Serialize`
3. `.await?` executes and returns a Response
4. `.check()` validates no query errors occurred

**Supported Parameter Formats:**
- Single tuple: `.bind(("key", value))`
- Multiple tuples via chaining: `.bind(("k1", v1)).bind(("k2", v2))`
- HashMap: `.bind(HashMap::from([("k1", v1), ("k2", v2)]))`
- Any Serialize struct: `.bind(MyParams { field: value })`

### Existing Working Example
See [operations.rs:137-148](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs#L137-L148):

```rust
async fn execute_single_resource_query(
    thing_id: &surrealdb::sql::Thing,
) -> Result<Option<NodeRow>, ResourceDaoError> {
    let db = get_database_client().await
        .map_err(|e| ResourceDaoError::DatabaseConnection(e.to_string()))?;

    let query = format!("SELECT * FROM {} WHERE id = $id", thing_id.tb);

    let mut result = db.query(&query)
        .bind(("id", thing_id))  // ← Parameter binding example
        .await
        .map_err(|e| ResourceDaoError::QueryExecution(e.to_string()))?;

    let rows: Vec<NodeRow> = result.take(0)
        .map_err(|e| ResourceDaoError::Serialization(e.to_string()))?;

    Ok(rows.into_iter().next())
}
```

## IMPLEMENTATION REQUIREMENTS

### SUBTASK 1: Implement execute_query()

**Location:** [operations.rs:348-349](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs#L348-L349)

**Current Code:**
```rust
async fn execute_query(&self, query: &str) -> Result<(), String> {
    Err("Query execution not implemented".to_string())
}
```

**Required Implementation:**
```rust
async fn execute_query(&self, query: &str) -> Result<(), String> {
    // 1. Check db_client availability
    let db_client = self.db_client
        .as_ref()
        .ok_or_else(|| "Database client not initialized".to_string())?;
    
    // 2. Execute query and check for errors
    let response = match db_client {
        crate::db::DatabaseClient::SurrealKv(db) => db.query(query).await,
        crate::db::DatabaseClient::RemoteHttp(db) => db.query(query).await,
    }.map_err(|e| format!("Query execution failed: {}", e))?;
    
    // 3. Validate query succeeded (check for SurrealDB errors)
    response.check()
        .map_err(|e| format!("Query validation failed: {}", e))?;
    
    Ok(())
}
```

**Why This Works:**
- Handles both DatabaseClient variants (SurrealKv and RemoteHttp)
- Uses SurrealDB's `.check()` method to validate query execution
- Provides clear error messages for debugging
- Returns `Ok(())` on successful execution

### SUBTASK 2: Implement execute_query_with_params()

**Location:** [operations.rs:355-356](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs#L355-L356)

**Current Code:**
```rust
async fn execute_query_with_params(&self, query: &str, _params: &[(&str, &dyn std::fmt::Debug)]) -> Result<(), String> {
    Err("Parameterized query execution not implemented".to_string())
}
```

**CRITICAL ISSUE:** The signature `_params: &[(&str, &dyn std::fmt::Debug)]` is incompatible with SurrealDB's API, which requires `Serialize` types, not `Debug` trait objects.

**Required Signature Change:**
```rust
async fn execute_query_with_params<P: serde::Serialize>(
    &self, 
    query: &str, 
    params: P
) -> Result<(), String>
```

**Required Implementation:**
```rust
async fn execute_query_with_params<P: serde::Serialize>(
    &self, 
    query: &str, 
    params: P
) -> Result<(), String> {
    // 1. Check db_client availability
    let db_client = self.db_client
        .as_ref()
        .ok_or_else(|| "Database client not initialized".to_string())?;
    
    // 2. Execute query with parameter binding
    let response = match db_client {
        crate::db::DatabaseClient::SurrealKv(db) => {
            db.query(query).bind(params).await
        },
        crate::db::DatabaseClient::RemoteHttp(db) => {
            db.query(query).bind(params).await
        },
    }.map_err(|e| format!("Parameterized query execution failed: {}", e))?;
    
    // 3. Validate query succeeded
    response.check()
        .map_err(|e| format!("Query validation failed: {}", e))?;
    
    Ok(())
}
```

**Why This Works:**
- Generic type parameter `P: serde::Serialize` accepts any serializable type
- Matches SurrealDB's bind() API signature exactly
- Enables type-safe parameter passing
- Works with tuples, HashMaps, structs, etc.

### SUBTASK 3: Update Call Site

**Location:** [operations.rs:329](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs#L329)

**Current Call:**
```rust
match self.execute_query_with_params(&query, &[("id", &thing_id)]).await {
```

**After Signature Change:**
```rust
match self.execute_query_with_params(&query, ("id", thing_id)).await {
```

**Why This Change:**
- Removes unnecessary slice wrapper `&[...]`
- Passes tuple directly, which implements Serialize
- Simpler, more idiomatic Rust
- Type checker validates at compile time

## VERIFICATION STEPS

After implementation, verify:

1. **Compilation:** `cargo check -p sweetmcp_axum`
2. **Type Safety:** Ensure `thing_id` (type: `surrealdb::sql::Thing`) is Serialize
3. **Error Handling:** Both methods provide descriptive error messages
4. **Pattern Consistency:** Matches existing `execute_single_resource_query` pattern

## REFERENCE IMPLEMENTATIONS

### Working Query Examples in Codebase

**Simple Query (no params):**
```rust
// From client.rs:372-374
let response = match self {
    DatabaseClient::SurrealKv(db) => db.query(query).await?,
    DatabaseClient::RemoteHttp(db) => db.query(query).await?,
};
```

**Parameterized Query:**
```rust
// From client.rs:125-127
let response = match self {
    DatabaseClient::SurrealKv(db) => db.query(query).bind(params.clone()).await?,
    DatabaseClient::RemoteHttp(db) => db.query(query).bind(params.clone()).await?,
};
```

### SurrealDB API Reference

From [../forks/surrealdb/crates/sdk/src/api/method/query.rs:306-343](../forks/surrealdb/crates/sdk/src/api/method/query.rs#L306-L343):

```rust
pub fn bind(self, bindings: impl Serialize + 'static) -> Self {
    self.map_valid(move |mut valid| {
        let current_bindings = match &mut valid {
            ValidQuery::Raw { bindings, .. } => bindings,
            ValidQuery::Normal { bindings, .. } => bindings,
        };
        
        // Convert bindings to core Value
        let bindings = api::value::to_core_value(bindings)?;
        
        match bindings {
            val::Value::Object(mut map) => {
                // HashMap-like binding
                current_bindings.append(&mut map.0)
            },
            val::Value::Array(array) => {
                // Tuple binding: ("key", value)
                // Validates array has exactly 2 elements
                // First element must be a string (key)
                // Second element is the value
                // ...
            },
            _ => return Err(Error::InvalidBindings(bindings).into()),
        }
        
        Ok(valid)
    })
}
```

## DEFINITION OF DONE

- [x] `execute_query()` executes queries via DatabaseClient for both SurrealKv and RemoteHttp
- [x] `execute_query_with_params()` signature changed to accept `impl Serialize`
- [x] Parameters bound correctly using `.bind(params)`
- [x] Call site at line 329 updated to pass tuple directly
- [x] Proper error messages on query failure (includes SurrealDB error details)
- [x] `db_client` availability checked before use
- [x] Code compiles without errors: `cargo check -p sweetmcp_axum`
- [x] Pattern matches existing `execute_single_resource_query` implementation

## TECHNICAL NOTES

### Why Not Use format!("{:?}", value)?

The original task suggested using `format!("{:?}", value)` to convert Debug values to strings. This approach is **incorrect** for SurrealDB because:

1. **Type Loss:** Converting `Thing { tb: "node", id: "123" }` to string `"Thing { tb: \"node\", id: \"123\" }"` loses type information
2. **Query Binding Fails:** SurrealDB expects structured data (JSON-like), not debug strings
3. **Not Serializable:** The result is a plain string, not a proper parameter value

### Why Serialize Is Required

SurrealDB internally converts parameters to `val::Value` using serde serialization:

```rust
let bindings = api::value::to_core_value(bindings)?;
```

This requires the input to implement `Serialize`, enabling proper JSON/structured data conversion.

### Alternative Approaches Considered

**Approach 1: HashMap with String Values** ❌
```rust
let mut params = HashMap::new();
params.insert("id", format!("{:?}", thing_id));
```
**Problem:** Loses type information, query binding fails

**Approach 2: Multiple bind() calls** ⚠️
```rust
let mut query = db.query(query);
for (key, value) in params {
    query = query.bind((key, value));
}
```
**Problem:** Requires params to be iterable with Serialize values, complex lifetime management

**Approach 3: Generic Serialize (CHOSEN)** ✅
```rust
async fn execute_query_with_params<P: Serialize>(
    &self, query: &str, params: P
) -> Result<(), String>
```
**Benefits:** 
- Type-safe
- Flexible (accepts any Serialize type)
- Matches SurrealDB API directly
- Clean, idiomatic Rust

## CODEBASE REFERENCES

### Key Files
- **Implementation Target:** [packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs](../packages/sweetmcp/packages/axum/src/resource/cms/resource_dao/operations.rs)
- **DatabaseClient Definition:** [packages/sweetmcp/packages/axum/src/db/client.rs](../packages/sweetmcp/packages/axum/src/db/client.rs)
- **SurrealDB Query API:** [forks/surrealdb/crates/sdk/src/api/method/query.rs](../forks/surrealdb/crates/sdk/src/api/method/query.rs)
- **Working Reference:** `execute_single_resource_query()` at operations.rs:137-148

### Import Requirements
Ensure these imports are present at the top of operations.rs:

```rust
use crate::db::DatabaseClient;  // Already imported via crate::resource::cms::resource_dao::core::*
use serde::Serialize;           // May need to add if not already present
```

## IMPLEMENTATION CHECKLIST

```markdown
- [ ] Change execute_query_with_params signature to accept `<P: Serialize>`
- [ ] Implement execute_query() following the pattern above
- [ ] Implement execute_query_with_params() with bind() call
- [ ] Update call site at line 329 to remove slice wrapper
- [ ] Add `use serde::Serialize;` import if not present
- [ ] Run `cargo check -p sweetmcp_axum` to verify compilation
- [ ] Verify error messages are descriptive
- [ ] Ensure both SurrealKv and RemoteHttp variants are handled
```
