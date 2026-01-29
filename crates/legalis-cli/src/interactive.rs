//! Interactive mode for guided user input.

use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::path::PathBuf;

/// Interactive prompt for building commands.
pub struct InteractivePrompt {
    editor: DefaultEditor,
}

impl InteractivePrompt {
    /// Create a new interactive prompt.
    pub fn new() -> Result<Self> {
        let editor = DefaultEditor::new()?;
        Ok(Self { editor })
    }

    /// Prompt for a required string input.
    pub fn prompt_string(&mut self, prompt: &str) -> Result<String> {
        loop {
            match self.editor.readline(&format!("{}: ", prompt)) {
                Ok(line) => {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        return Ok(trimmed.to_string());
                    }
                    println!("Input cannot be empty. Please try again.");
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    anyhow::bail!("Interactive mode cancelled");
                }
                Err(err) => {
                    anyhow::bail!("Error reading input: {}", err);
                }
            }
        }
    }

    /// Prompt for an optional string input.
    pub fn prompt_optional_string(
        &mut self,
        prompt: &str,
        default: Option<&str>,
    ) -> Result<Option<String>> {
        let prompt_text = if let Some(def) = default {
            format!("{} [{}]: ", prompt, def)
        } else {
            format!("{} (optional): ", prompt)
        };

        match self.editor.readline(&prompt_text) {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    Ok(default.map(String::from))
                } else {
                    Ok(Some(trimmed.to_string()))
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                Ok(default.map(String::from))
            }
            Err(err) => {
                anyhow::bail!("Error reading input: {}", err);
            }
        }
    }

    /// Prompt for a boolean (yes/no) input.
    pub fn prompt_bool(&mut self, prompt: &str, default: bool) -> Result<bool> {
        let default_str = if default { "Y/n" } else { "y/N" };
        let prompt_text = format!("{} [{}]: ", prompt, default_str);

        match self.editor.readline(&prompt_text) {
            Ok(line) => {
                let trimmed = line.trim().to_lowercase();
                match trimmed.as_str() {
                    "" => Ok(default),
                    "y" | "yes" => Ok(true),
                    "n" | "no" => Ok(false),
                    _ => {
                        println!("Invalid input. Please enter 'y' or 'n'.");
                        self.prompt_bool(prompt, default)
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => Ok(default),
            Err(err) => {
                anyhow::bail!("Error reading input: {}", err);
            }
        }
    }

    /// Prompt for a number input.
    pub fn prompt_number<T>(&mut self, prompt: &str) -> Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        loop {
            match self.editor.readline(&format!("{}: ", prompt)) {
                Ok(line) => {
                    let trimmed = line.trim();
                    match trimmed.parse::<T>() {
                        Ok(num) => return Ok(num),
                        Err(e) => {
                            println!("Invalid number: {}. Please try again.", e);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    anyhow::bail!("Interactive mode cancelled");
                }
                Err(err) => {
                    anyhow::bail!("Error reading input: {}", err);
                }
            }
        }
    }

    /// Prompt for a file path with validation.
    pub fn prompt_file_path(&mut self, prompt: &str, must_exist: bool) -> Result<PathBuf> {
        loop {
            match self.editor.readline(&format!("{}: ", prompt)) {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        println!("Path cannot be empty. Please try again.");
                        continue;
                    }

                    let path = PathBuf::from(trimmed);

                    if must_exist && !path.exists() {
                        println!("File does not exist: {}. Please try again.", path.display());
                        continue;
                    }

                    return Ok(path);
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    anyhow::bail!("Interactive mode cancelled");
                }
                Err(err) => {
                    anyhow::bail!("Error reading input: {}", err);
                }
            }
        }
    }

    /// Prompt for a selection from a list of options.
    pub fn prompt_select(&mut self, prompt: &str, options: &[&str]) -> Result<usize> {
        println!("\n{}", prompt);
        for (i, option) in options.iter().enumerate() {
            println!("  {}. {}", i + 1, option);
        }

        loop {
            match self.editor.readline("Select an option: ") {
                Ok(line) => {
                    let trimmed = line.trim();
                    match trimmed.parse::<usize>() {
                        Ok(num) if num > 0 && num <= options.len() => {
                            return Ok(num - 1);
                        }
                        _ => {
                            println!(
                                "Invalid selection. Please enter a number between 1 and {}.",
                                options.len()
                            );
                        }
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    anyhow::bail!("Interactive mode cancelled");
                }
                Err(err) => {
                    anyhow::bail!("Error reading input: {}", err);
                }
            }
        }
    }

    /// Display a message to the user.
    pub fn display(&self, message: &str) {
        println!("{}", message);
    }

    /// Display a header.
    pub fn display_header(&self, header: &str) {
        println!("\n=== {} ===\n", header);
    }
}

impl Default for InteractivePrompt {
    fn default() -> Self {
        Self::new().expect("Failed to create interactive prompt")
    }
}

/// Interactive wizard for creating a new statute.
pub fn interactive_new_statute() -> Result<(String, String, Option<String>)> {
    let mut prompt = InteractivePrompt::new()?;

    prompt.display_header("Create New Statute");

    let name = prompt.prompt_string("Statute name")?;

    let templates = vec![
        "basic - Basic statute with age condition",
        "income - Income-based statute",
        "geographic - Geographic/regional statute",
        "temporal - Time-based statute with effective dates",
        "complex - Complex statute with multiple conditions",
    ];

    let template_idx = prompt.prompt_select("Select template", &templates)?;
    let template = templates[template_idx]
        .split(" - ")
        .next()
        .expect("Template string should have at least one part")
        .to_string();

    let output = prompt.prompt_optional_string("Output directory", Some("."))?;

    Ok((name, template, output))
}

/// Interactive wizard for verification.
pub fn interactive_verify() -> Result<(Vec<String>, bool)> {
    let mut prompt = InteractivePrompt::new()?;

    prompt.display_header("Verify Statutes");

    let mut files = Vec::new();

    loop {
        let file = prompt.prompt_file_path("Statute file (or press Ctrl+C when done)", true)?;
        files.push(file.to_string_lossy().to_string());

        if !prompt.prompt_bool("Add another file?", false)? {
            break;
        }
    }

    let strict = prompt.prompt_bool("Enable strict mode (fail on warnings)?", false)?;

    Ok((files, strict))
}

/// Interactive wizard for export.
pub fn interactive_export() -> Result<(String, String, String)> {
    let mut prompt = InteractivePrompt::new()?;

    prompt.display_header("Export Statute");

    let input = prompt
        .prompt_file_path("Input statute file", true)?
        .to_string_lossy()
        .to_string();

    let formats = vec![
        "json - JSON format",
        "yaml - YAML format",
        "solidity - Solidity smart contract",
    ];

    let format_idx = prompt.prompt_select("Select export format", &formats)?;
    let format = formats[format_idx]
        .split(" - ")
        .next()
        .expect("Format string should have at least one part")
        .to_string();

    let output = prompt.prompt_string("Output file path")?;

    Ok((input, output, format))
}

/// Advanced interactive statute builder wizard with comprehensive options.
pub fn interactive_statute_builder() -> Result<StatuteBuilderResult> {
    let mut prompt = InteractivePrompt::new()?;

    prompt.display_header("Interactive Statute Builder Wizard");
    prompt.display("This wizard will guide you through creating a comprehensive statute.");

    // Basic information
    let statute_id = prompt.prompt_string("Statute ID (e.g., 'TAX_001')")?;
    let title = prompt.prompt_string("Statute title")?;
    let jurisdiction = prompt.prompt_string("Jurisdiction (e.g., 'US-CA', 'JP', 'EU')")?;

    // Effective dates
    let has_effective_date = prompt.prompt_bool("Specify effective date?", true)?;
    let effective_from = if has_effective_date {
        Some(prompt.prompt_string("Effective from (YYYY-MM-DD)")?)
    } else {
        None
    };

    let has_expiry_date = prompt.prompt_bool("Specify expiry date?", false)?;
    let effective_until = if has_expiry_date {
        Some(prompt.prompt_string("Effective until (YYYY-MM-DD)")?)
    } else {
        None
    };

    // Conditions
    prompt.display_header("Define Conditions");
    let mut conditions = Vec::new();

    loop {
        let condition_type_options = vec![
            "age - Age-based condition (e.g., age >= 18)",
            "income - Income-based condition (e.g., income < 50000)",
            "geographic - Location-based condition",
            "temporal - Time-based condition",
            "boolean - Custom boolean condition",
        ];

        let cond_idx = prompt.prompt_select("Select condition type", &condition_type_options)?;
        let cond_type = condition_type_options[cond_idx]
            .split(" - ")
            .next()
            .expect("Condition type option string should have at least one part");

        let condition = match cond_type {
            "age" => {
                let operator =
                    prompt.prompt_select("Operator", &[">=", "<=", "==", "!=", ">", "<"])?;
                let value = prompt.prompt_number::<u32>("Age value")?;
                ConditionSpec {
                    cond_type: "age".to_string(),
                    operator: vec![">=", "<=", "==", "!=", ">", "<"][operator].to_string(),
                    value: value.to_string(),
                    description: None,
                }
            }
            "income" => {
                let operator =
                    prompt.prompt_select("Operator", &[">=", "<=", "==", "!=", ">", "<"])?;
                let value = prompt.prompt_number::<f64>("Income value")?;
                ConditionSpec {
                    cond_type: "income".to_string(),
                    operator: vec![">=", "<=", "==", "!=", ">", "<"][operator].to_string(),
                    value: value.to_string(),
                    description: None,
                }
            }
            "geographic" => {
                let location = prompt.prompt_string("Location/Region")?;
                ConditionSpec {
                    cond_type: "geographic".to_string(),
                    operator: "in".to_string(),
                    value: location,
                    description: None,
                }
            }
            "temporal" => {
                let date = prompt.prompt_string("Date (YYYY-MM-DD)")?;
                let operator = prompt.prompt_select("Operator", &["before", "after", "on"])?;
                ConditionSpec {
                    cond_type: "temporal".to_string(),
                    operator: vec!["before", "after", "on"][operator].to_string(),
                    value: date,
                    description: None,
                }
            }
            "boolean" => {
                let expression = prompt.prompt_string("Boolean expression")?;
                ConditionSpec {
                    cond_type: "boolean".to_string(),
                    operator: "eval".to_string(),
                    value: expression,
                    description: None,
                }
            }
            _ => continue,
        };

        conditions.push(condition);

        if !prompt.prompt_bool("Add another condition?", false)? {
            break;
        }
    }

    // Combine conditions
    let combine_operator = if conditions.len() > 1 {
        let op_idx = prompt.prompt_select(
            "How should conditions be combined?",
            &[
                "AND - All conditions must be true",
                "OR - At least one condition must be true",
            ],
        )?;
        if op_idx == 0 { "AND" } else { "OR" }
    } else {
        "AND"
    };

    // Outcome
    prompt.display_header("Define Outcome");
    let outcome_type_options = vec![
        "eligible - Applicant is eligible",
        "ineligible - Applicant is ineligible",
        "benefit - Grant a specific benefit",
        "penalty - Apply a penalty",
        "custom - Custom outcome",
    ];

    let outcome_idx = prompt.prompt_select("Select outcome type", &outcome_type_options)?;
    let outcome_type = outcome_type_options[outcome_idx]
        .split(" - ")
        .next()
        .expect("Outcome type option string should have at least one part")
        .to_string();

    let outcome_value = if outcome_type == "benefit" || outcome_type == "penalty" {
        Some(prompt.prompt_string("Benefit/Penalty description")?)
    } else if outcome_type == "custom" {
        Some(prompt.prompt_string("Custom outcome value")?)
    } else {
        None
    };

    // Output path
    let output_path = prompt
        .prompt_optional_string("Output file path", Some("statute.ldsl"))?
        .unwrap_or_else(|| "statute.ldsl".to_string());

    Ok(StatuteBuilderResult {
        statute_id,
        title,
        jurisdiction,
        effective_from,
        effective_until,
        conditions,
        combine_operator: combine_operator.to_string(),
        outcome_type,
        outcome_value,
        output_path,
    })
}

/// Result from the interactive statute builder.
#[derive(Debug)]
pub struct StatuteBuilderResult {
    pub statute_id: String,
    pub title: String,
    pub jurisdiction: String,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
    pub conditions: Vec<ConditionSpec>,
    pub combine_operator: String,
    pub outcome_type: String,
    pub outcome_value: Option<String>,
    pub output_path: String,
}

/// Specification for a condition in the statute builder.
#[derive(Debug)]
pub struct ConditionSpec {
    pub cond_type: String,
    pub operator: String,
    pub value: String,
    pub description: Option<String>,
}

/// Interactive diff viewer with accept/reject functionality.
pub fn interactive_diff_viewer(old_path: &str, new_path: &str) -> Result<DiffViewerResult> {
    let mut prompt = InteractivePrompt::new()?;

    prompt.display_header("Interactive Diff Viewer");
    prompt.display(&format!("Comparing: {} -> {}", old_path, new_path));

    // Read files
    let old_content = std::fs::read_to_string(old_path)?;
    let new_content = std::fs::read_to_string(new_path)?;

    // Show diff (simplified - in real implementation would use proper diff library)
    prompt.display("\n--- Changes ---");
    prompt.display(&format!("Old: {} chars", old_content.len()));
    prompt.display(&format!("New: {} chars", new_content.len()));

    // Ask for action
    let action_options = vec![
        "accept - Accept the new version",
        "reject - Keep the old version",
        "merge - Attempt automatic merge",
        "edit - Manual edit",
        "cancel - Cancel without changes",
    ];

    let action_idx = prompt.prompt_select("Select action", &action_options)?;
    let action = action_options[action_idx]
        .split(" - ")
        .next()
        .expect("Action option string should have at least one part")
        .to_string();

    let should_backup = if action == "accept" || action == "merge" {
        prompt.prompt_bool("Create backup of old version?", true)?
    } else {
        false
    };

    Ok(DiffViewerResult {
        action,
        should_backup,
        old_path: old_path.to_string(),
        new_path: new_path.to_string(),
    })
}

/// Result from the interactive diff viewer.
#[derive(Debug)]
pub struct DiffViewerResult {
    pub action: String,
    pub should_backup: bool,
    pub old_path: String,
    pub new_path: String,
}

/// Interactive simulation parameter tuning wizard.
pub fn interactive_simulation_tuning() -> Result<SimulationParams> {
    let mut prompt = InteractivePrompt::new()?;

    prompt.display_header("Interactive Simulation Parameter Tuning");
    prompt.display("Configure simulation parameters for your statute analysis.");

    let population_size = prompt.prompt_number::<usize>("Population size")?;

    let use_custom_distribution = prompt.prompt_bool("Use custom age distribution?", false)?;
    let age_distribution = if use_custom_distribution {
        let min_age = prompt.prompt_number::<u32>("Minimum age")?;
        let max_age = prompt.prompt_number::<u32>("Maximum age")?;
        Some((min_age, max_age))
    } else {
        None
    };

    let use_custom_income = prompt.prompt_bool("Use custom income distribution?", false)?;
    let income_distribution = if use_custom_income {
        let min_income = prompt.prompt_number::<f64>("Minimum income")?;
        let max_income = prompt.prompt_number::<f64>("Maximum income")?;
        Some((min_income, max_income))
    } else {
        None
    };

    let iterations = prompt.prompt_number::<usize>("Number of simulation iterations")?;

    let enable_logging = prompt.prompt_bool("Enable detailed logging?", false)?;

    let output_format_options = vec![
        "json - JSON format",
        "csv - CSV format",
        "html - HTML report",
        "text - Plain text",
    ];
    let format_idx = prompt.prompt_select("Output format", &output_format_options)?;
    let output_format = output_format_options[format_idx]
        .split(" - ")
        .next()
        .expect("Output format option string should have at least one part")
        .to_string();

    let output_path = prompt
        .prompt_optional_string("Output file path", Some("simulation_results.json"))?
        .unwrap_or_else(|| "simulation_results.json".to_string());

    Ok(SimulationParams {
        population_size,
        age_distribution,
        income_distribution,
        iterations,
        enable_logging,
        output_format,
        output_path,
    })
}

/// Simulation parameters from interactive tuning.
#[derive(Debug)]
pub struct SimulationParams {
    pub population_size: usize,
    pub age_distribution: Option<(u32, u32)>,
    pub income_distribution: Option<(f64, f64)>,
    pub iterations: usize,
    pub enable_logging: bool,
    pub output_format: String,
    pub output_path: String,
}

/// Interactive conflict resolution UI for statute conflicts.
pub fn interactive_conflict_resolution(
    conflicts: &[ConflictInfo],
) -> Result<Vec<ConflictResolution>> {
    let mut prompt = InteractivePrompt::new()?;
    let mut resolutions = Vec::new();

    prompt.display_header("Interactive Conflict Resolution");
    prompt.display(&format!(
        "Found {} conflict(s) to resolve.",
        conflicts.len()
    ));

    for (idx, conflict) in conflicts.iter().enumerate() {
        prompt.display(&format!(
            "\n--- Conflict {} of {} ---",
            idx + 1,
            conflicts.len()
        ));
        prompt.display(&format!("Type: {}", conflict.conflict_type));
        prompt.display(&format!("Description: {}", conflict.description));

        if let Some(ref details) = conflict.details {
            prompt.display(&format!("Details: {}", details));
        }

        let resolution_options = vec![
            "keep_first - Keep first version",
            "keep_second - Keep second version",
            "merge - Merge both versions",
            "custom - Provide custom resolution",
            "skip - Skip this conflict",
        ];

        let resolution_idx =
            prompt.prompt_select("How to resolve this conflict?", &resolution_options)?;
        let resolution_type = resolution_options[resolution_idx]
            .split(" - ")
            .next()
            .expect("Resolution option string should have at least one part")
            .to_string();

        let custom_value = if resolution_type == "custom" {
            Some(prompt.prompt_string("Enter custom resolution value")?)
        } else {
            None
        };

        resolutions.push(ConflictResolution {
            conflict_id: conflict.id.clone(),
            resolution_type,
            custom_value,
        });
    }

    Ok(resolutions)
}

/// Information about a conflict.
#[derive(Debug, Clone)]
pub struct ConflictInfo {
    pub id: String,
    pub conflict_type: String,
    pub description: String,
    pub details: Option<String>,
}

/// Resolution for a conflict.
#[derive(Debug)]
pub struct ConflictResolution {
    pub conflict_id: String,
    pub resolution_type: String,
    pub custom_value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_prompt_creation() {
        // Just test that we can create a prompt
        // Actual interactive tests would require user input
        assert!(InteractivePrompt::new().is_ok());
    }
}
