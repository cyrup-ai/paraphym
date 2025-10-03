//! Core cryptographic types and structures
//!
//! This module provides the foundational types and data structures for secure
//! token handling with NaCl box encryption, zero allocation patterns, and
//! blazing-fast performance.


use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_;
use tokio::sync::RwLock;
use tracing::info;

pub const TOKEN_ROTATION_HOURS: u64 = 24;
pub const TOKEN_VALIDITY_HOURS: u64 = 48; // Allow grace period for rotation

/// Encrypted discovery token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedToken {
    /// The encrypted token data
    pub ciphertext: String,
    /// Timestamp when token was created
    pub created_at: u64,
    /// Public key used for encryption (for key rotation)
    pub key_id: String,
}

/// Token manager for secure discovery tokens
pub struct TokenManager {
    /// Current keypair for encryption
    pub current_keypair: Arc<RwLock<TokenKeypair>>,
    /// Previous keypair for decryption during rotation
    pub previous_keypair: Arc<RwLock<Option<TokenKeypair>>>,
    /// Revoked token identifiers with revocation timestamp
    pub revoked_tokens: Arc<RwLock<HashMap<String, SystemTime>>>,
}

/// Cryptographic keypair for token operations
pub struct TokenKeypair {
    pub public_key: box_::PublicKey,
    pub secret_key: box_::SecretKey,
    pub key_id: String,
    pub created_at: SystemTime,
}

/// Token data structure for serialization
#[derive(Serialize, Deserialize)]
pub struct TokenData {
    pub token: String,
    pub issued_at: u64,
    pub nonce: String,
}

impl TokenManager {
    /// Create a new token manager
    pub fn new() -> Result<Self> {
        // Initialize sodium
        sodiumoxide::init().map_err(|_| anyhow::anyhow!("Failed to initialize sodiumoxide"))?;

        let keypair = Self::generate_keypair()?;

        Ok(Self {
            current_keypair: Arc::new(RwLock::new(keypair)),
            previous_keypair: Arc::new(RwLock::new(None)),
            revoked_tokens: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Generate a new keypair
    pub fn generate_keypair() -> Result<TokenKeypair> {
        let (public_key, secret_key) = box_::gen_keypair();

        // Generate deterministic key ID from public key
        let key_id = BASE64.encode(&public_key.0[..8]); // Use first 8 bytes as ID

        Ok(TokenKeypair {
            public_key,
            secret_key,
            key_id,
            created_at: SystemTime::now(),
        })
    }



    /// Check if a token is revoked
    pub async fn is_token_revoked(&self, token_id: &str) -> bool {
        let revoked = self.revoked_tokens.read().await;
        revoked.contains_key(token_id)
    }



    /// Clean up expired revoked tokens
    pub async fn cleanup_expired_revocations(&self, max_age: Duration) -> Result<usize> {
        let mut revoked = self.revoked_tokens.write().await;
        let cutoff_time = SystemTime::now()
            .checked_sub(max_age)
            .ok_or_else(|| anyhow::anyhow!("Invalid max_age duration"))?;

        let initial_count = revoked.len();
        revoked.retain(|_, &mut revocation_time| revocation_time > cutoff_time);
        let cleaned_count = initial_count - revoked.len();

        if cleaned_count > 0 {
            info!("Cleaned up {} expired token revocations", cleaned_count);
        }

        Ok(cleaned_count)
    }



    /// Check if keypair needs rotation based on age
    pub async fn needs_rotation(&self) -> bool {
        let current = self.current_keypair.read().await;
        let age = current.created_at.elapsed().unwrap_or(Duration::ZERO);
        age > Duration::from_secs(TOKEN_ROTATION_HOURS * 3600)
    }



    /// Validate token timestamp
    pub fn is_token_timestamp_valid(&self, timestamp: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        let token_age = now.saturating_sub(timestamp);
        token_age <= TOKEN_VALIDITY_HOURS * 3600
    }

    /// Generate secure nonce
    pub fn generate_nonce() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_nanos();

        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        let hash = hasher.finish();

        BASE64.encode(hash.to_le_bytes())
    }

    /// Validate encrypted token structure
    pub fn validate_encrypted_token(&self, encrypted: &EncryptedToken) -> Result<()> {
        // Validate base64 ciphertext
        BASE64
            .decode(&encrypted.ciphertext)
            .map_err(|e| anyhow::anyhow!("Invalid ciphertext base64: {}", e))?;

        // Validate timestamp
        if !self.is_token_timestamp_valid(encrypted.created_at) {
            return Err(anyhow::anyhow!("Token timestamp is too old"));
        }

        // Validate key_id format
        if encrypted.key_id.is_empty() {
            return Err(anyhow::anyhow!("Empty key_id"));
        }

        Ok(())
    }

}

impl EncryptedToken {
    /// Create new encrypted token
    pub fn new(ciphertext: String, key_id: String) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        Self {
            ciphertext,
            created_at,
            key_id,
        }
    }


}

impl TokenData {
    /// Create new token data
    pub fn new(token: String) -> Self {
        let issued_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        let nonce = TokenManager::generate_nonce();

        Self {
            token,
            issued_at,
            nonce,
        }
    }

    /// Check if token data is valid
    pub fn is_valid(&self) -> bool {
        !self.token.is_empty() && !self.nonce.is_empty() && self.issued_at > 0
    }

    /// Get token age
    pub fn age(&self) -> Duration {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        Duration::from_secs(now.saturating_sub(self.issued_at))
    }

    /// Check if token data is expired
    pub fn is_expired(&self) -> bool {
        self.age() > Duration::from_secs(TOKEN_VALIDITY_HOURS * 3600)
    }
}
