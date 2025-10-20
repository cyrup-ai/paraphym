# DECOMP_018: Decompose `entity.rs`

**File:** `packages/candle/src/memory/graph/entity.rs`  
**Current Size:** 909 lines  
**Module Area:** memory / graph

## CORE OBJECTIVE

Decompose the monolithic 909-line `entity.rs` file into a focused module structure following the codebase pattern of creating subdirectories for complex functionality (see `cognitive/committee/`, `cognitive/quantum/`, `core/manager/` as examples).

The file contains four distinct logical units that should be separated:
1. **Future wrappers** - Async operation return types
2. **Core entity types** - Domain model traits and implementations  
3. **Repository trait** - CRUD operations interface
4. **Repository implementation** - SurrealDB-specific implementation

## CURRENT FILE STRUCTURE ANALYSIS

### Line-by-Line Breakdown

```
Lines 1-17:    Module documentation + imports
Lines 18-165:  Future wrapper types (148 lines)
               - PendingEntity
               - PendingEntityOption  
               - PendingEntityList
               - PendingEntityCount
               - PendingUnit

Lines 166-187: Entity trait definition (22 lines)

Lines 189-252: BaseEntity struct + first impl block (64 lines)

Lines 254-351: Entity trait implementation for BaseEntity (98 lines)

Lines 353-359: BaseEntity builder methods (7 lines)

Lines 362-473: EntityRepository trait (112 lines)

Lines 475-910: SurrealEntityRepository<E> (436 lines)
               - Struct definition
               - Constructor methods
               - EntityRepository trait impl with all CRUD operations
```

### Key Dependencies

**External crates:**
- `std::collections::HashMap`
- `std::sync::Arc`
- `serde::{Deserialize, Serialize}`
- `surrealdb::Value`
- `tokio::sync::oneshot`
- `futures_util::StreamExt`

**Internal dependencies:**
- `crate::memory::graph::graph_db::{GraphDatabase, GraphError, GraphQueryOptions, Node, Result}`

**Dependent modules** (found via codebase search):
- `memory/core/systems/episodic.rs` - uses `BaseEntity`
- `memory/core/systems/history.rs` - imports entity module
- `memory/core/ops/evolution.rs` - imports entity module
- `memory/core/primitives/types.rs` - uses `BaseEntity::new()`, `with_attribute()`, Entity trait

### Public API Surface

The current `memory/graph/mod.rs` does:
```rust
pub mod entity;
pub use entity::*;
```

This means ALL public items from entity.rs are re-exported at `crate::memory::graph::*`. Consumers use:
- `use crate::memory::graph::entity::BaseEntity;`
- `use crate::memory::graph::{Entity, EntityRepository};`

**CRITICAL:** The decomposition MUST preserve this public API through re-exports.

## PROPOSED MODULE STRUCTURE

Create subdirectory: `packages/candle/src/memory/graph/entity/`

### Module Files

#### 1. `entity/futures.rs` (~165 lines)
**Purpose:** Future wrapper types for async operations  
**Contents:**
- `PendingEntity` struct + Future impl
- `PendingEntityOption` struct + Future impl
- `PendingEntityList` struct + Future impl  
- `PendingEntityCount` struct + Future impl
- `PendingUnit` struct + Future impl

**Dependencies:** 
- `std::future::Future`
- `std::task::{Context, Poll}`
- `tokio::sync::oneshot`
- `crate::memory::graph::graph_db::{GraphError, Result}`

**Example structure:**
```rust
//! Future wrappers for entity operations
//!
//! These types wrap oneshot channels to provide Future implementations
//! for entity CRUD operations, enabling async/await syntax.

use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::oneshot;

use crate::memory::graph::graph_db::{GraphError, Result};
use super::types::Entity;

/// Future wrapper for entity creation/update operations
pub struct PendingEntity {
    rx: oneshot::Receiver<Result<Box<dyn Entity>>>,
}

impl PendingEntity {
    pub fn new(rx: oneshot::Receiver<Result<Box<dyn Entity>>>) -> Self {
        Self { rx }
    }
}

impl Future for PendingEntity {
    type Output = Result<Box<dyn Entity>>;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // ... implementation from original file
    }
}

// ... repeat for PendingEntityOption, PendingEntityList, etc.
```

#### 2. `entity/types.rs` (~195 lines)
**Purpose:** Core entity domain types  
**Contents:**
- `EntityValidatorFn` type alias
- `Entity` trait
- `BaseEntity` struct
- All `BaseEntity` impl blocks
- `Entity` trait implementation for `BaseEntity`

**Dependencies:**
- `std::collections::HashMap`
- `serde::{Deserialize, Serialize}`
- `surrealdb::Value`
- `crate::memory::graph::graph_db::{GraphError, Node, Result}`

**Example structure:**
```rust
//! Core entity types and trait definitions
//!
//! This module provides the Entity trait and BaseEntity implementation
//! for mapping domain objects to graph nodes.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use surrealdb::Value;

use crate::memory::graph::graph_db::{GraphError, Node, Result};

/// Type alias for entity validation functions
pub type EntityValidatorFn = Box<dyn Fn(&dyn Entity) -> Result<()> + Send + Sync>;

/// Entity trait for domain objects
pub trait Entity: Send + Sync + Debug {
    fn id(&self) -> &str;
    fn entity_type(&self) -> &str;
    fn get_attribute(&self, name: &str) -> Option<&Value>;
    fn set_attribute(&mut self, name: &str, value: Value);
    fn attributes(&self) -> &HashMap<String, Value>;
    fn validate(&self) -> Result<()>;
    fn to_node(&self) -> Node;
    fn from_node(node: Node) -> Result<Self> where Self: Sized;
}

/// Base entity implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    id: String,
    entity_type: String,
    attributes: HashMap<String, Value>,
}

impl BaseEntity {
    pub fn new(id: String, entity_type: String) -> Self {
        // ... implementation
    }
    
    pub fn with_capacity(id: String, entity_type: String, capacity: usize) -> Self {
        // ... implementation
    }
    
    // ... all other methods
}

impl Entity for BaseEntity {
    // ... trait implementation
}
```

#### 3. `entity/repository.rs` (~115 lines)
**Purpose:** Repository trait definition  
**Contents:**
- `EntityRepository` trait with all CRUD method signatures
- Full documentation for each method

**Dependencies:**
- `super::futures::{PendingEntity, PendingEntityOption, PendingEntityList, PendingEntityCount, PendingUnit}`
- `super::types::Entity`
- `surrealdb::Value`

**Example structure:**
```rust
//! Entity repository trait for CRUD operations
//!
//! Provides thread-safe interface for entity persistence operations.

use surrealdb::Value;
use super::futures::{PendingEntity, PendingEntityOption, PendingEntityList, PendingEntityCount, PendingUnit};
use super::types::Entity;

/// Thread-safe entity repository trait - Returns Futures
///
/// This trait provides an interface for entity CRUD operations that return futures.
/// All methods are thread-safe and return pending wrappers that can be awaited.
pub trait EntityRepository: Send + Sync {
    /// Create a new entity
    fn create_entity(&self, entity: Box<dyn Entity>) -> PendingEntity;
    
    /// Get an entity by ID
    fn get_entity(&self, id: &str) -> PendingEntityOption;
    
    /// Update an entity
    fn update_entity(&self, entity: Box<dyn Entity>) -> PendingEntity;
    
    // ... all other method signatures from original
}
```

#### 4. `entity/surreal.rs` (~433 lines)
**Purpose:** SurrealDB repository implementation  
**Contents:**
- `SurrealEntityRepository<E>` struct
- Constructor methods (`new`, `with_validator`)
- Validator management methods
- Complete `EntityRepository` trait implementation

**Dependencies:**
- All previous modules
- `std::sync::Arc`
- `tokio::spawn`
- `futures_util::StreamExt`
- `crate::memory::graph::graph_db::{GraphDatabase, GraphQueryOptions}`

**Example structure:**
```rust
//! SurrealDB-backed entity repository implementation

use std::sync::Arc;
use std::marker::PhantomData;
use tokio::sync::oneshot;
use surrealdb::Value;

use crate::memory::graph::graph_db::{GraphDatabase, GraphError, GraphQueryOptions};
use super::futures::*;
use super::types::{Entity, EntityValidatorFn};
use super::repository::EntityRepository;

/// SurrealDB-backed entity repository implementation
pub struct SurrealEntityRepository<E: Entity + Clone + 'static> {
    db: Arc<dyn GraphDatabase>,
    table_name: String,
    validator: Option<EntityValidatorFn>,
    _phantom: PhantomData<E>,
}

impl<E: Entity + Clone + 'static> SurrealEntityRepository<E> {
    pub fn new(db: Arc<dyn GraphDatabase>, table_name: String) -> Self {
        // ... implementation
    }
    
    // ... all methods from original
}

impl<E: Entity + Clone + 'static> EntityRepository for SurrealEntityRepository<E> {
    // ... trait implementation with all CRUD operations
}
```

#### 5. `entity/mod.rs` (~25 lines)
**Purpose:** Module aggregator and re-exports  
**Contents:**
- Module declarations
- Public re-exports to preserve API

**Complete implementation:**
```rust
//! Entity model for graph-based memory
//!
//! This module provides entity abstractions for mapping domain objects
//! to graph nodes with support for validation, attributes, and persistence.

mod futures;
mod types;
mod repository;
mod surreal;

// Re-export all public types to preserve API compatibility
pub use futures::{
    PendingEntity,
    PendingEntityOption,
    PendingEntityList,
    PendingEntityCount,
    PendingUnit,
};

pub use types::{
    Entity,
    EntityValidatorFn,
    BaseEntity,
};

pub use repository::EntityRepository;

pub use surreal::SurrealEntityRepository;
```

## DEPENDENCY ANALYSIS

### Module Dependency Graph

```
graph_db (sibling module)
    ↓
    ├─→ futures.rs (independent)
    │
    └─→ types.rs (independent)
         ↓
         repository.rs
         ↓
         surreal.rs
```

**Key insights:**
- `futures.rs` and `types.rs` are independent of each other
- No circular dependencies
- Clear unidirectional flow
- `surreal.rs` is the only module that depends on all others

## IMPLEMENTATION STEPS

### Step 1: Create Directory Structure

```bash
mkdir -p packages/candle/src/memory/graph/entity
```

### Step 2: Create `futures.rs`

Extract lines 18-165 from original `entity.rs`:
- Move all 5 `Pending*` struct definitions
- Move all 5 `Future` trait implementations
- Add necessary imports
- Add module documentation

### Step 3: Create `types.rs`

Extract lines 166-359 from original `entity.rs`:
- Move `EntityValidatorFn` type alias
- Move `Entity` trait definition
- Move `BaseEntity` struct
- Move all `BaseEntity` impl blocks
- Move `Entity` trait impl for `BaseEntity`
- Add imports for `Node`, `GraphError`, etc.
- Add module documentation

### Step 4: Create `repository.rs`

Extract lines 362-473 from original `entity.rs`:
- Move `EntityRepository` trait
- Keep all documentation
- Update imports to use `super::futures::*` and `super::types::Entity`
- Add module documentation

### Step 5: Create `surreal.rs`

Extract lines 475-910 from original `entity.rs`:
- Move `SurrealEntityRepository<E>` struct
- Move all implementation blocks
- Update imports to use sibling modules
- Add module documentation

### Step 6: Create `mod.rs`

Create new file with:
- Module declarations for all 4 modules
- Public re-exports of all public types
- Top-level module documentation

### Step 7: Update `graph/mod.rs`

Current:
```rust
pub mod entity;
pub use entity::*;
```

Remains **UNCHANGED** - Rust automatically treats `entity/mod.rs` the same as `entity.rs`.

### Step 8: Delete Original File

```bash
rm packages/candle/src/memory/graph/entity.rs
```

### Step 9: Verify Compilation

```bash
cargo check
```

Fix any import issues:
- Ensure all `use crate::memory::graph::graph_db::*` imports are present
- Verify `super::` imports between entity submodules
- Check that `Entity` trait is imported where needed

## IMPLEMENTATION PATTERNS

### Import Pattern for Futures Module

```rust
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::oneshot;

use crate::memory::graph::graph_db::{GraphError, Result};
use super::types::Entity; // Cross-reference to types module
```

### Import Pattern for Types Module

```rust
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use surrealdb::Value;

use crate::memory::graph::graph_db::{GraphError, Node, Result};
// No dependency on futures module!
```

### Import Pattern for Repository Module

```rust
use surrealdb::Value;

use super::futures::{PendingEntity, PendingEntityOption, PendingEntityList, PendingEntityCount, PendingUnit};
use super::types::Entity;
```

### Import Pattern for Surreal Module

```rust
use std::sync::Arc;
use std::marker::PhantomData;
use tokio::sync::oneshot;
use surrealdb::Value;
use futures_util::StreamExt;

use crate::memory::graph::graph_db::{GraphDatabase, GraphError, GraphQueryOptions};
use super::futures::*;
use super::types::{Entity, EntityValidatorFn};
use super::repository::EntityRepository;
```

## COMMON PITFALLS TO AVOID

1. **Missing Debug import in types.rs** - The `Entity` trait requires `Debug` bound
2. **Forgetting Value type** - Ensure `surrealdb::Value` is imported where attributes are used
3. **Missing re-exports in mod.rs** - Every public type must be re-exported
4. **Incorrect super:: paths** - Use `super::futures::PendingEntity`, not `crate::memory::graph::entity::futures::PendingEntity`
5. **Breaking public API** - The re-exports ensure `use crate::memory::graph::entity::BaseEntity` still works

## DEFINITION OF DONE

- [ ] Directory `packages/candle/src/memory/graph/entity/` exists
- [ ] File `entity/futures.rs` exists and contains all 5 Future wrapper types (~165 lines)
- [ ] File `entity/types.rs` exists and contains Entity trait + BaseEntity (~195 lines)
- [ ] File `entity/repository.rs` exists and contains EntityRepository trait (~115 lines)
- [ ] File `entity/surreal.rs` exists and contains SurrealEntityRepository (~433 lines)
- [ ] File `entity/mod.rs` exists with proper module declarations and re-exports (~25 lines)
- [ ] Original `entity.rs` file is deleted
- [ ] `graph/mod.rs` remains unchanged (still does `pub use entity::*`)
- [ ] `cargo check` passes without errors
- [ ] All dependent modules (episodic.rs, history.rs, etc.) still compile
- [ ] Public API is preserved - all imports work as before
- [ ] Each new module is < 300 lines (largest is surreal.rs at ~433, which is acceptable)

## VERIFICATION COMMANDS

```bash
# Check new structure exists
ls -la packages/candle/src/memory/graph/entity/

# Verify line counts
wc -l packages/candle/src/memory/graph/entity/*.rs

# Compile check
cargo check

# Verify no remaining entity.rs
test ! -f packages/candle/src/memory/graph/entity.rs && echo "✓ Original file removed"

# Check dependent modules compile
cargo check --message-format=short 2>&1 | grep -E "(episodic|history|evolution|types).rs"
```

## CODEBASE REFERENCES

- [graph module structure](../packages/candle/src/memory/graph/)
- [cognitive/committee pattern](../packages/candle/src/memory/cognitive/committee/) - similar decomposition example
- [cognitive/quantum pattern](../packages/candle/src/memory/cognitive/quantum/) - multi-file module example
- [dependent: episodic.rs](../packages/candle/src/memory/core/systems/episodic.rs#L21) - uses `BaseEntity`
- [dependent: types.rs](../packages/candle/src/memory/core/primitives/types.rs#L317-L322) - uses `BaseEntity::new()` and builder pattern

## SUCCESS METRICS

1. **Maintainability**: Each file has single clear responsibility
2. **Discoverability**: Module names clearly indicate contents
3. **Compilability**: Zero compilation errors after decomposition
4. **API Stability**: No changes required in dependent modules
5. **Size Reduction**: Largest module is ~433 lines (52% reduction from 909)