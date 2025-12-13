//! CLI command implementations.

use crate::{
    DiffFormat, ExportFormat, FormatStyle, ImportOutputFormat, LegalDslFormat, OutputFormat,
    PortFormat, RdfOutputFormat, VizFormat,
};
use anyhow::{Context, Result};
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
        OutputFormat::Text => format!("{:#?}", statute),
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
        OutputFormat::Yaml | OutputFormat::Text => {
            if result.passed {
                println!("✓ Verification passed");
            } else {
                println!("✗ Verification failed");
            }

            if !result.errors.is_empty() {
                println!("\nErrors:");
                for error in &result.errors {
                    println!("  ✗ {}", error);
                }
            }

            if !result.warnings.is_empty() {
                println!("\nWarnings:");
                for warning in &result.warnings {
                    println!("  ⚠ {}", warning);
                }
            }

            if !result.suggestions.is_empty() {
                println!("\nSuggestions:");
                for suggestion in &result.suggestions {
                    println!("  → {}", suggestion);
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
pub fn handle_init(path: &str) -> Result<()> {
    let project_path = Path::new(path);

    // Create directories
    fs::create_dir_all(project_path.join("statutes"))?;
    fs::create_dir_all(project_path.join("output"))?;

    // Create sample statute
    let sample_statute = r#"STATUTE sample-adult-rights: "Sample Adult Rights Act" {
    WHEN AGE >= 18
    THEN GRANT "Full legal capacity"
    DISCRETION "Consider individual maturity in exceptional cases"
}
"#;

    fs::write(project_path.join("statutes/sample.legal"), sample_statute)?;

    // Create config file
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

    fs::write(project_path.join("legalis.yaml"), config)?;

    println!("✓ Initialized Legalis project at: {}", path);
    println!("  Created:");
    println!("    - statutes/sample.legal");
    println!("    - legalis.yaml");
    println!("\nRun 'legalis verify -i statutes/sample.legal' to verify the sample statute.");

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
            println!("{}", legalis_diff::summarize(&diff));
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
    let statutes = parse_statutes(inputs)?;

    println!("Running simulation with {} entities...", population_size);

    let population = legalis_sim::PopulationBuilder::new()
        .generate_random(population_size)
        .build();

    let engine = legalis_sim::SimEngine::new(statutes, population);
    let metrics = engine.run_simulation().await;

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

    let converter = LegalConverter::new();

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

    let converter = LegalConverter::new();
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

    if inplace {
        fs::write(input, &formatted)
            .with_context(|| format!("Failed to write to file: {}", input))?;
        println!("Formatted: {}", input);
    } else if let Some(out_path) = output {
        fs::write(out_path, &formatted)
            .with_context(|| format!("Failed to write output file: {}", out_path))?;
        println!("Formatted output written to: {}", out_path);
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
