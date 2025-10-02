# NOCODE_3: Eliminate Quantum Cognitive Dead Code

## OBJECTIVE
Ruthlessly delete ALL experimental quantum-inspired cognitive features that are not actively integrated into production code paths.

## PRIORITY
🔴 CRITICAL - Zero dead code tolerance

## BACKGROUND
`domain/memory/cognitive/types.rs` contains extensive quantum-inspired features with `#[allow(dead_code)]` annotations. These represent experimental/research code that may not be production-ready.

**Rule:** If it's not actively used in quantum_router.rs or entanglement discovery, it gets DELETED.

## AFFECTED FILES
- `packages/candle/src/domain/memory/cognitive/types.rs`

---

## SUBTASK 1: Audit Quantum Feature Usage

**Action:** Determine which quantum features are actively used

**Commands:**
```bash
cd /Volumes/samsung_t9/paraphym

# Check quantum router usage
grep -rn "activation_pattern\|temporal_context\|collapse_probability\|quantum_entropy\|decoherence" packages/candle/src/memory/cognitive/quantum --include="*.rs"

# Check entanglement usage
grep -rn "activation_pattern\|temporal_context" packages/candle/src/memory/core/cognitive_worker.rs

# Find all dead_code in cognitive types
grep -n "#\[allow(dead_code)\]" packages/candle/src/domain/memory/cognitive/types.rs
```

**Create usage map:**
- List each field/method with dead_code annotation
- Mark as USED or UNUSED based on grep results
- USED → integrate and remove annotation
- UNUSED → DELETE

---

## SUBTASK 2: Delete or Integrate activation_pattern

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Line:** ~25

**Current:**
```rust
#[allow(dead_code)] // TODO: Implement in cognitive state system
activation_pattern: AlignedActivationPattern,
```

**Decision Tree:**

**If used in quantum routing or cognitive processing:**
- Remove `#[allow(dead_code)]` annotation
- Verify it's actually used in code paths
- Keep AlignedActivationPattern struct

**If NOT used:**
- DELETE field from CognitiveState struct
- DELETE AlignedActivationPattern struct entirely (if not used elsewhere)
- Remove from Debug, Clone, new() constructor

---

## SUBTASK 3: Delete or Integrate temporal_context

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Line:** ~38

**Current:**
```rust
#[allow(dead_code)] // TODO: Implement in cognitive state system
temporal_context: Arc<CachePadded<TemporalContext>>,
```

**Decision Tree:**

**If used in temporal memory processing:**
- Remove `#[allow(dead_code)]` annotation
- Verify integration in cognitive state
- Keep TemporalContext type

**If NOT used:**
- DELETE field from CognitiveState struct
- DELETE TemporalContext struct and related code
- Clean up all temporal_context references

---

## SUBTASK 4: Delete Quantum Probability/Entropy Code

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Lines:** Multiple (541, 547, 589, 601, 608, 616)

**Target code:**
```rust
#[allow(dead_code)] // TODO: Implement quantum collapse probability defaults
#[allow(dead_code)] // TODO: Implement quantum entropy defaults  
#[allow(dead_code)] // TODO: Implement quantum decoherence calculation
#[allow(dead_code)] // TODO: Implement collapse probability getter
#[allow(dead_code)] // TODO: Implement collapse probability setter
#[allow(dead_code)] // TODO: Implement quantum entropy getter
```

**Action:** DELETE all quantum probability/entropy code

**Rationale:**
- These are theoretical quantum features
- Not used in actual quantum_router.rs
- Over-engineered for production use
- Can be restored from git if truly needed

**Delete:**
- All methods marked with quantum probability TODOs
- All methods marked with quantum entropy TODOs
- All methods marked with decoherence TODOs
- Related fields in structs

---

## SUBTASK 5: Delete Quantum Entanglement Bonds (If Unused)

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Line:** ~692

**Current:**
```rust
#[allow(dead_code)] // TODO: Implement quantum entanglement bonds
```

**Action:** Check if entanglement bonds are used

**If used in entanglement discovery (cognitive_worker.rs):**
- Keep and remove `#[allow(dead_code)]`

**If NOT used:**
- DELETE the field/method
- Entanglement already handled via EntanglementLink in quantum/entanglement.rs

---

## SUBTASK 6: Clean Up CognitivePattern Dead Code

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Lines:** ~78, 88, 98, 105

**Target:**
```rust
#[allow(dead_code)] // TODO: Implement in cognitive pattern system
```

**Action:** DELETE all cognitive pattern experimental code

**Delete:**
- Unused pattern matching methods
- Unused pattern evolution code
- Unused pattern recognition features
- Keep only what's actively used in cognitive processing

---

## SUBTASK 7: Clean Up Temporal Context Dead Code

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Lines:** ~335, 342, 360

**Target:**
```rust
#[allow(dead_code)] // TODO: Implement in temporal context system
```

**Action:**

**If temporal context is deleted in SUBTASK 3:**
- Delete ALL TemporalContext-related code
- Delete the entire TemporalContext struct
- Delete all methods

**If temporal context is kept:**
- Remove all `#[allow(dead_code)]` annotations
- Implement the methods fully
- No half-implemented features allowed

---

## SUBTASK 8: Clean Up Causal Reasoning Dead Code

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Line:** ~397

**Target:**
```rust
#[allow(dead_code)] // TODO: Implement causal reasoning in cognitive state system
```

**Action:** DELETE causal reasoning experimental code

**Rationale:**
- Causal reasoning is not implemented
- Over-ambitious feature
- Delete now, implement in future dedicated task if needed

---

## SUBTASK 9: Delete Quantum-Inspired Processing Marker

**File:** `packages/candle/src/domain/memory/cognitive/types.rs`  
**Line:** ~416

**Target:**
```rust
#[allow(dead_code)] // TODO: Implement quantum-inspired memory processing
```

**Action:** DELETE or IMPLEMENT

**Decision:**
- If this is a feature flag/marker: DELETE it
- If this is actual code: Implement or delete

---

## SUBTASK 10: Final Dead Code Sweep

**Action:** Eliminate ALL remaining dead_code in cognitive types

**Commands:**
```bash
# Find any remaining dead code
grep -n "#\[allow(dead_code)\]" packages/candle/src/domain/memory/cognitive/types.rs
```

**For each result:**
1. Determine if feature is used
2. Used → Remove annotation, verify integration
3. Not used → DELETE the code

**Goal:** ZERO `#[allow(dead_code)]` annotations

---

## DEFINITION OF DONE

### Zero Dead Code
- [ ] NO `#[allow(dead_code)]` annotations in cognitive/types.rs
- [ ] NO quantum probability/entropy experimental code
- [ ] NO causal reasoning stubs
- [ ] NO temporal context code (unless fully integrated)
- [ ] NO unused pattern matching code

### File Size Reduction
- [ ] Significant reduction in file size (delete unused code)
- [ ] Only production-active code remains
- [ ] All kept code is actively used in quantum router or cognitive processing

### Compilation
- [ ] Code compiles without warnings
- [ ] No broken references to deleted code
- [ ] Quantum router still works (if it uses any cognitive types)

---

## CONSTRAINTS
- ❌ DO NOT write unit tests
- ❌ DO NOT write integration tests
- ❌ DO NOT write benchmarks
- ❌ DO NOT keep "experimental" code in production
- ✅ DELETE ruthlessly
- ✅ Keep only actively used code

---

## TECHNICAL NOTES

### Quantum Router Integration Check
Verify which cognitive types are actually used:
```bash
grep -rn "CognitiveState\|AlignedActivationPattern\|TemporalContext" packages/candle/src/memory/cognitive/quantum/router.rs
```

### Entanglement Integration Check
```bash
grep -rn "CognitiveState" packages/candle/src/memory/core/cognitive_worker.rs
```

### Safe Deletion Strategy
1. Comment out dead code first
2. Compile and verify nothing breaks
3. If compilation succeeds, delete permanently
4. If compilation fails, investigate the usage and integrate properly

### Git Recovery
All deleted code preserved in git:
```bash
git log --all --full-history -- packages/candle/src/domain/memory/cognitive/types.rs
```

### Expected Deletions
Based on dead_code annotations, expect to delete:
- ~200-400 lines of experimental quantum code
- Multiple unused structs and methods
- Theoretical quantum physics implementations
- Placeholder causal reasoning code

### What Should Remain
- Core CognitiveState struct
- Actively used fields (attention_weights, working_memory, long_term_memory)
- Quantum signature (if used in router)
- Statistics tracking
- Any code actively called in cognitive_worker.rs or quantum_router.rs

## PHILOSOPHY

**Quantum-inspired features are valuable in research, not in production.**

- If we need quantum experiments, create a separate research crate
- Production code should be simple, tested, and actively used
- Theoretical features belong in papers, not in src/
