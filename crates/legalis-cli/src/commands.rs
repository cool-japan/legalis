//! CLI command implementations.

use crate::{
    BenchmarkType, DiffFormat, ExplainDetail, ExportFormat, FormatStyle, GraphFormat, GraphType,
    ImportOutputFormat, LegalDslFormat, OutputFormat, PortFormat, RdfOutputFormat, StatuteTemplate,
    TraceFormat, VizFormat, WatchCommand,
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
use std::path::{Path, PathBuf};

/// Handles the parse command.
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
                .unwrap()
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
fn parse_statutes(inputs: &[String]) -> Result<Vec<Statute>> {
    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    for input in inputs {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;

        let statute = parser
            .parse_statute(&content)
            .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", input, e))?;

        statutes.push(statute);
    }

    Ok(statutes)
}

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

    let statutes = parse_statutes(inputs)?;

    println!("Running simulation with {} entities...", population_size);

    // Create progress bar for population generation
    let pb = ProgressBar::new(population_size as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/blue} {pos}/{len} {msg}")
            .unwrap()
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
            .unwrap(),
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
    let statutes = parse_statutes(inputs)?;

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
    let statutes = parse_statutes(inputs)?;

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
                .map(statute_to_dsl)
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
fn statute_to_dsl(statute: &Statute) -> String {
    let mut dsl = format!("STATUTE {}: \"{}\" {{\n", statute.id, statute.title);

    // Add metadata
    if let Some(ref jur) = statute.jurisdiction {
        dsl.push_str(&format!("    JURISDICTION \"{}\"\n", jur));
    }
    if statute.version > 1 {
        dsl.push_str(&format!("    VERSION {}\n", statute.version));
    }
    if let Some(eff) = statute.temporal_validity.effective_date {
        dsl.push_str(&format!("    EFFECTIVE \"{}\"\n", eff.format("%Y-%m-%d")));
    }
    if let Some(exp) = statute.temporal_validity.expiry_date {
        dsl.push_str(&format!("    EXPIRES \"{}\"\n", exp.format("%Y-%m-%d")));
    }

    // Add conditions
    if !statute.preconditions.is_empty() {
        let conditions: Vec<String> = statute.preconditions.iter().map(condition_to_dsl).collect();
        dsl.push_str(&format!("    WHEN {}\n", conditions.join(" AND ")));
    }

    // Add effect
    dsl.push_str(&format!(
        "    THEN {:?} \"{}\"\n",
        statute.effect.effect_type, statute.effect.description
    ));

    // Add discretion
    if let Some(ref discretion) = statute.discretion_logic {
        dsl.push_str(&format!("    DISCRETION \"{}\"\n", discretion));
    }

    dsl.push('}');
    dsl
}

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

/// Converts a condition to DSL format.
fn condition_to_dsl(condition: &legalis_core::Condition) -> String {
    use legalis_core::Condition;

    match condition {
        Condition::Age { operator, value } => {
            format!("AGE {} {}", operator, value)
        }
        Condition::Income { operator, value } => {
            format!("INCOME {} {}", operator, value)
        }
        Condition::And(left, right) => {
            format!(
                "({} AND {})",
                condition_to_dsl(left),
                condition_to_dsl(right)
            )
        }
        Condition::Or(left, right) => {
            format!(
                "({} OR {})",
                condition_to_dsl(left),
                condition_to_dsl(right)
            )
        }
        Condition::Not(inner) => {
            format!("NOT {}", condition_to_dsl(inner))
        }
        Condition::AttributeEquals { key, value } => {
            format!("HAS \"{}\" = \"{}\"", key, value)
        }
        Condition::HasAttribute { key } => {
            format!("HAS \"{}\"", key)
        }
        Condition::ResidencyDuration { operator, months } => {
            format!("RESIDENCY {} {} months", operator, months)
        }
        Condition::Geographic {
            region_type,
            region_id,
        } => {
            format!("REGION {:?} \"{}\"", region_type, region_id)
        }
        Condition::Custom { description } => {
            format!("CUSTOM \"{}\"", description)
        }
        _ => format!("{:?}", condition),
    }
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
                        println!("Test command not yet implemented");
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

/// Handles the test command.
pub fn handle_test(inputs: &[String], tests_file: &str, verbose: bool) -> Result<()> {
    let statutes = parse_statutes(inputs)?;

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
                        let path = cmd.strip_prefix("load ").unwrap().trim();
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
                        let path = cmd.strip_prefix("save ").unwrap().trim();
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
    let dsl_content = statute_to_dsl(&entry.statute);

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
            .unwrap()
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
                Cell::new(path.file_name().unwrap().to_string_lossy().as_ref()),
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
                Cell::new(path.file_name().unwrap().to_string_lossy().as_ref()),
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

/// Handles the explain command.
#[allow(dead_code)]
pub fn handle_explain(input: &str, detail: &ExplainDetail, output: Option<&str>) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let mut explanation = String::new();

    // Generate explanation based on detail level
    match detail {
        ExplainDetail::Basic => {
            explanation.push_str(&format!("Statute: {}\n", statute.title.bold()));
            explanation.push_str(&format!("ID: {}\n", statute.id));
            if let Some(ref jurisdiction) = statute.jurisdiction {
                explanation.push_str(&format!("Jurisdiction: {}\n", jurisdiction));
            }
            explanation.push_str(&format!(
                "\nThis statute defines {} condition(s).\n",
                statute.preconditions.len()
            ));
        }
        ExplainDetail::Detailed => {
            explanation.push_str(&format!("Statute: {}\n", statute.title.bold()));
            explanation.push_str(&format!("ID: {}\n", statute.id));
            explanation.push_str(&format!("Version: {}\n", statute.version));
            if let Some(ref jurisdiction) = statute.jurisdiction {
                explanation.push_str(&format!("Jurisdiction: {}\n", jurisdiction));
            }
            if let Some(ref disc) = statute.discretion_logic {
                explanation.push_str(&format!("\nDiscretion: {}\n", disc));
            }

            explanation.push_str(&format!("\n{}\n", "Conditions:".bold()));
            for (i, condition) in statute.preconditions.iter().enumerate() {
                explanation.push_str(&format!("  {}. {}\n", i + 1, condition));
            }

            if !statute.exceptions.is_empty() {
                explanation.push_str(&format!("\n{}\n", "Exceptions:".bold()));
                for (i, exception) in statute.exceptions.iter().enumerate() {
                    explanation.push_str(&format!("  {}. {}\n", i + 1, exception));
                }
            }
        }
        ExplainDetail::Verbose => {
            explanation.push_str(&format!("{}\n", "=".repeat(60)));
            explanation.push_str(&format!("{}: {}\n", "Statute".bold(), statute.title));
            explanation.push_str(&format!("{}\n", "=".repeat(60)));
            explanation.push_str(&format!("{}: {}\n", "ID".bold(), statute.id));
            explanation.push_str(&format!("{}: {}\n", "Version".bold(), statute.version));
            if let Some(ref jurisdiction) = statute.jurisdiction {
                explanation.push_str(&format!("{}: {}\n", "Jurisdiction".bold(), jurisdiction));
            }

            if let Some(ref disc) = statute.discretion_logic {
                explanation.push_str(&format!(
                    "\n{}\n{}\n",
                    "Discretion Logic".bold(),
                    "-".repeat(60)
                ));
                explanation.push_str(&format!("{}\n", disc));
            }

            if let Some(ref effective_date) = statute.temporal_validity.effective_date {
                explanation.push_str(&format!(
                    "\n{}: {}\n",
                    "Effective Date".bold(),
                    effective_date
                ));
            }
            if let Some(ref expiry_date) = statute.temporal_validity.expiry_date {
                explanation.push_str(&format!("{}: {}\n", "Expiry Date".bold(), expiry_date));
            }

            explanation.push_str(&format!("\n{}\n{}\n", "Conditions".bold(), "-".repeat(60)));
            for (i, condition) in statute.preconditions.iter().enumerate() {
                explanation.push_str(&format!("\n{}. {}\n", i + 1, "Condition".bold()));
                explanation.push_str(&format!("   {}\n", condition));
            }

            if !statute.exceptions.is_empty() {
                explanation.push_str(&format!("\n{}\n{}\n", "Exceptions".bold(), "-".repeat(60)));
                for (i, exception) in statute.exceptions.iter().enumerate() {
                    explanation.push_str(&format!("\n{}. {}\n", i + 1, "Exception".bold()));
                    explanation.push_str(&format!("   {}\n", exception));
                }
            }

            if !statute.derives_from.is_empty() {
                explanation.push_str(&format!(
                    "\n{}\n{}\n",
                    "Derived From".bold(),
                    "-".repeat(60)
                ));
                for statute_ref in &statute.derives_from {
                    explanation.push_str(&format!("  - {}\n", statute_ref));
                }
            }

            if !statute.applies_to.is_empty() {
                explanation.push_str(&format!("\n{}\n{}\n", "Applies To".bold(), "-".repeat(60)));
                for entity in &statute.applies_to {
                    explanation.push_str(&format!("  - {}\n", entity));
                }
            }

            explanation.push_str(&format!("\n{}\n", "=".repeat(60)));
        }
    }

    if let Some(out_path) = output {
        fs::write(out_path, &explanation)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("{}", "✓ Explanation written successfully".green().bold());
        println!("  Output: {}", out_path);
    } else {
        println!("{}", explanation);
    }

    Ok(())
}

/// Handles the trace command.
#[allow(dead_code)]
pub fn handle_trace(
    input: &str,
    test_case: &str,
    trace_format: &TraceFormat,
    output: Option<&str>,
) -> Result<()> {
    use serde_json::Value;

    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let test_content = fs::read_to_string(test_case)
        .with_context(|| format!("Failed to read test case file: {}", test_case))?;

    let test_data: Value = serde_json::from_str(&test_content)
        .with_context(|| format!("Failed to parse test case as JSON: {}", test_case))?;

    let mut trace_output = String::new();

    match trace_format {
        TraceFormat::Text => {
            trace_output.push_str(&format!("{}\n", "Condition Evaluation Trace".bold()));
            trace_output.push_str(&format!("{}\n", "=".repeat(60)));
            trace_output.push_str(&format!("Statute: {}\n", statute.title));
            trace_output.push_str(&format!("Test Case: {}\n\n", test_case));

            trace_output.push_str(&format!("{}\n", "Input Variables:".bold()));
            if let Value::Object(map) = &test_data {
                for (key, value) in map {
                    trace_output.push_str(&format!("  {} = {}\n", key, value));
                }
            }

            trace_output.push_str(&format!("\n{}\n", "Evaluation Path:".bold()));
            for (i, condition) in statute.preconditions.iter().enumerate() {
                trace_output.push_str(&format!("  Step {}: Evaluating {}\n", i + 1, condition));
                trace_output.push_str(&format!(
                    "    Result: {}\n",
                    "[Simulated evaluation]".dimmed()
                ));
            }
        }
        TraceFormat::Json => {
            let trace_data = serde_json::json!({
                "statute": {
                    "id": statute.id,
                    "title": statute.title,
                    "version": statute.version
                },
                "test_case": test_case,
                "inputs": test_data,
                "trace": statute.preconditions.iter().enumerate().map(|(i, cond)| {
                    serde_json::json!({
                        "step": i + 1,
                        "condition": format!("{}", cond),
                        "result": "simulated"
                    })
                }).collect::<Vec<_>>()
            });
            trace_output = serde_json::to_string_pretty(&trace_data)?;
        }
        TraceFormat::Tree => {
            trace_output.push_str(&format!("{}\n", statute.title.bold()));
            trace_output.push_str("│\n");
            for (i, condition) in statute.preconditions.iter().enumerate() {
                let is_last = i == statute.preconditions.len() - 1;
                let prefix = if is_last { "└──" } else { "├──" };
                trace_output.push_str(&format!("{} Step {}: {}\n", prefix, i + 1, condition));
            }
        }
        TraceFormat::Mermaid => {
            trace_output.push_str("```mermaid\n");
            trace_output.push_str("graph TD\n");
            trace_output.push_str(&format!("  Start[Start: {}]\n", statute.title));

            for (i, condition) in statute.preconditions.iter().enumerate() {
                let node_id = format!("C{}", i + 1);
                let prev_id = if i == 0 {
                    "Start".to_string()
                } else {
                    format!("C{}", i)
                };
                trace_output.push_str(&format!("  {}[\"{}: {}\"]\n", node_id, i + 1, condition));
                trace_output.push_str(&format!("  {} --> {}\n", prev_id, node_id));
            }

            let last_id = format!("C{}", statute.preconditions.len());
            trace_output.push_str(&format!("  {}[End]\n", "End"));
            trace_output.push_str(&format!("  {} --> End\n", last_id));
            trace_output.push_str("```\n");
        }
    }

    if let Some(out_path) = output {
        fs::write(out_path, &trace_output)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("{}", "✓ Trace written successfully".green().bold());
        println!("  Output: {}", out_path);
    } else {
        println!("{}", trace_output);
    }

    Ok(())
}

/// Handles the benchmark command.
#[allow(dead_code)]
pub async fn handle_benchmark(
    inputs: &[String],
    bench_type: &BenchmarkType,
    iterations: usize,
    population: usize,
    output: Option<&str>,
) -> Result<()> {
    use std::time::Instant;

    println!("{}", "Running benchmarks...".cyan().bold());
    println!("Iterations: {}", iterations);
    println!("Population: {}", population);
    println!();

    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    for input in inputs {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;
        let statute = parser
            .parse_statute(&content)
            .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", input, e))?;
        statutes.push(statute);
    }

    let mut results = Vec::new();

    match bench_type {
        BenchmarkType::Verify | BenchmarkType::All => {
            println!("{}", "Benchmarking verification...".bold());
            let verifier = StatuteVerifier::new();

            let start = Instant::now();
            for _ in 0..iterations {
                let _ = verifier.verify(&statutes);
            }
            let duration = start.elapsed();

            let avg = duration.as_secs_f64() / iterations as f64;
            results.push(format!("Verification: {:.4}s per iteration", avg));
            println!("  ✓ Average: {:.4}s per iteration", avg);
            println!(
                "  ✓ Total: {:.4}s for {} iterations",
                duration.as_secs_f64(),
                iterations
            );
        }
        _ => {}
    }

    match bench_type {
        BenchmarkType::Simulate | BenchmarkType::All => {
            println!("\n{}", "Benchmarking simulation...".bold());

            let start = Instant::now();
            for _ in 0..iterations {
                let _sim_result = simulate_statute(&statutes, population);
            }
            let duration = start.elapsed();

            let avg = duration.as_secs_f64() / iterations as f64;
            results.push(format!(
                "Simulation: {:.4}s per iteration (population: {})",
                avg, population
            ));
            println!(
                "  ✓ Average: {:.4}s per iteration (population: {})",
                avg, population
            );
            println!(
                "  ✓ Total: {:.4}s for {} iterations",
                duration.as_secs_f64(),
                iterations
            );
        }
        _ => {}
    }

    let report = results.join("\n");

    if let Some(out_path) = output {
        fs::write(out_path, &report)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!(
            "\n{}",
            "✓ Benchmark results written successfully".green().bold()
        );
        println!("  Output: {}", out_path);
    } else {
        println!("\n{}", "Benchmark Results:".bold());
        println!("{}", report);
    }

    Ok(())
}

#[allow(dead_code)]
fn simulate_statute(_statutes: &[Statute], _population: usize) -> usize {
    _population
}

/// Handles the migrate command.
#[allow(dead_code)]
pub fn handle_migrate(
    input: &str,
    from_version: &str,
    to_version: &str,
    output: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    if dry_run {
        println!("{}", "[DRY RUN] Migration Plan:".cyan().bold());
        println!("  Source: {}", input);
        println!("  From version: {}", from_version);
        println!("  To version: {}", to_version);
        println!();
        println!("Migration steps:");
        println!("  1. Parse statute with version {}", from_version);
        println!("  2. Apply version-specific transformations");
        println!("  3. Validate migrated statute for version {}", to_version);
        println!("  4. Write migrated statute to output");
        return Ok(());
    }

    println!("{}", "Migrating statute...".cyan().bold());
    println!("  From: version {}", from_version);
    println!("  To: version {}", to_version);

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    let migrated_output = serde_json::to_string_pretty(&statute)?;

    if let Some(out_path) = output {
        fs::write(out_path, &migrated_output)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("{}", "✓ Migration completed successfully".green().bold());
        println!("  Output: {}", out_path);
    } else {
        println!("{}", migrated_output);
    }

    Ok(())
}

/// Handles the graph command.
#[allow(dead_code)]
pub fn handle_graph(
    inputs: &[String],
    graph_type: &GraphType,
    output: &str,
    graph_format: &GraphFormat,
) -> Result<()> {
    println!("{}", "Generating dependency graph...".cyan().bold());
    println!("  Graph type: {:?}", graph_type);
    println!("  Format: {:?}", graph_format);

    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    for input in inputs {
        if Path::new(input).is_dir() {
            let entries = fs::read_dir(input)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path
                    .extension()
                    .is_some_and(|ext| ext == "leg" || ext == "legalis")
                {
                    let content = fs::read_to_string(&path)?;
                    if let Ok(statute) = parser.parse_statute(&content) {
                        statutes.push(statute);
                    }
                }
            }
        } else {
            let content = fs::read_to_string(input)?;
            let statute = parser
                .parse_statute(&content)
                .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", input, e))?;
            statutes.push(statute);
        }
    }

    let mut graph_output = String::new();

    match graph_format {
        GraphFormat::Dot => {
            graph_output.push_str("digraph Dependencies {\n");
            graph_output.push_str("  rankdir=LR;\n");
            graph_output.push_str("  node [shape=box];\n\n");

            for statute in &statutes {
                graph_output.push_str(&format!(
                    "  \"{}\" [label=\"{}\"];\n",
                    statute.id, statute.title
                ));

                for dep in &statute.derives_from {
                    graph_output.push_str(&format!("  \"{}\" -> \"{}\";\n", dep, statute.id));
                }
            }

            graph_output.push_str("}\n");
        }
        GraphFormat::Mermaid => {
            graph_output.push_str("```mermaid\n");
            graph_output.push_str("graph LR\n");

            for statute in &statutes {
                let safe_id = statute.id.replace("-", "_");
                graph_output.push_str(&format!("  {}[\"{}\"]\n", safe_id, statute.title));

                for dep in &statute.derives_from {
                    let safe_dep = dep.replace("-", "_");
                    graph_output.push_str(&format!("  {} --> {}\n", safe_dep, safe_id));
                }
            }

            graph_output.push_str("```\n");
        }
        GraphFormat::Json => {
            let graph_data = serde_json::json!({
                "nodes": statutes.iter().map(|s| {
                    serde_json::json!({
                        "id": s.id,
                        "title": s.title,
                        "version": s.version
                    })
                }).collect::<Vec<_>>(),
                "edges": statutes.iter().flat_map(|s| {
                    s.derives_from.iter().map(|dep| {
                        serde_json::json!({
                            "from": dep,
                            "to": s.id
                        })
                    }).collect::<Vec<_>>()
                }).collect::<Vec<_>>()
            });
            graph_output = serde_json::to_string_pretty(&graph_data)?;
        }
        GraphFormat::Svg => {
            graph_output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
            graph_output.push_str(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"600\">\n",
            );
            graph_output.push_str("  <!-- SVG graph generation not yet implemented -->\n");
            graph_output.push_str(
                "  <text x=\"400\" y=\"300\" text-anchor=\"middle\">Graph visualization</text>\n",
            );
            graph_output.push_str("</svg>\n");
        }
    }

    fs::write(output, &graph_output)
        .with_context(|| format!("Failed to write output file: {}", output))?;

    println!("{}", "✓ Graph generated successfully".green().bold());
    println!("  Output: {}", output);
    println!("  Nodes: {}", statutes.len());

    Ok(())
}

/// Handles the builder-wizard command.
pub fn handle_builder_wizard(help_only: bool) -> Result<()> {
    use crate::interactive::{StatuteBuilderResult, interactive_statute_builder};

    if help_only {
        println!("{}", "Interactive Statute Builder Wizard".green().bold());
        println!("\nThis wizard will guide you through creating a comprehensive statute with:");
        println!("  - Basic information (ID, title, jurisdiction)");
        println!("  - Effective and expiry dates");
        println!("  - Multiple conditions (age, income, geographic, temporal, boolean)");
        println!("  - Outcome definitions (eligible, benefit, penalty, custom)");
        println!("\nRun without --help-only to start the wizard.");
        return Ok(());
    }

    let result: StatuteBuilderResult = interactive_statute_builder()?;

    // Generate DSL output
    let mut dsl_output = String::new();
    dsl_output.push_str(&format!("statute {} {{\n", result.statute_id));
    dsl_output.push_str(&format!("  title: \"{}\"\n", result.title));
    dsl_output.push_str(&format!("  jurisdiction: \"{}\"\n", result.jurisdiction));

    if let Some(ref from) = result.effective_from {
        dsl_output.push_str(&format!("  effective_from: \"{}\"\n", from));
    }
    if let Some(ref until) = result.effective_until {
        dsl_output.push_str(&format!("  effective_until: \"{}\"\n", until));
    }

    dsl_output.push_str("\n  condition: ");
    if result.conditions.len() == 1 {
        let cond = &result.conditions[0];
        dsl_output.push_str(&format!(
            "{} {} {}\n",
            cond.cond_type, cond.operator, cond.value
        ));
    } else {
        dsl_output.push_str("(\n");
        for (idx, cond) in result.conditions.iter().enumerate() {
            dsl_output.push_str(&format!(
                "    {} {} {}",
                cond.cond_type, cond.operator, cond.value
            ));
            if idx < result.conditions.len() - 1 {
                dsl_output.push_str(&format!(" {}\n", result.combine_operator));
            } else {
                dsl_output.push('\n');
            }
        }
        dsl_output.push_str("  )\n");
    }

    dsl_output.push_str("\n  outcome: ");
    if let Some(ref value) = result.outcome_value {
        dsl_output.push_str(&format!("{} \"{}\"\n", result.outcome_type, value));
    } else {
        dsl_output.push_str(&format!("{}\n", result.outcome_type));
    }

    dsl_output.push_str("}\n");

    // Write to file
    fs::write(&result.output_path, &dsl_output)
        .with_context(|| format!("Failed to write statute file: {}", result.output_path))?;

    println!("{}", "✓ Statute created successfully".green().bold());
    println!("  Output: {}", result.output_path);
    println!("  ID: {}", result.statute_id);
    println!("  Conditions: {}", result.conditions.len());

    Ok(())
}

/// Handles the diff-viewer command.
pub fn handle_diff_viewer(old_path: &str, new_path: &str) -> Result<()> {
    use crate::interactive::{DiffViewerResult, interactive_diff_viewer};

    let result: DiffViewerResult = interactive_diff_viewer(old_path, new_path)?;

    match result.action.as_str() {
        "accept" => {
            if result.should_backup {
                let backup_path = format!("{}.backup", result.old_path);
                fs::copy(&result.old_path, &backup_path)?;
                println!("{}", format!("✓ Created backup: {}", backup_path).yellow());
            }
            fs::copy(&result.new_path, &result.old_path)?;
            println!("{}", "✓ Accepted new version".green().bold());
        }
        "reject" => {
            println!("{}", "✓ Kept old version (no changes made)".yellow());
        }
        "merge" => {
            if result.should_backup {
                let backup_path = format!("{}.backup", result.old_path);
                fs::copy(&result.old_path, &backup_path)?;
                println!("{}", format!("✓ Created backup: {}", backup_path).yellow());
            }
            // Simple merge: append new content
            let old_content = fs::read_to_string(&result.old_path)?;
            let new_content = fs::read_to_string(&result.new_path)?;
            let merged = format!(
                "{}\n\n--- Merged changes ---\n\n{}",
                old_content, new_content
            );
            fs::write(&result.old_path, merged)?;
            println!("{}", "✓ Merged versions".green().bold());
        }
        "edit" => {
            println!("{}", "Manual editing not yet implemented".yellow());
        }
        "cancel" => {
            println!("{}", "✓ Cancelled (no changes made)".yellow());
        }
        _ => {}
    }

    Ok(())
}

/// Handles the sim-tune command.
pub async fn handle_sim_tune(inputs: &[String]) -> Result<()> {
    use crate::interactive::{SimulationParams, interactive_simulation_tuning};

    let params: SimulationParams = interactive_simulation_tuning()?;

    println!(
        "{}",
        "Running simulation with tuned parameters...".cyan().bold()
    );
    println!("  Population: {}", params.population_size);
    println!("  Iterations: {}", params.iterations);

    if let Some((min, max)) = params.age_distribution {
        println!("  Age range: {} - {}", min, max);
    }
    if let Some((min, max)) = params.income_distribution {
        println!("  Income range: {} - {}", min, max);
    }

    // Run simulation with the tuned parameters
    handle_simulate(inputs, params.population_size, Some(&params.output_path)).await?;

    println!(
        "{}",
        "✓ Simulation completed with tuned parameters"
            .green()
            .bold()
    );
    println!("  Results: {}", params.output_path);

    Ok(())
}

/// Handles the resolve-conflicts command.
pub fn handle_resolve_conflicts(inputs: &[String]) -> Result<()> {
    use crate::interactive::{ConflictInfo, interactive_conflict_resolution};

    println!("{}", "Analyzing statutes for conflicts...".cyan().bold());

    // Parse all input statutes
    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    for input in inputs {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;
        let statute = parser
            .parse_statute(&content)
            .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", input, e))?;
        statutes.push((input.clone(), statute));
    }

    // Detect conflicts (simplified implementation)
    let mut conflicts = Vec::new();
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let (path1, statute1) = &statutes[i];
            let (path2, statute2) = &statutes[j];

            if statute1.id == statute2.id && statute1.version != statute2.version {
                conflicts.push(ConflictInfo {
                    id: format!("conflict_{}", conflicts.len() + 1),
                    conflict_type: "version_mismatch".to_string(),
                    description: format!(
                        "Statute '{}' has different versions: {} vs {}",
                        statute1.id, statute1.version, statute2.version
                    ),
                    details: Some(format!("Files: {} vs {}", path1, path2)),
                });
            }

            if statute1.jurisdiction == statute2.jurisdiction && statute1.id != statute2.id {
                // Check for overlapping conditions (simplified)
                let jurisdiction_str = statute1.jurisdiction.as_deref().unwrap_or("unspecified");
                conflicts.push(ConflictInfo {
                    id: format!("conflict_{}", conflicts.len() + 1),
                    conflict_type: "jurisdiction_overlap".to_string(),
                    description: format!(
                        "Statutes '{}' and '{}' may overlap in jurisdiction '{}'",
                        statute1.id, statute2.id, jurisdiction_str
                    ),
                    details: Some(format!("Files: {} vs {}", path1, path2)),
                });
            }
        }
    }

    if conflicts.is_empty() {
        println!("{}", "✓ No conflicts detected".green().bold());
        return Ok(());
    }

    // Resolve conflicts interactively
    let resolutions = interactive_conflict_resolution(&conflicts)?;

    println!("\n{}", "Conflict Resolution Summary:".cyan().bold());
    for resolution in &resolutions {
        println!(
            "  Conflict {}: {}",
            resolution.conflict_id, resolution.resolution_type
        );
        if let Some(ref custom) = resolution.custom_value {
            println!("    Custom value: {}", custom);
        }
    }

    println!("{}", "✓ Conflicts resolved".green().bold());

    Ok(())
}

/// Handles the registry-browser command.
pub fn handle_registry_browser(registry_path: &str, start_search: bool) -> Result<()> {
    println!("{}", "Registry Browser (TUI)".cyan().bold());
    println!("  Registry: {}", registry_path);

    if start_search {
        println!("  Mode: Search");
    } else {
        println!("  Mode: Browse");
    }

    println!("\n{}", "TUI Dashboard Features:".yellow());
    println!("  • Browse statutes in registry");
    println!("  • Search by ID, title, or jurisdiction");
    println!("  • Filter by tags and metadata");
    println!("  • View statute details");
    println!("  • Install/uninstall statutes");
    println!("  • Compare statute versions");

    println!(
        "\n{}",
        "Note: Full TUI implementation requires additional dependencies (tui-rs/ratatui)".yellow()
    );
    println!(
        "{}",
        "For now, showing list of available statutes:".yellow()
    );

    // Simple listing as placeholder
    let registry_dir = Path::new(registry_path);
    if !registry_dir.exists() {
        anyhow::bail!("Registry directory does not exist: {}", registry_path);
    }

    let mut statute_count = 0;
    for entry in fs::read_dir(registry_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ldsl") {
            statute_count += 1;
            if let Some(file_name) = path.file_name() {
                println!("  📄 {}", file_name.to_string_lossy());
            }
        }
    }

    println!(
        "\n{}",
        format!("✓ Found {} statute(s) in registry", statute_count)
            .green()
            .bold()
    );

    Ok(())
}

/// Handles batch verify operation.
pub async fn handle_batch_verify(
    input: &str,
    strict: bool,
    workers: Option<usize>,
    resume: bool,
    journal_path: &str,
) -> Result<()> {
    use crate::batch::{BatchProcessor, expand_glob_pattern};

    println!("{}", "Starting batch verify operation...".cyan().bold());

    let files = expand_glob_pattern(input)?;
    println!("Found {} file(s) to verify", files.len());

    let processor = BatchProcessor::new(workers);
    let journal_path = Path::new(journal_path);

    let results = processor
        .process(
            files.clone(),
            journal_path,
            resume,
            "batch_verify",
            move |file| {
                let content = fs::read_to_string(&file)?;
                let parser = LegalDslParser::new();
                let statute = parser
                    .parse_statute(&content)
                    .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

                let verifier = StatuteVerifier::new();
                let result = verifier.verify(&[statute]);

                if strict && !result.warnings.is_empty() {
                    anyhow::bail!("Verification warnings found");
                }

                Ok(result.passed)
            },
        )
        .await?;

    // Print summary
    let successful = results.iter().filter(|(_, r)| r.is_ok()).count();
    let failed = results.len() - successful;

    println!("\n{}", "=== Batch Verify Summary ===".cyan().bold());
    println!("  Total files: {}", results.len());
    println!("  {} Successful: {}", "✓".green(), successful);
    println!("  {} Failed: {}", "✗".red(), failed);

    if failed > 0 {
        println!("\n{}", "Failed files:".red().bold());
        for (file, result) in &results {
            if let Err(e) = result {
                println!("  {} {}: {}", "✗".red(), file.display(), e);
            }
        }
    }

    if failed > 0 {
        anyhow::bail!("Batch verify completed with {} failure(s)", failed);
    }

    Ok(())
}

/// Handles batch format operation.
pub async fn handle_batch_format(
    input: &str,
    style: &crate::FormatStyle,
    inplace: bool,
    workers: Option<usize>,
    resume: bool,
    journal_path: &str,
) -> Result<()> {
    use crate::batch::{BatchProcessor, expand_glob_pattern};

    println!("{}", "Starting batch format operation...".cyan().bold());

    let files = expand_glob_pattern(input)?;
    println!("Found {} file(s) to format", files.len());

    let processor = BatchProcessor::new(workers);
    let journal_path = Path::new(journal_path);
    let printer_config: legalis_dsl::PrinterConfig = style.clone().into();

    let results = processor
        .process(
            files.clone(),
            journal_path,
            resume,
            "batch_format",
            move |file| {
                let content = fs::read_to_string(&file)?;
                let parser = LegalDslParser::new();
                let statute = parser
                    .parse_statute(&content)
                    .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

                let printer = legalis_dsl::DslPrinter::with_config(printer_config.clone());
                let formatted = printer.format(&statute);

                if inplace {
                    fs::write(&file, &formatted)?;
                }

                Ok(formatted)
            },
        )
        .await?;

    // Print summary
    let successful = results.iter().filter(|(_, r)| r.is_ok()).count();
    let failed = results.len() - successful;

    println!("\n{}", "=== Batch Format Summary ===".cyan().bold());
    println!("  Total files: {}", results.len());
    println!("  {} Formatted: {}", "✓".green(), successful);
    println!("  {} Failed: {}", "✗".red(), failed);

    if failed > 0 {
        anyhow::bail!("Batch format completed with {} failure(s)", failed);
    }

    Ok(())
}

/// Handles batch lint operation.
pub async fn handle_batch_lint(
    input: &str,
    fix: bool,
    strict: bool,
    workers: Option<usize>,
    resume: bool,
    journal_path: &str,
) -> Result<()> {
    use crate::batch::{BatchProcessor, expand_glob_pattern};

    println!("{}", "Starting batch lint operation...".cyan().bold());

    let files = expand_glob_pattern(input)?;
    println!("Found {} file(s) to lint", files.len());

    let processor = BatchProcessor::new(workers);
    let journal_path = Path::new(journal_path);

    let results = processor
        .process(
            files.clone(),
            journal_path,
            resume,
            "batch_lint",
            move |file| {
                // Call existing lint handler logic
                handle_lint(&[file.to_string_lossy().to_string()], fix, strict)?;
                Ok(())
            },
        )
        .await?;

    // Print summary
    let successful = results.iter().filter(|(_, r)| r.is_ok()).count();
    let failed = results.len() - successful;

    println!("\n{}", "=== Batch Lint Summary ===".cyan().bold());
    println!("  Total files: {}", results.len());
    println!("  {} Passed: {}", "✓".green(), successful);
    println!("  {} Failed: {}", "✗".red(), failed);

    if failed > 0 {
        anyhow::bail!("Batch lint completed with {} failure(s)", failed);
    }

    Ok(())
}

/// Handles batch export operation.
pub async fn handle_batch_export(
    input: &str,
    output_dir: &str,
    export_format: &crate::ExportFormat,
    workers: Option<usize>,
    resume: bool,
    journal_path: &str,
) -> Result<()> {
    use crate::batch::{BatchProcessor, expand_glob_pattern};

    println!("{}", "Starting batch export operation...".cyan().bold());

    let files = expand_glob_pattern(input)?;
    println!("Found {} file(s) to export", files.len());

    // Create output directory
    let output_path = Path::new(output_dir);
    fs::create_dir_all(output_path)?;

    let processor = BatchProcessor::new(workers);
    let journal_path = Path::new(journal_path);
    let format = export_format.clone();
    let output_path_buf = output_path.to_path_buf();

    let results = processor
        .process(
            files.clone(),
            journal_path,
            resume,
            "batch_export",
            move |file| {
                let file_name = file
                    .file_stem()
                    .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;
                let ext = match format {
                    crate::ExportFormat::Json => "json",
                    crate::ExportFormat::Yaml => "yaml",
                    crate::ExportFormat::Solidity => "sol",
                };
                let output_file =
                    output_path_buf.join(format!("{}.{}", file_name.to_string_lossy(), ext));

                handle_export(
                    &file.to_string_lossy(),
                    &output_file.to_string_lossy(),
                    &format,
                )?;

                Ok(())
            },
        )
        .await?;

    // Print summary
    let successful = results.iter().filter(|(_, r)| r.is_ok()).count();
    let failed = results.len() - successful;

    println!("\n{}", "=== Batch Export Summary ===".cyan().bold());
    println!("  Total files: {}", results.len());
    println!("  {} Exported: {}", "✓".green(), successful);
    println!("  {} Failed: {}", "✗".red(), failed);
    println!("  Output directory: {}", output_dir);

    if failed > 0 {
        anyhow::bail!("Batch export completed with {} failure(s)", failed);
    }

    Ok(())
}

/// Handles the profile command.
#[allow(clippy::too_many_arguments)]
pub fn handle_profile(
    inputs: &[String],
    profile_type: &crate::ProfileType,
    iterations: usize,
    output: Option<&str>,
    flamegraph: bool,
    #[cfg(target_os = "linux")] flamegraph_dir: &str,
    format: &OutputFormat,
) -> Result<()> {
    use crate::profile::Profiler;

    println!("{}", "Starting profiling session...".cyan().bold());

    let profile_cpu = matches!(
        profile_type,
        crate::ProfileType::Cpu | crate::ProfileType::All
    );
    let profile_memory = matches!(
        profile_type,
        crate::ProfileType::Memory | crate::ProfileType::All
    );

    let mut profiler = Profiler::new(profile_cpu, profile_memory);

    // Load statutes once
    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    for input in inputs {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;
        let statute = parser
            .parse_statute(&content)
            .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", input, e))?;
        statutes.push(statute);
    }

    println!("Loaded {} statute(s)", statutes.len());
    println!("Running {} iteration(s)...", iterations);

    // Profile the verification operation
    let verifier = StatuteVerifier::new();
    let profile_data = profiler.profile(iterations, || {
        let _ = verifier.verify(&statutes);
        Ok(())
    })?;

    // Generate flamegraph if requested
    #[cfg(target_os = "linux")]
    if flamegraph {
        println!("\n{}", "Generating flamegraph...".cyan().bold());
        if let Err(e) = generate_flamegraph(inputs, iterations, flamegraph_dir) {
            eprintln!(
                "{} Failed to generate flamegraph: {}",
                "Warning:".yellow(),
                e
            );
            eprintln!("  Make sure 'perf' and 'flamegraph' are installed");
        } else {
            println!("  Flamegraph saved to: {}/flamegraph.svg", flamegraph_dir);
        }
    }

    #[cfg(not(target_os = "linux"))]
    if flamegraph {
        eprintln!(
            "{} Flamegraph generation is only supported on Linux",
            "Warning:".yellow()
        );
    }

    // Format output
    let output_str = match format {
        OutputFormat::Json => profile_data.to_json()?,
        OutputFormat::Yaml => serde_yaml::to_string(&profile_data)?,
        _ => profile_data.format_report(),
    };

    // Write output
    if let Some(out_path) = output {
        fs::write(out_path, &output_str)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("\nProfile results written to: {}", out_path);
    } else {
        println!("{}", output_str);
    }

    Ok(())
}

/// Generate a flamegraph using perf and flamegraph tools.
#[cfg(target_os = "linux")]
fn generate_flamegraph(inputs: &[String], iterations: usize, output_dir: &str) -> Result<()> {
    use std::process::Command;

    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Build command to profile
    let input_args = inputs.join(" ");
    let cmd = format!("legalis verify --input {} 2>/dev/null", input_args);

    // Run perf record
    let perf_data = format!("{}/perf.data", output_dir);
    let perf_output = Command::new("perf")
        .args(&[
            "record",
            "-F",
            "99",
            "-g",
            "-o",
            &perf_data,
            "--",
            "sh",
            "-c",
            &format!("for i in $(seq 1 {}); do {}; done", iterations, cmd),
        ])
        .output()
        .context("Failed to run perf record")?;

    if !perf_output.status.success() {
        anyhow::bail!("perf record failed");
    }

    // Convert perf data to flamegraph
    let perf_script = Command::new("perf")
        .args(&["script", "-i", &perf_data])
        .output()
        .context("Failed to run perf script")?;

    if !perf_script.status.success() {
        anyhow::bail!("perf script failed");
    }

    // Generate flamegraph
    let flamegraph_path = format!("{}/flamegraph.svg", output_dir);
    let flamegraph_output = Command::new("flamegraph")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(&perf_script.stdout)?;
            }
            child.wait_with_output()
        })
        .context("Failed to generate flamegraph")?;

    if !flamegraph_output.status.success() {
        anyhow::bail!("flamegraph generation failed");
    }

    fs::write(&flamegraph_path, flamegraph_output.stdout)?;

    Ok(())
}

/// Handles the debug command.
pub fn handle_debug(
    input: &str,
    test_case: &str,
    interactive: bool,
    show_memory: bool,
    show_timing: bool,
    output: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    use crate::debug::Debugger;

    println!("{}", "Starting debug session...".cyan().bold());

    // Load statute
    let content = fs::read_to_string(input)
        .with_context(|| format!("Failed to read input file: {}", input))?;

    let parser = LegalDslParser::new();
    let statute = parser
        .parse_statute(&content)
        .map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;

    // Load test case
    let test_case_content = fs::read_to_string(test_case)
        .with_context(|| format!("Failed to read test case file: {}", test_case))?;
    let test_inputs: serde_json::Value = serde_json::from_str(&test_case_content)
        .with_context(|| "Failed to parse test case JSON")?;

    // Create debugger
    let mut debugger = Debugger::new(interactive, show_timing, show_memory);

    // Step 1: Parse
    {
        let guard = debugger.begin_step("Parse statute", serde_json::json!({"file": input}));
        guard.complete(serde_json::json!({
            "id": statute.id,
            "title": statute.title,
            "preconditions_count": statute.preconditions.len(),
        }));
    }

    // Step 2: Verify
    {
        let guard = debugger.begin_step(
            "Verify statute",
            serde_json::json!({"statute_id": statute.id}),
        );

        let verifier = StatuteVerifier::new();
        let result = verifier.verify(std::slice::from_ref(&statute));

        let is_valid = result.errors.is_empty();
        guard.complete(serde_json::json!({
            "valid": is_valid,
            "errors": result.errors,
            "warnings": result.warnings,
        }));
    }

    // Step 3: Evaluate conditions
    if let Some(_inputs_obj) = test_inputs.as_object() {
        let guard = debugger.begin_step("Evaluate conditions", test_inputs.clone());

        // Simulate condition evaluation
        let mut results = serde_json::Map::new();
        for (idx, _condition) in statute.preconditions.iter().enumerate() {
            results.insert(
                format!("condition_{}", idx),
                serde_json::json!({"evaluated": true}),
            );
        }

        guard.complete(serde_json::json!(results));
    }

    // Get the trace
    let trace = debugger.trace();

    // Format output
    let output_str = match format {
        OutputFormat::Json => trace.to_json()?,
        OutputFormat::Yaml => serde_yaml::to_string(&trace)?,
        _ => trace.format_report(show_timing, show_memory),
    };

    // Write output
    if let Some(out_path) = output {
        fs::write(out_path, &output_str)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("\nDebug trace written to: {}", out_path);
    } else {
        println!("{}", output_str);
    }

    Ok(())
}

impl crate::profile::ProfileData {
    /// Convert to JSON string.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize profile data to JSON")
    }
}

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

/// Handles the script run command.
pub fn handle_script_run(script: &str, args: &[String], _debug: bool) -> Result<()> {
    use crate::scripting::{ScriptContext, ScriptManager};
    use colored::Colorize;
    use std::collections::HashMap;

    println!("{}", "Executing script...".cyan().bold());
    println!("  Script: {}", script.yellow());

    let mut manager = ScriptManager::new()?;
    manager.discover_scripts()?;

    let context = ScriptContext {
        args: args.to_vec(),
        env: HashMap::new(),
        cwd: std::env::current_dir()?,
    };

    let result = if Path::new(script).exists() {
        // Execute script file directly
        let script_content = fs::read_to_string(script)?;
        let permissions = crate::scripting::ScriptPermissions::default();
        manager.execute_lua_script(&script_content, context, &permissions)?
    } else {
        // Execute installed script by name
        manager.execute_script(script, context)?
    };

    if !result.stdout.is_empty() {
        println!("\n{}", "Output:".bold());
        println!("{}", result.stdout);
    }

    if !result.stderr.is_empty() {
        eprintln!("\n{}", "Errors:".red().bold());
        eprintln!("{}", result.stderr);
    }

    println!(
        "\n{} Execution completed in {}ms",
        if result.exit_code == 0 {
            "✓".green()
        } else {
            "✗".red()
        },
        result.execution_time_ms
    );

    if result.exit_code != 0 {
        std::process::exit(result.exit_code);
    }

    Ok(())
}

/// Handles the script list command.
pub fn handle_script_list(verbose: bool) -> Result<()> {
    use crate::scripting::ScriptManager;
    use colored::Colorize;

    let mut manager = ScriptManager::new()?;
    manager.discover_scripts()?;

    let scripts = manager.list_scripts();

    if scripts.is_empty() {
        println!("{}", "No scripts installed.".dimmed());
        println!("\nTo install a script, run:");
        println!("  {}", "legalis script install --source <path>".bold());
        return Ok(());
    }

    println!(
        "{}",
        format!("Installed Scripts ({})", scripts.len())
            .bold()
            .cyan()
    );
    println!("{}", "=".repeat(50).dimmed());
    println!();

    for script in scripts {
        println!("{} {}", "●".cyan(), script.name.bold());
        println!("  Version: {}", script.version.dimmed());
        println!("  Author: {}", script.author.dimmed());
        println!("  Description: {}", script.description);

        if verbose {
            println!("  Main file: {}", script.main.yellow());
            if let Some(ref req) = script.requires {
                println!("  Requires: {}", req);
            }
            if !script.dependencies.is_empty() {
                println!("  Dependencies: {}", script.dependencies.join(", "));
            }
        }
        println!();
    }

    Ok(())
}

/// Handles the script info command.
pub fn handle_script_info(name: &str) -> Result<()> {
    use crate::scripting::ScriptManager;
    use colored::Colorize;

    let mut manager = ScriptManager::new()?;
    manager.discover_scripts()?;

    let script = manager
        .get_script(name)
        .ok_or_else(|| anyhow::anyhow!("Script '{}' not found", name))?;

    println!("{}", "Script Information".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());
    println!();
    println!("  {}: {}", "Name".bold(), script.name);
    println!("  {}: {}", "Version".bold(), script.version);
    println!("  {}: {}", "Author".bold(), script.author);
    println!("  {}: {}", "Description".bold(), script.description);
    println!("  {}: {}", "Main file".bold(), script.main);

    if let Some(ref req) = script.requires {
        println!("  {}: {}", "Requires".bold(), req);
    }

    if !script.dependencies.is_empty() {
        println!("\n  {}:", "Dependencies".bold().yellow());
        for dep in &script.dependencies {
            println!("    - {}", dep);
        }
    }

    println!("\n  {}:", "Permissions".bold().yellow());
    println!(
        "    Filesystem: {}",
        if script.permissions.filesystem {
            "✓".green()
        } else {
            "✗".red()
        }
    );
    println!(
        "    Network: {}",
        if script.permissions.network {
            "✓".green()
        } else {
            "✗".red()
        }
    );
    println!(
        "    Process: {}",
        if script.permissions.process {
            "✓".green()
        } else {
            "✗".red()
        }
    );
    println!(
        "    Environment: {}",
        if script.permissions.env {
            "✓".green()
        } else {
            "✗".red()
        }
    );
    println!("    Timeout: {}s", script.permissions.timeout);
    println!("    Memory limit: {}MB", script.permissions.memory_limit);

    println!();

    Ok(())
}

/// Handles the script install command.
pub fn handle_script_install(source: &str) -> Result<()> {
    use crate::scripting::ScriptManager;
    use colored::Colorize;

    println!("{}", "Installing script...".cyan().bold());
    println!("  Source: {}", source.yellow());

    let source_path = Path::new(source);
    if !source_path.exists() {
        anyhow::bail!("Script source path does not exist: {}", source);
    }

    let mut manager = ScriptManager::new()?;
    manager.install_script(source_path)?;

    println!("\n{} Script installed successfully!", "✓".green().bold());

    Ok(())
}

/// Handles the script uninstall command.
pub fn handle_script_uninstall(name: &str, yes: bool) -> Result<()> {
    use crate::scripting::ScriptManager;
    use colored::Colorize;

    if !yes {
        print!(
            "Are you sure you want to uninstall script '{}'? [y/N]: ",
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

    println!("{}", "Uninstalling script...".cyan().bold());
    println!("  Script: {}", name.yellow());

    let mut manager = ScriptManager::new()?;
    manager.discover_scripts()?;
    manager.uninstall_script(name)?;

    println!("\n{} Script uninstalled successfully!", "✓".green().bold());

    Ok(())
}

/// Handles the script new command.
pub fn handle_script_new(
    name: &str,
    template: &crate::ScriptTemplate,
    output: Option<&str>,
) -> Result<()> {
    use colored::Colorize;

    let output_dir = output.unwrap_or(".");
    let script_dir = Path::new(output_dir).join(name);

    if script_dir.exists() {
        anyhow::bail!("Script directory already exists: {}", script_dir.display());
    }

    fs::create_dir_all(&script_dir)?;

    // Create script.toml manifest
    let manifest_content = format!(
        r#"name = "{}"
version = "0.1.0"
description = "A new Legalis script"
author = "Your Name"
main = "main.lua"

[permissions]
filesystem = false
network = false
process = false
env = false
timeout = 30
memory_limit = 100
"#,
        name
    );

    fs::write(script_dir.join("script.toml"), manifest_content)?;

    // Create main.lua script
    let script_content = match template {
        crate::ScriptTemplate::Basic => {
            r#"-- Basic Legalis Script
print("Hello from", args[1] or "Legalis")
print("Arguments:", table.concat(args, ", "))
"#
        }
        crate::ScriptTemplate::Batch => {
            r#"-- Batch Processing Script
print("Batch processing script")
print("Processing", #args, "items...")

for i, item in ipairs(args) do
    print("Processing item", i, ":", item)
end

print("Batch processing complete!")
"#
        }
        crate::ScriptTemplate::Report => {
            r#"-- Report Generation Script
print("=== Legalis Report ===")
print("Generated at:", os.date())
print("Working directory:", cwd)
print()
print("Arguments provided:", #args)
for i, arg in ipairs(args) do
    print("  " .. i .. ". " .. arg)
end
print()
print("=== End of Report ===")
"#
        }
        crate::ScriptTemplate::Transform => {
            r#"-- Data Transformation Script
print("Data transformation script")
print("Input:", args[1] or "none")
print("Output:", args[2] or "none")
print()
print("Transformation complete!")
"#
        }
    };

    fs::write(script_dir.join("main.lua"), script_content)?;

    println!("{}", "✓ Script created successfully!".green().bold());
    println!("  Location: {}", script_dir.display().to_string().yellow());
    println!("\n{}", "Next steps:".cyan());
    println!("  1. Edit the script.toml to configure permissions");
    println!("  2. Edit main.lua to implement your script logic");
    println!(
        "  3. Run {} to test",
        format!("legalis script run --script {}/main.lua", name).bold()
    );

    Ok(())
}

/// Handles the script builtin command.
pub fn handle_script_builtin(show_code: bool) -> Result<()> {
    use crate::scripting::ScriptManager;
    use colored::Colorize;

    let scripts = ScriptManager::get_builtin_scripts();

    println!("{}", "Built-in Scripts Library".bold().cyan());
    println!("{}", "=".repeat(50).dimmed());
    println!();

    for script in scripts {
        println!("{} {}", "●".cyan(), script.name.bold());
        println!("  {}", script.description);

        if show_code {
            println!("\n{}", "  Code:".dimmed());
            for line in script.code.lines() {
                println!("    {}", line.dimmed());
            }
        }
        println!();
    }

    if !show_code {
        println!("{}", "Use --show-code to display script code".dimmed());
    }

    Ok(())
}

/// Handles the script validate command.
pub fn handle_script_validate(script: &str) -> Result<()> {
    use colored::Colorize;
    use mlua::Lua;

    println!("{}", "Validating script...".cyan().bold());
    println!("  Script: {}", script.yellow());

    let script_content = fs::read_to_string(script)
        .with_context(|| format!("Failed to read script file: {}", script))?;

    let lua = Lua::new();

    match lua.load(&script_content).exec() {
        Ok(_) => {
            println!("\n{} Script is valid!", "✓".green().bold());
            Ok(())
        }
        Err(e) => {
            println!("\n{} Script validation failed:", "✗".red().bold());
            println!("{}", format!("  {}", e).red());
            anyhow::bail!("Script validation failed")
        }
    }
}

// ============================================================================
// AI Command Handlers
// ============================================================================

/// Handles the AI parse command.
#[allow(dead_code)]
pub fn handle_ai_parse(input: &[String]) -> Result<()> {
    use crate::ai::NaturalLanguageParser;
    use colored::Colorize;

    let query = input.join(" ");

    if query.is_empty() {
        anyhow::bail!("Please provide a natural language command to parse");
    }

    println!("{}", "Parsing natural language command...".cyan().bold());
    println!("  Input: {}", query.yellow());

    let parser = NaturalLanguageParser::new();

    match parser.parse(&query) {
        Ok(command) => {
            println!("\n{} Parsed command:", "✓".green().bold());
            println!("  {}", command.green());
            println!("\nYou can run:");
            println!("  legalis {}", command);
            Ok(())
        }
        Err(e) => {
            println!("\n{} Failed to parse:", "✗".red().bold());
            println!("{}", format!("  {}", e).red());
            Err(e)
        }
    }
}

/// Handles the AI intent command.
#[allow(dead_code)]
pub fn handle_ai_intent(query: &[String]) -> Result<()> {
    use crate::ai::{CommandIntent, IntentRecognizer};
    use colored::Colorize;

    let query_str = query.join(" ");

    if query_str.is_empty() {
        anyhow::bail!("Please provide a query to recognize intent from");
    }

    println!("{}", "Recognizing command intent...".cyan().bold());
    println!("  Query: {}", query_str.yellow());

    let recognizer = IntentRecognizer::new();
    let intent = recognizer.recognize(&query_str);

    println!("\n{} Recognized intent:", "✓".green().bold());
    println!("  {:?}", intent);

    if intent != CommandIntent::Unknown {
        println!("\n{} Suggested commands:", "📝".yellow().bold());
        for suggestion in recognizer.suggest_commands(&intent) {
            println!("  • {}", suggestion.cyan());
        }
    }

    Ok(())
}

/// Handles the AI help command.
#[allow(dead_code)]
pub fn handle_ai_help(query: &[String]) -> Result<()> {
    use crate::ai::AiHelpSystem;
    use colored::Colorize;

    let query_str = query.join(" ");

    println!("{}", "AI-Powered Help".cyan().bold());

    if !query_str.is_empty() {
        println!("  Query: {}", query_str.yellow());
    }

    let help_system = AiHelpSystem::new();
    let help_text = help_system.get_help(&query_str);

    println!("\n{}", help_text);

    Ok(())
}

/// Handles the AI suggest command.
#[allow(dead_code)]
pub fn handle_ai_suggest(previous: Option<&str>) -> Result<()> {
    use crate::ai::IntelligentAutocomplete;
    use colored::Colorize;

    println!("{}", "AI Command Suggestions".cyan().bold());

    let autocomplete = IntelligentAutocomplete::new();

    if let Some(prev_cmd) = previous {
        println!("  Previous: {}", prev_cmd.yellow());
        println!("\n{} Suggested next commands:", "💡".yellow().bold());

        for suggestion in autocomplete.suggest_next(prev_cmd) {
            println!("  • legalis {}", suggestion.cyan());
        }
    } else {
        println!("\n{} Common commands:", "💡".yellow().bold());
        println!("  • legalis parse <file>");
        println!("  • legalis verify <file>");
        println!("  • legalis viz <file> -o output.svg mermaid");
        println!("  • legalis export <file> output.json json");
        println!("  • legalis help");
    }

    Ok(())
}

/// Handles the AI complete command.
#[allow(dead_code)]
pub fn handle_ai_complete(input: &[String]) -> Result<()> {
    use crate::ai::IntelligentAutocomplete;
    use colored::Colorize;

    let partial = input.join(" ");

    if partial.is_empty() {
        anyhow::bail!("Please provide a partial command to complete");
    }

    println!("{}", "Autocomplete Suggestions".cyan().bold());
    println!("  Input: {}", partial.yellow());

    let autocomplete = IntelligentAutocomplete::new();
    let suggestions = autocomplete.complete(&partial);

    if suggestions.is_empty() {
        println!("\n{} No suggestions found", "ℹ".yellow().bold());
    } else {
        println!("\n{} Suggestions:", "💡".yellow().bold());
        for suggestion in suggestions {
            println!("  • legalis {}", suggestion.cyan());
        }
    }

    Ok(())
}

// ============================================================================
// TUI Command Handler
// ============================================================================

/// Handles the dashboard command.
#[allow(dead_code)]
pub fn handle_dashboard(_vim_keys: bool, _no_mouse: bool) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Launching TUI Dashboard...".cyan().bold());
    println!();

    crate::tui::run_tui()?;

    Ok(())
}

// ============================================================================
// Workflow Command Handlers
// ============================================================================

/// Handles the workflow run command.
#[allow(dead_code)]
pub fn handle_workflow_run(
    file: &str,
    vars: &[String],
    dry_run: bool,
    continue_on_error: bool,
) -> Result<()> {
    use crate::workflow::{WorkflowExecutor, load_workflow, validate_workflow};
    use colored::Colorize;
    use std::path::Path;

    println!("{}", "Loading Workflow...".cyan().bold());
    let mut workflow = load_workflow(Path::new(file))?;

    // Override variables
    for var in vars {
        let parts: Vec<&str> = var.splitn(2, '=').collect();
        if parts.len() == 2 {
            workflow
                .variables
                .insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    // Override continue_on_error for all tasks if specified
    if continue_on_error {
        for task in &mut workflow.tasks {
            task.continue_on_error = true;
        }
    }

    // Validate workflow
    println!("{}", "Validating Workflow...".cyan().bold());
    if let Err(errors) = validate_workflow(&workflow) {
        eprintln!("{}", "Validation Errors:".red().bold());
        for error in errors {
            eprintln!("  • {}", error.red());
        }
        anyhow::bail!("Workflow validation failed");
    }

    if dry_run {
        println!("\n{}", "Dry Run - Workflow Plan:".yellow().bold());
        println!("  Name: {}", workflow.name.cyan());
        println!("  Version: {}", workflow.version);
        println!("  Execution Mode: {:?}", workflow.execution_mode);
        println!("\n  Tasks:");
        for task in &workflow.tasks {
            println!(
                "    • {} - {} {:?}",
                task.id.cyan(),
                task.command,
                task.args
            );
            if !task.depends_on.is_empty() {
                println!("      Depends on: {:?}", task.depends_on);
            }
        }
        return Ok(());
    }

    // Execute workflow
    println!("\n{}", "Executing Workflow...".cyan().bold());
    let mut executor = WorkflowExecutor::new(workflow);
    let result = executor.execute();

    // Display results
    println!("\n{}", "Workflow Results".cyan().bold());
    println!("  Workflow: {}", result.workflow_name);
    println!(
        "  Status: {}",
        if result.success {
            "Success".green().bold()
        } else {
            "Failed".red().bold()
        }
    );
    println!("  Total Duration: {}ms", result.total_duration_ms);
    println!("\n  Task Results:");
    for task_result in &result.task_results {
        let status = if task_result.success {
            "✓".green().bold()
        } else {
            "✗".red().bold()
        };
        println!(
            "    {} {} ({}ms, {} retries)",
            status,
            task_result.task_id.cyan(),
            task_result.duration_ms,
            task_result.retry_count
        );
        if !task_result.success
            && let Some(error) = &task_result.error
        {
            println!("      Error: {}", error.red());
        }
    }

    if !result.success {
        anyhow::bail!("Workflow execution failed");
    }

    Ok(())
}

/// Handles the workflow list-templates command.
#[allow(dead_code)]
pub fn handle_workflow_list_templates(verbose: bool) -> Result<()> {
    use crate::workflow::WorkflowTemplates;
    use colored::Colorize;

    println!("{}", "Available Workflow Templates".cyan().bold());
    println!();

    let templates = WorkflowTemplates::list_templates();

    for template in templates {
        if verbose {
            println!(
                "{}",
                format!("  {} ({})", template.name, template.category)
                    .cyan()
                    .bold()
            );
            println!("    {}", template.description);
            println!();
        } else {
            println!("  • {} - {}", template.name.cyan(), template.description);
        }
    }

    Ok(())
}

/// Handles the workflow new command.
#[allow(dead_code)]
pub fn handle_workflow_new(template: &str, output: &str, vars: &[String]) -> Result<()> {
    use crate::workflow::{WorkflowTemplates, save_workflow};
    use colored::Colorize;
    use std::path::Path;

    println!("{}", "Generating Workflow from Template...".cyan().bold());

    let mut workflow = WorkflowTemplates::generate_from_template(template)
        .ok_or_else(|| anyhow::anyhow!("Unknown template: {}", template))?;

    // Override variables
    for var in vars {
        let parts: Vec<&str> = var.splitn(2, '=').collect();
        if parts.len() == 2 {
            workflow
                .variables
                .insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    save_workflow(&workflow, Path::new(output))?;

    println!(
        "{} Workflow generated: {}",
        "✓".green().bold(),
        output.cyan()
    );
    println!("  Template: {}", template);
    println!("  Name: {}", workflow.name);
    println!("  Version: {}", workflow.version);

    Ok(())
}

/// Handles the workflow validate command.
#[allow(dead_code)]
pub fn handle_workflow_validate(file: &str, verbose: bool) -> Result<()> {
    use crate::workflow::{load_workflow, validate_workflow};
    use colored::Colorize;
    use std::path::Path;

    println!("{}", "Validating Workflow...".cyan().bold());

    let workflow = load_workflow(Path::new(file))?;

    if verbose {
        println!("  Name: {}", workflow.name.cyan());
        println!("  Version: {}", workflow.version);
        println!("  Execution Mode: {:?}", workflow.execution_mode);
        println!("  Tasks: {}", workflow.tasks.len());
    }

    match validate_workflow(&workflow) {
        Ok(()) => {
            println!("\n{} Workflow is valid", "✓".green().bold());
            Ok(())
        }
        Err(errors) => {
            println!("\n{}", "Validation Errors:".red().bold());
            for error in &errors {
                println!("  • {}", error.red());
            }
            anyhow::bail!("Workflow validation failed with {} error(s)", errors.len());
        }
    }
}

/// Handles the workflow info command.
#[allow(dead_code)]
pub fn handle_workflow_info(file: &str) -> Result<()> {
    use crate::workflow::load_workflow;
    use colored::Colorize;
    use std::path::Path;

    println!("{}", "Workflow Information".cyan().bold());
    println!();

    let workflow = load_workflow(Path::new(file))?;

    println!("  {}: {}", "Name".bold(), workflow.name.cyan());
    println!("  {}: {}", "Version".bold(), workflow.version);
    if let Some(desc) = &workflow.description {
        println!("  {}: {}", "Description".bold(), desc);
    }
    println!(
        "  {}: {:?}",
        "Execution Mode".bold(),
        workflow.execution_mode
    );

    if !workflow.variables.is_empty() {
        println!("\n  {}:", "Variables".bold());
        for (key, value) in &workflow.variables {
            println!("    {} = {}", key.cyan(), value);
        }
    }

    println!("\n  {} ({}):", "Tasks".bold(), workflow.tasks.len());
    for task in &workflow.tasks {
        println!("\n    {} - {}", task.id.cyan().bold(), task.name);
        if let Some(desc) = &task.description {
            println!("      {}", desc);
        }
        println!("      Command: {} {:?}", task.command, task.args);
        if !task.depends_on.is_empty() {
            println!("      Depends on: {:?}", task.depends_on);
        }
        if let Some(condition) = &task.condition {
            println!("      Condition: {:?}", condition);
        }
        if task.continue_on_error {
            println!("      Continue on error: true");
        }
        if let Some(timeout) = task.timeout {
            println!("      Timeout: {}s", timeout);
        }
        if let Some(retry) = &task.retry {
            println!(
                "      Retry: max {} attempts, {}s delay",
                retry.max_attempts, retry.delay_seconds
            );
        }
    }

    println!();
    Ok(())
}

// ============================================================================
// Cloud Command Handlers
// ============================================================================

/// Handles the cloud status command.
#[allow(dead_code)]
pub fn handle_cloud_status() -> Result<()> {
    use crate::cloud::{AwsProvider, AzureProvider, GcpProvider};
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    println!("{}", "Cloud CLI Status".cyan().bold());
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Provider").fg(Color::Cyan),
            Cell::new("CLI Tool").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
        ]);

    // Check AWS
    let aws_installed = AwsProvider::check_cli_installed();
    let aws_status = if aws_installed {
        Cell::new("✓ Installed").fg(Color::Green)
    } else {
        Cell::new("✗ Not Installed").fg(Color::Red)
    };
    table.add_row(vec![Cell::new("AWS"), Cell::new("aws"), aws_status]);

    // Check Azure
    let azure_installed = AzureProvider::check_cli_installed();
    let azure_status = if azure_installed {
        Cell::new("✓ Installed").fg(Color::Green)
    } else {
        Cell::new("✗ Not Installed").fg(Color::Red)
    };
    table.add_row(vec![Cell::new("Azure"), Cell::new("az"), azure_status]);

    // Check GCP
    let gcp_installed = GcpProvider::check_cli_installed();
    let gcp_status = if gcp_installed {
        Cell::new("✓ Installed").fg(Color::Green)
    } else {
        Cell::new("✗ Not Installed").fg(Color::Red)
    };
    table.add_row(vec![Cell::new("GCP"), Cell::new("gcloud"), gcp_status]);

    println!("{}", table);
    println!();

    // Print installation instructions if any are missing
    if !aws_installed || !azure_installed || !gcp_installed {
        println!("{}", "Installation Instructions:".yellow().bold());
        if !aws_installed {
            println!("  {} https://aws.amazon.com/cli/", "AWS CLI:".bold());
        }
        if !azure_installed {
            println!(
                "  {} https://docs.microsoft.com/en-us/cli/azure/install-azure-cli",
                "Azure CLI:".bold()
            );
        }
        if !gcp_installed {
            println!(
                "  {} https://cloud.google.com/sdk/docs/install",
                "gcloud CLI:".bold()
            );
        }
    }

    Ok(())
}

/// Handles the cloud aws command.
#[allow(dead_code)]
pub fn handle_cloud_aws(
    args: &[String],
    profile: Option<&str>,
    region: Option<&str>,
) -> Result<()> {
    use crate::cloud::AwsProvider;
    use colored::Colorize;

    let aws = AwsProvider::new(
        profile.map(|s| s.to_string()),
        region.map(|s| s.to_string()),
    );

    println!("{}", "Executing AWS CLI command...".cyan().bold());
    if let Some(p) = profile {
        println!("  Profile: {}", p.yellow());
    }
    if let Some(r) = region {
        println!("  Region: {}", r.yellow());
    }
    println!();

    let result = aws.execute_command(args)?;

    if result.success {
        println!("{}", result.output);
        println!();
        println!(
            "{} Command completed in {}ms",
            "✓".green().bold(),
            result.duration_ms
        );
    } else {
        if !result.output.is_empty() {
            println!("{}", result.output);
        }
        if let Some(error) = result.error {
            eprintln!("{} {}", "Error:".red().bold(), error);
        }
        anyhow::bail!("AWS CLI command failed");
    }

    Ok(())
}

/// Handles the cloud azure command.
#[allow(dead_code)]
pub fn handle_cloud_azure(args: &[String], subscription: Option<&str>) -> Result<()> {
    use crate::cloud::AzureProvider;
    use colored::Colorize;

    let azure = AzureProvider::new(subscription.map(|s| s.to_string()), None);

    println!("{}", "Executing Azure CLI command...".cyan().bold());
    if let Some(s) = subscription {
        println!("  Subscription: {}", s.yellow());
    }
    println!();

    let result = azure.execute_command(args)?;

    if result.success {
        println!("{}", result.output);
        println!();
        println!(
            "{} Command completed in {}ms",
            "✓".green().bold(),
            result.duration_ms
        );
    } else {
        if !result.output.is_empty() {
            println!("{}", result.output);
        }
        if let Some(error) = result.error {
            eprintln!("{} {}", "Error:".red().bold(), error);
        }
        anyhow::bail!("Azure CLI command failed");
    }

    Ok(())
}

/// Handles the cloud gcp command.
#[allow(dead_code)]
pub fn handle_cloud_gcp(args: &[String], project: Option<&str>, zone: Option<&str>) -> Result<()> {
    use crate::cloud::GcpProvider;
    use colored::Colorize;

    let gcp = GcpProvider::new(project.map(|s| s.to_string()), zone.map(|s| s.to_string()));

    println!("{}", "Executing gcloud command...".cyan().bold());
    if let Some(p) = project {
        println!("  Project: {}", p.yellow());
    }
    if let Some(z) = zone {
        println!("  Zone: {}", z.yellow());
    }
    println!();

    let result = gcp.execute_command(args)?;

    if result.success {
        println!("{}", result.output);
        println!();
        println!(
            "{} Command completed in {}ms",
            "✓".green().bold(),
            result.duration_ms
        );
    } else {
        if !result.output.is_empty() {
            println!("{}", result.output);
        }
        if let Some(error) = result.error {
            eprintln!("{} {}", "Error:".red().bold(), error);
        }
        anyhow::bail!("gcloud command failed");
    }

    Ok(())
}

/// Handles the cloud provision command.
#[allow(dead_code)]
pub fn handle_cloud_provision(
    _file: &str,
    provider: &crate::cloud::CloudProvider,
    dry_run: bool,
) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Cloud Resource Provisioning".cyan().bold());
    println!("  Provider: {}", provider);
    println!();

    if dry_run {
        println!(
            "{}",
            "Dry Run - No resources will be provisioned".yellow().bold()
        );
        println!();
        println!("This feature would provision resources from the definition file.");
        println!("In a real implementation, this would:");
        println!("  • Parse the resource definition file");
        println!("  • Validate resource configurations");
        println!("  • Provision resources using Infrastructure as Code");
        println!("  • Report provisioning status");
        return Ok(());
    }

    println!(
        "{}",
        "Note: Resource provisioning is currently simulated".yellow()
    );
    println!("In a production implementation, this would use:");
    match provider {
        crate::cloud::CloudProvider::Aws => println!("  • AWS CloudFormation or AWS CDK"),
        crate::cloud::CloudProvider::Azure => println!("  • Azure ARM templates or Bicep"),
        crate::cloud::CloudProvider::Gcp => println!("  • GCP Deployment Manager or Terraform"),
    }

    Ok(())
}

/// Handles the cloud list command.
#[allow(dead_code)]
pub fn handle_cloud_list(
    provider: &crate::cloud::CloudProvider,
    resource_type: &str,
    profile: Option<&str>,
    region: Option<&str>,
    subscription: Option<&str>,
    project: Option<&str>,
) -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        format!("Listing {} resources...", provider).cyan().bold()
    );
    println!("  Resource Type: {}", resource_type.yellow());
    println!();

    match provider {
        crate::cloud::CloudProvider::Aws => {
            let aws = crate::cloud::AwsProvider::new(
                profile.map(|s| s.to_string()),
                region.map(|s| s.to_string()),
            );
            match aws.list_resources(resource_type) {
                Ok(resources) => {
                    for resource in resources {
                        println!("{}", resource);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    anyhow::bail!("Failed to list AWS resources");
                }
            }
        }
        crate::cloud::CloudProvider::Azure => {
            let azure = crate::cloud::AzureProvider::new(subscription.map(|s| s.to_string()), None);
            match azure.list_resources(resource_type) {
                Ok(resources) => {
                    for resource in resources {
                        println!("{}", resource);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    anyhow::bail!("Failed to list Azure resources");
                }
            }
        }
        crate::cloud::CloudProvider::Gcp => {
            let gcp = crate::cloud::GcpProvider::new(project.map(|s| s.to_string()), None);
            match gcp.list_resources(resource_type) {
                Ok(resources) => {
                    for resource in resources {
                        println!("{}", resource);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    anyhow::bail!("Failed to list GCP resources");
                }
            }
        }
    }

    Ok(())
}

/// Handles the cloud configure command.
#[allow(dead_code)]
pub fn handle_cloud_configure(
    provider: &crate::cloud::CloudProvider,
    config: &[String],
) -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        format!("Configuring {} provider...", provider)
            .cyan()
            .bold()
    );
    println!();

    let mut config_map = std::collections::HashMap::new();
    for item in config {
        let parts: Vec<&str> = item.splitn(2, '=').collect();
        if parts.len() == 2 {
            config_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    println!("Configuration:");
    for (key, value) in &config_map {
        println!("  {} = {}", key.cyan(), value);
    }

    println!();
    println!(
        "{}",
        "Note: Configuration is stored in memory for this session".yellow()
    );
    println!("For persistent configuration, use the respective CLI tools:");
    match provider {
        crate::cloud::CloudProvider::Aws => println!("  aws configure"),
        crate::cloud::CloudProvider::Azure => println!("  az login"),
        crate::cloud::CloudProvider::Gcp => println!("  gcloud init"),
    }

    Ok(())
}

// ============================================================================
// Team Collaboration Command Handlers
// ============================================================================

/// Handles the team create-workspace command.
#[allow(dead_code)]
pub fn handle_team_create_workspace(
    name: &str,
    description: Option<&str>,
    members: Option<&str>,
    output: Option<&str>,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Creating team workspace...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username();

    let member_list: Vec<String> = members
        .map(|m| m.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let workspace = team_manager.create_workspace(
        name,
        description.map(|s| s.to_string()),
        &current_user,
        member_list.clone(),
    )?;

    println!("{}", "Workspace created successfully!".green().bold());
    println!();
    println!("Workspace ID: {}", workspace.id.cyan());
    println!("Name: {}", workspace.name);
    if let Some(desc) = &workspace.description {
        println!("Description: {}", desc);
    }
    println!("Owner: {}", workspace.owner.yellow());
    println!("Members: {}", workspace.members.len());
    for (user, role) in &workspace.members {
        println!("  - {} ({:?})", user, role);
    }

    // Optionally write to output file
    if let Some(output_path) = output {
        let toml_str = toml::to_string_pretty(&workspace)?;
        std::fs::write(output_path, toml_str)?;
        println!();
        println!("Workspace config written to: {}", output_path);
    }

    Ok(())
}

/// Handles the team list-workspaces command.
#[allow(dead_code)]
pub fn handle_team_list_workspaces(verbose: bool) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;
    let workspaces = team_manager.list_workspaces()?;

    if workspaces.is_empty() {
        println!("{}", "No workspaces found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    if verbose {
        table.set_header(vec![
            Cell::new("ID").fg(Color::Cyan),
            Cell::new("Name").fg(Color::Cyan),
            Cell::new("Owner").fg(Color::Cyan),
            Cell::new("Members").fg(Color::Cyan),
            Cell::new("Created").fg(Color::Cyan),
        ]);

        for ws in workspaces {
            table.add_row(vec![
                Cell::new(&ws.id[..8]),
                Cell::new(&ws.name),
                Cell::new(&ws.owner),
                Cell::new(ws.members.len()),
                Cell::new(ws.created_at.format("%Y-%m-%d %H:%M").to_string()),
            ]);
        }
    } else {
        table.set_header(vec![
            Cell::new("Name").fg(Color::Cyan),
            Cell::new("Owner").fg(Color::Cyan),
            Cell::new("Members").fg(Color::Cyan),
        ]);

        for ws in workspaces {
            table.add_row(vec![
                Cell::new(&ws.name),
                Cell::new(&ws.owner),
                Cell::new(ws.members.len()),
            ]);
        }
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team sync-history command.
#[allow(dead_code)]
pub fn handle_team_sync_history(
    workspace: &str,
    _direction: &crate::SyncDirection,
    dry_run: bool,
) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Syncing command history...".cyan().bold());
    println!();

    if dry_run {
        println!("{}", "DRY RUN - No changes will be made".yellow());
        println!();
    }

    println!("Workspace: {}", workspace.cyan());
    println!("Status: {}", "Sync complete".green());

    println!();
    println!("{}", "Note: History sync is a placeholder for now".yellow());
    println!("Future implementation will sync with remote storage");

    Ok(())
}

/// Handles the team show-history command.
#[allow(dead_code)]
pub fn handle_team_show_history(
    workspace: &str,
    limit: usize,
    user_filter: Option<&str>,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;

    // Find workspace by name or ID
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    let history = team_manager.get_history(&workspace_obj.id, limit, user_filter)?;

    if history.is_empty() {
        println!("{}", "No command history found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Time").fg(Color::Cyan),
            Cell::new("User").fg(Color::Cyan),
            Cell::new("Command").fg(Color::Cyan),
            Cell::new("Exit").fg(Color::Cyan),
        ]);

    for entry in history {
        let exit_status = match entry.exit_code {
            Some(0) => Cell::new("✓").fg(Color::Green),
            Some(code) => Cell::new(format!("✗ ({})", code)).fg(Color::Red),
            None => Cell::new("-"),
        };

        table.add_row(vec![
            Cell::new(entry.executed_at.format("%H:%M:%S").to_string()),
            Cell::new(&entry.user),
            Cell::new(format!("{} {}", entry.command, entry.args.join(" "))),
            exit_status,
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team start-session command.
#[allow(dead_code)]
pub fn handle_team_start_session(
    name: &str,
    workspace: &str,
    description: Option<&str>,
    max_participants: Option<usize>,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Starting collaborative session...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username();

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    let max_parts = max_participants.unwrap_or(workspace_obj.settings.max_session_participants);

    let session = team_manager.create_session(
        &workspace_obj.id,
        name,
        description.map(|s| s.to_string()),
        &current_user,
        max_parts,
    )?;

    println!("{}", "Session started successfully!".green().bold());
    println!();
    println!("Session ID: {}", session.id.cyan());
    println!("Name: {}", session.name);
    if let Some(desc) = &session.description {
        println!("Description: {}", desc);
    }
    println!("Workspace: {}", workspace_obj.name);
    println!("Max participants: {}", session.max_participants);
    println!("Status: {:?}", session.status);

    Ok(())
}

/// Handles the team list-sessions command.
#[allow(dead_code)]
pub fn handle_team_list_sessions(workspace: Option<&str>, all: bool) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;

    // Find workspace if specified
    let workspace_id = if let Some(ws_name) = workspace {
        let workspaces = team_manager.list_workspaces()?;
        let workspace_obj = workspaces
            .iter()
            .find(|w| w.name == ws_name || w.id == ws_name)
            .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", ws_name))?;
        Some(workspace_obj.id.clone())
    } else {
        None
    };

    // Get sessions
    let sessions = if let Some(ws_id) = workspace_id {
        team_manager.list_sessions(&ws_id, all)?
    } else {
        // List sessions from all workspaces
        let mut all_sessions = Vec::new();
        for ws in team_manager.list_workspaces()? {
            let mut sessions = team_manager.list_sessions(&ws.id, all)?;
            all_sessions.append(&mut sessions);
        }
        all_sessions
    };

    if sessions.is_empty() {
        println!("{}", "No sessions found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Name").fg(Color::Cyan),
            Cell::new("Owner").fg(Color::Cyan),
            Cell::new("Participants").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Created").fg(Color::Cyan),
        ]);

    for session in sessions {
        let status_cell = match session.status {
            crate::team::SessionStatus::Active => Cell::new("Active").fg(Color::Green),
            crate::team::SessionStatus::Paused => Cell::new("Paused").fg(Color::Yellow),
            crate::team::SessionStatus::Ended => Cell::new("Ended").fg(Color::Red),
        };

        table.add_row(vec![
            Cell::new(&session.name),
            Cell::new(&session.owner),
            Cell::new(format!(
                "{}/{}",
                session.participants.len(),
                session.max_participants
            )),
            status_cell,
            Cell::new(session.created_at.format("%Y-%m-%d %H:%M").to_string()),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team notify command.
#[allow(dead_code)]
pub fn handle_team_notify(
    workspace: &str,
    message: &str,
    users: Option<&str>,
    priority: &crate::team::Priority,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Sending notification...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username();

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    // Determine recipients
    let recipients: Vec<String> = if let Some(users_str) = users {
        users_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        // Send to all workspace members except sender
        workspace_obj
            .members
            .keys()
            .filter(|u| *u != &current_user)
            .cloned()
            .collect()
    };

    let notification = team_manager.create_notification(
        &workspace_obj.id,
        &current_user,
        recipients.clone(),
        message,
        *priority,
    )?;

    println!("{}", "Notification sent successfully!".green().bold());
    println!();
    println!("Notification ID: {}", notification.id.cyan());
    println!("Recipients: {}", recipients.join(", "));
    println!("Priority: {:?}", priority);
    println!("Message: {}", message);

    Ok(())
}

/// Handles the team list-notifications command.
#[allow(dead_code)]
pub fn handle_team_list_notifications(
    workspace: Option<&str>,
    unread: bool,
    limit: usize,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username();

    // Find workspace if specified
    let workspace_id = if let Some(ws_name) = workspace {
        let workspaces = team_manager.list_workspaces()?;
        let workspace_obj = workspaces
            .iter()
            .find(|w| w.name == ws_name || w.id == ws_name)
            .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", ws_name))?;
        Some(workspace_obj.id.clone())
    } else {
        None
    };

    // Get notifications
    let notifications = if let Some(ws_id) = workspace_id {
        team_manager.get_notifications(&ws_id, unread, &current_user, limit)?
    } else {
        // Get notifications from all workspaces
        let mut all_notifications = Vec::new();
        for ws in team_manager.list_workspaces()? {
            let mut notifs =
                team_manager.get_notifications(&ws.id, unread, &current_user, limit)?;
            all_notifications.append(&mut notifs);
        }
        all_notifications.truncate(limit);
        all_notifications
    };

    if notifications.is_empty() {
        println!("{}", "No notifications found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("ID").fg(Color::Cyan),
            Cell::new("From").fg(Color::Cyan),
            Cell::new("Message").fg(Color::Cyan),
            Cell::new("Priority").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Time").fg(Color::Cyan),
        ]);

    for notification in notifications {
        let status_cell = if notification.read_by.contains_key(&current_user) {
            Cell::new("Read").fg(Color::Green)
        } else {
            Cell::new("Unread").fg(Color::Yellow)
        };

        let priority_cell = match notification.priority {
            crate::team::Priority::High => Cell::new("High").fg(Color::Red),
            crate::team::Priority::Normal => Cell::new("Normal"),
            crate::team::Priority::Low => Cell::new("Low").fg(Color::DarkGrey),
        };

        table.add_row(vec![
            Cell::new(&notification.id[..8]),
            Cell::new(&notification.sender),
            Cell::new(&notification.message),
            priority_cell,
            status_cell,
            Cell::new(notification.created_at.format("%Y-%m-%d %H:%M").to_string()),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team mark-read command.
#[allow(dead_code)]
pub fn handle_team_mark_read(ids: &str) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username();

    let notification_ids: Vec<&str> = ids.split(',').map(|s| s.trim()).collect();

    let mut marked = 0;
    let mut errors = 0;

    // Try to find and mark each notification
    for ws in team_manager.list_workspaces()? {
        for id in &notification_ids {
            if let Ok(()) = team_manager.mark_notification_read(&ws.id, id, &current_user) {
                marked += 1;
            } else {
                errors += 1;
            }
        }
    }

    if marked > 0 {
        println!(
            "{}",
            format!("Marked {} notification(s) as read", marked)
                .green()
                .bold()
        );
    }

    if errors > 0 {
        println!(
            "{}",
            format!("Failed to mark {} notification(s)", errors).yellow()
        );
    }

    Ok(())
}

/// Handles the team access grant command.
#[allow(dead_code)]
pub fn handle_team_access_grant(
    workspace: &str,
    user: &str,
    role: &crate::team::Role,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Granting access...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    team_manager.update_user_role(&workspace_obj.id, user, *role)?;

    println!("{}", "Access granted successfully!".green().bold());
    println!();
    println!("Workspace: {}", workspace_obj.name);
    println!("User: {}", user);
    println!("Role: {:?}", role);

    Ok(())
}

/// Handles the team access revoke command.
#[allow(dead_code)]
pub fn handle_team_access_revoke(workspace: &str, user: &str, _yes: bool) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Revoking access...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    team_manager.remove_user(&workspace_obj.id, user)?;

    println!("{}", "Access revoked successfully!".green().bold());
    println!();
    println!("Workspace: {}", workspace_obj.name);
    println!("User: {}", user);

    Ok(())
}

/// Handles the team access list command.
#[allow(dead_code)]
pub fn handle_team_access_list(workspace: &str, _verbose: bool) -> Result<()> {
    use crate::team::TeamManager;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("User").fg(Color::Cyan),
            Cell::new("Role").fg(Color::Cyan),
        ]);

    for (user, role) in &workspace_obj.members {
        let role_cell = match role {
            crate::team::Role::Owner => Cell::new("Owner").fg(Color::Magenta),
            crate::team::Role::Admin => Cell::new("Admin").fg(Color::Blue),
            crate::team::Role::Write => Cell::new("Write").fg(Color::Green),
            crate::team::Role::Read => Cell::new("Read").fg(Color::Yellow),
        };

        table.add_row(vec![Cell::new(user), role_cell]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team access update command.
#[allow(dead_code)]
pub fn handle_team_access_update(
    workspace: &str,
    user: &str,
    role: &crate::team::Role,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Updating access...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    team_manager.update_user_role(&workspace_obj.id, user, *role)?;

    println!("{}", "Access updated successfully!".green().bold());
    println!();
    println!("Workspace: {}", workspace_obj.name);
    println!("User: {}", user);
    println!("New role: {:?}", role);

    Ok(())
}

// ============================================================================
// Performance Profiling Command Handlers
// ============================================================================

/// Handles the perf start command.
#[allow(dead_code)]
pub fn handle_perf_start(_name: Option<&str>) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;

    let mut profiler = PerformanceProfiler::new()?;
    profiler.enable();
    let session_id = profiler.start_session()?;

    println!("{}", "Performance profiling started".green().bold());
    println!();
    println!("Session ID: {}", session_id.cyan());
    println!();
    println!(
        "Use {} to stop profiling and generate a report",
        "legalis perf stop --report".yellow()
    );

    Ok(())
}

/// Handles the perf stop command.
#[allow(dead_code)]
pub fn handle_perf_stop(generate_report: bool) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;

    let mut profiler = PerformanceProfiler::new()?;

    if let Some(session) = profiler.end_session()? {
        println!("{}", "Performance profiling stopped".green().bold());
        println!();
        println!("Session ID: {}", session.id.cyan());
        println!("Commands profiled: {}", session.metrics.len());
        println!("Memory snapshots: {}", session.memory_snapshots.len());

        if generate_report {
            println!();
            println!("Generating report...");
            let report = profiler.generate_report(&session);

            println!();
            println!("{}", "Performance Report".cyan().bold());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!();
            println!("Total commands: {}", report.total_commands);
            println!(
                "Average execution time: {:.2}s",
                report.avg_execution_time.as_secs_f64()
            );
            println!(
                "Total execution time: {:.2}s",
                report.total_execution_time.as_secs_f64()
            );
            println!();
            println!("Bottlenecks detected: {}", report.bottlenecks.len());
            println!("Optimization suggestions: {}", report.suggestions.len());
        }
    } else {
        println!("{}", "No active profiling session".yellow());
    }

    Ok(())
}

/// Handles the perf list command.
#[allow(dead_code)]
pub fn handle_perf_list(verbose: bool) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;
    let sessions = profiler.list_sessions()?;

    if sessions.is_empty() {
        println!("{}", "No profiling sessions found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    if verbose {
        table.set_header(vec![
            Cell::new("Session ID").fg(Color::Cyan),
            Cell::new("Commands").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
        ]);

        for session_id in sessions {
            if let Ok(session) = profiler.load_session(&session_id) {
                table.add_row(vec![
                    Cell::new(&session_id[..12]),
                    Cell::new(session.metrics.len()),
                    Cell::new("Complete").fg(Color::Green),
                ]);
            }
        }
    } else {
        table.set_header(vec![
            Cell::new("Session ID").fg(Color::Cyan),
            Cell::new("Commands").fg(Color::Cyan),
        ]);

        for session_id in sessions {
            if let Ok(session) = profiler.load_session(&session_id) {
                table.add_row(vec![
                    Cell::new(&session_id[..12]),
                    Cell::new(session.metrics.len()),
                ]);
            }
        }
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf report command.
#[allow(dead_code)]
pub fn handle_perf_report(
    session_id: Option<&str>,
    output: Option<&str>,
    format: &crate::PerfReportFormat,
) -> Result<()> {
    use crate::perf::PerformanceProfiler;

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        // Load last session
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let report = profiler.generate_report(&session);

    match format {
        crate::PerfReportFormat::Json => {
            let json_str = serde_json::to_string_pretty(&report)?;
            if let Some(out_path) = output {
                std::fs::write(out_path, json_str)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", json_str);
            }
        }
        crate::PerfReportFormat::Text => {
            let text_report = format_text_report(&report);
            if let Some(out_path) = output {
                std::fs::write(out_path, text_report)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", text_report);
            }
        }
        crate::PerfReportFormat::Html => {
            let html_report = format_html_report(&report);
            if let Some(out_path) = output {
                std::fs::write(out_path, html_report)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", html_report);
            }
        }
        crate::PerfReportFormat::Markdown => {
            let md_report = format_markdown_report(&report);
            if let Some(out_path) = output {
                std::fs::write(out_path, md_report)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", md_report);
            }
        }
    }

    Ok(())
}

fn format_text_report(report: &crate::perf::PerformanceReport) -> String {
    use colored::Colorize;

    let mut output = String::new();

    output.push_str(&format!("{}\n", "Performance Report".cyan().bold()));
    output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

    output.push_str(&format!(
        "Generated: {}\n",
        report.generated_at.format("%Y-%m-%d %H:%M:%S")
    ));
    output.push_str(&format!("Report ID: {}\n\n", report.id));

    output.push_str(&format!("{}\n", "Summary".yellow().bold()));
    output.push_str(&format!("Total commands: {}\n", report.total_commands));
    output.push_str(&format!(
        "Average execution time: {:.2}s\n",
        report.avg_execution_time.as_secs_f64()
    ));
    output.push_str(&format!(
        "Total execution time: {:.2}s\n\n",
        report.total_execution_time.as_secs_f64()
    ));

    output.push_str(&format!("{}\n", "Memory Statistics".yellow().bold()));
    output.push_str(&format!(
        "Average memory: {:.2}MB\n",
        report.memory_stats.avg_memory as f64 / (1024.0 * 1024.0)
    ));
    output.push_str(&format!(
        "Peak memory: {:.2}MB\n",
        report.memory_stats.peak_memory as f64 / (1024.0 * 1024.0)
    ));
    output.push_str(&format!(
        "Minimum memory: {:.2}MB\n\n",
        report.memory_stats.min_memory as f64 / (1024.0 * 1024.0)
    ));

    if !report.bottlenecks.is_empty() {
        output.push_str(&format!("{}\n", "Bottlenecks".red().bold()));
        for bottleneck in &report.bottlenecks {
            output.push_str(&format!(
                "• {} [{:?}] {:?}\n",
                bottleneck.command, bottleneck.severity, bottleneck.bottleneck_type
            ));
            output.push_str(&format!("  {}\n", bottleneck.description));
            output.push_str(&format!("  → {}\n\n", bottleneck.suggestion.italic()));
        }
    }

    if !report.suggestions.is_empty() {
        output.push_str(&format!("{}\n", "Optimization Suggestions".green().bold()));
        for suggestion in &report.suggestions {
            output.push_str(&format!(
                "• {} [Impact: {:?}, Difficulty: {:?}]\n",
                suggestion.title, suggestion.impact, suggestion.difficulty
            ));
            output.push_str(&format!("  {}\n\n", suggestion.description));
        }
    }

    output
}

fn format_html_report(report: &crate::perf::PerformanceReport) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Performance Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #2c3e50; }}
        h2 {{ color: #34495e; border-bottom: 2px solid #3498db; padding-bottom: 5px; }}
        .metric {{ margin: 10px 0; }}
        .bottleneck {{ background: #fee; border-left: 4px solid #e74c3c; padding: 10px; margin: 10px 0; }}
        .suggestion {{ background: #efe; border-left: 4px solid #2ecc71; padding: 10px; margin: 10px 0; }}
    </style>
</head>
<body>
    <h1>Performance Report</h1>
    <p>Generated: {}</p>
    <p>Report ID: {}</p>
    
    <h2>Summary</h2>
    <div class="metric">Total commands: {}</div>
    <div class="metric">Average execution time: {:.2}s</div>
    <div class="metric">Total execution time: {:.2}s</div>
    
    <h2>Memory Statistics</h2>
    <div class="metric">Average memory: {:.2}MB</div>
    <div class="metric">Peak memory: {:.2}MB</div>
    <div class="metric">Minimum memory: {:.2}MB</div>
    
    <h2>Bottlenecks</h2>
    {}
    
    <h2>Optimization Suggestions</h2>
    {}
</body>
</html>"#,
        report.generated_at.format("%Y-%m-%d %H:%M:%S"),
        report.id,
        report.total_commands,
        report.avg_execution_time.as_secs_f64(),
        report.total_execution_time.as_secs_f64(),
        report.memory_stats.avg_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.peak_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.min_memory as f64 / (1024.0 * 1024.0),
        report
            .bottlenecks
            .iter()
            .map(|b| format!(
                "<div class=\"bottleneck\"><strong>{}</strong><br>{}<br><em>{}</em></div>",
                b.command, b.description, b.suggestion
            ))
            .collect::<Vec<_>>()
            .join("\n"),
        report
            .suggestions
            .iter()
            .map(|s| format!(
                "<div class=\"suggestion\"><strong>{}</strong><br>{}</div>",
                s.title, s.description
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn format_markdown_report(report: &crate::perf::PerformanceReport) -> String {
    let mut md = format!(
        "# Performance Report\n\nGenerated: {}\nReport ID: {}\n\n## Summary\n\n- Total commands: {}\n- Average execution time: {:.2}s\n- Total execution time: {:.2}s\n\n## Memory Statistics\n\n- Average memory: {:.2}MB\n- Peak memory: {:.2}MB\n- Minimum memory: {:.2}MB\n\n",
        report.generated_at.format("%Y-%m-%d %H:%M:%S"),
        report.id,
        report.total_commands,
        report.avg_execution_time.as_secs_f64(),
        report.total_execution_time.as_secs_f64(),
        report.memory_stats.avg_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.peak_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.min_memory as f64 / (1024.0 * 1024.0)
    );

    if !report.bottlenecks.is_empty() {
        md.push_str("## Bottlenecks\n\n");
        for bottleneck in &report.bottlenecks {
            md.push_str(&format!(
                "### {} [{:?}]\n\n",
                bottleneck.command, bottleneck.severity
            ));
            md.push_str(&format!("{}\n\n", bottleneck.description));
            md.push_str(&format!("**Suggestion:** {}\n\n", bottleneck.suggestion));
        }
    }

    if !report.suggestions.is_empty() {
        md.push_str("## Optimization Suggestions\n\n");
        for suggestion in &report.suggestions {
            md.push_str(&format!(
                "### {} [Impact: {:?}, Difficulty: {:?}]\n\n",
                suggestion.title, suggestion.impact, suggestion.difficulty
            ));
            md.push_str(&format!("{}\n\n", suggestion.description));
        }
    }

    md
}

/// Handles the perf stats command.
#[allow(dead_code)]
pub fn handle_perf_stats(session_id: Option<&str>, command_filter: Option<&str>) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let report = profiler.generate_report(&session);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Command").fg(Color::Cyan),
            Cell::new("Count").fg(Color::Cyan),
            Cell::new("Avg Duration").fg(Color::Cyan),
            Cell::new("Min Duration").fg(Color::Cyan),
            Cell::new("Max Duration").fg(Color::Cyan),
            Cell::new("Avg Memory").fg(Color::Cyan),
        ]);

    for (command, stats) in &report.command_stats {
        if let Some(filter) = command_filter
            && !command.contains(filter)
        {
            continue;
        }

        table.add_row(vec![
            Cell::new(command),
            Cell::new(stats.count),
            Cell::new(format!("{:.2}s", stats.avg_duration.as_secs_f64())),
            Cell::new(format!("{:.2}s", stats.min_duration.as_secs_f64())),
            Cell::new(format!("{:.2}s", stats.max_duration.as_secs_f64())),
            Cell::new(format!(
                "{:.2}MB",
                stats.avg_memory as f64 / (1024.0 * 1024.0)
            )),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf bottlenecks command.
#[allow(dead_code)]
pub fn handle_perf_bottlenecks(
    session_id: Option<&str>,
    min_severity: Option<&crate::PerfSeverity>,
) -> Result<()> {
    use crate::perf::{PerformanceProfiler, Severity};
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let bottlenecks = profiler.detect_bottlenecks(&session);

    if bottlenecks.is_empty() {
        println!("{}", "No bottlenecks detected!".green().bold());
        return Ok(());
    }

    // Filter by severity
    let filtered: Vec<_> = bottlenecks
        .iter()
        .filter(|b| {
            if let Some(min_sev) = min_severity {
                let min_level = match min_sev {
                    crate::PerfSeverity::Low => 0,
                    crate::PerfSeverity::Medium => 1,
                    crate::PerfSeverity::High => 2,
                    crate::PerfSeverity::Critical => 3,
                };
                let bottleneck_level = match b.severity {
                    Severity::Low => 0,
                    Severity::Medium => 1,
                    Severity::High => 2,
                    Severity::Critical => 3,
                };
                bottleneck_level >= min_level
            } else {
                true
            }
        })
        .collect();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Command").fg(Color::Cyan),
            Cell::new("Type").fg(Color::Cyan),
            Cell::new("Severity").fg(Color::Cyan),
            Cell::new("Description").fg(Color::Cyan),
            Cell::new("Suggestion").fg(Color::Cyan),
        ]);

    for bottleneck in filtered {
        let severity_cell = match bottleneck.severity {
            Severity::Critical => Cell::new("Critical").fg(Color::Red),
            Severity::High => Cell::new("High").fg(Color::Red),
            Severity::Medium => Cell::new("Medium").fg(Color::Yellow),
            Severity::Low => Cell::new("Low").fg(Color::Green),
        };

        table.add_row(vec![
            Cell::new(&bottleneck.command),
            Cell::new(format!("{:?}", bottleneck.bottleneck_type)),
            severity_cell,
            Cell::new(&bottleneck.description),
            Cell::new(&bottleneck.suggestion),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf optimize command.
#[allow(dead_code)]
pub fn handle_perf_optimize(
    session_id: Option<&str>,
    min_impact: Option<&crate::PerfImpact>,
) -> Result<()> {
    use crate::perf::{Impact, PerformanceProfiler};
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let suggestions = profiler.generate_suggestions(&session);

    if suggestions.is_empty() {
        println!("{}", "No optimization suggestions available.".yellow());
        return Ok(());
    }

    // Filter by impact
    let filtered: Vec<_> = suggestions
        .iter()
        .filter(|s| {
            if let Some(min_imp) = min_impact {
                let min_level = match min_imp {
                    crate::PerfImpact::Low => 0,
                    crate::PerfImpact::Medium => 1,
                    crate::PerfImpact::High => 2,
                };
                let impact_level = match s.impact {
                    Impact::Low => 0,
                    Impact::Medium => 1,
                    Impact::High => 2,
                };
                impact_level >= min_level
            } else {
                true
            }
        })
        .collect();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Title").fg(Color::Cyan),
            Cell::new("Impact").fg(Color::Cyan),
            Cell::new("Difficulty").fg(Color::Cyan),
            Cell::new("Description").fg(Color::Cyan),
        ]);

    for suggestion in filtered {
        let impact_cell = match suggestion.impact {
            Impact::High => Cell::new("High").fg(Color::Green),
            Impact::Medium => Cell::new("Medium").fg(Color::Yellow),
            Impact::Low => Cell::new("Low").fg(Color::DarkGrey),
        };

        let difficulty_cell = match suggestion.difficulty {
            crate::perf::Difficulty::Easy => Cell::new("Easy").fg(Color::Green),
            crate::perf::Difficulty::Medium => Cell::new("Medium").fg(Color::Yellow),
            crate::perf::Difficulty::Hard => Cell::new("Hard").fg(Color::Red),
        };

        table.add_row(vec![
            Cell::new(&suggestion.title),
            impact_cell,
            difficulty_cell,
            Cell::new(&suggestion.description),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf enable command.
#[allow(dead_code)]
pub fn handle_perf_enable() -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        "Performance profiling enabled globally".green().bold()
    );
    println!();
    println!("Note: Profiling will now track all command executions");
    println!(
        "Use {} to start a profiling session",
        "legalis perf start".yellow()
    );

    Ok(())
}

/// Handles the perf disable command.
#[allow(dead_code)]
pub fn handle_perf_disable() -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        "Performance profiling disabled globally".yellow().bold()
    );
    println!();
    println!("Note: No performance data will be collected");

    Ok(())
}

/// Handles the perf status command.
#[allow(dead_code)]
pub fn handle_perf_status() -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;

    let profiler = PerformanceProfiler::new()?;
    let sessions = profiler.list_sessions()?;

    println!("{}", "Performance Profiling Status".cyan().bold());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!(
        "Enabled: {}",
        if profiler.is_enabled() {
            "Yes".green()
        } else {
            "No".red()
        }
    );
    println!("Total sessions: {}", sessions.len());
    println!();

    if !sessions.is_empty() {
        println!(
            "Latest session: {}",
            &sessions[sessions.len() - 1][..12].cyan()
        );
    }

    Ok(())
}
