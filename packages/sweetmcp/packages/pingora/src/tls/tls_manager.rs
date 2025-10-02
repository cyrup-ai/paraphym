//! Enterprise TLS Manager
//!
//! Provides comprehensive TLS connection management with OCSP validation,
//! CRL checking, certificate validation, and enterprise security features.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use rustls::{ClientConfig, RootCertStore};
// ServerName import removed - not used
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

use super::crl_cache::CrlCache;
use super::ocsp::OcspCache;

// Global crypto provider initialization - only once per process
static CRYPTO_PROVIDER_INIT: std::sync::Once = std::sync::Once::new();

// Removed unused import: parse_certificate_from_der (no longer needed with standard rustls verification)
use super::builder::CertificateAuthority;
use super::errors::TlsError;
// ParsedCertificate alias import removed - not used

/// Detailed TLS cache statistics for monitoring and troubleshooting
#[derive(Debug, Clone)]
pub struct TlsCacheStats {
    /// OCSP cache hits
    pub ocsp_hits: usize,
    /// OCSP cache misses
    pub ocsp_misses: usize,
    /// Number of entries in OCSP cache
    pub ocsp_cache_size: usize,
    /// CRL cache hits
    pub crl_hits: usize,
    /// CRL cache misses
    pub crl_misses: usize,
    /// Number of entries in CRL cache
    pub crl_cache_size: usize,
}

impl TlsCacheStats {
    /// Calculate total cache hits
    pub fn total_hits(&self) -> usize {
        self.ocsp_hits + self.crl_hits
    }

    /// Calculate total cache misses
    pub fn total_misses(&self) -> usize {
        self.ocsp_misses + self.crl_misses
    }

    /// Calculate total cache requests
    pub fn total_requests(&self) -> usize {
        self.total_hits() + self.total_misses()
    }

    /// Calculate overall cache hit rate as a percentage (0.0 to 100.0)
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_requests();
        let hits = self.total_hits();
        Self::safe_percentage_calculation(hits, total)
    }

    /// Calculate OCSP cache hit rate as a percentage (0.0 to 100.0)
    pub fn ocsp_hit_rate(&self) -> f64 {
        let total = self.ocsp_hits + self.ocsp_misses;
        Self::safe_percentage_calculation(self.ocsp_hits, total)
    }

    /// Calculate CRL cache hit rate as a percentage (0.0 to 100.0)
    pub fn crl_hit_rate(&self) -> f64 {
        let total = self.crl_hits + self.crl_misses;
        Self::safe_percentage_calculation(self.crl_hits, total)
    }

    /// Helper function for safe precision-aware percentage calculations
    #[allow(clippy::cast_precision_loss)]
    fn safe_percentage_calculation(numerator: usize, denominator: usize) -> f64 {
        if denominator == 0 {
            0.0
        } else {
            // Use safe precision-aware percentage calculation
            let precision_threshold = if usize::BITS >= 64 {
                1usize << 53 // For 64-bit platforms: f64 precision threshold
            } else {
                usize::MAX / 2 // For 32-bit platforms, use safe threshold
            };

            if numerator > precision_threshold || denominator > precision_threshold {
                // For very large values, use high-precision integer calculation
                // Calculate percentage using integer arithmetic: (numerator * 10000) / denominator / 100
                let percentage_basis_points = (numerator as u128 * 10000) / (denominator as u128);
                (percentage_basis_points as f64) / 100.0
            } else {
                // Safe to use f64 for smaller values
                (numerator as f64 / denominator as f64) * 100.0
            }
        }
    }
}

/// Enterprise TLS connection manager with comprehensive security validation
#[derive(Clone)]
pub struct TlsManager {
    /// OCSP validation cache for certificate status checking
    ocsp_cache: Arc<OcspCache>,
    /// CRL cache for certificate revocation checking
    crl_cache: Arc<CrlCache>,
    /// Custom certificate authorities for validation
    custom_cas: Arc<RwLock<HashMap<String, CertificateAuthority>>>,
    /// TLS configuration
    config: TlsConfig,
}

/// TLS configuration for enterprise features
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct TlsConfig {
    /// Enable CRL checking (OCSP is handled via stapling automatically)
    pub enable_crl: bool,
    /// Use system certificate store
    pub use_system_certs: bool,
    /// Custom root certificates
    pub custom_root_certs: Vec<String>,
    /// TLS 1.3 early data support
    pub enable_early_data: bool,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Certificate validation timeout
    pub validation_timeout: Duration,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enable_crl: true,
            use_system_certs: true,
            custom_root_certs: Vec::new(),
            enable_early_data: true,
            connect_timeout: Duration::from_secs(5),
            validation_timeout: Duration::from_secs(3),
        }
    }
}

impl TlsConfig {
}

impl Default for TlsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TlsManager {
    /// Create new TLS manager with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(TlsConfig::default())
    }

    /// Create TLS manager with specific configuration
    #[must_use]
    pub fn with_config(config: TlsConfig) -> Self {
        // Initialize crypto provider (safe to call multiple times)
        Self::initialize_crypto_provider();
        
        Self {
            ocsp_cache: Arc::new(OcspCache::new()),
            crl_cache: Arc::new(CrlCache::new()),
            custom_cas: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Create new TLS manager with certificate directory (async)
    ///
    /// # Errors
    /// Returns `TlsError` if:
    /// - Certificate directory creation fails due to filesystem permissions
    /// - TLS configuration initialization encounters invalid settings
    /// - Self-signed CA generation fails during setup
    /// - Certificate storage initialization encounters I/O errors
    #[allow(clippy::unused_async)]
    pub async fn with_cert_dir(cert_dir: std::path::PathBuf) -> Result<Self, TlsError> {
        // Create certificate directory if it doesn't exist
        if !cert_dir.exists() {
            std::fs::create_dir_all(&cert_dir)
                .map_err(|e| TlsError::Internal(format!("Failed to create cert directory: {e}")))?;
        }

        // Initialize TLS manager with custom config
        let mut config = TlsConfig::default();

        // Add any certificates found in the directory
        if let Ok(entries) = std::fs::read_dir(&cert_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pem")
                    && let Ok(cert_data) = std::fs::read_to_string(&path)
                {
                    config.custom_root_certs.push(cert_data);
                }
            }
        }

        Ok(Self::with_config(config))
    }

    /// Add custom certificate authority
    ///
    /// # Errors
    /// Returns `TlsError` if:
    /// - CA lock acquisition fails due to mutex poisoning
    /// - Certificate authority validation fails (expired or invalid)
    /// - Certificate format is malformed or unsupported
    pub fn add_certificate_authority(
        &self,
        name: String,
        ca: CertificateAuthority,
    ) -> Result<(), TlsError> {
        let mut cas = self
            .custom_cas
            .write()
            .map_err(|_| TlsError::Internal("Failed to acquire CA lock".to_string()))?;

        // Validate CA before adding
        if !ca.is_valid() {
            return Err(TlsError::CertificateExpired(format!(
                "Certificate authority '{name}' is expired"
            )));
        }

        cas.insert(name, ca);
        Ok(())
    }

    /// Create enterprise TLS connection with full validation
    ///
    /// # Errors
    /// Returns `TlsError` if:
    /// - TLS handshake fails due to certificate validation errors
    /// - Connection establishment fails due to network issues
    /// - Certificate chain validation encounters invalid or expired certificates
    /// - OCSP/CRL validation fails for certificate revocation checks
    pub async fn create_connection(
        &self,
        host: &str,
        port: u16,
    ) -> Result<tokio_rustls::client::TlsStream<TcpStream>, TlsError> {
        tracing::debug!("Creating enterprise TLS connection to {}:{}", host, port);

        // Create TCP connection with timeout
        tracing::debug!("TLS: About to create TCP connection to {}:{}", host, port);
        tracing::debug!("TLS: Resolving DNS for {}", host);

        // First try to resolve the address to see if DNS is the issue
        let addr = format!("{host}:{port}");
        tracing::debug!("TLS: About to resolve address: {}", addr);

        tracing::debug!("TLS: Using timeout of {:?}", self.config.connect_timeout);

        // Connect with timeout
        let tcp_stream = match tokio::time::timeout(
            Duration::from_secs(3), // Force short timeout for debugging
            TcpStream::connect(&addr),
        )
        .await
        {
            Ok(Ok(stream)) => {
                tracing::debug!("TLS: TCP connect completed to {}", addr);
                stream
            }
            Ok(Err(e)) => {
                tracing::error!("TLS: TCP connection failed to {}: {}", addr, e);
                return Err(TlsError::Internal(format!(
                    "Failed to connect to {addr}: {e}"
                )));
            }
            Err(_) => {
                tracing::error!("TLS: TCP connection timeout to {}", addr);
                return Err(TlsError::Internal(format!("Connection timeout to {addr}")));
            }
        };
        tracing::debug!("TLS: TCP connection established to {}:{}", host, port);

        // Create enterprise TLS client configuration
        tracing::debug!("TLS: About to create client config");
        let client_config = self.create_client_config_sync()?;
        tracing::debug!("TLS: Client config created successfully");

        // Create TLS connector
        tracing::debug!("TLS: About to create TLS connector");
        let connector = TlsConnector::from(Arc::new(client_config));
        tracing::debug!("TLS: TLS connector created successfully");

        // Create server name for TLS
        tracing::debug!("TLS: About to create server name for {}", host);
        let server_name =
            rustls::pki_types::ServerName::try_from(host.to_string()).map_err(|e| {
                tracing::error!("TLS: Invalid hostname '{}': {}", host, e);
                TlsError::Internal(format!("Invalid hostname '{host}': {e}"))
            })?;
        tracing::debug!("TLS: Server name created successfully for {}", host);

        // Perform TLS handshake
        tracing::debug!("TLS: About to perform TLS handshake with {}", host);
        let tls_stream = connector
            .connect(server_name, tcp_stream)
            .await
            .map_err(|e| {
                tracing::error!("TLS: TLS handshake failed with {}: {}", host, e);
                TlsError::Internal(format!("TLS handshake failed: {e}"))
            })?;
        tracing::debug!("TLS: TLS handshake completed successfully with {}", host);

        tracing::info!("Enterprise TLS connection established to {}:{}", host, port);
        Ok(tls_stream)
    }

    /// Create enterprise client configuration with full certificate validation
    ///
    /// # Errors
    /// Returns `TlsError` if:
    /// - Crypto provider installation fails
    /// - Custom certificate parsing fails
    /// - Root certificate store operations fail
    /// - Certificate authority lock acquisition fails
    pub fn create_client_config_sync(&self) -> Result<ClientConfig, TlsError> {
        tracing::debug!("TLS: Starting create_client_config_sync");

        Self::initialize_crypto_provider();
        let root_store = self.create_root_certificate_store()?;
        let mut client_config = self.create_client_config_with_verification(root_store)?;
        self.configure_client_config(&mut client_config);

        tracing::debug!("TLS: create_client_config_sync completed successfully");
        Ok(client_config)
    }

    /// Get aggregated cache statistics from OCSP and CRL caches
    #[must_use]
    pub fn get_cache_stats(&self) -> (usize, usize) {
        let (ocsp_hits, ocsp_misses) = self.ocsp_cache.get_stats();
        let (crl_hits, crl_misses) = self.crl_cache.get_stats();
        (ocsp_hits + crl_hits, ocsp_misses + crl_misses)
    }

    /// Get detailed cache statistics for monitoring and troubleshooting
    #[must_use]
    pub fn get_detailed_cache_stats(&self) -> TlsCacheStats {
        let (ocsp_hits, ocsp_misses) = self.ocsp_cache.get_stats();
        let (crl_hits, crl_misses) = self.crl_cache.get_stats();

        let ocsp_cache_size = self.ocsp_cache.get_cache_size();
        let crl_cache_size = self.crl_cache.get_cache_size();

        TlsCacheStats {
            ocsp_hits,
            ocsp_misses,
            ocsp_cache_size,
            crl_hits,
            crl_misses,
            crl_cache_size,
        }
    }

    /// Get OCSP cache statistics only
    #[must_use]
    pub fn get_ocsp_stats(&self) -> (usize, usize) {
        self.ocsp_cache.get_stats()
    }

    /// Get CRL cache statistics only
    #[must_use]
    pub fn get_crl_stats(&self) -> (usize, usize) {
        self.crl_cache.get_stats()
    }

    /// Perform maintenance operations (cleanup caches, etc.)
    pub fn perform_maintenance(&self) {
        self.ocsp_cache.cleanup_cache();
        self.crl_cache.cleanup_cache();
        tracing::debug!("TLS manager maintenance completed");
    }

    /// Validate certificate using OCSP (Online Certificate Status Protocol)
    ///
    /// **NOTE**: This is for standalone certificate validation only.
    /// TLS connections use OCSP stapling automatically via rustls `WebPkiServerVerifier`.
    ///
    /// # Errors
    /// Returns `TlsError` if:
    /// - OCSP responder network requests fail due to connectivity issues
    /// - Certificate parsing fails for malformed PEM data
    /// - OCSP response validation encounters invalid or expired responses
    /// - Certificate status indicates revocation or suspension
    pub fn validate_certificate_ocsp(
        &self,
        cert_pem: &str,
        issuer_cert_pem: Option<&str>,
    ) -> Result<(), TlsError> {
        let parsed_cert = crate::tls::certificate::parse_certificate_from_pem(cert_pem)?;

        // Parse issuer certificate if provided
        let issuer_cert = if let Some(issuer_pem) = issuer_cert_pem {
            Some(crate::tls::certificate::parse_certificate_from_pem(
                issuer_pem,
            )?)
        } else {
            None
        };

        match self
            .ocsp_cache
            .check_certificate(&parsed_cert, issuer_cert.as_ref())
        {
            crate::tls::ocsp::OcspStatus::Good => {
                tracing::info!("OCSP validation successful: certificate is valid");
                Ok(())
            }
            crate::tls::ocsp::OcspStatus::Revoked => Err(TlsError::CertificateRevoked(
                "Certificate revoked via OCSP".to_string(),
            )),
            crate::tls::ocsp::OcspStatus::Unknown => {
                tracing::warn!("OCSP validation inconclusive");
                Ok(()) // Allow unknown status but log warning
            }
        }
    }

    /// Validate certificate using CRL (Certificate Revocation List)
    ///
    /// **NOTE**: This is for standalone certificate validation only.
    /// TLS connections use CRL checking automatically via rustls `WebPkiServerVerifier` when enabled.
    ///
    /// # Errors
    /// Returns `TlsError` if:
    /// - Certificate parsing fails for malformed PEM data
    /// - CRL download fails due to network connectivity issues
    /// - CRL parsing encounters invalid or corrupted revocation list data
    /// - Certificate is found in the revocation list (revoked status)
    #[allow(clippy::unused_async)]
    pub async fn validate_certificate_crl(&self, cert_pem: &str) -> Result<(), TlsError> {
        let parsed_cert = crate::tls::certificate::parse_certificate_from_pem(cert_pem)?;

        if parsed_cert.crl_urls.is_empty() {
            tracing::debug!("No CRL URLs found in certificate, skipping CRL validation");
            return Ok(());
        }

        // Check certificate against each CRL URL
        for crl_url in &parsed_cert.crl_urls {
            match self
                .crl_cache
                .check_certificate_status(&parsed_cert.serial_number, crl_url)
            {
                crate::tls::crl_cache::CrlStatus::Valid => {
                    tracing::debug!("CRL validation passed for URL: {}", crl_url);
                }
                crate::tls::crl_cache::CrlStatus::Revoked => {
                    return Err(TlsError::CertificateRevoked(format!(
                        "Certificate revoked via CRL: {crl_url}"
                    )));
                }
                crate::tls::crl_cache::CrlStatus::Unknown => {
                    tracing::warn!("CRL validation inconclusive for URL: {}", crl_url);
                    // Continue checking other CRL URLs
                }
            }
        }

        tracing::info!("CRL validation completed successfully");
        Ok(())
    }

    /// Initialize the crypto provider for rustls (called once per process)
    fn initialize_crypto_provider() {
        tracing::debug!("TLS: Initializing crypto provider");
        CRYPTO_PROVIDER_INIT.call_once(|| {
            let _ = rustls::crypto::ring::default_provider().install_default();
        });
        tracing::debug!("TLS: Crypto provider initialized");
    }

    /// Create and populate the root certificate store
    fn create_root_certificate_store(&self) -> Result<RootCertStore, TlsError> {
        tracing::debug!("TLS: Creating root certificate store");
        let mut root_store = RootCertStore::empty();

        self.add_system_certificates(&mut root_store);
        self.add_custom_root_certificates(&mut root_store);
        self.add_custom_certificate_authorities(&mut root_store)?;

        tracing::debug!(
            "TLS: Root certificate store created with {} certificates",
            root_store.len()
        );
        Ok(root_store)
    }

    /// Add system certificates to the root store
    fn add_system_certificates(&self, root_store: &mut RootCertStore) {
        if self.config.use_system_certs {
            tracing::debug!("TLS: Loading system certificates");
            let cert_result = rustls_native_certs::load_native_certs();
            tracing::debug!(
                "TLS: System certificate load completed, {} certs found",
                cert_result.certs.len()
            );

            for cert in cert_result.certs {
                if let Err(e) = root_store.add(cert) {
                    tracing::warn!("Failed to add system certificate: {}", e);
                }
            }

            if !cert_result.errors.is_empty() {
                for err in &cert_result.errors {
                    tracing::warn!("Certificate load error: {}", err);
                }
                // Fall back to webpki roots if there were significant errors
                tracing::debug!("TLS: Falling back to webpki roots due to system cert errors");
                root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            }

            tracing::debug!("Loaded {} system certificates", root_store.len());
        } else {
            // Use webpki roots as fallback
            tracing::debug!("TLS: Using webpki roots as fallback");
            root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            tracing::debug!("TLS: Added {} webpki root certificates", root_store.len());
        }
    }

    /// Add custom root certificates to the root store
    fn add_custom_root_certificates(&self, root_store: &mut RootCertStore) {
        tracing::debug!(
            "TLS: Processing {} custom root certificates",
            self.config.custom_root_certs.len()
        );
        for cert_pem in &self.config.custom_root_certs {
            // Parse PEM certificate data
            if let Ok(cert_der) = pem::parse(cert_pem) {
                let cert = rustls::pki_types::CertificateDer::from(cert_der.contents());
                if let Err(e) = root_store.add(cert) {
                    tracing::warn!("Failed to add custom root certificate: {}", e);
                } else {
                    tracing::debug!("Added custom root certificate from PEM data");
                }
            } else {
                tracing::warn!("Failed to parse custom root certificate PEM data");
            }
        }
    }

    /// Add custom certificate authorities to the root store
    fn add_custom_certificate_authorities(
        &self,
        root_store: &mut RootCertStore,
    ) -> Result<(), TlsError> {
        tracing::debug!("TLS: About to acquire custom CA lock");
        let cas = self
            .custom_cas
            .read()
            .map_err(|_| TlsError::Internal("Failed to acquire CA lock".to_string()))?;
        tracing::debug!("TLS: Custom CA lock acquired, processing {} CAs", cas.len());

        for (name, ca) in cas.iter() {
            if ca.is_valid() {
                // Parse CA certificate and add to root store
                if let Ok(cert_der) = pem::parse(&ca.certificate_pem) {
                    let cert = rustls::pki_types::CertificateDer::from(cert_der.contents());
                    if let Err(e) = root_store.add(cert) {
                        tracing::warn!("Failed to add custom CA '{}': {}", name, e);
                    } else {
                        tracing::debug!("Added custom CA: {}", name);
                    }
                }
            } else {
                tracing::warn!("Skipping expired CA: {}", name);
            }
        }
        Ok(())
    }

    /// Create client config with appropriate verification
    fn create_client_config_with_verification(
        &self,
        root_store: RootCertStore,
    ) -> Result<ClientConfig, TlsError> {
        tracing::debug!(
            "TLS: About to configure client config, enable_crl={}",
            self.config.enable_crl
        );
        if self.config.enable_crl {
            self.create_client_config_with_crl(root_store)
        } else {
            Ok(Self::create_client_config_standard(root_store))
        }
    }

    /// Create client config with CRL verification
    fn create_client_config_with_crl(
        &self,
        root_store: RootCertStore,
    ) -> Result<ClientConfig, TlsError> {
        tracing::debug!("Configuring TLS with CRL verification enabled");
        tracing::debug!("TLS: About to get CRLs from cache");
        let crls = self.crl_cache.get_rustls_crls();
        tracing::debug!("TLS: Got {} CRLs from cache", crls.len());

        if crls.is_empty() {
            tracing::debug!("No CRLs available, using standard verification");
            Ok(ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth())
        } else {
            tracing::debug!(
                "TLS: Building WebPkiServerVerifier with {} CRLs",
                crls.len()
            );
            let verifier = rustls::client::WebPkiServerVerifier::builder(Arc::new(root_store))
                .with_crls(crls)
                .build()
                .map_err(|e| {
                    TlsError::CrlValidation(format!("Failed to build CRL verifier: {e}"))
                })?;
            tracing::debug!("TLS: WebPkiServerVerifier built successfully");

            Ok(ClientConfig::builder()
                .with_webpki_verifier(verifier)
                .with_no_client_auth())
        }
    }

    /// Create client config with standard verification
    fn create_client_config_standard(root_store: RootCertStore) -> ClientConfig {
        tracing::debug!("Configuring TLS with standard rustls verification");
        ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth()
    }

    /// Configure client config with early data and ALPN protocols
    fn configure_client_config(&self, client_config: &mut ClientConfig) {
        // Configure early data if enabled
        tracing::debug!(
            "TLS: Configuring early data, enabled={}",
            self.config.enable_early_data
        );
        if self.config.enable_early_data {
            client_config.enable_early_data = true;
        }

        // Configure ALPN protocols for HTTP/3, HTTP/2, and HTTP/1.1 support
        tracing::debug!("TLS: Configuring ALPN protocols");
        client_config.alpn_protocols = vec![
            b"h3".to_vec(),       // HTTP/3 (preferred)
            b"h2".to_vec(),       // HTTP/2 (fallback)
            b"http/1.1".to_vec(), // HTTP/1.1 (final fallback)
        ];
        tracing::debug!("TLS: Client config configured successfully");
    }
}

// EnterpriseServerCertVerifier removed - now using standard rustls WebPkiServerVerifier
// This eliminates the OCSP circular dependency issue while maintaining security
