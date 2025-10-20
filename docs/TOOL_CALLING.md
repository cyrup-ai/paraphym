# Tool Calling Architecture

## Overview

Cyrup implements a multi-stage tool calling system that uses constrained generation to ensure reliable JSON responses at each stage. The system follows OpenAI-compatible function calling patterns while leveraging SweetMCP's unified tool interface and our internal constraint system for guaranteed valid outputs.

**Key Architecture**: Just like the current Claude Code environment, the LLM sees one unified tool list containing both MCP protocol tools and native tools, with SweetMCP handling transparent routing to the appropriate execution method.

## Tool Discovery Process

Before any tool calling can occur, the system must discover and aggregate available tools from multiple sources. This process creates the unified tool interface that the LLM sees.

### Discovery Architecture

**Core Discovery Mechanism**: SweetMCP uses the JsonClient to discover tools from MCP servers via the Model Context Protocol.

```rust
// JsonClient implements McpClient trait for tool discovery
use sweetmcp_json_client::JsonClient;
use mcp_client_traits::McpClient;

// Connect to an MCP server
let client = JsonClient::new("https://localhost:8443")?;

// Discover all available tools from this server
let tools: Vec<ToolInfo> = client.list_tools().await?;
```

**ToolInfo Structure**: Each discovered tool provides standardized metadata:
```rust
pub struct ToolInfo {
    pub name: String,                    // Tool identifier ("hash", "time", "weather")
    pub description: Option<String>,     // Human-readable description  
    pub input_schema: JsonValue,         // JSON schema for parameters
}
```

### MCP Protocol Details

Tool discovery uses **JSON-RPC 2.0** over HTTP with standardized methods:

- **`tools/list`**: Discover available tools from server
- **`tools/call`**: Execute a specific tool (used later in Stage 3)
- **`initialize`**: Negotiate capabilities between client and server
- **`ping`**: Test server connectivity

**Discovery Request Example**:
```json
{
  "jsonrpc": "2.0",
  "id": "discover-123",
  "method": "tools/list",
  "params": {}
}
```

**Discovery Response Example**:
```json
{
  "jsonrpc": "2.0", 
  "id": "discover-123",
  "result": {
    "tools": [
      {
        "name": "hash",
        "description": "Generate cryptographic hashes and encode data",
        "inputSchema": {
          "type": "object",
          "properties": {
            "data": {"type": "string"},
            "algorithm": {"type": "string", "enum": ["sha256", "md5", "base64"]}
          },
          "required": ["data", "algorithm"]
        }
      },
      {
        "name": "time", 
        "description": "Get current time and parse time strings",
        "inputSchema": {
          "type": "object",
          "properties": {
            "name": {"type": "string", "enum": ["get_time_utc", "parse_time"]},
            "time_string": {"type": "string"}
          },
          "required": ["name"]
        }
      }
    ]
  }
}
```

### Discovery Integration

**Chat Loop Initialization Flow**:
1. **Server Connections**: Create JsonClient instances for each configured MCP server
2. **Capability Negotiation**: Call `initialize()` on each client to establish capabilities
3. **Tool Discovery**: Call `list_tools()` on each client to discover available tools
4. **Registry Aggregation**: Combine discovered tools with preregistered tools
5. **Unified Interface**: Present single tool list to LLM (Stage 1 tool selection)

```rust
pub async fn initialize_tool_registry(
    mcp_servers: Vec<String>,
    preregistered_tools: Vec<ToolInfo>,
) -> Result<Vec<ToolInfo>, DiscoveryError> {
    let mut all_tools = preregistered_tools;
    
    // Discover tools from each MCP server
    for server_url in mcp_servers {
        let client = JsonClient::new(&server_url)?;
        
        // Initialize connection and negotiate capabilities
        let client_info = Implementation {
            name: "Cyrup".to_string(),
            version: "1.0.0".to_string(),
        };
        client.initialize(JsonValue::from({}), client_info).await?;
        
        // Discover tools from this server
        let server_tools = client.list_tools().await?;
        all_tools.extend(server_tools);
    }
    
    Ok(all_tools)
}
```

### Discovery Patterns

**1. Startup Discovery** (Recommended)
- Tools discovered once during system initialization
- Cached for entire session duration
- Fast tool selection (no network calls during chat)
- Requires system restart to pick up new tools

**2. Runtime Discovery** (Advanced)
- Tools discovered dynamically during chat session
- Fresh tool lists for each conversation
- Slower tool selection due to network calls
- Automatically picks up newly deployed tools

**3. Hybrid Discovery** (Production)
- Core tools cached at startup for performance
- Optional runtime refresh for specific servers
- Configurable cache TTL (time-to-live)
- Background refresh with fallback to cache

### Multiple Server Aggregation

**Tool Sources**:
```rust
// Multiple MCP servers can provide different tool categories
let servers = vec![
    "https://weather-service:8443",    // Weather tools
    "https://math-service:8444",       // Calculation tools  
    "https://local-plugins:8445",      // Local SweetMCP plugins
];

let registry = ToolRegistry::new()
    .add_mcp_servers(servers).await?
    .add_preregistered_tools(native_tools)
    .build();

// Result: Unified tool list combining all sources
let unified_tools = registry.get_all_tools();
```

**Conflict Resolution**:
- Tool name conflicts resolved by server priority
- Higher priority servers override lower priority tools
- Preregistered tools typically have highest priority
- Duplicate tool names logged as warnings

### Error Handling and Fallbacks

**Discovery Failures**:
```rust
// Graceful degradation when servers are unavailable
match client.list_tools().await {
    Ok(tools) => registry.add_tools(tools),
    Err(e) => {
        warn!("Failed to discover tools from {}: {}", server_url, e);
        // Continue with other servers, use cached tools if available
        registry.add_cached_tools(&server_url)?;
    }
}
```

**Fallback Strategy**:
1. **Server Unavailable**: Skip server, continue with others
2. **Network Timeout**: Use cached tools from previous discovery
3. **Invalid Response**: Log error, exclude malformed tools
4. **No Tools Discovered**: Fall back to preregistered tools only

## Multi-Stage Chat Loop

The chat loop follows a 5-stage process: **Discovery** → **Selection** → **Function Calling** → **Execution** → **Interpretation**

### Stage 0: Tool Discovery (Initialization Phase)
**Purpose**: Discover and aggregate available tools from multiple sources before any user interaction.

**When**: Happens once during chat loop initialization, before user prompts are processed.

**Process**:
1. **MCP Server Discovery**: Connect to configured MCP servers via JsonClient
2. **Capability Negotiation**: Initialize connections with `initialize()` method
3. **Tool Enumeration**: Call `list_tools()` on each MCP server
4. **Registry Aggregation**: Combine MCP tools with preregistered tools
5. **Unified Interface**: Create single tool list for LLM consumption

**Input**: MCP server URLs + preregistered tool definitions  
**Output**: Unified `Vec<ToolInfo>` containing all available tools  
**No LLM involvement** - pure discovery and aggregation phase

```rust
// Stage 0: Tool Discovery (happens once during initialization)
pub async fn initialize_chat_loop() -> Result<ChatLoop, InitError> {
    // Discover tools from all configured sources
    let available_tools = initialize_tool_registry(
        vec!["https://weather-api:8443", "https://math-service:8444"],
        get_preregistered_tools(),
    ).await?;
    
    ChatLoop::new(available_tools, generator)
}
```

### Stage 1: Tool Selection
**Purpose**: Analyze user input and determine which tools (if any) would help accomplish the goal.

**Input**: User message + unified tool list from Stage 0 discovery
**Output**: Reasoning + selected tool names
**Constraint**: `ToolSelectionResponse` schema

**Request JSON**:
```json
{
  "model": "model_name",
  "messages": [
    {
      "role": "system",
      "content": "You are analyzing user requests to determine which tools would help. Respond with reasoning and selected tool names only."
    },
    {
      "role": "user",
      "content": "User input: [actual user message]\n\nAvailable tools: [serialized ToolInfo list from Stage 0 discovery]\n\nWhich tools would help with this request? Respond in JSON format: {\"reasoning\": \"...\", \"selected_tools\": [\"tool1\", \"tool2\"]}"
    }
  ],
  "response_format": { "type": "json_object" }
}
```

**Response Schema**:
```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ToolSelectionResponse {
    pub reasoning: String,
    pub selected_tools: Vec<String>, // tool names only
}
```

### Stage 2: Function Calling
**Purpose**: Generate OpenAI-compatible function calls for selected tools with proper parameter formatting.

**Input**: User message + selected tools with their schemas
**Output**: OpenAI function calling format with tool_calls
**Constraint**: `OpenAIFunctionCallResponse` schema

**Request JSON**:
```json
{
  "model": "model_name",
  "messages": [
    {
      "role": "user",
      "content": "Generate function calls for: [user input]"
    }
  ],
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "tool_name",
        "description": "Tool description",
        "parameters": {
          "type": "object",
          "properties": {
            "param1": {
              "type": "string",
              "description": "Parameter description"
            }
          },
          "required": ["param1"]
        }
      }
    }
  ],
  "tool_choice": "auto"
}
```

**Response Schema**:
```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct OpenAIFunctionCallResponse {
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ToolCall {
    pub id: String, // generated unique ID
    pub function: FunctionCall,
    #[serde(rename = "type")]
    pub call_type: String, // always "function"
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String, // JSON string of parameters
}
```

### Stage 3: Tool Execution
**Purpose**: Execute tool calls in parallel using Cylo containers.

**Process**:
1. Parse function calls from Stage 2
2. Execute each tool call in secure Cylo container
3. Collect results and errors
4. Aggregate results for Stage 4

**No LLM generation involved** - pure execution phase.

### Stage 4: Result Interpretation
**Purpose**: Interpret tool execution results and provide final response to user.

**Input**: Original user message + tool execution results
**Output**: Final response content
**Constraint**: `FinalResponse` schema

**Request JSON**:
```json
{
  "model": "model_name",
  "messages": [
    {
      "role": "user",
      "content": "[original user input]"
    },
    {
      "role": "assistant",
      "tool_calls": [
        {
          "id": "call_123",
          "type": "function",
          "function": {
            "name": "tool_name",
            "arguments": "{\"param\": \"value\"}"
          }
        }
      ]
    },
    {
      "role": "tool",
      "content": "[tool execution results]",
      "tool_call_id": "call_123"
    }
  ]
}
```

**Response Schema**:
```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FinalResponse {
    pub content: String,
}
```

## Constrained Generation Integration

### Automatic Constraint Application

Each stage automatically applies the appropriate constraint:

1. **Stage 1**: `constraint_for_type::<ToolSelectionResponse>()`
2. **Stage 2**: `constraint_for_type::<OpenAIFunctionCallResponse>()`
3. **Stage 4**: `constraint_for_type::<FinalResponse>()`

### Implementation Points

The chat loop orchestration layer knows which stage it's in and applies constraints accordingly:

```rust
// Stage 1: Tool Selection
let tool_selection = generator
    .with_constraint_for_type::<ToolSelectionResponse>()
    .generate(tool_selection_prompt, max_tokens)?;

// Stage 2: Function Calling
let function_calls = generator
    .with_constraint_for_type::<OpenAIFunctionCallResponse>()
    .generate(function_calling_prompt, max_tokens)?;

// Stage 4: Final Response
let final_response = generator
    .with_constraint_for_type::<FinalResponse>()
    .generate(interpretation_prompt, max_tokens)?;
```

## JSON Repair and Fallbacks

### Common LLM JSON Issues
- Single quotes instead of double quotes
- Unescaped special characters
- Missing closing braces
- Trailing commas

### Repair Logic
Post-constraint generation includes programmatic JSON repair for common mistakes:

```rust
pub fn repair_json(json_str: &str) -> Result<String, JsonRepairError> {
    // 1. Replace single quotes with double quotes
    // 2. Escape unescaped entities
    // 3. Remove trailing commas
    // 4. Fix missing braces
    // 5. Validate final JSON
}
```

## Error Handling

### Stage Failures
- **Tool Selection Fails**: Default to no tools, continue with direct response
- **Function Calling Fails**: Retry once, then fall back to direct response
- **Tool Execution Fails**: Include error in results, continue to interpretation
- **Result Interpretation Fails**: Return raw tool results with error message

### Validation
Each stage validates:
1. JSON structure correctness
2. Schema compliance via constraints
3. Required field presence
4. Tool name validity (Stage 1)
5. Parameter schema matching (Stage 2)

## Performance Considerations

### Caching
- **Tool Schemas**: Cache converted JSON schemas per tool
- **Constraints**: Pre-compile constraints for each stage
- **Template Rendering**: Cache compiled templates

### Parallelization
- **Tool Execution**: All tool calls in Stage 3 execute in parallel
- **Constraint Validation**: SIMD-optimized token validation
- **Schema Conversion**: Batch convert ToolInfo → JSON schemas

## Chat Loop Flow

### User Experience
User types: `"What's the weather in San Francisco and calculate 2+2"`
User gets: Complete response with weather data and calculation result

### Internal Chat Loop Implementation

```rust
// STAGE 0: Tool Discovery (called once during initialization)
pub async fn initialize_chat_session() -> Result<ChatSession, InitError> {
    let available_tools = initialize_tool_registry(
        vec!["https://weather-api:8443", "https://math-service:8444"], // MCP servers
        get_preregistered_tools(), // Native tools
    ).await?;
    
    Ok(ChatSession::new(available_tools))
}

// STAGES 1-4: Process individual user messages using discovered tools
pub async fn process_user_message(
    user_input: String,
    available_tools: Vec<sweet_mcp_type::ToolInfo>, // From Stage 0 discovery
    generator: &mut TextGenerator,
) -> Result<String, ChatError> {

    // STAGE 1: Tool Selection (happens automatically)
    let tool_selection_request = json!({
        "model": "model_name",
        "messages": [
            {
                "role": "system",
                "content": "You are analyzing user requests to determine which tools would help. Respond with reasoning and selected tool names only."
            },
            {
                "role": "user",
                "content": format!("User input: {}\n\nAvailable tools: {}\n\nWhich tools would help with this request? Respond in JSON format: {{\"reasoning\": \"...\", \"selected_tools\": [\"tool1\", \"tool2\"]}}", user_input, serialize_tools(&available_tools))
            }
        ],
        "response_format": { "type": "json_object" }
    });

    let tool_selection: ToolSelectionResponse = generator
        .with_constraint_for_type::<ToolSelectionResponse>()
        .generate_from_json(tool_selection_request)?;

    // If no tools selected, return direct response
    if tool_selection.selected_tools.is_empty() {
        return generate_direct_response(user_input, generator).await;
    }

    // STAGE 2: Function Calling (happens automatically)
    let selected_tool_schemas = get_schemas_for_tools(&tool_selection.selected_tools, &available_tools);
    let function_call_request = json!({
        "model": "model_name",
        "messages": [
            {
                "role": "user",
                "content": format!("Generate function calls for: {}", user_input)
            }
        ],
        "tools": selected_tool_schemas,
        "tool_choice": "auto"
    });

    let function_calls: OpenAIFunctionCallResponse = generator
        .with_constraint_for_type::<OpenAIFunctionCallResponse>()
        .generate_from_json(function_call_request)?;

    // STAGE 3: Tool Execution (happens automatically)
    let mut tool_results = Vec::new();
    if let Some(tool_calls) = function_calls.tool_calls {
        // Execute all tools in parallel via SweetMCP unified interface
        let execution_futures: Vec<_> = tool_calls.iter()
            .map(|call| async {
                // SweetMCP determines execution method automatically:
                // - If tool is from MCP server → route via JsonClient.call_tool()
                // - If tool is preregistered → execute via registered handler
                // - Security/isolation via Cylo when needed
                // - All return same ToolResult format
                sweetmcp_router.execute_tool(&call.function.name, &call.function.arguments).await
            })
            .collect();

        tool_results = futures::future::join_all(execution_futures).await;
    }

    // STAGE 4: Result Interpretation (happens automatically)
    let interpretation_request = json!({
        "model": "model_name",
        "messages": [
            {
                "role": "user",
                "content": user_input
            },
            {
                "role": "assistant",
                "tool_calls": function_calls.tool_calls.unwrap_or_default()
            },
            {
                "role": "tool",
                "content": serialize_tool_results(&tool_results),
                "tool_call_id": "aggregated"
            }
        ]
    });

    let final_response: FinalResponse = generator
        .with_constraint_for_type::<FinalResponse>()
        .generate_from_json(interpretation_request)?;

    Ok(final_response.content)
}
```

### What Actually Happens

0. **Discovery Phase (Initialization)**: System discovers tools from MCP servers:
   ```rust
   // Connect to weather and math MCP servers
   let weather_client = JsonClient::new("https://weather-api:8443")?;
   let math_client = JsonClient::new("https://math-service:8444")?;
   
   // Discover available tools from each server
   let weather_tools = weather_client.list_tools().await?; // ["get_weather", "get_forecast"]  
   let math_tools = math_client.list_tools().await?;       // ["calculate", "solve_equation"]
   
   // Aggregate into unified tool registry
   let available_tools = [weather_tools, math_tools].concat();
   ```

1. **User Input**: `"What's the weather in San Francisco and calculate 2+2"`

2. **Stage 1 (Internal)**: LLM analyzes and outputs:
   ```json
   {
     "reasoning": "User needs weather data and mathematical calculation",
     "selected_tools": ["get_weather", "calculate"]
   }
   ```

3. **Stage 2 (Internal)**: LLM generates function calls:
   ```json
   {
     "tool_calls": [
       {
         "id": "call_1",
         "type": "function",
         "function": {
           "name": "get_weather",
           "arguments": "{\"location\": \"San Francisco\"}"
         }
       },
       {
         "id": "call_2",
         "type": "function",
         "function": {
           "name": "calculate",
           "arguments": "{\"expression\": \"2+2\"}"
         }
       }
     ]
   }
   ```

4. **Stage 3 (Internal)**: Tools execute in parallel via SweetMCP unified interface:
   - `get_weather("San Francisco")` → **SweetMCP Router** → (routes to weather MCP server) → "Sunny, 72°F"
   - `calculate("2+2")` → **SweetMCP Router** → (routes to math plugin handler) → "4"
   
   Note: Both tools appear identical to the LLM despite different execution methods

5. **Stage 4 (Internal)**: LLM interprets results:
   ```json
   {
     "content": "The weather in San Francisco is sunny and 72°F. Also, 2+2 equals 4."
   }
   ```

6. **User Sees**: "The weather in San Francisco is sunny and 72°F. Also, 2+2 equals 4."

The entire multi-stage process is invisible to the user - they just get a complete, accurate response.

## Integration with Existing Systems

### TextGenerator Integration
The `TextGenerator` in `/packages/candle/src/core/generation/generator.rs` provides the constraint application methods used by the chat loop orchestration.

### Template System Integration
Uses the existing template system in `/packages/candle/src/domain/chat/templates/` for prompt generation and variable substitution.

### Tool Integration
Integrates with the unified tool interface in `/packages/candle/src/domain/tool/unified.rs` for tool discovery and execution.

### SweetMCP Unified Tool Interface

**Key Insight**: SweetMCP provides a unified interface that makes all tools appear identical to the LLM, regardless of how they're actually executed. This is exactly like the current Claude Code environment where you have both MCP tools (like `mcp__desktop-commander__read_file`) and native tools (like `Task`, `Read`) in the same tool list.

**Architecture**: SweetMCP acts as an abstraction layer that:
1. **Aggregates Tools**: Discovers tools from MCP servers and registers prebuilt tools
2. **Presents Unified Interface**: All tools appear as identical `ToolInfo` structures
3. **Routes Transparently**: Automatically routes execution to the correct backend
4. **Handles Protocols**: Manages MCP protocol communication when needed
5. **Provides Security**: Integrates with Cylo for secure execution when appropriate

#### Tool Discovery and Aggregation
Tools come from multiple sources but are presented as one unified list:

```rust
// All tools use the same interface regardless of execution method
pub struct ToolInfo {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: JsonValue, // simd_json::Value
}
```

**Tool Sources:**
1. **MCP Server Discovery**: Tools discovered from MCP servers via `JsonClient.list_tools()`
   - Local SweetMCP plugins (hash, fetch, time, browser, etc.)
   - Remote MCP servers (external services, APIs, etc.)

2. **Preregistered Tools**: Tools built into the system at compile/startup time
   - Can be executed via MCP protocol or directly
   - All appear as standard `ToolInfo` regardless of implementation

#### Transparent Execution Routing

**The LLM Never Knows The Difference** - all tools appear identical with the same `ToolInfo` structure. SweetMCP's routing system automatically determines the best execution method:

- **Runtime-discovered tools**: Routed to appropriate MCP server via `JsonClient.call_tool()`
- **Preregistered tools**: Executed via their registered handler (which may be MCP-based or direct)

This abstraction is identical to how you (Claude) currently work - you call tools by name without knowing their execution method. For example, your `mcp__desktop-commander__read_file` and `Task` tools both appear in your tool list but execute via different backends.

---

# Replace Confusing CandleTool Traits with SweetMCP Architecture

## Current Problem (Based on Code Analysis)
- **Multiple Conflicting Tool Traits**: Tool, CandleTool, McpTool, CandleMcpTool in different modules
- **Architectural Confusion**: Trait-based tool objects vs SweetMCP's protocol-based approach
- **Compilation Issues**: Missing HashMap import, missing CandleMcpClient type alias

## SweetMCP's Real Tool Architecture (From Code Analysis)
- **No Tool traits** - tools are protocol operations called by name
- **McpClient trait** - `call_tool("tool_name", JsonValue_args)` approach
- **Protocol-based discovery** - tools discovered via MCP protocol, not Rust traits
- **sweet-mcp-type::JsonValue** - High-performance JSON with simd acceleration

## Implementation Plan

### Phase 1: Remove Confusing Traits
1. **Delete Current Tool Hierarchies**:
   - Remove `/domain/tool/traits.rs` (CandleTool, CandleMcpTool)
   - Remove `/tool/traits.rs` (Tool, McpTool)
   - Remove duplicate trait definitions in `/domain/tool/core.rs`

2. **Clean Up Imports**:
   - Remove all imports of deleted traits throughout codebase
   - Update module exports in `/tool/mod.rs` and `/domain/tool/mod.rs`

### Phase 2: Implement SweetMCP Client Integration
1. **Fix Immediate Compilation Issues**:
   - Add missing HashMap import in `/builders/mcp_client.rs:36`
   - Add missing CandleMcpClient type alias

2. **Replace MCP Client Implementation**:
   - Use `sweetmcp_json_client::JsonClient` as the real MCP client
   - Replace current stdio-only MCP implementation with SweetMCP's multi-transport client
   - Preserve ystream::AsyncStream return patterns by wrapping SweetMCP responses

3. **Tool Execution Architecture**:
   ```rust
   // OLD: trait-based tool objects
   tool.execute(args) -> AsyncStream<Value>

   // NEW: protocol-based tool calls
   mcp_client.call_tool("tool_name", args) -> Future<Response>
   // Wrapped to preserve ystream patterns
   ```

### Phase 2.5: Code Execution Integration (Missing from Original Plan)
1. **Enable Builder Code Execution Configuration**:
   - The builder pattern already has code execution fields but they're ignored
   - Extract `code_execution_enabled`, `persistent_env_name`, `execution_timeout` in chat() method
   - Connect these fields to actual code execution via native tools

2. **Native Code Execution Tool**:
   ```rust
   // NEW: Native code execution using Cylo
   pub struct CodeExecutionTool {
       persistent_env_name: Option<String>,
       execution_timeout: Option<u64>,
   }

   impl CodeExecutionTool {
       pub async fn execute(&self, code: &str, language: &str) -> ToolContent {
           // Use cylo::execute_code_auto() for automatic backend selection
           // Returns ToolContent::Text(execution_result)
       }
   }
   ```

3. **Integration with Completion System**:
   - When `code_execution_enabled = true`, add CodeExecutionTool to completion request
   - Use existing `CompletionRequest.tools: ZeroOneOrMany<ToolDefinition>` field
   - No new APIs needed - leverages existing completion tool calling infrastructure

4. **Fluent API Behavior**:
   ```rust
   // User calls:
   .with_code_execution()
   .chat(|conversation| CandleChatLoop::UserPrompt("Calculate 2+2".to_string()))

   // System automatically:
   // 1. Sets code_execution_enabled = true
   // 2. Creates CodeExecutionTool using cylo
   // 3. Adds tool to CompletionRequest
   // 4. LLM can call execute_code tool automatically
   ```

### Phase 3: Native Tool Integration with Cylo
1. **Enhance SweetMCP Backend**:
   - Update tool execution to use cylo for secure execution when needed
   - SweetMCP router determines when secure execution is required
   - Cylo handles secure execution automatically (no configuration needed)

2. **Code Execution Priority**:
   - Code execution via builder configuration takes precedence
   - SweetMCP handles routing to appropriate execution method
   - Cylo's automatic backend selection provides security when needed

2. **Unified Tool System**:
   - All tools called via SweetMCP unified interface
   - SweetMCP handles routing to appropriate execution method transparently
   - All return ystream::AsyncStream for consistency

### Phase 4: Builder Integration
1. **Update Agent Builder**:
   - Replace tool trait registration with SweetMCP client configuration
   - Update fluent API to use protocol-based tool calls
   - Maintain backward compatibility for existing builder patterns

2. **MCP Client Builder**:
   - Use `sweetmcp_json_client::JsonClient` in CandleMcpClientBuilder
   - Support HTTP/HTTPS and stdio transports
   - Integrate with existing timeout and configuration options

### Phase 5: Performance Integration
1. **JSON Performance**:
   - Replace `serde_json::Value` with `sweet_mcp_type::JsonValue` (simd-accelerated)
   - Update all tool argument handling to use simd-json

2. **Preserve ystream Architecture**:
   - Wrap SweetMCP Future responses in ystream::AsyncStream
   - Maintain MessageChunk trait compatibility
   - Keep all existing streaming patterns intact

## Expected Results
- **Single Clear Architecture**: SweetMCP's protocol-based tool system only
- **Better Performance**: simd-json acceleration, HTTP transport options
- **Secure Native Tools**: Automatic cylo integration for all native tool execution
- **Automatic Code Execution**: `.with_code_execution()` enables secure code execution with zero configuration
- **Builder Pattern Completion**: Code execution configuration fields now actually work
- **Backward Compatibility**: Existing fluent API continues working
- **Zero Configuration**: Cylo and SweetMCP "just work" without user configuration

## Key Implementation Notes
- **No Stubs**: Full production-quality implementations only
- **Preserve ystream**: Cornerstone architecture maintained throughout
- **Protocol-First**: Tools are MCP protocol operations, not Rust trait objects
- **Performance-Focused**: Use SweetMCP's zero-allocation patterns