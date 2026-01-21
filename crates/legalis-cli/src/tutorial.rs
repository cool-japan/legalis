//! Interactive tutorials for learning Legalis CLI features.

use anyhow::Result;
use rustyline::{DefaultEditor, error::ReadlineError};

/// Available tutorial topics.
#[derive(Debug, Clone)]
pub enum TutorialTopic {
    /// Introduction to Legalis and basic concepts
    Introduction,
    /// Parsing and validating DSL files
    ParsingBasics,
    /// Creating statutes from templates
    CreatingStatutes,
    /// Verification and testing
    Verification,
    /// Visualization techniques
    Visualization,
    /// Export formats
    Exporting,
    /// Registry usage
    RegistryUsage,
    /// Advanced features
    Advanced,
}

impl TutorialTopic {
    /// Get all available tutorials.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Introduction,
            Self::ParsingBasics,
            Self::CreatingStatutes,
            Self::Verification,
            Self::Visualization,
            Self::Exporting,
            Self::RegistryUsage,
            Self::Advanced,
        ]
    }

    /// Get the display name for a tutorial.
    pub fn display_name(&self) -> &str {
        match self {
            Self::Introduction => "Introduction to Legalis",
            Self::ParsingBasics => "Parsing & Validating DSL Files",
            Self::CreatingStatutes => "Creating Statutes from Templates",
            Self::Verification => "Verification & Testing",
            Self::Visualization => "Visualization Techniques",
            Self::Exporting => "Export Formats & Interoperability",
            Self::RegistryUsage => "Using the Statute Registry",
            Self::Advanced => "Advanced Features",
        }
    }

    /// Get the description for a tutorial.
    pub fn description(&self) -> &str {
        match self {
            Self::Introduction => "Learn about Legalis, legal DSL, and basic concepts",
            Self::ParsingBasics => "Parse and validate legal DSL files",
            Self::CreatingStatutes => "Use templates to create new statutes",
            Self::Verification => "Verify statutes for logical consistency",
            Self::Visualization => "Generate visualizations in various formats",
            Self::Exporting => "Export to different formats (JSON, YAML, Solidity)",
            Self::RegistryUsage => "Search, install, and publish statutes",
            Self::Advanced => "Simulation, porting, and LOD features",
        }
    }
}

/// Tutorial step with explanation and example command.
#[derive(Debug)]
struct TutorialStep {
    title: String,
    explanation: String,
    example_command: Option<String>,
    hint: Option<String>,
}

impl TutorialStep {
    fn new(title: &str, explanation: &str) -> Self {
        Self {
            title: title.to_string(),
            explanation: explanation.to_string(),
            example_command: None,
            hint: None,
        }
    }

    fn with_command(mut self, command: &str) -> Self {
        self.example_command = Some(command.to_string());
        self
    }

    fn with_hint(mut self, hint: &str) -> Self {
        self.hint = Some(hint.to_string());
        self
    }

    fn display(&self) {
        println!("\n{}", "=".repeat(60));
        println!("{}", self.title);
        println!("{}", "=".repeat(60));
        println!("\n{}", self.explanation);

        if let Some(cmd) = &self.example_command {
            println!("\nExample Command:");
            println!("  $ {}", cmd);
        }

        if let Some(hint) = &self.hint {
            println!("\nHint: {}", hint);
        }
        println!();
    }
}

/// Run a tutorial interactively.
pub fn run_tutorial(topic: Option<TutorialTopic>) -> Result<()> {
    let mut editor = DefaultEditor::new()?;

    let topic = match topic {
        Some(t) => t,
        None => select_tutorial(&mut editor)?,
    };

    let steps = get_tutorial_steps(&topic);

    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║        Welcome to Legalis Interactive Tutorial!       ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");
    println!("Tutorial: {}", topic.display_name());
    println!("Description: {}", topic.description());
    println!("\nThis tutorial has {} steps.", steps.len());

    if !prompt_yes_no(&mut editor, "Ready to begin? (y/n)", true)? {
        println!("Tutorial cancelled.");
        return Ok(());
    }

    let mut current_step = 0;
    while current_step < steps.len() {
        let step = &steps[current_step];
        step.display();

        if current_step < steps.len() - 1 {
            println!("\nOptions:");
            println!("  1. Continue to next step");
            println!("  2. Repeat this step");
            println!("  3. Exit tutorial");

            let choice = prompt_choice(&mut editor, "Select an option (1-3)", 1, 3)?;

            match choice {
                1 => current_step += 1,
                2 => continue,
                3 => {
                    println!("Tutorial exited. You can resume anytime with 'legalis tutorial'");
                    return Ok(());
                }
                _ => unreachable!(),
            }
        } else {
            current_step += 1;
        }
    }

    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║              Tutorial Completed!                      ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");
    println!(
        "Great job! You've completed the '{}' tutorial.",
        topic.display_name()
    );
    println!("\nTry running the commands yourself to practice!");

    if prompt_yes_no(
        &mut editor,
        "Would you like to start another tutorial? (y/n)",
        false,
    )? {
        run_tutorial(None)?;
    }

    Ok(())
}

/// Select a tutorial from the list.
fn select_tutorial(editor: &mut DefaultEditor) -> Result<TutorialTopic> {
    let tutorials = TutorialTopic::all();

    println!("\nAvailable Tutorials:");
    for (i, topic) in tutorials.iter().enumerate() {
        println!(
            "  {}. {} - {}",
            i + 1,
            topic.display_name(),
            topic.description()
        );
    }

    let selection = prompt_choice(
        editor,
        &format!("Select a tutorial (1-{})", tutorials.len()),
        1,
        tutorials.len(),
    )?;
    Ok(tutorials[selection - 1].clone())
}

/// Prompt for a yes/no answer.
fn prompt_yes_no(editor: &mut DefaultEditor, prompt: &str, default: bool) -> Result<bool> {
    let default_str = if default { "Y/n" } else { "y/N" };
    loop {
        match editor.readline(&format!("{} [{}]: ", prompt, default_str)) {
            Ok(line) => {
                let trimmed = line.trim().to_lowercase();
                if trimmed.is_empty() {
                    return Ok(default);
                }
                match trimmed.as_str() {
                    "y" | "yes" => return Ok(true),
                    "n" | "no" => return Ok(false),
                    _ => println!("Please enter 'y' or 'n'"),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                anyhow::bail!("Cancelled");
            }
            Err(err) => {
                anyhow::bail!("Error reading input: {}", err);
            }
        }
    }
}

/// Prompt for a numeric choice within a range.
fn prompt_choice(
    editor: &mut DefaultEditor,
    prompt: &str,
    min: usize,
    max: usize,
) -> Result<usize> {
    loop {
        match editor.readline(&format!("{}: ", prompt)) {
            Ok(line) => {
                if let Ok(num) = line.trim().parse::<usize>()
                    && num >= min
                    && num <= max
                {
                    return Ok(num);
                }
                println!("Please enter a number between {} and {}", min, max);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                anyhow::bail!("Cancelled");
            }
            Err(err) => {
                anyhow::bail!("Error reading input: {}", err);
            }
        }
    }
}

/// Get tutorial steps for a specific topic.
fn get_tutorial_steps(topic: &TutorialTopic) -> Vec<TutorialStep> {
    match topic {
        TutorialTopic::Introduction => introduction_tutorial(),
        TutorialTopic::ParsingBasics => parsing_basics_tutorial(),
        TutorialTopic::CreatingStatutes => creating_statutes_tutorial(),
        TutorialTopic::Verification => verification_tutorial(),
        TutorialTopic::Visualization => visualization_tutorial(),
        TutorialTopic::Exporting => exporting_tutorial(),
        TutorialTopic::RegistryUsage => registry_usage_tutorial(),
        TutorialTopic::Advanced => advanced_tutorial(),
    }
}

fn introduction_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: What is Legalis?",
            "Legalis is a Domain-Specific Language (DSL) for modeling legal statutes and regulations.\n\
             It allows you to:\n\
             - Define legal rules in a precise, machine-readable format\n\
             - Verify logical consistency\n\
             - Simulate application to populations\n\
             - Export to various formats including smart contracts",
        ),
        TutorialStep::new(
            "Step 2: Basic DSL Structure",
            "A Legalis statute consists of:\n\
             - Statute declaration with metadata (name, jurisdiction, effective dates)\n\
             - Conditions (Boolean expressions)\n\
             - Actions or implications\n\n\
             Example:\n\
             statute TaxCredit {\n\
                 jurisdiction: \"US\"\n\
                 condition: age >= 18 && income < 50000\n\
                 action: grant_credit(1000)\n\
             }",
        ),
        TutorialStep::new(
            "Step 3: CLI Commands Overview",
            "The Legalis CLI provides commands for:\n\
             - parse: Parse and validate DSL files\n\
             - verify: Check logical consistency\n\
             - viz: Generate visualizations\n\
             - export: Convert to other formats\n\
             - simulate: Apply to populations\n\
             - And many more!",
        )
        .with_hint("Run 'legalis --help' to see all available commands"),
        TutorialStep::new(
            "Step 4: Getting Help",
            "You can get help at any time:\n\
             - 'legalis --help' - Show all commands\n\
             - 'legalis <command> --help' - Show command-specific help\n\
             - 'legalis doctor' - Run diagnostics\n\
             - 'legalis tutorial' - Start a tutorial",
        )
        .with_command("legalis --help"),
    ]
}

fn parsing_basics_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: Creating a Simple Statute File",
            "First, let's create a simple statute file. Create a file named 'example.leg' with:\n\n\
             statute SimpleAge {\n\
                 jurisdiction: \"US\"\n\
                 condition: age >= 18\n\
             }",
        )
        .with_hint("Use your favorite text editor to create the file"),
        TutorialStep::new(
            "Step 2: Parsing the File",
            "The 'parse' command reads and validates your DSL file.\n\
             It checks for syntax errors and outputs the parsed structure.",
        )
        .with_command("legalis parse -i example.leg"),
        TutorialStep::new(
            "Step 3: Output Formats",
            "You can control the output format using the --format flag:\n\
             - text (default): Human-readable\n\
             - json: JSON format\n\
             - yaml: YAML format",
        )
        .with_command("legalis --format json parse -i example.leg"),
        TutorialStep::new(
            "Step 4: Saving Output",
            "Save the parsed output to a file using the -o flag.",
        )
        .with_command("legalis parse -i example.leg -o output.json --format json"),
    ]
}

fn creating_statutes_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: Statute Templates",
            "Legalis provides templates to help you create new statutes:\n\
             - basic: Simple age-based statute\n\
             - income: Income-based statute\n\
             - geographic: Regional/geographic statute\n\
             - temporal: Time-based statute\n\
             - complex: Multi-condition statute",
        ),
        TutorialStep::new(
            "Step 2: Creating from Template",
            "Use the 'new' command to create a statute from a template.",
        )
        .with_command("legalis new -n MyStatute -t income -o my_statute.leg"),
        TutorialStep::new(
            "Step 3: Interactive Mode",
            "You can use interactive mode to be guided through the process.",
        )
        .with_command("legalis --interactive new"),
        TutorialStep::new(
            "Step 4: Editing and Customizing",
            "After creating a statute from a template, edit it to fit your needs.\n\
             Remember to update:\n\
             - Jurisdiction\n\
             - Conditions\n\
             - Effective dates\n\
             - Metadata",
        ),
    ]
}

fn verification_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: Why Verify?",
            "Verification checks your statutes for:\n\
             - Logical consistency\n\
             - Contradictions\n\
             - Unreachable conditions\n\
             - Type errors",
        ),
        TutorialStep::new(
            "Step 2: Running Verification",
            "Use the 'verify' command to check your statutes.",
        )
        .with_command("legalis verify -i example.leg"),
        TutorialStep::new(
            "Step 3: Strict Mode",
            "Use --strict to fail on warnings, not just errors.",
        )
        .with_command("legalis verify -i example.leg --strict"),
        TutorialStep::new(
            "Step 4: Verifying Multiple Files",
            "You can verify multiple files at once.",
        )
        .with_command("legalis verify -i statute1.leg -i statute2.leg"),
    ]
}

fn visualization_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: Visualization Formats",
            "Legalis supports multiple visualization formats:\n\
             - mermaid: Mermaid diagrams (for GitHub/documentation)\n\
             - dot: GraphViz DOT format\n\
             - ascii: Terminal-friendly ASCII tree\n\
             - box: Terminal-friendly ASCII box",
        ),
        TutorialStep::new(
            "Step 2: Generating a Mermaid Diagram",
            "Mermaid diagrams can be rendered in GitHub, GitLab, and many editors.",
        )
        .with_command("legalis viz -i example.leg -o diagram.mmd --viz-format mermaid"),
        TutorialStep::new(
            "Step 3: ASCII Visualization",
            "For quick terminal viewing, use ASCII format.",
        )
        .with_command("legalis viz -i example.leg -o tree.txt --viz-format ascii"),
        TutorialStep::new(
            "Step 4: GraphViz Integration",
            "DOT format can be rendered with GraphViz tools.",
        )
        .with_command("legalis viz -i example.leg -o graph.dot --viz-format dot")
        .with_hint("Render with: dot -Tpng graph.dot -o graph.png"),
    ]
}

fn exporting_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: Export Formats",
            "Legalis can export to:\n\
             - json: JSON format\n\
             - yaml: YAML format\n\
             - solidity: Ethereum smart contract",
        ),
        TutorialStep::new(
            "Step 2: Exporting to JSON",
            "Export your statute to JSON for integration with other tools.",
        )
        .with_command("legalis export -i example.leg -o output.json --export-format json"),
        TutorialStep::new(
            "Step 3: Exporting to Solidity",
            "Generate a Solidity smart contract from your statute.",
        )
        .with_command("legalis export -i example.leg -o MyStatute.sol --export-format solidity"),
        TutorialStep::new(
            "Step 4: Import from Other Formats",
            "You can also import from other legal DSL formats like Catala, L4, etc.",
        )
        .with_command("legalis import -i statute.catala --from catala -o output.leg"),
    ]
}

fn registry_usage_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: What is the Registry?",
            "The Legalis registry is a collection of statutes that can be:\n\
             - Searched\n\
             - Installed\n\
             - Published\n\
             - Shared across projects",
        ),
        TutorialStep::new(
            "Step 2: Searching the Registry",
            "Search for statutes by name, jurisdiction, or tags.",
        )
        .with_command("legalis search -q \"tax\" -j US --limit 10"),
        TutorialStep::new(
            "Step 3: Installing a Statute",
            "Install a statute from the registry to your project.",
        )
        .with_command("legalis install -s statute-id -o ./statutes"),
        TutorialStep::new(
            "Step 4: Publishing a Statute",
            "Share your statutes by publishing to the registry.",
        )
        .with_command("legalis publish -i my_statute.leg -t tax -t income"),
        TutorialStep::new(
            "Step 5: Listing Installed Statutes",
            "See what statutes are installed in your project.",
        )
        .with_command("legalis list -d ./statutes --verbose"),
    ]
}

fn advanced_tutorial() -> Vec<TutorialStep> {
    vec![
        TutorialStep::new(
            "Step 1: Simulation",
            "Simulate how a statute applies to a population.\n\
             This generates synthetic data and applies your rules.",
        )
        .with_command("legalis simulate -i example.leg -p 1000 -o results.json"),
        TutorialStep::new(
            "Step 2: Porting to Other Jurisdictions",
            "Adapt a statute for a different jurisdiction.\n\
             Legalis analyzes compatibility and suggests changes.",
        )
        .with_command("legalis port -i us_statute.leg -t JP -o ported.json"),
        TutorialStep::new(
            "Step 3: Linked Open Data (LOD)",
            "Export to RDF/Turtle for semantic web integration.",
        )
        .with_command("legalis lod -i example.leg -o statute.ttl --rdf-format turtle"),
        TutorialStep::new(
            "Step 4: Complexity Analysis",
            "Analyze the complexity of your statutes.",
        )
        .with_command("legalis complexity -i example.leg"),
        TutorialStep::new(
            "Step 5: Interactive REPL",
            "Use the REPL for interactive exploration and testing.",
        )
        .with_command("legalis repl"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tutorials_have_steps() {
        for topic in TutorialTopic::all() {
            let steps = get_tutorial_steps(&topic);
            assert!(!steps.is_empty(), "{:?} tutorial has no steps", topic);
        }
    }

    #[test]
    fn test_tutorial_display_names() {
        for topic in TutorialTopic::all() {
            assert!(!topic.display_name().is_empty());
            assert!(!topic.description().is_empty());
        }
    }

    #[test]
    fn test_step_creation() {
        let step = TutorialStep::new("Test", "Description")
            .with_command("legalis test")
            .with_hint("This is a hint");

        assert_eq!(step.title, "Test");
        assert_eq!(step.explanation, "Description");
        assert_eq!(step.example_command, Some("legalis test".to_string()));
        assert_eq!(step.hint, Some("This is a hint".to_string()));
    }
}
