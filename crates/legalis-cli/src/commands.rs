//! CLI command implementations.

use crate::{ExportFormat, OutputFormat, VizFormat};
use anyhow::{Context, Result};
use legalis_dsl::LegalDslParser;
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
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                "passed": result.passed,
                "errors": result.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
                "warnings": result.warnings,
                "suggestions": result.suggestions
            }))?);
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
    };

    fs::write(output, &output_str)
        .with_context(|| format!("Failed to write output file: {}", output))?;

    println!("Visualization written to: {}", output);
    println!("Nodes: {}, Discretionary: {}", tree.node_count(), tree.discretionary_count());

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
            let generator = legalis_chain::ContractGenerator::new(
                legalis_chain::TargetPlatform::Solidity,
            );
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

    fs::write(
        project_path.join("statutes/sample.legal"),
        sample_statute,
    )?;

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
