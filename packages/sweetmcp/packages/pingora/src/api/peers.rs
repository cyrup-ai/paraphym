//! Peer discovery API endpoint handlers with token verification

use std::sync::Arc;

use anyhow::{Context, Result};
use http::HeaderMap;

use crate::crypto::core::{EncryptedToken, TokenManager};
use crate::peer_discovery::{PeerRegistry, PeersResponse, BUILD_ID};

/// Verify discovery token from request headers
pub async fn verify_discovery_token(
    token_manager: &TokenManager,
    expected_token: &str,
    header_value: &str,
) -> Result<bool> {
    // Parse encrypted token from header
    let encrypted_token: EncryptedToken = serde_json::from_str(header_value)
        .context("Invalid encrypted token format")?;
    
    // Decrypt token
    let decrypted_token = token_manager.decrypt_token(&encrypted_token)
        .await
        .context("Failed to decrypt discovery token")?;
    
    // Constant-time comparison to prevent timing attacks
    Ok(constant_time_compare(
        decrypted_token.as_bytes(),
        expected_token.as_bytes()
    ))
}

/// Constant-time string comparison
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Handle /api/peers endpoint
pub async fn handle_peers_request(
    token_manager: Arc<TokenManager>,
    peer_registry: Arc<PeerRegistry>,
    expected_token: &str,
    request_headers: &HeaderMap,
) -> Result<PeersResponse> {
    // Extract and verify discovery token
    if let Some(token_header) = request_headers.get("x-discovery-token") {
        let token_str = token_header.to_str()
            .context("Invalid token header encoding")?;
        
        if !verify_discovery_token(&token_manager, expected_token, token_str).await? {
            anyhow::bail!("Invalid discovery token");
        }
    } else {
        anyhow::bail!("Missing x-discovery-token header");
    }
    
    // Token verified - return peer list
    let peers = peer_registry.get_healthy_peers();
    let peer_strs: Vec<String> = peers.iter()
        .map(|addr| addr.to_string())
        .collect();
    
    Ok(PeersResponse {
        build_id: BUILD_ID.to_string(),
        peers: peer_strs,
    })
}
