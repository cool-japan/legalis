//! Medical Licensing and Interstate Practice
//!
//! This module handles physician licensing across US states, focusing on:
//! - Interstate Medical Licensure Compact (IMLC)
//! - Telemedicine regulations
//! - Controlled substances prescribing authority
//!
//! # Interstate Medical Licensure Compact (IMLC)
//!
//! The IMLC expedites the licensing process for physicians seeking to practice
//! in multiple states. As of 2024, 35+ states participate in the compact,
//! allowing physicians with a primary license in good standing to obtain
//! expedited licenses in other member states.
//!
//! ## Eligibility Requirements
//! - Primary license in a Compact state
//! - Board certification or completion of residency
//! - No disciplinary actions or investigations
//! - Clean background check
//!
//! ## Benefits
//! - Expedited processing (typically 30-90 days)
//! - Standardized application process
//! - Reduced fees in many states

use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// IMLC membership status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IMLCStatus {
    /// State is an active IMLC member
    Member {
        /// Year joined
        join_year: u16,
        /// Processing time (days)
        typical_processing_days: u8,
    },
    /// State is not an IMLC member
    NonMember,
    /// State has passed legislation but not yet active
    Pending {
        /// Expected activation date
        expected_year: u16,
    },
}

/// Telemedicine licensing requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelemedicineRules {
    /// State identifier
    pub state_id: StateId,
    /// License required for telemedicine into this state
    pub license_required: bool,
    /// Special telemedicine license available
    pub special_telemedicine_license: bool,
    /// IMLC expedites telemedicine licensing
    pub imlc_expedites: bool,
    /// Restrictions on modality (video, audio, etc.)
    pub modality_restrictions: Vec<ModalityRestriction>,
    /// In-person visit required before telemedicine
    pub initial_in_person_required: bool,
}

/// Telemedicine modality restrictions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalityRestriction {
    /// Audio-only (phone) restricted or prohibited
    AudioOnlyRestricted,
    /// Video required for certain services
    VideoRequired { services: Vec<String> },
    /// Store-and-forward (asynchronous) allowed
    StoreAndForwardAllowed,
    /// Remote patient monitoring allowed
    RemoteMonitoringAllowed,
}

/// Controlled substances prescribing authority
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrescribingAuthority {
    /// State identifier
    pub state_id: StateId,
    /// DEA registration required
    pub dea_required: bool,
    /// State-level controlled substance registration
    pub state_registration_required: bool,
    /// PDMP (Prescription Drug Monitoring Program) check required
    pub pdmp_check_required: bool,
    /// Opioid prescribing limits
    pub opioid_limits: Option<OpioidLimits>,
    /// Telemedicine prescribing allowed
    pub telemedicine_prescribing: TelemedicinePrescribing,
}

/// Opioid prescribing limits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpioidLimits {
    /// Initial prescription limit (days supply)
    pub initial_limit_days: u8,
    /// Acute pain limit (days supply)
    pub acute_pain_limit_days: Option<u8>,
    /// Exceptions for chronic pain
    pub chronic_pain_exceptions: bool,
}

/// Telemedicine prescribing rules
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TelemedicinePrescribing {
    /// Full prescribing authority via telemedicine
    FullAuthority,
    /// Limited authority (non-controlled substances only)
    NonControlledOnly,
    /// Controlled substances prohibited
    ControlledProhibited {
        /// Exceptions for established patients
        established_patient_exception: bool,
    },
    /// Prohibited entirely
    Prohibited,
}

/// Check if a state is an IMLC member
///
/// # Example
/// ```
/// use legalis_us::professional_licensing::medical::is_imlc_member;
///
/// assert!(is_imlc_member("TX"));
/// assert!(is_imlc_member("CO"));
/// assert!(!is_imlc_member("CA"));
/// ```
pub fn is_imlc_member(state_code: &str) -> bool {
    matches!(imlc_status(state_code), IMLCStatus::Member { .. })
}

/// Get IMLC status for a state
pub fn imlc_status(state_code: &str) -> IMLCStatus {
    match state_code {
        // IMLC member states (as of 2024)
        "AL" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "AZ" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 45,
        },
        "CO" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 30,
        },
        "ID" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "IL" => IMLCStatus::Member {
            join_year: 2016,
            typical_processing_days: 45,
        },
        "IA" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "KS" => IMLCStatus::Member {
            join_year: 2016,
            typical_processing_days: 60,
        },
        "ME" => IMLCStatus::Member {
            join_year: 2017,
            typical_processing_days: 45,
        },
        "MD" => IMLCStatus::Member {
            join_year: 2018,
            typical_processing_days: 45,
        },
        "MN" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "MS" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "MT" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "NE" => IMLCStatus::Member {
            join_year: 2017,
            typical_processing_days: 45,
        },
        "NH" => IMLCStatus::Member {
            join_year: 2017,
            typical_processing_days: 60,
        },
        "NV" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 45,
        },
        "ND" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "OH" => IMLCStatus::Member {
            join_year: 2019,
            typical_processing_days: 45,
        },
        "OK" => IMLCStatus::Member {
            join_year: 2019,
            typical_processing_days: 60,
        },
        "PA" => IMLCStatus::Member {
            join_year: 2017,
            typical_processing_days: 45,
        },
        "SD" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "TN" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "TX" => IMLCStatus::Member {
            join_year: 2019,
            typical_processing_days: 45,
        },
        "UT" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "WA" => IMLCStatus::Member {
            join_year: 2017,
            typical_processing_days: 45,
        },
        "WV" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "WI" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "WY" => IMLCStatus::Member {
            join_year: 2015,
            typical_processing_days: 60,
        },
        "DC" => IMLCStatus::Member {
            join_year: 2016,
            typical_processing_days: 45,
        },

        // Non-member states
        "CA" | "CT" | "DE" | "FL" | "GA" | "HI" | "IN" | "KY" | "LA" | "MA" | "MI" | "MO"
        | "NJ" | "NM" | "NY" | "NC" | "OR" | "RI" | "SC" | "VT" | "VA" => IMLCStatus::NonMember,

        // Default
        _ => IMLCStatus::NonMember,
    }
}

/// Get telemedicine requirements for a state
pub fn telemedicine_requirements(state_code: &str) -> TelemedicineRules {
    let state_id = StateId::from_code(state_code);
    let is_member = is_imlc_member(state_code);

    TelemedicineRules {
        state_id,
        license_required: true, // All states require license
        special_telemedicine_license: matches!(state_code, "TX" | "FL"),
        imlc_expedites: is_member,
        modality_restrictions: match state_code {
            "TX" => vec![
                ModalityRestriction::VideoRequired {
                    services: vec!["Initial consultation".to_string()],
                },
                ModalityRestriction::RemoteMonitoringAllowed,
            ],
            "CA" => vec![ModalityRestriction::VideoRequired {
                services: vec!["Most services".to_string()],
            }],
            _ => vec![
                ModalityRestriction::StoreAndForwardAllowed,
                ModalityRestriction::RemoteMonitoringAllowed,
            ],
        },
        initial_in_person_required: matches!(state_code, "TX" | "AR"),
    }
}

/// Get prescribing authority rules for a state
pub fn prescribing_authority(state_code: &str) -> PrescribingAuthority {
    let state_id = StateId::from_code(state_code);

    PrescribingAuthority {
        state_id,
        dea_required: true, // Federal requirement
        state_registration_required: !matches!(state_code, "WY"), // Most states require
        pdmp_check_required: !matches!(state_code, "SD" | "NH"), // Most states require
        opioid_limits: match state_code {
            // States with 7-day limits
            "AZ" | "CT" | "FL" | "IN" | "MA" | "NJ" | "NY" | "OH" | "PA" | "RI" | "VT" => {
                Some(OpioidLimits {
                    initial_limit_days: 7,
                    acute_pain_limit_days: Some(7),
                    chronic_pain_exceptions: true,
                })
            }
            // States with 5-day limits
            "MD" | "ME" | "UT" => Some(OpioidLimits {
                initial_limit_days: 5,
                acute_pain_limit_days: Some(5),
                chronic_pain_exceptions: true,
            }),
            // No specific limit
            _ => None,
        },
        telemedicine_prescribing: match state_code {
            "TX" => TelemedicinePrescribing::ControlledProhibited {
                established_patient_exception: true,
            },
            "CA" | "NY" => TelemedicinePrescribing::ControlledProhibited {
                established_patient_exception: false,
            },
            _ => TelemedicinePrescribing::NonControlledOnly,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imlc_membership() {
        // Member states
        assert!(is_imlc_member("TX"));
        assert!(is_imlc_member("CO"));
        assert!(is_imlc_member("AZ"));
        assert!(is_imlc_member("IL"));

        // Non-member states
        assert!(!is_imlc_member("CA"));
        assert!(!is_imlc_member("NY"));
        assert!(!is_imlc_member("FL"));
    }

    #[test]
    fn test_imlc_processing_times() {
        let co_status = imlc_status("CO");
        if let IMLCStatus::Member {
            typical_processing_days,
            ..
        } = co_status
        {
            assert_eq!(typical_processing_days, 30); // Colorado is fastest
        }

        let tx_status = imlc_status("TX");
        if let IMLCStatus::Member {
            typical_processing_days,
            ..
        } = tx_status
        {
            assert_eq!(typical_processing_days, 45);
        }
    }

    #[test]
    fn test_telemedicine_requirements() {
        let tx_rules = telemedicine_requirements("TX");
        assert!(tx_rules.license_required);
        assert!(tx_rules.special_telemedicine_license);
        assert!(tx_rules.initial_in_person_required);

        let ca_rules = telemedicine_requirements("CA");
        assert!(ca_rules.license_required);
        assert!(!ca_rules.special_telemedicine_license);
        assert!(!ca_rules.imlc_expedites); // CA not in IMLC
    }

    #[test]
    fn test_prescribing_authority() {
        let tx_authority = prescribing_authority("TX");
        assert!(tx_authority.dea_required);
        assert!(tx_authority.state_registration_required);
        assert!(tx_authority.pdmp_check_required);
        assert!(matches!(
            tx_authority.telemedicine_prescribing,
            TelemedicinePrescribing::ControlledProhibited { .. }
        ));

        let az_authority = prescribing_authority("AZ");
        assert!(az_authority.opioid_limits.is_some());
        if let Some(limits) = az_authority.opioid_limits {
            assert_eq!(limits.initial_limit_days, 7);
        }
    }

    #[test]
    fn test_opioid_limits() {
        // 7-day limit states
        let ny_authority = prescribing_authority("NY");
        assert!(ny_authority.opioid_limits.is_some());
        if let Some(limits) = ny_authority.opioid_limits {
            assert_eq!(limits.initial_limit_days, 7);
            assert!(limits.chronic_pain_exceptions);
        }

        // 5-day limit states
        let md_authority = prescribing_authority("MD");
        assert!(md_authority.opioid_limits.is_some());
        if let Some(limits) = md_authority.opioid_limits {
            assert_eq!(limits.initial_limit_days, 5);
        }

        // No specific limit
        let wa_authority = prescribing_authority("WA");
        assert!(wa_authority.opioid_limits.is_none());
    }

    #[test]
    fn test_all_states_have_dea_requirement() {
        let states = vec!["CA", "TX", "NY", "FL", "IL", "PA", "OH", "GA", "NC", "MI"];

        for state in states {
            let authority = prescribing_authority(state);
            assert!(authority.dea_required, "DEA required for {}", state);
        }
    }

    #[test]
    fn test_imlc_count() {
        let states = vec![
            "AL", "AZ", "CO", "ID", "IL", "IA", "KS", "ME", "MD", "MN", "MS", "MT", "NE", "NH",
            "NV", "ND", "OH", "OK", "PA", "SD", "TN", "TX", "UT", "WA", "WV", "WI", "WY", "DC",
        ];

        let mut member_count = 0;
        for state in &states {
            if is_imlc_member(state) {
                member_count += 1;
            }
        }

        assert!(member_count >= 28, "Should have at least 28 IMLC members");
    }
}
