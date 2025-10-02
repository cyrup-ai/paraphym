// xtask - Build automation for sixel6vt
// Run with: cargo xtask <command>

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("setup") => setup(),
        Some("clean") => clean(),
        Some("help") | None => {
            print_help();
            Ok(())
        }
        Some(task) => {
            eprintln!("Unknown task: {}", task);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!(r#"
sixel6vt build automation

USAGE:
    cargo xtask <TASK>

TASKS:
    setup       Clone Rio from GitHub and prepare it as a library
    clean       Remove vendor directory and cached artifacts
    help        Show this help message

EXAMPLES:
    cargo xtask setup      # First-time setup
    cargo xtask clean      # Clean and start fresh
"#);
}

fn setup() -> Result<()> {
    println!("\n╔═══════════════════════════════════════╗");
    println!("║   Rio Library Setup - Full Automation   ║");
    println!("╚═══════════════════════════════════════╝\n");

    let project_root = project_root()?;
    let rio_dir = project_root.join("vendor/rio");
    let rioterm_path = rio_dir.join("frontends/rioterm");

    // Step 1: Clone or update Rio
    if rio_dir.exists() {
        println!("📦 Rio already cloned, updating...");
        update_rio(&rio_dir)?;
    } else {
        println!("📦 Cloning Rio from GitHub...");
        clone_rio(&project_root)?;
    }

    // Step 2: Convert to library
    println!("\n🔧 Converting rioterm to library...");
    inject_lib_rs(&rioterm_path)?;
    add_lib_target(&rioterm_path)?;
    
    // Step 2.5: Patch Application to expose router
    println!("\n🔧 Patching Application for sixel injection...");
    patch_application(&rioterm_path)?;

    // Step 3: Verify
    println!("\n✅ Verifying library builds...");
    verify_build(&rioterm_path)?;

    println!("\n╔═══════════════════════════════════════╗");
    println!("║           Setup Complete! ✅          ║");
    println!("╚═══════════════════════════════════════╝");
    println!("\nYou can now build sixel6vt:");
    println!("  cargo build\n");

    Ok(())
}

fn clean() -> Result<()> {
    println!("🧹 Cleaning vendor directory...");
    let project_root = project_root()?;
    let vendor = project_root.join("vendor");
    
    if vendor.exists() {
        fs::remove_dir_all(&vendor)?;
        println!("✓ Removed {}", vendor.display());
    } else {
        println!("✓ No vendor directory to clean");
    }
    
    println!("\n✅ Clean complete. Run 'cargo xtask setup' to re-clone Rio.");
    Ok(())
}

fn project_root() -> Result<std::path::PathBuf> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    Ok(Path::new(&manifest_dir).parent().unwrap().to_path_buf())
}

fn clone_rio(project_root: &Path) -> Result<()> {
    let vendor = project_root.join("vendor");
    fs::create_dir_all(&vendor)?;

    run_cmd(
        &vendor,
        "git",
        &[
            "clone",
            "--depth", "1",
            "--branch", "main",
            "https://github.com/raphamorim/rio.git",
            "rio",
        ],
    )?;

    println!("✓ Cloned Rio repository");
    Ok(())
}

fn update_rio(rio_dir: &Path) -> Result<()> {
    run_cmd(rio_dir, "git", &["fetch", "origin", "main"])?;
    run_cmd(rio_dir, "git", &["reset", "--hard", "origin/main"])?;
    run_cmd(rio_dir, "git", &["clean", "-fd"])?;
    println!("✓ Updated Rio repository");
    Ok(())
}

fn inject_lib_rs(rioterm_path: &Path) -> Result<()> {
    let main_rs = rioterm_path.join("src/main.rs");
    let lib_rs = rioterm_path.join("src/lib.rs");

    let content = fs::read_to_string(&main_rs)?;
    let mut modules = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("mod ") && line.ends_with(';') {
            if let Some(name) = line.strip_prefix("mod ").and_then(|s| s.strip_suffix(';')) {
                modules.push(name.trim().to_string());
            }
        }
    }

    let mut lib_content = String::from(
        r#"//! Rioterm library interface
#![allow(dead_code)]
#![allow(unused_imports)]

// Re-export rio_backend modules that internal modules depend on
// These match the imports in main.rs that make them available at crate root
pub use rio_backend::{ansi, crosswords, event, performer, selection};

"#,
    );

    for module in &modules {
        // Platform-specific modules need cfg gates
        if module == "panic" {
            lib_content.push_str("#[cfg(windows)]\n");
        }
        lib_content.push_str(&format!("pub mod {};\n", module));
    }

    lib_content.push_str("\n// Re-export commonly used types\n");
    lib_content.push_str("pub use application::Application;\n");
    lib_content.push_str("pub use router::Router;\n");
    lib_content.push_str("pub use screen::Screen;\n");
    lib_content.push_str("pub use renderer::Renderer;\n");
    
    // Add sixel injection functions
    lib_content.push_str(r#"

use rio_backend::performer::handler::{Processor, StdSyncHandler};

/// Inject sixel data directly into the terminal for rendering
/// This processes the data through the terminal's parser as if it came from the PTY
pub fn inject_sixel(app: &mut Application, sixel_data: &str) {
    for route in app.router.routes.values_mut() {
        // Get the terminal from the current context
        let mut terminal = route.window.screen.context_manager.current_mut().terminal.lock();
        
        // Create a processor (use StdSyncHandler for type parameter)
        let mut processor: Processor<StdSyncHandler> = Processor::new();
        
        // Process the sixel data through the terminal parser
        processor.advance(&mut *terminal, sixel_data.as_bytes());
        
        // Request a redraw to show the graphics
        route.window.winit_window.request_redraw();
    }
}
"#);

    fs::write(&lib_rs, lib_content)?;
    println!("✓ Created lib.rs with {} modules", modules.len());
    Ok(())
}

fn add_lib_target(rioterm_path: &Path) -> Result<()> {
    let cargo_toml = rioterm_path.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml)?;

    if content.contains("[lib]") {
        println!("✓ [lib] target already exists");
        return Ok(());
    }

    let bin_pos = content
        .find("[[bin]]")
        .ok_or("Could not find [[bin]] in Cargo.toml")?;

    let lib_section = "\n[lib]\nname = \"rioterm\"\npath = \"src/lib.rs\"\n\n";
    let new_content = format!("{}{}{}", &content[..bin_pos], lib_section, &content[bin_pos..]);

    fs::write(&cargo_toml, new_content)?;
    println!("✓ Added [lib] target to Cargo.toml");
    Ok(())
}

fn patch_application(rioterm_path: &Path) -> Result<()> {
    // Patch Application to make fields public
    let app_rs = rioterm_path.join("src/application.rs");
    let content = fs::read_to_string(&app_rs)?;
    
    let patched = content
        .replace(
            "pub struct Application<'a> {\n    config: rio_backend::config::Config,\n    event_proxy: EventProxy,\n    router: Router<'a>,",
            "pub struct Application<'a> {\n    config: rio_backend::config::Config,\n    pub event_proxy: EventProxy,\n    pub router: Router<'a>,"
        );
    
    fs::write(&app_rs, patched)?;
    println!("✓ Patched Application struct to expose router and event_proxy");
    
    // Patch Router to make routes public
    let router_rs = rioterm_path.join("src/router/mod.rs");
    let content = fs::read_to_string(&router_rs)?;
    
    let patched = content
        .replace(
            "pub struct Router<'a> {\n    pub routes: FxHashMap<WindowId, Route<'a>>,",
            "pub struct Router<'a> {\n    pub routes: FxHashMap<WindowId, Route<'a>>,"
        )
        .replace(
            "pub struct Router<'a> {\n    routes: FxHashMap<WindowId, Route<'a>>,",
            "pub struct Router<'a> {\n    pub routes: FxHashMap<WindowId, Route<'a>>,"
        );
    
    fs::write(&router_rs, patched)?;
    println!("✓ Patched Router to expose routes");
    
    Ok(())
}

fn verify_build(rioterm_path: &Path) -> Result<()> {
    run_cmd(rioterm_path, "cargo", &["build", "--lib"])?;
    println!("✓ Library builds successfully");
    Ok(())
}

fn run_cmd(dir: &Path, cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .status()?;

    if !status.success() {
        return Err(format!("Command failed: {} {:?}", cmd, args).into());
    }
    Ok(())
}
