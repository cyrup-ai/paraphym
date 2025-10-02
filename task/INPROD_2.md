# INPROD_2: Delete Deprecated Agent Code

## SEVERITY: CRITICAL - CODE DELETION REQUIRED

## OBJECTIVE

**DELETE deprecated agent files** that contain stub/incomplete implementations.

ALL functionality has been migrated to `domain/agent/` with complete implementations and additional features.

## FILES TO DELETE

```bash
rm packages/candle/src/agent/chat.rs
rm packages/candle/src/agent/role.rs
```

## COMPLETE FEATURE MAPPING: OLD → NEW

### Chat Implementation

| Feature | OLD (agent/chat.rs) | NEW (domain/agent/chat.rs) | Migration Status |
|---------|---------------------|----------------------------|------------------|
| ChatError enum | ✓ Basic | ✓ + Box<MemoryToolError> | ✅ Enhanced |
| ContextInjectionResult | ✓ | ✓ + Default + MessageChunk | ✅ Enhanced |
| MemoryEnhancedChatResponse | ✓ | ✓ + Default + MessageChunk | ✅ Enhanced |
| chat() method | ✓ Uses `Memory` | ✓ Uses `Arc<dyn MemoryManager>` | ✅ Upgraded |
| inject_memory_context() | ❌ **STUB/TODO** | ✅ **FULLY IMPLEMENTED** | ✅ **Completed** |
| calculate_relevance_score() | ✓ Basic | ❌ Replaced | ⚠️ MemoryManager handles scoring |
| memorize_conversation() | ✓ | ✓ + CandleCollectionChunk | ✅ Enhanced |
| generate_ai_response() | ❌ Missing | ✅ **NEW** | ✅ Added |
| generate_ai_response_with_sectioning() | ❌ Missing | ✅ **NEW** | ✅ Added |

### Role Implementation

| Feature | OLD (agent/role.rs) | NEW (domain/agent/role.rs) | Migration Status |
|---------|---------------------|----------------------------|------------------|
| McpServerConfig | Private struct | Public struct + methods | ✅ Enhanced |
| AgentRole trait | ✓ | CandleAgentRole | ✅ Renamed |
| AgentRoleImpl | ✓ Basic fields | CandleAgentRoleImpl + full impl | ✅ Complete |
| AgentRoleAgent | Empty struct | Documented helper | ✅ Enhanced |
| AgentConversation | ✓ | + latest_user_message() | ✅ Enhanced |
| AgentConversationMessage | ✓ | ✓ | ✅ Migrated |
| Context/Tool/History traits | ✓ | Candle-prefixed | ✅ Migrated |
| **Memory system** | with_memory_tool() | with_memory_manager() | ✅ **Upgraded** |
| **Completion provider** | ❌ Missing | ✅ with_completion_provider() | ✅ **Added** |
| **Context loading** | ❌ Missing | ✅ add_context_from_file/directory() | ✅ **Added** |
| **Code execution** | ❌ Missing | ✅ with_code_execution() | ✅ **Added** |
| **Tool execution** | ❌ Missing | ✅ execute_tool() | ✅ **Added** |
| **MCP server mgmt** | ❌ Missing | ✅ add_mcp_server() | ✅ **Added** |
| **Tool initialization** | ❌ Missing | ✅ initialize_tools() | ✅ **Added** |
| **Available tools** | ❌ Missing | ✅ get_available_tools() | ✅ **Added** |
| **Event handlers** | ❌ Missing | ✅ on_tool_result/conversation_turn | ✅ **Added** |
| **JSON conversion** | ❌ Missing | ✅ convert_serde_to_sweet_json() | ✅ **Added** |

## CRITICAL DIFFERENCES

### 1. Memory System Architecture

**OLD (Deprecated)**:
```rust
// agent/chat.rs
impl AgentRoleImpl {
    pub fn chat(
        &self,
        message: impl Into<String>,
        memory: &Memory,  // ❌ OLD concrete type
        memory_tool: &MemoryTool,
    ) -> AsyncStream<MemoryEnhancedChatResponse>
    
    pub fn inject_memory_context(
        &self,
        _message: &str,
        _memory: &Arc<Memory>,
    ) -> AsyncStream<ContextInjectionResult> {
        // ❌ STUB - Returns empty context!
        let injected_context = String::new();
        let relevance_score = 0.0;
    }
}
```

**NEW (Working)**:
```rust
// domain/agent/chat.rs  
impl CandleAgentRoleImpl {
    pub fn chat(
        &self,
        message: impl Into<String>,
        memory_manager: &Arc<dyn MemoryManager>,  // ✅ NEW trait-based
        memory_tool: &MemoryTool,
    ) -> ystream::AsyncStream<MemoryEnhancedChatResponse>
    
    pub fn inject_memory_context(
        &self,
        message: &str,
        memory_manager: &Arc<dyn MemoryManager>,
    ) -> ystream::AsyncStream<ContextInjectionResult> {
        // ✅ FULLY IMPLEMENTED
        let memory_stream = memory_manager.search_by_content(message);
        // ... collect results, convert to RetrievalResult
        // ... format with PromptFormatter
        // ... return actual context
    }
}
```

### 2. Features ONLY in NEW

**Completion Provider System**:
```rust
// domain/agent/role.rs
pub enum CandleCompletionProviderType {
    KimiK2(CandleKimiK2Provider),
    Qwen3Coder(CandleQwen3CoderProvider),
}

impl CandleAgentRoleImpl {
    pub fn with_completion_provider(
        mut self, 
        provider: CandleCompletionProviderType
    ) -> Self
    
    pub fn get_completion_provider(&self) 
        -> Result<&CandleCompletionProviderType, AgentError>
}
```

**Context Loading**:
```rust
// domain/agent/role.rs
impl CandleAgentRoleImpl {
    pub fn add_context_from_file(
        mut self, 
        file_path: impl AsRef<Path>
    ) -> Self
    
    pub fn add_context_from_directory(
        mut self, 
        dir_path: impl AsRef<Path>
    ) -> Self
}
```

**Tool Execution**:
```rust
// domain/agent/role.rs
impl CandleAgentRoleImpl {
    pub fn with_code_execution(mut self, enabled: bool) -> Self
    
    pub async fn initialize_tools(&mut self) -> Result<(), ToolError>
    
    pub fn execute_tool(
        &self, 
        tool_name: &str, 
        args: Value
    ) -> ystream::AsyncStream<CandleJsonChunk>
    
    pub async fn get_available_tools(&self) -> Vec<ToolInfo>
}
```

**MCP Server Management**:
```rust
// domain/agent/role.rs
pub struct McpServerConfig {
    server_type: String,
    bin_path: Option<String>,
    init_command: Option<String>,
}

impl McpServerConfig {
    pub fn stdio(bin_path: impl Into<String>) -> Self
    pub fn socket(init_command: impl Into<String>) -> Self
}

impl CandleAgentRoleImpl {
    pub fn add_mcp_server(mut self, config: McpServerConfig) -> Self
    pub fn get_mcp_servers(&self) -> Option<&ZeroOneOrMany<McpServerConfig>>
}
```

**Event System**:
```rust
// domain/agent/role.rs
impl CandleAgentRoleImpl {
    pub fn on_tool_result<F>(mut self, handler: F) -> Self
    where F: Fn(ZeroOneOrMany<Value>) + Send + Sync + 'static
    
    pub fn on_conversation_turn<F>(mut self, handler: F) -> Self
    where F: Fn(&CandleAgentConversation, &CandleAgentRoleAgent) + Send + Sync + 'static
}
```

## What OLD Code Does (Why It's Broken)

### agent/chat.rs Line 152-153
```rust
// TODO: Implement actual memory querying logic
// For now, return empty context
let injected_context = String::new();
let relevance_score = 0.0;
```

**Problem**: Agent has NO access to conversation history or context. Memory querying is completely non-functional.

### agent/role.rs 
```rust
pub struct AgentRoleImpl {
    completion_provider: Option<Box<dyn std::any::Any + Send + Sync>>,  // ❌ Never used
    contexts: Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>,  // ❌ Never used
    tools: Option<ZeroOneOrMany<Box<dyn std::any::Any + Send + Sync>>>,  // ❌ Never used
    mcp_servers: Option<ZeroOneOrMany<McpServerConfig>>,  // ❌ Never used
    // All fields marked #[allow(dead_code)]
}
```

**Problem**: All advanced fields are unused placeholders. No implementation exists.

## What NEW Code Does (Why It Works)

### Memory Query Implementation (domain/agent/chat.rs:301-408)

```rust
pub fn inject_memory_context(
    &self,
    message: &str,
    memory_manager: &Arc<dyn MemoryManager>,
) -> ystream::AsyncStream<ContextInjectionResult> {
    // 1. Search memory using MemoryManager
    let memory_stream = memory_manager.search_by_content(&message);
    
    // 2. Collect up to MAX_RELEVANT_MEMORIES (10)
    let mut results = Vec::new();
    while let Some(memory_result) = stream.next().await {
        if results.len() >= MAX_RELEVANT_MEMORIES { break; }
        
        // 3. Convert to RetrievalResult with scoring
        let retrieval_result = RetrievalResult {
            id: memory_node.id.clone(),
            score: memory_node.metadata.importance,  // ✅ Actual scoring
            method: RetrievalMethod::Semantic,
            metadata: { /* content, type, importance */ }
        };
        results.push(retrieval_result);
    }
    
    // 4. Calculate average relevance
    let avg_relevance_score = if memory_nodes_used > 0 {
        let total: f32 = results.iter().map(|r| r.score).sum();
        f64::from(total / memory_nodes_used as f32)
    } else {
        0.0
    };
    
    // 5. Format with PromptFormatter
    let formatter = PromptFormatter::new()
        .with_max_memory_length(Some(2000))
        .with_headers(true);
    let injected_context = formatter
        .format_memory_section(&memories)
        .unwrap_or_default();
    
    // 6. Return actual context
    ContextInjectionResult {
        injected_context,      // ✅ Real context, not empty string
        relevance_score,       // ✅ Actual score, not 0.0
        memory_nodes_used,     // ✅ Actual count
    }
}
```

### AI Response Generation (domain/agent/chat.rs:233-274)

```rust
fn generate_ai_response(&self, message: &str, context: &str) -> Result<String, ChatError> {
    // 1. Get configured provider
    let provider = self.get_completion_provider()
        .map_err(|e| ChatError::System(format!("Provider error: {e}")))?;
    
    // 2. Create prompt with context
    let full_prompt = if context.is_empty() {
        message.to_string()
    } else {
        format!("Context: {context}\n\nUser: {message}")
    };
    
    // 3. Call provider for real inference
    let candle_prompt = CandlePrompt::new(full_prompt);
    let candle_params = CandleCompletionParams { /* ... */ };
    let completion_stream = provider.prompt(candle_prompt, &candle_params);
    
    // 4. Process completion chunks
    if let Some(completion_chunk) = completion_stream.try_next() {
        match completion_chunk {
            CandleCompletionChunk::Text(text) | 
            CandleCompletionChunk::Complete { text, .. } => Ok(text),
            CandleCompletionChunk::Error(error) => Err(ChatError::System(error)),
            _ => Err(ChatError::System("Unexpected chunk type".to_string())),
        }
    } else {
        Err(ChatError::System("No response from completion stream".to_string()))
    }
}
```

## Memory System Integration

### MemoryManager Trait (Used by NEW)
**Location**: [`./src/memory/core/manager/surreal.rs`](../packages/candle/src/memory/core/manager/surreal.rs)

```rust
pub trait MemoryManager: Send + Sync {
    fn search_by_content(&self, query: &str) -> MemoryStream;
    fn search_by_vector(&self, embedding: Vec<f32>, top_k: usize) -> MemoryStream;
    fn query_by_type(&self, memory_type: MemoryTypeEnum) -> MemoryStream;
    fn create_memory(&self, memory: MemoryNode) -> PendingMemory;
    fn get_memory(&self, id: &str) -> MemoryQuery;
}
```

**Implementation**: `SurrealDBMemoryManager`
- Native SurrealDB vector search
- Streaming architecture
- Production persistence

### MemoryCoordinator (High-Level API)
**Location**: [`./src/memory/core/manager/coordinator.rs`](../packages/candle/src/memory/core/manager/coordinator.rs)

**Features**:
- BERT embedding generation via `CandleBertEmbeddingProvider`
- Temporal decay: `e^(-decay_rate * days_old)` (default: 0.1)
- Cognitive task queuing
- Committee evaluation for quality
- Quantum routing for search optimization
- Evaluation caching (10K entries, 5min TTL)

**Methods**:
```rust
pub async fn search_memories(
    &self,
    query: &str,
    filter: Option<MemoryFilter>,
    top_k: usize,
) -> Result<Vec<MemoryNode>>

pub async fn add_memory(
    &self,
    content: String,
    memory_type: MemoryTypeEnum,
    metadata: MemoryMetadata,
) -> Result<MemoryNode>

pub async fn apply_temporal_decay(&self, memory: &mut MemoryNode) -> Result<()>
```

### Retrieval Strategies
**Location**: [`./src/memory/core/ops/retrieval.rs`](../packages/candle/src/memory/core/ops/retrieval.rs)

**SemanticRetrieval**:
```rust
pub struct SemanticRetrieval<V: VectorStore> {
    vector_store: Arc<V>,
}
// Vector similarity using embeddings
```

**TemporalRetrieval**:
```rust
pub struct TemporalRetrieval {
    time_decay_factor: f32,
    memory_manager: Arc<dyn MemoryManager>,
}
// score = e^(-decay_factor * age_hours)
// Combined: temporal (70%) + relevance (30%)
```

**HybridRetrieval**:
```rust
pub struct HybridRetrieval<V: VectorStore> {
    strategies: Arc<Vec<Arc<dyn RetrievalStrategy>>>,
    weights: Arc<HashMap<String, f32>>,
}
// Default weights: semantic (0.6), keyword (0.2), temporal (0.2)
```

## Data Flow Comparison

### OLD (Broken)
```
User Message
    ↓
AgentRoleImpl::inject_memory_context()
    ↓
// TODO: Implement actual memory querying
    ↓
Return empty string + 0.0 score  ❌
```

### NEW (Working)
```
User Message
    ↓
CandleAgentRoleImpl::inject_memory_context()
    ↓
MemoryManager::search_by_content(message)
    ↓
QuantumRouter (decides: vector, temporal, or hybrid)
    ↓
SurrealDB Vector Search + Temporal Decay
    ↓
MemoryStream<MemoryNode> (up to 10 results)
    ↓
Convert to RetrievalResult with importance scores
    ↓
PromptFormatter::format_memory_section()
    ↓
ContextInjectionResult {
    injected_context: String,     // ✅ Formatted for LLM
    relevance_score: f64,          // ✅ Average importance
    memory_nodes_used: usize       // ✅ Actual count
}
```

## Task Execution Steps

### Step 1: Delete Deprecated Files
```bash
cd /Volumes/samsung_t9/paraphym
rm packages/candle/src/agent/chat.rs
rm packages/candle/src/agent/role.rs
```

### Step 2: Clean Up Module Exports

Check `packages/candle/src/agent/mod.rs`:
```rust
// If it exports chat or role:
pub mod chat;  // ❌ REMOVE
pub mod role;  // ❌ REMOVE

// Only keep:
pub mod agent;
pub mod builder;
pub mod completion;
pub mod prompt;
```

### Step 3: Verify No References

```bash
# Should return NOTHING:
grep -r "use crate::agent::chat" packages/candle/src/
grep -r "use crate::agent::role" packages/candle/src/
grep -r "AgentRoleImpl" packages/candle/src/ | grep -v "domain/agent"
```

### Step 4: Verify Exports

```bash
# Ensure domain/agent is properly exported:
grep -r "pub use.*domain::agent" packages/candle/src/lib.rs
```

## Definition of Done

- [ ] Delete `packages/candle/src/agent/chat.rs`
- [ ] Delete `packages/candle/src/agent/role.rs`  
- [ ] Clean up `packages/candle/src/agent/mod.rs` (remove chat/role exports if present)
- [ ] Verify `grep -r "agent::chat" src/` returns NOTHING
- [ ] Verify `grep -r "agent::role" src/` returns NOTHING (except domain/agent/role)
- [ ] Confirm only `domain::agent::chat` and `domain::agent::role` exist
- [ ] Run `cargo check` to ensure no broken imports

## Files to Keep (Working Implementation)

- [`./src/domain/agent/chat.rs`](../packages/candle/src/domain/agent/chat.rs) - ✅ Full implementation
- [`./src/domain/agent/role.rs`](../packages/candle/src/domain/agent/role.rs) - ✅ Complete features
- [`./src/domain/agent/core.rs`](../packages/candle/src/domain/agent/core.rs) - ✅ Core types
- [`./src/domain/agent/types.rs`](../packages/candle/src/domain/agent/types.rs) - ✅ Type definitions
- [`./src/domain/agent/mod.rs`](../packages/candle/src/domain/agent/mod.rs) - ✅ Module exports
- [`./src/memory/core/manager/coordinator.rs`](../packages/candle/src/memory/core/manager/coordinator.rs) - ✅ Memory ops
- [`./src/memory/core/ops/retrieval.rs`](../packages/candle/src/memory/core/ops/retrieval.rs) - ✅ Retrieval

## Usage Example (NEW System)

```rust
use paraphym_candle::prelude::*;
use paraphym_candle::domain::agent::CandleAgentRoleImpl;
use paraphym_candle::memory::core::manager::surreal::SurrealDBMemoryManager;
use paraphym_candle::domain::memory::MemoryTool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create memory manager
    let db_manager = Arc::new(SurrealDBMemoryManager::new("surrealkv://./data").await?);
    let memory_tool = MemoryTool::new(db_manager.clone());
    
    // Create completion provider
    let provider = CandleKimiK2Provider::new().await?;
    
    // Build agent with NEW system
    let agent = CandleAgentRoleImpl::new("assistant")
        .with_completion_provider(CandleCompletionProviderType::KimiK2(provider))
        .with_memory_manager(db_manager.clone())
        .add_context_from_file("./docs/context.md")
        .with_code_execution(true);
    
    // Use the chat agent - memory querying works!
    let response_stream = agent.chat(
        "What did we discuss about embeddings?",
        &db_manager,
        &memory_tool
    );
    
    if let Some(response) = response_stream.try_next() {
        println!("Response: {}", response.response);
        println!("Context relevance: {}", response.context_injection.relevance_score);
        println!("Memories used: {}", response.context_injection.memory_nodes_used);
        println!("Context: {}", response.context_injection.injected_context);
    }
    
    Ok(())
}
```

## CONSTRAINTS

- NO backward compatibility - delete old code entirely
- NO test code (separate team)
- NO benchmark code (separate team)
- NO documentation files
- Focus: Delete deprecated files, verify NEW system is complete