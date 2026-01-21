//! Book I: General Provisions (ບົດບັນຍັດທົ່ວໄປ) - Articles 1-161
//!
//! This module implements the general provisions of the Lao Civil Code 2020,
//! covering basic principles, legal capacity, juristic acts, agency, and periods.
//!
//! ## Structure
//! - Chapter 1: Basic Principles (Articles 1-19)
//! - Chapter 2: Legal Capacity (Articles 20-40)
//! - Chapter 3: Juristic Acts (Articles 41-100)
//! - Chapter 4: Agency (Articles 101-140)
//! - Chapter 5: Period of Time (Articles 141-161)
//!
//! ## Comparative Law Notes
//! - Heavily influenced by Japanese Civil Code Book I (総則編)
//! - Basic principles adapted from French Code civil Article 1-6
//! - Legal capacity system follows Japanese model (行為能力制度)

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for general provisions
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum GeneralProvisionsError {
    #[error("Lack of legal capacity: {0}")]
    LackOfCapacity(String),

    #[error("Invalid juristic act: {0}")]
    InvalidJuristicAct(String),

    #[error("Agency authority exceeded: {0}")]
    AgencyAuthorityExceeded(String),

    #[error("Invalid period: {0}")]
    InvalidPeriod(String),

    #[error("Good faith violation: {0}")]
    GoodFaithViolation(String),
}

pub type Result<T> = std::result::Result<T, GeneralProvisionsError>;

/// Article 1: Basic Principle - Protection of Rights
///
/// Civil rights shall be protected by law. No person shall abuse their rights.
///
/// Comparative: Japanese Civil Code Article 1(1), French Code civil Article 4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicPrinciple {
    pub protection_of_rights: bool,
    pub prohibition_of_abuse: bool,
}

/// Article 1: Validates basic principle of rights protection
///
/// # Japanese Influence
/// Follows Japanese Civil Code Article 1(1): "私権は、公共の福祉に適合しなければならない"
/// (Private rights must conform to the public welfare)
pub fn article1(principle: &BasicPrinciple) -> Result<()> {
    if !principle.protection_of_rights {
        return Err(GeneralProvisionsError::GoodFaithViolation(
            "Civil rights must be protected".to_string(),
        ));
    }
    if !principle.prohibition_of_abuse {
        return Err(GeneralProvisionsError::GoodFaithViolation(
            "Abuse of rights is prohibited".to_string(),
        ));
    }
    Ok(())
}

/// Article 3: Good Faith Principle
///
/// The exercise of rights and performance of obligations must be made in good faith.
///
/// Comparative: Japanese Civil Code Article 1(2), French Code civil Article 1104
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodFaithPrinciple {
    pub exercise_in_good_faith: bool,
    pub performance_in_good_faith: bool,
}

/// Article 3: Validates good faith principle
///
/// # Japanese Influence
/// Directly based on Japanese Civil Code Article 1(2): "信義誠実の原則"
/// (Principle of good faith and sincerity)
pub fn article3(principle: &GoodFaithPrinciple) -> Result<()> {
    if !principle.exercise_in_good_faith {
        return Err(GeneralProvisionsError::GoodFaithViolation(
            "Rights must be exercised in good faith".to_string(),
        ));
    }
    if !principle.performance_in_good_faith {
        return Err(GeneralProvisionsError::GoodFaithViolation(
            "Obligations must be performed in good faith".to_string(),
        ));
    }
    Ok(())
}

/// Legal capacity status under Lao law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapacityStatus {
    /// Full capacity (age 18+)
    Full,
    /// Minor (under 18)
    Minor,
    /// Adult under guardianship
    UnderGuardianship,
}

/// Article 20: Legal Capacity
///
/// A person who has attained the age of eighteen years has full legal capacity.
///
/// Comparative: Japanese Civil Code Article 4 (age 18 since 2022 reform)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalCapacity {
    pub age: u32,
    pub status: CapacityStatus,
    pub guardian: Option<String>,
}

/// Article 20: Validates legal capacity based on age
///
/// # Japanese Influence
/// Follows Japanese 2022 reform lowering age of majority from 20 to 18
/// (成年年齢の引下げ - Civil Code Article 4 amendment)
pub fn article20(capacity: &LegalCapacity) -> Result<CapacityStatus> {
    if capacity.age >= 18 && capacity.guardian.is_none() {
        Ok(CapacityStatus::Full)
    } else if capacity.age < 18 {
        Ok(CapacityStatus::Minor)
    } else if capacity.guardian.is_some() {
        Ok(CapacityStatus::UnderGuardianship)
    } else {
        Ok(CapacityStatus::Full)
    }
}

/// Article 21: Protection of Minors
///
/// A minor cannot perform juristic acts without consent of their legal representative.
///
/// Comparative: Japanese Civil Code Article 5
pub fn article21(capacity: &LegalCapacity) -> Result<bool> {
    match article20(capacity)? {
        CapacityStatus::Minor => {
            if capacity.guardian.is_some() {
                Ok(true) // Requires consent
            } else {
                Err(GeneralProvisionsError::LackOfCapacity(
                    "Minor lacks legal representative".to_string(),
                ))
            }
        }
        CapacityStatus::UnderGuardianship => Ok(true),
        CapacityStatus::Full => Ok(false), // No consent required
    }
}

/// Validates legal capacity for juristic acts
pub fn validate_legal_capacity(capacity: &LegalCapacity) -> Result<()> {
    let status = article20(capacity)?;

    if (status == CapacityStatus::Minor || status == CapacityStatus::UnderGuardianship)
        && capacity.guardian.is_none()
    {
        return Err(GeneralProvisionsError::LackOfCapacity(
            "Person under legal protection requires guardian".to_string(),
        ));
    }

    Ok(())
}

/// Type of juristic act
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JuristicActType {
    /// Unilateral act (e.g., will, waiver)
    Unilateral,
    /// Bilateral act (e.g., contract)
    Bilateral,
    /// Multilateral act (e.g., partnership agreement)
    Multilateral,
}

/// Defect in juristic act
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JuristicActDefect {
    /// Lack of capacity
    LackOfCapacity,
    /// Mistake in intention
    Mistake,
    /// Fraud
    Fraud,
    /// Duress
    Duress,
    /// Violation of public order or morals
    PublicPolicyViolation,
    /// Impossibility
    Impossibility,
}

/// Article 41-100: Juristic Act
///
/// A juristic act is an act intended to bring about legal effects.
///
/// Comparative: Japanese Civil Code Chapter 5 (法律行為), French Code civil Articles 1100-1107
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JuristicAct {
    pub act_type: JuristicActType,
    pub parties: Vec<String>,
    pub intention: String,
    pub capacity_verified: bool,
    pub free_consent: bool,
    pub lawful_purpose: bool,
    pub possible: bool,
    pub defects: Vec<JuristicActDefect>,
}

/// Validates juristic act requirements
///
/// # Requirements (Articles 41-100)
/// 1. Capacity of parties
/// 2. Free consent
/// 3. Lawful purpose
/// 4. Possibility of performance
/// 5. No defects vitiating consent
pub fn validate_juristic_act(act: &JuristicAct) -> Result<()> {
    // Check capacity
    if !act.capacity_verified {
        return Err(GeneralProvisionsError::InvalidJuristicAct(
            "Capacity of parties not verified".to_string(),
        ));
    }

    // Check free consent
    if !act.free_consent {
        return Err(GeneralProvisionsError::InvalidJuristicAct(
            "Consent not freely given".to_string(),
        ));
    }

    // Check lawful purpose
    if !act.lawful_purpose {
        return Err(GeneralProvisionsError::InvalidJuristicAct(
            "Purpose violates law or public policy".to_string(),
        ));
    }

    // Check possibility
    if !act.possible {
        return Err(GeneralProvisionsError::InvalidJuristicAct(
            "Performance is impossible".to_string(),
        ));
    }

    // Check for defects
    for defect in &act.defects {
        match defect {
            JuristicActDefect::LackOfCapacity => {
                return Err(GeneralProvisionsError::LackOfCapacity(
                    "Party lacks legal capacity".to_string(),
                ));
            }
            JuristicActDefect::Fraud => {
                return Err(GeneralProvisionsError::InvalidJuristicAct(
                    "Act vitiated by fraud".to_string(),
                ));
            }
            JuristicActDefect::Duress => {
                return Err(GeneralProvisionsError::InvalidJuristicAct(
                    "Act vitiated by duress".to_string(),
                ));
            }
            JuristicActDefect::PublicPolicyViolation => {
                return Err(GeneralProvisionsError::InvalidJuristicAct(
                    "Act violates public policy".to_string(),
                ));
            }
            JuristicActDefect::Impossibility => {
                return Err(GeneralProvisionsError::InvalidJuristicAct(
                    "Performance is impossible".to_string(),
                ));
            }
            _ => {}
        }
    }

    // Verify parties based on act type
    match act.act_type {
        JuristicActType::Unilateral => {
            if act.parties.len() != 1 {
                return Err(GeneralProvisionsError::InvalidJuristicAct(
                    "Unilateral act must have one party".to_string(),
                ));
            }
        }
        JuristicActType::Bilateral => {
            if act.parties.len() != 2 {
                return Err(GeneralProvisionsError::InvalidJuristicAct(
                    "Bilateral act must have two parties".to_string(),
                ));
            }
        }
        JuristicActType::Multilateral => {
            if act.parties.len() < 3 {
                return Err(GeneralProvisionsError::InvalidJuristicAct(
                    "Multilateral act must have at least three parties".to_string(),
                ));
            }
        }
    }

    Ok(())
}

/// Type of agency authority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgencyAuthority {
    /// General authority for all acts
    General,
    /// Special authority for specific acts
    Special(Vec<String>),
    /// Commercial agency
    Commercial,
}

/// Article 101-140: Agency (ການແທນຕົວ)
///
/// An agent may perform juristic acts on behalf of a principal within their authority.
///
/// Comparative: Japanese Civil Code Articles 99-118 (代理), French Code civil Articles 1984-2010
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agency {
    pub principal: String,
    pub agent: String,
    pub authority: AgencyAuthority,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub in_good_faith: bool,
}

/// Validates agency authority for specific act
pub fn validate_agency_authority(agency: &Agency, act: &str) -> Result<()> {
    // Check expiration
    if let Some(expires) = agency.expires_at
        && Utc::now() > expires
    {
        return Err(GeneralProvisionsError::AgencyAuthorityExceeded(
            "Agency authority has expired".to_string(),
        ));
    }

    // Check good faith
    if !agency.in_good_faith {
        return Err(GeneralProvisionsError::GoodFaithViolation(
            "Agent not acting in good faith".to_string(),
        ));
    }

    // Check authority scope
    match &agency.authority {
        AgencyAuthority::General => Ok(()),
        AgencyAuthority::Special(acts) => {
            if acts.contains(&act.to_string()) {
                Ok(())
            } else {
                Err(GeneralProvisionsError::AgencyAuthorityExceeded(format!(
                    "Agent lacks authority for act: {}",
                    act
                )))
            }
        }
        AgencyAuthority::Commercial => {
            // Commercial agency has broad authority for business acts
            Ok(())
        }
    }
}

/// Article 141-161: Period of Time (ກຳນົດເວລາ)
///
/// Calculation of periods for legal purposes.
///
/// Comparative: Japanese Civil Code Articles 138-143, French Code civil Articles 2224-2227
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    pub start: DateTime<Utc>,
    pub duration_days: i64,
    pub exclude_holidays: bool,
}

impl Period {
    pub fn new(start: DateTime<Utc>, duration_days: i64) -> Self {
        Self {
            start,
            duration_days,
            exclude_holidays: false,
        }
    }

    /// Calculate end date of period
    pub fn end_date(&self) -> DateTime<Utc> {
        self.start + Duration::days(self.duration_days)
    }

    /// Check if period has elapsed
    pub fn has_elapsed(&self) -> bool {
        Utc::now() >= self.end_date()
    }

    /// Days remaining in period
    pub fn days_remaining(&self) -> i64 {
        let end = self.end_date();
        let now = Utc::now();
        if now >= end {
            0
        } else {
            (end - now).num_days()
        }
    }
}

/// Validates period calculation
pub fn validate_period(period: &Period) -> Result<()> {
    if period.duration_days < 0 {
        return Err(GeneralProvisionsError::InvalidPeriod(
            "Period duration cannot be negative".to_string(),
        ));
    }

    if period.start > Utc::now() + Duration::days(365 * 100) {
        return Err(GeneralProvisionsError::InvalidPeriod(
            "Period start date is too far in future".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article1_basic_principle() {
        let principle = BasicPrinciple {
            protection_of_rights: true,
            prohibition_of_abuse: true,
        };
        assert!(article1(&principle).is_ok());

        let bad_principle = BasicPrinciple {
            protection_of_rights: true,
            prohibition_of_abuse: false,
        };
        assert!(article1(&bad_principle).is_err());
    }

    #[test]
    fn test_article3_good_faith() {
        let principle = GoodFaithPrinciple {
            exercise_in_good_faith: true,
            performance_in_good_faith: true,
        };
        assert!(article3(&principle).is_ok());
    }

    #[test]
    fn test_article20_legal_capacity() {
        // Full capacity at age 18
        let adult = LegalCapacity {
            age: 18,
            status: CapacityStatus::Full,
            guardian: None,
        };
        assert_eq!(article20(&adult).unwrap(), CapacityStatus::Full);

        // Minor under 18
        let minor = LegalCapacity {
            age: 15,
            status: CapacityStatus::Minor,
            guardian: Some("Parent".to_string()),
        };
        assert_eq!(article20(&minor).unwrap(), CapacityStatus::Minor);
    }

    #[test]
    fn test_article21_minor_protection() {
        let minor = LegalCapacity {
            age: 15,
            status: CapacityStatus::Minor,
            guardian: Some("Parent".to_string()),
        };
        // Minor requires consent
        assert!(article21(&minor).unwrap());

        let adult = LegalCapacity {
            age: 20,
            status: CapacityStatus::Full,
            guardian: None,
        };
        // Adult does not require consent
        assert!(!article21(&adult).unwrap());
    }

    #[test]
    fn test_validate_juristic_act() {
        let valid_act = JuristicAct {
            act_type: JuristicActType::Bilateral,
            parties: vec!["Party A".to_string(), "Party B".to_string()],
            intention: "Contract for sale".to_string(),
            capacity_verified: true,
            free_consent: true,
            lawful_purpose: true,
            possible: true,
            defects: vec![],
        };
        assert!(validate_juristic_act(&valid_act).is_ok());

        // Act with fraud defect
        let fraud_act = JuristicAct {
            defects: vec![JuristicActDefect::Fraud],
            ..valid_act.clone()
        };
        assert!(validate_juristic_act(&fraud_act).is_err());
    }

    #[test]
    fn test_agency_authority() {
        let agency = Agency {
            principal: "Principal".to_string(),
            agent: "Agent".to_string(),
            authority: AgencyAuthority::Special(vec!["buy".to_string(), "sell".to_string()]),
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + Duration::days(30)),
            in_good_faith: true,
        };

        assert!(validate_agency_authority(&agency, "buy").is_ok());
        assert!(validate_agency_authority(&agency, "lease").is_err());
    }

    #[test]
    fn test_period_calculation() {
        let start = Utc::now();
        let period = Period::new(start, 30);

        assert_eq!(period.end_date(), start + Duration::days(30));
        assert!(!period.has_elapsed());
        // Days remaining may be 29 or 30 due to timing
        let days = period.days_remaining();
        assert!((29..=30).contains(&days));

        assert!(validate_period(&period).is_ok());
    }
}
