//! Consumer law types
//!
//! Types for the Competition and Consumer Act 2010 (Cth) and
//! Australian Consumer Law (ACL) enforcement.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// =============================================================================
// Product Safety
// =============================================================================

/// Product safety status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductSafetyStatus {
    /// No safety issues
    Safe,
    /// Under investigation
    UnderInvestigation,
    /// Voluntary recall
    VoluntaryRecall,
    /// Mandatory recall
    MandatoryRecall,
    /// Banned
    Banned,
    /// Interim ban
    InterimBan,
}

/// Product recall type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecallType {
    /// Voluntary recall initiated by supplier
    Voluntary,
    /// Mandatory recall ordered by Minister
    Mandatory,
}

/// Product recall
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductRecall {
    /// Recall number
    pub recall_number: String,
    /// Product name
    pub product_name: String,
    /// Product description
    pub product_description: String,
    /// Supplier name
    pub supplier_name: String,
    /// Recall type
    pub recall_type: RecallType,
    /// Recall date
    pub recall_date: NaiveDate,
    /// Hazard description
    pub hazard_description: String,
    /// Defect description
    pub defect_description: String,
    /// Remedy offered (refund, repair, replacement)
    pub remedy: String,
    /// Contact details
    pub contact_details: String,
}

/// Mandatory safety standard
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyStandard {
    /// Standard reference
    pub standard_reference: String,
    /// Standard name
    pub standard_name: String,
    /// Product category
    pub product_category: ProductCategory,
    /// Effective date
    pub effective_date: NaiveDate,
    /// Key requirements
    pub key_requirements: Vec<String>,
}

/// Product category for safety standards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductCategory {
    /// Children's toys
    ChildrensToys,
    /// Children's clothing
    ChildrensClothing,
    /// Baby products (cots, strollers, etc.)
    BabyProducts,
    /// Electrical goods
    ElectricalGoods,
    /// Cosmetics
    Cosmetics,
    /// Motor vehicles
    MotorVehicles,
    /// Sunglasses/fashion spectacles
    Sunglasses,
    /// Furniture
    Furniture,
    /// Blinds/curtains
    BlindsCurtains,
    /// Swimming/flotation aids
    SwimmingAids,
    /// Other
    Other,
}

/// Mandatory injury report
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjuryReport {
    /// Report date
    pub report_date: NaiveDate,
    /// Product name
    pub product_name: String,
    /// Product identifier (model, batch, etc.)
    pub product_identifier: String,
    /// Supplier name
    pub supplier_name: String,
    /// Injury type
    pub injury_type: InjuryType,
    /// Injury description
    pub injury_description: String,
    /// Date of incident
    pub incident_date: NaiveDate,
    /// Reporter details
    pub reporter_name: String,
}

/// Injury type for mandatory reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InjuryType {
    /// Death
    Death,
    /// Serious injury requiring medical/surgical treatment
    SeriousInjury,
    /// Illness requiring medical treatment
    Illness,
}

// =============================================================================
// ACCC Enforcement
// =============================================================================

/// ACCC enforcement action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnforcementActionType {
    /// Infringement notice
    InfringementNotice,
    /// Court undertaking (s.87B)
    CourtUndertaking,
    /// Civil penalty proceedings
    CivilPenalty,
    /// Criminal prosecution
    CriminalProsecution,
    /// Injunction
    Injunction,
    /// Adverse publicity order
    AdversePublicityOrder,
    /// Non-punitive order
    NonPunitiveOrder,
    /// Disqualification order
    DisqualificationOrder,
}

/// Infringement notice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InfringementNotice {
    /// Notice number
    pub notice_number: String,
    /// Date issued
    pub date_issued: NaiveDate,
    /// Recipient (individual or body corporate)
    pub recipient: String,
    /// Recipient type
    pub recipient_type: RecipientType,
    /// Contravention
    pub contravention: AclContravention,
    /// Penalty amount
    pub penalty_amount: f64,
    /// Compliance period (days)
    pub compliance_period_days: u32,
    /// Status
    pub status: InfringementNoticeStatus,
}

/// Recipient type for penalties
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RecipientType {
    /// Individual
    Individual,
    /// Body corporate
    BodyCorporate,
    /// Listed corporation
    ListedCorporation,
}

impl RecipientType {
    /// Get penalty multiplier (individual = 1, body corporate = 5, listed = 10)
    pub fn penalty_multiplier(&self) -> u32 {
        match self {
            RecipientType::Individual => 1,
            RecipientType::BodyCorporate => 5,
            RecipientType::ListedCorporation => 10,
        }
    }
}

/// Infringement notice status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InfringementNoticeStatus {
    /// Issued, awaiting response
    Issued,
    /// Paid
    Paid,
    /// Withdrawn
    Withdrawn,
    /// Not paid - proceeding to court
    UnpaidProceeding,
    /// Expired
    Expired,
}

/// ACL contravention type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AclContravention {
    /// s.18 - Misleading or deceptive conduct
    MisleadingConduct,
    /// s.29 - False or misleading representations
    FalseRepresentations,
    /// s.34 - Bait advertising
    BaitAdvertising,
    /// s.35 - Accepting payment without intent to supply
    AcceptPaymentNoSupply,
    /// s.50 - Unconscionable conduct (general)
    UnconscionableGeneral,
    /// s.51 - Unconscionable conduct (trade/commerce)
    UnconscionableTradeCommerce,
    /// Consumer guarantee breach
    ConsumerGuaranteeBreach,
    /// Product safety breach
    ProductSafetyBreach,
    /// Unfair contract term
    UnfairContractTerm,
    /// Unsolicited consumer agreement breach
    UnsolicitedAgreementBreach,
    /// Country of origin breach
    CountryOfOriginBreach,
}

impl AclContravention {
    /// Get the ACL section
    pub fn section(&self) -> &'static str {
        match self {
            AclContravention::MisleadingConduct => "18",
            AclContravention::FalseRepresentations => "29",
            AclContravention::BaitAdvertising => "34",
            AclContravention::AcceptPaymentNoSupply => "35",
            AclContravention::UnconscionableGeneral => "50",
            AclContravention::UnconscionableTradeCommerce => "51",
            AclContravention::ConsumerGuaranteeBreach => "54-65",
            AclContravention::ProductSafetyBreach => "Part 3-3",
            AclContravention::UnfairContractTerm => "Part 2-3",
            AclContravention::UnsolicitedAgreementBreach => "Part 3-2 Div 2",
            AclContravention::CountryOfOriginBreach => "Part 5-3",
        }
    }

    /// Get base penalty units (individual)
    pub fn base_penalty_units(&self) -> u32 {
        match self {
            // Consumer protection provisions - 2,500 penalty units
            AclContravention::MisleadingConduct
            | AclContravention::FalseRepresentations
            | AclContravention::UnconscionableGeneral
            | AclContravention::UnconscionableTradeCommerce => 2500,

            // Bait advertising, no intent to supply - 1,000 penalty units
            AclContravention::BaitAdvertising | AclContravention::AcceptPaymentNoSupply => 1000,

            // Product safety - 2,500 penalty units
            AclContravention::ProductSafetyBreach => 2500,

            // Other
            _ => 500,
        }
    }

    /// Get current penalty unit value (2024-25: $313)
    pub fn penalty_unit_value() -> f64 {
        313.0
    }
}

/// Section 87B undertaking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourtUndertaking {
    /// Undertaking ID
    pub undertaking_id: String,
    /// Party giving undertaking
    pub party: String,
    /// Date accepted
    pub date_accepted: NaiveDate,
    /// Duration (if applicable)
    pub duration_months: Option<u32>,
    /// Key obligations
    pub obligations: Vec<String>,
    /// Compliance program required
    pub compliance_program_required: bool,
    /// Publication required
    pub publication_required: bool,
}

// =============================================================================
// Unsolicited Consumer Agreements
// =============================================================================

/// Unsolicited consumer agreement type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnsolicitedAgreementType {
    /// Door-to-door sales
    DoorToDoor,
    /// Telemarketing
    Telemarketing,
    /// In-person away from business premises
    InPersonAwayFromPremises,
}

/// Unsolicited consumer agreement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsolicitedConsumerAgreement {
    /// Agreement type
    pub agreement_type: UnsolicitedAgreementType,
    /// Dealer/salesperson name
    pub dealer_name: String,
    /// Supplier name
    pub supplier_name: String,
    /// Agreement date
    pub agreement_date: NaiveDate,
    /// Agreement value
    pub agreement_value: f64,
    /// Product/service description
    pub product_service: String,
    /// Consumer's name
    pub consumer_name: String,
    /// Consumer's address
    pub consumer_address: String,
    /// Cooling-off period end date
    pub cooling_off_end_date: NaiveDate,
    /// Compliant with ACL requirements
    pub compliant: bool,
}

impl UnsolicitedConsumerAgreement {
    /// Check if agreement is within cooling-off period
    pub fn is_within_cooling_off(&self, current_date: NaiveDate) -> bool {
        current_date <= self.cooling_off_end_date
    }

    /// Cooling-off period is 10 business days
    pub const COOLING_OFF_BUSINESS_DAYS: u32 = 10;
}

/// Permitted contact times for unsolicited agreements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermittedContactTime {
    /// Monday-Friday 9am-6pm
    WeekdayBusiness,
    /// Saturday 9am-5pm
    SaturdayBusiness,
    /// Not permitted (Sunday, public holidays, outside hours)
    NotPermitted,
}

// =============================================================================
// Country of Origin
// =============================================================================

/// Country of origin claim type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CountryOfOriginClaim {
    /// "Made in \[country\]"
    MadeIn,
    /// "Product of \[country\]"
    ProductOf,
    /// "Grown in \[country\]"
    GrownIn,
    /// "Packed in \[country\]" (lower threshold)
    PackedIn,
    /// Australian-made logo
    AustralianMadeLogo,
    /// Bar chart (food products)
    BarChart,
}

/// Country of origin claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryOfOriginClaimDetails {
    /// Claim type
    pub claim_type: CountryOfOriginClaim,
    /// Country claimed
    pub country: String,
    /// Product
    pub product: String,
    /// Is food product
    pub is_food: bool,
    /// Percentage of Australian ingredients (for bar chart)
    pub australian_ingredient_percentage: Option<f64>,
    /// Substantially transformed in Australia
    pub substantially_transformed: bool,
    /// 50% or more production costs in Australia
    pub fifty_percent_production_costs: bool,
    /// Last substantial transformation in Australia
    pub last_transformation_in_australia: bool,
}

impl CountryOfOriginClaimDetails {
    /// Check if "Made in Australia" safe harbour applies
    pub fn made_in_australia_safe_harbour(&self) -> bool {
        self.country == "Australia"
            && self.substantially_transformed
            && self.fifty_percent_production_costs
    }

    /// Check if "Product of Australia" safe harbour applies
    pub fn product_of_australia_safe_harbour(&self) -> bool {
        self.country == "Australia"
            && self.australian_ingredient_percentage == Some(100.0)
            && self.substantially_transformed
    }

    /// Check if "Grown in Australia" safe harbour applies
    pub fn grown_in_australia_safe_harbour(&self) -> bool {
        self.country == "Australia" && self.australian_ingredient_percentage == Some(100.0)
    }
}

// =============================================================================
// Lay-by Agreements
// =============================================================================

/// Lay-by agreement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayByAgreement {
    /// Agreement ID
    pub agreement_id: String,
    /// Goods description
    pub goods_description: String,
    /// Total price
    pub total_price: f64,
    /// Deposit paid
    pub deposit_paid: f64,
    /// Number of installments
    pub installments: u32,
    /// Installment amount
    pub installment_amount: f64,
    /// Agreement date
    pub agreement_date: NaiveDate,
    /// Expected completion date
    pub completion_date: NaiveDate,
    /// Termination fee (if any)
    pub termination_fee: Option<f64>,
    /// Status
    pub status: LayByStatus,
}

/// Lay-by agreement status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayByStatus {
    /// Active
    Active,
    /// Completed (goods delivered)
    Completed,
    /// Terminated by consumer
    TerminatedByConsumer,
    /// Terminated by supplier
    TerminatedBySupplier,
    /// Cancelled (goods unavailable)
    Cancelled,
}

impl LayByAgreement {
    /// Calculate amounts paid
    pub fn amounts_paid(&self, installments_paid: u32) -> f64 {
        self.deposit_paid + (installments_paid as f64 * self.installment_amount)
    }

    /// Calculate refund on consumer termination
    /// Supplier can retain reasonable fee, must refund remainder
    pub fn calculate_consumer_termination_refund(
        &self,
        installments_paid: u32,
        reasonable_fee: f64,
    ) -> f64 {
        let paid = self.amounts_paid(installments_paid);
        (paid - reasonable_fee).max(0.0)
    }

    /// On supplier termination, full refund required
    pub fn calculate_supplier_termination_refund(&self, installments_paid: u32) -> f64 {
        self.amounts_paid(installments_paid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penalty_multiplier() {
        assert_eq!(RecipientType::Individual.penalty_multiplier(), 1);
        assert_eq!(RecipientType::BodyCorporate.penalty_multiplier(), 5);
        assert_eq!(RecipientType::ListedCorporation.penalty_multiplier(), 10);
    }

    #[test]
    fn test_contravention_sections() {
        assert_eq!(AclContravention::MisleadingConduct.section(), "18");
        assert_eq!(AclContravention::FalseRepresentations.section(), "29");
        assert_eq!(
            AclContravention::UnconscionableTradeCommerce.section(),
            "51"
        );
    }

    #[test]
    fn test_made_in_australia_safe_harbour() {
        let claim = CountryOfOriginClaimDetails {
            claim_type: CountryOfOriginClaim::MadeIn,
            country: "Australia".to_string(),
            product: "Widget".to_string(),
            is_food: false,
            australian_ingredient_percentage: None,
            substantially_transformed: true,
            fifty_percent_production_costs: true,
            last_transformation_in_australia: true,
        };
        assert!(claim.made_in_australia_safe_harbour());
    }

    #[test]
    fn test_product_of_australia_safe_harbour() {
        let claim = CountryOfOriginClaimDetails {
            claim_type: CountryOfOriginClaim::ProductOf,
            country: "Australia".to_string(),
            product: "Honey".to_string(),
            is_food: true,
            australian_ingredient_percentage: Some(100.0),
            substantially_transformed: true,
            fifty_percent_production_costs: true,
            last_transformation_in_australia: true,
        };
        assert!(claim.product_of_australia_safe_harbour());
    }

    #[test]
    fn test_layby_refund_calculation() {
        let layby = LayByAgreement {
            agreement_id: "LB001".to_string(),
            goods_description: "Television".to_string(),
            total_price: 1000.0,
            deposit_paid: 100.0,
            installments: 9,
            installment_amount: 100.0,
            agreement_date: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
            completion_date: NaiveDate::from_ymd_opt(2026, 10, 1).expect("valid date"),
            termination_fee: Some(50.0),
            status: LayByStatus::Active,
        };

        // 5 installments paid = $100 deposit + $500 = $600
        assert_eq!(layby.amounts_paid(5), 600.0);

        // Consumer termination refund with $50 fee = $550
        assert_eq!(layby.calculate_consumer_termination_refund(5, 50.0), 550.0);

        // Supplier termination = full refund $600
        assert_eq!(layby.calculate_supplier_termination_refund(5), 600.0);
    }

    #[test]
    fn test_cooling_off_period() {
        let agreement = UnsolicitedConsumerAgreement {
            agreement_type: UnsolicitedAgreementType::DoorToDoor,
            dealer_name: "Sales Rep".to_string(),
            supplier_name: "Acme Corp".to_string(),
            agreement_date: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
            agreement_value: 500.0,
            product_service: "Vacuum cleaner".to_string(),
            consumer_name: "John Doe".to_string(),
            consumer_address: "123 Main St".to_string(),
            cooling_off_end_date: NaiveDate::from_ymd_opt(2026, 1, 15).expect("valid date"),
            compliant: true,
        };

        let check_date = NaiveDate::from_ymd_opt(2026, 1, 10).expect("valid date");
        assert!(agreement.is_within_cooling_off(check_date));

        let after_date = NaiveDate::from_ymd_opt(2026, 1, 20).expect("valid date");
        assert!(!agreement.is_within_cooling_off(after_date));
    }
}
