//! UK Family Law - Financial Remedies
//!
//! Implementation of financial remedies on divorce under the Matrimonial Causes Act 1973.
//!
//! # Key Legislation
//!
//! ## Matrimonial Causes Act 1973
//!
//! ### Section 25 Factors
//!
//! The court must have regard to all circumstances of the case, first consideration
//! being given to the welfare of any minor child. The court must then consider:
//!
//! - **(a)** Income, earning capacity, property and financial resources (present and foreseeable)
//! - **(b)** Financial needs, obligations and responsibilities (present and foreseeable)
//! - **(c)** Standard of living enjoyed before breakdown
//! - **(d)** Age and duration of marriage
//! - **(e)** Physical or mental disability
//! - **(f)** Contributions to welfare of family (including homemaking)
//! - **(g)** Conduct (if inequitable to disregard)
//! - **(h)** Value of benefits lost (e.g., pension)
//!
//! ### Clean Break Principle (s.25A)
//!
//! Court must consider whether to:
//! - Terminate financial obligations as soon as just and reasonable
//! - Dismiss application for periodical payments after lump sum/property adjustment
//!
//! ### Orders Available
//!
//! - **s.22**: Maintenance pending suit
//! - **s.23**: Periodical payments and lump sum
//! - **s.24**: Property adjustment orders
//! - **s.24B**: Pension sharing orders
//! - **s.25B**: Pension attachment orders
//!
//! # Key Case Law
//!
//! - **White v White \[2001\]**: Yardstick of equality; discrimination against homemakers
//! - **Miller v Miller; McFarlane v McFarlane \[2006\]**: Three strands: needs, compensation, sharing
//! - **Charman v Charman \[2007\]**: Matrimonial vs non-matrimonial property
//! - **Radmacher v Granatino \[2010\]**: Prenuptial agreements given weight if fair
//! - **Waggott v Waggott \[2018\]**: No sharing of future earning capacity

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::error::{FamilyLawError, Result};
use super::types::{AssetOwnership, AssetType};

// ============================================================================
// Section 25 Factors
// ============================================================================

/// Section 25 factor from MCA 1973
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section25Factor {
    /// (a) Income, earning capacity, property and resources
    IncomeAndResources,
    /// (b) Financial needs, obligations and responsibilities
    NeedsAndObligations,
    /// (c) Standard of living before breakdown
    StandardOfLiving,
    /// (d) Age and duration of marriage
    AgeAndDuration,
    /// (e) Physical or mental disability
    Disability,
    /// (f) Contributions to welfare of family
    Contributions,
    /// (g) Conduct (if inequitable to disregard)
    Conduct,
    /// (h) Value of benefits lost
    BenefitsLost,
}

impl Section25Factor {
    /// Get statutory reference
    pub fn statutory_reference(&self) -> &'static str {
        match self {
            Self::IncomeAndResources => "MCA 1973 s.25(2)(a)",
            Self::NeedsAndObligations => "MCA 1973 s.25(2)(b)",
            Self::StandardOfLiving => "MCA 1973 s.25(2)(c)",
            Self::AgeAndDuration => "MCA 1973 s.25(2)(d)",
            Self::Disability => "MCA 1973 s.25(2)(e)",
            Self::Contributions => "MCA 1973 s.25(2)(f)",
            Self::Conduct => "MCA 1973 s.25(2)(g)",
            Self::BenefitsLost => "MCA 1973 s.25(2)(h)",
        }
    }

    /// Get all factors
    pub fn all() -> Vec<Self> {
        vec![
            Self::IncomeAndResources,
            Self::NeedsAndObligations,
            Self::StandardOfLiving,
            Self::AgeAndDuration,
            Self::Disability,
            Self::Contributions,
            Self::Conduct,
            Self::BenefitsLost,
        ]
    }
}

/// Assessment of a section 25 factor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section25Assessment {
    /// Factor
    pub factor: Section25Factor,
    /// Assessment for party 1
    pub party1_assessment: String,
    /// Assessment for party 2
    pub party2_assessment: String,
    /// Impact on outcome
    pub impact: FactorImpact,
    /// Notes
    pub notes: String,
}

/// Impact of factor on outcome
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorImpact {
    /// Favours party 1
    FavoursParty1,
    /// Favours party 2
    FavoursParty2,
    /// Neutral
    Neutral,
    /// Not applicable
    NotApplicable,
}

// ============================================================================
// Financial Remedy Analysis
// ============================================================================

/// Three strands analysis (Miller v Miller; McFarlane v McFarlane \[2006\])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreeStrandsAnalysis {
    /// Needs analysis
    pub needs: NeedsAnalysis,
    /// Compensation analysis
    pub compensation: CompensationAnalysis,
    /// Sharing analysis
    pub sharing: SharingAnalysis,
    /// Overall recommendation
    pub recommendation: String,
}

/// Needs-based analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeedsAnalysis {
    /// Party 1 needs
    pub party1_needs: f64,
    /// Party 2 needs
    pub party2_needs: f64,
    /// Housing needs identified
    pub housing_needs: Vec<HousingNeedsAssessment>,
    /// Income needs identified
    pub income_needs: Vec<IncomeNeedsAssessment>,
    /// Can needs be met from available resources?
    pub needs_can_be_met: bool,
    /// Shortfall (if any)
    pub shortfall: f64,
    /// Analysis
    pub analysis: String,
}

/// Housing needs assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HousingNeedsAssessment {
    /// Party
    pub party: String,
    /// Minimum housing requirement
    pub minimum_requirement: String,
    /// Cost estimate
    pub cost_estimate: f64,
    /// Children's needs factor
    pub children_needs_factor: bool,
    /// Analysis
    pub analysis: String,
}

/// Income needs assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomeNeedsAssessment {
    /// Party
    pub party: String,
    /// Current income
    pub current_income: f64,
    /// Required income
    pub required_income: f64,
    /// Can increase earning capacity?
    pub can_increase_earnings: bool,
    /// Timeframe to increase
    pub timeframe: Option<String>,
    /// Analysis
    pub analysis: String,
}

/// Compensation analysis (relationship-generated disadvantage)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompensationAnalysis {
    /// Is compensation claim applicable?
    pub applicable: bool,
    /// Party claiming compensation
    pub claimant: Option<String>,
    /// Nature of disadvantage
    pub disadvantage: Option<String>,
    /// Quantification (if possible)
    pub quantification: Option<f64>,
    /// Analysis
    pub analysis: String,
}

impl CompensationAnalysis {
    /// Analyze compensation claim
    pub fn analyze(
        party1: &str,
        party2: &str,
        party1_career_sacrifice: bool,
        party1_earning_capacity_lost: Option<f64>,
        party2_career_sacrifice: bool,
        party2_earning_capacity_lost: Option<f64>,
    ) -> Self {
        let (applicable, claimant, disadvantage, quantification) =
            if party1_career_sacrifice && party1_earning_capacity_lost.is_some() {
                (
                    true,
                    Some(party1.to_string()),
                    Some("Career sacrifice to support family/spouse's career".to_string()),
                    party1_earning_capacity_lost,
                )
            } else if party2_career_sacrifice && party2_earning_capacity_lost.is_some() {
                (
                    true,
                    Some(party2.to_string()),
                    Some("Career sacrifice to support family/spouse's career".to_string()),
                    party2_earning_capacity_lost,
                )
            } else {
                (false, None, None, None)
            };

        let analysis = if applicable {
            format!(
                "Compensation claim APPLICABLE per McFarlane v McFarlane [2006]. \
                 {} suffered relationship-generated disadvantage: {:?}. \
                 Estimated loss: £{:.0}. Court should compensate for lost earning capacity.",
                claimant.as_deref().unwrap_or("Party"),
                disadvantage,
                quantification.unwrap_or(0.0)
            )
        } else {
            "Compensation NOT applicable - no relationship-generated disadvantage identified. \
             Both parties maintained earning capacity throughout marriage."
                .to_string()
        };

        Self {
            applicable,
            claimant,
            disadvantage,
            quantification,
            analysis,
        }
    }
}

/// Sharing analysis (division of matrimonial assets)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharingAnalysis {
    /// Total matrimonial assets
    pub total_matrimonial: f64,
    /// Total non-matrimonial assets
    pub total_non_matrimonial: f64,
    /// Party 1 non-matrimonial
    pub party1_non_matrimonial: f64,
    /// Party 2 non-matrimonial
    pub party2_non_matrimonial: f64,
    /// Starting point (usually 50%)
    pub starting_point_percentage: f64,
    /// Departure from equality justified?
    pub departure_justified: bool,
    /// Reason for departure
    pub departure_reason: Option<String>,
    /// Final sharing percentage
    pub final_percentage: f64,
    /// Analysis
    pub analysis: String,
}

impl SharingAnalysis {
    /// Analyze sharing entitlement
    pub fn analyze(
        matrimonial_assets: f64,
        party1_non_matrimonial: f64,
        party2_non_matrimonial: f64,
        marriage_duration_years: u32,
        special_contribution_claimed: bool,
    ) -> Self {
        let total_non_matrimonial = party1_non_matrimonial + party2_non_matrimonial;
        let starting_point_percentage = 50.0;

        // Determine if departure justified
        let (departure_justified, departure_reason, final_percentage) =
            if marriage_duration_years < 5 {
                // Short marriage - may justify departure for non-matrimonial
                (
                    true,
                    Some("Short marriage - non-matrimonial property may not be shared".to_string()),
                    50.0, // Still 50% of matrimonial, but non-matrimonial excluded
                )
            } else if special_contribution_claimed {
                // Special contribution rarely succeeds (Charman v Charman)
                (
                    false,
                    Some(
                        "Special contribution claimed but rarely justifies departure \
                     (Charman v Charman [2007])"
                            .to_string(),
                    ),
                    50.0,
                )
            } else {
                (false, None, 50.0)
            };

        let analysis = format!(
            "Sharing analysis per White v White [2001]: \
             Matrimonial assets: £{:.0}. Non-matrimonial: £{:.0}. \
             Starting point: {:.0}% (yardstick of equality). \
             Marriage duration: {} years. {}",
            matrimonial_assets,
            total_non_matrimonial,
            starting_point_percentage,
            marriage_duration_years,
            if departure_justified {
                format!("Departure justified: {:?}", departure_reason)
            } else {
                "No departure from equality justified.".to_string()
            }
        );

        Self {
            total_matrimonial: matrimonial_assets,
            total_non_matrimonial,
            party1_non_matrimonial,
            party2_non_matrimonial,
            starting_point_percentage,
            departure_justified,
            departure_reason,
            final_percentage,
            analysis,
        }
    }
}

// ============================================================================
// Asset Schedule
// ============================================================================

/// Schedule of assets (Form E basis)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetSchedule {
    /// Party 1 name
    pub party1: String,
    /// Party 2 name
    pub party2: String,
    /// All assets
    pub assets: Vec<ScheduledAsset>,
    /// Total party 1
    pub total_party1: f64,
    /// Total party 2
    pub total_party2: f64,
    /// Total joint
    pub total_joint: f64,
    /// Grand total
    pub grand_total: f64,
    /// Matrimonial total
    pub matrimonial_total: f64,
    /// Non-matrimonial total
    pub non_matrimonial_total: f64,
}

/// Asset in schedule
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledAsset {
    /// Description
    pub description: String,
    /// Asset type
    pub asset_type: AssetType,
    /// Gross value
    pub gross_value: f64,
    /// Liabilities
    pub liabilities: f64,
    /// Net value
    pub net_value: f64,
    /// Owner
    pub owner: AssetOwnership,
    /// Matrimonial or non-matrimonial
    pub matrimonial: bool,
    /// Notes
    pub notes: String,
}

impl AssetSchedule {
    /// Create new schedule
    pub fn new(party1: &str, party2: &str) -> Self {
        Self {
            party1: party1.to_string(),
            party2: party2.to_string(),
            assets: Vec::new(),
            total_party1: 0.0,
            total_party2: 0.0,
            total_joint: 0.0,
            grand_total: 0.0,
            matrimonial_total: 0.0,
            non_matrimonial_total: 0.0,
        }
    }

    /// Add asset to schedule
    pub fn add_asset(&mut self, asset: ScheduledAsset) {
        // Update totals
        match &asset.owner {
            AssetOwnership::Sole(owner) if owner == &self.party1 => {
                self.total_party1 += asset.net_value;
            }
            AssetOwnership::Sole(owner) if owner == &self.party2 => {
                self.total_party2 += asset.net_value;
            }
            AssetOwnership::Joint => {
                self.total_joint += asset.net_value;
            }
            _ => {}
        }

        if asset.matrimonial {
            self.matrimonial_total += asset.net_value;
        } else {
            self.non_matrimonial_total += asset.net_value;
        }

        self.grand_total += asset.net_value;
        self.assets.push(asset);
    }

    /// Calculate equal division
    pub fn calculate_equal_division(&self) -> EqualDivisionCalculation {
        let target_each = self.matrimonial_total / 2.0;
        let party1_current = self.total_party1 + (self.total_joint / 2.0);
        let party2_current = self.total_party2 + (self.total_joint / 2.0);

        let party1_adjustment = target_each - party1_current;
        let party2_adjustment = target_each - party2_current;

        EqualDivisionCalculation {
            total_matrimonial: self.matrimonial_total,
            target_each,
            party1_current,
            party2_current,
            party1_receives: if party1_adjustment > 0.0 {
                party1_adjustment
            } else {
                0.0
            },
            party2_receives: if party2_adjustment > 0.0 {
                party2_adjustment
            } else {
                0.0
            },
        }
    }
}

/// Equal division calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EqualDivisionCalculation {
    /// Total matrimonial assets
    pub total_matrimonial: f64,
    /// Target each party
    pub target_each: f64,
    /// Party 1 current holdings
    pub party1_current: f64,
    /// Party 2 current holdings
    pub party2_current: f64,
    /// Amount party 1 should receive
    pub party1_receives: f64,
    /// Amount party 2 should receive
    pub party2_receives: f64,
}

// ============================================================================
// Pension Analysis
// ============================================================================

/// Pension sharing analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PensionAnalysis {
    /// Party 1 pensions
    pub party1_pensions: Vec<PensionDetails>,
    /// Party 2 pensions
    pub party2_pensions: Vec<PensionDetails>,
    /// Total party 1 CETV
    pub party1_total_cetv: f64,
    /// Total party 2 CETV
    pub party2_total_cetv: f64,
    /// Recommended approach
    pub recommended_approach: PensionApproach,
    /// Sharing percentages (if sharing)
    pub sharing_orders: Vec<PensionSharingOrder>,
    /// Analysis
    pub analysis: String,
}

/// Pension details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PensionDetails {
    /// Scheme name
    pub scheme_name: String,
    /// Type of pension
    pub pension_type: PensionType,
    /// CETV (Cash Equivalent Transfer Value)
    pub cetv: f64,
    /// Annual benefit (if DB)
    pub annual_benefit: Option<f64>,
    /// In payment?
    pub in_payment: bool,
    /// Retirement age
    pub retirement_age: u32,
    /// Portion accrued during marriage
    pub marital_portion: f64,
}

/// Type of pension
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PensionType {
    /// Defined benefit (final salary)
    DefinedBenefit,
    /// Defined contribution (money purchase)
    DefinedContribution,
    /// State pension
    StatePension,
    /// SIPP
    SIPP,
    /// Other
    Other,
}

/// Approach to pensions on divorce
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PensionApproach {
    /// Pension sharing (s.24B MCA 1973)
    Sharing,
    /// Pension attachment (s.25B MCA 1973)
    Attachment,
    /// Offsetting against other assets
    Offsetting,
    /// Combination of approaches
    Combination,
    /// No order needed
    NoOrderNeeded,
}

/// Pension sharing order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PensionSharingOrder {
    /// Scheme
    pub scheme: String,
    /// Sharing percentage
    pub percentage: f64,
    /// CETV to be shared
    pub cetv_shared: f64,
    /// Recipient
    pub recipient: String,
}

impl PensionAnalysis {
    /// Analyze pensions
    pub fn analyze(
        party1_pensions: Vec<PensionDetails>,
        party2_pensions: Vec<PensionDetails>,
    ) -> Self {
        let party1_total_cetv: f64 = party1_pensions.iter().map(|p| p.cetv).sum();
        let party2_total_cetv: f64 = party2_pensions.iter().map(|p| p.cetv).sum();

        let total_pension = party1_total_cetv + party2_total_cetv;
        let difference = (party1_total_cetv - party2_total_cetv).abs();

        // Determine approach
        let (recommended_approach, sharing_orders) = if difference < 10000.0 {
            (PensionApproach::NoOrderNeeded, vec![])
        } else if party1_total_cetv > party2_total_cetv {
            let share_percentage =
                ((party1_total_cetv - party2_total_cetv) / 2.0) / party1_total_cetv * 100.0;
            let orders = party1_pensions
                .iter()
                .filter(|p| p.cetv > 10000.0)
                .map(|p| PensionSharingOrder {
                    scheme: p.scheme_name.clone(),
                    percentage: share_percentage.min(50.0),
                    cetv_shared: p.cetv * share_percentage.min(50.0) / 100.0,
                    recipient: "Party 2".to_string(),
                })
                .collect();
            (PensionApproach::Sharing, orders)
        } else {
            let share_percentage =
                ((party2_total_cetv - party1_total_cetv) / 2.0) / party2_total_cetv * 100.0;
            let orders = party2_pensions
                .iter()
                .filter(|p| p.cetv > 10000.0)
                .map(|p| PensionSharingOrder {
                    scheme: p.scheme_name.clone(),
                    percentage: share_percentage.min(50.0),
                    cetv_shared: p.cetv * share_percentage.min(50.0) / 100.0,
                    recipient: "Party 1".to_string(),
                })
                .collect();
            (PensionApproach::Sharing, orders)
        };

        let analysis = format!(
            "Pension analysis: Party 1 CETV: £{:.0}, Party 2 CETV: £{:.0}. \
             Total: £{:.0}. Difference: £{:.0}. \
             Recommended approach: {:?}. \
             (MCA 1973 s.24B pension sharing / s.25B attachment)",
            party1_total_cetv, party2_total_cetv, total_pension, difference, recommended_approach
        );

        Self {
            party1_pensions,
            party2_pensions,
            party1_total_cetv,
            party2_total_cetv,
            recommended_approach,
            sharing_orders,
            analysis,
        }
    }
}

// ============================================================================
// Clean Break Analysis
// ============================================================================

/// Clean break analysis (MCA 1973 s.25A)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanBreakAnalysis {
    /// Is clean break appropriate?
    pub clean_break_appropriate: bool,
    /// Immediate clean break possible?
    pub immediate_clean_break: bool,
    /// Deferred clean break possible?
    pub deferred_clean_break: bool,
    /// Reasons for/against clean break
    pub reasons: Vec<String>,
    /// Factors preventing clean break
    pub preventing_factors: Vec<String>,
    /// Term for periodical payments (if deferred)
    pub periodical_payments_term: Option<String>,
    /// Analysis
    pub analysis: String,
}

impl CleanBreakAnalysis {
    /// Analyze clean break possibility
    pub fn analyze(
        party1_self_sufficient: bool,
        party2_self_sufficient: bool,
        young_children: bool,
        capital_available: bool,
        earning_capacity_can_develop: bool,
        timeframe_to_self_sufficiency: Option<u32>,
    ) -> Self {
        let mut reasons = Vec::new();
        let mut preventing_factors = Vec::new();

        if party1_self_sufficient && party2_self_sufficient {
            reasons.push("Both parties financially self-sufficient".to_string());
        }

        if capital_available {
            reasons.push("Sufficient capital available to meet needs".to_string());
        }

        if young_children {
            preventing_factors
                .push("Young children - primary carer may need ongoing support".to_string());
        }

        if !party1_self_sufficient || !party2_self_sufficient {
            preventing_factors.push("Party not yet financially self-sufficient".to_string());
        }

        let immediate_clean_break =
            party1_self_sufficient && party2_self_sufficient && capital_available;

        let deferred_clean_break = !immediate_clean_break && earning_capacity_can_develop;

        let clean_break_appropriate = immediate_clean_break || deferred_clean_break;

        let periodical_payments_term = if deferred_clean_break {
            timeframe_to_self_sufficiency.map(|years| format!("{} years", years))
        } else {
            None
        };

        let analysis = if immediate_clean_break {
            "IMMEDIATE clean break appropriate per MCA 1973 s.25A. \
             Both parties self-sufficient, capital available to meet needs. \
             Court should dismiss spousal maintenance claims."
                .to_string()
        } else if deferred_clean_break {
            format!(
                "DEFERRED clean break appropriate per MCA 1973 s.25A(2). \
                 Term order for periodical payments: {:?}. \
                 Party can become self-sufficient within reasonable period.",
                periodical_payments_term
            )
        } else {
            "Clean break NOT appropriate. Joint lives order or long-term \
             maintenance may be required. Factors: "
                .to_string()
                + preventing_factors.join("; ").as_str()
        };

        Self {
            clean_break_appropriate,
            immediate_clean_break,
            deferred_clean_break,
            reasons,
            preventing_factors,
            periodical_payments_term,
            analysis,
        }
    }
}

// ============================================================================
// Prenuptial Agreement Analysis
// ============================================================================

/// Prenuptial agreement analysis (Radmacher v Granatino \[2010\])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrenupAnalysis {
    /// Agreement date
    pub agreement_date: NaiveDate,
    /// Marriage date
    pub marriage_date: NaiveDate,
    /// Both parties received independent legal advice?
    pub independent_legal_advice: bool,
    /// Full financial disclosure made?
    pub full_disclosure: bool,
    /// Any undue pressure or duress?
    pub undue_pressure: bool,
    /// Needs of parties met?
    pub needs_met: bool,
    /// Needs of children met?
    pub children_needs_met: bool,
    /// Change in circumstances since agreement?
    pub change_in_circumstances: bool,
    /// Weight to be given to agreement
    pub weight: PrenupWeight,
    /// Should court give effect to agreement?
    pub give_effect: bool,
    /// Analysis
    pub analysis: String,
}

/// Weight to give prenuptial agreement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrenupWeight {
    /// Decisive
    Decisive,
    /// Significant
    Significant,
    /// Limited
    Limited,
    /// None
    None,
}

impl PrenupAnalysis {
    /// Analyze prenuptial agreement
    #[allow(clippy::too_many_arguments)]
    pub fn analyze(
        agreement_date: NaiveDate,
        marriage_date: NaiveDate,
        independent_legal_advice: bool,
        full_disclosure: bool,
        undue_pressure: bool,
        needs_met: bool,
        children_needs_met: bool,
        change_in_circumstances: bool,
    ) -> Self {
        // Determine weight per Radmacher v Granatino [2010]
        let (weight, give_effect) = if undue_pressure {
            (PrenupWeight::None, false)
        } else if !independent_legal_advice || !full_disclosure || !needs_met || !children_needs_met
        {
            (PrenupWeight::Limited, false)
        } else if change_in_circumstances {
            (PrenupWeight::Significant, true) // May modify for changed circumstances
        } else {
            (PrenupWeight::Decisive, true)
        };

        let analysis = format!(
            "Prenuptial agreement analysis per Radmacher v Granatino [2010] UKSC 42: \
             ILA: {}, Disclosure: {}, Pressure: {}, Needs met: {}, Children's needs: {}. \
             Weight: {:?}. Court {} give effect to agreement.",
            if independent_legal_advice {
                "Yes"
            } else {
                "No"
            },
            if full_disclosure { "Yes" } else { "No" },
            if undue_pressure { "Yes" } else { "No" },
            if needs_met { "Yes" } else { "No" },
            if children_needs_met { "Yes" } else { "No" },
            weight,
            if give_effect { "SHOULD" } else { "should NOT" }
        );

        Self {
            agreement_date,
            marriage_date,
            independent_legal_advice,
            full_disclosure,
            undue_pressure,
            needs_met,
            children_needs_met,
            change_in_circumstances,
            weight,
            give_effect,
            analysis,
        }
    }
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate section 25 factors addressed
pub fn validate_section25_factors(assessments: &[Section25Assessment]) -> Result<()> {
    let assessed_factors: Vec<_> = assessments.iter().map(|a| &a.factor).collect();

    let missing: Vec<_> = Section25Factor::all()
        .iter()
        .filter(|f| !assessed_factors.contains(f))
        .map(|f| format!("{:?}", f))
        .collect();

    if !missing.is_empty() {
        return Err(FamilyLawError::Section25FactorsNotAddressed {
            missing_factors: missing,
        });
    }

    Ok(())
}

/// Validate clean break considered
pub fn validate_clean_break_considered(
    clean_break_analysis: Option<&CleanBreakAnalysis>,
) -> Result<()> {
    if clean_break_analysis.is_none() {
        return Err(FamilyLawError::CleanBreakNotConsidered);
    }
    Ok(())
}

/// Validate Form E filed
pub fn validate_form_e_filed(
    party1_filed: bool,
    party2_filed: bool,
    party1: &str,
    party2: &str,
) -> Result<()> {
    if !party1_filed {
        return Err(FamilyLawError::FormENotFiled {
            party: party1.to_string(),
        });
    }
    if !party2_filed {
        return Err(FamilyLawError::FormENotFiled {
            party: party2.to_string(),
        });
    }
    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).expect("valid date")
    }

    #[test]
    fn test_section25_factors() {
        let factors = Section25Factor::all();
        assert_eq!(factors.len(), 8);
    }

    #[test]
    fn test_compensation_analysis_applicable() {
        let analysis =
            CompensationAnalysis::analyze("Wife", "Husband", true, Some(500000.0), false, None);

        assert!(analysis.applicable);
        assert_eq!(analysis.claimant, Some("Wife".to_string()));
        assert_eq!(analysis.quantification, Some(500000.0));
    }

    #[test]
    fn test_compensation_analysis_not_applicable() {
        let analysis = CompensationAnalysis::analyze("Wife", "Husband", false, None, false, None);

        assert!(!analysis.applicable);
        assert!(analysis.claimant.is_none());
    }

    #[test]
    fn test_sharing_analysis_equality() {
        let analysis = SharingAnalysis::analyze(1000000.0, 0.0, 0.0, 15, false);

        assert_eq!(analysis.starting_point_percentage, 50.0);
        assert_eq!(analysis.final_percentage, 50.0);
        assert!(!analysis.departure_justified);
    }

    #[test]
    fn test_sharing_analysis_short_marriage() {
        let analysis = SharingAnalysis::analyze(500000.0, 200000.0, 0.0, 3, false);

        assert!(analysis.departure_justified);
        assert!(analysis.departure_reason.is_some());
    }

    #[test]
    fn test_asset_schedule() {
        let mut schedule = AssetSchedule::new("Husband", "Wife");

        schedule.add_asset(ScheduledAsset {
            description: "Family home".to_string(),
            asset_type: AssetType::FamilyHome,
            gross_value: 500000.0,
            liabilities: 200000.0,
            net_value: 300000.0,
            owner: AssetOwnership::Joint,
            matrimonial: true,
            notes: String::new(),
        });

        assert_eq!(schedule.grand_total, 300000.0);
        assert_eq!(schedule.total_joint, 300000.0);
        assert_eq!(schedule.matrimonial_total, 300000.0);
    }

    #[test]
    fn test_clean_break_immediate() {
        let analysis = CleanBreakAnalysis::analyze(true, true, false, true, false, None);

        assert!(analysis.clean_break_appropriate);
        assert!(analysis.immediate_clean_break);
        assert!(!analysis.deferred_clean_break);
    }

    #[test]
    fn test_clean_break_deferred() {
        let analysis = CleanBreakAnalysis::analyze(true, false, false, true, true, Some(3));

        assert!(analysis.clean_break_appropriate);
        assert!(!analysis.immediate_clean_break);
        assert!(analysis.deferred_clean_break);
        assert_eq!(
            analysis.periodical_payments_term,
            Some("3 years".to_string())
        );
    }

    #[test]
    fn test_prenup_decisive() {
        let analysis = PrenupAnalysis::analyze(
            test_date(2019, 1, 1),
            test_date(2020, 1, 1),
            true,
            true,
            false,
            true,
            true,
            false,
        );

        assert_eq!(analysis.weight, PrenupWeight::Decisive);
        assert!(analysis.give_effect);
    }

    #[test]
    fn test_prenup_no_weight_pressure() {
        let analysis = PrenupAnalysis::analyze(
            test_date(2019, 1, 1),
            test_date(2020, 1, 1),
            true,
            true,
            true, // Undue pressure
            true,
            true,
            false,
        );

        assert_eq!(analysis.weight, PrenupWeight::None);
        assert!(!analysis.give_effect);
    }

    #[test]
    fn test_validate_section25_complete() {
        let assessments: Vec<_> = Section25Factor::all()
            .iter()
            .map(|f| Section25Assessment {
                factor: f.clone(),
                party1_assessment: "Assessment".to_string(),
                party2_assessment: "Assessment".to_string(),
                impact: FactorImpact::Neutral,
                notes: String::new(),
            })
            .collect();

        assert!(validate_section25_factors(&assessments).is_ok());
    }

    #[test]
    fn test_validate_section25_incomplete() {
        let assessments = vec![Section25Assessment {
            factor: Section25Factor::IncomeAndResources,
            party1_assessment: "Assessment".to_string(),
            party2_assessment: "Assessment".to_string(),
            impact: FactorImpact::Neutral,
            notes: String::new(),
        }];

        let result = validate_section25_factors(&assessments);
        assert!(matches!(
            result,
            Err(FamilyLawError::Section25FactorsNotAddressed { .. })
        ));
    }

    #[test]
    fn test_pension_analysis() {
        let party1_pensions = vec![PensionDetails {
            scheme_name: "DB Pension".to_string(),
            pension_type: PensionType::DefinedBenefit,
            cetv: 500000.0,
            annual_benefit: Some(25000.0),
            in_payment: false,
            retirement_age: 65,
            marital_portion: 0.8,
        }];

        let party2_pensions = vec![PensionDetails {
            scheme_name: "DC Pension".to_string(),
            pension_type: PensionType::DefinedContribution,
            cetv: 100000.0,
            annual_benefit: None,
            in_payment: false,
            retirement_age: 65,
            marital_portion: 1.0,
        }];

        let analysis = PensionAnalysis::analyze(party1_pensions, party2_pensions);

        assert_eq!(analysis.party1_total_cetv, 500000.0);
        assert_eq!(analysis.party2_total_cetv, 100000.0);
        assert_eq!(analysis.recommended_approach, PensionApproach::Sharing);
        assert!(!analysis.sharing_orders.is_empty());
    }
}
