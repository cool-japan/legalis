//! Strata and Community Title
//!
//! Implementation of strata schemes and community title under state legislation:
//! - Strata Schemes Management Act 2015 (NSW)
//! - Owners Corporation Act 2006 (VIC)
//! - Body Corporate and Community Management Act 1997 (QLD)
//!
//! ## Key Concepts
//!
//! - Lots and common property
//! - Owners corporations / bodies corporate
//! - Unit entitlements and contributions
//! - By-laws and rules
//! - Building defects and rectification
//!
//! ## Key Cases
//!
//! - The Owners - Strata Plan No 50276 v Thoo (2013) - By-law validity
//! - Railton v Owners Corporation PS423037 (2020) - Reasonableness of by-laws

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// =============================================================================
// Strata Scheme Structure
// =============================================================================

/// Strata scheme
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrataScheme {
    /// Strata plan number
    pub plan_number: String,
    /// Scheme name
    pub name: String,
    /// Jurisdiction
    pub jurisdiction: StrataJurisdiction,
    /// Scheme type
    pub scheme_type: StrataSchemeType,
    /// Registration date
    pub registration_date: NaiveDate,
    /// Total lots
    pub total_lots: u32,
    /// Total unit entitlements
    pub total_unit_entitlements: u64,
    /// Common property details
    pub common_property: CommonProperty,
    /// Building details
    pub building: BuildingDetails,
}

/// State jurisdiction for strata
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrataJurisdiction {
    /// New South Wales
    Nsw,
    /// Victoria
    Vic,
    /// Queensland
    Qld,
    /// Western Australia
    Wa,
    /// South Australia
    Sa,
    /// Tasmania
    Tas,
    /// Australian Capital Territory
    Act,
    /// Northern Territory
    Nt,
}

impl StrataJurisdiction {
    /// Get governing legislation
    pub fn legislation(&self) -> &'static str {
        match self {
            Self::Nsw => "Strata Schemes Management Act 2015 (NSW)",
            Self::Vic => "Owners Corporation Act 2006 (VIC)",
            Self::Qld => "Body Corporate and Community Management Act 1997 (QLD)",
            Self::Wa => "Strata Titles Act 1985 (WA)",
            Self::Sa => "Strata Titles Act 1988 (SA)",
            Self::Tas => "Strata Titles Act 1998 (TAS)",
            Self::Act => "Unit Titles (Management) Act 2011 (ACT)",
            Self::Nt => "Unit Titles Schemes Act 2009 (NT)",
        }
    }

    /// Get regulatory body terminology
    pub fn body_terminology(&self) -> &'static str {
        match self {
            Self::Nsw | Self::Act | Self::Nt => "Owners Corporation",
            Self::Vic => "Owners Corporation",
            Self::Qld => "Body Corporate",
            Self::Wa | Self::Sa | Self::Tas => "Strata Company",
        }
    }
}

/// Type of strata scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrataSchemeType {
    /// Building strata (standard)
    BuildingStrata,
    /// Strata subdivison of land
    LandStrata,
    /// Two-lot scheme
    TwoLotScheme,
    /// Community title
    CommunityTitle,
    /// Layered arrangement
    LayeredArrangement,
    /// Leasehold strata
    LeaseholdStrata,
}

/// Common property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonProperty {
    /// Description
    pub description: String,
    /// Includes structural elements
    pub structural_elements: bool,
    /// External walls
    pub external_walls: bool,
    /// Roof
    pub roof: bool,
    /// Common facilities
    pub facilities: Vec<CommonFacility>,
    /// Exclusive use areas
    pub exclusive_use_areas: Vec<ExclusiveUseArea>,
}

/// Common facility type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommonFacility {
    /// Swimming pool
    SwimmingPool,
    /// Gymnasium
    Gymnasium,
    /// Garden areas
    Garden,
    /// Lifts/elevators
    Lifts,
    /// Car parking (visitor)
    VisitorParking,
    /// Foyer/lobby
    Foyer,
    /// Function room
    FunctionRoom,
    /// BBQ area
    BbqArea,
    /// Tennis court
    TennisCourt,
    /// Security system
    SecuritySystem,
}

/// Exclusive use area
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExclusiveUseArea {
    /// Area description
    pub description: String,
    /// Lot number with exclusive use
    pub lot_number: u32,
    /// Area type
    pub area_type: ExclusiveUseType,
    /// By-law reference
    pub bylaw_reference: Option<String>,
}

/// Exclusive use area type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExclusiveUseType {
    /// Car parking space
    CarSpace,
    /// Storage cage
    StorageCage,
    /// Courtyard/garden
    Courtyard,
    /// Balcony extension
    BalconyExtension,
    /// Rooftop area
    Rooftop,
    /// Other
    Other,
}

/// Building details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingDetails {
    /// Year of construction
    pub year_constructed: Option<u32>,
    /// Number of storeys
    pub storeys: u32,
    /// Building class (BCA)
    pub building_class: BuildingClass,
    /// Heritage listed
    pub heritage_listed: bool,
    /// Developer/builder
    pub developer: Option<String>,
}

/// Building class under BCA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingClass {
    /// Class 2 - Residential apartment
    Class2,
    /// Class 3 - Residential (hostel, etc.)
    Class3,
    /// Mixed use
    Mixed,
    /// Commercial
    Commercial,
}

// =============================================================================
// Lots and Entitlements
// =============================================================================

/// Strata lot
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrataLot {
    /// Lot number
    pub lot_number: u32,
    /// Unit entitlement
    pub unit_entitlement: u64,
    /// Lot type
    pub lot_type: LotType,
    /// Level/floor
    pub level: Option<i32>,
    /// Floor area (sqm)
    pub floor_area: Option<f64>,
    /// Current owner
    pub owner: Option<LotOwner>,
    /// Tenanted
    pub tenanted: bool,
    /// Special by-laws applicable
    pub special_bylaws: Vec<String>,
}

/// Lot type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LotType {
    /// Residential
    Residential,
    /// Commercial
    Commercial,
    /// Retail
    Retail,
    /// Car space
    CarSpace,
    /// Storage
    Storage,
    /// Utility lot
    Utility,
}

/// Lot owner
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LotOwner {
    /// Owner name(s)
    pub names: Vec<String>,
    /// Ownership type
    pub ownership_type: OwnershipType,
    /// Address for notices
    pub address_for_notices: String,
    /// Email for electronic notices
    pub email: Option<String>,
}

/// Ownership type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OwnershipType {
    /// Individual
    Individual,
    /// Joint tenants
    JointTenants,
    /// Tenants in common
    TenantsInCommon,
    /// Company
    Company,
    /// Trust
    Trust,
}

// =============================================================================
// Owners Corporation
// =============================================================================

/// Owners corporation (body corporate)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnersCorporation {
    /// Name
    pub name: String,
    /// ABN
    pub abn: Option<String>,
    /// Strata manager
    pub strata_manager: Option<StrataManager>,
    /// Committee members
    pub committee: Vec<CommitteeMember>,
    /// Financial year end
    pub financial_year_end: FinancialYearEnd,
    /// Insurance details
    pub insurance: InsuranceDetails,
    /// Current levies
    pub levies: LevyStructure,
}

/// Strata manager
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrataManager {
    /// Company name
    pub company_name: String,
    /// License number
    pub license_number: String,
    /// Contact name
    pub contact_name: String,
    /// Contact details
    pub contact_phone: String,
    /// Email
    pub email: String,
    /// Appointment date
    pub appointment_date: NaiveDate,
    /// Agreement expiry
    pub agreement_expiry: Option<NaiveDate>,
}

/// Committee member
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitteeMember {
    /// Name
    pub name: String,
    /// Lot number
    pub lot_number: u32,
    /// Position
    pub position: CommitteePosition,
    /// Appointment date
    pub appointment_date: NaiveDate,
}

/// Committee position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommitteePosition {
    /// Chairperson
    Chairperson,
    /// Secretary
    Secretary,
    /// Treasurer
    Treasurer,
    /// Ordinary member
    OrdinaryMember,
}

/// Financial year end
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialYearEnd {
    /// June
    June,
    /// December
    December,
    /// Other month
    Other(u8),
}

/// Insurance details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InsuranceDetails {
    /// Building sum insured
    pub building_sum_insured: f64,
    /// Public liability amount
    pub public_liability: f64,
    /// Workers compensation (if applicable)
    pub workers_compensation: bool,
    /// Office bearers liability
    pub office_bearers_liability: bool,
    /// Insurer name
    pub insurer: String,
    /// Policy number
    pub policy_number: String,
    /// Renewal date
    pub renewal_date: NaiveDate,
}

/// Levy structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevyStructure {
    /// Administrative fund levy rate (per unit entitlement)
    pub admin_fund_rate: f64,
    /// Capital works fund levy rate
    pub capital_works_rate: f64,
    /// Special levy (if any)
    pub special_levy: Option<SpecialLevy>,
    /// Payment frequency
    pub payment_frequency: PaymentFrequency,
}

/// Special levy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialLevy {
    /// Purpose
    pub purpose: String,
    /// Total amount
    pub total_amount: f64,
    /// Rate per unit entitlement
    pub rate_per_ue: f64,
    /// Payment due date
    pub due_date: NaiveDate,
}

/// Payment frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentFrequency {
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Half-yearly
    HalfYearly,
    /// Yearly
    Yearly,
}

// =============================================================================
// By-Laws
// =============================================================================

/// By-law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByLaw {
    /// By-law number
    pub number: u32,
    /// Title
    pub title: String,
    /// Category
    pub category: ByLawCategory,
    /// Content
    pub content: String,
    /// Registration date
    pub registration_date: Option<NaiveDate>,
    /// Model by-law or custom
    pub is_model: bool,
}

/// By-law category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ByLawCategory {
    /// Safety and security
    SafetySecurity,
    /// Pets/animals
    Pets,
    /// Parking
    Parking,
    /// Noise and behaviour
    NoiseBehaviour,
    /// Common property use
    CommonPropertyUse,
    /// Appearance/aesthetics
    Appearance,
    /// Short-term letting
    ShortTermLetting,
    /// Renovation/alterations
    Renovations,
    /// Other
    Other,
}

/// By-law validity assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ByLawValidity {
    /// Is by-law valid
    pub valid: bool,
    /// Issues found
    pub issues: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

// =============================================================================
// Building Defects
// =============================================================================

/// Building defect claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingDefect {
    /// Defect description
    pub description: String,
    /// Defect type
    pub defect_type: DefectType,
    /// Location
    pub location: DefectLocation,
    /// Discovery date
    pub discovery_date: NaiveDate,
    /// Severity
    pub severity: DefectSeverity,
    /// Responsible party
    pub responsible_party: ResponsibleParty,
    /// Warranty status
    pub warranty_status: WarrantyStatus,
}

/// Type of defect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefectType {
    /// Structural defect
    Structural,
    /// Waterproofing failure
    Waterproofing,
    /// Fire safety
    FireSafety,
    /// Electrical
    Electrical,
    /// Plumbing
    Plumbing,
    /// Finishing/cosmetic
    Finishing,
    /// HVAC
    Hvac,
    /// Facade/cladding
    FacadeCladding,
    /// Other major defect
    OtherMajor,
}

/// Defect location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefectLocation {
    /// Common property
    CommonProperty,
    /// Individual lot
    IndividualLot,
    /// Both
    Both,
}

/// Defect severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefectSeverity {
    /// Minor
    Minor,
    /// Moderate
    Moderate,
    /// Major
    Major,
    /// Critical (safety risk)
    Critical,
}

/// Party responsible for defect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResponsibleParty {
    /// Developer
    Developer,
    /// Builder
    Builder,
    /// Subcontractor
    Subcontractor,
    /// Certifier
    Certifier,
    /// Unknown
    Unknown,
}

/// Warranty status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarrantyStatus {
    /// Within statutory warranty period
    pub within_statutory_warranty: bool,
    /// Warranty type applicable
    pub warranty_type: WarrantyType,
    /// Years remaining
    pub years_remaining: Option<f64>,
    /// Builder warranty insurance
    pub builder_warranty_insurance: bool,
}

/// Warranty type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WarrantyType {
    /// Structural (typically 10 years NSW)
    Structural,
    /// Major defects (6 years NSW)
    MajorDefects,
    /// Minor defects (2 years NSW)
    MinorDefects,
    /// None applicable
    None,
}

// =============================================================================
// Meetings and Resolutions
// =============================================================================

/// General meeting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralMeeting {
    /// Meeting type
    pub meeting_type: MeetingType,
    /// Date
    pub date: NaiveDate,
    /// Location
    pub location: String,
    /// Notice given
    pub notice_date: NaiveDate,
    /// Agenda items
    pub agenda: Vec<AgendaItem>,
    /// Quorum present
    pub quorum_present: bool,
    /// Minutes recorded
    pub minutes_recorded: bool,
}

/// Meeting type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeetingType {
    /// Annual General Meeting
    Agm,
    /// Extraordinary General Meeting
    Egm,
    /// First AGM (developer handover)
    FirstAgm,
}

/// Agenda item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgendaItem {
    /// Item number
    pub number: u32,
    /// Description
    pub description: String,
    /// Resolution type required
    pub resolution_type: ResolutionType,
    /// Result
    pub result: Option<ResolutionResult>,
}

/// Resolution type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Ordinary resolution (simple majority)
    Ordinary,
    /// Special resolution (75%)
    Special,
    /// Unanimous resolution (100%)
    Unanimous,
    /// No resolution required (information only)
    InformationOnly,
}

/// Resolution result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionResult {
    /// Passed
    pub passed: bool,
    /// Votes for
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Abstentions
    pub abstentions: u64,
    /// Percentage in favor
    pub percentage_in_favor: f64,
}

// =============================================================================
// Validation Functions
// =============================================================================

/// Calculate levy for a lot
pub fn calculate_lot_levy(
    lot: &StrataLot,
    scheme: &StrataScheme,
    levy_structure: &LevyStructure,
) -> LevyCalculation {
    let ue_proportion = lot.unit_entitlement as f64 / scheme.total_unit_entitlements as f64;

    let admin_levy = levy_structure.admin_fund_rate * lot.unit_entitlement as f64;
    let capital_works_levy = levy_structure.capital_works_rate * lot.unit_entitlement as f64;
    let special_levy = levy_structure
        .special_levy
        .as_ref()
        .map(|s| s.rate_per_ue * lot.unit_entitlement as f64)
        .unwrap_or(0.0);

    let total = admin_levy + capital_works_levy + special_levy;
    let per_period = match levy_structure.payment_frequency {
        PaymentFrequency::Monthly => total / 12.0,
        PaymentFrequency::Quarterly => total / 4.0,
        PaymentFrequency::HalfYearly => total / 2.0,
        PaymentFrequency::Yearly => total,
    };

    LevyCalculation {
        lot_number: lot.lot_number,
        unit_entitlement: lot.unit_entitlement,
        ue_proportion,
        admin_fund_annual: admin_levy,
        capital_works_annual: capital_works_levy,
        special_levy_annual: special_levy,
        total_annual: total,
        per_period,
        payment_frequency: levy_structure.payment_frequency,
    }
}

/// Levy calculation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevyCalculation {
    /// Lot number
    pub lot_number: u32,
    /// Unit entitlement
    pub unit_entitlement: u64,
    /// UE proportion
    pub ue_proportion: f64,
    /// Admin fund annual levy
    pub admin_fund_annual: f64,
    /// Capital works annual levy
    pub capital_works_annual: f64,
    /// Special levy annual
    pub special_levy_annual: f64,
    /// Total annual levy
    pub total_annual: f64,
    /// Amount per payment period
    pub per_period: f64,
    /// Payment frequency
    pub payment_frequency: PaymentFrequency,
}

/// Validate by-law
pub fn validate_bylaw(bylaw: &ByLaw, jurisdiction: StrataJurisdiction) -> ByLawValidity {
    let mut issues = Vec::new();
    let mut legal_references = vec![jurisdiction.legislation().to_string()];

    // Check for prohibited content
    let content_lower = bylaw.content.to_lowercase();

    // Discriminatory by-laws
    if content_lower.contains("race")
        || content_lower.contains("religion")
        || content_lower.contains("gender")
        || content_lower.contains("disability")
    {
        issues.push("By-law may be discriminatory".to_string());
    }

    // Unreasonable restrictions on lot owners
    if content_lower.contains("prohibit")
        && (content_lower.contains("children") || content_lower.contains("family"))
    {
        issues.push("Cannot unreasonably restrict families".to_string());
        legal_references
            .push("The Owners - Strata Plan No 50276 v Thoo [2013] NSWCA 270".to_string());
    }

    // Pet by-laws after 2021 reforms (NSW)
    if matches!(jurisdiction, StrataJurisdiction::Nsw)
        && matches!(bylaw.category, ByLawCategory::Pets)
        && content_lower.contains("ban")
        && content_lower.contains("pet")
    {
        issues.push("Blanket pet bans not enforceable in NSW since 2021".to_string());
        legal_references.push("Strata Schemes Management Amendment Act 2021 (NSW)".to_string());
    }

    // Short-term letting restrictions
    if matches!(bylaw.category, ByLawCategory::ShortTermLetting) {
        legal_references.push("Check state-specific short-term letting regulations".to_string());
    }

    ByLawValidity {
        valid: issues.is_empty(),
        issues,
        legal_references,
    }
}

/// Assess building defect claim
pub fn assess_defect_claim(
    defect: &BuildingDefect,
    completion_date: NaiveDate,
    assessment_date: NaiveDate,
) -> DefectClaimAssessment {
    let years_since_completion = (assessment_date - completion_date).num_days() as f64 / 365.25;

    // Determine warranty period based on defect type
    let (warranty_years, warranty_type) = match defect.defect_type {
        DefectType::Structural => (10.0, WarrantyType::Structural),
        DefectType::Waterproofing
        | DefectType::FireSafety
        | DefectType::FacadeCladding
        | DefectType::OtherMajor => (6.0, WarrantyType::MajorDefects),
        _ => (2.0, WarrantyType::MinorDefects),
    };

    let within_warranty = years_since_completion <= warranty_years;
    let years_remaining = if within_warranty {
        Some(warranty_years - years_since_completion)
    } else {
        None
    };

    let mut remedies = Vec::new();
    let mut legal_references = vec!["Home Building Act 1989 (NSW)".to_string()];

    if within_warranty {
        remedies.push(DefectRemedy::RectificationOrder);
        if defect.warranty_status.builder_warranty_insurance {
            remedies.push(DefectRemedy::InsuranceClaim);
        }
    }

    // NCAT/tribunal claim
    if matches!(
        defect.severity,
        DefectSeverity::Major | DefectSeverity::Critical
    ) {
        remedies.push(DefectRemedy::TribunalClaim);
    }

    // Court action for major defects
    if matches!(defect.severity, DefectSeverity::Critical) {
        remedies.push(DefectRemedy::CourtAction);
        legal_references
            .push("Owners Corporation SP 62930 v Kell & Rigby [2010] NSWSC 612".to_string());
    }

    DefectClaimAssessment {
        defect_type: defect.defect_type,
        within_warranty,
        warranty_type,
        years_remaining,
        available_remedies: remedies,
        urgency: if matches!(defect.severity, DefectSeverity::Critical) {
            ClaimUrgency::Urgent
        } else if matches!(defect.severity, DefectSeverity::Major) {
            ClaimUrgency::High
        } else {
            ClaimUrgency::Normal
        },
        legal_references,
    }
}

/// Defect claim assessment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefectClaimAssessment {
    /// Defect type
    pub defect_type: DefectType,
    /// Within warranty period
    pub within_warranty: bool,
    /// Warranty type
    pub warranty_type: WarrantyType,
    /// Years remaining
    pub years_remaining: Option<f64>,
    /// Available remedies
    pub available_remedies: Vec<DefectRemedy>,
    /// Urgency
    pub urgency: ClaimUrgency,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Defect remedy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefectRemedy {
    /// Builder rectification
    RectificationOrder,
    /// Insurance claim
    InsuranceClaim,
    /// Tribunal/NCAT claim
    TribunalClaim,
    /// Court action
    CourtAction,
}

/// Claim urgency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClaimUrgency {
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Urgent (safety)
    Urgent,
}

/// Check if resolution threshold met
pub fn check_resolution_threshold(
    resolution_type: ResolutionType,
    votes_for: u64,
    votes_against: u64,
    total_unit_entitlements: u64,
) -> ResolutionCheck {
    let total_votes = votes_for + votes_against;
    let percentage = if total_votes > 0 {
        (votes_for as f64 / total_votes as f64) * 100.0
    } else {
        0.0
    };

    let (threshold_met, required_percentage) = match resolution_type {
        ResolutionType::Ordinary => (percentage > 50.0, 50.0),
        ResolutionType::Special => (percentage >= 75.0, 75.0),
        ResolutionType::Unanimous => (
            votes_for == total_unit_entitlements && votes_against == 0,
            100.0,
        ),
        ResolutionType::InformationOnly => (true, 0.0),
    };

    ResolutionCheck {
        resolution_type,
        votes_for,
        votes_against,
        percentage_for: percentage,
        required_percentage,
        threshold_met,
    }
}

/// Resolution check result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionCheck {
    /// Resolution type
    pub resolution_type: ResolutionType,
    /// Votes for
    pub votes_for: u64,
    /// Votes against
    pub votes_against: u64,
    /// Percentage in favor
    pub percentage_for: f64,
    /// Required percentage
    pub required_percentage: f64,
    /// Threshold met
    pub threshold_met: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_scheme() -> StrataScheme {
        StrataScheme {
            plan_number: "SP12345".to_string(),
            name: "Test Apartments".to_string(),
            jurisdiction: StrataJurisdiction::Nsw,
            scheme_type: StrataSchemeType::BuildingStrata,
            registration_date: NaiveDate::from_ymd_opt(2015, 1, 1).expect("valid date"),
            total_lots: 50,
            total_unit_entitlements: 10000,
            common_property: CommonProperty {
                description: "Common property".to_string(),
                structural_elements: true,
                external_walls: true,
                roof: true,
                facilities: vec![CommonFacility::SwimmingPool, CommonFacility::Gymnasium],
                exclusive_use_areas: vec![],
            },
            building: BuildingDetails {
                year_constructed: Some(2015),
                storeys: 10,
                building_class: BuildingClass::Class2,
                heritage_listed: false,
                developer: Some("Test Developer".to_string()),
            },
        }
    }

    #[test]
    fn test_calculate_lot_levy() {
        let scheme = create_test_scheme();
        let lot = StrataLot {
            lot_number: 1,
            unit_entitlement: 200,
            lot_type: LotType::Residential,
            level: Some(3),
            floor_area: Some(80.0),
            owner: None,
            tenanted: false,
            special_bylaws: vec![],
        };
        let levy_structure = LevyStructure {
            admin_fund_rate: 10.0,
            capital_works_rate: 5.0,
            special_levy: None,
            payment_frequency: PaymentFrequency::Quarterly,
        };

        let result = calculate_lot_levy(&lot, &scheme, &levy_structure);
        assert_eq!(result.lot_number, 1);
        assert_eq!(result.admin_fund_annual, 2000.0); // 200 UE * $10
        assert_eq!(result.capital_works_annual, 1000.0); // 200 UE * $5
        assert_eq!(result.total_annual, 3000.0);
        assert_eq!(result.per_period, 750.0); // Quarterly
    }

    #[test]
    fn test_validate_bylaw_valid() {
        let bylaw = ByLaw {
            number: 1,
            title: "Noise restrictions".to_string(),
            category: ByLawCategory::NoiseBehaviour,
            content: "No excessive noise after 10pm".to_string(),
            registration_date: Some(NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date")),
            is_model: true,
        };

        let result = validate_bylaw(&bylaw, StrataJurisdiction::Nsw);
        assert!(result.valid);
    }

    #[test]
    fn test_validate_bylaw_pet_ban_invalid() {
        let bylaw = ByLaw {
            number: 5,
            title: "Pet restrictions".to_string(),
            category: ByLawCategory::Pets,
            content: "All pets are banned from the building".to_string(),
            registration_date: None,
            is_model: false,
        };

        let result = validate_bylaw(&bylaw, StrataJurisdiction::Nsw);
        assert!(!result.valid);
        assert!(result.issues.iter().any(|i| i.contains("pet ban")));
    }

    #[test]
    fn test_assess_defect_within_warranty() {
        let defect = BuildingDefect {
            description: "Waterproofing failure in bathroom".to_string(),
            defect_type: DefectType::Waterproofing,
            location: DefectLocation::CommonProperty,
            discovery_date: NaiveDate::from_ymd_opt(2023, 1, 1).expect("valid date"),
            severity: DefectSeverity::Major,
            responsible_party: ResponsibleParty::Builder,
            warranty_status: WarrantyStatus {
                within_statutory_warranty: true,
                warranty_type: WarrantyType::MajorDefects,
                years_remaining: Some(3.0),
                builder_warranty_insurance: true,
            },
        };

        let completion = NaiveDate::from_ymd_opt(2020, 1, 1).expect("valid date");
        let assessment = NaiveDate::from_ymd_opt(2023, 1, 1).expect("valid date");

        let result = assess_defect_claim(&defect, completion, assessment);
        assert!(result.within_warranty);
        assert!(matches!(result.warranty_type, WarrantyType::MajorDefects));
    }

    #[test]
    fn test_resolution_ordinary_passed() {
        let result = check_resolution_threshold(ResolutionType::Ordinary, 60, 40, 10000);
        assert!(result.threshold_met);
        assert_eq!(result.percentage_for, 60.0);
    }

    #[test]
    fn test_resolution_special_failed() {
        let result = check_resolution_threshold(ResolutionType::Special, 70, 30, 10000);
        assert!(!result.threshold_met);
        assert_eq!(result.required_percentage, 75.0);
    }

    #[test]
    fn test_jurisdiction_legislation() {
        assert!(
            StrataJurisdiction::Nsw
                .legislation()
                .contains("Strata Schemes Management Act")
        );
        assert!(
            StrataJurisdiction::Vic
                .legislation()
                .contains("Owners Corporation Act")
        );
        assert!(
            StrataJurisdiction::Qld
                .legislation()
                .contains("Body Corporate")
        );
    }
}
