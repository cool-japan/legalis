//! Legal Simulation & Outcome Prediction
//!
//! Comprehensive simulation framework for legal outcomes including case prediction,
//! litigation risk assessment, settlement estimation, judge/jury modeling,
//! multi-agent negotiation, contract scenarios, regulatory compliance, and what-if analysis.

use anyhow::{Result, anyhow};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Case Outcome Prediction
// ============================================================================

/// Case outcome prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseOutcome {
    /// Predicted verdict
    pub verdict: Verdict,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Probability of plaintiff win
    pub plaintiff_win_probability: f64,
    /// Probability of defendant win
    pub defendant_win_probability: f64,
    /// Expected damages (if applicable)
    pub expected_damages: Option<f64>,
    /// Key factors influencing the outcome
    pub key_factors: Vec<String>,
    /// Similar historical cases
    pub similar_cases: Vec<String>,
}

/// Verdict type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Verdict {
    /// Plaintiff wins
    PlaintiffWin,
    /// Defendant wins
    DefendantWin,
    /// Settlement
    Settlement,
    /// Dismissed
    Dismissed,
    /// Hung jury
    HungJury,
}

/// Case outcome predictor
#[derive(Debug, Clone)]
pub struct CaseOutcomePredictor {
    /// Historical case database
    historical_cases: Vec<HistoricalCase>,
    /// Feature weights for prediction
    feature_weights: HashMap<String, f64>,
}

impl CaseOutcomePredictor {
    /// Creates a new case outcome predictor.
    pub fn new() -> Self {
        Self {
            historical_cases: Vec::new(),
            feature_weights: Self::default_weights(),
        }
    }

    /// Default feature weights.
    fn default_weights() -> HashMap<String, f64> {
        let mut weights = HashMap::new();
        weights.insert("evidence_strength".to_string(), 0.35);
        weights.insert("precedent_alignment".to_string(), 0.25);
        weights.insert("legal_representation".to_string(), 0.15);
        weights.insert("jurisdiction_bias".to_string(), 0.10);
        weights.insert("case_complexity".to_string(), 0.10);
        weights.insert("public_sentiment".to_string(), 0.05);
        weights
    }

    /// Adds a historical case to the database.
    pub fn add_historical_case(&mut self, case: HistoricalCase) {
        self.historical_cases.push(case);
    }

    /// Predicts the outcome of a case.
    pub fn predict(&self, case: &CaseFeatures) -> Result<CaseOutcome> {
        if self.historical_cases.is_empty() {
            return Err(anyhow!("No historical cases available for prediction"));
        }

        let similar_cases = self.find_similar_cases(case, 5);

        let plaintiff_wins = similar_cases
            .iter()
            .filter(|c| c.outcome == Verdict::PlaintiffWin)
            .count();

        let total_cases = similar_cases.len().max(1);
        let plaintiff_win_prob = plaintiff_wins as f64 / total_cases as f64;
        let defendant_win_prob = 1.0 - plaintiff_win_prob;

        let verdict = if plaintiff_win_prob > 0.5 {
            Verdict::PlaintiffWin
        } else {
            Verdict::DefendantWin
        };

        let confidence = (plaintiff_win_prob - 0.5).abs() * 2.0;

        let expected_damages = if verdict == Verdict::PlaintiffWin {
            let avg_damages: f64 = similar_cases
                .iter()
                .filter_map(|c| c.damages_awarded)
                .sum::<f64>()
                / similar_cases
                    .iter()
                    .filter(|c| c.damages_awarded.is_some())
                    .count()
                    .max(1) as f64;
            Some(avg_damages)
        } else {
            None
        };

        Ok(CaseOutcome {
            verdict,
            confidence,
            plaintiff_win_probability: plaintiff_win_prob,
            defendant_win_probability: defendant_win_prob,
            expected_damages,
            key_factors: case.extract_key_factors(),
            similar_cases: similar_cases.iter().map(|c| c.id.clone()).collect(),
        })
    }

    /// Finds similar historical cases.
    fn find_similar_cases(&self, case: &CaseFeatures, limit: usize) -> Vec<HistoricalCase> {
        let mut scored_cases: Vec<(f64, &HistoricalCase)> = self
            .historical_cases
            .iter()
            .map(|hist| (self.calculate_similarity(&hist.features, case), hist))
            .collect();

        scored_cases.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        scored_cases
            .into_iter()
            .take(limit)
            .map(|(_, case)| case.clone())
            .collect()
    }

    /// Calculates similarity between two cases.
    fn calculate_similarity(&self, case1: &CaseFeatures, case2: &CaseFeatures) -> f64 {
        let mut similarity = 0.0;

        for (feature, weight) in &self.feature_weights {
            let feature_sim = case1.get_feature_similarity(case2, feature);
            similarity += feature_sim * weight;
        }

        similarity
    }
}

impl Default for CaseOutcomePredictor {
    fn default() -> Self {
        Self::new()
    }
}

/// Historical case data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalCase {
    /// Case ID
    pub id: String,
    /// Case features
    pub features: CaseFeatures,
    /// Actual outcome
    pub outcome: Verdict,
    /// Damages awarded (if applicable)
    pub damages_awarded: Option<f64>,
    /// Year decided
    pub year: u32,
}

/// Case features for prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseFeatures {
    /// Evidence strength (0.0 - 1.0)
    pub evidence_strength: f64,
    /// Precedent alignment (0.0 - 1.0)
    pub precedent_alignment: f64,
    /// Legal representation quality (0.0 - 1.0)
    pub legal_representation: f64,
    /// Jurisdiction bias (-1.0 to 1.0, negative favors defendant)
    pub jurisdiction_bias: f64,
    /// Case complexity (0.0 - 1.0)
    pub case_complexity: f64,
    /// Public sentiment (-1.0 to 1.0)
    pub public_sentiment: f64,
    /// Case type
    pub case_type: String,
    /// Jurisdiction
    pub jurisdiction: String,
}

impl CaseFeatures {
    /// Gets feature similarity with another case.
    fn get_feature_similarity(&self, other: &CaseFeatures, feature: &str) -> f64 {
        match feature {
            "evidence_strength" => 1.0 - (self.evidence_strength - other.evidence_strength).abs(),
            "precedent_alignment" => {
                1.0 - (self.precedent_alignment - other.precedent_alignment).abs()
            }
            "legal_representation" => {
                1.0 - (self.legal_representation - other.legal_representation).abs()
            }
            "jurisdiction_bias" => {
                1.0 - (self.jurisdiction_bias - other.jurisdiction_bias).abs() / 2.0
            }
            "case_complexity" => 1.0 - (self.case_complexity - other.case_complexity).abs(),
            "public_sentiment" => {
                1.0 - (self.public_sentiment - other.public_sentiment).abs() / 2.0
            }
            _ => 0.5,
        }
    }

    /// Extracts key factors from features.
    fn extract_key_factors(&self) -> Vec<String> {
        let mut factors = Vec::new();

        if self.evidence_strength > 0.7 {
            factors.push("Strong evidence".to_string());
        } else if self.evidence_strength < 0.3 {
            factors.push("Weak evidence".to_string());
        }

        if self.precedent_alignment > 0.7 {
            factors.push("Favorable precedent".to_string());
        } else if self.precedent_alignment < 0.3 {
            factors.push("Unfavorable precedent".to_string());
        }

        if self.case_complexity > 0.7 {
            factors.push("High complexity".to_string());
        }

        if self.jurisdiction_bias.abs() > 0.5 {
            factors.push(format!(
                "Jurisdiction bias: {:.1}%",
                self.jurisdiction_bias * 100.0
            ));
        }

        factors
    }
}

// ============================================================================
// Litigation Risk Assessment
// ============================================================================

/// Litigation risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LitigationRisk {
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// Risk score (0.0 - 1.0)
    pub risk_score: f64,
    /// Estimated costs
    pub estimated_costs: CostEstimate,
    /// Risk factors
    pub risk_factors: Vec<RiskFactor>,
    /// Mitigation strategies
    pub mitigation_strategies: Vec<String>,
    /// Recommended action
    pub recommended_action: RecommendedAction,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

/// Cost estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    /// Minimum expected cost
    pub min_cost: f64,
    /// Maximum expected cost
    pub max_cost: f64,
    /// Most likely cost
    pub expected_cost: f64,
    /// Breakdown by category
    pub breakdown: HashMap<String, f64>,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Factor name
    pub name: String,
    /// Impact level (0.0 - 1.0)
    pub impact: f64,
    /// Probability (0.0 - 1.0)
    pub probability: f64,
    /// Description
    pub description: String,
}

/// Recommended action
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RecommendedAction {
    /// Proceed with litigation
    Proceed,
    /// Settle out of court
    Settle,
    /// Seek mediation
    Mediate,
    /// Abandon claim
    Abandon,
}

/// Litigation risk assessor
#[derive(Debug, Clone)]
pub struct LitigationRiskAssessor;

impl LitigationRiskAssessor {
    /// Creates a new litigation risk assessor.
    pub fn new() -> Self {
        Self
    }

    /// Assesses litigation risk.
    pub fn assess(&self, case: &CaseFeatures, factors: &[RiskFactor]) -> Result<LitigationRisk> {
        let risk_score = self.calculate_risk_score(case, factors);
        let risk_level = self.determine_risk_level(risk_score);
        let estimated_costs = self.estimate_costs(case, risk_level);
        let mitigation_strategies = self.suggest_mitigation(risk_level, factors);
        let recommended_action = self.recommend_action(risk_level, &estimated_costs);

        Ok(LitigationRisk {
            risk_level,
            risk_score,
            estimated_costs,
            risk_factors: factors.to_vec(),
            mitigation_strategies,
            recommended_action,
        })
    }

    /// Calculates overall risk score.
    fn calculate_risk_score(&self, case: &CaseFeatures, factors: &[RiskFactor]) -> f64 {
        let mut score = 0.0;

        score += (1.0 - case.evidence_strength) * 0.3;
        score += (1.0 - case.precedent_alignment) * 0.25;
        score += case.case_complexity * 0.2;

        for factor in factors {
            score += factor.impact * factor.probability * 0.25 / factors.len().max(1) as f64;
        }

        score.min(1.0)
    }

    /// Determines risk level from score.
    fn determine_risk_level(&self, score: f64) -> RiskLevel {
        if score < 0.25 {
            RiskLevel::Low
        } else if score < 0.5 {
            RiskLevel::Medium
        } else if score < 0.75 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        }
    }

    /// Estimates litigation costs.
    fn estimate_costs(&self, case: &CaseFeatures, risk_level: RiskLevel) -> CostEstimate {
        let base_cost = match risk_level {
            RiskLevel::Low => 50000.0,
            RiskLevel::Medium => 150000.0,
            RiskLevel::High => 300000.0,
            RiskLevel::Critical => 500000.0,
        };

        let complexity_multiplier = 1.0 + case.case_complexity;
        let expected_cost = base_cost * complexity_multiplier;

        let mut breakdown = HashMap::new();
        breakdown.insert("attorney_fees".to_string(), expected_cost * 0.6);
        breakdown.insert("court_fees".to_string(), expected_cost * 0.1);
        breakdown.insert("expert_witnesses".to_string(), expected_cost * 0.15);
        breakdown.insert("discovery".to_string(), expected_cost * 0.1);
        breakdown.insert("misc".to_string(), expected_cost * 0.05);

        CostEstimate {
            min_cost: expected_cost * 0.7,
            max_cost: expected_cost * 1.5,
            expected_cost,
            breakdown,
        }
    }

    /// Suggests mitigation strategies.
    fn suggest_mitigation(&self, risk_level: RiskLevel, factors: &[RiskFactor]) -> Vec<String> {
        let mut strategies = Vec::new();

        match risk_level {
            RiskLevel::Low => {
                strategies.push("Proceed with standard litigation strategy".to_string());
            }
            RiskLevel::Medium => {
                strategies.push("Consider early settlement discussions".to_string());
                strategies.push("Strengthen evidence collection".to_string());
            }
            RiskLevel::High => {
                strategies.push("Strongly recommend settlement negotiations".to_string());
                strategies.push("Engage expert witnesses".to_string());
                strategies.push("Consider alternative dispute resolution".to_string());
            }
            RiskLevel::Critical => {
                strategies.push("Avoid litigation if possible".to_string());
                strategies.push("Seek immediate settlement".to_string());
                strategies.push("Consider abandoning claim".to_string());
            }
        }

        for factor in factors {
            if factor.impact > 0.7 {
                strategies.push(format!("Address high-impact factor: {}", factor.name));
            }
        }

        strategies
    }

    /// Recommends action based on risk assessment.
    fn recommend_action(&self, risk_level: RiskLevel, costs: &CostEstimate) -> RecommendedAction {
        match risk_level {
            RiskLevel::Low => RecommendedAction::Proceed,
            RiskLevel::Medium => {
                if costs.expected_cost > 200000.0 {
                    RecommendedAction::Mediate
                } else {
                    RecommendedAction::Proceed
                }
            }
            RiskLevel::High => RecommendedAction::Settle,
            RiskLevel::Critical => RecommendedAction::Abandon,
        }
    }
}

impl Default for LitigationRiskAssessor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Settlement Value Estimation
// ============================================================================

/// Settlement value estimation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementEstimate {
    /// Minimum settlement value
    pub min_value: f64,
    /// Maximum settlement value
    pub max_value: f64,
    /// Expected settlement value
    pub expected_value: f64,
    /// Confidence interval (0.0 - 1.0)
    pub confidence: f64,
    /// Factors affecting valuation
    pub valuation_factors: Vec<ValuationFactor>,
    /// Recommended settlement range
    pub recommended_range: (f64, f64),
}

/// Valuation factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationFactor {
    /// Factor name
    pub name: String,
    /// Impact on value
    pub value_impact: f64,
    /// Description
    pub description: String,
}

/// Settlement value estimator
#[derive(Debug, Clone)]
pub struct SettlementEstimator;

impl SettlementEstimator {
    /// Creates a new settlement estimator.
    pub fn new() -> Self {
        Self
    }

    /// Estimates settlement value.
    pub fn estimate(
        &self,
        case: &CaseFeatures,
        claimed_damages: f64,
        trial_probability: f64,
    ) -> Result<SettlementEstimate> {
        let valuation_factors = self.identify_valuation_factors(case);

        let base_value = claimed_damages * trial_probability;

        let adjustment = valuation_factors
            .iter()
            .map(|f| f.value_impact)
            .sum::<f64>();

        let expected_value = base_value * (1.0 + adjustment);
        let min_value = expected_value * 0.6;
        let max_value = expected_value * 1.4;

        let confidence = case.evidence_strength * 0.5 + case.precedent_alignment * 0.5;

        let recommended_range = (expected_value * 0.8, expected_value * 1.2);

        Ok(SettlementEstimate {
            min_value,
            max_value,
            expected_value,
            confidence,
            valuation_factors,
            recommended_range,
        })
    }

    /// Identifies factors affecting valuation.
    fn identify_valuation_factors(&self, case: &CaseFeatures) -> Vec<ValuationFactor> {
        let mut factors = Vec::new();

        if case.evidence_strength > 0.7 {
            factors.push(ValuationFactor {
                name: "Strong Evidence".to_string(),
                value_impact: 0.2,
                description: "Strong evidence increases settlement value".to_string(),
            });
        }

        if case.case_complexity > 0.7 {
            factors.push(ValuationFactor {
                name: "High Complexity".to_string(),
                value_impact: -0.1,
                description: "Complex cases may reduce settlement offers".to_string(),
            });
        }

        if case.precedent_alignment > 0.7 {
            factors.push(ValuationFactor {
                name: "Favorable Precedent".to_string(),
                value_impact: 0.15,
                description: "Strong precedent increases value".to_string(),
            });
        }

        factors
    }
}

impl Default for SettlementEstimator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Judge/Jury Behavior Modeling
// ============================================================================

/// Judge/jury behavior model
#[derive(Debug, Clone)]
pub struct JudgeBehaviorModel {
    /// Judge/jury type
    pub decision_maker: DecisionMaker,
    /// Behavioral tendencies
    pub tendencies: BehavioralTendencies,
    /// Historical decisions
    pub historical_decisions: Vec<JudicialDecision>,
}

/// Decision maker type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DecisionMaker {
    /// Judge (bench trial)
    Judge,
    /// Jury
    Jury,
}

/// Behavioral tendencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralTendencies {
    /// Plaintiff bias (-1.0 to 1.0)
    pub plaintiff_bias: f64,
    /// Risk aversion (0.0 - 1.0)
    pub risk_aversion: f64,
    /// Sympathy factor (0.0 - 1.0)
    pub sympathy_factor: f64,
    /// Procedural strictness (0.0 - 1.0)
    pub procedural_strictness: f64,
    /// Damages generosity (0.0 - 1.0)
    pub damages_generosity: f64,
}

/// Judicial decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudicialDecision {
    /// Case ID
    pub case_id: String,
    /// Verdict
    pub verdict: Verdict,
    /// Damages awarded
    pub damages: Option<f64>,
    /// Decision factors
    pub factors: Vec<String>,
}

impl JudgeBehaviorModel {
    /// Creates a new judge behavior model.
    pub fn new(decision_maker: DecisionMaker) -> Self {
        let tendencies = match decision_maker {
            DecisionMaker::Judge => BehavioralTendencies {
                plaintiff_bias: 0.0,
                risk_aversion: 0.6,
                sympathy_factor: 0.3,
                procedural_strictness: 0.8,
                damages_generosity: 0.4,
            },
            DecisionMaker::Jury => BehavioralTendencies {
                plaintiff_bias: 0.1,
                risk_aversion: 0.4,
                sympathy_factor: 0.7,
                procedural_strictness: 0.3,
                damages_generosity: 0.6,
            },
        };

        Self {
            decision_maker,
            tendencies,
            historical_decisions: Vec::new(),
        }
    }

    /// Predicts behavior for a case.
    pub fn predict_behavior(&self, case: &CaseFeatures) -> Result<BehaviorPrediction> {
        let mut plaintiff_adjustment = self.tendencies.plaintiff_bias;

        if case.public_sentiment > 0.5 {
            plaintiff_adjustment += self.tendencies.sympathy_factor * case.public_sentiment * 0.2;
        }

        if case.case_complexity > 0.7 && self.tendencies.risk_aversion > 0.5 {
            plaintiff_adjustment -= 0.1;
        }

        let verdict_probability = 0.5 + plaintiff_adjustment;

        let damages_multiplier = if self.decision_maker == DecisionMaker::Jury {
            1.0 + self.tendencies.damages_generosity * 0.5
        } else {
            1.0 + self.tendencies.damages_generosity * 0.2
        };

        Ok(BehaviorPrediction {
            decision_maker: self.decision_maker,
            plaintiff_win_probability: verdict_probability.clamp(0.0, 1.0),
            damages_multiplier,
            key_influences: self.identify_key_influences(case),
        })
    }

    /// Identifies key influences on decision-making.
    fn identify_key_influences(&self, case: &CaseFeatures) -> Vec<String> {
        let mut influences = Vec::new();

        if self.tendencies.sympathy_factor > 0.6 && case.public_sentiment > 0.5 {
            influences.push("High sympathy for plaintiff".to_string());
        }

        if self.tendencies.procedural_strictness > 0.7 {
            influences.push("Strict adherence to procedure".to_string());
        }

        if self.tendencies.damages_generosity > 0.6 {
            influences.push("Generous damages awards".to_string());
        }

        influences
    }
}

/// Behavior prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPrediction {
    /// Type of decision maker
    pub decision_maker: DecisionMaker,
    /// Probability of plaintiff win
    pub plaintiff_win_probability: f64,
    /// Damages multiplier (relative to claimed damages)
    pub damages_multiplier: f64,
    /// Key influences on decision
    pub key_influences: Vec<String>,
}

// ============================================================================
// Multi-Agent Negotiation Simulation
// ============================================================================

/// Negotiation simulation
#[derive(Debug, Clone)]
pub struct NegotiationSimulation {
    /// Negotiating parties
    pub parties: Vec<NegotiationParty>,
    /// Rounds of negotiation
    pub rounds: Vec<NegotiationRound>,
    /// Current round
    pub current_round: usize,
}

/// Negotiation party
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationParty {
    /// Party name
    pub name: String,
    /// Minimum acceptable value
    pub reservation_value: f64,
    /// Target value
    pub target_value: f64,
    /// Negotiation strategy
    pub strategy: NegotiationStrategy,
    /// Concession rate (0.0 - 1.0)
    pub concession_rate: f64,
}

/// Negotiation strategy
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum NegotiationStrategy {
    /// Competitive (maximize own value)
    Competitive,
    /// Collaborative (find mutual value)
    Collaborative,
    /// Accommodating (quick settlement)
    Accommodating,
    /// Avoiding (delay/withdraw)
    Avoiding,
}

/// Negotiation round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationRound {
    /// Round number
    pub round: usize,
    /// Offers made
    pub offers: HashMap<String, f64>,
    /// Counteroffers
    pub counteroffers: HashMap<String, f64>,
    /// Round outcome
    pub outcome: RoundOutcome,
}

/// Round outcome
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoundOutcome {
    /// Continued negotiation
    Continued,
    /// Agreement reached
    Agreement { value: f64 },
    /// Negotiation failed
    Failed,
}

impl NegotiationSimulation {
    /// Creates a new negotiation simulation.
    pub fn new(parties: Vec<NegotiationParty>) -> Self {
        Self {
            parties,
            rounds: Vec::new(),
            current_round: 0,
        }
    }

    /// Simulates a negotiation round.
    pub fn simulate_round(&mut self) -> Result<RoundOutcome> {
        self.current_round += 1;

        let mut offers = HashMap::new();
        let mut counteroffers = HashMap::new();

        for party in &self.parties {
            let offer = self.calculate_offer(party);
            offers.insert(party.name.clone(), offer);
        }

        for party in &self.parties {
            let other_offers: Vec<f64> = offers
                .iter()
                .filter(|(name, _)| *name != &party.name)
                .map(|(_, v)| *v)
                .collect();

            if !other_offers.is_empty() {
                let counteroffer = self.calculate_counteroffer(party, &other_offers);
                counteroffers.insert(party.name.clone(), counteroffer);
            }
        }

        let outcome = self.evaluate_round(&offers, &counteroffers);

        self.rounds.push(NegotiationRound {
            round: self.current_round,
            offers,
            counteroffers,
            outcome: outcome.clone(),
        });

        Ok(outcome)
    }

    /// Calculates an offer for a party.
    fn calculate_offer(&self, party: &NegotiationParty) -> f64 {
        let progress = self.current_round as f64 / 10.0;
        let concession = party.concession_rate * progress;

        match party.strategy {
            NegotiationStrategy::Competitive => party.target_value * (1.0 - concession * 0.3),
            NegotiationStrategy::Collaborative => {
                party.reservation_value
                    + (party.target_value - party.reservation_value) * (1.0 - concession * 0.5)
            }
            NegotiationStrategy::Accommodating => {
                party.reservation_value
                    + (party.target_value - party.reservation_value) * (1.0 - concession * 0.7)
            }
            NegotiationStrategy::Avoiding => party.target_value * 1.2,
        }
    }

    /// Calculates a counteroffer.
    fn calculate_counteroffer(&self, party: &NegotiationParty, other_offers: &[f64]) -> f64 {
        let avg_other = other_offers.iter().sum::<f64>() / other_offers.len() as f64;
        let my_offer = self.calculate_offer(party);

        (my_offer + avg_other) / 2.0
    }

    /// Evaluates if round reached agreement.
    fn evaluate_round(
        &self,
        offers: &HashMap<String, f64>,
        counteroffers: &HashMap<String, f64>,
    ) -> RoundOutcome {
        if self.current_round > 20 {
            return RoundOutcome::Failed;
        }

        let all_values: Vec<f64> = offers
            .values()
            .chain(counteroffers.values())
            .copied()
            .collect();
        if all_values.is_empty() {
            return RoundOutcome::Continued;
        }

        let min_val = all_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max_val = all_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        if (max_val - min_val) / min_val < 0.1 {
            let agreement_value = (min_val + max_val) / 2.0;
            RoundOutcome::Agreement {
                value: agreement_value,
            }
        } else {
            RoundOutcome::Continued
        }
    }

    /// Runs full simulation until agreement or failure.
    pub fn run_full_simulation(&mut self, max_rounds: usize) -> Result<RoundOutcome> {
        for _ in 0..max_rounds {
            let outcome = self.simulate_round()?;
            match outcome {
                RoundOutcome::Agreement { .. } | RoundOutcome::Failed => return Ok(outcome),
                RoundOutcome::Continued => continue,
            }
        }
        Ok(RoundOutcome::Failed)
    }
}

// ============================================================================
// Contract Scenario Simulation
// ============================================================================

/// Contract scenario simulator
#[derive(Debug, Clone)]
pub struct ContractScenarioSimulator {
    /// Contract terms
    pub contract: ContractTerms,
    /// Scenarios to simulate
    pub scenarios: Vec<ContractScenario>,
}

/// Contract terms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTerms {
    /// Contract ID
    pub id: String,
    /// Contract type
    pub contract_type: String,
    /// Parties involved
    pub parties: Vec<String>,
    /// Key clauses
    pub clauses: Vec<SimContractClause>,
    /// Duration (days)
    pub duration_days: u32,
}

/// Contract clause for simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimContractClause {
    /// Clause ID
    pub id: String,
    /// Clause type
    pub clause_type: ClauseType,
    /// Clause text
    pub text: String,
    /// Conditions
    pub conditions: Vec<String>,
}

/// Clause type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ClauseType {
    /// Performance obligation
    Performance,
    /// Payment terms
    Payment,
    /// Termination
    Termination,
    /// Liability
    Liability,
    /// Confidentiality
    Confidentiality,
    /// Dispute resolution
    DisputeResolution,
}

/// Contract scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractScenario {
    /// Scenario name
    pub name: String,
    /// Description
    pub description: String,
    /// Events that occur
    pub events: Vec<ContractEvent>,
}

/// Contract event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractEvent {
    /// Event type
    pub event_type: String,
    /// Day of occurrence
    pub day: u32,
    /// Description
    pub description: String,
    /// Affected clauses
    pub affected_clauses: Vec<String>,
}

/// Scenario simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    /// Scenario name
    pub scenario: String,
    /// Triggered clauses
    pub triggered_clauses: Vec<String>,
    /// Potential liabilities
    pub liabilities: HashMap<String, f64>,
    /// Breach analysis
    pub breaches: Vec<BreachAnalysis>,
    /// Risk assessment
    pub risk_level: RiskLevel,
}

/// Breach analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreachAnalysis {
    /// Breaching party
    pub party: String,
    /// Breached clause
    pub clause_id: String,
    /// Severity (0.0 - 1.0)
    pub severity: f64,
    /// Remedies available
    pub remedies: Vec<String>,
}

impl ContractScenarioSimulator {
    /// Creates a new contract scenario simulator.
    pub fn new(contract: ContractTerms) -> Self {
        Self {
            contract,
            scenarios: Vec::new(),
        }
    }

    /// Adds a scenario to simulate.
    pub fn add_scenario(&mut self, scenario: ContractScenario) {
        self.scenarios.push(scenario);
    }

    /// Simulates a specific scenario.
    pub fn simulate_scenario(&self, scenario: &ContractScenario) -> Result<ScenarioResult> {
        let mut triggered_clauses = Vec::new();
        let mut liabilities = HashMap::new();
        let mut breaches = Vec::new();

        for event in &scenario.events {
            for clause_id in &event.affected_clauses {
                if let Some(clause) = self.contract.clauses.iter().find(|c| c.id == *clause_id) {
                    triggered_clauses.push(clause_id.clone());

                    if self.is_breach(clause, event) {
                        let severity = self.calculate_breach_severity(clause, event);
                        breaches.push(BreachAnalysis {
                            party: self.identify_breaching_party(event),
                            clause_id: clause_id.clone(),
                            severity,
                            remedies: self.identify_remedies(clause),
                        });

                        let liability = self.estimate_liability(clause, severity);
                        *liabilities.entry(clause_id.clone()).or_insert(0.0) += liability;
                    }
                }
            }
        }

        let risk_level = if breaches.is_empty() {
            RiskLevel::Low
        } else if breaches.len() == 1 {
            RiskLevel::Medium
        } else if breaches.len() <= 3 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        };

        Ok(ScenarioResult {
            scenario: scenario.name.clone(),
            triggered_clauses,
            liabilities,
            breaches,
            risk_level,
        })
    }

    /// Checks if an event constitutes a breach.
    fn is_breach(&self, _clause: &SimContractClause, event: &ContractEvent) -> bool {
        event.event_type.contains("breach") || event.event_type.contains("violation")
    }

    /// Calculates breach severity.
    fn calculate_breach_severity(&self, clause: &SimContractClause, _event: &ContractEvent) -> f64 {
        match clause.clause_type {
            ClauseType::Payment => 0.8,
            ClauseType::Performance => 0.7,
            ClauseType::Confidentiality => 0.9,
            ClauseType::Liability => 0.6,
            ClauseType::Termination => 0.5,
            ClauseType::DisputeResolution => 0.3,
        }
    }

    /// Identifies the breaching party.
    fn identify_breaching_party(&self, event: &ContractEvent) -> String {
        if event.description.contains("Party A") {
            self.contract
                .parties
                .first()
                .unwrap_or(&"Unknown".to_string())
                .clone()
        } else {
            self.contract
                .parties
                .get(1)
                .unwrap_or(&"Unknown".to_string())
                .clone()
        }
    }

    /// Identifies available remedies.
    fn identify_remedies(&self, clause: &SimContractClause) -> Vec<String> {
        match clause.clause_type {
            ClauseType::Payment => vec!["Damages".to_string(), "Specific performance".to_string()],
            ClauseType::Performance => vec!["Cure period".to_string(), "Termination".to_string()],
            ClauseType::Confidentiality => {
                vec!["Injunction".to_string(), "Liquidated damages".to_string()]
            }
            _ => vec!["Negotiated settlement".to_string()],
        }
    }

    /// Estimates liability for a breach.
    fn estimate_liability(&self, _clause: &SimContractClause, severity: f64) -> f64 {
        100000.0 * severity
    }
}

// ============================================================================
// Regulatory Compliance Simulation
// ============================================================================

/// Regulatory compliance simulator
#[derive(Debug, Clone)]
pub struct ComplianceSimulator {
    /// Applicable regulations
    pub regulations: Vec<Regulation>,
    /// Business operations
    pub operations: Vec<BusinessOperation>,
}

/// Regulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regulation {
    /// Regulation ID
    pub id: String,
    /// Regulation name
    pub name: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Requirements
    pub requirements: Vec<ComplianceRequirement>,
    /// Penalties for non-compliance
    pub penalties: Vec<Penalty>,
}

/// Compliance requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    /// Requirement ID
    pub id: String,
    /// Description
    pub description: String,
    /// Severity if violated
    pub severity: f64,
    /// Applicable scenarios
    pub applicable_to: Vec<String>,
}

/// Penalty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    /// Penalty type
    pub penalty_type: PenaltyType,
    /// Amount (if monetary)
    pub amount: Option<f64>,
    /// Description
    pub description: String,
}

/// Penalty type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PenaltyType {
    /// Monetary fine
    Fine,
    /// License suspension
    Suspension,
    /// License revocation
    Revocation,
    /// Criminal prosecution
    Criminal,
}

/// Business operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessOperation {
    /// Operation ID
    pub id: String,
    /// Operation type
    pub operation_type: String,
    /// Description
    pub description: String,
    /// Current compliance status
    pub compliant: bool,
}

/// Compliance simulation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    /// Overall compliance score (0.0 - 1.0)
    pub compliance_score: f64,
    /// Violations detected
    pub violations: Vec<Violation>,
    /// Total potential penalties
    pub total_penalties: f64,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Regulation ID
    pub regulation_id: String,
    /// Requirement ID
    pub requirement_id: String,
    /// Violating operation
    pub operation_id: String,
    /// Severity (0.0 - 1.0)
    pub severity: f64,
    /// Applicable penalties
    pub penalties: Vec<Penalty>,
}

impl ComplianceSimulator {
    /// Creates a new compliance simulator.
    pub fn new() -> Self {
        Self {
            regulations: Vec::new(),
            operations: Vec::new(),
        }
    }

    /// Adds a regulation.
    pub fn add_regulation(&mut self, regulation: Regulation) {
        self.regulations.push(regulation);
    }

    /// Adds a business operation.
    pub fn add_operation(&mut self, operation: BusinessOperation) {
        self.operations.push(operation);
    }

    /// Simulates compliance check.
    pub fn simulate(&self) -> Result<ComplianceResult> {
        let mut violations = Vec::new();
        let mut total_penalties = 0.0;

        for regulation in &self.regulations {
            for requirement in &regulation.requirements {
                for operation in &self.operations {
                    if self.is_applicable(&requirement.applicable_to, &operation.operation_type)
                        && !operation.compliant
                    {
                        violations.push(Violation {
                            regulation_id: regulation.id.clone(),
                            requirement_id: requirement.id.clone(),
                            operation_id: operation.id.clone(),
                            severity: requirement.severity,
                            penalties: regulation.penalties.clone(),
                        });

                        for penalty in &regulation.penalties {
                            if let Some(amount) = penalty.amount {
                                total_penalties += amount;
                            }
                        }
                    }
                }
            }
        }

        let total_checks = self.regulations.len() * self.operations.len();
        let compliance_score = if total_checks > 0 {
            1.0 - (violations.len() as f64 / total_checks as f64)
        } else {
            1.0
        };

        let recommendations = self.generate_recommendations(&violations);

        Ok(ComplianceResult {
            compliance_score,
            violations,
            total_penalties,
            recommendations,
        })
    }

    /// Checks if requirement applies to operation type.
    fn is_applicable(&self, applicable_to: &[String], operation_type: &str) -> bool {
        applicable_to.is_empty() || applicable_to.iter().any(|t| t == operation_type)
    }

    /// Generates compliance recommendations.
    fn generate_recommendations(&self, violations: &[Violation]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if violations.is_empty() {
            recommendations.push("All operations are compliant".to_string());
        } else {
            recommendations.push(format!(
                "Address {} compliance violations",
                violations.len()
            ));

            let high_severity: Vec<_> = violations.iter().filter(|v| v.severity > 0.7).collect();
            if !high_severity.is_empty() {
                recommendations.push(format!(
                    "Prioritize {} high-severity violations",
                    high_severity.len()
                ));
            }
        }

        recommendations
    }
}

impl Default for ComplianceSimulator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// What-If Analysis
// ============================================================================

/// What-if analysis engine
#[derive(Debug, Clone)]
pub struct WhatIfAnalyzer {
    /// Base scenario
    pub base_scenario: LegalScenario,
    /// Alternative scenarios
    pub alternatives: Vec<LegalScenario>,
}

/// Legal scenario for what-if analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalScenario {
    /// Scenario ID
    pub id: String,
    /// Scenario name
    pub name: String,
    /// Scenario description
    pub description: String,
    /// Input variables
    pub variables: HashMap<String, f64>,
    /// Expected outcome
    pub expected_outcome: Option<String>,
}

/// What-if analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfResult {
    /// Base scenario result
    pub base_result: ScenarioOutcome,
    /// Alternative scenario results
    pub alternative_results: Vec<ScenarioOutcome>,
    /// Comparison analysis
    pub comparison: Vec<Comparison>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Scenario outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioOutcome {
    /// Scenario ID
    pub scenario_id: String,
    /// Outcome description
    pub outcome: String,
    /// Success probability
    pub success_probability: f64,
    /// Estimated value
    pub estimated_value: f64,
    /// Risk score
    pub risk_score: f64,
}

/// Comparison between scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    /// Scenario A ID
    pub scenario_a: String,
    /// Scenario B ID
    pub scenario_b: String,
    /// Difference in success probability
    pub probability_diff: f64,
    /// Difference in value
    pub value_diff: f64,
    /// Better scenario
    pub recommendation: String,
}

impl WhatIfAnalyzer {
    /// Creates a new what-if analyzer.
    pub fn new(base_scenario: LegalScenario) -> Self {
        Self {
            base_scenario,
            alternatives: Vec::new(),
        }
    }

    /// Adds an alternative scenario.
    pub fn add_alternative(&mut self, scenario: LegalScenario) {
        self.alternatives.push(scenario);
    }

    /// Runs what-if analysis.
    pub fn analyze(&self) -> Result<WhatIfResult> {
        let base_result = self.evaluate_scenario(&self.base_scenario)?;

        let mut alternative_results = Vec::new();
        for alt in &self.alternatives {
            alternative_results.push(self.evaluate_scenario(alt)?);
        }

        let comparison = self.compare_scenarios(&base_result, &alternative_results);
        let recommendations = self.generate_recommendations(&base_result, &alternative_results);

        Ok(WhatIfResult {
            base_result,
            alternative_results,
            comparison,
            recommendations,
        })
    }

    /// Evaluates a scenario.
    fn evaluate_scenario(&self, scenario: &LegalScenario) -> Result<ScenarioOutcome> {
        let mut rng = rand::rng();

        let base_probability = scenario
            .variables
            .get("win_probability")
            .copied()
            .unwrap_or(0.5);
        let evidence_strength = scenario
            .variables
            .get("evidence_strength")
            .copied()
            .unwrap_or(0.5);
        let legal_cost = scenario
            .variables
            .get("legal_cost")
            .copied()
            .unwrap_or(100000.0);

        let success_probability = (base_probability + evidence_strength) / 2.0;
        let estimated_value = scenario
            .variables
            .get("claimed_damages")
            .copied()
            .unwrap_or(0.0)
            * success_probability
            - legal_cost;
        let risk_score = 1.0 - success_probability;

        let noise: f64 = rng.random_range(-0.05..0.05);
        let adjusted_probability = (success_probability + noise).clamp(0.0, 1.0);

        Ok(ScenarioOutcome {
            scenario_id: scenario.id.clone(),
            outcome: format!(
                "Scenario: {} - Probability: {:.1}%",
                scenario.name,
                adjusted_probability * 100.0
            ),
            success_probability: adjusted_probability,
            estimated_value,
            risk_score,
        })
    }

    /// Compares scenarios.
    fn compare_scenarios(
        &self,
        base: &ScenarioOutcome,
        alternatives: &[ScenarioOutcome],
    ) -> Vec<Comparison> {
        let mut comparisons = Vec::new();

        for alt in alternatives {
            let probability_diff = alt.success_probability - base.success_probability;
            let value_diff = alt.estimated_value - base.estimated_value;

            let recommendation = if value_diff > 0.0 && probability_diff > 0.0 {
                format!("{} is superior to base scenario", alt.scenario_id)
            } else if value_diff > 0.0 {
                format!("{} has higher value but lower probability", alt.scenario_id)
            } else if probability_diff > 0.0 {
                format!("{} has higher probability but lower value", alt.scenario_id)
            } else {
                "Base scenario is preferable".to_string()
            };

            comparisons.push(Comparison {
                scenario_a: base.scenario_id.clone(),
                scenario_b: alt.scenario_id.clone(),
                probability_diff,
                value_diff,
                recommendation,
            });
        }

        comparisons
    }

    /// Generates recommendations based on analysis.
    fn generate_recommendations(
        &self,
        base: &ScenarioOutcome,
        alternatives: &[ScenarioOutcome],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        let best_probability = alternatives
            .iter()
            .map(|a| a.success_probability)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(base.success_probability);

        let best_value = alternatives
            .iter()
            .map(|a| a.estimated_value)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(base.estimated_value);

        if best_probability > base.success_probability {
            recommendations.push(format!(
                "Consider alternative scenarios for {:.1}% higher success rate",
                (best_probability - base.success_probability) * 100.0
            ));
        }

        if best_value > base.estimated_value {
            recommendations.push(format!(
                "Alternative scenarios could increase value by ${:.0}",
                best_value - base.estimated_value
            ));
        }

        if base.risk_score > 0.5 {
            recommendations
                .push("Current scenario has high risk - explore alternatives".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Base scenario is optimal".to_string());
        }

        recommendations
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_outcome_predictor() {
        let mut predictor = CaseOutcomePredictor::new();

        let historical = HistoricalCase {
            id: "case1".to_string(),
            features: CaseFeatures {
                evidence_strength: 0.8,
                precedent_alignment: 0.7,
                legal_representation: 0.6,
                jurisdiction_bias: 0.0,
                case_complexity: 0.5,
                public_sentiment: 0.3,
                case_type: "tort".to_string(),
                jurisdiction: "CA".to_string(),
            },
            outcome: Verdict::PlaintiffWin,
            damages_awarded: Some(500000.0),
            year: 2023,
        };

        predictor.add_historical_case(historical);

        let case = CaseFeatures {
            evidence_strength: 0.75,
            precedent_alignment: 0.65,
            legal_representation: 0.7,
            jurisdiction_bias: 0.1,
            case_complexity: 0.4,
            public_sentiment: 0.4,
            case_type: "tort".to_string(),
            jurisdiction: "CA".to_string(),
        };

        let outcome = predictor.predict(&case).unwrap();
        assert!(outcome.confidence >= 0.0 && outcome.confidence <= 1.0);
        assert!(outcome.plaintiff_win_probability + outcome.defendant_win_probability <= 1.1);
    }

    #[test]
    fn test_litigation_risk_assessor() {
        let assessor = LitigationRiskAssessor::new();

        let case = CaseFeatures {
            evidence_strength: 0.3,
            precedent_alignment: 0.4,
            legal_representation: 0.5,
            jurisdiction_bias: -0.2,
            case_complexity: 0.8,
            public_sentiment: -0.1,
            case_type: "contract".to_string(),
            jurisdiction: "NY".to_string(),
        };

        let factors = vec![RiskFactor {
            name: "Weak evidence".to_string(),
            impact: 0.8,
            probability: 0.9,
            description: "Evidence is circumstantial".to_string(),
        }];

        let risk = assessor.assess(&case, &factors).unwrap();
        assert!(risk.risk_score >= 0.0 && risk.risk_score <= 1.0);
        assert!(risk.estimated_costs.expected_cost > 0.0);
    }

    #[test]
    fn test_settlement_estimator() {
        let estimator = SettlementEstimator::new();

        let case = CaseFeatures {
            evidence_strength: 0.7,
            precedent_alignment: 0.6,
            legal_representation: 0.8,
            jurisdiction_bias: 0.1,
            case_complexity: 0.5,
            public_sentiment: 0.2,
            case_type: "personal_injury".to_string(),
            jurisdiction: "TX".to_string(),
        };

        let estimate = estimator.estimate(&case, 1000000.0, 0.6).unwrap();
        assert!(estimate.expected_value > 0.0);
        assert!(estimate.min_value < estimate.expected_value);
        assert!(estimate.expected_value < estimate.max_value);
    }

    #[test]
    fn test_judge_behavior_model() {
        let model = JudgeBehaviorModel::new(DecisionMaker::Jury);

        let case = CaseFeatures {
            evidence_strength: 0.6,
            precedent_alignment: 0.5,
            legal_representation: 0.7,
            jurisdiction_bias: 0.0,
            case_complexity: 0.4,
            public_sentiment: 0.8,
            case_type: "civil_rights".to_string(),
            jurisdiction: "CA".to_string(),
        };

        let prediction = model.predict_behavior(&case).unwrap();
        assert_eq!(prediction.decision_maker, DecisionMaker::Jury);
        assert!(
            prediction.plaintiff_win_probability >= 0.0
                && prediction.plaintiff_win_probability <= 1.0
        );
    }

    #[test]
    fn test_negotiation_simulation() {
        let parties = vec![
            NegotiationParty {
                name: "Plaintiff".to_string(),
                reservation_value: 200000.0,
                target_value: 500000.0,
                strategy: NegotiationStrategy::Competitive,
                concession_rate: 0.3,
            },
            NegotiationParty {
                name: "Defendant".to_string(),
                reservation_value: 100000.0,
                target_value: 250000.0,
                strategy: NegotiationStrategy::Collaborative,
                concession_rate: 0.4,
            },
        ];

        let mut sim = NegotiationSimulation::new(parties);
        let outcome = sim.run_full_simulation(20).unwrap();

        match outcome {
            RoundOutcome::Agreement { value } => assert!(value > 0.0),
            RoundOutcome::Failed => {}
            RoundOutcome::Continued => {}
        }
    }

    #[test]
    fn test_contract_scenario_simulator() {
        let contract = ContractTerms {
            id: "contract1".to_string(),
            contract_type: "Service Agreement".to_string(),
            parties: vec!["Company A".to_string(), "Company B".to_string()],
            clauses: vec![SimContractClause {
                id: "clause1".to_string(),
                clause_type: ClauseType::Payment,
                text: "Payment due within 30 days".to_string(),
                conditions: vec![],
            }],
            duration_days: 365,
        };

        let scenario = ContractScenario {
            name: "Late Payment".to_string(),
            description: "Party A fails to pay on time".to_string(),
            events: vec![ContractEvent {
                event_type: "payment_breach".to_string(),
                day: 45,
                description: "Party A misses payment deadline".to_string(),
                affected_clauses: vec!["clause1".to_string()],
            }],
        };

        let simulator = ContractScenarioSimulator::new(contract);
        let result = simulator.simulate_scenario(&scenario).unwrap();

        assert!(!result.triggered_clauses.is_empty());
    }

    #[test]
    fn test_compliance_simulator() {
        let mut simulator = ComplianceSimulator::new();

        let regulation = Regulation {
            id: "reg1".to_string(),
            name: "Data Protection Regulation".to_string(),
            jurisdiction: "EU".to_string(),
            requirements: vec![ComplianceRequirement {
                id: "req1".to_string(),
                description: "Obtain user consent".to_string(),
                severity: 0.9,
                applicable_to: vec!["data_processing".to_string()],
            }],
            penalties: vec![Penalty {
                penalty_type: PenaltyType::Fine,
                amount: Some(100000.0),
                description: "GDPR violation".to_string(),
            }],
        };

        let operation = BusinessOperation {
            id: "op1".to_string(),
            operation_type: "data_processing".to_string(),
            description: "User data collection".to_string(),
            compliant: false,
        };

        simulator.add_regulation(regulation);
        simulator.add_operation(operation);

        let result = simulator.simulate().unwrap();
        assert!(!result.violations.is_empty());
        assert!(result.compliance_score < 1.0);
    }

    #[test]
    fn test_what_if_analyzer() {
        let mut vars = HashMap::new();
        vars.insert("win_probability".to_string(), 0.6);
        vars.insert("evidence_strength".to_string(), 0.7);
        vars.insert("claimed_damages".to_string(), 500000.0);
        vars.insert("legal_cost".to_string(), 100000.0);

        let base = LegalScenario {
            id: "base".to_string(),
            name: "Current Strategy".to_string(),
            description: "Proceed with litigation".to_string(),
            variables: vars.clone(),
            expected_outcome: None,
        };

        let mut analyzer = WhatIfAnalyzer::new(base);

        let mut alt_vars = vars;
        alt_vars.insert("win_probability".to_string(), 0.8);
        let alternative = LegalScenario {
            id: "alt1".to_string(),
            name: "Strengthen Evidence".to_string(),
            description: "Gather more evidence before trial".to_string(),
            variables: alt_vars,
            expected_outcome: None,
        };

        analyzer.add_alternative(alternative);

        let result = analyzer.analyze().unwrap();
        assert!(!result.alternative_results.is_empty());
        assert!(!result.comparison.is_empty());
    }
}
