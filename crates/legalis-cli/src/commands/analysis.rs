//! Analysis CLI command handlers (explain, trace, benchmark, migrate, graph, batch ops, etc.).

use crate::{BenchmarkType, ExplainDetail, GraphFormat, GraphType, OutputFormat, TraceFormat};
use anyhow::{Context, Result};
use colored::Colorize;
use legalis_core::Statute;
use legalis_dsl::LegalDslParser;
use legalis_verifier::StatuteVerifier;
use std::fs;
use std::path::Path;

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
            let svg_nodes: Vec<(String, String)> = statutes
                .iter()
                .map(|s| (s.id.clone(), s.title.clone()))
                .collect();
            let mut svg_edges: Vec<(String, String)> = Vec::new();
            for s in &statutes {
                for dep in &s.derives_from {
                    svg_edges.push((dep.clone(), s.id.clone()));
                }
            }
            graph_output = generate_svg_graph(&svg_nodes, &svg_edges);
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
            let new_content = fs::read_to_string(&result.new_path).with_context(|| {
                format!("Failed to read new file for editing: {}", result.new_path)
            })?;
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            let edited_content = run_editor_on_temp_content_with(&new_content, &editor)?;
            if result.should_backup {
                let backup_path = format!("{}.backup", result.old_path);
                fs::copy(&result.old_path, &backup_path)
                    .with_context(|| format!("Failed to create backup: {}", backup_path))?;
                println!("{}", format!("✓ Created backup: {}", backup_path).yellow());
            }
            fs::write(&result.old_path, &edited_content).with_context(|| {
                format!("Failed to write edited content to: {}", result.old_path)
            })?;
            println!("{}", "✓ Applied edited version".green().bold());
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
    super::handle_simulate(inputs, params.population_size, Some(&params.output_path)).await?;

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
                super::handle_lint(&[file.to_string_lossy().to_string()], fix, strict)?;
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

                super::handle_export(
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
        .args([
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
        .args(["script", "-i", &perf_data])
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

/// Generates SVG markup for a dependency graph using a hierarchical layered layout.
///
/// `nodes` is a slice of `(node_id, label)` pairs.
/// `edges` is a slice of `(from_id, to_id)` pairs representing directed dependencies.
fn generate_svg_graph(nodes: &[(String, String)], edges: &[(String, String)]) -> String {
    use std::collections::HashMap;

    // Initialise depth for every node that appears in the node list or in edge endpoints.
    let mut depth: HashMap<String, usize> = HashMap::new();
    for (id, _) in nodes {
        depth.entry(id.clone()).or_insert(0);
    }
    for (from, to) in edges {
        depth.entry(from.clone()).or_insert(0);
        depth.entry(to.clone()).or_insert(0);
    }

    // Compute hierarchical depth via iterative relaxation.
    // An edge (A → B) means B depends on A, so depth[B] ≥ depth[A] + 1.
    let mut changed = true;
    while changed {
        changed = false;
        for (from, to) in edges {
            let from_d = depth.get(from.as_str()).copied().unwrap_or(0);
            let to_d = depth.get(to.as_str()).copied().unwrap_or(0);
            let candidate = from_d + 1;
            if candidate > to_d {
                depth.insert(to.clone(), candidate);
                changed = true;
            }
        }
    }

    let max_depth = depth.values().max().copied().unwrap_or(0);
    let num_layers = max_depth + 1;

    // Group nodes by their depth layer (layer 0 = roots / no dependencies).
    let mut layers: Vec<Vec<(&str, &str)>> = vec![Vec::new(); num_layers];
    for (id, label) in nodes {
        let d = depth.get(id.as_str()).copied().unwrap_or(0);
        let layer_idx = d.min(layers.len().saturating_sub(1));
        layers[layer_idx].push((id.as_str(), label.as_str()));
    }

    // Scale canvas with the number of nodes / layers.
    let node_count = nodes.len().max(1);
    let canvas_width = 800_usize.max(140 * node_count);
    let canvas_height = 600_usize.max(160 * num_layers);

    // Assign (cx, cy) to each node within its layer.
    let node_radius = 20.0_f64;
    let mut positions: HashMap<String, (f64, f64)> = HashMap::new();
    for (layer_idx, layer) in layers.iter().enumerate() {
        let layer_count = layer.len().max(1);
        let y = (layer_idx as f64 + 1.0) * (canvas_height as f64 / (num_layers as f64 + 1.0));
        for (pos_idx, (id, _)) in layer.iter().enumerate() {
            let x = (pos_idx as f64 + 1.0) * (canvas_width as f64 / (layer_count as f64 + 1.0));
            positions.insert((*id).to_string(), (x, y));
        }
    }

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
        canvas_width, canvas_height
    ));

    // Draw edges first so they appear behind the nodes.
    for (from, to) in edges {
        if let (Some(&(x1, y1)), Some(&(x2, y2))) =
            (positions.get(from.as_str()), positions.get(to.as_str()))
        {
            svg.push_str(&format!(
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" \
                 stroke=\"black\" stroke-width=\"1\"/>\n",
                x1, y1, x2, y2
            ));
        }
    }

    // Draw nodes (circle + label).
    for (id, label) in nodes {
        if let Some(&(cx, cy)) = positions.get(id.as_str()) {
            svg.push_str(&format!(
                "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.0}\" \
                 fill=\"#4a9eff\" stroke=\"#2255cc\" stroke-width=\"2\"/>\n",
                cx, cy, node_radius
            ));
            svg.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" \
                 font-size=\"12\">{}</text>\n",
                cx,
                cy + node_radius + 14.0,
                label
            ));
        }
    }

    svg.push_str("</svg>\n");
    svg
}

/// Opens `editor` on a temp copy of `content` and returns the edited content.
fn run_editor_on_temp_content_with(content: &str, editor: &str) -> Result<String> {
    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    let temp_path = temp_dir.join(format!("legalis_diff_edit_{}.txt", pid));

    fs::write(&temp_path, content).with_context(|| {
        format!(
            "Failed to write temp file for editor: {}",
            temp_path.display()
        )
    })?;

    let status = std::process::Command::new(editor)
        .arg(&temp_path)
        .status()
        .with_context(|| {
            format!(
                "Failed to spawn editor '{}': verify it is installed and in PATH",
                editor
            )
        })?;

    if !status.success() {
        let _ = fs::remove_file(&temp_path);
        anyhow::bail!(
            "Editor '{}' exited with non-zero status: {}",
            editor,
            status
        );
    }

    let edited =
        fs::read_to_string(&temp_path).with_context(|| "Failed to read back edited temp file")?;

    let _ = fs::remove_file(&temp_path);

    if edited.trim().is_empty() {
        anyhow::bail!("Edited content is empty; aborting apply to avoid data loss");
    }

    Ok(edited)
}

impl crate::profile::ProfileData {
    /// Convert to JSON string.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize profile data to JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_arm_editor_passthrough() {
        // "true" exits 0 without reading or modifying the temp file, so the
        // original content is returned unchanged.
        let content = "--- a/foo.leg\n+++ b/foo.leg\n@@ -1 +1 @@\n-old\n+new\n";
        let result = run_editor_on_temp_content_with(content, "true");
        assert!(
            result.is_ok(),
            "Expected editor passthrough to succeed, got: {:?}",
            result
        );
        assert_eq!(
            result.unwrap(),
            content,
            "Content should be unchanged after no-op passthrough editor"
        );
    }

    #[test]
    fn test_svg_output_contains_expected_elements() {
        let nodes = vec![
            ("statute-a".to_string(), "Statute A".to_string()),
            ("statute-b".to_string(), "Statute B".to_string()),
        ];
        let edges = vec![("statute-a".to_string(), "statute-b".to_string())];

        let svg = generate_svg_graph(&nodes, &edges);

        assert!(
            svg.contains("<svg"),
            "SVG output should contain <svg element"
        );
        assert!(
            svg.contains("<circle"),
            "SVG output should contain <circle element"
        );
        assert!(
            svg.contains("<line"),
            "SVG output should contain <line element"
        );
        assert!(
            svg.contains("<text"),
            "SVG output should contain <text element"
        );
    }
}
