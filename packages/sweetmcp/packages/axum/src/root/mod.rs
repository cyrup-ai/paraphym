use std::sync::Arc;
use log::debug;
use rpc_router::HandlerResult;
use crate::types::{ListRootsRequest, ListRootsResult};

mod discovery;
pub mod watcher;

use discovery::RootDiscovery;

// Global state (initialize on server startup)
pub static ROOT_DISCOVERY: once_cell::sync::OnceCell<Arc<RootDiscovery>> = once_cell::sync::OnceCell::new();

/// Initialize root discovery (call on server startup)
pub fn init_root_discovery(config: crate::config::RootsConfig) {
    let discovery = Arc::new(RootDiscovery::new(config));
    ROOT_DISCOVERY.set(discovery).ok();
}

/// Handler for the roots/list MCP method
/// Lists all available workspace roots
pub async fn roots_list(_request: Option<ListRootsRequest>) -> HandlerResult<ListRootsResult> {
    debug!("Listing available roots");
    
    let discovery = ROOT_DISCOVERY.get()
        .expect("RootDiscovery not initialized");
    
    // Load roots (static + discovered)
    let roots = discovery.load_roots().await;
    
    Ok(ListRootsResult { roots })
}
