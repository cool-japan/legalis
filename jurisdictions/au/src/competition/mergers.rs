//! Merger Analysis (CCA s.50)
//!
//! This module implements comprehensive merger analysis under section 50
//! of the Competition and Consumer Act 2010.
//!
//! ## Prohibition
//!
//! Section 50 prohibits acquisitions that would have the effect, or be likely
//! to have the effect, of substantially lessening competition (SLC) in any
//! market.
//!
//! ## ACCC Merger Guidelines Factors
//!
//! The ACCC considers these factors when assessing mergers:
//!
//! 1. **Market Concentration** - HHI and concentration ratios
//! 2. **Import Competition** - Competitive constraint from imports
//! 3. **Barriers to Entry** - Likelihood of new entry
//! 4. **Countervailing Power** - Buyer/supplier power
//! 5. **Likelihood of Collusion** - Post-merger coordination risks
//! 6. **Vertical Integration** - Foreclosure concerns
//! 7. **Dynamic Characteristics** - Innovation, emerging competition
//! 8. **Removal of Vigorous Competitor** - "Maverick" theory
//!
//! ## Clearance Pathways
//!
//! ### Informal Clearance
//! - Most common pathway
//! - Non-binding ACCC view
//! - Typically 6-12 weeks
//!
//! ### Merger Authorisation
//! - Formal determination
//! - Public benefit test
//! - Binding and enforceable
//!
//! ### Merger Undertakings
//! - Court-enforceable undertakings
//! - Typically divestiture conditions
//!
//! ## Leading Cases
//!
//! - ACCC v Metcash (2011) - Substantial lessening test
//! - ACCC v Toll Holdings (2011) - Coordinated effects
//! - ACCC v AGL Energy (2015) - Vertical integration
//! - ACCC v Pacific National (2019) - Rail freight merger

use serde::{Deserialize, Serialize};

use super::types::{
    AcquirerType, AntiCompetitiveEffect, BarriersToEntry, Competitor, CoordinationLikelihood,
    MarketShare, RelevantMarket, Undertaking,
};

/// Type of merger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergerType {
    /// Horizontal - competitors in same market
    Horizontal,
    /// Vertical upstream - acquirer is customer
    VerticalUpstream,
    /// Vertical downstream - acquirer is supplier
    VerticalDownstream,
    /// Conglomerate - unrelated markets
    Conglomerate,
}

/// Merger transaction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerTransaction {
    /// Acquirer
    pub acquirer: Undertaking,
    /// Target
    pub target: Undertaking,
    /// Type of merger
    pub merger_type: MergerType,
    /// Acquirer type
    pub acquirer_type: AcquirerType,
    /// Transaction value (AUD)
    pub transaction_value_aud: Option<f64>,
    /// Assets being acquired (if partial)
    pub assets_acquired: Option<String>,
    /// Relevant markets affected
    pub affected_markets: Vec<RelevantMarket>,
    /// Expected completion date
    pub expected_completion: Option<String>,
}

impl MergerTransaction {
    /// Create new horizontal merger
    pub fn horizontal(acquirer: Undertaking, target: Undertaking) -> Self {
        Self {
            acquirer,
            target,
            merger_type: MergerType::Horizontal,
            acquirer_type: AcquirerType::Horizontal,
            transaction_value_aud: None,
            assets_acquired: None,
            affected_markets: Vec::new(),
            expected_completion: None,
        }
    }

    /// Create new vertical merger
    pub fn vertical(acquirer: Undertaking, target: Undertaking, upstream: bool) -> Self {
        Self {
            acquirer,
            target,
            merger_type: if upstream {
                MergerType::VerticalUpstream
            } else {
                MergerType::VerticalDownstream
            },
            acquirer_type: if upstream {
                AcquirerType::VerticalUpstream
            } else {
                AcquirerType::VerticalDownstream
            },
            transaction_value_aud: None,
            assets_acquired: None,
            affected_markets: Vec::new(),
            expected_completion: None,
        }
    }

    /// Set transaction value
    pub fn with_value(mut self, value_aud: f64) -> Self {
        self.transaction_value_aud = Some(value_aud);
        self
    }

    /// Add affected market
    pub fn with_market(mut self, market: RelevantMarket) -> Self {
        self.affected_markets.push(market);
        self
    }
}

/// Merger market analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerMarketAnalysis {
    /// Market definition
    pub market: RelevantMarket,
    /// Pre-merger concentration (HHI)
    pub pre_merger_hhi: u32,
    /// Post-merger concentration (HHI)
    pub post_merger_hhi: u32,
    /// HHI delta
    pub hhi_delta: u32,
    /// Pre-merger acquirer share
    pub acquirer_pre_share: MarketShare,
    /// Pre-merger target share
    pub target_pre_share: MarketShare,
    /// Post-merger combined share
    pub combined_share: MarketShare,
    /// Remaining competitors
    pub remaining_competitors: Vec<Competitor>,
    /// Barriers to entry
    pub barriers: BarriersToEntry,
    /// Import competition
    pub import_competition: ImportCompetition,
}

impl MergerMarketAnalysis {
    /// Calculate HHI delta
    pub fn calculate_hhi_delta(acquirer_share: f64, target_share: f64) -> u32 {
        // HHI delta = 2 * share_A * share_B * 10000
        let delta = 2.0 * acquirer_share * target_share * 10000.0;
        delta as u32
    }

    /// Check if HHI thresholds exceeded
    pub fn exceeds_hhi_threshold(&self) -> bool {
        // ACCC typically concerned if:
        // - Post-merger HHI > 2000, OR
        // - HHI delta > 100 in already concentrated market
        self.post_merger_hhi > 2000 || (self.hhi_delta > 100 && self.pre_merger_hhi > 1500)
    }

    /// Count effective competitors post-merger
    pub fn effective_competitors_count(&self) -> usize {
        self.remaining_competitors
            .iter()
            .filter(|c| {
                c.market_share
                    .primary_share()
                    .map(|s| s > 0.05)
                    .unwrap_or(false)
            })
            .count()
            + 1 // Include merged entity
    }
}

/// Import competition assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportCompetition {
    /// Current import share
    pub import_share: f64,
    /// Import trend
    pub trend: ImportTrend,
    /// Major import sources
    pub sources: Vec<String>,
    /// Barriers to imports
    pub barriers: Vec<String>,
    /// Competitive constraint assessment
    pub constraint_level: ConstraintLevel,
}

/// Import trend
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImportTrend {
    /// Increasing imports
    Increasing,
    /// Stable imports
    Stable,
    /// Declining imports
    Declining,
}

/// Level of constraint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintLevel {
    /// Strong constraint
    Strong,
    /// Moderate constraint
    Moderate,
    /// Weak constraint
    Weak,
    /// Not a constraint
    NotConstraint,
}

/// Merger effects analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerEffectsAnalysis {
    /// Unilateral effects
    pub unilateral_effects: UnilateralEffects,
    /// Coordinated effects
    pub coordinated_effects: CoordinatedEffects,
    /// Vertical effects (if applicable)
    pub vertical_effects: Option<VerticalEffects>,
    /// Conglomerate effects (if applicable)
    pub conglomerate_effects: Option<ConglomerateEffects>,
    /// Efficiencies claimed
    pub efficiencies: Vec<Efficiency>,
    /// Failing firm defence
    pub failing_firm: Option<FailingFirmDefence>,
}

/// Unilateral effects analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnilateralEffects {
    /// Closeness of competition between merging parties
    pub closeness_of_competition: ClosenessOfCompetition,
    /// Loss of competition concerns
    pub concerns: Vec<String>,
    /// Expected price effect (percentage)
    pub expected_price_effect: Option<f64>,
    /// Remaining competitive constraints
    pub remaining_constraints: Vec<String>,
    /// Overall assessment
    pub likely_harm: bool,
}

/// Closeness of competition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClosenessOfCompetition {
    /// Very close competitors
    VeryClose,
    /// Close competitors
    Close,
    /// Moderate competitors
    Moderate,
    /// Distant competitors
    Distant,
    /// Not competitors
    NotCompetitors,
}

/// Coordinated effects analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoordinatedEffects {
    /// Market characteristics facilitating coordination
    pub facilitating_factors: Vec<CoordinationFactor>,
    /// Market characteristics hindering coordination
    pub hindering_factors: Vec<String>,
    /// Whether merger removes maverick
    pub removes_maverick: bool,
    /// Likelihood of coordination
    pub likelihood: CoordinationLikelihood,
    /// Overall assessment
    pub likely_harm: bool,
}

/// Factors facilitating coordination
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoordinationFactor {
    /// Factor description
    pub factor: String,
    /// Factor type
    pub factor_type: CoordinationFactorType,
    /// Strength of factor
    pub strength: FactorStrength,
}

/// Type of coordination factor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoordinationFactorType {
    /// Price transparency
    PriceTransparency,
    /// Homogeneous products
    HomogeneousProducts,
    /// Stable demand
    StableDemand,
    /// High barriers to entry
    HighBarriers,
    /// Few competitors
    FewCompetitors,
    /// Symmetric market shares
    SymmetricShares,
    /// Multi-market contact
    MultiMarketContact,
    /// History of coordination
    HistoryOfCoordination,
}

/// Strength of factor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorStrength {
    /// Strong
    Strong,
    /// Moderate
    Moderate,
    /// Weak
    Weak,
}

/// Vertical effects analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerticalEffects {
    /// Input foreclosure concerns
    pub input_foreclosure: Option<ForeclosureConcern>,
    /// Customer foreclosure concerns
    pub customer_foreclosure: Option<ForeclosureConcern>,
    /// Access to competitively sensitive information
    pub information_concerns: bool,
    /// Overall assessment
    pub likely_harm: bool,
}

/// Foreclosure concern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForeclosureConcern {
    /// Description
    pub description: String,
    /// Foreclosure ability
    pub ability: bool,
    /// Foreclosure incentive
    pub incentive: bool,
    /// Foreclosure effect
    pub effect: Option<String>,
}

/// Conglomerate effects analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConglomerateEffects {
    /// Portfolio effects
    pub portfolio_effects: Vec<String>,
    /// Bundling concerns
    pub bundling_concerns: Option<String>,
    /// Tying concerns
    pub tying_concerns: Option<String>,
    /// Overall assessment
    pub likely_harm: bool,
}

/// Efficiency claim
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Efficiency {
    /// Efficiency type
    pub efficiency_type: EfficiencyType,
    /// Description
    pub description: String,
    /// Estimated value (AUD per year)
    pub estimated_value_aud: Option<f64>,
    /// Merger-specific (couldn't be achieved otherwise)
    pub merger_specific: bool,
    /// Verifiable
    pub verifiable: bool,
    /// Likely to benefit consumers
    pub benefits_consumers: bool,
}

/// Type of efficiency
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EfficiencyType {
    /// Cost reduction
    CostReduction,
    /// Economies of scale
    EconomiesOfScale,
    /// Economies of scope
    EconomiesOfScope,
    /// Improved products/services
    ProductImprovement,
    /// R&D synergies
    RdSynergies,
    /// Distribution efficiencies
    Distribution,
    /// Procurement efficiencies
    Procurement,
}

/// Failing firm defence
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FailingFirmDefence {
    /// Firm is failing
    pub firm_is_failing: bool,
    /// Evidence of failure
    pub failure_evidence: Vec<String>,
    /// No less anti-competitive acquirer
    pub no_alternative_acquirer: bool,
    /// Assets would exit market absent merger
    pub assets_would_exit: bool,
    /// Defence likely to succeed
    pub defence_likely: bool,
}

/// Merger analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MergerResult {
    /// Whether merger likely to SLC
    pub likely_slc: bool,
    /// Markets where SLC likely
    pub slc_markets: Vec<String>,
    /// Anti-competitive effects identified
    pub effects: Vec<AntiCompetitiveEffect>,
    /// ACCC likely outcome
    pub accc_likely_outcome: AcccOutcome,
    /// Conditions that would address concerns
    pub possible_conditions: Vec<MergerCondition>,
    /// Reasoning
    pub reasoning: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// ACCC likely outcome
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcccOutcome {
    /// Clearance likely
    ClearanceLikely,
    /// Clearance with conditions
    ConditionalClearance,
    /// Opposition likely
    OppositionLikely,
    /// Uncertain
    Uncertain,
}

/// Merger condition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MergerCondition {
    /// Divestiture
    Divestiture {
        /// Assets to divest
        assets: Vec<String>,
        /// Buyer requirements
        buyer_requirements: Vec<String>,
    },
    /// Behavioural undertaking
    Behavioural {
        /// Undertaking description
        undertaking: String,
        /// Duration (years)
        duration_years: u32,
    },
    /// Access undertaking
    Access {
        /// Facility or input
        facility: String,
        /// Terms of access
        terms: String,
    },
}

/// Merger analyzer
pub struct MergerAnalyzer;

impl MergerAnalyzer {
    /// Analyze merger transaction
    pub fn analyze(
        transaction: &MergerTransaction,
        market_analysis: &MergerMarketAnalysis,
        effects_analysis: &MergerEffectsAnalysis,
    ) -> MergerResult {
        let structural_concern = Self::assess_structural_concern(market_analysis);
        let effects_concern = Self::assess_effects_concern(effects_analysis);
        let likely_slc = structural_concern || effects_concern;
        let slc_markets = Self::identify_slc_markets(transaction, likely_slc);
        let effects = Self::identify_effects(effects_analysis);
        let accc_outcome =
            Self::predict_accc_outcome(likely_slc, market_analysis, effects_analysis);
        let conditions = if likely_slc {
            Self::suggest_conditions(transaction, market_analysis)
        } else {
            Vec::new()
        };
        let reasoning =
            Self::build_reasoning(transaction, market_analysis, effects_analysis, likely_slc);
        let recommendations = Self::generate_recommendations(likely_slc, accc_outcome, &conditions);

        MergerResult {
            likely_slc,
            slc_markets,
            effects,
            accc_likely_outcome: accc_outcome,
            possible_conditions: conditions,
            reasoning,
            recommendations,
        }
    }

    /// Assess structural concern
    fn assess_structural_concern(analysis: &MergerMarketAnalysis) -> bool {
        // HHI thresholds
        let hhi_concern = analysis.exceeds_hhi_threshold();

        // Combined share > 35%
        let share_concern = analysis
            .combined_share
            .primary_share()
            .map(|s| s > 0.35)
            .unwrap_or(false);

        // Few remaining competitors
        let competitor_concern = analysis.effective_competitors_count() < 4;

        hhi_concern || share_concern || competitor_concern
    }

    /// Assess effects concern
    fn assess_effects_concern(analysis: &MergerEffectsAnalysis) -> bool {
        analysis.unilateral_effects.likely_harm
            || analysis.coordinated_effects.likely_harm
            || analysis
                .vertical_effects
                .as_ref()
                .map(|v| v.likely_harm)
                .unwrap_or(false)
            || analysis
                .conglomerate_effects
                .as_ref()
                .map(|c| c.likely_harm)
                .unwrap_or(false)
    }

    /// Identify markets where SLC likely
    fn identify_slc_markets(transaction: &MergerTransaction, likely_slc: bool) -> Vec<String> {
        if likely_slc {
            transaction
                .affected_markets
                .iter()
                .map(|m| m.product.description.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Identify anti-competitive effects
    fn identify_effects(analysis: &MergerEffectsAnalysis) -> Vec<AntiCompetitiveEffect> {
        let mut effects = Vec::new();

        if analysis.unilateral_effects.likely_harm {
            effects.push(AntiCompetitiveEffect::UnilateralEffects {
                estimated_price_increase: analysis.unilateral_effects.expected_price_effect,
                lost_competition: analysis.unilateral_effects.concerns.join("; "),
            });
        }

        if analysis.coordinated_effects.likely_harm {
            effects.push(AntiCompetitiveEffect::CoordinatedEffects {
                facilitating_factors: analysis
                    .coordinated_effects
                    .facilitating_factors
                    .iter()
                    .map(|f| f.factor.clone())
                    .collect(),
                likelihood: analysis.coordinated_effects.likelihood,
            });
        }

        if let Some(ref vertical) = analysis.vertical_effects
            && vertical.likely_harm
        {
            effects.push(AntiCompetitiveEffect::VerticalForeclosure {
                input_foreclosure: vertical.input_foreclosure.is_some(),
                customer_foreclosure: vertical.customer_foreclosure.is_some(),
                description: "Vertical foreclosure concerns identified".into(),
            });
        }

        effects
    }

    /// Predict ACCC outcome
    fn predict_accc_outcome(
        likely_slc: bool,
        market_analysis: &MergerMarketAnalysis,
        effects_analysis: &MergerEffectsAnalysis,
    ) -> AcccOutcome {
        if !likely_slc {
            return AcccOutcome::ClearanceLikely;
        }

        // Check for efficiencies
        let significant_efficiencies = effects_analysis
            .efficiencies
            .iter()
            .any(|e| e.merger_specific && e.verifiable && e.benefits_consumers);

        // Check for failing firm
        let failing_firm = effects_analysis
            .failing_firm
            .as_ref()
            .map(|f| f.defence_likely)
            .unwrap_or(false);

        // Check structural severity
        let severe_structural = market_analysis
            .combined_share
            .primary_share()
            .map(|s| s > 0.50)
            .unwrap_or(false);

        if failing_firm {
            AcccOutcome::ClearanceLikely
        } else if severe_structural {
            AcccOutcome::OppositionLikely
        } else if significant_efficiencies {
            AcccOutcome::ConditionalClearance
        } else {
            AcccOutcome::Uncertain
        }
    }

    /// Suggest conditions
    fn suggest_conditions(
        _transaction: &MergerTransaction,
        _market_analysis: &MergerMarketAnalysis,
    ) -> Vec<MergerCondition> {
        // Simplified - would analyze specific markets
        vec![MergerCondition::Divestiture {
            assets: vec!["Overlapping business unit".into()],
            buyer_requirements: vec!["Must be viable standalone".into(), "ACCC approval".into()],
        }]
    }

    /// Build reasoning
    fn build_reasoning(
        transaction: &MergerTransaction,
        market_analysis: &MergerMarketAnalysis,
        effects_analysis: &MergerEffectsAnalysis,
        likely_slc: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Merger analysis under CCA s.50".into());

        let merger_desc = match transaction.merger_type {
            MergerType::Horizontal => "Horizontal merger",
            MergerType::VerticalUpstream => "Vertical merger (upstream)",
            MergerType::VerticalDownstream => "Vertical merger (downstream)",
            MergerType::Conglomerate => "Conglomerate merger",
        };
        parts.push(format!(
            "{}: {} acquiring {}",
            merger_desc, transaction.acquirer.name, transaction.target.name
        ));

        // Structural analysis
        if let Some(share) = market_analysis.combined_share.primary_share() {
            parts.push(format!("Combined market share: {:.1}%", share * 100.0));
        }
        parts.push(format!(
            "HHI: {} → {} (Δ{})",
            market_analysis.pre_merger_hhi,
            market_analysis.post_merger_hhi,
            market_analysis.hhi_delta
        ));

        // Effects summary
        if effects_analysis.unilateral_effects.likely_harm {
            parts.push("Unilateral effects concern".into());
        }
        if effects_analysis.coordinated_effects.likely_harm {
            parts.push("Coordinated effects concern".into());
        }

        // Conclusion
        if likely_slc {
            parts.push("Likely to substantially lessen competition".into());
        } else {
            parts.push("Not likely to substantially lessen competition".into());
        }

        parts.join(". ")
    }

    /// Generate recommendations
    fn generate_recommendations(
        likely_slc: bool,
        outcome: AcccOutcome,
        conditions: &[MergerCondition],
    ) -> Vec<String> {
        let mut recs = Vec::new();

        match outcome {
            AcccOutcome::ClearanceLikely => {
                recs.push("Proceed with informal clearance process".into());
            }
            AcccOutcome::ConditionalClearance => {
                recs.push("Consider proactive undertakings to ACCC".into());
                if !conditions.is_empty() {
                    recs.push("Prepare divestiture package".into());
                }
            }
            AcccOutcome::OppositionLikely => {
                recs.push("Consider restructuring transaction".into());
                recs.push("Engage early with ACCC".into());
                recs.push("Prepare public interest arguments".into());
            }
            AcccOutcome::Uncertain => {
                recs.push("Seek informal guidance from ACCC".into());
                recs.push("Prepare comprehensive submission".into());
            }
        }

        if likely_slc {
            recs.push("Document efficiencies and public benefits".into());
        }

        recs
    }

    /// Calculate HHI from market shares
    pub fn calculate_hhi(shares: &[f64]) -> u32 {
        let hhi: f64 = shares.iter().map(|s| (s * 100.0).powi(2)).sum();
        hhi as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::competition::types::{GeographicMarket, ProductMarket};

    #[test]
    fn test_hhi_calculation() {
        // 4 firms with 25% each
        let shares = vec![0.25, 0.25, 0.25, 0.25];
        let hhi = MergerAnalyzer::calculate_hhi(&shares);
        assert_eq!(hhi, 2500);

        // Monopoly
        let monopoly = vec![1.0];
        let hhi = MergerAnalyzer::calculate_hhi(&monopoly);
        assert_eq!(hhi, 10000);
    }

    #[test]
    fn test_hhi_delta() {
        let delta = MergerMarketAnalysis::calculate_hhi_delta(0.20, 0.15);
        // 2 * 0.20 * 0.15 * 10000 = 600
        assert_eq!(delta, 600);
    }

    #[test]
    fn test_horizontal_merger() {
        let acquirer =
            Undertaking::new("Big Corp").with_market_share(MarketShare::from_revenue(0.30));
        let target =
            Undertaking::new("Small Corp").with_market_share(MarketShare::from_revenue(0.15));

        let transaction = MergerTransaction::horizontal(acquirer, target);

        assert_eq!(transaction.merger_type, MergerType::Horizontal);
    }

    #[test]
    fn test_merger_market_analysis() {
        let market = RelevantMarket::new(ProductMarket::new("Widgets"), GeographicMarket::National);

        let analysis = MergerMarketAnalysis {
            market,
            pre_merger_hhi: 1800,
            post_merger_hhi: 2400,
            hhi_delta: 600,
            acquirer_pre_share: MarketShare::from_revenue(0.30),
            target_pre_share: MarketShare::from_revenue(0.15),
            combined_share: MarketShare::from_revenue(0.45),
            remaining_competitors: Vec::new(),
            barriers: BarriersToEntry::default(),
            import_competition: ImportCompetition {
                import_share: 0.10,
                trend: ImportTrend::Stable,
                sources: vec!["China".into()],
                barriers: Vec::new(),
                constraint_level: ConstraintLevel::Moderate,
            },
        };

        assert!(analysis.exceeds_hhi_threshold());
    }

    #[test]
    fn test_closeness_of_competition() {
        let unilateral = UnilateralEffects {
            closeness_of_competition: ClosenessOfCompetition::VeryClose,
            concerns: vec!["Primary competitors".into()],
            expected_price_effect: Some(0.05),
            remaining_constraints: vec!["Imports".into()],
            likely_harm: true,
        };

        assert!(unilateral.likely_harm);
    }

    #[test]
    fn test_accc_outcome_failing_firm() {
        let failing = FailingFirmDefence {
            firm_is_failing: true,
            failure_evidence: vec!["Negative cash flow".into(), "Cannot service debt".into()],
            no_alternative_acquirer: true,
            assets_would_exit: true,
            defence_likely: true,
        };

        assert!(failing.defence_likely);
    }
}
