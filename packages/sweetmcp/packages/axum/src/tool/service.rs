use futures::StreamExt;
use rpc_router::{HandlerResult, IntoHandlerError};
use tokio::sync::{mpsc, oneshot};

use super::{super::types::*, model::*};
// Removed unused db imports
use crate::plugin::PluginManager; // Updated path

// Stream-based tools_list
pub fn tools_list_stream(
    pm: crate::plugin::PluginManager, // Updated path
    _request: Option<ListToolsRequest>,
) -> ToolStream {
    let (tx, rx) = mpsc::channel(16);

    tokio::spawn(async move {
        // Lock-free operations using DashMap
        pm.tool_to_plugin.clear();

        for mut plugin_entry in pm.plugins.iter_mut() {
            let plugin_name = plugin_entry.key().clone();
            let plugin = plugin_entry.value_mut();
            match plugin.call::<&str, &str>("describe", "") {
                Ok(result) => {
                    match serde_json::from_str::<ListToolsResult>(result) {
                        Ok(parsed) => {
                            for tool in parsed.tools {
                                pm.tool_to_plugin
                                    .insert(tool.name.clone(), plugin_name.clone());
                                if tx.send(Ok(tool)).await.is_err() {
                                    // Receiver likely dropped, stop sending
                                    log::warn!("Receiver dropped for tools_list_stream");
                                    break; // Exit the inner loop
                                }
                            }
                        }
                        Err(e) => log::error!(
                            "Failed to parse describe result for plugin {}: {}",
                            plugin_name,
                            e
                        ),
                    }
                    // Removed extra closing brace here
                }
                Err(e) => {
                    log::error!("tool {} describe() error: {}", plugin_name, e);
                } // Corrected closing brace for Err arm
            }
        }
    });

    ToolStream::new(rx)
}

/// Future-based tools_call (returns ToolCallExecution).
pub fn tools_call_pending(
    pm: crate::plugin::PluginManager, // Updated path
    request: ToolCallRequestParams,
) -> ToolCallExecution {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        // Lock-free access using DashMap

        let tool_name = request.name.as_str();
        log::info!("request: {:?}", request);

        let call_payload = serde_json::json!({
            "params": request.clone(),
        });
        let json_string = match serde_json::to_string(&call_payload) {
            // Already fixed
            Ok(s) => s,
            Err(e) => {
                let _ = tx.send(Err(serde_json::json!({"code": -32603, "message": format!("Failed to serialize request: {}", e)}).into_handler_error()));
                return;
            }
        };

        let result = if let Some(plugin_name_entry) = pm.tool_to_plugin.get(tool_name) {
            let plugin_name = plugin_name_entry.value();
            if let Some(mut plugin_entry) = pm.plugins.get_mut(plugin_name) {
                match plugin_entry.call::<&str, &str>("call", &json_string) {
                    Ok(result) => match serde_json::from_str::<CallToolResult>(result) {
                        Ok(parsed) => Ok(parsed),
                        Err(e) => {
                            log::error!("Failed to deserialize data: {} with {}", result, e);
                            Err(
                                serde_json::json!({"code": -32602, "message": "Failed to deserialized data"})
                                    .into_handler_error(),
                            )
                        }
                    },
                    Err(e) => {
                        log::error!(
                            "Failed to execute plugin {}: {}, request: {:?}",
                            plugin_name,
                            e,
                            request
                        );
                        Err(
                            serde_json::json!({"code": -32602, "message": format!("Failed to execute plugin {}: {}", plugin_name, e)})
                                .into_handler_error(),
                        )
                    }
                }
            } else {
                Err(
                    serde_json::json!({"code": -32602, "message": format!("Tool '{}' not found in any plugin", tool_name)})
                        .into_handler_error(),
                )
            }
        } else {
            Err(
                serde_json::json!({"code": -32602, "message": format!("Tool '{}' not found in any plugin", tool_name)})
                    .into_handler_error(),
            )
        };

        let _ = tx.send(result);
    });

    ToolCallExecution { rx }
}

/// Router-compatible async handler for tools/list
pub async fn tools_list_handler(
    pm: PluginManager,                 // Resource first
    request: Option<ListToolsRequest>, // Request second
) -> HandlerResult<ListToolsResult> {
    // Use ToolService instead of calling functions directly
    let service = ToolService::new(pm);
    let stream = service.list(request.unwrap_or(ListToolsRequest { cursor: None }));

    // Collect results from stream
    let mut tools = Vec::new();
    let mut stream = std::pin::Pin::new(Box::new(stream));

    // Use StreamExt::next for clarity
    while let Some(result) = StreamExt::next(&mut stream).await {
        match result {
            Ok(tool) => tools.push(tool),
            Err(e) => return Err(e),
        }
    }

    Ok(ListToolsResult {
        tools,
        next_cursor: None, // No pagination implemented yet
    })
}

/// Router-compatible async handler for tools/call
pub async fn tools_call_handler(
    pm: PluginManager,        // Resource first
    request: CallToolRequest, // Request second
) -> HandlerResult<CallToolResult> {
    let start_time = std::time::Instant::now();
    
    // Use ToolService instead of calling functions directly
    let service = ToolService::new(pm);
    let result = service.call(request.clone()).await;
    
    // Record statistics (non-blocking)
    let duration_ms = start_time.elapsed().as_secs_f64() * 1000.0;
    if let Ok(client) = crate::db::client::get_db_client() {
        let persistence = crate::tool::persistence::ToolPersistenceService::new((*client).clone());
        tokio::spawn(async move {
            if let Err(e) = persistence.record_tool_call(
                &request.params.name,
                duration_ms,
            ).await {
                log::warn!("Failed to record tool statistics: {}", e);
            }
        });
    }
    
    result
}

// Restore ToolService struct and impl
#[derive(Clone)]
pub struct ToolService {
    plugin_manager: PluginManager, // Uses PluginManager from crate::plugin
}

impl ToolService {
    pub fn new(plugin_manager: PluginManager) -> Self {
        // Updated path
        Self { plugin_manager }
    }

    // Changed to return ToolStream
    pub fn list(&self, req: ListToolsRequest) -> ToolStream {
        // Delegate to the stream-based function
        tools_list_stream(self.plugin_manager.clone(), Some(req))
    }

    // Changed to return ToolCallExecution
    pub fn call(&self, req: CallToolRequest) -> ToolCallExecution {
        // Delegate to the future-based function
        tools_call_pending(self.plugin_manager.clone(), req.params)
    }
}

// Removed stubs and leftovers
