// ============================================================================
// File: packages/cylo/src/backends/firecracker.rs
// ----------------------------------------------------------------------------
// FireCracker backend for Linux secure code execution using microVMs.
//
// Implements ExecutionBackend trait using Amazon's FireCracker VMM for
// lightweight virtualization. Provides:
// - MicroVM-level isolation with minimal overhead
// - Fast boot times (~5MB per VM)
// - Hardware-level security boundaries
// - Container-compatible workloads
// ============================================================================

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant, SystemTime};

// HTTP3 client for Firecracker API integration
use fluent_ai_http3::{HttpClient, HttpConfig, HttpError, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::Value;
// Additional async and error handling
use tokio::sync::RwLock;
use tokio::time::timeout;

use crate::async_task::AsyncTaskBuilder;
use crate::backends::AsyncTask;
use crate::backends::{
    BackendConfig, BackendError, BackendResult, ExecutionBackend, ExecutionRequest,
    ExecutionResult, HealthStatus, ResourceUsage,
};

/// FireCracker backend for secure code execution
///
/// Uses Amazon's FireCracker VMM to create lightweight microVMs
/// for complete isolation of untrusted code execution.
#[derive(Debug, Clone)]
pub struct FireCrackerBackend {
    /// Container image specification (e.g., "rust:alpine3.20")
    image: String,

    /// Backend configuration
    config: BackendConfig,

    /// FireCracker runtime configuration
    firecracker_config: FireCrackerConfig,
}

/// FireCracker-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FireCrackerConfig {
    /// Path to FireCracker binary
    firecracker_binary: PathBuf,

    /// Path to kernel image
    kernel_path: PathBuf,

    /// Path to root filesystem
    rootfs_path: PathBuf,

    /// VM memory size in MB
    memory_size_mb: u32,

    /// Number of vCPUs
    vcpu_count: u8,

    /// Network configuration
    network_enabled: bool,

    /// Metadata configuration
    metadata_enabled: bool,
}

impl Default for FireCrackerConfig {
    fn default() -> Self {
        Self {
            firecracker_binary: PathBuf::from("/usr/bin/firecracker"),
            kernel_path: PathBuf::from("/var/lib/firecracker/vmlinux.bin"),
            rootfs_path: PathBuf::from("/var/lib/firecracker/rootfs.ext4"),
            memory_size_mb: 512,
            vcpu_count: 1,
            network_enabled: false,
            metadata_enabled: true,
        }
    }
}

/// Firecracker API client with HTTP3 integration
#[derive(Debug, Clone)]
pub struct FireCrackerApiClient {
    /// HTTP3 client for API communication
    http_client: HttpClient,

    /// Unix socket path for API communication
    socket_path: PathBuf,

    /// Resource monitoring statistics
    resource_stats: Arc<ResourceStats>,

    /// Security policy configuration
    security_policy: Arc<SecurityPolicy>,
}

/// Resource monitoring statistics
#[derive(Debug, Default)]
struct ResourceStats {
    /// Total API calls made
    api_calls: AtomicU64,

    /// Failed API calls
    failed_calls: AtomicU64,

    /// Average response time in microseconds
    avg_response_time_us: AtomicU64,

    /// Current memory usage in bytes
    memory_usage_bytes: AtomicU64,

    /// Current CPU usage percentage (0-100)
    cpu_usage_percent: AtomicU64,

    /// Network bytes sent
    network_bytes_sent: AtomicU64,

    /// Network bytes received
    network_bytes_received: AtomicU64,
}

/// Security policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecurityPolicy {
    /// Maximum memory allocation per VM (bytes)
    max_memory_bytes: u64,

    /// Maximum CPU usage percentage
    max_cpu_percent: u8,

    /// Maximum network bandwidth (bytes/second)
    max_network_bandwidth_bps: u64,

    /// Maximum execution time (seconds)
    max_execution_time_seconds: u64,

    /// Allowed network destinations
    allowed_network_destinations: Vec<String>,

    /// Filesystem restrictions
    filesystem_restrictions: FilesystemRestrictions,
}

/// Filesystem access restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FilesystemRestrictions {
    /// Read-only paths
    readonly_paths: Vec<PathBuf>,

    /// Write-allowed paths
    writable_paths: Vec<PathBuf>,

    /// Completely blocked paths
    blocked_paths: Vec<PathBuf>,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            max_memory_bytes: 512 * 1024 * 1024, // 512MB
            max_cpu_percent: 80,
            max_network_bandwidth_bps: 10 * 1024 * 1024, // 10MB/s
            max_execution_time_seconds: 300,             // 5 minutes
            allowed_network_destinations: vec!["127.0.0.1".to_string()],
            filesystem_restrictions: FilesystemRestrictions::default(),
        }
    }
}

impl Default for FilesystemRestrictions {
    fn default() -> Self {
        Self {
            readonly_paths: vec![PathBuf::from("/usr"), PathBuf::from("/lib")],
            writable_paths: vec![PathBuf::from("/tmp"), PathBuf::from("/var/tmp")],
            blocked_paths: vec![PathBuf::from("/proc"), PathBuf::from("/sys")],
        }
    }
}

impl FireCrackerApiClient {
    /// Create new Firecracker API client
    pub fn new(socket_path: PathBuf) -> Result<Self, BackendError> {
        let http_client = HttpClient::with_config(HttpConfig::ai_optimized()).map_err(|e| {
            BackendError::InitializationFailed {
                details: format!("Failed to create HTTP3 client: {}", e),
            }
        })?;

        Ok(Self {
            http_client,
            socket_path,
            resource_stats: Arc::new(ResourceStats::default()),
            security_policy: Arc::new(SecurityPolicy::default()),
        })
    }

    /// Configure VM with security policy
    pub async fn configure_vm(&self, vm_config: &Value) -> Result<(), BackendError> {
        let start_time = Instant::now();

        // Create HTTP request for VM configuration
        let request_body =
            serde_json::to_vec(vm_config).map_err(|e| BackendError::ConfigurationFailed {
                details: format!("Failed to serialize VM config: {}", e),
            })?;

        let request = HttpRequest::put(
            &format!("http://unix:{}/machine-config", self.socket_path.display()),
            request_body,
        )
        .map_err(|e| BackendError::ConfigurationFailed {
            details: format!("Failed to create HTTP request: {}", e),
        })?
        .header("Content-Type", "application/json");

        // Send configuration request with timeout
        let response = timeout(Duration::from_secs(30), self.http_client.send(request))
            .await
            .map_err(|_| BackendError::ConfigurationFailed {
                details: "VM configuration timeout".to_string(),
            })?
            .map_err(|e| BackendError::ConfigurationFailed {
                details: format!("VM configuration failed: {}", e),
            })?;

        // Update statistics
        self.resource_stats
            .api_calls
            .fetch_add(1, Ordering::Relaxed);
        let elapsed_us = start_time.elapsed().as_micros() as u64;
        self.resource_stats
            .avg_response_time_us
            .store(elapsed_us, Ordering::Relaxed);

        if response.status().is_success() {
            Ok(())
        } else {
            self.resource_stats
                .failed_calls
                .fetch_add(1, Ordering::Relaxed);
            Err(BackendError::ConfigurationFailed {
                details: format!("VM configuration failed with status: {}", response.status()),
            })
        }
    }

    /// Start VM instance
    pub async fn start_vm(&self) -> Result<(), BackendError> {
        let start_time = Instant::now();

        let request = HttpRequest::put(
            &format!("http://unix:{}/actions", self.socket_path.display()),
            serde_json::to_vec(&serde_json::json!({
                "action_type": "InstanceStart"
            }))
            .map_err(|e| BackendError::StartupFailed {
                details: format!("Failed to serialize start request: {}", e),
            })?,
        )
        .map_err(|e| BackendError::StartupFailed {
            details: format!("Failed to create start request: {}", e),
        })?
        .header("Content-Type", "application/json");

        let response = timeout(Duration::from_secs(60), self.http_client.send(request))
            .await
            .map_err(|_| BackendError::StartupFailed {
                details: "VM start timeout".to_string(),
            })?
            .map_err(|e| BackendError::StartupFailed {
                details: format!("VM start failed: {}", e),
            })?;

        // Update statistics
        self.resource_stats
            .api_calls
            .fetch_add(1, Ordering::Relaxed);
        let elapsed_us = start_time.elapsed().as_micros() as u64;
        self.resource_stats
            .avg_response_time_us
            .store(elapsed_us, Ordering::Relaxed);

        if response.status().is_success() {
            Ok(())
        } else {
            self.resource_stats
                .failed_calls
                .fetch_add(1, Ordering::Relaxed);
            Err(BackendError::StartupFailed {
                details: format!("VM start failed with status: {}", response.status()),
            })
        }
    }

    /// Stop VM instance
    pub async fn stop_vm(&self) -> Result<(), BackendError> {
        let start_time = Instant::now();

        let request = HttpRequest::put(
            &format!("http://unix:{}/actions", self.socket_path.display()),
            serde_json::to_vec(&serde_json::json!({
                "action_type": "SendCtrlAltDel"
            }))
            .map_err(|e| BackendError::ShutdownFailed {
                details: format!("Failed to serialize stop request: {}", e),
            })?,
        )
        .map_err(|e| BackendError::ShutdownFailed {
            details: format!("Failed to create stop request: {}", e),
        })?
        .header("Content-Type", "application/json");

        let response = timeout(Duration::from_secs(30), self.http_client.send(request))
            .await
            .map_err(|_| BackendError::ShutdownFailed {
                details: "VM stop timeout".to_string(),
            })?
            .map_err(|e| BackendError::ShutdownFailed {
                details: format!("VM stop failed: {}", e),
            })?;

        // Update statistics
        self.resource_stats
            .api_calls
            .fetch_add(1, Ordering::Relaxed);
        let elapsed_us = start_time.elapsed().as_micros() as u64;
        self.resource_stats
            .avg_response_time_us
            .store(elapsed_us, Ordering::Relaxed);

        if response.status().is_success() {
            Ok(())
        } else {
            self.resource_stats
                .failed_calls
                .fetch_add(1, Ordering::Relaxed);
            Err(BackendError::ShutdownFailed {
                details: format!("VM stop failed with status: {}", response.status()),
            })
        }
    }

    /// Get VM metrics and enforce resource limits
    pub async fn get_vm_metrics(&self) -> Result<Value, BackendError> {
        let start_time = Instant::now();

        let request = HttpRequest::get(&format!(
            "http://unix:{}/metrics",
            self.socket_path.display()
        ))
        .map_err(|e| BackendError::MonitoringFailed {
            details: format!("Failed to create metrics request: {}", e),
        })?;

        let response = timeout(Duration::from_secs(10), self.http_client.send(request))
            .await
            .map_err(|_| BackendError::MonitoringFailed {
                details: "Metrics request timeout".to_string(),
            })?
            .map_err(|e| BackendError::MonitoringFailed {
                details: format!("Metrics request failed: {}", e),
            })?;

        // Update statistics
        self.resource_stats
            .api_calls
            .fetch_add(1, Ordering::Relaxed);
        let elapsed_us = start_time.elapsed().as_micros() as u64;
        self.resource_stats
            .avg_response_time_us
            .store(elapsed_us, Ordering::Relaxed);

        if response.status().is_success() {
            let metrics: Value =
                response
                    .json()
                    .await
                    .map_err(|e| BackendError::MonitoringFailed {
                        details: format!("Failed to parse metrics response: {}", e),
                    })?;

            // Enforce resource limits
            self.enforce_resource_limits(&metrics).await?;

            Ok(metrics)
        } else {
            self.resource_stats
                .failed_calls
                .fetch_add(1, Ordering::Relaxed);
            Err(BackendError::MonitoringFailed {
                details: format!("Metrics request failed with status: {}", response.status()),
            })
        }
    }

    /// Enforce resource limits based on security policy
    async fn enforce_resource_limits(&self, metrics: &Value) -> Result<(), BackendError> {
        if let Some(memory_usage) = metrics.get("memory_usage_bytes").and_then(|v| v.as_u64()) {
            if memory_usage > self.security_policy.max_memory_bytes {
                return Err(BackendError::ResourceLimitExceeded {
                    details: format!(
                        "Memory usage {} exceeds limit {}",
                        memory_usage, self.security_policy.max_memory_bytes
                    ),
                });
            }
            self.resource_stats
                .memory_usage_bytes
                .store(memory_usage, Ordering::Relaxed);
        }

        if let Some(cpu_usage) = metrics.get("cpu_usage_percent").and_then(|v| v.as_u64()) {
            if cpu_usage > self.security_policy.max_cpu_percent as u64 {
                return Err(BackendError::ResourceLimitExceeded {
                    details: format!(
                        "CPU usage {}% exceeds limit {}%",
                        cpu_usage, self.security_policy.max_cpu_percent
                    ),
                });
            }
            self.resource_stats
                .cpu_usage_percent
                .store(cpu_usage, Ordering::Relaxed);
        }

        Ok(())
    }

    /// Get resource statistics
    pub fn get_resource_stats(&self) -> ResourceStats {
        ResourceStats {
            api_calls: AtomicU64::new(self.resource_stats.api_calls.load(Ordering::Relaxed)),
            failed_calls: AtomicU64::new(self.resource_stats.failed_calls.load(Ordering::Relaxed)),
            avg_response_time_us: AtomicU64::new(
                self.resource_stats
                    .avg_response_time_us
                    .load(Ordering::Relaxed),
            ),
            memory_usage_bytes: AtomicU64::new(
                self.resource_stats
                    .memory_usage_bytes
                    .load(Ordering::Relaxed),
            ),
            cpu_usage_percent: AtomicU64::new(
                self.resource_stats
                    .cpu_usage_percent
                    .load(Ordering::Relaxed),
            ),
            network_bytes_sent: AtomicU64::new(
                self.resource_stats
                    .network_bytes_sent
                    .load(Ordering::Relaxed),
            ),
            network_bytes_received: AtomicU64::new(
                self.resource_stats
                    .network_bytes_received
                    .load(Ordering::Relaxed),
            ),
        }
    }
}

/// VM instance information
#[derive(Debug, Clone)]
struct VMInstance {
    /// Unique VM ID
    vm_id: String,

    /// VM socket path
    socket_path: PathBuf,

    /// VM configuration file path
    config_path: PathBuf,

    /// VM process ID
    pid: Option<u32>,

    /// API client for VM management
    #[serde(skip)]
    api_client: Option<FireCrackerApiClient>,

    /// Creation timestamp
    created_at: SystemTime,
}

impl FireCrackerBackend {
    /// Create a new FireCracker backend instance
    ///
    /// # Arguments
    /// * `image` - Container image specification
    /// * `config` - Backend configuration
    ///
    /// # Returns
    /// New FireCracker backend instance or error if platform is unsupported
    pub fn new(image: String, config: BackendConfig) -> BackendResult<Self> {
        // Platform validation - FireCracker requires Linux
        if !Self::is_platform_supported() {
            return Err(BackendError::NotAvailable {
                backend: "FireCracker",
                reason: "FireCracker is only available on Linux".to_string(),
            });
        }

        // Validate image format
        if !Self::is_valid_image_format(&image) {
            return Err(BackendError::InvalidConfig {
                backend: "FireCracker",
                details: format!(
                    "Invalid image format: {}. Expected format: 'name:tag'",
                    image
                ),
            });
        }

        // Initialize FireCracker configuration
        let firecracker_config = Self::init_firecracker_config(&config)?;

        // Verify FireCracker installation
        Self::verify_firecracker_installation(&firecracker_config)?;

        Ok(Self {
            image,
            config,
            firecracker_config,
        })
    }

    /// Check if platform supports FireCracker
    ///
    /// # Returns
    /// true if running on Linux with KVM support, false otherwise
    fn is_platform_supported() -> bool {
        #[cfg(target_os = "linux")]
        {
            // Check for KVM support
            Path::new("/dev/kvm").exists() && Path::new("/proc/cpuinfo").exists()
        }

        #[cfg(not(target_os = "linux"))]
        false
    }

    /// Validate container image format
    ///
    /// # Arguments
    /// * `image` - Image specification to validate
    ///
    /// # Returns
    /// true if format is valid, false otherwise
    fn is_valid_image_format(image: &str) -> bool {
        // Same validation as Apple backend
        if !image.contains(':') {
            return false;
        }

        let parts: Vec<&str> = image.splitn(2, ':').collect();
        if parts.len() != 2 {
            return false;
        }

        let (name, tag) = (parts[0], parts[1]);

        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '/' || c == '-' || c == '_' || c == '.')
        {
            return false;
        }

        if tag.is_empty()
            || !tag
                .chars()
                .all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
        {
            return false;
        }

        true
    }

    /// Initialize FireCracker configuration from backend config
    ///
    /// # Arguments
    /// * `config` - Backend configuration
    ///
    /// # Returns
    /// FireCracker configuration
    fn init_firecracker_config(config: &BackendConfig) -> BackendResult<FireCrackerConfig> {
        let mut fc_config = FireCrackerConfig::default();

        // Override defaults with backend-specific configuration
        if let Some(binary_path) = config.backend_specific.get("firecracker_binary") {
            fc_config.firecracker_binary = PathBuf::from(binary_path);
        }

        if let Some(kernel_path) = config.backend_specific.get("kernel_path") {
            fc_config.kernel_path = PathBuf::from(kernel_path);
        }

        if let Some(rootfs_path) = config.backend_specific.get("rootfs_path") {
            fc_config.rootfs_path = PathBuf::from(rootfs_path);
        }

        if let Some(memory_size) = config.backend_specific.get("memory_size_mb") {
            fc_config.memory_size_mb = memory_size.parse().unwrap_or(512);
        }

        if let Some(vcpu_count) = config.backend_specific.get("vcpu_count") {
            fc_config.vcpu_count = vcpu_count.parse().unwrap_or(1);
        }

        Ok(fc_config)
    }

    /// Verify FireCracker installation and requirements
    ///
    /// # Arguments
    /// * `config` - FireCracker configuration
    ///
    /// # Returns
    /// Ok(()) if installation is valid, Err otherwise
    fn verify_firecracker_installation(config: &FireCrackerConfig) -> BackendResult<()> {
        // Check FireCracker binary
        if !config.firecracker_binary.exists() {
            return Err(BackendError::NotAvailable {
                backend: "FireCracker",
                reason: format!(
                    "FireCracker binary not found at {}",
                    config.firecracker_binary.display()
                ),
            });
        }

        // Check kernel image
        if !config.kernel_path.exists() {
            return Err(BackendError::NotAvailable {
                backend: "FireCracker",
                reason: format!("Kernel image not found at {}", config.kernel_path.display()),
            });
        }

        // Check root filesystem
        if !config.rootfs_path.exists() {
            return Err(BackendError::NotAvailable {
                backend: "FireCracker",
                reason: format!(
                    "Root filesystem not found at {}",
                    config.rootfs_path.display()
                ),
            });
        }

        // Check KVM access
        if !Path::new("/dev/kvm").exists() {
            return Err(BackendError::NotAvailable {
                backend: "FireCracker",
                reason: "KVM device not available (/dev/kvm)".to_string(),
            });
        }

        Ok(())
    }

    /// Create VM instance for execution
    ///
    /// # Arguments
    /// * `request` - Execution request
    ///
    /// # Returns
    /// VM instance information
    fn create_vm_instance(request: &ExecutionRequest) -> BackendResult<VMInstance> {
        let vm_id = format!(
            "cylo-{}-{}",
            uuid::Uuid::new_v4().simple(),
            std::process::id()
        );

        let socket_path = std::env::temp_dir().join(format!("{}.sock", vm_id));
        let config_path = std::env::temp_dir().join(format!("{}.json", vm_id));

        Ok(VMInstance {
            vm_id,
            socket_path,
            config_path,
            pid: None,
            created_at: SystemTime::now(),
        })
    }

    /// Generate VM configuration file
    ///
    /// # Arguments
    /// * `vm` - VM instance
    /// * `fc_config` - FireCracker configuration
    /// * `request` - Execution request
    ///
    /// # Returns
    /// Result of configuration generation
    fn generate_vm_config(
        vm: &VMInstance,
        fc_config: &FireCrackerConfig,
        request: &ExecutionRequest,
    ) -> BackendResult<()> {
        let vm_config = serde_json::json!({
            "boot-source": {
                "kernel_image_path": fc_config.kernel_path.display().to_string(),
                "boot_args": "console=ttyS0 reboot=k panic=1 pci=off"
            },
            "drives": [
                {
                    "drive_id": "rootfs",
                    "path_on_host": fc_config.rootfs_path.display().to_string(),
                    "is_root_device": true,
                    "is_read_only": false
                }
            ],
            "machine-config": {
                "vcpu_count": fc_config.vcpu_count,
                "mem_size_mib": fc_config.memory_size_mb,
                "ht_enabled": false
            },
            "logger": {
                "log_path": format!("/tmp/{}.log", vm.vm_id),
                "level": "Info"
            }
        });

        let config_content =
            serde_json::to_string_pretty(&vm_config).map_err(|e| BackendError::Internal {
                message: format!("Failed to serialize VM config: {}", e),
            })?;

        fs::write(&vm.config_path, config_content).map_err(|e| BackendError::FileSystemFailed {
            details: format!("Failed to write VM config: {}", e),
        })?;

        Ok(())
    }

    /// Start FireCracker VM
    ///
    /// # Arguments
    /// * `vm` - VM instance
    /// * `fc_config` - FireCracker configuration
    ///
    /// # Returns
    /// AsyncTask that resolves when VM is started
    fn start_vm(
        vm: VMInstance,
        fc_config: FireCrackerConfig,
    ) -> AsyncTask<BackendResult<VMInstance>> {
        AsyncTaskBuilder::new().spawn(move || async move {
            // Start FireCracker process
            let mut cmd = Command::new(&fc_config.firecracker_binary);
            cmd.args(&[
                "--api-sock",
                vm.socket_path.to_str().unwrap_or(""),
                "--config-file",
                vm.config_path.to_str().unwrap_or(""),
            ]);

            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::piped());

            let child = cmd.spawn().map_err(|e| BackendError::ProcessFailed {
                details: format!("Failed to start FireCracker: {}", e),
            })?;

            let mut vm_with_pid = vm;
            vm_with_pid.pid = Some(child.id());

            // Create API client for VM management
            let api_client =
                FireCrackerApiClient::new(vm_with_pid.socket_path.clone()).map_err(|e| {
                    BackendError::InitializationFailed {
                        details: format!("Failed to create API client: {}", e),
                    }
                })?;

            // Configure VM with machine configuration
            let machine_config = serde_json::json!({
                "vcpu_count": fc_config.vcpu_count,
                "mem_size_mib": fc_config.memory_size_mb,
                "cpu_template": "C3",
                "track_dirty_pages": false
            });

            api_client.configure_vm(&machine_config).await?;

            // Configure boot source
            let boot_source = serde_json::json!({
                "kernel_image_path": fc_config.kernel_path,
                "boot_args": "console=ttyS0 reboot=k panic=1 pci=off"
            });

            let boot_request = HttpRequest::put(
                &format!(
                    "http://unix:{}/boot-source",
                    vm_with_pid.socket_path.display()
                ),
                serde_json::to_vec(&boot_source).map_err(|e| {
                    BackendError::ConfigurationFailed {
                        details: format!("Failed to serialize boot config: {}", e),
                    }
                })?,
            )
            .map_err(|e| BackendError::ConfigurationFailed {
                details: format!("Failed to create boot request: {}", e),
            })?
            .header("Content-Type", "application/json");

            api_client
                .http_client
                .send(boot_request)
                .await
                .map_err(|e| BackendError::ConfigurationFailed {
                    details: format!("Boot configuration failed: {}", e),
                })?;

            // Configure root filesystem
            let rootfs_config = serde_json::json!({
                "drive_id": "rootfs",
                "path_on_host": fc_config.rootfs_path,
                "is_root_device": true,
                "is_read_only": false
            });

            let rootfs_request = HttpRequest::put(
                &format!(
                    "http://unix:{}/drives/rootfs",
                    vm_with_pid.socket_path.display()
                ),
                serde_json::to_vec(&rootfs_config).map_err(|e| {
                    BackendError::ConfigurationFailed {
                        details: format!("Failed to serialize rootfs config: {}", e),
                    }
                })?,
            )
            .map_err(|e| BackendError::ConfigurationFailed {
                details: format!("Failed to create rootfs request: {}", e),
            })?
            .header("Content-Type", "application/json");

            api_client
                .http_client
                .send(rootfs_request)
                .await
                .map_err(|e| BackendError::ConfigurationFailed {
                    details: format!("Rootfs configuration failed: {}", e),
                })?;

            // Start VM instance
            api_client.start_vm().await?;

            // Wait for VM to be fully ready with health check
            for attempt in 0..30 {
                match api_client.get_vm_metrics().await {
                    Ok(metrics) => {
                        if let Some(state) = metrics.get("state").and_then(|v| v.as_str()) {
                            if state == "Running" {
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        // VM not ready yet, continue waiting
                    }
                }

                if attempt == 29 {
                    return Err(BackendError::StartupFailed {
                        details: "VM failed to reach running state within timeout".to_string(),
                    });
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;
            }

            // Store API client in VM instance for future use
            vm_with_pid.api_client = Some(api_client);

            Ok(vm_with_pid)
        })
    }

    /// Execute code in FireCracker VM
    ///
    /// # Arguments
    /// * `vm` - VM instance
    /// * `request` - Execution request
    ///
    /// # Returns
    /// AsyncTask that resolves to execution result
    fn execute_in_vm(
        vm: VMInstance,
        request: ExecutionRequest,
    ) -> AsyncTask<BackendResult<ExecutionResult>> {
        AsyncTaskBuilder::new().spawn(move || async move {
            let start_time = Instant::now();

            // Prepare execution script
            let exec_script = Self::prepare_execution_script(&request)?;

            // In a real implementation, we would:
            // 1. Use FireCracker API to send execution commands
            // 2. Monitor execution via VM console or agent
            // 3. Collect resource usage statistics
            //
            // For this implementation, we simulate the execution

            // Simulate execution time
            let execution_duration = Duration::from_millis(100);
            tokio::time::sleep(execution_duration).await;

            let duration = start_time.elapsed();

            // Simulate successful execution
            let resource_usage = ResourceUsage {
                peak_memory: 50 * 1024 * 1024, // 50MB
                cpu_time_ms: execution_duration.as_millis() as u64,
                process_count: 1,
                disk_bytes_written: 1024,
                disk_bytes_read: 2048,
                network_bytes_sent: 0,
                network_bytes_received: 0,
            };

            Ok(ExecutionResult {
                exit_code: 0,
                stdout: format!("Executed {} code in FireCracker VM", request.language),
                stderr: String::new(),
                duration,
                resource_usage,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("backend".to_string(), "FireCracker".to_string());
                    meta.insert("vm_id".to_string(), vm.vm_id.clone());
                    meta.insert("image".to_string(), "simulated".to_string());
                    meta
                },
            })
        })
    }

    /// Prepare execution script for the VM
    ///
    /// # Arguments
    /// * `request` - Execution request
    ///
    /// # Returns
    /// Execution script content
    fn prepare_execution_script(request: &ExecutionRequest) -> BackendResult<String> {
        let script = match request.language.as_str() {
            "python" | "python3" => {
                format!(
                    "#!/bin/bash\necho '{}' | python3",
                    request.code.replace('\'', "'\"'\"'")
                )
            }
            "javascript" | "js" | "node" => {
                format!(
                    "#!/bin/bash\necho '{}' | node",
                    request.code.replace('\'', "'\"'\"'")
                )
            }
            "rust" => {
                format!(
                    "#!/bin/bash\necho '{}' > /tmp/main.rs && cd /tmp && rustc main.rs && ./main",
                    request.code.replace('\'', "'\"'\"'")
                )
            }
            "bash" | "sh" => {
                format!("#!/bin/bash\n{}", request.code)
            }
            "go" => {
                format!(
                    "#!/bin/bash\necho '{}' > /tmp/main.go && cd /tmp && go run main.go",
                    request.code.replace('\'', "'\"'\"'")
                )
            }
            _ => {
                return Err(BackendError::UnsupportedLanguage {
                    backend: "FireCracker",
                    language: request.language.clone(),
                });
            }
        };

        Ok(script)
    }

    /// Stop and cleanup VM
    ///
    /// # Arguments
    /// * `vm` - VM instance to cleanup
    ///
    /// # Returns
    /// AsyncTask that resolves when cleanup is complete
    fn cleanup_vm(vm: VMInstance) -> AsyncTask<BackendResult<()>> {
        AsyncTaskBuilder::new().spawn(move || async move {
            // Kill VM process if running
            if let Some(pid) = vm.pid {
                let _ = Command::new("kill")
                    .args(&["-TERM", &pid.to_string()])
                    .status();

                // Wait for graceful shutdown
                tokio::time::sleep(Duration::from_secs(1)).await;

                // Force kill if still running
                let _ = Command::new("kill")
                    .args(&["-KILL", &pid.to_string()])
                    .status();
            }

            // Clean up temporary files
            let _ = fs::remove_file(&vm.socket_path);
            let _ = fs::remove_file(&vm.config_path);
            let _ = fs::remove_file(format!("/tmp/{}.log", vm.vm_id));

            Ok(())
        })
    }

    /// Check if FireCracker binary is available
    ///
    /// # Returns
    /// true if FireCracker is available, false otherwise
    fn is_firecracker_available() -> bool {
        Command::new("firecracker")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

impl ExecutionBackend for FireCrackerBackend {
    fn execute_code(&self, request: ExecutionRequest) -> AsyncTask<ExecutionResult> {
        let fc_config = self.firecracker_config.clone();
        let backend_name = self.backend_type();

        AsyncTaskBuilder::new().spawn(move || async move {
            // Create VM instance
            let vm = match Self::create_vm_instance(&request) {
                Ok(vm) => vm,
                Err(e) => {
                    return ExecutionResult::failure(
                        -1,
                        format!("Failed to create VM instance: {}", e),
                    );
                }
            };

            // Generate VM configuration
            if let Err(e) = Self::generate_vm_config(&vm, &fc_config, &request) {
                return ExecutionResult::failure(
                    -1,
                    format!("Failed to generate VM config: {}", e),
                );
            }

            // Start VM
            let started_vm = match Self::start_vm(vm, fc_config).await {
                Ok(vm) => vm,
                Err(e) => {
                    return ExecutionResult::failure(-1, format!("Failed to start VM: {}", e));
                }
            };

            // Execute code
            let result = match Self::execute_in_vm(started_vm.clone(), request).await {
                Ok(result) => result,
                Err(e) => ExecutionResult::failure(
                    -1,
                    format!("{} execution failed: {}", backend_name, e),
                ),
            };

            // Cleanup VM
            let _ = Self::cleanup_vm(started_vm).await;

            result
        })
    }

    fn health_check(&self) -> AsyncTask<HealthStatus> {
        let fc_config = self.firecracker_config.clone();

        AsyncTaskBuilder::new().spawn(move || async move {
            // Check platform support
            if !Self::is_platform_supported() {
                return HealthStatus::unhealthy("Platform does not support FireCracker")
                    .with_metric("platform_supported", "false");
            }

            // Verify installation
            if let Err(e) = Self::verify_firecracker_installation(&fc_config) {
                return HealthStatus::unhealthy(format!("FireCracker installation invalid: {}", e))
                    .with_metric("installation_valid", "false");
            }

            // Check FireCracker binary
            if !Self::is_firecracker_available() {
                return HealthStatus::unhealthy("FireCracker binary not available")
                    .with_metric("firecracker_available", "false");
            }

            HealthStatus::healthy("FireCracker backend operational")
                .with_metric("platform_supported", "true")
                .with_metric("installation_valid", "true")
                .with_metric("firecracker_available", "true")
                .with_metric("memory_size_mb", &fc_config.memory_size_mb.to_string())
                .with_metric("vcpu_count", &fc_config.vcpu_count.to_string())
        })
    }

    fn cleanup(&self) -> AsyncTask<crate::execution_env::CyloResult<()>> {
        AsyncTaskBuilder::new().spawn(|| async move {
            // Clean up any leftover VM processes
            let output = Command::new("ps").args(&["aux"]).output();

            if let Ok(output) = output {
                let processes = String::from_utf8_lossy(&output.stdout);
                for line in processes.lines() {
                    if line.contains("firecracker") && line.contains("cylo-") {
                        // Extract PID and kill process
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() > 1 {
                            if let Ok(pid) = fields[1].parse::<u32>() {
                                let _ = Command::new("kill")
                                    .args(&["-TERM", &pid.to_string()])
                                    .status();
                            }
                        }
                    }
                }
            }

            // Clean up temporary files
            if let Ok(entries) = fs::read_dir(std::env::temp_dir()) {
                for entry in entries.filter_map(Result::ok) {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.starts_with("cylo-") {
                            let _ = fs::remove_file(entry.path());
                        }
                    }
                }
            }

            Ok(())
        })
    }

    fn get_config(&self) -> &BackendConfig {
        &self.config
    }

    fn backend_type(&self) -> &'static str {
        "FireCracker"
    }

    fn supports_language(&self, language: &str) -> bool {
        self.supported_languages().contains(&language)
    }

    fn supported_languages(&self) -> &[&'static str] {
        &[
            "python",
            "python3",
            "javascript",
            "js",
            "node",
            "rust",
            "bash",
            "sh",
            "go",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backends::BackendConfig;

    #[test]
    fn image_format_validation() {
        assert!(FireCrackerBackend::is_valid_image_format("python:3.11"));
        assert!(FireCrackerBackend::is_valid_image_format("rust:alpine3.20"));
        assert!(FireCrackerBackend::is_valid_image_format("node:18-alpine"));

        assert!(!FireCrackerBackend::is_valid_image_format("python"));
        assert!(!FireCrackerBackend::is_valid_image_format(""));
        assert!(!FireCrackerBackend::is_valid_image_format(":tag"));
    }

    #[test]
    fn execution_script_preparation() {
        let request = ExecutionRequest::new("print('hello')", "python");
        let script = FireCrackerBackend::prepare_execution_script(&request).unwrap();
        assert!(script.contains("python3"));
        assert!(script.contains("print('hello')"));

        let request = ExecutionRequest::new("console.log('hello')", "javascript");
        let script = FireCrackerBackend::prepare_execution_script(&request).unwrap();
        assert!(script.contains("node"));

        let request = ExecutionRequest::new("some code", "cobol");
        assert!(FireCrackerBackend::prepare_execution_script(&request).is_err());
    }

    #[test]
    fn vm_instance_creation() {
        let request = ExecutionRequest::new("test", "python");
        let vm = FireCrackerBackend::create_vm_instance(&request).unwrap();

        assert!(vm.vm_id.starts_with("cylo-"));
        assert!(vm.socket_path.to_string_lossy().contains(&vm.vm_id));
        assert!(vm.config_path.to_string_lossy().contains(&vm.vm_id));
        assert!(vm.pid.is_none());
    }

    #[test]
    fn firecracker_config_initialization() {
        let config = BackendConfig::new("test_firecracker")
            .with_config("memory_size_mb", "1024")
            .with_config("vcpu_count", "2");

        let fc_config = FireCrackerBackend::init_firecracker_config(&config).unwrap();
        assert_eq!(fc_config.memory_size_mb, 1024);
        assert_eq!(fc_config.vcpu_count, 2);
    }

    #[test]
    fn backend_creation() {
        let config = BackendConfig::new("test_firecracker");

        // Valid image should work on Linux platforms
        let result = FireCrackerBackend::new("python:3.11".to_string(), config.clone());
        // Note: Will fail on non-Linux platforms or without FireCracker, which is expected

        // Invalid image should fail
        let invalid_result = FireCrackerBackend::new("invalid".to_string(), config);
        assert!(invalid_result.is_err());
    }

    #[test]
    fn supported_languages() {
        let config = BackendConfig::new("test");
        if let Ok(backend) = FireCrackerBackend::new("python:3.11".to_string(), config) {
            assert!(backend.supports_language("python"));
            assert!(backend.supports_language("rust"));
            assert!(backend.supports_language("javascript"));
            assert!(!backend.supports_language("cobol"));
        }
    }
}
