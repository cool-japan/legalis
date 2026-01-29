//! South African Environmental Law
//!
//! Constitutional environmental right and comprehensive environmental management.
//!
//! ## Key Legislation
//!
//! - Constitution s24 (environmental right)
//! - National Environmental Management Act 107 of 1998 (NEMA)
//! - National Environmental Management: Waste Act 59 of 2008
//! - National Water Act 36 of 1998
//! - National Environmental Management: Air Quality Act 39 of 2004
//! - National Environmental Management: Biodiversity Act 10 of 2004
//! - National Environmental Management: Protected Areas Act 57 of 2003
//!
//! ## Principles
//!
//! - Sustainable development
//! - Precautionary principle
//! - Polluter pays
//! - Cradle to grave responsibility

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for environmental operations
pub type EnvironmentalResult<T> = Result<T, EnvironmentalError>;

/// Constitutional environmental right (s24)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnvironmentalRight {
    /// Environment not harmful to health or well-being
    NotHarmful,
    /// Protected for present and future generations through:
    /// - Reasonable legislative measures
    /// - Prevention of pollution and ecological degradation
    /// - Promotion of conservation
    /// - Ecologically sustainable development and use of natural resources
    Protected,
}

/// NEMA principles (s2)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NemaPrinciple {
    /// Sustainable development
    SustainableDevelopment,
    /// Precautionary principle
    PrecautionaryPrinciple,
    /// Polluter pays
    PolluterPays,
    /// Waste minimization and recycling
    WasteMinimization,
    /// Public participation
    PublicParticipation,
    /// Environmental justice
    EnvironmentalJustice,
    /// Cradle to grave responsibility
    CradleToGrave,
}

impl NemaPrinciple {
    /// Get principle description
    pub fn description(&self) -> &'static str {
        match self {
            Self::SustainableDevelopment => {
                "Development must be socially, environmentally and economically sustainable"
            }
            Self::PrecautionaryPrinciple => {
                "Where threats of serious or irreversible damage exist, lack of full scientific certainty shall not postpone cost-effective measures"
            }
            Self::PolluterPays => {
                "Costs of remedying pollution, environmental degradation and adverse health effects shall be paid by responsible parties"
            }
            Self::WasteMinimization => {
                "Waste must be avoided or minimized, reused or recycled where possible"
            }
            Self::PublicParticipation => {
                "Affected parties must have opportunity to participate in environmental governance"
            }
            Self::EnvironmentalJustice => {
                "Adverse environmental impacts shall not be distributed inequitably"
            }
            Self::CradleToGrave => {
                "Responsibility for environmental impacts throughout product life cycle"
            }
        }
    }
}

/// Environmental authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalAuthorization {
    /// Activity description
    pub activity: String,
    /// Authorization type
    pub authorization_type: AuthorizationType,
    /// Environmental Impact Assessment (EIA) conducted
    pub eia_conducted: bool,
    /// Public participation process completed
    pub public_participation: bool,
    /// Authorized
    pub authorized: bool,
    /// Environmental Management Programme (EMPr)
    pub empr_approved: bool,
}

impl EnvironmentalAuthorization {
    /// Validate authorization requirements
    pub fn is_valid(&self) -> bool {
        match self.authorization_type {
            AuthorizationType::BasicAssessment => {
                self.eia_conducted && self.public_participation && self.authorized
            }
            AuthorizationType::FullScoping => {
                self.eia_conducted
                    && self.public_participation
                    && self.authorized
                    && self.empr_approved
            }
            AuthorizationType::Listing1
            | AuthorizationType::Listing2
            | AuthorizationType::Listing3 => self.authorized,
        }
    }
}

/// Authorization types under NEMA
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthorizationType {
    /// Basic Assessment
    BasicAssessment,
    /// Full Scoping and EIA
    FullScoping,
    /// Listed Activity 1 (basic assessment)
    Listing1,
    /// Listed Activity 2 (EIA)
    Listing2,
    /// Listed Activity 3 (specific geographical areas)
    Listing3,
}

impl AuthorizationType {
    /// Processing timeline (days)
    pub fn processing_timeline_days(&self) -> u32 {
        match self {
            Self::BasicAssessment | Self::Listing1 => 90,
            Self::FullScoping | Self::Listing2 => 106, // Scoping 44 days + EIA 62 days
            Self::Listing3 => 90,
        }
    }
}

/// Waste classification (Waste Act s69)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WasteClassification {
    /// General waste
    GeneralWaste,
    /// Hazardous waste
    HazardousWaste,
}

impl WasteClassification {
    /// Requires waste management license
    pub fn requires_license(&self, activity: &WasteActivity) -> bool {
        match (self, activity) {
            (Self::HazardousWaste, _) => true,
            (Self::GeneralWaste, WasteActivity::Storage) => false,
            (Self::GeneralWaste, _) => true,
        }
    }
}

/// Waste management activities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WasteActivity {
    /// Storage
    Storage,
    /// Treatment
    Treatment,
    /// Disposal
    Disposal,
    /// Recovery
    Recovery,
}

/// Water use authorization (National Water Act s21)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WaterUse {
    /// Taking and storing water
    TakingAndStoring,
    /// Impeding or diverting flow
    ImpedingDiverting,
    /// Stream flow reduction activity
    StreamFlowReduction,
    /// Controlled activity (alters bed/banks/course/characteristics)
    ControlledActivity,
    /// Discharge of waste/water containing waste
    DischargeWaste,
    /// Disposing waste in manner which may detrimentally impact water resource
    DisposingWaste,
}

impl WaterUse {
    /// Requires water use license
    pub fn requires_license(&self) -> bool {
        true // All s21 water uses require authorization (license or general authorization)
    }

    /// Statutory reference
    pub fn section_reference(&self) -> &'static str {
        match self {
            Self::TakingAndStoring => "s21(a)",
            Self::ImpedingDiverting => "s21(b)",
            Self::StreamFlowReduction => "s21(c)",
            Self::ControlledActivity => "s21(i)",
            Self::DischargeWaste => "s21(f)",
            Self::DisposingWaste => "s21(g)",
        }
    }
}

/// Air quality management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirQualityLicense {
    /// Activity subject to atmospheric emission license
    pub activity: String,
    /// Licensed
    pub is_licensed: bool,
    /// Emission standards complied with
    pub emission_standards_compliant: bool,
    /// Monitoring and reporting
    pub monitoring_and_reporting: bool,
}

impl AirQualityLicense {
    /// Validate compliance
    pub fn is_compliant(&self) -> bool {
        self.is_licensed && self.emission_standards_compliant && self.monitoring_and_reporting
    }
}

/// Biodiversity management
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiodiversityThreat {
    /// Threatened or protected species
    ThreatenedSpecies,
    /// Alien invasive species
    AlienInvasiveSpecies,
    /// Habitat loss/degradation
    HabitatLoss,
    /// Protected areas
    ProtectedAreas,
}

impl BiodiversityThreat {
    /// Permit required
    pub fn requires_permit(&self) -> bool {
        matches!(
            self,
            Self::ThreatenedSpecies | Self::AlienInvasiveSpecies | Self::ProtectedAreas
        )
    }
}

/// Environmental compliance and enforcement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceNotice {
    /// Pre-compliance notice (s31A NEMA)
    PreCompliance,
    /// Compliance notice (s31L)
    ComplianceNotice,
    /// Directive (s28)
    Directive,
    /// Stop work order
    StopWorkOrder,
}

impl ComplianceNotice {
    /// Timeline to comply (days)
    pub fn compliance_timeline_days(&self) -> u32 {
        match self {
            Self::PreCompliance => 20,
            Self::ComplianceNotice => 60, // Variable, set in notice
            Self::Directive => 30,        // Variable
            Self::StopWorkOrder => 0,     // Immediate
        }
    }
}

/// Duty of care (s28 NEMA)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyOfCare {
    /// Pollution/degradation caused
    pub caused_pollution: bool,
    /// Reasonable measures taken to prevent/mitigate
    pub reasonable_measures: bool,
    /// Remediation conducted
    pub remediation_conducted: bool,
}

impl DutyOfCare {
    /// Liable under s28
    pub fn is_liable(&self) -> bool {
        self.caused_pollution && !self.reasonable_measures
    }

    /// Who is liable (s28(1))
    pub fn liable_parties() -> Vec<&'static str> {
        vec![
            "Person who caused pollution/degradation",
            "Person who owns/controls/occupied land at time of pollution",
            "Person who negligently failed to prevent pollution",
        ]
    }
}

/// Environmental errors
#[derive(Debug, Error)]
pub enum EnvironmentalError {
    /// No environmental authorization
    #[error("Activity requires environmental authorization (NEMA s24): {activity}")]
    NoAuthorization { activity: String },

    /// Violation of environmental right
    #[error("Violation of environmental right (Constitution s24): {description}")]
    EnvironmentalRightViolation { description: String },

    /// Duty of care breach
    #[error("Duty of care breach (s28 NEMA): {breach}")]
    DutyOfCareBreach { breach: String },

    /// Waste management license required
    #[error("Waste management license required (Waste Act s20): {activity}")]
    WasteLicenseRequired { activity: String },

    /// Water use license required
    #[error("Water use license required (National Water Act s21): {use_type}")]
    WaterLicenseRequired { use_type: String },

    /// Non-compliance with NEMA principles
    #[error("Non-compliance with NEMA principles (s2): {principle}")]
    PrincipleViolation { principle: String },

    /// Administrative penalty
    #[error("Environmental administrative penalty: {description}")]
    AdministrativePenalty { description: String },
}

/// Validate environmental authorization
pub fn validate_environmental_authorization(
    authorization: &EnvironmentalAuthorization,
) -> EnvironmentalResult<()> {
    if !authorization.authorized {
        return Err(EnvironmentalError::NoAuthorization {
            activity: authorization.activity.clone(),
        });
    }

    if !authorization.is_valid() {
        return Err(EnvironmentalError::NoAuthorization {
            activity: format!("{} (incomplete assessment process)", authorization.activity),
        });
    }

    Ok(())
}

/// Validate duty of care compliance
pub fn validate_duty_of_care(duty: &DutyOfCare) -> EnvironmentalResult<()> {
    if duty.is_liable() {
        return Err(EnvironmentalError::DutyOfCareBreach {
            breach: "Failed to take reasonable measures to prevent/mitigate pollution".to_string(),
        });
    }

    if duty.caused_pollution && !duty.remediation_conducted {
        return Err(EnvironmentalError::DutyOfCareBreach {
            breach: "Remediation not conducted".to_string(),
        });
    }

    Ok(())
}

/// Get environmental compliance checklist
pub fn get_environmental_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Environmental authorization obtained", "s24 NEMA"),
        ("EIA conducted (if required)", "s24(2)"),
        ("Public participation completed", "s2(4)(f)"),
        ("EMPr approved and implemented", "s24N"),
        ("Waste management license", "s20 Waste Act"),
        ("Water use license/authorization", "s21 NWA"),
        ("Air quality license (if applicable)", "s36 AQA"),
        ("Biodiversity permits", "s57 Biodiversity Act"),
        ("Duty of care compliance", "s28 NEMA"),
        ("NEMA principles applied", "s2"),
        ("Environmental monitoring", "EMPr"),
        ("Incident reporting (significant impact)", "s30 NEMA"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nema_principles() {
        let precautionary = NemaPrinciple::PrecautionaryPrinciple;
        assert!(precautionary.description().contains("scientific certainty"));

        let polluter_pays = NemaPrinciple::PolluterPays;
        assert!(polluter_pays.description().contains("remedying pollution"));
    }

    #[test]
    fn test_environmental_authorization_valid() {
        let auth = EnvironmentalAuthorization {
            activity: "Mining operation".to_string(),
            authorization_type: AuthorizationType::BasicAssessment,
            eia_conducted: true,
            public_participation: true,
            authorized: true,
            empr_approved: false,
        };
        assert!(auth.is_valid());
        assert!(validate_environmental_authorization(&auth).is_ok());
    }

    #[test]
    fn test_environmental_authorization_invalid() {
        let auth = EnvironmentalAuthorization {
            activity: "Development".to_string(),
            authorization_type: AuthorizationType::FullScoping,
            eia_conducted: true,
            public_participation: true,
            authorized: true,
            empr_approved: false, // Required for full scoping
        };
        assert!(!auth.is_valid());
    }

    #[test]
    fn test_authorization_timelines() {
        assert_eq!(
            AuthorizationType::BasicAssessment.processing_timeline_days(),
            90
        );
        assert_eq!(
            AuthorizationType::FullScoping.processing_timeline_days(),
            106
        );
    }

    #[test]
    fn test_waste_classification() {
        assert!(WasteClassification::HazardousWaste.requires_license(&WasteActivity::Storage));
        assert!(!WasteClassification::GeneralWaste.requires_license(&WasteActivity::Storage));
        assert!(WasteClassification::GeneralWaste.requires_license(&WasteActivity::Treatment));
    }

    #[test]
    fn test_water_use_authorization() {
        assert!(WaterUse::TakingAndStoring.requires_license());
        assert_eq!(WaterUse::DischargeWaste.section_reference(), "s21(f)");
    }

    #[test]
    fn test_air_quality_compliance() {
        let license = AirQualityLicense {
            activity: "Industrial emissions".to_string(),
            is_licensed: true,
            emission_standards_compliant: true,
            monitoring_and_reporting: true,
        };
        assert!(license.is_compliant());
    }

    #[test]
    fn test_biodiversity_permits() {
        assert!(BiodiversityThreat::ThreatenedSpecies.requires_permit());
        assert!(BiodiversityThreat::AlienInvasiveSpecies.requires_permit());
        assert!(!BiodiversityThreat::HabitatLoss.requires_permit());
    }

    #[test]
    fn test_compliance_notice_timelines() {
        assert_eq!(
            ComplianceNotice::PreCompliance.compliance_timeline_days(),
            20
        );
        assert_eq!(
            ComplianceNotice::StopWorkOrder.compliance_timeline_days(),
            0
        );
    }

    #[test]
    fn test_duty_of_care_liable() {
        let duty = DutyOfCare {
            caused_pollution: true,
            reasonable_measures: false,
            remediation_conducted: false,
        };
        assert!(duty.is_liable());
        assert!(validate_duty_of_care(&duty).is_err());
    }

    #[test]
    fn test_duty_of_care_compliant() {
        let duty = DutyOfCare {
            caused_pollution: true,
            reasonable_measures: true,
            remediation_conducted: true,
        };
        assert!(!duty.is_liable());
        assert!(validate_duty_of_care(&duty).is_ok());
    }

    #[test]
    fn test_duty_of_care_no_remediation() {
        let duty = DutyOfCare {
            caused_pollution: true,
            reasonable_measures: true,
            remediation_conducted: false,
        };
        assert!(validate_duty_of_care(&duty).is_err());
    }

    #[test]
    fn test_environmental_checklist() {
        let checklist = get_environmental_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 10);
    }
}
