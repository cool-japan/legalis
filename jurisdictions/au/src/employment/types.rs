//! Employment Law Types
//!
//! Types for Australian employment law analysis.

use serde::{Deserialize, Serialize};

// ============================================================================
// Employment Types
// ============================================================================

/// Type of employment
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EmploymentType {
    /// Full-time permanent
    #[default]
    FullTime,
    /// Part-time permanent
    PartTime,
    /// Casual employee
    Casual,
    /// Fixed-term contract
    FixedTerm,
    /// Seasonal worker
    Seasonal,
    /// Labour hire
    LabourHire,
}

/// Employee category for protection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmployeeCategory {
    /// National system employee (covered by Fair Work Act)
    NationalSystem,
    /// State system employee (WA, some others)
    StateSystem,
    /// Federal public servant
    FederalPublicService,
    /// State public servant
    StatePublicService,
}

/// Employment instrument
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentInstrument {
    /// Modern award
    ModernAward(String),
    /// Enterprise agreement
    EnterpriseAgreement,
    /// Individual flexibility arrangement
    IndividualFlexibility,
    /// Contract of employment (award/agreement free)
    ContractOnly,
}

// ============================================================================
// National Employment Standards (NES)
// ============================================================================

/// National Employment Standard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NationalEmploymentStandard {
    /// Maximum weekly hours (s.62)
    MaximumWeeklyHours,
    /// Request for flexible working arrangements (s.65)
    FlexibleWorkingArrangements,
    /// Parental leave (s.67-85)
    ParentalLeave,
    /// Annual leave (s.86-94)
    AnnualLeave,
    /// Personal/carer's leave (s.95-107)
    PersonalCarersLeave,
    /// Compassionate leave (s.104-106)
    CompassionateLeave,
    /// Community service leave (s.108-112)
    CommunityServiceLeave,
    /// Long service leave (s.113)
    LongServiceLeave,
    /// Public holidays (s.114-116)
    PublicHolidays,
    /// Notice of termination (s.117-123)
    NoticeOfTermination,
    /// Redundancy pay (s.119-123)
    RedundancyPay,
    /// Fair Work Information Statement (s.125)
    FairWorkInformationStatement,
}

impl NationalEmploymentStandard {
    /// Get Fair Work Act section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::MaximumWeeklyHours => "s.62",
            Self::FlexibleWorkingArrangements => "ss.65-66",
            Self::ParentalLeave => "ss.67-85",
            Self::AnnualLeave => "ss.86-94",
            Self::PersonalCarersLeave => "ss.95-107",
            Self::CompassionateLeave => "ss.104-106",
            Self::CommunityServiceLeave => "ss.108-112",
            Self::LongServiceLeave => "s.113",
            Self::PublicHolidays => "ss.114-116",
            Self::NoticeOfTermination => "ss.117-123",
            Self::RedundancyPay => "ss.119-123",
            Self::FairWorkInformationStatement => "s.125",
        }
    }

    /// Get minimum entitlement
    pub fn minimum_entitlement(&self) -> &'static str {
        match self {
            Self::MaximumWeeklyHours => "38 hours (plus reasonable additional hours)",
            Self::FlexibleWorkingArrangements => "Right to request after 12 months",
            Self::ParentalLeave => "12 months unpaid (plus 12 months additional)",
            Self::AnnualLeave => "4 weeks per year (pro rata for part-time)",
            Self::PersonalCarersLeave => "10 days per year (pro rata)",
            Self::CompassionateLeave => "2 days per occasion",
            Self::CommunityServiceLeave => "Jury duty and emergency activities",
            Self::LongServiceLeave => "Varies by state (typically 2 months after 10 years)",
            Self::PublicHolidays => "Day off or penalty rates",
            Self::NoticeOfTermination => "1-4 weeks based on service",
            Self::RedundancyPay => "4-16 weeks based on service",
            Self::FairWorkInformationStatement => "Must be provided to new employees",
        }
    }
}

// ============================================================================
// Dismissal Types
// ============================================================================

/// Type of dismissal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DismissalType {
    /// Summary dismissal for serious misconduct
    Summary,
    /// Dismissal with notice
    WithNotice,
    /// Payment in lieu of notice
    PaymentInLieu,
    /// Constructive dismissal
    Constructive,
    /// Redundancy
    Redundancy,
    /// Fixed-term expiry
    FixedTermExpiry,
}

/// Reason for dismissal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DismissalReason {
    /// Conduct issues
    Conduct(ConductType),
    /// Performance issues
    Performance,
    /// Redundancy
    Redundancy,
    /// Capacity (ill-health)
    Capacity,
    /// Operational requirements
    OperationalRequirements,
    /// Fixed-term expiry
    FixedTermExpiry,
    /// Probationary period
    Probation,
    /// Abandonment of employment
    Abandonment,
}

/// Type of conduct issue
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConductType {
    /// Serious misconduct
    SeriousMisconduct,
    /// Misconduct (not serious)
    Misconduct,
    /// Policy breach
    PolicyBreach,
    /// Insubordination
    Insubordination,
    /// Dishonesty
    Dishonesty,
    /// Breach of duty
    BreachOfDuty,
}

// ============================================================================
// Unfair Dismissal
// ============================================================================

/// Unfair dismissal claim elements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnfairDismissalElement {
    /// Person was dismissed (s.385(a))
    PersonDismissed,
    /// Dismissal was harsh, unjust or unreasonable (s.385(b))
    HarshUnjustUnreasonable,
    /// Dismissal not consistent with Small Business Fair Dismissal Code (s.385(c))
    NotConsistentWithCode,
    /// Dismissal not genuine redundancy (s.385(d))
    NotGenuineRedundancy,
}

/// Unfair dismissal remedy type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnfairDismissalRemedy {
    /// Reinstatement to former position (s.391)
    Reinstatement,
    /// Reemployment in suitable position (s.391)
    Reemployment,
    /// Compensation (s.392)
    Compensation,
    /// Continuity of service order
    ContinuityOrder,
}

/// Factors for harsh, unjust, unreasonable (s.387)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section387Factor {
    /// Valid reason related to capacity or conduct (s.387(a))
    ValidReason,
    /// Notified of reason (s.387(b))
    NotifiedOfReason,
    /// Opportunity to respond (s.387(c))
    OpportunityToRespond,
    /// Unreasonable to have support person (s.387(d))
    SupportPerson,
    /// Warnings for unsatisfactory performance (s.387(e))
    WarningsGiven,
    /// Size of employer's enterprise (s.387(f))
    EnterpriseSize,
    /// HR specialists available (s.387(g))
    HRSpecialists,
    /// Other relevant matters (s.387(h))
    OtherMatters,
}

// ============================================================================
// General Protections (Adverse Action)
// ============================================================================

/// Protected attribute (Part 3-1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectedAttribute {
    /// Workplace right
    WorkplaceRight,
    /// Industrial activity
    IndustrialActivity,
    /// Discrimination (race, colour, sex, etc.)
    Discrimination(DiscriminationGround),
    /// Temporary absence (illness/injury)
    TemporaryAbsence,
}

/// Discrimination ground
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscriminationGround {
    /// Race
    Race,
    /// Colour
    Colour,
    /// Sex
    Sex,
    /// Sexual orientation
    SexualOrientation,
    /// Age
    Age,
    /// Physical or mental disability
    Disability,
    /// Marital status
    MaritalStatus,
    /// Family or carer's responsibilities
    FamilyResponsibilities,
    /// Pregnancy
    Pregnancy,
    /// Religion
    Religion,
    /// Political opinion
    PoliticalOpinion,
    /// National extraction
    NationalExtraction,
    /// Social origin
    SocialOrigin,
}

/// Type of adverse action (s.342)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdverseAction {
    /// Dismissal
    Dismissal,
    /// Injuring in employment
    InjuringInEmployment,
    /// Altering position to detriment
    AlteringPosition,
    /// Discriminating between employees
    Discriminating,
    /// Refusing to employ
    RefusingToEmploy,
    /// Discriminating in offers of employment
    DiscriminatingInOffers,
}

// ============================================================================
// Modern Awards
// ============================================================================

/// Award type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AwardType {
    /// Modern award (post-2010)
    Modern,
    /// Transitional instrument
    Transitional,
    /// Enterprise-specific
    Enterprise,
    /// State reference public sector
    StateReference,
}

/// Award coverage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwardCoverage {
    /// Award identifier
    pub award_id: String,
    /// Award name
    pub award_name: String,
    /// Industry/occupation covered
    pub coverage: String,
    /// Award type
    pub award_type: AwardType,
}

// ============================================================================
// Enterprise Agreements
// ============================================================================

/// Enterprise agreement type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnterpriseAgreementType {
    /// Single enterprise agreement (s.172(2)(a))
    SingleEnterprise,
    /// Multi-enterprise agreement (s.172(2)(b))
    MultiEnterprise,
    /// Greenfields agreement (s.172(4))
    Greenfields,
}

/// Better Off Overall Test (BOOT) result
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BootResult {
    /// Employees better off overall
    #[default]
    BetterOff,
    /// Employees not better off
    NotBetterOff,
    /// Marginal - requires undertakings
    Marginal,
}

// ============================================================================
// Key Cases
// ============================================================================

/// Key Australian employment law case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentCase {
    /// Case name
    pub name: String,
    /// Citation
    pub citation: String,
    /// Court
    pub court: String,
    /// Key principle
    pub principle: String,
}

impl EmploymentCase {
    /// Csomore v Public Service Board (1987) - Fair procedures
    pub fn csomore() -> Self {
        Self {
            name: "Csomore v Public Service Board".to_string(),
            citation: "(1987) 10 FCR 189".to_string(),
            court: "Federal Court".to_string(),
            principle: "Breach of Browne v Dunn rule in disciplinary context".to_string(),
        }
    }

    /// Byrne v Australian Airlines (1995) - Unfair dismissal common law
    pub fn byrne() -> Self {
        Self {
            name: "Byrne v Australian Airlines".to_string(),
            citation: "(1995) 185 CLR 410".to_string(),
            court: "High Court".to_string(),
            principle: "No implied term of fair procedure in employment contracts".to_string(),
        }
    }

    /// Sayer v Melsteel (2011) - Constructive dismissal
    pub fn sayer() -> Self {
        Self {
            name: "Sayer v Melsteel".to_string(),
            citation: "[2011] FWAFB 7498".to_string(),
            court: "Fair Work Australia Full Bench".to_string(),
            principle: "Resignation at employer's initiative can be dismissal".to_string(),
        }
    }

    /// Rankin v Marine Power (2001) - Redundancy genuineness
    pub fn rankin() -> Self {
        Self {
            name: "Rankin v Marine Power International".to_string(),
            citation: "(2001) 107 IR 117".to_string(),
            court: "Industrial Relations Court".to_string(),
            principle: "Genuine redundancy requires objective operational reason".to_string(),
        }
    }

    /// Spotless v Industrial Commission (1991) - Consultation
    pub fn spotless() -> Self {
        Self {
            name: "Re Spotless Services".to_string(),
            citation: "[1991] AIRC 2186".to_string(),
            court: "Australian Industrial Relations Commission".to_string(),
            principle: "Redundancy consultation requirements".to_string(),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nes_sections() {
        assert_eq!(
            NationalEmploymentStandard::MaximumWeeklyHours.section(),
            "s.62"
        );
        assert_eq!(
            NationalEmploymentStandard::AnnualLeave.section(),
            "ss.86-94"
        );
    }

    #[test]
    fn test_nes_entitlements() {
        let leave = NationalEmploymentStandard::AnnualLeave;
        assert!(leave.minimum_entitlement().contains("4 weeks"));
    }

    #[test]
    fn test_section_387_factors() {
        let factors = [
            Section387Factor::ValidReason,
            Section387Factor::NotifiedOfReason,
            Section387Factor::OpportunityToRespond,
            Section387Factor::SupportPerson,
            Section387Factor::WarningsGiven,
            Section387Factor::EnterpriseSize,
            Section387Factor::HRSpecialists,
            Section387Factor::OtherMatters,
        ];
        assert_eq!(factors.len(), 8);
    }

    #[test]
    fn test_byrne_case() {
        let case = EmploymentCase::byrne();
        assert!(case.citation.contains("185 CLR"));
    }
}
