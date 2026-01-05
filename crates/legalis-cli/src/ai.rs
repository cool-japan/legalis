//! AI-powered CLI features for natural language command parsing and intelligent assistance.
//!
//! This module provides:
//! - Natural language command parsing
//! - AI-suggested commands based on context
//! - Intelligent autocomplete
//! - AI-powered help system
//! - Command intent recognition

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the intent of a user's command.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommandIntent {
    /// Parse a statute file
    Parse,
    /// Verify a statute
    Verify,
    /// Visualize a statute
    Visualize,
    /// Export a statute
    Export,
    /// Serve API
    Serve,
    /// Initialize a project
    Initialize,
    /// Compare statutes
    Diff,
    /// Simulate statute execution
    Simulate,
    /// Audit a statute
    Audit,
    /// Analyze complexity
    Complexity,
    /// Get help
    Help,
    /// Search for statutes
    Search,
    /// Install a statute
    Install,
    /// Update statutes
    Update,
    /// Format statute files
    Format,
    /// Lint statute files
    Lint,
    /// Test statutes
    Test,
    /// Debug statutes
    Debug,
    /// Profile performance
    Profile,
    /// Unknown intent
    Unknown,
}

/// AI-powered command parser that converts natural language to CLI commands.
#[derive(Debug, Clone)]
pub struct NaturalLanguageParser {
    /// Command patterns mapped to their CLI equivalents
    patterns: HashMap<String, String>,
}

impl Default for NaturalLanguageParser {
    fn default() -> Self {
        Self::new()
    }
}

impl NaturalLanguageParser {
    /// Create a new natural language parser with predefined patterns.
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // Parse patterns
        patterns.insert("parse".to_string(), "parse".to_string());
        patterns.insert("read".to_string(), "parse".to_string());
        patterns.insert("load".to_string(), "parse".to_string());
        patterns.insert("open".to_string(), "parse".to_string());

        // Verify patterns
        patterns.insert("verify".to_string(), "verify".to_string());
        patterns.insert("check".to_string(), "verify".to_string());
        patterns.insert("validate".to_string(), "verify".to_string());
        patterns.insert("test validity".to_string(), "verify".to_string());

        // Visualize patterns
        patterns.insert("visualize".to_string(), "viz".to_string());
        patterns.insert("show".to_string(), "viz".to_string());
        patterns.insert("display".to_string(), "viz".to_string());
        patterns.insert("draw".to_string(), "viz".to_string());
        patterns.insert("graph".to_string(), "viz".to_string());

        // Export patterns
        patterns.insert("export".to_string(), "export".to_string());
        patterns.insert("convert".to_string(), "export".to_string());
        patterns.insert("save as".to_string(), "export".to_string());

        // Serve patterns
        patterns.insert("serve".to_string(), "serve".to_string());
        patterns.insert("start server".to_string(), "serve".to_string());
        patterns.insert("run server".to_string(), "serve".to_string());

        // Init patterns
        patterns.insert("init".to_string(), "init".to_string());
        patterns.insert("initialize".to_string(), "init".to_string());
        patterns.insert("create project".to_string(), "init".to_string());
        patterns.insert("new project".to_string(), "init".to_string());

        // Diff patterns
        patterns.insert("diff".to_string(), "diff".to_string());
        patterns.insert("compare".to_string(), "diff".to_string());
        patterns.insert("difference".to_string(), "diff".to_string());

        // Simulate patterns
        patterns.insert("simulate".to_string(), "simulate".to_string());
        patterns.insert("run simulation".to_string(), "simulate".to_string());
        patterns.insert("execute".to_string(), "simulate".to_string());

        // Audit patterns
        patterns.insert("audit".to_string(), "audit".to_string());
        patterns.insert("analyze".to_string(), "audit".to_string());
        patterns.insert("review".to_string(), "audit".to_string());

        // Format patterns
        patterns.insert("format".to_string(), "format".to_string());
        patterns.insert("fmt".to_string(), "format".to_string());
        patterns.insert("prettify".to_string(), "format".to_string());

        // Lint patterns
        patterns.insert("lint".to_string(), "lint".to_string());
        patterns.insert("check style".to_string(), "lint".to_string());

        // Help patterns
        patterns.insert("help".to_string(), "help".to_string());
        patterns.insert("how to".to_string(), "help".to_string());
        patterns.insert("what is".to_string(), "help".to_string());

        Self { patterns }
    }

    /// Parse natural language input into a CLI command.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis::ai::NaturalLanguageParser;
    ///
    /// let parser = NaturalLanguageParser::new();
    /// let cmd = parser.parse("I want to verify my statute file").unwrap();
    /// assert!(cmd.contains("verify"));
    /// ```
    pub fn parse(&self, input: &str) -> Result<String> {
        let input_lower = input.to_lowercase();

        // Try to match patterns
        for (pattern, command) in &self.patterns {
            if input_lower.contains(pattern) {
                return self.extract_command(&input_lower, command);
            }
        }

        bail!("Could not understand the command. Try: 'legalis help' for available commands.")
    }

    /// Extract a full command with arguments from natural language.
    fn extract_command(&self, input: &str, base_command: &str) -> Result<String> {
        let mut command = base_command.to_string();

        // Extract file paths (simple heuristic: look for .legalis extension)
        if let Some(file) = self.extract_file_path(input) {
            command.push(' ');
            command.push_str(&file);
        }

        // Extract format flags
        if input.contains("json") {
            command.push_str(" --format json");
        } else if input.contains("yaml") {
            command.push_str(" --format yaml");
        } else if input.contains("toml") {
            command.push_str(" --format toml");
        }

        // Extract verbosity
        if input.contains("verbose") || input.contains("detailed") {
            command.push_str(" -v");
        }

        // Extract strict mode
        if input.contains("strict") || input.contains("strictly") {
            command.push_str(" --strict");
        }

        Ok(command)
    }

    /// Extract a file path from natural language input.
    fn extract_file_path(&self, input: &str) -> Option<String> {
        // Look for quoted strings
        if let Some(start) = input.find('"') {
            if let Some(end) = input[start + 1..].find('"') {
                return Some(input[start + 1..start + 1 + end].to_string());
            }
        }

        // Look for file extensions
        for word in input.split_whitespace() {
            if word.ends_with(".legalis") || word.ends_with(".json") || word.ends_with(".yaml") {
                return Some(word.to_string());
            }
        }

        None
    }
}

/// Intent recognizer that determines what the user wants to do.
#[derive(Debug, Clone)]
pub struct IntentRecognizer {
    /// Intent keywords
    keywords: HashMap<CommandIntent, Vec<String>>,
}

impl Default for IntentRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

impl IntentRecognizer {
    /// Create a new intent recognizer.
    pub fn new() -> Self {
        let mut keywords = HashMap::new();

        keywords.insert(
            CommandIntent::Parse,
            vec!["parse", "read", "load", "open"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Verify,
            vec!["verify", "check", "validate", "test"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Visualize,
            vec!["visualize", "show", "display", "draw", "graph"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Export,
            vec!["export", "convert", "save"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Serve,
            vec!["serve", "server", "api", "start"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Initialize,
            vec!["init", "initialize", "create", "new"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Diff,
            vec!["diff", "compare", "difference"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Simulate,
            vec!["simulate", "simulation", "run", "execute"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Audit,
            vec!["audit", "analyze", "review"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        keywords.insert(
            CommandIntent::Help,
            vec!["help", "how", "what", "explain"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        Self { keywords }
    }

    /// Recognize the intent from user input.
    pub fn recognize(&self, input: &str) -> CommandIntent {
        let input_lower = input.to_lowercase();

        let mut scores: HashMap<CommandIntent, usize> = HashMap::new();

        for (intent, keywords) in &self.keywords {
            let score = keywords
                .iter()
                .filter(|kw| input_lower.contains(kw.as_str()))
                .count();

            if score > 0 {
                scores.insert(intent.clone(), score);
            }
        }

        if scores.is_empty() {
            return CommandIntent::Unknown;
        }

        // Return the intent with the highest score
        scores
            .into_iter()
            .max_by_key(|(_, score)| *score)
            .map(|(intent, _)| intent)
            .unwrap_or(CommandIntent::Unknown)
    }

    /// Get suggested commands based on intent.
    pub fn suggest_commands(&self, intent: &CommandIntent) -> Vec<String> {
        match intent {
            CommandIntent::Parse => vec![
                "legalis parse <file>".to_string(),
                "legalis parse <file> --format json".to_string(),
                "legalis parse <file> --output result.json".to_string(),
            ],
            CommandIntent::Verify => vec![
                "legalis verify <file>".to_string(),
                "legalis verify <file> --strict".to_string(),
                "legalis verify <file> --format json".to_string(),
            ],
            CommandIntent::Visualize => vec![
                "legalis viz <file> -o output.svg mermaid".to_string(),
                "legalis viz <file> -o output.dot dot".to_string(),
                "legalis viz <file> -o output.txt ascii".to_string(),
            ],
            CommandIntent::Export => vec![
                "legalis export <file> <output> json".to_string(),
                "legalis export <file> <output> yaml".to_string(),
                "legalis export <file> <output> solidity".to_string(),
            ],
            CommandIntent::Serve => vec![
                "legalis serve".to_string(),
                "legalis serve --host 0.0.0.0".to_string(),
                "legalis serve --port 8080".to_string(),
            ],
            CommandIntent::Initialize => vec![
                "legalis init".to_string(),
                "legalis init <path>".to_string(),
                "legalis init --dry-run".to_string(),
            ],
            CommandIntent::Diff => vec![
                "legalis diff <old> <new> unified".to_string(),
                "legalis diff <old> <new> side-by-side".to_string(),
                "legalis diff <old> <new> context".to_string(),
            ],
            CommandIntent::Simulate => vec![
                "legalis simulate <file>".to_string(),
                "legalis simulate <file> --population 1000".to_string(),
                "legalis simulate <file> --output results.json".to_string(),
            ],
            CommandIntent::Audit => vec![
                "legalis audit <file>".to_string(),
                "legalis audit <file> --with-complexity".to_string(),
                "legalis audit <file> --output report.json".to_string(),
            ],
            CommandIntent::Format => vec![
                "legalis format <file>".to_string(),
                "legalis format <file> --inplace".to_string(),
                "legalis format <file> --style compact".to_string(),
            ],
            CommandIntent::Lint => vec![
                "legalis lint <file>".to_string(),
                "legalis lint <file> --fix".to_string(),
                "legalis lint <file> --strict".to_string(),
            ],
            CommandIntent::Help => vec![
                "legalis help".to_string(),
                "legalis <command> --help".to_string(),
                "legalis tutorial".to_string(),
            ],
            _ => vec!["legalis help".to_string()],
        }
    }
}

/// AI-powered help system that provides context-aware assistance.
#[derive(Debug, Clone)]
pub struct AiHelpSystem {
    /// Intent recognizer
    recognizer: IntentRecognizer,
}

impl Default for AiHelpSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AiHelpSystem {
    /// Create a new AI help system.
    pub fn new() -> Self {
        Self {
            recognizer: IntentRecognizer::new(),
        }
    }

    /// Get context-aware help for user input.
    pub fn get_help(&self, query: &str) -> String {
        let intent = self.recognizer.recognize(query);

        match intent {
            CommandIntent::Parse => self.parse_help(),
            CommandIntent::Verify => self.verify_help(),
            CommandIntent::Visualize => self.visualize_help(),
            CommandIntent::Export => self.export_help(),
            CommandIntent::Serve => self.serve_help(),
            CommandIntent::Initialize => self.init_help(),
            CommandIntent::Diff => self.diff_help(),
            CommandIntent::Simulate => self.simulate_help(),
            CommandIntent::Audit => self.audit_help(),
            CommandIntent::Format => self.format_help(),
            CommandIntent::Lint => self.lint_help(),
            CommandIntent::Help => self.general_help(),
            _ => self.unknown_help(query),
        }
    }

    fn parse_help(&self) -> String {
        "Parse Command Help:\n\n\
        The 'parse' command reads and validates a Legalis statute file.\n\n\
        Usage:\n\
          legalis parse <file> [--format <format>] [--output <output>]\n\n\
        Examples:\n\
          legalis parse statute.legalis\n\
          legalis parse statute.legalis --format json\n\
          legalis parse statute.legalis --output parsed.json\n\n\
        Options:\n\
          --format: Output format (text, json, yaml, toml)\n\
          --output: Output file path\n"
            .to_string()
    }

    fn verify_help(&self) -> String {
        "Verify Command Help:\n\n\
        The 'verify' command validates statute consistency and correctness.\n\n\
        Usage:\n\
          legalis verify <file> [--strict] [--format <format>]\n\n\
        Examples:\n\
          legalis verify statute.legalis\n\
          legalis verify statute.legalis --strict\n\
          legalis verify statute.legalis --format json\n\n\
        Options:\n\
          --strict: Enable strict validation mode\n\
          --format: Output format (text, json, yaml, toml)\n"
            .to_string()
    }

    fn visualize_help(&self) -> String {
        "Visualize Command Help:\n\n\
        The 'viz' command generates visual representations of statutes.\n\n\
        Usage:\n\
          legalis viz <file> -o <output> <format>\n\n\
        Examples:\n\
          legalis viz statute.legalis -o graph.svg mermaid\n\
          legalis viz statute.legalis -o graph.dot dot\n\
          legalis viz statute.legalis -o graph.txt ascii\n\n\
        Formats:\n\
          mermaid: Mermaid diagram format\n\
          dot: GraphViz DOT format\n\
          ascii: ASCII art format\n"
            .to_string()
    }

    fn export_help(&self) -> String {
        "Export Command Help:\n\n\
        The 'export' command converts statutes to different formats.\n\n\
        Usage:\n\
          legalis export <input> <output> <format>\n\n\
        Examples:\n\
          legalis export statute.legalis output.json json\n\
          legalis export statute.legalis output.yaml yaml\n\
          legalis export statute.legalis contract.sol solidity\n\n\
        Formats:\n\
          json: JSON format\n\
          yaml: YAML format\n\
          solidity: Solidity smart contract\n"
            .to_string()
    }

    fn serve_help(&self) -> String {
        "Serve Command Help:\n\n\
        The 'serve' command starts an API server for Legalis.\n\n\
        Usage:\n\
          legalis serve [--host <host>] [--port <port>]\n\n\
        Examples:\n\
          legalis serve\n\
          legalis serve --host 0.0.0.0\n\
          legalis serve --port 8080\n\n\
        Options:\n\
          --host: Server host (default: 127.0.0.1)\n\
          --port: Server port (default: 3000)\n"
            .to_string()
    }

    fn init_help(&self) -> String {
        "Init Command Help:\n\n\
        The 'init' command initializes a new Legalis project.\n\n\
        Usage:\n\
          legalis init [path] [--dry-run]\n\n\
        Examples:\n\
          legalis init\n\
          legalis init my-project\n\
          legalis init --dry-run\n\n\
        Options:\n\
          --dry-run: Show what would be created without creating\n"
            .to_string()
    }

    fn diff_help(&self) -> String {
        "Diff Command Help:\n\n\
        The 'diff' command compares two statute files.\n\n\
        Usage:\n\
          legalis diff <old> <new> <format>\n\n\
        Examples:\n\
          legalis diff old.legalis new.legalis unified\n\
          legalis diff old.legalis new.legalis side-by-side\n\
          legalis diff old.legalis new.legalis context\n\n\
        Formats:\n\
          unified: Unified diff format\n\
          side-by-side: Side-by-side comparison\n\
          context: Context diff format\n"
            .to_string()
    }

    fn simulate_help(&self) -> String {
        "Simulate Command Help:\n\n\
        The 'simulate' command runs statute simulations.\n\n\
        Usage:\n\
          legalis simulate <file> [--population <n>] [--output <output>]\n\n\
        Examples:\n\
          legalis simulate statute.legalis\n\
          legalis simulate statute.legalis --population 1000\n\
          legalis simulate statute.legalis --output results.json\n\n\
        Options:\n\
          --population: Simulation population size (default: 100)\n\
          --output: Output file for results\n"
            .to_string()
    }

    fn audit_help(&self) -> String {
        "Audit Command Help:\n\n\
        The 'audit' command analyzes statute quality and complexity.\n\n\
        Usage:\n\
          legalis audit <file> [--with-complexity] [--output <output>]\n\n\
        Examples:\n\
          legalis audit statute.legalis\n\
          legalis audit statute.legalis --with-complexity\n\
          legalis audit statute.legalis --output report.json\n\n\
        Options:\n\
          --with-complexity: Include complexity analysis\n\
          --output: Output file for audit report\n"
            .to_string()
    }

    fn format_help(&self) -> String {
        "Format Command Help:\n\n\
        The 'format' command formats statute files.\n\n\
        Usage:\n\
          legalis format <file> [--inplace] [--style <style>] [--dry-run]\n\n\
        Examples:\n\
          legalis format statute.legalis\n\
          legalis format statute.legalis --inplace\n\
          legalis format statute.legalis --style compact\n\n\
        Options:\n\
          --inplace: Format file in place\n\
          --style: Formatting style (standard, compact, verbose)\n\
          --dry-run: Show changes without applying\n"
            .to_string()
    }

    fn lint_help(&self) -> String {
        "Lint Command Help:\n\n\
        The 'lint' command checks statute style and best practices.\n\n\
        Usage:\n\
          legalis lint <file> [--fix] [--strict]\n\n\
        Examples:\n\
          legalis lint statute.legalis\n\
          legalis lint statute.legalis --fix\n\
          legalis lint statute.legalis --strict\n\n\
        Options:\n\
          --fix: Automatically fix issues\n\
          --strict: Enable strict linting rules\n"
            .to_string()
    }

    fn general_help(&self) -> String {
        "Legalis CLI Help:\n\n\
        Available commands:\n\
          parse      - Parse and validate statute files\n\
          verify     - Verify statute consistency\n\
          viz        - Generate visualizations\n\
          export     - Export to different formats\n\
          serve      - Start API server\n\
          init       - Initialize new project\n\
          diff       - Compare statute files\n\
          simulate   - Run simulations\n\
          audit      - Analyze statute quality\n\
          format     - Format statute files\n\
          lint       - Check statute style\n\
          help       - Show help information\n\n\
        For more information on a specific command:\n\
          legalis <command> --help\n\
          legalis help <command>\n\n\
        For interactive tutorials:\n\
          legalis tutorial\n"
            .to_string()
    }

    fn unknown_help(&self, query: &str) -> String {
        format!(
            "I couldn't understand your query: '{}'\n\n\
            Try one of these:\n\
              legalis help\n\
              legalis <command> --help\n\
              legalis tutorial\n\n\
            Common commands:\n\
              parse, verify, viz, export, serve, init, diff, simulate, audit\n",
            query
        )
    }
}

/// Intelligent autocomplete system for CLI commands.
#[derive(Debug, Clone)]
pub struct IntelligentAutocomplete {
    /// Command suggestions
    suggestions: HashMap<String, Vec<String>>,
}

impl Default for IntelligentAutocomplete {
    fn default() -> Self {
        Self::new()
    }
}

impl IntelligentAutocomplete {
    /// Create a new intelligent autocomplete system.
    pub fn new() -> Self {
        let mut suggestions = HashMap::new();

        // Add suggestions for common command prefixes
        suggestions.insert("p".to_string(), vec!["parse".to_string()]);
        suggestions.insert(
            "v".to_string(),
            vec!["verify".to_string(), "viz".to_string()],
        );
        suggestions.insert("e".to_string(), vec!["export".to_string()]);
        suggestions.insert(
            "s".to_string(),
            vec!["serve".to_string(), "simulate".to_string()],
        );
        suggestions.insert("i".to_string(), vec!["init".to_string()]);
        suggestions.insert("d".to_string(), vec!["diff".to_string()]);
        suggestions.insert("a".to_string(), vec!["audit".to_string()]);
        suggestions.insert("f".to_string(), vec!["format".to_string()]);
        suggestions.insert("l".to_string(), vec!["lint".to_string()]);
        suggestions.insert("h".to_string(), vec!["help".to_string()]);

        Self { suggestions }
    }

    /// Get autocomplete suggestions for the given input.
    pub fn complete(&self, input: &str) -> Vec<String> {
        if input.is_empty() {
            return Vec::new();
        }

        let input_lower = input.to_lowercase();

        // Check if we have direct suggestions
        if let Some(suggs) = self.suggestions.get(&input_lower) {
            return suggs.clone();
        }

        // Try prefix matching
        let mut matches = Vec::new();
        for (_, commands) in &self.suggestions {
            for cmd in commands {
                if cmd.starts_with(&input_lower) && !matches.contains(cmd) {
                    matches.push(cmd.clone());
                }
            }
        }

        matches.sort();
        matches
    }

    /// Get context-aware suggestions based on previous commands.
    pub fn suggest_next(&self, previous_command: &str) -> Vec<String> {
        match previous_command {
            "parse" => vec![
                "verify".to_string(),
                "viz".to_string(),
                "export".to_string(),
            ],
            "verify" => vec![
                "lint".to_string(),
                "format".to_string(),
                "audit".to_string(),
            ],
            "init" => vec!["new".to_string(), "format".to_string()],
            "simulate" => vec!["audit".to_string(), "export".to_string()],
            _ => vec![
                "parse".to_string(),
                "verify".to_string(),
                "help".to_string(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_language_parser() {
        let parser = NaturalLanguageParser::new();

        let cmd = parser.parse("verify my file").unwrap();
        assert!(cmd.contains("verify"));

        let cmd = parser.parse("show me the visualization").unwrap();
        assert!(cmd.contains("viz"));

        let cmd = parser.parse("parse statute.legalis").unwrap();
        assert!(cmd.contains("parse"));
        assert!(cmd.contains("statute.legalis"));
    }

    #[test]
    fn test_intent_recognizer() {
        let recognizer = IntentRecognizer::new();

        assert_eq!(
            recognizer.recognize("I want to verify this file"),
            CommandIntent::Verify
        );

        assert_eq!(
            recognizer.recognize("show me the graph"),
            CommandIntent::Visualize
        );

        assert_eq!(
            recognizer.recognize("parse my statute"),
            CommandIntent::Parse
        );
    }

    #[test]
    fn test_ai_help_system() {
        let help = AiHelpSystem::new();

        let result = help.get_help("how do I verify a file?");
        assert!(result.contains("verify"));

        let result = help.get_help("what is parse?");
        assert!(result.contains("parse"));
    }

    #[test]
    fn test_intelligent_autocomplete() {
        let autocomplete = IntelligentAutocomplete::new();

        let suggestions = autocomplete.complete("v");
        assert!(
            suggestions.contains(&"verify".to_string()) || suggestions.contains(&"viz".to_string())
        );

        let suggestions = autocomplete.complete("pa");
        assert!(suggestions.contains(&"parse".to_string()));
    }

    #[test]
    fn test_suggest_next_command() {
        let autocomplete = IntelligentAutocomplete::new();

        let suggestions = autocomplete.suggest_next("parse");
        assert!(suggestions.contains(&"verify".to_string()));

        let suggestions = autocomplete.suggest_next("verify");
        assert!(suggestions.contains(&"lint".to_string()));
    }
}
