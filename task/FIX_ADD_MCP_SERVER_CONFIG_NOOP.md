# FIX: add_mcp_server_config() No-Op - MCP Servers Never Connect

**Status**: BROKEN - Zero implementation, architecture gap  
**Impact**: CRITICAL - External MCP tools completely non-functional  
**Scope**: Router extension + Builder integration

---

## Current Broken State

### Problem Code (role_builder_impl.rs:156)

```rust
fn add_mcp_server_config(self, _config: McpServerConfig) -> impl CandleAgentRoleBuilder {
    // MCP servers are handled through tools
    self  // ← Config thrown away, nothing happens
}
```

### Architecture Gap

**SweetMcpRouter has NO MCP server support**. Inspected [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) - only supports:

```rust
pub enum ToolRoute {
    SweetMcpPlugin { plugin_path: String },  // WASM plugins
    CyloExecution { backend_type: String, config: String },  // Container execution
    // NO MCP SERVER VARIANT
}
```

MCP servers are external processes providing tools via JSON-RPC stdio protocol - **completely different** from WASM plugins.

---

## What Exists: StdioClient (Ready to Use)

**Location**: [`packages/sweetmcp/packages/stdio-client/src/lib.rs`](../../packages/sweetmcp/packages/stdio-client/src/lib.rs)

**Full API** (already implemented):

```rust
impl StdioClient {
    // Spawn MCP server subprocess
    pub async fn new(command: &str, args: &[String], env: &[(&str, &str)]) -> Result<Self, StdioClientError>
    
    // Internal JSON-RPC communication
    async fn send_request(&self, method: &str, params: Value) -> Result<Value, StdioClientError>
    
    // Graceful shutdown
    pub async fn shutdown(self) -> Result<std::process::ExitStatus, StdioClientError>
}

impl McpClient for StdioClient {
    // MCP protocol handshake
    fn initialize(&self, capabilities: JsonValue, client_info: Implementation) -> Result<Response>
    
    // Discover available tools from server
    fn list_tools(&self) -> Result<Vec<ToolInfo>>
    
    // Execute a tool
    fn call_tool(&self, name: &str, arguments: JsonValue) -> Result<Response>
    
    // Health check
    fn ping(&self) -> Result<Response>
}
```

**Dependencies**: Already available as workspace members, no external crates needed.

---

## Complete Implementation Plan

### Phase 1: Builder Storage (3 files)

#### 1.1 Add Field to CandleAgentRoleBuilderImpl

**File**: [`src/builders/agent_role/role_builder.rs`](../packages/candle/src/builders/agent_role/role_builder.rs)

```rust
pub struct CandleAgentRoleBuilderImpl {
    // ... existing 19 fields ...
    pub(super) mcp_server_configs: Vec<McpServerConfig>,  // ADD THIS
}
```

**Initialize in new()**:

```rust
impl CandleAgentRoleBuilderImpl {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            // ... existing fields ...
            mcp_server_configs: Vec::new(),  // ADD THIS
        }
    }
}
```

#### 1.2 Store Config in Method

**File**: [`src/builders/agent_role/role_builder_impl.rs`](../packages/candle/src/builders/agent_role/role_builder_impl.rs) line ~156

```rust
fn add_mcp_server_config(mut self, config: McpServerConfig) -> impl CandleAgentRoleBuilder {
    self.mcp_server_configs.push(config);
    self
}
```

#### 1.3 Add Field to CandleAgentBuilderImpl

**File**: [`src/builders/agent_role/agent_builder.rs`](../packages/candle/src/builders/agent_role/agent_builder.rs)

```rust
pub struct CandleAgentBuilderImpl {
    // ... existing 19 fields ...
    pub(super) mcp_server_configs: Vec<McpServerConfig>,  // ADD THIS
}
```

#### 1.4 Propagate in into_agent()

**File**: [`src/builders/agent_role/role_builder_impl.rs`](../packages/candle/src/builders/agent_role/role_builder_impl.rs) line ~218

```rust
fn into_agent(self) -> impl CandleAgentBuilder {
    // ... existing model resolution ...
    
    CandleAgentBuilderImpl {
        // ... existing 19 field assignments ...
        mcp_server_configs: self.mcp_server_configs,  // ADD THIS
    }
}
```

---

### Phase 2: Router Extension (1 file)

#### 2.1 Add MCP ToolRoute Variant

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) line ~36

```rust
pub enum ToolRoute {
    SweetMcpPlugin { plugin_path: String },
    CyloExecution { backend_type: String, config: String },
    McpServer { server_id: String },  // ADD THIS
}
```

#### 2.2 Add MCP Client Storage

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) line ~24

```rust
pub struct SweetMcpRouter {
    available_tools: Arc<tokio::sync::RwLock<Vec<ToolInfo>>>,
    tool_routes: Arc<tokio::sync::RwLock<HashMap<String, ToolRoute>>>,
    plugin_configs: Vec<PluginConfig>,
    cylo_config: Option<CyloBackendConfig>,
    mcp_clients: Arc<tokio::sync::RwLock<HashMap<String, sweetmcp_stdio_client::StdioClient>>>,  // ADD THIS
}
```

#### 2.3 Add MCP Config Parameter

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) line ~80

```rust
pub fn with_configs(
    plugin_configs: Vec<PluginConfig>,
    cylo_config: Option<CyloBackendConfig>,
    mcp_configs: Vec<McpServerConfig>,  // ADD THIS
) -> Self {
    Self {
        available_tools: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        tool_routes: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        plugin_configs,
        cylo_config,
        mcp_clients: Arc::new(tokio::sync::RwLock::new(HashMap::new())),  // ADD THIS
    }
}
```

Update `new()` to pass empty Vec:

```rust
pub fn new() -> Self {
    Self::with_configs(Vec::new(), None, Vec::new())  // ADD third param
}
```

#### 2.4 Initialize MCP Clients

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) line ~100

Add new method after `discover_sweetmcp_plugins`:

```rust
/// Initialize MCP server clients and discover their tools
async fn initialize_mcp_servers(
    &self,
    tools: &mut Vec<ToolInfo>,
    routes: &mut HashMap<String, ToolRoute>,
    mcp_clients: &mut HashMap<String, sweetmcp_stdio_client::StdioClient>,
    configs: Vec<McpServerConfig>,
) {
    use mcp_client_traits::McpClient;
    use sweet_mcp_type::Implementation;
    
    for config in configs {
        if let Some(ref binary_path) = config.binary_path {
            let server_id = format!("mcp_{}", uuid::Uuid::new_v4());
            
            // Spawn MCP server subprocess
            match sweetmcp_stdio_client::StdioClient::new(
                binary_path,
                &[config.init_command.clone()],
                &[],
            ).await {
                Ok(client) => {
                    // MCP protocol handshake
                    let client_info = Implementation {
                        name: "candle-agent".to_string(),
                        version: "0.1.0".to_string(),
                    };
                    
                    match client.initialize(
                        sweet_mcp_type::JsonValue::Object(Default::default()),
                        client_info
                    ).await {
                        Ok(_) => {
                            // Discover tools from this MCP server
                            match client.list_tools().await {
                                Ok(mcp_tools) => {
                                    log::info!(
                                        "MCP server '{}' provided {} tools",
                                        binary_path,
                                        mcp_tools.len()
                                    );
                                    
                                    // Register each tool
                                    for tool in mcp_tools {
                                        log::info!("  - {}: {}", 
                                            tool.name, 
                                            tool.description.as_deref().unwrap_or("(no description)")
                                        );
                                        
                                        tools.push(tool.clone());
                                        routes.insert(
                                            tool.name.clone(),
                                            ToolRoute::McpServer {
                                                server_id: server_id.clone(),
                                            },
                                        );
                                    }
                                    
                                    // Store client for later tool calls
                                    mcp_clients.insert(server_id, client);
                                }
                                Err(e) => {
                                    log::error!(
                                        "Failed to list tools from MCP server '{}': {}",
                                        binary_path,
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            log::error!(
                                "Failed to initialize MCP server '{}': {}",
                                binary_path,
                                e
                            );
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to spawn MCP server '{}': {}",
                        binary_path,
                        e
                    );
                }
            }
        }
    }
}
```

#### 2.5 Call MCP Initialization

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) line ~108

Modify `initialize()` method:

```rust
pub async fn initialize(&mut self, mcp_configs: Vec<McpServerConfig>) -> Result<(), RouterError> {
    let mut tools = Vec::new();
    let mut routes = HashMap::new();
    let mut mcp_clients_map = HashMap::new();

    // Discover SweetMCP plugins
    self.discover_sweetmcp_plugins(&mut tools, &mut routes);

    // Initialize MCP servers
    self.initialize_mcp_servers(&mut tools, &mut routes, &mut mcp_clients_map, mcp_configs).await;

    // Add native execution tools
    self.add_native_execution_tools(&mut tools, &mut routes);

    // Store discovered tools and routes
    {
        let mut available_tools = self.available_tools.write().await;
        *available_tools = tools;
    }
    {
        let mut tool_routes = self.tool_routes.write().await;
        *tool_routes = routes;
    }
    {
        let mut mcp_clients = self.mcp_clients.write().await;
        *mcp_clients = mcp_clients_map;
    }

    Ok(())
}
```

#### 2.6 Route MCP Tool Calls

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) line ~135

Modify `call_tool()` match statement:

```rust
pub async fn call_tool(&self, tool_name: &str, args: JsonValue) -> Result<Value, RouterError> {
    let route = {
        let routes = self.tool_routes.read().await;
        routes
            .get(tool_name)
            .cloned()
            .ok_or_else(|| RouterError::ToolNotFound(tool_name.to_string()))?
    };

    match route {
        ToolRoute::SweetMcpPlugin { plugin_path } => {
            self.execute_sweetmcp_plugin(&plugin_path, args).await
        }
        ToolRoute::CyloExecution { backend_type, config } => {
            self.execute_cylo_backend(&backend_type, &config, args).await
        }
        ToolRoute::McpServer { server_id } => {  // ADD THIS CASE
            self.execute_mcp_tool(&server_id, tool_name, args).await
        }
    }
}
```

#### 2.7 Implement MCP Execution

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) - add after `execute_cylo_backend`

```rust
/// Execute tool via MCP server
async fn execute_mcp_tool(
    &self,
    server_id: &str,
    tool_name: &str,
    args: JsonValue,
) -> Result<Value, RouterError> {
    use mcp_client_traits::McpClient;
    
    let mcp_clients = self.mcp_clients.read().await;
    let client = mcp_clients
        .get(server_id)
        .ok_or_else(|| RouterError::ToolNotFound(format!("MCP server {} not found", server_id)))?;
    
    match client.call_tool(tool_name, args).await {
        Ok(response) => {
            if let Some(result) = response.result {
                // Convert JsonValue (simd_json) to serde_json::Value
                Ok(Self::convert_sweet_json_to_serde(result))
            } else if let Some(error) = response.error {
                Err(RouterError::ExecutionFailed(format!(
                    "MCP tool error: {:?}",
                    error
                )))
            } else {
                Err(RouterError::ExecutionFailed(
                    "MCP tool returned no result or error".to_string()
                ))
            }
        }
        Err(e) => Err(RouterError::ExecutionFailed(format!(
            "MCP tool call failed: {}",
            e
        ))),
    }
}
```

#### 2.8 Update clone_for_async

**File**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs) line ~425

```rust
fn clone_for_async(&self) -> Self {
    Self {
        available_tools: Arc::clone(&self.available_tools),
        tool_routes: Arc::clone(&self.tool_routes),
        plugin_configs: self.plugin_configs.clone(),
        cylo_config: self.cylo_config.clone(),
        mcp_clients: Arc::clone(&self.mcp_clients),  // ADD THIS
    }
}
```

---

### Phase 3: Chat Integration (1 file)

#### 3.1 Pass MCP Configs to Router

**File**: [`src/builders/agent_role/chat.rs`](../packages/candle/src/builders/agent_role/chat.rs) line ~405

**Current**:
```rust
let mut router = SweetMcpRouter::with_configs(plugin_configs, None);
match router.initialize().await {
```

**Replace with**:
```rust
let mut router = SweetMcpRouter::with_configs(plugin_configs, None, mcp_server_configs.clone());
match router.initialize(mcp_server_configs).await {
```

---

## Dependencies

**Add to `Cargo.toml`** if not present:

```toml
[dependencies]
sweetmcp-stdio-client = { path = "../../../sweetmcp/packages/stdio-client" }
uuid = { version = "1.0", features = ["v4"] }
```

Both are workspace members - no external dependencies needed.

---

## McpServerConfig Definition

**Already exists in** `role_builder_impl.rs`:

```rust
pub struct McpServerConfig {
    pub binary_path: Option<String>,
    pub init_command: String,
}
```

**Used by fluent API**:
```rust
.mcp_server::<Stdio>()
    .bin("/path/to/mcp/server")
    .init("--flag value")
```

Creates config and passes to `add_mcp_server_config()`.

---

## Execution Flow (Lazy Initialization)

**CRITICAL**: MCP servers do NOT spawn during builder configuration. Everything happens when `chat()` is invoked.

### Build Phase (No Process Spawning)

1. **`.mcp_server().bin().init()`** → Config stored in `Vec<McpServerConfig>` (NO subprocess)
2. **`.into_agent()`** → Config Vec copied to `CandleAgentBuilderImpl` (NO subprocess)
3. **Builder ready** → Zero MCP servers running yet

### Runtime Phase (Triggered by chat() call)

4. **`.chat()` invoked** → MCP initialization begins NOW
5. **Router creation** → `SweetMcpRouter::with_configs(plugin_configs, None, mcp_server_configs.clone())`
6. **`router.initialize(mcp_server_configs)`** called → THIS spawns MCP servers:
   - Calls `StdioClient::new()` for each config
   - Spawns subprocess with `Command::new(binary_path).spawn()`
   - Captures stdin/stdout for JSON-RPC
7. **MCP handshake** → `client.initialize()` - Protocol negotiation
8. **Tool discovery** → `client.list_tools()` - Tools registered in router
9. **Conversation loop** → LLM can now call MCP tools via router

**Why lazy?** MCP servers are conversation-scoped. Each `chat()` invocation gets fresh MCP instances.

---

## Definition of Done

### Compilation
- [ ] `cargo check --lib` exits 0
- [ ] All 7 builder files compile
- [ ] Router compiles with new MCP support

### Storage
- [ ] `mcp_server_configs` field in both builder structs
- [ ] `add_mcp_server_config()` stores config
- [ ] Configs propagated to `CandleAgentBuilderImpl`

### Router
- [ ] `ToolRoute::McpServer` variant added
- [ ] `mcp_clients` HashMap storage added
- [ ] `initialize_mcp_servers()` method implemented
- [ ] `execute_mcp_tool()` method implemented
- [ ] Router initialization calls MCP setup

### Integration
- [ ] Chat passes MCP configs to router
- [ ] MCP servers spawn successfully
- [ ] Tools discovered and registered
- [ ] Tool calls route to MCP servers
- [ ] Results flow back through router

### Functional
- [ ] MCP server subprocess spawns
- [ ] `initialize()` completes successfully
- [ ] `list_tools()` returns available tools
- [ ] Tool calls execute and return results
- [ ] Process cleanup on shutdown

---

## File Change Summary

| File | Changes | Lines Added |
|------|---------|-------------|
| `role_builder.rs` | Add field + initialize | +2 |
| `role_builder_impl.rs` | Store config + propagate | +2 |
| `agent_builder.rs` | Add field | +1 |
| `router.rs` | Full MCP support | ~120 |
| `chat.rs` | Pass configs to router | +1 |
| **TOTAL** | | **~126 lines** |

---

## Reference Implementations

**StdioClient**: [`packages/sweetmcp/packages/stdio-client/src/lib.rs`](../../packages/sweetmcp/packages/stdio-client/src/lib.rs)  
**McpClient trait**: [`packages/sweetmcp/packages/mcp-client-traits/src/lib.rs`](../../packages/sweetmcp/packages/mcp-client-traits/src/lib.rs)  
**Current Router**: [`src/domain/tool/router.rs`](../packages/candle/src/domain/tool/router.rs)  
**Implementation type**: Simple {name, version} struct in `sweet-mcp-type`
