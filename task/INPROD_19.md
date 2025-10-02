# INPROD_19: FireCracker Backend Execution Implementation

## SEVERITY: MEDIUM

## OBJECTIVE
Implement actual FireCracker API integration for VM code execution instead of placeholder logic in the execute_in_vm function.

## LOCATION
- Primary: `packages/cylo/src/backends/firecracker.rs:916` (execute_in_vm function)
- Reference: `packages/cylo/src/firecracker.rs` (working SSH-based execution example)

## CURRENT STATE ANALYSIS

### Stub Implementation at Line 916
The `execute_in_vm` function currently contains placeholder logic:
```rust
// In a real implementation, we would:
// 1. Use FireCracker API to send execution commands
// 2. Monitor execution via VM console or agent
// 3. Collect resource usage statistics
//
// For this implementation, we simulate the execution

// Simulate execution time
let execution_duration = Duration::from_millis(100);
tokio::time::sleep(execution_duration).await;
```

The function prepares execution scripts via `prepare_execution_script()` but never actually executes them. It returns simulated output instead of real execution results.

### Architecture Analysis

**Two FireCracker Implementations Exist:**

1. **backends/firecracker.rs** (1273 lines) - Backend trait implementation
   - Has `FireCrackerApiClient` with HTTP3 support for VM lifecycle
   - Has `VMInstance` struct with api_client field
   - VM lifecycle working (configure, start, stop, metrics)
   - **Code execution is STUBBED** ❌

2. **firecracker.rs** (502 lines) - Standalone VM manager
   - Has working SSH-based code execution ✅
   - Methods: `execute_code()`, `copy_to_vm()`, `execute_command()`
   - Uses ssh2 library for guest communication
   - Used by state.rs for actual code execution

**Key Discovery:** The FireCracker API (HTTP over Unix socket) only controls VM **lifecycle**. It cannot directly execute code in the VM. Code execution requires one of:
- SSH/SCP (most common, used in firecracker.rs)
- virtio-vsock (socket communication)
- Serial console (debugging, limited)
- Metadata service (data sharing only)

## RESEARCH FINDINGS

### FireCracker Communication Architecture
Based on [official FireCracker documentation](../tmp/firecracker/docs/getting-started.md) and [FAQ](https://github.com/firecracker-microvm/firecracker/blob/main/FAQ.md):

1. **API Socket (Unix domain socket)**
   - RESTful control API
   - Endpoints: /machine-config, /boot-source, /drives, /actions, /metrics
   - Controls: VM configuration, start/stop, resource monitoring
   - **Cannot execute commands inside VM**

2. **Guest Communication Methods**
   - **SSH**: Primary method for command execution (requires network setup)
   - **Serial Console**: Enabled via `console=ttyS0` kernel arg (debugging)
   - **virtio-vsock**: One of 6 emulated devices, socket communication
   - **MMDS**: Metadata service for host-guest data sharing

3. **Network Requirement for SSH**
   - TAP device on host
   - Network interface configuration via API
   - Guest OS needs SSH daemon
   - SSH key or password authentication

### Existing Working Pattern (firecracker.rs)

The standalone firecracker.rs shows the complete pattern:

```rust
// SSH configuration structure
pub struct SshConfig {
    pub host: String,        // e.g., "127.0.0.1" or VM IP
    pub port: u16,           // e.g., 22
    pub username: String,    // e.g., "root"
    pub auth: SshAuth,       // Agent, Key, or Password
}

// Creating SSH session
fn create_ssh_session(&self, ssh_config: &SshConfig) -> Result<Session> {
    let tcp = TcpStream::connect(format!("{}:{}", ssh_config.host, ssh_config.port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;
    
    match &ssh_config.auth {
        SshAuth::Agent => session.userauth_agent(&ssh_config.username)?,
        SshAuth::Key(path) => session.userauth_pubkey_file(&ssh_config.username, None, path, None)?,
        SshAuth::Password(pwd) => session.userauth_password(&ssh_config.username, pwd)?,
    }
    
    Ok(session)
}

// Copying file to VM
fn copy_to_vm(&self, host_path: &str, guest_path: &str) -> Result<()> {
    let session = self.create_ssh_session(ssh_config)?;
    let metadata = fs::metadata(host_path)?;
    let mut local_file = File::open(host_path)?;
    let mut remote_file = session.scp_send(Path::new(guest_path), 0o644, metadata.len(), None)?;
    std::io::copy(&mut local_file, &mut remote_file)?;
    remote_file.send_eof()?;
    remote_file.wait_close()?;
    Ok(())
}

// Executing command in VM
fn execute_command(&self, command: &str) -> Result<String> {
    let session = self.create_ssh_session(ssh_config)?;
    let mut channel = session.channel_session()?;
    channel.exec(command)?;
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;
    let exit_status = channel.exit_status()?;
    if exit_status != 0 {
        return Err(anyhow!("Command failed with exit code {}: {}", exit_status, output));
    }
    Ok(output)
}
```

See: [../packages/cylo/src/firecracker.rs:313-428](../packages/cylo/src/firecracker.rs)

### Dependencies Available
From [Cargo.toml](../packages/cylo/Cargo.toml:35):
```toml
ssh2 = "0.9"
```

The ssh2 library is already available for SSH client implementation.

## IMPLEMENTATION REQUIREMENTS

### SUBTASK 1: Extend VMInstance Structure

**Location:** `packages/cylo/src/backends/firecracker.rs:475`

Add SSH configuration to VMInstance:

```rust
struct VMInstance {
    vm_id: String,
    socket_path: PathBuf,
    config_path: PathBuf,
    pid: Option<u32>,
    api_client: Option<FireCrackerApiClient>,
    created_at: SystemTime,
    // ADD:
    ssh_config: Option<SshConfig>,  // SSH configuration for guest access
}

// ADD: SSH configuration structure (can reuse from firecracker.rs or define here)
#[derive(Debug, Clone)]
struct SshConfig {
    host: String,
    port: u16,
    username: String,
    auth: SshAuth,
}

#[derive(Debug, Clone)]
enum SshAuth {
    Agent,
    Key(PathBuf),
    Password(String),
}
```

### SUBTASK 2: Configure Network in start_vm

**Location:** `packages/cylo/src/backends/firecracker.rs:857`

After configuring rootfs and before starting VM, add network interface configuration:

```rust
// Configure network interface (if network enabled)
if fc_config.network_enabled {
    let network_config = serde_json::json!({
        "iface_id": "eth0",
        "host_dev_name": "tap0",  // TAP device name (must exist on host)
        "guest_mac": "AA:FC:00:00:00:01",
        "allow_mmds_requests": true
    });

    let network_request = HttpRequest::put(
        &format!("http://unix:{}/network-interfaces/eth0", vm_with_pid.socket_path.display()),
        serde_json::to_vec(&network_config).map_err(|e| {
            BackendError::ConfigurationFailed {
                details: format!("Failed to serialize network config: {}", e),
            }
        })?,
    )
    .map_err(|e| BackendError::ConfigurationFailed {
        details: format!("Failed to create network request: {}", e),
    })?
    .header("Content-Type", "application/json");

    api_client.http_client.send(network_request).await.map_err(|e| {
        BackendError::ConfigurationFailed {
            details: format!("Network configuration failed: {}", e),
        }
    })?;
}
```

**Note:** Network configuration from backend_specific config in FireCrackerConfig, or from environment.

### SUBTASK 3: Wait for SSH Availability

After VM starts and reaches "Running" state, wait for SSH to be available:

```rust
// Wait for SSH to be ready (after VM is Running)
if let Some(ssh_cfg) = &vm_with_pid.ssh_config {
    for attempt in 0..30 {
        if let Ok(tcp) = TcpStream::connect_timeout(
            &format!("{}:{}", ssh_cfg.host, ssh_cfg.port).parse().unwrap(),
            Duration::from_secs(1)
        ) {
            drop(tcp);
            break;
        }
        if attempt == 29 {
            return Err(BackendError::StartupFailed {
                details: "SSH not available within timeout".to_string(),
            });
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}
```

### SUBTASK 4: Implement execute_in_vm with SSH

**Location:** `packages/cylo/src/backends/firecracker.rs:916`

Replace stub implementation with actual SSH-based execution:

```rust
fn execute_in_vm(
    vm: VMInstance,
    request: ExecutionRequest,
) -> AsyncTask<BackendResult<ExecutionResult>> {
    AsyncTaskBuilder::new().spawn(move || async move {
        let start_time = Instant::now();

        // Prepare execution script
        let exec_script = Self::prepare_execution_script(&request)?;
        
        // Get SSH config or return error
        let ssh_config = vm.ssh_config.ok_or_else(|| BackendError::ConfigurationFailed {
            details: "SSH configuration not available for VM".to_string(),
        })?;

        // Create temporary script file on host
        let script_path = format!("/tmp/exec-{}.sh", vm.vm_id);
        let guest_script_path = format!("/tmp/exec-{}.sh", vm.vm_id);
        
        fs::write(&script_path, &exec_script).map_err(|e| BackendError::FileSystemFailed {
            details: format!("Failed to write script: {}", e),
        })?;

        // Copy script to VM via SCP
        let copy_result = tokio::task::spawn_blocking({
            let ssh_cfg = ssh_config.clone();
            let script = script_path.clone();
            let guest_script = guest_script_path.clone();
            move || -> BackendResult<()> {
                let session = create_ssh_session(&ssh_cfg)?;
                let metadata = fs::metadata(&script).map_err(|e| BackendError::FileSystemFailed {
                    details: format!("Failed to read script metadata: {}", e),
                })?;
                
                let mut local_file = File::open(&script).map_err(|e| BackendError::FileSystemFailed {
                    details: format!("Failed to open script: {}", e),
                })?;
                
                let mut remote_file = session.scp_send(
                    Path::new(&guest_script),
                    0o755,
                    metadata.len(),
                    None,
                ).map_err(|e| BackendError::ProcessFailed {
                    details: format!("SCP failed: {}", e),
                })?;
                
                std::io::copy(&mut local_file, &mut remote_file).map_err(|e| {
                    BackendError::ProcessFailed {
                        details: format!("File copy failed: {}", e),
                    }
                })?;
                
                remote_file.send_eof().map_err(|e| BackendError::ProcessFailed {
                    details: format!("EOF failed: {}", e),
                })?;
                remote_file.wait_close().map_err(|e| BackendError::ProcessFailed {
                    details: format!("Wait close failed: {}", e),
                })?;
                
                Ok(())
            }
        }).await.map_err(|e| BackendError::ProcessFailed {
            details: format!("Task join failed: {}", e),
        })??;

        // Execute script in VM via SSH
        let (exit_code, stdout, stderr) = tokio::task::spawn_blocking({
            let ssh_cfg = ssh_config.clone();
            let guest_script = guest_script_path.clone();
            move || -> BackendResult<(i32, String, String)> {
                let session = create_ssh_session(&ssh_cfg)?;
                let mut channel = session.channel_session().map_err(|e| {
                    BackendError::ProcessFailed {
                        details: format!("Failed to create channel: {}", e),
                    }
                })?;
                
                channel.exec(&format!("bash {}", guest_script)).map_err(|e| {
                    BackendError::ProcessFailed {
                        details: format!("Exec failed: {}", e),
                    }
                })?;
                
                let mut stdout = String::new();
                channel.read_to_string(&mut stdout).map_err(|e| {
                    BackendError::ProcessFailed {
                        details: format!("Read stdout failed: {}", e),
                    }
                })?;
                
                let mut stderr = String::new();
                channel.stderr().read_to_string(&mut stderr).map_err(|e| {
                    BackendError::ProcessFailed {
                        details: format!("Read stderr failed: {}", e),
                    }
                })?;
                
                channel.wait_close().map_err(|e| BackendError::ProcessFailed {
                    details: format!("Wait close failed: {}", e),
                })?;
                
                let exit_code = channel.exit_status().map_err(|e| {
                    BackendError::ProcessFailed {
                        details: format!("Get exit status failed: {}", e),
                    }
                })?;
                
                Ok((exit_code, stdout, stderr))
            }
        }).await.map_err(|e| BackendError::ProcessFailed {
            details: format!("Task join failed: {}", e),
        })??;

        // Cleanup temp script
        let _ = fs::remove_file(&script_path);

        // Collect resource metrics from VM
        let resource_usage = if let Some(api_client) = &vm.api_client {
            match api_client.get_vm_metrics().await {
                Ok(metrics) => ResourceUsage {
                    peak_memory: metrics.get("memory_usage_bytes")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0),
                    cpu_time_ms: metrics.get("cpu_usage_us")
                        .and_then(|v| v.as_u64())
                        .map(|us| us / 1000)
                        .unwrap_or(0),
                    process_count: 1,
                    disk_bytes_written: metrics.get("disk_write_bytes")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0),
                    disk_bytes_read: metrics.get("disk_read_bytes")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0),
                    network_bytes_sent: 0,
                    network_bytes_received: 0,
                },
                Err(_) => ResourceUsage::default(),
            }
        } else {
            ResourceUsage::default()
        };

        let duration = start_time.elapsed();

        Ok(ExecutionResult {
            exit_code,
            stdout,
            stderr,
            duration,
            resource_usage,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("backend".to_string(), "FireCracker".to_string());
                meta.insert("vm_id".to_string(), vm.vm_id.clone());
                meta.insert("execution_method".to_string(), "SSH".to_string());
                meta
            },
        })
    })
}

// ADD: Helper function for SSH session creation
fn create_ssh_session(ssh_config: &SshConfig) -> BackendResult<ssh2::Session> {
    use std::net::TcpStream;
    
    let tcp = TcpStream::connect(format!("{}:{}", ssh_config.host, ssh_config.port))
        .map_err(|e| BackendError::ProcessFailed {
            details: format!("TCP connection failed: {}", e),
        })?;
    
    let mut session = ssh2::Session::new().map_err(|e| BackendError::ProcessFailed {
        details: format!("SSH session creation failed: {}", e),
    })?;
    
    session.set_tcp_stream(tcp);
    session.handshake().map_err(|e| BackendError::ProcessFailed {
        details: format!("SSH handshake failed: {}", e),
    })?;
    
    match &ssh_config.auth {
        SshAuth::Agent => {
            session.userauth_agent(&ssh_config.username).map_err(|e| {
                BackendError::ProcessFailed {
                    details: format!("SSH agent auth failed: {}", e),
                }
            })?;
        }
        SshAuth::Key(key_path) => {
            session.userauth_pubkey_file(&ssh_config.username, None, key_path, None)
                .map_err(|e| BackendError::ProcessFailed {
                    details: format!("SSH key auth failed: {}", e),
                })?;
        }
        SshAuth::Password(password) => {
            session.userauth_password(&ssh_config.username, password)
                .map_err(|e| BackendError::ProcessFailed {
                    details: format!("SSH password auth failed: {}", e),
                })?;
        }
    }
    
    if !session.authenticated() {
        return Err(BackendError::ProcessFailed {
            details: "SSH authentication failed".to_string(),
        });
    }
    
    Ok(session)
}
```

### SUBTASK 5: Configuration Integration

**Location:** `packages/cylo/src/backends/firecracker.rs:587`

Update `init_firecracker_config` to support SSH configuration from backend_specific:

```rust
fn init_firecracker_config(config: &BackendConfig) -> BackendResult<FireCrackerConfig> {
    let mut fc_config = FireCrackerConfig::default();

    // Existing overrides...
    
    // ADD: Network and SSH configuration
    if let Some(network_enabled) = config.backend_specific.get("network_enabled") {
        fc_config.network_enabled = network_enabled.parse().unwrap_or(false);
    }
    
    // SSH configuration can be provided via backend_specific:
    // - ssh_host: "172.16.0.2"
    // - ssh_port: "22"
    // - ssh_username: "root"
    // - ssh_key_path: "/path/to/key" OR ssh_password: "password"
    
    Ok(fc_config)
}
```

And update `create_vm_instance` to populate SSH config:

```rust
fn create_vm_instance(
    request: &ExecutionRequest,
    backend_config: &BackendConfig,
) -> BackendResult<VMInstance> {
    let vm_id = format!("cylo-{}-{}", uuid::Uuid::new_v4().simple(), std::process::id());
    let socket_path = std::env::temp_dir().join(format!("{}.sock", vm_id));
    let config_path = std::env::temp_dir().join(format!("{}.json", vm_id));

    // Build SSH config from backend_specific
    let ssh_config = if backend_config.backend_specific.contains_key("ssh_host") {
        let host = backend_config.backend_specific.get("ssh_host")
            .cloned().unwrap_or_else(|| "172.16.0.2".to_string());
        let port = backend_config.backend_specific.get("ssh_port")
            .and_then(|p| p.parse().ok()).unwrap_or(22);
        let username = backend_config.backend_specific.get("ssh_username")
            .cloned().unwrap_or_else(|| "root".to_string());
        
        let auth = if let Some(key_path) = backend_config.backend_specific.get("ssh_key_path") {
            SshAuth::Key(PathBuf::from(key_path))
        } else if let Some(password) = backend_config.backend_specific.get("ssh_password") {
            SshAuth::Password(password.clone())
        } else {
            SshAuth::Agent
        };
        
        Some(SshConfig { host, port, username, auth })
    } else {
        None
    };

    Ok(VMInstance {
        vm_id,
        socket_path,
        config_path,
        pid: None,
        api_client: None,
        created_at: SystemTime::now(),
        ssh_config,
    })
}
```

## DEFINITION OF DONE

- [ ] VMInstance struct extended with ssh_config field
- [ ] SshConfig and SshAuth structures defined (or imported from firecracker.rs)
- [ ] Network configuration added to start_vm when network_enabled is true
- [ ] SSH availability check implemented after VM starts
- [ ] execute_in_vm function implements real SSH-based code execution:
  - [ ] Script preparation and file writing
  - [ ] SCP file transfer to VM
  - [ ] SSH command execution
  - [ ] stdout/stderr capture
  - [ ] Exit code capture
  - [ ] Resource metrics collection from VM
- [ ] create_ssh_session helper function implemented
- [ ] Configuration integration in init_firecracker_config and create_vm_instance
- [ ] Stub comment at line 916 removed
- [ ] Error handling for SSH failures implemented
- [ ] Cleanup of temporary files

## CONSTRAINTS

- NO test code to be written (separate team responsibility)
- NO benchmark code to be written (separate team responsibility)
- Focus solely on ./src implementation
- NO extensive documentation files

## NOTES

### Alternative Approaches Considered

1. **Serial Console Execution**: Simpler but unreliable for capturing output
2. **virtio-vsock**: Lower overhead but requires guest-side agent
3. **Metadata Service**: Data passing only, not command execution

**Chosen Approach: SSH** - Industry standard, reliable, well-supported by ssh2 library, pattern already proven in firecracker.rs.

### Prerequisites for Runtime

For this implementation to work in production:
1. Host must have TAP device configured (e.g., `ip tuntap add tap0 mode tap`)
2. Guest rootfs must have SSH server installed and configured
3. Guest must have network configuration (static IP or DHCP)
4. SSH credentials must be known (key file or password)

These are typically handled during rootfs preparation (outside this task scope).
