# IPCIDR_1: Implement CIDR Matching

## OBJECTIVE
Replace CIDR matching stub with proper implementation using ipnetwork crate.

## LOCATION
`packages/sweetmcp/plugins/ip/src/lib.rs:371-391`

## RESEARCH SUMMARY

### ipnetwork Crate API
Cloned and analyzed: [./tmp/ipnetwork](../../tmp/ipnetwork)

The ipnetwork crate provides:
- `IpNetwork` enum that handles both IPv4 and IPv6
- `IpNetwork::from_str()` via `parse()` for CIDR parsing (e.g., "192.168.1.0/24")
- `contains(&self, ip: IpAddr) -> bool` method for checking IP membership
- Automatic handling of IPv4/IPv6 mismatch (returns false)

**Key API Usage Pattern** (from [src/lib.rs:243-256](../../tmp/ipnetwork/src/lib.rs)):
```rust
use std::net::IpAddr;
use ipnetwork::IpNetwork;

let net: IpNetwork = "127.0.0.0/24".parse().unwrap();
let ip1: IpAddr = "127.0.0.1".parse().unwrap();
let ip2: IpAddr = "172.0.0.1".parse().unwrap();
let ip4: IpAddr = "::1".parse().unwrap();
assert!(net.contains(ip1));   // true - IPv4 in range
assert!(!net.contains(ip2));  // false - IPv4 out of range
assert!(!net.contains(ip4));  // false - IPv6 vs IPv4 mismatch
```

### Current Code Analysis

**Existing Infrastructure** (in [packages/sweetmcp/plugins/ip/src/lib.rs](../../packages/sweetmcp/plugins/ip/src/lib.rs)):
- Line 1: Already imports `std::net::{IpAddr, Ipv4Addr, Ipv6Addr}` ✓
- Line 36: Schema declares "cidr_contains" operation ✓
- Line 52: Operation enum includes "cidr_contains" ✓  
- Line 57-58: Schema has `cidr` parameter (CIDR notation like "192.168.1.0/24") ✓
- Line 79: Router dispatches to `cidr_contains(args_map)` ✓
- Lines 371-391: **STUB IMPLEMENTATION** - needs replacement

**Response Pattern** (from line 232-247):
All IP operation functions use:
```rust
Ok(ContentBuilder::text(json!({...}).to_string()))  // Success
Ok(ContentBuilder::error("message"))                 // Error
```

## IMPLEMENTATION SPECIFICATION

### SUBTASK 1: Add ipnetwork Dependency
**File:** `packages/sweetmcp/plugins/ip/Cargo.toml`

**Action:** Add to `[dependencies]` section (after line 24):
```toml
ipnetwork = "0.20"
```

### SUBTASK 2: Add Import Statement
**File:** `packages/sweetmcp/plugins/ip/src/lib.rs`

**Action:** Add to imports at top (after line 1):
```rust
use ipnetwork::IpNetwork;
```

### SUBTASK 3: Replace Stub Implementation
**File:** `packages/sweetmcp/plugins/ip/src/lib.rs:371-391`

**Replace entire function with:**
```rust
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

    // Parse CIDR notation (e.g., "192.168.1.0/24")
    let network: IpNetwork = cidr_str.parse()
        .map_err(|e| Error::msg(format!("Invalid CIDR notation '{}': {}", cidr_str, e)))?;
    
    // Parse IP address
    let ip: IpAddr = ip_str.parse()
        .map_err(|e| Error::msg(format!("Invalid IP address '{}': {}", ip_str, e)))?;
    
    // Check if IP is contained in the CIDR range
    let contains = network.contains(ip);
    
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
```

## DEFINITION OF DONE

### Functional Requirements
- ✓ Parse CIDR notation (e.g., "192.168.1.0/24", "2001:db8::/32")
- ✓ Parse IP addresses (IPv4 and IPv6)
- ✓ Return boolean `contains` result
- ✓ Handle IPv4 and IPv6 networks correctly
- ✓ Return proper error messages for invalid input
- ✓ Remove stub message completely

### Technical Requirements
- ✓ No compilation errors or warnings
- ✓ Follows existing code patterns in the file
- ✓ Uses ContentBuilder::text() for success responses
- ✓ Uses proper error handling with descriptive messages
- ✓ Returns JSON with `ip`, `cidr`, `contains`, and `network_type` fields

### Example Usage
```bash
# IPv4 match - should return contains: true
{"operation": "cidr_contains", "ip": "192.168.1.100", "cidr": "192.168.1.0/24"}

# IPv4 no match - should return contains: false  
{"operation": "cidr_contains", "ip": "10.0.0.1", "cidr": "192.168.1.0/24"}

# IPv6 match - should return contains: true
{"operation": "cidr_contains", "ip": "2001:db8::1", "cidr": "2001:db8::/32"}

# Type mismatch - should return contains: false
{"operation": "cidr_contains", "ip": "::1", "cidr": "192.168.1.0/24"}
```

## REFERENCES
- ipnetwork crate docs: https://docs.rs/ipnetwork/0.20
- Cloned source: [./tmp/ipnetwork](../../tmp/ipnetwork)
- IpNetwork API: [./tmp/ipnetwork/src/lib.rs](../../tmp/ipnetwork/src/lib.rs)
- CIDR notation: RFC 4632 (IPv4), RFC 4291 (IPv6)
- Current stub: [./packages/sweetmcp/plugins/ip/src/lib.rs:371](../../packages/sweetmcp/plugins/ip/src/lib.rs)
