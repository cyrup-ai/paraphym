// memory_governor.rs - System-wide memory management and pressure handling

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::collections::BTreeMap;
use parking_lot::RwLock;
use sysinfo::{System, SystemExt, ProcessExt};
use tokio::sync::Semaphore;
use tracing::{info, warn, error};

/// Memory pressure levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryPressure {
    Low,      // < 50% usage
    Normal,   // 50-70% usage
    High,     // 70-85% usage
    Critical, // > 85% usage
}

/// System-wide memory governor
pub struct MemoryGovernor {
    /// Total system memory in MB
    total_system_mb: AtomicU64,
    
    /// Current allocated memory in MB
    allocated_mb: AtomicU64,
    
    /// Memory limit in MB (e.g., 80% of system)
    limit_mb: AtomicU64,
    
    /// Reserved memory for system in MB
    reserved_mb: AtomicU64,
    
    /// Current memory pressure
    pressure: Arc<RwLock<MemoryPressure>>,
    
    /// Memory allocations by model
    allocations: Arc<RwLock<BTreeMap<String, ModelMemory>>>,
    
    /// Memory pools for different sizes
    memory_pools: Arc<RwLock<MemoryPools>>,
    
    /// Allocation semaphore for backpressure
    allocation_sem: Arc<Semaphore>,
    
    /// Configuration
    config: MemoryConfig,
    
    /// System info handle
    system: Arc<RwLock<System>>,
}

#[derive(Clone)]
pub struct ModelMemory {
    pub model_name: String,
    pub workers: usize,
    pub per_worker_mb: usize,
    pub total_mb: usize,
    pub last_accessed: Instant,
    pub allocations: Vec<AllocationInfo>,
}

#[derive(Clone)]
pub struct AllocationInfo {
    pub worker_id: u64,
    pub size_mb: usize,
    pub allocated_at: Instant,
    pub numa_node: Option<usize>,
    pub huge_pages: bool,
}

pub struct MemoryPools {
    small_pool: Vec<MemoryChunk>,   // < 100 MB
    medium_pool: Vec<MemoryChunk>,  // 100-1000 MB
    large_pool: Vec<MemoryChunk>,   // > 1000 MB
}

pub struct MemoryChunk {
    size_mb: usize,
    allocated: bool,
    last_used: Instant,
    numa_node: Option<usize>,
}

#[derive(Clone)]
pub struct MemoryConfig {
    pub memory_limit_percent: f64,
    pub reserved_system_mb: usize,
    pub enable_huge_pages: bool,
    pub enable_numa_aware: bool,
    pub enable_memory_pools: bool,
    pub compaction_threshold: f64,
    pub pressure_check_interval: Duration,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            memory_limit_percent: 0.80,
            reserved_system_mb: 2048,  // Reserve 2GB for system
            enable_huge_pages: true,
            enable_numa_aware: true,
            enable_memory_pools: true,
            compaction_threshold: 0.75,  // Compact when 75% fragmented
            pressure_check_interval: Duration::from_secs(5),
        }
    }
}

impl MemoryGovernor {
    pub fn new(memory_limit_percent: f64) -> Self {
        let config = MemoryConfig {
            memory_limit_percent,
            ..Default::default()
        };
        Self::with_config(config)
    }
    
    pub fn with_config(config: MemoryConfig) -> Self {
        let mut system = System::new_all();
        system.refresh_memory();
        
        let total_system_mb = (system.total_memory() / 1024 / 1024) as u64;
        let limit_mb = ((total_system_mb as f64) * config.memory_limit_percent) as u64;
        let max_concurrent_allocs = 100;
        
        let governor = Self {
            total_system_mb: AtomicU64::new(total_system_mb),
            allocated_mb: AtomicU64::new(0),
            limit_mb: AtomicU64::new(limit_mb),
            reserved_mb: AtomicU64::new(config.reserved_system_mb as u64),
            pressure: Arc::new(RwLock::new(MemoryPressure::Low)),
            allocations: Arc::new(RwLock::new(BTreeMap::new())),
            memory_pools: Arc::new(RwLock::new(MemoryPools {
                small_pool: Vec::new(),
                medium_pool: Vec::new(),
                large_pool: Vec::new(),
            })),
            allocation_sem: Arc::new(Semaphore::new(max_concurrent_allocs)),
            config: config.clone(),
            system: Arc::new(RwLock::new(system)),
        };
        
        // Start pressure monitor
        governor.start_pressure_monitor();
        
        // Initialize memory pools if enabled
        if config.enable_memory_pools {
            governor.initialize_memory_pools();
        }
        
        governor
    }
    
    /// Try to allocate memory for a worker
    pub async fn try_allocate(&self, size_mb: usize) -> bool {
        // Acquire allocation permit
        let _permit = match self.allocation_sem.try_acquire() {
            Ok(p) => p,
            Err(_) => {
                warn!("Allocation semaphore exhausted, rejecting allocation");
                return false;
            }
        };
        
        let current = self.allocated_mb.load(Ordering::Acquire);
        let limit = self.limit_mb.load(Ordering::Acquire);
        let reserved = self.reserved_mb.load(Ordering::Acquire);
        
        // Check if allocation would exceed limit
        if current + size_mb as u64 > limit - reserved {
            // Try memory compaction
            if self.config.enable_memory_pools {
                self.compact_memory_pools();
                
                // Retry after compaction
                let current_after = self.allocated_mb.load(Ordering::Acquire);
                if current_after + size_mb as u64 > limit - reserved {
                    warn!(
                        "Memory allocation rejected: {} MB requested, {} MB available",
                        size_mb,
                        limit - reserved - current_after
                    );
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // Try pool allocation first
        if self.config.enable_memory_pools {
            if let Some(chunk) = self.allocate_from_pool(size_mb) {
                info!("Allocated {} MB from memory pool", size_mb);
                return true;
            }
        }
        
        // Direct allocation
        self.allocated_mb.fetch_add(size_mb as u64, Ordering::Release);
        
        // Update pressure
        self.update_pressure();
        
        // Enable huge pages if configured
        #[cfg(target_os = "linux")]
        if self.config.enable_huge_pages && size_mb >= 100 {
            self.enable_huge_pages_for_allocation(size_mb);
        }
        
        info!("Allocated {} MB (total: {} MB)", size_mb, current + size_mb as u64);
        true
    }
    
    /// Release allocated memory
    pub fn release(&self, size_mb: usize) {
        let previous = self.allocated_mb.fetch_sub(size_mb as u64, Ordering::Release);
        info!("Released {} MB (total: {} MB)", size_mb, previous - size_mb as u64);
        
        // Return to pool if enabled
        if self.config.enable_memory_pools {
            self.return_to_pool(size_mb);
        }
        
        // Update pressure
        self.update_pressure();
    }
    
    /// Register model allocation
    pub fn register_model_allocation(
        &self,
        model_name: &str,
        worker_id: u64,
        size_mb: usize,
    ) {
        let mut allocations = self.allocations.write();
        
        let allocation = AllocationInfo {
            worker_id,
            size_mb,
            allocated_at: Instant::now(),
            numa_node: self.get_numa_node(),
            huge_pages: self.config.enable_huge_pages,
        };
        
        allocations
            .entry(model_name.to_string())
            .and_modify(|m| {
                m.workers += 1;
                m.total_mb += size_mb;
                m.last_accessed = Instant::now();
                m.allocations.push(allocation.clone());
            })
            .or_insert_with(|| ModelMemory {
                model_name: model_name.to_string(),
                workers: 1,
                per_worker_mb: size_mb,
                total_mb: size_mb,
                last_accessed: Instant::now(),
                allocations: vec![allocation],
            });
    }
    
    /// Get current memory pressure
    pub fn get_pressure(&self) -> MemoryPressure {
        *self.pressure.read()
    }
    
    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        let allocated = self.allocated_mb.load(Ordering::Acquire);
        let total = self.total_system_mb.load(Ordering::Acquire);
        let limit = self.limit_mb.load(Ordering::Acquire);
        
        MemoryStats {
            total_system_mb: total,
            allocated_mb: allocated,
            available_mb: limit - allocated,
            limit_mb: limit,
            pressure: self.get_pressure(),
            utilization: (allocated as f64) / (total as f64),
        }
    }
    
    /// Get models sorted by memory usage
    pub fn get_models_by_memory(&self) -> Vec<ModelMemory> {
        let allocations = self.allocations.read();
        let mut models: Vec<_> = allocations.values().cloned().collect();
        models.sort_by(|a, b| b.total_mb.cmp(&a.total_mb));
        models
    }
    
    /// Suggest models to evict under memory pressure
    pub fn suggest_evictions(&self, target_mb: usize) -> Vec<String> {
        let models = self.get_models_by_memory();
        let mut eviction_list = Vec::new();
        let mut freed_mb = 0;
        
        // Start with least recently used models
        let mut lru_models = models.clone();
        lru_models.sort_by(|a, b| a.last_accessed.cmp(&b.last_accessed));
        
        for model in lru_models {
            if freed_mb >= target_mb {
                break;
            }
            
            eviction_list.push(model.model_name.clone());
            freed_mb += model.per_worker_mb;  // Evict one worker at a time
        }
        
        eviction_list
    }
    
    fn update_pressure(&self) {
        let allocated = self.allocated_mb.load(Ordering::Acquire);
        let total = self.total_system_mb.load(Ordering::Acquire);
        let usage_percent = (allocated as f64) / (total as f64);
        
        let new_pressure = match usage_percent {
            p if p < 0.50 => MemoryPressure::Low,
            p if p < 0.70 => MemoryPressure::Normal,
            p if p < 0.85 => MemoryPressure::High,
            _ => MemoryPressure::Critical,
        };
        
        let mut pressure = self.pressure.write();
        if *pressure != new_pressure {
            info!("Memory pressure changed: {:?} -> {:?}", *pressure, new_pressure);
            *pressure = new_pressure;
        }
    }
    
    fn start_pressure_monitor(&self) {
        let governor = self.clone();
        let interval = self.config.pressure_check_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval);
            loop {
                interval.tick().await;
                
                // Refresh system memory info
                governor.system.write().refresh_memory();
                
                // Update pressure
                governor.update_pressure();
                
                // Check for critical pressure
                if governor.get_pressure() == MemoryPressure::Critical {
                    warn!("Critical memory pressure detected!");
                    
                    // Suggest evictions
                    let target = (governor.allocated_mb.load(Ordering::Acquire) / 10) as usize;
                    let evictions = governor.suggest_evictions(target);
                    
                    if !evictions.is_empty() {
                        warn!("Suggested evictions: {:?}", evictions);
                    }
                }
            }
        });
    }
    
    fn initialize_memory_pools(&self) {
        // Pre-allocate memory chunks for pooling
        let mut pools = self.memory_pools.write();
        
        // Create small chunks (50 MB each)
        for _ in 0..10 {
            pools.small_pool.push(MemoryChunk {
                size_mb: 50,
                allocated: false,
                last_used: Instant::now(),
                numa_node: self.get_numa_node(),
            });
        }
        
        // Create medium chunks (500 MB each)
        for _ in 0..5 {
            pools.medium_pool.push(MemoryChunk {
                size_mb: 500,
                allocated: false,
                last_used: Instant::now(),
                numa_node: self.get_numa_node(),
            });
        }
        
        // Create large chunks (2000 MB each)
        for _ in 0..2 {
            pools.large_pool.push(MemoryChunk {
                size_mb: 2000,
                allocated: false,
                last_used: Instant::now(),
                numa_node: self.get_numa_node(),
            });
        }
    }
    
    fn allocate_from_pool(&self, size_mb: usize) -> Option<MemoryChunk> {
        let mut pools = self.memory_pools.write();
        
        let pool = if size_mb <= 100 {
            &mut pools.small_pool
        } else if size_mb <= 1000 {
            &mut pools.medium_pool
        } else {
            &mut pools.large_pool
        };
        
        // Find best fit chunk
        pool.iter_mut()
            .find(|chunk| !chunk.allocated && chunk.size_mb >= size_mb)
            .map(|chunk| {
                chunk.allocated = true;
                chunk.last_used = Instant::now();
                chunk.clone()
            })
    }
    
    fn return_to_pool(&self, size_mb: usize) {
        let mut pools = self.memory_pools.write();
        
        let pool = if size_mb <= 100 {
            &mut pools.small_pool
        } else if size_mb <= 1000 {
            &mut pools.medium_pool
        } else {
            &mut pools.large_pool
        };
        
        // Mark chunk as available
        if let Some(chunk) = pool.iter_mut().find(|c| c.allocated && c.size_mb == size_mb) {
            chunk.allocated = false;
            chunk.last_used = Instant::now();
        }
    }
    
    fn compact_memory_pools(&self) {
        info!("Running memory pool compaction");
        
        let mut pools = self.memory_pools.write();
        
        // Defragment each pool
        pools.small_pool.retain(|chunk| {
            chunk.allocated || chunk.last_used.elapsed() < Duration::from_secs(300)
        });
        
        pools.medium_pool.retain(|chunk| {
            chunk.allocated || chunk.last_used.elapsed() < Duration::from_secs(300)
        });
        
        pools.large_pool.retain(|chunk| {
            chunk.allocated || chunk.last_used.elapsed() < Duration::from_secs(300)
        });
    }
    
    #[cfg(target_os = "linux")]
    fn enable_huge_pages_for_allocation(&self, size_mb: usize) {
        use std::process::Command;
        
        let pages_needed = size_mb / 2;  // 2MB huge pages
        
        let output = Command::new("sysctl")
            .arg("-w")
            .arg(format!("vm.nr_hugepages={}", pages_needed))
            .output();
        
        match output {
            Ok(o) if o.status.success() => {
                info!("Enabled {} huge pages for allocation", pages_needed);
            }
            _ => {
                warn!("Failed to enable huge pages");
            }
        }
    }
    
    #[cfg(not(target_os = "linux"))]
    fn enable_huge_pages_for_allocation(&self, _size_mb: usize) {
        // No-op on non-Linux systems
    }
    
    fn get_numa_node(&self) -> Option<usize> {
        #[cfg(target_os = "linux")]
        if self.config.enable_numa_aware {
            // Get current NUMA node
            std::fs::read_to_string("/proc/self/numa_node")
                .ok()
                .and_then(|s| s.trim().parse().ok())
        } else {
            None
        }
        
        #[cfg(not(target_os = "linux"))]
        None
    }
}

impl Clone for MemoryGovernor {
    fn clone(&self) -> Self {
        Self {
            total_system_mb: AtomicU64::new(self.total_system_mb.load(Ordering::Acquire)),
            allocated_mb: AtomicU64::new(self.allocated_mb.load(Ordering::Acquire)),
            limit_mb: AtomicU64::new(self.limit_mb.load(Ordering::Acquire)),
            reserved_mb: AtomicU64::new(self.reserved_mb.load(Ordering::Acquire)),
            pressure: self.pressure.clone(),
            allocations: self.allocations.clone(),
            memory_pools: self.memory_pools.clone(),
            allocation_sem: self.allocation_sem.clone(),
            config: self.config.clone(),
            system: self.system.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_system_mb: u64,
    pub allocated_mb: u64,
    pub available_mb: u64,
    pub limit_mb: u64,
    pub pressure: MemoryPressure,
    pub utilization: f64,
}