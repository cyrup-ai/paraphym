//! Certificate Authority types and data structures
//!
//! This module contains the core type definitions for certificate authorities,
//! including metadata, source tracking, and serialization support.

use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

use crate::tls::errors::TlsError;

/// Certificate Authority domain object with serialization support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateAuthority {
    pub name: String,
    pub certificate_pem: String,
    pub private_key_pem: String,
    pub metadata: CaMetadata,
}

/// Certificate Authority metadata with comprehensive information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaMetadata {
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub valid_from: SystemTime,
    pub valid_until: SystemTime,
    pub key_algorithm: String,
    pub key_size: Option<u32>,
    pub created_at: SystemTime,
    pub source: CaSource,
}

/// Source of the certificate authority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaSource {
    /// Loaded from filesystem
    Filesystem { path: PathBuf },
    /// Loaded from system keychain
    Keychain,
    /// Downloaded from remote URL
    Remote { url: String },
    /// Generated programmatically
    Generated,
}

impl CertificateAuthority {
    /// Load certificate authority from a PEM file
    ///
    /// # Errors
    ///
    /// Returns `TlsError` if the file cannot be read or parsed
    pub fn load(cert_path: &std::path::Path) -> Result<Self, TlsError> {
        use crate::tls::certificate::parsing::parse_certificate_from_pem;
        
        // Read certificate file
        let cert_pem = std::fs::read_to_string(cert_path).map_err(|e| {
            TlsError::FileOperation(format!("Failed to read certificate: {}", e))
        })?;

        // Parse certificate to extract metadata
        let parsed_cert = parse_certificate_from_pem(&cert_pem)?;

        // Try to find corresponding key file
        let key_path = cert_path.with_extension("key");
        let private_key_pem = if key_path.exists() {
            std::fs::read_to_string(&key_path).unwrap_or_default()
        } else {
            String::new()
        };

        let authority = Self {
            name: cert_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string(),
            certificate_pem: cert_pem,
            private_key_pem,
            metadata: CaMetadata {
                subject: format!("{:?}", parsed_cert.subject),
                issuer: format!("{:?}", parsed_cert.issuer),
                serial_number: hex::encode(&parsed_cert.serial_number),
                valid_from: parsed_cert.not_before,
                valid_until: parsed_cert.not_after,
                key_algorithm: parsed_cert.key_algorithm.clone(),
                key_size: parsed_cert.key_size,
                created_at: SystemTime::now(),
                source: CaSource::Filesystem {
                    path: cert_path.parent().unwrap_or(std::path::Path::new("/")).to_path_buf(),
                },
            },
        };

        Ok(authority)
    }

    /// Check if the certificate authority is currently valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let now = SystemTime::now();
        now >= self.metadata.valid_from && now <= self.metadata.valid_until
    }

    /// Get duration until expiry
    ///
    /// # Errors
    ///
    /// Returns `TlsError::CertificateExpired` if the certificate authority has already expired.
    pub fn expires_in(&self) -> Result<Duration, TlsError> {
        let now = SystemTime::now();
        self.metadata.valid_until.duration_since(now).map_err(|_| {
            TlsError::CertificateExpired("Certificate authority has expired".to_string())
        })
    }

    /// Check if this CA can sign certificates for the given domain
    pub fn can_sign_for_domain(&self, domain: &str) -> bool {
        use crate::tls::certificate::parsing::{parse_certificate_from_pem, verify_hostname};

        if !self.is_valid() {
            return false;
        }

        // Parse CA certificate to check constraints
        let ca_cert = match parse_certificate_from_pem(&self.certificate_pem) {
            Ok(cert) => cert,
            Err(e) => {
                tracing::error!(
                    "Failed to parse CA certificate for domain validation: {}",
                    e
                );
                return false;
            }
        };

        // Check if this is a proper CA
        if !ca_cert.is_ca {
            tracing::warn!(
                "Certificate is not marked as CA, cannot sign for domain: {}",
                domain
            );
            return false;
        }

        // Delegate to existing hostname verification logic
        // If the CA certificate itself can validate this domain, then it can sign for it
        if let Ok(()) = verify_hostname(&ca_cert, domain) {
            tracing::debug!(
                "CA can sign for domain '{}' - matches CA constraints",
                domain
            );
            true
        } else {
            tracing::warn!(
                "CA certificate cannot sign for domain '{}' - no matching constraints",
                domain
            );
            false
        }
    }
}
