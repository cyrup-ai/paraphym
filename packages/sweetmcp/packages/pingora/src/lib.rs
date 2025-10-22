pub mod api;
pub mod auth;
pub mod circuit_breaker;
pub mod config;
pub mod crypto;
pub mod dns_discovery;
pub mod mdns_discovery;
pub mod metrics;
pub mod normalize;
pub mod peer_discovery;
pub mod rate_limit;
pub mod shutdown;
pub mod tls;
pub mod edge;
pub mod load;
pub mod metric_picker;
pub mod mcp_bridge;

/// Get the directory where TLS certificates are stored
pub fn get_cert_dir() -> std::path::PathBuf {
    let xdg_config = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.config", home)
    });
    std::path::PathBuf::from(xdg_config)
        .join("sweetmcp")
        .join("certs")
}
