# TURD: Medium Priority Violations

These violations represent incomplete features or hardcoded values that should be replaced with proper implementations.

---

## VIOLATION #10: Hardcoded DNS Resolution Time

**File:** `packages/cylo/src/platform.rs`  
**Line:** 503  
**Severity:** MEDIUM - Inaccurate Performance Metrics

### Current Code
```rust
dns_resolution_ms: 50,   // Placeholder
```

### Description
DNS resolution time is hardcoded to 50ms instead of being measured.

### Impact
- Inaccurate network capability reporting
- Performance profiling data is fake
- Cannot detect slow DNS resolution

### Solution: Measure Actual DNS Resolution Time

```rust
use std::time::Instant;
use tokio::net::lookup_host;

async fn measure_dns_resolution() -> u64 {
    let start = Instant::now();
    
    // Test DNS resolution with a reliable domain
    let test_domains = [
        "google.com:80",
        "cloudflare.com:80",
        "1.1.1.1:80",
    ];
    
    let mut total_time = 0u64;
    let mut successful_lookups = 0;
    
    for domain in test_domains {
        if let Ok(_) = lookup_host(domain).await {
            successful_lookups += 1;
        }
    }
    
    if successful_lookups > 0 {
        total_time = start.elapsed().as_millis() as u64;
        total_time / successful_lookups
    } else {
        // Fallback if all DNS lookups fail
        100 // Conservative estimate
    }
}

// In get_network_capabilities():
dns_resolution_ms: measure_dns_resolution().await,
```

**Note:** This requires making `get_network_capabilities()` async or using `tokio::runtime::Handle::current().block_on()` if it must remain sync.

---

## VIOLATION #11: Placeholder Screenshot Data in Hyper Fetcher

**File:** `packages/sweetmcp/plugins/fetch/src/hyper.rs`  
**Lines:** 266-268  
**Severity:** MEDIUM - Fake Screenshot Data

### Current Code
```rust
// Generate a placeholder screenshot since hyper doesn't support screenshots
let screenshot_base64 =
    base64::engine::general_purpose::STANDARD.encode(b"placeholder-screenshot-data");
```

### Description
The Hyper-based fetcher returns fake screenshot data instead of actual screenshots.

### Impact
- Screenshot feature doesn't work with Hyper backend
- Users receive invalid screenshot data
- Feature appears to work but returns garbage

### Solution Options

**Option 1: Return None for Screenshots**
```rust
// Hyper doesn't support screenshots - return None
let screenshot_base64 = None;

Ok(FetchResult {
    content: cleaned_content,
    screenshot: screenshot_base64,
    url: url.to_string(),
})
```

Change `FetchResult.screenshot` to `Option<String>` and document that Hyper backend doesn't support screenshots.

**Option 2: Use wkhtmltoimage for Screenshot Generation**
```rust
use std::process::Command;
use tempfile::NamedTempFile;

async fn generate_screenshot_wkhtmltoimage(url: &str) -> Result<Option<String>, Error> {
    // Check if wkhtmltoimage is available
    if !Command::new("wkhtmltoimage").arg("--version").output().is_ok() {
        return Ok(None);
    }
    
    let temp_file = NamedTempFile::new()?;
    let output_path = temp_file.path();
    
    // Generate screenshot
    let status = Command::new("wkhtmltoimage")
        .args(&[
            "--width", "1920",
            "--height", "1080",
            "--quality", "90",
            url,
            output_path.to_str().unwrap(),
        ])
        .status()?;
    
    if !status.success() {
        return Ok(None);
    }
    
    // Read and encode screenshot
    let screenshot_bytes = tokio::fs::read(output_path).await?;
    let screenshot_base64 = base64::engine::general_purpose::STANDARD.encode(&screenshot_bytes);
    
    Ok(Some(screenshot_base64))
}

// In fetch code:
let screenshot_base64 = generate_screenshot_wkhtmltoimage(url).await?;
```

**Option 3: Delegate to Chromium/Bevy Renderer**
```rust
// Hyper doesn't support screenshots - delegate to chromium backend
let screenshot_base64 = if cfg!(not(target_family = "wasm")) {
    crate::chromiumoxide::fetch_screenshot(url).await.ok()
} else {
    None
};
```

**Recommended:** Option 1 (return None) for honesty, or Option 3 (delegate) for functionality.

---

## VIOLATION #12: RPM Package Creation Not Implemented

**File:** `packages/sweetmcp/packages/sixel6vt/build-installers.rs`  
**Line:** 580-581  
**Severity:** LOW-MEDIUM - Installation Method Missing

### Current Code
```rust
// For brevity, returning a placeholder error
Err(anyhow::anyhow!("RPM package creation not yet implemented"))
```

### Description
RPM package creation is stubbed out, preventing installation on RPM-based Linux distros (Fedora, RHEL, CentOS, OpenSUSE).

### Impact
- Cannot install on RPM-based systems
- Limited distribution coverage
- Manual installation required

### Solution: Implement RPM Package Creation

```rust
use std::process::Command;
use std::fs;

fn create_rpm_package(
    target: &Target,
    binary_path: &Path,
    ctx: &BuildContext,
) -> anyhow::Result<PathBuf> {
    let rpm_dir = ctx.output_dir.join("rpm");
    fs::create_dir_all(&rpm_dir)?;
    
    // Create RPM build directory structure
    let build_root = rpm_dir.join("BUILD");
    let rpms_dir = rpm_dir.join("RPMS");
    let specs_dir = rpm_dir.join("SPECS");
    let sources_dir = rpm_dir.join("SOURCES");
    
    for dir in [&build_root, &rpms_dir, &specs_dir, &sources_dir] {
        fs::create_dir_all(dir)?;
    }
    
    // Create RPM spec file
    let spec_content = format!(r#"
Name:           {name}
Version:        {version}
Release:        1
Summary:        {summary}
License:        {license}
URL:            {url}

%description
{description}

%install
mkdir -p %{{buildroot}}/usr/bin
install -m 755 {binary_path} %{{buildroot}}/usr/bin/{name}

%files
/usr/bin/{name}

%changelog
* {date} {maintainer}
- Initial package
"#,
        name = ctx.package_name,
        version = ctx.version,
        summary = ctx.description.lines().next().unwrap_or("Sixel terminal emulator"),
        license = "MIT", // Adjust as needed
        url = "https://github.com/your-org/sixel6vt",
        description = ctx.description,
        binary_path = binary_path.display(),
        date = chrono::Local::now().format("%a %b %d %Y"),
        maintainer = "Package Maintainer <maintainer@example.com>",
    );
    
    let spec_file = specs_dir.join(format!("{}.spec", ctx.package_name));
    fs::write(&spec_file, spec_content)?;
    
    // Build RPM using rpmbuild
    let output = Command::new("rpmbuild")
        .args(&[
            "-bb",
            "--define", &format!("_topdir {}", rpm_dir.display()),
            spec_file.to_str().unwrap(),
        ])
        .output()?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "rpmbuild failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    // Find generated RPM
    let arch = match target.architecture {
        Architecture::X64 => "x86_64",
        Architecture::Arm64 => "aarch64",
        _ => "noarch",
    };
    
    let rpm_file = rpms_dir
        .join(arch)
        .join(format!("{}-{}-1.{}.rpm", ctx.package_name, ctx.version, arch));
    
    if !rpm_file.exists() {
        return Err(anyhow::anyhow!("RPM file not found at {:?}", rpm_file));
    }
    
    Ok(rpm_file)
}
```

**Dependencies Required:**
- `rpmbuild` command-line tool must be available
- Consider using Docker for cross-platform RPM builds
- Add error handling for missing rpmbuild

---

## VIOLATION #13: Firecracker Using curl Instead of API Client

**File:** `packages/cylo/src/firecracker.rs`  
**Line:** 160  
**Severity:** MEDIUM - Fragile Integration

### Current Code
```rust
// This would normally use the Firecracker API client
// For now, we'll use curl commands for simplicity
```

### Description
The code uses curl commands to interact with Firecracker API instead of using a proper HTTP client or the official Firecracker SDK.

### Impact
- Fragile command-line parsing
- No type safety for API requests/responses
- Error handling is difficult
- Cannot validate API schemas
- Brittle against API changes

### Solution: Use Proper HTTP Client with Typed API

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct BootSource {
    kernel_image_path: String,
    boot_args: String,
    initrd_path: Option<String>,
}

#[derive(Serialize)]
struct MachineConfig {
    vcpu_count: u32,
    mem_size_mib: u32,
    ht_enabled: bool,
}

#[derive(Serialize)]
struct DriveConfig {
    drive_id: String,
    path_on_host: String,
    is_root_device: bool,
    is_read_only: bool,
}

impl FirecrackerConfig {
    async fn configure_vm(&self) -> Result<()> {
        let client = Client::new();
        let api_base = format!("http://localhost:{}", self.api_port);
        
        // Configure boot source
        let boot_source = BootSource {
            kernel_image_path: self.kernel_path.to_string_lossy().to_string(),
            boot_args: self.kernel_args.clone(),
            initrd_path: self.initrd_path.as_ref().map(|p| p.to_string_lossy().to_string()),
        };
        
        let response = client
            .put(&format!("{}/boot-source", api_base))
            .json(&boot_source)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to configure boot source: {}",
                response.text().await?
            ));
        }
        
        // Configure machine
        let machine_config = MachineConfig {
            vcpu_count: self.vcpu_count,
            mem_size_mib: self.memory_size_mb,
            ht_enabled: false,
        };
        
        let response = client
            .put(&format!("{}/machine-config", api_base))
            .json(&machine_config)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to configure machine: {}",
                response.text().await?
            ));
        }
        
        // Configure root drive
        if let Some(rootfs_path) = &self.rootfs_path {
            let drive_config = DriveConfig {
                drive_id: "rootfs".to_string(),
                path_on_host: rootfs_path.to_string_lossy().to_string(),
                is_root_device: true,
                is_read_only: false,
            };
            
            let response = client
                .put(&format!("{}/drives/rootfs", api_base))
                .json(&drive_config)
                .send()
                .await?;
            
            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "Failed to configure drive: {}",
                    response.text().await?
                ));
            }
        }
        
        // Start VM
        let response = client
            .put(&format!("{}/actions", api_base))
            .json(&serde_json::json!({"action_type": "InstanceStart"}))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to start VM: {}",
                response.text().await?
            ));
        }
        
        Ok(())
    }
}
```

**Better Alternative:** Use the official Firecracker SDK if available, or create a dedicated API client module with all Firecracker API types defined.

---

## VIOLATION #14: Hardcoded Embedding Dimensions

**File:** `packages/candle/src/memory/monitoring/health.rs`  
**Line:** 378-379  
**Severity:** LOW-MEDIUM - Configuration Inflexibility

### Current Code
```rust
// For now, use sensible defaults or get from config
let dimensions = 768u32; // Could be passed to constructor
```

### Description
Embedding dimensions are hardcoded to 768 instead of reading from configuration or vector store metadata.

### Impact
- Incorrect dimensions if using different embedding model
- Health monitoring reports wrong dimension count
- Inflexible for different models (384, 1024, 1536 dimensions)

### Solution: Get Dimensions from VectorStore or Config

```rust
// Option 1: Get from VectorStore metadata
let dimensions = {
    let vs = vs_idx.blocking_read();
    vs.dimensions().unwrap_or(768)  // If VectorStore has this method
} as u32;

// Option 2: Get from embedding config
let dimensions = self.embedding_config
    .as_ref()
    .map(|cfg| cfg.dimensions as u32)
    .unwrap_or(768);

// Option 3: Store in HealthMonitor during construction
pub struct HealthMonitor {
    // ... existing fields ...
    embedding_dimensions: u32,
}

impl HealthMonitor {
    pub fn new(
        memory_manager: Arc<SurrealDBMemoryManager>,
        vector_store: Arc<RwLock<Box<dyn VectorStore>>>,
        embedding_dimensions: u32,  // Pass as parameter
    ) -> Self {
        Self {
            memory_manager,
            vector_store,
            embedding_dimensions,
            // ... other fields ...
        }
    }
}

// Then in check_vector_store_health:
let dimensions = self.embedding_dimensions;
```

**Recommended:** Option 3 (pass as constructor parameter) for explicit configuration.

---

## VIOLATION #15: Sixel Browser Title Not Fetched

**File:** `packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/browser/mod.rs`  
**Lines:** 100-102  
**Severity:** LOW - Missing Feature

### Current Code
```rust
// For now, we don't fetch the title from the browser
// This could be enhanced later
inner.title = "Web Page".to_string();
```

### Description
Browser component doesn't fetch the actual page title.

### Impact
- All pages show generic "Web Page" title
- Poor user experience
- Cannot identify tabs by page title

### Solution: Fetch Title from Chromium

```rust
pub async fn navigate_to(&mut self, url: &str) -> Result<String> {
    // ... existing navigation code ...
    
    // Fetch page title after navigation
    if let Some(browser) = &self.browser {
        match browser.get_title().await {
            Ok(title) => {
                let mut inner = self.inner.write().await;
                inner.current_url = url.to_string();
                inner.title = title;
                return Ok(title);
            }
            Err(e) => {
                warn!("Failed to fetch page title: {}", e);
                // Fallback to URL as title
                let mut inner = self.inner.write().await;
                inner.current_url = url.to_string();
                inner.title = url.to_string();
                return Ok(url.to_string());
            }
        }
    }
    
    Ok(url.to_string())
}
```

---

## VIOLATION #16: Hardcoded Terminal Text Color

**File:** `packages/sweetmcp/packages/sixel6vt/packages/sixel6vt/src/components/terminal/mod.rs`  
**Line:** 493  
**Severity:** LOW - Poor Theming

### Current Code
```rust
style.color = [1.0, 1.0, 1.0, 1.0]; // White text for now
```

### Description
Terminal text color is hardcoded to white instead of using theme colors or terminal attributes.

### Impact
- Ignores color themes
- All text appears white regardless of styling
- Cannot support dark/light themes properly

### Solution: Use Terminal Cell Colors

```rust
// Get color from terminal cell attributes
let fg_color = match square.fg {
    rio_backend::crosswords::Color::Named(named) => {
        // Map named colors to RGBA
        self.map_named_color_to_rgba(named)
    }
    rio_backend::crosswords::Color::Spec(rgb) => {
        // Direct RGB color
        [
            rgb.r as f32 / 255.0,
            rgb.g as f32 / 255.0,
            rgb.b as f32 / 255.0,
            1.0,
        ]
    }
    rio_backend::crosswords::Color::Indexed(idx) => {
        // Map color palette index to RGBA
        self.map_indexed_color_to_rgba(idx)
    }
};

style.color = fg_color;
```

Add color mapping methods:
```rust
impl TerminalComponent {
    fn map_named_color_to_rgba(&self, color: rio_backend::crosswords::NamedColor) -> [f32; 4] {
        // Map named colors from theme or defaults
        match color {
            NamedColor::Black => [0.0, 0.0, 0.0, 1.0],
            NamedColor::Red => [1.0, 0.0, 0.0, 1.0],
            NamedColor::Green => [0.0, 1.0, 0.0, 1.0],
            // ... other colors ...
            NamedColor::Foreground => [1.0, 1.0, 1.0, 1.0],
            _ => [1.0, 1.0, 1.0, 1.0],
        }
    }
    
    fn map_indexed_color_to_rgba(&self, idx: u8) -> [f32; 4] {
        // Use color palette from configuration
        self.config
            .colors
            .get(idx as usize)
            .map(|c| [c.r, c.g, c.b, 1.0])
            .unwrap_or([1.0, 1.0, 1.0, 1.0])
    }
}
```

---

## Summary

Medium priority violations should be addressed after critical and high priority items. Total estimated effort: 3-4 days.

Priority order:
1. DNS measurement (#10) - Metrics accuracy
2. Firecracker API client (#13) - Code quality/maintainability
3. Screenshot data (#11) - Feature honesty
4. Embedding dimensions (#14) - Configuration correctness
5. RPM packaging (#12) - Distribution coverage
6. Browser title (#15) - User experience
7. Terminal colors (#16) - Theming support
