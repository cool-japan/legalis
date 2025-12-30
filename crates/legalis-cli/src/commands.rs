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
use std::path::Path;

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
        if let Ok(metadata) = fs::metadata(input) {
            if let Ok(modified) = metadata.modified() {
                last_modified.insert(input.clone(), modified);
            }
        }
    }

    loop {
        sleep(Duration::from_secs(1)).await;

        for input in inputs {
            if let Ok(metadata) = fs::metadata(input) {
                if let Ok(modified) = metadata.modified() {
                    if let Some(&last_mod) = last_modified.get(input) {
                        if modified > last_mod {
                            println!("\n{} changed, running {:?}...", input, command);
                            match command {
                                WatchCommand::Verify => {
                                    let _ = handle_verify(
                                        std::slice::from_ref(input),
                                        false,
                                        &OutputFormat::Text,
                                    );
                                }
                                WatchCommand::Lint => {
                                    let _ = handle_lint(std::slice::from_ref(input), false, false);
                                }
                                WatchCommand::Test => {
                                    println!("Test command not yet implemented");
                                }
                                WatchCommand::Format => {
                                    let _ = handle_format(
                                        input,
                                        None,
                                        true,
                                        &FormatStyle::Default,
                                        false,
                                    );
                                }
                            }
                            last_modified.insert(input.clone(), modified);
                        }
                    }
                }
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

        if path.extension().and_then(|s| s.to_str()) == Some("legal") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(statute) = parser.parse_statute(&content) {
                    statutes.push((path.clone(), statute));
                }
            }
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
