use std::path::PathBuf;
use ystream::AsyncTask;

/// Result type for model downloads
#[derive(Debug, Clone)]
pub struct ModelDownloadResult {
    /// Model identifier (HF repo name)
    pub model_id: String,
    /// Downloaded file paths
    pub files: Vec<PathBuf>,
    /// Total bytes downloaded
    pub total_bytes: u64,
    /// Cache directory used
    pub cache_dir: PathBuf,
}

/// Find a specific file by name in download results
/// 
/// Searches the downloaded file paths for a file with the exact filename match.
/// 
/// # Arguments
/// * `result` - ModelDownloadResult containing downloaded file paths
/// * `filename` - Exact filename to search for (e.g., "tokenizer.json")
/// 
/// # Returns
/// Reference to the PathBuf if found
/// 
/// # Errors
/// Returns error if file not found in results
pub fn find_file_by_name<'a>(
    result: &'a ModelDownloadResult,
    filename: &str,
) -> Result<&'a PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    result.files.iter()
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n == filename)
        })
        .ok_or_else(|| {
            format!(
                "File '{}' not found in downloaded files. Available: {:?}",
                filename,
                result.files.iter()
                    .filter_map(|p| p.file_name().and_then(|n| n.to_str()))
                    .collect::<Vec<_>>()
            ).into()
        })
}

/// Common trait for model download providers
pub trait ModelDownloadProvider: Send + Sync + 'static {
    /// Download a model with optional quantization filter
    fn download_model(
        &self,
        model_id: &str,
        files: Vec<String>,
        quantization: Option<String>,
    ) -> AsyncTask<Result<ModelDownloadResult, Box<dyn std::error::Error + Send + Sync>>>;

    /// Check if model is cached
    fn is_cached(&self, model_id: &str, files: &[String]) -> AsyncTask<bool>;

    /// Get cache directory
    fn cache_dir(&self) -> PathBuf;
}

// Re-export provider implementations
#[cfg(feature = "download-hf-hub")]
pub mod hf_hub_provider;
#[cfg(feature = "download-hf-hub")]
pub use hf_hub_provider::HfHubDownloadProvider;

#[cfg(feature = "download-progresshub")]
pub mod progresshub_provider;
#[cfg(feature = "download-progresshub")]
pub use progresshub_provider::ProgressHubDownloadProvider;

// Factory for provider selection
pub mod factory;
pub use factory::DownloadProviderFactory;