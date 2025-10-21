# AGENT_1: Implement Agent Performance Statistics

## OBJECTIVE

Implement the agent monitoring system to track agent lifecycle, completion counts, and performance metrics using the existing atomic counter infrastructure.

## BACKGROUND

Agent statistics tracking exists as dead code with a TODO comment. The static atomic counter is unused, preventing monitoring of agent performance and resource usage.

## SUBTASK 1: Design Agent Statistics API

**Location:** `packages/candle/src/domain/agent/core.rs:24-25`

**Current State:**
```rust
#[allow(dead_code)] // TODO: Implement in agent monitoring system
static AGENT_STATS: AtomicUsize = AtomicUsize::new(0);
```

**Required Changes:**
- Remove `#[allow(dead_code)]` attribute
- Design statistics struct to track:
  - Total agents created
  - Active agents (created - destroyed)
  - Total completions processed
  - Total tokens processed
  - Average completion time
- Use lock-free atomic operations (pattern exists in memory/monitoring/operations.rs)

**Why:** Agent monitoring is critical for understanding system load and performance.

## SUBTASK 2: Implement Statistics Collection

**Location:** `packages/candle/src/domain/agent/core.rs`

**Required Changes:**
- Extend `AGENT_STATS` to a proper statistics structure using multiple atomics
- Add increment methods: `record_agent_created()`, `record_agent_destroyed()`
- Add tracking methods: `record_completion()`, `record_tokens()`
- Implement `get_stats()` method to return current snapshot
- Use `Ordering::Relaxed` for counters, `Ordering::SeqCst` for critical stats

**Why:** Lock-free statistics collection ensures minimal performance overhead.

## SUBTASK 3: Integrate Statistics into Agent Lifecycle

**Location:** `packages/candle/src/domain/agent/core.rs` and agent builder

**Required Changes:**
- Call `record_agent_created()` in agent construction
- Call `record_agent_destroyed()` in agent Drop implementation
- Add completion tracking to agent execution paths
- Wire token counting from completion responses
- Ensure statistics are updated in all agent lifecycle paths

**Why:** Statistics must be collected at lifecycle events to be accurate.

## SUBTASK 4: Add Statistics Access Methods

**Location:** `packages/candle/src/domain/agent/core.rs`

**Required Changes:**
- Add public API to retrieve current statistics
- Add method to reset statistics (for testing/monitoring)
- Add formatted output method for logging
- Document statistics API in module docs

**Why:** Monitoring systems need access to collected statistics.

## DEFINITION OF DONE

- [ ] No `#[allow(dead_code)]` attribute on AGENT_STATS
- [ ] Statistics structure tracks: created, active, completions, tokens
- [ ] Lock-free atomic operations used throughout
- [ ] Statistics updated at all agent lifecycle events
- [ ] Public API available for reading statistics
- [ ] Module documentation explains statistics usage
- [ ] NO test code written (separate team responsibility)
- [ ] NO benchmark code written (separate team responsibility)

## RESEARCH NOTES

### Existing Patterns
- Reference: `packages/candle/src/memory/monitoring/operations.rs:4`
  - Shows lock-free atomic pattern for statistics
  - Uses `AtomicU64` and `AtomicUsize` with `Ordering::Relaxed`
  - Zero-allocation pattern for performance

### Integration Points
- Agent creation in `CandleAgentBuilder`
- Agent execution in workflow engine
- Completion tracking in providers
- Token counting in streaming responses

## CONSTRAINTS

- DO NOT write unit tests (separate team handles testing)
- DO NOT write benchmarks (separate team handles benchmarks)
- Use lock-free atomic operations only (no Mutex/RwLock)
- Follow zero-allocation patterns from existing monitoring code
