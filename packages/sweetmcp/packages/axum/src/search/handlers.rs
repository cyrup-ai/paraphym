use rpc_router::{HandlerResult, IntoHandlerError, RpcParams};
use serde::{Deserialize, Serialize};
use crate::db::dao::entities::{ToolEntity, PromptEntity};

#[derive(Deserialize, RpcParams)]
pub struct SearchRequest {
    pub query: String,
    pub entity_type: String, // "tool", "prompt", or "all"
    pub limit: Option<usize>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub tools: Vec<ToolEntity>,
    pub prompts: Vec<PromptEntity>,
    pub total_count: usize,
}

pub async fn search_handler(request: SearchRequest) -> HandlerResult<SearchResponse> {
    let client = crate::db::client::get_db_client()
        .map_err(|e| {
            serde_json::json!({
                "code": -32603,
                "message": format!("Database unavailable: {}", e)
            }).into_handler_error()
        })?;
    
    let mut tools = Vec::new();
    let mut prompts = Vec::new();
    
    if request.entity_type == "tool" || request.entity_type == "all" {
        let service = crate::tool::persistence::ToolPersistenceService::new((*client).clone());
        tools = service.search_by_tag(&request.query).await;
    }
    
    if request.entity_type == "prompt" || request.entity_type == "all" {
        let service = crate::prompt::persistence::PromptPersistenceService::new((*client).clone());
        prompts = service.search_by_tag(&request.query).await;
    }
    
    let total_count = tools.len() + prompts.len();
    
    if let Some(limit) = request.limit {
        tools.truncate(limit);
        prompts.truncate(limit);
    }
    
    Ok(SearchResponse {
        tools,
        prompts,
        total_count,
    })
}

#[derive(Serialize)]
pub struct AnalyticsResponse {
    pub total_tools: usize,
    pub total_prompts: usize,
    pub total_tool_calls: i64,
    pub total_prompt_uses: i64,
    pub popular_tools: Vec<ToolEntity>,
}

pub async fn analytics_handler() -> HandlerResult<AnalyticsResponse> {
    let client = crate::db::client::get_db_client()
        .map_err(|e| {
            serde_json::json!({
                "code": -32603,
                "message": format!("Database unavailable: {}", e)
            }).into_handler_error()
        })?;
    
    let tool_service = crate::tool::persistence::ToolPersistenceService::new((*client).clone());
    let prompt_service = crate::prompt::persistence::PromptPersistenceService::new((*client).clone());
    
    let popular_tools = tool_service.get_popular_tools(10).await;
    let popular_prompts = prompt_service.get_popular_prompts(10).await;
    
    let total_tool_calls = popular_tools.iter().map(|t| t.call_count).sum();
    let total_prompt_uses = popular_prompts.iter().map(|p| p.use_count).sum();
    
    Ok(AnalyticsResponse {
        total_tools: popular_tools.len(),
        total_prompts: popular_prompts.len(),
        total_tool_calls,
        total_prompt_uses,
        popular_tools,
    })
}
