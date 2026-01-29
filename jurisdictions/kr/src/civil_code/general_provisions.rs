//! General Provisions of the Civil Code (민법 총칙)
//!
//! # 민법 총칙편 / General Provisions
//!
//! Articles 1-184 (제1조 - 제184조)
//!
//! Covers:
//! - Legal personality and capacity
//! - Juridical acts
//! - Agency
//! - Period calculation
//! - Extinctive prescription

use crate::common::DateResult;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to general provisions
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GeneralProvisionsError {
    /// Invalid legal capacity
    #[error("Invalid legal capacity: {0}")]
    InvalidCapacity(String),

    /// Invalid juridical act
    #[error("Invalid juridical act: {0}")]
    InvalidJuridicalAct(String),

    /// Agency error
    #[error("Agency error: {0}")]
    AgencyError(String),

    /// Prescription error
    #[error("Prescription error: {0}")]
    PrescriptionError(String),
}

/// Result type for general provisions operations
pub type GeneralProvisionsResult<T> = Result<T, GeneralProvisionsError>;

/// Legal capacity (권리능력)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalCapacity {
    /// Full capacity (완전 능력)
    Full,
    /// Limited capacity - minor (제한 능력 - 미성년자)
    LimitedMinor,
    /// Limited capacity - adult ward (제한 능력 - 피성년후견인)
    LimitedAdultWard,
    /// Limited capacity - person under limited guardianship (한정후견인)
    LimitedGuardianship,
}

/// Natural person (자연인)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalPerson {
    /// Name
    pub name: String,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// Legal capacity
    pub capacity: LegalCapacity,
}

impl NaturalPerson {
    /// Create new natural person
    pub fn new(name: impl Into<String>, date_of_birth: NaiveDate) -> Self {
        let capacity = if Self::is_adult(&date_of_birth) {
            LegalCapacity::Full
        } else {
            LegalCapacity::LimitedMinor
        };

        Self {
            name: name.into(),
            date_of_birth,
            capacity,
        }
    }

    /// Check if person is adult (19 years old in Korea)
    pub fn is_adult(date_of_birth: &NaiveDate) -> bool {
        let today = chrono::Utc::now().date_naive();
        let age = today.years_since(*date_of_birth).unwrap_or(0);
        age >= 19
    }

    /// Check if person is minor
    pub fn is_minor(&self) -> bool {
        !Self::is_adult(&self.date_of_birth)
    }

    /// Get age
    pub fn age(&self) -> u32 {
        let today = chrono::Utc::now().date_naive();
        today.years_since(self.date_of_birth).unwrap_or(0)
    }
}

/// Juridical act (법률행위)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JuridicalAct {
    /// Actor
    pub actor: String,
    /// Date of act
    pub date: NaiveDate,
    /// Description
    pub description: String,
    /// Legal capacity at time of act
    pub actor_capacity: LegalCapacity,
}

/// Validity of juridical act
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActValidity {
    /// Valid (유효)
    Valid,
    /// Voidable (취소할 수 있음)
    Voidable,
    /// Void (무효)
    Void,
}

/// Validate juridical act
pub fn validate_juridical_act(act: &JuridicalAct) -> GeneralProvisionsResult<ActValidity> {
    match act.actor_capacity {
        LegalCapacity::Full => Ok(ActValidity::Valid),
        LegalCapacity::LimitedMinor | LegalCapacity::LimitedAdultWard => {
            // Acts by minors and adult wards are generally voidable
            // unless with legal representative's consent
            Ok(ActValidity::Voidable)
        }
        LegalCapacity::LimitedGuardianship => {
            // Depends on the type of act
            Ok(ActValidity::Voidable)
        }
    }
}

/// Agency type (대리)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgencyType {
    /// Statutory agency (법정대리)
    Statutory,
    /// Voluntary agency (임의대리)
    Voluntary,
}

/// Agency relationship
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agency {
    /// Principal (본인)
    pub principal: String,
    /// Agent (대리인)
    pub agent: String,
    /// Type of agency
    pub agency_type: AgencyType,
    /// Scope of authority
    pub scope: String,
}

/// Validate agency
pub fn validate_agency(agency: &Agency) -> GeneralProvisionsResult<()> {
    if agency.principal.is_empty() {
        return Err(GeneralProvisionsError::AgencyError(
            "Principal cannot be empty".to_string(),
        ));
    }

    if agency.agent.is_empty() {
        return Err(GeneralProvisionsError::AgencyError(
            "Agent cannot be empty".to_string(),
        ));
    }

    if agency.scope.is_empty() {
        return Err(GeneralProvisionsError::AgencyError(
            "Scope of authority cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Extinctive prescription period (소멸시효)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrescriptionPeriod {
    /// Start date
    pub start_date: NaiveDate,
    /// Period in years
    pub years: u32,
    /// Description of the claim
    pub claim_description: String,
}

impl PrescriptionPeriod {
    /// Create new prescription period
    pub fn new(start_date: NaiveDate, years: u32, claim_description: impl Into<String>) -> Self {
        Self {
            start_date,
            years,
            claim_description: claim_description.into(),
        }
    }

    /// Calculate end date
    pub fn end_date(&self) -> DateResult<NaiveDate> {
        crate::common::calculate_deadline(
            self.start_date,
            self.years as i32,
            crate::common::DeadlineType::Years,
        )
    }

    /// Check if prescription has expired
    pub fn is_expired(&self) -> bool {
        if let Ok(end) = self.end_date() {
            let today = chrono::Utc::now().date_naive();
            today > end
        } else {
            false
        }
    }
}

/// Common prescription periods
pub mod prescription_periods {
    /// General claims (일반 채권) - 10 years (Article 162)
    pub const GENERAL_CLAIMS_YEARS: u32 = 10;

    /// Commercial claims (상사 채권) - 5 years
    pub const COMMERCIAL_CLAIMS_YEARS: u32 = 5;

    /// Limitation period for tort claims from date of occurrence (years)
    /// Tort claims (불법행위) - 3 years from knowledge, 10 years from occurrence (Article 766)
    pub const TORT_KNOWLEDGE_YEARS: u32 = 3;
    /// Limitation period for product liability claims (years)
    pub const TORT_OCCURRENCE_YEARS: u32 = 10;

    /// Product liability (제조물책임) - 3 years from knowledge, 10 years from occurrence
    pub const PRODUCT_LIABILITY_KNOWLEDGE_YEARS: u32 = 3;
    /// Limitation period for product liability claims from date of occurrence (years)
    pub const PRODUCT_LIABILITY_OCCURRENCE_YEARS: u32 = 10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_person_adult() {
        if let Some(dob) = NaiveDate::from_ymd_opt(2000, 1, 1) {
            let person = NaturalPerson::new("김철수", dob);
            assert!(person.age() >= 19);
            assert_eq!(person.capacity, LegalCapacity::Full);
        }
    }

    #[test]
    fn test_natural_person_minor() {
        if let Some(dob) = NaiveDate::from_ymd_opt(2010, 1, 1) {
            let person = NaturalPerson::new("박영희", dob);
            assert!(person.is_minor());
            assert_eq!(person.capacity, LegalCapacity::LimitedMinor);
        }
    }

    #[test]
    fn test_validate_juridical_act() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let act = JuridicalAct {
                actor: "김철수".to_string(),
                date,
                description: "매매계약".to_string(),
                actor_capacity: LegalCapacity::Full,
            };

            let result = validate_juridical_act(&act);
            assert!(result.is_ok());
            assert_eq!(result.unwrap_or(ActValidity::Void), ActValidity::Valid);
        }
    }

    #[test]
    fn test_validate_agency() {
        let agency = Agency {
            principal: "김철수".to_string(),
            agent: "박영희".to_string(),
            agency_type: AgencyType::Voluntary,
            scope: "부동산 매매".to_string(),
        };

        let result = validate_agency(&agency);
        assert!(result.is_ok());
    }

    #[test]
    fn test_prescription_period() {
        if let Some(start) = NaiveDate::from_ymd_opt(2014, 1, 1) {
            let prescription = PrescriptionPeriod::new(start, 10, "일반 채권");
            let end = prescription.end_date();
            assert!(end.is_ok());
        }
    }
}
