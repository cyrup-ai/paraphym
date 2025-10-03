# MESH_TLS - Server-Side mTLS Validation

**STATUS**: 🔴 CRITICAL SECURITY VULNERABILITY  
**QA RATING**: 3/10  
**PRIORITY**: P0 - Security Issue

## Executive Summary

Client-side mTLS is fully implemented and working. However, **server-side mTLS validation is completely missing**, creating a critical security vulnerability where unauthenticated clients can access peer mesh endpoints.

## ❌ CRITICAL MISSING REQUIREMENT

### Server-Side mTLS Validation (proxy_impl.rs)

**CURRENT STATE**: The `request_filter` method in `packages/sweetmcp/packages/pingora/src/edge/core/proxy_impl.rs` does NOT enforce TLS or validate client certificates for `/api/peers` endpoints.

**SECURITY IMPACT**:
- Any client (even without valid certificates) can register as a peer
- Attackers can query the peer list without authentication
- Malicious peers can be injected into the mesh
- MITM attacks possible on peer-to-peer communication

**REQUIRED IMPLEMENTATION**:

The `request_filter` method must validate that `/api/peers` requests come over mTLS with valid client certificates:

```rust
fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool> {
    let req_header = session.req_header_mut();
    let path = req_header.uri.path().to_string();
    let method = req_header.method.clone();

    // PHASE 0: mTLS Client Certificate Validation for /api/peers
    if path.starts_with("/api/peers") {
        // Verify TLS connection exists
        let ssl_digest = session.digest();
        if ssl_digest.is_none() {
            warn!("Rejecting non-TLS request to {}", path);
            let mut response = ResponseHeader::build(403, None)?;
            response.insert_header("Content-Type", "text/plain")?;
            session.write_response_header(Box::new(response), true)?;
            session.write_response_body(
                Some(Bytes::from("Forbidden: TLS required for peer endpoints")),
                true,
            )?;
            return Ok(true); // Request handled, stop processing
        }

        // Extract and validate client certificate from TLS session
        if let Some(digest) = ssl_digest {
            // Access peer certificate chain
            if let Some(peer_cert) = digest.peer_cert() {
                // Verify certificate is valid and trusted
                // TODO: Implement certificate validation against trusted CA
                info!("mTLS authenticated request to {} from cert: {:?}", path, peer_cert);
            } else {
                warn!("Rejecting TLS request without client certificate to {}", path);
                let mut response = ResponseHeader::build(403, None)?;
                response.insert_header("Content-Type", "text/plain")?;
                session.write_response_header(Box::new(response), true)?;
                session.write_response_body(
                    Some(Bytes::from("Forbidden: Client certificate required")),
                    true,
                )?;
                return Ok(true); // Request handled, stop processing
            }
        }
    }

    // ... rest of existing request_filter logic ...
```

**VERIFICATION CHECKLIST**:
- [ ] Add TLS connection check using `session.digest()`
- [ ] Reject non-TLS requests to `/api/peers` with 403 Forbidden
- [ ] Extract client certificate from TLS session
- [ ] Validate client certificate against trusted CA
- [ ] Log mTLS authentication events
- [ ] Test with valid client certificate (should succeed)
- [ ] Test with missing client certificate (should reject with 403)
- [ ] Test with HTTP instead of HTTPS (should reject with 403)
- [ ] Test with invalid/expired client certificate (should reject with 403)

## ✅ COMPLETED REQUIREMENTS

The following are fully implemented and working correctly:

1. **TlsConfig Extension** (`tls_manager.rs:34-37`)
   - Added `client_cert_path` and `client_key_path` fields
   
2. **Client Config Builder** (`tls_manager.rs:673-720`)
   - Implemented `finalize_client_config_builder` method
   - Loads client certificate and private key
   - Configures rustls with client auth

3. **Config Builder Integration** (`tls_manager.rs:626,639,650`)
   - All client config builders call finalize method
   
4. **DiscoveryService Integration** (`peer_discovery.rs:304-379`)
   - Refactored to use TlsManager instead of ad-hoc reqwest::Identity
   - Properly configures client certificates

5. **Reqwest Client** (`peer_discovery.rs:375-378`)
   - Uses preconfigured TLS from TlsManager
   
6. **HTTPS Enforcement** (`peer_discovery.rs:469`)
   - Peer discovery uses HTTPS URLs

## Implementation Priority

**IMMEDIATE ACTION REQUIRED**: Implement server-side mTLS validation in `proxy_impl.rs` before deploying to production. The current state provides a false sense of security.
