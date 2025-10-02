use std::path::Path;
use std::sync::Arc;
use anyhow::Result;
use tokio::fs;
use log::{info, error};
use watchexec::Watchexec;
use watchexec_events::Tag;

use crate::config::Config;
use super::discovery::RootDiscovery;

pub struct RootConfigWatcher {
    config_path: String,
    discovery: Arc<RootDiscovery>,
}

impl RootConfigWatcher {
    pub fn new(config_path: String, discovery: Arc<RootDiscovery>) -> Self {
        Self { config_path, discovery }
    }
    
    pub async fn run(self) -> Result<()> {
        let config_path = self.config_path.clone();
        let discovery = self.discovery.clone();
        
        let wx = Watchexec::new(move |mut action| {
            for event in action.events.iter() {
                for tag in &event.tags {
                    if let Tag::Path { path, .. } = tag {
                        if path.to_str() == Some(&config_path) {
                            info!("Root config changed, reloading...");
                            
                            let discovery_clone = discovery.clone();
                            let config_path_clone = config_path.clone();
                            
                            tokio::spawn(async move {
                                match Self::reload_roots(
                                    &config_path_clone, 
                                    discovery_clone
                                ).await {
                                    Ok(count) => info!("Reloaded {} roots", count),
                                    Err(e) => error!("Failed to reload roots: {}", e),
                                }
                            });
                        }
                    }
                }
            }
            action
        })?;
        
        let config_dir = Path::new(&self.config_path)
            .parent()
            .expect("Config must have parent directory");
        
        wx.config.pathset(vec![config_dir.to_path_buf()]);
        wx.main().await?;
        
        Ok(())
    }
    
    async fn reload_roots(
        config_path: &str, 
        discovery: Arc<RootDiscovery>
    ) -> Result<usize> {
        let content = fs::read_to_string(config_path).await?;
        let config: Config = crate::config::parse_config_from_str(&content)?;
        
        // Update discovery config before reloading
        discovery.update_config(config.roots);
        let roots = discovery.load_roots().await;
        Ok(roots.len())
    }
}
