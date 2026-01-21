//! Child Support Assessment and Collection
//!
//! Implementation of Child Support (Assessment) Act 1989 (Cth) including:
//! - Formula assessment (Part 5)
//! - Care percentage calculations
//! - Income tests and adjustments
//! - Departure from assessment
//!
//! ## Key Legislation
//!
//! - Child Support (Assessment) Act 1989 (Cth)
//! - Child Support (Registration and Collection) Act 1988 (Cth)
//!
//! ## Key Concepts
//!
//! The child support formula considers:
//! - Each parent's adjusted taxable income
//! - Care percentage for each child
//! - Number and ages of children
//! - Self-support amount (currently ~$27,508)
//! - Cost of children tables

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

// =============================================================================
// Child Support Case
// =============================================================================

/// Child support case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildSupportCase {
    /// Case registration date
    pub registration_date: NaiveDate,
    /// Assessment period start
    pub assessment_period_start: NaiveDate,
    /// Assessment period end
    pub assessment_period_end: NaiveDate,
    /// Parent 1 (payer or payee)
    pub parent1: Parent,
    /// Parent 2 (payer or payee)
    pub parent2: Parent,
    /// Children in the case
    pub children: Vec<Child>,
    /// Assessment type
    pub assessment_type: AssessmentType,
}

/// Parent in child support case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parent {
    /// Parent identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Adjusted taxable income
    pub adjusted_taxable_income: f64,
    /// Relevant dependent children (non-case children)
    pub relevant_dependent_children: u32,
    /// Multi-case children
    pub multi_case_children: u32,
    /// Care percentage for case children
    pub care_percentages: Vec<CarePercentage>,
    /// Is the payer
    pub is_payer: Option<bool>,
}

/// Care percentage for a child
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarePercentage {
    /// Child ID
    pub child_id: String,
    /// Care percentage (0-100)
    pub percentage: f64,
    /// Care level
    pub care_level: CareLevel,
    /// Nights per year
    pub nights_per_year: Option<u32>,
}

/// Care level based on percentage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CareLevel {
    /// Below regular care (0-13%)
    BelowRegular,
    /// Regular care (14-34%)
    Regular,
    /// Shared care (35-65%)
    Shared,
    /// Primary care (66-86%)
    Primary,
    /// Above primary care (87-100%)
    AbovePrimary,
}

impl CareLevel {
    /// Determine care level from percentage
    pub fn from_percentage(percentage: f64) -> Self {
        if percentage < 14.0 {
            Self::BelowRegular
        } else if percentage < 35.0 {
            Self::Regular
        } else if percentage <= 65.0 {
            Self::Shared
        } else if percentage <= 86.0 {
            Self::Primary
        } else {
            Self::AbovePrimary
        }
    }

    /// Get cost percentage (percentage of child costs attributed)
    pub fn cost_percentage(&self) -> f64 {
        match self {
            Self::BelowRegular => 0.0,
            Self::Regular => 0.24,
            Self::Shared => 0.25, // Variable 25-50%
            Self::Primary => 0.76,
            Self::AbovePrimary => 1.0,
        }
    }
}

/// Child in case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Child {
    /// Child identifier
    pub id: String,
    /// Child's name
    pub name: String,
    /// Date of birth
    pub date_of_birth: NaiveDate,
    /// Child age group
    pub age_group: ChildAgeGroup,
}

impl Child {
    /// Calculate age
    pub fn age(&self, as_of: NaiveDate) -> u32 {
        let years = as_of.year() - self.date_of_birth.year();
        let had_birthday =
            (as_of.month(), as_of.day()) >= (self.date_of_birth.month(), self.date_of_birth.day());
        if had_birthday {
            years as u32
        } else {
            (years - 1).max(0) as u32
        }
    }

    /// Get age group
    pub fn get_age_group(&self, as_of: NaiveDate) -> ChildAgeGroup {
        let age = self.age(as_of);
        ChildAgeGroup::from_age(age)
    }
}

/// Child age group for cost tables
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChildAgeGroup {
    /// 0-12 years
    ZeroToTwelve,
    /// 13+ years
    ThirteenPlus,
}

impl ChildAgeGroup {
    /// Determine age group from age
    pub fn from_age(age: u32) -> Self {
        if age < 13 {
            Self::ZeroToTwelve
        } else {
            Self::ThirteenPlus
        }
    }
}

/// Assessment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssessmentType {
    /// Formula assessment
    Formula,
    /// Agreement (binding)
    BindingAgreement,
    /// Agreement (limited)
    LimitedAgreement,
    /// Court order
    CourtOrder,
    /// Departure determination
    DepartureDetermination,
}

// =============================================================================
// Formula Assessment
// =============================================================================

/// Child support formula assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaAssessment {
    /// Assessment date
    pub assessment_date: NaiveDate,
    /// Parent 1 income details
    pub parent1_income: ParentIncomeDetails,
    /// Parent 2 income details
    pub parent2_income: ParentIncomeDetails,
    /// Combined child support income
    pub combined_child_support_income: f64,
    /// Cost of children
    pub cost_of_children: f64,
    /// Each parent's share of costs
    pub parent1_cost_share: f64,
    /// Each parent's share of costs
    pub parent2_cost_share: f64,
    /// Annual child support payable
    pub annual_payable: f64,
    /// Monthly amount
    pub monthly_amount: f64,
    /// Payer parent
    pub payer: PayerDetails,
}

/// Parent income details for formula
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentIncomeDetails {
    /// Parent ID
    pub parent_id: String,
    /// Adjusted taxable income
    pub adjusted_taxable_income: f64,
    /// Self-support amount deducted
    pub self_support_amount: f64,
    /// Relevant dependent child amount
    pub relevant_dependent_child_amount: f64,
    /// Multi-case allowance
    pub multi_case_allowance: f64,
    /// Child support income
    pub child_support_income: f64,
    /// Income percentage
    pub income_percentage: f64,
}

/// Payer details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PayerDetails {
    /// Payer parent ID
    pub parent_id: String,
    /// Annual amount payable
    pub annual_amount: f64,
    /// Is payee due to higher care
    pub is_payee_higher_care: bool,
}

// =============================================================================
// Cost of Children Tables
// =============================================================================

/// Cost of children table (simplified 2024 rates)
pub struct CostOfChildrenTable;

impl CostOfChildrenTable {
    /// Self-support amount (2024)
    pub const SELF_SUPPORT_AMOUNT: f64 = 27_508.0;

    /// Maximum child support income (2024)
    pub const MAX_CHILD_SUPPORT_INCOME: f64 = 194_522.0;

    /// Calculate cost of children
    pub fn calculate_cost(combined_income: f64, children_0_12: u32, children_13_plus: u32) -> f64 {
        let total_children = children_0_12 + children_13_plus;
        if total_children == 0 {
            return 0.0;
        }

        // Cost percentage based on income and number of children
        // Simplified model - actual tables are more complex
        let cost_percentage = Self::get_cost_percentage(combined_income, total_children);

        // Older children cost more (approximately 25% more)
        let child_units = children_0_12 as f64 + (children_13_plus as f64 * 1.25);
        let average_units = child_units / total_children as f64;

        combined_income * cost_percentage * average_units
            / (if total_children > 1 {
                total_children as f64 * 0.9
            } else {
                1.0
            })
    }

    /// Get cost percentage based on income bracket
    fn get_cost_percentage(combined_income: f64, num_children: u32) -> f64 {
        // Simplified percentage based on combined income
        // Real table has multiple income brackets
        let base_percentage = if combined_income < 40_000.0 {
            0.17
        } else if combined_income < 80_000.0 {
            0.22
        } else if combined_income < 120_000.0 {
            0.27
        } else if combined_income < 160_000.0 {
            0.30
        } else {
            0.32
        };

        // Adjust for number of children (diminishing returns)
        match num_children {
            1 => base_percentage,
            2 => base_percentage * 1.5,
            3 => base_percentage * 1.9,
            _ => base_percentage * 2.2,
        }
    }
}

// =============================================================================
// Departure from Assessment
// =============================================================================

/// Departure ground under Part 6A/6B
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DepartureGround {
    /// High costs of child (education, medical)
    HighCostsOfChild,
    /// High costs of contact
    HighCostsOfContact,
    /// Special needs of child
    SpecialNeedsOfChild,
    /// Earning capacity
    EarningCapacity,
    /// Property/financial resources
    PropertyFinancialResources,
    /// Necessary commitments
    NecessaryCommitments,
    /// High child support paid by paying parent
    HighChildSupportPaid,
    /// Prior support of child
    PriorSupportOfChild,
    /// Special circumstances (catchall)
    SpecialCircumstances,
}

impl DepartureGround {
    /// Get section reference
    pub fn section(&self) -> &'static str {
        match self {
            Self::HighCostsOfChild => "s.117(2)(b)(i)",
            Self::HighCostsOfContact => "s.117(2)(b)(ii)",
            Self::SpecialNeedsOfChild => "s.117(2)(b)(iii)",
            Self::EarningCapacity => "s.117(2)(c)(i)",
            Self::PropertyFinancialResources => "s.117(2)(c)(ii)",
            Self::NecessaryCommitments => "s.117(2)(c)(iii)",
            Self::HighChildSupportPaid => "s.117(2)(c)(iv)",
            Self::PriorSupportOfChild => "s.117(2)(c)(v)",
            Self::SpecialCircumstances => "s.117(2)(d)",
        }
    }
}

/// Departure application
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepartureApplication {
    /// Applicant parent ID
    pub applicant: String,
    /// Ground(s) relied on
    pub grounds: Vec<DepartureGround>,
    /// Current assessment amount
    pub current_assessment: f64,
    /// Proposed amount
    pub proposed_amount: f64,
    /// Reason for departure
    pub reasons: Vec<String>,
    /// Supporting evidence
    pub evidence: Vec<String>,
}

/// Departure decision
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepartureDecision {
    /// Decision outcome
    pub outcome: DepartureOutcome,
    /// Ground(s) accepted
    pub grounds_accepted: Vec<DepartureGround>,
    /// Revised amount (if different)
    pub revised_amount: Option<f64>,
    /// Reasons for decision
    pub reasons: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Departure outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DepartureOutcome {
    /// Departure granted
    Granted,
    /// Departure refused
    Refused,
    /// Partially granted
    PartiallyGranted,
}

// =============================================================================
// Agreements
// =============================================================================

/// Child support agreement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildSupportAgreement {
    /// Agreement type
    pub agreement_type: AgreementType,
    /// Date entered
    pub date_entered: NaiveDate,
    /// Parties
    pub parties: Vec<String>,
    /// Children covered
    pub children: Vec<String>,
    /// Agreed amount (if fixed)
    pub agreed_amount: Option<f64>,
    /// Payment frequency
    pub payment_frequency: PaymentFrequency,
    /// Agreement terms
    pub terms: Vec<AgreementTerm>,
    /// Legal advice requirements met
    pub legal_advice_requirements: LegalAdviceRequirements,
}

/// Agreement type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AgreementType {
    /// Binding agreement (requires legal advice)
    Binding,
    /// Limited agreement (can be terminated by either party)
    Limited,
}

impl AgreementType {
    /// Termination mechanism
    pub fn termination(&self) -> &'static str {
        match self {
            Self::Binding => "By agreement of both parties or court order",
            Self::Limited => "Written notice by either party after 3 years",
        }
    }
}

/// Payment frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PaymentFrequency {
    /// Weekly
    Weekly,
    /// Fortnightly
    Fortnightly,
    /// Monthly
    Monthly,
    /// Annually
    Annually,
    /// Lump sum
    LumpSum,
}

/// Agreement term
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgreementTerm {
    /// Term type
    pub term_type: TermType,
    /// Description
    pub description: String,
    /// Value (if applicable)
    pub value: Option<f64>,
}

/// Term type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TermType {
    /// Fixed periodic payment
    FixedPeriodic,
    /// Formula-based with adjustment
    FormulaWithAdjustment,
    /// Non-periodic (education, medical)
    NonPeriodic,
    /// Lump sum
    LumpSum,
    /// Property transfer
    PropertyTransfer,
    /// Review mechanism
    ReviewMechanism,
}

/// Legal advice requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAdviceRequirements {
    /// Parent 1 received advice
    pub parent1_received_advice: bool,
    /// Parent 2 received advice
    pub parent2_received_advice: bool,
    /// Certificates attached
    pub certificates_attached: bool,
}

// =============================================================================
// Collection
// =============================================================================

/// Collection method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollectionMethod {
    /// Private collection (between parents)
    Private,
    /// Child Support Collect (agency collection)
    ChildSupportCollect,
}

/// Collection status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionStatus {
    /// Collection method
    pub method: CollectionMethod,
    /// Total amount due
    pub amount_due: f64,
    /// Amount paid
    pub amount_paid: f64,
    /// Arrears (if any)
    pub arrears: f64,
    /// Employer withholding in place
    pub employer_withholding: bool,
    /// Departure prohibition order in place
    pub departure_prohibition: bool,
}

/// Enforcement action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnforcementAction {
    /// Employer withholding
    EmployerWithholding,
    /// Tax refund interception
    TaxRefundInterception,
    /// Departure prohibition order
    DepartureProhibitionOrder,
    /// Litigation
    Litigation,
    /// Serious defaulter (passport cancellation)
    SeriousDefaulter,
}

// =============================================================================
// Objections and Appeals
// =============================================================================

/// Objection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Objection {
    /// Objection date
    pub date: NaiveDate,
    /// Decision objected to
    pub decision_type: ObjectionDecisionType,
    /// Grounds
    pub grounds: Vec<String>,
    /// Outcome
    pub outcome: Option<ObjectionOutcome>,
}

/// Type of decision being objected to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectionDecisionType {
    /// Assessment
    Assessment,
    /// Care percentage
    CarePercentage,
    /// Income estimate
    IncomeEstimate,
    /// Departure decision
    Departure,
    /// Other decision
    Other,
}

/// Objection outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectionOutcome {
    /// Allowed (decision changed)
    Allowed,
    /// Disallowed (decision confirmed)
    Disallowed,
    /// Partially allowed
    PartiallyAllowed,
}

// =============================================================================
// Assessment Functions
// =============================================================================

/// Calculate formula assessment
pub fn calculate_formula_assessment(
    parent1: &Parent,
    parent2: &Parent,
    children: &[Child],
    assessment_date: NaiveDate,
) -> FormulaAssessment {
    let self_support = CostOfChildrenTable::SELF_SUPPORT_AMOUNT;

    // Calculate child support income for each parent
    let p1_csi = (parent1.adjusted_taxable_income - self_support).max(0.0);
    let p2_csi = (parent2.adjusted_taxable_income - self_support).max(0.0);

    // Combined child support income (capped)
    let combined_csi = (p1_csi + p2_csi).min(CostOfChildrenTable::MAX_CHILD_SUPPORT_INCOME);

    // Income percentages
    let total_csi = p1_csi + p2_csi;
    let p1_income_pct = if total_csi > 0.0 {
        p1_csi / total_csi
    } else {
        0.5
    };
    let p2_income_pct = 1.0 - p1_income_pct;

    // Count children by age group
    let children_0_12 = children
        .iter()
        .filter(|c| {
            matches!(
                c.get_age_group(assessment_date),
                ChildAgeGroup::ZeroToTwelve
            )
        })
        .count() as u32;
    let children_13_plus = children.len() as u32 - children_0_12;

    // Cost of children
    let cost_of_children =
        CostOfChildrenTable::calculate_cost(combined_csi, children_0_12, children_13_plus);

    // Get average care percentages
    let p1_care = parent1
        .care_percentages
        .iter()
        .map(|c| c.percentage)
        .sum::<f64>()
        / parent1.care_percentages.len().max(1) as f64;
    let p2_care = parent2
        .care_percentages
        .iter()
        .map(|c| c.percentage)
        .sum::<f64>()
        / parent2.care_percentages.len().max(1) as f64;

    // Calculate cost percentages based on care
    let p1_cost_pct = CareLevel::from_percentage(p1_care).cost_percentage();
    let p2_cost_pct = CareLevel::from_percentage(p2_care).cost_percentage();

    // Each parent's share of costs
    let p1_cost_share = cost_of_children * p1_income_pct;
    let p2_cost_share = cost_of_children * p2_income_pct;

    // Calculate child support payable
    // Parent pays: their income share of costs minus their care share
    let p1_cost_liability = p1_cost_share - (cost_of_children * p1_cost_pct);
    let p2_cost_liability = p2_cost_share - (cost_of_children * p2_cost_pct);

    // Determine payer (positive liability = payer)
    let (payer_id, annual_payable) = if p1_cost_liability > p2_cost_liability {
        (
            parent1.id.clone(),
            (p1_cost_liability - p2_cost_liability).max(0.0),
        )
    } else {
        (
            parent2.id.clone(),
            (p2_cost_liability - p1_cost_liability).max(0.0),
        )
    };

    FormulaAssessment {
        assessment_date,
        parent1_income: ParentIncomeDetails {
            parent_id: parent1.id.clone(),
            adjusted_taxable_income: parent1.adjusted_taxable_income,
            self_support_amount: self_support,
            relevant_dependent_child_amount: 0.0, // Simplified
            multi_case_allowance: 0.0,
            child_support_income: p1_csi,
            income_percentage: p1_income_pct,
        },
        parent2_income: ParentIncomeDetails {
            parent_id: parent2.id.clone(),
            adjusted_taxable_income: parent2.adjusted_taxable_income,
            self_support_amount: self_support,
            relevant_dependent_child_amount: 0.0,
            multi_case_allowance: 0.0,
            child_support_income: p2_csi,
            income_percentage: p2_income_pct,
        },
        combined_child_support_income: combined_csi,
        cost_of_children,
        parent1_cost_share: p1_cost_share,
        parent2_cost_share: p2_cost_share,
        annual_payable,
        monthly_amount: annual_payable / 12.0,
        payer: PayerDetails {
            parent_id: payer_id,
            annual_amount: annual_payable,
            is_payee_higher_care: p1_care > 50.0 && p1_cost_liability > 0.0,
        },
    }
}

/// Validate child support agreement
pub fn validate_agreement(agreement: &ChildSupportAgreement) -> AgreementValidation {
    let mut issues = Vec::new();
    let mut legal_references = vec!["Child Support (Assessment) Act 1989 Part 6".to_string()];

    // Check legal advice requirements for binding agreement
    if matches!(agreement.agreement_type, AgreementType::Binding) {
        if !agreement.legal_advice_requirements.parent1_received_advice
            || !agreement.legal_advice_requirements.parent2_received_advice
        {
            issues.push(
                "Binding agreement requires both parties to receive legal advice".to_string(),
            );
            legal_references.push("s.80C".to_string());
        }
        if !agreement.legal_advice_requirements.certificates_attached {
            issues.push("Legal advice certificates must be attached".to_string());
        }
    }

    // Check minimum amount (for limited agreements)
    if matches!(agreement.agreement_type, AgreementType::Limited)
        && let Some(amount) = agreement.agreed_amount
    {
        // Limited agreement cannot be less than formula assessment
        // (This would require formula calculation to verify)
        if amount <= 0.0 {
            issues.push("Amount must be positive".to_string());
        }
    }

    AgreementValidation {
        valid: issues.is_empty(),
        agreement_type: agreement.agreement_type,
        issues,
        legal_references,
    }
}

/// Agreement validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgreementValidation {
    /// Is agreement valid
    pub valid: bool,
    /// Agreement type
    pub agreement_type: AgreementType,
    /// Issues found
    pub issues: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

/// Assess departure application
pub fn assess_departure(
    application: &DepartureApplication,
    current_assessment: &FormulaAssessment,
) -> DepartureAssessment {
    let mut accepted_grounds = Vec::new();
    let mut rejected_grounds = Vec::new();
    let mut recommendations = Vec::new();
    let legal_references = vec![
        "Child Support (Assessment) Act 1989 s.117".to_string(),
        "Child Support (Assessment) Act 1989 s.98S (SCO)".to_string(),
    ];

    // Assess each ground
    for ground in &application.grounds {
        // Simplified assessment - in practice requires evidence
        let accepted = match ground {
            DepartureGround::HighCostsOfChild => {
                recommendations.push("Verify high costs with receipts/invoices".to_string());
                true // Assume accepted if claimed
            }
            DepartureGround::HighCostsOfContact => {
                if current_assessment.payer.annual_amount > 5000.0 {
                    recommendations.push("Verify travel/accommodation costs".to_string());
                    true
                } else {
                    false
                }
            }
            DepartureGround::EarningCapacity => {
                recommendations.push("Consider earning capacity vs actual income".to_string());
                true
            }
            _ => {
                recommendations.push(format!("Assess {} ground with evidence", ground.section()));
                true
            }
        };

        if accepted {
            accepted_grounds.push(*ground);
        } else {
            rejected_grounds.push(*ground);
        }
    }

    // Determine outcome
    let outcome = if accepted_grounds.is_empty() {
        DepartureOutcome::Refused
    } else if rejected_grounds.is_empty() {
        DepartureOutcome::Granted
    } else {
        DepartureOutcome::PartiallyGranted
    };

    // Calculate suggested amount if departure granted
    let suggested_amount = if outcome != DepartureOutcome::Refused {
        let adjustment_factor = 0.8 + (rejected_grounds.len() as f64 * 0.1);
        Some(
            (application.proposed_amount + (current_assessment.annual_payable * adjustment_factor))
                / 2.0,
        )
    } else {
        None
    };

    DepartureAssessment {
        outcome,
        grounds_accepted: accepted_grounds,
        grounds_rejected: rejected_grounds,
        suggested_amount,
        recommendations,
        legal_references,
    }
}

/// Departure assessment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepartureAssessment {
    /// Outcome
    pub outcome: DepartureOutcome,
    /// Grounds accepted
    pub grounds_accepted: Vec<DepartureGround>,
    /// Grounds rejected
    pub grounds_rejected: Vec<DepartureGround>,
    /// Suggested amount
    pub suggested_amount: Option<f64>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Legal references
    pub legal_references: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_care_level_from_percentage() {
        assert!(matches!(
            CareLevel::from_percentage(10.0),
            CareLevel::BelowRegular
        ));
        assert!(matches!(
            CareLevel::from_percentage(25.0),
            CareLevel::Regular
        ));
        assert!(matches!(
            CareLevel::from_percentage(50.0),
            CareLevel::Shared
        ));
        assert!(matches!(
            CareLevel::from_percentage(75.0),
            CareLevel::Primary
        ));
        assert!(matches!(
            CareLevel::from_percentage(90.0),
            CareLevel::AbovePrimary
        ));
    }

    #[test]
    fn test_child_age_calculation() {
        let child = Child {
            id: "C1".to_string(),
            name: "Test Child".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2015, 6, 15).expect("valid date"),
            age_group: ChildAgeGroup::ZeroToTwelve,
        };

        let as_of = NaiveDate::from_ymd_opt(2026, 1, 20).expect("valid date");
        assert_eq!(child.age(as_of), 10);
    }

    #[test]
    fn test_formula_assessment() {
        let parent1 = Parent {
            id: "P1".to_string(),
            name: "Parent 1".to_string(),
            adjusted_taxable_income: 80_000.0,
            relevant_dependent_children: 0,
            multi_case_children: 0,
            care_percentages: vec![CarePercentage {
                child_id: "C1".to_string(),
                percentage: 20.0,
                care_level: CareLevel::Regular,
                nights_per_year: Some(73),
            }],
            is_payer: Some(true),
        };

        let parent2 = Parent {
            id: "P2".to_string(),
            name: "Parent 2".to_string(),
            adjusted_taxable_income: 40_000.0,
            relevant_dependent_children: 0,
            multi_case_children: 0,
            care_percentages: vec![CarePercentage {
                child_id: "C1".to_string(),
                percentage: 80.0,
                care_level: CareLevel::Primary,
                nights_per_year: Some(292),
            }],
            is_payer: Some(false),
        };

        let children = vec![Child {
            id: "C1".to_string(),
            name: "Child 1".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2018, 3, 1).expect("valid date"),
            age_group: ChildAgeGroup::ZeroToTwelve,
        }];

        let assessment_date = NaiveDate::from_ymd_opt(2026, 1, 20).expect("valid date");
        let result = calculate_formula_assessment(&parent1, &parent2, &children, assessment_date);

        // Parent 1 (higher income, lower care) should be payer
        assert_eq!(result.payer.parent_id, "P1");
        assert!(result.annual_payable > 0.0);
        assert!(result.monthly_amount > 0.0);
    }

    #[test]
    fn test_cost_of_children() {
        let cost = CostOfChildrenTable::calculate_cost(60_000.0, 2, 0);
        assert!(cost > 0.0);
        assert!(cost < 60_000.0); // Cost should be less than income
    }

    #[test]
    fn test_validate_binding_agreement_missing_advice() {
        let agreement = ChildSupportAgreement {
            agreement_type: AgreementType::Binding,
            date_entered: NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date"),
            parties: vec!["P1".to_string(), "P2".to_string()],
            children: vec!["C1".to_string()],
            agreed_amount: Some(10_000.0),
            payment_frequency: PaymentFrequency::Monthly,
            terms: vec![],
            legal_advice_requirements: LegalAdviceRequirements {
                parent1_received_advice: true,
                parent2_received_advice: false, // Missing
                certificates_attached: false,
            },
        };

        let result = validate_agreement(&agreement);
        assert!(!result.valid);
        assert!(result.issues.iter().any(|i| i.contains("legal advice")));
    }

    #[test]
    fn test_departure_ground_sections() {
        assert_eq!(
            DepartureGround::HighCostsOfChild.section(),
            "s.117(2)(b)(i)"
        );
        assert_eq!(DepartureGround::EarningCapacity.section(), "s.117(2)(c)(i)");
    }
}
