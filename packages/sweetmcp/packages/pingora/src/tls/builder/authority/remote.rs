//! Remote certificate authority builder
//!
//! This module provides functionality for downloading certificate authorities
//! from remote URLs over HTTP/HTTPS with timeout and validation support.


#![allow(dead_code)]

use std::time::Duration;

use super::types::{CaMetadata, CaSource, CertificateAuthority};
use super::utils::{format_dn_hashmap, format_serial_number};

/// Builder for remote certificate authority operations
#[derive(Debug, Clone)]
pub struct AuthorityRemoteBuilder {
    pub(super) name: String,
    pub(super) url: String,
    timeout: Duration,
}

impl AuthorityRemoteBuilder {
    /// Create a new remote authority builder
    #[must_use]
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            timeout: Duration::from_secs(30), // Default timeout
        }
    }

    /// Set timeout for remote operations
    #[must_use]
    pub fn with_timeout(self, timeout: Duration) -> Self {
        Self { timeout, ..self }
    }

    /// Load certificate authority from remote URL
    pub async fn load(self) -> super::responses::CertificateAuthorityResponse {
        log::debug!("Loading CA '{}' from remote URL: {}", self.name, self.url);

        let url = match self.parse_and_validate_url() {
            Ok(u) => u,
            Err(response) => return *response,
        };

        let cert_content = match self.download_certificate_content(url).await {
            Ok(content) => content,
            Err(response) => return *response,
        };

        let parsed_cert = match Self::parse_and_validate_certificate(&cert_content) {
            Ok(cert) => cert,
            Err(response) => return *response,
        };

        if let Err(response) = self.validate_ca_certificate(&parsed_cert) {
            return *response;
        }

        let authority = self.create_certificate_authority(cert_content, &parsed_cert);

        log::info!(
            "Successfully loaded CA '{}' from remote URL (valid until: {:?})",
            self.name,
            authority.metadata.valid_until
        );

        super::responses::CertificateAuthorityResponse {
            success: true,
            authority: Some(authority),
            operation: super::responses::CaOperation::Loaded,
            issues: vec![],
            files_created: vec![],
        }
    }
    /// Parse and validate the remote URL
    fn parse_and_validate_url(
        &self,
    ) -> Result<url::Url, super::responses::BoxedCertificateAuthorityResponse> {
        url::Url::parse(&self.url).map_err(|e| {
            Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec![format!("Invalid URL: {}", e)],
                files_created: vec![],
            })
        })
    }

    /// Download certificate content from remote URL
    async fn download_certificate_content(
        &self,
        url: url::Url,
    ) -> Result<String, super::responses::BoxedCertificateAuthorityResponse> {
        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(|e| {
                Box::new(super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::LoadFailed,
                    issues: vec![format!("Failed to create HTTP client: {}", e)],
                    files_created: vec![],
                })
            })?;

        let cert_content = client
            .get(url.as_str())
            .send()
            .await
            .map_err(|e| {
                Box::new(super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::LoadFailed,
                    issues: vec![format!("Failed to fetch from remote URL: {}", e)],
                    files_created: vec![],
                })
            })?
            .text()
            .await
            .map_err(|e| {
                Box::new(super::responses::CertificateAuthorityResponse {
                    success: false,
                    authority: None,
                    operation: super::responses::CaOperation::LoadFailed,
                    issues: vec![format!("Failed to read response body: {}", e)],
                    files_created: vec![],
                })
            })?;

        if cert_content.is_empty() {
            return Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec!["No body data received".to_string()],
                files_created: vec![],
            }));
        }

        Self::validate_pem_content(&cert_content)?;
        Ok(cert_content)
    }

    /// Validate that content appears to be a PEM certificate
    fn validate_pem_content(
        cert_content: &str,
    ) -> Result<(), super::responses::BoxedCertificateAuthorityResponse> {
        if !cert_content.contains("-----BEGIN CERTIFICATE-----")
            || !cert_content.contains("-----END CERTIFICATE-----")
        {
            return Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec!["Remote content does not appear to be a PEM certificate".to_string()],
                files_created: vec![],
            }));
        }
        Ok(())
    }
    /// Parse and validate the certificate
    fn parse_and_validate_certificate(
        cert_content: &str,
    ) -> Result<
        crate::tls::types::ParsedCertificate,
        super::responses::BoxedCertificateAuthorityResponse,
    > {
        use crate::tls::certificate::parse_certificate_from_pem;

        parse_certificate_from_pem(cert_content).map_err(|e| {
            Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec![format!(
                    "Failed to parse certificate from remote URL: {}",
                    e
                )],
                files_created: vec![],
            })
        })
    }

    /// Validate that certificate is a CA and currently valid
    fn validate_ca_certificate(
        &self,
        parsed_cert: &crate::tls::types::ParsedCertificate,
    ) -> Result<(), super::responses::BoxedCertificateAuthorityResponse> {
        if !parsed_cert.is_ca {
            return Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec![format!(
                    "Certificate from '{}' is not a Certificate Authority",
                    self.url
                )],
                files_created: vec![],
            }));
        }

        let now = std::time::SystemTime::now();
        if now < parsed_cert.not_before || now > parsed_cert.not_after {
            return Err(Box::new(super::responses::CertificateAuthorityResponse {
                success: false,
                authority: None,
                operation: super::responses::CaOperation::LoadFailed,
                issues: vec![format!(
                    "Certificate Authority from '{}' has expired or is not yet valid",
                    self.url
                )],
                files_created: vec![],
            }));
        }

        Ok(())
    }

    /// Create `CertificateAuthority` object from parsed certificate
    fn create_certificate_authority(
        &self,
        cert_content: String,
        parsed_cert: &crate::tls::types::ParsedCertificate,
    ) -> CertificateAuthority {
        CertificateAuthority {
            name: self.name.clone(),
            certificate_pem: cert_content,
            private_key_pem: String::new(), // Remote CAs don't include private keys
            metadata: CaMetadata {
                subject: format_dn_hashmap(&parsed_cert.subject),
                issuer: format_dn_hashmap(&parsed_cert.issuer),
                serial_number: format_serial_number(&parsed_cert.serial_number),
                valid_from: parsed_cert.not_before,
                valid_until: parsed_cert.not_after,
                key_algorithm: parsed_cert.key_algorithm.clone(),
                key_size: parsed_cert.key_size,
                created_at: std::time::SystemTime::now(),
                source: CaSource::Remote {
                    url: self.url.clone(),
                },
            },
        }
    }
}
