//! Autonomous Legal Agents (v0.3.4)
//!
//! This module provides autonomous legal agent capabilities including:
//! - Autonomous negotiation agents with various strategies
//! - Multi-agent legal systems with communication protocols
//! - Agent-based compliance monitoring
//! - Legal chatbot framework
//! - Self-improving legal reasoning agents with reinforcement learning

use crate::{LegalEntity, Statute};

#[allow(unused)]
use crate::{Condition, Effect};
use std::collections::{HashMap, VecDeque};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ================================================================================================
// Feature 1: Autonomous Negotiation Agents
// ================================================================================================

/// Strategy for negotiation agents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NegotiationStrategy {
    /// Cooperative strategy - maximize joint utility
    Cooperative,
    /// Competitive strategy - maximize own utility
    Competitive,
    /// Mixed strategy - balance between cooperation and competition
    Mixed,
}

impl fmt::Display for NegotiationStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NegotiationStrategy::Cooperative => write!(f, "Cooperative"),
            NegotiationStrategy::Competitive => write!(f, "Competitive"),
            NegotiationStrategy::Mixed => write!(f, "Mixed"),
        }
    }
}

/// Proposal in a negotiation
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{Proposal, NegotiationAgent, NegotiationStrategy};
///
/// let agent = NegotiationAgent::new("agent1".to_string(), NegotiationStrategy::Cooperative);
/// let proposal = Proposal::new("proposal1".to_string(), "agent1".to_string(), 100.0);
///
/// assert_eq!(proposal.value(), 100.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Proposal {
    id: String,
    proposer: String,
    value: f64,
    terms: HashMap<String, String>,
}

impl Proposal {
    pub fn new(id: String, proposer: String, value: f64) -> Self {
        Self {
            id,
            proposer,
            value,
            terms: HashMap::new(),
        }
    }

    pub fn with_term(mut self, key: String, value: String) -> Self {
        self.terms.insert(key, value);
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn proposer(&self) -> &str {
        &self.proposer
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn terms(&self) -> &HashMap<String, String> {
        &self.terms
    }
}

/// Negotiation agent that can propose, accept, or reject offers
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{NegotiationAgent, NegotiationStrategy, Proposal};
///
/// let mut agent = NegotiationAgent::new("agent1".to_string(), NegotiationStrategy::Cooperative);
/// let proposal = Proposal::new("prop1".to_string(), "agent2".to_string(), 100.0);
///
/// // Cooperative agents are more likely to accept reasonable proposals
/// agent.set_reservation_value(50.0);
/// assert!(agent.evaluate_proposal(&proposal) > 0.0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NegotiationAgent {
    id: String,
    strategy: NegotiationStrategy,
    reservation_value: f64,
    aspiration_level: f64,
    proposals_made: Vec<Proposal>,
    proposals_received: Vec<Proposal>,
}

impl NegotiationAgent {
    pub fn new(id: String, strategy: NegotiationStrategy) -> Self {
        Self {
            id,
            strategy,
            reservation_value: 0.0,
            aspiration_level: 100.0,
            proposals_made: Vec::new(),
            proposals_received: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn strategy(&self) -> NegotiationStrategy {
        self.strategy
    }

    pub fn set_reservation_value(&mut self, value: f64) {
        self.reservation_value = value;
    }

    pub fn set_aspiration_level(&mut self, level: f64) {
        self.aspiration_level = level;
    }

    pub fn make_proposal(&mut self, value: f64) -> Proposal {
        let proposal = Proposal::new(
            format!("prop-{}-{}", self.id, self.proposals_made.len()),
            self.id.clone(),
            value,
        );
        self.proposals_made.push(proposal.clone());
        proposal
    }

    pub fn receive_proposal(&mut self, proposal: Proposal) {
        self.proposals_received.push(proposal);
    }

    /// Evaluate a proposal and return utility score
    pub fn evaluate_proposal(&self, proposal: &Proposal) -> f64 {
        match self.strategy {
            NegotiationStrategy::Cooperative => {
                // Cooperative: accept if above reservation value
                if proposal.value >= self.reservation_value {
                    proposal.value - self.reservation_value
                } else {
                    -1.0
                }
            }
            NegotiationStrategy::Competitive => {
                // Competitive: only accept if close to aspiration level
                if proposal.value >= self.aspiration_level * 0.9 {
                    proposal.value
                } else {
                    -1.0
                }
            }
            NegotiationStrategy::Mixed => {
                // Mixed: balance between cooperation and competition
                let coop_score = if proposal.value >= self.reservation_value {
                    proposal.value - self.reservation_value
                } else {
                    -1.0
                };
                let comp_score = if proposal.value >= self.aspiration_level * 0.8 {
                    proposal.value
                } else {
                    -1.0
                };
                (coop_score + comp_score) / 2.0
            }
        }
    }

    pub fn counter_proposal(&mut self, original: &Proposal) -> Option<Proposal> {
        let counter_value = match self.strategy {
            NegotiationStrategy::Cooperative => {
                // Offer midpoint
                (original.value + self.aspiration_level) / 2.0
            }
            NegotiationStrategy::Competitive => {
                // Offer slightly above reservation
                self.reservation_value * 1.1
            }
            NegotiationStrategy::Mixed => {
                // Offer weighted average
                original.value * 0.6 + self.aspiration_level * 0.4
            }
        };

        Some(self.make_proposal(counter_value))
    }
}

/// Multi-party negotiation coordinator
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{MultiPartyNegotiation, NegotiationAgent, NegotiationStrategy};
///
/// let mut negotiation = MultiPartyNegotiation::new("negotiation1".to_string());
/// let agent1 = NegotiationAgent::new("agent1".to_string(), NegotiationStrategy::Cooperative);
/// let agent2 = NegotiationAgent::new("agent2".to_string(), NegotiationStrategy::Competitive);
///
/// negotiation.add_agent(agent1);
/// negotiation.add_agent(agent2);
///
/// assert_eq!(negotiation.agent_count(), 2);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MultiPartyNegotiation {
    id: String,
    agents: Vec<NegotiationAgent>,
    proposals: Vec<Proposal>,
    agreement_reached: bool,
}

impl MultiPartyNegotiation {
    pub fn new(id: String) -> Self {
        Self {
            id,
            agents: Vec::new(),
            proposals: Vec::new(),
            agreement_reached: false,
        }
    }

    pub fn add_agent(&mut self, agent: NegotiationAgent) {
        self.agents.push(agent);
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn submit_proposal(&mut self, proposal: Proposal) {
        self.proposals.push(proposal);
    }

    /// Find Nash bargaining solution (simplified implementation)
    pub fn nash_bargaining_solution(&self) -> Option<f64> {
        if self.agents.is_empty() {
            return None;
        }

        // Nash solution maximizes product of utilities
        // Simplified: average of aspiration levels
        let sum: f64 = self.agents.iter().map(|a| a.aspiration_level).sum();
        Some(sum / self.agents.len() as f64)
    }

    /// Check if outcome is Pareto optimal
    pub fn is_pareto_optimal(&self, value: f64) -> bool {
        // Simplified: check if all agents benefit above reservation
        self.agents
            .iter()
            .all(|agent| value >= agent.reservation_value)
    }

    pub fn is_agreement_reached(&self) -> bool {
        self.agreement_reached
    }
}

// ================================================================================================
// Feature 2: Multi-Agent Legal Systems
// ================================================================================================

/// Agent communication message types (FIPA ACL compatible)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MessageType {
    /// Inform another agent of a fact
    Inform,
    /// Query another agent for information
    Query,
    /// Request an action from another agent
    Request,
    /// Propose an action or deal
    Propose,
    /// Accept a proposal
    Accept,
    /// Reject a proposal
    Reject,
    /// Confirm receipt and understanding
    Confirm,
    /// Notify failure or inability
    Failure,
}

/// Message in agent communication protocol
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{Message, MessageType};
///
/// let message = Message::new(
///     MessageType::Inform,
///     "agent1".to_string(),
///     "agent2".to_string(),
///     "compliance_status".to_string(),
///     "compliant".to_string(),
/// );
///
/// assert_eq!(message.message_type(), &MessageType::Inform);
/// assert_eq!(message.sender(), "agent1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message {
    message_type: MessageType,
    sender: String,
    receiver: String,
    subject: String,
    content: String,
    timestamp: i64,
}

impl Message {
    pub fn new(
        message_type: MessageType,
        sender: String,
        receiver: String,
        subject: String,
        content: String,
    ) -> Self {
        Self {
            message_type,
            sender,
            receiver,
            subject,
            content,
            timestamp: 0, // In real implementation, use actual timestamp
        }
    }

    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    pub fn sender(&self) -> &str {
        &self.sender
    }

    pub fn receiver(&self) -> &str {
        &self.receiver
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

/// Shared knowledge base for agent society
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::SharedKnowledgeBase;
///
/// let mut kb = SharedKnowledgeBase::new();
/// kb.add_fact("statute123".to_string(), "Tax rate is 20%");
/// kb.add_fact("regulation456".to_string(), "Filing deadline is April 15");
///
/// assert!(kb.has_fact("statute123"));
/// assert_eq!(kb.fact_count(), 2);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SharedKnowledgeBase {
    facts: HashMap<String, String>,
    rules: Vec<String>,
}

impl SharedKnowledgeBase {
    pub fn new() -> Self {
        Self {
            facts: HashMap::new(),
            rules: Vec::new(),
        }
    }

    pub fn add_fact(&mut self, key: String, value: &str) {
        self.facts.insert(key, value.to_string());
    }

    pub fn get_fact(&self, key: &str) -> Option<&String> {
        self.facts.get(key)
    }

    pub fn has_fact(&self, key: &str) -> bool {
        self.facts.contains_key(key)
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.push(rule);
    }

    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for SharedKnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal agent society for coordinating multiple agents
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{LegalAgentSociety, Message, MessageType};
///
/// let mut society = LegalAgentSociety::new("legal_system".to_string());
/// society.register_agent("compliance_agent");
/// society.register_agent("audit_agent");
///
/// let message = Message::new(
///     MessageType::Inform,
///     "compliance_agent".to_string(),
///     "audit_agent".to_string(),
///     "status".to_string(),
///     "all_clear".to_string(),
/// );
///
/// society.send_message(message);
/// assert_eq!(society.agent_count(), 2);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LegalAgentSociety {
    id: String,
    agents: Vec<String>,
    knowledge_base: SharedKnowledgeBase,
    message_queue: VecDeque<Message>,
}

impl LegalAgentSociety {
    pub fn new(id: String) -> Self {
        Self {
            id,
            agents: Vec::new(),
            knowledge_base: SharedKnowledgeBase::new(),
            message_queue: VecDeque::new(),
        }
    }

    pub fn register_agent(&mut self, agent_id: &str) {
        self.agents.push(agent_id.to_string());
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    pub fn send_message(&mut self, message: Message) {
        self.message_queue.push_back(message);
    }

    pub fn receive_messages(&mut self, agent_id: &str) -> Vec<Message> {
        let mut received = Vec::new();
        let mut remaining = VecDeque::new();

        while let Some(msg) = self.message_queue.pop_front() {
            if msg.receiver == agent_id {
                received.push(msg);
            } else {
                remaining.push_back(msg);
            }
        }

        self.message_queue = remaining;
        received
    }

    pub fn knowledge_base(&self) -> &SharedKnowledgeBase {
        &self.knowledge_base
    }

    pub fn knowledge_base_mut(&mut self) -> &mut SharedKnowledgeBase {
        &mut self.knowledge_base
    }

    /// Resolve conflicts between agents using voting or other mechanisms
    pub fn resolve_conflict(&self, _issue: &str) -> Option<String> {
        // Simplified: majority vote among agents
        // In real implementation, would have more sophisticated conflict resolution
        if !self.agents.is_empty() {
            Some(self.agents[0].clone())
        } else {
            None
        }
    }
}

// ================================================================================================
// Feature 3: Agent-Based Compliance Monitoring
// ================================================================================================

/// Compliance violation severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ViolationSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ViolationSeverity::Low => write!(f, "Low"),
            ViolationSeverity::Medium => write!(f, "Medium"),
            ViolationSeverity::High => write!(f, "High"),
            ViolationSeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Compliance violation detected by monitoring agent
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComplianceViolation {
    id: String,
    statute_id: String,
    description: String,
    severity: ViolationSeverity,
    detected_at: i64,
    suggestion: Option<String>,
}

impl ComplianceViolation {
    pub fn new(
        id: String,
        statute_id: String,
        description: String,
        severity: ViolationSeverity,
    ) -> Self {
        Self {
            id,
            statute_id,
            description,
            severity,
            detected_at: 0,
            suggestion: None,
        }
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn statute_id(&self) -> &str {
        &self.statute_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn severity(&self) -> ViolationSeverity {
        self.severity
    }

    pub fn suggestion(&self) -> Option<&String> {
        self.suggestion.as_ref()
    }
}

/// Compliance monitoring agent with learning capabilities
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{ComplianceMonitorAgent, ComplianceViolation, ViolationSeverity};
///
/// let mut agent = ComplianceMonitorAgent::new("monitor1".to_string());
/// let violation = ComplianceViolation::new(
///     "v1".to_string(),
///     "statute123".to_string(),
///     "Missing required field".to_string(),
///     ViolationSeverity::High,
/// );
///
/// agent.record_violation(violation);
/// assert_eq!(agent.violation_count(), 1);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComplianceMonitorAgent {
    id: String,
    violations: Vec<ComplianceViolation>,
    past_violations: HashMap<String, usize>, // statute_id -> count
    monitoring_rules: Vec<String>,
}

impl ComplianceMonitorAgent {
    pub fn new(id: String) -> Self {
        Self {
            id,
            violations: Vec::new(),
            past_violations: HashMap::new(),
            monitoring_rules: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn add_monitoring_rule(&mut self, rule: String) {
        self.monitoring_rules.push(rule);
    }

    pub fn record_violation(&mut self, violation: ComplianceViolation) {
        let statute_id = violation.statute_id.clone();
        *self.past_violations.entry(statute_id).or_insert(0) += 1;
        self.violations.push(violation);
    }

    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }

    pub fn violations(&self) -> &[ComplianceViolation] {
        &self.violations
    }

    /// Check for violations autonomously
    pub fn monitor<E: LegalEntity>(
        &mut self,
        _entity: &E,
        _statutes: &[Statute],
    ) -> Vec<ComplianceViolation> {
        // Simplified: In real implementation, would evaluate statutes against entity
        Vec::new()
    }

    /// Generate alert for violations above threshold
    pub fn generate_alerts(&self, min_severity: ViolationSeverity) -> Vec<String> {
        self.violations
            .iter()
            .filter(|v| v.severity >= min_severity)
            .map(|v| format!("ALERT: {} - {}", v.severity, v.description))
            .collect()
    }

    /// Self-healing: suggest fixes based on past patterns
    pub fn suggest_self_healing(&self, statute_id: &str) -> Option<String> {
        let count = self.past_violations.get(statute_id).unwrap_or(&0);

        if *count > 0 {
            Some(format!(
                "This statute has been violated {} time(s) before. Consider implementing automated checks.",
                count
            ))
        } else {
            None
        }
    }

    /// Learn from past violations to improve detection
    pub fn learn_from_violations(&mut self) {
        // Update monitoring rules based on patterns in past violations
        let frequent_violations: Vec<_> = self
            .past_violations
            .iter()
            .filter(|(_, count)| **count > 2)
            .map(|(statute_id, _)| statute_id.clone())
            .collect();

        for statute_id in frequent_violations {
            let rule = format!("Enhanced monitoring for {}", statute_id);
            if !self.monitoring_rules.contains(&rule) {
                self.monitoring_rules.push(rule);
            }
        }
    }
}

// ================================================================================================
// Feature 4: Legal Chatbot Framework
// ================================================================================================

/// Intent recognized from user query
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LegalIntent {
    /// Query about legal rights
    QueryRights,
    /// Query about obligations
    QueryObligations,
    /// Request for legal advice
    RequestAdvice,
    /// Question about compliance
    CheckCompliance,
    /// General information request
    GeneralInfo,
    /// Unknown intent
    Unknown,
}

impl fmt::Display for LegalIntent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LegalIntent::QueryRights => write!(f, "Query about legal rights"),
            LegalIntent::QueryObligations => write!(f, "Query about obligations"),
            LegalIntent::RequestAdvice => write!(f, "Request for legal advice"),
            LegalIntent::CheckCompliance => write!(f, "Compliance check"),
            LegalIntent::GeneralInfo => write!(f, "General information"),
            LegalIntent::Unknown => write!(f, "Unknown intent"),
        }
    }
}

/// Conversation turn in multi-turn dialogue
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConversationTurn {
    user_message: String,
    bot_response: String,
    intent: LegalIntent,
    timestamp: i64,
}

impl ConversationTurn {
    pub fn new(user_message: String, bot_response: String, intent: LegalIntent) -> Self {
        Self {
            user_message,
            bot_response,
            intent,
            timestamp: 0,
        }
    }

    pub fn user_message(&self) -> &str {
        &self.user_message
    }

    pub fn bot_response(&self) -> &str {
        &self.bot_response
    }

    pub fn intent(&self) -> &LegalIntent {
        &self.intent
    }
}

/// Legal chatbot trait for conversational interfaces
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{LegalChatbot, SimpleLegalChatbot, LegalIntent};
///
/// let mut chatbot = SimpleLegalChatbot::new("assistant".to_string());
/// let response = chatbot.process_query("What are my tax obligations?");
///
/// assert!(!response.is_empty());
/// assert_eq!(chatbot.conversation_length(), 1);
/// ```
pub trait LegalChatbot {
    /// Process user query and return response
    fn process_query(&mut self, query: &str) -> String;

    /// Recognize intent from query
    fn recognize_intent(&self, query: &str) -> LegalIntent;

    /// Generate context-aware response
    fn generate_response(&self, query: &str, intent: &LegalIntent) -> String;

    /// Get conversation history
    fn conversation_history(&self) -> &[ConversationTurn];

    /// Clear conversation history
    fn clear_conversation(&mut self);
}

/// Simple implementation of legal chatbot
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimpleLegalChatbot {
    id: String,
    conversation: Vec<ConversationTurn>,
}

impl SimpleLegalChatbot {
    pub fn new(id: String) -> Self {
        Self {
            id,
            conversation: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn conversation_length(&self) -> usize {
        self.conversation.len()
    }
}

impl LegalChatbot for SimpleLegalChatbot {
    fn process_query(&mut self, query: &str) -> String {
        let intent = self.recognize_intent(query);
        let response = self.generate_response(query, &intent);

        let turn = ConversationTurn::new(query.to_string(), response.clone(), intent);
        self.conversation.push(turn);

        response
    }

    fn recognize_intent(&self, query: &str) -> LegalIntent {
        let query_lower = query.to_lowercase();

        if query_lower.contains("right") || query_lower.contains("entitled") {
            LegalIntent::QueryRights
        } else if query_lower.contains("obligation")
            || query_lower.contains("must")
            || query_lower.contains("required")
        {
            LegalIntent::QueryObligations
        } else if query_lower.contains("advice") || query_lower.contains("should i") {
            LegalIntent::RequestAdvice
        } else if query_lower.contains("compliant") || query_lower.contains("comply") {
            LegalIntent::CheckCompliance
        } else if query_lower.contains("what")
            || query_lower.contains("how")
            || query_lower.contains("when")
        {
            LegalIntent::GeneralInfo
        } else {
            LegalIntent::Unknown
        }
    }

    fn generate_response(&self, _query: &str, intent: &LegalIntent) -> String {
        match intent {
            LegalIntent::QueryRights => {
                "Your rights depend on the specific statute and jurisdiction. Let me help you find the relevant information.".to_string()
            }
            LegalIntent::QueryObligations => {
                "Regarding your obligations, I can help you understand the applicable requirements. Could you provide more details?".to_string()
            }
            LegalIntent::RequestAdvice => {
                "I can provide general legal information. For specific advice, please consult a licensed attorney.".to_string()
            }
            LegalIntent::CheckCompliance => {
                "To check compliance, I'll need to review the relevant statutes and your current status.".to_string()
            }
            LegalIntent::GeneralInfo => {
                "I can provide general legal information. What specific topic are you interested in?".to_string()
            }
            LegalIntent::Unknown => {
                "I'm not sure I understand. Could you rephrase your question?".to_string()
            }
        }
    }

    fn conversation_history(&self) -> &[ConversationTurn] {
        &self.conversation
    }

    fn clear_conversation(&mut self) {
        self.conversation.clear();
    }
}

// ================================================================================================
// Feature 5: Self-Improving Legal Reasoning Agents
// ================================================================================================

/// State-action pair for reinforcement learning
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StateAction {
    state: Vec<f64>,
    action: usize,
}

impl StateAction {
    pub fn new(state: Vec<f64>, action: usize) -> Self {
        Self { state, action }
    }

    pub fn state(&self) -> &[f64] {
        &self.state
    }

    pub fn action(&self) -> usize {
        self.action
    }
}

/// Experience for replay buffer
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Experience {
    state: Vec<f64>,
    action: usize,
    reward: f64,
    next_state: Vec<f64>,
    done: bool,
}

impl Experience {
    pub fn new(
        state: Vec<f64>,
        action: usize,
        reward: f64,
        next_state: Vec<f64>,
        done: bool,
    ) -> Self {
        Self {
            state,
            action,
            reward,
            next_state,
            done,
        }
    }

    pub fn state(&self) -> &[f64] {
        &self.state
    }

    pub fn action(&self) -> usize {
        self.action
    }

    pub fn reward(&self) -> f64 {
        self.reward
    }

    pub fn next_state(&self) -> &[f64] {
        &self.next_state
    }

    pub fn is_done(&self) -> bool {
        self.done
    }
}

/// Experience replay buffer for reinforcement learning
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{ExperienceReplayBuffer, Experience};
///
/// let mut buffer = ExperienceReplayBuffer::new(100);
/// let exp = Experience::new(vec![1.0, 2.0], 0, 1.0, vec![1.5, 2.5], false);
///
/// buffer.add(exp);
/// assert_eq!(buffer.len(), 1);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExperienceReplayBuffer {
    buffer: VecDeque<Experience>,
    capacity: usize,
}

impl ExperienceReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn add(&mut self, experience: Experience) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(experience);
    }

    pub fn sample(&self, batch_size: usize) -> Vec<&Experience> {
        // Simplified: return last N experiences
        // In real implementation, would do random sampling
        self.buffer
            .iter()
            .rev()
            .take(batch_size.min(self.buffer.len()))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

/// Reinforcement learning agent for legal strategy optimization
///
/// # Examples
///
/// ```
/// use legalis_core::autonomous_agents::{ReinforcementLearningAgent, Experience};
///
/// let mut agent = ReinforcementLearningAgent::new("rl_agent".to_string(), 4, 10);
/// let exp = Experience::new(vec![1.0, 2.0], 0, 1.0, vec![1.5, 2.5], false);
///
/// agent.learn_from_experience(exp);
/// assert!(agent.experience_count() > 0);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReinforcementLearningAgent {
    id: String,
    q_table: HashMap<String, Vec<f64>>, // state_key -> action values
    replay_buffer: ExperienceReplayBuffer,
    learning_rate: f64,
    discount_factor: f64,
    epsilon: f64, // exploration rate
    num_actions: usize,
}

impl ReinforcementLearningAgent {
    pub fn new(id: String, num_actions: usize, buffer_capacity: usize) -> Self {
        Self {
            id,
            q_table: HashMap::new(),
            replay_buffer: ExperienceReplayBuffer::new(buffer_capacity),
            learning_rate: 0.1,
            discount_factor: 0.95,
            epsilon: 0.1,
            num_actions,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_learning_rate(&mut self, rate: f64) {
        self.learning_rate = rate;
    }

    pub fn set_discount_factor(&mut self, factor: f64) {
        self.discount_factor = factor;
    }

    pub fn set_epsilon(&mut self, epsilon: f64) {
        self.epsilon = epsilon;
    }

    fn state_key(state: &[f64]) -> String {
        state
            .iter()
            .map(|x| format!("{:.2}", x))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Choose action using epsilon-greedy policy
    pub fn choose_action(&self, state: &[f64]) -> usize {
        let key = Self::state_key(state);

        // Exploration: random action
        if self.epsilon > 0.5 {
            0 // Simplified: return first action
        } else {
            // Exploitation: best known action
            if let Some(q_values) = self.q_table.get(&key) {
                q_values
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(idx, _)| idx)
                    .unwrap_or(0)
            } else {
                0
            }
        }
    }

    /// Update Q-values using Q-learning
    pub fn learn_from_experience(&mut self, experience: Experience) {
        self.update_q_values(&experience);

        // Add to replay buffer
        self.replay_buffer.add(experience);
    }

    /// Update Q-values without adding to replay buffer
    fn update_q_values(&mut self, experience: &Experience) {
        let state_key = Self::state_key(&experience.state);
        let next_state_key = Self::state_key(&experience.next_state);

        // Get current Q-value
        let q_values = self
            .q_table
            .entry(state_key.clone())
            .or_insert_with(|| vec![0.0; self.num_actions]);
        let current_q = q_values[experience.action];

        // Get max Q-value for next state
        let next_q_values = self
            .q_table
            .entry(next_state_key)
            .or_insert_with(|| vec![0.0; self.num_actions]);
        let max_next_q = next_q_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        // Q-learning update: Q(s,a) = Q(s,a) + α[r + γ*max(Q(s',a')) - Q(s,a)]
        let target = experience.reward
            + if experience.done {
                0.0
            } else {
                self.discount_factor * max_next_q
            };
        let q_values = self.q_table.get_mut(&state_key).unwrap();
        q_values[experience.action] = current_q + self.learning_rate * (target - current_q);
    }

    /// Train on batch from replay buffer
    pub fn train_on_batch(&mut self, batch_size: usize) {
        let experiences: Vec<Experience> = self
            .replay_buffer
            .sample(batch_size)
            .into_iter()
            .cloned()
            .collect();

        for exp in experiences {
            self.update_q_values(&exp);
        }
    }

    pub fn experience_count(&self) -> usize {
        self.replay_buffer.len()
    }

    /// Self-evaluation: compute average Q-value
    pub fn self_evaluate(&self) -> f64 {
        if self.q_table.is_empty() {
            return 0.0;
        }

        let total: f64 = self.q_table.values().flat_map(|v| v.iter()).sum();
        let count = self.q_table.values().map(|v| v.len()).sum::<usize>();

        if count > 0 { total / count as f64 } else { 0.0 }
    }

    /// Policy gradient method (simplified)
    pub fn update_policy(&mut self, _state: &[f64], _action: usize, _advantage: f64) {
        // Simplified implementation
        // In real implementation, would update policy network parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negotiation_agent_cooperative() {
        let mut agent =
            NegotiationAgent::new("agent1".to_string(), NegotiationStrategy::Cooperative);
        agent.set_reservation_value(50.0);
        agent.set_aspiration_level(100.0);

        let proposal = Proposal::new("prop1".to_string(), "agent2".to_string(), 75.0);
        let score = agent.evaluate_proposal(&proposal);

        assert!(
            score > 0.0,
            "Cooperative agent should accept reasonable proposal"
        );
    }

    #[test]
    fn test_negotiation_agent_competitive() {
        let mut agent =
            NegotiationAgent::new("agent1".to_string(), NegotiationStrategy::Competitive);
        agent.set_reservation_value(50.0);
        agent.set_aspiration_level(100.0);

        let low_proposal = Proposal::new("prop1".to_string(), "agent2".to_string(), 60.0);
        let score = agent.evaluate_proposal(&low_proposal);

        assert!(score < 0.0, "Competitive agent should reject low proposals");
    }

    #[test]
    fn test_multi_party_negotiation() {
        let mut negotiation = MultiPartyNegotiation::new("neg1".to_string());

        let mut agent1 =
            NegotiationAgent::new("agent1".to_string(), NegotiationStrategy::Cooperative);
        agent1.set_aspiration_level(80.0);
        agent1.set_reservation_value(40.0);

        let mut agent2 =
            NegotiationAgent::new("agent2".to_string(), NegotiationStrategy::Cooperative);
        agent2.set_aspiration_level(90.0);
        agent2.set_reservation_value(50.0);

        negotiation.add_agent(agent1);
        negotiation.add_agent(agent2);

        let nash_solution = negotiation.nash_bargaining_solution();
        assert!(nash_solution.is_some());
        assert!(nash_solution.unwrap() > 0.0);
    }

    #[test]
    fn test_pareto_optimal() {
        let mut negotiation = MultiPartyNegotiation::new("neg1".to_string());

        let mut agent1 =
            NegotiationAgent::new("agent1".to_string(), NegotiationStrategy::Cooperative);
        agent1.set_reservation_value(40.0);

        let mut agent2 =
            NegotiationAgent::new("agent2".to_string(), NegotiationStrategy::Cooperative);
        agent2.set_reservation_value(50.0);

        negotiation.add_agent(agent1);
        negotiation.add_agent(agent2);

        assert!(negotiation.is_pareto_optimal(60.0));
        assert!(!negotiation.is_pareto_optimal(30.0));
    }

    #[test]
    fn test_message_passing() {
        let message = Message::new(
            MessageType::Inform,
            "agent1".to_string(),
            "agent2".to_string(),
            "status".to_string(),
            "compliant".to_string(),
        );

        assert_eq!(message.sender(), "agent1");
        assert_eq!(message.receiver(), "agent2");
        assert_eq!(message.message_type(), &MessageType::Inform);
    }

    #[test]
    fn test_legal_agent_society() {
        let mut society = LegalAgentSociety::new("society1".to_string());
        society.register_agent("agent1");
        society.register_agent("agent2");

        assert_eq!(society.agent_count(), 2);

        let message = Message::new(
            MessageType::Query,
            "agent1".to_string(),
            "agent2".to_string(),
            "compliance".to_string(),
            "check status".to_string(),
        );

        society.send_message(message);
        let messages = society.receive_messages("agent2");
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_shared_knowledge_base() {
        let mut kb = SharedKnowledgeBase::new();
        kb.add_fact("statute1".to_string(), "Tax rate is 20%");
        kb.add_rule("Always check compliance".to_string());

        assert!(kb.has_fact("statute1"));
        assert_eq!(kb.fact_count(), 1);
        assert_eq!(kb.rule_count(), 1);
    }

    #[test]
    fn test_compliance_monitor_agent() {
        let mut agent = ComplianceMonitorAgent::new("monitor1".to_string());

        let violation = ComplianceViolation::new(
            "v1".to_string(),
            "statute1".to_string(),
            "Missing field".to_string(),
            ViolationSeverity::High,
        )
        .with_suggestion("Add the missing field".to_string());

        agent.record_violation(violation);
        assert_eq!(agent.violation_count(), 1);

        let alerts = agent.generate_alerts(ViolationSeverity::High);
        assert_eq!(alerts.len(), 1);
    }

    #[test]
    fn test_compliance_self_healing() {
        let mut agent = ComplianceMonitorAgent::new("monitor1".to_string());

        let violation1 = ComplianceViolation::new(
            "v1".to_string(),
            "statute1".to_string(),
            "Missing field".to_string(),
            ViolationSeverity::Medium,
        );
        agent.record_violation(violation1);

        let suggestion = agent.suggest_self_healing("statute1");
        assert!(suggestion.is_some());
    }

    #[test]
    fn test_compliance_learning() {
        let mut agent = ComplianceMonitorAgent::new("monitor1".to_string());

        // Record multiple violations for same statute
        for i in 0..3 {
            let violation = ComplianceViolation::new(
                format!("v{}", i),
                "statute1".to_string(),
                "Violation".to_string(),
                ViolationSeverity::Medium,
            );
            agent.record_violation(violation);
        }

        agent.learn_from_violations();
        // After learning, agent should have enhanced monitoring rules
        assert!(!agent.monitoring_rules.is_empty());
    }

    #[test]
    fn test_legal_chatbot_intent_recognition() {
        let chatbot = SimpleLegalChatbot::new("bot1".to_string());

        let intent1 = chatbot.recognize_intent("What are my rights?");
        assert_eq!(intent1, LegalIntent::QueryRights);

        let intent2 = chatbot.recognize_intent("What are my obligations?");
        assert_eq!(intent2, LegalIntent::QueryObligations);

        let intent3 = chatbot.recognize_intent("Should I file this?");
        assert_eq!(intent3, LegalIntent::RequestAdvice);
    }

    #[test]
    fn test_legal_chatbot_conversation() {
        let mut chatbot = SimpleLegalChatbot::new("bot1".to_string());

        let response1 = chatbot.process_query("What are my tax obligations?");
        assert!(!response1.is_empty());
        assert_eq!(chatbot.conversation_length(), 1);

        let response2 = chatbot.process_query("How do I comply?");
        assert!(!response2.is_empty());
        assert_eq!(chatbot.conversation_length(), 2);

        chatbot.clear_conversation();
        assert_eq!(chatbot.conversation_length(), 0);
    }

    #[test]
    fn test_experience_replay_buffer() {
        let mut buffer = ExperienceReplayBuffer::new(10);

        let exp1 = Experience::new(vec![1.0, 2.0], 0, 1.0, vec![1.5, 2.5], false);
        let exp2 = Experience::new(vec![2.0, 3.0], 1, 0.5, vec![2.5, 3.5], false);

        buffer.add(exp1);
        buffer.add(exp2);

        assert_eq!(buffer.len(), 2);

        let sample = buffer.sample(1);
        assert_eq!(sample.len(), 1);
    }

    #[test]
    fn test_reinforcement_learning_agent() {
        let mut agent = ReinforcementLearningAgent::new("rl1".to_string(), 4, 10);
        agent.set_learning_rate(0.1);
        agent.set_discount_factor(0.95);

        let exp = Experience::new(vec![1.0, 2.0], 0, 1.0, vec![1.5, 2.5], false);
        agent.learn_from_experience(exp);

        assert_eq!(agent.experience_count(), 1);

        let action = agent.choose_action(&[1.0, 2.0]);
        assert!(action < 4);
    }

    #[test]
    fn test_q_learning_update() {
        let mut agent = ReinforcementLearningAgent::new("rl1".to_string(), 2, 10);
        agent.set_learning_rate(0.5);
        agent.set_discount_factor(0.9);

        let exp = Experience::new(vec![1.0], 0, 10.0, vec![2.0], false);
        agent.learn_from_experience(exp);

        // After learning, agent should have updated Q-values
        let eval = agent.self_evaluate();
        assert!(eval >= 0.0);
    }

    #[test]
    fn test_batch_training() {
        let mut agent = ReinforcementLearningAgent::new("rl1".to_string(), 2, 10);

        // Add multiple experiences
        for i in 0..5 {
            let exp = Experience::new(vec![i as f64], i % 2, 1.0, vec![(i + 1) as f64], false);
            agent.learn_from_experience(exp);
        }

        agent.train_on_batch(3);
        assert_eq!(agent.experience_count(), 5);
    }
}
