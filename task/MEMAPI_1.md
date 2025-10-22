# MEMAPI_1: Activate Memory API Router

## OBJECTIVE

Activate the fully-implemented memory API router by uncommenting the `routes::create_router()` call and removing the placeholder empty router.

## BACKGROUND

The memory API is fully implemented with routes, handlers, middleware, and models in `packages/candle/src/memory/api/`. However, the router initialization in `mod.rs` is commented out and uses an empty `Router::new()` instead. This is a trivial one-line fix to activate production-ready code.

The `routes::create_router()` function exists and is fully implemented with all endpoints:
- `/memories` (POST, GET, PUT, DELETE)
- `/memories/search` (POST with streaming)
- `/health` (GET)
- `/metrics` (GET for Prometheus)

## CONSTRAINTS

- **NO TESTS**: Do not write unit tests, integration tests, or test code. Another team handles testing.
- **NO BENCHMARKS**: Do not write benchmark code. Another team handles performance testing.
- **FOCUS**: Only modify `./src` files to implement the feature.

## SUBTASK 1: Review Existing Router Implementation

**Location**: `packages/candle/src/memory/api/routes.rs`

**What to verify**: The `create_router()` function exists and is fully implemented.

**Expected signature**:
```rust
pub fn create_router(memory_manager: Arc<SurrealMemoryManager>) -> Router {
    Router::new()
        .route("/memories", post(create_memory))
        .route("/memories/:id", get(get_memory))
        .route("/memories/:id", put(update_memory))
        .route("/memories/:id", delete(delete_memory))
        .route("/memories/search", post(search_memories))
        .route("/health", get(get_health))
        .route("/metrics", get(get_metrics))
        .with_state(memory_manager)
}
```

**Note**: Check if the function signature matches the generic type `M` used in `mod.rs` or if it's specifically `SurrealMemoryManager`. May need minor adjustment.

## SUBTASK 2: Review Handler Implementations

**Location**: `packages/candle/src/memory/api/handlers.rs`

**What to verify**: All handlers referenced in routes are implemented:
- `create_memory`
- `get_memory`
- `update_memory`
- `delete_memory`
- `search_memories`
- `get_health`
- `get_metrics`

## SUBTASK 3: Activate Router in mod.rs

**Location**: `packages/candle/src/memory/api/mod.rs` (lines 47-49)

**Current code**:
```rust
pub fn new(memory_manager: Arc<M>, config: APIConfig) -> Self {
    // TODO: Implement routes module
    // let router = routes::create_router(memory_manager.clone(), &config);
    let router = Router::new();
```

**What to change**:
1. Uncomment line 48: `let router = routes::create_router(memory_manager.clone(), &config);`
2. Delete line 49: `let router = Router::new();`
3. Remove the TODO comment on line 47
4. Verify the function signature matches (check if `&config` parameter is needed)

**Expected implementation**:
```rust
pub fn new(memory_manager: Arc<M>, config: APIConfig) -> Self {
    let router = routes::create_router(memory_manager.clone(), &config);
    
    Self {
        _memory_manager: memory_manager,
        config,
        router,
    }
}
```

**Important**: Check if `routes::create_router()` actually takes a `&config` parameter. If not, remove it from the call.

## SUBTASK 4: Handle Type Compatibility

**What to check**: The `routes::create_router()` function may expect `Arc<SurrealMemoryManager>` specifically, while `mod.rs` uses a generic `M: MemoryManager`.

**Possible scenarios**:
1. **If types match**: No changes needed
2. **If generic needs constraint**: May need to adjust the function signature or add trait bounds
3. **If concrete type needed**: May need to adjust how the router is created

**Resolution**: Check the actual implementation and ensure type compatibility.

## SUBTASK 5: Verify Compilation with API Feature

**Commands**:
```bash
cargo check -p paraphym_candle --features api
cargo clippy -p paraphym_candle --features api
cargo fmt -p paraphym_candle
```

**What to verify**:
- Code compiles without errors with the `api` feature enabled
- No new clippy warnings
- TODO comment is removed
- Empty router line is deleted
- Router is properly initialized with routes

## DEFINITION OF DONE

- [ ] `routes::create_router()` is called instead of `Router::new()`
- [ ] Empty router line is deleted
- [ ] TODO comment is removed
- [ ] Type compatibility is verified and resolved
- [ ] Code compiles without errors with `--features api`
- [ ] No new clippy warnings

## WHY THIS MATTERS

The memory API is fully implemented but not activated. This is literally a one-line fix that enables a complete, production-ready REST API for memory management.

## REFERENCE FILES

- **File to modify**: `packages/candle/src/memory/api/mod.rs` (lines 47-49)
- **Router implementation**: `packages/candle/src/memory/api/routes.rs`
- **Handler implementations**: `packages/candle/src/memory/api/handlers.rs`
- **Middleware**: `packages/candle/src/memory/api/middleware.rs`
- **Models**: `packages/candle/src/memory/api/models.rs`
