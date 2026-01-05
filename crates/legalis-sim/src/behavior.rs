//! Agent behavior modeling for legal simulations.
//!
//! This module provides:
//! - Decision models for agent behavior
//! - Compliance probability modeling
//! - Evasion behavior simulation
//! - Learning and adaptation mechanisms

use chrono::NaiveDate;
use legalis_core::{Effect, LegalEntity, LegalResult, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Decision-making strategy for agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecisionStrategy {
    /// Fully rational actor (always complies if beneficial)
    Rational,
    /// Bounded rationality (limited information, cognitive constraints)
    BoundedRational,
    /// Rule-following (always complies regardless of benefit)
    RuleFollowing,
    /// Opportunistic (complies only when enforcement is certain)
    Opportunistic,
    /// Random decision making
    Random,
}

/// Behavioral parameters for an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralProfile {
    /// Agent's decision strategy
    pub strategy: DecisionStrategy,
    /// Base compliance probability (0.0 to 1.0)
    pub base_compliance: f64,
    /// Risk aversion coefficient (0.0 = risk-neutral, 1.0 = highly risk-averse)
    pub risk_aversion: f64,
    /// Time discount rate (how much they value future vs present)
    pub discount_rate: f64,
    /// Knowledge level (0.0 = unaware, 1.0 = perfect knowledge)
    pub knowledge_level: f64,
    /// Learning rate (how fast they adapt from experience)
    pub learning_rate: f64,
    /// Social influence sensitivity (0.0 = independent, 1.0 = conformist)
    pub social_influence: f64,
}

impl Default for BehavioralProfile {
    fn default() -> Self {
        Self {
            strategy: DecisionStrategy::BoundedRational,
            base_compliance: 0.8,
            risk_aversion: 0.5,
            discount_rate: 0.03,
            knowledge_level: 0.7,
            learning_rate: 0.1,
            social_influence: 0.4,
        }
    }
}

impl BehavioralProfile {
    /// Creates a new behavioral profile.
    pub fn new(strategy: DecisionStrategy) -> Self {
        Self {
            strategy,
            ..Default::default()
        }
    }

    /// Creates a rational agent profile.
    pub fn rational() -> Self {
        Self {
            strategy: DecisionStrategy::Rational,
            base_compliance: 1.0,
            risk_aversion: 0.0,
            knowledge_level: 1.0,
            learning_rate: 0.0,
            social_influence: 0.0,
            ..Default::default()
        }
    }

    /// Creates a rule-following agent profile.
    pub fn rule_following() -> Self {
        Self {
            strategy: DecisionStrategy::RuleFollowing,
            base_compliance: 1.0,
            risk_aversion: 1.0,
            knowledge_level: 0.8,
            learning_rate: 0.05,
            social_influence: 0.3,
            ..Default::default()
        }
    }

    /// Creates an opportunistic agent profile.
    pub fn opportunistic() -> Self {
        Self {
            strategy: DecisionStrategy::Opportunistic,
            base_compliance: 0.3,
            risk_aversion: 0.2,
            knowledge_level: 0.9,
            learning_rate: 0.15,
            social_influence: 0.1,
            ..Default::default()
        }
    }
}

/// Compliance decision result.
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceDecision {
    /// Agent chooses to comply
    Comply,
    /// Agent chooses not to comply
    Evade,
    /// Agent is unaware of the requirement
    Unaware,
    /// Agent is uncertain and seeks guidance
    SeekGuidance,
}

/// Context for a compliance decision.
#[derive(Debug, Clone)]
pub struct ComplianceContext {
    /// The statute being evaluated
    pub statute_id: String,
    /// Legal result if agent were to comply
    pub legal_result: LegalResult<Effect>,
    /// Perceived enforcement probability (0.0 to 1.0)
    pub enforcement_probability: f64,
    /// Penalty if caught not complying
    pub penalty_severity: f64,
    /// Benefit of non-compliance
    pub evasion_benefit: f64,
    /// Cost of compliance
    pub compliance_cost: f64,
    /// Social norm (what proportion of others comply)
    pub social_norm: f64,
}

/// Agent decision model for compliance.
pub struct ComplianceModel {
    profile: BehavioralProfile,
    /// History of past decisions (statute_id -> (complied, outcome))
    decision_history: HashMap<String, Vec<(bool, f64)>>,
    /// Accumulated experience weight
    experience: f64,
}

impl ComplianceModel {
    /// Creates a new compliance model.
    pub fn new(profile: BehavioralProfile) -> Self {
        Self {
            profile,
            decision_history: HashMap::new(),
            experience: 0.0,
        }
    }

    /// Decides whether to comply with a statute.
    pub fn decide(&mut self, context: &ComplianceContext) -> ComplianceDecision {
        // Check awareness first
        if !self.is_aware(&context.statute_id) {
            return ComplianceDecision::Unaware;
        }

        match self.profile.strategy {
            DecisionStrategy::Rational => self.rational_decision(context),
            DecisionStrategy::BoundedRational => self.bounded_rational_decision(context),
            DecisionStrategy::RuleFollowing => self.rule_following_decision(context),
            DecisionStrategy::Opportunistic => self.opportunistic_decision(context),
            DecisionStrategy::Random => self.random_decision(),
        }
    }

    /// Records the outcome of a compliance decision.
    pub fn record_outcome(&mut self, statute_id: &str, complied: bool, outcome: f64) {
        self.decision_history
            .entry(statute_id.to_string())
            .or_default()
            .push((complied, outcome));
        self.experience += self.profile.learning_rate;
    }

    /// Checks if agent is aware of a statute.
    fn is_aware(&self, statute_id: &str) -> bool {
        // Knowledge level determines awareness
        // If agent has seen this statute before, they're more likely to be aware
        let has_history = self.decision_history.contains_key(statute_id);
        let base_awareness = self.profile.knowledge_level;
        let awareness_prob = if has_history {
            (base_awareness + 0.2).min(1.0)
        } else {
            base_awareness
        };

        simple_random() < awareness_prob
    }

    /// Rational decision: maximize expected utility.
    fn rational_decision(&self, context: &ComplianceContext) -> ComplianceDecision {
        let comply_utility = -context.compliance_cost;
        let evade_utility =
            context.evasion_benefit - (context.enforcement_probability * context.penalty_severity);

        if comply_utility >= evade_utility {
            ComplianceDecision::Comply
        } else {
            ComplianceDecision::Evade
        }
    }

    /// Bounded rational decision: imperfect calculation with noise.
    fn bounded_rational_decision(&self, context: &ComplianceContext) -> ComplianceDecision {
        // Add noise to perception based on knowledge level
        let noise_factor = 1.0 - self.profile.knowledge_level;
        let perceived_enforcement =
            context.enforcement_probability * (1.0 + (simple_random() - 0.5) * noise_factor);
        let perceived_penalty =
            context.penalty_severity * (1.0 + (simple_random() - 0.5) * noise_factor);

        // Risk aversion affects how penalties are weighted
        let risk_adjusted_penalty = perceived_penalty * (1.0 + self.profile.risk_aversion);

        // Social influence affects decision
        let social_pressure = context.social_norm * self.profile.social_influence;

        let comply_utility = -context.compliance_cost + social_pressure * 10.0;
        let evade_utility = context.evasion_benefit
            - (perceived_enforcement.clamp(0.0, 1.0) * risk_adjusted_penalty);

        // Add base compliance bias
        let compliance_bias = self.profile.base_compliance * 5.0;

        if comply_utility + compliance_bias >= evade_utility {
            ComplianceDecision::Comply
        } else {
            ComplianceDecision::Evade
        }
    }

    /// Rule-following decision: high compliance regardless of utility.
    fn rule_following_decision(&self, context: &ComplianceContext) -> ComplianceDecision {
        // Rule followers comply unless the cost is extremely high
        if context.compliance_cost > 100.0 && context.social_norm < 0.3 {
            ComplianceDecision::SeekGuidance
        } else {
            ComplianceDecision::Comply
        }
    }

    /// Opportunistic decision: comply only if enforcement is high.
    fn opportunistic_decision(&self, context: &ComplianceContext) -> ComplianceDecision {
        let threshold = 0.5 - (self.profile.risk_aversion * 0.3);
        if context.enforcement_probability > threshold {
            ComplianceDecision::Comply
        } else {
            ComplianceDecision::Evade
        }
    }

    /// Random decision with bias toward base compliance.
    fn random_decision(&self) -> ComplianceDecision {
        if simple_random() < self.profile.base_compliance {
            ComplianceDecision::Comply
        } else {
            ComplianceDecision::Evade
        }
    }

    /// Calculates compliance probability for a given context.
    pub fn compliance_probability(&self, context: &ComplianceContext) -> f64 {
        // Run simulation many times to estimate probability
        let mut comply_count = 0;
        let iterations = 100;

        for _ in 0..iterations {
            let mut temp_model = self.clone();
            if matches!(
                temp_model.decide(context),
                ComplianceDecision::Comply | ComplianceDecision::SeekGuidance
            ) {
                comply_count += 1;
            }
        }

        comply_count as f64 / iterations as f64
    }
}

impl Clone for ComplianceModel {
    fn clone(&self) -> Self {
        Self {
            profile: self.profile.clone(),
            decision_history: self.decision_history.clone(),
            experience: self.experience,
        }
    }
}

/// Agent with behavioral modeling.
pub struct BehavioralAgent {
    id: Uuid,
    entity: Box<dyn LegalEntity>,
    compliance_model: ComplianceModel,
    interaction_history: Vec<(NaiveDate, String, ComplianceDecision)>,
}

impl BehavioralAgent {
    /// Creates a new behavioral agent.
    pub fn new(entity: Box<dyn LegalEntity>, profile: BehavioralProfile) -> Self {
        Self {
            id: entity.id(),
            entity,
            compliance_model: ComplianceModel::new(profile),
            interaction_history: Vec::new(),
        }
    }

    /// Gets the agent's ID.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Gets a reference to the underlying entity.
    pub fn entity(&self) -> &dyn LegalEntity {
        self.entity.as_ref()
    }

    /// Makes a compliance decision for a statute.
    pub fn decide_compliance(
        &mut self,
        statute: &Statute,
        context: ComplianceContext,
        date: NaiveDate,
    ) -> ComplianceDecision {
        let decision = self.compliance_model.decide(&context);
        self.interaction_history
            .push((date, statute.id.clone(), decision.clone()));
        decision
    }

    /// Records an outcome for learning.
    pub fn learn_from_outcome(&mut self, statute_id: &str, complied: bool, outcome: f64) {
        self.compliance_model
            .record_outcome(statute_id, complied, outcome);
    }

    /// Gets compliance history.
    pub fn history(&self) -> &[(NaiveDate, String, ComplianceDecision)] {
        &self.interaction_history
    }

    /// Gets the behavioral profile.
    pub fn profile(&self) -> &BehavioralProfile {
        &self.compliance_model.profile
    }
}

/// Population-level compliance statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComplianceStats {
    /// Statute ID
    pub statute_id: String,
    /// Total agents evaluated
    pub total_agents: usize,
    /// Number who complied
    pub complied: usize,
    /// Number who evaded
    pub evaded: usize,
    /// Number who were unaware
    pub unaware: usize,
    /// Number who sought guidance
    pub sought_guidance: usize,
    /// Average compliance probability
    pub avg_compliance_prob: f64,
}

impl ComplianceStats {
    /// Creates new compliance stats.
    pub fn new(statute_id: String) -> Self {
        Self {
            statute_id,
            ..Default::default()
        }
    }

    /// Records a decision.
    pub fn record(&mut self, decision: &ComplianceDecision, probability: f64) {
        self.total_agents += 1;
        self.avg_compliance_prob = (self.avg_compliance_prob * (self.total_agents - 1) as f64
            + probability)
            / self.total_agents as f64;

        match decision {
            ComplianceDecision::Comply => self.complied += 1,
            ComplianceDecision::Evade => self.evaded += 1,
            ComplianceDecision::Unaware => self.unaware += 1,
            ComplianceDecision::SeekGuidance => self.sought_guidance += 1,
        }
    }

    /// Gets the compliance rate.
    pub fn compliance_rate(&self) -> f64 {
        if self.total_agents == 0 {
            0.0
        } else {
            self.complied as f64 / self.total_agents as f64
        }
    }

    /// Gets the evasion rate.
    pub fn evasion_rate(&self) -> f64 {
        if self.total_agents == 0 {
            0.0
        } else {
            self.evaded as f64 / self.total_agents as f64
        }
    }

    /// Generates a summary report.
    pub fn summary(&self) -> String {
        format!(
            "Statute {}: Compliance={:.1}% ({}/{}), Evasion={:.1}% ({}/{}), Unaware={} Avg P(comply)={:.2}",
            self.statute_id,
            self.compliance_rate() * 100.0,
            self.complied,
            self.total_agents,
            self.evasion_rate() * 100.0,
            self.evaded,
            self.total_agents,
            self.unaware,
            self.avg_compliance_prob
        )
    }
}

// Shared random seed for reproducibility
static mut BEHAVIOR_RNG_SEED: u64 = 0;

/// Simple random number generator (0.0 to 1.0).
fn simple_random() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    unsafe {
        if BEHAVIOR_RNG_SEED == 0 {
            BEHAVIOR_RNG_SEED = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
        }
        BEHAVIOR_RNG_SEED = BEHAVIOR_RNG_SEED
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (BEHAVIOR_RNG_SEED >> 33) as f64 / (1u64 << 31) as f64
    }
}

/// Sets the random seed for reproducible behavior tests.
///
/// # Safety
/// This function is unsafe because it modifies a static mutable variable.
/// It should only be called in single-threaded test contexts.
pub unsafe fn set_behavior_seed(seed: u64) {
    unsafe {
        BEHAVIOR_RNG_SEED = seed;
    }
}

/// Type of message agents can communicate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Information about a statute
    StatuteInfo {
        statute_id: String,
        compliance_recommended: bool,
        reason: String,
    },
    /// Share compliance experience
    ComplianceExperience {
        statute_id: String,
        complied: bool,
        outcome: f64,
    },
    /// Alert about enforcement activity
    EnforcementAlert {
        statute_id: String,
        enforcement_level: f64,
        location: Option<String>,
    },
    /// Social norm information
    SocialNorm {
        statute_id: String,
        compliance_rate: f64,
        peer_count: usize,
    },
    /// General advice
    Advice { topic: String, content: String },
}

/// A message sent between agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Unique message ID
    pub id: Uuid,
    /// Sender agent ID
    pub sender: Uuid,
    /// Receiver agent ID (None for broadcast)
    pub receiver: Option<Uuid>,
    /// Message type and content
    pub message_type: MessageType,
    /// Timestamp
    pub timestamp: NaiveDate,
    /// Credibility of the message (0.0 to 1.0)
    pub credibility: f64,
}

impl AgentMessage {
    /// Creates a new message.
    pub fn new(
        sender: Uuid,
        receiver: Option<Uuid>,
        message_type: MessageType,
        timestamp: NaiveDate,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            receiver,
            message_type,
            timestamp,
            credibility: 0.8,
        }
    }

    /// Sets message credibility.
    pub fn with_credibility(mut self, credibility: f64) -> Self {
        self.credibility = credibility.clamp(0.0, 1.0);
        self
    }
}

/// Agent communication network.
pub struct CommunicationNetwork {
    /// Messages in the network
    messages: Vec<AgentMessage>,
    /// Network connections (agent_id -> list of connected agent_ids)
    connections: HashMap<Uuid, Vec<Uuid>>,
    /// Trust levels between agents (agent_id -> (trusted_id -> trust_level))
    trust: HashMap<Uuid, HashMap<Uuid, f64>>,
}

impl CommunicationNetwork {
    /// Creates a new communication network.
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            connections: HashMap::new(),
            trust: HashMap::new(),
        }
    }

    /// Adds a connection between two agents.
    pub fn connect(&mut self, agent1: Uuid, agent2: Uuid) {
        self.connections.entry(agent1).or_default().push(agent2);
        self.connections.entry(agent2).or_default().push(agent1);
    }

    /// Sets trust level from one agent to another.
    pub fn set_trust(&mut self, from: Uuid, to: Uuid, trust_level: f64) {
        self.trust
            .entry(from)
            .or_default()
            .insert(to, trust_level.clamp(0.0, 1.0));
    }

    /// Gets trust level from one agent to another.
    pub fn get_trust(&self, from: Uuid, to: Uuid) -> f64 {
        self.trust
            .get(&from)
            .and_then(|t| t.get(&to))
            .copied()
            .unwrap_or(0.5)
    }

    /// Sends a message from one agent to another (or broadcast).
    pub fn send_message(&mut self, message: AgentMessage) {
        self.messages.push(message);
    }

    /// Gets messages for a specific agent since a given date.
    pub fn get_messages_for(&self, agent_id: Uuid, since: NaiveDate) -> Vec<&AgentMessage> {
        self.messages
            .iter()
            .filter(|m| {
                m.timestamp >= since
                    && (m.receiver.is_none() || m.receiver == Some(agent_id))
                    && self.is_connected(agent_id, m.sender)
            })
            .collect()
    }

    /// Checks if two agents are connected.
    pub fn is_connected(&self, agent1: Uuid, agent2: Uuid) -> bool {
        self.connections
            .get(&agent1)
            .map(|conns| conns.contains(&agent2))
            .unwrap_or(false)
    }

    /// Gets all connections for an agent.
    pub fn get_connections(&self, agent_id: Uuid) -> Vec<Uuid> {
        self.connections.get(&agent_id).cloned().unwrap_or_default()
    }

    /// Processes messages to update agent behavior.
    pub fn process_messages_for_agent(
        &self,
        agent_id: Uuid,
        profile: &mut BehavioralProfile,
        since: NaiveDate,
    ) {
        let messages = self.get_messages_for(agent_id, since);

        for message in messages {
            let trust = self.get_trust(agent_id, message.sender);
            let influence = trust * message.credibility * profile.social_influence;

            match &message.message_type {
                MessageType::StatuteInfo {
                    compliance_recommended,
                    ..
                } => {
                    if *compliance_recommended {
                        profile.base_compliance += influence * 0.1;
                    } else {
                        profile.base_compliance -= influence * 0.1;
                    }
                    profile.knowledge_level += influence * 0.05;
                }
                MessageType::ComplianceExperience { complied, .. } => {
                    if *complied {
                        profile.base_compliance += influence * 0.05;
                    } else {
                        profile.base_compliance -= influence * 0.05;
                    }
                }
                MessageType::EnforcementAlert {
                    enforcement_level, ..
                } => {
                    if *enforcement_level > 0.7 {
                        profile.risk_aversion += influence * 0.1;
                    }
                }
                MessageType::SocialNorm {
                    compliance_rate, ..
                } => {
                    let target_compliance = *compliance_rate;
                    let diff = target_compliance - profile.base_compliance;
                    profile.base_compliance += diff * influence;
                }
                MessageType::Advice { .. } => {
                    profile.knowledge_level += influence * 0.02;
                }
            }

            profile.base_compliance = profile.base_compliance.clamp(0.0, 1.0);
            profile.knowledge_level = profile.knowledge_level.clamp(0.0, 1.0);
            profile.risk_aversion = profile.risk_aversion.clamp(0.0, 1.0);
        }
    }

    /// Clears old messages.
    pub fn clear_messages_before(&mut self, date: NaiveDate) {
        self.messages.retain(|m| m.timestamp >= date);
    }
}

impl Default for CommunicationNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{BasicEntity, EffectType};

    #[test]
    fn test_behavioral_profiles() {
        let rational = BehavioralProfile::rational();
        assert_eq!(rational.strategy, DecisionStrategy::Rational);
        assert_eq!(rational.base_compliance, 1.0);

        let opportunistic = BehavioralProfile::opportunistic();
        assert_eq!(opportunistic.strategy, DecisionStrategy::Opportunistic);
        assert!(opportunistic.base_compliance < 0.5);
    }

    #[test]
    fn test_compliance_decision_rational() {
        let profile = BehavioralProfile::rational();
        let mut model = ComplianceModel::new(profile);

        // High enforcement, high penalty -> comply
        let context1 = ComplianceContext {
            statute_id: "test".to_string(),
            legal_result: LegalResult::Deterministic(Effect::new(
                EffectType::Obligation,
                "Pay tax",
            )),
            enforcement_probability: 0.9,
            penalty_severity: 100.0,
            evasion_benefit: 10.0,
            compliance_cost: 5.0,
            social_norm: 0.8,
        };

        // Low enforcement, low penalty -> evade
        let context2 = ComplianceContext {
            statute_id: "test".to_string(),
            legal_result: LegalResult::Deterministic(Effect::new(
                EffectType::Obligation,
                "Pay tax",
            )),
            enforcement_probability: 0.1,
            penalty_severity: 10.0,
            evasion_benefit: 50.0,
            compliance_cost: 5.0,
            social_norm: 0.2,
        };

        // Note: Awareness is probabilistic, so we test multiple times
        let mut comply_count = 0;
        for _ in 0..10 {
            if matches!(model.decide(&context1), ComplianceDecision::Comply) {
                comply_count += 1;
            }
        }
        assert!(comply_count > 0); // Should comply at least sometimes

        let mut evade_count = 0;
        for _ in 0..10 {
            if matches!(model.decide(&context2), ComplianceDecision::Evade) {
                evade_count += 1;
            }
        }
        assert!(evade_count > 0); // Should evade at least sometimes
    }

    #[test]
    fn test_rule_following_compliance() {
        let profile = BehavioralProfile::rule_following();
        let mut model = ComplianceModel::new(profile);

        let context = ComplianceContext {
            statute_id: "test".to_string(),
            legal_result: LegalResult::Deterministic(Effect::new(
                EffectType::Obligation,
                "Follow rule",
            )),
            enforcement_probability: 0.01, // Very low enforcement
            penalty_severity: 1.0,         // Low penalty
            evasion_benefit: 100.0,        // High benefit to evade
            compliance_cost: 10.0,         // Moderate cost to comply
            social_norm: 0.9,              // Most people comply
        };

        // Rule followers should still comply despite low enforcement
        let mut comply_count = 0;
        for _ in 0..20 {
            if matches!(
                model.decide(&context),
                ComplianceDecision::Comply | ComplianceDecision::SeekGuidance
            ) {
                comply_count += 1;
            }
        }
        assert!(comply_count > 10); // Should comply most of the time
    }

    #[test]
    fn test_compliance_stats() {
        let mut stats = ComplianceStats::new("test-statute".to_string());

        stats.record(&ComplianceDecision::Comply, 0.9);
        stats.record(&ComplianceDecision::Comply, 0.8);
        stats.record(&ComplianceDecision::Evade, 0.3);
        stats.record(&ComplianceDecision::Unaware, 0.0);

        assert_eq!(stats.total_agents, 4);
        assert_eq!(stats.complied, 2);
        assert_eq!(stats.evaded, 1);
        assert_eq!(stats.unaware, 1);
        assert_eq!(stats.compliance_rate(), 0.5);
        assert_eq!(stats.evasion_rate(), 0.25);
    }

    #[test]
    fn test_behavioral_agent() {
        let entity = Box::new(BasicEntity::new()) as Box<dyn LegalEntity>;
        let profile = BehavioralProfile::rational();
        let mut agent = BehavioralAgent::new(entity, profile);

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Benefit"));

        let context = ComplianceContext {
            statute_id: "test".to_string(),
            legal_result: LegalResult::Deterministic(Effect::new(EffectType::Obligation, "Comply")),
            enforcement_probability: 0.8,
            penalty_severity: 50.0,
            evasion_benefit: 10.0,
            compliance_cost: 5.0,
            social_norm: 0.7,
        };

        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        agent.decide_compliance(&statute, context, date);

        assert_eq!(agent.history().len(), 1);
        assert_eq!(agent.history()[0].1, "test");
    }

    #[test]
    fn test_learning_from_outcomes() {
        let profile = BehavioralProfile::new(DecisionStrategy::BoundedRational);
        let mut model = ComplianceModel::new(profile);

        // Record several outcomes
        model.record_outcome("statute1", true, 10.0);
        model.record_outcome("statute1", true, 15.0);
        model.record_outcome("statute2", false, -50.0);

        assert!(model.experience > 0.0);
        assert_eq!(model.decision_history.len(), 2);
        assert_eq!(model.decision_history.get("statute1").unwrap().len(), 2);
    }
}
