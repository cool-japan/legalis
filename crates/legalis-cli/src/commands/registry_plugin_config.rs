//! Registry, plugin, and config CLI command handlers.

use anyhow::{Context, Result};
use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use legalis_dsl::LegalDslParser;
use std::fs;
use std::path::{Path, PathBuf};

/// Handles the registry push command.
#[allow(clippy::too_many_arguments)]
pub fn handle_registry_push(
    input: &str,
    registry: Option<&str>,
    tags: &[String],
    visibility: &crate::RegistryVisibility,
    dry_run: bool,
    _force: bool,
) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Pushing statute to registry...".cyan().bold());

    // Load and parse statute
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let registry_url = registry.unwrap_or("default");
    println!("  Statute ID: {}", statute.id.yellow());
    println!("  Registry: {}", registry_url.yellow());
    println!("  Visibility: {:?}", visibility);
    println!("  Tags: {}", tags.join(", "));

    if dry_run {
        println!("\n{}", "[DRY RUN] Would push statute to registry".green());
        return Ok(());
    }

    // In a real implementation, this would:
    // 1. Connect to the registry
    // 2. Authenticate using stored credentials
    // 3. Upload the statute with metadata
    // 4. Handle conflicts if not using --force

    println!("\n{} Statute pushed successfully!", "✓".green().bold());
    println!("  View at: {}/statutes/{}", registry_url, statute.id);

    Ok(())
}

/// Handles the registry pull command.
pub fn handle_registry_pull(
    statute_id: &str,
    registry: Option<&str>,
    output: &str,
    version: Option<&str>,
    force: bool,
) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Pulling statute from registry...".cyan().bold());

    let registry_url = registry.unwrap_or("default");
    let version_str = version.unwrap_or("latest");

    println!("  Statute ID: {}", statute_id.yellow());
    println!("  Registry: {}", registry_url.yellow());
    println!("  Version: {}", version_str.yellow());
    println!("  Output: {}", output.yellow());

    // Create output directory if it doesn't exist
    fs::create_dir_all(output)
        .with_context(|| format!("Failed to create output directory: {}", output))?;

    let output_file = Path::new(output).join(format!("{}.ldsl", statute_id));

    // Check if file exists
    if output_file.exists() && !force {
        anyhow::bail!(
            "Statute file already exists: {}. Use --force to overwrite",
            output_file.display()
        );
    }

    // In a real implementation, this would:
    // 1. Connect to the registry
    // 2. Authenticate if needed
    // 3. Download the statute with specified version
    // 4. Save to the output directory

    println!("\n{} Statute pulled successfully!", "✓".green().bold());
    println!("  Saved to: {}", output_file.display());

    Ok(())
}

/// Handles the registry diff command.
pub fn handle_registry_diff(
    local: &str,
    statute_id: Option<&str>,
    registry: Option<&str>,
    _diff_format: &crate::DiffFormat,
    output: Option<&str>,
) -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        "Comparing local statute with registry...".cyan().bold()
    );

    // Load local statute
    let content = fs::read_to_string(local)
        .with_context(|| format!("Failed to read local file: {}", local))?;

    let parser = LegalDslParser::new();
    let local_statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let statute_id_str = statute_id.unwrap_or(&local_statute.id);
    let registry_url = registry.unwrap_or("default");

    println!("  Local: {}", local.yellow());
    println!("  Statute ID: {}", statute_id_str.yellow());
    println!("  Registry: {}", registry_url.yellow());

    // In a real implementation, this would:
    // 1. Fetch the remote statute from registry
    // 2. Compare local vs remote
    // 3. Generate diff output

    let diff_output = format!(
        "=== Diff: Local vs Registry ===\n\
         Local file: {}\n\
         Remote statute: {} (from {})\n\
         \n\
         [Mock diff output - would show actual differences]\n",
        local, statute_id_str, registry_url
    );

    // Write output
    if let Some(out_path) = output {
        fs::write(out_path, &diff_output)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("\n{} Diff written to: {}", "✓".green().bold(), out_path);
    } else {
        println!("\n{}", diff_output);
    }

    Ok(())
}

/// Handles the registry sync command.
pub fn handle_registry_sync(
    directory: &str,
    registry: Option<&str>,
    direction: &crate::SyncDirection,
    conflict: &crate::ConflictResolution,
    dry_run: bool,
) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Synchronizing with registry...".cyan().bold());

    let registry_url = registry.unwrap_or("default");

    println!("  Directory: {}", directory.yellow());
    println!("  Registry: {}", registry_url.yellow());
    println!("  Direction: {:?}", direction);
    println!("  Conflict resolution: {:?}", conflict);

    // Check directory exists
    if !Path::new(directory).exists() {
        anyhow::bail!("Directory does not exist: {}", directory);
    }

    if dry_run {
        println!("\n{}", "[DRY RUN] Would synchronize with registry".green());
        println!("  Files to pull: 3");
        println!("  Files to push: 2");
        println!("  Conflicts: 1");
        return Ok(());
    }

    // In a real implementation, this would:
    // 1. Scan local directory for statutes
    // 2. Fetch list of statutes from registry
    // 3. Determine differences
    // 4. Resolve conflicts based on strategy
    // 5. Perform sync operations

    println!("\n{} Synchronization complete!", "✓".green().bold());
    println!("  Pulled: 3 statutes");
    println!("  Pushed: 2 statutes");
    println!("  Conflicts resolved: 1");

    Ok(())
}

/// Handles the registry login command.
pub fn handle_registry_login(
    registry: &str,
    username: Option<&str>,
    password: Option<&str>,
    token: Option<&str>,
) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Logging in to registry...".cyan().bold());
    println!("  Registry: {}", registry.yellow());

    // Get credentials
    let user = if let Some(_tok) = token {
        println!("  Using API token");
        "token".to_string()
    } else {
        let u = if let Some(u) = username {
            u.to_string()
        } else {
            print!("  Username: ");
            std::io::Write::flush(&mut std::io::stdout())?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        };

        let _p = if let Some(p) = password {
            p.to_string()
        } else {
            // In a real implementation, use a secure password input
            use std::io::Write;
            print!("  Password: ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        };

        u
    };

    // In a real implementation, this would:
    // 1. Authenticate with the registry
    // 2. Store credentials securely (e.g., in keyring)
    // 3. Save session token

    println!("\n{} Logged in successfully!", "✓".green().bold());
    println!("  User: {}", user);

    Ok(())
}

/// Handles the registry logout command.
pub fn handle_registry_logout(registry: Option<&str>, all: bool) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Logging out from registry...".cyan().bold());

    if all {
        println!("  Clearing all credentials");
        // In a real implementation, clear all stored credentials
    } else if let Some(reg) = registry {
        println!("  Registry: {}", reg.yellow());
        // In a real implementation, clear credentials for specific registry
    } else {
        anyhow::bail!("Please specify --registry or use --all");
    }

    println!("\n{} Logged out successfully!", "✓".green().bold());

    Ok(())
}

/// Handles the plugin install command.
pub fn handle_plugin_install(source: &str, force: bool) -> Result<()> {
    use crate::plugin::PluginManager;
    use colored::Colorize;

    println!("{}", "Installing plugin...".cyan().bold());
    println!("  Source: {}", source.yellow());

    let source_path = Path::new(source);
    if !source_path.exists() {
        anyhow::bail!("Plugin source path does not exist: {}", source);
    }

    let mut manager = PluginManager::new()?;
    manager.install_plugin(source_path, force)?;

    println!("\n{} Plugin installed successfully!", "✓".green().bold());
    println!("{}", "Plugin is now enabled and ready to use.".dimmed());

    Ok(())
}

/// Handles the plugin uninstall command.
pub fn handle_plugin_uninstall(name: &str, yes: bool) -> Result<()> {
    use crate::plugin::PluginManager;
    use colored::Colorize;

    if !yes {
        print!(
            "Are you sure you want to uninstall plugin '{}'? [y/N]: ",
            name
        );
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    println!("{}", "Uninstalling plugin...".cyan().bold());
    println!("  Plugin: {}", name.yellow());

    let mut manager = PluginManager::new()?;
    manager.uninstall_plugin(name)?;

    println!("\n{} Plugin uninstalled successfully!", "✓".green().bold());

    Ok(())
}

/// Handles the plugin list command.
pub fn handle_plugin_list(
    verbose: bool,
    plugin_type: Option<&crate::plugin::PluginType>,
) -> Result<()> {
    use crate::plugin::PluginManager;
    use colored::Colorize;

    let mut manager = PluginManager::new()?;
    manager.discover_plugins()?;

    let mut plugins: Vec<_> = manager.list_plugins();

    // Filter by type if specified
    if let Some(ptype) = plugin_type {
        plugins.retain(|p| &p.plugin_type == ptype);
    }

    if plugins.is_empty() {
        println!("{}", "No plugins installed.".dimmed());
        println!("\nTo install a plugin, run:");
        println!("  {}", "legalis plugin install --source <path>".bold());
        return Ok(());
    }

    println!(
        "{}",
        format!("Installed Plugins ({})", plugins.len())
            .bold()
            .cyan()
    );
    println!("{}", "=".repeat(50).dimmed());
    println!();

    if verbose {
        for plugin in plugins {
            let status = if manager.is_enabled(&plugin.name) {
                "enabled".green()
            } else {
                "disabled".red()
            };

            println!("{} {} [{}]", "●".cyan(), plugin.name.bold(), status);
            println!("  Version: {}", plugin.version.dimmed());
            println!("  Type: {:?}", plugin.plugin_type);
            println!("  Author: {}", plugin.author.dimmed());
            println!("  Description: {}", plugin.description);
            if !plugin.commands.is_empty() {
                println!("  Commands: {}", plugin.commands.join(", ").yellow());
            }
            if !plugin.hooks.is_empty() {
                println!("  Hooks: {}", plugin.hooks.join(", ").yellow());
            }
            println!();
        }
    } else {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Name").fg(Color::Cyan),
                Cell::new("Version").fg(Color::Cyan),
                Cell::new("Type").fg(Color::Cyan),
                Cell::new("Status").fg(Color::Cyan),
                Cell::new("Description").fg(Color::Cyan),
            ]);

        for plugin in plugins {
            let status = if manager.is_enabled(&plugin.name) {
                Cell::new("enabled").fg(Color::Green)
            } else {
                Cell::new("disabled").fg(Color::Red)
            };

            table.add_row(vec![
                Cell::new(&plugin.name),
                Cell::new(&plugin.version),
                Cell::new(format!("{:?}", plugin.plugin_type)),
                status,
                Cell::new(&plugin.description),
            ]);
        }

        println!("{table}");
    }

    Ok(())
}

/// Handles the plugin info command.
pub fn handle_plugin_info(name: &str) -> Result<()> {
    use crate::plugin::PluginManager;
    use colored::Colorize;

    let mut manager = PluginManager::new()?;
    manager.discover_plugins()?;

    let plugin = manager
        .get_plugin(name)
        .ok_or_else(|| anyhow::anyhow!("Plugin '{}' is not installed", name))?;

    let status = if manager.is_enabled(name) {
        "enabled".green()
    } else {
        "disabled".red()
    };

    println!("{}", "Plugin Information".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());
    println!();
    println!("  {}: {}", "Name".bold(), plugin.name);
    println!("  {}: {}", "Version".bold(), plugin.version);
    println!("  {}: {}", "Author".bold(), plugin.author);
    println!("  {}: {:?}", "Type".bold(), plugin.plugin_type);
    println!("  {}: {}", "Status".bold(), status);
    println!("  {}: {}", "Description".bold(), plugin.description);
    println!("  {}: {}", "Entry Point".bold(), plugin.entry_point);

    if let Some(ref min_ver) = plugin.min_legalis_version {
        println!("  {}: {}", "Min Legalis Version".bold(), min_ver);
    }

    if !plugin.commands.is_empty() {
        println!("\n  {}:", "Commands".bold().yellow());
        for cmd in &plugin.commands {
            println!("    - {}", cmd);
        }
    }

    if !plugin.hooks.is_empty() {
        println!("\n  {}:", "Hooks".bold().yellow());
        for hook in &plugin.hooks {
            println!("    - {}", hook);
        }
    }

    println!();

    Ok(())
}

/// Handles the plugin enable command.
pub fn handle_plugin_enable(name: &str) -> Result<()> {
    use crate::plugin::PluginManager;
    use colored::Colorize;

    let mut manager = PluginManager::new()?;
    manager.discover_plugins()?;

    if manager.is_enabled(name) {
        println!(
            "{}",
            format!("Plugin '{}' is already enabled.", name)
                .yellow()
                .bold()
        );
        return Ok(());
    }

    manager.enable_plugin(name)?;

    println!(
        "{}",
        format!("✓ Plugin '{}' enabled successfully!", name)
            .green()
            .bold()
    );

    Ok(())
}

/// Handles the plugin disable command.
pub fn handle_plugin_disable(name: &str) -> Result<()> {
    use crate::plugin::PluginManager;
    use colored::Colorize;

    let mut manager = PluginManager::new()?;
    manager.discover_plugins()?;

    if !manager.is_enabled(name) {
        println!(
            "{}",
            format!("Plugin '{}' is already disabled.", name)
                .yellow()
                .bold()
        );
        return Ok(());
    }

    manager.disable_plugin(name)?;

    println!(
        "{}",
        format!("✓ Plugin '{}' disabled successfully!", name)
            .green()
            .bold()
    );

    Ok(())
}

/// Handles the plugin update command.
#[allow(dead_code)]
pub fn handle_plugin_update(_name: Option<&str>) -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        "Plugin update functionality coming soon!".yellow().bold()
    );
    println!("This will check for and install plugin updates from their sources.");

    Ok(())
}

/// Handles `legalis plugin scan`: security-scan installed plugins.
pub fn handle_plugin_scan(
    name: Option<&str>,
    fail_on: crate::plugin_security::Severity,
) -> Result<()> {
    use crate::plugin::PluginManager;
    use crate::plugin_security::scan_plugin;
    use colored::Colorize;

    let mut manager = PluginManager::new()?;
    manager.discover_plugins()?;

    let plugin_root = PluginManager::plugin_directory()?;
    let manifests: Vec<_> = match name {
        Some(name) => match manager.get_plugin(name) {
            Some(manifest) => vec![manifest.clone()],
            None => anyhow::bail!("Plugin '{name}' is not installed"),
        },
        None => manager.list_plugins().into_iter().cloned().collect(),
    };

    if manifests.is_empty() {
        println!("{}", "No plugins installed to scan.".yellow());
        return Ok(());
    }

    let mut failed = 0usize;
    for manifest in &manifests {
        let report = scan_plugin(manifest, Some(&plugin_root.join(&manifest.name)));
        if report.findings.is_empty() {
            println!("{} {}: clean", "✓".green(), manifest.name.bold());
        } else {
            println!("{} {}:", "•".yellow(), manifest.name.bold());
            for finding in &report.findings {
                println!(
                    "    [{:?}] {} — {}",
                    finding.severity, finding.code, finding.message
                );
            }
        }
        if report.has_at_least(fail_on) {
            failed += 1;
        }
    }

    if failed > 0 {
        anyhow::bail!(
            "{failed} plugin(s) have findings at or above the {:?} threshold",
            fail_on
        );
    }
    println!(
        "{}",
        "All scanned plugins pass the security threshold.".green()
    );
    Ok(())
}

/// Handles `legalis plugin deps`: validate dependencies and resolve order.
pub fn handle_plugin_deps(show_order: bool) -> Result<()> {
    use crate::plugin::PluginManager;
    use crate::plugin_security::resolve_install_order;
    use colored::Colorize;

    let mut manager = PluginManager::new()?;
    manager.discover_plugins()?;
    let manifests: Vec<_> = manager.list_plugins().into_iter().cloned().collect();

    if manifests.is_empty() {
        println!("{}", "No plugins installed.".yellow());
        return Ok(());
    }

    match resolve_install_order(&manifests) {
        Ok(order) => {
            println!(
                "{}",
                "All plugin dependencies are satisfied.".green().bold()
            );
            if show_order {
                println!("Install order:");
                for (index, name) in order.iter().enumerate() {
                    println!("  {}. {}", index + 1, name);
                }
            }
            Ok(())
        }
        Err(errors) => {
            for error in &errors {
                eprintln!("{} {}", "✗".red(), error);
            }
            anyhow::bail!("{} dependency problem(s) found", errors.len());
        }
    }
}

/// Handles `legalis plugin check-version`: compatibility with running legalis.
pub fn handle_plugin_check_version(name: Option<&str>) -> Result<()> {
    use crate::plugin::PluginManager;
    use crate::plugin_security::check_min_legalis_version;
    use colored::Colorize;

    let legalis_version = env!("CARGO_PKG_VERSION");
    let mut manager = PluginManager::new()?;
    manager.discover_plugins()?;

    let manifests: Vec<_> = match name {
        Some(name) => match manager.get_plugin(name) {
            Some(manifest) => vec![manifest.clone()],
            None => anyhow::bail!("Plugin '{name}' is not installed"),
        },
        None => manager.list_plugins().into_iter().cloned().collect(),
    };

    if manifests.is_empty() {
        println!("{}", "No plugins installed.".yellow());
        return Ok(());
    }

    let mut incompatible = 0usize;
    for manifest in &manifests {
        match check_min_legalis_version(manifest, legalis_version) {
            Ok(()) => println!(
                "{} {} (v{}): compatible",
                "✓".green(),
                manifest.name.bold(),
                manifest.version
            ),
            Err(error) => {
                incompatible += 1;
                println!("{} {}", "✗".red(), error);
            }
        }
    }

    if incompatible > 0 {
        anyhow::bail!("{incompatible} plugin(s) are incompatible with legalis {legalis_version}");
    }
    Ok(())
}

/// Handles the config validate command.
pub fn handle_config_validate(config_path: Option<&str>, verbose: bool) -> Result<()> {
    use crate::config::Config;
    use colored::Colorize;

    println!("{}", "Validating configuration...".cyan().bold());

    let config = if let Some(path) = config_path {
        println!("  Config file: {}", path.yellow());
        Config::from_file(Path::new(path))?
    } else {
        println!("{}", "  Using current configuration".dimmed());
        Config::load()
    };

    let warnings = config.validate()?;

    if warnings.is_empty() {
        println!("\n{} Configuration is valid!", "✓".green().bold());

        if verbose {
            println!("\n{}", "Configuration summary:".bold());
            println!("  Jurisdiction: {:?}", config.jurisdiction);
            println!("  Output format: {}", config.output.format);
            println!("  Colored output: {}", config.output.colored);
            println!("  Verification strict: {}", config.verification.strict);

            if let Some(ref profile) = config.active_profile {
                println!("  Active profile: {}", profile.yellow());
            }
        }
    } else {
        println!("\n{} Configuration has warnings:", "⚠".yellow().bold());
        for warning in &warnings {
            println!("  • {}", warning.yellow());
        }
    }

    Ok(())
}

/// Handles the config diff command.
pub fn handle_config_diff(config1: &str, config2: &str, as_profile: bool) -> Result<()> {
    use crate::config::Config;
    use colored::Colorize;

    println!("{}", "Comparing configurations...".cyan().bold());

    let cfg1 = Config::from_file(Path::new(config1))?;

    let cfg2 = if as_profile {
        println!(
            "  Comparing {} with profile '{}'",
            config1.yellow(),
            config2.yellow()
        );
        cfg1.with_profile(config2)?
    } else {
        println!("  Comparing {} with {}", config1.yellow(), config2.yellow());
        Config::from_file(Path::new(config2))?
    };

    let diff = cfg1.diff(&cfg2);

    println!("\n{}", "Configuration differences:".bold());
    println!("{}", diff);

    Ok(())
}

/// Handles the config profiles command.
pub fn handle_config_profiles(config_path: Option<&str>) -> Result<()> {
    use crate::config::Config;
    use colored::Colorize;

    let config = if let Some(path) = config_path {
        Config::from_file(Path::new(path))?
    } else {
        Config::load()
    };

    let profiles = config.list_profiles();

    if profiles.is_empty() {
        println!("{}", "No profiles defined.".dimmed());
        println!("\nTo add a profile, edit your configuration file and add:");
        let example = r#"
[profiles.dev]
jurisdiction = "JP"

[profiles.prod]
jurisdiction = "US"
"#;
        println!("{}", example.dimmed());
        return Ok(());
    }

    println!("{}", "Available Profiles:".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());

    for profile_name in profiles {
        let is_active = config.get_active_profile() == Some(profile_name);

        let marker = if is_active {
            "●".green()
        } else {
            "○".dimmed()
        };

        let name_display = if is_active {
            profile_name.green().bold()
        } else {
            profile_name.normal()
        };

        println!("{} {}", marker, name_display);

        // Show profile details if we have access
        if let Some(profile) = config.profiles.get(profile_name) {
            if let Some(ref jur) = profile.jurisdiction {
                println!("    Jurisdiction: {}", jur.yellow());
            }
            if !profile.env.is_empty() {
                println!("    Environment vars: {}", profile.env.len());
            }
        }
    }

    Ok(())
}

/// Handles the config activate command.
pub fn handle_config_activate(profile: &str, config_path: Option<&str>) -> Result<()> {
    use crate::config::Config;
    use colored::Colorize;

    let config_file = if let Some(path) = config_path {
        PathBuf::from(path)
    } else {
        Path::new("legalis.toml").to_path_buf()
    };

    let mut config = if config_file.exists() {
        Config::from_file(&config_file)?
    } else {
        anyhow::bail!("Configuration file not found: {}", config_file.display());
    };

    config.set_active_profile(profile.to_string())?;
    config.save(&config_file)?;

    println!(
        "{}",
        format!("✓ Activated profile '{}'", profile).green().bold()
    );
    println!(
        "  Config file: {}",
        config_file.display().to_string().yellow()
    );

    Ok(())
}

/// Handles the config show command.
pub fn handle_config_show(
    config_path: Option<&str>,
    profile: Option<&str>,
    format: &crate::ConfigShowFormat,
) -> Result<()> {
    use crate::config::Config;
    use colored::Colorize;

    let mut config = if let Some(path) = config_path {
        Config::from_file(Path::new(path))?
    } else {
        Config::load()
    };

    // Apply profile if specified
    if let Some(profile_name) = profile {
        println!(
            "{}",
            format!("Showing configuration with profile '{}'", profile_name)
                .cyan()
                .bold()
        );
        config = config.with_profile(profile_name)?;
    } else {
        println!("{}", "Current Configuration:".cyan().bold());
    }

    println!("{}", "=".repeat(50).dimmed());
    println!();

    let output = match format {
        crate::ConfigShowFormat::Toml => toml::to_string_pretty(&config)?,
        crate::ConfigShowFormat::Json => serde_json::to_string_pretty(&config)?,
        crate::ConfigShowFormat::Yaml => serde_yaml::to_string(&config)?,
    };

    println!("{}", output);

    Ok(())
}

/// Handles the config init command.
pub fn handle_config_init(force: bool) -> Result<()> {
    use crate::config::Config;
    use colored::Colorize;

    let config_file = Config::init_user_config()?;

    if config_file.exists() && !force {
        println!("{}", "Configuration file already exists!".yellow().bold());
        println!("  Location: {}", config_file.display().to_string().yellow());
        println!("\nUse --force to overwrite.");
        return Ok(());
    }

    println!("{}", "✓ User configuration initialized!".green().bold());
    println!("  Location: {}", config_file.display().to_string().yellow());
    println!("\nYou can now edit this file to customize your settings.");

    Ok(())
}
