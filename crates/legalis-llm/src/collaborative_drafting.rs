//! Collaborative AI Drafting (v0.3.2)
//!
//! This module provides real-time collaborative AI editing, multi-stakeholder
//! negotiation AI, version-aware drafting assistance, clause suggestion ranking,
//! and contract optimization recommendations.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a collaborative editing session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeSession {
    pub id: String,
    pub document_id: String,
    pub participants: Vec<Participant>,
    pub current_version: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub role: ParticipantRole,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ParticipantRole {
    Editor,
    Reviewer,
    Observer,
    AI,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Suggest,
    Approve,
    Delete,
}

/// An edit operation in a collaborative session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditOperation {
    pub id: String,
    pub session_id: String,
    pub participant_id: String,
    pub operation_type: OperationType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Insert {
        position: usize,
        content: String,
    },
    Delete {
        position: usize,
        length: usize,
    },
    Replace {
        position: usize,
        old_content: String,
        new_content: String,
    },
    Suggest {
        position: usize,
        suggestion: String,
        reason: String,
    },
}

/// Real-time collaborative AI editor.
pub struct CollaborativeEditor {
    sessions: Arc<RwLock<HashMap<String, CollaborativeSession>>>,
    operations: Arc<RwLock<HashMap<String, Vec<EditOperation>>>>, // session_id -> operations
    suggestions: Arc<RwLock<HashMap<String, Vec<AISuggestion>>>>, // session_id -> suggestions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISuggestion {
    pub id: String,
    pub session_id: String,
    pub position: usize,
    pub suggestion_type: SuggestionType,
    pub content: String,
    pub reason: String,
    pub confidence: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub status: SuggestionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuggestionType {
    Grammar,
    Legal,
    Clarity,
    Consistency,
    Clause,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuggestionStatus {
    Pending,
    Accepted,
    Rejected,
}

impl CollaborativeEditor {
    /// Creates a new collaborative editor.
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            operations: Arc::new(RwLock::new(HashMap::new())),
            suggestions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new collaborative session.
    pub async fn create_session(
        &self,
        document_id: impl Into<String>,
    ) -> Result<CollaborativeSession> {
        let session = CollaborativeSession {
            id: uuid::Uuid::new_v4().to_string(),
            document_id: document_id.into(),
            participants: Vec::new(),
            current_version: 0,
            created_at: chrono::Utc::now(),
            last_modified: chrono::Utc::now(),
        };

        let session_id = session.id.clone();
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session.clone());

        Ok(session)
    }

    /// Adds a participant to a session.
    pub async fn add_participant(&self, session_id: &str, participant: Participant) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.participants.push(participant);
            session.last_modified = chrono::Utc::now();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    /// Applies an edit operation to a session.
    pub async fn apply_operation(&self, operation: EditOperation) -> Result<()> {
        let session_id = operation.session_id.clone();

        // Add operation to history
        {
            let mut operations = self.operations.write().await;
            operations
                .entry(session_id.clone())
                .or_insert_with(Vec::new)
                .push(operation.clone());
        }

        // Update session version
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(&session_id) {
                session.current_version += 1;
                session.last_modified = chrono::Utc::now();
            }
        }

        Ok(())
    }

    /// Generates AI suggestions for a session.
    pub async fn generate_suggestions(
        &self,
        session_id: &str,
        content: &str,
    ) -> Result<Vec<AISuggestion>> {
        let mut suggestions = Vec::new();

        // Example: Grammar check
        if content.contains("their") && content.contains("there") {
            suggestions.push(AISuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: session_id.to_string(),
                position: content.find("their").unwrap_or(0),
                suggestion_type: SuggestionType::Grammar,
                content: "Check 'their' vs 'there' usage".to_string(),
                reason: "Common grammar confusion".to_string(),
                confidence: 0.8,
                created_at: chrono::Utc::now(),
                status: SuggestionStatus::Pending,
            });
        }

        // Store suggestions
        {
            let mut sugs = self.suggestions.write().await;
            sugs.entry(session_id.to_string())
                .or_insert_with(Vec::new)
                .extend(suggestions.clone());
        }

        Ok(suggestions)
    }

    /// Gets all suggestions for a session.
    pub async fn get_suggestions(&self, session_id: &str) -> Vec<AISuggestion> {
        let sugs = self.suggestions.read().await;
        sugs.get(session_id).cloned().unwrap_or_default()
    }

    /// Accepts a suggestion.
    pub async fn accept_suggestion(&self, session_id: &str, suggestion_id: &str) -> Result<()> {
        let mut sugs = self.suggestions.write().await;
        if let Some(session_sugs) = sugs.get_mut(session_id) {
            for sug in session_sugs.iter_mut() {
                if sug.id == suggestion_id {
                    sug.status = SuggestionStatus::Accepted;
                    return Ok(());
                }
            }
        }
        Err(anyhow::anyhow!("Suggestion not found"))
    }
}

impl Default for CollaborativeEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-stakeholder negotiation AI.
pub struct NegotiationAI {
    stakeholders: Arc<RwLock<Vec<Stakeholder>>>,
    proposals: Arc<RwLock<Vec<Proposal>>>,
    negotiation_state: Arc<RwLock<NegotiationState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stakeholder {
    pub id: String,
    pub name: String,
    pub interests: Vec<String>,
    pub priorities: HashMap<String, f64>, // interest -> priority (0.0-1.0)
    pub flexibility: f64,                 // 0.0 (rigid) to 1.0 (flexible)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub proposer_id: String,
    pub content: String,
    pub terms: Vec<Term>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub votes: HashMap<String, Vote>, // stakeholder_id -> vote
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    pub key: String,
    pub value: String,
    pub negotiable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Vote {
    Accept,
    Reject,
    CounterPropose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationState {
    pub phase: NegotiationPhase,
    pub rounds: usize,
    pub consensus_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NegotiationPhase {
    Initial,
    Proposing,
    Voting,
    CounterProposing,
    Finalizing,
    Concluded,
}

impl NegotiationAI {
    /// Creates a new negotiation AI.
    pub fn new() -> Self {
        Self {
            stakeholders: Arc::new(RwLock::new(Vec::new())),
            proposals: Arc::new(RwLock::new(Vec::new())),
            negotiation_state: Arc::new(RwLock::new(NegotiationState {
                phase: NegotiationPhase::Initial,
                rounds: 0,
                consensus_threshold: 0.7,
            })),
        }
    }

    /// Adds a stakeholder to the negotiation.
    pub async fn add_stakeholder(&self, stakeholder: Stakeholder) -> Result<()> {
        let mut stakeholders = self.stakeholders.write().await;
        stakeholders.push(stakeholder);
        Ok(())
    }

    /// Submits a proposal.
    pub async fn submit_proposal(&self, proposal: Proposal) -> Result<()> {
        let mut proposals = self.proposals.write().await;
        proposals.push(proposal);

        let mut state = self.negotiation_state.write().await;
        state.phase = NegotiationPhase::Voting;

        Ok(())
    }

    /// Records a vote on a proposal.
    pub async fn vote(&self, proposal_id: &str, stakeholder_id: &str, vote: Vote) -> Result<()> {
        let mut proposals = self.proposals.write().await;
        for proposal in proposals.iter_mut() {
            if proposal.id == proposal_id {
                proposal.votes.insert(stakeholder_id.to_string(), vote);
                return Ok(());
            }
        }
        Err(anyhow::anyhow!("Proposal not found"))
    }

    /// Checks if consensus has been reached.
    pub async fn check_consensus(&self, proposal_id: &str) -> Option<bool> {
        let proposals = self.proposals.read().await;
        let state = self.negotiation_state.read().await;
        let stakeholders = self.stakeholders.read().await;

        for proposal in proposals.iter() {
            if proposal.id == proposal_id {
                let accept_count = proposal
                    .votes
                    .values()
                    .filter(|v| **v == Vote::Accept)
                    .count();
                let total_stakeholders = stakeholders.len();

                if total_stakeholders > 0 {
                    let consensus_ratio = accept_count as f64 / total_stakeholders as f64;
                    return Some(consensus_ratio >= state.consensus_threshold);
                }
            }
        }
        None
    }

    /// Generates a compromise proposal based on stakeholder interests.
    pub async fn generate_compromise(&self) -> Result<Proposal> {
        let stakeholders = self.stakeholders.read().await;

        let mut terms = Vec::new();
        let mut all_interests = HashMap::new();

        // Aggregate interests and priorities
        for stakeholder in stakeholders.iter() {
            for (interest, priority) in &stakeholder.priorities {
                *all_interests.entry(interest.clone()).or_insert(0.0) += priority;
            }
        }

        // Create terms based on highest priority interests
        for (interest, total_priority) in all_interests {
            terms.push(Term {
                key: interest.clone(),
                value: format!("Compromise on {}", interest),
                negotiable: total_priority < stakeholders.len() as f64 * 0.8,
            });
        }

        Ok(Proposal {
            id: uuid::Uuid::new_v4().to_string(),
            proposer_id: "AI".to_string(),
            content: "AI-generated compromise proposal".to_string(),
            terms,
            timestamp: chrono::Utc::now(),
            votes: HashMap::new(),
        })
    }
}

impl Default for NegotiationAI {
    fn default() -> Self {
        Self::new()
    }
}

/// Version-aware drafting assistant.
pub struct VersionAwareDrafter {
    versions: Arc<RwLock<Vec<DocumentVersion>>>,
    current_version_id: Arc<RwLock<Option<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub id: String,
    pub version_number: u32,
    pub content: String,
    pub author: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub parent_version: Option<String>,
    pub changes: Vec<VersionChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionChange {
    pub change_type: VersionChangeType,
    pub location: usize,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersionChangeType {
    Addition,
    Deletion,
    Modification,
}

impl VersionAwareDrafter {
    /// Creates a new version-aware drafter.
    pub fn new() -> Self {
        Self {
            versions: Arc::new(RwLock::new(Vec::new())),
            current_version_id: Arc::new(RwLock::new(None)),
        }
    }

    /// Creates a new document version.
    pub async fn create_version(&self, content: String, author: String) -> Result<DocumentVersion> {
        let mut versions = self.versions.write().await;
        // Clone the parent version ID and release the read lock before acquiring write lock
        let parent_id = {
            let current_id = self.current_version_id.read().await;
            current_id.clone()
        }; // read lock released here

        let version = DocumentVersion {
            id: uuid::Uuid::new_v4().to_string(),
            version_number: versions.len() as u32 + 1,
            content,
            author,
            timestamp: chrono::Utc::now(),
            parent_version: parent_id,
            changes: Vec::new(),
        };

        let version_id = version.id.clone();
        versions.push(version.clone());

        drop(versions);
        let mut current = self.current_version_id.write().await;
        *current = Some(version_id);

        Ok(version)
    }

    /// Gets a specific version by ID.
    pub async fn get_version(&self, version_id: &str) -> Option<DocumentVersion> {
        let versions = self.versions.read().await;
        versions.iter().find(|v| v.id == version_id).cloned()
    }

    /// Compares two versions and returns the differences.
    pub async fn compare_versions(
        &self,
        version1_id: &str,
        version2_id: &str,
    ) -> Result<Vec<VersionChange>> {
        let versions = self.versions.read().await;

        let v1 = versions.iter().find(|v| v.id == version1_id);
        let v2 = versions.iter().find(|v| v.id == version2_id);

        match (v1, v2) {
            (Some(ver1), Some(ver2)) => {
                let mut changes = Vec::new();

                // Simple diff: just check if content is different
                if ver1.content != ver2.content {
                    changes.push(VersionChange {
                        change_type: VersionChangeType::Modification,
                        location: 0,
                        description: "Content modified".to_string(),
                    });
                }

                Ok(changes)
            }
            _ => Err(anyhow::anyhow!("Version not found")),
        }
    }

    /// Gets the version history.
    pub async fn get_history(&self) -> Vec<DocumentVersion> {
        let versions = self.versions.read().await;
        versions.clone()
    }
}

impl Default for VersionAwareDrafter {
    fn default() -> Self {
        Self::new()
    }
}

/// Clause suggestion ranker.
pub struct ClauseRanker {
    clauses: Arc<RwLock<Vec<ClauseTemplate>>>,
    usage_stats: Arc<RwLock<HashMap<String, UsageStats>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClauseTemplate {
    pub id: String,
    pub name: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub context_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub times_used: usize,
    pub times_accepted: usize,
    pub avg_rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedClause {
    pub clause: ClauseTemplate,
    pub score: f64,
    pub reasons: Vec<String>,
}

impl ClauseRanker {
    /// Creates a new clause ranker.
    pub fn new() -> Self {
        Self {
            clauses: Arc::new(RwLock::new(Vec::new())),
            usage_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds a clause template.
    pub async fn add_clause(&self, clause: ClauseTemplate) -> Result<()> {
        let mut clauses = self.clauses.write().await;
        clauses.push(clause);
        Ok(())
    }

    /// Ranks clauses based on context and usage.
    pub async fn rank_clauses(&self, context: &str, category: &str) -> Vec<RankedClause> {
        let clauses = self.clauses.read().await;
        let stats = self.usage_stats.read().await;

        let mut ranked = Vec::new();

        for clause in clauses.iter() {
            if clause.category != category {
                continue;
            }

            let mut score = 0.0;
            let mut reasons = Vec::new();

            // Context matching
            let context_match = clause
                .context_requirements
                .iter()
                .filter(|req| context.contains(req.as_str()))
                .count() as f64
                / clause.context_requirements.len().max(1) as f64;
            score += context_match * 0.4;
            if context_match > 0.5 {
                reasons.push("Good context match".to_string());
            }

            // Usage statistics
            if let Some(usage) = stats.get(&clause.id) {
                let acceptance_rate = if usage.times_used > 0 {
                    usage.times_accepted as f64 / usage.times_used as f64
                } else {
                    0.0
                };
                score += acceptance_rate * 0.3;
                score += (usage.avg_rating / 5.0) * 0.3;

                if acceptance_rate > 0.7 {
                    reasons.push("High acceptance rate".to_string());
                }
            }

            ranked.push(RankedClause {
                clause: clause.clone(),
                score,
                reasons,
            });
        }

        ranked.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        ranked
    }

    /// Records usage of a clause.
    pub async fn record_usage(&self, clause_id: &str, accepted: bool, rating: f64) -> Result<()> {
        let mut stats = self.usage_stats.write().await;

        let usage = stats.entry(clause_id.to_string()).or_insert(UsageStats {
            times_used: 0,
            times_accepted: 0,
            avg_rating: 0.0,
        });

        usage.times_used += 1;
        if accepted {
            usage.times_accepted += 1;
        }

        // Update average rating
        let old_total = usage.avg_rating * (usage.times_used - 1) as f64;
        usage.avg_rating = (old_total + rating) / usage.times_used as f64;

        Ok(())
    }
}

impl Default for ClauseRanker {
    fn default() -> Self {
        Self::new()
    }
}

/// Contract optimization recommender.
pub struct ContractOptimizer {
    optimization_rules: Arc<RwLock<Vec<OptimizationRule>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub recommendation: String,
    pub category: OptimizationCategory,
    pub priority: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationCategory {
    Clarity,
    Legal,
    Financial,
    Risk,
    Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractOptimizationRecommendation {
    pub rule: OptimizationRule,
    pub location: usize,
    pub impact_score: f64,
    pub reasoning: String,
}

impl ContractOptimizer {
    /// Creates a new contract optimizer.
    pub fn new() -> Self {
        Self {
            optimization_rules: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds an optimization rule.
    pub async fn add_rule(&self, rule: OptimizationRule) -> Result<()> {
        let mut rules = self.optimization_rules.write().await;
        rules.push(rule);
        Ok(())
    }

    /// Analyzes a contract and generates optimization recommendations.
    pub async fn analyze_contract(&self, content: &str) -> Vec<ContractOptimizationRecommendation> {
        let rules = self.optimization_rules.read().await;
        let mut recommendations = Vec::new();

        for rule in rules.iter() {
            // Simple pattern matching (in real implementation, use regex or NLP)
            if content.contains(&rule.pattern) {
                let location = content.find(&rule.pattern).unwrap_or(0);
                recommendations.push(ContractOptimizationRecommendation {
                    rule: rule.clone(),
                    location,
                    impact_score: rule.priority,
                    reasoning: format!("Found pattern '{}': {}", rule.pattern, rule.description),
                });
            }
        }

        recommendations.sort_by(|a, b| {
            b.impact_score
                .partial_cmp(&a.impact_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        recommendations
    }

    /// Gets recommendations by category.
    pub async fn get_recommendations_by_category(
        &self,
        content: &str,
        category: OptimizationCategory,
    ) -> Vec<ContractOptimizationRecommendation> {
        let all_recs = self.analyze_contract(content).await;
        all_recs
            .into_iter()
            .filter(|rec| rec.rule.category == category)
            .collect()
    }
}

impl Default for ContractOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_collaborative_editor() {
        let editor = CollaborativeEditor::new();
        let session = editor.create_session("doc1").await.unwrap();

        let participant = Participant {
            id: "user1".to_string(),
            name: "Alice".to_string(),
            role: ParticipantRole::Editor,
            permissions: vec![Permission::Read, Permission::Write],
        };

        editor
            .add_participant(&session.id, participant)
            .await
            .unwrap();

        let suggestions = editor
            .generate_suggestions(&session.id, "Check their work there")
            .await
            .unwrap();
        assert!(!suggestions.is_empty());
    }

    #[tokio::test]
    async fn test_negotiation_ai() {
        let ai = NegotiationAI::new();

        let stakeholder = Stakeholder {
            id: "sh1".to_string(),
            name: "Company A".to_string(),
            interests: vec!["price".to_string()],
            priorities: [("price".to_string(), 0.9)].iter().cloned().collect(),
            flexibility: 0.5,
        };

        ai.add_stakeholder(stakeholder).await.unwrap();

        let proposal = Proposal {
            id: "prop1".to_string(),
            proposer_id: "sh1".to_string(),
            content: "Initial proposal".to_string(),
            terms: vec![],
            timestamp: chrono::Utc::now(),
            votes: HashMap::new(),
        };

        ai.submit_proposal(proposal).await.unwrap();
        ai.vote("prop1", "sh1", Vote::Accept).await.unwrap();
    }

    #[tokio::test]
    async fn test_version_aware_drafter() {
        let drafter = VersionAwareDrafter::new();

        let v1 = drafter
            .create_version("Version 1 content".to_string(), "Alice".to_string())
            .await
            .unwrap();
        let v2 = drafter
            .create_version("Version 2 content".to_string(), "Bob".to_string())
            .await
            .unwrap();

        let history = drafter.get_history().await;
        assert_eq!(history.len(), 2);

        let changes = drafter.compare_versions(&v1.id, &v2.id).await.unwrap();
        assert!(!changes.is_empty());
    }

    #[tokio::test]
    async fn test_clause_ranker() {
        let ranker = ClauseRanker::new();

        let clause = ClauseTemplate {
            id: "c1".to_string(),
            name: "Payment Clause".to_string(),
            content: "Payment terms...".to_string(),
            category: "payment".to_string(),
            tags: vec!["financial".to_string()],
            context_requirements: vec!["payment".to_string()],
        };

        ranker.add_clause(clause).await.unwrap();

        let ranked = ranker
            .rank_clauses("This contract covers payment terms", "payment")
            .await;
        assert!(!ranked.is_empty());
    }

    #[tokio::test]
    async fn test_contract_optimizer() {
        let optimizer = ContractOptimizer::new();

        let rule = OptimizationRule {
            id: "r1".to_string(),
            name: "Simplify language".to_string(),
            description: "Use simpler terms".to_string(),
            pattern: "hereinafter".to_string(),
            recommendation: "Use 'later in this document' instead".to_string(),
            category: OptimizationCategory::Clarity,
            priority: 0.8,
        };

        optimizer.add_rule(rule).await.unwrap();

        let recs = optimizer
            .analyze_contract("The party hereinafter referred to as...")
            .await;
        assert!(!recs.is_empty());
    }
}
