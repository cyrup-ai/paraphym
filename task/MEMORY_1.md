# MEMORY_1: Implement Memory Health Monitoring

## OBJECTIVE

Replace stubbed memory health monitoring with actual index quality metrics and proper dimension tracking.

## BACKGROUND

Memory health monitoring uses placeholder defaults instead of actual index metrics. This prevents detecting degraded vector search performance and index corruption.

## SUBTASK 1: Extract Index Quality Metrics

**Location:** `packages/candle/src/memory/monitoring/health.rs:389`

**Current State:**
```rust
// Dimensions and index quality require trait extension or type-specific methods
// For now, use sensible defaults or get from config
let dimensions = embedding_dims as u32;
let index_quality = 100.0f32; // Assume healthy if count() succeeds
```

**Required Changes:**
- Remove hardcoded `index_quality = 100.0`
- Query actual index quality metrics from vector store
- Calculate quality based on: search accuracy, index fragmentation, entry count
- Use database-specific quality metrics where available
- Return actual dimension count from embeddings

**Why:** Index quality degrades over time and needs monitoring to detect issues.

## SUBTASK 2: Add Index Quality Trait Method

**Location:** Vector store trait definition

**Required Changes:**
- Add `get_index_quality()` method to vector store trait
- Return struct with: search_quality, fragmentation_ratio, entry_count, dimension
- Implement for SurrealDB vector backend
- Implement for in-memory vector backend
- Document quality metric calculation

**Why:** Different backends have different quality metrics.

## SUBTASK 3: Implement Quality Calculation

**Location:** Vector store implementations

**Required Changes:**
- For SurrealDB: query index statistics
- Calculate fragmentation ratio (deleted / total entries)
- Track search accuracy over recent queries
- Measure index rebuild needs
- Return comprehensive quality metrics

**Why:** Quality calculation must be backend-specific.

## SUBTASK 4: Integrate with Health Monitoring

**Location:** `packages/candle/src/memory/monitoring/health.rs`

**Required Changes:**
- Call `get_index_quality()` instead of hardcoding
- Set health thresholds for quality metrics
- Emit warnings when quality degrades
- Trigger index optimization when needed
- Log quality metrics for trending

**Why:** Health monitoring must use real data to be useful.

## DEFINITION OF DONE

- [ ] No hardcoded `index_quality = 100.0` values
- [ ] Actual index quality extracted from vector stores
- [ ] Quality metrics include: accuracy, fragmentation, dimensions
- [ ] Health thresholds configured for quality degradation
- [ ] Index optimization triggered when quality drops
- [ ] Documentation explains quality metrics
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Index Quality Metrics
- Search accuracy: % of queries returning expected results
- Fragmentation: deleted entries / total entries
- Dimension consistency: all embeddings have same dimension
- Entry count: total vectors in index

### Vector Store Integration
- SurrealDB: query system tables for index stats
- In-memory: track metrics directly
- Consider lazy quality calculation (cached with TTL)

### Health Thresholds
- Quality < 80%: warning
- Quality < 60%: trigger reindex
- Fragmentation > 30%: optimize index
- Dimension mismatch: error

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Quality checks should be lightweight (use caching)
- Maintain compatibility with existing health monitoring API
