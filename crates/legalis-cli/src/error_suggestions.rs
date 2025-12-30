//! Error suggestions for common CLI errors.

use std::collections::HashMap;

/// Provides helpful suggestions for common errors.
pub struct ErrorSuggestions {
    suggestions: HashMap<String, Vec<String>>,
}

impl ErrorSuggestions {
    /// Create a new error suggestions system.
    pub fn new() -> Self {
        let mut suggestions = HashMap::new();

        // File not found errors
        suggestions.insert(
            "no such file".to_string(),
            vec![
                "Check if the file path is correct".to_string(),
                "Make sure the file exists using 'ls' or 'find'".to_string(),
                "Use an absolute path instead of a relative path".to_string(),
                "Verify file permissions with 'ls -l'".to_string(),
            ],
        );

        // Permission errors
        suggestions.insert(
            "permission denied".to_string(),
            vec![
                "Check file permissions with 'ls -l'".to_string(),
                "You may need to run with elevated privileges".to_string(),
                "Verify you have write access to the output directory".to_string(),
            ],
        );

        // Parse errors
        suggestions.insert(
            "parse error".to_string(),
            vec![
                "Check for syntax errors in your DSL file".to_string(),
                "Verify all brackets and parentheses are balanced".to_string(),
                "Use 'legalis lint' to check for common formatting issues".to_string(),
                "Compare with example files in the documentation".to_string(),
            ],
        );

        // Verification errors
        suggestions.insert(
            "verification failed".to_string(),
            vec![
                "Review the logical consistency of your conditions".to_string(),
                "Check for circular dependencies between statutes".to_string(),
                "Use 'legalis verify --strict' for detailed error messages".to_string(),
                "Run 'legalis complexity' to identify overly complex statutes".to_string(),
            ],
        );

        // Invalid command errors
        suggestions.insert(
            "invalid command".to_string(),
            vec![
                "Run 'legalis --help' to see available commands".to_string(),
                "Check for typos in the command name".to_string(),
                "Use 'legalis completions' to enable shell autocomplete".to_string(),
            ],
        );

        // Network errors
        suggestions.insert(
            "connection".to_string(),
            vec![
                "Check your internet connection".to_string(),
                "Verify the registry URL in your configuration".to_string(),
                "Try again later if the service is temporarily unavailable".to_string(),
            ],
        );

        // Format errors
        suggestions.insert(
            "invalid format".to_string(),
            vec![
                "Check supported formats with 'legalis --help'".to_string(),
                "Verify the input file is in the expected format".to_string(),
                "Use 'legalis validate' to check format compliance".to_string(),
            ],
        );

        Self { suggestions }
    }

    /// Get suggestions for an error message.
    pub fn get_suggestions(&self, error: &str) -> Vec<String> {
        let error_lower = error.to_lowercase();

        // Find all matching suggestions
        let mut all_suggestions = Vec::new();

        for (key, suggestions) in &self.suggestions {
            if error_lower.contains(key) {
                all_suggestions.extend(suggestions.clone());
            }
        }

        // Remove duplicates
        all_suggestions.sort();
        all_suggestions.dedup();

        all_suggestions
    }

    /// Format suggestions for display.
    pub fn format_suggestions(&self, error: &str) -> Option<String> {
        let suggestions = self.get_suggestions(error);

        if suggestions.is_empty() {
            return None;
        }

        let mut output = String::from("\nSuggestions:\n");
        for (i, suggestion) in suggestions.iter().enumerate() {
            output.push_str(&format!("  {}. {}\n", i + 1, suggestion));
        }

        Some(output)
    }

    /// Get a suggestion for a similar command name (for typos).
    pub fn suggest_command(&self, attempted: &str, valid_commands: &[&str]) -> Option<String> {
        let mut best_match = None;
        let mut best_distance = usize::MAX;

        for cmd in valid_commands {
            let distance = levenshtein_distance(attempted, cmd);
            if distance < best_distance && distance <= 3 {
                // Maximum edit distance of 3
                best_distance = distance;
                best_match = Some(*cmd);
            }
        }

        best_match.map(|m| format!("Did you mean '{}'?", m))
    }
}

impl Default for ErrorSuggestions {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate Levenshtein distance between two strings.
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
        row[0] = i;
    }

    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };

            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("verify", "verfy"), 1);
        assert_eq!(levenshtein_distance("test", "test"), 0);
    }

    #[test]
    fn test_error_suggestions() {
        let suggestions = ErrorSuggestions::new();

        let file_suggestions = suggestions.get_suggestions("Error: no such file or directory");
        assert!(!file_suggestions.is_empty());
        assert!(file_suggestions.iter().any(|s| s.contains("file path")));

        let parse_suggestions = suggestions.get_suggestions("Parse error at line 10");
        assert!(!parse_suggestions.is_empty());
        assert!(parse_suggestions.iter().any(|s| s.contains("syntax")));
    }

    #[test]
    fn test_command_suggestions() {
        let suggestions = ErrorSuggestions::new();
        let commands = vec!["verify", "parse", "export", "import"];

        assert_eq!(
            suggestions.suggest_command("verfy", &commands),
            Some("Did you mean 'verify'?".to_string())
        );

        assert_eq!(
            suggestions.suggest_command("pars", &commands),
            Some("Did you mean 'parse'?".to_string())
        );

        assert_eq!(suggestions.suggest_command("xyz", &commands), None);
    }
}
