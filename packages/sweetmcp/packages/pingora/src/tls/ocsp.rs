//! OCSP (Online Certificate Status Protocol) validation module

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use rand::Rng;
// AsyncStream wrappers removed - using direct async methods per cryypt pattern

use super::types::ParsedCertificate;

/// OCSP response status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OcspStatus {
    Good,
    Revoked,
    Unknown,
}

/// OCSP response cache entry
#[derive(Debug, Clone)]
pub struct OcspCacheEntry {
    pub status: OcspStatus,
    pub cached_at: SystemTime,
    pub next_update: Option<SystemTime>,
}

/// OCSP response cache for performance optimization
///
/// NOTE: This cache is for standalone OCSP validation only.
/// TLS connections use OCSP stapling automatically via rustls `WebPkiServerVerifier`.
#[derive(Clone)]
pub struct OcspCache {
    cache: Arc<RwLock<HashMap<String, OcspCacheEntry>>>,
    /// Pre-generated random bytes for nonce generation
    nonce_pool: Arc<RwLock<Vec<u8>>>,
    /// Cache hit statistics
    cache_hits: Arc<AtomicUsize>,
    /// Cache miss statistics
    cache_misses: Arc<AtomicUsize>,
}

impl std::fmt::Debug for OcspCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cache_size = match self.cache.read() {
            Ok(cache) => cache.len(),
            Err(_) => 0, // Graceful fallback for poisoned lock
        };
        f.debug_struct("OcspCache")
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

impl Default for OcspCache {
    fn default() -> Self {
        Self::new()
    }
}

impl OcspCache {
    pub fn new() -> Self {
        // Pre-generate 1KB of random bytes for nonce generation
        let mut nonce_pool = vec![0u8; 1024];
        rand::rng().fill(&mut nonce_pool[..]);

        Self {
            cache: Arc::new(RwLock::new(HashMap::with_capacity(128))),
            nonce_pool: Arc::new(RwLock::new(nonce_pool)),
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
                tracing::warn!("OCSP cache read lock poisoned during size check, recovering");
                poisoned.into_inner().len()
            }
        }
    }

    /// Check OCSP status for a certificate with caching
    pub fn check_certificate(
        &self,
        cert: &ParsedCertificate,
        _issuer_cert: Option<&ParsedCertificate>,
    ) -> OcspStatus {
        let cache_key = Self::make_cache_key(&cert.serial_number);

        // Check cache first
        if let Some(cached) = self.get_cached_status(&cache_key)
            && !Self::is_cache_expired(&cached)
        {
            self.cache_hits.fetch_add(1, Ordering::Relaxed);
            tracing::debug!(
                "OCSP cache hit for certificate serial: {:?}",
                hex::encode(&cert.serial_number)
            );
            return cached.status;
        }

        // Cache miss - increment counter
        self.cache_misses.fetch_add(1, Ordering::Relaxed);

        // OCSP validation disabled to prevent circular dependency during TLS handshake
        // TLS connections use OCSP stapling automatically via rustls WebPkiServerVerifier
        tracing::debug!(
            "OCSP validation skipped for certificate serial: {:?} (using OCSP stapling instead)",
            hex::encode(&cert.serial_number)
        );

        let status = OcspStatus::Unknown;

        // Cache the result
        self.cache_status(cache_key, status, None);
        status
    }

    fn make_cache_key(serial_number: &[u8]) -> String {
        hex::encode(serial_number)
    }

    #[inline]
    fn get_cached_status(&self, cache_key: &str) -> Option<OcspCacheEntry> {
        match self.cache.read() {
            Ok(cache) => cache.get(cache_key).cloned(),
            Err(poisoned) => {
                tracing::warn!("OCSP cache read lock poisoned, recovering");
                poisoned.into_inner().get(cache_key).cloned()
            }
        }
    }

    fn is_cache_expired(entry: &OcspCacheEntry) -> bool {
        let now = SystemTime::now();

        // Check if we have next_update time and it's passed
        if let Some(next_update) = entry.next_update {
            return now > next_update;
        }

        // Default cache expiry: 1 hour
        let cache_duration = Duration::from_secs(3600);
        now.duration_since(entry.cached_at)
            .unwrap_or(Duration::ZERO)
            > cache_duration
    }

    #[inline]
    fn cache_status(&self, cache_key: String, status: OcspStatus, next_update: Option<SystemTime>) {
        let entry = OcspCacheEntry {
            status,
            cached_at: SystemTime::now(),
            next_update,
        };

        match self.cache.write() {
            Ok(mut cache) => {
                cache.insert(cache_key, entry);
            }
            Err(poisoned) => {
                tracing::warn!("OCSP cache write lock poisoned, recovering");
                poisoned.into_inner().insert(cache_key, entry);
            }
        }
    }

    fn perform_ocsp_check(
        cert: &ParsedCertificate,
        _issuer_cert: Option<&ParsedCertificate>,
    ) -> (OcspStatus, Option<SystemTime>) {
        // OCSP validation disabled to prevent circular dependency during TLS handshake
        // TLS connections use OCSP stapling automatically via rustls WebPkiServerVerifier
        tracing::debug!(
            "OCSP validation skipped for certificate serial: {:?} (using OCSP stapling instead)",
            hex::encode(&cert.serial_number)
        );
        (OcspStatus::Unknown, None)
    }

    // HTTP-based OCSP validation methods removed to prevent circular dependency
    // TLS connections use OCSP stapling automatically via rustls WebPkiServerVerifier

    #[inline]
    fn generate_nonce(&self) -> Vec<u8> {
        let mut nonce = vec![0u8; 16];

        // Get random bytes from pre-generated pool
        {
            let mut pool = match self.nonce_pool.write() {
                Ok(pool) => pool,
                Err(poisoned) => {
                    tracing::warn!("OCSP nonce pool write lock poisoned, recovering");
                    poisoned.into_inner()
                }
            };
            if pool.len() >= 16 {
                nonce.copy_from_slice(&pool[..16]);
                pool.drain(..16);
            } else {
                // Refill pool if exhausted
                pool.resize(1024, 0);
                rand::rng().fill(&mut pool[..]);
                nonce.copy_from_slice(&pool[..16]);
                pool.drain(..16);
            }
        }

        nonce
    }

    /// Cleanup expired cache entries
    pub fn cleanup_cache(&self) {
        let mut cache = match self.cache.write() {
            Ok(cache) => cache,
            Err(poisoned) => {
                tracing::warn!("OCSP cache write lock poisoned during cleanup, recovering");
                poisoned.into_inner()
            }
        };

        cache.retain(|_key, entry| !Self::is_cache_expired(entry));

        tracing::debug!(
            "OCSP cache cleanup completed, {} entries remaining",
            cache.len()
        );
    }
}
