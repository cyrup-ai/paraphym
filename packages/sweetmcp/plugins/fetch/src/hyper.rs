use std::error::Error as StdError;
use std::fmt;

use async_trait::async_trait;
use base64::Engine;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::{Request, Uri};
#[cfg(not(target_family = "wasm"))]
use hyper_rustls::ConfigBuilderExt;
use hyper_util::rt::TokioIo;
#[cfg(not(target_family = "wasm"))]
use tokio_rustls::TlsConnector;

#[cfg(target_family = "wasm")]
use gloo_net::http::Request as GlooRequest;
#[cfg(target_family = "wasm")]
use gloo_timers::future::TimeoutFuture;
#[cfg(target_family = "wasm")]
use futures::{select, FutureExt};

#[cfg(not(target_family = "wasm"))]
pub use crate::chromiumoxide::{ContentFetcher, FetchResult};

// WASM: Define the trait and types locally since chromiumoxide module doesn't exist
#[cfg(target_family = "wasm")]
pub struct FetchResult {
    pub content: String,
    pub screenshot_base64: Option<String>,
    pub content_type: String,
}

#[cfg(target_family = "wasm")]
#[async_trait]
pub trait ContentFetcher {
    async fn fetch_content(&self, url: &str) -> Result<FetchResult, Box<dyn StdError + Send + Sync>>;
}

#[derive(Debug)]
pub enum FetchError {
    Hyper(hyper::Error),
    Http(hyper::http::Error),
    InvalidUri(hyper::http::uri::InvalidUri),
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Hyper(e) => write!(f, "Hyper error: {}", e),
            FetchError::Http(e) => write!(f, "HTTP error: {}", e),
            FetchError::InvalidUri(e) => write!(f, "Invalid URI: {}", e),
            FetchError::Io(e) => write!(f, "IO error: {}", e),
            FetchError::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl StdError for FetchError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            FetchError::Hyper(e) => Some(e),
            FetchError::Http(e) => Some(e),
            FetchError::InvalidUri(e) => Some(e),
            FetchError::Io(e) => Some(e),
            FetchError::Other(_) => None,
        }
    }
}

impl From<hyper::Error> for FetchError {
    fn from(e: hyper::Error) -> Self {
        FetchError::Hyper(e)
    }
}

impl From<hyper::http::Error> for FetchError {
    fn from(e: hyper::http::Error) -> Self {
        FetchError::Http(e)
    }
}

impl From<hyper::http::uri::InvalidUri> for FetchError {
    fn from(e: hyper::http::uri::InvalidUri) -> Self {
        FetchError::InvalidUri(e)
    }
}

impl From<std::io::Error> for FetchError {
    fn from(e: std::io::Error) -> Self {
        FetchError::Io(e)
    }
}

pub struct HyperFetcher;

impl HyperFetcher {
    #[cfg(not(target_family = "wasm"))]
    pub async fn fetch(url: &str) -> Result<String, FetchError> {
        // Parse the URL
        let uri: Uri = url.parse()?;

        // Extract components
        let scheme = uri
            .scheme_str()
            .ok_or_else(|| FetchError::Other("URL must have a scheme".to_string()))?;
        let host = uri
            .host()
            .ok_or_else(|| FetchError::Other("URL must have a host".to_string()))?;
        let port = uri.port_u16().unwrap_or_else(|| {
            match scheme {
                "https" => 443,
                "http" => 80,
                _ => return 443, // default to HTTPS
            }
        });

        // For this plugin, we only support HTTPS
        if scheme != "https" {
            return Err(FetchError::Other("Only HTTPS is supported".to_string()));
        }

        let addr = format!("{}:{}", host, port);

        // Connect TCP
        let tcp_stream = tokio::net::TcpStream::connect(&addr).await?;
        tcp_stream.set_nodelay(true)?;

        // TLS setup with zero-copy server name
        let tls_config = rustls::ClientConfig::builder()
            .with_native_roots()?
            .with_no_client_auth();

        let server_name = rustls::pki_types::ServerName::try_from(host.to_string())
            .map_err(|_| FetchError::Other("Invalid server name".to_string()))?;

        let connector = TlsConnector::from(std::sync::Arc::new(tls_config));
        let tls_stream = connector
            .connect(server_name, tcp_stream)
            .await
            .map_err(|e| FetchError::Other(format!("TLS handshake failed: {}", e)))?;

        // Wrap for hyper
        let io = TokioIo::new(tls_stream);

        // Use HTTP/2 with automatic protocol selection
        let (mut sender, conn) =
            hyper::client::conn::http2::Builder::new(hyper_util::rt::TokioExecutor::new())
                .adaptive_window(true)
                .max_frame_size(16_384)
                .max_send_buf_size(1024 * 1024)
                .handshake(io)
                .await?;

        // Spawn connection handler without blocking
        tokio::spawn(async move {
            let _ = conn.await;
        });

        // Build optimized request
        let authority = uri
            .authority()
            .ok_or_else(|| FetchError::Other("Invalid authority".to_string()))?
            .as_str();

        let path_and_query = uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/");

        let request = Request::builder()
            .method("GET")
            .uri(path_and_query)
            .header(hyper::header::HOST, authority)
            .header(hyper::header::USER_AGENT, "fetch-hyper/1.0")
            .header(hyper::header::ACCEPT, "*/*")
            .header(hyper::header::ACCEPT_ENCODING, "identity")
            .body(Empty::<Bytes>::new())?;

        // Send request
        let response = sender.send_request(request).await?;
        let status = response.status();

        if !status.is_success() {
            return Err(FetchError::Other(format!(
                "HTTP {}: {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown")
            )));
        }

        // Collect body with pre-allocated buffer
        let content_length = response
            .headers()
            .get(hyper::header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<usize>().ok());

        let mut body_bytes = if let Some(len) = content_length {
            Vec::with_capacity(len.min(10 * 1024 * 1024)) // Cap at 10MB pre-allocation
        } else {
            Vec::with_capacity(64 * 1024) // 64KB default
        };

        let mut body = response.into_body();
        while let Some(frame) = body.frame().await {
            let frame = frame.map_err(|e| FetchError::Other(format!("Frame error: {}", e)))?;
            if let Some(chunk) = frame.data_ref() {
                body_bytes.extend_from_slice(chunk);
            }
        }

        // Convert to string without re-allocation
        String::from_utf8(body_bytes)
            .map_err(|e| FetchError::Other(format!("Invalid UTF-8: {}", e)))
    }

    // WASM version: uses browser's fetch API via gloo-net
    // Supports both HTTP and HTTPS through the browser's native fetch implementation
    // CORS restrictions apply as per browser security policies
    // Timeout: 30 seconds (consistent with chromiumoxide pattern)
    #[cfg(target_family = "wasm")]
    pub async fn fetch(url: &str) -> Result<String, FetchError> {
        // Validate URL scheme (browser enforces HTTPS for secure contexts)
        let uri: Uri = url.parse()?;
        let scheme = uri
            .scheme_str()
            .ok_or_else(|| FetchError::Other("URL must have a scheme".to_string()))?;
        
        // Browser fetch supports both HTTP and HTTPS
        // Note: HTTPS pages cannot fetch HTTP resources (mixed content blocking)
        if scheme != "http" && scheme != "https" {
            return Err(FetchError::Other(format!(
                "Unsupported scheme '{}'. Only HTTP and HTTPS are supported in WASM.",
                scheme
            )));
        }

        // Create fetch future and timeout future (30 seconds = 30,000 milliseconds)
        let fetch_future = async {
            // Create and send request using gloo-net (wraps browser fetch API)
            let response = GlooRequest::get(url)
                .send()
                .await
                .map_err(|e| FetchError::Other(format!("Failed to send request: {}", e)))?;

            // Check response status
            let status = response.status();
            if status < 200 || status >= 300 {
                return Err(FetchError::Other(format!(
                    "HTTP {}: Request failed",
                    status
                )));
            }

            // Extract response body as text
            response
                .text()
                .await
                .map_err(|e| FetchError::Other(format!("Failed to read response body: {}", e)))
        };

        let timeout_future = TimeoutFuture::new(30_000);

        // Race fetch against timeout
        select! {
            result = fetch_future.fuse() => result,
            _ = timeout_future.fuse() => {
                Err(FetchError::Other(format!("Request to {} timed out after 30 seconds", url)))
            },
        }
    }

    pub fn clean_html(html: &str) -> String {
        // Use a simple approach to remove script and style tags
        // A more robust approach would use an HTML parser like html5ever
        let mut result = String::new();
        let mut in_script = false;
        let mut in_style = false;

        for line in html.lines() {
            let lower = line.to_lowercase();

            if lower.contains("<script") {
                in_script = true;
            }

            if lower.contains("<style") {
                in_style = true;
            }

            if !in_script && !in_style {
                result.push_str(line);
                result.push('\n');
            }

            if lower.contains("</script>") {
                in_script = false;
            }

            if lower.contains("</style>") {
                in_style = false;
            }
        }

        result
    }
}

#[async_trait]
impl ContentFetcher for HyperFetcher {
    async fn fetch_content(
        &self,
        url: &str,
    ) -> Result<FetchResult, Box<dyn StdError + Send + Sync>> {
        // Fetch HTML content using hyper
        let content = Self::fetch(url)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        // Clean the HTML content
        let cleaned_content = Self::clean_html(&content);

        // Hyper is HTTP-only and cannot generate screenshots
        // Return None to be honest about this limitation
        Ok(FetchResult {
            content: cleaned_content,
            screenshot_base64: None,
            content_type: "text/html".to_string(),
        })
    }
}
