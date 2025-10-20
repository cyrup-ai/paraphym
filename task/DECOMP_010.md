# DECOMP_010: Decompose `chunk.rs`

**File:** `packages/candle/src/domain/context/chunk.rs`  
**Current Size:** 1,117 lines  
**Module Area:** domain / context

## OBJECTIVE

Decompose the monolithic `chunk.rs` (1,117 lines) into smaller, focused, maintainable modules while preserving all existing functionality.

## CONSTRAINTS

- **MAINTAIN FUNCTIONALITY:** All existing functionality must be preserved exactly as-is
- **SINGLE SESSION:** This task must be completable in one focused Claude session
- **NO BREAKING CHANGES:** Public API must remain identical

## ANALYSIS: CURRENT STRUCTURE

The file contains chunk types for streaming operations implementing the `MessageChunk` trait from `cyrup_sugars`. Analysis reveals these distinct logical groups:

### 1. Media Chunks (~250 lines)
- `CandleImageFormat` enum (PNG, JPEG, GIF, WebP, BMP, TIFF)
- `CandleImageChunk` struct
- `AudioFormat` enum (MP3, WAV, FLAC, OGG, M4A, OPUS)
- `VoiceChunk` struct
- `TranscriptionChunk` struct
- `SpeechChunk` struct

### 2. Completion Chunks (~150 lines)
- `FinishReason` enum (Stop, Length, ContentFilter, ToolCalls, Error)
- `CandleCompletionChunk` enum (Text, ToolCallStart, ToolCall, ToolCallComplete, Complete, Error)
- `ChatMessageChunk` struct

### 3. Document & Embedding Chunks (~100 lines)
- `CandleDocumentChunk` struct
- `EmbeddingChunk` struct

### 4. Result Types (~300 lines)
- `CandleResult<T>` wrapper
- `ParallelResult<T>` for N-way parallel operations
- `CandleRefreshResult`
- `CandleMemoryOperationResult`

### 5. Generic & Primitive Wrappers (~250 lines)
- `CandleUnitChunk` (unit type wrapper)
- `CandleStringChunk` (String wrapper)
- `CandleJsonChunk` (serde_json::Value wrapper)
- `CandleCollectionChunk<T>` (generic collection wrapper)
- `CandleUuidChunk` (UUID wrapper)
- `CandleBoolChunk` (bool wrapper)
- `CandleDurationChunk` (Duration wrapper)
- `CandleDateTimeChunk` (DateTime<Utc> wrapper)
- `CandleZeroOneOrManyChunk<T>` (ZeroOneOrMany wrapper)
- `ZeroOneOrManyF32Chunk` (specialized f32 wrapper)
- `duration_serde` module for Duration serialization

### 6. Workflow Chunks (~60 lines)
- `WorkflowDataChunk` struct

**Total:** ~1,110 lines (matches actual file size)

## DECOMPOSITION STRATEGY

Transform `chunk.rs` into a module directory: `chunk/`

### Directory Structure

```
packages/candle/src/domain/context/
├── chunk/
│   ├── mod.rs           # Module aggregator with re-exports (~50 lines)
│   ├── media.rs         # Media chunk types (~250 lines)
│   ├── completion.rs    # Completion streaming chunks (~150 lines)
│   ├── data.rs          # Document and embedding chunks (~100 lines)
│   ├── results.rs       # Result wrapper types (~300 lines)
│   ├── wrappers.rs      # Generic and primitive wrappers (~250 lines)
│   └── workflow.rs      # Workflow data chunks (~60 lines)
└── mod.rs              # NO CHANGES NEEDED - Rust handles chunk.rs vs chunk/mod.rs automatically
```

### Module Responsibilities

#### `media.rs` - Media Chunk Types
**Purpose:** Handle all media-related chunk types for streaming operations

**Exports:**
- `CandleImageFormat` - Image format enumeration
- `CandleImageChunk` - Image data chunks
- `AudioFormat` - Audio format enumeration
- `VoiceChunk` - Voice/audio data chunks
- `TranscriptionChunk` - Speech-to-text chunks
- `SpeechChunk` - Text-to-speech chunks

**Dependencies:**
```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;
```

**Code Pattern Example:**
```rust
//! Media Chunk Types
//!
//! Chunk types for streaming media operations including images, audio, 
//! voice, and speech data.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;

/// Candle image format types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CandleImageFormat {
    /// Portable Network Graphics format
    PNG,
    /// Joint Photographic Experts Group format
    JPEG,
    // ... etc
}

/// Candle chunk of image data for streaming image operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleImageChunk {
    /// Raw image data
    pub data: Vec<u8>,
    /// Image format
    pub format: CandleImageFormat,
    /// Optional dimensions (width, height)
    pub dimensions: Option<(u32, u32)>,
    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
```

#### `completion.rs` - Completion Streaming Chunks
**Purpose:** Handle LLM completion and chat message streaming

**Exports:**
- `FinishReason` - Why completion finished
- `CandleCompletionChunk` - Comprehensive completion chunk enum
- `ChatMessageChunk` - Chat message chunks

**Dependencies:**
```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;
use crate::domain::model::CandleUsage;
use crate::domain::chat::message::types::CandleMessageRole;
```

**Code Pattern Example:**
```rust
//! Completion Streaming Chunk Types
//!
//! Chunk types for LLM completion streaming including text generation,
//! tool calls, and chat messages.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;
use crate::domain::model::CandleUsage;
use crate::domain::chat::message::types::CandleMessageRole;

/// Reason why a completion finished
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FinishReason {
    /// Completion finished naturally at a stopping point
    Stop,
    /// Completion reached maximum token length limit
    Length,
    // ... etc
}
```

#### `data.rs` - Document and Embedding Chunks
**Purpose:** Handle document content and vector embedding chunks

**Exports:**
- `CandleDocumentChunk` - Document content chunks
- `EmbeddingChunk` - Vector embedding chunks

**Dependencies:**
```rust
use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::{ZeroOneOrMany, prelude::MessageChunk};
```

#### `results.rs` - Result Wrapper Types
**Purpose:** Provide MessageChunk-compliant result wrappers

**Exports:**
- `CandleResult<T>` - Result wrapper implementing MessageChunk
- `ParallelResult<T>` - Zero-cost result wrapper for N-way parallel operations
- `CandleRefreshResult` - Context refresh operation results
- `CandleMemoryOperationResult` - Memory operation results

**Dependencies:**
```rust
use serde::{Deserialize, Serialize};
use cyrup_sugars::prelude::MessageChunk;
```

**Key Performance Note:**
`ParallelResult<T>` is designed as a zero-runtime-overhead wrapper. Preserve all inline annotations and documentation about performance characteristics.

#### `wrappers.rs` - Generic and Primitive Wrappers
**Purpose:** Orphan rule compliant wrappers for external types

**Exports:**
- `CandleUnitChunk` - Unit type wrapper
- `CandleStringChunk` - String wrapper
- `CandleJsonChunk` - JSON Value wrapper
- `CandleCollectionChunk<T>` - Generic collection wrapper
- `CandleUuidChunk` - UUID wrapper
- `CandleBoolChunk` - Bool wrapper
- `CandleDurationChunk` - Duration wrapper (with duration_serde module)
- `CandleDateTimeChunk` - DateTime<Utc> wrapper
- `CandleZeroOneOrManyChunk<T>` - Generic ZeroOneOrMany wrapper
- `ZeroOneOrManyF32Chunk` - Specialized f32 wrapper

**Dependencies:**
```rust
use serde::{Deserialize, Serialize};
use cyrup_sugars::{ZeroOneOrMany, prelude::MessageChunk};
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;
```

**Important:** Keep the `duration_serde` private module for Duration serialization helpers.

#### `workflow.rs` - Workflow Data Chunks
**Purpose:** Handle workflow-specific data streaming

**Exports:**
- `WorkflowDataChunk` - Workflow JSON data chunks

**Dependencies:**
```rust
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;
use std::time::{SystemTime, UNIX_EPOCH};
```

#### `mod.rs` - Module Aggregator
**Purpose:** Declare submodules and re-export all public types

**Complete Implementation:**
```rust
//! Chunk Types for Streaming Operations
//!
//! These types represent partial data that flows through `AsyncStream<T>`
//! and are designed to work with the `NotResult` constraint.
//!
//! Originally consolidated from a monolithic chunk.rs file, now organized into
//! focused modules for better maintainability:
//!
//! - [`media`] - Image, audio, voice, and speech chunks
//! - [`completion`] - LLM completion and chat message chunks
//! - [`data`] - Document and embedding chunks
//! - [`results`] - Result wrapper types for async operations
//! - [`wrappers`] - Generic and primitive type wrappers
//! - [`workflow`] - Workflow-specific data chunks

pub mod media;
pub mod completion;
pub mod data;
pub mod results;
pub mod wrappers;
pub mod workflow;

// Re-export all public types to maintain flat namespace
pub use media::*;
pub use completion::*;
pub use data::*;
pub use results::*;
pub use wrappers::*;
pub use workflow::*;
```

## IMPLEMENTATION STEPS

### STEP 1: Create Directory Structure
```bash
cd /Volumes/samsung_t9/cyrup/packages/candle/src/domain/context
mkdir chunk
```

### STEP 2: Create mod.rs
Create `/Volumes/samsung_t9/cyrup/packages/candle/src/domain/context/chunk/mod.rs` with the complete implementation shown above.

### STEP 3: Extract and Create Submodules

For each submodule:

1. **Create the file** (e.g., `media.rs`)
2. **Add module documentation** describing purpose
3. **Add necessary imports** from dependencies
4. **Copy relevant types** from original chunk.rs
5. **Preserve all implementations** including:
   - Struct/enum definitions with doc comments
   - `Default` implementations
   - `MessageChunk` trait implementations
   - Helper methods (e.g., `CandleDocumentChunk::new()`)
   - Conversion traits (`From`, `Into`, `Deref`, `DerefMut`)
   - Trait bounds and generics

**Critical:** Maintain exact same code - only change imports and module location.

### STEP 4: Create Files in This Order

1. `media.rs` - Self-contained, no internal dependencies
2. `wrappers.rs` - Self-contained, only external dependencies
3. `data.rs` - Self-contained, uses external types
4. `completion.rs` - Depends on `crate::domain::model::CandleUsage` and `crate::domain::chat::message::types::CandleMessageRole`
5. `results.rs` - Self-contained generic wrappers
6. `workflow.rs` - Self-contained, uses serde_json::Value

### STEP 5: Delete Original File
```bash
rm /Volumes/samsung_t9/cyrup/packages/candle/src/domain/context/chunk.rs
```

### STEP 6: Verify Compilation
```bash
cd /Volumes/samsung_t9/cyrup/packages/candle
cargo check
```

Fix any import errors or visibility issues. Common issues:
- Missing `pub` keywords
- Incorrect import paths
- Missing trait bounds on generics

## IMPORT PATTERNS IN CODEBASE

Current usage patterns show imports like:
```rust
use crate::domain::context::chunk::*;
use crate::domain::context::chunk::CandleCompletionChunk;
```

These patterns will continue to work identically due to re-exports in `mod.rs`. No changes needed in dependent code.

## DEPENDENCIES BETWEEN TYPES

### External Dependencies
- `cyrup_sugars::prelude::MessageChunk` - Core trait all chunks implement
- `cyrup_sugars::ZeroOneOrMany` - Used in embedding and wrapper types
- `serde::{Serialize, Deserialize}` - Universal serialization
- `serde_json::Value` - Metadata storage
- `std::collections::HashMap` - Metadata maps
- `uuid::Uuid` - UUID wrapper
- `chrono::{DateTime, Utc}` - DateTime wrapper
- `std::time::Duration` - Duration wrapper

### Internal Dependencies
- `crate::domain::model::CandleUsage` - Used in `CandleCompletionChunk::Complete`
- `crate::domain::chat::message::types::CandleMessageRole` - Used in `ChatMessageChunk`

**No circular dependencies** - Clean module separation confirmed.

## PUBLIC API PRESERVATION

The re-export strategy in `mod.rs` ensures:

✅ `use crate::domain::context::chunk::*;` continues to work
✅ `use crate::domain::context::chunk::CandleImageChunk;` continues to work  
✅ All existing code compiles without modification
✅ No breaking changes to consumers

## DEFINITION OF DONE

- [x] `chunk.rs` deleted and replaced with `chunk/` directory
- [x] `chunk/mod.rs` created with module declarations and re-exports (<100 lines)
- [x] All 6 submodules created, each <300 lines:
  - [x] `media.rs` (~250 lines)
  - [x] `completion.rs` (~150 lines)
  - [x] `data.rs` (~100 lines)
  - [x] `results.rs` (~300 lines)
  - [x] `wrappers.rs` (~250 lines)
  - [x] `workflow.rs` (~60 lines)
- [x] All functionality preserved with identical behavior
- [x] Public API unchanged (verified by re-export structure)
- [x] `cargo check` passes without errors
- [x] All MessageChunk implementations preserved
- [x] All doc comments preserved
- [x] All helper methods and trait implementations preserved

## VERIFICATION COMMANDS

```bash
# Navigate to candle package
cd /Volumes/samsung_t9/cyrup/packages/candle

# Verify structure
ls -la src/domain/context/chunk/

# Expected output:
# mod.rs
# media.rs
# completion.rs
# data.rs
# results.rs
# wrappers.rs
# workflow.rs

# Check compilation
cargo check

# Expected: No errors, all existing tests pass
```

## NOTES

- This follows established patterns in the codebase (see `src/domain/agent/`, `src/builders/chat/`)
- Rust automatically handles `chunk.rs` vs `chunk/mod.rs` - parent `context/mod.rs` needs NO changes
- Each module is highly cohesive with a single responsibility
- No new code added - pure reorganization
- All performance characteristics preserved (especially `ParallelResult` zero-cost wrapper)
- Orphan rule compliance maintained through wrapper types

## REFERENCE PATTERNS FROM CODEBASE

Similar decompositions already exist:
- [`src/domain/agent/`](../../packages/candle/src/domain/agent/) - Decomposed into core.rs, chat.rs, role.rs, types.rs
- [`src/builders/chat/`](../../packages/candle/src/builders/chat/) - Multiple builder files with mod.rs aggregator
- [`src/capability/registry/pool/`](../../packages/candle/src/capability/registry/pool/) - Complex module hierarchy

This decomposition follows the same proven patterns.