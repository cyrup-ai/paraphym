use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use ipnetwork::IpNetwork;

use extism_pdk::*;
use log::{debug, trace};
use serde_json::{Value, json};
use sweetmcp_plugin_builder::prelude::*;
use sweetmcp_plugin_builder::{CallToolResult, Ready};

#[cfg(not(target_arch = "wasm32"))]
use hyper::body::Bytes;
#[cfg(not(target_arch = "wasm32"))]
use hyper_util::client::legacy::Client;
#[cfg(not(target_arch = "wasm32"))]
use hyper_rustls::HttpsConnectorBuilder;
#[cfg(not(target_arch = "wasm32"))]
use http_body_util::{BodyExt, Empty};
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::{timeout, Duration};

/// IP operations tool using plugin-builder
struct IpTool;

impl McpTool for IpTool {
    const NAME: &'static str = "ip";

    fn description(builder: DescriptionBuilder) -> DescriptionBuilder {
        builder
            .does("Perform comprehensive IP address operations including validation, analysis, and network calculations")
            .when("you need to validate IP address formats (IPv4/IPv6)")
            .when("you need to check if an IP is within a private range")
            .when("you need to convert IP addresses to binary representation")
            .when("you need to create IP addresses programmatically")
            .when("you need to perform CIDR subnet calculations")
            .when("you need to analyze network ranges and memberships")
            .perfect_for("network administration, security analysis, subnet planning, and IP address management")
            .operation("get_public_ip", "Get the public IP address of the current system")
            .operation("validate_ip", "Validate if a string is a proper IP address and determine its type")
            .operation("ip_info", "Get detailed information about an IP address")
            .operation("is_private", "Check if an IP address is in a private range")
            .operation("ip_to_binary", "Convert IP address to binary representation")
            .operation("create_ipv4", "Create IPv4 address from octets and analyze properties")
            .operation("create_ipv6", "Create IPv6 address from segments and analyze properties")
            .operation("cidr_contains", "Check if an IP address is within a CIDR range")
    }

    fn schema(builder: SchemaBuilder) -> Value {
        #[cfg(not(target_arch = "wasm32"))]
        let operations = &[
            "get_public_ip",
            "validate_ip",
            "ip_info",
            "is_private",
            "ip_to_binary",
            "create_ipv4",
            "create_ipv6",
            "cidr_contains",
        ];
        
        #[cfg(target_arch = "wasm32")]
        let operations = &[
            "validate_ip",
            "ip_info",
            "is_private",
            "ip_to_binary",
            "create_ipv4",
            "create_ipv6",
            "cidr_contains",
        ];
        
        builder
            .required_enum(
                "name",
                "IP operation to perform",
                operations,
            )
            .optional_string("ip", "IP address to analyze (required for most operations)")
            .optional_string(
                "cidr",
                "CIDR notation for subnet operations (e.g., '192.168.1.0/24')",
            )
            .build()
    }

    fn execute(args: Value) -> Result<CallToolResult, Error> {
        let name = args
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::msg("name parameter required"))?;

        debug!("Executing IP operation: {}", name);

        let args_map = args.as_object().unwrap_or(&serde_json::Map::new()).clone();

        match name {
            #[cfg(not(target_arch = "wasm32"))]
            "get_public_ip" => get_public_ip(),
            "validate_ip" => validate_ip(args_map),
            "ip_info" => get_ip_info(args_map),
            "is_private" => check_private_ip(args_map),
            "ip_to_binary" => ip_to_binary(args_map),
            "create_ipv4" => create_ipv4(args_map),
            "create_ipv6" => create_ipv6(args_map),
            "cidr_contains" => cidr_contains(args_map),
            _ => {
                debug!("Unknown IP operation requested: {}", name);
                Ok(ContentBuilder::error(format!(
                    "Unknown IP operation: {}",
                    name
                )))
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Fetch public IP from external service
async fn fetch_public_ip_from_service(url: &str) -> Result<String, String> {
    // Build HTTPS connector with native root certificates
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .map_err(|e| format!("TLS init failed: {}", e))?
        .https_only()
        .enable_http1()
        .build();
    
    // Create HTTP client with legacy builder API
    let client: Client<_, Empty<Bytes>> = Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(https);
    
    // Build GET request
    let req = hyper::Request::builder()
        .uri(url)
        .header("User-Agent", "sweetmcp-ip-plugin/0.1.0")
        .body(Empty::<Bytes>::new())
        .map_err(|e| format!("Request build failed: {}", e))?;
    
    // Execute request with 5-second timeout
    let res = timeout(Duration::from_secs(5), client.request(req))
        .await
        .map_err(|_| "Request timeout after 5 seconds".to_string())?
        .map_err(|e| format!("HTTP request failed: {}", e))?;
    
    // Read response body
    let body = res.into_body()
        .collect()
        .await
        .map_err(|e| format!("Body read failed: {}", e))?
        .to_bytes();
    
    // Convert to trimmed string
    String::from_utf8(body.to_vec())
        .map_err(|e| format!("UTF-8 decode failed: {}", e))
        .map(|s| s.trim().to_string())
}

#[cfg(not(target_arch = "wasm32"))]
/// Get public IP address
fn get_public_ip() -> Result<CallToolResult, Error> {
    debug!("Fetching public IP address from external services");
    
    // Use multiple services for redundancy (fallback on failure)
    let services = [
        "https://api.ipify.org",      // Heroku-backed, highly reliable
        "https://icanhazip.com",       // Cloudflare-backed since 2021
        "https://ifconfig.me/ip",      // Alternative fallback
    ];
    
    trace!("Available services: {:?}", services);
    
    // Create Tokio runtime for async execution in sync plugin context
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::msg(format!("Tokio runtime failed: {}", e)))?;
    
    // Try each service sequentially until one succeeds
    for service in &services {
        debug!("Trying public IP service: {}", service);
        if let Ok(ip) = rt.block_on(fetch_public_ip_from_service(service)) {
            debug!("Successfully retrieved public IP: {} from {}", ip, service);
            return Ok(ContentBuilder::text(
                json!({
                    "ip": ip,
                    "source": service,
                    "success": true
                })
                .to_string(),
            ));
        }
        trace!("Service {} failed, trying next", service);
    }
    
    // All services failed
    debug!("All public IP services failed");
    Err(Error::msg("All public IP services failed"))
}

/// Validate IP address format
fn validate_ip(args: serde_json::Map<String, Value>) -> Result<CallToolResult, Error> {
    let ip_str = args
        .get("ip")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("ip parameter required for validate_ip"))?;

    debug!("Validating IP address: {}", ip_str);
    trace!("Attempting to parse IP string");

    let result = match ip_str.parse::<IpAddr>() {
        Ok(ip) => {
            let ip_type = match ip {
                IpAddr::V4(_) => {
                    debug!("Valid IPv4 address detected");
                    "IPv4"
                }
                IpAddr::V6(_) => {
                    debug!("Valid IPv6 address detected");
                    "IPv6"
                }
            };
            trace!("IP validation successful: {} ({})", ip_str, ip_type);
            json!({
                "valid": true,
                "type": ip_type,
                "address": ip_str
            })
        }
        Err(_) => {
            debug!("IP validation failed: invalid format");
            json!({
                "valid": false,
                "error": "Invalid IP address format"
            })
        }
    };

    Ok(ContentBuilder::text(result.to_string()))
}

/// Get detailed IP information
fn get_ip_info(args: serde_json::Map<String, Value>) -> Result<CallToolResult, Error> {
    let ip_str = args
        .get("ip")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("ip parameter required for ip_info"))?;

    debug!("Getting detailed information for IP: {}", ip_str);

    match ip_str.parse::<IpAddr>() {
        Ok(ip) => {
            let info = match ip {
                IpAddr::V4(ipv4) => {
                    debug!("Analyzing IPv4 address");
                    trace!("Private: {}", ipv4.is_private());
                    trace!("Loopback: {}", ipv4.is_loopback());
                    trace!("Multicast: {}", ipv4.is_multicast());
                    json!({
                        "address": ip_str,
                        "type": "IPv4",
                        "is_private": ipv4.is_private(),
                        "is_loopback": ipv4.is_loopback(),
                        "is_multicast": ipv4.is_multicast(),
                        "octets": ipv4.octets()
                    })
                }
                IpAddr::V6(ipv6) => {
                    debug!("Analyzing IPv6 address");
                    trace!("Loopback: {}", ipv6.is_loopback());
                    trace!("Multicast: {}", ipv6.is_multicast());
                    json!({
                        "address": ip_str,
                        "type": "IPv6",
                        "is_loopback": ipv6.is_loopback(),
                        "is_multicast": ipv6.is_multicast(),
                        "segments": ipv6.segments()
                    })
                }
            };
            debug!("IP info analysis complete");
            Ok(ContentBuilder::text(info.to_string()))
        }
        Err(_) => {
            debug!("IP info failed: invalid address format");
            Ok(ContentBuilder::error("Invalid IP address format"))
        }
    }
}

/// Check if IP is private
fn check_private_ip(args: serde_json::Map<String, Value>) -> Result<CallToolResult, Error> {
    let ip_str = args
        .get("ip")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("ip parameter required for is_private"))?;

    debug!("Checking if IP is private: {}", ip_str);

    match ip_str.parse::<IpAddr>() {
        Ok(IpAddr::V4(ipv4)) => {
            trace!("Checking IPv4 private range");
            let is_private = ipv4.is_private();
            debug!("IPv4 private check result: {}", is_private);
            Ok(ContentBuilder::text(
                json!({
                    "ip": ip_str,
                    "is_private": is_private,
                    "type": "IPv4"
                })
                .to_string(),
            ))
        }
        Ok(IpAddr::V6(_)) => {
            debug!("IPv6 address - private detection not fully implemented");
            Ok(ContentBuilder::text(
                json!({
                    "ip": ip_str,
                    "is_private": false,
                    "type": "IPv6",
                    "note": "IPv6 private detection not fully implemented"
                })
                .to_string(),
            ))
        }
        Err(_) => {
            debug!("Private check failed: invalid IP format");
            Ok(ContentBuilder::error("Invalid IP address format"))
        }
    }
}

/// Convert IP to binary
fn ip_to_binary(args: serde_json::Map<String, Value>) -> Result<CallToolResult, Error> {
    let ip_str = args
        .get("ip")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("ip parameter required for ip_to_binary"))?;

    debug!("Converting IP to binary representation: {}", ip_str);

    match ip_str.parse::<IpAddr>() {
        Ok(IpAddr::V4(ipv4)) => {
            debug!("Converting IPv4 to binary");
            let octets = ipv4.octets();
            trace!("IPv4 octets: {:?}", octets);
            
            let binary = octets
                .iter()
                .map(|octet| format!("{:08b}", octet))
                .collect::<Vec<_>>()
                .join(".");
            
            debug!("Binary conversion complete");
            Ok(ContentBuilder::text(json!({
                "ip": ip_str,
                "binary": binary,
                "octets_binary": octets.iter().map(|octet| format!("{:08b}", octet)).collect::<Vec<_>>()
            }).to_string()))
        }
        Ok(IpAddr::V6(ipv6)) => {
            debug!("Converting IPv6 to binary");
            let segments = ipv6.segments();
            trace!("IPv6 segments: {:?}", segments);
            
            let binary = segments
                .iter()
                .map(|segment| format!("{:016b}", segment))
                .collect::<Vec<_>>()
                .join(":");
            
            debug!("Binary conversion complete");
            Ok(ContentBuilder::text(json!({
                "ip": ip_str,
                "binary": binary,
                "segments_binary": segments.iter().map(|segment| format!("{:016b}", segment)).collect::<Vec<_>>()
            }).to_string()))
        }
        Err(_) => {
            debug!("Binary conversion failed: invalid IP format");
            Ok(ContentBuilder::error("Invalid IP address format"))
        }
    }
}

/// Create IPv4 from octets
fn create_ipv4(args: serde_json::Map<String, Value>) -> Result<CallToolResult, Error> {
    let octets = args
        .get("octets")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::msg("octets array required for create_ipv4"))?;

    debug!("Creating IPv4 address from octets");
    trace!("Input octets: {:?}", octets);

    if octets.len() != 4 {
        debug!("IPv4 creation failed: incorrect number of octets ({})", octets.len());
        return Ok(ContentBuilder::error("IPv4 requires exactly 4 octets"));
    }

    let octet_values: Result<Vec<u8>, _> = octets
        .iter()
        .map(|v| {
            v.as_u64()
                .and_then(|n| if n <= 255 { Some(n as u8) } else { None })
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| Error::msg("All octets must be numbers 0-255"));

    match octet_values {
        Ok(vals) => {
            debug!("Octets validated: {:?}", vals);
            let ipv4 = Ipv4Addr::new(vals[0], vals[1], vals[2], vals[3]);
            debug!("IPv4 address created: {}", ipv4);
            trace!("Analyzing IPv4 properties");
            Ok(ContentBuilder::text(
                json!({
                    "address": ipv4.to_string(),
                    "octets": vals,
                    "is_private": ipv4.is_private(),
                    "is_loopback": ipv4.is_loopback(),
                    "is_multicast": ipv4.is_multicast()
                })
                .to_string(),
            ))
        }
        Err(e) => {
            debug!("IPv4 creation failed: {}", e);
            Ok(ContentBuilder::error(&format!("Invalid octets: {}", e)))
        }
    }
}

/// Create IPv6 from segments
fn create_ipv6(args: serde_json::Map<String, Value>) -> Result<CallToolResult, Error> {
    let segments = args
        .get("segments")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Error::msg("segments array required for create_ipv6"))?;

    debug!("Creating IPv6 address from segments");
    trace!("Input segments: {:?}", segments);

    if segments.len() != 8 {
        debug!("IPv6 creation failed: incorrect number of segments ({})", segments.len());
        return Ok(ContentBuilder::error("IPv6 requires exactly 8 segments"));
    }

    let segment_values: Result<Vec<u16>, _> = segments
        .iter()
        .map(|v| {
            v.as_u64()
                .and_then(|n| if n <= 65535 { Some(n as u16) } else { None })
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| Error::msg("All segments must be numbers 0-65535"));

    match segment_values {
        Ok(vals) => {
            debug!("Segments validated: {:?}", vals);
            let ipv6 = Ipv6Addr::new(
                vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7],
            );
            debug!("IPv6 address created: {}", ipv6);
            trace!("Analyzing IPv6 properties");
            Ok(ContentBuilder::text(
                json!({
                    "address": ipv6.to_string(),
                    "segments": vals,
                    "is_loopback": ipv6.is_loopback(),
                    "is_multicast": ipv6.is_multicast()
                })
                .to_string(),
            ))
        }
        Err(e) => {
            debug!("IPv6 creation failed: {}", e);
            Ok(ContentBuilder::error(&format!("Invalid segments: {}", e)))
        }
    }
}

/// Check if IP is in CIDR range
fn cidr_contains(args: serde_json::Map<String, Value>) -> Result<CallToolResult, Error> {
    let ip_str = args
        .get("ip")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("ip parameter required for cidr_contains"))?;

    let cidr_str = args
        .get("cidr")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("cidr parameter required for cidr_contains"))?;

    debug!("Checking if IP {} is in CIDR range {}", ip_str, cidr_str);

    // Parse CIDR notation (e.g., "192.168.1.0/24")
    trace!("Parsing CIDR notation: {}", cidr_str);
    let network: IpNetwork = cidr_str.parse()
        .map_err(|e| Error::msg(format!("Invalid CIDR notation '{}': {}", cidr_str, e)))?;
    debug!("CIDR network parsed successfully");
    
    // Parse IP address
    trace!("Parsing IP address: {}", ip_str);
    let ip: IpAddr = ip_str.parse()
        .map_err(|e| Error::msg(format!("Invalid IP address '{}': {}", ip_str, e)))?;
    debug!("IP address parsed successfully");
    
    // Check if IP is contained in the CIDR range
    trace!("Performing subnet membership check");
    let contains = network.contains(ip);
    debug!("CIDR contains check result: {}", contains);
    
    Ok(ContentBuilder::text(
        json!({
            "ip": ip_str,
            "cidr": cidr_str,
            "contains": contains,
            "network_type": match network {
                IpNetwork::V4(_) => "IPv4",
                IpNetwork::V6(_) => "IPv6"
            }
        })
        .to_string(),
    ))
}

/// Create the plugin instance
#[allow(dead_code)]
fn plugin() -> McpPlugin<Ready> {
    mcp_plugin("ip")
        .description("Comprehensive IP address operations and network utilities")
        .tool::<IpTool>()
        .serve()
}

// Generate standard MCP entry points
sweetmcp_plugin_builder::generate_mcp_functions!(plugin);
