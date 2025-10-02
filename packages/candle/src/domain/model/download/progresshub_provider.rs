use progresshub::{ProgressHub, OneOrMany as PHOneOrMany, ZeroOneOrMany};
use std::path::PathBuf;
use async_trait::async_trait;
use super::{ModelDownloadProvider, ModelDownloadResult};

pub struct ProgressHubDownloadProvider {
    force_download: bool,
    show_cli_progress: bool,
}

impl ProgressHubDownloadProvider {
    pub fn new() -> Self {
        Self {
            force_download: false,
            show_cli_progress: false, // Default to no CLI progress for library usage
        }
    }

    pub fn with_force(mut self, force: bool) -> Self {
        self.force_download = force;
        self
    }

    pub fn with_cli_progress(mut self, show: bool) -> Self {
        self.show_cli_progress = show;
        self
    }
}

#[async_trait]
impl ModelDownloadProvider for ProgressHubDownloadProvider {
    async fn download_model(
        &self,
        model_id: &str,
        _files: Vec<String>, // ProgressHub handles file selection internally
        quantization: Option<String>,
    ) -> Result<ModelDownloadResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut builder = ProgressHub::builder()
            .model(model_id)
            .force(self.force_download);

        if let Some(quant) = quantization {
            builder = builder.quantization(quant);
        }

        let result = if self.show_cli_progress {
            let results = builder.with_cli_progress().download().await?;
            // Convert OneOrMany to single result
            match results {
                PHOneOrMany::One(result) => result,
                PHOneOrMany::Many(mut results) => {
                    results.pop().ok_or_else(|| {
                        Box::<dyn std::error::Error + Send + Sync>::from("No download results")
                    })?
                }
            }
        } else {
            builder.build().model(model_id).await?
        };

        // Extract files from progresshub result structure
        let files = match &result.models {
            ZeroOneOrMany::One(model) => {
                model.files.iter().map(|f| f.path.clone()).collect()
            },
            ZeroOneOrMany::Many(models) => {
                models.iter()
                    .flat_map(|m| m.files.iter().map(|f| f.path.clone()))
                    .collect()
            },
            ZeroOneOrMany::Zero => Vec::new(),
        };

        // Get cache directory from the first model or use default
        let cache_dir = match &result.models {
            ZeroOneOrMany::One(model) => model.model_cache_path.clone(),
            ZeroOneOrMany::Many(models) => {
                models.first()
                    .map_or_else(|| {
                        std::env::var("HF_HOME")
                            .ok()
                            .map_or_else(|| {
                                dirs::home_dir()
                                    .map_or_else(|| PathBuf::from(".cache/huggingface"), |h| h.join(".cache/huggingface"))
                            }, PathBuf::from)
                    }, |m| m.model_cache_path.clone())
            },
            ZeroOneOrMany::Zero => {
                std::env::var("HF_HOME")
                    .ok()
                    .map_or_else(|| {
                        dirs::home_dir()
                            .map_or_else(|| PathBuf::from(".cache/huggingface"), |h| h.join(".cache/huggingface"))
                    }, PathBuf::from)
            }
        };

        Ok(ModelDownloadResult {
            model_id: model_id.to_string(),
            files,
            total_bytes: result.total_downloaded_bytes,
            cache_dir,
        })
    }

    async fn is_cached(&self, _model_id: &str, _files: &[String]) -> bool {
        // ProgressHub handles caching internally
        // Could potentially check cache directory directly
        false // Conservative approach - let progresshub decide
    }

    fn cache_dir(&self) -> PathBuf {
        // Get from HF_HOME or default
        std::env::var("HF_HOME")
            .ok()
            .map_or_else(|| {
                dirs::home_dir()
                    .map_or_else(|| PathBuf::from(".cache/huggingface"), |h| h.join(".cache/huggingface"))
            }, PathBuf::from)
    }
}