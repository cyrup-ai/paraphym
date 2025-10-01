// riovt/src/setup.rs
use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::Write;
use toml_edit;

fn run_command(dir: &str, cmd: &str, args: &[&str]) -> Result<(), String> {
    println!("Running command: {} {:?} in dir {}", cmd, args, dir);
    let output = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| format!("Failed to execute command '{}': {}", cmd, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Command '{} {:?}' failed in dir {}:\nStatus: {}\nStdout:\n{}\nStderr:\n{}",
            cmd, args, dir, output.status, stdout, stderr
        ));
    }
    println!("Command successful: {} {:?} for dir {}", cmd, args, dir);
    Ok(())
}

fn inject_library_file(rioterm_crate_path: &str) -> Result<(), String> {
    let main_rs_path = Path::new(rioterm_crate_path).join("src").join("main.rs");
    let lib_rs_path = Path::new(rioterm_crate_path).join("src").join("lib.rs");
    
    println!("Analyzing rioterm structure at: {}", rioterm_crate_path);
    
    // Verify main.rs exists
    if !main_rs_path.exists() {
        return Err(format!("main.rs not found at: {}", main_rs_path.display()));
    }
    
    // Read main.rs to understand the module structure
    let main_rs_content = fs::read_to_string(&main_rs_path)
        .map_err(|e| format!("Failed to read main.rs: {}", e))?;
    
    // Extract module declarations (lines starting with "mod ")
    let mut modules = Vec::new();
    for line in main_rs_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("mod ") {
            // Extract module name: "mod application;" -> "application"
            if let Some(mod_name) = trimmed
                .strip_prefix("mod ")
                .and_then(|s| s.strip_suffix(';'))
            {
                modules.push(mod_name.trim().to_string());
            }
        }
    }
    
    println!("Found {} modules in main.rs", modules.len());
    
    // Generate lib.rs content that re-exports the modules
    let mut lib_rs_content = String::from(
r#"//! Rioterm library interface
//! 
//! This library provides programmatic access to rioterm terminal emulator components.
//! 
//! **Note:** Most functionality is available through the separate library crates:
//! - `rio-backend` - Configuration, events, clipboard
//! - `rio-window` - Window management, event loop
//! - `sugarloaf` - Rendering engine
//! - `teletypewriter` - Terminal emulation
//!
//! This library exposes rioterm's internal modules for advanced use cases.

#![allow(dead_code)]
#![allow(unused_imports)]

// Re-export all modules from main.rs
"#);
    
    // Add module declarations and re-exports
    for module in &modules {
        lib_rs_content.push_str(&format!("pub mod {};\n", module));
    }
    
    lib_rs_content.push_str("\n");
    lib_rs_content.push_str(
r#"
// Re-export commonly used types and functions
// These would need to be made public in their respective modules

// Example: if you want to expose setup_environment_variables from main.rs:
// pub use crate::setup_environment_variables;

// Note: Most modules in rioterm are private. To use them as a library,
// you'll need to add `pub` to the module declarations in main.rs:
// Change `mod application;` to `pub mod application;`
"#);
    
    println!("Writing lib.rs to: {}", lib_rs_path.display());
    
    let parent_dir = lib_rs_path.parent()
        .ok_or_else(|| "Failed to get parent directory for lib.rs".to_string())?;
    fs::create_dir_all(parent_dir)
        .map_err(|e| format!("Failed to create directory {}: {}", parent_dir.display(), e))?;
    
    let mut file = fs::File::create(&lib_rs_path)
        .map_err(|e| format!("Failed to create {}: {}", lib_rs_path.display(), e))?;
    
    file.write_all(lib_rs_content.as_bytes())
        .map_err(|e| format!("Failed to write to {}: {}", lib_rs_path.display(), e))?;
    
    println!("✓ Successfully created lib.rs with {} module re-exports", modules.len());
    
    // Warning about module visibility
    println!("\n⚠️  WARNING: The modules in rioterm/src/ are not public by default.");
    println!("   To use them as a library, you'll need to modify main.rs to make modules public:");
    println!("   Change `mod application;` to `pub mod application;` for each module.");
    
    Ok(())
}

fn ensure_cargo_lib_target(rioterm_crate_path: &str) -> Result<(), String> {
    let cargo_toml_path = Path::new(rioterm_crate_path).join("Cargo.toml");
    
    println!("Checking Cargo.toml for [lib] section at: {}", cargo_toml_path.display());
    
    // Read the current Cargo.toml
    let toml_content = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;
    
    // Parse as toml_edit::DocumentMut for editing
    let mut doc = toml_content.parse::<toml_edit::DocumentMut>()
        .map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;
    
    // Check if [lib] section already exists
    if doc.contains_key("lib") {
        println!("✓ [lib] section already exists in Cargo.toml");
        
        // Verify it has the right configuration
        if let Some(lib_item) = doc.get("lib") {
            if let Some(lib_table) = lib_item.as_table() {
                let name = lib_table.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let path = lib_table.get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                println!("  Library name: {}", name);
                println!("  Library path: {}", path);
            }
        }
        
        return Ok(());
    }
    
    println!("Adding [lib] section to Cargo.toml...");
    
    // Create a new [lib] table
    let mut lib_table = toml_edit::Table::new();
    
    // Set the library name to "rioterm" (same as package name)
    lib_table.insert("name", toml_edit::value("rioterm"));
    
    // Set the library path to src/lib.rs
    lib_table.insert("path", toml_edit::value("src/lib.rs"));
    
    // Add the [lib] table to the document
    doc["lib"] = toml_edit::Item::Table(lib_table);
    
    // Write the modified TOML back to file
    fs::write(&cargo_toml_path, doc.to_string())
        .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;
    
    println!("✓ Added [lib] section to Cargo.toml:");
    println!("  [lib]");
    println!("  name = \"rioterm\"");
    println!("  path = \"src/lib.rs\"");
    
    Ok(())
}


fn verify_library_builds(rioterm_crate_path: &str) -> Result<(), String> {
    println!("\nVerifying library builds...");
    
    let output = Command::new("cargo")
        .args(&["build", "--lib", "--message-format=short"])
        .current_dir(rioterm_crate_path)
        .output()
        .map_err(|e| format!("Failed to run cargo: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Library build failed:\n\nStdout:\n{}\n\nStderr:\n{}",
            stdout, stderr
        ));
    }
    
    println!("✓ Library builds successfully");
    
    // Also verify the binary still builds
    println!("\nVerifying binary still builds...");
    
    let output = Command::new("cargo")
        .args(&["build", "--bin", "rio", "--message-format=short"])
        .current_dir(rioterm_crate_path)
        .output()
        .map_err(|e| format!("Failed to run cargo: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Binary build failed after library addition:\n\nStdout:\n{}\n\nStderr:\n{}",
            stdout, stderr
        ));
    }
    
    println!("✓ Binary still builds successfully");
    println!("\n✅ Both library and binary build successfully!");
    
    Ok(())
}

fn main() -> Result<(), String> {
    // Path to the rio source directory within the project structure
    let rio_source_dir = "riovt/vendor/rio";
    // Path to the rioterm crate within the rio source directory
    let rioterm_crate_path = Path::new(rio_source_dir).join("frontends").join("rioterm");
    let rioterm_crate_path_str = rioterm_crate_path.to_str()
        .ok_or("Invalid path for rioterm crate")?;

    // --- Git Operations ---
    println!("Starting Git operations in {}", rio_source_dir);
    if !Path::new(rio_source_dir).exists() {
        return Err(format!(
            "RIO_SOURCE_DIR '{}' does not exist. Please check the path.", 
            rio_source_dir
        ));
    }
    if !Path::new(rio_source_dir).join(".git").exists() {
         return Err(format!(
             "RIO_SOURCE_DIR '{}' does not appear to be a git repository.", 
             rio_source_dir
         ));
    }

    run_command(rio_source_dir, "git", &["reset", "--hard"])?;
    run_command(rio_source_dir, "git", &["clean", "-fd"])?;
    run_command(rio_source_dir, "git", &["fetch", "origin"])?;
    run_command(rio_source_dir, "git", &["merge", "origin/main"])?;
    println!("✓ Git operations completed successfully.");

    // --- Dynamic Library Injection ---
    println!("\n=== Starting Library Injection ===\n");
    
    inject_library_file(rioterm_crate_path_str)?;
    ensure_cargo_lib_target(rioterm_crate_path_str)?;
    verify_library_builds(rioterm_crate_path_str)?;
    
    println!("\n=== Library injection complete ===");
    println!("\n✅ Setup script finished successfully.");
    Ok(())
}
