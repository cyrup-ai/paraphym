//! SweetMCP Server - Sugora Gateway
//!
//! A production-grade, multi-protocol edge proxy built on Pingora 0.5 that normalizes
//! GraphQL, JSON-RPC 2.0, and Cap'n Proto into Model Context Protocol (MCP) requests.

mod api;
mod auth;
mod circuit_breaker;
mod config;
mod crypto;
mod dns_discovery;
mod edge;
mod load;
mod mcp_bridge;
mod mdns_discovery;
mod metric_picker;
mod metrics;
mod normalize;
mod peer_discovery;
mod rate_limit;
mod shutdown;
mod tls;

use std::sync::Arc;

use anyhow::{Context, Result};
use config::Config;
use opentelemetry::global;
use opentelemetry_prometheus::PrometheusExporter;
use pingora::prelude::*;
use pingora_load_balancing::discovery::ServiceDiscovery;
use tokio::sync::mpsc;

fn get_cert_dir() -> std::path::PathBuf {
    let xdg_config = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{}/.config", home)
    });
    std::path::PathBuf::from(xdg_config)
        .join("sweetmcp")
        .join("certs")
}

fn main() {
    env_logger::init();

    if let Err(e) = run_server() {
        eprintln!("🚫 SweetMCP Server failed to start: {}", e);
        std::process::exit(1);
    }
}

fn run_server() -> Result<()> {
    log::info!("🍬 Starting SweetMCP Server with Sugora Gateway");

    // Load configuration
    let cfg = Arc::new(Config::from_env()?);
    log::info!("✅ Configuration loaded successfully");

    // Initialize OpenTelemetry
    let _exporter = init_otel()?;
    log::info!("📊 OpenTelemetry initialized");

    // Setup MCP bridge
    let (bridge_tx, bridge_rx) = mpsc::channel::<mcp_bridge::BridgeMsg>(1024);

    // Create server with default options
    let mut server =
        Server::new(None).map_err(|e| anyhow::anyhow!("Failed to create Pingora server: {}", e))?;
    server.bootstrap();

    // Initialize circuit breaker manager
    let circuit_config = circuit_breaker::CircuitBreakerConfig {
        error_threshold_percentage: cfg.circuit_breaker_threshold,
        request_volume_threshold: 20,
        sleep_window: std::time::Duration::from_secs(5),
        half_open_requests: 3,
        metrics_window: std::time::Duration::from_secs(10),
    };
    let circuit_breaker_manager = Arc::new(circuit_breaker::CircuitBreakerManager::new(circuit_config));

    // Create peer registry with circuit breaker manager
    let peer_registry = peer_discovery::PeerRegistry::new(circuit_breaker_manager.clone());

    // Extract port from TCP bind address
    let local_port = cfg
        .tcp_bind
        .split(':')
        .next_back()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8443);

    // Create and configure shutdown coordinator
    let mut shutdown_coordinator = shutdown::ShutdownCoordinator::new(
        std::env::temp_dir().join("sweetmcp")
    );
    shutdown_coordinator.set_local_port(local_port);
    shutdown_coordinator.set_peer_registry(peer_registry.clone());

    let shutdown_coordinator = Arc::new(shutdown_coordinator);

    // Spawn signal listener for graceful shutdown
    let coord_clone = shutdown_coordinator.clone();
    tokio::spawn(async move {
        coord_clone.listen_for_shutdown().await;
    });

    // Create background services
    let mcp_bridge = background_service(
        "mcp-bridge",
        McpBridgeService {
            rx: Some(bridge_rx),
        },
    );

    // Create discovery services based on configuration
    if let Some(service_name) = dns_discovery::should_use_dns_discovery() {
        let dns_discovery = dns_discovery::DnsDiscovery::new(
            service_name.clone(),
            peer_registry.clone(),
        );
        let dns_service = background_service(
            "dns-discovery",
            DnsDiscoveryService {
                service_name,
                discovery: dns_discovery,
            },
        );
        server.add_service(dns_service);
    } else {
        // Fallback: mDNS for local network discovery
        let mdns_discovery = mdns_discovery::MdnsDiscovery::new(peer_registry.clone(), local_port);
        let mdns_service = background_service(
            "mdns-discovery",
            MdnsDiscoveryService {
                discovery: mdns_discovery,
            },
        );
        server.add_service(mdns_service);
    }

    // Always start HTTP-based peer exchange for mesh formation
    let discovery_service = peer_discovery::DiscoveryService::new(
        peer_registry.clone(),
        cfg.auth.discovery_token.clone(),
    )?;
    let peer_service = background_service(
        "peer-discovery",
        PeerDiscoveryService {
            service: discovery_service,
        },
    );

    // Add background services
    server.add_service(mcp_bridge);
    server.add_service(peer_service);

    // Load TLS certificates from XDG_CONFIG_HOME/sweetmcp/certs
    let cert_dir = get_cert_dir();

    // Add TLS certificate monitoring and rotation service
    let tls_dir = cert_dir.to_string_lossy().to_string();
    let authority = tls::CertificateAuthority::load(&cert_dir.join("ca.crt"))?;
    let server_domain = "localhost".to_string(); // Default server domain

    let tls_service = background_service(
        "tls-certificate-manager",
        TlsCertificateManagerService {
            tls_dir,
            authority,
            server_domain,
        },
    );
    server.add_service(tls_service);

    // Create HTTP proxy service
    let edge_service = edge::EdgeServiceBuilder::new()
        .with_config(cfg.clone())
        .with_bridge_channel(bridge_tx.clone())
        .with_peer_registry(peer_registry.clone())
        .with_custom_shutdown_coordinator(shutdown_coordinator)
        .build()?;

    // Create backend update service
    let static_upstreams: Vec<pingora_load_balancing::Backend> = edge_service.picker().load()
        .backends
        .to_vec();

    let backend_update = background_service(
        "backend-update",
        BackendUpdateService {
            picker_swap: edge_service.picker().clone(),
            peer_discovery: edge_service.peer_discovery.clone(),
            static_upstreams,
        },
    );
    server.add_service(backend_update);

    // Add rate limit cleanup service
    if let Some(distributed_limiter) = edge_service.rate_limit_manager().as_distributed() {
        let rate_limit_service = background_service(
            "rate-limit-cleanup",
            RateLimitCleanupService {
                rate_limiter: distributed_limiter,
            },
        );
        server.add_service(rate_limit_service);
    }

    // Add metrics collector service
    let metrics_service = background_service(
        "metrics-collector",
        MetricsCollectorService {
            metric_picker: edge_service.picker().clone(),
            circuit_breaker_manager: edge_service.circuit_breaker_manager.clone(),
        },
    );
    server.add_service(metrics_service);

    let mut proxy_service = pingora_proxy::http_proxy_service(&server.configuration, edge_service);

    // Verify certificate files exist (generated during installation)
    let ca_cert_path = cert_dir.join("ca.crt");
    let server_cert_path = cert_dir.join("server.crt");
    let server_key_path = cert_dir.join("server.key");

    for (file_type, path) in [
        ("CA certificate", &ca_cert_path),
        ("Server certificate", &server_cert_path),
        ("Server key", &server_key_path),
    ] {
        if !path.exists() {
            return Err(anyhow::anyhow!(
                "{} not found: {}. Please run the installer to generate certificates.",
                file_type,
                path.display()
            ));
        }
    }

    log::info!("✅ Loading TLS certificates from {}", cert_dir.display());

    // Configure TLS settings for Pingora using existing certificates
    use pingora::listeners::tls::TlsSettings;

    let server_cert_str = server_cert_path
        .to_str()
        .context("Server certificate path contains invalid UTF-8")?;
    let server_key_str = server_key_path
        .to_str()
        .context("Server key path contains invalid UTF-8")?;

    let tls_settings = TlsSettings::intermediate(server_cert_str, server_key_str)
        .context("Failed to create TLS settings")?
        .with_client_ca(cert_dir.join("ca.crt").to_string_lossy().to_string());

    // Add HTTPS listeners
    proxy_service.add_tls_with_settings(&cfg.tcp_bind, None, tls_settings);

    // Add separate TLS listener for MCP if different from main TCP
    if cfg.mcp_bind != cfg.tcp_bind {
        let tls_settings_mcp = TlsSettings::intermediate(server_cert_str, server_key_str)
            .context("Failed to create TLS settings for MCP")?
            .with_client_ca(cert_dir.join("ca.crt").to_string_lossy().to_string());
        proxy_service.add_tls_with_settings(&cfg.mcp_bind, None, tls_settings_mcp);
    }

    log::info!("🔒 TLS enabled on {} and {}", cfg.tcp_bind, cfg.mcp_bind);

    // Add Unix socket listener
    // Ensure directory exists
    if let Some(parent) = std::path::Path::new(&cfg.uds_path).parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            log::warn!("Failed to create UDS directory {:?}: {}", parent, e);
        } else {
            log::info!("Created UDS directory {:?}", parent);
        }
    }

    // Remove old socket file if it exists
    if std::path::Path::new(&cfg.uds_path).exists()
        && let Err(e) = std::fs::remove_file(&cfg.uds_path) {
            log::warn!("Failed to remove old socket file: {}", e);
        }

    proxy_service.add_uds(&cfg.uds_path, None);

    // Add the proxy service to server
    server.add_service(proxy_service);

    // Setup Prometheus metrics service
    let mut prometheus_service = pingora::services::listening::Service::prometheus_http_service();
    prometheus_service.add_tcp(&cfg.metrics_bind);
    server.add_service(prometheus_service);

    // The exporter automatically registers with the default prometheus registry

    log::info!("🚀 Sugora Gateway ready!");
    log::info!("  TCP: {}", cfg.tcp_bind);
    log::info!("  MCP HTTP: {}", cfg.mcp_bind);
    log::info!("  UDS: {}", cfg.uds_path);
    log::info!("  Metrics: http://{}/metrics", cfg.metrics_bind);

    // Run the server - this never returns
    server.run_forever();
}

fn init_otel() -> Result<PrometheusExporter> {
    let exporter = opentelemetry_prometheus::exporter().build()?;

    // Set up trace propagation
    global::set_text_map_propagator(opentelemetry_sdk::propagation::TraceContextPropagator::new());

    Ok(exporter)
}

// Background service implementations
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use pingora::server::ShutdownWatch;
use pingora::services::background::{background_service, BackgroundService};

struct McpBridgeService {
    rx: Option<mpsc::Receiver<mcp_bridge::BridgeMsg>>,
}

impl BackgroundService for McpBridgeService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        // Validate before unsafe operation - prevents panic in unsafe context
        if self.rx.is_none() {
            return Box::pin(async move {
                log::error!("MCP bridge start called twice - service already running");
            });
        }

        // SAFETY: Validated self.rx.is_some() above
        // We need unsafe to mutate through &self because BackgroundService trait
        // only provides &self but we need to move rx into the async task
        let rx = unsafe {
            let this = self as *const Self as *mut Self;
            (*this).rx.take().unwrap()
        };

        Box::pin(async move {
            log::info!("🔌 Starting MCP bridge");
            tokio::select! {
                _ = mcp_bridge::run(rx) => {
                    log::info!("MCP bridge stopped");
                }
                _ = shutdown.changed() => {
                    log::info!("MCP bridge shutting down");
                }
            }
        })
    }
}

struct DnsDiscoveryService {
    service_name: String,
    discovery: dns_discovery::DnsDiscovery,
}

impl BackgroundService for DnsDiscoveryService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        // We need to move the discovery out of self
        let service_name = self.service_name.clone();
        let discovery =
            unsafe { std::ptr::read(&self.discovery as *const dns_discovery::DnsDiscovery) };

        Box::pin(async move {
            log::info!("🌍 Starting DNS discovery for: {}", service_name);
            tokio::select! {
                _ = discovery.run() => {
                    log::info!("DNS discovery stopped");
                }
                _ = shutdown.changed() => {
                    log::info!("DNS discovery shutting down");
                }
            }
        })
    }
}

struct MdnsDiscoveryService {
    discovery: mdns_discovery::MdnsDiscovery,
}

impl BackgroundService for MdnsDiscoveryService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        // We need to move the discovery out of self
        let discovery =
            unsafe { std::ptr::read(&self.discovery as *const mdns_discovery::MdnsDiscovery) };

        Box::pin(async move {
            log::info!("🔍 Starting mDNS local discovery");
            tokio::select! {
                _ = discovery.run() => {
                    log::info!("mDNS discovery stopped");
                }
                _ = shutdown.changed() => {
                    log::info!("mDNS discovery shutting down");
                }
            }
        })
    }
}

struct PeerDiscoveryService {
    service: peer_discovery::DiscoveryService,
}

impl BackgroundService for PeerDiscoveryService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        // We need to move the service out of self
        let service =
            unsafe { std::ptr::read(&self.service as *const peer_discovery::DiscoveryService) };

        Box::pin(async move {
            log::info!("🔄 Starting HTTP peer exchange");
            tokio::select! {
                _ = service.run() => {
                    log::info!("Peer discovery stopped");
                }
                _ = shutdown.changed() => {
                    log::info!("Peer discovery shutting down");
                }
            }
        })
    }
}

struct RateLimitCleanupService {
    rate_limiter: Arc<rate_limit::distributed::DistributedRateLimitManager>,
}

impl BackgroundService for RateLimitCleanupService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let rate_limiter = self.rate_limiter.clone();

        Box::pin(async move {
            log::info!("🧹 Starting rate limit cleanup service");
            let mut cleanup_interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes

            loop {
                tokio::select! {
                    _ = cleanup_interval.tick() => {
                        rate_limiter.cleanup_unused_limiters();
                    }
                    _ = shutdown.changed() => {
                        log::info!("Rate limit cleanup shutting down");
                        break;
                    }
                }
            }
        })
    }
}

/// Background service that updates MetricPicker with discovered peers
struct BackendUpdateService {
    picker_swap: Arc<arc_swap::ArcSwap<metric_picker::MetricPicker>>,
    peer_discovery: Arc<peer_discovery::PeerDiscovery>,
    static_upstreams: Vec<pingora_load_balancing::Backend>,
}

impl BackgroundService for BackendUpdateService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let picker_swap = self.picker_swap.clone();
        let peer_discovery = self.peer_discovery.clone();
        let static_upstreams = self.static_upstreams.clone();
        
        Box::pin(async move {
            log::info!("🔄 Starting backend update service");
            let mut update_interval = tokio::time::interval(Duration::from_secs(5));
            
            loop {
                tokio::select! {
                    _ = update_interval.tick() => {
                        // Discover current peers
                        match peer_discovery.discover().await {
                            Ok((discovered_peers, _)) => {
                                // Merge static upstreams + discovered peers
                                let discovered_count = discovered_peers.len();
                                let mut all_backends = std::collections::BTreeSet::new();
                                all_backends.extend(static_upstreams.iter().cloned());
                                all_backends.extend(discovered_peers);
                                
                                // Create new picker with all backends
                                let new_picker = metric_picker::MetricPicker::from_backends(&all_backends);
                                
                                // Preserve load values from old picker
                                let old_picker = picker_swap.load();
                                for (new_idx, new_backend) in new_picker.backends.iter().enumerate() {
                                    for (old_idx, old_backend) in old_picker.backends.iter().enumerate() {
                                        if new_backend.addr.to_string() == old_backend.addr.to_string() {
                                            let old_load = old_picker.load_values[old_idx].load(std::sync::atomic::Ordering::Acquire);
                                            new_picker.load_values[new_idx].store(old_load, std::sync::atomic::Ordering::Release);
                                            break;
                                        }
                                    }
                                }
                                
                                // Atomic swap
                                picker_swap.store(Arc::new(new_picker));
                                
                                log::debug!("Updated picker with {} total backends ({} static + {} discovered)",
                                    all_backends.len(), static_upstreams.len(), discovered_count);
                            }
                            Err(e) => {
                                log::warn!("Failed to discover peers: {}", e);
                            }
                        }
                    }
                    _ = shutdown.changed() => {
                        log::info!("Backend update service shutting down");
                        break;
                    }
                }
            }
        })
    }
}

struct MetricsCollectorService {
    metric_picker: Arc<arc_swap::ArcSwap<metric_picker::MetricPicker>>,
    circuit_breaker_manager: Arc<circuit_breaker::CircuitBreakerManager>,
}

struct TlsCertificateManagerService {
    tls_dir: String,
    authority: tls::CertificateAuthority,
    server_domain: String,
}

impl BackgroundService for TlsCertificateManagerService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let tls_dir = self.tls_dir.clone();
        let authority = self.authority.clone();
        let server_domain = self.server_domain.clone();

        Box::pin(async move {
            log::info!("🔐 Starting TLS certificate manager service");
            
            // Certificate monitoring interval (check every hour)
            let mut check_interval = tokio::time::interval(Duration::from_secs(3600));

            loop {
                tokio::select! {
                    _ = check_interval.tick() => {
                        // Check certificate expiry
                        let cert_path = std::path::PathBuf::from(&tls_dir).join("server.crt");
                        
                        match check_certificate_expiry(&cert_path) {
                            Ok(days_until_expiry) => {
                                log::info!("Server certificate expires in {} days", days_until_expiry);
                                
                                // Renew if less than 30 days
                                if days_until_expiry < 30 {
                                    log::warn!("Certificate expiring soon, attempting renewal...");
                                    
                                    match renew_server_certificate(&authority, &server_domain, &tls_dir).await {
                                        Ok(()) => {
                                            log::info!("✅ Certificate renewed successfully");
                                        }
                                        Err(e) => {
                                            log::error!("❌ Certificate renewal failed: {}", e);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("Failed to check certificate expiry: {}", e);
                            }
                        }
                    }
                    _ = shutdown.changed() => {
                        log::info!("TLS certificate manager shutting down");
                        break;
                    }
                }
            }
        })
    }
}

impl BackgroundService for MetricsCollectorService {
    fn start<'life0, 'async_trait>(
        &'life0 self,
        mut shutdown: ShutdownWatch,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        let metric_picker = self.metric_picker.clone();
        let circuit_breaker_manager = self.circuit_breaker_manager.clone();

        Box::pin(async move {
            log::info!("📊 Starting metrics collector service");
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(2))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new());

            let mut scrape_interval = tokio::time::interval(Duration::from_secs(5));

            loop {
                tokio::select! {
                    _ = scrape_interval.tick() => {
                        let current_picker = metric_picker.load();
                        let targets = current_picker.get_metrics_targets();
                        for (idx, url) in targets {
                            let client_clone = client.clone();
                            let picker_swap_clone = metric_picker.clone();

                            // Spawn individual metric fetches to run concurrently
                            tokio::spawn(async move {
                                if let Ok(response) = client_clone.get(&url).send().await
                                    && let Ok(text) = response.text().await {
                                        // Parse prometheus metrics for node_load1
                                        for line in text.lines() {
                                            if line.starts_with("node_load1 ")
                                                && let Some(value_str) = line.split_whitespace().nth(1)
                                                && let Ok(value) = value_str.parse::<f64>() {
                                                    let current_picker = picker_swap_clone.load();
                                                    current_picker.update_load(idx, value);
                                                    break;
                                                }
                                        }
                                    }
                            });
                        }
                        
                        // Export circuit breaker metrics for all peers
                        let all_breakers = circuit_breaker_manager.get_all_breakers().await;
                        for (peer_id, breaker) in all_breakers {
                            let state = breaker.get_state().await;
                            let (total, failed) = breaker.get_metrics();
                            
                            log::debug!(
                                "Circuit breaker metrics for {}: state={:?}, total={}, failed={}",
                                peer_id, state, total, failed
                            );
                        }
                    }
                    _ = shutdown.changed() => {
                        log::info!("Metrics collector shutting down");
                        break;
                    }
                }
            }
        })
    }
}

/// Check certificate expiry and return days until expiration
fn check_certificate_expiry(cert_path: &std::path::Path) -> anyhow::Result<i64> {
    use x509_parser::prelude::*;
    
    let pem_data = std::fs::read(cert_path)?;
    let (_, pem) = parse_x509_pem(&pem_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse PEM: {}", e))?;
    let cert = pem.parse_x509()
        .map_err(|e| anyhow::anyhow!("Failed to parse X.509 certificate: {}", e))?;
    
    let not_after = cert.validity().not_after;
    let expiry_time = not_after.timestamp();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as i64;
    
    let days_until_expiry = (expiry_time - now) / 86400;
    Ok(days_until_expiry)
}

/// Renew server certificate using existing CA
async fn renew_server_certificate(
    authority: &tls::CertificateAuthority,
    domain: &str,
    tls_dir: &str,
) -> anyhow::Result<()> {
    use tls::builder::Tls;
    
    let response = Tls::certificate()
        .generator()
        .domain(domain)
        .authority(authority)
        .valid_for_days(365)
        .generate()
        .await;
    
    if !response.success {
        anyhow::bail!("Certificate generation failed: {:?}", response.issues);
    }
    
    // Extract certificate and key from response (directly available as fields)
    let cert_pem = response.certificate_pem
        .ok_or_else(|| anyhow::anyhow!("Certificate not found in response"))?;
    
    let key_pem = response.private_key_pem
        .ok_or_else(|| anyhow::anyhow!("Private key not found in response"))?;
    
    // Write new certificate and key
    let cert_path = std::path::PathBuf::from(tls_dir).join("server.crt");
    let key_path = std::path::PathBuf::from(tls_dir).join("server.key");
    
    std::fs::write(&cert_path, cert_pem)?;
    std::fs::write(&key_path, key_pem)?;
    
    Ok(())
}
