//! Product Liability
//!
//! Implementation of product liability under Australian Consumer Law
//! (Competition and Consumer Act 2010, Schedule 2, Part 3-5).
//!
//! ## Key Legislation
//!
//! - Competition and Consumer Act 2010 (Cth), Schedule 2, Part 3-5
//! - Formerly Trade Practices Act 1974, Part VA
//!
//! ## Key Concepts
//!
//! Australian product liability law provides strict liability for defective goods:
//! - No need to prove negligence
//! - Manufacturer is liable for defects
//! - Multiple defendants (manufacturer, importer, supplier)
//! - Defences available (state of the art, compliance with mandatory standard)
//!
//! ## Key Cases
//!
//! - Graham Barclay Oysters v Ryan (2002) - Product defect definition
//! - Carey v Lake Macquarie City Council (2007) - Safety expectation
//! - Peterson v Merck Sharp & Dohme (2010) - Causation in drug cases

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// =============================================================================
// Product Definitions
// =============================================================================

/// Product subject to liability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Product {
    /// Product name
    pub name: String,
    /// Product category
    pub category: ProductCategory,
    /// Manufacturer
    pub manufacturer: ManufacturerDetails,
    /// Date of manufacture
    pub manufacture_date: Option<NaiveDate>,
    /// Date of supply to consumer
    pub supply_date: NaiveDate,
    /// Serial/batch number
    pub batch_number: Option<String>,
    /// Safety standards applicable
    pub applicable_standards: Vec<SafetyStandard>,
}

/// Product category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductCategory {
    /// Food and beverages
    Food,
    /// Pharmaceuticals and medicines
    Pharmaceutical,
    /// Medical devices
    MedicalDevice,
    /// Motor vehicles
    MotorVehicle,
    /// Electrical appliances
    ElectricalAppliance,
    /// Children's products/toys
    ChildrenProduct,
    /// Machinery and equipment
    Machinery,
    /// Building products
    BuildingProduct,
    /// Cosmetics and personal care
    Cosmetics,
    /// Other consumer goods
    Other,
}

/// Manufacturer details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManufacturerDetails {
    /// Company name
    pub name: String,
    /// Country of manufacture
    pub country: String,
    /// Is manufacturer identifiable
    pub identifiable: bool,
    /// Is Australian entity
    pub is_australian_entity: bool,
}

/// Safety standard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyStandard {
    /// Standard name/number
    pub name: String,
    /// Is mandatory standard
    pub mandatory: bool,
    /// Standard body (e.g., AS/NZS, ISO)
    pub standard_body: String,
    /// Product complies
    pub product_complies: bool,
}

// =============================================================================
// Defect Types
// =============================================================================

/// Product defect
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductDefect {
    /// Defect type
    pub defect_type: DefectType,
    /// Description of defect
    pub description: String,
    /// When defect discovered
    pub discovery_date: NaiveDate,
    /// Defect existed at time of supply
    pub existed_at_supply: bool,
    /// Safety expectation test
    pub safety_expectation: SafetyExpectation,
}

/// Type of defect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefectType {
    /// Manufacturing defect (product differs from design)
    Manufacturing,
    /// Design defect (all products have same flaw)
    Design,
    /// Warning/instruction defect (inadequate warnings)
    Warning,
    /// Marketing defect (misleading representations)
    Marketing,
}

impl DefectType {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Manufacturing => "Product differs from intended design or specification",
            Self::Design => "Inherent flaw in product design affecting all units",
            Self::Warning => "Inadequate warnings or instructions for safe use",
            Self::Marketing => "Misleading marketing about product safety or use",
        }
    }
}

/// Safety expectation test (s.9 ACL)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyExpectation {
    /// What persons generally expected
    pub general_expectation: String,
    /// Manner of marketing
    pub marketing_manner: Option<String>,
    /// Packaging and labelling
    pub packaging: Option<String>,
    /// Instructions provided
    pub instructions_provided: bool,
    /// Warnings provided
    pub warnings_provided: bool,
    /// What reasonable person would expect
    pub reasonable_expectation: String,
    /// Does product meet expectation
    pub meets_expectation: bool,
}

/// Factors for safety expectation (s.9(2) ACL)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SafetyFactor {
    /// Manner of marketing
    MarketingManner,
    /// Get-up (packaging, labelling)
    GetUp,
    /// Instructions and warnings
    InstructionsWarnings,
    /// Reasonably foreseeable use
    ForeseeableUse,
    /// Time of supply
    TimeOfSupply,
}

// =============================================================================
// Parties and Liability
// =============================================================================

/// Potential defendant in product liability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PotentialDefendant {
    /// Party name
    pub name: String,
    /// Role in supply chain
    pub role: SupplyChainRole,
    /// Basis of liability
    pub liability_basis: Vec<LiabilityBasis>,
    /// Available defences
    pub available_defences: Vec<ProductLiabilityDefence>,
    /// Within jurisdiction
    pub within_jurisdiction: bool,
}

/// Role in supply chain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupplyChainRole {
    /// Manufacturer (s.7 ACL)
    Manufacturer,
    /// Importer (deemed manufacturer if true manufacturer unknown)
    Importer,
    /// Distributor
    Distributor,
    /// Retailer/supplier
    Retailer,
    /// Brand owner (who holds out as manufacturer)
    BrandOwner,
    /// Component manufacturer
    ComponentManufacturer,
}

impl SupplyChainRole {
    /// Get primary section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::Manufacturer => "s.138, s.7(1)(a)",
            Self::Importer => "s.7(1)(b)",
            Self::Distributor => "s.141",
            Self::Retailer => "s.141",
            Self::BrandOwner => "s.7(1)(c)",
            Self::ComponentManufacturer => "s.138",
        }
    }

    /// Is strictly liable (without need to prove fault)
    pub fn is_strictly_liable(&self) -> bool {
        matches!(self, Self::Manufacturer | Self::BrandOwner | Self::Importer)
    }
}

/// Basis of liability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiabilityBasis {
    /// Strict liability under Part 3-5
    StrictLiability,
    /// Negligence at common law
    Negligence,
    /// Consumer guarantee breach (s.54)
    ConsumerGuaranteeBreach,
    /// Misleading conduct (s.18)
    MisleadingConduct,
}

// =============================================================================
// Defences
// =============================================================================

/// Product liability defence (s.142 ACL)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductLiabilityDefence {
    /// Defect did not exist at supply time (s.142(a))
    NoDefectAtSupply,
    /// Defect only exists due to compliance with mandatory standard (s.142(c))
    MandatoryStandardCompliance,
    /// State of scientific/technical knowledge (s.142(c))
    StateOfTheArt,
    /// Defect only in component, due to compliance with design instructions (s.142(d))
    ComponentCompliance,
    /// Limitation period expired (10 years)
    LimitationExpired,
}

impl ProductLiabilityDefence {
    /// Get section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::NoDefectAtSupply => "s.142(a)",
            Self::MandatoryStandardCompliance => "s.142(c)",
            Self::StateOfTheArt => "s.142(c)",
            Self::ComponentCompliance => "s.142(d)",
            Self::LimitationExpired => "s.143",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::NoDefectAtSupply => {
                "Defect did not exist at time goods were supplied by manufacturer"
            }
            Self::MandatoryStandardCompliance => {
                "Defect attributable only to compliance with mandatory standard"
            }
            Self::StateOfTheArt => {
                "State of scientific/technical knowledge at supply could not discover defect"
            }
            Self::ComponentCompliance => {
                "Component manufacturer followed design/instructions from product manufacturer"
            }
            Self::LimitationExpired => "More than 10 years since supply by manufacturer",
        }
    }
}

// =============================================================================
// Harm and Damages
// =============================================================================

/// Harm suffered
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HarmSuffered {
    /// Harm type
    pub harm_type: HarmType,
    /// Description
    pub description: String,
    /// Date of harm
    pub harm_date: NaiveDate,
    /// Injuries/losses
    pub injuries: Vec<Injury>,
    /// Property damage
    pub property_damage: Option<PropertyDamage>,
    /// Consequential economic loss
    pub economic_loss: Option<f64>,
}

/// Type of harm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HarmType {
    /// Death
    Death,
    /// Personal injury
    PersonalInjury,
    /// Property damage (other than the defective product)
    PropertyDamage,
    /// Economic loss
    EconomicLoss,
}

/// Injury details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Injury {
    /// Injury description
    pub description: String,
    /// Severity
    pub severity: InjurySeverity,
    /// Is permanent
    pub permanent: bool,
    /// Treatment required
    pub treatment: String,
}

/// Injury severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InjurySeverity {
    /// Minor
    Minor,
    /// Moderate
    Moderate,
    /// Serious
    Serious,
    /// Severe/catastrophic
    Severe,
}

/// Property damage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertyDamage {
    /// Description of property
    pub property_description: String,
    /// Value of damage
    pub damage_value: f64,
    /// Is property ordinarily for private use
    pub private_use_property: bool,
}

/// Recoverable loss types under Part 3-5
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecoverableLoss {
    /// Death damages
    Death,
    /// Personal injury damages
    PersonalInjury,
    /// Property damage (if private use property > $500)
    PropertyDamage,
    /// Consequential economic loss
    ConsequentialEconomicLoss,
}

// =============================================================================
// Limitation Periods
// =============================================================================

/// Limitation period analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitationAnalysis {
    /// Date of supply by manufacturer
    pub supply_date: NaiveDate,
    /// Date harm first suffered
    pub harm_date: NaiveDate,
    /// Date cause of action discovered/discoverable
    pub discovery_date: NaiveDate,
    /// Assessment date
    pub assessment_date: NaiveDate,
    /// 10-year long-stop expired
    pub long_stop_expired: bool,
    /// 3-year limitation expired
    pub three_year_expired: bool,
    /// Limitation extended (disability, fraud)
    pub limitation_extended: bool,
    /// Time remaining (if any)
    pub time_remaining_days: Option<i64>,
}

impl LimitationAnalysis {
    /// Calculate limitation status
    pub fn calculate(
        supply_date: NaiveDate,
        harm_date: NaiveDate,
        discovery_date: NaiveDate,
        assessment_date: NaiveDate,
    ) -> Self {
        let days_since_supply = (assessment_date - supply_date).num_days();
        let days_since_discovery = (assessment_date - discovery_date).num_days();

        let long_stop_expired = days_since_supply > 365 * 10;
        let three_year_expired = days_since_discovery > 365 * 3;

        let time_remaining_days = if !long_stop_expired && !three_year_expired {
            let long_stop_remaining = (365 * 10) - days_since_supply;
            let three_year_remaining = (365 * 3) - days_since_discovery;
            Some(long_stop_remaining.min(three_year_remaining))
        } else {
            None
        };

        Self {
            supply_date,
            harm_date,
            discovery_date,
            assessment_date,
            long_stop_expired,
            three_year_expired,
            limitation_extended: false,
            time_remaining_days,
        }
    }
}

// =============================================================================
// Liability Assessment
// =============================================================================

/// Product liability claim facts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLiabilityFacts {
    /// Product details
    pub product: Product,
    /// Defect details
    pub defect: ProductDefect,
    /// Harm suffered
    pub harm: HarmSuffered,
    /// Potential defendants
    pub defendants: Vec<PotentialDefendant>,
    /// Plaintiff details
    pub plaintiff: PlaintiffDetails,
}

/// Plaintiff details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaintiffDetails {
    /// Name
    pub name: String,
    /// Was purchaser
    pub was_purchaser: bool,
    /// Was user
    pub was_user: bool,
    /// Was bystander
    pub was_bystander: bool,
    /// Contributed to harm
    pub contributory_negligence: Option<ContributoryNegligence>,
}

/// Contributory negligence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributoryNegligence {
    /// Description
    pub description: String,
    /// Apportionment percentage (0-100)
    pub apportionment_percentage: u32,
}

/// Product liability assessment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLiabilityAssessment {
    /// Claim likely to succeed
    pub liable: bool,
    /// Elements established
    pub elements: ProductLiabilityElements,
    /// Liable defendants
    pub liable_defendants: Vec<String>,
    /// Defences that may apply
    pub applicable_defences: Vec<ProductLiabilityDefence>,
    /// Limitation status
    pub limitation: LimitationAnalysis,
    /// Damages assessment
    pub damages: DamagesAssessment,
    /// Issues/concerns
    pub issues: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Product liability elements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductLiabilityElements {
    /// Product supplied by manufacturer
    pub supplied_by_manufacturer: bool,
    /// Product has defect
    pub product_has_defect: bool,
    /// Defect caused harm
    pub defect_caused_harm: bool,
    /// Harm is recoverable type
    pub recoverable_harm: bool,
    /// Within limitation period
    pub within_limitation: bool,
}

/// Damages assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamagesAssessment {
    /// Personal injury damages
    pub personal_injury: Option<f64>,
    /// Property damage
    pub property_damage: Option<f64>,
    /// Economic loss
    pub economic_loss: Option<f64>,
    /// Total estimated damages
    pub total_estimated: f64,
    /// Contributory negligence reduction
    pub contributory_reduction_percentage: Option<u32>,
    /// Net amount after reduction
    pub net_amount: f64,
}

// =============================================================================
// Assessment Functions
// =============================================================================

/// Assess product liability claim
pub fn assess_product_liability(facts: &ProductLiabilityFacts) -> ProductLiabilityAssessment {
    let mut issues = Vec::new();
    let mut legal_references = vec![
        "Competition and Consumer Act 2010 (Cth), Schedule 2, Part 3-5".to_string(),
        "Australian Consumer Law s.138-141".to_string(),
    ];

    // Check elements
    let supplied_by_manufacturer = facts.defendants.iter().any(|d| {
        matches!(
            d.role,
            SupplyChainRole::Manufacturer | SupplyChainRole::BrandOwner
        )
    });

    if !supplied_by_manufacturer {
        issues.push("No identifiable manufacturer defendant".to_string());
        legal_references.push("s.7(1) - manufacturer definition".to_string());
    }

    let product_has_defect =
        facts.defect.existed_at_supply && !facts.defect.safety_expectation.meets_expectation;
    if !product_has_defect {
        issues.push("Defect at time of supply not established".to_string());
    }

    // Causation
    let defect_caused_harm = true; // Simplified - would need detailed analysis
    if !defect_caused_harm {
        issues.push("Causation between defect and harm not established".to_string());
    }

    // Recoverable harm
    let recoverable_harm = match facts.harm.harm_type {
        HarmType::Death | HarmType::PersonalInjury => true,
        HarmType::PropertyDamage => {
            if let Some(ref pd) = facts.harm.property_damage {
                pd.private_use_property && pd.damage_value > 500.0
            } else {
                false
            }
        }
        HarmType::EconomicLoss => false, // Pure economic loss not recoverable under Part 3-5
    };

    if !recoverable_harm {
        issues.push("Type of loss may not be recoverable under Part 3-5".to_string());
        legal_references.push("s.138 - recoverable loss types".to_string());
    }

    // Limitation
    let limitation = LimitationAnalysis::calculate(
        facts.product.supply_date,
        facts.harm.harm_date,
        facts.defect.discovery_date,
        chrono::Local::now().naive_local().date(),
    );

    let within_limitation = !limitation.long_stop_expired && !limitation.three_year_expired;
    if !within_limitation {
        issues.push("Claim may be statute-barred".to_string());
        legal_references.push("s.143 - limitation period".to_string());
    }

    // Identify liable defendants
    let liable_defendants: Vec<String> = facts
        .defendants
        .iter()
        .filter(|d| d.role.is_strictly_liable() && d.within_jurisdiction)
        .map(|d| d.name.clone())
        .collect();

    // Check defences
    let mut applicable_defences = Vec::new();
    for defendant in &facts.defendants {
        for defence in &defendant.available_defences {
            if !applicable_defences.contains(defence) {
                applicable_defences.push(*defence);
            }
        }
    }

    // Calculate damages
    let personal_injury = facts.harm.injuries.iter().fold(0.0, |acc, injury| {
        acc + match injury.severity {
            InjurySeverity::Minor => 5_000.0,
            InjurySeverity::Moderate => 50_000.0,
            InjurySeverity::Serious => 200_000.0,
            InjurySeverity::Severe => 500_000.0,
        }
    });

    let property_damage = facts
        .harm
        .property_damage
        .as_ref()
        .map(|pd| pd.damage_value);
    let economic_loss = facts.harm.economic_loss;

    let total_estimated =
        personal_injury + property_damage.unwrap_or(0.0) + economic_loss.unwrap_or(0.0);

    let contributory_reduction = facts
        .plaintiff
        .contributory_negligence
        .as_ref()
        .map(|cn| cn.apportionment_percentage);

    let net_amount = if let Some(reduction) = contributory_reduction {
        total_estimated * (100 - reduction) as f64 / 100.0
    } else {
        total_estimated
    };

    let elements = ProductLiabilityElements {
        supplied_by_manufacturer,
        product_has_defect,
        defect_caused_harm,
        recoverable_harm,
        within_limitation,
    };

    let liable = supplied_by_manufacturer
        && product_has_defect
        && defect_caused_harm
        && recoverable_harm
        && within_limitation
        && applicable_defences.is_empty();

    ProductLiabilityAssessment {
        liable,
        elements,
        liable_defendants,
        applicable_defences,
        limitation,
        damages: DamagesAssessment {
            personal_injury: if personal_injury > 0.0 {
                Some(personal_injury)
            } else {
                None
            },
            property_damage,
            economic_loss,
            total_estimated,
            contributory_reduction_percentage: contributory_reduction,
            net_amount,
        },
        issues,
        legal_references,
    }
}

/// Assess whether product has defect under s.9 safety test
pub fn assess_product_defect(product: &Product, defect: &ProductDefect) -> DefectAssessment {
    let mut factors = Vec::new();
    let mut is_defective = false;

    // Marketing manner
    if defect.safety_expectation.marketing_manner.is_some() {
        factors.push(SafetyFactor::MarketingManner);
    }

    // Get-up (packaging)
    if defect.safety_expectation.packaging.is_some() {
        factors.push(SafetyFactor::GetUp);
    }

    // Instructions and warnings
    if !defect.safety_expectation.instructions_provided
        || !defect.safety_expectation.warnings_provided
    {
        factors.push(SafetyFactor::InstructionsWarnings);
        if matches!(defect.defect_type, DefectType::Warning) {
            is_defective = true;
        }
    }

    // Safety expectation not met
    if !defect.safety_expectation.meets_expectation {
        is_defective = true;
    }

    // Check mandatory standards
    for standard in &product.applicable_standards {
        if standard.mandatory && !standard.product_complies {
            is_defective = true;
        }
    }

    DefectAssessment {
        is_defective,
        defect_type: defect.defect_type,
        factors_considered: factors,
        safety_expectation_met: defect.safety_expectation.meets_expectation,
        mandatory_standards_complied: product
            .applicable_standards
            .iter()
            .filter(|s| s.mandatory)
            .all(|s| s.product_complies),
        legal_references: vec!["ACL s.9 - Meaning of defect".to_string()],
    }
}

/// Defect assessment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefectAssessment {
    /// Is product defective
    pub is_defective: bool,
    /// Type of defect
    pub defect_type: DefectType,
    /// Factors considered
    pub factors_considered: Vec<SafetyFactor>,
    /// Safety expectation met
    pub safety_expectation_met: bool,
    /// Mandatory standards complied
    pub mandatory_standards_complied: bool,
    /// Legal references
    pub legal_references: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_product() -> Product {
        Product {
            name: "Test Product".to_string(),
            category: ProductCategory::ElectricalAppliance,
            manufacturer: ManufacturerDetails {
                name: "Test Manufacturer".to_string(),
                country: "Australia".to_string(),
                identifiable: true,
                is_australian_entity: true,
            },
            manufacture_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date")),
            supply_date: NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date"),
            batch_number: Some("BATCH001".to_string()),
            applicable_standards: vec![SafetyStandard {
                name: "AS/NZS 3820:2009".to_string(),
                mandatory: true,
                standard_body: "Standards Australia".to_string(),
                product_complies: true,
            }],
        }
    }

    #[test]
    fn test_defect_type_descriptions() {
        assert!(DefectType::Manufacturing.description().contains("differs"));
        assert!(DefectType::Design.description().contains("design"));
        assert!(DefectType::Warning.description().contains("warning"));
    }

    #[test]
    fn test_supply_chain_role_sections() {
        assert!(SupplyChainRole::Manufacturer.section().contains("138"));
        assert!(SupplyChainRole::Importer.section().contains("7(1)(b)"));
        assert!(SupplyChainRole::Manufacturer.is_strictly_liable());
        assert!(!SupplyChainRole::Retailer.is_strictly_liable());
    }

    #[test]
    fn test_defence_sections() {
        assert_eq!(
            ProductLiabilityDefence::NoDefectAtSupply.section(),
            "s.142(a)"
        );
        assert_eq!(ProductLiabilityDefence::StateOfTheArt.section(), "s.142(c)");
        assert_eq!(
            ProductLiabilityDefence::LimitationExpired.section(),
            "s.143"
        );
    }

    #[test]
    fn test_limitation_analysis_within_period() {
        let supply = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let harm = NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date");
        let discovery = NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date");
        let assessment = NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date");

        let result = LimitationAnalysis::calculate(supply, harm, discovery, assessment);

        assert!(!result.long_stop_expired);
        assert!(!result.three_year_expired);
        assert!(result.time_remaining_days.is_some());
    }

    #[test]
    fn test_limitation_analysis_expired() {
        let supply = NaiveDate::from_ymd_opt(2010, 1, 1).expect("valid date");
        let harm = NaiveDate::from_ymd_opt(2015, 1, 1).expect("valid date");
        let discovery = NaiveDate::from_ymd_opt(2015, 1, 1).expect("valid date");
        let assessment = NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date");

        let result = LimitationAnalysis::calculate(supply, harm, discovery, assessment);

        assert!(result.long_stop_expired);
        assert!(result.time_remaining_days.is_none());
    }

    #[test]
    fn test_assess_product_defect_defective() {
        let product = create_test_product();
        let defect = ProductDefect {
            defect_type: DefectType::Manufacturing,
            description: "Faulty wiring".to_string(),
            discovery_date: NaiveDate::from_ymd_opt(2024, 7, 1).expect("valid date"),
            existed_at_supply: true,
            safety_expectation: SafetyExpectation {
                general_expectation: "Product should be safe".to_string(),
                marketing_manner: Some("Promoted as safe".to_string()),
                packaging: Some("Standard packaging".to_string()),
                instructions_provided: true,
                warnings_provided: true,
                reasonable_expectation: "Should not cause electrical shock".to_string(),
                meets_expectation: false,
            },
        };

        let result = assess_product_defect(&product, &defect);
        assert!(result.is_defective);
        assert!(!result.safety_expectation_met);
    }

    #[test]
    fn test_assess_product_defect_no_warnings() {
        let product = create_test_product();
        let defect = ProductDefect {
            defect_type: DefectType::Warning,
            description: "No safety warnings provided".to_string(),
            discovery_date: NaiveDate::from_ymd_opt(2024, 7, 1).expect("valid date"),
            existed_at_supply: true,
            safety_expectation: SafetyExpectation {
                general_expectation: "Should have warnings".to_string(),
                marketing_manner: None,
                packaging: None,
                instructions_provided: true,
                warnings_provided: false,
                reasonable_expectation: "Warnings about risks".to_string(),
                meets_expectation: false,
            },
        };

        let result = assess_product_defect(&product, &defect);
        assert!(result.is_defective);
        assert!(
            result
                .factors_considered
                .contains(&SafetyFactor::InstructionsWarnings)
        );
    }

    #[test]
    fn test_injury_severity() {
        let minor = InjurySeverity::Minor;
        let severe = InjurySeverity::Severe;
        assert!(matches!(minor, InjurySeverity::Minor));
        assert!(matches!(severe, InjurySeverity::Severe));
    }
}
