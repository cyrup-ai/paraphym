//! TLS configuration builders and management

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::{ClientConfig, RootCertStore, ServerConfig};
use tracing::info;

use super::certificate::{parse_certificate_from_pem, verify_peer_certificate};
use super::crl_cache;
use super::errors::TlsError;
use super::ocsp::OcspCache;

/// Production TLS manager with comprehensive certificate lifecycle management
pub struct TlsManager {
    #[allow(dead_code)]
    cert_dir: std::path::PathBuf,
    ca_cert: CertificateDer<'static>,
    #[allow(dead_code)]
    ca_key: PrivatePkcs8KeyDer<'static>,
    server_cert: CertificateDer<'static>,
    server_key: PrivatePkcs8KeyDer<'static>,
    ocsp_cache: OcspCache,
    crl_cache: crl_cache::CrlCache,
}

impl TlsManager {
    /// Create a new TLS manager with self-signed certificates
    pub async fn new(cert_dir: std::path::PathBuf) -> Result<Self> {
        let (ca_cert, ca_key, server_cert, server_key, ocsp_cache, crl_cache) =
            super::certificate::new(cert_dir.clone()).await?;

        let tls_manager = Self {
            cert_dir,
            ca_cert,
            ca_key,
            server_cert,
            server_key,
            ocsp_cache,
            crl_cache,
        };

        // Start cache cleanup tasks
        tls_manager.start_ocsp_cleanup_task();
        tls_manager.start_crl_cleanup_task();

        Ok(tls_manager)
    }

    /// Get server TLS configuration
    pub fn server_config(&self) -> Result<ServerConfig> {
        let mut root_store = RootCertStore::empty();
        root_store.add(self.ca_cert.clone())?;

        let config = ServerConfig::builder()
            .with_client_cert_verifier(
                rustls::server::WebPkiClientVerifier::builder(Arc::new(root_store)).build()?,
            )
            .with_single_cert(
                vec![self.server_cert.clone()],
                PrivateKeyDer::Pkcs8(self.server_key.clone_key()),
            )?;

        Ok(config)
    }

    /// Get client TLS configuration
    pub fn client_config(&self) -> Result<ClientConfig> {
        let mut root_store = RootCertStore::empty();
        root_store.add(self.ca_cert.clone())?;

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_client_auth_cert(
                vec![self.server_cert.clone()],
                PrivateKeyDer::Pkcs8(self.server_key.clone_key()),
            )?;

        Ok(config)
    }

    /// Start periodic OCSP cache cleanup task
    pub fn start_ocsp_cleanup_task(&self) {
        let ocsp_cache = self.ocsp_cache.clone();
        tokio::spawn(async move {
            let mut cleanup_interval = tokio::time::interval(Duration::from_secs(3600)); // Cleanup every hour

            loop {
                cleanup_interval.tick().await;
                ocsp_cache.cleanup_cache();
            }
        });
    }

    /// Start periodic CRL cache cleanup task
    pub fn start_crl_cleanup_task(&self) {
        let crl_cache = self.crl_cache.clone();
        tokio::spawn(async move {
            let mut cleanup_interval = tokio::time::interval(Duration::from_secs(6 * 3600)); // Cleanup every 6 hours

            loop {
                cleanup_interval.tick().await;
                crl_cache.cleanup_cache();
            }
        });
    }

    /// Validate certificate using OCSP (Online Certificate Status Protocol)
    pub async fn validate_certificate_ocsp(
        &self,
        cert_pem: &str,
        issuer_cert_pem: Option<&str>,
    ) -> Result<(), TlsError> {
        let parsed_cert = parse_certificate_from_pem(cert_pem)?;

        // Parse issuer certificate if provided
        let issuer_cert = if let Some(issuer_pem) = issuer_cert_pem {
            Some(parse_certificate_from_pem(issuer_pem)?)
        } else {
            None
        };

        match self
            .ocsp_cache
            .check_certificate(&parsed_cert, issuer_cert.as_ref())
            .await
        {
            Ok(super::ocsp::OcspStatus::Good) => {
                tracing::info!("OCSP validation successful: certificate is valid");
                Ok(())
            }
            Ok(super::ocsp::OcspStatus::Revoked) => Err(TlsError::OcspValidation(
                "Certificate has been revoked".to_string(),
            )),
            Ok(super::ocsp::OcspStatus::Unknown) => {
                tracing::warn!("OCSP status unknown, proceeding with validation");
                Ok(())
            }
            Err(e) => {
                tracing::warn!("OCSP validation failed: {}, proceeding without OCSP", e);
                Ok(())
            }
        }
    }

    /// Validate certificate using CRL (Certificate Revocation List)
    pub async fn validate_certificate_crl(&self, cert_pem: &str) -> Result<(), TlsError> {
        let parsed_cert = parse_certificate_from_pem(cert_pem)?;

        match self
            .crl_cache
            .check_certificate_revocation(&parsed_cert)
            .await
        {
            Ok(false) => {
                tracing::info!("CRL validation successful: certificate is not revoked");
                Ok(())
            }
            Ok(true) => Err(TlsError::CrlValidation(
                "Certificate has been revoked according to CRL".to_string(),
            )),
            Err(e) => {
                tracing::warn!("CRL validation failed: {}, proceeding without CRL", e);
                Ok(())
            }
        }
    }

    /// Validate certificate chain to root CA
    pub async fn validate_certificate_chain(&self, cert_chain_pem: &str) -> Result<(), TlsError> {
        super::certificate_validation::validate_certificate_chain(cert_chain_pem, &self.ca_cert)
            .await
    }

    /// Verify peer certificate against expected hostname
    pub fn verify_peer_certificate(
        cert_pem: &str,
        expected_hostname: &str,
    ) -> Result<(), TlsError> {
        verify_peer_certificate(cert_pem, expected_hostname)
    }

    /// Verify peer certificate with OCSP validation
    pub async fn verify_peer_certificate_with_ocsp(
        &self,
        cert_pem: &str,
        expected_hostname: &str,
        issuer_cert_pem: Option<&str>,
    ) -> Result<(), TlsError> {
        // Perform standard certificate validation
        Self::verify_peer_certificate(cert_pem, expected_hostname)?;

        // Additional OCSP validation
        self.validate_certificate_ocsp(cert_pem, issuer_cert_pem)
            .await?;

        info!(
            "Successfully verified peer certificate with OCSP for hostname: {}",
            expected_hostname
        );
        Ok(())
    }

    /// Verify peer certificate with comprehensive revocation checking (OCSP + CRL + Chain)
    pub async fn verify_peer_certificate_comprehensive(
        &self,
        cert_pem: &str,
        expected_hostname: &str,
        full_chain_pem: Option<&str>,
    ) -> Result<(), TlsError> {
        super::certificate_validation::verify_peer_certificate_comprehensive(
            self,
            cert_pem,
            expected_hostname,
            full_chain_pem,
            &self.ca_cert,
        )
        .await
    }
}
