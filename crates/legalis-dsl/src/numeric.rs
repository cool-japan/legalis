//! Numeric literal parsing with support for various formats.
//!
//! This module provides parsing for:
//! - Scientific notation (1.5e6, 2.3e-4)
//! - Binary literals (0b1010, 0B1111)
//! - Hexadecimal literals (0xFF, 0XAB)
//! - Octal literals (0o755, 0O644)
//! - Regular integers and floats

use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

/// Errors that can occur during numeric literal parsing
#[derive(Debug, Error, Clone, PartialEq)]
pub enum NumericError {
    #[error("Invalid integer literal: {0}")]
    InvalidInteger(String),

    #[error("Invalid float literal: {0}")]
    InvalidFloat(String),

    #[error("Invalid binary literal: {0}")]
    InvalidBinary(String),

    #[error("Invalid hexadecimal literal: {0}")]
    InvalidHex(String),

    #[error("Invalid octal literal: {0}")]
    InvalidOctal(String),

    #[error("Number out of range: {0}")]
    OutOfRange(String),

    #[error("Empty numeric literal")]
    EmptyLiteral,
}

impl From<ParseIntError> for NumericError {
    fn from(e: ParseIntError) -> Self {
        NumericError::InvalidInteger(e.to_string())
    }
}

impl From<ParseFloatError> for NumericError {
    fn from(e: ParseFloatError) -> Self {
        NumericError::InvalidFloat(e.to_string())
    }
}

/// Represents a parsed numeric value
#[derive(Debug, Clone, PartialEq)]
pub enum NumericValue {
    /// Integer value
    Integer(i64),
    /// Floating point value
    Float(f64),
}

impl NumericValue {
    /// Converts the numeric value to i64, truncating if necessary
    pub fn to_i64(&self) -> i64 {
        match self {
            NumericValue::Integer(i) => *i,
            NumericValue::Float(f) => *f as i64,
        }
    }

    /// Converts the numeric value to f64
    pub fn to_f64(&self) -> f64 {
        match self {
            NumericValue::Integer(i) => *i as f64,
            NumericValue::Float(f) => *f,
        }
    }

    /// Checks if the value is an integer
    pub fn is_integer(&self) -> bool {
        matches!(self, NumericValue::Integer(_))
    }

    /// Checks if the value is a float
    pub fn is_float(&self) -> bool {
        matches!(self, NumericValue::Float(_))
    }
}

/// Parser for numeric literals
pub struct NumericParser;

impl NumericParser {
    /// Parses a numeric literal string into a NumericValue
    pub fn parse(input: &str) -> Result<NumericValue, NumericError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(NumericError::EmptyLiteral);
        }

        // Remove underscores (digit separators)
        let cleaned = input.replace('_', "");

        // Check for binary literal (0b or 0B prefix)
        if cleaned.starts_with("0b") || cleaned.starts_with("0B") {
            return Self::parse_binary(&cleaned[2..]);
        }

        // Check for hexadecimal literal (0x or 0X prefix)
        if cleaned.starts_with("0x") || cleaned.starts_with("0X") {
            return Self::parse_hex(&cleaned[2..]);
        }

        // Check for octal literal (0o or 0O prefix)
        if cleaned.starts_with("0o") || cleaned.starts_with("0O") {
            return Self::parse_octal(&cleaned[2..]);
        }

        // Check for scientific notation or decimal point
        if cleaned.contains('e')
            || cleaned.contains('E')
            || cleaned.contains('.')
        {
            return Self::parse_float(&cleaned);
        }

        // Parse as regular integer
        Self::parse_integer(&cleaned)
    }

    /// Parses a binary literal (without the 0b prefix)
    fn parse_binary(input: &str) -> Result<NumericValue, NumericError> {
        if input.is_empty() {
            return Err(NumericError::InvalidBinary("empty binary literal".to_string()));
        }

        i64::from_str_radix(input, 2)
            .map(NumericValue::Integer)
            .map_err(|_| NumericError::InvalidBinary(input.to_string()))
    }

    /// Parses a hexadecimal literal (without the 0x prefix)
    fn parse_hex(input: &str) -> Result<NumericValue, NumericError> {
        if input.is_empty() {
            return Err(NumericError::InvalidHex("empty hex literal".to_string()));
        }

        i64::from_str_radix(input, 16)
            .map(NumericValue::Integer)
            .map_err(|_| NumericError::InvalidHex(input.to_string()))
    }

    /// Parses an octal literal (without the 0o prefix)
    fn parse_octal(input: &str) -> Result<NumericValue, NumericError> {
        if input.is_empty() {
            return Err(NumericError::InvalidOctal("empty octal literal".to_string()));
        }

        i64::from_str_radix(input, 8)
            .map(NumericValue::Integer)
            .map_err(|_| NumericError::InvalidOctal(input.to_string()))
    }

    /// Parses a floating point number (possibly with scientific notation)
    fn parse_float(input: &str) -> Result<NumericValue, NumericError> {
        input
            .parse::<f64>()
            .map(NumericValue::Float)
            .map_err(|_| NumericError::InvalidFloat(input.to_string()))
    }

    /// Parses a regular integer
    fn parse_integer(input: &str) -> Result<NumericValue, NumericError> {
        input
            .parse::<i64>()
            .map(NumericValue::Integer)
            .map_err(|_| NumericError::InvalidInteger(input.to_string()))
    }
}

/// Convenience function to parse a numeric literal
pub fn parse_numeric(input: &str) -> Result<NumericValue, NumericError> {
    NumericParser::parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_regular_integer() {
        let result = parse_numeric("42").unwrap();
        assert_eq!(result, NumericValue::Integer(42));
    }

    #[test]
    fn test_parse_negative_integer() {
        let result = parse_numeric("-42").unwrap();
        assert_eq!(result, NumericValue::Integer(-42));
    }

    #[test]
    fn test_parse_float() {
        let result = parse_numeric("3.14").unwrap();
        assert_eq!(result, NumericValue::Float(3.14));
    }

    #[test]
    fn test_parse_scientific_notation_positive_exp() {
        let result = parse_numeric("1.5e6").unwrap();
        assert_eq!(result, NumericValue::Float(1.5e6));
        assert_eq!(result.to_f64(), 1_500_000.0);
    }

    #[test]
    fn test_parse_scientific_notation_negative_exp() {
        let result = parse_numeric("2.3e-4").unwrap();
        assert_eq!(result, NumericValue::Float(2.3e-4));
        assert_eq!(result.to_f64(), 0.00023);
    }

    #[test]
    fn test_parse_scientific_notation_uppercase() {
        let result = parse_numeric("1.5E6").unwrap();
        assert_eq!(result, NumericValue::Float(1.5e6));
    }

    #[test]
    fn test_parse_binary_lowercase() {
        let result = parse_numeric("0b1010").unwrap();
        assert_eq!(result, NumericValue::Integer(10));
    }

    #[test]
    fn test_parse_binary_uppercase() {
        let result = parse_numeric("0B1111").unwrap();
        assert_eq!(result, NumericValue::Integer(15));
    }

    #[test]
    fn test_parse_hex_lowercase() {
        let result = parse_numeric("0xff").unwrap();
        assert_eq!(result, NumericValue::Integer(255));
    }

    #[test]
    fn test_parse_hex_uppercase() {
        let result = parse_numeric("0XFF").unwrap();
        assert_eq!(result, NumericValue::Integer(255));
    }

    #[test]
    fn test_parse_hex_mixed_case() {
        let result = parse_numeric("0xAbCd").unwrap();
        assert_eq!(result, NumericValue::Integer(0xABCD));
    }

    #[test]
    fn test_parse_octal_lowercase() {
        let result = parse_numeric("0o755").unwrap();
        assert_eq!(result, NumericValue::Integer(0o755));
    }

    #[test]
    fn test_parse_octal_uppercase() {
        let result = parse_numeric("0O644").unwrap();
        assert_eq!(result, NumericValue::Integer(0o644));
    }

    #[test]
    fn test_parse_with_underscores() {
        let result = parse_numeric("1_000_000").unwrap();
        assert_eq!(result, NumericValue::Integer(1_000_000));
    }

    #[test]
    fn test_parse_binary_with_underscores() {
        let result = parse_numeric("0b1111_0000").unwrap();
        assert_eq!(result, NumericValue::Integer(0b1111_0000));
    }

    #[test]
    fn test_parse_hex_with_underscores() {
        let result = parse_numeric("0xFF_FF").unwrap();
        assert_eq!(result, NumericValue::Integer(0xFFFF));
    }

    #[test]
    fn test_parse_float_with_underscores() {
        let result = parse_numeric("1_234.567_8").unwrap();
        assert_eq!(result, NumericValue::Float(1234.5678));
    }

    #[test]
    fn test_parse_empty_string() {
        let result = parse_numeric("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NumericError::EmptyLiteral));
    }

    #[test]
    fn test_parse_invalid_binary() {
        let result = parse_numeric("0b1012");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NumericError::InvalidBinary(_)));
    }

    #[test]
    fn test_parse_invalid_hex() {
        let result = parse_numeric("0xGHI");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NumericError::InvalidHex(_)));
    }

    #[test]
    fn test_parse_invalid_octal() {
        let result = parse_numeric("0o789");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), NumericError::InvalidOctal(_)));
    }

    #[test]
    fn test_numeric_value_to_i64() {
        let int_val = NumericValue::Integer(42);
        assert_eq!(int_val.to_i64(), 42);

        let float_val = NumericValue::Float(3.7);
        assert_eq!(float_val.to_i64(), 3);
    }

    #[test]
    fn test_numeric_value_to_f64() {
        let int_val = NumericValue::Integer(42);
        assert_eq!(int_val.to_f64(), 42.0);

        let float_val = NumericValue::Float(3.14);
        assert_eq!(float_val.to_f64(), 3.14);
    }

    #[test]
    fn test_numeric_value_is_integer() {
        let int_val = NumericValue::Integer(42);
        assert!(int_val.is_integer());
        assert!(!int_val.is_float());
    }

    #[test]
    fn test_numeric_value_is_float() {
        let float_val = NumericValue::Float(3.14);
        assert!(float_val.is_float());
        assert!(!float_val.is_integer());
    }

    #[test]
    fn test_large_scientific_notation() {
        let result = parse_numeric("6.022e23").unwrap();
        assert_eq!(result.to_f64(), 6.022e23);
    }

    #[test]
    fn test_small_scientific_notation() {
        let result = parse_numeric("1.602e-19").unwrap();
        assert_eq!(result.to_f64(), 1.602e-19);
    }
}
