//! DNS-based service discovery for SweetMCP using DNS SRV records
//!
//! This module implements secure, zero-configuration service discovery using
//! DNS SRV records - the industry standard approach used by Consul, Kubernetes,
//! etcd, and other distributed systems.
//!
//! ## DNS SRV Record Format
//! ```text
//! _service._proto.domain TTL class SRV priority weight port target
//! _sweetmcp._tcp.example.com. 300 IN SRV 10 60 8443 node1.example.com.
//! ```

use std::net::SocketAddr;
use std::time::Duration;

use hickory_resolver::config::ResolverConfig;
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::TokioResolver;
use tokio::time::interval;
use log::{debug, error, info, warn};

use crate::peer_discovery::PeerRegistry;

const DISCOVERY_INTERVAL: Duration = Duration::from_secs(60);
const RESOLUTION_TIMEOUT: Duration = Duration::from_secs(5);

/// DNS-based discovery service using SRV records
pub struct DnsDiscovery {
    resolver: TokioResolver,
    service_name: String,
    registry: PeerRegistry,
}

impl DnsDiscovery {
    /// Creates a new DNS discovery instance
    ///
    /// # Arguments
    /// - `service_name`: The SRV service name (e.g., "_sweetmcp._tcp.example.com")
    /// - `registry`: The peer registry to update with discovered services
    ///
    /// # Example Service Names
    /// - `_sweetmcp._tcp.local` - mDNS/local network
    /// - `_sweetmcp._tcp.cluster.local` - Kubernetes cluster
    /// - `_sweetmcp._tcp.consul` - Consul service mesh
    /// - `_sweetmcp._tcp.example.com` - Custom domain
    pub fn new(service_name: String, registry: PeerRegistry) -> Self {
        // Create resolver with system configuration
        // This automatically uses /etc/resolv.conf on Unix or registry on Windows
        let builder = TokioResolver::builder_tokio()
            .unwrap_or_else(|e| {
                warn!(
                    "Failed to read system DNS config, using Google Public DNS (8.8.8.8, 8.8.4.4): {}",
                    e
                );
                TokioResolver::builder_with_config(
                    ResolverConfig::google(),
                    TokioConnectionProvider::default()
                )
            });
        
        let resolver = builder.build();

        info!(
            "DNS discovery initialized for service: {}",
            service_name
        );

        Self {
            resolver,
            service_name,
            registry,
        }
    }

    /// Start the DNS discovery service
    ///
    /// Performs initial discovery immediately, then polls at DISCOVERY_INTERVAL
    pub async fn run(self) {
        info!("Starting DNS discovery for service: {}", self.service_name);

        let mut discovery_interval = interval(DISCOVERY_INTERVAL);

        // Discover immediately on startup
        self.discover_peers().await;

        loop {
            discovery_interval.tick().await;
            self.discover_peers().await;
        }
    }

    /// Perform DNS SRV lookup and register discovered peers
    async fn discover_peers(&self) {
        debug!(
            "Performing DNS SRV lookup for service: {}",
            self.service_name
        );

        // Perform SRV lookup with timeout
        let lookup_result = tokio::time::timeout(
            RESOLUTION_TIMEOUT,
            self.resolver.srv_lookup(&self.service_name),
        )
        .await;

        match lookup_result {
            Ok(Ok(srv_lookup)) => {
                let srv_records: Vec<_> = srv_lookup.iter().collect();

                if srv_records.is_empty() {
                    warn!(
                        "No SRV records found for service: {}",
                        self.service_name
                    );
                    return;
                }

                info!(
                    "Found {} SRV records for service: {}",
                    srv_records.len(),
                    self.service_name
                );

                // Resolve each SRV target with its specific port
                for srv in &srv_records {
                    let target = srv.target().to_string();
                    let port = srv.port();
                    
                    match tokio::time::timeout(RESOLUTION_TIMEOUT, self.resolver.lookup_ip(&target)).await {
                        Ok(Ok(lookup)) => {
                            for ip in lookup.iter() {
                                let addr = SocketAddr::new(ip, port);
                                self.registry.add_peer(addr);
                            }
                        }
                        Ok(Err(e)) => warn!("Failed to resolve {}: {}", target, e),
                        Err(_) => warn!("Timeout resolving {}", target),
                    }
                }
            }
            Ok(Err(e)) => {
                error!(
                    "DNS SRV lookup failed for {}: {}",
                    self.service_name, e
                );
            }
            Err(_) => {
                error!(
                    "DNS SRV lookup timed out after {:?} for {}",
                    RESOLUTION_TIMEOUT, self.service_name
                );
            }
        }
    }

}

/// Check if we should use DNS discovery based on environment
///
/// # Environment Variables
/// - `SWEETMCP_DNS_SERVICE`: Explicit DNS SRV service name to use
/// - `SWEETMCP_DOMAIN`: Domain to construct SRV name (_sweetmcp._tcp.DOMAIN)
///
/// # Returns
/// - `Some(service_name)` if DNS discovery should be enabled
/// - `None` if DNS discovery should be disabled (use mDNS instead)
pub fn should_use_dns_discovery() -> Option<String> {
    // Check for explicit DNS service name
    if let Ok(service) = std::env::var("SWEETMCP_DNS_SERVICE") {
        info!("Using explicit DNS service name from SWEETMCP_DNS_SERVICE: {}", service);
        return Some(service);
    }

    // Auto-construct from domain
    if let Ok(domain) = std::env::var("SWEETMCP_DOMAIN") {
        let service_name = format!("_sweetmcp._tcp.{}", domain);
        info!("Auto-constructed DNS service name: {}", service_name);
        return Some(service_name);
    }

    // No DNS configuration found
    debug!("No DNS discovery configuration found, will use mDNS instead");
    None
}
