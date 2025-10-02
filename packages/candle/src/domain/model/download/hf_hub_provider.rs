use hf_hub::{api::tokio::Api, Cache};
use std::path::PathBuf;
use async_trait::async_trait;
use super::{ModelDownloadProvider, ModelDownloadResult};

/// API response types for `HuggingFace` repository metadata
#[derive(serde::Deserialize, Debug)]
struct RepoInfo {
    siblings: Vec<RepoFile>,
    #[allow(dead_code)]
    #[serde(rename = "modelId")]
    model_id: String,
    #[allow(dead_code)]
    sha: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct RepoFile {
    rfilename: String,
    size: Option<u64>,
    lfs: Option<LfsInfo>,
}

#[derive(serde::Deserialize, Debug)]
struct LfsInfo {
    size: u64,
    #[allow(dead_code)]
    sha256: Option<String>,
    #[allow(dead_code)]
    pointer_size: Option<u64>,
}

/// `HuggingFace` Hub download provider with production-quality implementation
pub struct HfHubDownloadProvider {
    api: Api,
    cache_dir: PathBuf,
    http_client: reqwest::Client,
    auth_token: Option<String>,
}

impl HfHubDownloadProvider {
    /// Create a new `HuggingFace` Hub download provider
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP client or `HuggingFace` API initialization fails
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let api = Api::new()?;
        let cache = Cache::default();
        let cache_dir = cache.path().clone();
        
        // Build HTTP client with reasonable defaults for production
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .connect_timeout(std::time::Duration::from_secs(10))
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"), "/",
                env!("CARGO_PKG_VERSION"), " ",
                "(+https://github.com/cyrusnimda/paraphym)"
            ))
            .pool_idle_timeout(Some(std::time::Duration::from_secs(90)))
            .pool_max_idle_per_host(10)
            .build()?;
        
        // Try multiple environment variables for auth token
        let auth_token = std::env::var("HF_TOKEN")
            .or_else(|_| std::env::var("HUGGING_FACE_HUB_TOKEN"))
            .or_else(|_| std::env::var("HUGGINGFACE_TOKEN"))
            .ok()
            .filter(|s| !s.is_empty());
        
        Ok(Self { 
            api, 
            cache_dir, 
            http_client, 
            auth_token 
        })
    }
    
    /// List all files in a repository using `HuggingFace` API
    async fn list_repo_files(&self, model_id: &str) -> Result<Vec<RepoFile>, Box<dyn std::error::Error + Send + Sync>> {
        const MAX_RETRIES: u32 = 3;
        
        let url = format!("https://huggingface.co/api/models/{model_id}");
        
        // Retry logic with exponential backoff
        let mut retry_count = 0;
        let mut backoff = std::time::Duration::from_secs(1);
        
        loop {
            let mut request = self.http_client.get(&url);
            
            // Add auth header if token is available
            if let Some(ref token) = self.auth_token {
                request = request.header("Authorization", format!("Bearer {token}"));
            }
            
            let response = request.send().await?;
            let status = response.status();
            
            // Handle different status codes
            match status.as_u16() {
                200 => {
                    // Success - parse and return
                    let repo_info: RepoInfo = response.json().await?;
                    return Ok(repo_info.siblings);
                }
                401 | 403 => {
                    // Authentication error
                    return Err(format!(
                        "Authentication failed for {model_id}: {status}. Set HF_TOKEN environment variable for private models."
                    ).into());
                }
                404 => {
                    // Model not found
                    return Err(format!("Model '{model_id}' not found on HuggingFace Hub").into());
                }
                429 => {
                    // Rate limited - retry with backoff
                    if retry_count >= MAX_RETRIES {
                        return Err(format!("Rate limited after {MAX_RETRIES} retries").into());
                    }
                    
                    // Check for Retry-After header
                    if let Some(retry_after) = response.headers().get("retry-after")
                        && let Ok(seconds_str) = retry_after.to_str()
                        && let Ok(seconds) = seconds_str.parse::<u64>() {
                            backoff = std::time::Duration::from_secs(seconds);
                        }
                    
                    tokio::time::sleep(backoff).await;
                    retry_count += 1;
                    backoff *= 2; // Exponential backoff
                }
                500..=599 => {
                    // Server error - retry
                    if retry_count >= MAX_RETRIES {
                        return Err(format!("Server error after {MAX_RETRIES} retries: {status}").into());
                    }
                    
                    tokio::time::sleep(backoff).await;
                    retry_count += 1;
                    backoff *= 2;
                }
                _ => {
                    // Other error
                    let error_text = response.text().await.unwrap_or_else(|_| String::new());
                    return Err(format!("API error {status}: {error_text}").into());
                }
            }
        }
    }
    
    /// Match a file against a glob pattern using globset
    #[inline(always)]
    fn matches_pattern(pattern: &str, filename: &str) -> bool {
        use globset::{Glob, GlobBuilder};
        
        // Optimize for exact matches (no wildcards)
        if !pattern.contains('*') && !pattern.contains('?') && !pattern.contains('[') {
            return pattern == filename;
        }
        
        // Handle directory-aware patterns
        let glob_result = if pattern.contains('/') {
            GlobBuilder::new(pattern)
                .literal_separator(true)
                .build()
        } else {
            Glob::new(pattern)
        };
        
        match glob_result {
            Ok(glob) => {
                let matcher = glob.compile_matcher();
                matcher.is_match(filename)
            }
            Err(_) => {
                // Fallback to exact match for invalid patterns
                pattern == filename
            }
        }
    }
}

#[async_trait]
impl ModelDownloadProvider for HfHubDownloadProvider {
    async fn download_model(
        &self,
        model_id: &str,
        files: Vec<String>,
        quantization: Option<String>,
    ) -> Result<ModelDownloadResult, Box<dyn std::error::Error + Send + Sync>> {
        let repo = self.api.model(model_id.to_string());
        
        // Get actual file list from API
        let repo_files = self.list_repo_files(model_id).await?;
        
        // Pre-allocate with estimated capacity
        let mut downloaded_files = Vec::with_capacity(files.len());
        let mut total_bytes = 0u64;
        
        // Track which patterns matched anything
        let mut any_matches = false;
        
        for file_pattern in &files {
            let matching_files: Vec<&RepoFile> = if file_pattern.contains('*') || file_pattern.contains('?') {
                // Pattern matching
                repo_files.iter()
                    .filter(|f| Self::matches_pattern(file_pattern, &f.rfilename))
                    .filter(|f| {
                        // Apply quantization filter if specified
                        quantization.as_ref()
                            .is_none_or(|q| f.rfilename.contains(q))
                    })
                    .collect()
            } else {
                // Exact match
                repo_files.iter()
                    .filter(|f| &f.rfilename == file_pattern)
                    .filter(|f| {
                        quantization.as_ref()
                            .is_none_or(|q| f.rfilename.contains(q))
                    })
                    .collect()
            };
            
            if !matching_files.is_empty() {
                any_matches = true;
            }
            
            // Download matching files sequentially (ApiRepo doesn't implement Clone)
            for repo_file in matching_files {
                let filename = &repo_file.rfilename;
                let size = repo_file.lfs.as_ref().map(|l| l.size)
                    .or(repo_file.size)
                    .unwrap_or(0);
                
                match repo.get(filename).await {
                    Ok(path) => {
                        total_bytes += size;
                        downloaded_files.push(path);
                    }
                    Err(e) => {
                        // Log warning but continue with other files
                        eprintln!("Warning: Failed to download {filename}: {e}");
                    }
                }
            }
        }
        
        // Validate that we downloaded something if patterns were provided
        if !files.is_empty() && !any_matches {
            return Err(format!(
                "No files matched patterns {:?} in model '{}'. Available files: {:?}",
                files,
                model_id,
                repo_files.iter()
                    .take(10)
                    .map(|f| &f.rfilename)
                    .collect::<Vec<_>>()
            ).into());
        }
        
        // Construct cache directory following hf-hub convention
        let model_id_safe = model_id.replace('/', "--");
        let cache_path = if downloaded_files.is_empty() {
            self.cache_dir.join(format!("models--{model_id_safe}"))
        } else {
            // Use actual download location from first file
            downloaded_files[0].parent()
                .map_or_else(|| self.cache_dir.join(format!("models--{model_id_safe}")), std::path::Path::to_path_buf)
        };
        
        Ok(ModelDownloadResult {
            model_id: model_id.to_string(),
            files: downloaded_files,
            total_bytes,
            cache_dir: cache_path,
        })
    }

    async fn is_cached(&self, model_id: &str, files: &[String]) -> bool {
        // Early return for empty file list
        if files.is_empty() {
            return true;
        }
        
        // Check local cache directory WITHOUT making network calls
        // This is a best-effort check - we check if the model directory exists
        // For exact file checking, we'd need to know the exact filenames which
        // requires API calls (defeating the purpose of a cache check)
        
        // Construct the expected cache directory path
        let model_id_safe = model_id.replace('/', "--");
        let model_cache_dir = self.cache_dir.join(format!("models--{model_id_safe}"));
        
        // For patterns, we can't check without knowing actual filenames
        // For exact files, check if they exist in expected locations
        if files.iter().any(|f| f.contains('*') || f.contains('?')) {
            // If patterns are used, we can only check if model dir exists
            // This is conservative - returns false even if some files might be cached
            tokio::fs::try_exists(&model_cache_dir)
                .await
                .unwrap_or(false)
        } else {
            // For exact filenames, check each one in snapshots dir
            // HF cache structure: models--{org}--{model}/snapshots/{revision}/{files}
            
            // Check if model directory exists first
            if !tokio::fs::try_exists(&model_cache_dir).await.unwrap_or(false) {
                return false;
            }
            
            // We can't check exact files without knowing the revision
            // So we check if snapshots directory has any content
            let snapshots_dir = model_cache_dir.join("snapshots");
            if let Ok(mut entries) = tokio::fs::read_dir(&snapshots_dir).await {
                // Just check if there's at least one snapshot
                if entries.next_entry().await.unwrap_or(None).is_some() {
                    // Conservatively assume files might be cached if snapshots exist
                    return true;
                }
            }
            
            false
        }
    }

    fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone()
    }
}