//! Core mining and resources types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Australian state/territory for mining purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiningJurisdiction {
    /// New South Wales
    NSW,
    /// Victoria
    VIC,
    /// Queensland
    QLD,
    /// South Australia
    SA,
    /// Western Australia
    WA,
    /// Tasmania
    TAS,
    /// Northern Territory
    NT,
}

impl MiningJurisdiction {
    /// Get the Mining Act for this jurisdiction
    pub fn mining_act(&self) -> &'static str {
        match self {
            MiningJurisdiction::NSW => "Mining Act 1992 (NSW)",
            MiningJurisdiction::VIC => "Mineral Resources (Sustainable Development) Act 1990 (Vic)",
            MiningJurisdiction::QLD => "Mineral Resources Act 1989 (Qld)",
            MiningJurisdiction::SA => "Mining Act 1971 (SA)",
            MiningJurisdiction::WA => "Mining Act 1978 (WA)",
            MiningJurisdiction::TAS => "Mineral Resources Development Act 1995 (Tas)",
            MiningJurisdiction::NT => "Mineral Titles Act 2010 (NT)",
        }
    }

    /// Get the mine safety legislation for this jurisdiction
    pub fn safety_act(&self) -> &'static str {
        match self {
            MiningJurisdiction::NSW => {
                "Work Health and Safety (Mines and Petroleum Sites) Act 2013 (NSW)"
            }
            MiningJurisdiction::VIC => "Occupational Health and Safety Act 2004 (Vic)",
            MiningJurisdiction::QLD => "Mining and Quarrying Safety and Health Act 1999 (Qld)",
            MiningJurisdiction::SA => "Work Health and Safety Act 2012 (SA)",
            MiningJurisdiction::WA => "Mines Safety and Inspection Act 1994 (WA)",
            MiningJurisdiction::TAS => "Work Health and Safety Act 2012 (Tas)",
            MiningJurisdiction::NT => {
                "Work Health and Safety (National Uniform Legislation) Act 2011 (NT)"
            }
        }
    }
}

/// Mining tenure type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TenureType {
    /// Exploration licence/permit
    ExplorationLicence,
    /// Prospecting licence (smaller scale exploration)
    ProspectingLicence,
    /// Mining lease (production)
    MiningLease,
    /// General purpose lease (infrastructure)
    GeneralPurposeLease,
    /// Retention licence (identified resource, not yet economic)
    RetentionLicence,
    /// Mineral development licence (evaluation/feasibility)
    MineralDevelopmentLicence,
    /// Assessment lease
    AssessmentLease,
}

impl TenureType {
    /// Whether this tenure allows production
    pub fn allows_production(&self) -> bool {
        matches!(self, TenureType::MiningLease)
    }

    /// Whether this tenure allows exploration
    pub fn allows_exploration(&self) -> bool {
        matches!(
            self,
            TenureType::ExplorationLicence
                | TenureType::ProspectingLicence
                | TenureType::MineralDevelopmentLicence
                | TenureType::AssessmentLease
        )
    }

    /// Typical maximum term in years
    pub fn typical_max_term_years(&self) -> u32 {
        match self {
            TenureType::ExplorationLicence => 5,
            TenureType::ProspectingLicence => 2,
            TenureType::MiningLease => 21,
            TenureType::GeneralPurposeLease => 21,
            TenureType::RetentionLicence => 5,
            TenureType::MineralDevelopmentLicence => 5,
            TenureType::AssessmentLease => 3,
        }
    }
}

/// Mining tenure/title
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiningTenure {
    /// Tenure identifier
    pub tenure_id: String,
    /// Tenure type
    pub tenure_type: TenureType,
    /// Jurisdiction
    pub jurisdiction: MiningJurisdiction,
    /// Holder name(s)
    pub holders: Vec<TenureHolder>,
    /// Grant date
    pub grant_date: NaiveDate,
    /// Expiry date
    pub expiry_date: NaiveDate,
    /// Area (hectares)
    pub area_hectares: f64,
    /// Minerals covered
    pub minerals: Vec<MineralType>,
    /// Current status
    pub status: TenureStatus,
    /// Native title status
    pub native_title_status: NativeTitleStatus,
    /// Environmental approvals
    pub environmental_approvals: Vec<EnvironmentalApproval>,
    /// Rental paid to date
    pub rental_paid: bool,
    /// Financial assurance/bond
    pub financial_assurance: Option<f64>,
}

/// Tenure holder
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenureHolder {
    /// Name
    pub name: String,
    /// Percentage interest
    pub interest_percentage: f64,
    /// ABN (if company)
    pub abn: Option<String>,
    /// Is operator
    pub is_operator: bool,
}

/// Tenure status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TenureStatus {
    /// Application pending
    Application,
    /// Granted and current
    Current,
    /// Suspended
    Suspended,
    /// Under renewal
    UnderRenewal,
    /// Expired
    Expired,
    /// Surrendered
    Surrendered,
    /// Cancelled/forfeited
    Cancelled,
}

/// Native title status for tenure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NativeTitleStatus {
    /// No native title claim or determination
    NotApplicable,
    /// Native title claim registered
    ClaimRegistered,
    /// Native title determined to exist
    NativeTitleExists,
    /// Native title extinguished
    Extinguished,
    /// Indigenous Land Use Agreement (ILUA) in place
    IluaInPlace,
    /// Right to negotiate in progress
    RightToNegotiate,
}

/// Mineral type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MineralType {
    /// Gold
    Gold,
    /// Iron ore
    IronOre,
    /// Coal
    Coal,
    /// Copper
    Copper,
    /// Lithium
    Lithium,
    /// Nickel
    Nickel,
    /// Zinc
    Zinc,
    /// Lead
    Lead,
    /// Silver
    Silver,
    /// Uranium
    Uranium,
    /// Rare earth elements
    RareEarth,
    /// Bauxite (aluminium)
    Bauxite,
    /// Manganese
    Manganese,
    /// Titanium minerals
    TitaniumMinerals,
    /// Industrial minerals (sand, gravel, etc.)
    IndustrialMinerals,
    /// Other
    Other,
}

impl MineralType {
    /// Whether this mineral requires special environmental assessment
    pub fn requires_special_assessment(&self) -> bool {
        matches!(self, MineralType::Uranium | MineralType::Coal)
    }

    /// Whether this is a strategic/critical mineral
    pub fn is_critical_mineral(&self) -> bool {
        matches!(
            self,
            MineralType::Lithium
                | MineralType::RareEarth
                | MineralType::Nickel
                | MineralType::Copper
                | MineralType::Manganese
        )
    }
}

/// Environmental approval
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalApproval {
    /// Approval type
    pub approval_type: EnvironmentalApprovalType,
    /// Approval reference
    pub reference: String,
    /// Grant date
    pub grant_date: NaiveDate,
    /// Expiry date (if applicable)
    pub expiry_date: Option<NaiveDate>,
    /// Conditions count
    pub conditions_count: u32,
}

/// Environmental approval type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnvironmentalApprovalType {
    /// EPBC Act approval (Commonwealth)
    EpbcApproval,
    /// State EIA approval
    StateEiaApproval,
    /// Environmental authority
    EnvironmentalAuthority,
    /// Licence to clear native vegetation
    ClearingPermit,
    /// Water licence
    WaterLicence,
    /// Works approval
    WorksApproval,
}

/// Mining project phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectPhase {
    /// Exploration/discovery
    Exploration,
    /// Feasibility study
    Feasibility,
    /// Approvals/permitting
    Approvals,
    /// Construction
    Construction,
    /// Operations/production
    Operations,
    /// Care and maintenance
    CareAndMaintenance,
    /// Closure
    Closure,
    /// Rehabilitation
    Rehabilitation,
    /// Post-closure monitoring
    PostClosure,
}

/// Mining project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiningProject {
    /// Project name
    pub name: String,
    /// Operator
    pub operator: String,
    /// Current phase
    pub phase: ProjectPhase,
    /// Tenures
    pub tenures: Vec<String>,
    /// Primary mineral
    pub primary_mineral: MineralType,
    /// Estimated mine life (years)
    pub estimated_mine_life: Option<u32>,
    /// Annual production capacity
    pub annual_production_capacity: Option<ProductionCapacity>,
    /// Workforce
    pub workforce: Option<u32>,
    /// Rehabilitation liability
    pub rehabilitation_liability: f64,
    /// Financial assurance held
    pub financial_assurance_held: f64,
}

/// Production capacity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductionCapacity {
    /// Quantity
    pub quantity: f64,
    /// Unit (tonnes, ounces, etc.)
    pub unit: ProductionUnit,
}

/// Production unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductionUnit {
    /// Tonnes
    Tonnes,
    /// Million tonnes
    MillionTonnes,
    /// Troy ounces
    TroyOunces,
    /// Kilograms
    Kilograms,
    /// Carats
    Carats,
}

/// Royalty type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoyaltyType {
    /// Ad valorem (percentage of value)
    AdValorem,
    /// Specific (per unit)
    Specific,
    /// Profit-based
    ProfitBased,
    /// Hybrid
    Hybrid,
}

/// Royalty calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoyaltyCalculation {
    /// Mineral type
    pub mineral: MineralType,
    /// Royalty type
    pub royalty_type: RoyaltyType,
    /// Rate (percentage or per unit)
    pub rate: f64,
    /// Value/quantity
    pub base_value: f64,
    /// Calculated royalty
    pub royalty_amount: f64,
    /// Period
    pub period: String,
}

/// Aboriginal heritage site
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeritageSite {
    /// Site identifier
    pub site_id: String,
    /// Site name
    pub name: String,
    /// Site type
    pub site_type: HeritageSiteType,
    /// Significance level
    pub significance: HeritageSignificance,
    /// Protected under
    pub protected_under: String,
    /// Traditional owners
    pub traditional_owners: String,
}

/// Heritage site type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeritageSiteType {
    /// Archaeological site
    Archaeological,
    /// Ceremonial/sacred site
    Ceremonial,
    /// Rock art
    RockArt,
    /// Burial site
    Burial,
    /// Artefact scatter
    ArtefactScatter,
    /// Mythological/story site
    Mythological,
    /// Resource/food gathering area
    Resource,
}

/// Heritage significance level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeritageSignificance {
    /// Local significance
    Local,
    /// State significance
    State,
    /// National significance
    National,
    /// World heritage
    World,
}

/// Mine closure plan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MineClosurePlan {
    /// Plan version
    pub version: String,
    /// Approval date
    pub approval_date: Option<NaiveDate>,
    /// Estimated closure date
    pub estimated_closure_date: Option<NaiveDate>,
    /// Total rehabilitation area (hectares)
    pub rehabilitation_area_hectares: f64,
    /// Estimated closure cost
    pub estimated_closure_cost: f64,
    /// Progressive rehabilitation completed (%)
    pub progressive_rehabilitation_percent: f64,
    /// Completion criteria defined
    pub completion_criteria_defined: bool,
    /// Post-closure monitoring period (years)
    pub post_closure_monitoring_years: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jurisdiction_acts() {
        assert!(
            MiningJurisdiction::WA
                .mining_act()
                .contains("Mining Act 1978")
        );
        assert!(
            MiningJurisdiction::QLD
                .mining_act()
                .contains("Mineral Resources Act")
        );
    }

    #[test]
    fn test_tenure_type_production() {
        assert!(TenureType::MiningLease.allows_production());
        assert!(!TenureType::ExplorationLicence.allows_production());
        assert!(TenureType::ExplorationLicence.allows_exploration());
    }

    #[test]
    fn test_mineral_classification() {
        assert!(MineralType::Uranium.requires_special_assessment());
        assert!(!MineralType::Gold.requires_special_assessment());
        assert!(MineralType::Lithium.is_critical_mineral());
        assert!(!MineralType::Gold.is_critical_mineral());
    }

    #[test]
    fn test_tenure_max_terms() {
        assert_eq!(TenureType::ExplorationLicence.typical_max_term_years(), 5);
        assert_eq!(TenureType::MiningLease.typical_max_term_years(), 21);
    }
}
