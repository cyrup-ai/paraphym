# STATS_1: Implement Search Statistics Update with Atomic Operations

## OBJECTIVE

Implement the `update_statistics()` method to populate `SearchStatistics` fields from existing atomic counters and index state, enabling accurate monitoring of search index health and performance.

## BACKGROUND

The `ChatSearchIndex` struct has atomic counters and a `SearchStatistics` struct, but the `update_statistics()` method is currently a no-op stub. The infrastructure is already in place:

- Atomic counters: `document_count`, `query_counter`, `index_update_counter`
- Statistics struct: `SearchStatistics` with fields for messages, terms, queries, etc.
- The method exists in two places with identical stubs

## CONSTRAINTS

- **NO TESTS**: Do not write unit tests, integration tests, or test code. Another team handles testing.
- **NO BENCHMARKS**: Do not write benchmark code. Another team handles performance testing.
- **FOCUS**: Only modify `./src` files to implement the feature.

## SUBTASK 1: Review SearchStatistics Structure

**Location**: `packages/candle/src/domain/chat/search/types.rs`

**What to find**: The `SearchStatistics` struct definition with all fields that need to be populated.

**Expected fields**:
- `total_messages: usize`
- `total_terms: usize`
- `total_queries: u64`
- `average_query_time: f64`
- `index_size: usize`
- `last_index_update: u64`

## SUBTASK 2: Review ChatSearchIndex Atomic Counters

**Location**: `packages/candle/src/domain/chat/search/index.rs` (lines 73-79)

**What to find**: The atomic counters and data structures available in `ChatSearchIndex`:
- `document_count: Arc<AtomicUsize>`
- `query_counter: Arc<ConsistentCounter>`
- `index_update_counter: Arc<ConsistentCounter>`
- `statistics: Arc<RwLock<SearchStatistics>>`
- `term_frequencies: DashMap<String, usize>`
- `inverted_index: DashMap<String, Vec<DocumentPosting>>`

## SUBTASK 3: Implement update_statistics in index.rs

**Location**: `packages/candle/src/domain/chat/search/index.rs` (line 278)

**Current code**:
```rust
/// Update search statistics with performance tracking
pub fn update_statistics(&self) {
    // TODO: Implement statistics update with atomic operations
    // This will be enhanced with atomic counters for query time averaging
}
```

**What to change**:
1. Acquire write lock on `self.statistics`
2. Load values from atomic counters using `Ordering::Relaxed`
3. Get sizes from DashMap structures
4. Update timestamp to current Unix time
5. Remove TODO comment

**Expected implementation**:
```rust
/// Update search statistics with performance tracking
pub fn update_statistics(&self) {
    let mut stats = self.statistics.blocking_write();
    stats.total_messages = self.document_count.load(Ordering::Relaxed);
    stats.total_terms = self.term_frequencies.len();
    stats.total_queries = self.query_counter.get() as u64;
    stats.index_size = self.inverted_index.len();
    stats.last_index_update = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
}
```

**Note**: `average_query_time` is not updated here as it requires more complex tracking (mentioned in the TODO comment about "query time averaging").

## SUBTASK 4: Implement update_statistics in mod.rs

**Location**: `packages/candle/src/domain/chat/search/mod.rs` (line 262)

**Current code**: Identical stub to the one in index.rs

**What to change**: Apply the exact same implementation as SUBTASK 3.

## SUBTASK 5: Verify Compilation

**Commands**:
```bash
cargo check -p paraphym_candle
cargo clippy -p paraphym_candle
cargo fmt -p paraphym_candle
```

**What to verify**:
- Code compiles without errors
- No new clippy warnings
- Both implementations are consistent
- TODO comments are removed from both files
- Proper use of `Ordering::Relaxed` for atomic loads

## DEFINITION OF DONE

- [ ] `update_statistics()` populates all `SearchStatistics` fields (except `average_query_time`)
- [ ] Uses atomic loads for counters with `Ordering::Relaxed`
- [ ] Updates `last_index_update` timestamp to current Unix time
- [ ] TODO comments removed from both `index.rs` and `mod.rs`
- [ ] Both implementations are identical and consistent
- [ ] Code compiles without errors
- [ ] No new clippy warnings

## WHY THIS MATTERS

Statistics are used for monitoring search index health, query performance, and capacity planning. Without this implementation, operators have no visibility into the actual state of the search index.

## REFERENCE FILES

- **File to modify**: `packages/candle/src/domain/chat/search/index.rs` (line 278)
- **File to modify**: `packages/candle/src/domain/chat/search/mod.rs` (line 262)
- **Structure reference**: `packages/candle/src/domain/chat/search/types.rs` (SearchStatistics)
- **Counter reference**: `packages/candle/src/domain/chat/search/index.rs` (lines 73-79)
