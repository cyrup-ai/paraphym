pub mod cms;

// Re-export public interface
pub use cms::{
    cms_dao::{find_by_slug, find_by_tags, init_cms_dao, resource_read},
    resources_list_handler,
};

// Define a wrapper function with the proper type for the router
pub async fn resources_list(
    request: Option<crate::types::ListResourcesRequest>,
) -> rpc_router::HandlerResult<crate::types::ListResourcesResult> {
    // Extract cursor and limit from request
    let _cursor = request.as_ref().and_then(|r| r.cursor.clone());
    let limit = request.as_ref().and_then(|r| r.limit).unwrap_or(100) as usize;
    
    // Fetch limit+1 resources to check if there are more
    let mut request_modified = request.clone().unwrap_or_default();
    request_modified.limit = Some((limit + 1) as u32);
    
    let mut resources = resources_list_handler(Some(request_modified)).await?;
    
    // Check if there are more results
    let next_cursor = if resources.len() > limit {
        // Remove the extra item and create cursor from last returned item
        resources.pop();
        resources.last().map(|r| create_cursor(r))
    } else {
        None
    };
    
    Ok(crate::types::ListResourcesResult {
        resources,
        next_cursor,
    })
}

/// Create a cursor from a resource
fn create_cursor(resource: &crate::types::Resource) -> String {
    // Use base64 encoding of URI for safe cursor transmission
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(resource.uri.as_str())
}

/// Parse a cursor to extract the last URI
pub fn parse_cursor(cursor: &str) -> Result<String, String> {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.decode(cursor)
        .map_err(|e| format!("Invalid cursor format: {}", e))
        .and_then(|bytes| String::from_utf8(bytes)
            .map_err(|e| format!("Invalid cursor encoding: {}", e)))
}
