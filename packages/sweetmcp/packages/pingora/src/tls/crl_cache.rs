//! CRL cache implementation and validation logic

use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use base64::engine::Engine;
use http_body_util::BodyExt;

use x509_parser::prelude::*;

// AsyncStream wrappers removed - using direct async methods per cryypt pattern
use super::bootstrap_client::BootstrapHttpClient;
use super::errors::TlsError;
use super::types::{CrlCacheEntry, ParsedCertificate};

/// CRL validation status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrlStatus {
    Valid,
    Revoked,
    Unknown,
}

// MessageChunk trait implementation removed - direct async methods per cryypt pattern

#[derive(Clone)]
pub struct CrlCache {
    cache: Arc<RwLock<std::collections::HashMap<String, CrlCacheEntry>>>,
    http_client: BootstrapHttpClient,
    /// Cache hit statistics
    cache_hits: Arc<AtomicUsize>,
    /// Cache miss statistics
    cache_misses: Arc<AtomicUsize>,
}

impl std::fmt::Debug for CrlCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cache_size = match self.cache.read() {
            Ok(cache) => cache.len(),
            Err(_) => 0, // Graceful fallback for poisoned lock
        };
        f.debug_struct("CrlCache")
            .field("cache_size", &cache_size)
            .field(
                "cache_hits",
                &self.cache_hits.load(std::sync::atomic::Ordering::Relaxed),
            )
            .field(
                "cache_misses",
                &self.cache_misses.load(std::sync::atomic::Ordering::Relaxed),
            )
            .finish_non_exhaustive()
    }
}

impl Default for CrlCache {
    fn default() -> Self {
        Self::new()
    }
}

impl CrlCache {
    pub fn new() -> Self {
        let http_client = BootstrapHttpClient::new();

        Self {
            cache: Arc::new(RwLock::new(std::collections::HashMap::with_capacity(64))),
            http_client,
            cache_hits: Arc::new(AtomicUsize::new(0)),
            cache_misses: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Get cache statistics (hits, misses)
    pub fn get_stats(&self) -> (usize, usize) {
        (
            self.cache_hits.load(Ordering::Relaxed),
            self.cache_misses.load(Ordering::Relaxed),
        )
    }

    /// Get current cache size (number of entries)
    pub fn get_cache_size(&self) -> usize {
        match self.cache.read() {
            Ok(cache) => cache.len(),
            Err(poisoned) => {
                tracing::warn!("CRL cache read lock poisoned during size check, recovering");
                poisoned.into_inner().len()
            }
        }
    }

    // Streaming wrapper removed - using direct async methods per cryypt pattern

    /// Check certificate status against specific CRL URL - used by TLS verifier
    pub async fn check_certificate_status(&self, serial_number: &[u8], crl_url: &str) -> CrlStatus {
        let is_revoked = self.check_against_crl(serial_number, crl_url).await;
        if is_revoked {
            CrlStatus::Revoked
        } else {
            CrlStatus::Valid
        }
    }

    /// Check if certificate serial number is revoked using CRL
    pub async fn check_certificate_revocation(&self, cert: &ParsedCertificate) -> bool {
        if cert.crl_urls.is_empty() {
            tracing::warn!("No CRL URLs found in certificate, skipping CRL validation");
            return false; // Not revoked (no CRL available)
        }

        // Try each CRL URL until one succeeds
        for crl_url in &cert.crl_urls {
            let is_revoked = self.check_against_crl(&cert.serial_number, crl_url).await;
            if is_revoked {
                tracing::warn!(
                    "Certificate serial {:?} found in CRL from {}",
                    hex::encode(&cert.serial_number),
                    crl_url
                );
                return true;
            }
            tracing::info!(
                "Certificate serial {:?} not found in CRL from {}",
                hex::encode(&cert.serial_number),
                crl_url
            );
        }

        // If all CRLs were checked and certificate not found in any, it's not revoked
        false
    }

    async fn check_against_crl(&self, serial_number: &[u8], crl_url: &str) -> bool {
        let cache_key = crl_url.to_string();

        // Check cache first
        if let Some(cached_crl) = self.get_cached_crl(&cache_key)
            && !Self::is_crl_cache_expired(&cached_crl)
        {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
            tracing::debug!("CRL cache hit for URL: {}", crl_url);
            return cached_crl.revoked_serials.contains(serial_number);
        }

        // Cache miss - download CRL asynchronously
        self.cache_misses.fetch_add(1, Ordering::Relaxed);

        tracing::debug!("CRL cache miss for URL: {}, downloading...", crl_url);

        // Download and cache CRL using BootstrapHttpClient
        match self.download_and_parse_crl(crl_url).await {
            Ok(entry) => {
                let is_revoked = entry.revoked_serials.contains(serial_number);
                self.cache_crl(cache_key, entry);
                tracing::info!(
                    "Downloaded and cached CRL from {}: serial {} is {}",
                    crl_url,
                    hex::encode(serial_number),
                    if is_revoked { "REVOKED" } else { "valid" }
                );
                is_revoked
            }
            Err(e) => {
                tracing::warn!("Failed to download CRL from {}: {}", crl_url, e);
                false // Assume not revoked if download fails (soft-fail)
            }
        }
    }

    #[inline]
    fn get_cached_crl(&self, cache_key: &str) -> Option<CrlCacheEntry> {
        match self.cache.read() {
            Ok(cache) => cache.get(cache_key).cloned(),
            Err(poisoned) => {
                tracing::warn!("CRL cache read lock poisoned, recovering");
                poisoned.into_inner().get(cache_key).cloned()
            }
        }
    }

    fn is_crl_cache_expired(entry: &CrlCacheEntry) -> bool {
        let now = SystemTime::now();

        // Check if we have next_update time and it's passed
        if let Some(next_update) = entry.next_update {
            return now > next_update;
        }

        // Default cache expiry: 24 hours (CRLs are typically updated daily)
        let cache_duration = Duration::from_secs(24 * 3600);
        now.duration_since(entry.cached_at)
            .unwrap_or(Duration::ZERO)
            > cache_duration
    }

    #[inline]
    fn cache_crl(&self, cache_key: String, entry: CrlCacheEntry) {
        match self.cache.write() {
            Ok(mut cache) => {
                cache.insert(cache_key, entry);
            }
            Err(poisoned) => {
                tracing::warn!("CRL cache write lock poisoned, recovering");
                poisoned.into_inner().insert(cache_key, entry);
            }
        }
    }

    async fn download_and_parse_crl(&self, crl_url: &str) -> Result<CrlCacheEntry, TlsError> {
        tracing::debug!("Downloading CRL from: {}", crl_url);
        
        // Create HTTP GET request
        let request = BootstrapHttpClient::get(crl_url)
            .map_err(|e| TlsError::CrlValidation(format!("Failed to create request: {}", e)))?;
        
        // Execute request using BootstrapHttpClient (basic TLS, no OCSP/CRL validation)
        let response = self.http_client.execute(request).await
            .map_err(|e| TlsError::CrlValidation(format!("CRL download failed: {}", e)))?;
        
        // Read response body using http-body-util
        let body_bytes = response.into_body().collect().await
            .map_err(|e| TlsError::CrlValidation(format!("Failed to read CRL body: {}", e)))?
            .to_bytes();
        
        // Parse CRL data (existing function works correctly)
        let entry = Self::parse_crl_data(&body_bytes)?;
        
        tracing::info!(
            "Downloaded CRL from {} - {} revoked certificates", 
            crl_url, 
            entry.revoked_serials.len()
        );
        
        Ok(entry)
    }

    #[allow(clippy::cast_sign_loss)]
    fn parse_crl_data(crl_bytes: &[u8]) -> Result<CrlCacheEntry, TlsError> {
        // Parse PEM if it starts with "-----BEGIN"
        let der_bytes = if crl_bytes.starts_with(b"-----BEGIN") {
            let crl_pem = std::str::from_utf8(crl_bytes)
                .map_err(|_| TlsError::CrlValidation("Invalid UTF-8 in PEM CRL".to_string()))?;

            // Extract DER from PEM
            let mut der_data = Vec::new();
            let mut in_crl = false;
            for line in crl_pem.lines() {
                if line.contains("-----BEGIN") && line.contains("CRL") {
                    in_crl = true;
                    continue;
                }
                if line.contains("-----END") && line.contains("CRL") {
                    break;
                }
                if in_crl
                    && let Ok(decoded) = base64::engine::general_purpose::STANDARD.decode(line)
                {
                    der_data.extend(decoded);
                }
            }

            if der_data.is_empty() {
                return Err(TlsError::CrlValidation(
                    "No CRL data found in PEM".to_string(),
                ));
            }

            der_data
        } else {
            // Assume DER format
            crl_bytes.to_vec()
        };

        // Parse X.509 CRL using x509-parser
        let (_, crl) = parse_x509_crl(&der_bytes)
            .map_err(|e| TlsError::CrlValidation(format!("CRL parsing failed: {e}")))?;

        // Extract revoked certificate serial numbers
        let mut revoked_serials = HashSet::new();
        for revoked_cert in crl.iter_revoked_certificates() {
            revoked_serials.insert(revoked_cert.user_certificate.to_bytes_be());
        }

        // Extract next update time
        let next_update = crl.next_update().map(|time| {
            let timestamp = time.timestamp();
            #[allow(clippy::cast_sign_loss)]
            let timestamp_u64 = if timestamp < 0 {
                0u64 // Use epoch for negative timestamps
            } else {
                timestamp as u64
            };
            std::time::UNIX_EPOCH + std::time::Duration::from_secs(timestamp_u64)
        });

        tracing::info!(
            "Parsed CRL with {} revoked certificates, next update: {:?}",
            revoked_serials.len(),
            next_update
        );

        Ok(CrlCacheEntry {
            revoked_serials,
            cached_at: SystemTime::now(),
            next_update,
            crl_der: der_bytes,
        })
    }

    /// Cleanup expired CRL cache entries
    pub fn cleanup_cache(&self) {
        let mut cache = match self.cache.write() {
            Ok(cache) => cache,
            Err(poisoned) => {
                tracing::warn!("CRL cache write lock poisoned during cleanup, recovering");
                poisoned.into_inner()
            }
        };

        cache.retain(|_url, entry| !Self::is_crl_cache_expired(entry));

        tracing::debug!(
            "CRL cache cleanup completed, {} CRLs remaining",
            cache.len()
        );
    }

    /// Get all cached CRLs in rustls format for use with `WebPkiServerVerifier`
    pub fn get_rustls_crls(&self) -> Vec<rustls::pki_types::CertificateRevocationListDer<'static>> {
        let cache = match self.cache.read() {
            Ok(cache) => cache,
            Err(poisoned) => {
                tracing::warn!(
                    "CRL cache read lock poisoned during rustls CRL retrieval, recovering"
                );
                poisoned.into_inner()
            }
        };

        let mut rustls_crls = Vec::new();
        let now = std::time::SystemTime::now();

        for (url, entry) in cache.iter() {
            // Check if CRL is still valid
            let is_expired = entry
                .next_update
                .is_some_and(|next_update| now > next_update);

            if is_expired {
                tracing::debug!("Skipping expired CRL from {}", url);
            } else {
                // Convert DER bytes to rustls CRL format
                let crl_der =
                    rustls::pki_types::CertificateRevocationListDer::from(entry.crl_der.clone());
                rustls_crls.push(crl_der);
                tracing::debug!("Added CRL from {} to rustls verifier", url);
            }
        }

        tracing::info!(
            "Loaded {} CRLs for rustls certificate verification",
            rustls_crls.len()
        );
        rustls_crls
    }
}
