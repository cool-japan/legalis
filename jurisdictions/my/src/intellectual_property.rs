//! Intellectual Property Laws in Malaysia
//!
//! # Key Legislation
//!
//! - **Patents Act 1983**: Patent protection (20 years)
//! - **Trade Marks Act 1976**: Trademark registration and protection
//! - **Copyright Act 1987**: Copyright protection
//! - **Industrial Designs Act 1996**: Design registration
//!
//! # Administration
//!
//! - **MyIPO**: Intellectual Property Corporation of Malaysia

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// IP error types.
#[derive(Debug, Error)]
pub enum IpError {
    /// Invalid patent application.
    #[error("Invalid patent application: {reason}")]
    InvalidPatent { reason: String },

    /// Invalid trademark application.
    #[error("Invalid trademark application: {reason}")]
    InvalidTrademark { reason: String },

    /// Copyright infringement.
    #[error("Copyright infringement: {description}")]
    CopyrightInfringement { description: String },
}

/// Result type for IP operations.
pub type Result<T> = std::result::Result<T, IpError>;

/// Patent under Patents Act 1983.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Patent {
    /// Patent ID.
    pub id: Uuid,
    /// Patent title.
    pub title: String,
    /// Inventor name(s).
    pub inventors: Vec<String>,
    /// Patent application number.
    pub application_number: Option<String>,
    /// Filing date.
    pub filing_date: DateTime<Utc>,
    /// Whether patent is granted.
    pub granted: bool,
    /// Grant date.
    pub grant_date: Option<DateTime<Utc>>,
}

impl Patent {
    /// Creates a new patent application.
    #[must_use]
    pub fn new(title: impl Into<String>, inventors: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            inventors,
            application_number: None,
            filing_date: Utc::now(),
            granted: false,
            grant_date: None,
        }
    }
}

/// Trademark under Trade Marks Act 1976.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trademark {
    /// Trademark ID.
    pub id: Uuid,
    /// Trademark name/logo.
    pub mark: String,
    /// Class of goods/services (Nice Classification).
    pub class: u8,
    /// Owner name.
    pub owner: String,
    /// Application number.
    pub application_number: Option<String>,
    /// Filing date.
    pub filing_date: DateTime<Utc>,
    /// Whether trademark is registered.
    pub registered: bool,
}

impl Trademark {
    /// Creates a new trademark application.
    #[must_use]
    pub fn new(mark: impl Into<String>, class: u8, owner: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            mark: mark.into(),
            class,
            owner: owner.into(),
            application_number: None,
            filing_date: Utc::now(),
            registered: false,
        }
    }
}

/// Copyright under Copyright Act 1987.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Copyright {
    /// Copyright ID.
    pub id: Uuid,
    /// Work title.
    pub title: String,
    /// Work type.
    pub work_type: WorkType,
    /// Author/creator name.
    pub author: String,
    /// Year of creation.
    pub year: u16,
    /// Whether copyright is registered (voluntary in Malaysia).
    pub registered: bool,
}

/// Type of copyrighted work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkType {
    /// Literary work.
    Literary,
    /// Musical work.
    Musical,
    /// Artistic work.
    Artistic,
    /// Film.
    Film,
    /// Sound recording.
    SoundRecording,
    /// Broadcast.
    Broadcast,
}

impl Copyright {
    /// Creates a new copyright registration.
    #[must_use]
    pub fn new(
        title: impl Into<String>,
        work_type: WorkType,
        author: impl Into<String>,
        year: u16,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            work_type,
            author: author.into(),
            year,
            registered: false,
        }
    }
}

/// Industrial design under Industrial Designs Act 1996.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndustrialDesign {
    /// Design ID.
    pub id: Uuid,
    /// Design title/description.
    pub title: String,
    /// Designer name.
    pub designer: String,
    /// Application number.
    pub application_number: Option<String>,
    /// Filing date.
    pub filing_date: DateTime<Utc>,
    /// Whether design is registered.
    pub registered: bool,
}

impl IndustrialDesign {
    /// Creates a new industrial design application.
    #[must_use]
    pub fn new(title: impl Into<String>, designer: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            designer: designer.into(),
            application_number: None,
            filing_date: Utc::now(),
            registered: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_creation() {
        let patent = Patent::new(
            "Innovative Widget",
            vec!["Dr. Ahmad".to_string(), "Dr. Siti".to_string()],
        );

        assert_eq!(patent.title, "Innovative Widget");
        assert_eq!(patent.inventors.len(), 2);
        assert!(!patent.granted);
    }

    #[test]
    fn test_trademark_creation() {
        let trademark = Trademark::new("TechBrand", 9, "Tech Sdn Bhd"); // Class 9: Computer software

        assert_eq!(trademark.mark, "TechBrand");
        assert_eq!(trademark.class, 9);
        assert!(!trademark.registered);
    }

    #[test]
    fn test_copyright_creation() {
        let copyright = Copyright::new("My Novel", WorkType::Literary, "Ahmad bin Ali", 2024);

        assert_eq!(copyright.title, "My Novel");
        assert_eq!(copyright.work_type, WorkType::Literary);
        assert!(!copyright.registered);
    }
}
