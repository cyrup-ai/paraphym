use super::ModelDownloadProvider;

// Compile-time type alias for default download provider based on feature flags
// This enables static dispatch instead of dynamic dispatch through trait objects
#[cfg(feature = "download-hf-hub")]
pub type DefaultDownloadProvider = super::HfHubDownloadProvider;

#[cfg(all(feature = "download-progresshub", not(feature = "download-hf-hub")))]
pub type DefaultDownloadProvider = super::ProgressHubDownloadProvider;

/// Factory for creating download providers based on feature flags
pub struct DownloadProviderFactory;

impl DownloadProviderFactory {
    /// Create the default download provider based on enabled features
    ///
    /// Returns a concrete type selected at compile time via feature flags.
    /// Uses static dispatch - no heap allocation or virtual dispatch overhead.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No download backend features are enabled
    /// - The enabled backend fails to initialize
    #[cfg(any(feature = "download-hf-hub", feature = "download-progresshub"))]
    pub fn create_default() -> Result<DefaultDownloadProvider, Box<dyn std::error::Error + Send + Sync>> {
        #[cfg(feature = "download-hf-hub")]
        {
            return super::HfHubDownloadProvider::new();
        }

        #[cfg(all(feature = "download-progresshub", not(feature = "download-hf-hub")))]
        {
            return Ok(super::ProgressHubDownloadProvider::new());
        }
    }
    
    #[cfg(not(any(feature = "download-hf-hub", feature = "download-progresshub")))]
    pub fn create_default() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err(Box::<dyn std::error::Error + Send + Sync>::from(
            "No download backend enabled. Enable either 'download-hf-hub' or 'download-progresshub' feature."
        ))
    }

    /// Create provider with specific backend selection
    ///
    /// Note: This method uses dynamic dispatch since the backend is selected at runtime.
    /// For compile-time selection with static dispatch, use create_default() instead.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested backend feature is not enabled
    /// - The backend fails to initialize
    #[allow(dead_code)]
    pub fn create_with_backend(backend: DownloadBackend) -> Result<Box<dyn ModelDownloadProvider>, Box<dyn std::error::Error + Send + Sync>> {
        match backend {
            #[cfg(feature = "download-hf-hub")]
            DownloadBackend::HfHub => {
                use super::HfHubDownloadProvider;
                Ok(Box::new(HfHubDownloadProvider::new()?))
            }

            #[cfg(feature = "download-progresshub")]
            DownloadBackend::ProgressHub => {
                use super::ProgressHubDownloadProvider;
                Ok(Box::new(ProgressHubDownloadProvider::new()))
            }

            #[allow(unreachable_patterns)]
            _ => {
                Err(Box::<dyn std::error::Error + Send + Sync>::from(
                    format!("Backend {backend:?} not available. Enable the corresponding feature.")
                ))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DownloadBackend {
    HfHub,
    ProgressHub,
}