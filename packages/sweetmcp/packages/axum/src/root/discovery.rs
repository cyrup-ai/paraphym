use std::path::Path;
use std::sync::{Arc, RwLock};
use dashmap::DashMap;
use walkdir::WalkDir;
use crate::config::RootsConfig;
use crate::types::Root;

pub struct RootDiscovery {
    config: Arc<RwLock<RootsConfig>>,
    cache: Arc<DashMap<String, Root>>,
}

impl RootDiscovery {
    pub fn new(config: RootsConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            cache: Arc::new(DashMap::new()),
        }
    }
    
    /// Update configuration (for hot reload)
    pub fn update_config(&self, new_config: RootsConfig) {
        if let Ok(mut config) = self.config.write() {
            *config = new_config;
        }
    }
    
    /// Load all roots (static + discovered)
    pub async fn load_roots(&self) -> Vec<Root> {
        let mut roots = Vec::new();
        
        // Clone data from config while holding lock
        let (static_roots, scan_config) = {
            if let Ok(config) = self.config.read() {
                (config.static_roots.clone(), config.scan.clone())
            } else {
                return roots;  // Return empty on lock failure
            }
        };
        
        // Add static roots from config
        for entry in &static_roots {
            roots.push(Root {
                name: entry.name.clone(),
                url: entry.url.clone(),
            });
        }
        
        // Scan filesystem if configured
        if let Some(scan_cfg) = scan_config {
            let discovered = self.scan_filesystem(&scan_cfg).await;
            roots.extend(discovered);
        }
        
        // Update cache
        self.cache.clear();
        for root in &roots {
            self.cache.insert(root.name.clone(), root.clone());
        }
        
        roots
    }
    
    async fn scan_filesystem(&self, config: &crate::config::RootScanConfig) -> Vec<Root> {
        let mut discovered = Vec::new();
        
        for base_path in &config.paths {
            if !Path::new(base_path).exists() {
                continue;
            }
            
            for entry in WalkDir::new(base_path)
                .max_depth(config.max_depth)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_dir() {
                    // Check if directory contains any marker files
                    for marker in &config.markers {
                        let marker_path = entry.path().join(marker);
                        if marker_path.exists() {
                            let name = entry.path()
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();
                            
                            let url = format!("file://{}", entry.path().display());
                            
                            discovered.push(Root { name, url });
                            break; // Found marker, no need to check others
                        }
                    }
                }
            }
        }
        
        discovered
    }
    
    /// Get cached roots
    pub fn get_cached(&self) -> Vec<Root> {
        self.cache.iter().map(|kv| kv.value().clone()).collect()
    }
}
