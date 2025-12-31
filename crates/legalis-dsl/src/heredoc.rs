//! Heredoc (Here Document) syntax support for multi-line strings.
//!
//! This module provides parsing for heredoc-style multi-line strings:
//! - `<<EOF ... EOF` - Basic heredoc
//! - `<<-EOF ... EOF` - Indented heredoc (strips leading whitespace)
//! - `<<"EOF" ... EOF` - Quoted heredoc (preserves exact formatting)

use thiserror::Error;

/// Errors that can occur during heredoc parsing
#[derive(Debug, Error, Clone, PartialEq)]
pub enum HeredocError {
    #[error("Unclosed heredoc: expected delimiter '{0}'")]
    UnclosedHeredoc(String),

    #[error("Invalid heredoc delimiter: '{0}'")]
    InvalidDelimiter(String),

    #[error("Heredoc delimiter must start with <<")]
    MissingPrefix,

    #[error("Empty heredoc delimiter")]
    EmptyDelimiter,
}

/// Type of heredoc
#[derive(Debug, Clone, PartialEq)]
pub enum HeredocType {
    /// Basic heredoc (<<EOF)
    Basic,
    /// Indented heredoc (<<-EOF) - strips leading whitespace
    Indented,
    /// Quoted heredoc (<<"EOF") - preserves exact formatting
    Quoted,
}

/// Parsed heredoc result
#[derive(Debug, Clone, PartialEq)]
pub struct HeredocResult {
    /// The extracted content
    pub content: String,
    /// The type of heredoc
    pub heredoc_type: HeredocType,
    /// The delimiter used
    pub delimiter: String,
    /// Number of lines consumed (including delimiter lines)
    pub lines_consumed: usize,
}

/// Parser for heredoc syntax
pub struct HeredocParser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> HeredocParser<'a> {
    /// Creates a new heredoc parser
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    /// Parses a heredoc from the current position
    ///
    /// Returns Ok(Some(result)) if heredoc is found and parsed
    /// Returns Ok(None) if no heredoc at current position
    /// Returns Err if heredoc is malformed
    pub fn parse(&mut self) -> Result<Option<HeredocResult>, HeredocError> {
        // Check if input starts with <<
        if !self.input[self.position..].starts_with("<<") {
            return Ok(None);
        }

        self.position += 2; // Skip <<

        // Determine heredoc type
        let heredoc_type = if self.peek_char() == Some('-') {
            self.position += 1;
            HeredocType::Indented
        } else if self.peek_char() == Some('"') {
            HeredocType::Quoted
        } else {
            HeredocType::Basic
        };

        // Parse delimiter
        let delimiter = self.parse_delimiter(heredoc_type == HeredocType::Quoted)?;

        if delimiter.is_empty() {
            return Err(HeredocError::EmptyDelimiter);
        }

        // Skip to next line
        self.skip_to_next_line();

        // Collect content until we find the delimiter
        let mut content_lines = Vec::new();
        let mut lines_consumed = 1; // Start line with <<delimiter

        loop {
            if self.position >= self.input.len() {
                return Err(HeredocError::UnclosedHeredoc(delimiter));
            }

            let line_start = self.position;
            let line = self.read_line();
            lines_consumed += 1;

            // Check if this line is the end delimiter
            if line.trim() == delimiter {
                break;
            }

            content_lines.push(&self.input[line_start..self.position - 1]); // -1 to exclude newline
        }

        // Process content based on heredoc type
        let content = match heredoc_type {
            HeredocType::Basic | HeredocType::Quoted => content_lines.join("\n"),
            HeredocType::Indented => {
                // Strip leading whitespace from all lines
                self.strip_leading_whitespace(&content_lines).join("\n")
            }
        };

        Ok(Some(HeredocResult {
            content,
            heredoc_type,
            delimiter,
            lines_consumed,
        }))
    }

    /// Parses the delimiter
    fn parse_delimiter(&mut self, is_quoted: bool) -> Result<String, HeredocError> {
        let mut delimiter = String::new();

        if is_quoted {
            // Skip opening quote
            if self.peek_char() != Some('"') {
                return Err(HeredocError::InvalidDelimiter(
                    "expected opening quote".to_string(),
                ));
            }
            self.position += 1;

            // Read until closing quote
            while let Some(ch) = self.peek_char() {
                if ch == '"' {
                    self.position += 1;
                    break;
                }
                delimiter.push(ch);
                self.position += 1;
            }
        } else {
            // Read alphanumeric identifier
            while let Some(ch) = self.peek_char() {
                if ch.is_alphanumeric() || ch == '_' {
                    delimiter.push(ch);
                    self.position += 1;
                } else {
                    break;
                }
            }
        }

        Ok(delimiter)
    }

    /// Skips to the next line
    fn skip_to_next_line(&mut self) {
        while let Some(ch) = self.peek_char() {
            self.position += 1;
            if ch == '\n' {
                break;
            }
        }
    }

    /// Reads a line from the current position
    fn read_line(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            self.position += 1;
            if ch == '\n' {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    /// Peeks at the current character
    fn peek_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    /// Strips leading whitespace from all lines based on minimum indentation
    fn strip_leading_whitespace(&self, lines: &[&str]) -> Vec<String> {
        // Find minimum indentation (excluding empty lines)
        let min_indent = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start().len())
            .min()
            .unwrap_or(0);

        // Strip that many spaces from each line
        lines
            .iter()
            .map(|line| {
                if line.len() >= min_indent {
                    line[min_indent..].to_string()
                } else {
                    line.to_string()
                }
            })
            .collect()
    }
}

/// Convenience function to parse a heredoc from a string
pub fn parse_heredoc(input: &str) -> Result<Option<HeredocResult>, HeredocError> {
    let mut parser = HeredocParser::new(input);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_heredoc() {
        let input = "<<EOF\nLine 1\nLine 2\nLine 3\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.content, "Line 1\nLine 2\nLine 3");
        assert_eq!(result.heredoc_type, HeredocType::Basic);
        assert_eq!(result.delimiter, "EOF");
        assert_eq!(result.lines_consumed, 5);
    }

    #[test]
    fn test_indented_heredoc() {
        let input = "<<-EOF\n    Line 1\n    Line 2\n    Line 3\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        // Indented heredoc should strip leading whitespace
        assert_eq!(result.content, "Line 1\nLine 2\nLine 3");
        assert_eq!(result.heredoc_type, HeredocType::Indented);
        assert_eq!(result.delimiter, "EOF");
    }

    #[test]
    fn test_quoted_heredoc() {
        let input = "<<\"EOF\"\nLine 1\nLine 2\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.content, "Line 1\nLine 2");
        assert_eq!(result.heredoc_type, HeredocType::Quoted);
        assert_eq!(result.delimiter, "EOF");
    }

    #[test]
    fn test_heredoc_with_empty_lines() {
        let input = "<<EOF\nLine 1\n\nLine 3\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.content, "Line 1\n\nLine 3");
    }

    #[test]
    fn test_heredoc_different_delimiter() {
        let input = "<<END\nContent here\nEND\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.content, "Content here");
        assert_eq!(result.delimiter, "END");
    }

    #[test]
    fn test_unclosed_heredoc() {
        let input = "<<EOF\nLine 1\nLine 2\n";
        let result = parse_heredoc(input);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            HeredocError::UnclosedHeredoc(_)
        ));
    }

    #[test]
    fn test_empty_delimiter() {
        let input = "<<\nContent\n";
        let result = parse_heredoc(input);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), HeredocError::EmptyDelimiter));
    }

    #[test]
    fn test_no_heredoc() {
        let input = "Just a regular string";
        let result = parse_heredoc(input).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_indented_heredoc_mixed_indentation() {
        let input = "<<-EOF\n  Line 1\n    Line 2\n  Line 3\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        // Should strip 2 spaces (minimum indentation)
        assert_eq!(result.content, "Line 1\n  Line 2\nLine 3");
    }

    #[test]
    fn test_heredoc_with_special_chars() {
        let input = "<<EOF\nLine with $special {chars}\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.content, "Line with $special {chars}");
    }

    #[test]
    fn test_heredoc_single_line() {
        let input = "<<EOF\nSingle line\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.content, "Single line");
    }

    #[test]
    fn test_heredoc_numeric_delimiter() {
        let input = "<<EOF123\nContent\nEOF123\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.delimiter, "EOF123");
        assert_eq!(result.content, "Content");
    }

    #[test]
    fn test_quoted_heredoc_with_spaces() {
        let input = "<<\"END OF TEXT\"\nMulti-line\ncontent\nEND OF TEXT\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        assert_eq!(result.delimiter, "END OF TEXT");
        assert_eq!(result.content, "Multi-line\ncontent");
    }

    #[test]
    fn test_indented_heredoc_empty_lines() {
        let input = "<<-EOF\n    Line 1\n\n    Line 3\nEOF\n";
        let result = parse_heredoc(input).unwrap().unwrap();

        // Empty lines should be preserved
        assert_eq!(result.content, "Line 1\n\nLine 3");
    }
}
