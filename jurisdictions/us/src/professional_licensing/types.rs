//! Common types for professional licensing across jurisdictions
//!
//! This module provides shared types used across different professional
//! licensing domains (attorneys, physicians, architects, etc.).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// State identifier (using existing state IDs from states module)
pub type StateId = crate::states::types::StateId;

/// Type of professional license
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LicenseType {
    /// Attorney license (bar admission)
    Attorney,
    /// Medical doctor (MD or DO)
    Physician,
    /// Registered nurse
    Nurse,
    /// Licensed architect
    Architect,
    /// Certified public accountant
    CPA,
    /// Licensed engineer
    Engineer,
    /// Licensed clinical social worker
    SocialWorker,
    /// Other professional license
    Other(String),
}

/// Reciprocity arrangements between states
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReciprocityType {
    /// Full reciprocity - automatic recognition
    Full,
    /// Conditional reciprocity - requires additional steps
    Conditional {
        /// Additional requirements (e.g., state exam, CLE)
        requirements: Vec<String>,
    },
    /// Compact-based reciprocity (e.g., IMLC, Nurse Licensure Compact)
    Compact {
        /// Name of the compact
        compact_name: String,
        /// Member states
        member_states: Vec<String>,
    },
    /// Score transfer (e.g., UBE for attorneys)
    ScoreTransfer {
        /// Minimum score required
        minimum_score: u16,
        /// Additional requirements
        additional_requirements: Vec<String>,
    },
    /// No reciprocity - must complete full licensing process
    None,
}

/// Licensing authority for a jurisdiction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicensingAuthority {
    /// State identifier
    pub state_id: StateId,
    /// License type
    pub license_type: LicenseType,
    /// Name of the licensing authority
    pub authority_name: String,
    /// Website URL
    pub website_url: Option<String>,
    /// Contact information
    pub contact_info: Option<ContactInfo>,
}

/// Contact information for licensing authority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Mailing address
    pub address: Option<String>,
    /// Phone number
    pub phone: Option<String>,
    /// Email address
    pub email: Option<String>,
}

/// Professional license
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfessionalLicense {
    /// License number
    pub license_number: String,
    /// Issuing state
    pub issuing_state: StateId,
    /// License type
    pub license_type: LicenseType,
    /// Issue date
    pub issue_date: DateTime<Utc>,
    /// Expiration date
    pub expiration_date: Option<DateTime<Utc>>,
    /// Current status
    pub status: LicenseStatus,
    /// License holder name
    pub holder_name: String,
    /// Disciplinary actions
    pub disciplinary_actions: Vec<DisciplinaryAction>,
}

/// License status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseStatus {
    /// Active and in good standing
    Active,
    /// Expired but renewable
    Expired,
    /// Inactive (voluntary)
    Inactive,
    /// Suspended
    Suspended {
        /// Reason for suspension
        reason: String,
        /// End date of suspension
        end_date: Option<DateTime<Utc>>,
    },
    /// Revoked
    Revoked {
        /// Reason for revocation
        reason: String,
        /// Date of revocation
        revocation_date: DateTime<Utc>,
    },
    /// Under investigation
    UnderInvestigation,
}

/// Disciplinary action against a license
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisciplinaryAction {
    /// Date of action
    pub action_date: DateTime<Utc>,
    /// Type of action
    pub action_type: DisciplinaryActionType,
    /// Description
    pub description: String,
    /// Resolution (if any)
    pub resolution: Option<String>,
}

/// Types of disciplinary actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisciplinaryActionType {
    /// Written warning
    Warning,
    /// Monetary fine
    Fine { amount: u64 },
    /// Suspension
    Suspension { duration_days: u32 },
    /// Probation
    Probation { duration_days: u32 },
    /// Revocation
    Revocation,
    /// Reprimand
    Reprimand,
    /// Continuing education requirement
    ContinuingEducation { hours: u16 },
}

impl ProfessionalLicense {
    /// Check if license is currently valid
    pub fn is_valid(&self) -> bool {
        matches!(self.status, LicenseStatus::Active)
            && self.expiration_date.is_none_or(|exp| exp > Utc::now())
    }

    /// Check if license is expired
    pub fn is_expired(&self) -> bool {
        self.expiration_date.is_some_and(|exp| exp <= Utc::now())
    }

    /// Days until expiration (negative if expired)
    pub fn days_until_expiration(&self) -> Option<i64> {
        self.expiration_date
            .map(|exp| (exp - Utc::now()).num_days())
    }

    /// Check if license has disciplinary history
    pub fn has_disciplinary_history(&self) -> bool {
        !self.disciplinary_actions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_license_validity() {
        let license = ProfessionalLicense {
            license_number: "12345".to_string(),
            issuing_state: StateId::from_code("NY"),
            license_type: LicenseType::Attorney,
            issue_date: Utc::now() - Duration::days(365),
            expiration_date: Some(Utc::now() + Duration::days(365)),
            status: LicenseStatus::Active,
            holder_name: "John Doe".to_string(),
            disciplinary_actions: vec![],
        };

        assert!(license.is_valid());
        assert!(!license.is_expired());
        assert!(license.days_until_expiration().unwrap() > 0);
    }

    #[test]
    fn test_expired_license() {
        let license = ProfessionalLicense {
            license_number: "12345".to_string(),
            issuing_state: StateId::from_code("CA"),
            license_type: LicenseType::Physician,
            issue_date: Utc::now() - Duration::days(730),
            expiration_date: Some(Utc::now() - Duration::days(30)),
            status: LicenseStatus::Active,
            holder_name: "Jane Smith".to_string(),
            disciplinary_actions: vec![],
        };

        assert!(!license.is_valid()); // Expired even if status is Active
        assert!(license.is_expired());
        assert!(license.days_until_expiration().unwrap() < 0);
    }

    #[test]
    fn test_suspended_license() {
        let license = ProfessionalLicense {
            license_number: "67890".to_string(),
            issuing_state: StateId::from_code("TX"),
            license_type: LicenseType::Attorney,
            issue_date: Utc::now() - Duration::days(1000),
            expiration_date: Some(Utc::now() + Duration::days(365)),
            status: LicenseStatus::Suspended {
                reason: "Ethics violation".to_string(),
                end_date: Some(Utc::now() + Duration::days(180)),
            },
            holder_name: "Bob Johnson".to_string(),
            disciplinary_actions: vec![],
        };

        assert!(!license.is_valid()); // Not valid if suspended
    }

    #[test]
    fn test_disciplinary_history() {
        let mut license = ProfessionalLicense {
            license_number: "11111".to_string(),
            issuing_state: StateId::from_code("FL"),
            license_type: LicenseType::Architect,
            issue_date: Utc::now() - Duration::days(500),
            expiration_date: Some(Utc::now() + Duration::days(500)),
            status: LicenseStatus::Active,
            holder_name: "Alice Williams".to_string(),
            disciplinary_actions: vec![],
        };

        assert!(!license.has_disciplinary_history());

        license.disciplinary_actions.push(DisciplinaryAction {
            action_date: Utc::now() - Duration::days(100),
            action_type: DisciplinaryActionType::Warning,
            description: "Minor violation".to_string(),
            resolution: Some("Resolved".to_string()),
        });

        assert!(license.has_disciplinary_history());
    }

    #[test]
    fn test_reciprocity_types() {
        let full = ReciprocityType::Full;
        assert_eq!(full, ReciprocityType::Full);

        let conditional = ReciprocityType::Conditional {
            requirements: vec!["Pass state exam".to_string()],
        };
        assert!(matches!(conditional, ReciprocityType::Conditional { .. }));

        let score_transfer = ReciprocityType::ScoreTransfer {
            minimum_score: 270,
            additional_requirements: vec!["NY Law Exam".to_string()],
        };
        assert!(matches!(
            score_transfer,
            ReciprocityType::ScoreTransfer { .. }
        ));
    }
}
