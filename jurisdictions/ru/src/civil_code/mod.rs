//! Civil Code of the Russian Federation (Гражданский кодекс РФ).
//!
//! The Civil Code consists of four parts:
//! - Part 1: General provisions, persons, property (1994)
//! - Part 2: Obligations and contracts (1996)
//! - Part 3: Succession law (2001)
//! - Part 4: Intellectual property rights (2006)

pub mod part1;
pub mod part2;
pub mod part3;
pub mod part4;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use part1::{PropertyRight, PropertyType};
pub use part2::{ContractType, ObligationType};
pub use part3::{SuccessionOrder, SuccessionRights};
pub use part4::{IntellectualPropertyRight, WorkType as IpWorkType};

/// Errors related to Civil Code operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CivilCodeError {
    /// Invalid property right
    #[error("Invalid property right: {0}")]
    InvalidPropertyRight(String),

    /// Invalid contract
    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    /// Invalid succession claim
    #[error("Invalid succession claim: {0}")]
    InvalidSuccession(String),

    /// Invalid intellectual property claim
    #[error("Invalid intellectual property claim: {0}")]
    InvalidIntellectualProperty(String),

    /// Capacity requirement not met
    #[error("Legal capacity requirement not met: {0}")]
    CapacityNotMet(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Civil law representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilLaw {
    /// Part of the Civil Code (1-4)
    pub part: u8,
    /// Article number
    pub article: u32,
    /// Paragraph number (if applicable)
    pub paragraph: Option<u32>,
    /// Description in Russian
    pub description_ru: String,
    /// Description in English
    pub description_en: Option<String>,
}

impl CivilLaw {
    /// Creates a new Civil Law article reference
    pub fn new(part: u8, article: u32, description_ru: impl Into<String>) -> Self {
        Self {
            part,
            article,
            paragraph: None,
            description_ru: description_ru.into(),
            description_en: None,
        }
    }

    /// Sets the paragraph number
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Sets the English description
    pub fn with_description_en(mut self, description: impl Into<String>) -> Self {
        self.description_en = Some(description.into());
        self
    }

    /// Formats the article reference
    pub fn format_reference(&self) -> String {
        let mut result = format!("ГК РФ, часть {}, ст. {}", self.part, self.article);
        if let Some(p) = self.paragraph {
            result.push_str(&format!(", п. {}", p));
        }
        result
    }
}

/// Quick validation for contract validity
pub fn quick_validate_contract(contract_type: &ContractType) -> Result<(), CivilCodeError> {
    part2::validate_contract(contract_type)
}

/// Validates property rights claim
pub fn validate_property_rights(
    property_type: &PropertyType,
    right: &PropertyRight,
) -> Result<(), CivilCodeError> {
    part1::validate_property_right(property_type, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_law_reference() {
        let law = CivilLaw::new(1, 128, "Виды объектов гражданских прав")
            .with_paragraph(1)
            .with_description_en("Types of civil rights objects");

        assert_eq!(law.part, 1);
        assert_eq!(law.article, 128);
        assert_eq!(law.paragraph, Some(1));

        let reference = law.format_reference();
        assert!(reference.contains("ГК РФ"));
        assert!(reference.contains("часть 1"));
        assert!(reference.contains("ст. 128"));
    }
}
