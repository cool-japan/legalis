//! Interactive REPL for the Legalis DSL parser.
//!
//! This provides an interactive shell where users can:
//! - Enter DSL statements and see parsed results
//! - Test syntax and see immediate feedback
//! - View warnings for deprecated syntax
//! - Inspect AST structures

use legalis_dsl::LegalDslParser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

const HELP_TEXT: &str = r#"
Legalis DSL REPL - Interactive Parser Shell

Commands:
  :help, :h         Show this help message
  :quit, :q         Exit the REPL
  :clear, :c        Clear the screen
  :format, :f       Toggle pretty-print format (compact/verbose)
  :json             Show AST as JSON
  :yaml             Show AST as YAML
  :warnings, :w     Show parsing warnings

Multi-line Input:
  Use backslash (\) at the end of a line to continue on the next line
  Or simply type incomplete statements - the REPL will accumulate input

Example DSL:
  STATUTE test: "Test Statute" {
      WHEN AGE >= 18
      THEN GRANT "Rights"
  }
"#;

fn main() -> Result<()> {
    println!("Legalis DSL REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Type ':help' for help, ':quit' to exit\n");

    let mut rl = DefaultEditor::new()?;
    let parser = LegalDslParser::new();
    let mut verbose_mode = false;
    let mut buffer = String::new();

    loop {
        let prompt = if buffer.is_empty() {
            "legalis> "
        } else {
            "      ... "
        };

        let readline = rl.readline(prompt);
        match readline {
            Ok(mut line) => {
                // Check for line continuation
                let continues = line.ends_with('\\');
                if continues {
                    line.pop(); // Remove backslash
                }

                // Accumulate input
                if !buffer.is_empty() {
                    buffer.push('\n');
                }
                buffer.push_str(&line);

                if continues {
                    continue;
                }

                // Save to history
                let _ = rl.add_history_entry(&buffer);

                // Handle commands
                let trimmed = buffer.trim();
                if trimmed.starts_with(':') {
                    match trimmed {
                        ":help" | ":h" => {
                            println!("{}", HELP_TEXT);
                        }
                        ":quit" | ":q" => {
                            println!("Goodbye!");
                            break;
                        }
                        ":clear" | ":c" => {
                            print!("\x1B[2J\x1B[1;1H");
                        }
                        ":format" | ":f" => {
                            verbose_mode = !verbose_mode;
                            println!(
                                "Format mode: {}",
                                if verbose_mode { "verbose" } else { "compact" }
                            );
                        }
                        ":json" => {
                            if let Some(input) = get_last_valid_input(&buffer) {
                                parse_and_show_json(&parser, input);
                            } else {
                                println!("No valid input to convert");
                            }
                        }
                        ":yaml" => {
                            if let Some(input) = get_last_valid_input(&buffer) {
                                parse_and_show_yaml(&parser, input);
                            } else {
                                println!("No valid input to convert");
                            }
                        }
                        ":warnings" | ":w" => {
                            show_warnings(&parser);
                        }
                        _ => {
                            println!("Unknown command: {}", trimmed);
                            println!("Type ':help' for available commands");
                        }
                    }
                    buffer.clear();
                    continue;
                }

                // Try to parse as a document
                parser.clear_warnings();
                match parser.parse_document(&buffer) {
                    Ok(doc) => {
                        if verbose_mode {
                            println!("Parsed successfully!");
                            println!("\nDocument:");
                            println!("  Imports: {}", doc.imports.len());
                            println!("  Statutes: {}", doc.statutes.len());

                            for statute in &doc.statutes {
                                println!("\nStatute: {} - \"{}\"", statute.id, statute.title);
                                println!("  Conditions: {}", statute.conditions.len());
                                println!("  Effects: {}", statute.effects.len());
                                if !statute.exceptions.is_empty() {
                                    println!("  Exceptions: {}", statute.exceptions.len());
                                }
                                if !statute.defaults.is_empty() {
                                    println!("  Defaults: {}", statute.defaults.len());
                                }
                                if !statute.requires.is_empty() {
                                    println!("  Requires: {}", statute.requires.len());
                                }
                                if !statute.supersedes.is_empty() {
                                    println!("  Supersedes: {}", statute.supersedes.len());
                                }
                            }
                        } else {
                            println!("OK - {} statute(s) parsed", doc.statutes.len());
                        }

                        // Show warnings if any
                        let warnings = parser.warnings();
                        if !warnings.is_empty() {
                            println!("\nWarnings ({})", warnings.len());
                            for warning in warnings {
                                println!("  {}", warning);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Parse error: {}", e);
                    }
                }

                buffer.clear();
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                buffer.clear();
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn get_last_valid_input(buffer: &str) -> Option<String> {
    if buffer.trim().is_empty() {
        None
    } else {
        Some(buffer.to_string())
    }
}

fn parse_and_show_json(parser: &LegalDslParser, input: String) {
    match parser.parse_document(&input) {
        Ok(doc) => match legalis_dsl::to_json(&doc) {
            Ok(json) => println!("{}", json),
            Err(e) => println!("JSON serialization error: {}", e),
        },
        Err(e) => println!("Parse error: {}", e),
    }
}

fn parse_and_show_yaml(parser: &LegalDslParser, input: String) {
    match parser.parse_document(&input) {
        Ok(doc) => match legalis_dsl::to_yaml(&doc) {
            Ok(yaml) => println!("{}", yaml),
            Err(e) => println!("YAML serialization error: {}", e),
        },
        Err(e) => println!("Parse error: {}", e),
    }
}

fn show_warnings(parser: &LegalDslParser) {
    let warnings = parser.warnings();
    if warnings.is_empty() {
        println!("No warnings");
    } else {
        println!("Warnings ({}):", warnings.len());
        for (i, warning) in warnings.iter().enumerate() {
            println!("  {}: {}", i + 1, warning);
        }
    }
}
