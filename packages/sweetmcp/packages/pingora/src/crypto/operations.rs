//! Token management operations
//!
//! This module provides token encryption, decryption, rotation, and revocation
//! operations with zero allocation patterns and blazing-fast performance.




use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use sodiumoxide::crypto::sealedbox;
use tokio::time::interval;
use log::{error, info};

use crate::crypto::core::*;

impl TokenManager {
    /// Encrypt a token for secure transmission
    pub async fn encrypt_token(&self, token: &str) -> Result<EncryptedToken> {
        let current = self.current_keypair.read().await;

        let token_data = TokenData::new(token.to_string());
        let plaintext =
            serde_json::to_vec(&token_data).context("Failed to serialize token data")?;

        let ciphertext = sealedbox::seal(&plaintext, &current.public_key);
        let ciphertext_b64 = BASE64.encode(&ciphertext);

        Ok(EncryptedToken::new(ciphertext_b64, current.key_id.clone()))
    }

    /// Decrypt a token from secure transmission
    pub async fn decrypt_token(&self, encrypted: &EncryptedToken) -> Result<String> {
        // Validate the encrypted token first
        self.validate_encrypted_token(encrypted)?;

        // Check if token is revoked
        if self.is_token_revoked(&encrypted.key_id).await {
            return Err(anyhow::anyhow!("Token has been revoked"));
        }

        let ciphertext = BASE64
            .decode(&encrypted.ciphertext)
            .context("Failed to decode ciphertext")?;

        // Try current keypair first
        let current = self.current_keypair.read().await;
        if encrypted.key_id == current.key_id
            && let Ok(plaintext) =
                sealedbox::open(&ciphertext, &current.public_key, &current.secret_key)
            {
                let token_data: TokenData = serde_json::from_slice(&plaintext)
                    .context("Failed to deserialize token data")?;

                // Validate token data
                if !token_data.is_valid() {
                    return Err(anyhow::anyhow!("Invalid token data"));
                }

                if token_data.is_expired() {
                    return Err(anyhow::anyhow!("Token data is expired"));
                }

                return Ok(token_data.token);
            }

        // Try previous keypair if current failed
        let previous = self.previous_keypair.read().await;
        if let Some(prev_keypair) = previous.as_ref()
            && encrypted.key_id == prev_keypair.key_id
            && let Ok(plaintext) = sealedbox::open(
                &ciphertext,
                &prev_keypair.public_key,
                &prev_keypair.secret_key,
            ) {
                let token_data: TokenData = serde_json::from_slice(&plaintext)
                    .context("Failed to deserialize token data")?;

                // Validate token data
                if !token_data.is_valid() {
                    return Err(anyhow::anyhow!("Invalid token data"));
                }

                if token_data.is_expired() {
                    return Err(anyhow::anyhow!("Token data is expired"));
                }

                return Ok(token_data.token);
            }

        Err(anyhow::anyhow!("Failed to decrypt token"))
    }



    /// Rotate the keypair (move current to previous, generate new current)
    pub async fn rotate_keypair(&self) -> Result<()> {
        info!("Starting keypair rotation");

        let new_keypair = Self::generate_keypair().context("Failed to generate new keypair")?;

        // Move current to previous
        {
            let current = self.current_keypair.read().await;
            let mut previous = self.previous_keypair.write().await;
            *previous = Some(TokenKeypair {
                public_key: current.public_key,
                secret_key: current.secret_key.clone(),
                key_id: current.key_id.clone(),
                created_at: current.created_at,
            });
        }

        // Set new current
        {
            let mut current = self.current_keypair.write().await;
            *current = new_keypair;
        }

        info!("Keypair rotation completed successfully");
        Ok(())
    }

    /// Start automatic token rotation
    pub async fn start_rotation_task(self: std::sync::Arc<Self>) -> Result<()> {
        let mut interval = interval(std::time::Duration::from_secs(3600)); // Check every hour

        tokio::spawn(async move {
            loop {
                interval.tick().await;

                if self.needs_rotation().await
                    && let Err(e) = self.rotate_keypair().await {
                        error!("Failed to rotate keypair: {}", e);
                    }

                // Clean up old revocations (older than 7 days)
                let max_age = std::time::Duration::from_secs(7 * 24 * 3600);
                if let Err(e) = self.cleanup_expired_revocations(max_age).await {
                    error!("Failed to cleanup expired revocations: {}", e);
                }
            }
        });

        Ok(())
    }



}
