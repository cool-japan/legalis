//! South African Financial Services Law
//!
//! Regulation of financial services, advice, and intermediary services.
//!
//! ## Key Legislation
//!
//! - Financial Advisory and Intermediary Services Act 37 of 2002 (FAIS)
//! - Financial Sector Regulation Act 9 of 2017 (Twin Peaks model)
//! - Banks Act 94 of 1990
//! - Insurance Act 18 of 2017
//! - Financial Markets Act 19 of 2012
//!
//! ## Regulators (Twin Peaks)
//!
//! - Prudential Authority (PA) - prudential regulation
//! - Financial Sector Conduct Authority (FSCA) - market conduct

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for financial services operations
pub type FinancialServicesResult<T> = Result<T, FinancialServicesError>;

/// Financial services under FAIS Act
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialService {
    /// Advice (s1 FAIS Act)
    Advice,
    /// Intermediary services
    IntermediaryServices,
    /// Rendering of advice and intermediary services
    AdviceAndIntermediaryServices,
    /// Assistance or guidance
    AssistanceOrGuidance,
}

/// Financial products (s1 FAIS Act)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialProduct {
    /// Long-term insurance
    LongTermInsurance,
    /// Short-term insurance
    ShortTermInsurance,
    /// Securities and instruments (JSE-listed, bonds, etc.)
    SecuritiesAndInstruments,
    /// Participatory interests (collective investment schemes)
    ParticipatoryInterests,
    /// Pension fund benefits
    PensionFundBenefits,
    /// Friendly society benefits
    FriendlySocietyBenefits,
    /// Health service benefits
    HealthServiceBenefits,
}

/// FSP (Financial Services Provider) categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FspCategory {
    /// Category I - Discretionary FSP
    CategoryI,
    /// Category II - Advice FSP
    CategoryII,
    /// Category IIA - Administrative FSP
    CategoryIia,
    /// Category III - Assistance FSP
    CategoryIii,
}

impl FspCategory {
    /// Minimum capital requirements (FSCA rules)
    pub fn minimum_capital_zar(&self) -> i64 {
        match self {
            Self::CategoryI => 1_000_000, // R1 million
            Self::CategoryII => 250_000,  // R250,000
            Self::CategoryIia => 100_000, // R100,000
            Self::CategoryIii => 50_000,  // R50,000
        }
    }

    /// Professional indemnity insurance required
    pub fn requires_pi_insurance(&self) -> bool {
        true
    }
}

/// FAIS licensing requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaisLicense {
    /// FSP license number
    pub license_number: Option<String>,
    /// FSP category
    pub category: FspCategory,
    /// Licensed
    pub is_licensed: bool,
    /// Fit and proper
    pub fit_and_proper: bool,
    /// Professional indemnity insurance
    pub has_pi_insurance: bool,
    /// Compliance officer appointed
    pub compliance_officer_appointed: bool,
}

impl FaisLicense {
    /// Validate licensing requirements
    pub fn is_compliant(&self) -> bool {
        self.is_licensed
            && self.fit_and_proper
            && self.has_pi_insurance
            && self.compliance_officer_appointed
    }
}

/// Fit and proper requirements (s13 FAIS Act)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitAndProper {
    /// Competency (qualifications)
    pub competent: bool,
    /// Financial soundness
    pub financially_sound: bool,
    /// Honesty and integrity
    pub honesty_and_integrity: bool,
    /// No disqualifying convictions
    pub no_disqualifying_convictions: bool,
}

impl FitAndProper {
    /// Check if meets fit and proper requirements
    pub fn meets_requirements(&self) -> bool {
        self.competent
            && self.financially_sound
            && self.honesty_and_integrity
            && self.no_disqualifying_convictions
    }
}

/// Treating Customers Fairly (TCF) outcomes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TcfOutcome {
    /// Outcome 1: Fair treatment culture
    FairTreatmentCulture,
    /// Outcome 2: Products and services designed to meet needs
    ProductsDesignedForNeeds,
    /// Outcome 3: Clear information before, during, after
    ClearInformation,
    /// Outcome 4: Suitable advice
    SuitableAdvice,
    /// Outcome 5: Products perform as expected
    ProductsPerformAsExpected,
    /// Outcome 6: No unreasonable barriers to claims/switching
    NoUnreasonableBarriers,
}

impl TcfOutcome {
    /// Get all TCF outcomes
    pub fn all_outcomes() -> Vec<Self> {
        vec![
            Self::FairTreatmentCulture,
            Self::ProductsDesignedForNeeds,
            Self::ClearInformation,
            Self::SuitableAdvice,
            Self::ProductsPerformAsExpected,
            Self::NoUnreasonableBarriers,
        ]
    }
}

/// Twin Peaks regulators
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TwinPeaksRegulator {
    /// Prudential Authority (PA) - safety and soundness
    PrudentialAuthority,
    /// Financial Sector Conduct Authority (FSCA) - market conduct
    FinancialSectorConductAuthority,
}

impl TwinPeaksRegulator {
    /// Regulatory focus
    pub fn regulatory_focus(&self) -> &'static str {
        match self {
            Self::PrudentialAuthority => {
                "Prudential regulation - financial soundness, capital adequacy"
            }
            Self::FinancialSectorConductAuthority => {
                "Market conduct - fair treatment, disclosure, consumer protection"
            }
        }
    }
}

/// Financial sector conduct standards
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConductStandard {
    /// Honesty and fairness
    HonestyAndFairness,
    /// Due skill, care and diligence
    DueSkillCareAndDiligence,
    /// Avoid conflicts of interest
    AvoidConflictsOfInterest,
    /// Rendering financial services efficiently, honestly and fairly
    EfficientHonestFair,
    /// Compliance with legislation
    ComplianceWithLegislation,
}

/// General Code of Conduct (s2-15 FAIS Act)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeOfConduct {
    /// Honest and fair dealings
    pub honest_and_fair: bool,
    /// Act with due skill, care and diligence
    pub skill_care_diligence: bool,
    /// Avoid conflicts of interest
    pub avoid_conflicts: bool,
    /// Disclose material information
    pub disclose_material_info: bool,
    /// Know your customer (KYC)
    pub know_your_customer: bool,
    /// Needs analysis conducted
    pub needs_analysis: bool,
}

impl CodeOfConduct {
    /// Check compliance with Code of Conduct
    pub fn is_compliant(&self) -> bool {
        self.honest_and_fair
            && self.skill_care_diligence
            && self.avoid_conflicts
            && self.disclose_material_info
            && self.know_your_customer
            && self.needs_analysis
    }
}

/// Financial Sector Regulation Act objectives
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulatoryObjective {
    /// Financial stability
    FinancialStability,
    /// Safety and soundness of financial institutions
    SafetyAndSoundness,
    /// Fair treatment and protection of financial customers
    FairTreatmentAndProtection,
    /// Efficiency and integrity of financial system
    EfficiencyAndIntegrity,
    /// Financial inclusion
    FinancialInclusion,
    /// Prevention of financial crime
    PreventionOfFinancialCrime,
}

/// Financial services errors
#[derive(Debug, Error)]
pub enum FinancialServicesError {
    /// Unlicensed FSP
    #[error("Unlicensed FSP (FAIS Act s7 - operating without license)")]
    UnlicensedFsp,

    /// Not fit and proper
    #[error("Not fit and proper (s13): {reason}")]
    NotFitAndProper { reason: String },

    /// Code of Conduct breach
    #[error("Code of Conduct breach (FAIS Act s2-15): {breach}")]
    CodeOfConductBreach { breach: String },

    /// TCF outcome not achieved
    #[error("TCF outcome not achieved: {outcome}")]
    TcfOutcomeNotAchieved { outcome: String },

    /// Insufficient capital
    #[error("Insufficient capital (required R{required}, actual R{actual})")]
    InsufficientCapital { required: i64, actual: i64 },

    /// No professional indemnity insurance
    #[error("No professional indemnity insurance (s18 FAIS Act)")]
    NoPiInsurance,

    /// Conflict of interest not disclosed
    #[error("Conflict of interest not disclosed (s3A FAIS Act)")]
    ConflictNotDisclosed,
}

/// Validate FAIS licensing
pub fn validate_fais_license(license: &FaisLicense) -> FinancialServicesResult<()> {
    if !license.is_licensed {
        return Err(FinancialServicesError::UnlicensedFsp);
    }

    if !license.fit_and_proper {
        return Err(FinancialServicesError::NotFitAndProper {
            reason: "Does not meet fit and proper requirements".to_string(),
        });
    }

    if !license.has_pi_insurance {
        return Err(FinancialServicesError::NoPiInsurance);
    }

    Ok(())
}

/// Validate fit and proper requirements
pub fn validate_fit_and_proper(fit_and_proper: &FitAndProper) -> FinancialServicesResult<()> {
    if !fit_and_proper.meets_requirements() {
        let reason = if !fit_and_proper.competent {
            "Lacks required competency/qualifications"
        } else if !fit_and_proper.financially_sound {
            "Not financially sound"
        } else if !fit_and_proper.honesty_and_integrity {
            "Lacks honesty and integrity"
        } else {
            "Has disqualifying convictions"
        };

        return Err(FinancialServicesError::NotFitAndProper {
            reason: reason.to_string(),
        });
    }

    Ok(())
}

/// Get financial services compliance checklist
pub fn get_financial_services_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("FSP license obtained", "s7 FAIS Act"),
        ("Fit and proper requirements met", "s13"),
        ("Professional indemnity insurance", "s18"),
        ("Compliance officer appointed", "s17"),
        ("Code of Conduct compliance", "s2-15"),
        ("TCF outcomes embedded", "FSCA TCF"),
        ("Know Your Customer (KYC)", "s3"),
        ("Needs analysis conducted", "s8"),
        ("Conflicts of interest disclosed", "s3A"),
        ("Record keeping (5 years)", "s19"),
        ("Complaints management", "s26 FAIS Act"),
        ("FICA compliance (AML/CFT)", "FICA"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fsp_category_capital() {
        assert_eq!(FspCategory::CategoryI.minimum_capital_zar(), 1_000_000);
        assert_eq!(FspCategory::CategoryII.minimum_capital_zar(), 250_000);
        assert!(FspCategory::CategoryI.requires_pi_insurance());
    }

    #[test]
    fn test_fais_license_compliant() {
        let license = FaisLicense {
            license_number: Some("FSP12345".to_string()),
            category: FspCategory::CategoryII,
            is_licensed: true,
            fit_and_proper: true,
            has_pi_insurance: true,
            compliance_officer_appointed: true,
        };
        assert!(license.is_compliant());
        assert!(validate_fais_license(&license).is_ok());
    }

    #[test]
    fn test_fais_license_unlicensed() {
        let license = FaisLicense {
            license_number: None,
            category: FspCategory::CategoryII,
            is_licensed: false,
            fit_and_proper: true,
            has_pi_insurance: true,
            compliance_officer_appointed: true,
        };
        assert!(!license.is_compliant());
        assert!(validate_fais_license(&license).is_err());
    }

    #[test]
    fn test_fais_license_no_pi() {
        let license = FaisLicense {
            license_number: Some("FSP12345".to_string()),
            category: FspCategory::CategoryII,
            is_licensed: true,
            fit_and_proper: true,
            has_pi_insurance: false,
            compliance_officer_appointed: true,
        };
        assert!(!license.is_compliant());
        assert!(validate_fais_license(&license).is_err());
    }

    #[test]
    fn test_fit_and_proper_compliant() {
        let fit_and_proper = FitAndProper {
            competent: true,
            financially_sound: true,
            honesty_and_integrity: true,
            no_disqualifying_convictions: true,
        };
        assert!(fit_and_proper.meets_requirements());
        assert!(validate_fit_and_proper(&fit_and_proper).is_ok());
    }

    #[test]
    fn test_fit_and_proper_not_competent() {
        let fit_and_proper = FitAndProper {
            competent: false,
            financially_sound: true,
            honesty_and_integrity: true,
            no_disqualifying_convictions: true,
        };
        assert!(!fit_and_proper.meets_requirements());
        assert!(validate_fit_and_proper(&fit_and_proper).is_err());
    }

    #[test]
    fn test_code_of_conduct() {
        let code = CodeOfConduct {
            honest_and_fair: true,
            skill_care_diligence: true,
            avoid_conflicts: true,
            disclose_material_info: true,
            know_your_customer: true,
            needs_analysis: true,
        };
        assert!(code.is_compliant());
    }

    #[test]
    fn test_tcf_outcomes() {
        let outcomes = TcfOutcome::all_outcomes();
        assert_eq!(outcomes.len(), 6);
    }

    #[test]
    fn test_twin_peaks_regulators() {
        let pa = TwinPeaksRegulator::PrudentialAuthority;
        assert!(pa.regulatory_focus().contains("Prudential"));

        let fsca = TwinPeaksRegulator::FinancialSectorConductAuthority;
        assert!(fsca.regulatory_focus().contains("Market conduct"));
    }

    #[test]
    fn test_financial_services_checklist() {
        let checklist = get_financial_services_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
