# NODE_1: Implement Proper Memory Node Representation

## OBJECTIVE

Replace the stubbed `base_memory()` method that returns a reference to self with proper memory node abstraction and transformation logic.

## BACKGROUND

The memory node `base_memory()` method returns self "for now", suggesting a need for separate base and enriched memory representations.

## SUBTASK 1: Design Memory Node Architecture

**Location:** `packages/candle/src/memory/core/primitives/node.rs:138`

**Current State:**
```rust
/// Get base memory representation - returns a reference to self for now
pub fn base_memory(&self) -> &Self {
    self
}
```

**Required Changes:**
- Remove "for now" comment
- Define separate types: `BaseMemory` (core data) and `EnrichedMemory` (with computed fields)
- Document distinction between base and enriched memory
- Design transformation pipeline: base → enriched
- Consider using builder pattern or separate structs

**Why:** Base memory is stored data; enriched memory includes computed fields like embeddings and metadata.

## SUBTASK 2: Implement Base Memory Extraction

**Location:** Same file

**Required Changes:**
- Create `BaseMemory` struct with core fields: id, content, created_at
- Implement `to_base_memory(&self) -> BaseMemory` conversion
- Strip computed/transient fields (embeddings, search scores, cached data)
- Ensure base memory is serializable for storage
- Document what belongs in base vs enriched

**Why:** Base memory represents persistent storage; enriched includes runtime additions.

## SUBTASK 3: Implement Memory Enrichment

**Location:** Same file or related module

**Required Changes:**
- Add `enrich()` method to compute derived fields
- Add embedding generation for base memories
- Add relationship loading for enriched memories
- Add metadata computation (usage stats, relevance scores)
- Cache enriched data to avoid recomputation

**Why:** Enrichment adds computed data without modifying stored base memory.

## SUBTASK 4: Update Memory Operations

**Location:** Memory operations that use base_memory()

**Required Changes:**
- Use `to_base_memory()` for storage operations
- Use enriched memory for search and retrieval
- Update serialization to use base memory
- Update deserialization to create base then enrich
- Ensure lazy enrichment where appropriate

**Why:** Storage and retrieval need to use correct memory representation.

## DEFINITION OF DONE

- [ ] No "for now" comments in memory node code
- [ ] Clear separation between BaseMemory and EnrichedMemory
- [ ] `to_base_memory()` extracts core persistent data
- [ ] Enrichment process adds computed fields
- [ ] Storage operations use base memory only
- [ ] Documentation explains memory representation model
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Memory Representation Pattern
- Base memory: pure data, no computation, serializable
- Enriched memory: includes embeddings, relationships, scores
- Separation enables efficient storage and caching

### Fields Classification
**Base (persistent):**
- id, content, created_at, updated_at
- user-provided metadata

**Enriched (computed):**
- embedding vectors
- relationship graph data
- search relevance scores
- usage statistics

### Integration Points
- Memory storage in SurrealDB
- Vector search operations
- Memory retrieval and caching
- Serialization for export/import

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Maintain backward compatibility with existing memory APIs
- Ensure enrichment is lazy/cached for performance
