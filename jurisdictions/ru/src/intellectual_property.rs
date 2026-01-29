//! Intellectual Property Law of the Russian Federation.
//!
//! This module covers IP laws including:
//! - Copyright (covered in Civil Code Part 4)
//! - Patent Law
//! - Trademark Law
//!
//! Complements civil_code/part4.rs with additional functionality

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to intellectual property operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum IntellectualPropertyError {
    /// Invalid copyright claim
    #[error("Invalid copyright claim: {0}")]
    InvalidCopyright(String),

    /// Invalid patent
    #[error("Invalid patent: {0}")]
    InvalidPatent(String),

    /// Invalid trademark
    #[error("Invalid trademark: {0}")]
    InvalidTrademark(String),

    /// Infringement detected
    #[error("IP infringement: {0}")]
    Infringement(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Copyright representation (simplified from Civil Code Part 4)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Copyright {
    /// Title of work
    pub title: String,
    /// Author name
    pub author: String,
    /// Creation date
    pub creation_date: chrono::NaiveDate,
    /// Type of work
    pub work_type: WorkType,
    /// Is registered (optional in Russia)
    pub registered: bool,
    /// Registration number if registered
    pub registration_number: Option<String>,
}

/// Types of copyrighted works
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkType {
    /// Literary work
    Literary,
    /// Musical work
    Musical,
    /// Audiovisual work
    Audiovisual,
    /// Computer program
    ComputerProgram,
    /// Database
    Database,
    /// Artistic work
    Artistic,
    /// Other
    Other,
}

impl Copyright {
    /// Creates a new copyright
    pub fn new(
        title: impl Into<String>,
        author: impl Into<String>,
        creation_date: chrono::NaiveDate,
        work_type: WorkType,
    ) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            creation_date,
            work_type,
            registered: false,
            registration_number: None,
        }
    }

    /// Registers the copyright
    pub fn register(mut self, registration_number: impl Into<String>) -> Self {
        self.registered = true;
        self.registration_number = Some(registration_number.into());
        self
    }

    /// Gets protection period (life + 70 years for most works)
    pub fn protection_period_years(&self) -> u32 {
        match self.work_type {
            WorkType::ComputerProgram | WorkType::Database => 70,
            _ => 70, // Life + 70 years
        }
    }

    /// Validates the copyright
    pub fn validate(&self) -> Result<(), IntellectualPropertyError> {
        if self.title.is_empty() {
            return Err(IntellectualPropertyError::InvalidCopyright(
                "Copyright must have a title".to_string(),
            ));
        }

        if self.author.is_empty() {
            return Err(IntellectualPropertyError::InvalidCopyright(
                "Copyright must have an author".to_string(),
            ));
        }

        Ok(())
    }
}

/// Patent representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patent {
    /// Patent number
    pub number: String,
    /// Patent title
    pub title: String,
    /// Inventor(s)
    pub inventors: Vec<String>,
    /// Patent holder
    pub holder: String,
    /// Filing date
    pub filing_date: chrono::NaiveDate,
    /// Grant date
    pub grant_date: Option<chrono::NaiveDate>,
    /// Patent type
    pub patent_type: PatentType,
    /// Is valid
    pub is_valid: bool,
}

/// Types of patents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatentType {
    /// Invention (20 years)
    Invention,
    /// Utility model (10 years)
    UtilityModel,
    /// Industrial design (5 years, renewable)
    IndustrialDesign,
}

impl Patent {
    /// Creates a new patent
    pub fn new(
        number: impl Into<String>,
        title: impl Into<String>,
        holder: impl Into<String>,
        filing_date: chrono::NaiveDate,
        patent_type: PatentType,
    ) -> Self {
        Self {
            number: number.into(),
            title: title.into(),
            inventors: Vec::new(),
            holder: holder.into(),
            filing_date,
            grant_date: None,
            patent_type,
            is_valid: false,
        }
    }

    /// Adds an inventor
    pub fn add_inventor(mut self, inventor: impl Into<String>) -> Self {
        self.inventors.push(inventor.into());
        self
    }

    /// Grants the patent
    pub fn grant(mut self, grant_date: chrono::NaiveDate) -> Self {
        self.grant_date = Some(grant_date);
        self.is_valid = true;
        self
    }

    /// Gets protection period in years
    pub fn protection_period_years(&self) -> u32 {
        match self.patent_type {
            PatentType::Invention => 20,
            PatentType::UtilityModel => 10,
            PatentType::IndustrialDesign => 5,
        }
    }

    /// Checks if patent is still valid
    pub fn is_protection_valid(&self, current_date: &chrono::NaiveDate) -> bool {
        if !self.is_valid {
            return false;
        }

        if let Some(grant_date) = self.grant_date {
            let years_since_grant = current_date.years_since(grant_date).unwrap_or(0);
            years_since_grant < self.protection_period_years()
        } else {
            false
        }
    }

    /// Validates the patent
    pub fn validate(&self) -> Result<(), IntellectualPropertyError> {
        if self.number.is_empty() {
            return Err(IntellectualPropertyError::InvalidPatent(
                "Patent must have a number".to_string(),
            ));
        }

        if self.inventors.is_empty() {
            return Err(IntellectualPropertyError::InvalidPatent(
                "Patent must have at least one inventor".to_string(),
            ));
        }

        Ok(())
    }
}

/// Trademark representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trademark {
    /// Registration number
    pub registration_number: String,
    /// Trademark text/logo
    pub mark: String,
    /// Holder
    pub holder: String,
    /// Registration date
    pub registration_date: chrono::NaiveDate,
    /// Goods and services classes (Nice Classification)
    pub classes: Vec<u32>,
    /// Is valid
    pub is_valid: bool,
}

impl Trademark {
    /// Creates a new trademark
    pub fn new(
        registration_number: impl Into<String>,
        mark: impl Into<String>,
        holder: impl Into<String>,
        registration_date: chrono::NaiveDate,
    ) -> Self {
        Self {
            registration_number: registration_number.into(),
            mark: mark.into(),
            holder: holder.into(),
            registration_date,
            classes: Vec::new(),
            is_valid: true,
        }
    }

    /// Adds a Nice Classification class
    pub fn add_class(mut self, class: u32) -> Self {
        if (1..=45).contains(&class) {
            self.classes.push(class);
        }
        self
    }

    /// Checks if trademark protection is valid (10 years, renewable)
    pub fn is_protection_valid(&self, current_date: &chrono::NaiveDate) -> bool {
        if !self.is_valid {
            return false;
        }

        let years_since_registration = current_date
            .years_since(self.registration_date)
            .unwrap_or(0);

        // Renewable every 10 years
        years_since_registration < 10
    }

    /// Validates the trademark
    pub fn validate(&self) -> Result<(), IntellectualPropertyError> {
        if self.registration_number.is_empty() {
            return Err(IntellectualPropertyError::InvalidTrademark(
                "Trademark must have registration number".to_string(),
            ));
        }

        if self.mark.is_empty() {
            return Err(IntellectualPropertyError::InvalidTrademark(
                "Trademark must have a mark".to_string(),
            ));
        }

        if self.classes.is_empty() {
            return Err(IntellectualPropertyError::InvalidTrademark(
                "Trademark must specify at least one Nice class".to_string(),
            ));
        }

        Ok(())
    }
}

/// Quick validation for copyright
pub fn quick_validate_copyright(copyright: &Copyright) -> Result<(), IntellectualPropertyError> {
    copyright.validate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copyright() {
        let copyright = Copyright::new(
            "My Novel",
            "Tolstoy L.N.",
            chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date"),
            WorkType::Literary,
        )
        .register("RU-2020-001");

        assert!(copyright.registered);
        assert_eq!(copyright.protection_period_years(), 70);
        assert!(copyright.validate().is_ok());
    }

    #[test]
    fn test_patent() {
        let patent = Patent::new(
            "RU-2700000",
            "Новое изобретение",
            "ООО Инноваций",
            chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date"),
            PatentType::Invention,
        )
        .add_inventor("Иванов И.И.")
        .add_inventor("Петров П.П.")
        .grant(chrono::NaiveDate::from_ymd_opt(2021, 1, 1).expect("Valid date"));

        assert_eq!(patent.protection_period_years(), 20);
        assert!(patent.validate().is_ok());

        let current = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).expect("Valid date");
        assert!(patent.is_protection_valid(&current));
    }

    #[test]
    fn test_trademark() {
        let trademark = Trademark::new(
            "RU-500000",
            "BRAND™",
            "ООО Компания",
            chrono::NaiveDate::from_ymd_opt(2020, 1, 1).expect("Valid date"),
        )
        .add_class(9)
        .add_class(35)
        .add_class(42);

        assert_eq!(trademark.classes.len(), 3);
        assert!(trademark.validate().is_ok());

        let current = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).expect("Valid date");
        assert!(trademark.is_protection_valid(&current));
    }
}
