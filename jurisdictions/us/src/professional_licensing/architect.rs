//! Architect Licensing and NCARB Certification
//!
//! This module handles architect licensing across US jurisdictions, focusing
//! on the National Council of Architectural Registration Boards (NCARB) system.
//!
//! # NCARB Certificate
//!
//! The NCARB Certificate is a credential that facilitates reciprocal licensure
//! across state lines. It validates that an architect has:
//! - Met education requirements (NAAB-accredited degree)
//! - Completed experience requirements (Architectural Experience Program - AXP)
//! - Passed the Architect Registration Examination (ARE)
//! - Maintained good standing
//!
//! ## Benefits
//! - Expedited reciprocal licensing in 54 US jurisdictions
//! - Streamlined application process
//! - Credential verification services
//! - International practice support (via mutual recognition agreements)
//!
//! ## Reciprocity
//! Most US jurisdictions grant reciprocal licensure to NCARB Certificate holders,
//! though some require additional state-specific exams or experience.

use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// NCARB reciprocity status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NCARBStatus {
    /// Fully reciprocal - NCARB Certificate holders can obtain license
    FullReciprocity {
        /// Additional requirements (if any)
        additional_requirements: Vec<String>,
    },
    /// Conditional reciprocity - additional steps required
    ConditionalReciprocity {
        /// Required conditions
        conditions: Vec<String>,
    },
    /// No reciprocity - full examination required
    NoReciprocity,
}

/// Architect licensing requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchitectLicensing {
    /// State identifier
    pub state_id: StateId,
    /// NCARB reciprocity status
    pub ncarb_status: NCARBStatus,
    /// Education requirements
    pub education_requirements: EducationRequirements,
    /// Experience requirements
    pub experience_requirements: ExperienceRequirements,
    /// Examination requirements
    pub examination_requirements: ExaminationRequirements,
    /// Continuing education required
    pub continuing_education: Option<ContinuingEducation>,
}

/// Education requirements for licensure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EducationRequirements {
    /// NAAB-accredited degree required
    pub naab_accredited_required: bool,
    /// Alternative pathways available
    pub alternative_pathways: Vec<String>,
    /// Minimum degree level
    pub minimum_degree: DegreeLevel,
}

/// Degree level requirements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DegreeLevel {
    /// Bachelor's degree in architecture (5-year B.Arch)
    Bachelor,
    /// Master's degree in architecture (M.Arch)
    Master,
    /// Either bachelor's or master's
    BachelorOrMaster,
}

/// Experience requirements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperienceRequirements {
    /// Hours of experience required
    pub required_hours: u32,
    /// AXP (Architectural Experience Program) required
    pub axp_required: bool,
    /// Supervised experience required
    pub supervision_required: bool,
    /// Diversified experience across practice areas
    pub diversified_experience: bool,
}

/// Examination requirements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExaminationRequirements {
    /// ARE (Architect Registration Examination) required
    pub are_required: bool,
    /// ARE version accepted (5.0, 4.0, etc.)
    pub are_version: String,
    /// State-specific examination
    pub state_specific_exam: Option<String>,
}

/// Continuing education requirements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuingEducation {
    /// Hours required per renewal period
    pub hours_required: u16,
    /// Renewal period (years)
    pub renewal_period_years: u8,
    /// Health/safety/welfare (HSW) hours required
    pub hsw_hours_required: Option<u16>,
}

/// Check if NCARB Certificate can be used in a state
///
/// # Example
/// ```
/// use legalis_us::professional_licensing::architect::can_use_ncarb_certificate;
///
/// assert!(can_use_ncarb_certificate("TX"));
/// assert!(can_use_ncarb_certificate("NY"));
/// assert!(can_use_ncarb_certificate("CA")); // With additional requirements
/// ```
pub fn can_use_ncarb_certificate(state_code: &str) -> bool {
    !matches!(ncarb_status(state_code), NCARBStatus::NoReciprocity)
}

/// Get NCARB reciprocity status for a state
pub fn ncarb_status(state_code: &str) -> NCARBStatus {
    match state_code {
        // States with full reciprocity (most common)
        "AL" | "AK" | "AZ" | "AR" | "CO" | "CT" | "DE" | "FL" | "GA" | "HI" | "ID" | "IL"
        | "IN" | "IA" | "KS" | "KY" | "LA" | "ME" | "MD" | "MA" | "MI" | "MN" | "MS" | "MO"
        | "MT" | "NE" | "NV" | "NH" | "NJ" | "NM" | "NC" | "ND" | "OH" | "OK" | "OR" | "PA"
        | "RI" | "SC" | "SD" | "TN" | "TX" | "UT" | "VT" | "VA" | "WA" | "WV" | "WI" | "WY"
        | "DC" => NCARBStatus::FullReciprocity {
            additional_requirements: vec![],
        },

        // California - conditional reciprocity
        "CA" => NCARBStatus::ConditionalReciprocity {
            conditions: vec![
                "California Supplemental Examination (CSE)".to_string(),
                "Seismic principles examination".to_string(),
                "Disability access requirements".to_string(),
            ],
        },

        // New York - conditional reciprocity
        "NY" => NCARBStatus::FullReciprocity {
            additional_requirements: vec![
                "Fire/Energy Code of New York State examination".to_string(),
            ],
        },

        // Default: full reciprocity
        _ => NCARBStatus::FullReciprocity {
            additional_requirements: vec![],
        },
    }
}

/// Get architect licensing requirements for a state
pub fn licensing_requirements(state_code: &str) -> ArchitectLicensing {
    let state_id = StateId::from_code(state_code);

    ArchitectLicensing {
        state_id,
        ncarb_status: ncarb_status(state_code),
        education_requirements: EducationRequirements {
            naab_accredited_required: true,
            alternative_pathways: if state_code == "CA" {
                vec!["Combination of education and experience".to_string()]
            } else {
                vec![]
            },
            minimum_degree: DegreeLevel::BachelorOrMaster,
        },
        experience_requirements: ExperienceRequirements {
            required_hours: 3740, // Standard AXP requirement
            axp_required: true,
            supervision_required: true,
            diversified_experience: true,
        },
        examination_requirements: ExaminationRequirements {
            are_required: true,
            are_version: "5.0".to_string(),
            state_specific_exam: match state_code {
                "CA" => Some("California Supplemental Examination (CSE)".to_string()),
                "NY" => Some("Fire/Energy Code examination".to_string()),
                _ => None,
            },
        },
        continuing_education: match state_code {
            // States with CE requirements
            "CA" => Some(ContinuingEducation {
                hours_required: 40,
                renewal_period_years: 2,
                hsw_hours_required: Some(5),
            }),
            "FL" => Some(ContinuingEducation {
                hours_required: 20,
                renewal_period_years: 2,
                hsw_hours_required: Some(4),
            }),
            "IL" => Some(ContinuingEducation {
                hours_required: 24,
                renewal_period_years: 2,
                hsw_hours_required: Some(4),
            }),
            "NY" => Some(ContinuingEducation {
                hours_required: 36,
                renewal_period_years: 3,
                hsw_hours_required: Some(24),
            }),
            "TX" => Some(ContinuingEducation {
                hours_required: 12,
                renewal_period_years: 1,
                hsw_hours_required: Some(1),
            }),
            "WA" => Some(ContinuingEducation {
                hours_required: 24,
                renewal_period_years: 2,
                hsw_hours_required: Some(4),
            }),
            // States without CE requirements
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ncarb_full_reciprocity() {
        // Most states have full reciprocity
        assert!(matches!(
            ncarb_status("TX"),
            NCARBStatus::FullReciprocity { .. }
        ));
        assert!(matches!(
            ncarb_status("FL"),
            NCARBStatus::FullReciprocity { .. }
        ));
        assert!(matches!(
            ncarb_status("WA"),
            NCARBStatus::FullReciprocity { .. }
        ));
    }

    #[test]
    fn test_california_conditional_reciprocity() {
        let ca_status = ncarb_status("CA");
        assert!(matches!(
            ca_status,
            NCARBStatus::ConditionalReciprocity { .. }
        ));

        if let NCARBStatus::ConditionalReciprocity { conditions } = ca_status {
            assert!(!conditions.is_empty());
            assert!(conditions.iter().any(|c| c.contains("CSE")));
            assert!(conditions.iter().any(|c| c.contains("Seismic")));
        }
    }

    #[test]
    fn test_new_york_additional_requirements() {
        let ny_status = ncarb_status("NY");
        assert!(matches!(ny_status, NCARBStatus::FullReciprocity { .. }));

        if let NCARBStatus::FullReciprocity {
            additional_requirements,
        } = ny_status
        {
            assert!(!additional_requirements.is_empty());
            assert!(additional_requirements.iter().any(|r| r.contains("Fire")));
        }
    }

    #[test]
    fn test_can_use_ncarb_certificate() {
        // NCARB works in all major states
        assert!(can_use_ncarb_certificate("CA"));
        assert!(can_use_ncarb_certificate("TX"));
        assert!(can_use_ncarb_certificate("NY"));
        assert!(can_use_ncarb_certificate("FL"));
        assert!(can_use_ncarb_certificate("IL"));
    }

    #[test]
    fn test_licensing_requirements() {
        let tx_reqs = licensing_requirements("TX");
        assert!(matches!(
            tx_reqs.ncarb_status,
            NCARBStatus::FullReciprocity { .. }
        ));
        assert!(tx_reqs.education_requirements.naab_accredited_required);
        assert_eq!(tx_reqs.experience_requirements.required_hours, 3740);
        assert!(tx_reqs.examination_requirements.are_required);

        let ca_reqs = licensing_requirements("CA");
        assert!(matches!(
            ca_reqs.ncarb_status,
            NCARBStatus::ConditionalReciprocity { .. }
        ));
        assert!(
            ca_reqs
                .examination_requirements
                .state_specific_exam
                .is_some()
        );
    }

    #[test]
    fn test_continuing_education_requirements() {
        // States with CE
        let ca_reqs = licensing_requirements("CA");
        assert!(ca_reqs.continuing_education.is_some());
        if let Some(ce) = ca_reqs.continuing_education {
            assert_eq!(ce.hours_required, 40);
            assert_eq!(ce.renewal_period_years, 2);
        }

        let tx_reqs = licensing_requirements("TX");
        assert!(tx_reqs.continuing_education.is_some());
        if let Some(ce) = tx_reqs.continuing_education {
            assert_eq!(ce.hours_required, 12);
            assert_eq!(ce.renewal_period_years, 1);
        }

        // States without CE
        let al_reqs = licensing_requirements("AL");
        assert!(al_reqs.continuing_education.is_none());
    }

    #[test]
    fn test_standard_axp_requirements() {
        let states = vec!["CA", "TX", "NY", "FL", "IL"];

        for state in states {
            let reqs = licensing_requirements(state);
            assert!(reqs.experience_requirements.axp_required);
            assert_eq!(reqs.experience_requirements.required_hours, 3740);
            assert!(reqs.experience_requirements.supervision_required);
        }
    }

    #[test]
    fn test_all_states_require_are() {
        let states = vec![
            "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN",
            "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV",
            "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN",
            "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY", "DC",
        ];

        for state in states {
            let reqs = licensing_requirements(state);
            assert!(
                reqs.examination_requirements.are_required,
                "ARE should be required in {}",
                state
            );
        }
    }
}
