//! Filesystem-based certificate authority builder
//!
//! This module provides functionality for creating and loading certificate
//! authorities from the filesystem, including certificate generation and
//! file management operations.


#![allow(dead_code)]

use std::path::PathBuf;
use std::time::SystemTime;

use super::types::{CaMetadata, CaSource, CertificateAuthority};
use super::utils::{format_dn_hashmap, format_serial_number};
use crate::tls::certificate::parse_certificate_from_pem;

/// Builder for filesystem certificate authority operations
#[derive(Debug, Clone)]
pub struct AuthorityFilesystemBuilder {
    pub(super) name: String,
    pub(super) path: PathBuf,
    pub(super) common_name: Option<String>,
    pub(super) key_size: u32,
}

impl AuthorityFilesystemBuilder {
    /// Set common name for the certificate authority
    #[must_use]
    pub fn common_name<S: Into<String>>(self, cn: S) -> Self {
        Self {
            common_name: Some(cn.into()),
            ..self
        }
    }

    /// Set key size for certificate authority creation
    #[must_use]
    pub fn key_size(self, bits: u32) -> Self {
        Self {
            key_size: bits,
            ..self
        }
    }

    /// Create a new certificate authority
    #[allow(clippy::unused_async)]
    pub async fn create(self) -> super::responses::CertificateAuthorityResponse {
        // Create output directory
        if let Err(response) = self.create_output_directory() {
            return *response;
        }

        // Setup certificate parameters and generate certificates
        let (cert_pem, key_pem, common_name, now) = match self.generate_ca_certificate() {
            Ok(result) => result,
            Err(response) => return *response,
        };

        // Save certificate files to disk
        let files_created = match self.save_certificate_files(&cert_pem, &key_pem) {
            Ok(files) => files,
            Err(response) => return *response,
        };

        // Create and return the authority object
        let authority = self.create_authority_object(cert_pem, key_pem, &common_name, now);

        super::responses::CertificateAuthorityResponse {
            success: true,
            authority: Some(authority),
            operation: super::responses::CaOperation::Created,
            issues: vec![],
            files_created,
        }
    }

    fn create_output_directory(
        &self,
    ) -> Result<(), super::responses::BoxedCertificateAuthorityResponse> {
        if let Err(e) = std::fs::create_dir_all(&self.path) {
            return Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::CreateFailed,
                issues: vec![format!("Failed to create directory: {}", e)],
                files_created: vec![],
            }));
        }
        Ok(())
    }

    fn generate_ca_certificate(
        &self,
    ) -> Result<
        (String, String, String, std::time::SystemTime),
        super::responses::BoxedCertificateAuthorityResponse,
    > {
        let mut params = Self::setup_certificate_params()?;
        let common_name = self.setup_distinguished_name(&mut params);
        let now = Self::setup_validity_period(&mut params);

        let key_pair = Self::generate_key_pair()?;
        let cert = Self::generate_self_signed_cert(&params, &key_pair)?;

        let cert_pem = cert.pem();
        let key_pem = key_pair.serialize_pem();

        Ok((cert_pem, key_pem, common_name, now))
    }

    fn setup_certificate_params()
    -> Result<rcgen::CertificateParams, super::responses::BoxedCertificateAuthorityResponse> {
        match rcgen::CertificateParams::new(vec![]) {
            Ok(mut params) => {
                params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
                Ok(params)
            }
            Err(e) => Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::CreateFailed,
                issues: vec![format!("Failed to create certificate parameters: {}", e)],
                files_created: vec![],
            })),
        }
    }

    fn setup_distinguished_name(&self, params: &mut rcgen::CertificateParams) -> String {
        use rcgen::{DistinguishedName, DnType};

        let mut distinguished_name = DistinguishedName::new();
        let common_name = self.common_name.as_ref().unwrap_or(&self.name).clone();
        distinguished_name.push(DnType::CommonName, &common_name);
        params.distinguished_name = distinguished_name;
        common_name
    }

    fn setup_validity_period(params: &mut rcgen::CertificateParams) -> std::time::SystemTime {
        let now = SystemTime::now();
        params.not_before = now.into();
        params.not_after = (now + std::time::Duration::from_secs(365 * 24 * 60 * 60)).into(); // 1 year
        now
    }

    fn generate_key_pair()
    -> Result<rcgen::KeyPair, super::responses::BoxedCertificateAuthorityResponse> {
        match rcgen::KeyPair::generate() {
            Ok(key_pair) => Ok(key_pair),
            Err(e) => Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::CreateFailed,
                issues: vec![format!("Failed to generate key pair: {}", e)],
                files_created: vec![],
            })),
        }
    }

    fn generate_self_signed_cert(
        params: &rcgen::CertificateParams,
        key_pair: &rcgen::KeyPair,
    ) -> Result<rcgen::Certificate, super::responses::BoxedCertificateAuthorityResponse> {
        match params.self_signed(key_pair) {
            Ok(cert) => Ok(cert),
            Err(e) => Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::CreateFailed,
                issues: vec![format!("Failed to generate self-signed certificate: {}", e)],
                files_created: vec![],
            })),
        }
    }

    fn save_certificate_files(
        &self,
        cert_pem: &str,
        key_pem: &str,
    ) -> Result<Vec<PathBuf>, super::responses::BoxedCertificateAuthorityResponse> {
        let cert_path = self.path.join(format!("{}.crt", self.name));
        let key_path = self.path.join(format!("{}.key", self.name));

        if let Err(e) = std::fs::write(&cert_path, cert_pem) {
            return Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::CreateFailed,
                issues: vec![format!("Failed to write certificate: {}", e)],
                files_created: vec![],
            }));
        }

        if let Err(e) = std::fs::write(&key_path, key_pem) {
            return Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::CreateFailed,
                issues: vec![format!("Failed to write private key: {}", e)],
                files_created: vec![cert_path.clone()],
            }));
        }

        Ok(vec![cert_path.clone(), key_path.clone()])
    }

    fn create_authority_object(
        &self,
        cert_pem: String,
        key_pem: String,
        common_name: &str,
        now: SystemTime,
    ) -> CertificateAuthority {
        CertificateAuthority {
            name: self.name.clone(),
            certificate_pem: cert_pem,
            private_key_pem: key_pem,
            metadata: CaMetadata {
                subject: format!("CN={common_name}"),
                issuer: format!("CN={common_name}"), // Self-signed
                serial_number: "01".to_string(),
                valid_from: now,
                valid_until: now + std::time::Duration::from_secs(365 * 24 * 60 * 60), // 1 year
                key_algorithm: "RSA".to_string(),
                key_size: Some(self.key_size),
                created_at: now,
                source: CaSource::Generated,
            },
        }
    }

    /// Load existing certificate authority from filesystem
    #[allow(clippy::unused_async)]
    pub async fn load(self) -> super::responses::CertificateAuthorityResponse {
        let cert_path = self.path.join(format!("{}.crt", self.name));
        let key_path = self.path.join(format!("{}.key", self.name));

        // Check if files exist
        if !cert_path.exists() {
            return super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec![format!(
                    "Certificate file not found: {}",
                    cert_path.display()
                )],
                files_created: vec![],
            };
        }

        if !key_path.exists() {
            return super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec![format!(
                    "Private key file not found: {}",
                    key_path.display()
                )],
                files_created: vec![],
            };
        }

        // Read certificate and key files
        let cert_pem = match std::fs::read_to_string(&cert_path) {
            Ok(content) => content,
            Err(e) => {
                return super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::LoadFailed,
                    issues: vec![format!("Failed to read certificate: {}", e)],
                    files_created: vec![],
                };
            }
        };

        let key_pem = match std::fs::read_to_string(&key_path) {
            Ok(content) => content,
            Err(e) => {
                return super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::LoadFailed,
                    issues: vec![format!("Failed to read private key: {}", e)],
                    files_created: vec![],
                };
            }
        };

        // Parse certificate to extract metadata
        let parsed_cert = match parse_certificate_from_pem(&cert_pem) {
            Ok(cert) => cert,
            Err(e) => {
                return super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::LoadFailed,
                    issues: vec![format!("Failed to parse certificate: {}", e)],
                    files_created: vec![],
                };
            }
        };

        let authority = CertificateAuthority {
            name: self.name.clone(),
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
                source: CaSource::Filesystem {
                    path: self.path.clone(),
                },
            },
        };

        super::responses::CertificateAuthorityResponse {
            success: true,
            authority: Some(authority),
            operation: super::responses::CaOperation::Loaded,
            issues: vec![],
            files_created: vec![],
        }
    }
}
