# Technical Debt & Incomplete Implementation Tracking

This document tracks non-production code patterns, incomplete implementations, and technical debt that must be resolved before production deployment.

---

## CRITICAL ISSUES

### 1. Template Engine Simplified Array/Object Rendering
**File:** `packages/candle/src/domain/chat/templates/engines.rs:55-56`  
**Line:** 55-56  
**Violation:** Placeholder implementation that renders arrays and objects as "[array]" and "[object]"

**Current Code:**
```rust
CandleTemplateValue::Array(_) => "[array]", // Simplified
CandleTemplateValue::Object(_) => "[object]", // Simplified
```

**Issue:**
- Data loss: actual array/object values are replaced with placeholder strings
- Breaking: templates expecting real data will fail
- No serialization: should use `serde_json` to serialize complex types

**Resolution:**
```rust
CandleTemplateValue::Array(arr) => {
    serde_json::to_string(arr)
        .unwrap_or_else(|_| "[]".to_string())
        .leak() // Safe: template strings are static-lived
}
CandleTemplateValue::Object(obj) => {
    serde_json::to_string(obj)
        .unwrap_or_else(|_| "{}".to_string())
        .leak()
}
```

**Alternative (if allocation is concern):**
```rust
// Use static buffer with formatting
CandleTemplateValue::Array(arr) => {
    format_array_compact(arr) // Returns &'static str via arena
}
```

---

### 2. Temporal Distance Hardcoded to Zero
**File:** `packages/candle/src/domain/memory/cognitive/types.rs:1420`  
**Line:** 1420  
**Violation:** Temporal causal link uses placeholder value instead of actual timestamp lookup

**Current Code:**
```rust
// For now, use sequence-based distance as proxy
let temporal_distance = 0i64; // Placeholder - would need memory timestamp lookup
```

**Issue:**
- All temporal causal links have identical distance (0)
- Breaks temporal reasoning in cognitive system
- Comment admits this is a placeholder: "would need memory timestamp lookup"

**Resolution:**
```rust
pub fn add_temporal_causal_link(&mut self, source_id: Uuid, target_id: Uuid, strength: f32) {
    // Calculate actual temporal distance using memory store
    let temporal_distance = self.calculate_temporal_distance(source_id, target_id)
        .unwrap_or(0i64); // Only fallback to 0 if lookup fails

    let link = CausalLink::new(
        source_id,
        target_id,
        strength.clamp(0.0, 1.0),
        temporal_distance,
    );

    let temporal_ctx_mut = Arc::make_mut(&mut self.temporal_context);
    temporal_ctx_mut.add_causal_dependency(link);
}

/// Calculate actual temporal distance between two memories
fn calculate_temporal_distance(&self, source_id: Uuid, target_id: Uuid) -> Result<i64, CognitiveError> {
    // Access memory store to get timestamps
    let source_timestamp = self.memory_store.get_timestamp(source_id)?;
    let target_timestamp = self.memory_store.get_timestamp(target_id)?;
    
    // Return absolute difference in milliseconds
    Ok((target_timestamp - source_timestamp).abs())
}
```

**Dependencies:**
- Requires access to memory store with timestamp lookup
- May need to pass memory store reference to CognitiveState
- Add `memory_store: Arc<dyn MemoryStore>` field to CognitiveState

---

### 3. TODO: Variable System for Macro Expansion
**File:** `packages/candle/src/domain/chat/macros.rs:1387`  
**Line:** 1387  
**Violation:** Variable system marked as TODO but field is allocated and never used

**Current Code:**
```rust
/// Variable context for macro execution
#[allow(dead_code)] // TODO: Implement variable system for macro expansion
variables: Arc<RwLock<HashMap<String, String>>>,
```

**Issue:**
- Memory allocated for variables HashMap but never populated
- Dead code allowed with TODO indicating missing implementation
- Methods `set_global_variable`, `get_global_variable` exist but are incomplete

**Resolution:**
The variable system IS partially implemented (methods exist at lines 1832-1860). Remove the `#[allow(dead_code)]` and TODO:

```rust
/// Variable context for macro execution
variables: Arc<RwLock<HashMap<String, String>>>,
```

The implementation is actually complete - this is a FALSE ALARM. The TODO should be removed.

---

### 4. TODO: Execution Queue Implementation
**File:** `packages/candle/src/domain/chat/macros.rs:1390`  
**Line:** 1390  
**Violation:** Execution queue allocated but never used

**Current Code:**
```rust
/// Execution queue for async processing
#[allow(dead_code)] // TODO: Implement in macro execution system
execution_queue: Arc<Mutex<Vec<MacroExecutionRequest>>>,
```

**Issue:**
- Queue is created but never populated or processed
- Async macro execution happens directly, not via queue
- Memory waste: allocates Mutex and Vec that are never used

**Resolution Options:**

**Option A - Implement Queue System:**
```rust
/// Execution queue for async processing
execution_queue: Arc<Mutex<Vec<MacroExecutionRequest>>>,

// Add queue processing method
pub async fn enqueue_execution(&self, request: MacroExecutionRequest) -> Result<(), MacroSystemError> {
    let mut queue = self.execution_queue.lock().await;
    
    // Priority queue insertion (higher priority first)
    let insert_pos = queue.iter()
        .position(|r| r.priority < request.priority)
        .unwrap_or(queue.len());
    
    queue.insert(insert_pos, request);
    Ok(())
}

pub async fn process_queue(&self) -> Pin<Box<dyn Stream<Item = MacroExecutionResult> + Send>> {
    // Process queued executions in priority order
    // Return stream of execution results
}
```

**Option B - Remove Unused Field:**
```rust
// Remove execution_queue field entirely if not needed
// Current direct execution via execute_macro_stream() is sufficient
```

**Recommendation:** Option B - remove the field. Current direct execution is production-ready.

---

### 5. Work Stealing Placeholder
**File:** `packages/candle/src/pool/core/worker_state.rs:67`  
**Line:** 67  
**Violation:** Work stealing marked as future feature with placeholder type

**Current Code:**
```rust
// Work stealing (placeholder for future async work stealing implementation)
pub steal_handle: Option<()>,
```

**Issue:**
- `Option<()>` is a placeholder type with no functionality
- Comment admits this is for "future async work stealing implementation"
- Field exists but provides no value

**Resolution:**

**Option A - Implement Work Stealing:**
```rust
/// Work stealing handle for load balancing across workers
pub steal_handle: Option<Arc<WorkStealHandle>>,

pub struct WorkStealHandle {
    /// Deque for work stealing
    deque: crossbeam::deque::Worker<Request>,
    /// Stealer references for other workers
    stealers: Arc<Vec<crossbeam::deque::Stealer<Request>>>,
}

impl WorkStealHandle {
    pub fn try_steal(&self) -> Option<Request> {
        // Attempt to steal work from other workers
        for stealer in self.stealers.iter() {
            match stealer.steal() {
                crossbeam::deque::Steal::Success(req) => return Some(req),
                _ => continue,
            }
        }
        None
    }
}
```

**Option B - Remove Placeholder:**
```rust
// Remove steal_handle field if work stealing is not needed
// Current Power of Two Choices load balancing is sufficient
```

**Recommendation:** Option B - current load balancing is production-ready. Work stealing adds complexity without proven benefit.

---

## MEDIUM PRIORITY ISSUES

### 6. "For Now" Temporary Solutions

#### 6a. Pool Health Check Assumption
**File:** `packages/candle/src/pool/core/types.rs:351-353`  
**Line:** 351-353  

**Current Code:**
```rust
Err(_) => {
    // Channel empty or closed - assume alive for now
    // (worker may not have responded yet)
    true
}
```

**Issue:**
- Assumes worker is alive when health check channel is empty/closed
- "for now" indicates temporary solution
- Could mask dead workers if they fail to respond

**Resolution:**
```rust
Err(mpsc::error::TryRecvError::Empty) => {
    // Channel empty - worker hasn't responded yet
    // Check staleness: if last_used is too old, worker may be dead
    let now = unix_timestamp_secs();
    let last = self.last_used.load(Ordering::Acquire);
    let age_secs = now.saturating_sub(last);
    
    // Consider dead if no activity for 60 seconds
    age_secs < 60
}
Err(mpsc::error::TryRecvError::Disconnected) => {
    // Channel disconnected - worker is definitely dead
    false
}
```

#### 6b. Health Metrics Hardcoded Values
**File:** `packages/candle/src/memory/monitoring/health.rs:389-391`  
**Line:** 389-391  

**Current Code:**
```rust
// For now, return hardcoded values
let dimensions = embedding_dims as u32;
let index_quality = 100.0f32; // Assume healthy if count() succeeds
```

**Issue:**
- Index quality hardcoded to 100% (perfect)
- Masks real index health issues
- No actual quality measurement

**Resolution:**
```rust
// Calculate actual index quality metrics
let index_quality = self.calculate_index_quality(&store).await?;

async fn calculate_index_quality<S: VectorStore>(&self, store: &S) -> Result<f32, HealthError> {
    // Sample vectors and check retrieval accuracy
    let sample_size = 100.min(store.count().await? / 10);
    let mut correct_retrievals = 0;
    
    for _ in 0..sample_size {
        // Test vector retrieval accuracy
        if self.test_retrieval_accuracy(store).await? {
            correct_retrievals += 1;
        }
    }
    
    // Return percentage as quality score
    Ok((correct_retrievals as f32 / sample_size as f32) * 100.0)
}
```

#### 6c. Base Memory Accessor Temporary
**File:** `packages/candle/src/memory/core/primitives/node.rs:138-141`  
**Line:** 138-141  

**Current Code:**
```rust
// Temporary accessor for base memory - for now just returns self
pub fn base_memory(&self) -> &Self {
    self
}
```

**Issue:**
- Method name suggests accessing a "base" memory but just returns self
- "for now" indicates temporary implementation
- Unclear purpose of this method

**Resolution:**

**If keeping method:**
```rust
/// Get reference to this memory node
/// 
/// Note: This method exists for API compatibility and simply returns self.
/// Memory nodes are not hierarchical in current implementation.
#[inline]
pub fn as_memory(&self) -> &Self {
    self
}
```

**If removing:**
```rust
// Remove method - callers can use the node directly
// No need for identity wrapper
```

---

## FALSE POSITIVES (Not Issues)

### spawn_blocking Usage (131 matches)
**Status:** ✅ CORRECT - NOT AN ISSUE

**Files:** Multiple files in `capability/text_to_image/`, `memory/vector/`, `domain/context/`

**Analysis:**
- `tokio::spawn_blocking` is the CORRECT way to handle CPU-intensive operations
- Used for:
  - Model loading (flux_schnell.rs, stable_diffusion_35_turbo.rs)
  - Tokenizer initialization (synchronous I/O)
  - Vector search operations (CPU-intensive)
  - File system operations (glob matching)

**This is production-ready async code, not a problem.**

---

### "in production" References (4 matches)
**Status:** ✅ DOCUMENTATION - NOT AN ISSUE

**Examples:**
- `pool/mod.rs:180` - Documentation explaining production deployment strategy
- `domain/chat/realtime/events.rs:412` - Method name `is_production_level()`
- `memory/monitoring/operations.rs:4` - Documentation about production performance

**These are documentation strings and method names, not placeholder code.**

---

### "fallback" / "fall back" References (2 matches)
**Status:** ✅ LEGITIMATE LOGIC - NOT AN ISSUE

**Examples:**
- `pool/maintenance.rs:228` - Fallback to default memory value
- `domain/context/provider.rs:751` - Fall back to extension-based detection

**These are proper error handling and fallback logic, not hacks.**

---

## LANGUAGE IMPROVEMENTS NEEDED

### Inaccurate Comments to Fix

#### 1. "Simplified" Should Be "Placeholder"
**File:** `packages/candle/src/domain/chat/templates/engines.rs:55-56`  
**Current:** `// Simplified`  
**Should Be:** `// TODO: Implement proper JSON serialization for arrays and objects`

#### 2. "Assume" Language
**File:** `packages/candle/src/memory/monitoring/health.rs:391`  
**Current:** `// Assume healthy if count() succeeds`  
**Should Be:** `// TODO: Implement actual index quality measurement`

#### 3. Remove "For Now" Phrases
**Files:** Multiple  
**Action:** Replace "for now" with specific TODO or implementation plan

---

## SUMMARY

**Critical Issues:** 5 items requiring immediate action  
**Medium Priority:** 3 items requiring resolution before production  
**False Positives:** 137+ items that are actually correct  
**Language Improvements:** 3 documentation fixes

**Next Steps:**
1. Fix template engine array/object rendering (Critical #1)
2. Implement temporal distance calculation (Critical #2)
3. Remove or implement execution queue (Critical #4)
4. Remove work stealing placeholder (Critical #5)
5. Improve health check logic (Medium #6)
