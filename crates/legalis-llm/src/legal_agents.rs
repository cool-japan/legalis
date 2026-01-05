//! Legal Agent Framework (v0.2.5)
//!
//! This module provides autonomous AI agents specialized for legal tasks including:
//! - Legal research
//! - Contract review
//! - Compliance monitoring
//! - Negotiation assistance
//! - Dispute resolution

use crate::{
    LLMProvider,
    legal::{Jurisdiction, LegalCitation},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Agent task status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentTaskStatus {
    /// Task is queued
    Queued,
    /// Task is in progress
    InProgress,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task cancelled
    Cancelled,
}

/// Agent task result with detailed information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTaskResult {
    /// Task identifier
    pub task_id: String,
    /// Task status
    pub status: AgentTaskStatus,
    /// Result content
    pub content: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Supporting evidence and references
    pub evidence: Vec<String>,
    /// Citations found or used
    pub citations: Vec<LegalCitation>,
    /// Warnings or issues identified
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Metadata
    pub metadata: serde_json::Value,
}

impl AgentTaskResult {
    /// Creates a new task result.
    pub fn new(task_id: impl Into<String>) -> Self {
        Self {
            task_id: task_id.into(),
            status: AgentTaskStatus::Queued,
            content: String::new(),
            confidence: 0.0,
            evidence: Vec::new(),
            citations: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
            metadata: serde_json::json!({}),
        }
    }

    /// Sets the status.
    pub fn with_status(mut self, status: AgentTaskStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the content.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Adds evidence.
    pub fn add_evidence(&mut self, evidence: impl Into<String>) {
        self.evidence.push(evidence.into());
    }

    /// Adds a citation.
    pub fn add_citation(&mut self, citation: LegalCitation) {
        self.citations.push(citation);
    }

    /// Adds a warning.
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Adds a recommendation.
    pub fn add_recommendation(&mut self, recommendation: impl Into<String>) {
        self.recommendations.push(recommendation.into());
    }
}

/// Autonomous Legal Research Agent.
///
/// This agent can autonomously:
/// - Search case law and statutes
/// - Identify relevant precedents
/// - Analyze legal questions
/// - Provide comprehensive research reports
pub struct LegalResearchAgent<P> {
    provider: Arc<P>,
    jurisdiction: Option<Jurisdiction>,
    search_depth: usize,
    max_citations: usize,
}

impl<P: LLMProvider> LegalResearchAgent<P> {
    /// Creates a new legal research agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider: Arc::new(provider),
            jurisdiction: None,
            search_depth: 3,
            max_citations: 20,
        }
    }

    /// Sets the jurisdiction for research.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Sets the search depth (number of recursive searches).
    pub fn with_search_depth(mut self, depth: usize) -> Self {
        self.search_depth = depth;
        self
    }

    /// Sets the maximum number of citations to return.
    pub fn with_max_citations(mut self, max: usize) -> Self {
        self.max_citations = max;
        self
    }

    /// Conducts autonomous legal research on a question.
    pub async fn research(&self, question: &str) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in the jurisdiction of {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"You are an autonomous legal research agent{jurisdiction}.

Research Question: {question}

Conduct comprehensive legal research and provide:
1. Summary of relevant legal principles
2. Key case law and precedents
3. Relevant statutes and regulations
4. Analysis of how these apply to the question
5. Potential counterarguments or limitations
6. Recommendations for further research

Format your response as JSON with the following structure:
{{
    "summary": "Brief summary of findings",
    "legal_principles": ["principle 1", "principle 2", ...],
    "key_cases": ["case 1", "case 2", ...],
    "statutes": ["statute 1", "statute 2", ...],
    "analysis": "Detailed analysis",
    "counterarguments": ["argument 1", "argument 2", ...],
    "recommendations": ["recommendation 1", "recommendation 2", ...],
    "confidence": 0.85
}}"#,
            jurisdiction = jurisdiction_context,
            question = question
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                // Parse the JSON response
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                    result.content = parsed["summary"]
                        .as_str()
                        .unwrap_or("No summary available")
                        .to_string();

                    result.confidence = parsed["confidence"].as_f64().unwrap_or(0.5);

                    if let Some(principles) = parsed["legal_principles"].as_array() {
                        for principle in principles {
                            if let Some(p) = principle.as_str() {
                                result.add_evidence(p);
                            }
                        }
                    }

                    if let Some(recommendations) = parsed["recommendations"].as_array() {
                        for rec in recommendations {
                            if let Some(r) = rec.as_str() {
                                result.add_recommendation(r);
                            }
                        }
                    }

                    result.metadata = parsed;
                    result.status = AgentTaskStatus::Completed;
                } else {
                    // If JSON parsing fails, use raw response
                    result.content = response;
                    result.confidence = 0.6;
                    result.status = AgentTaskStatus::Completed;
                    result.add_warning("Response was not in expected JSON format");
                }

                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                result.add_warning(format!("Research failed: {}", e));
                Err(e).context("Legal research failed")
            }
        }
    }

    /// Finds relevant precedents for a legal issue.
    pub async fn find_precedents(&self, issue: &str) -> Result<Vec<LegalCitation>> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"Find relevant legal precedents{jurisdiction} for this issue:

{issue}

List the most relevant cases with:
- Case name
- Citation
- Year
- Key holding
- Relevance to the issue

Limit to {max_citations} most relevant cases."#,
            jurisdiction = jurisdiction_context,
            issue = issue,
            max_citations = self.max_citations
        );

        let response = self.provider.generate_text(&prompt).await?;

        // Parse citations from response (simplified - in production would use proper citation parsing)
        let mut citations = Vec::new();
        for line in response.lines() {
            if line.contains("v.") || line.contains("U.S.") || line.contains("F.") {
                citations.push(LegalCitation {
                    citation: line.trim().to_string(),
                    citation_type: "Case Law".to_string(),
                    context: None,
                });
            }
        }

        Ok(citations.into_iter().take(self.max_citations).collect())
    }
}

/// Contract Review Agent.
///
/// This agent can:
/// - Analyze contracts for risks and issues
/// - Identify problematic clauses
/// - Suggest improvements
/// - Check compliance with standards
pub struct ContractReviewAgent<P> {
    provider: Arc<P>,
    review_checklist: Vec<String>,
    risk_threshold: f64,
}

impl<P: LLMProvider> ContractReviewAgent<P> {
    /// Creates a new contract review agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider: Arc::new(provider),
            review_checklist: Self::default_checklist(),
            risk_threshold: 0.7,
        }
    }

    /// Default review checklist.
    fn default_checklist() -> Vec<String> {
        vec![
            "Termination clauses".to_string(),
            "Liability limitations".to_string(),
            "Indemnification provisions".to_string(),
            "Intellectual property rights".to_string(),
            "Confidentiality obligations".to_string(),
            "Dispute resolution mechanisms".to_string(),
            "Payment terms".to_string(),
            "Force majeure provisions".to_string(),
            "Governing law and jurisdiction".to_string(),
            "Amendment procedures".to_string(),
        ]
    }

    /// Sets a custom review checklist.
    pub fn with_checklist(mut self, checklist: Vec<String>) -> Self {
        self.review_checklist = checklist;
        self
    }

    /// Sets the risk threshold (0.0 - 1.0).
    pub fn with_risk_threshold(mut self, threshold: f64) -> Self {
        self.risk_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Reviews a contract and identifies issues.
    pub async fn review_contract(&self, contract_text: &str) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let checklist_items = self.review_checklist.join("\n- ");

        let prompt = format!(
            r#"You are a contract review agent. Analyze this contract and identify potential issues, risks, and areas for improvement.

Review Checklist:
- {checklist}

Contract:
{contract}

Provide a detailed review including:
1. Overall risk assessment (0.0 - 1.0)
2. Identified issues and their severity
3. Missing or problematic clauses
4. Recommendations for improvement
5. Compliance concerns

Format as JSON:
{{
    "risk_score": 0.65,
    "issues": [
        {{
            "clause": "Termination clause",
            "severity": "High",
            "description": "...",
            "recommendation": "..."
        }}
    ],
    "missing_clauses": ["..."],
    "recommendations": ["..."],
    "compliance_concerns": ["..."]
}}"#,
            checklist = checklist_items,
            contract = contract_text
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                    let risk_score = parsed["risk_score"].as_f64().unwrap_or(0.5);
                    result.confidence = 1.0 - risk_score;

                    let mut summary =
                        format!("Contract review completed. Risk score: {:.2}", risk_score);

                    if let Some(issues) = parsed["issues"].as_array() {
                        summary.push_str(&format!("\nIdentified {} issues.", issues.len()));
                        for issue in issues {
                            if let Some(desc) = issue["description"].as_str() {
                                result.add_warning(desc);
                            }
                        }
                    }

                    if let Some(recommendations) = parsed["recommendations"].as_array() {
                        for rec in recommendations {
                            if let Some(r) = rec.as_str() {
                                result.add_recommendation(r);
                            }
                        }
                    }

                    result.content = summary;
                    result.metadata = parsed;

                    if risk_score >= self.risk_threshold {
                        result.add_warning("Contract risk score exceeds threshold");
                    }

                    result.status = AgentTaskStatus::Completed;
                } else {
                    result.content = response;
                    result.confidence = 0.6;
                    result.status = AgentTaskStatus::Completed;
                    result.add_warning("Response was not in expected JSON format");
                }

                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                result.add_warning(format!("Contract review failed: {}", e));
                Err(e).context("Contract review failed")
            }
        }
    }

    /// Compares two contract versions and identifies changes.
    pub async fn compare_versions(&self, original: &str, revised: &str) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let prompt = format!(
            r#"Compare these two contract versions and identify all material changes:

Original Contract:
{original}

Revised Contract:
{revised}

Provide:
1. List of additions
2. List of deletions
3. List of modifications
4. Risk assessment of changes
5. Recommendations

Format as JSON with arrays of changes."#,
            original = original,
            revised = revised
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                result.content = response.clone();
                result.status = AgentTaskStatus::Completed;
                result.confidence = 0.85;
                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                Err(e).context("Contract comparison failed")
            }
        }
    }
}

/// Compliance Monitoring Agent.
///
/// This agent monitors for:
/// - Regulatory compliance
/// - Policy violations
/// - Legal requirement adherence
/// - Risk indicators
pub struct ComplianceMonitoringAgent<P> {
    provider: Arc<P>,
    regulations: Vec<String>,
    alert_threshold: f64,
    monitoring_active: Arc<RwLock<bool>>,
}

impl<P: LLMProvider> ComplianceMonitoringAgent<P> {
    /// Creates a new compliance monitoring agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider: Arc::new(provider),
            regulations: Vec::new(),
            alert_threshold: 0.8,
            monitoring_active: Arc::new(RwLock::new(false)),
        }
    }

    /// Adds a regulation to monitor.
    pub fn add_regulation(mut self, regulation: impl Into<String>) -> Self {
        self.regulations.push(regulation.into());
        self
    }

    /// Sets the alert threshold for compliance violations.
    pub fn with_alert_threshold(mut self, threshold: f64) -> Self {
        self.alert_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Checks compliance of a document or action.
    pub async fn check_compliance(&self, subject: &str) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let regulations_list = if self.regulations.is_empty() {
            "all applicable regulations".to_string()
        } else {
            self.regulations.join(", ")
        };

        let prompt = format!(
            r#"You are a compliance monitoring agent. Check the following for compliance with {regulations}:

{subject}

Analyze for:
1. Regulatory compliance
2. Policy violations
3. Legal risks
4. Required actions

Provide compliance assessment as JSON:
{{
    "compliant": true/false,
    "compliance_score": 0.95,
    "violations": [
        {{
            "regulation": "...",
            "severity": "High/Medium/Low",
            "description": "...",
            "remediation": "..."
        }}
    ],
    "risks": ["..."],
    "required_actions": ["..."]
}}"#,
            regulations = regulations_list,
            subject = subject
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                    let compliance_score = parsed["compliance_score"].as_f64().unwrap_or(1.0);
                    let compliant = parsed["compliant"].as_bool().unwrap_or(true);

                    result.confidence = compliance_score;
                    result.content = if compliant {
                        format!("Compliant (score: {:.2})", compliance_score)
                    } else {
                        format!("Non-compliant (score: {:.2})", compliance_score)
                    };

                    if let Some(violations) = parsed["violations"].as_array() {
                        for violation in violations {
                            if let Some(desc) = violation["description"].as_str() {
                                result.add_warning(desc);
                            }
                        }
                    }

                    if let Some(actions) = parsed["required_actions"].as_array() {
                        for action in actions {
                            if let Some(a) = action.as_str() {
                                result.add_recommendation(a);
                            }
                        }
                    }

                    result.metadata = parsed;

                    if !compliant || compliance_score < self.alert_threshold {
                        result.add_warning("Compliance alert triggered");
                    }

                    result.status = AgentTaskStatus::Completed;
                } else {
                    result.content = response;
                    result.confidence = 0.7;
                    result.status = AgentTaskStatus::Completed;
                    result.add_warning("Response was not in expected JSON format");
                }

                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                Err(e).context("Compliance check failed")
            }
        }
    }

    /// Starts continuous compliance monitoring.
    pub async fn start_monitoring(&self) {
        let mut active = self.monitoring_active.write().await;
        *active = true;
    }

    /// Stops compliance monitoring.
    pub async fn stop_monitoring(&self) {
        let mut active = self.monitoring_active.write().await;
        *active = false;
    }

    /// Checks if monitoring is active.
    pub async fn is_monitoring(&self) -> bool {
        *self.monitoring_active.read().await
    }
}

/// Negotiation Assistance Agent.
///
/// This agent helps with:
/// - Contract negotiations
/// - Settlement discussions
/// - Proposal drafting
/// - Counter-offer generation
pub struct NegotiationAssistanceAgent<P> {
    provider: Arc<P>,
    negotiation_style: NegotiationStyle,
    objectives: Vec<String>,
}

/// Negotiation style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NegotiationStyle {
    /// Collaborative/win-win approach
    Collaborative,
    /// Competitive/aggressive approach
    Competitive,
    /// Accommodating/conciliatory approach
    Accommodating,
    /// Compromising/balanced approach
    Compromising,
}

impl NegotiationStyle {
    fn description(&self) -> &str {
        match self {
            Self::Collaborative => "collaborative, seeking mutually beneficial outcomes",
            Self::Competitive => "competitive, assertively pursuing client interests",
            Self::Accommodating => "accommodating, prioritizing relationship preservation",
            Self::Compromising => "compromising, seeking balanced middle-ground solutions",
        }
    }
}

impl<P: LLMProvider> NegotiationAssistanceAgent<P> {
    /// Creates a new negotiation assistance agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider: Arc::new(provider),
            negotiation_style: NegotiationStyle::Collaborative,
            objectives: Vec::new(),
        }
    }

    /// Sets the negotiation style.
    pub fn with_style(mut self, style: NegotiationStyle) -> Self {
        self.negotiation_style = style;
        self
    }

    /// Adds a negotiation objective.
    pub fn add_objective(mut self, objective: impl Into<String>) -> Self {
        self.objectives.push(objective.into());
        self
    }

    /// Generates a counter-offer.
    pub async fn generate_counter_offer(&self, original_offer: &str) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let objectives_text = if self.objectives.is_empty() {
            "the client's best interests".to_string()
        } else {
            self.objectives.join(", ")
        };

        let prompt = format!(
            r#"You are a negotiation assistance agent with a {style} approach.

Original Offer:
{offer}

Client Objectives:
{objectives}

Generate a counter-offer that:
1. Addresses the client's key objectives
2. Maintains a {style} tone
3. Identifies areas for compromise
4. Highlights strengths of our position
5. Proposes specific terms

Provide as JSON:
{{
    "counter_offer": "Full text of counter-offer",
    "key_changes": ["change 1", "change 2", ...],
    "rationale": "Explanation of strategy",
    "compromise_areas": ["area 1", "area 2", ...],
    "non_negotiables": ["item 1", "item 2", ...]
}}"#,
            style = self.negotiation_style.description(),
            offer = original_offer,
            objectives = objectives_text
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                    result.content = parsed["counter_offer"]
                        .as_str()
                        .unwrap_or("No counter-offer generated")
                        .to_string();

                    result.metadata = parsed.clone();
                    result.confidence = 0.8;

                    if let Some(changes) = parsed["key_changes"].as_array() {
                        for change in changes {
                            if let Some(c) = change.as_str() {
                                result.add_evidence(format!("Key change: {}", c));
                            }
                        }
                    }

                    result.status = AgentTaskStatus::Completed;
                } else {
                    result.content = response;
                    result.confidence = 0.6;
                    result.status = AgentTaskStatus::Completed;
                    result.add_warning("Response was not in expected JSON format");
                }

                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                Err(e).context("Counter-offer generation failed")
            }
        }
    }

    /// Analyzes a negotiation position.
    pub async fn analyze_position(&self, position: &str) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let prompt = format!(
            r#"Analyze this negotiation position:

{position}

Provide:
1. Strengths of the position
2. Weaknesses or vulnerabilities
3. Likely counter-arguments
4. Suggested improvements
5. BATNA (Best Alternative to Negotiated Agreement) considerations

Format as detailed analysis."#,
            position = position
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                result.content = response;
                result.confidence = 0.8;
                result.status = AgentTaskStatus::Completed;
                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                Err(e).context("Position analysis failed")
            }
        }
    }
}

/// Dispute Resolution Agent.
///
/// This agent assists with:
/// - Dispute analysis
/// - Resolution strategy
/// - Mediation support
/// - Settlement proposals
pub struct DisputeResolutionAgent<P> {
    provider: Arc<P>,
    resolution_preference: ResolutionPreference,
    jurisdiction: Option<Jurisdiction>,
}

/// Dispute resolution preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionPreference {
    /// Prefer mediation/settlement
    Mediation,
    /// Prefer arbitration
    Arbitration,
    /// Prefer litigation
    Litigation,
    /// Adaptive based on case
    Adaptive,
}

impl ResolutionPreference {
    fn description(&self) -> &str {
        match self {
            Self::Mediation => "mediation and settlement",
            Self::Arbitration => "arbitration",
            Self::Litigation => "litigation",
            Self::Adaptive => "the most appropriate method based on the circumstances",
        }
    }
}

impl<P: LLMProvider> DisputeResolutionAgent<P> {
    /// Creates a new dispute resolution agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider: Arc::new(provider),
            resolution_preference: ResolutionPreference::Adaptive,
            jurisdiction: None,
        }
    }

    /// Sets the resolution preference.
    pub fn with_preference(mut self, preference: ResolutionPreference) -> Self {
        self.resolution_preference = preference;
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Analyzes a dispute and suggests resolution strategy.
    pub async fn analyze_dispute(&self, dispute_details: &str) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in the jurisdiction of {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"You are a dispute resolution agent{jurisdiction}. Analyze this dispute and recommend resolution strategies, with preference for {preference}.

Dispute Details:
{dispute}

Provide comprehensive analysis including:
1. Key issues and claims
2. Strengths and weaknesses of each party's position
3. Applicable law and precedents
4. Recommended resolution approach
5. Settlement value range (if applicable)
6. Timeline and cost estimates

Format as JSON:
{{
    "summary": "Brief dispute summary",
    "key_issues": ["issue 1", "issue 2", ...],
    "plaintiff_strengths": ["..."],
    "defendant_strengths": ["..."],
    "recommended_approach": "...",
    "settlement_range": {{"min": 0, "max": 0}},
    "timeline_estimate": "...",
    "likelihood_of_success": 0.75
}}"#,
            jurisdiction = jurisdiction_context,
            preference = self.resolution_preference.description(),
            dispute = dispute_details
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response) {
                    result.content = parsed["summary"]
                        .as_str()
                        .unwrap_or("No summary available")
                        .to_string();

                    result.confidence = parsed["likelihood_of_success"].as_f64().unwrap_or(0.5);

                    if let Some(approach) = parsed["recommended_approach"].as_str() {
                        result.add_recommendation(approach);
                    }

                    result.metadata = parsed;
                    result.status = AgentTaskStatus::Completed;
                } else {
                    result.content = response;
                    result.confidence = 0.7;
                    result.status = AgentTaskStatus::Completed;
                    result.add_warning("Response was not in expected JSON format");
                }

                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                Err(e).context("Dispute analysis failed")
            }
        }
    }

    /// Generates a settlement proposal.
    pub async fn generate_settlement_proposal(
        &self,
        dispute_summary: &str,
        target_amount: Option<f64>,
    ) -> Result<AgentTaskResult> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let mut result = AgentTaskResult::new(&task_id).with_status(AgentTaskStatus::InProgress);

        let amount_guidance = target_amount
            .map(|a| format!("Target settlement amount: ${:.2}", a))
            .unwrap_or_else(|| "No specific target amount".to_string());

        let prompt = format!(
            r#"Generate a settlement proposal for this dispute:

{dispute}

{amount_guidance}

Create a comprehensive settlement proposal including:
1. Proposed settlement terms
2. Payment structure (if applicable)
3. Non-monetary terms
4. Release and waiver language
5. Implementation timeline
6. Confidentiality provisions

Provide the full settlement proposal text."#,
            dispute = dispute_summary,
            amount_guidance = amount_guidance
        );

        match self.provider.generate_text(&prompt).await {
            Ok(response) => {
                result.content = response;
                result.confidence = 0.8;
                result.status = AgentTaskStatus::Completed;
                result.add_recommendation("Review with legal counsel before submission");
                Ok(result)
            }
            Err(e) => {
                result.status = AgentTaskStatus::Failed;
                Err(e).context("Settlement proposal generation failed")
            }
        }
    }

    /// Evaluates the strength of a legal position in a dispute.
    pub async fn evaluate_legal_position(&self, position: &str) -> Result<f64> {
        let prompt = format!(
            r#"Evaluate the strength of this legal position on a scale of 0.0 to 1.0:

{position}

Consider:
- Legal merit
- Evidence quality
- Applicable precedents
- Potential defenses

Respond with just a number between 0.0 and 1.0."#,
            position = position
        );

        let response = self.provider.generate_text(&prompt).await?;
        let score = response
            .trim()
            .parse::<f64>()
            .unwrap_or(0.5)
            .clamp(0.0, 1.0);

        Ok(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_task_result_creation() {
        let result = AgentTaskResult::new("test-123");
        assert_eq!(result.task_id, "test-123");
        assert_eq!(result.status, AgentTaskStatus::Queued);
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_agent_task_result_builder() {
        let mut result = AgentTaskResult::new("test-456")
            .with_status(AgentTaskStatus::Completed)
            .with_content("Test content")
            .with_confidence(0.95);

        result.add_warning("Test warning");
        result.add_recommendation("Test recommendation");

        assert_eq!(result.status, AgentTaskStatus::Completed);
        assert_eq!(result.content, "Test content");
        assert!((result.confidence - 0.95).abs() < f64::EPSILON);
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.recommendations.len(), 1);
    }

    #[test]
    fn test_negotiation_style_description() {
        assert_eq!(
            NegotiationStyle::Collaborative.description(),
            "collaborative, seeking mutually beneficial outcomes"
        );
        assert_eq!(
            NegotiationStyle::Competitive.description(),
            "competitive, assertively pursuing client interests"
        );
    }

    #[test]
    fn test_resolution_preference_description() {
        assert_eq!(
            ResolutionPreference::Mediation.description(),
            "mediation and settlement"
        );
        assert_eq!(
            ResolutionPreference::Arbitration.description(),
            "arbitration"
        );
    }

    #[test]
    fn test_confidence_clamping() {
        let result = AgentTaskResult::new("test").with_confidence(1.5);
        assert!((result.confidence - 1.0).abs() < f64::EPSILON);

        let result = AgentTaskResult::new("test").with_confidence(-0.5);
        assert!(result.confidence.abs() < f64::EPSILON);
    }
}
