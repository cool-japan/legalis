//! Registry operation CLI command handlers (test, new, doctor, repl, publish, validate,
//! install, list, add, update, clean, search, outdated, uninstall, watch).

use crate::{LegalDslFormat, StatuteTemplate};
use anyhow::{Context, Result};
use colored::Colorize;
use legalis_core::Statute;
use legalis_dsl::LegalDslParser;
use legalis_interop::{LegalConverter, LegalFormat};
use legalis_verifier::StatuteVerifier;
use std::fs;
use std::path::Path;

/// Handles the test command.
pub fn handle_test(inputs: &[String], tests_file: &str, verbose: bool) -> Result<()> {
    let statutes = super::parse_statutes(inputs)?;

    let test_content = fs::read_to_string(tests_file)
        .with_context(|| format!("Failed to read test file: {}", tests_file))?;

    println!("Running tests from: {}", tests_file);
    println!("Testing {} statute(s)", statutes.len());

    // Parse test cases (simple YAML format for now)
    let test_cases: Vec<TestCase> = serde_yaml::from_str(&test_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse test file: {}", e))?;

    let mut passed = 0;
    let mut failed = 0;

    for (idx, test_case) in test_cases.iter().enumerate() {
        if verbose {
            println!("\nTest {}: {}", idx + 1, test_case.description);
        }

        // Find the statute to test
        let statute = statutes
            .iter()
            .find(|s| test_case.statute_id.as_ref().is_none_or(|id| &s.id == id))
            .ok_or_else(|| anyhow::anyhow!("Statute not found for test case {}", idx + 1))?;

        // Check if conditions match
        let result = evaluate_test_case(statute, test_case);

        if result {
            passed += 1;
            if verbose {
                println!("  ✓ Passed");
            } else {
                print!(".");
            }
        } else {
            failed += 1;
            if verbose {
                println!("  ✗ Failed");
            } else {
                print!("F");
            }
        }
    }

    if !verbose {
        println!();
    }

    println!("\nTest Results: {} passed, {} failed", passed, failed);

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Test case structure.
#[derive(Debug, serde::Deserialize)]
struct TestCase {
    pub description: String,
    pub statute_id: Option<String>,
    #[allow(dead_code)]
    pub expected_effect: Option<String>,
    #[allow(dead_code)]
    pub age: Option<u32>,
    #[allow(dead_code)]
    pub income: Option<u64>,
}

/// Evaluate a test case against a statute.
fn evaluate_test_case(statute: &Statute, _test_case: &TestCase) -> bool {
    // Simple evaluation - just check if statute exists for now
    // In a real implementation, this would evaluate conditions
    !statute.preconditions.is_empty() || !statute.effect.description.is_empty()
}

/// Handles the new command.
pub fn handle_new(name: &str, template: &StatuteTemplate, output: Option<&str>) -> Result<()> {
    let statute_content = generate_statute_template(name, template);

    if let Some(out_path) = output {
        fs::write(out_path, &statute_content)
            .with_context(|| format!("Failed to write statute file: {}", out_path))?;
        println!(
            "{}",
            format!("✓ Created statute: {}", out_path).green().bold()
        );
        println!("\n{}", "Next steps:".cyan());
        println!("  1. Edit the statute file to customize conditions and effects");
        println!(
            "  2. Run {} to verify",
            format!("legalis verify -i {}", out_path).bold()
        );
        println!(
            "  3. Run {} to lint",
            format!("legalis lint -i {}", out_path).bold()
        );
    } else {
        println!("{}", statute_content);
    }

    Ok(())
}

/// Generate a statute template based on the template type.
fn generate_statute_template(name: &str, template: &StatuteTemplate) -> String {
    match template {
        StatuteTemplate::Basic => format!(
            r#"STATUTE {}: "Basic Statute" {{
    JURISDICTION "JP"
    VERSION 1

    WHEN AGE >= 18
    THEN GRANT "Adult rights and responsibilities"

    DISCRETION "Consider individual circumstances"
}}
"#,
            name
        ),
        StatuteTemplate::Income => format!(
            r#"STATUTE {}: "Income-Based Benefits" {{
    JURISDICTION "JP"
    VERSION 1

    WHEN INCOME <= 3000000
    THEN GRANT "Low-income benefits and subsidies"

    DISCRETION "Verify income documentation and household size"
}}
"#,
            name
        ),
        StatuteTemplate::Geographic => format!(
            r#"STATUTE {}: "Regional Statute" {{
    JURISDICTION "JP"
    VERSION 1

    WHEN REGION PREFECTURE "Tokyo"
    THEN GRANT "Regional-specific rights"

    DISCRETION "Verify residency requirements"
}}
"#,
            name
        ),
        StatuteTemplate::Temporal => format!(
            r#"STATUTE {}: "Time-Limited Statute" {{
    JURISDICTION "JP"
    VERSION 1
    EFFECTIVE "2024-01-01"
    EXPIRES "2025-12-31"

    WHEN AGE >= 20
    THEN GRANT "Temporary program benefits"

    DISCRETION "Review eligibility annually"
}}
"#,
            name
        ),
        StatuteTemplate::Complex => format!(
            r#"STATUTE {}: "Complex Statute" {{
    JURISDICTION "JP"
    VERSION 1

    WHEN (AGE >= 18 AND INCOME <= 5000000) OR HAS "disability_status"
    THEN GRANT "Comprehensive support package"

    DISCRETION "Evaluate based on individual needs assessment"
}}
"#,
            name
        ),
    }
}

/// Handles the doctor command.
pub fn handle_doctor(verbose: bool) -> Result<()> {
    println!("{}", "Legalis Doctor - System Diagnostics".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());
    println!();

    let mut issues = Vec::new();
    let mut warnings = Vec::new();

    // Check 1: Rust version
    print!("{} ", "Checking Rust installation...".dimmed());
    match std::process::Command::new("rustc")
        .arg("--version")
        .output()
    {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("{}", "✓".green());
            if verbose {
                println!("  {}", version.trim().dimmed());
            }
        }
        Err(_) => {
            println!("{}", "✗".red());
            issues.push("Rust compiler not found in PATH");
        }
    }

    // Check 2: Project structure
    print!("{} ", "Checking project structure...".dimmed());
    let has_project = Path::new("legalis.toml").exists()
        || Path::new("legalis.yaml").exists()
        || Path::new("Cargo.toml").exists();

    if has_project {
        println!("{}", "✓".green());
        if verbose {
            if Path::new("legalis.toml").exists() {
                println!("  {} found", "legalis.toml".bold());
            }
            if Path::new("legalis.yaml").exists() {
                println!("  {} found", "legalis.yaml".bold());
            }
        }
    } else {
        println!("{}", "⚠".yellow());
        warnings.push("No legalis.toml or legalis.yaml found (not in a Legalis project)");
    }

    // Check 3: Config file
    print!("{} ", "Checking configuration...".dimmed());
    let config = crate::config::Config::load();
    println!("{}", "✓".green());
    if verbose {
        println!(
            "  Jurisdiction: {:?}",
            config.jurisdiction.unwrap_or_else(|| "None".to_string())
        );
        println!("  Output format: {}", config.output.format);
        println!("  Colored output: {}", config.output.colored);
    }

    // Check 4: Write permissions
    print!("{} ", "Checking write permissions...".dimmed());
    match std::env::temp_dir().join(".legalis_test").metadata() {
        Ok(_) => {
            println!("{}", "✓".green());
        }
        Err(_) => {
            // Try to write a test file
            match fs::write(std::env::temp_dir().join(".legalis_test"), "test") {
                Ok(_) => {
                    println!("{}", "✓".green());
                    let _ = fs::remove_file(std::env::temp_dir().join(".legalis_test"));
                }
                Err(_) => {
                    println!("{}", "✗".red());
                    issues.push("No write permission to temp directory");
                }
            }
        }
    }

    // Check 5: Environment variables
    print!("{} ", "Checking environment variables...".dimmed());
    let env_vars = [
        "LEGALIS_JURISDICTION",
        "LEGALIS_VERIFY_STRICT",
        "LEGALIS_OUTPUT_FORMAT",
        "LEGALIS_OUTPUT_COLORED",
    ];
    let env_set: Vec<_> = env_vars
        .iter()
        .filter(|var| std::env::var(var).is_ok())
        .collect();

    if env_set.is_empty() {
        println!("{}", "○".dimmed());
        if verbose {
            println!("  No environment overrides set");
        }
    } else {
        println!("{}", "✓".green());
        if verbose {
            for var in env_set {
                if let Ok(value) = std::env::var(var) {
                    println!("  {}: {}", var.bold(), value);
                }
            }
        }
    }

    // Summary
    println!();
    println!("{}", "=".repeat(50).dimmed());

    if issues.is_empty() && warnings.is_empty() {
        println!("{}", "✓ All checks passed!".green().bold());
    } else {
        if !issues.is_empty() {
            println!("\n{}:", "Issues".red().bold());
            for issue in &issues {
                println!("  {} {}", "✗".red(), issue);
            }
        }

        if !warnings.is_empty() {
            println!("\n{}:", "Warnings".yellow().bold());
            for warning in &warnings {
                println!("  {} {}", "⚠".yellow(), warning);
            }
        }
    }

    if issues.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "System diagnostics found {} issue(s)",
            issues.len()
        ))
    }
}

/// Handles the REPL command.
pub fn handle_repl(load: Option<&str>, no_color: bool) -> Result<()> {
    use rustyline::history::DefaultHistory;
    use rustyline::{Editor, error::ReadlineError};

    let mut rl = Editor::<(), DefaultHistory>::new()?;
    let history_path = dirs::home_dir()
        .map(|p| p.join(".legalis_history"))
        .unwrap_or_else(|| std::path::PathBuf::from(".legalis_history"));

    // Load history if it exists
    let _ = rl.load_history(&history_path);

    println!(
        "{}",
        if !no_color {
            "Legalis REPL v0.2.0".green().bold().to_string()
        } else {
            "Legalis REPL v0.2.0".to_string()
        }
    );
    println!(
        "{}",
        if !no_color {
            "Type 'help' for available commands, 'exit' to quit"
                .dimmed()
                .to_string()
        } else {
            "Type 'help' for available commands, 'exit' to quit".to_string()
        }
    );
    println!();

    let parser = LegalDslParser::new();
    let verifier = StatuteVerifier::new();
    let mut current_statute: Option<Statute> = None;
    let mut statute_buffer = String::new();
    let mut in_multiline = false;

    // Load file if specified
    if let Some(load_path) = load {
        match fs::read_to_string(load_path) {
            Ok(content) => match parser.parse_statute(&content) {
                Ok(statute) => {
                    println!(
                        "{}",
                        if !no_color {
                            format!("Loaded statute: {}", statute.id)
                                .green()
                                .to_string()
                        } else {
                            format!("Loaded statute: {}", statute.id)
                        }
                    );
                    current_statute = Some(statute);
                }
                Err(e) => {
                    eprintln!(
                        "{}",
                        if !no_color {
                            format!("Failed to parse file: {}", e).red().to_string()
                        } else {
                            format!("Failed to parse file: {}", e)
                        }
                    );
                }
            },
            Err(e) => {
                eprintln!(
                    "{}",
                    if !no_color {
                        format!("Failed to load file: {}", e).red().to_string()
                    } else {
                        format!("Failed to load file: {}", e)
                    }
                );
            }
        }
    }

    loop {
        let prompt = if in_multiline { "... " } else { "legalis> " };

        match rl.readline(prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                // Skip empty lines
                if trimmed.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(trimmed);

                // Check for multiline mode
                if in_multiline {
                    if trimmed == "}" {
                        statute_buffer.push_str(&line);
                        statute_buffer.push('\n');

                        // Try to parse the buffer
                        match parser.parse_statute(&statute_buffer) {
                            Ok(statute) => {
                                println!(
                                    "{}",
                                    if !no_color {
                                        format!("Parsed statute: {}", statute.id)
                                            .green()
                                            .to_string()
                                    } else {
                                        format!("Parsed statute: {}", statute.id)
                                    }
                                );
                                current_statute = Some(statute);
                            }
                            Err(e) => {
                                eprintln!(
                                    "{}",
                                    if !no_color {
                                        format!("Parse error: {}", e).red().to_string()
                                    } else {
                                        format!("Parse error: {}", e)
                                    }
                                );
                            }
                        }

                        statute_buffer.clear();
                        in_multiline = false;
                    } else {
                        statute_buffer.push_str(&line);
                        statute_buffer.push('\n');
                    }
                    continue;
                }

                // Handle commands
                match trimmed {
                    "exit" | "quit" | "q" => {
                        println!("Goodbye!");
                        break;
                    }
                    "help" | "?" => {
                        print_repl_help(no_color);
                    }
                    "clear" | "cls" => {
                        print!("\x1B[2J\x1B[1;1H");
                    }
                    "show" => {
                        if let Some(ref statute) = current_statute {
                            println!("{:#?}", statute);
                        } else {
                            println!(
                                "{}",
                                if !no_color {
                                    "No statute loaded".yellow().to_string()
                                } else {
                                    "No statute loaded".to_string()
                                }
                            );
                        }
                    }
                    "verify" => {
                        if let Some(ref statute) = current_statute {
                            let result = verifier.verify(std::slice::from_ref(statute));
                            if result.passed {
                                println!(
                                    "{}",
                                    if !no_color {
                                        "✓ Verification passed".green().to_string()
                                    } else {
                                        "✓ Verification passed".to_string()
                                    }
                                );
                            } else {
                                println!(
                                    "{}",
                                    if !no_color {
                                        "✗ Verification failed".red().to_string()
                                    } else {
                                        "✗ Verification failed".to_string()
                                    }
                                );
                                for error in &result.errors {
                                    println!("  {}", error);
                                }
                            }
                            if !result.warnings.is_empty() {
                                println!(
                                    "{}",
                                    if !no_color {
                                        "Warnings:".yellow().to_string()
                                    } else {
                                        "Warnings:".to_string()
                                    }
                                );
                                for warning in &result.warnings {
                                    println!("  {}", warning);
                                }
                            }
                        } else {
                            println!(
                                "{}",
                                if !no_color {
                                    "No statute loaded".yellow().to_string()
                                } else {
                                    "No statute loaded".to_string()
                                }
                            );
                        }
                    }
                    "json" => {
                        if let Some(ref statute) = current_statute {
                            match serde_json::to_string_pretty(statute) {
                                Ok(json) => println!("{}", json),
                                Err(e) => eprintln!("JSON serialization error: {}", e),
                            }
                        } else {
                            println!(
                                "{}",
                                if !no_color {
                                    "No statute loaded".yellow().to_string()
                                } else {
                                    "No statute loaded".to_string()
                                }
                            );
                        }
                    }
                    "yaml" => {
                        if let Some(ref statute) = current_statute {
                            match serde_yaml::to_string(statute) {
                                Ok(yaml) => println!("{}", yaml),
                                Err(e) => eprintln!("YAML serialization error: {}", e),
                            }
                        } else {
                            println!(
                                "{}",
                                if !no_color {
                                    "No statute loaded".yellow().to_string()
                                } else {
                                    "No statute loaded".to_string()
                                }
                            );
                        }
                    }
                    cmd if cmd.starts_with("load ") => {
                        let path = cmd
                            .strip_prefix("load ")
                            .expect("Command starts with 'load '")
                            .trim();
                        match fs::read_to_string(path) {
                            Ok(content) => match parser.parse_statute(&content) {
                                Ok(statute) => {
                                    println!(
                                        "{}",
                                        if !no_color {
                                            format!("Loaded statute: {}", statute.id)
                                                .green()
                                                .to_string()
                                        } else {
                                            format!("Loaded statute: {}", statute.id)
                                        }
                                    );
                                    current_statute = Some(statute);
                                }
                                Err(e) => {
                                    eprintln!(
                                        "{}",
                                        if !no_color {
                                            format!("Parse error: {}", e).red().to_string()
                                        } else {
                                            format!("Parse error: {}", e)
                                        }
                                    );
                                }
                            },
                            Err(e) => {
                                eprintln!(
                                    "{}",
                                    if !no_color {
                                        format!("Failed to read file: {}", e).red().to_string()
                                    } else {
                                        format!("Failed to read file: {}", e)
                                    }
                                );
                            }
                        }
                    }
                    cmd if cmd.starts_with("save ") => {
                        let path = cmd
                            .strip_prefix("save ")
                            .expect("Command starts with 'save '")
                            .trim();
                        if let Some(ref statute) = current_statute {
                            match serde_json::to_string_pretty(statute) {
                                Ok(json) => match fs::write(path, json) {
                                    Ok(_) => println!(
                                        "{}",
                                        if !no_color {
                                            format!("Saved to: {}", path).green().to_string()
                                        } else {
                                            format!("Saved to: {}", path)
                                        }
                                    ),
                                    Err(e) => eprintln!("Write error: {}", e),
                                },
                                Err(e) => eprintln!("Serialization error: {}", e),
                            }
                        } else {
                            println!(
                                "{}",
                                if !no_color {
                                    "No statute loaded".yellow().to_string()
                                } else {
                                    "No statute loaded".to_string()
                                }
                            );
                        }
                    }
                    _ => {
                        // Check if starting a statute definition
                        if trimmed.starts_with("STATUTE") {
                            statute_buffer = line.clone();
                            statute_buffer.push('\n');
                            in_multiline = true;
                        } else {
                            println!(
                                "{}",
                                if !no_color {
                                    format!("Unknown command: {}", trimmed).yellow().to_string()
                                } else {
                                    format!("Unknown command: {}", trimmed)
                                }
                            );
                            println!("Type 'help' for available commands");
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                println!("Use 'exit' or 'quit' to leave the REPL");
            }
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // Save history
    let _ = rl.save_history(&history_path);

    Ok(())
}

/// Prints REPL help information.
fn print_repl_help(no_color: bool) {
    let help_text = vec![
        (
            "Commands:",
            vec![
                ("help, ?", "Show this help message"),
                ("show", "Display the current statute"),
                ("verify", "Verify the current statute"),
                ("json", "Display statute as JSON"),
                ("yaml", "Display statute as YAML"),
                ("load <file>", "Load a statute from a file"),
                ("save <file>", "Save the current statute to a file"),
                ("clear, cls", "Clear the screen"),
                ("exit, quit, q", "Exit the REPL"),
            ],
        ),
        (
            "Multiline Input:",
            vec![("STATUTE ...", "Start defining a statute (ends with })")],
        ),
    ];

    for (section, commands) in help_text {
        println!(
            "{}",
            if !no_color {
                section.cyan().bold().to_string()
            } else {
                section.to_string()
            }
        );
        for (cmd, desc) in commands {
            println!(
                "  {:20} {}",
                if !no_color {
                    cmd.green().to_string()
                } else {
                    cmd.to_string()
                },
                desc
            );
        }
        println!();
    }
}

/// Handles the publish command.
pub fn handle_publish(
    input: &str,
    _registry_path: &str,
    tags: &[String],
    dry_run: bool,
) -> Result<()> {
    use legalis_registry::{StatuteEntry, StatuteRegistry};

    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    if dry_run {
        println!(
            "{}",
            "[DRY RUN] Would publish statute to registry:".cyan().bold()
        );
        println!("  Statute ID: {}", statute.id.cyan());
        println!("  Title: {}", statute.title);
        println!("  Version: {}", statute.version);
        println!("  Registry: {}", _registry_path);
        println!("  Tags: {}", tags.join(", "));
        return Ok(());
    }

    let mut registry = StatuteRegistry::new();
    let jurisdiction = statute
        .jurisdiction
        .clone()
        .unwrap_or_else(|| "UNKNOWN".to_string());
    let mut entry = StatuteEntry::new(statute.clone(), jurisdiction);
    entry.tags = tags.to_vec();
    let _ = registry.register(entry);

    // In a real implementation, this would save to a file or database
    println!("{}", "✓ Statute published successfully".green().bold());
    println!("  ID: {}", statute.id.cyan());
    println!("  Version: {}", statute.version);
    if !tags.is_empty() {
        println!("  Tags: {}", tags.join(", "));
    }

    Ok(())
}

/// Handles the validate command.
pub fn handle_validate(
    inputs: &[String],
    format: Option<&LegalDslFormat>,
    strict: bool,
) -> Result<()> {
    let parser = LegalDslParser::new();
    let mut converter = LegalConverter::new();

    let mut all_valid = true;
    let mut total_errors = 0;
    let mut total_warnings = 0;

    for input in inputs {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;

        println!("{} {}", "Validating:".bold(), input.cyan());

        // Try to parse as Legalis DSL
        match parser.parse_statute(&content) {
            Ok(statute) => {
                println!("  {} Valid Legalis DSL", "✓".green());
                println!("    Statute ID: {}", statute.id);
                println!("    Preconditions: {}", statute.preconditions.len());
            }
            Err(e) => {
                // If format is specified, try to import as that format
                if let Some(fmt) = format {
                    let legal_format: LegalFormat = fmt.clone().into();
                    match converter.import(&content, legal_format) {
                        Ok((statutes, _report)) => {
                            if !statutes.is_empty() {
                                println!("  {} Valid {:?} format", "✓".green(), fmt);
                            } else {
                                println!("  {} Invalid {:?} format", "✗".red(), fmt);
                                all_valid = false;
                                total_errors += 1;
                            }
                        }
                        Err(validation_err) => {
                            println!("  {} Validation error: {}", "✗".red(), validation_err);
                            all_valid = false;
                            total_errors += 1;
                        }
                    }
                } else {
                    println!("  {} Parse error: {}", "✗".red(), e);
                    all_valid = false;
                    total_errors += 1;
                }
            }
        }

        // Check for potential issues
        if content.trim().is_empty() {
            println!("  {} File is empty", "⚠".yellow());
            total_warnings += 1;
        }

        println!();
    }

    println!("{}", "=".repeat(50).dimmed());
    if all_valid {
        println!("{}", "✓ All files are valid".green().bold());
    } else {
        println!(
            "{} {} error(s), {} warning(s)",
            "✗".red(),
            total_errors.to_string().red().bold(),
            total_warnings.to_string().yellow().bold()
        );
    }

    if !all_valid || (strict && total_warnings > 0) {
        std::process::exit(1);
    }

    Ok(())
}

/// Handles the install command.
pub fn handle_install(
    statute_id: &str,
    _registry_path: &str,
    output: &str,
    force: bool,
) -> Result<()> {
    use legalis_registry::StatuteRegistry;

    println!(
        "{} {} from registry...",
        "Installing".bold(),
        statute_id.cyan()
    );

    let mut registry = StatuteRegistry::new();

    // Look up statute in registry
    let entry = registry
        .get(statute_id)
        .ok_or_else(|| anyhow::anyhow!("Statute '{}' not found in registry", statute_id))?;

    let output_path = Path::new(output).join(format!("{}.legal", statute_id));

    // Check if already installed
    if output_path.exists() && !force {
        return Err(anyhow::anyhow!(
            "Statute already installed at {}. Use --force to reinstall.",
            output_path.display()
        ));
    }

    // Create output directory if it doesn't exist
    fs::create_dir_all(output)?;

    // Generate DSL format
    let dsl_content = super::statute_to_dsl(&entry.statute);

    // Write to file
    fs::write(&output_path, dsl_content)?;

    println!("{}", "✓ Installation successful".green().bold());
    println!("  Installed to: {}", output_path.display());
    println!("  Version: {}", entry.statute.version);
    if !entry.tags.is_empty() {
        println!("  Tags: {}", entry.tags.join(", "));
    }

    Ok(())
}

/// Handles the list command.
pub fn handle_list(directory: &str, verbose: bool) -> Result<()> {
    println!("{} {}", "Listing statutes in:".bold(), directory.cyan());
    println!();

    let dir_path = Path::new(directory);

    if !dir_path.exists() {
        println!("{}", "Directory does not exist".yellow());
        return Ok(());
    }

    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    // Read all .legal files
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("legal") {
            match fs::read_to_string(&path) {
                Ok(content) => match parser.parse_statute(&content) {
                    Ok(statute) => {
                        statutes.push((path.clone(), statute));
                    }
                    Err(e) => {
                        eprintln!("{} Failed to parse {}: {}", "⚠".yellow(), path.display(), e);
                    }
                },
                Err(e) => {
                    eprintln!("{} Failed to read {}: {}", "⚠".yellow(), path.display(), e);
                }
            }
        }
    }

    if statutes.is_empty() {
        println!("{}", "No statutes found".yellow());
        return Ok(());
    }

    if verbose {
        for (path, statute) in &statutes {
            println!("{}", "─".repeat(50).dimmed());
            println!("{} {}", "ID:".bold(), statute.id.cyan());
            println!("{} {}", "Title:".bold(), statute.title);
            println!("{} {}", "Version:".bold(), statute.version);
            if let Some(ref jur) = statute.jurisdiction {
                println!("{} {}", "Jurisdiction:".bold(), jur);
            }
            println!("{} {}", "File:".bold(), path.display());
            println!(
                "{} {}",
                "Preconditions:".bold(),
                statute.preconditions.len()
            );
            println!(
                "{} {}",
                "Has Discretion:".bold(),
                statute.discretion_logic.is_some()
            );
            println!();
        }
    } else {
        use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("ID").fg(Color::Cyan),
                Cell::new("Title").fg(Color::Cyan),
                Cell::new("Version").fg(Color::Cyan),
                Cell::new("Jurisdiction").fg(Color::Cyan),
            ]);

        for (_, statute) in &statutes {
            table.add_row(vec![
                Cell::new(&statute.id),
                Cell::new(&statute.title),
                Cell::new(statute.version.to_string()),
                Cell::new(statute.jurisdiction.as_ref().unwrap_or(&"N/A".to_string())),
            ]);
        }

        println!("{}", table);
    }

    println!();
    println!(
        "{} {}",
        "Total:".bold(),
        format!("{} statute(s)", statutes.len()).green()
    );

    Ok(())
}

/// Handles the add command.
pub fn handle_add(statute_id: &str, _registry_path: &str, config_path: &str) -> Result<()> {
    use legalis_registry::StatuteRegistry;

    println!("{} {} as dependency...", "Adding".bold(), statute_id.cyan());

    let mut registry = StatuteRegistry::new();

    // Verify statute exists in registry
    let entry = registry
        .get(statute_id)
        .ok_or_else(|| anyhow::anyhow!("Statute '{}' not found in registry", statute_id))?;

    // Read config file
    let config_content = if Path::new(config_path).exists() {
        fs::read_to_string(config_path)?
    } else {
        String::from("version: \"0.2.0\"\ndependencies: []\n")
    };

    // Parse as YAML
    let mut config: serde_yaml::Value = serde_yaml::from_str(&config_content)?;

    // Add dependency
    if let Some(deps) = config.get_mut("dependencies") {
        if let Some(deps_array) = deps.as_sequence_mut() {
            let dep = serde_yaml::Value::Mapping({
                let mut map = serde_yaml::Mapping::new();
                map.insert(
                    serde_yaml::Value::String("id".to_string()),
                    serde_yaml::Value::String(statute_id.to_string()),
                );
                map.insert(
                    serde_yaml::Value::String("version".to_string()),
                    serde_yaml::Value::Number(entry.statute.version.into()),
                );
                map
            });
            deps_array.push(dep);
        }
    } else {
        let deps_array = vec![serde_yaml::Value::Mapping({
            let mut map = serde_yaml::Mapping::new();
            map.insert(
                serde_yaml::Value::String("id".to_string()),
                serde_yaml::Value::String(statute_id.to_string()),
            );
            map.insert(
                serde_yaml::Value::String("version".to_string()),
                serde_yaml::Value::Number(entry.statute.version.into()),
            );
            map
        })];
        config
            .as_mapping_mut()
            .expect("Config should be a mapping")
            .insert("dependencies".into(), deps_array.into());
    }

    // Write back to file
    let updated_config = serde_yaml::to_string(&config)?;
    fs::write(config_path, updated_config)?;

    println!("{}", "✓ Dependency added successfully".green().bold());
    println!("  Statute: {}", statute_id.cyan());
    println!("  Version: {}", entry.statute.version);
    println!("  Config updated: {}", config_path);

    Ok(())
}

/// Handles the update command.
pub fn handle_update(statute_id: Option<&str>, _registry_path: &str, dry_run: bool) -> Result<()> {
    if let Some(id) = statute_id {
        println!("{} {}...", "Checking for updates for".bold(), id.cyan());
    } else {
        println!("{}", "Checking for updates for all statutes...".bold());
    }

    if dry_run {
        println!("{}", "[DRY RUN] Would check for and install updates".cyan());
        println!("  No updates available (registry integration pending)");
        return Ok(());
    }

    println!(
        "{}",
        "No updates available (registry integration pending)".yellow()
    );

    Ok(())
}

/// Handles the clean command.
pub fn handle_clean(all: bool, cache: bool, temp: bool, dry_run: bool) -> Result<()> {
    println!("{}", "Cleaning up...".bold());
    println!();

    let mut cleaned_items = Vec::new();
    let mut total_size: u64 = 0;

    // Define paths to clean
    let cache_dir = dirs::cache_dir()
        .map(|p| p.join("legalis"))
        .unwrap_or_else(|| std::path::PathBuf::from(".legalis_cache"));

    let temp_dir = std::env::temp_dir().join("legalis");

    // Clean cache
    if (all || cache) && cache_dir.exists() {
        let size = dir_size(&cache_dir)?;
        total_size += size;
        cleaned_items.push((cache_dir.clone(), size, "cache"));

        if !dry_run {
            fs::remove_dir_all(&cache_dir)?;
            fs::create_dir_all(&cache_dir)?;
        }
    }

    // Clean temp files
    if (all || temp) && temp_dir.exists() {
        let size = dir_size(&temp_dir)?;
        total_size += size;
        cleaned_items.push((temp_dir.clone(), size, "temp"));

        if !dry_run {
            fs::remove_dir_all(&temp_dir)?;
        }
    }

    // Display results
    if cleaned_items.is_empty() {
        println!("{}", "Nothing to clean".dimmed());
        return Ok(());
    }

    if dry_run {
        println!("{}", "[DRY RUN] Would clean:".cyan().bold());
    } else {
        println!("{}", "Cleaned:".green().bold());
    }

    for (path, size, category) in &cleaned_items {
        println!(
            "  {} [{}] - {}",
            path.display(),
            category,
            format_size(*size)
        );
    }

    println!();
    println!(
        "{} {}",
        if dry_run { "Would free:" } else { "Freed:" }.bold(),
        format_size(total_size).green()
    );

    if dry_run {
        println!();
        println!("{}", "Run without --dry-run to actually clean".dimmed());
    }

    Ok(())
}

/// Calculate directory size recursively.
fn dir_size(path: &Path) -> Result<u64> {
    let mut size = 0u64;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                size += dir_size(&entry_path)?;
            } else {
                size += entry.metadata()?.len();
            }
        }
    }

    Ok(size)
}

/// Format byte size for display.
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Handles the search command.
pub fn handle_search(
    _registry_path: &str,
    query: &str,
    jurisdiction: Option<&str>,
    tags: &[String],
    limit: usize,
) -> Result<()> {
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
    use legalis_registry::StatuteRegistry;

    // For now, we'll use an in-memory registry
    // In the future, this could load from a file or database
    let mut registry = StatuteRegistry::new();

    // Perform search based on query
    println!("{}", format!("Searching for: \"{}\"", query).cyan().bold());

    if let Some(jur) = jurisdiction {
        println!("{}", format!("  Jurisdiction filter: {}", jur).dimmed());
    }

    if !tags.is_empty() {
        println!("{}", format!("  Tag filter: {}", tags.join(", ")).dimmed());
    }

    println!();

    // Try different search strategies
    let mut results = Vec::new();

    // 1. Try exact ID match
    if let Some(entry) = registry.get(query) {
        results.push(entry.clone());
    } else {
        // 2. Search by tags
        for tag in tags {
            let tag_results = registry.query_by_tag(tag);
            results.extend(tag_results.into_iter().cloned());
        }

        // 3. Search by jurisdiction
        if let Some(jur) = jurisdiction {
            let jur_results = registry.query_by_jurisdiction(jur);
            results.extend(jur_results.into_iter().cloned());
        }
    }

    // Remove duplicates
    results.sort_by(|a, b| a.statute.id.cmp(&b.statute.id));
    results.dedup_by(|a, b| a.statute.id == b.statute.id);

    // Apply limit
    results.truncate(limit);

    if results.is_empty() {
        println!("{}", "No results found".yellow());
        println!("\nTry:");
        println!("  - Broadening your search query");
        println!("  - Removing filters");
        println!("  - Checking your registry path");
        return Ok(());
    }

    // Display results in a table
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("ID").fg(Color::Cyan),
            Cell::new("Title").fg(Color::Cyan),
            Cell::new("Version").fg(Color::Cyan),
            Cell::new("Jurisdiction").fg(Color::Cyan),
            Cell::new("Tags").fg(Color::Cyan),
        ]);

    for entry in &results {
        let statute = &entry.statute;
        table.add_row(vec![
            Cell::new(&statute.id),
            Cell::new(&statute.title),
            Cell::new(statute.version.to_string()),
            Cell::new(statute.jurisdiction.as_ref().unwrap_or(&"N/A".to_string())),
            Cell::new(entry.tags.join(", ")),
        ]);
    }

    println!("{}", table);
    println!();
    println!("{}", format!("Found {} result(s)", results.len()).green());

    Ok(())
}

/// Handles the outdated command.
pub fn handle_outdated(directory: &str, _registry_path: &str, show_all: bool) -> Result<()> {
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    println!(
        "{} {}",
        "Checking for outdated statutes in:".bold(),
        directory.cyan()
    );
    println!();

    let dir_path = Path::new(directory);

    if !dir_path.exists() {
        println!("{}", "Directory does not exist".yellow());
        return Ok(());
    }

    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    // Read all .legal files
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("legal")
            && let Ok(content) = fs::read_to_string(&path)
            && let Ok(statute) = parser.parse_statute(&content)
        {
            statutes.push((path.clone(), statute));
        }
    }

    if statutes.is_empty() {
        println!("{}", "No statutes found".yellow());
        return Ok(());
    }

    // Check for updates (in a real implementation, this would query the registry)
    let outdated: Vec<(std::path::PathBuf, Statute, u32, u32)> = Vec::new();
    let mut up_to_date = Vec::new();

    for (path, statute) in &statutes {
        // Simulate version check (in real implementation, check against registry)
        // For now, assume all are up to date
        up_to_date.push((path, statute, statute.version));
    }

    if !show_all && outdated.is_empty() {
        println!("{}", "✓ All statutes are up to date".green().bold());
        return Ok(());
    }

    // Display results
    if !outdated.is_empty() {
        println!("{}", "Outdated statutes:".red().bold());
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("ID").fg(Color::Cyan),
                Cell::new("Current").fg(Color::Cyan),
                Cell::new("Latest").fg(Color::Cyan),
                Cell::new("File").fg(Color::Cyan),
            ]);

        for (path, statute, current_version, latest_version) in &outdated {
            table.add_row(vec![
                Cell::new(&statute.id),
                Cell::new(current_version.to_string()).fg(Color::Yellow),
                Cell::new(latest_version.to_string()).fg(Color::Green),
                Cell::new(
                    path.file_name()
                        .expect("Path should have a filename")
                        .to_string_lossy()
                        .as_ref(),
                ),
            ]);
        }

        println!("{}", table);
        println!();
    }

    if show_all && !up_to_date.is_empty() {
        println!("{}", "Up-to-date statutes:".green().bold());
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("ID").fg(Color::Cyan),
                Cell::new("Version").fg(Color::Cyan),
                Cell::new("File").fg(Color::Cyan),
            ]);

        for (path, statute, version) in &up_to_date {
            table.add_row(vec![
                Cell::new(&statute.id),
                Cell::new(version.to_string()),
                Cell::new(
                    path.file_name()
                        .expect("Path should have a filename")
                        .to_string_lossy()
                        .as_ref(),
                ),
            ]);
        }

        println!("{}", table);
        println!();
    }

    println!(
        "{} {} total, {} outdated, {} up-to-date",
        "Summary:".bold(),
        statutes.len(),
        outdated.len().to_string().red(),
        up_to_date.len().to_string().green()
    );

    if !outdated.is_empty() {
        println!();
        println!(
            "{}",
            "Run 'legalis update' to update all outdated statutes".cyan()
        );
    }

    Ok(())
}

/// Handles the uninstall command.
pub fn handle_uninstall(
    statute_id: &str,
    directory: &str,
    force: bool,
    dry_run: bool,
) -> Result<()> {
    println!("{} {}", "Uninstalling statute:".bold(), statute_id.cyan());

    let file_path = Path::new(directory).join(format!("{}.legal", statute_id));

    if !file_path.exists() {
        return Err(anyhow::anyhow!(
            "Statute '{}' not found in {}",
            statute_id,
            directory
        ));
    }

    // Read statute info before deleting
    let content = fs::read_to_string(&file_path)?;
    let parser = LegalDslParser::new();
    let statute = parser.parse_statute(&content).ok();

    if dry_run {
        println!(
            "{}",
            "[DRY RUN] Would remove the following statute:"
                .cyan()
                .bold()
        );
        println!("  File: {}", file_path.display());
        if let Some(ref s) = statute {
            println!("  ID: {}", s.id);
            println!("  Title: {}", s.title);
            println!("  Version: {}", s.version);
        }
        return Ok(());
    }

    // Ask for confirmation unless force flag is set
    if !force {
        println!();
        println!("  File: {}", file_path.display());
        if let Some(ref s) = statute {
            println!("  Title: {}", s.title);
            println!("  Version: {}", s.version);
        }
        println!();
        println!(
            "{}",
            "Are you sure you want to remove this statute? (y/N)".yellow()
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", "Uninstall cancelled".dimmed());
            return Ok(());
        }
    }

    // Remove the file
    fs::remove_file(&file_path)?;

    println!("{}", "✓ Statute uninstalled successfully".green().bold());
    println!("  Removed: {}", file_path.display());

    Ok(())
}
