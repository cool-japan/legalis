//! DPA 2018 Exemptions
//!
//! Data Protection Act 2018 provides various exemptions from UK GDPR obligations.
//! These exemptions are set out in Schedule 2 and other provisions of the Act.
//!
//! # Important Note
//!
//! Exemptions should be applied narrowly and only when genuinely necessary.
//! They do NOT provide blanket immunity from data protection obligations.

use serde::{Deserialize, Serialize};

/// DPA 2018 exemptions from UK GDPR requirements
///
/// Schedule 2 and other provisions of DPA 2018 provide exemptions
/// for specific purposes and circumstances.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dpa2018Exemption {
    /// National security (DPA 2018 s.26)
    /// Exemption from all UK GDPR provisions
    NationalSecurity {
        /// Certificate issued by Minister
        ministerial_certificate: bool,
    },

    /// Defense (Schedule 2 Part 1)
    /// Combat capability, armed forces deployment
    Defense {
        /// Specific defense purpose
        purpose: DefensePurpose,
    },

    /// Crime and taxation (Schedule 2 Part 2)
    /// Prevention or detection of crime, tax assessment/collection
    CrimeTaxation {
        /// Type of crime/tax purpose
        purpose: CrimeTaxPurpose,
    },

    /// Immigration (Schedule 2 Part 3)
    /// Maintenance of effective immigration control
    Immigration,

    /// Legal proceedings (Schedule 2 Part 4)
    /// Exercise of legal rights, judicial functions
    LegalProceedings {
        /// Type of legal purpose
        purpose: LegalPurpose,
    },

    /// Journalism (Schedule 2 Part 5)
    /// Special purposes: journalism, academic/artistic/literary purposes
    /// Balancing with freedom of expression (Article 10 ECHR)
    Journalism {
        /// Processing must be for publication in the public interest
        public_interest: bool,
        /// Controller must reasonably believe publication would be in public interest
        reasonable_belief: bool,
        /// Compliance with UK GDPR would be incompatible with journalism
        incompatible_with_journalism: bool,
    },

    /// Academic research (Schedule 2 Part 6)
    /// Research, statistical and archival purposes
    AcademicResearch {
        /// Appropriate safeguards for data subject rights
        safeguards_in_place: bool,
        /// Not processed for decisions about particular individuals
        not_for_individual_decisions: bool,
        /// Not likely to cause substantial damage or distress
        no_substantial_harm: bool,
    },

    /// Archiving in public interest (Schedule 2 Part 6)
    /// Archives, museums, libraries
    Archiving {
        /// Appropriate safeguards for data subject rights
        safeguards_in_place: bool,
    },

    /// Health data (Schedule 3)
    /// Health or social care purposes
    HealthData {
        /// Specific health/social care purpose
        purpose: HealthDataPurpose,
    },

    /// Legal professional privilege (Schedule 2 Part 4)
    /// Communications between lawyer and client
    LegalPrivilege,

    /// Self-incrimination (Schedule 2 Part 4)
    /// Disclosure would expose data subject to criminal proceedings
    SelfIncrimination,

    /// Regulatory functions (Schedule 2 Part 2)
    /// Functions of regulatory bodies
    RegulatoryFunctions {
        /// Name of regulatory body
        regulator: String,
    },

    /// Parliamentary privilege (DPA 2018 s.36)
    /// Protection of parliamentary proceedings
    ParliamentaryPrivilege,

    /// Judicial independence (DPA 2018 s.37)
    /// Judicial appointments and discipline
    JudicialIndependence,

    /// Crown honours and dignities (Schedule 2 Part 1)
    CrownHonours,

    /// Management forecasts (Schedule 2 Part 1)
    /// Confidential management planning
    ManagementForecasts,

    /// Corporate finance (Schedule 2 Part 1)
    /// Confidential corporate finance activities
    CorporateFinance,

    /// Negotiations (Schedule 2 Part 1)
    /// Prejudice to commercial negotiations
    Negotiations,

    /// Exam scripts and marks (Schedule 2 Part 1)
    ExamScripts,

    /// References (Schedule 2 Part 1)
    /// Confidential employment or educational references
    ConfidentialReferences,

    /// Armed forces (Schedule 2 Part 1)
    /// Combat effectiveness, capability, security, morale
    ArmedForces { purpose: ArmedForcesPurpose },
}

/// Defense purposes under Schedule 2 Part 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefensePurpose {
    /// Combat effectiveness of armed forces
    CombatEffectiveness,
    /// Security of armed forces
    Security,
    /// Morale of armed forces
    Morale,
    /// Recruitment of armed forces
    Recruitment,
}

/// Crime and taxation purposes under Schedule 2 Part 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrimeTaxPurpose {
    /// Prevention of crime
    CrimePrevention,
    /// Detection of crime
    CrimeDetection,
    /// Investigation of crime
    CrimeInvestigation,
    /// Prosecution of offenders
    Prosecution,
    /// Assessment or collection of tax or duty
    TaxAssessment,
}

/// Legal purposes under Schedule 2 Part 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalPurpose {
    /// Legal advice (legal professional privilege)
    LegalAdvice,
    /// Legal proceedings
    LegalProceedings,
    /// Judicial functions
    JudicialFunctions,
    /// Exercise of rights in legal proceedings
    ExerciseOfRights,
}

/// Health and social care purposes under Schedule 3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthDataPurpose {
    /// Preventive or occupational medicine
    PreventiveMedicine,
    /// Medical diagnosis
    MedicalDiagnosis,
    /// Provision of health care
    HealthCareProvision,
    /// Provision of social care
    SocialCareProvision,
    /// Management of health or social care systems
    SystemsManagement,
    /// Public health (Article 9(2)(i) UK GDPR)
    PublicHealth,
}

/// Armed forces purposes under Schedule 2 Part 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmedForcesPurpose {
    /// Combat effectiveness
    CombatEffectiveness,
    /// Capability of armed forces
    Capability,
    /// Security of armed forces
    Security,
    /// Morale of armed forces
    Morale,
}

/// Exemption type (for error handling)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExemptionType {
    /// National security
    NationalSecurity,
    /// Defense
    Defense,
    /// Crime and taxation
    CrimeTaxation,
    /// Immigration
    Immigration,
    /// Legal proceedings
    LegalProceedings,
    /// Journalism
    Journalism,
    /// Academic research
    AcademicResearch,
    /// Archiving
    Archiving,
    /// Health data
    HealthData,
    /// Legal privilege
    LegalPrivilege,
    /// Self-incrimination
    SelfIncrimination,
    /// Regulatory functions
    RegulatoryFunctions,
    /// Parliamentary privilege
    ParliamentaryPrivilege,
    /// Judicial independence
    JudicialIndependence,
    /// Crown honours
    CrownHonours,
    /// Management forecasts
    ManagementForecasts,
    /// Corporate finance
    CorporateFinance,
    /// Negotiations
    Negotiations,
    /// Exam scripts
    ExamScripts,
    /// References
    ConfidentialReferences,
    /// Armed forces
    ArmedForces,
}

impl Dpa2018Exemption {
    /// Get the statutory provision for this exemption
    pub fn statutory_provision(&self) -> &'static str {
        match self {
            Self::NationalSecurity { .. } => "DPA 2018 s.26",
            Self::Defense { .. } => "DPA 2018 Schedule 2 Part 1 para 1",
            Self::CrimeTaxation { .. } => "DPA 2018 Schedule 2 Part 2 para 2-6",
            Self::Immigration => "DPA 2018 Schedule 2 Part 3 para 7",
            Self::LegalProceedings { .. } => "DPA 2018 Schedule 2 Part 4 para 8-11",
            Self::Journalism { .. } => "DPA 2018 Schedule 2 Part 5 para 26",
            Self::AcademicResearch { .. } => "DPA 2018 Schedule 2 Part 6 para 27",
            Self::Archiving { .. } => "DPA 2018 Schedule 2 Part 6 para 28",
            Self::HealthData { .. } => "DPA 2018 Schedule 3",
            Self::LegalPrivilege => "DPA 2018 Schedule 2 Part 4 para 9",
            Self::SelfIncrimination => "DPA 2018 Schedule 2 Part 4 para 11",
            Self::RegulatoryFunctions { .. } => "DPA 2018 Schedule 2 Part 2 para 5",
            Self::ParliamentaryPrivilege => "DPA 2018 s.36",
            Self::JudicialIndependence => "DPA 2018 s.37",
            Self::CrownHonours => "DPA 2018 Schedule 2 Part 1 para 20",
            Self::ManagementForecasts => "DPA 2018 Schedule 2 Part 1 para 15",
            Self::CorporateFinance => "DPA 2018 Schedule 2 Part 1 para 16",
            Self::Negotiations => "DPA 2018 Schedule 2 Part 1 para 17",
            Self::ExamScripts => "DPA 2018 Schedule 2 Part 1 para 23",
            Self::ConfidentialReferences => "DPA 2018 Schedule 2 Part 1 para 24",
            Self::ArmedForces { .. } => "DPA 2018 Schedule 2 Part 1 para 13",
        }
    }

    /// Get exemption type
    pub fn exemption_type(&self) -> ExemptionType {
        match self {
            Self::NationalSecurity { .. } => ExemptionType::NationalSecurity,
            Self::Defense { .. } => ExemptionType::Defense,
            Self::CrimeTaxation { .. } => ExemptionType::CrimeTaxation,
            Self::Immigration => ExemptionType::Immigration,
            Self::LegalProceedings { .. } => ExemptionType::LegalProceedings,
            Self::Journalism { .. } => ExemptionType::Journalism,
            Self::AcademicResearch { .. } => ExemptionType::AcademicResearch,
            Self::Archiving { .. } => ExemptionType::Archiving,
            Self::HealthData { .. } => ExemptionType::HealthData,
            Self::LegalPrivilege => ExemptionType::LegalPrivilege,
            Self::SelfIncrimination => ExemptionType::SelfIncrimination,
            Self::RegulatoryFunctions { .. } => ExemptionType::RegulatoryFunctions,
            Self::ParliamentaryPrivilege => ExemptionType::ParliamentaryPrivilege,
            Self::JudicialIndependence => ExemptionType::JudicialIndependence,
            Self::CrownHonours => ExemptionType::CrownHonours,
            Self::ManagementForecasts => ExemptionType::ManagementForecasts,
            Self::CorporateFinance => ExemptionType::CorporateFinance,
            Self::Negotiations => ExemptionType::Negotiations,
            Self::ExamScripts => ExemptionType::ExamScripts,
            Self::ConfidentialReferences => ExemptionType::ConfidentialReferences,
            Self::ArmedForces { .. } => ExemptionType::ArmedForces,
        }
    }

    /// Check if exemption requires narrow application
    ///
    /// Most exemptions should be applied narrowly. Returns true if the
    /// exemption is particularly sensitive and requires careful justification.
    pub fn requires_narrow_application(&self) -> bool {
        matches!(
            self,
            Self::NationalSecurity { .. }
                | Self::Journalism { .. }
                | Self::CrimeTaxation { .. }
                | Self::LegalPrivilege
                | Self::SelfIncrimination
        )
    }

    /// Get ICO guidance reference
    pub fn ico_guidance(&self) -> &'static str {
        match self {
            Self::Journalism { .. } => {
                "https://ico.org.uk/for-organisations/guide-to-data-protection/\
                 guide-to-the-general-data-protection-regulation-gdpr/exemptions/"
            }
            Self::AcademicResearch { .. } => {
                "https://ico.org.uk/for-organisations/guide-to-data-protection/\
                 guide-to-the-general-data-protection-regulation-gdpr/\
                 lawful-basis-for-processing/research/"
            }
            Self::CrimeTaxation { .. } => {
                "https://ico.org.uk/for-organisations/guide-to-data-protection/\
                 guide-to-the-general-data-protection-regulation-gdpr/\
                 exemptions/#crime"
            }
            Self::HealthData { .. } => {
                "https://ico.org.uk/for-organisations/guide-to-data-protection/\
                 guide-to-the-general-data-protection-regulation-gdpr/\
                 lawful-basis-for-processing/special-category-data/"
            }
            _ => {
                "https://ico.org.uk/for-organisations/guide-to-data-protection/\
                 guide-to-the-general-data-protection-regulation-gdpr/exemptions/"
            }
        }
    }
}

/// Validate journalism exemption under DPA 2018 Schedule 2 Part 5
///
/// The journalism exemption requires:
/// 1. Processing is for journalism (special purposes)
/// 2. Processing is undertaken with a view to publication
/// 3. Controller reasonably believes publication would be in public interest
/// 4. Compliance with UK GDPR would be incompatible with journalism
///
/// This balances data protection with freedom of expression (Article 10 ECHR).
///
/// # Important
///
/// This exemption does NOT apply to non-journalistic activities of media
/// organizations (e.g., HR, marketing, subscriber management).
pub fn validate_journalism_exemption(
    public_interest: bool,
    reasonable_belief: bool,
    incompatible: bool,
) -> Result<(), String> {
    if !public_interest {
        return Err(
            "Journalism exemption requires processing to be in the public interest.".to_string(),
        );
    }

    if !reasonable_belief {
        return Err(
            "Controller must reasonably believe publication would be in public interest."
                .to_string(),
        );
    }

    if !incompatible {
        return Err(
            "Journalism exemption only applies if UK GDPR compliance would be incompatible \
             with journalism."
                .to_string(),
        );
    }

    Ok(())
}

/// Validate academic research exemption under DPA 2018 Schedule 2 Part 6
///
/// The research exemption requires:
/// 1. Appropriate safeguards for data subject rights and freedoms
/// 2. Not processed to support decisions about particular individuals
/// 3. Not likely to cause substantial damage or distress
///
/// If these conditions are met, certain UK GDPR provisions can be disapplied
/// (e.g., Article 15 access right can be restricted).
pub fn validate_academic_research_exemption(
    safeguards: bool,
    no_individual_decisions: bool,
    no_harm: bool,
) -> Result<(), String> {
    if !safeguards {
        return Err(
            "Research exemption requires appropriate safeguards for data subject rights."
                .to_string(),
        );
    }

    if !no_individual_decisions {
        return Err(
            "Research exemption does not apply if data is processed for decisions about \
             particular individuals."
                .to_string(),
        );
    }

    if !no_harm {
        return Err(
            "Research exemption does not apply if processing is likely to cause substantial \
             damage or distress."
                .to_string(),
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journalism_exemption_valid() {
        let result = validate_journalism_exemption(true, true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_journalism_exemption_not_public_interest() {
        let result = validate_journalism_exemption(false, true, true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("public interest"));
    }

    #[test]
    fn test_journalism_exemption_not_reasonable_belief() {
        let result = validate_journalism_exemption(true, false, true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("reasonably believe"));
    }

    #[test]
    fn test_journalism_exemption_not_incompatible() {
        let result = validate_journalism_exemption(true, true, false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("incompatible"));
    }

    #[test]
    fn test_academic_research_exemption_valid() {
        let result = validate_academic_research_exemption(true, true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_academic_research_exemption_no_safeguards() {
        let result = validate_academic_research_exemption(false, true, true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("safeguards"));
    }

    #[test]
    fn test_academic_research_exemption_individual_decisions() {
        let result = validate_academic_research_exemption(true, false, true);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("decisions about particular individuals")
        );
    }

    #[test]
    fn test_exemption_statutory_provisions() {
        let national_security = Dpa2018Exemption::NationalSecurity {
            ministerial_certificate: true,
        };
        assert_eq!(national_security.statutory_provision(), "DPA 2018 s.26");

        let journalism = Dpa2018Exemption::Journalism {
            public_interest: true,
            reasonable_belief: true,
            incompatible_with_journalism: true,
        };
        assert_eq!(
            journalism.statutory_provision(),
            "DPA 2018 Schedule 2 Part 5 para 26"
        );
    }

    #[test]
    fn test_exemption_requires_narrow_application() {
        let national_security = Dpa2018Exemption::NationalSecurity {
            ministerial_certificate: true,
        };
        assert!(national_security.requires_narrow_application());

        let exam_scripts = Dpa2018Exemption::ExamScripts;
        assert!(!exam_scripts.requires_narrow_application());
    }
}
