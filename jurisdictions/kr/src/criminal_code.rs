//! Criminal Code (형법)
//!
//! # 대한민국 형법 / Criminal Code of the Republic of Korea
//!
//! Enacted: 1953
//! Last amendment: 2023
//!
//! Covers:
//! - General principles
//! - Individual crimes
//! - Penalties

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Criminal code errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CriminalCodeError {
    /// Invalid criminal act
    #[error("Invalid criminal act: {0}")]
    InvalidAct(String),

    /// Penalty calculation error
    #[error("Penalty calculation error: {0}")]
    PenaltyError(String),
}

/// Result type for criminal code operations
pub type CriminalCodeResult<T> = Result<T, CriminalCodeError>;

/// Criminal offense category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenseCategory {
    /// Crimes against the state (내란의 죄)
    AgainstState,
    /// Crimes against public safety (공공의 안전을 해하는 죄)
    PublicSafety,
    /// Crimes of falsification (문서에 관한 죄)
    Falsification,
    /// Economic crimes (재산죄)
    Economic,
    /// Crimes against life and body (생명과 신체에 대한 죄)
    LifeAndBody,
    /// Crimes against sexual self-determination (성풍속에 관한 죄)
    SexualCrimes,
    /// Crimes against reputation (명예에 관한 죄)
    Reputation,
    /// Crimes against privacy (비밀침해의 죄)
    Privacy,
}

/// Penalty type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PenaltyType {
    /// Death penalty (사형)
    Death,
    /// Imprisonment (징역)
    Imprisonment,
    /// Imprisonment without labor (금고)
    ImprisonmentWithoutLabor,
    /// Detention (구류)
    Detention,
    /// Fine (벌금)
    Fine,
    /// Minor fine (과료)
    MinorFine,
    /// Confiscation (몰수)
    Confiscation,
}

/// Criminal offense
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriminalOffense {
    /// Offense name
    pub name: String,
    /// Article number
    pub article: u32,
    /// Category
    pub category: OffenseCategory,
    /// Description
    pub description: String,
    /// Penalty type
    pub penalty_type: PenaltyType,
}

impl CriminalOffense {
    /// Create new criminal offense
    pub fn new(
        name: impl Into<String>,
        article: u32,
        category: OffenseCategory,
        description: impl Into<String>,
        penalty_type: PenaltyType,
    ) -> Self {
        Self {
            name: name.into(),
            article,
            category,
            description: description.into(),
            penalty_type,
        }
    }
}

/// Criminal case
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriminalCase {
    /// Case identifier
    pub case_id: String,
    /// Defendant
    pub defendant: String,
    /// Offense
    pub offense: CriminalOffense,
    /// Date of offense
    pub offense_date: NaiveDate,
    /// Statute of limitations (공소시효)
    pub statute_of_limitations_years: u32,
}

impl CriminalCase {
    /// Create new criminal case
    pub fn new(
        case_id: impl Into<String>,
        defendant: impl Into<String>,
        offense: CriminalOffense,
        offense_date: NaiveDate,
        statute_of_limitations_years: u32,
    ) -> Self {
        Self {
            case_id: case_id.into(),
            defendant: defendant.into(),
            offense,
            offense_date,
            statute_of_limitations_years,
        }
    }

    /// Check if statute of limitations has expired
    pub fn is_statute_expired(&self) -> bool {
        let today = chrono::Utc::now().date_naive();
        if let Ok(expiry) = crate::common::calculate_deadline(
            self.offense_date,
            self.statute_of_limitations_years as i32,
            crate::common::DeadlineType::Years,
        ) {
            today > expiry
        } else {
            false
        }
    }
}

/// Statute of limitations periods (공소시효)
pub mod statute_of_limitations {
    /// Death penalty or life imprisonment - 25 years (Article 249)
    pub const DEATH_OR_LIFE: u32 = 25;

    /// Imprisonment 10+ years - 15 years
    pub const IMPRISONMENT_10_PLUS: u32 = 15;

    /// Imprisonment 3-10 years - 10 years
    pub const IMPRISONMENT_3_TO_10: u32 = 10;

    /// Imprisonment under 3 years - 7 years
    pub const IMPRISONMENT_UNDER_3: u32 = 7;

    /// Detention, fine - 5 years
    pub const DETENTION_OR_FINE: u32 = 5;

    /// Minor fine - 3 years
    pub const MINOR_FINE: u32 = 3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criminal_offense_creation() {
        let offense = CriminalOffense::new(
            "절도",
            329,
            OffenseCategory::Economic,
            "타인의 재물을 절취한 자",
            PenaltyType::Imprisonment,
        );

        assert_eq!(offense.name, "절도");
        assert_eq!(offense.article, 329);
    }

    #[test]
    fn test_criminal_case_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let offense = CriminalOffense::new(
                "절도",
                329,
                OffenseCategory::Economic,
                "타인의 재물을 절취한 자",
                PenaltyType::Imprisonment,
            );

            let case = CriminalCase::new("2024-001", "김철수", offense, date, 7);

            assert_eq!(case.case_id, "2024-001");
            assert_eq!(case.statute_of_limitations_years, 7);
        }
    }

    #[test]
    fn test_statute_expired() {
        if let Some(old_date) = NaiveDate::from_ymd_opt(2010, 1, 1) {
            let offense = CriminalOffense::new(
                "절도",
                329,
                OffenseCategory::Economic,
                "타인의 재물을 절취한 자",
                PenaltyType::Imprisonment,
            );

            let case = CriminalCase::new("2010-001", "김철수", offense, old_date, 7);

            assert!(case.is_statute_expired());
        }
    }
}
