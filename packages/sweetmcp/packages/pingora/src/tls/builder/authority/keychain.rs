//! Keychain-based certificate authority builder
//!
//! This module provides functionality for loading and storing certificate
//! authorities in the system keychain using the keyring crate.


#![allow(dead_code)]

use super::types::{CaMetadata, CaSource, CertificateAuthority};
use super::utils::{format_dn_hashmap, format_serial_number};
use crate::tls::certificate::parse_certificate_from_pem;
use crate::tls::types::ParsedCertificate;

/// Builder for keychain certificate authority operations
#[derive(Debug, Clone)]
pub struct AuthorityKeychainBuilder {
    pub(super) name: String,
}

impl AuthorityKeychainBuilder {
    /// Load certificate authority from system keychain
    #[allow(clippy::unused_async)]
    pub async fn load(self) -> super::responses::CertificateAuthorityResponse {
        log::debug!("Loading CA '{}' from system keychain", self.name);

        let service_name = "fluent-ai-http3";
        let cert_key_id = format!("ca-cert-{}", self.name);
        let private_key_id = format!("ca-key-{}", self.name);

        // Create keychain entries
        let cert_entry = match Self::create_keychain_entry(service_name, &cert_key_id) {
            Ok(entry) => entry,
            Err(msg) => return Self::load_failed_response(msg),
        };

        let key_entry = match Self::create_keychain_entry(service_name, &private_key_id) {
            Ok(entry) => entry,
            Err(msg) => return Self::load_failed_response(msg),
        };
        // Retrieve certificate and private key from keychain
        let cert_pem = match Self::retrieve_from_keychain(
            cert_entry,
            &format!("Certificate for CA '{}'", self.name),
        ) {
            Ok(pem) => pem,
            Err(msg) => return Self::load_failed_response(msg),
        };

        let key_pem = match Self::retrieve_from_keychain(
            key_entry,
            &format!("Private key for CA '{}'", self.name),
        ) {
            Ok(pem) => pem,
            Err(msg) => return Self::load_failed_response(msg),
        };

        // Parse and validate certificate
        let parsed_cert = match parse_certificate_from_pem(&cert_pem) {
            Ok(cert) => cert,
            Err(e) => {
                return Self::load_failed_response(format!(
                    "Failed to parse certificate from keychain: {e}"
                ));
            }
        };

        if let Err(msg) = Self::validate_ca_certificate(&parsed_cert, &self.name) {
            return Self::load_failed_response(msg);
        }

        // Create the CA object with metadata
        let authority =
            Self::build_certificate_authority(self.name.clone(), cert_pem, key_pem, &parsed_cert);

        log::info!(
            "Successfully loaded CA '{}' from keychain (valid until: {:?})",
            self.name,
            parsed_cert.not_after
        );

        super::responses::CertificateAuthorityResponse {
            success: true,
            authority: Some(authority),
            operation: super::responses::CaOperation::Loaded,
            issues: vec![],
            files_created: vec![],
        }
    }
    /// Create keychain entry with error handling
    fn create_keychain_entry(service_name: &str, key_id: &str) -> Result<keyring::Entry, String> {
        keyring::Entry::new(service_name, key_id)
            .map_err(|e| format!("Failed to create keychain entry for '{key_id}': {e}"))
    }

    /// Retrieve data from keychain with threaded access
    fn retrieve_from_keychain(entry: keyring::Entry, description: &str) -> Result<String, String> {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = entry.get_password();
            let _ = tx.send(result);
        });

        match rx.recv() {
            Ok(Ok(pem)) => Ok(pem),
            Ok(Err(keyring::Error::NoEntry)) => Err(format!("{description} not found in keychain")),
            Ok(Err(e)) => Err(format!("Failed to retrieve {description}: {e}")),
            Err(e) => Err(format!("Keychain operation failed: {e}")),
        }
    }

    /// Validate that certificate is a valid CA
    fn validate_ca_certificate(parsed_cert: &ParsedCertificate, name: &str) -> Result<(), String> {
        use std::time::SystemTime;

        if !parsed_cert.is_ca {
            return Err(format!(
                "Certificate for '{name}' is not a Certificate Authority"
            ));
        }

        let now = SystemTime::now();
        if now < parsed_cert.not_before || now > parsed_cert.not_after {
            return Err(format!(
                "Certificate Authority '{name}' has expired or is not yet valid"
            ));
        }

        Ok(())
    }
    /// Build `CertificateAuthority` object from parsed certificate
    fn build_certificate_authority(
        name: String,
        cert_pem: String,
        key_pem: String,
        parsed_cert: &ParsedCertificate,
    ) -> CertificateAuthority {
        use std::time::SystemTime;

        CertificateAuthority {
            name,
            certificate_pem: cert_pem,
            private_key_pem: key_pem,
            metadata: CaMetadata {
                subject: format_dn_hashmap(&parsed_cert.subject),
                issuer: format_dn_hashmap(&parsed_cert.issuer),
                serial_number: format_serial_number(&parsed_cert.serial_number),
                valid_from: parsed_cert.not_before,
                valid_until: parsed_cert.not_after,
                key_algorithm: parsed_cert.key_algorithm.clone(),
                key_size: parsed_cert.key_size,
                created_at: SystemTime::now(),
                source: CaSource::Keychain,
            },
        }
    }

    /// Create a load failed response with consistent formatting
    fn load_failed_response(issue: String) -> super::responses::CertificateAuthorityResponse {
        super::responses::CertificateAuthorityResponse {
            success: false,
            authority: None,
            operation: super::responses::CaOperation::LoadFailed,
            issues: vec![issue],
            files_created: vec![],
        }
    }
    /// Store certificate authority in system keychain
    #[allow(clippy::unused_async)]
    pub async fn store(
        &self,
        authority: &CertificateAuthority,
    ) -> super::responses::CertificateAuthorityResponse {
        log::debug!("Storing CA '{}' to system keychain", authority.name);

        // Use fluent-ai service pattern for keychain access
        let service_name = "fluent-ai-http3";
        let cert_key_id = format!("ca-cert-{}", authority.name);
        let private_key_id = format!("ca-key-{}", authority.name);

        // Store certificate in keychain
        let cert_entry = match keyring::Entry::new(service_name, &cert_key_id) {
            Ok(entry) => entry,
            Err(e) => {
                return super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::StoreFailed,
                    issues: vec![format!(
                        "Failed to create keychain entry for certificate: {}",
                        e
                    )],
                    files_created: vec![],
                };
            }
        };

        let cert_pem = authority.certificate_pem.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = cert_entry.set_password(&cert_pem);
            let _ = tx.send(result);
        });

        if let Err(e) = rx.recv().unwrap_or_else(|_e| Err(keyring::Error::NoEntry)) {
            return super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::StoreFailed,
                issues: vec![format!("Failed to store certificate in keychain: {}", e)],
                files_created: vec![],
            };
        }
        // Store private key in keychain
        let key_entry = match keyring::Entry::new(service_name, &private_key_id) {
            Ok(entry) => entry,
            Err(e) => {
                return super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::StoreFailed,
                    issues: vec![format!(
                        "Failed to create keychain entry for private key: {}",
                        e
                    )],
                    files_created: vec![],
                };
            }
        };

        let key_pem = authority.private_key_pem.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let result = key_entry.set_password(&key_pem);
            let _ = tx.send(result);
        });

        if let Err(e) = rx.recv().unwrap_or_else(|_e| Err(keyring::Error::NoEntry)) {
            return super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::StoreFailed,
                issues: vec![format!("Failed to store private key in keychain: {}", e)],
                files_created: vec![],
            };
        }

        log::info!("Successfully stored CA '{}' in keychain", authority.name);

        super::responses::CertificateAuthorityResponse {
            success: true,
            authority: Some(authority.clone()),
            operation: super::responses::CaOperation::Stored,
            issues: vec![],
            files_created: vec![],
        }
    }
}
