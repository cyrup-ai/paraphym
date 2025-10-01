# MIGRATE_2: Implement V2 Vector Index Migration

## OBJECTIVE

Implement V2AddVectorIndex migration with real SurrealDB commands to add MTREE vector index on memory_nodes.metadata.embedding field for efficient vector similarity search.

## CONSTRAINTS

- DO NOT write any unit tests, integration tests, or benchmarks
- DO NOT use unwrap() or expect() in src/* code
- Make ONLY minimal, surgical changes required
- DO NOT modify code outside the scope of this task

## SUBTASK 1: Update V2AddVectorIndex.up() Implementation

**File**: `packages/candle/src/memory/migration/schema_migrations.rs`
**Lines**: 137-145

**What needs to change**:
- Replace stub implementation with real SurrealDB vector index creation
- Create MTREE index on metadata.embedding field
- Set correct dimension (1536 for OpenAI embeddings)
- Proper error handling without unwrap/expect

**Why**:
- Current implementation is a stub that does nothing
- Vector similarity search requires specialized index for performance
- MTREE index enables efficient nearest-neighbor queries

**Current code**:
```rust
fn up(&self) -> PendingMigration {
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        // Add vector index
        let _ = tx.send(Ok(()));
    });
    PendingMigration::new(rx)
}
```

**Required implementation**:
```rust
fn up(&self, db: Arc<Surreal<Any>>) -> PendingMigration {
    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        let result = db.query("
            DEFINE INDEX idx_embedding ON memory_nodes
            FIELDS metadata.embedding
            MTREE DIMENSION 1536;
        ").await;

        if let Err(e) = result {
            let _ = tx.send(Err(MigrationError::DatabaseError(
                format!("Failed to create vector index: {}", e)
            )));
            return;
        }

        let _ = tx.send(Ok(()));
    });

    PendingMigration::new(rx)
}
```

## SUBTASK 2: Update V2AddVectorIndex.down() Implementation

**File**: `packages/candle/src/memory/migration/schema_migrations.rs`
**Lines**: 148-156

**What needs to change**:
- Replace stub implementation with real index removal command
- Remove the vector index created by up()
- Proper error handling without unwrap/expect

**Why**:
- Current implementation is a stub that does nothing
- Need to support rollback by removing created index
- down() must reverse everything up() does

**Current code**:
```rust
fn down(&self) -> PendingMigration {
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        // Remove vector index
        let _ = tx.send(Ok(()));
    });
    PendingMigration::new(rx)
}
```

**Required implementation**:
```rust
fn down(&self, db: Arc<Surreal<Any>>) -> PendingMigration {
    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        let result = db.query("
            REMOVE INDEX idx_embedding ON memory_nodes;
        ").await;

        if let Err(e) = result {
            let _ = tx.send(Err(MigrationError::DatabaseError(
                format!("Failed to remove vector index: {}", e)
            )));
            return;
        }

        let _ = tx.send(Ok(()));
    });

    PendingMigration::new(rx)
}
```

## DEFINITION OF DONE

- V2AddVectorIndex.up() creates MTREE vector index on metadata.embedding
- Index dimension is correctly set to 1536
- V2AddVectorIndex.down() removes the created index
- No unwrap() or expect() calls in implementation
- Proper error handling with descriptive error messages
- Code compiles with cargo check
- Code passes clippy with no warnings

## RESEARCH NOTES

**SurrealDB Vector Index Syntax**:
```
DEFINE INDEX <name> ON <table>
FIELDS <field>
MTREE DIMENSION <n>;
```

**MTREE Index Type**:
- M-Tree (Metric Tree) is specialized for vector similarity search
- Enables efficient k-nearest-neighbor (kNN) queries
- Supports cosine similarity and other distance metrics
- Required for performant embedding-based memory retrieval

**Embedding Dimensions**:
- OpenAI text-embedding-ada-002: 1536 dimensions
- OpenAI text-embedding-3-small: 1536 dimensions
- OpenAI text-embedding-3-large: 3072 dimensions
- This implementation assumes 1536 (most common)

**Why Vector Index is Important**:
- Without index: O(n) linear scan of all embeddings
- With MTREE index: O(log n) approximate nearest neighbor search
- Critical for scaling to large memory stores (>10k memories)
- Enables semantic search by similarity

**Query Usage After Index**:
```sql
-- Find similar memories using vector index
SELECT * FROM memory_nodes
WHERE metadata.embedding <|> $query_vector
ORDER BY metadata.embedding <|> $query_vector
LIMIT 10;
```

**Index Creation Performance**:
- Index creation scans existing embeddings
- May take time on large datasets (acceptable during migration)
- Index is incrementally updated on new inserts
- No performance impact on queries during creation

**Error Handling**:
- Check for database errors during index creation
- Index creation might fail if embeddings field doesn't exist yet
- This is acceptable - V1 creates schema, V2 adds optimization
