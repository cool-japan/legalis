//! String interpolation support for effect descriptions and other text fields.
//!
//! This module provides functionality to parse and evaluate string interpolations
//! in the format `${variable}` or `${expression}`.

use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during string interpolation
#[derive(Debug, Error, Clone, PartialEq)]
pub enum InterpolationError {
    #[error("Undefined variable '{0}' in interpolation")]
    UndefinedVariable(String),

    #[error("Invalid interpolation syntax at position {0}: {1}")]
    InvalidSyntax(usize, String),

    #[error("Unclosed interpolation at position {0}")]
    UnclosedInterpolation(usize),
}

/// Represents a parsed interpolation token
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Literal text (no interpolation)
    Literal(String),
    /// Variable reference ${var}
    Variable(String),
}

/// Parser for string interpolations
#[derive(Debug)]
pub struct InterpolationParser {
    input: String,
    position: usize,
}

impl InterpolationParser {
    /// Creates a new interpolation parser
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    /// Parses the input string into tokens
    pub fn parse(&mut self) -> Result<Vec<Token>, InterpolationError> {
        let mut tokens = Vec::new();
        let mut current_literal = String::new();

        while self.position < self.input.len() {
            if self.peek_char() == Some('$') && self.peek_char_at(self.position + 1) == Some('{') {
                // Save any accumulated literal
                if !current_literal.is_empty() {
                    tokens.push(Token::Literal(current_literal.clone()));
                    current_literal.clear();
                }

                // Parse interpolation
                let var_token = self.parse_interpolation()?;
                tokens.push(var_token);
            } else if self.peek_char() == Some('\\') {
                // Handle escape sequences
                self.position += 1; // Skip backslash
                if let Some(next_ch) = self.peek_char() {
                    let escaped = match next_ch {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        '$' => '$',
                        '0' => '\0',
                        _ => {
                            // Unknown escape sequence - treat as literal
                            current_literal.push('\\');
                            next_ch
                        }
                    };
                    current_literal.push(escaped);
                    self.position += 1;
                } else {
                    // Backslash at end of string
                    current_literal.push('\\');
                }
            } else {
                // Regular character
                if let Some(ch) = self.peek_char() {
                    current_literal.push(ch);
                    self.position += 1;
                }
            }
        }

        // Save any remaining literal
        if !current_literal.is_empty() {
            tokens.push(Token::Literal(current_literal));
        }

        Ok(tokens)
    }

    /// Parses a single interpolation ${variable}
    fn parse_interpolation(&mut self) -> Result<Token, InterpolationError> {
        let start_pos = self.position;

        // Skip ${
        self.position += 2;

        // Parse variable name
        let mut var_name = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == '}' {
                self.position += 1; // Skip }
                if var_name.is_empty() {
                    return Err(InterpolationError::InvalidSyntax(
                        start_pos,
                        "Empty variable name".to_string(),
                    ));
                }
                return Ok(Token::Variable(var_name));
            } else if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                var_name.push(ch);
                self.position += 1;
            } else {
                return Err(InterpolationError::InvalidSyntax(
                    self.position,
                    format!("Invalid character '{}' in variable name", ch),
                ));
            }
        }

        Err(InterpolationError::UnclosedInterpolation(start_pos))
    }

    /// Peeks at the current character without consuming it
    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    /// Peeks at a character at a specific position
    fn peek_char_at(&self, pos: usize) -> Option<char> {
        self.input.chars().nth(pos)
    }
}

/// Evaluates interpolation tokens with a given context
pub struct InterpolationEvaluator {
    context: HashMap<String, String>,
}

impl InterpolationEvaluator {
    /// Creates a new evaluator with an empty context
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    /// Creates a new evaluator with a given context
    pub fn with_context(context: HashMap<String, String>) -> Self {
        Self { context }
    }

    /// Sets a variable in the context
    pub fn set(&mut self, key: String, value: String) {
        self.context.insert(key, value);
    }

    /// Evaluates tokens and produces the final string
    pub fn evaluate(&self, tokens: &[Token]) -> Result<String, InterpolationError> {
        let mut result = String::new();

        for token in tokens {
            match token {
                Token::Literal(text) => result.push_str(text),
                Token::Variable(var_name) => {
                    // Support nested access like "user.name"
                    let value = self.resolve_variable(var_name)?;
                    result.push_str(&value);
                }
            }
        }

        Ok(result)
    }

    /// Resolves a variable from the context
    fn resolve_variable(&self, var_name: &str) -> Result<String, InterpolationError> {
        // Check if it's a nested access
        if var_name.contains('.') {
            // For now, treat dotted names as simple keys
            // Future: could support nested maps
            if let Some(value) = self.context.get(var_name) {
                return Ok(value.clone());
            }
        }

        // Simple variable lookup
        self.context
            .get(var_name)
            .cloned()
            .ok_or_else(|| InterpolationError::UndefinedVariable(var_name.to_string()))
    }
}

impl Default for InterpolationEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// High-level function to interpolate a string with a context
pub fn interpolate(
    template: &str,
    context: &HashMap<String, String>,
) -> Result<String, InterpolationError> {
    let mut parser = InterpolationParser::new(template.to_string());
    let tokens = parser.parse()?;
    let evaluator = InterpolationEvaluator::with_context(context.clone());
    evaluator.evaluate(&tokens)
}

/// Extracts all variable names used in a template string
pub fn extract_variables(template: &str) -> Result<Vec<String>, InterpolationError> {
    let mut parser = InterpolationParser::new(template.to_string());
    let tokens = parser.parse()?;

    Ok(tokens
        .iter()
        .filter_map(|token| {
            if let Token::Variable(var) = token {
                Some(var.clone())
            } else {
                None
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_literal() {
        let mut parser = InterpolationParser::new("Hello, world!".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Hello, world!".to_string()));
    }

    #[test]
    fn test_parse_simple_variable() {
        let mut parser = InterpolationParser::new("Hello, ${name}!".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Literal("Hello, ".to_string()));
        assert_eq!(tokens[1], Token::Variable("name".to_string()));
        assert_eq!(tokens[2], Token::Literal("!".to_string()));
    }

    #[test]
    fn test_parse_multiple_variables() {
        let mut parser = InterpolationParser::new("Grant ${amount} to ${recipient}".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], Token::Literal("Grant ".to_string()));
        assert_eq!(tokens[1], Token::Variable("amount".to_string()));
        assert_eq!(tokens[2], Token::Literal(" to ".to_string()));
        assert_eq!(tokens[3], Token::Variable("recipient".to_string()));
    }

    #[test]
    fn test_parse_escaped_dollar() {
        let mut parser = InterpolationParser::new("Price: \\$100".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Price: $100".to_string()));
    }

    #[test]
    fn test_parse_unclosed_interpolation() {
        let mut parser = InterpolationParser::new("Hello, ${name".to_string());
        let result = parser.parse();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InterpolationError::UnclosedInterpolation(_)
        ));
    }

    #[test]
    fn test_parse_empty_variable() {
        let mut parser = InterpolationParser::new("Hello, ${}!".to_string());
        let result = parser.parse();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InterpolationError::InvalidSyntax(_, _)
        ));
    }

    #[test]
    fn test_parse_dotted_variable() {
        let mut parser = InterpolationParser::new("Hello, ${user.name}!".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[1], Token::Variable("user.name".to_string()));
    }

    #[test]
    fn test_evaluate_simple() {
        let tokens = vec![
            Token::Literal("Hello, ".to_string()),
            Token::Variable("name".to_string()),
            Token::Literal("!".to_string()),
        ];

        let mut context = HashMap::new();
        context.insert("name".to_string(), "Alice".to_string());

        let evaluator = InterpolationEvaluator::with_context(context);
        let result = evaluator.evaluate(&tokens).unwrap();

        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn test_evaluate_undefined_variable() {
        let tokens = vec![Token::Variable("missing".to_string())];
        let evaluator = InterpolationEvaluator::new();
        let result = evaluator.evaluate(&tokens);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InterpolationError::UndefinedVariable(_)
        ));
    }

    #[test]
    fn test_interpolate_function() {
        let template = "Grant ${amount} to ${recipient}";
        let mut context = HashMap::new();
        context.insert("amount".to_string(), "$1000".to_string());
        context.insert("recipient".to_string(), "John Doe".to_string());

        let result = interpolate(template, &context).unwrap();
        assert_eq!(result, "Grant $1000 to John Doe");
    }

    #[test]
    fn test_extract_variables() {
        let template = "Grant ${amount} to ${recipient} on ${date}";
        let vars = extract_variables(template).unwrap();

        assert_eq!(vars.len(), 3);
        assert!(vars.contains(&"amount".to_string()));
        assert!(vars.contains(&"recipient".to_string()));
        assert!(vars.contains(&"date".to_string()));
    }

    #[test]
    fn test_no_variables() {
        let template = "This is a plain string";
        let vars = extract_variables(template).unwrap();
        assert!(vars.is_empty());
    }

    #[test]
    fn test_escape_sequences_newline() {
        let mut parser = InterpolationParser::new("Line1\\nLine2".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Line1\nLine2".to_string()));
    }

    #[test]
    fn test_escape_sequences_tab() {
        let mut parser = InterpolationParser::new("Col1\\tCol2".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Col1\tCol2".to_string()));
    }

    #[test]
    fn test_escape_sequences_backslash() {
        let mut parser = InterpolationParser::new("Path\\\\File".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Path\\File".to_string()));
    }

    #[test]
    fn test_escape_sequences_quote() {
        let mut parser = InterpolationParser::new("Say \\\"Hello\\\"".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Say \"Hello\"".to_string()));
    }

    #[test]
    fn test_escape_sequences_mixed() {
        let mut parser = InterpolationParser::new("Line1\\nTab\\tQuote\\\"".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Line1\nTab\tQuote\"".to_string()));
    }

    #[test]
    fn test_escape_with_interpolation() {
        let mut parser = InterpolationParser::new("Path: C:\\\\Users\\\\${name}\\nEnd".to_string());
        let tokens = parser.parse().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Literal("Path: C:\\Users\\".to_string()));
        assert_eq!(tokens[1], Token::Variable("name".to_string()));
        assert_eq!(tokens[2], Token::Literal("\nEnd".to_string()));
    }

    #[test]
    fn test_unknown_escape_sequence() {
        let mut parser = InterpolationParser::new("Text\\xValue".to_string());
        let tokens = parser.parse().unwrap();

        // Unknown escape sequences are kept as-is (backslash + character)
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal("Text\\xValue".to_string()));
    }
}
