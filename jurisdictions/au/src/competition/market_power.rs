//! Misuse of Market Power Analysis (CCA s.46)
//!
//! This module implements analysis of misuse of market power under section 46
//! of the Competition and Consumer Act 2010, as amended by the Harper Review
//! reforms in 2017.
//!
//! ## Post-Harper Reform (2017)
//!
//! Section 46 was fundamentally reformed to introduce an effects-based test:
//!
//! **Old test (pre-2017):**
//! - Corporation has substantial market power
//! - Takes advantage of that power
//! - For a proscribed purpose
//!
//! **New test (post-2017):**
//! - Corporation has substantial degree of market power
//! - Engages in conduct
//! - Conduct has purpose, effect, or likely effect of substantially lessening
//!   competition (SLC)
//!
//! ## Key Elements
//!
//! ### Substantial Market Power
//!
//! Market power is the ability to behave persistently in a manner different from
//! competitive behaviour. Factors include:
//! - Market share (threshold typically >40%)
//! - Barriers to entry
//! - Vertical integration
//! - Product differentiation
//! - Countervailing power of buyers
//!
//! ### Substantial Lessening of Competition
//!
//! Assessed by reference to:
//! - Competitive constraints removed or reduced
//! - Likely price, quality, service impacts
//! - Innovation effects
//! - Entry/expansion deterrence
//!
//! ## Leading Cases
//!
//! - Queensland Wire Industries v BHP (1989) - Market power definition
//! - ACCC v Boral Besser Masonry (2003) - Failed predatory pricing case
//! - ACCC v TPG Telecom (2020) - First s.46 case under new test
//! - ACCC v Pacific National (2020) - Rail freight market power

use serde::{Deserialize, Serialize};

use super::types::{BarriersToEntry, Competitor, MarketShare, RelevantMarket, Undertaking};

/// Substantial market power assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubstantialMarketPower {
    /// The undertaking assessed
    pub undertaking: Undertaking,
    /// Relevant market
    pub market: RelevantMarket,
    /// Market share
    pub market_share: MarketShare,
    /// Competitors and their shares
    pub competitors: Vec<Competitor>,
    /// Barriers to entry
    pub barriers: BarriersToEntry,
    /// Market power indicators
    pub indicators: MarketPowerIndicators,
}

impl SubstantialMarketPower {
    /// Create new assessment
    pub fn new(
        undertaking: Undertaking,
        market: RelevantMarket,
        market_share: MarketShare,
    ) -> Self {
        Self {
            undertaking,
            market,
            market_share,
            competitors: Vec::new(),
            barriers: BarriersToEntry::default(),
            indicators: MarketPowerIndicators::default(),
        }
    }

    /// Add competitor
    pub fn with_competitor(mut self, competitor: Competitor) -> Self {
        self.competitors.push(competitor);
        self
    }

    /// Set barriers to entry
    pub fn with_barriers(mut self, barriers: BarriersToEntry) -> Self {
        self.barriers = barriers;
        self
    }

    /// Set indicators
    pub fn with_indicators(mut self, indicators: MarketPowerIndicators) -> Self {
        self.indicators = indicators;
        self
    }
}

/// Market power indicators
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MarketPowerIndicators {
    /// Persistent high profitability
    pub high_profitability: bool,
    /// Profit margin above competitive level
    pub excess_profit_margin: Option<f64>,
    /// Ability to price discriminate
    pub price_discrimination: bool,
    /// Vertical integration
    pub vertically_integrated: bool,
    /// Access to superior technology
    pub superior_technology: bool,
    /// Control of essential inputs
    pub controls_essential_inputs: bool,
    /// Network effects
    pub network_effects: bool,
    /// Countervailing buyer power
    pub buyer_countervailing_power: CountervailingPower,
    /// History of price leadership
    pub price_leadership_history: bool,
}

/// Level of countervailing power
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CountervailingPower {
    /// Strong countervailing power
    Strong,
    /// Moderate countervailing power
    Moderate,
    /// Weak countervailing power
    Weak,
    /// No countervailing power
    #[default]
    None,
}

/// Type of allegedly anti-competitive conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Section46Conduct {
    /// Predatory pricing
    PredatoryPricing(PredatoryPricingFacts),
    /// Refusal to deal
    RefusalToDeal(RefusalToDealFacts),
    /// Price squeeze / margin squeeze
    MarginSqueeze(MarginSqueezeFacts),
    /// Exclusive dealing
    ExclusiveDealing {
        /// Description
        description: String,
        /// Duration
        duration_months: Option<u32>,
    },
    /// Tying
    Tying {
        /// Tying product
        tying_product: String,
        /// Tied product
        tied_product: String,
    },
    /// Bundling
    Bundling {
        /// Products in bundle
        products: Vec<String>,
        /// Discount percentage
        discount_percentage: f64,
    },
    /// Raising rivals' costs
    RaisingRivalsCosts {
        /// Method
        method: String,
        /// Estimated cost increase
        cost_increase_percentage: Option<f64>,
    },
    /// Deterring entry
    DeterringEntry {
        /// Methods used
        methods: Vec<String>,
        /// Potential entrant affected
        potential_entrant: Option<String>,
    },
    /// Other conduct
    Other {
        /// Description
        description: String,
    },
}

/// Facts for predatory pricing analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredatoryPricingFacts {
    /// Price charged
    pub price: f64,
    /// Average variable cost (AVC)
    pub average_variable_cost: f64,
    /// Average total cost (ATC)
    pub average_total_cost: f64,
    /// Long-run average incremental cost (LRAIC)
    pub lraic: Option<f64>,
    /// Duration of below-cost pricing (months)
    pub duration_months: u32,
    /// Geographic scope
    pub geographic_scope: String,
    /// Targeted competitor
    pub targeted_competitor: Option<String>,
    /// Evidence of recoupment strategy
    pub recoupment_evidence: bool,
}

impl PredatoryPricingFacts {
    /// Check if price is below AVC (per se predatory under Areeda-Turner)
    pub fn below_avc(&self) -> bool {
        self.price < self.average_variable_cost
    }

    /// Check if price is below ATC but above AVC
    pub fn below_atc_above_avc(&self) -> bool {
        self.price < self.average_total_cost && self.price >= self.average_variable_cost
    }

    /// Get pricing status
    pub fn pricing_status(&self) -> PricingStatus {
        if self.below_avc() {
            PricingStatus::BelowAvc
        } else if self.below_atc_above_avc() {
            PricingStatus::BelowAtcAboveAvc
        } else {
            PricingStatus::AboveAtc
        }
    }
}

/// Pricing status relative to costs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PricingStatus {
    /// Below average variable cost
    BelowAvc,
    /// Between AVC and ATC
    BelowAtcAboveAvc,
    /// Above average total cost
    AboveAtc,
}

/// Facts for refusal to deal analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefusalToDealFacts {
    /// What was refused (product/service/access)
    pub refused_supply: String,
    /// Is it an essential facility
    pub essential_facility: bool,
    /// Previous supply relationship
    pub previous_supply: bool,
    /// Reason given for refusal
    pub stated_reason: Option<String>,
    /// Is refusal selective (some customers served)
    pub selective_refusal: bool,
    /// Competitor affected
    pub affected_competitor: String,
    /// Downstream market
    pub downstream_market: String,
}

/// Facts for margin squeeze analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginSqueezeFacts {
    /// Upstream (wholesale) price
    pub wholesale_price: f64,
    /// Downstream (retail) price
    pub retail_price: f64,
    /// Competitor's downstream costs (non-input)
    pub competitor_downstream_costs: f64,
    /// Does dominant firm supply to downstream market
    pub vertically_integrated: bool,
    /// Margin available to competitors
    pub competitor_margin: f64,
}

impl MarginSqueezeFacts {
    /// Calculate available margin
    pub fn calculate_margin(&self) -> f64 {
        self.retail_price - self.wholesale_price - self.competitor_downstream_costs
    }

    /// Check if margin is negative (squeeze)
    pub fn is_margin_squeeze(&self) -> bool {
        self.calculate_margin() < 0.0
    }

    /// Check if margin is inadequate (less than reasonable return)
    pub fn is_inadequate_margin(&self, reasonable_return: f64) -> bool {
        self.calculate_margin() < reasonable_return
    }
}

/// Section 46 analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section46Analysis {
    /// The conduct being analyzed
    pub conduct: Section46Conduct,
    /// Market power assessment
    pub market_power: SubstantialMarketPower,
    /// Purpose analysis
    pub purpose: PurposeAnalysis,
    /// Effect analysis
    pub effect: EffectAnalysis,
    /// Likely effect analysis
    pub likely_effect: LikelyEffectAnalysis,
}

/// Purpose analysis (subjective element)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurposeAnalysis {
    /// Evidence of anti-competitive purpose
    pub evidence: Vec<String>,
    /// Documents showing purpose
    pub documents: Vec<String>,
    /// Witness statements
    pub witness_statements: Vec<String>,
    /// Inferred purpose from conduct
    pub inferred_purpose: Option<String>,
    /// Assessment
    pub has_anti_competitive_purpose: bool,
}

/// Effect analysis (actual effect)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectAnalysis {
    /// Competitors exited market
    pub competitor_exits: Vec<String>,
    /// Price changes observed
    pub price_changes: Option<f64>,
    /// Output changes observed
    pub output_changes: Option<f64>,
    /// Quality/innovation changes
    pub quality_changes: Option<String>,
    /// Assessment
    pub has_slc_effect: bool,
}

/// Likely effect analysis (predictive)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LikelyEffectAnalysis {
    /// Predicted competitor impacts
    pub predicted_competitor_impacts: Vec<String>,
    /// Predicted price impacts
    pub predicted_price_impacts: Option<String>,
    /// Predicted entry deterrence
    pub entry_deterrence: bool,
    /// Time horizon for analysis
    pub time_horizon_years: u32,
    /// Counterfactual description
    pub counterfactual: String,
    /// Assessment
    pub has_likely_slc_effect: bool,
}

/// Market power analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketPowerResult {
    /// Has substantial market power
    pub has_substantial_power: bool,
    /// Market share
    pub market_share: f64,
    /// Key factors supporting finding
    pub supporting_factors: Vec<String>,
    /// Key factors against finding
    pub factors_against: Vec<String>,
    /// Overall assessment reasoning
    pub reasoning: String,
}

/// Section 46 contravention result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section46Result {
    /// Contravention likely
    pub contravention_likely: bool,
    /// Market power finding
    pub market_power: MarketPowerResult,
    /// Purpose/effect/likely effect finding
    pub slc_finding: SlcFinding,
    /// Defences available
    pub defences: Vec<Section46Defence>,
    /// Penalty estimate (AUD)
    pub penalty_estimate: Option<f64>,
    /// Reasoning
    pub reasoning: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// SLC finding
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlcFinding {
    /// Purpose element satisfied
    pub purpose: bool,
    /// Effect element satisfied
    pub effect: bool,
    /// Likely effect element satisfied
    pub likely_effect: bool,
    /// At least one element satisfied
    pub any_satisfied: bool,
    /// Reasoning
    pub reasoning: String,
}

/// Defences to section 46
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Section46Defence {
    /// No substantial market power
    NoSubstantialPower { reason: String },
    /// No SLC purpose, effect, or likely effect
    NoSlc { reason: String },
    /// Legitimate business justification
    BusinessJustification {
        justification: String,
        pro_competitive_effects: Vec<String>,
    },
    /// Meeting competition
    MeetingCompetition { competitor: String, price: f64 },
    /// Efficiency defence
    Efficiency { efficiencies: Vec<String> },
}

/// Market power analyzer
pub struct MarketPowerAnalyzer;

impl MarketPowerAnalyzer {
    /// Analyze market power
    pub fn analyze_market_power(assessment: &SubstantialMarketPower) -> MarketPowerResult {
        let market_share = assessment.market_share.primary_share().unwrap_or(0.0);

        let mut supporting = Vec::new();
        let mut against = Vec::new();

        // Market share analysis
        if market_share > 0.40 {
            supporting.push(format!("High market share: {:.1}%", market_share * 100.0));
        } else if market_share > 0.25 {
            supporting.push(format!(
                "Moderate market share: {:.1}%",
                market_share * 100.0
            ));
        } else {
            against.push(format!("Low market share: {:.1}%", market_share * 100.0));
        }

        // Barriers analysis
        match assessment.barriers.overall_level {
            super::types::BarrierLevel::VeryHigh | super::types::BarrierLevel::High => {
                supporting.push("High barriers to entry".into());
            }
            super::types::BarrierLevel::Moderate => {
                // Neutral
            }
            super::types::BarrierLevel::Low | super::types::BarrierLevel::Minimal => {
                against.push("Low barriers to entry".into());
            }
        }

        // Indicator analysis
        if assessment.indicators.controls_essential_inputs {
            supporting.push("Controls essential inputs".into());
        }
        if assessment.indicators.network_effects {
            supporting.push("Network effects present".into());
        }
        if assessment.indicators.vertically_integrated {
            supporting.push("Vertically integrated".into());
        }

        // Countervailing power
        match assessment.indicators.buyer_countervailing_power {
            CountervailingPower::Strong => {
                against.push("Strong buyer countervailing power".into());
            }
            CountervailingPower::Moderate => {
                // Partially mitigating
            }
            CountervailingPower::Weak | CountervailingPower::None => {
                supporting.push("Weak/no buyer countervailing power".into());
            }
        }

        // Competitor analysis
        let strong_competitors = assessment
            .competitors
            .iter()
            .filter(|c| {
                c.market_share
                    .primary_share()
                    .map(|s| s > 0.15)
                    .unwrap_or(false)
            })
            .count();

        if strong_competitors == 0 {
            supporting.push("No strong competitors".into());
        } else if strong_competitors >= 2 {
            against.push(format!("{} strong competitors present", strong_competitors));
        }

        // Overall determination
        let has_power = market_share > 0.40
            || (market_share > 0.25 && supporting.len() > against.len())
            || assessment.indicators.controls_essential_inputs;

        let reasoning =
            Self::build_market_power_reasoning(has_power, market_share, &supporting, &against);

        MarketPowerResult {
            has_substantial_power: has_power,
            market_share,
            supporting_factors: supporting,
            factors_against: against,
            reasoning,
        }
    }

    /// Analyze section 46 contravention
    pub fn analyze_contravention(analysis: &Section46Analysis) -> Section46Result {
        let power_result = Self::analyze_market_power(&analysis.market_power);
        let slc = Self::analyze_slc(&analysis.purpose, &analysis.effect, &analysis.likely_effect);
        let defences = Self::identify_defences(analysis, &power_result, &slc);
        let contravention = power_result.has_substantial_power && slc.any_satisfied;
        let penalty = if contravention {
            Self::estimate_penalty(analysis)
        } else {
            None
        };
        let reasoning =
            Self::build_contravention_reasoning(contravention, &power_result, &slc, &defences);
        let recommendations = Self::generate_recommendations(contravention, &defences);

        Section46Result {
            contravention_likely: contravention,
            market_power: power_result,
            slc_finding: slc,
            defences,
            penalty_estimate: penalty,
            reasoning,
            recommendations,
        }
    }

    /// Analyze SLC elements
    fn analyze_slc(
        purpose: &PurposeAnalysis,
        effect: &EffectAnalysis,
        likely_effect: &LikelyEffectAnalysis,
    ) -> SlcFinding {
        let any = purpose.has_anti_competitive_purpose
            || effect.has_slc_effect
            || likely_effect.has_likely_slc_effect;

        let reasoning = Self::build_slc_reasoning(purpose, effect, likely_effect);

        SlcFinding {
            purpose: purpose.has_anti_competitive_purpose,
            effect: effect.has_slc_effect,
            likely_effect: likely_effect.has_likely_slc_effect,
            any_satisfied: any,
            reasoning,
        }
    }

    /// Identify defences
    fn identify_defences(
        _analysis: &Section46Analysis,
        power: &MarketPowerResult,
        slc: &SlcFinding,
    ) -> Vec<Section46Defence> {
        let mut defences = Vec::new();

        if !power.has_substantial_power {
            defences.push(Section46Defence::NoSubstantialPower {
                reason:
                    "Market share and other factors insufficient to establish substantial power"
                        .into(),
            });
        }

        if !slc.any_satisfied {
            defences.push(Section46Defence::NoSlc {
                reason:
                    "No purpose, effect, or likely effect of substantially lessening competition"
                        .into(),
            });
        }

        defences
    }

    /// Estimate penalty
    fn estimate_penalty(_analysis: &Section46Analysis) -> Option<f64> {
        // Simplified - actual calculation would consider many factors
        Some(5_000_000.0)
    }

    /// Build market power reasoning
    fn build_market_power_reasoning(
        has_power: bool,
        share: f64,
        supporting: &[String],
        against: &[String],
    ) -> String {
        let mut parts = Vec::new();

        parts.push(format!(
            "Market power analysis: market share {:.1}%",
            share * 100.0
        ));

        if has_power {
            parts.push("Substantial market power likely established".into());
            if !supporting.is_empty() {
                parts.push(format!("Supporting factors: {}", supporting.join("; ")));
            }
        } else {
            parts.push("Substantial market power not established".into());
            if !against.is_empty() {
                parts.push(format!("Factors against: {}", against.join("; ")));
            }
        }

        parts.join(". ")
    }

    /// Build SLC reasoning
    fn build_slc_reasoning(
        purpose: &PurposeAnalysis,
        effect: &EffectAnalysis,
        likely_effect: &LikelyEffectAnalysis,
    ) -> String {
        let mut parts: Vec<String> = Vec::new();

        parts.push("SLC analysis under s.46".into());

        if purpose.has_anti_competitive_purpose {
            parts.push("Anti-competitive purpose established".into());
        }

        if effect.has_slc_effect {
            parts.push("Actual effect of SLC established".into());
        }

        if likely_effect.has_likely_slc_effect {
            parts.push("Likely effect of SLC established".into());
        }

        if !purpose.has_anti_competitive_purpose
            && !effect.has_slc_effect
            && !likely_effect.has_likely_slc_effect
        {
            parts.push("No SLC purpose, effect, or likely effect established".into());
        }

        parts.join(". ")
    }

    /// Build contravention reasoning
    fn build_contravention_reasoning(
        contravention: bool,
        power: &MarketPowerResult,
        slc: &SlcFinding,
        defences: &[Section46Defence],
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Section 46 CCA analysis".into());

        if contravention {
            parts.push("Contravention likely".into());
            parts.push(format!(
                "Market power: {}",
                if power.has_substantial_power {
                    "Yes"
                } else {
                    "No"
                }
            ));
            parts.push(format!(
                "SLC: purpose={}, effect={}, likely_effect={}",
                slc.purpose, slc.effect, slc.likely_effect
            ));
        } else {
            parts.push("Contravention unlikely".into());
            if !defences.is_empty() {
                parts.push(format!("{} defence(s) available", defences.len()));
            }
        }

        parts.join(". ")
    }

    /// Generate recommendations
    fn generate_recommendations(contravention: bool, defences: &[Section46Defence]) -> Vec<String> {
        let mut recs = Vec::new();

        if contravention {
            recs.push("Seek legal advice immediately".into());
            recs.push("Consider modifying conduct".into());
            recs.push("Document business justifications".into());
        } else if !defences.is_empty() {
            recs.push("Maintain documentation of business justification".into());
            recs.push("Monitor market conditions for changes".into());
        }

        recs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::competition::types::{GeographicMarket, ProductMarket};

    #[test]
    fn test_predatory_pricing_below_avc() {
        let facts = PredatoryPricingFacts {
            price: 8.0,
            average_variable_cost: 10.0,
            average_total_cost: 15.0,
            lraic: None,
            duration_months: 12,
            geographic_scope: "NSW".into(),
            targeted_competitor: Some("Small Competitor".into()),
            recoupment_evidence: true,
        };

        assert!(facts.below_avc());
        assert_eq!(facts.pricing_status(), PricingStatus::BelowAvc);
    }

    #[test]
    fn test_margin_squeeze() {
        let facts = MarginSqueezeFacts {
            wholesale_price: 80.0,
            retail_price: 100.0,
            competitor_downstream_costs: 25.0,
            vertically_integrated: true,
            competitor_margin: -5.0,
        };

        assert!(facts.is_margin_squeeze());
        assert!(facts.is_inadequate_margin(5.0));
    }

    #[test]
    fn test_market_power_analysis() {
        let undertaking =
            Undertaking::new("Big Corp").with_market_share(MarketShare::from_revenue(0.55));

        let market = RelevantMarket::new(ProductMarket::new("Widgets"), GeographicMarket::National);

        let assessment =
            SubstantialMarketPower::new(undertaking, market, MarketShare::from_revenue(0.55));

        let result = MarketPowerAnalyzer::analyze_market_power(&assessment);

        assert!(result.has_substantial_power);
        assert!(!result.supporting_factors.is_empty());
    }

    #[test]
    fn test_countervailing_power() {
        let mut indicators = MarketPowerIndicators::default();
        assert_eq!(
            indicators.buyer_countervailing_power,
            CountervailingPower::None
        );

        indicators.buyer_countervailing_power = CountervailingPower::Strong;
        assert_eq!(
            indicators.buyer_countervailing_power,
            CountervailingPower::Strong
        );
    }
}
