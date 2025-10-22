# Issue: Task Parameter Not Validated

## Severity: LOW
**Impact**: Silent fallback to default, user confusion

## Location
`/Volumes/samsung_t9/cyrup/packages/candle/src/capability/text_embedding/stella/instruction.rs:5-22`

## Problem Description

The `task` parameter accepts any string but only recognizes specific values:

```rust
pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
    let instruct = match task {
        Some("s2p") => "...",
        Some("s2s") => "...",
        Some("search_query") => "...",
        Some("search_document") => "...",
        Some("classification") => "...",
        Some("clustering") => "...",
        Some("retrieval") => "...",
        _ => "...",  // ← Default for ANY other value
    };
    // ...
}
```

## Problem Scenarios

### Scenario 1: Typo
```rust
let emb = model.embed("query text", Some("serach_query".to_string())).await?;
//                                          ↑ Typo: "serach" instead of "search"
// ↑ Silently uses default instruction, user doesn't know
```

### Scenario 2: Invalid Task
```rust
let emb = model.embed("query text", Some("translation".to_string())).await?;
//                                          ↑ Not a supported task
// ↑ Silently uses default, user thinks it's using translation mode
```

### Scenario 3: Case Sensitivity
```rust
let emb = model.embed("query text", Some("S2P".to_string())).await?;
//                                          ↑ Uppercase
// ↑ Doesn't match "s2p", uses default
```

## Impact

Users get **incorrect embeddings** without knowing:
- Wrong instruction prefix
- Suboptimal similarity scores
- Silent degradation

## Better Approach

### Option 1: Validate and Error

```rust
const VALID_TASKS: &[&str] = &[
    "s2p", "s2s", "search_query", "search_document",
    "classification", "clustering", "retrieval"
];

pub(crate) fn format_with_instruction(
    texts: &[&str],
    task: Option<&str>,
) -> Result<Vec<String>, String> {
    // Validate task
    if let Some(t) = task {
        if !VALID_TASKS.contains(&t) {
            return Err(format!(
                "Invalid task '{}'. Valid tasks: {:?}",
                t, VALID_TASKS
            ));
        }
    }
    
    let instruct = get_instruction(task);
    Ok(texts
        .iter()
        .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
        .collect())
}
```

**Pros**:
- Catches errors early
- Clear error messages
- Forces correct usage

**Cons**:
- Breaking change (returns Result)
- More strict

### Option 2: Validate and Warn

```rust
pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
    // Warn on invalid task
    if let Some(t) = task {
        if !VALID_TASKS.contains(&t) {
            log::warn!(
                "Unknown task '{}', using default. Valid tasks: {:?}",
                t, VALID_TASKS
            );
        }
    }
    
    let instruct = get_instruction(task);
    // ... rest of code
}
```

**Pros**:
- No breaking change
- Alerts user to problem
- Maintains fallback behavior

**Cons**:
- Still allows invalid input
- Warning might be missed

### Option 3: Use Enum

```rust
#[derive(Debug, Clone, Copy)]
pub enum EmbeddingTask {
    SearchQuery,      // s2p
    Similarity,       // s2s
    Classification,
    Clustering,
    Retrieval,
}

impl EmbeddingTask {
    fn instruction(&self) -> &'static str {
        match self {
            Self::SearchQuery | Self::Retrieval => 
                "Given a web search query, retrieve relevant passages that answer the query.",
            Self::Similarity | Self::Classification | Self::Clustering =>
                "Retrieve semantically similar text.",
        }
    }
}

// API becomes:
pub fn embed(&self, text: &str, task: Option<EmbeddingTask>) -> ...
```

**Pros**:
- Type-safe
- No invalid values possible
- Clear API

**Cons**:
- Breaking change
- More complex API

## Current API

```rust
pub fn embed(
    &self,
    text: &str,
    task: Option<String>,  // ← Any string accepted
) -> ...
```

## Recommendation

**Use Option 2 (Validate and Warn)** as a non-breaking improvement:

```rust
pub(crate) fn format_with_instruction(texts: &[&str], task: Option<&str>) -> Vec<String> {
    const VALID_TASKS: &[&str] = &[
        "s2p", "s2s", "search_query", "search_document",
        "classification", "clustering", "retrieval"
    ];
    
    if let Some(t) = task {
        if !VALID_TASKS.contains(&t) {
            log::warn!(
                "Unknown embedding task '{}'. Using default 's2p'. \
                 Valid tasks: {}",
                t,
                VALID_TASKS.join(", ")
            );
        }
    }
    
    let instruct = match task {
        // ... existing match
    };
    
    texts
        .iter()
        .map(|text| format!("Instruct: {}\nQuery: {}", instruct, text))
        .collect()
}
```

Then in a future major version, migrate to Option 3 (Enum) for type safety.

## Documentation Needed

Regardless of approach, document the valid task values:

```rust
/// Generate embedding for text with optional task-specific instruction.
///
/// # Task Types
/// - `"s2p"` or `"search_query"` or `"retrieval"`: Search query → passage retrieval
/// - `"s2s"` or `"classification"` or `"clustering"`: Semantic similarity
/// - `None`: Defaults to search query mode
///
/// # Example
/// ```
/// let emb = model.embed("What is Rust?", Some("search_query".to_string())).await?;
/// ```
pub fn embed(&self, text: &str, task: Option<String>) -> ...
```
