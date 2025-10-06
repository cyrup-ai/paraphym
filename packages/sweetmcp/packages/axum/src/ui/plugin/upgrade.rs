use std::{
    fs,
    path::Path,
    process::Command,
};

use clap::Args;
use crate::context::logger::ConsoleLogger;

#[derive(Args, Debug)]
pub struct UpgradeArgs {
    /// Upgrade all plugins in the plugins directory
    #[arg(short, long)]
    pub all: bool,

    /// Specific plugin name to upgrade
    #[arg(short, long)]
    pub name: Option<String>,

    /// Only print what would be upgraded without making changes
    #[arg(short, long)]
    pub dry_run: bool,
}

pub fn upgrade_plugins(args: &UpgradeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let logger = ConsoleLogger::new();
    log::info!("Upgrading plugin dependencies...");

    let project_root = std::env::current_dir()?;
    let plugins_dir = project_root.join("plugins");

    if !plugins_dir.exists() {
        return Err(format!("Plugins directory not found at {}", plugins_dir.display()).into());
    }

    // Get latest versions of common dependencies
    let latest_versions = get_latest_versions()?;

    // If a specific plugin is specified, only upgrade that one
    if let Some(plugin_name) = &args.name {
        let plugin_path = plugins_dir.join(plugin_name);
        if !plugin_path.exists() {
            return Err(format!(
                "Plugin '{}' not found at {}",
                plugin_name,
                plugin_path.display()
            )
            .into());
        }

        upgrade_plugin_dependencies(&plugin_path, &latest_versions, args.dry_run)?;
    } else if args.all {
        // Upgrade all plugins
        let mut upgraded_plugins = Vec::new();
        let mut failed_plugins = Vec::new();

        for entry in fs::read_dir(&plugins_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let plugin_name = match path.file_name() {
                // Already fixed
                Some(name) => name.to_string_lossy().to_string(),
                None => {
                    log::warn!("Skipping directory with invalid name: {}", path.display());
                    continue;
                }
            };

            match upgrade_plugin_dependencies(&path, &latest_versions, args.dry_run) {
                Ok(upgraded) => {
                    if upgraded {
                        upgraded_plugins.push(plugin_name);
                    }
                }
                Err(e) => {
                    failed_plugins.push((plugin_name, e.to_string()));
                }
            }
        }

        // Report summary
        log::info!("Upgrade Summary");

        if !upgraded_plugins.is_empty() {
            logger.success(&format!("{} plugins upgraded:", upgraded_plugins.len()));
            for name in upgraded_plugins {
                log::info!("  - {}", name);
            }
        } else if args.dry_run {
            logger.warn("No plugins need upgrading");
        }

        if !failed_plugins.is_empty() {
            logger.error(&format!("{} plugins failed to upgrade:", failed_plugins.len()));
            for (name, error) in failed_plugins {
                logger.error(&format!("  - {}: {}", name, error));
            }
            return Err("Some plugins failed to upgrade".into());
        }
    } else {
        // No plugin specified and --all not set
        log::info!("No plugin specified. Use --name <plugin_name> or --all to upgrade plugins.");
        log::info!("Available plugins:");

        for entry in fs::read_dir(&plugins_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Already fixed
                if let Some(plugin_name) = path.file_name() {
                    log::info!("  - {}", plugin_name.to_string_lossy());
                }
            }
        }
    }

    Ok(())
}
// Removed extra closing brace here

fn get_latest_versions() -> Result<Vec<(&'static str, String)>, Box<dyn std::error::Error>> {
    let logger = ConsoleLogger::new();
    let dependencies = [
        "extism-pdk",
        "serde",
        "serde_json",
        "base64",
        "base64-serde",
    ];

    let mut latest_versions = Vec::new();

    for &dep in &dependencies {
        log::info!("Checking latest version of {}...", dep);

        let output = Command::new("cargo")
            .args(["search", dep, "--limit", "1"])
            .output()?;

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let version = parse_crate_version(&output_str, dep);

            if let Some(version) = version {
                log::info!("  {} version: {}", dep, version);
                latest_versions.push((dep, version));
            } else {
                logger.warn(&format!("  {} version not found", dep));
            }
        } else {
            logger.error(&format!("  Failed to check {}", dep));
        }
    }

    Ok(latest_versions)
}

fn parse_crate_version(output: &str, crate_name: &str) -> Option<String> {
    let line = output.lines().next()?;
    let prefix = format!("{} = \"", crate_name);

    if line.starts_with(&prefix) {
        let version_start = prefix.len();
        let version_end = line[version_start..].find('"')?;

        Some(line[version_start..(version_start + version_end)].to_string())
    } else {
        None
    }
}

fn upgrade_plugin_dependencies(
    plugin_path: &Path,
    latest_versions: &[(&str, String)],
    dry_run: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let logger = ConsoleLogger::new();
    // Already fixed
    let plugin_name = plugin_path
        .file_name()
        .ok_or_else(|| format!("Invalid plugin path: {}", plugin_path.display()))?
        .to_string_lossy();
    log::info!("Upgrading dependencies for: {}", plugin_name);

    // Check Cargo.toml exists
    let cargo_path = plugin_path.join("Cargo.toml");
    if !cargo_path.exists() {
        return Err(format!("Cargo.toml not found in {}", plugin_path.display()).into());
    }

    // Read Cargo.toml
    let cargo_content = fs::read_to_string(&cargo_path)?;
    let mut lines: Vec<String> = cargo_content.lines().map(String::from).collect();
    let mut upgraded = false;

    // Update dependency versions
    for line in &mut lines {
        for &(dep, ref ver) in latest_versions {
            if line.contains(&format!("{} = ", dep)) {
                if dep == "serde" && line.contains("features") {
                    let current_ver = line.clone();
                    if current_ver.contains(&format!("version = \"{}", ver)) {
                        // Already at latest version
                        continue;
                    }

                    let new_line = line.replace(
                        &format!("{} = {{ version = \"", dep),
                        &format!("{} = {{ version = \"{}\"", dep, ver),
                    );

                    log::info!("  {} → {}", current_ver, new_line);

                    if !dry_run {
                        *line = new_line;
                    }
                    upgraded = true;
                } else {
                    let current_ver = line.clone();
                    if current_ver.contains(&format!("{} = \"{}\"", dep, ver)) {
                        // Already at latest version
                        continue;
                    }

                    let new_line = line
                        .replace(&format!("{} = \"", dep), &format!("{} = \"{}\"", dep, ver));

                    log::info!("  {} → {}", current_ver, new_line);

                    if !dry_run {
                        *line = new_line;
                    }
                    upgraded = true;
                }
            }
        }
    }

    if !upgraded {
        log::info!("  All dependencies are already at the latest version");
        return Ok(false);
    }

    // Write updated content back if not dry run
    if !dry_run {
        fs::write(&cargo_path, lines.join("\n"))?;
        logger.success(&format!("Dependencies upgraded in {}", cargo_path.display()));
    } else {
        log::info!("  Dry run - no changes made");
    }

    Ok(upgraded)
}
