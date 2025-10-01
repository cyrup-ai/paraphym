use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;
use std::env;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg(not(target_family = "wasm"))]
use crate::chromiumoxide::{create_browser, take_screenshot};

use crate::hyper::{ContentFetcher, FetchResult};

#[derive(Debug)]
pub enum FirecrawlError {
    Network(String),
    Parse(String),
    Timeout(String),
    Internal(String),
}

impl fmt::Display for FirecrawlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirecrawlError::Network(e) => write!(f, "Network error: {}", e),
            FirecrawlError::Parse(e) => write!(f, "Parse error: {}", e),
            FirecrawlError::Timeout(e) => write!(f, "Timeout error: {}", e),
            FirecrawlError::Internal(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl StdError for FirecrawlError {}

#[derive(Debug, Serialize)]
struct FirecrawlRequest {
    url: String,
    formats: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FirecrawlResponse {
    success: bool,
    data: Option<FirecrawlData>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FirecrawlData {
    markdown: Option<String>,
    html: Option<String>,
    metadata: Option<serde_json::Value>,
}

pub struct FirecrawlFetcher;

// Capture real screenshot using shared chromiumoxide functions (native only)
#[cfg(not(target_family = "wasm"))]
async fn capture_real_screenshot(url: &str) -> Result<String, FirecrawlError> {
    // Launch browser using shared function
    let mut browser = create_browser()
        .await
        .map_err(|e| FirecrawlError::Internal(format!("Browser launch failed: {}", e)))?;

    // Create page and navigate
    let page = browser
        .new_page("")
        .await
        .map_err(|e| FirecrawlError::Internal(format!("Failed to create page: {}", e)))?;

    // Navigate with timeout
    let nav_result = tokio::time::timeout(
        Duration::from_secs(30),
        page.goto(url)
    ).await;

    match nav_result {
        Ok(Ok(_)) => {}, // Navigation succeeded
        Ok(Err(e)) => {
            return Err(FirecrawlError::Network(format!("Navigation failed: {}", e)));
        }
        Err(_) => {
            return Err(FirecrawlError::Timeout("Navigation timeout".to_string()));
        }
    }

    // Wait for page to fully load
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Take screenshot using shared function
    let screenshot = take_screenshot(&page)
        .await
        .map_err(|e| FirecrawlError::Internal(format!("Screenshot failed: {}", e)))?;

    // Cleanup
    browser.close().await.ok();

    Ok(screenshot)
}

// WASM version: no screenshot capability
#[cfg(target_family = "wasm")]
async fn capture_real_screenshot(_url: &str) -> Result<String, FirecrawlError> {
    // WASM: Browser automation not available, return empty screenshot
    Ok(String::new())
}

impl FirecrawlFetcher {
    // Helper function to clean HTML
    fn clean_html(html: &str) -> String {
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

    // Real Firecrawl API integration (native only)
    #[cfg(not(target_family = "wasm"))]
    async fn fetch_with_firecrawl(url: &str) -> Result<String, FirecrawlError> {
        use hyper::{Request, Method, StatusCode};
        use hyper_util::client::legacy::Client;
        use hyper_rustls::HttpsConnectorBuilder;
        use http_body_util::{BodyExt, Full};
        use hyper::body::Bytes;
        
        // Validate URL format
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(FirecrawlError::Parse(format!(
                "Invalid URL format: {}",
                url
            )));
        }
        
        // Get API key from environment
        let api_key = env::var("FIRECRAWL_API_KEY")
            .map_err(|_| FirecrawlError::Internal(
                "FIRECRAWL_API_KEY not set. Get one at https://firecrawl.dev".to_string()
            ))?;
        
        // Build request payload
        let payload = FirecrawlRequest {
            url: url.to_string(),
            formats: vec!["html".to_string(), "markdown".to_string()],
        };
        
        let json_body = serde_json::to_string(&payload)
            .map_err(|e| FirecrawlError::Parse(format!("Failed to serialize request: {}", e)))?;
        
        // Build HTTPS connector
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .map_err(|e| FirecrawlError::Internal(format!("TLS init failed: {}", e)))?
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();
        
        // Create HTTP client with Full<Bytes> body type
        let client = Client::builder(hyper_util::rt::TokioExecutor::new())
            .build(https);
        
        // Build HTTP POST request
        let req = Request::builder()
            .method(Method::POST)
            .uri("https://api.firecrawl.dev/v2/scrape")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", "sweetmcp-fetch-plugin/0.1.0")
            .body(Full::new(Bytes::from(json_body)))
            .map_err(|e| FirecrawlError::Parse(format!("Request build failed: {}", e)))?;
        
        // Execute request with timeout
        let fetch_future = async {
            let res = client.request(req)
                .await
                .map_err(|e| FirecrawlError::Network(format!("HTTP request failed: {}", e)))?;
            
            let status = res.status();
            
            // Collect response body
            let body = res.into_body()
                .collect()
                .await
                .map_err(|e| FirecrawlError::Network(format!("Failed to read response: {}", e)))?
                .to_bytes();
            
            let body_str = String::from_utf8(body.to_vec())
                .map_err(|e| FirecrawlError::Parse(format!("Invalid UTF-8: {}", e)))?;
            
            // Handle non-200 status codes
            if status != StatusCode::OK {
                return Err(FirecrawlError::Network(format!(
                    "Firecrawl API returned {}: {}",
                    status.as_u16(),
                    body_str
                )));
            }
            
            // Parse response
            let response: FirecrawlResponse = serde_json::from_str(&body_str)
                .map_err(|e| FirecrawlError::Parse(format!("Invalid JSON response: {}", e)))?;
            
            if !response.success {
                return Err(FirecrawlError::Internal(
                    response.error.unwrap_or_else(|| "Unknown Firecrawl error".to_string())
                ));
            }
            
            // Extract HTML content (prefer HTML over markdown for consistency with other fetchers)
            let data = response.data
                .ok_or_else(|| FirecrawlError::Parse("No data in response".to_string()))?;
            
            let html_content = data.html
                .or(data.markdown.map(|md| format!("<html><body>{}</body></html>", md)))
                .ok_or_else(|| FirecrawlError::Parse("No content in response".to_string()))?;
            
            Ok(html_content)
        };
        
        // Apply 30-second timeout for API calls
        tokio::time::timeout(Duration::from_secs(30), fetch_future)
            .await
            .map_err(|_| FirecrawlError::Timeout(format!("Firecrawl API timeout for {}", url)))?
    }

    // WASM version: Firecrawl not available
    #[cfg(target_family = "wasm")]
    async fn fetch_with_firecrawl(_url: &str) -> Result<String, FirecrawlError> {
        Err(FirecrawlError::Internal(
            "Firecrawl API not available in WASM build. Requires native HTTPS client.".to_string()
        ))
    }
}

#[async_trait]
impl ContentFetcher for FirecrawlFetcher {
    async fn fetch_content(
        &self,
        url: &str,
    ) -> Result<FetchResult, Box<dyn StdError + Send + Sync>> {
        // Fetch content using Firecrawl
        let html_content = Self::fetch_with_firecrawl(url)
            .await
            .map_err(|e| FirecrawlError::Network(format!("Failed to fetch content: {}", e)))?;

        // Clean the HTML (remove scripts and styles)
        let cleaned_html = Self::clean_html(&html_content);

        // Capture real screenshot using chromiumoxide
        let screenshot_base64 = capture_real_screenshot(url)
            .await
            .unwrap_or_else(|e| {
                eprintln!("Screenshot capture failed: {}", e);
                String::new()  // Return empty string on failure
            });

        Ok(FetchResult {
            content: cleaned_html,
            screenshot_base64,
            content_type: "text/html".to_string(),
        })
    }
}
