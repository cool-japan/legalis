//! Core CLI command handlers (parse, verify, viz, export, diff, simulate, etc.).

use crate::{
    DiffFormat, ExportFormat, FormatStyle, ImportOutputFormat, LegalDslFormat, OutputFormat,
    PortFormat, RdfOutputFormat, VizFormat, WatchCommand,
};
use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use legalis_core::Statute;
use legalis_dsl::LegalDslParser;
use legalis_i18n::JurisdictionRegistry;
use legalis_interop::{LegalConverter, LegalFormat};
use legalis_porting::{CompatibilityReport, PortedStatute, PortingEngine, PortingOptions};
use legalis_verifier::StatuteVerifier;
use legalis_viz::DecisionTree;
use std::fs;
use std::path::Path;

pub fn handle_parse(input: &str, output: Option<&str>, format: &OutputFormat) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let output_str = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&statute)?,
        OutputFormat::Yaml => serde_yaml::to_string(&statute)?,
        OutputFormat::Toml => toml::to_string_pretty(&statute)?,
        OutputFormat::Text | OutputFormat::Table | OutputFormat::Csv | OutputFormat::Html => {
            format!("{:#?}", statute)
        }
    };

    if let Some(out_path) = output {
        fs::write(out_path, output_str)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("Output written to: {}", out_path);
    } else {
        println!("{}", output_str);
    }

    Ok(())
}

/// Handles the verify command.
pub fn handle_verify(inputs: &[String], strict: bool, format: &OutputFormat) -> Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};

    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    // Create progress bar for parsing if we have multiple files
    let pb = if inputs.len() > 1 {
        let pb = ProgressBar::new(inputs.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .expect("Failed to create progress bar template")
                .progress_chars("=>-"),
        );
        Some(pb)
    } else {
        None
    };

    for input in inputs {
        if let Some(ref pb) = pb {
            pb.set_message(format!("Parsing {}", input));
        }

        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;

        let statute = parser
            .parse_statute(&content)
            .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", input, e))?;

        statutes.push(statute);

        if let Some(ref pb) = pb {
            pb.inc(1);
        }
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Parsing complete");
    }

    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);

    match format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "passed": result.passed,
                    "errors": result.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                    "warnings": result.warnings,
                    "suggestions": result.suggestions
                }))?
            );
        }
        OutputFormat::Toml => {
            println!(
                "{}",
                toml::to_string_pretty(&toml::Value::Table({
                    let mut map = toml::map::Map::new();
                    map.insert("passed".to_string(), toml::Value::Boolean(result.passed));
                    map.insert(
                        "errors".to_string(),
                        toml::Value::Array(
                            result
                                .errors
                                .iter()
                                .map(|e| toml::Value::String(e.to_string()))
                                .collect(),
                        ),
                    );
                    map.insert(
                        "warnings".to_string(),
                        toml::Value::Array(
                            result
                                .warnings
                                .iter()
                                .map(|w| toml::Value::String(w.clone()))
                                .collect(),
                        ),
                    );
                    map.insert(
                        "suggestions".to_string(),
                        toml::Value::Array(
                            result
                                .suggestions
                                .iter()
                                .map(|s| toml::Value::String(s.clone()))
                                .collect(),
                        ),
                    );
                    map
                }))?
            );
        }
        OutputFormat::Table => {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_header(vec![
                    Cell::new("Type").fg(Color::Cyan),
                    Cell::new("Status").fg(Color::Cyan),
                    Cell::new("Message").fg(Color::Cyan),
                ]);

            // Add verification status row
            let status_cell = if result.passed {
                Cell::new("✓ Passed").fg(Color::Green)
            } else {
                Cell::new("✗ Failed").fg(Color::Red)
            };
            table.add_row(vec![
                Cell::new("Verification"),
                status_cell,
                Cell::new(format!("{} statutes verified", statutes.len())),
            ]);

            // Add errors
            for error in &result.errors {
                table.add_row(vec![
                    Cell::new("Error").fg(Color::Red),
                    Cell::new("✗").fg(Color::Red),
                    Cell::new(error.to_string()),
                ]);
            }

            // Add warnings
            for warning in &result.warnings {
                table.add_row(vec![
                    Cell::new("Warning").fg(Color::Yellow),
                    Cell::new("⚠").fg(Color::Yellow),
                    Cell::new(warning),
                ]);
            }

            // Add suggestions
            for suggestion in &result.suggestions {
                table.add_row(vec![
                    Cell::new("Suggestion").fg(Color::Cyan),
                    Cell::new("→").fg(Color::Cyan),
                    Cell::new(suggestion),
                ]);
            }

            println!("{}", table);
        }
        OutputFormat::Csv => {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            wtr.write_record(["Type", "Status", "Message"])?;

            // Write verification status
            wtr.write_record([
                "Verification",
                if result.passed { "Passed" } else { "Failed" },
                &format!("{} statutes verified", statutes.len()),
            ])?;

            // Write errors
            for error in &result.errors {
                wtr.write_record(["Error", "Failed", &error.to_string()])?;
            }

            // Write warnings
            for warning in &result.warnings {
                wtr.write_record(["Warning", "Warning", warning])?;
            }

            // Write suggestions
            for suggestion in &result.suggestions {
                wtr.write_record(["Suggestion", "Suggestion", suggestion])?;
            }

            wtr.flush()?;
        }
        OutputFormat::Html => {
            println!("<!DOCTYPE html>");
            println!("<html><head>");
            println!("<meta charset=\"UTF-8\">");
            println!("<title>Verification Results</title>");
            println!("<style>");
            println!(
                "  body {{ font-family: Arial, sans-serif; margin: 20px; background-color: #f5f5f5; }}"
            );
            println!("  h1 {{ color: #333; }}");
            println!(
                "  table {{ width: 100%; border-collapse: collapse; background-color: white; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}"
            );
            println!(
                "  th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}"
            );
            println!("  th {{ background-color: #4CAF50; color: white; font-weight: bold; }}");
            println!("  tr:hover {{ background-color: #f5f5f5; }}");
            println!("  .status-pass {{ color: #4CAF50; font-weight: bold; }}");
            println!("  .status-fail {{ color: #f44336; font-weight: bold; }}");
            println!("  .type-error {{ color: #f44336; }}");
            println!("  .type-warning {{ color: #ff9800; }}");
            println!("  .type-suggestion {{ color: #2196F3; }}");
            println!("</style>");
            println!("</head><body>");

            println!("<h1>Verification Results</h1>");
            println!("<table>");
            println!("  <tr><th>Type</th><th>Status</th><th>Message</th></tr>");

            // Verification status row
            let status_class = if result.passed {
                "status-pass"
            } else {
                "status-fail"
            };
            let status_text = if result.passed {
                "✓ Passed"
            } else {
                "✗ Failed"
            };
            println!("  <tr>");
            println!("    <td>Verification</td>");
            println!("    <td class=\"{}\">", status_class);
            println!("      {}", status_text);
            println!("    </td>");
            println!("    <td>{} statutes verified</td>", statutes.len());
            println!("  </tr>");

            // Errors
            for error in &result.errors {
                println!("  <tr>");
                println!("    <td class=\"type-error\">Error</td>");
                println!("    <td class=\"status-fail\">✗ Failed</td>");
                println!("    <td>{}</td>", error);
                println!("  </tr>");
            }

            // Warnings
            for warning in &result.warnings {
                println!("  <tr>");
                println!("    <td class=\"type-warning\">Warning</td>");
                println!("    <td>⚠ Warning</td>");
                println!("    <td>{}</td>", warning);
                println!("  </tr>");
            }

            // Suggestions
            for suggestion in &result.suggestions {
                println!("  <tr>");
                println!("    <td class=\"type-suggestion\">Suggestion</td>");
                println!("    <td>→ Suggestion</td>");
                println!("    <td>{}</td>", suggestion);
                println!("  </tr>");
            }

            println!("</table>");
            println!("</body></html>");
        }
        OutputFormat::Yaml | OutputFormat::Text => {
            if result.passed {
                println!("{}", "✓ Verification passed".green().bold());
            } else {
                println!("{}", "✗ Verification failed".red().bold());
            }

            if !result.errors.is_empty() {
                println!("\n{}:", "Errors".red().bold());
                for error in &result.errors {
                    println!("  {} {}", "✗".red(), error);
                }
            }

            if !result.warnings.is_empty() {
                println!("\n{}:", "Warnings".yellow().bold());
                for warning in &result.warnings {
                    println!("  {} {}", "⚠".yellow(), warning);
                }
            }

            if !result.suggestions.is_empty() {
                println!("\n{}:", "Suggestions".cyan().bold());
                for suggestion in &result.suggestions {
                    println!("  {} {}", "→".cyan(), suggestion);
                }
            }
        }
    }

    if !result.passed || (strict && !result.warnings.is_empty()) {
        std::process::exit(1);
    }

    Ok(())
}

/// Handles the viz command.
pub fn handle_viz(input: &str, output: &str, viz_format: &VizFormat) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let tree = DecisionTree::from_statute(&statute)
        .map_err(|e| anyhow::anyhow!("Visualization error: {}", e))?;

    let output_str = match viz_format {
        VizFormat::Dot => tree.to_dot(),
        VizFormat::Mermaid => tree.to_mermaid(),
        VizFormat::Ascii => tree.to_ascii(),
        VizFormat::Box => tree.to_box(),
    };

    fs::write(output, &output_str)
        .with_context(|| format!("Failed to write output file: {}", output))?;

    println!("Visualization written to: {}", output);
    println!(
        "Nodes: {}, Discretionary: {}",
        tree.node_count(),
        tree.discretionary_count()
    );

    Ok(())
}

/// Handles the export command.
pub fn handle_export(input: &str, output: &str, export_format: &ExportFormat) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let output_str = match export_format {
        ExportFormat::Json => serde_json::to_string_pretty(&statute)?,
        ExportFormat::Yaml => serde_yaml::to_string(&statute)?,
        ExportFormat::Solidity => {
            let generator =
                legalis_chain::ContractGenerator::new(legalis_chain::TargetPlatform::Solidity);
            let contract = generator
                .generate(&statute)
                .map_err(|e| anyhow::anyhow!("Export error: {}", e))?;
            contract.source
        }
    };

    fs::write(output, output_str)
        .with_context(|| format!("Failed to write output file: {}", output))?;

    println!("Exported to: {}", output);

    Ok(())
}

/// Handles the init command.
pub fn handle_init(path: &str, dry_run: bool) -> Result<()> {
    let project_path = Path::new(path);

    // Create sample statute content
    let sample_statute = r#"STATUTE sample-adult-rights: "Sample Adult Rights Act" {
    WHEN AGE >= 18
    THEN GRANT "Full legal capacity"
    DISCRETION "Consider individual maturity in exceptional cases"
}
"#;

    // Create config file content
    let config = r#"# Legalis Project Configuration
version: "0.2.0"

# Default jurisdiction
jurisdiction: "JP"

# Verification settings
verification:
  strict: false
  constitutional_checks: true

# Output settings
output:
  format: "json"
  directory: "./output"
"#;

    if dry_run {
        println!(
            "{}",
            "[DRY RUN] Would initialize Legalis project at:"
                .cyan()
                .bold()
        );
        println!("  {}", path);
        println!("\n{}", "Would create:".cyan().bold());
        println!("  - statutes/ {}", "(directory)".dimmed());
        println!("  - output/ {}", "(directory)".dimmed());
        println!("  - statutes/sample.legal");
        println!("  - legalis.yaml");
        println!("\n{}", "Sample statute content:".cyan());
        println!("{}", sample_statute.dimmed());
        println!("{}", "Config file content:".cyan());
        println!("{}", config.dimmed());
        return Ok(());
    }

    // Create directories
    fs::create_dir_all(project_path.join("statutes"))?;
    fs::create_dir_all(project_path.join("output"))?;

    fs::write(project_path.join("statutes/sample.legal"), sample_statute)?;
    fs::write(project_path.join("legalis.yaml"), config)?;

    println!(
        "{}",
        format!("✓ Initialized Legalis project at: {}", path)
            .green()
            .bold()
    );
    println!("  {}:", "Created".green());
    println!("    - statutes/sample.legal");
    println!("    - legalis.yaml");
    println!(
        "\n{}",
        "Run 'legalis verify -i statutes/sample.legal' to verify the sample statute."
            .to_string()
            .cyan()
    );

    Ok(())
}

/// Parses multiple statute files.
/// Handles the diff command.
pub fn handle_diff(old_path: &str, new_path: &str, format: &DiffFormat) -> Result<()> {
    let parser = LegalDslParser::new();

    let old_content = fs::read_to_string(old_path)
        .with_context(|| format!("Failed to read old file: {}", old_path))?;
    let new_content = fs::read_to_string(new_path)
        .with_context(|| format!("Failed to read new file: {}", new_path))?;

    let old_statute = parser
        .parse_statute(&old_content)
        .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", old_path, e))?;
    let new_statute = parser
        .parse_statute(&new_content)
        .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", new_path, e))?;

    let diff = legalis_diff::diff(&old_statute, &new_statute)
        .map_err(|e| anyhow::anyhow!("Diff error: {}", e))?;

    match format {
        DiffFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&diff)?);
        }
        DiffFormat::Markdown => {
            println!("# Statute Diff: {}\n", diff.statute_id);
            println!("**Severity:** {:?}\n", diff.impact.severity);
            println!("## Changes\n");
            for change in &diff.changes {
                println!(
                    "- **{:?}** {}: {}",
                    change.change_type, change.target, change.description
                );
                if let Some(ref old) = change.old_value {
                    println!("  - Old: `{}`", old);
                }
                if let Some(ref new) = change.new_value {
                    println!("  - New: `{}`", new);
                }
            }
            if !diff.impact.notes.is_empty() {
                println!("\n## Impact Notes\n");
                for note in &diff.impact.notes {
                    println!("- {}", note);
                }
            }
        }
        DiffFormat::Text => {
            use colored::Colorize;

            // Print header
            println!("{} {}", "Statute Diff:".bold(), diff.statute_id.cyan());

            // Print severity with color coding
            let severity_str = format!("{:?}", diff.impact.severity);
            let colored_severity = match diff.impact.severity {
                legalis_diff::Severity::None => severity_str.dimmed(),
                legalis_diff::Severity::Minor => severity_str.green(),
                legalis_diff::Severity::Moderate => severity_str.yellow(),
                legalis_diff::Severity::Major => severity_str.red(),
                legalis_diff::Severity::Breaking => severity_str.red().bold(),
            };
            println!("{} {}", "Severity:".bold(), colored_severity);

            // Print changes
            if !diff.changes.is_empty() {
                println!("\n{}:", "Changes".bold().underline());
                for change in &diff.changes {
                    // Color code change type
                    let change_type_str = format!("{:?}", change.change_type);
                    let colored_type = match change.change_type {
                        legalis_diff::ChangeType::Added => change_type_str.green(),
                        legalis_diff::ChangeType::Removed => change_type_str.red(),
                        legalis_diff::ChangeType::Modified => change_type_str.yellow(),
                        legalis_diff::ChangeType::Reordered => change_type_str.blue(),
                    };

                    println!(
                        "  {} {}: {}",
                        colored_type.bold(),
                        format!("{:?}", change.target).cyan(),
                        change.description
                    );

                    if let Some(ref old) = change.old_value {
                        println!("    {} {}", "−".red(), old.red());
                    }
                    if let Some(ref new) = change.new_value {
                        println!("    {} {}", "+".green(), new.green());
                    }
                }
            }

            // Print impact notes
            if !diff.impact.notes.is_empty() {
                println!("\n{}:", "Impact Notes".bold().underline());
                for note in &diff.impact.notes {
                    println!("  {} {}", "•".cyan(), note);
                }
            }

            // Print summary statistics
            println!("\n{}:", "Summary".bold().underline());
            let added = diff
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, legalis_diff::ChangeType::Added))
                .count();
            let removed = diff
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, legalis_diff::ChangeType::Removed))
                .count();
            let modified = diff
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, legalis_diff::ChangeType::Modified))
                .count();
            let reordered = diff
                .changes
                .iter()
                .filter(|c| matches!(c.change_type, legalis_diff::ChangeType::Reordered))
                .count();

            if added > 0 {
                println!("  {} {} additions", "+".green(), added.to_string().green());
            }
            if removed > 0 {
                println!("  {} {} deletions", "−".red(), removed.to_string().red());
            }
            if modified > 0 {
                println!(
                    "  {} {} modifications",
                    "~".yellow(),
                    modified.to_string().yellow()
                );
            }
            if reordered > 0 {
                println!(
                    "  {} {} reorderings",
                    "↕".blue(),
                    reordered.to_string().blue()
                );
            }
        }
    }

    Ok(())
}

/// Handles the simulate command.
pub async fn handle_simulate(
    inputs: &[String],
    population_size: usize,
    output: Option<&str>,
) -> Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};

    let statutes = super::parse_statutes(inputs)?;

    println!("Running simulation with {} entities...", population_size);

    // Create progress bar for population generation
    let pb = ProgressBar::new(population_size as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/blue} {pos}/{len} {msg}")
            .expect("Failed to create progress bar template")
            .progress_chars("##-"),
    );
    pb.set_message("Generating population...");

    let population = legalis_sim::PopulationBuilder::new()
        .generate_random(population_size)
        .build();

    pb.finish_with_message("Population generated");

    // Create spinner for simulation
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .expect("Failed to create progress spinner template"),
    );
    spinner.set_message("Running simulation...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let engine = legalis_sim::SimEngine::new(statutes, population);
    let metrics = engine.run_simulation().await;

    spinner.finish_with_message("Simulation complete");

    let summary = metrics.summary();

    if let Some(out_path) = output {
        fs::write(out_path, &summary)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("Results written to: {}", out_path);
    }

    println!("\n{}", summary);

    Ok(())
}

/// Handles the audit command.
pub fn handle_audit(inputs: &[String], output: &str, with_complexity: bool) -> Result<()> {
    let statutes = super::parse_statutes(inputs)?;

    let mut report = String::new();
    report.push_str("# Legalis Audit Report\n\n");
    report.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    report.push_str(&format!("Statutes analyzed: {}\n\n", statutes.len()));

    // Verification
    report.push_str("## Verification Results\n\n");
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(&statutes);

    if result.passed {
        report.push_str("✓ All statutes passed verification\n\n");
    } else {
        report.push_str("✗ Verification failed\n\n");
        for error in &result.errors {
            report.push_str(&format!("- Error: {}\n", error));
        }
        report.push('\n');
    }

    if !result.warnings.is_empty() {
        report.push_str("### Warnings\n\n");
        for warning in &result.warnings {
            report.push_str(&format!("- {}\n", warning));
        }
        report.push('\n');
    }

    // Complexity analysis
    if with_complexity {
        report.push_str("## Complexity Analysis\n\n");
        report.push_str(&legalis_verifier::complexity_report(&statutes));
    }

    // Statute summary
    report.push_str("## Statute Summary\n\n");
    for statute in &statutes {
        report.push_str(&format!("### {}\n\n", statute.id));
        report.push_str(&format!("- Title: {}\n", statute.title));
        report.push_str(&format!(
            "- Preconditions: {}\n",
            statute.preconditions.len()
        ));
        report.push_str(&format!(
            "- Has Discretion: {}\n",
            statute.discretion_logic.is_some()
        ));
        report.push_str(&format!("- Version: {}\n", statute.version));
        if let Some(ref jur) = statute.jurisdiction {
            report.push_str(&format!("- Jurisdiction: {}\n", jur));
        }
        report.push('\n');
    }

    fs::write(output, &report)
        .with_context(|| format!("Failed to write output file: {}", output))?;

    println!("Audit report written to: {}", output);

    Ok(())
}

/// Handles the complexity command.
pub fn handle_complexity(inputs: &[String], output: Option<&str>) -> Result<()> {
    let statutes = super::parse_statutes(inputs)?;

    let report = legalis_verifier::complexity_report(&statutes);

    if let Some(out_path) = output {
        fs::write(out_path, &report)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("Complexity report written to: {}", out_path);
    } else {
        println!("{}", report);
    }

    Ok(())
}

/// Handles the port command.
pub fn handle_port(
    input: &str,
    target: &str,
    output: Option<&str>,
    format: &PortFormat,
) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    // Create jurisdiction registry with defaults
    let registry = JurisdictionRegistry::with_defaults();

    // Get source jurisdiction from statute or default to JP
    let source_jur_id = statute.jurisdiction.as_deref().unwrap_or("JP");
    let source_jur = registry
        .get(source_jur_id)
        .ok_or_else(|| anyhow::anyhow!("Source jurisdiction '{}' not found", source_jur_id))?
        .clone();

    // Get target jurisdiction
    let target_jur = registry
        .get(target)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Target jurisdiction '{}' not found. Available: JP, US, DE, FR",
                target
            )
        })?
        .clone();

    // Create porting engine and port the statute
    let engine = PortingEngine::new(source_jur, target_jur);
    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };
    let ported = engine
        .port_statute(&statute, &options)
        .map_err(|e| anyhow::anyhow!("Porting error: {}", e))?;

    // Generate output based on format
    let output_str = match format {
        PortFormat::Json => serde_json::to_string_pretty(&ported.statute)?,
        PortFormat::Yaml => serde_yaml::to_string(&ported.statute)?,
        PortFormat::Report => {
            let report = engine.generate_report(std::slice::from_ref(&statute));
            format_port_report(&ported, &report, target)
        }
    };

    if let Some(out_path) = output {
        fs::write(out_path, &output_str)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("Ported statute written to: {}", out_path);
    } else {
        println!("{}", output_str);
    }

    // Print summary
    if !ported.changes.is_empty() {
        println!("\nChanges made during porting:");
        for change in &ported.changes {
            println!("  - {}: {}", change.description, change.reason);
        }
    }

    Ok(())
}

/// Formats a port report for display.
fn format_port_report(
    ported: &PortedStatute,
    report: &CompatibilityReport,
    target: &str,
) -> String {
    let mut output = String::new();

    output.push_str(&format!(
        "# Porting Report: {} -> {}\n\n",
        ported.original_id, target
    ));
    output.push_str(&format!(
        "## Compatibility Score: {:.0}%\n\n",
        report.compatibility_score * 100.0
    ));

    output.push_str(&format!(
        "- Adaptations required: {}\n",
        report.adaptations_required
    ));
    output.push_str(&format!(
        "- Incompatibilities: {}\n\n",
        report.incompatibilities
    ));

    if !report.findings.is_empty() {
        output.push_str("## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!(
                "- **{:?}** [{}]: {}\n",
                finding.severity, finding.category, finding.description
            ));
            if let Some(ref statute_id) = finding.statute_id {
                output.push_str(&format!("  - Statute: {}\n", statute_id));
            }
        }
        output.push('\n');
    }

    if !report.recommendations.is_empty() {
        output.push_str("## Recommendations\n\n");
        for rec in &report.recommendations {
            output.push_str(&format!("- {}\n", rec));
        }
        output.push('\n');
    }

    if !ported.changes.is_empty() {
        output.push_str("## Changes Applied\n\n");
        for change in &ported.changes {
            output.push_str(&format!(
                "- **{:?}**: {}\n",
                change.change_type, change.description
            ));
            if let Some(ref orig) = change.original {
                output.push_str(&format!("  - Original: {}\n", orig));
            }
            if let Some(ref adapted) = change.adapted {
                output.push_str(&format!("  - Adapted: {}\n", adapted));
            }
            output.push_str(&format!("  - Reason: {}\n", change.reason));
        }
        output.push('\n');
    }

    output.push_str("## Ported Statute\n\n");
    output.push_str(&format!("{}\n", ported.statute));

    output
}

/// Handles the import command.
pub fn handle_import(
    input: &str,
    from: Option<&LegalDslFormat>,
    output: Option<&str>,
    import_output: &ImportOutputFormat,
) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let mut converter = LegalConverter::new();

    // Import statutes
    let (statutes, report) = if let Some(format) = from {
        let legal_format: LegalFormat = format.clone().into();
        converter
            .import(&content, legal_format)
            .map_err(|e| anyhow::anyhow!("Import error: {}", e))?
    } else {
        // Auto-detect format
        converter
            .auto_import(&content)
            .map_err(|e| anyhow::anyhow!("Import error (auto-detect): {}", e))?
    };

    // Generate output
    let output_str = match import_output {
        ImportOutputFormat::Json => serde_json::to_string_pretty(&statutes)?,
        ImportOutputFormat::Yaml => serde_yaml::to_string(&statutes)?,
        ImportOutputFormat::Legalis => {
            // Generate native DSL format
            statutes
                .iter()
                .map(super::statute_to_dsl)
                .collect::<Vec<_>>()
                .join("\n\n")
        }
    };

    if let Some(out_path) = output {
        fs::write(out_path, &output_str)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("Imported {} statutes to: {}", statutes.len(), out_path);
    } else {
        println!("{}", output_str);
    }

    // Print conversion report
    println!("\n--- Conversion Report ---");
    if let Some(src) = report.source_format {
        println!("Source format: {:?}", src);
    }
    println!("Statutes converted: {}", report.statutes_converted);
    println!("Confidence: {:.0}%", report.confidence * 100.0);

    if !report.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &report.warnings {
            println!("  - {}", warning);
        }
    }

    if !report.unsupported_features.is_empty() {
        println!("\nUnsupported features:");
        for feature in &report.unsupported_features {
            println!("  - {}", feature);
        }
    }

    Ok(())
}

/// Handles the convert command.
pub fn handle_convert(
    input: &str,
    from: Option<&LegalDslFormat>,
    to: &LegalDslFormat,
    output: Option<&str>,
) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let mut converter = LegalConverter::new();
    let target_format: LegalFormat = to.clone().into();

    let (output_str, report) = if let Some(source_format) = from {
        let src_format: LegalFormat = source_format.clone().into();
        converter
            .convert(&content, src_format, target_format)
            .map_err(|e| anyhow::anyhow!("Conversion error: {}", e))?
    } else {
        // Auto-detect source format and convert
        let (statutes, _import_report) = converter
            .auto_import(&content)
            .map_err(|e| anyhow::anyhow!("Auto-detect error: {}", e))?;

        converter
            .export(&statutes, target_format)
            .map_err(|e| anyhow::anyhow!("Export error: {}", e))?
    };

    if let Some(out_path) = output {
        fs::write(out_path, &output_str)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("Converted to {:?} format: {}", to, out_path);
    } else {
        println!("{}", output_str);
    }

    // Print conversion report
    println!("\n--- Conversion Report ---");
    if let Some(src) = report.source_format {
        println!("Source format: {:?}", src);
    }
    if let Some(tgt) = report.target_format {
        println!("Target format: {:?}", tgt);
    }
    println!("Statutes converted: {}", report.statutes_converted);
    println!("Confidence: {:.0}%", report.confidence * 100.0);

    if !report.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &report.warnings {
            println!("  - {}", warning);
        }
    }

    Ok(())
}

/// Converts a statute to native Legalis DSL format.
/// Handles the LOD export command.
pub fn handle_lod(
    input: &str,
    output: Option<&str>,
    format: &RdfOutputFormat,
    base_uri: &str,
) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    // Create exporter with custom base URI
    let namespaces = legalis_lod::Namespaces::with_base(base_uri);
    let rdf_format: legalis_lod::RdfFormat = format.clone().into();
    let exporter = legalis_lod::LodExporter::with_namespaces(rdf_format, namespaces);

    let output_str = exporter
        .export(&statute)
        .map_err(|e| anyhow::anyhow!("LOD export error: {}", e))?;

    if let Some(out_path) = output {
        fs::write(out_path, &output_str)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!(
            "Exported to {} format: {}",
            rdf_format.extension(),
            out_path
        );
    } else {
        println!("{}", output_str);
    }

    println!(
        "\nExported statute '{}' to {} format",
        statute.id,
        format_name(format)
    );
    println!("Base URI: {}", base_uri);
    println!("MIME type: {}", rdf_format.mime_type());

    Ok(())
}

/// Returns a human-readable name for the RDF format.
fn format_name(format: &RdfOutputFormat) -> &'static str {
    match format {
        RdfOutputFormat::Turtle => "Turtle (TTL)",
        RdfOutputFormat::NTriples => "N-Triples",
        RdfOutputFormat::RdfXml => "RDF/XML",
        RdfOutputFormat::JsonLd => "JSON-LD",
    }
}

/// Handles the format command.
pub fn handle_format(
    input: &str,
    output: Option<&str>,
    inplace: bool,
    style: &FormatStyle,
    dry_run: bool,
) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    // Create printer with specified style
    let config: legalis_dsl::PrinterConfig = style.clone().into();
    let printer = legalis_dsl::DslPrinter::with_config(config);
    let formatted = printer.format(&statute);

    if dry_run {
        if inplace {
            println!(
                "{}",
                format!("[DRY RUN] Would format file in place: {}", input)
                    .cyan()
                    .bold()
            );
        } else if let Some(out_path) = output {
            println!(
                "{}",
                format!("[DRY RUN] Would write formatted output to: {}", out_path)
                    .cyan()
                    .bold()
            );
        } else {
            println!("{}", "[DRY RUN] Would write to stdout:".cyan().bold());
        }
        println!("\n{}", "Formatted content:".cyan());
        println!("{}", formatted);
        return Ok(());
    }

    if inplace {
        fs::write(input, &formatted)
            .with_context(|| format!("Failed to write to file: {}", input))?;
        println!("{}", format!("✓ Formatted: {}", input).green());
    } else if let Some(out_path) = output {
        fs::write(out_path, &formatted)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!(
            "{}",
            format!("✓ Formatted output written to: {}", out_path).green()
        );
    } else {
        println!("{}", formatted);
    }

    Ok(())
}

/// Lint rule violations.
#[derive(Debug)]
pub struct LintViolation {
    pub file: String,
    pub line: Option<usize>,
    pub rule: String,
    pub severity: LintSeverity,
    pub message: String,
    pub suggestion: Option<String>,
}

/// Lint severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LintSeverity {
    Error,
    Warning,
    Style,
}

/// Handles the lint command.
pub fn handle_lint(inputs: &[String], fix: bool, strict: bool) -> Result<()> {
    let mut all_violations = Vec::new();
    let parser = LegalDslParser::new();

    for input in inputs {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;

        // Parse the file
        let statute = match parser.parse_statute(&content) {
            Ok(s) => s,
            Err(e) => {
                all_violations.push(LintViolation {
                    file: input.clone(),
                    line: None,
                    rule: "syntax".to_string(),
                    severity: LintSeverity::Error,
                    message: format!("Parse error: {}", e),
                    suggestion: None,
                });
                continue;
            }
        };

        // Check for style violations
        let violations = check_style(&statute, input);
        all_violations.extend(violations);
    }

    // Display violations
    if all_violations.is_empty() {
        println!("{}", "✓ No linting issues found".green().bold());
        return Ok(());
    }

    for violation in &all_violations {
        let (severity_str, color_fn): (&str, fn(&str) -> colored::ColoredString) =
            match violation.severity {
                LintSeverity::Error => ("error", |s| s.red()),
                LintSeverity::Warning => ("warning", |s| s.yellow()),
                LintSeverity::Style => ("style", |s| s.cyan()),
            };

        if let Some(line) = violation.line {
            println!(
                "{}:{}: {} [{}] {}",
                violation.file.bold(),
                line,
                color_fn(severity_str).bold(),
                violation.rule,
                violation.message
            );
        } else {
            println!(
                "{}: {} [{}] {}",
                violation.file.bold(),
                color_fn(severity_str).bold(),
                violation.rule,
                violation.message
            );
        }

        if let Some(ref suggestion) = violation.suggestion {
            println!("  {}: {}", "Suggestion".cyan(), suggestion);
        }
    }

    let error_count = all_violations
        .iter()
        .filter(|v| v.severity == LintSeverity::Error)
        .count();
    let warning_count = all_violations
        .iter()
        .filter(|v| v.severity == LintSeverity::Warning)
        .count();

    println!(
        "\nFound {} {}, {} {}",
        error_count.to_string().red().bold(),
        "error(s)".red(),
        warning_count.to_string().yellow().bold(),
        "warning(s)".yellow()
    );

    if fix {
        println!("\nNote: Auto-fix is not yet implemented");
    }

    if error_count > 0 || (strict && warning_count > 0) {
        std::process::exit(1);
    }

    Ok(())
}

/// Check statute for style violations.
fn check_style(statute: &Statute, file: &str) -> Vec<LintViolation> {
    let mut violations = Vec::new();

    // Check for empty title
    if statute.title.trim().is_empty() {
        violations.push(LintViolation {
            file: file.to_string(),
            line: None,
            rule: "empty-title".to_string(),
            severity: LintSeverity::Error,
            message: "Statute title should not be empty".to_string(),
            suggestion: Some("Add a descriptive title".to_string()),
        });
    }

    // Check for missing jurisdiction
    if statute.jurisdiction.is_none() {
        violations.push(LintViolation {
            file: file.to_string(),
            line: None,
            rule: "missing-jurisdiction".to_string(),
            severity: LintSeverity::Warning,
            message: "Statute should specify a jurisdiction".to_string(),
            suggestion: Some("Add a JURISDICTION directive".to_string()),
        });
    }

    // Check for missing preconditions
    if statute.preconditions.is_empty() {
        violations.push(LintViolation {
            file: file.to_string(),
            line: None,
            rule: "no-preconditions".to_string(),
            severity: LintSeverity::Style,
            message: "Statute has no preconditions".to_string(),
            suggestion: Some("Consider adding WHEN conditions".to_string()),
        });
    }

    // Check for vague effect descriptions
    if statute.effect.description.trim().len() < 10 {
        violations.push(LintViolation {
            file: file.to_string(),
            line: None,
            rule: "vague-effect".to_string(),
            severity: LintSeverity::Style,
            message: "Effect description is too short".to_string(),
            suggestion: Some("Provide a more detailed description".to_string()),
        });
    }

    violations
}

/// Handles the watch command.
#[allow(dead_code)]
pub async fn handle_watch(inputs: &[String], command: &WatchCommand) -> Result<()> {
    use std::time::Duration;
    use tokio::time::sleep;

    if inputs.is_empty() {
        eprintln!("Error: at least one file path is required.");
        eprintln!("Usage: legalis watch <FILE>... <COMMAND>");
        eprintln!("Run `legalis watch --help` for more information.");
        return Err(anyhow::anyhow!("No file paths provided to watch"));
    }

    println!("Watching files: {:?}", inputs);
    println!("Command: {:?}", command);

    // Simple polling-based watcher (in a real implementation, use notify crate)
    let mut last_modified = std::collections::HashMap::new();

    for input in inputs {
        if let Ok(metadata) = fs::metadata(input)
            && let Ok(modified) = metadata.modified()
        {
            last_modified.insert(input.clone(), modified);
        }
    }

    loop {
        sleep(Duration::from_secs(1)).await;

        for input in inputs {
            if let Ok(metadata) = fs::metadata(input)
                && let Ok(modified) = metadata.modified()
                && let Some(&last_mod) = last_modified.get(input)
                && modified > last_mod
            {
                println!("\n{} changed, running {:?}...", input, command);
                match command {
                    WatchCommand::Verify => {
                        let _ =
                            handle_verify(std::slice::from_ref(input), false, &OutputFormat::Text);
                    }
                    WatchCommand::Lint => {
                        let _ = handle_lint(std::slice::from_ref(input), false, false);
                    }
                    WatchCommand::Test => {
                        match std::process::Command::new("cargo")
                            .args(["test"])
                            .stdout(std::process::Stdio::piped())
                            .stderr(std::process::Stdio::piped())
                            .output()
                        {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                let combined = format!("{}{}", stdout, stderr);

                                let mut total_passed = 0usize;
                                let mut total_failed = 0usize;
                                for line in combined.lines() {
                                    if line.contains("test result:") {
                                        if let Some(p) = parse_test_count(line, "passed") {
                                            total_passed += p;
                                        }
                                        if let Some(f) = parse_test_count(line, "failed") {
                                            total_failed += f;
                                        }
                                    }
                                }

                                if output.status.success() {
                                    println!(
                                        "{} Tests: {} passed",
                                        "✓".green(),
                                        total_passed.to_string().green()
                                    );
                                } else {
                                    println!(
                                        "{} Tests: {} passed, {} failed",
                                        "✗".red(),
                                        total_passed.to_string().green(),
                                        total_failed.to_string().red()
                                    );
                                    for line in combined.lines().filter(|l| {
                                        l.contains("FAILED")
                                            || l.contains("test result:")
                                            || l.starts_with("error")
                                    }) {
                                        println!("  {}", line);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("{} Failed to run cargo test: {}", "Error:".red(), e);
                            }
                        }
                    }
                    WatchCommand::Format => {
                        let _ = handle_format(input, None, true, &FormatStyle::Default, false);
                    }
                }
                last_modified.insert(input.clone(), modified);
            }
        }
    }
}

/// Parses an integer count that precedes `keyword` in a `cargo test` output line.
///
/// For example, given `"test result: ok. 5 passed; 0 failed;"` and keyword `"passed"`,
/// this returns `Some(5)`.
fn parse_test_count(line: &str, keyword: &str) -> Option<usize> {
    let pos = line.find(keyword)?;
    // Grab all text before the keyword, trim trailing whitespace, then take
    // the last whitespace-separated token (the count immediately before the keyword).
    let before = line[..pos].trim_end();
    before.split_whitespace().last()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_test_arm_exists() {
        // Structural test: verify WatchCommand::Test is a valid variant and
        // that the match in handle_watch handles it (compilation proves this).
        let cmd = WatchCommand::Test;
        let is_test = matches!(cmd, WatchCommand::Test);
        assert!(is_test, "WatchCommand::Test should be a valid variant");
    }

    #[test]
    fn test_parse_test_count_passed() {
        let line = "test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured;";
        assert_eq!(parse_test_count(line, "passed"), Some(5));
    }

    #[test]
    fn test_parse_test_count_failed() {
        let line = "test result: FAILED. 3 passed; 2 failed; 0 ignored;";
        assert_eq!(parse_test_count(line, "failed"), Some(2));
    }

    #[test]
    fn test_parse_test_count_missing() {
        // keyword absent → None
        assert_eq!(parse_test_count("unrelated line", "passed"), None);
    }
}
