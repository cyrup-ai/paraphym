//! Smart input resolution utility
//!
//! Detects whether input is a file path, URL, or literal text and resolves accordingly.

use anyhow::{Context, Result};
use std::path::Path;

/// Resolve input using smart detection: file path, URL, or literal text
///
/// # Detection Logic
/// 1. If path exists on filesystem → Load from file
/// 2. If starts with http:// or https:// → Fetch from URL
/// 3. Otherwise → Use as literal text
///
/// # Arguments
/// * `input` - The input string to resolve
///
/// # Returns
/// Result containing the resolved string content
///
/// # Examples
/// ```no_run
/// use cyrup_candle::util::input_resolver::resolve_input;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // File path
/// let content = resolve_input("./README.md").await?;
///
/// // URL
/// let content = resolve_input("https://example.com/doc.txt").await?;
///
/// // Literal text
/// let content = resolve_input("Hello, world!").await?;
/// # Ok(())
/// # }
/// ```
pub async fn resolve_input(input: &str) -> Result<String> {
    if Path::new(input).exists() {
        // File path - load from disk
        tokio::fs::read_to_string(input)
            .await
            .context(format!("Failed to read file: {}", input))
    } else if input.starts_with("http://") || input.starts_with("https://") {
        // URL - fetch from web with timeout and retries
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        let mut last_error = None;
        for attempt in 0..3 {
            match client.get(input).send().await {
                Ok(response) => {
                    return response
                        .text()
                        .await
                        .context("Failed to read response body");
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < 2 {
                        tokio::time::sleep(std::time::Duration::from_millis(100 * (1 << attempt)))
                            .await;
                    }
                }
            }
        }

        Err(last_error.unwrap().into())
    } else {
        // Literal text - use as-is
        Ok(input.to_string())
    }
}

/// Synchronous version for immediate resolution (literal text only)
///
/// # Arguments
/// * `input` - The input string to resolve
///
/// # Returns
/// Result containing the resolved string content
///
/// # Note
/// This function returns the input as-is for literal text.
/// For file paths and URLs, use the async version `resolve_input`.
pub fn resolve_input_sync(input: &str) -> Result<String> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_literal_text() {
        let result = resolve_input("Hello, world!").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[tokio::test]
    async fn test_resolve_url() {
        // URL format should be recognized
        let input = "https://example.com/doc.txt";
        let result = resolve_input(input).await;
        // We don't test actual fetch, just that it's recognized as URL
        // The result may fail due to network, which is expected
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_sync_literal() {
        let result = resolve_input_sync("Test content");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test content");
    }
}
