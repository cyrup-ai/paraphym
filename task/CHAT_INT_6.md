# CHAT_INT_6: Implement Keyword Extraction for Context Documents

## OBJECTIVE
Implement automatic keyword extraction from context documents to enable precise semantic search and memory retrieval. Currently context documents are stored with empty keywords, severely limiting search effectiveness.

## SEVERITY: CRITICAL FUNCTIONALITY GAP

**Current Behavior**:
```rust
// Line 123 in session.rs
keywords: vec![],  // ❌ EMPTY - no keyword indexing
```

**Impact**:
- Context documents cannot be found by keyword search
- Semantic memory search is degraded without keywords
- User queries fail to retrieve relevant context
- **Memory system is effectively blind to context content**
- Same critical gap exists for conversation memories (line 492)

## CURRENT STATUS

**Files Affected**:
- `packages/candle/src/domain/chat/session.rs:123, 155, 187, 219` (context loading)
- `packages/candle/src/domain/chat/session.rs:492, 502` (conversation storage)

**Issues**:
1. ❌ No keyword extraction from document content
2. ❌ Search queries cannot match context documents by keywords
3. ❌ Conversation messages also stored without keywords
4. ❌ Memory retrieval effectiveness severely degraded

## REQUIRED IMPLEMENTATION

### Strategy 1: Simple TF-IDF Keyword Extraction (Fast, No Dependencies)

```rust
/// Extract top keywords from text using simple frequency analysis
fn extract_keywords(text: &str, max_keywords: usize) -> Vec<String> {
    use std::collections::HashMap;

    // Common stop words to filter out
    const STOP_WORDS: &[&str] = &[
        "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
        "of", "with", "by", "from", "as", "is", "was", "are", "were", "be",
        "been", "being", "have", "has", "had", "do", "does", "did", "will",
        "would", "should", "could", "may", "might", "must", "can", "this",
        "that", "these", "those", "i", "you", "he", "she", "it", "we", "they",
    ];

    let mut word_freq: HashMap<String, usize> = HashMap::new();

    // Tokenize and count word frequencies
    for word in text
        .split_whitespace()
        .filter_map(|w| {
            let clean = w
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if clean.len() > 3 && !STOP_WORDS.contains(&clean.as_str()) {
                Some(clean)
            } else {
                None
            }
        })
    {
        *word_freq.entry(word).or_insert(0) += 1;
    }

    // Sort by frequency and take top N
    let mut keywords: Vec<_> = word_freq.into_iter().collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1));

    keywords
        .into_iter()
        .take(max_keywords)
        .map(|(word, _)| word)
        .collect()
}
```

**Usage in session.rs**:
```rust
// Line 118-133 (replace keywords: vec![] with):
keywords: extract_keywords(&doc.data, 10),
```

### Strategy 2: Advanced Keyword Extraction (Better Quality, Requires Dependency)

Add dependency to `Cargo.toml`:
```toml
[dependencies]
rake = "0.1"  # RAKE keyword extraction algorithm
```

Implementation:
```rust
use rake::Rake;

fn extract_keywords_rake(text: &str, max_keywords: usize) -> Vec<String> {
    let rake = Rake::new(text);
    rake.keywords()
        .take(max_keywords)
        .map(|kw| kw.keyword.to_string())
        .collect()
}
```

### Strategy 3: Context-Aware Extraction (Recommended)

Extract keywords based on document type and content:

```rust
fn extract_contextual_keywords(
    doc: &CandleDocument,
    max_keywords: usize,
) -> Vec<String> {
    let mut keywords = Vec::new();

    // Extract from filename/path
    if let Some(path) = doc.additional_props.get("path")
        .and_then(|v| v.as_str())
    {
        // Add filename without extension as keyword
        if let Some(filename) = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
        {
            keywords.push(filename.to_lowercase());
        }

        // Add directory names as keywords
        for component in std::path::Path::new(path).components() {
            if let Some(dir) = component.as_os_str().to_str() {
                if dir.len() > 2 && dir != "src" && dir != "lib" {
                    keywords.push(dir.to_lowercase());
                }
            }
        }
    }

    // Extract file extension as keyword
    if let Some(media_type) = &doc.media_type {
        keywords.push(format!("{:?}", media_type).to_lowercase());
    }

    // Extract from content using frequency analysis
    let content_keywords = extract_keywords(&doc.data, max_keywords - keywords.len());
    keywords.extend(content_keywords);

    // Deduplicate and limit
    keywords.sort();
    keywords.dedup();
    keywords.truncate(max_keywords);

    keywords
}
```

## IMPLEMENTATION REQUIREMENTS

### Phase 1: Context Document Keywords (CRITICAL)

**File**: `packages/candle/src/domain/chat/session.rs`

Replace all 4 instances of `keywords: vec![]` in context loading:
- Line 123: context_file loading
- Line 155: context_files loading
- Line 187: context_directory loading
- Line 219: context_github loading

**Change**:
```rust
// Before:
keywords: vec![],

// After:
keywords: extract_contextual_keywords(&doc, 10),
```

### Phase 2: Conversation Keywords (CRITICAL)

**File**: `packages/candle/src/domain/chat/session.rs`

Replace empty keywords in conversation storage:
- Line 492: user message storage
- Line 502: assistant response storage (inherited from user_meta)

**Change**:
```rust
// Line 492 - Before:
keywords: vec![],

// After:
keywords: extract_keywords(&user_message, 5),

// Line 522 - Add for assistant:
let mut assistant_meta = MemoryMetadata {
    tags: vec!["assistant_response".to_string()],
    keywords: extract_keywords(&assistant_response, 5),
    ..user_meta.clone()
};
```

### Phase 3: Add Helper Module

Create new file: `packages/candle/src/domain/chat/keywords.rs`

```rust
//! Keyword extraction for memory search optimization

use std::collections::HashMap;

/// Common English stop words to filter
const STOP_WORDS: &[&str] = &[
    "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
    "of", "with", "by", "from", "as", "is", "was", "are", "were", "be",
    "been", "being", "have", "has", "had", "do", "does", "did", "will",
    "would", "should", "could", "may", "might", "must", "can", "this",
    "that", "these", "those", "i", "you", "he", "she", "it", "we", "they",
];

/// Extract keywords using frequency analysis
pub fn extract_keywords(text: &str, max_keywords: usize) -> Vec<String> {
    // Implementation from Strategy 1
}

/// Extract keywords with document context awareness
pub fn extract_contextual_keywords(
    doc: &crate::domain::context::CandleDocument,
    max_keywords: usize,
) -> Vec<String> {
    // Implementation from Strategy 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_extraction() {
        let text = "Rust programming language memory safety concurrency";
        let keywords = extract_keywords(text, 3);
        assert!(keywords.contains(&"rust".to_string()));
        assert!(keywords.contains(&"programming".to_string()));
        assert!(keywords.contains(&"memory".to_string()));
    }
}
```

Update `packages/candle/src/domain/chat/mod.rs`:
```rust
pub mod keywords;
```

## EXPECTED BEHAVIOR AFTER FIX

### Memory Search Effectiveness

**Before** (empty keywords):
```rust
memory.search_memories("authentication code", 10, None).await
// Returns: No results (keywords empty, only embedding similarity)
```

**After** (with keywords):
```rust
memory.search_memories("authentication code", 10, None).await
// Returns: Documents with "authentication", "auth", "security" keywords
// Plus: Conversations mentioning "authentication"
```

### Search Precision

**Keyword matching provides**:
- Exact term matches (fast)
- Related term discovery
- Topic clustering
- Improved relevance ranking
- Fallback when embeddings fail

## PERFORMANCE IMPACT

**Keyword Extraction Cost**:
- Simple frequency: ~0.1-1ms per document
- Context-aware: ~1-2ms per document
- **Worth the cost**: Enables fast keyword-based retrieval

**Memory Search Speedup**:
- Keyword pre-filter reduces embedding search space
- 10-100x faster for exact term matches
- Dramatically improved recall for relevant documents

## TESTING

Test keyword extraction:
```rust
#[test]
fn test_context_document_keywords() {
    let doc = CandleDocument {
        data: "This is a Rust implementation of authentication middleware".to_string(),
        additional_props: {
            let mut props = HashMap::new();
            props.insert("path".to_string(), json!("src/auth/middleware.rs"));
            props
        },
        ..Default::default()
    };

    let keywords = extract_contextual_keywords(&doc, 10);

    assert!(keywords.contains(&"auth".to_string()));
    assert!(keywords.contains(&"middleware".to_string()));
    assert!(keywords.contains(&"rust".to_string()));
    assert!(keywords.contains(&"implementation".to_string()));
}
```

Test memory search:
```rust
#[tokio::test]
async fn test_keyword_based_search() {
    // Load context with keywords
    let agent = create_test_agent_with_context().await;

    // Search should find documents by keyword
    let results = agent.memory.search_memories("authentication", 10, None).await.unwrap();

    assert!(!results.is_empty(), "Should find documents with 'authentication' keyword");
}
```

## DEFINITION OF DONE

- [ ] Keyword extraction function implemented (strategy 1 or 3)
- [ ] Context documents populated with keywords (4 locations)
- [ ] Conversation messages populated with keywords (2 locations)
- [ ] Helper module created at `domain/chat/keywords.rs`
- [ ] Unit tests for keyword extraction
- [ ] Integration test for keyword-based search
- [ ] Performance acceptable (<2ms per document)
- [ ] Code compiles: `cargo check -p cyrup_candle`
- [ ] Memory search effectiveness validated

## CRITICAL NOTES

**This is NOT optional metadata** - keywords are critical for memory system functionality:

1. **Search Failures**: Without keywords, semantic search relies only on embeddings
2. **Poor Recall**: Exact term matches fail without keyword index
3. **Degraded UX**: Users cannot find relevant context in memory
4. **System Blindness**: Context documents invisible to keyword search

**Empty keywords = broken memory search system.**

The fact that conversation memories ALSO have empty keywords (line 492) proves this is a systemic issue, not just a context loading problem. The entire memory storage pipeline needs keyword extraction.
