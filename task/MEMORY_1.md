# MEMORY_1: Implement Memory Health Monitoring

## OBJECTIVE

Replace stubbed memory health monitoring with actual index quality metrics and proper dimension tracking by extending the VectorStore trait and implementing real metrics extraction.

## BACKGROUND

Memory health monitoring at [`packages/candle/src/memory/monitoring/health.rs:389`](../packages/candle/src/memory/monitoring/health.rs) uses placeholder defaults instead of actual index metrics:

```rust
let dimensions = embedding_dims as u32;
let index_quality = 100.0f32; // Assume healthy if count() succeeds
```

This prevents detecting degraded vector search performance, dimension mismatches, and memory inefficiencies.

## CODEBASE ARCHITECTURE

### Vector Store Hierarchy

The codebase has **two separate VectorStore traits**:

1. **Sync VectorStore** (used by health monitoring): [`packages/candle/src/memory/vector/vector_store.rs`](../packages/candle/src/memory/vector/vector_store.rs)
   - Thread-safe synchronous operations
   - Methods: `add_vector()`, `get_vector()`, `search()`, `count()`, `clear()`
   - Implementation: [`InMemoryVectorStore`](../packages/candle/src/memory/vector/in_memory.rs)

2. **Async VectorStore** (different interface): [`packages/candle/src/memory/vector/mod.rs:112`](../packages/candle/src/memory/vector/mod.rs)
   - Returns futures and streams
   - Not used by health monitoring

**This task only modifies the sync VectorStore trait and InMemoryVectorStore.**

### Existing Capabilities

`InMemoryVectorStore` already has:
- [`memory_usage()`](../packages/candle/src/memory/vector/in_memory.rs#L87) - Returns `(vector_bytes, metadata_bytes)`
- [`capacity()`](../packages/candle/src/memory/vector/in_memory.rs#L62) - Returns HashMap capacity
- Internal `vectors: HashMap<String, Vec<f32>>` for dimension checking

## IMPLEMENTATION PLAN

### SUBTASK 1: Extend VectorStore Trait

**File:** [`packages/candle/src/memory/vector/vector_store.rs`](../packages/candle/src/memory/vector/vector_store.rs)

**Location:** After line 152 (end of trait definition)

**Add these methods to the VectorStore trait:**

```rust
/// Get index quality metrics
///
/// # Returns
/// Result containing index quality score (0.0-100.0)
/// - 100.0 = perfect health
/// - 80-99 = good health
/// - 60-79 = degraded
/// - <60 = unhealthy
fn get_index_quality(&self) -> Result<f32> {
    // Default implementation for stores without quality tracking
    Ok(100.0)
}

/// Get vector dimensions
///
/// # Returns
/// Result containing the dimension size, or None if store is empty
fn get_dimensions(&self) -> Result<Option<u32>> {
    // Default implementation returns None
    Ok(None)
}

/// Get index statistics
///
/// # Returns
/// Result containing detailed index statistics
fn get_index_stats(&self) -> Result<IndexStats> {
    Ok(IndexStats {
        entry_count: self.count()? as u64,
        dimensions: self.get_dimensions()?,
        quality_score: self.get_index_quality()?,
        memory_bytes: 0, // Override in implementations
        fragmentation_ratio: 0.0,
    })
}
```

**Add IndexStats struct before the trait definition (around line 12):**

```rust
/// Index statistics for health monitoring
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexStats {
    /// Total number of entries
    pub entry_count: u64,
    
    /// Vector dimensions (None if empty)
    pub dimensions: Option<u32>,
    
    /// Quality score (0.0-100.0)
    pub quality_score: f32,
    
    /// Total memory usage in bytes
    pub memory_bytes: u64,
    
    /// Fragmentation ratio (0.0-1.0, 0.0 = no fragmentation)
    pub fragmentation_ratio: f32,
}
```

### SUBTASK 2: Implement Quality Metrics in InMemoryVectorStore

**File:** [`packages/candle/src/memory/vector/in_memory.rs`](../packages/candle/src/memory/vector/in_memory.rs)

**Location:** In the `impl VectorStore for InMemoryVectorStore` block (after line 312)

**Add these implementations:**

```rust
fn get_dimensions(&self) -> Result<Option<u32>> {
    // Get dimensions from first vector
    if let Some((_, vector)) = self.vectors.iter().next() {
        Ok(Some(vector.len() as u32))
    } else {
        Ok(None)
    }
}

fn get_index_quality(&self) -> Result<f32> {
    if self.vectors.is_empty() {
        return Ok(100.0); // Empty store is "healthy"
    }
    
    // Check dimension consistency
    let first_dim = self.vectors.values().next().map(|v| v.len());
    let dimension_consistent = first_dim.map_or(true, |expected_dim| {
        self.vectors.values().all(|v| v.len() == expected_dim)
    });
    
    if !dimension_consistent {
        return Ok(0.0); // Critical error: dimension mismatch
    }
    
    // Calculate memory efficiency
    let (vector_bytes, metadata_bytes) = self.memory_usage();
    let total_bytes = vector_bytes + metadata_bytes;
    let capacity_bytes = self.capacity() * 64; // Rough estimate
    
    let efficiency = if capacity_bytes > 0 {
        (total_bytes as f32 / capacity_bytes as f32).min(1.0)
    } else {
        1.0
    };
    
    // Quality score: 100 if efficient, scales down with waste
    // Efficiency < 0.3 means lots of wasted capacity
    let quality = if efficiency < 0.3 {
        70.0 + (efficiency * 100.0) // 70-100 range
    } else {
        100.0
    };
    
    Ok(quality)
}

fn get_index_stats(&self) -> Result<IndexStats> {
    let (vector_bytes, metadata_bytes) = self.memory_usage();
    
    Ok(IndexStats {
        entry_count: self.count()? as u64,
        dimensions: self.get_dimensions()?,
        quality_score: self.get_index_quality()?,
        memory_bytes: (vector_bytes + metadata_bytes) as u64,
        fragmentation_ratio: 0.0, // InMemory has no fragmentation
    })
}
```

**Why this approach:**
- Uses existing `memory_usage()` method
- Dimension consistency is critical for vector operations
- Memory efficiency indicates if HashMap needs shrinking
- No fragmentation in HashMap (immediate deletion)

### SUBTASK 3: Update Health Monitoring

**File:** [`packages/candle/src/memory/monitoring/health.rs`](../packages/candle/src/memory/monitoring/health.rs)

**Location:** Lines 387-392

**Replace:**
```rust
// Dimensions and index quality require trait extension or type-specific methods
// For now, use sensible defaults or get from config
let dimensions = embedding_dims as u32;
let index_quality = 100.0f32; // Assume healthy if count() succeeds
```

**With:**
```rust
// Get actual index statistics from vector store
let stats = tokio::task::spawn_blocking(move || {
    let vs = vs_idx.blocking_read();
    vs.get_index_stats()
})
.await
.unwrap_or_else(|_| Ok(crate::memory::vector::vector_store::IndexStats {
    entry_count: 0,
    dimensions: None,
    quality_score: 0.0,
    memory_bytes: 0,
    fragmentation_ratio: 0.0,
}))
.unwrap_or_else(|_| crate::memory::vector::vector_store::IndexStats {
    entry_count: 0,
    dimensions: None,
    quality_score: 0.0,
    memory_bytes: 0,
    fragmentation_ratio: 0.0,
});

let dimensions = stats.dimensions.unwrap_or(embedding_dims as u32);
let index_quality = stats.quality_score;
```

**Update the return statement to use stats:**
```rust
(stats.entry_count, dimensions, index_quality)
```

**Add import at top of file (around line 14):**
```rust
use crate::memory::vector::vector_store::IndexStats;
```

### SUBTASK 4: Add Quality Thresholds to Health Checks

**File:** [`packages/candle/src/memory/monitoring/health.rs`](../packages/candle/src/memory/monitoring/health.rs)

**Location:** After getting index_quality (around line 410), add threshold checks:

```rust
// Set health status based on quality thresholds
if index_quality < 60.0 {
    health.status = HealthStatus::Unhealthy;
    health.message = Some(format!("Index quality critically low: {:.1}%", index_quality));
} else if index_quality < 80.0 {
    health.status = HealthStatus::Degraded;
    health.message = Some(format!("Index quality degraded: {:.1}%", index_quality));
}

// Check for dimension mismatch
if dimensions != embedding_dims as u32 {
    health.status = HealthStatus::Unhealthy;
    health.message = Some(format!(
        "Dimension mismatch: expected {}, got {}",
        embedding_dims, dimensions
    ));
}
```

## CODE PATTERNS & EXAMPLES

### Pattern 1: Dimension Checking

```rust
// Check all vectors have same dimension
let first_dim = self.vectors.values().next().map(|v| v.len());
let all_same = first_dim.map_or(true, |expected| {
    self.vectors.values().all(|v| v.len() == expected)
});
```

### Pattern 2: Memory Efficiency Calculation

```rust
// Use existing memory_usage() method
let (vector_bytes, metadata_bytes) = self.memory_usage();
let total_used = vector_bytes + metadata_bytes;
let total_capacity = self.capacity() * estimated_bytes_per_entry;
let efficiency = total_used as f32 / total_capacity as f32;
```

### Pattern 3: Quality Score Formula

```rust
// Quality score based on multiple factors
let quality = match () {
    _ if !dimension_consistent => 0.0,  // Critical failure
    _ if efficiency < 0.3 => 70.0 + (efficiency * 100.0), // Wasteful
    _ => 100.0, // Healthy
};
```

## RESEARCH FINDINGS

### Vector Index Quality Metrics (Industry Standard)

1. **Dimension Consistency**: All vectors must have identical dimensions
   - Critical for SIMD operations and similarity calculations
   - Mismatch = immediate failure (quality = 0)

2. **Memory Efficiency**: Ratio of used/allocated memory
   - HashMap over-allocation is common after deletions
   - Low efficiency (<30%) suggests need for `shrink_to_fit()`
   - Tracked via existing `memory_usage()` and `capacity()` methods

3. **Fragmentation**: Deleted entries still occupying space
   - Not applicable to InMemoryVectorStore (HashMap removes immediately)
   - Would be relevant for SurrealDB or disk-based stores

4. **Entry Count**: Total vectors in index
   - Already available via `count()` method
   - Used for capacity planning

### Existing Code Capabilities

From [`in_memory.rs:87-98`](../packages/candle/src/memory/vector/in_memory.rs#L87):
```rust
pub fn memory_usage(&self) -> (usize, usize) {
    let vector_bytes = self.vectors.iter()
        .map(|(k, v)| k.len() + v.len() * std::mem::size_of::<f32>())
        .sum::<usize>();
    
    let metadata_bytes = self.metadata.iter()
        .map(|(k, v)| k.len() + v.len() * 64)
        .sum::<usize>();
    
    (vector_bytes, metadata_bytes)
}
```

This method already provides accurate memory tracking - we just need to expose it through the trait.

## FILE MODIFICATION SUMMARY

### Files to Modify

1. **[`packages/candle/src/memory/vector/vector_store.rs`](../packages/candle/src/memory/vector/vector_store.rs)**
   - Add `IndexStats` struct (line ~12)
   - Add 3 new trait methods: `get_index_quality()`, `get_dimensions()`, `get_index_stats()` (line ~152)

2. **[`packages/candle/src/memory/vector/in_memory.rs`](../packages/candle/src/memory/vector/in_memory.rs)**
   - Implement 3 trait methods in `impl VectorStore for InMemoryVectorStore` block (line ~312)
   - Use existing `memory_usage()`, `capacity()`, and `vectors` HashMap

3. **[`packages/candle/src/memory/monitoring/health.rs`](../packages/candle/src/memory/monitoring/health.rs)**
   - Add import for `IndexStats` (line ~14)
   - Replace hardcoded values with `get_index_stats()` call (lines 387-392)
   - Add quality threshold checks (line ~410)

### Files NOT Modified

- **No changes to async VectorStore** in `mod.rs` (different interface)
- **No SurrealDB implementation** (doesn't exist yet in codebase)
- **No changes to VectorIndex** trait (separate abstraction layer)

## DEFINITION OF DONE

- [ ] `IndexStats` struct added to `vector_store.rs`
- [ ] Three new methods added to `VectorStore` trait with default implementations
- [ ] `InMemoryVectorStore` implements all three methods using existing data
- [ ] Health monitoring calls `get_index_stats()` instead of hardcoding
- [ ] Quality thresholds implemented: <60% unhealthy, <80% degraded
- [ ] Dimension mismatch detection triggers unhealthy status
- [ ] No hardcoded `index_quality = 100.0` remains in codebase
- [ ] Code compiles without errors
- [ ] Health endpoint returns actual quality metrics

## CONSTRAINTS

- **DO NOT** write unit tests (separate team responsibility)
- **DO NOT** write benchmarks (separate team responsibility)
- **DO NOT** add extensive documentation beyond inline comments
- **DO NOT** modify the async VectorStore trait in `mod.rs`
- **DO NOT** create SurrealDB implementation (out of scope)
- Use existing methods (`memory_usage()`, `capacity()`, `count()`)
- Maintain backward compatibility with existing health monitoring API
- Keep quality calculations lightweight (no expensive operations)

## TECHNICAL NOTES

### Why Default Implementations?

The trait provides default implementations so that:
1. Future VectorStore implementations aren't forced to implement quality tracking
2. Backward compatibility is maintained
3. Stores can override with backend-specific metrics when available

### Quality Score Interpretation

- **100**: Perfect health, optimal memory usage, consistent dimensions
- **80-99**: Good health, minor inefficiencies
- **60-79**: Degraded, significant memory waste or approaching capacity
- **<60**: Unhealthy, requires intervention (reindex, shrink, etc.)
- **0**: Critical failure (dimension mismatch, corrupted data)

### Performance Considerations

- `get_dimensions()`: O(1) - just checks first vector
- `get_index_quality()`: O(n) worst case for dimension checking, but short-circuits
- `get_index_stats()`: Combines existing O(1) operations
- All methods use existing data structures, no additional memory allocation

### Future Extensions

When SurrealDB VectorStore is implemented, it can override these methods to:
- Query database index statistics tables
- Calculate fragmentation from deleted record counts
- Use database-specific quality metrics
- Cache results with TTL to avoid repeated queries
