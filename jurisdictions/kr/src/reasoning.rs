//! Legal Reasoning for Korean Law
//!
//! # 법적 추론 / Legal Reasoning
//!
//! Utilities for legal interpretation and reasoning

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Reasoning errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ReasoningError {
    /// Invalid argument
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Conflicting provisions
    #[error("Conflicting provisions: {0}")]
    ConflictingProvisions(String),
}

/// Result type for legal reasoning operations
pub type ReasoningResult<T> = Result<T, ReasoningError>;

/// Interpretation method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpretationMethod {
    /// Literal interpretation (문리해석)
    Literal,
    /// Purposive interpretation (목적론적 해석)
    Purposive,
    /// Systematic interpretation (체계적 해석)
    Systematic,
    /// Historical interpretation (역사적 해석)
    Historical,
}

/// Legal provision
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalProvision {
    /// Law name
    pub law_name: String,
    /// Article number
    pub article: u32,
    /// Text
    pub text: String,
}

impl LegalProvision {
    /// Create new legal provision
    pub fn new(law_name: impl Into<String>, article: u32, text: impl Into<String>) -> Self {
        Self {
            law_name: law_name.into(),
            article,
            text: text.into(),
        }
    }
}

/// Supreme Court precedent hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrecedentLevel {
    /// Supreme Court (대법원)
    SupremeCourt,
    /// High Court (고등법원)
    HighCourt,
    /// District Court (지방법원)
    DistrictCourt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_provision() {
        let provision = LegalProvision::new(
            "민법",
            103,
            "선량한 풍속 기타 사회질서에 위반한 사항을 내용으로 하는 법률행위는 무효로 한다",
        );
        assert_eq!(provision.law_name, "민법");
        assert_eq!(provision.article, 103);
    }

    #[test]
    fn test_interpretation_method() {
        let method = InterpretationMethod::Literal;
        assert_eq!(method, InterpretationMethod::Literal);
    }
}
