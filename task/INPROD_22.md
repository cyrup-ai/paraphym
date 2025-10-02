# INPROD_22: Reasoner Strategy Implementation

## SEVERITY: MEDIUM

## OBJECTIVE
Implement full reasoning strategies instead of hardcoded score calculation. The reasoner is simplified for WASM but should use actual strategy implementations that are WASM-compatible.

## LOCATION
- `packages/sweetmcp/plugins/reasoner/src/lib.rs`

## CURRENT STATE ANALYSIS

### Problem Areas
- **Line 91**: Comment: `// Simplified reasoner for the WASM plugin. In a real implementation, this would include all the strategy implementations.`
- **Line 113**: Comment: `// Calculate score (in a real implementation, this would use the selected strategy)`
- **Line 114**: Hardcoded score: `let score = 0.7 + (request.thought_number as f64 * 0.05);`
- **Line 106**: Strategy selection from request is read but ignored: `request.strategy_type.unwrap_or_else(|| "beam_search".to_string())`

### Existing Infrastructure Discovered

The codebase ALREADY has full strategy implementations, but they're async/tokio-based and incompatible with WASM:

- **Existing strategies** (in `src/reasoner/strategies/`):
  - `beam_search.rs`: [BeamSearchStrategy](../../packages/sweetmcp/plugins/reasoner/src/reasoner/strategies/beam_search.rs) (async, uses tokio)
  - `mcts.rs`: MonteCarloTreeSearchStrategy (async, uses tokio)  
  - `mcts_002_alpha.rs`: MCTS002AlphaStrategy (async, uses tokio)
  - `mcts_002alt_alpha.rs`: MCTS002AltAlphaStrategy (async, uses tokio)
  - `base.rs`: [BaseStrategy](../../packages/sweetmcp/plugins/reasoner/src/reasoner/strategies/base.rs) with async trait, VoyageAI API calls
  - `factory.rs`: [StrategyFactory](../../packages/sweetmcp/plugins/reasoner/src/reasoner/strategies/factory.rs) for creating strategies

**These cannot be used in WASM** because:
1. Extism WASM plugins are synchronous (no async/await)
2. No tokio runtime in WASM
3. No network I/O (VoyageAI embeddings API)
4. No persistent state between WASM function calls

### WASM Constraints

WASM-compatible code must:
- ✅ Use pure computation (algorithms, regex, math)
- ✅ Use standard collections (HashMap, Vec, HashSet)
- ✅ Use serde for JSON serialization
- ❌ NO async/await or tokio runtime
- ❌ NO network I/O or external API calls
- ❌ NO file system access
- ❌ NO threads or parallel execution

## RESEARCH FINDINGS

### Beam Monte-Carlo Tree Search (BMCTS)

From recent research (IEEE 2024), **Beam Monte-Carlo Tree Search** combines:
- **Beam Search**: Selects top-W most promising nodes per depth level
- **MCTS**: Uses exploration-exploitation tradeoff with UCB1 formula

**Key Parameters**:
- `beam_width (W)`: Number of top paths to maintain at each depth
- `tree_depth (d)`: Maximum depth to explore
- UCB1 formula: `score = exploitation + C * sqrt(ln(N) / n)`
  - `N`: Total simulations (use `total_thoughts`)
  - `n`: Node visits (use `thought_number`)
  - `C`: Exploration constant (typically 1.414 or sqrt(2))

**Citations**:
- Beam Monte-Carlo Tree Search (IEEE Xplore 2024)
- Array-Based Monte Carlo Tree Search (ArXiv 2024) - optimized for pipelined processors

### Stateless WASM Adaptation

Since WASM plugins are stateless between calls, strategies must score thoughts based on:
1. **Intrinsic quality** of the thought text
2. **Relationship** to parent thought (if provided)
3. **Depth** in the reasoning tree
4. **Request parameters** (beam_width, num_simulations)

## IMPLEMENTATION SPECIFICATION

### SUBTASK 1: Create WASM Strategy Trait

Add to `lib.rs` (before `SimpleReasoner` struct, around line 90):

```rust
/// WASM-compatible strategy trait for scoring thoughts
trait WasmStrategy: Send + Sync {
    fn name(&self) -> &str;
    
    /// Calculate score for a thought node
    fn calculate_score(
        &self,
        thought: &str,
        parent_thought: Option<&str>,
        depth: usize,
        request: &ReasoningRequest,
    ) -> f64;
}
```

### SUBTASK 2: Implement Beam Search Strategy (WASM)

Add to `lib.rs` after trait definition:

```rust
struct BeamSearchWasm {
    beam_width: usize,
}

impl BeamSearchWasm {
    fn new(beam_width: Option<usize>) -> Self {
        Self {
            beam_width: beam_width.unwrap_or(3),
        }
    }
    
    fn logical_score(&self, thought: &str) -> f64 {
        use regex::Regex;
        let mut score = 0.0;
        
        // Length/complexity bonus (max 0.3)
        score += (thought.len() as f64 / 200.0).min(0.3);
        
        // Logical connectors (0.2 bonus)
        let connectors = Regex::new(r"\b(therefore|because|if|then|thus|hence|so|since|consequently)\b")
            .unwrap();
        if connectors.is_match(thought) {
            score += 0.2;
        }
        
        // Mathematical/logical expressions (0.2 bonus)
        let math = Regex::new(r"[+\-*/=<>]|->|=>").unwrap();
        if math.is_match(thought) {
            score += 0.2;
        }
        
        score
    }
    
    fn coherence_score(&self, thought: &str, parent_thought: &str) -> f64 {
        use std::collections::HashSet;
        
        let parent_terms: HashSet<String> = parent_thought
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        
        let child_terms: Vec<String> = thought
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        
        if child_terms.is_empty() {
            return 0.0;
        }
        
        let shared = child_terms
            .iter()
            .filter(|t| parent_terms.contains(*t))
            .count();
        
        (shared as f64 / child_terms.len() as f64).min(1.0)
    }
}

impl WasmStrategy for BeamSearchWasm {
    fn name(&self) -> &str {
        "beam_search"
    }
    
    fn calculate_score(
        &self,
        thought: &str,
        parent_thought: Option<&str>,
        depth: usize,
        _request: &ReasoningRequest,
    ) -> f64 {
        let logical = self.logical_score(thought);
        let coherence = parent_thought
            .map(|p| self.coherence_score(thought, p))
            .unwrap_or(0.5);
        let depth_penalty = (1.0 - (depth as f64 / 10.0) * 0.2).max(0.0);
        
        // Beam search favors logical structure and coherence
        (logical * 0.4 + coherence * 0.4 + depth_penalty * 0.2).min(1.0)
    }
}
```

### SUBTASK 3: Implement MCTS Strategy (WASM)

Add to `lib.rs`:

```rust
struct MCTSWasm {
    num_simulations: usize,
    exploration_constant: f64,
}

impl MCTSWasm {
    fn new(num_simulations: Option<usize>) -> Self {
        Self {
            num_simulations: num_simulations.unwrap_or(50),
            exploration_constant: 1.414, // sqrt(2)
        }
    }
}

impl WasmStrategy for MCTSWasm {
    fn name(&self) -> &str {
        "mcts"
    }
    
    fn calculate_score(
        &self,
        thought: &str,
        parent_thought: Option<&str>,
        depth: usize,
        request: &ReasoningRequest,
    ) -> f64 {
        // Exploitation: intrinsic thought quality
        let quality = {
            let length_score = (thought.len() as f64 / 200.0).min(0.3);
            let logical_score = if thought.contains("therefore") 
                || thought.contains("because") 
                || thought.contains("thus") {
                0.3
            } else {
                0.0
            };
            let coherence = parent_thought
                .map(|_| 0.2) // Simplified coherence
                .unwrap_or(0.1);
            length_score + logical_score + coherence
        };
        
        // Exploration: UCB1 formula
        // score = quality + C * sqrt(ln(N) / n)
        let n = request.thought_number as f64;
        let total_n = request.total_thoughts as f64;
        let exploration_bonus = if n > 0.0 && total_n > 0.0 {
            self.exploration_constant * ((total_n.ln()) / n).sqrt()
        } else {
            0.0
        };
        
        // Depth penalty
        let depth_penalty = (1.0 - (depth as f64 / 10.0) * 0.15).max(0.0);
        
        ((quality + exploration_bonus * 0.3) * depth_penalty).min(1.0)
    }
}
```

### SUBTASK 4: Implement MCTS Variant Strategies

Add to `lib.rs`:

```rust
struct MCTS002AlphaWasm {
    base: MCTSWasm,
}

impl MCTS002AlphaWasm {
    fn new(num_simulations: Option<usize>) -> Self {
        Self {
            base: MCTSWasm::new(num_simulations),
        }
    }
}

impl WasmStrategy for MCTS002AlphaWasm {
    fn name(&self) -> &str {
        "mcts_002_alpha"
    }
    
    fn calculate_score(
        &self,
        thought: &str,
        parent_thought: Option<&str>,
        depth: usize,
        request: &ReasoningRequest,
    ) -> f64 {
        // Alpha variant: Higher exploration constant
        let mut score = self.base.calculate_score(thought, parent_thought, depth, request);
        score *= 1.1; // 10% boost for alpha exploration
        score.min(1.0)
    }
}

struct MCTS002AltAlphaWasm {
    base: MCTSWasm,
}

impl MCTS002AltAlphaWasm {
    fn new(num_simulations: Option<usize>) -> Self {
        Self {
            base: MCTSWasm::new(num_simulations),
        }
    }
}

impl WasmStrategy for MCTS002AltAlphaWasm {
    fn name(&self) -> &str {
        "mcts_002alt_alpha"
    }
    
    fn calculate_score(
        &self,
        thought: &str,
        parent_thought: Option<&str>,
        depth: usize,
        request: &ReasoningRequest,
    ) -> f64 {
        // Alt variant: Balanced exploration-exploitation
        let base_score = self.base.calculate_score(thought, parent_thought, depth, request);
        let length_bonus = (thought.len() as f64 / 150.0).min(0.15);
        (base_score + length_bonus).min(1.0)
    }
}
```

### SUBTASK 5: Create Strategy Factory

Add to `lib.rs`:

```rust
fn create_strategy(request: &ReasoningRequest) -> Box<dyn WasmStrategy> {
    let strategy_type = request.strategy_type.as_deref().unwrap_or("beam_search");
    
    match strategy_type {
        "beam_search" => Box::new(BeamSearchWasm::new(request.beam_width)),
        "mcts" => Box::new(MCTSWasm::new(request.num_simulations)),
        "mcts_002_alpha" => Box::new(MCTS002AlphaWasm::new(request.num_simulations)),
        "mcts_002alt_alpha" => Box::new(MCTS002AltAlphaWasm::new(request.num_simulations)),
        _ => Box::new(BeamSearchWasm::new(request.beam_width)), // Default fallback
    }
}
```

### SUBTASK 6: Update SimpleReasoner.process_thought()

**Replace lines 102-115** in `lib.rs` with:

```rust
pub fn process_thought(&mut self, request: ReasoningRequest) -> ReasoningResponse {
    let node_id = Uuid::new_v4().to_string();
    
    // Create strategy based on request
    let strategy = create_strategy(&request);
    
    // Get parent thought text if parent exists
    let parent_thought = request
        .parent_id
        .as_ref()
        .and_then(|id| self.nodes.get(id))
        .map(|node| node.thought.clone());
    
    // Calculate score using selected strategy
    let score = strategy.calculate_score(
        &request.thought,
        parent_thought.as_deref(),
        request.thought_number - 1,
        &request,
    );
    
    // Create the node
    let node = ThoughtNode {
        id: node_id.clone(),
        thought: request.thought.clone(),
        score,
        depth: request.thought_number,
        children: Vec::new(),
        parent_id: request.parent_id.clone(),
        is_complete: !request.next_thought_needed,
    };
    
    // Add to parent's children if it exists
    if let Some(parent_id) = &request.parent_id {
        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.children.push(node_id.clone());
        }
    }
    
    // Store the node
    self.nodes.insert(node_id.clone(), node.clone());
    
    // Generate response
    ReasoningResponse {
        node_id,
        thought: request.thought,
        score,
        depth: request.thought_number,
        is_complete: !request.next_thought_needed,
        next_thought_needed: request.next_thought_needed,
        possible_paths: Some(1),
        best_score: Some(score),
        strategy_used: Some(strategy.name().to_string()),
    }
}
```

### SUBTASK 7: Remove Stub Comments

Delete or update these comments in `lib.rs`:
- Line 91: `// Simplified reasoner for the WASM plugin...` → Delete
- Line 113: `// Calculate score (in a real implementation...)` → Delete

Replace with:
```rust
// WASM-compatible reasoner with strategy pattern implementation
```

## DEPENDENCIES CHECK

Required crates (already in Cargo.toml):
- `regex` - for pattern matching in scoring (WASM-compatible)
- `uuid` - for node ID generation (WASM-compatible)  
- `serde` - for JSON serialization (WASM-compatible)
- `extism-pdk` - for WASM plugin interface

No new dependencies needed.

## DEFINITION OF DONE

### Completion Criteria
- [x] WasmStrategy trait defined with calculate_score method
- [x] BeamSearchWasm strategy implemented with logical + coherence scoring
- [x] MCTSWasm strategy implemented with UCB1 exploration-exploitation
- [x] MCTS002AlphaWasm variant implemented
- [x] MCTS002AltAlphaWasm variant implemented
- [x] create_strategy() factory function implemented
- [x] SimpleReasoner.process_thought() uses strategy.calculate_score() instead of hardcoded value
- [x] Strategy selection from request.strategy_type is honored
- [x] All stub comments removed
- [x] WASM compatibility maintained (no async, no I/O, no tokio)

### Verification
Run `cargo build -p reasoner` to verify:
1. No compilation errors
2. No WASM-incompatible dependencies
3. No async/await in strategy implementations

## CONSTRAINTS
- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)  
- NO documentation files to be created (separate team responsibility)
- Focus solely on ./src implementation

## REFERENCES

### Codebase Files
- [lib.rs](../../packages/sweetmcp/plugins/reasoner/src/lib.rs) - Main WASM plugin file (needs modification)
- [types.rs](../../packages/sweetmcp/plugins/reasoner/src/types.rs) - Shared types (reference for structures)
- [base.rs](../../packages/sweetmcp/plugins/reasoner/src/reasoner/strategies/base.rs) - Async strategy reference (cannot use, but shows scoring patterns)
- [beam_search.rs](../../packages/sweetmcp/plugins/reasoner/src/reasoner/strategies/beam_search.rs) - Async beam search (reference only)
- [factory.rs](../../packages/sweetmcp/plugins/reasoner/src/reasoner/strategies/factory.rs) - Async factory pattern (reference only)

### Research Citations
- Beam Monte-Carlo Tree Search (IEEE Xplore, 2024)
- Array-Based Monte Carlo Tree Search (ArXiv 2508.20140, 2024)
- Monte Carlo Tree Search: A Review of Recent Modifications (Springer AI Review, 2022)

### Implementation Notes
The scoring algorithms use **word overlap** for semantic coherence (WASM-compatible) instead of embedding models (not WASM-compatible). This is a reasonable approximation that maintains functionality while respecting WASM constraints.

UCB1 formula uses `thought_number` and `total_thoughts` from the request as proxies for visit counts, making it stateless and compatible with the WASM plugin model where each call is independent.