//! Bootstrap HTTP client for TLS validation
//!
//! This client uses basic TLS without OCSP/CRL validation to avoid circular dependencies
//! when fetching CRLs or OCSP responses during TLS handshake.

use hyper::body::Incoming;
use hyper::{Request, Response};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use rustls::{ClientConfig, RootCertStore};
use std::time::Duration;

/// Bootstrap HTTP client that uses basic TLS without OCSP/CRL validation
///
/// This breaks the circular dependency:
/// `H2Strategy` → TLS handshake → CRL validation → `HttpClient` → `AutoStrategy` → `H2Strategy`
#[derive(Clone)]
pub struct BootstrapHttpClient {
    client: Client<HttpsConnector<HttpConnector>, String>,
}

impl BootstrapHttpClient {
    /// Create a new bootstrap HTTP client with basic TLS
    pub fn new() -> Self {
        // Create basic TLS config without OCSP/CRL validation
        let mut root_store = RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let tls_config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        // Create HTTPS connector with basic TLS
        let https_connector = HttpsConnectorBuilder::new()
            .with_tls_config(tls_config)
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder(hyper_util::rt::TokioExecutor::new())
            .pool_idle_timeout(Duration::from_secs(30))
            .build(https_connector);

        Self { client }
    }

    /// Execute an HTTP request using basic TLS
    pub async fn execute(
        &self,
        request: Request<String>,
    ) -> Result<Response<Incoming>, Box<dyn std::error::Error + Send + Sync>> {
        self.client.request(request).await.map_err(Into::into)
    }

    /// Create a simple GET request
    pub fn get(url: &str) -> Result<Request<String>, Box<dyn std::error::Error + Send + Sync>> {
        Request::builder()
            .method("GET")
            .uri(url)
            .header("User-Agent", "quyc-bootstrap/1.0")
            .body(String::new())
            .map_err(Into::into)
    }
}

impl Default for BootstrapHttpClient {
    fn default() -> Self {
        Self::new()
    }
}
