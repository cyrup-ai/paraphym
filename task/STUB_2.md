# STUB_2: Restore Activation Pattern Processing Pipeline

## CRITICAL DISCOVERY

**THE ACTIVATION PIPELINE IS ALREADY FULLY IMPLEMENTED** - This task is about restoring connections, NOT writing new code.

All required methods exist and are correctly implemented. The only issues are:
1. Methods marked with `#[allow(dead_code)]` preventing compilation warnings
2. Missing integration calls to wire up the existing pipeline
3. Missing cognitive_state fields in coordinator/retrieval components

## CORE OBJECTIVE

Restore the activation pattern processing pipeline by:
1. **Removing dead_code annotations** from 6 fully-implemented methods
2. **Adding cognitive_state fields** to MemoryCoordinator and SemanticRetrieval
3. **Wiring integration points** to call update_activation_from_stimulus() when embeddings are available
4. **Ensuring cargo check passes** without warnings

## IMPLEMENTATION STATUS

### ✅ COMPLETE: CognitiveState with Activation Pattern

**File:** [`packages/candle/src/domain/memory/cognitive/types/state.rs`](../../packages/candle/src/domain/memory/cognitive/types/state.rs)

**Line 26-32:** CognitiveState struct with activation_pattern field
```rust
#[derive(Debug, Clone)]
pub struct CognitiveState {
    /// SIMD-aligned activation pattern for parallel neural processing
    ///
    /// Updated via `update_activation()` when processing stimuli.
    /// Energy calculations drive attention weight updates.
    /// Remains `#[allow(dead_code)]` until cognitive system fully activated.
    #[allow(dead_code)] // ← REMOVE THIS LINE
    activation_pattern: AlignedActivationPattern,
    
    attention_weights: Arc<AtomicAttentionWeights>,
    // ... other fields
}
```

**Line 110-123:** Initialization in `new()` method - ALREADY CREATES activation_pattern
```rust
pub fn new() -> Self {
    Self {
        activation_pattern: AlignedActivationPattern::default(), // ← Memory allocated
        attention_weights: default_attention_weights(),
        working_memory: default_working_memory(),
        // ... rest of initialization
    }
}
```

### ✅ COMPLETE: update_activation_from_stimulus() Method

**File:** [`packages/candle/src/domain/memory/cognitive/types/state.rs`](../../packages/candle/src/domain/memory/cognitive/types/state.rs)  
**Line 396-428:** FULLY IMPLEMENTED activation update pipeline

```rust
#[allow(dead_code)] // ← REMOVE THIS LINE
pub fn update_activation_from_stimulus(
    &mut self,
    stimulus: Vec<f32>,
) -> Result<(), CognitiveError> {
    // Validate stimulus is not empty
    if stimulus.is_empty() {
        return Err(CognitiveError::OperationFailed(
            "Stimulus vector cannot be empty".to_string(),
        ));
    }

    // Step 1: Update activation pattern with stimulus data
    self.activation_pattern.update(stimulus);

    // Step 2: Apply sigmoid activation: σ(x) = 1 / (1 + e^(-x))
    self.activation_pattern
        .apply_activation(|x| 1.0 / (1.0 + (-x).exp()));

    // Step 3: Calculate activation energy for attention update
    let energy = self.activation_pattern.energy();
    
    // Step 4: Normalize energy by pattern dimension
    #[allow(clippy::cast_precision_loss)]
    let normalized_energy = (energy / self.activation_pattern.dimension as f32)
        .sqrt()
        .clamp(0.0, 1.0);

    // Step 5: Update attention weights based on activation energy
    self.attention_weights.update_from_energy(normalized_energy);

    Ok(())
}
```

**THE COMPLETE PIPELINE ALREADY EXISTS:**
- ✅ Stimulus validation
- ✅ Pattern update
- ✅ Sigmoid activation
- ✅ Energy calculation
- ✅ Energy normalization
- ✅ Attention weight update

### ✅ COMPLETE: AlignedActivationPattern Methods

**File:** [`packages/candle/src/domain/memory/cognitive/types/activation.rs`](../../packages/candle/src/domain/memory/cognitive/types/activation.rs)

**Line 31-40:** update() method
```rust
#[allow(dead_code)] // ← REMOVE THIS LINE
#[inline]
pub fn update(&mut self, new_data: Vec<f32>) {
    if new_data.len() == self.dimension {
        self.data = new_data;
        self.last_update = SystemTime::now();
    }
}
```

**Line 42-50:** apply_activation() method
```rust
#[allow(dead_code)] // ← REMOVE THIS LINE
#[inline]
pub fn apply_activation(&mut self, activation_fn: impl Fn(f32) -> f32) {
    for value in &mut self.data {
        *value = activation_fn(*value);
    }
    self.last_update = SystemTime::now();
}
```

**Line 52-58:** energy() method - L2 norm calculation
```rust
#[allow(dead_code)] // ← REMOVE THIS LINE
#[inline]
#[must_use]
pub fn energy(&self) -> f32 {
    self.data.iter().map(|x| x * x).sum::<f32>().sqrt()
}
```

**Line 60-66:** is_empty() method
```rust
#[allow(dead_code)] // ← REMOVE THIS LINE
#[inline]
#[must_use]
pub fn is_empty(&self) -> bool {
    self.data.is_empty()
}
```

### ✅ COMPLETE: AtomicAttentionWeights.update_from_energy()

**File:** [`packages/candle/src/domain/memory/cognitive/types/attention.rs`](../../packages/candle/src/domain/memory/cognitive/types/attention.rs)  
**Line 98-106:** Energy-to-attention mapping (NOT DEAD CODE - already in use)

```rust
/// Update primary attention weight from normalized activation energy
///
/// Maps activation energy [0, 1] to primary attention weight.
/// Other weights are adjusted proportionally to maintain normalization.
pub fn update_from_energy(&self, energy: f32) {
    let clamped = energy.clamp(0.0, 1.0);
    self.set_primary(clamped);

    // Reduce other weights proportionally
    let remaining = 1.0 - clamped;
    self.set_secondary(remaining * 0.5);
    self.set_background(remaining * 0.3);
    self.set_meta(remaining * 0.2);
}
```

**FORMULA:** 
- Primary attention = energy
- Secondary = (1 - energy) × 0.5
- Background = (1 - energy) × 0.3  
- Meta = (1 - energy) × 0.2
- Sum = 1.0 ✓

## TASK 1: REMOVE DEAD CODE ANNOTATIONS

### Changes to CognitiveState

**File:** [`packages/candle/src/domain/memory/cognitive/types/state.rs`](../../packages/candle/src/domain/memory/cognitive/types/state.rs)

**Line 32:** Remove `#[allow(dead_code)]` from activation_pattern field
```rust
// BEFORE:
#[allow(dead_code)] // TODO: Implement in cognitive state system
activation_pattern: AlignedActivationPattern,

// AFTER:
activation_pattern: AlignedActivationPattern,
```

**Line 396:** Remove `#[allow(dead_code)]` from update_activation_from_stimulus()
```rust
// BEFORE:
#[allow(dead_code)] // TODO: Implement activation pattern update from stimulus
pub fn update_activation_from_stimulus(

// AFTER:
pub fn update_activation_from_stimulus(
```

### Changes to AlignedActivationPattern

**File:** [`packages/candle/src/domain/memory/cognitive/types/activation.rs`](../../packages/candle/src/domain/memory/cognitive/types/activation.rs)

**Line 31:** Remove from update()
**Line 42:** Remove from apply_activation()
**Line 52:** Remove from energy()
**Line 60:** Remove from is_empty()

All follow same pattern:
```rust
// BEFORE:
#[allow(dead_code)] // TODO: Implement in cognitive pattern system
pub fn method_name(...) {

// AFTER:
pub fn method_name(...) {
```

## TASK 2: ADD COGNITIVE_STATE FIELDS

### Integration Point 1: MemoryCoordinator

**Current Struct Definition:**  
**File:** [`packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`](../../packages/candle/src/memory/core/manager/coordinator/lifecycle.rs)  
**Line 19-37:**

```rust
#[allow(dead_code)]
#[derive(Clone)]
pub struct MemoryCoordinator {
    pub(super) surreal_manager: Arc<SurrealDBMemoryManager>,
    pub(super) repository: Arc<RwLock<MemoryRepository>>,
    pub(super) embedding_model: TextEmbeddingModel,
    // NEW COGNITIVE FIELDS:
    pub(super) cognitive_queue: Arc<CognitiveProcessingQueue>,
    pub(super) committee_evaluator: Arc<ModelCommitteeEvaluator>,
    pub(super) quantum_router: Arc<QuantumRouter>,
    pub(super) quantum_state: Arc<RwLock<QuantumState>>,
    pub(super) cognitive_workers: Arc<tokio::sync::RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    // LAZY EVALUATION FIELDS:
    pub(super) lazy_eval_strategy: LazyEvalStrategy,
    pub(super) evaluation_cache: Cache<String, f64>,
    // TEMPORAL DECAY:
    pub(super) decay_rate: f64,
}
```

**ADD THIS FIELD** after quantum_state:
```rust
pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,
```

**Updated Struct:**
```rust
#[allow(dead_code)]
#[derive(Clone)]
pub struct MemoryCoordinator {
    pub(super) surreal_manager: Arc<SurrealDBMemoryManager>,
    pub(super) repository: Arc<RwLock<MemoryRepository>>,
    pub(super) embedding_model: TextEmbeddingModel,
    pub(super) cognitive_queue: Arc<CognitiveProcessingQueue>,
    pub(super) committee_evaluator: Arc<ModelCommitteeEvaluator>,
    pub(super) quantum_router: Arc<QuantumRouter>,
    pub(super) quantum_state: Arc<RwLock<QuantumState>>,
    pub(super) cognitive_state: Arc<RwLock<CognitiveState>>, // ← NEW FIELD
    pub(super) cognitive_workers: Arc<tokio::sync::RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    pub(super) lazy_eval_strategy: LazyEvalStrategy,
    pub(super) evaluation_cache: Cache<String, f64>,
    pub(super) decay_rate: f64,
}
```

**Initialize in MemoryCoordinator::new():**  
**File:** [`packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`](../../packages/candle/src/memory/core/manager/coordinator/lifecycle.rs)  
**Line 78-95:** In the Ok(Self { ... }) block, add:

```rust
cognitive_state: Arc::new(RwLock::new(CognitiveState::new())),
```

### Integration Point 2: SemanticRetrieval

**Current Struct Definition:**  
**File:** [`packages/candle/src/memory/core/ops/retrieval/semantic.rs`](../../packages/candle/src/memory/core/ops/retrieval/semantic.rs)  
**Line 18-23:**

```rust
/// Semantic similarity retrieval using vector embeddings
pub struct SemanticRetrieval<V: VectorStore> {
    vector_store: Arc<V>,
}
```

**ADD cognitive_state FIELD:**
```rust
/// Semantic similarity retrieval using vector embeddings
pub struct SemanticRetrieval<V: VectorStore> {
    vector_store: Arc<V>,
    cognitive_state: Option<Arc<RwLock<CognitiveState>>>, // ← NEW FIELD
}
```

**Update constructor:**
```rust
// CURRENT (Line 25-29):
impl<V: VectorStore> SemanticRetrieval<V> {
    pub fn new(vector_store: V) -> Self {
        Self {
            vector_store: Arc::new(vector_store),
        }
    }
}

// ADD NEW CONSTRUCTOR:
impl<V: VectorStore> SemanticRetrieval<V> {
    pub fn new(vector_store: V) -> Self {
        Self {
            vector_store: Arc::new(vector_store),
            cognitive_state: None,
        }
    }

    pub fn with_cognitive_state(
        vector_store: V,
        cognitive_state: Arc<RwLock<CognitiveState>>,
    ) -> Self {
        Self {
            vector_store: Arc::new(vector_store),
            cognitive_state: Some(cognitive_state),
        }
    }
}
```

## TASK 3: WIRE UP INTEGRATION POINTS

### Integration Point 1: Memory Retrieval

**File:** [`packages/candle/src/memory/core/manager/coordinator/operations.rs`](../../packages/candle/src/memory/core/manager/coordinator/operations.rs)  
**Method:** `get_memory()` starting at line 166  
**Current flow (Line 166-175):**

```rust
pub async fn get_memory(&self, memory_id: &str) -> Result<Option<MemoryNode>> {
    // Retrieve from SurrealDB
    let memory_node = match self.surreal_manager.get_memory(memory_id).await? {
        Some(node) => node,
        None => return Ok(None),
    };

    // Convert to domain node
    let mut domain_memory = self.convert_memory_to_domain_node(&memory_node)?;
    
    // Apply temporal decay before returning
    self.apply_temporal_decay(&mut domain_memory).await?;
    
    // ... rest of method
}
```

**WHERE TO ADD:** After converting to domain_memory (Line 175) but before temporal decay

**WHAT TO ADD:**
```rust
// Generate stimulus from memory embedding and update cognitive state
if let Some(ref embedding) = domain_memory.embedding {
    let stimulus = embedding.data().to_vec();
    if let Some(ref cognitive_state) = self.cognitive_state {
        match cognitive_state.write().await.update_activation_from_stimulus(stimulus) {
            Ok(()) => {
                log::trace!("Updated cognitive activation from memory retrieval: {}", memory_id);
            }
            Err(e) => {
                log::warn!("Failed to update cognitive activation from memory retrieval: {}", e);
            }
        }
    }
}
```

**UPDATED METHOD:**
```rust
pub async fn get_memory(&self, memory_id: &str) -> Result<Option<MemoryNode>> {
    // Retrieve from SurrealDB
    let memory_node = match self.surreal_manager.get_memory(memory_id).await? {
        Some(node) => node,
        None => return Ok(None),
    };

    // Convert to domain node
    let mut domain_memory = self.convert_memory_to_domain_node(&memory_node)?;

    // Generate stimulus from memory embedding and update cognitive state
    if let Some(ref embedding) = domain_memory.embedding {
        let stimulus = embedding.data().to_vec();
        if let Some(ref cognitive_state) = self.cognitive_state {
            match cognitive_state.write().await.update_activation_from_stimulus(stimulus) {
                Ok(()) => {
                    log::trace!("Updated cognitive activation from memory retrieval: {}", memory_id);
                }
                Err(e) => {
                    log::warn!("Failed to update cognitive activation from memory retrieval: {}", e);
                }
            }
        }
    }
    
    // Apply temporal decay before returning
    self.apply_temporal_decay(&mut domain_memory).await?;
    
    // ... rest of method unchanged
}
```

### Integration Point 2: Vector Search Results

**File:** [`packages/candle/src/memory/core/ops/retrieval/semantic.rs`](../../packages/candle/src/memory/core/ops/retrieval/semantic.rs)  
**Method:** `retrieve()` at line 30  
**Current flow (Line 30-62):**

```rust
fn retrieve(
    &self,
    query: String,
    limit: usize,
    filter: Option<MemoryFilter>,
) -> PendingRetrieval {
    let (tx, rx) = oneshot::channel();
    let vector_store = self.vector_store.clone();

    tokio::spawn(async move {
        let result: Result<Vec<RetrievalResult>> = (async {
            // Generate query embedding
            let query_embedding = vector_store.embed(query).await?;

            // Search in vector store
            let search_stream = vector_store.search(query_embedding, limit, filter);

            // Collect all results from the stream
            let results: Vec<_> = search_stream.collect().await;
            
            // ... rest of method
        }).await;
        
        let _ = tx.send(result);
    });

    PendingRetrieval::new(rx)
}
```

**WHERE TO ADD:** After embedding generation (Line 42), before search

**WHAT TO ADD:**
```rust
// Update cognitive state with query embedding as stimulus
if let Some(ref cognitive_state) = self.cognitive_state {
    let stimulus = query_embedding.clone();
    match cognitive_state.write().await.update_activation_from_stimulus(stimulus) {
        Ok(()) => {
            log::trace!("Updated cognitive activation from query embedding");
        }
        Err(e) => {
            log::warn!("Failed to update cognitive activation from query: {}", e);
        }
    }
}
```

**UPDATED METHOD:**
```rust
fn retrieve(
    &self,
    query: String,
    limit: usize,
    filter: Option<MemoryFilter>,
) -> PendingRetrieval {
    let (tx, rx) = oneshot::channel();
    let vector_store = self.vector_store.clone();
    let cognitive_state = self.cognitive_state.clone(); // ← Clone Arc for spawn

    tokio::spawn(async move {
        let result: Result<Vec<RetrievalResult>> = (async {
            // Generate query embedding
            let query_embedding = vector_store.embed(query).await?;

            // Update cognitive state with query embedding as stimulus
            if let Some(ref cognitive_state) = cognitive_state {
                let stimulus = query_embedding.clone();
                match cognitive_state.write().await.update_activation_from_stimulus(stimulus) {
                    Ok(()) => {
                        log::trace!("Updated cognitive activation from query embedding");
                    }
                    Err(e) => {
                        log::warn!("Failed to update cognitive activation from query: {}", e);
                    }
                }
            }

            // Search in vector store
            let search_stream = vector_store.search(query_embedding, limit, filter);

            // Collect all results from the stream
            let results: Vec<_> = search_stream.collect().await;
            
            // ... rest unchanged
        }).await;
        
        let _ = tx.send(result);
    });

    PendingRetrieval::new(rx)
}
```

### Integration Point 3: Quantum Router (Already Uses CognitiveState)

**File:** [`packages/candle/src/memory/cognitive/quantum/router.rs`](../../packages/candle/src/memory/cognitive/quantum/router.rs)  
**Method:** `route()` at line 42

**ALREADY HAS ACCESS:** Method signature includes `cognitive_state: Option<&CognitiveState>`

```rust
pub async fn route(
    &self,
    query: EnhancedQuery,
    cognitive_state: Option<&CognitiveState>,
) -> Result<RoutingDecision, QuantumRouterError> {
    // Extract quantum metrics from cognitive state
    let (coherence, entropy, collapse_prob) = if let Some(state) = cognitive_state {
        let coherence = state.coherence_state_probability();
        let entropy = state.quantum_entropy();
        let collapse_prob = state.quantum_collapse_probability();
        // ... uses these metrics for routing decisions
    }
    // ...
}
```

**NO CHANGES NEEDED** - QuantumRouter already consumes CognitiveState as a parameter.  
The router uses quantum metrics (coherence, entropy, collapse_prob) to adjust routing confidence.

## WHAT IS A "STIMULUS"?

Based on the implementation, a stimulus is:
- **Type:** `Vec<f32>` - a vector of floating-point activation values
- **Source:** Embeddings from text, retrieved memories, or other neural representations
- **Dimension:** Must match activation_pattern.dimension for update to work (or dimension auto-adjusts)
- **Purpose:** Represents external neural input that drives cognitive state changes

**Common stimulus sources:**
1. **Memory embeddings** - `MemoryNode.embedding.data()` from retrieved memories
2. **Query embeddings** - From `vector_store.embed(query)` during search
3. **Message embeddings** - Generated from `CandleMessage.content` (future)
4. **Context embeddings** - From loaded context documents (future)

## EMBEDDING FLOW ARCHITECTURE

```
┌─────────────────────────────────────────────────────────┐
│  External Event                                         │
│  (Memory retrieval, Vector search, Message)             │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Generate/Extract Embedding (Vec<f32>)                  │
│                                                          │
│  • Memory: domain_memory.embedding.data().to_vec()      │
│  • Query: vector_store.embed(query_text).await          │
│  • Model: embedding_model.generate(text)                │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  cognitive_state.write().await                          │
│    .update_activation_from_stimulus(embedding)          │
│                                                          │
│  1. Validate stimulus (non-empty)                       │
│  2. activation_pattern.update(stimulus)                 │
│  3. activation_pattern.apply_activation(sigmoid)        │
│  4. energy = activation_pattern.energy() [L2 norm]      │
│  5. normalized = (energy / dim).sqrt().clamp(0,1)       │
│  6. attention_weights.update_from_energy(normalized)    │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Updated Cognitive State                                │
│                                                          │
│  • Activation pattern reflects recent stimulus          │
│  • Attention weights adjusted (primary/secondary/...)   │
│  • Quantum metrics influenced by attention state        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Influences Downstream Operations                       │
│                                                          │
│  • QuantumRouter uses coherence/entropy for routing     │
│  • Attention weights influence memory consolidation     │
│  • Activation energy affects working memory priority    │
└─────────────────────────────────────────────────────────┘
```

## STRUCT RELATIONSHIPS

```
MemoryCoordinator (lifecycle.rs)
├── surreal_manager: Arc<SurrealDBMemoryManager>
├── repository: Arc<RwLock<MemoryRepository>>
├── embedding_model: TextEmbeddingModel ← generates embeddings
├── cognitive_state: Arc<RwLock<CognitiveState>> ← NEW FIELD
├── quantum_router: Arc<QuantumRouter>
└── quantum_state: Arc<RwLock<QuantumState>>

SemanticRetrieval<V: VectorStore> (semantic.rs)
├── vector_store: Arc<V> ← has embed() method
└── cognitive_state: Option<Arc<RwLock<CognitiveState>>> ← NEW FIELD

CognitiveState (state.rs)
├── activation_pattern: AlignedActivationPattern ← REMOVE dead_code
├── attention_weights: Arc<AtomicAttentionWeights>
├── quantum_signature: Arc<QuantumSignature>
└── update_activation_from_stimulus(Vec<f32>) ← REMOVE dead_code

AlignedActivationPattern (activation.rs)
├── data: Vec<f32>
├── dimension: usize
├── update(Vec<f32>) ← REMOVE dead_code
├── apply_activation(Fn) ← REMOVE dead_code
├── energy() -> f32 ← REMOVE dead_code
└── is_empty() -> bool ← REMOVE dead_code

AtomicAttentionWeights (attention.rs)
└── update_from_energy(f32) ← ALREADY ACTIVE (no dead_code)
```

## ENERGY NORMALIZATION FORMULA

From state.rs:419-423:
```rust
let energy = self.activation_pattern.energy();  // L2 norm = sqrt(Σx²)
let normalized_energy = (energy / self.activation_pattern.dimension as f32)
    .sqrt()
    .clamp(0.0, 1.0);
```

**Why sqrt twice?**
1. First sqrt in `energy()`: Computes L2 norm = sqrt(Σx²)
2. Division by dimension: Averages the squared values → mean(x²)
3. Second sqrt: Takes root mean square (RMS) → sqrt(mean(x²))
4. Clamp: Ensures result in [0, 1]
5. Result: Normalized RMS energy suitable for attention weights

**Example calculation:**
- Pattern: [0.5, 0.8, 0.3, 0.6] (dim=4)
- Squared: [0.25, 0.64, 0.09, 0.36] = 1.34
- L2 norm (energy): sqrt(1.34) = 1.158
- Mean squared: 1.34 / 4 = 0.335
- RMS: sqrt(0.335) = 0.579
- Clamped: 0.579 (already in [0,1])

## SIGMOID ACTIVATION FUNCTION

From state.rs:413-414:
```rust
self.activation_pattern
    .apply_activation(|x| 1.0 / (1.0 + (-x).exp()));
```

Standard sigmoid: **σ(x) = 1 / (1 + e^(-x))**
- Maps (-∞, ∞) → (0, 1)
- Smooth non-linearity
- Centered at 0.5 when x=0
- Asymptotic behavior: approaches 0 as x→-∞, approaches 1 as x→+∞

**Why sigmoid?**
- Squashes unbounded embeddings into (0,1) range
- Preserves relative magnitudes (larger inputs → larger outputs)
- Differentiable for potential future gradient-based learning
- Biologically-inspired (similar to neuron activation)

## ATTENTION WEIGHT DISTRIBUTION

From attention.rs:98-106, when energy = 0.7:
- Primary = 0.7 (70% attention to current stimulus)
- Secondary = 0.15 (0.3 × 0.5 = 15% to related context)
- Background = 0.09 (0.3 × 0.3 = 9% to background info)
- Meta = 0.06 (0.3 × 0.2 = 6% to meta-cognition)
- **Sum = 1.0 ✓** (normalized probability distribution)

**Interpretation:**
- High energy (e.g., 0.9) → Strong primary focus, little background
- Low energy (e.g., 0.2) → Distributed attention, more background processing
- This mirrors human attention: intense stimuli capture primary focus

## DEFINITION OF DONE

### Phase 1: Remove Dead Code Annotations
- [ ] Removed `#[allow(dead_code)]` from activation_pattern field (state.rs:32)
- [ ] Removed `#[allow(dead_code)]` from update_activation_from_stimulus() (state.rs:396)
- [ ] Removed `#[allow(dead_code)]` from AlignedActivationPattern.update() (activation.rs:31)
- [ ] Removed `#[allow(dead_code)]` from AlignedActivationPattern.apply_activation() (activation.rs:42)
- [ ] Removed `#[allow(dead_code)]` from AlignedActivationPattern.energy() (activation.rs:52)
- [ ] Removed `#[allow(dead_code)]` from AlignedActivationPattern.is_empty() (activation.rs:60)

### Phase 2: Add Cognitive State Fields
- [ ] Added `cognitive_state: Arc<RwLock<CognitiveState>>` field to MemoryCoordinator (lifecycle.rs:~29)
- [ ] Initialized cognitive_state in MemoryCoordinator::new() (lifecycle.rs:~92)
- [ ] Added `cognitive_state: Option<Arc<RwLock<CognitiveState>>>` to SemanticRetrieval (semantic.rs:~20)
- [ ] Updated SemanticRetrieval::new() to initialize cognitive_state as None (semantic.rs:~27)
- [ ] Added SemanticRetrieval::with_cognitive_state() constructor (semantic.rs:~31)

### Phase 3: Wire Integration Points
- [ ] Wired update_activation_from_stimulus() into get_memory() after embedding extraction (operations.rs:~176)
- [ ] Wired update_activation_from_stimulus() into semantic retrieve() after query embedding (semantic.rs:~43)
- [ ] Cloned cognitive_state Arc for tokio::spawn in retrieve() (semantic.rs:~37)

### Phase 4: Verification
- [ ] `cargo check` passes without warnings about dead code
- [ ] `cargo check -p paraphym_candle` compiles successfully
- [ ] Activation pattern updates occur when memories are retrieved (traced in logs)
- [ ] Activation pattern updates occur when vector searches execute (traced in logs)

## FILE LOCATIONS SUMMARY

```
Core Implementation (All Complete):
├── packages/candle/src/domain/memory/cognitive/types/state.rs
│   ├── Line 26: CognitiveState struct definition
│   ├── Line 32: activation_pattern field (REMOVE dead_code)
│   ├── Line 110: CognitiveState::new() - initializes activation_pattern
│   └── Line 396: update_activation_from_stimulus() (REMOVE dead_code)
├── packages/candle/src/domain/memory/cognitive/types/activation.rs
│   ├── Line 7: AlignedActivationPattern struct
│   ├── Line 31: update() method (REMOVE dead_code)
│   ├── Line 42: apply_activation() method (REMOVE dead_code)
│   ├── Line 52: energy() method (REMOVE dead_code)
│   └── Line 60: is_empty() method (REMOVE dead_code)
└── packages/candle/src/domain/memory/cognitive/types/attention.rs
    └── Line 98: update_from_energy() - ALREADY ACTIVE (no dead_code)

Integration Points (Need Wiring):
├── packages/candle/src/memory/core/manager/coordinator/lifecycle.rs
│   ├── Line 19-37: MemoryCoordinator struct (ADD cognitive_state field)
│   └── Line 78-95: MemoryCoordinator::new() (INITIALIZE cognitive_state)
├── packages/candle/src/memory/core/manager/coordinator/operations.rs
│   └── Line 166-175: get_memory() (ADD activation update after embedding)
├── packages/candle/src/memory/core/ops/retrieval/semantic.rs
│   ├── Line 18-23: SemanticRetrieval struct (ADD cognitive_state field)
│   ├── Line 25-29: SemanticRetrieval::new() (INITIALIZE as None)
│   ├── Line ~31: ADD with_cognitive_state() constructor
│   └── Line 30-62: retrieve() (ADD activation update after query embedding)
└── packages/candle/src/memory/cognitive/quantum/router.rs
    └── Line 42: route() - ALREADY HAS CognitiveState access (no changes)
```

## IMPLEMENTATION SEQUENCE

Execute in this order to minimize compilation errors:

### Step 1: Remove Dead Code Annotations (Safe - No Behavior Change)
1. Edit `packages/candle/src/domain/memory/cognitive/types/activation.rs`
   - Remove 4 `#[allow(dead_code)]` annotations (lines 31, 42, 52, 60)
2. Edit `packages/candle/src/domain/memory/cognitive/types/state.rs`
   - Remove 2 `#[allow(dead_code)]` annotations (lines 32, 396)
3. Run `cargo check -p paraphym_candle` to verify no dead code warnings

### Step 2: Add Cognitive State to MemoryCoordinator
1. Edit `packages/candle/src/memory/core/manager/coordinator/lifecycle.rs`
   - Add field to struct (line ~29): `pub(super) cognitive_state: Arc<RwLock<CognitiveState>>,`
   - Initialize in new() (line ~92): `cognitive_state: Arc::new(RwLock::new(CognitiveState::new())),`
2. Run `cargo check -p paraphym_candle` to verify struct compiles

### Step 3: Wire MemoryCoordinator Integration
1. Edit `packages/candle/src/memory/core/manager/coordinator/operations.rs`
   - Add stimulus update in get_memory() after line 175 (embedding extraction)
2. Run `cargo check -p paraphym_candle` to verify integration compiles

### Step 4: Add Cognitive State to SemanticRetrieval
1. Edit `packages/candle/src/memory/core/ops/retrieval/semantic.rs`
   - Add field to struct: `cognitive_state: Option<Arc<RwLock<CognitiveState>>>,`
   - Update new() to initialize as None
   - Add with_cognitive_state() constructor
2. Run `cargo check -p paraphym_candle` to verify struct compiles

### Step 5: Wire SemanticRetrieval Integration
1. Edit `packages/candle/src/memory/core/ops/retrieval/semantic.rs`
   - Clone cognitive_state for spawn
   - Add stimulus update in retrieve() after line 42 (query embedding)
2. Run `cargo check -p paraphym_candle` to verify integration compiles

### Step 6: Final Verification
1. Run `cargo check` on full workspace
2. Run `cargo build -p paraphym_candle` to verify successful compilation
3. Verify no dead_code warnings appear
4. Verify logs show "Updated cognitive activation from..." when running

## REFERENCES

- [AlignedActivationPattern](../../packages/candle/src/domain/memory/cognitive/types/activation.rs)
- [CognitiveState](../../packages/candle/src/domain/memory/cognitive/types/state.rs)
- [AtomicAttentionWeights](../../packages/candle/src/domain/memory/cognitive/types/attention.rs)
- [MemoryCoordinator](../../packages/candle/src/memory/core/manager/coordinator/lifecycle.rs)
- [MemoryCoordinator Operations](../../packages/candle/src/memory/core/manager/coordinator/operations.rs)
- [SemanticRetrieval](../../packages/candle/src/memory/core/ops/retrieval/semantic.rs)
- [QuantumRouter](../../packages/candle/src/memory/cognitive/quantum/router.rs)
- [VectorStore Trait](../../packages/candle/src/memory/vector/vector_store.rs)
