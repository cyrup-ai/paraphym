use std::{fs, os::unix::fs::PermissionsExt, sync::Arc};

use anyhow::{Context, Result};
use log::{debug, error, info};
use rpc_router::{HandlerResult, Request, Router as RpcRouter, RouterBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, UnixListener, UnixStream},
};

// Only import what's actually used
use crate::resource::resource_read;
use crate::{
    JSONRPC_VERSION, PROTOCOL_VERSION, SERVER_NAME, SERVER_VERSION,
    config::Config,
    plugin::manager::PluginManager,
    prompt,
    resource::cms::resources_list_handler,
    sampling::sampling_create_message,
    tool,
    tool::notifications::{notifications_cancelled, notifications_initialized},
    types::*,
    ui::ServeArgs,
};

/// Build the JSON-RPC router with all registered handlers
fn build_rpc_router(plugin_manager: PluginManager) -> RpcRouter {
    // Use the provided PluginManager directly (lock-free implementation)

    // Register standard handlers first
    let builder = RouterBuilder::default()
        .append("initialize", initialize)
        .append("ping", ping)
        .append("logging/setLevel", logging_set_level)
        .append("roots/list", roots_list)
        // Resource handlers
        .append("resources/list", resources_list_handler)
        .append("resources/read", resource_read)
        // TODO: Add when handlers are implemented
        // .append("resources/subscribe", resource_subscribe_handler)
        // .append("resources/unsubscribe", resource_unsubscribe_handler)
        // Sampling handlers
        .append("sampling/createMessage", sampling_create_message)
        // Prompt handlers
        .append("prompts/list", prompt::prompts_list_handler)
        .append("prompts/get", prompt::prompts_get_handler)
        // Tool handlers
        .append("tools/list", tool::tools_list_handler)
        .append("tools/call", tool::tools_call_handler)
        // Search and Analytics handlers (DAO queries)
        .append("search", crate::search::handlers::search_handler)
        .append("analytics", crate::search::handlers::analytics_handler)
        // Context handlers
        .append("context/get", crate::context::rpc::context_get)
        .append("context/subscribe", crate::context::rpc::context_subscribe);

    // Add resource and register handlers that need access to it
    let builder = builder.append_resource(plugin_manager);

    // Build and return the router
    builder.build()
}

/// Structure for JSON-RPC Error responses
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    jsonrpc: String,
    error: rpc_router::Error, // Use qualified type
    id: Value,
}

/// Structure for JSON-RPC standard responses
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    result: Value,
    id: Value,
}

impl JsonRpcResponse {
    pub fn new(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            result,
            id,
        }
    }
}

/// Run the JSON-RPC server, handling stdin/stdout communication or daemon socket
pub async fn run_server(
    config: Config,
    plugin_manager: PluginManager,
    serve_args: ServeArgs,
) -> Result<()> {
    // Initialize DAO database client if configured
    if let Some(db_config) = &config.database {
        tracing::info!("Initializing database for DAO persistence layer");
        crate::db::client::init_db_client(db_config.clone())
            .await
            .context("Failed to initialize database client")?;
        
        // Create tables
        if let Ok(client) = crate::db::client::get_db_client() {
            create_dao_tables(client.clone()).await?;
            
            // Initialize ResourceDao with the database client
            let resource_dao_config = crate::resource::cms::resource_dao::ResourceDaoConfig::default();
            crate::resource::cms::resource_dao::init_resource_dao(
                resource_dao_config,
                (*client).clone()
            ).map_err(|e| anyhow::anyhow!("Failed to initialize ResourceDao: {}", e))?;
            
            tracing::info!("ResourceDao initialized successfully");
        }
    }
    
    // Memory system initialization
    // The production memory system is now initialized within MemoryContextAdapter::new()
    // using paraphym_candle's SurrealDB-backed memory with vector embeddings
    if let Some(_db_config) = &config.database {
        log::info!("Memory system will be initialized by context adapter");
    }

    if serve_args.daemon {
        run_daemon(plugin_manager, serve_args).await
    } else {
        run_stdio_server(plugin_manager).await
    }
}

/// Create all DAO tables
async fn create_dao_tables(client: Arc<crate::db::client::DatabaseClient>) -> Result<()> {
    use crate::db::dao::{Dao, entities::{PluginEntity, ToolEntity, PromptEntity, AuditLog}};
    
    // Create tables using DAO
    let plugin_dao = Dao::<PluginEntity>::new((*client).clone());
    plugin_dao.create_table().await;
    
    let tool_dao = Dao::<ToolEntity>::new((*client).clone());
    tool_dao.create_table().await;
    
    let prompt_dao = Dao::<PromptEntity>::new((*client).clone());
    prompt_dao.create_table().await;
    
    let audit_dao = Dao::<AuditLog>::new((*client).clone());
    audit_dao.create_table().await;
    
    info!("DAO tables created successfully");
    Ok(())
}

/// Run the server using stdin/stdout
async fn run_stdio_server(plugin_manager: PluginManager) -> Result<()> {
    info!("Starting MCP JSON-RPC server (stdin/stdout mode)");

    // Build RPC router with lock-free plugin manager
    let rpc_router = build_rpc_router(plugin_manager);

    // Process stdin lines asynchronously as JSON-RPC requests
    let stdin = tokio::io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    info!("Ready to process JSON-RPC messages");

    while let Some(line) = lines.next_line().await? {
        debug!("Received: {}", line);

        if !line.is_empty() {
            // Parse input as JSON value
            if let Ok(json_value) = serde_json::from_str::<Value>(&line) {
                // Handle notifications (no response required)
                if json_value.is_object() && json_value.get("id").is_none() {
                    if let Some(method) = json_value.get("method") {
                        if method == "notifications/initialized" {
                            notifications_initialized();
                        } else if method == "notifications/cancelled"
                            && let Some(params_value) = json_value.get("params")
                            && let Ok(cancel_params) =
                                serde_json::from_value(params_value.clone())
                        {
                            notifications_cancelled(cancel_params);
                        }
                    }
                } else {
                    // Process regular requests
                    if let Ok(mut rpc_request) = Request::from_value(json_value) {
                        // Ensure params exist for ping method
                        if rpc_request.method == "ping" && rpc_request.params.is_none() {
                            rpc_request.params = Some(json!({}));
                        }

                        let id = rpc_request.id.clone();

                        match rpc_router.call(rpc_request).await {
                            Ok(call_response) => {
                                if !call_response.value.is_null() {
                                    let response = JsonRpcResponse::new(id, call_response.value);
                                    if let Ok(response_json) = serde_json::to_string(&response) {
                                        debug!("Response: {}", response_json);
                                        eprintln!("{}", response_json);
                                    }
                                }
                            }
                            Err(error) => match &error.error {
                                rpc_router::Error::Handler(handler) => {
                                    if let Some(error_value) = handler.get::<Value>() {
                                        let json_error = json!({
                                            "jsonrpc": JSONRPC_VERSION,
                                            "error": error_value,
                                            "id": id
                                        });
                                        if let Ok(response) = serde_json::to_string(&json_error) {
                                            error!("Error: {}", response);
                                            eprintln!("{}", response);
                                        }
                                    }
                                }
                                _ => {
                                    error!("Unexpected error: {:?}", error);
                                    let json_error = json!({
                                        "jsonrpc": JSONRPC_VERSION,
                                        "error": {
                                            "code": -1,
                                            "message": "Invalid JSON-RPC call"
                                        },
                                        "id": id
                                    });
                                    if let Ok(response) = serde_json::to_string(&json_error) {
                                        eprintln!("{}", response);
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }

    info!("JSON-RPC server shutdown");
    Ok(())
}

/// Run the server using HTTP binding
pub async fn run_http_server(plugin_manager: PluginManager, bind_addr: &str) -> Result<()> {
    info!("Starting MCP JSON-RPC server (HTTP mode on {})", bind_addr);

    // Build RPC router with lock-free plugin manager
    let rpc_router = Arc::new(build_rpc_router(plugin_manager));

    // Bind TCP listener
    let listener = TcpListener::bind(bind_addr)
        .await
        .context("Failed to bind HTTP server")?;

    info!("HTTP JSON-RPC server listening on {}", bind_addr);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                debug!("New HTTP connection from {}", addr);
                let router = rpc_router.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_http_connection(stream, router).await {
                        error!("Failed to handle HTTP connection: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept HTTP connection: {}", e);
            }
        }
    }
}

/// Handle a single HTTP connection
async fn handle_http_connection(
    mut stream: tokio::net::TcpStream,
    rpc_router: Arc<RpcRouter>,
) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut buffer = vec![0; 4096];
    let n = stream.read(&mut buffer).await?;
    let request_data = String::from_utf8_lossy(&buffer[..n]);

    // Simple HTTP parsing to extract JSON body
    if let Some(body_start) = request_data.find("\r\n\r\n") {
        let body = &request_data[body_start + 4..];

        if !body.trim().is_empty()
            && let Ok(json_value) = serde_json::from_str::<Value>(body)
            && let Ok(mut rpc_request) = Request::from_value(json_value) {
                    // Ensure params exist for ping method
                    if rpc_request.method == "ping" && rpc_request.params.is_none() {
                        rpc_request.params = Some(json!({}));
                    }

                    let id = rpc_request.id.clone();

                    let (status_code, response_body) = match rpc_router.call(rpc_request).await {
                        Ok(call_response) => {
                            let response = JsonRpcResponse::new(id, call_response.value);
                            let response_json = serde_json::to_string(&response)?;
                            ("200 OK", response_json)
                        }
                        Err(error) => {
                            error!("RPC call failed: {:?}", error);
                            let json_error = json!({
                                "jsonrpc": JSONRPC_VERSION,
                                "error": {
                                    "code": -32603,
                                    "message": "Internal server error"
                                },
                                "id": id
                            });
                            let response_json = serde_json::to_string(&json_error)?;
                            ("502 Bad Gateway", response_json)
                        }
                    };

                    let response = format!(
                        "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        status_code,
                        response_body.len(),
                        response_body
                    );

                    stream.write_all(response.as_bytes()).await?;
                    stream.flush().await?;
                    return Ok(());
        }
    }

    // Send 400 for invalid requests
    let error_response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
    stream.write_all(error_response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

/// Handler for the initialize method
pub async fn initialize(request: InitializeRequest) -> HandlerResult<InitializeResponse> {
    info!(
        "Initializing with protocol version: {}",
        request.protocol_version
    );

    let result = InitializeResponse {
        protocol_version: PROTOCOL_VERSION.to_string(),
        server_info: Implementation {
            name: SERVER_NAME.to_string(),
            version: SERVER_VERSION.to_string(),
        },
        capabilities: ServerCapabilities {
            experimental: None,
            prompts: Some(PromptCapabilities::default()),
            resources: Some(ResourceCapabilities::default()),
            tools: Some(json!({})),
            roots: Some(json!({})),
            sampling: Some(json!({})),
            logging: Some(json!({})),
        },
        instructions: None,
    };

    Ok(result)
}

/// Handler for the ping method
pub async fn ping(_request: PingRequest) -> HandlerResult<EmptyResult> {
    debug!("Received ping request");
    Ok(EmptyResult {})
}

/// Handler for logging/setLevel method
pub async fn logging_set_level(request: SetLevelRequest) -> HandlerResult<LoggingResponse> {
    info!("Setting log level to: {}", request.level);
    // Implementation for changing log level would go here
    Ok(LoggingResponse {})
}

/// Handler for roots/list method
pub async fn roots_list(_request: Option<ListRootsRequest>) -> HandlerResult<ListRootsResult> {
    debug!("Listing available roots");
    let response = ListRootsResult {
        roots: vec![Root {
            name: "workspace".to_string(),
            url: "file:///workspace".to_string(),
        }],
    };
    Ok(response)
}

/// Run the server as a system daemon using our sophisticated daemon manager
async fn run_daemon(_plugin_manager: PluginManager, _serve_args: ServeArgs) -> Result<()> {
    Err(anyhow::anyhow!(
        "Daemon mode is not supported in axum - use the daemon to run pingora"
    ))
}

/// Create and run Unix domain socket listener
pub async fn create_socket_listener(
    plugin_manager: PluginManager,
    socket_path: &std::path::Path,
) -> Result<()> {
    // Remove existing socket file if it exists
    if socket_path.exists() {
        fs::remove_file(socket_path).context("Failed to remove existing socket file")?;
    }

    // Ensure parent directory exists
    if let Some(parent) = socket_path.parent() {
        fs::create_dir_all(parent).context("Failed to create socket directory")?;
    }

    // Create Unix domain socket listener
    let listener = UnixListener::bind(socket_path).context("Failed to bind Unix domain socket")?;

    // Set socket permissions to 0666 (rw-rw-rw-)
    let metadata = fs::metadata(socket_path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o666);
    fs::set_permissions(socket_path, permissions)?;

    info!("MCP daemon listening on socket: {}", socket_path.display());

    // Accept connections
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                // Clone plugin manager for this connection
                let pm = plugin_manager.clone();

                // Spawn task to handle this connection
                tokio::spawn(async move {
                    if let Err(e) = handle_socket_connection(stream, pm).await {
                        error!("Failed to handle socket connection: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept socket connection: {}", e);
            }
        }
    }
}

/// Handle a single socket connection
async fn handle_socket_connection(stream: UnixStream, plugin_manager: PluginManager) -> Result<()> {
    info!("New socket connection established");

    let (reader, mut writer) = stream.into_split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    // Build RPC router with lock-free plugin manager
    let rpc_router = build_rpc_router(plugin_manager);

    while let Some(line) = lines.next_line().await? {
        debug!("Socket received: {}", line);

        if !line.is_empty() {
            // Parse input as JSON value
            if let Ok(json_value) = serde_json::from_str::<Value>(&line) {
                // Handle notifications (no response required)
                if json_value.is_object() && json_value.get("id").is_none() {
                    if let Some(method) = json_value.get("method") {
                        if method == "notifications/initialized" {
                            notifications_initialized();
                        } else if method == "notifications/cancelled"
                            && let Some(params_value) = json_value.get("params")
                            && let Ok(cancel_params) =
                                serde_json::from_value(params_value.clone())
                        {
                            notifications_cancelled(cancel_params);
                        }
                    }
                } else {
                    // Process regular requests
                    if let Ok(mut rpc_request) = Request::from_value(json_value) {
                        // Ensure params exist for ping method
                        if rpc_request.method == "ping" && rpc_request.params.is_none() {
                            rpc_request.params = Some(json!({}));
                        }

                        let id = rpc_request.id.clone();

                        match rpc_router.call(rpc_request).await {
                            Ok(call_response) => {
                                if !call_response.value.is_null() {
                                    let response = JsonRpcResponse::new(id, call_response.value);
                                    if let Ok(response_json) = serde_json::to_string(&response) {
                                        debug!("Socket response: {}", response_json);
                                        writer.write_all(response_json.as_bytes()).await?;
                                        writer.write_all(b"\n").await?;
                                        writer.flush().await?;
                                    }
                                }
                            }
                            Err(error) => {
                                let json_error = match &error.error {
                                    rpc_router::Error::Handler(handler) => {
                                        if let Some(error_value) = handler.get::<Value>() {
                                            json!({
                                                "jsonrpc": JSONRPC_VERSION,
                                                "error": error_value,
                                                "id": id
                                            })
                                        } else {
                                            json!({
                                                "jsonrpc": JSONRPC_VERSION,
                                                "error": {
                                                    "code": -1,
                                                    "message": "Handler error"
                                                },
                                                "id": id
                                            })
                                        }
                                    }
                                    _ => {
                                        json!({
                                            "jsonrpc": JSONRPC_VERSION,
                                            "error": {
                                                "code": -1,
                                                "message": "Invalid JSON-RPC call"
                                            },
                                            "id": id
                                        })
                                    }
                                };

                                if let Ok(response) = serde_json::to_string(&json_error) {
                                    error!("Socket error: {}", response);
                                    writer.write_all(response.as_bytes()).await?;
                                    writer.write_all(b"\n").await?;
                                    writer.flush().await?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    info!("Socket connection closed");
    Ok(())
}
