# MIGARCH_2: Fix MigrationManager Integration

## OBJECTIVE

Complete the SchemaTracker persistence integration by updating MigrationManager to use the new persistence methods and fix the rollback bug.

## CONTEXT

### What's Already Done ✅

SchemaTracker ([schema_migrations.rs](../packages/candle/src/memory/migration/schema_migrations.rs)) has all required persistence methods implemented:

1. **`load_from_db(db: &Surreal<Any>) -> Result<Self>`** (lines 71-88)
   - Queries `schema_migrations` table
   - Builds HashMap of applied migrations
   - Returns populated SchemaTracker

2. **`save_to_db(&self, db: &Surreal<Any>) -> Result<()>`** (lines 90-109)
   - Deletes all existing records
   - Inserts current tracker state
   - Atomic persistence

3. **`remove_migration(&mut self, version: u32) -> Option<MigrationRecord>`** (lines 111-114)
   - Removes migration from tracker's HashMap
   - Returns the removed record if it existed

### The Problem

MigrationManager ([mod.rs](../packages/candle/src/memory/migration/mod.rs)) still uses manual persistence patterns:

1. **Lines 117-130**: Manual loading in `new()` instead of using `load_from_db()`
2. **Line 225**: Missing `tracker.remove_migration()` after `db.delete()` in `rollback_to()`

This causes a critical bug: after rollback, the tracker still thinks migrations are applied, causing `is_applied()` to return incorrect results.

## THE BUG EXPLAINED

**File**: [packages/candle/src/memory/migration/mod.rs](../packages/candle/src/memory/migration/mod.rs)

**Current rollback_to() flow** (lines 200-228):
```rust
pub async fn rollback_to(&mut self, target_version: u32) -> Result<()> {
    // ... loop through migrations ...
    
    migration.down(Arc::clone(&self.db)).await?;
    
    // Remove from database ✅
    self.db.delete::<Option<MigrationRecord>>(
        ("schema_migrations", format!("v{}", version))
    ).await?;
    
    // ❌ BUG: Tracker not updated! 
    // DB says: migration removed
    // Tracker says: migration still applied
    // Result: is_applied() returns TRUE for a rolled-back migration
}
```

**Impact**: 
- `tracker.is_applied(version)` returns `true` for rolled-back migrations
- Re-running migrations skips them (thinks they're applied)
- Tracker state becomes permanently stale until restart

## REQUIRED CHANGES

### Change 1: Use load_from_db() in new()

**File**: `packages/candle/src/memory/migration/mod.rs`  
**Lines**: 117-130

**Current code** (14 lines):
```rust
// Load existing migration records
let mut tracker = SchemaTracker::new();

// Query all applied migrations from database
let query = "SELECT * FROM schema_migrations";
let mut response = db.query(query)
    .await
    .map_err(|e| MigrationError::DatabaseError(format!("{:?}", e)))?;

let records: Vec<MigrationRecord> = response.take(0).unwrap_or_default();

for record in records {
    tracker.record_migration(record.version, record.name, record.checksum);
}
```

**Replace with** (1 line):
```rust
// Load existing migration records using SchemaTracker's persistence
let tracker = SchemaTracker::load_from_db(&db).await?;
```

**Why this works**:
- SchemaTracker::load_from_db() is a static method (see [schema_migrations.rs:71](../packages/candle/src/memory/migration/schema_migrations.rs#L71))
- Takes `&Surreal<Any>` reference (same as current code uses)
- Returns `Result<Self>` - error propagates correctly with `?`
- Internally does exact same query + HashMap building
- Eliminates 13 lines of code duplication

### Change 2: Fix Rollback Bug

**File**: `packages/candle/src/memory/migration/mod.rs`  
**Line**: 225 (immediately after `db.delete()`)

**Current code** (lines 223-226):
```rust
// Remove from database
self.db.delete::<Option<MigrationRecord>>(("schema_migrations", format!("v{}", version)))
    .await
    .map_err(|e| MigrationError::DatabaseError(format!("{:?}", e)))?;
```

**Add immediately after** (1 line):
```rust
// Remove from database
self.db.delete::<Option<MigrationRecord>>(("schema_migrations", format!("v{}", version)))
    .await
    .map_err(|e| MigrationError::DatabaseError(format!("{:?}", e)))?;

// FIX: Update tracker to reflect rollback
self.tracker.remove_migration(version);
```

**Why this works**:
- `remove_migration(&mut self, version: u32)` is a mutable instance method (see [schema_migrations.rs:111](../packages/candle/src/memory/migration/schema_migrations.rs#L111))
- `self.tracker` is accessible (see [mod.rs:106](../packages/candle/src/memory/migration/mod.rs#L106))
- Returns `Option<MigrationRecord>` but we don't need the return value
- Keeps tracker in sync with database state

## MIGRATION FLOW ANALYSIS

### Current State (Before Changes)

**On startup** ([mod.rs:111-135](../packages/candle/src/memory/migration/mod.rs#L111)):
```
MigrationManager::new()
├─ Create schema_migrations table
├─ Query all records from DB
├─ Loop: tracker.record_migration() for each
└─ Return MigrationManager with populated tracker
```

**On migration** ([mod.rs:143-196](../packages/candle/src/memory/migration/mod.rs#L143)):
```
MigrationManager::migrate()
├─ Check: tracker.is_applied() ✅
├─ Execute: migration.up()
├─ Save: db.create() ✅
├─ Update: tracker.record_migration() ✅
└─ Tracker and DB in sync ✅
```

**On rollback** ([mod.rs:200-228](../packages/candle/src/memory/migration/mod.rs#L200)):
```
MigrationManager::rollback_to()
├─ Check: tracker.is_applied() ✅
├─ Execute: migration.down()
├─ Delete: db.delete() ✅
└─ Update: ❌ MISSING!
    Result: Tracker thinks migration still applied
```

### After Changes

**On startup** (new):
```
MigrationManager::new()
├─ Create schema_migrations table
├─ Load: tracker = SchemaTracker::load_from_db() ✅
└─ Return MigrationManager (1 line instead of 14)
```

**On rollback** (fixed):
```
MigrationManager::rollback_to()
├─ Check: tracker.is_applied() ✅
├─ Execute: migration.down()
├─ Delete: db.delete() ✅
├─ Update: tracker.remove_migration() ✅
└─ Tracker and DB in sync ✅
```

## CONSTRAINTS

- **NO** unwrap() or expect() in src/* code
- **NO** changes outside migration module
- **NO** new functionality - just connecting existing code
- **NO** modification to SchemaTracker (already complete)
- Make ONLY the two surgical changes specified above

## DEFINITION OF DONE

### Required Changes
- [ ] Replace lines 117-130 in mod.rs with `SchemaTracker::load_from_db(&db).await?`
- [ ] Add `self.tracker.remove_migration(version);` after line 225 in mod.rs

### Validation
- [ ] Code compiles: `cargo check -p paraphym_candle`
- [ ] Code passes linting: `cargo clippy -p paraphym_candle`

## FILE REFERENCES

### Source Files
- **[packages/candle/src/memory/migration/mod.rs](../packages/candle/src/memory/migration/mod.rs)** - MigrationManager (needs changes)
- **[packages/candle/src/memory/migration/schema_migrations.rs](../packages/candle/src/memory/migration/schema_migrations.rs)** - SchemaTracker (complete)

### Key Locations

**SchemaTracker methods** (schema_migrations.rs):
- Clone derive: [line 30](../packages/candle/src/memory/migration/schema_migrations.rs#L30)
- load_from_db: [lines 71-88](../packages/candle/src/memory/migration/schema_migrations.rs#L71)
- save_to_db: [lines 90-109](../packages/candle/src/memory/migration/schema_migrations.rs#L90)
- remove_migration: [lines 111-114](../packages/candle/src/memory/migration/schema_migrations.rs#L111)

**MigrationManager code** (mod.rs):
- Struct definition: [line 103](../packages/candle/src/memory/migration/mod.rs#L103)
- tracker field: [line 106](../packages/candle/src/memory/migration/mod.rs#L106)
- new() method: [lines 111-135](../packages/candle/src/memory/migration/mod.rs#L111)
- Manual loading (REMOVE): [lines 117-130](../packages/candle/src/memory/migration/mod.rs#L117)
- migrate() method: [lines 143-196](../packages/candle/src/memory/migration/mod.rs#L143)
- rollback_to() method: [lines 200-228](../packages/candle/src/memory/migration/mod.rs#L200)
- Missing tracker update: [after line 225](../packages/candle/src/memory/migration/mod.rs#L225)

**Usage location** (surreal.rs):
- run_migrations(): [line 382](../packages/candle/src/memory/core/manager/surreal.rs#L382) - creates MigrationManager

## IMPLEMENTATION PATTERNS

### Pattern 1: Static Method Call
```rust
// Before: manual implementation
let mut tracker = SchemaTracker::new();
let query = "SELECT * FROM schema_migrations";
// ... 12 more lines

// After: use static method
let tracker = SchemaTracker::load_from_db(&db).await?;
```

### Pattern 2: Mutable Method Call  
```rust
// Before: missing sync
self.db.delete::<Option<MigrationRecord>>(
    ("schema_migrations", format!("v{}", version))
).await?;
// tracker not updated ❌

// After: keep in sync
self.db.delete::<Option<MigrationRecord>>(
    ("schema_migrations", format!("v{}", version))
).await?;
self.tracker.remove_migration(version); // ✅
```

### Pattern 3: Error Propagation
```rust
// Both changes use ? operator for error propagation
load_from_db(&db).await?  // Returns Result<Self>
// If error: propagates up to caller
// If ok: returns SchemaTracker
```

## EDGE CASES HANDLED

### Change 1: load_from_db()
- **Empty DB**: Returns SchemaTracker with empty HashMap ✓
- **Load failure**: Error propagates via `?` operator ✓
- **No records**: `take(0).unwrap_or_default()` returns empty Vec ✓

### Change 2: remove_migration()
- **Version not in tracker**: Returns None, no error ✓
- **Mutable borrow**: Already have `&mut self` in rollback_to() ✓
- **Return value**: Discarded (we don't need it) ✓

## WHY THIS TASK EXISTS

The original implementation was written before SchemaTracker had persistence methods. The manual patterns were reasonable at the time but now create:

1. **Code duplication**: Loading logic exists in both places
2. **Bug potential**: Easy to forget tracker updates (rollback bug)
3. **Maintenance burden**: Two places to update for persistence changes

This task eliminates all three issues by using SchemaTracker's persistence layer consistently.

## VERIFICATION

After implementation, verify the fix works:

```bash
# 1. Code compiles
cargo check -p paraphym_candle

# 2. No linting issues  
cargo clippy -p paraphym_candle

# 3. Verify changes are minimal
git diff packages/candle/src/memory/migration/mod.rs
# Should show:
# - Line 117-130: replaced with single load_from_db() call
# - After line 225: added single remove_migration() call
```
