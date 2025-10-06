# Verify Memory System Deduplication & Age Management

## Objective

Verify that the memory system already implements content deduplication and age management for context documents. If missing or incomplete, enhance the system to support these features.

## Background

Context documents loaded via `--context` flag must be:
1. **Deduplicated by content hashsum** - Never ingest the same document twice
2. **Age-refreshed on re-ingestion** - Update document age to "brand new" when identical content is re-ingested
3. **Decay model aware** - Document age affects retrieval priority in memory system

## Requirements

### 1. Deduplication by Hashsum

**Expected Behavior:**
- Calculate hashsum (SHA-256 or similar) of document content
- Before ingestion, check if document with same hashsum exists
- If exists: Skip ingestion, proceed to age update
- If new: Ingest document with hashsum stored

**Hashsum Sources:**
- File content (not file path - same content in different files = same hashsum)
- URL content (after fetching)
- Literal text string

### 2. Age Management

**Expected Behavior:**
- Each document has an `age` or `timestamp` field
- On re-ingestion of identical content:
  - Find existing document by hashsum
  - Update its age/timestamp to current time ("brand new")
  - Do NOT create duplicate entry

**Age Impact:**
- Fresh documents (recent age) have higher retrieval priority
- Decay model reduces relevance of older documents
- Re-ingestion "freshens" frequently referenced documents

### 3. Context Ingestion Flow

```rust
// Pseudo-code for CLI context ingestion

for context_input in cli_args.contexts {
    // 1. Resolve input (file/URL/text) using util/input_resolver.rs
    let document = resolve_and_load(&context_input).await?;
    
    // 2. Calculate content hashsum
    let content_hash = hash_document_content(&document.data);
    
    // 3. Check if document exists by hashsum
    if memory.document_exists_by_hash(content_hash).await? {
        // Document exists - refresh age only
        memory.update_document_age_by_hash(content_hash, now()).await?;
        eprintln!("✓ Context already exists, age refreshed: {}", context_input);
    } else {
        // New document - ingest with hashsum
        memory.ingest_document_with_hash(document, content_hash).await?;
        eprintln!("✓ Context ingested: {}", context_input);
    }
}
```

## Verification Tasks

### Step 1: Locate Memory Implementation

- [ ] Find memory manager implementation
  - Likely in: `packages/candle/src/memory/core/manager/`
  - Look for: SurrealDB integration, document storage
  
- [ ] Identify document schema
  - Fields: `id`, `content`, `hash/hashsum`, `age/timestamp`, `metadata`
  - Check if hashsum field exists
  - Check if age/timestamp field exists

### Step 2: Check Existing Deduplication

- [ ] Search for hashsum calculation
  - Pattern: `hash`, `sha256`, `blake3`, `DefaultHasher`
  - Location: Memory manager, document ingestion code
  
- [ ] Search for deduplication logic
  - Pattern: `find_by_hash`, `exists_by_hash`, `duplicate`
  - Check if ingestion checks for existing documents

### Step 3: Check Age Management

- [ ] Search for age/timestamp fields
  - Pattern: `age`, `timestamp`, `created_at`, `updated_at`
  - Check if documents track temporal information
  
- [ ] Search for age update logic
  - Pattern: `update_age`, `refresh_age`, `update_timestamp`
  - Check if re-ingestion updates existing documents

### Step 4: Verify Decay Model

- [ ] Search for decay/relevance scoring
  - Pattern: `decay`, `relevance`, `score`, `recency`
  - Location: Memory retrieval, vector search
  
- [ ] Check if age affects retrieval
  - Age-based weighting in search results
  - Freshness bonus for recent documents

## Enhancement Requirements (If Missing)

### If Deduplication Missing:

**Add to Memory Manager:**
```rust
// In memory/core/manager/mod.rs or surreal.rs

impl MemoryManager {
    /// Check if document exists by content hash
    async fn document_exists_by_hash(&self, hash: u64) -> Result<bool, MemoryError> {
        // Query SurrealDB for document with matching hash
        // SELECT * FROM documents WHERE content_hash = $hash LIMIT 1
    }
    
    /// Find document by content hash
    async fn find_document_by_hash(&self, hash: u64) -> Result<Option<Document>, MemoryError> {
        // Return full document if found
    }
}
```

### If Age Management Missing:

**Add to Memory Manager:**
```rust
impl MemoryManager {
    /// Update document age/timestamp by hash
    async fn update_document_age_by_hash(&self, hash: u64, timestamp: i64) -> Result<(), MemoryError> {
        // UPDATE documents SET age = $timestamp WHERE content_hash = $hash
    }
}
```

**Add to Document Schema:**
```rust
pub struct Document {
    pub id: String,
    pub content: String,
    pub content_hash: u64,  // Add if missing
    pub age: i64,           // Unix timestamp - Add if missing
    pub metadata: HashMap<String, String>,
}
```

### If Decay Model Missing:

**Add to Retrieval Logic:**
```rust
// In memory retrieval/search code

fn calculate_relevance_score(
    vector_similarity: f64,
    document_age: i64,
    current_time: i64,
) -> f64 {
    let age_seconds = current_time - document_age;
    let decay_factor = (-age_seconds as f64 / DECAY_HALF_LIFE).exp();
    
    // Combine vector similarity with freshness
    vector_similarity * (0.7 + 0.3 * decay_factor)
}
```

## Files to Inspect

### Memory Core:
- `packages/candle/src/memory/core/manager/mod.rs`
- `packages/candle/src/memory/core/manager/surreal.rs`
- `packages/candle/src/memory/core/manager/coordinator.rs`

### Memory Schema:
- `packages/candle/src/memory/schema/`
- Look for document/entity definitions

### Vector/Retrieval:
- `packages/candle/src/memory/vector/vector_repository.rs`
- `packages/candle/src/memory/vector/vector_index.rs`

### Context/Document:
- `packages/candle/src/domain/context/`
- Document ingestion pipeline

## Success Criteria

- ✅ Memory system has hashsum-based deduplication
- ✅ Re-ingesting identical content updates age to "brand new"
- ✅ Decay model incorporates document age in retrieval scoring
- ✅ CLI context ingestion uses existing memory methods (no duplication of logic)

## Related Task Files

- `task/fix-cli-options.md` - Question 9 documents context ingestion requirements
- CLI should call existing memory methods, not reimplement deduplication logic

## Notes

- User confirmed: "that functionality likely already exists in the memory system (deep)"
- Goal: Verify and document what exists, enhance only if needed
- Do NOT duplicate deduplication logic in CLI - use what's in memory system
