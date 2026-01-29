//! Commercial Code (상법)
//!
//! # 대한민국 상법 / Commercial Code of the Republic of Korea
//!
//! Enacted: 1962
//! Last major amendment: 2021
//!
//! Covers:
//! - General provisions on commercial activities
//! - Companies (회사)
//! - Commercial transactions
//! - Bills and notes
//! - Maritime commerce
//! - Insurance

use crate::common::{CompanyName, KrwAmount, OrganizationForm};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Commercial code errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CommercialCodeError {
    /// Invalid company
    #[error("Invalid company: {0}")]
    InvalidCompany(String),

    /// Capital error
    #[error("Capital error: {0}")]
    CapitalError(String),

    /// Corporate governance error
    #[error("Corporate governance error: {0}")]
    GovernanceError(String),
}

/// Result type for commercial code operations
pub type CommercialCodeResult<T> = Result<T, CommercialCodeError>;

/// Company type under Commercial Code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    /// Stock company (주식회사)
    StockCompany,
    /// Limited company (유한회사)
    LimitedCompany,
    /// Limited liability company (유한책임회사)
    Llc,
    /// General partnership (합명회사)
    GeneralPartnership,
    /// Limited partnership (합자회사)
    LimitedPartnership,
}

impl From<OrganizationForm> for CompanyType {
    fn from(form: OrganizationForm) -> Self {
        match form {
            OrganizationForm::StockCompany => CompanyType::StockCompany,
            OrganizationForm::LimitedCompany => CompanyType::LimitedCompany,
            OrganizationForm::Llc => CompanyType::Llc,
            OrganizationForm::GeneralPartnership => CompanyType::GeneralPartnership,
            OrganizationForm::LimitedPartnership => CompanyType::LimitedPartnership,
        }
    }
}

/// Company (회사)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    /// Company name
    pub name: CompanyName,
    /// Company type
    pub company_type: CompanyType,
    /// Registered capital (자본금)
    pub registered_capital: KrwAmount,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Registration number
    pub registration_number: String,
}

impl Company {
    /// Create new company
    pub fn new(
        name: CompanyName,
        registered_capital: KrwAmount,
        registration_date: NaiveDate,
        registration_number: impl Into<String>,
    ) -> Self {
        let company_type = name.organization_form.into();
        Self {
            name,
            company_type,
            registered_capital,
            registration_date,
            registration_number: registration_number.into(),
        }
    }
}

/// Validate company capital requirements
pub fn validate_capital_requirements(
    company_type: CompanyType,
    _capital: &KrwAmount,
) -> CommercialCodeResult<()> {
    match company_type {
        CompanyType::StockCompany => {
            // No minimum capital requirement as of 2022 amendment (Article 329 deleted)
            Ok(())
        }
        CompanyType::LimitedCompany => {
            // No minimum capital requirement
            Ok(())
        }
        CompanyType::Llc => {
            // No specific minimum
            Ok(())
        }
        CompanyType::GeneralPartnership | CompanyType::LimitedPartnership => {
            // No specific minimum
            Ok(())
        }
    }
}

/// Director (이사)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Director {
    /// Name
    pub name: String,
    /// Position
    pub position: String,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// Term (years)
    pub term_years: u32,
}

impl Director {
    /// Create new director
    pub fn new(
        name: impl Into<String>,
        position: impl Into<String>,
        appointment_date: NaiveDate,
        term_years: u32,
    ) -> Self {
        Self {
            name: name.into(),
            position: position.into(),
            appointment_date,
            term_years,
        }
    }

    /// Check if term has expired
    pub fn is_term_expired(&self) -> bool {
        if let Ok(end) = crate::common::calculate_deadline(
            self.appointment_date,
            self.term_years as i32,
            crate::common::DeadlineType::Years,
        ) {
            let today = chrono::Utc::now().date_naive();
            today > end
        } else {
            false
        }
    }
}

/// Board of Directors (이사회)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardOfDirectors {
    /// Directors
    pub directors: Vec<Director>,
    /// Quorum (의결 정족수)
    pub quorum: u32,
}

impl BoardOfDirectors {
    /// Create new board
    pub fn new(quorum: u32) -> Self {
        Self {
            directors: Vec::new(),
            quorum,
        }
    }

    /// Add director
    pub fn add_director(mut self, director: Director) -> Self {
        self.directors.push(director);
        self
    }

    /// Check if quorum is met
    pub fn is_quorum_met(&self, present: u32) -> bool {
        present >= self.quorum
    }
}

/// Validate board composition for stock company
/// Article 383: Stock company must have 3+ directors
pub fn validate_board_composition(
    company_type: CompanyType,
    directors: &[Director],
) -> CommercialCodeResult<()> {
    match company_type {
        CompanyType::StockCompany => {
            if directors.len() < 3 {
                return Err(CommercialCodeError::GovernanceError(
                    "Stock company must have at least 3 directors".to_string(),
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let company_name = CompanyName::new("테스트", OrganizationForm::StockCompany);
            let company = Company::new(
                company_name,
                KrwAmount::from_man(1_000.0),
                date,
                "110111-1234567",
            );

            assert_eq!(company.company_type, CompanyType::StockCompany);
        }
    }

    #[test]
    fn test_validate_capital_requirements() {
        let capital = KrwAmount::from_man(500.0);
        let result = validate_capital_requirements(CompanyType::StockCompany, &capital);
        assert!(result.is_ok());
    }

    #[test]
    fn test_director_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let director = Director::new("김철수", "대표이사", date, 3);
            assert_eq!(director.name, "김철수");
            assert_eq!(director.term_years, 3);
        }
    }

    #[test]
    fn test_board_of_directors() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let board = BoardOfDirectors::new(3)
                .add_director(Director::new("김철수", "대표이사", date, 3))
                .add_director(Director::new("박영희", "이사", date, 3))
                .add_director(Director::new("이민호", "이사", date, 3));

            assert_eq!(board.directors.len(), 3);
            assert!(board.is_quorum_met(3));
        }
    }

    #[test]
    fn test_validate_board_composition() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let directors = vec![
                Director::new("김철수", "대표이사", date, 3),
                Director::new("박영희", "이사", date, 3),
                Director::new("이민호", "이사", date, 3),
            ];

            let result = validate_board_composition(CompanyType::StockCompany, &directors);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_validate_board_composition_insufficient() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let directors = vec![
                Director::new("김철수", "대표이사", date, 3),
                Director::new("박영희", "이사", date, 3),
            ];

            let result = validate_board_composition(CompanyType::StockCompany, &directors);
            assert!(result.is_err());
        }
    }
}
