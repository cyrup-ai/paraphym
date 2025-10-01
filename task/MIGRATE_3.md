# MIGRATE_3: Implement V3 Relationship Strength Migration

## OBJECTIVE

Implement V3AddRelationshipStrength migration as a validation/no-op migration since the strength field already exists in V1 schema. This ensures migration compatibility and proper version tracking.

## CONSTRAINTS

- DO NOT write any unit tests, integration tests, or benchmarks
- DO NOT use unwrap() or expect() in src/* code
- Make ONLY minimal, surgical changes required
- DO NOT modify code outside the scope of this task

## SUBTASK 1: Update V3AddRelationshipStrength.up() Implementation

**File**: `packages/candle/src/memory/migration/schema_migrations.rs`
**Lines**: 172-180

**What needs to change**:
- Replace stub implementation with validation logic
- Check if relationships table exists
- Log that strength field already exists from V1
- Return success (no-op migration)
- Proper error handling without unwrap/expect

**Why**:
- strength field is already defined in V1 migration (relationships table)
- This migration exists for version compatibility/tracking
- Still need to record that "V3" was executed
- Provides clear logging for migration history

**Current code**:
```rust
fn up(&self) -> PendingMigration {
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        // Add strength column to relationships
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
        // Check if relationships table exists
        let result = db.query("INFO FOR TABLE relationships").await;

        match result {
            Ok(_) => {
                // Field already exists in V1, this is a no-op migration for compatibility
                tracing::info!("V3 migration: strength field already exists from V1");
                let _ = tx.send(Ok(()));
            }
            Err(e) => {
                let _ = tx.send(Err(MigrationError::DatabaseError(
                    format!("Failed to verify relationships table: {}", e)
                )));
            }
        }
    });

    PendingMigration::new(rx)
}
```

## SUBTASK 2: Update V3AddRelationshipStrength.down() Implementation

**File**: `packages/candle/src/memory/migration/schema_migrations.rs`
**Lines**: 183-191

**What needs to change**:
- Replace stub implementation with no-op rollback
- Log that strength field is part of base schema
- Return success (no action needed)
- Proper error handling without unwrap/expect

**Why**:
- Rollback can't remove strength field (it's part of V1 base schema)
- Attempting to remove it would break V1 schema
- Log clearly that no action is taken
- Migration system still tracks V3 as rolled back

**Current code**:
```rust
fn down(&self) -> PendingMigration {
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        // Remove strength column
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
        // No-op since strength field is part of V1 schema
        tracing::info!("V3 rollback: strength field is part of base schema, no action needed");
        let _ = tx.send(Ok(()));
    });

    PendingMigration::new(rx)
}
```

## DEFINITION OF DONE

- V3AddRelationshipStrength.up() validates table exists and logs no-op
- V3AddRelationshipStrength.down() logs no-op rollback
- No unwrap() or expect() calls in implementation
- Clear logging messages explain why no action is taken
- Migration is idempotent (can run multiple times safely)
- Code compiles with cargo check
- Code passes clippy with no warnings

## RESEARCH NOTES

**Why This Migration Exists**:
- Originally planned as separate migration before V1 was implemented
- V1 implementation included strength field from the start
- Kept for version numbering compatibility
- Documents that strength was considered a separate feature

**No-Op Migrations Are Valid**:
- Common pattern in migration systems
- Allows version history to reflect feature planning
- Safe to execute repeatedly (idempotent)
- Still tracked in migrations table

**Idempotency**:
- Migration can run multiple times with same result
- Critical for migration systems (may need to retry)
- up() checks before modifying
- down() checks before removing

**Logging Best Practices**:
- Use tracing::info! for normal migration events
- Clearly explain why no action taken
- Helps debugging migration history
- Documents architectural decisions

**INFO FOR TABLE Syntax**:
```sql
INFO FOR TABLE <name>;
```
Returns schema information:
- Field definitions
- Index definitions
- Table configuration

**Alternative Approach (Not Used)**:
Could check if field exists specifically:
```sql
SELECT * FROM information_schema.columns
WHERE table_name = 'relationships'
AND column_name = 'strength';
```
But INFO FOR TABLE is simpler and confirms table exists.

**Migration Version History**:
- V1: Base schema (memory_nodes, relationships, migrations)
- V2: Vector index optimization
- V3: Validation/compatibility (strength already in V1)
- Future: V4+ for actual new features
