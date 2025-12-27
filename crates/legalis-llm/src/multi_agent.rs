//! Multi-agent system for legal analysis.
//!
//! This module provides specialized AI agents that work together to perform
//! complex legal tasks like statute interpretation, verification, drafting, and research.

use crate::{LLMProvider, legal::{Jurisdiction, LegalCitation}};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Agent role in the multi-agent system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentRole {
    /// Legal expert for statute interpretation
    LegalExpert,
    /// Reviewer for verification and quality control
    Reviewer,
    /// Drafter for statute and document generation
    Drafter,
    /// Researcher for case law and precedent search
    Researcher,
}

impl AgentRole {
    /// Returns a description of the agent's role.
    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match self {
            AgentRole::LegalExpert => "Legal expert specializing in statute interpretation and legal analysis",
            AgentRole::Reviewer => "Legal reviewer for verification, quality control, and consistency checking",
            AgentRole::Drafter => "Legal drafter for creating statutes, contracts, and legal documents",
            AgentRole::Researcher => "Legal researcher for finding case law, precedents, and legal authorities",
        }
    }
}

/// Result from an agent's analysis or task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    /// Agent role that produced this result
    pub agent_role: String,
    /// Task or query processed
    pub task: String,
    /// Result content
    pub content: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Supporting evidence or citations
    pub evidence: Vec<String>,
    /// Warnings or concerns
    pub warnings: Vec<String>,
}

/// Legal expert agent for statute interpretation.
pub struct LegalExpertAgent<P> {
    provider: P,
    jurisdiction: Option<Jurisdiction>,
}

impl<P: LLMProvider> LegalExpertAgent<P> {
    /// Creates a new legal expert agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            jurisdiction: None,
        }
    }

    /// Sets the jurisdiction for legal analysis.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Interprets a statute and provides expert analysis.
    pub async fn interpret_statute(&self, statute_text: &str, query: &str) -> Result<AgentResult> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in the context of {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"You are a legal expert{jurisdiction}. Interpret the following statute and answer the query.

Statute:
{statute}

Query: {query}

Provide your interpretation in the following JSON format:
{{
    "agent_role": "legal_expert",
    "task": "{query}",
    "content": "Your detailed interpretation and analysis",
    "confidence": 0.85,
    "evidence": ["Evidence or reasoning 1", "Evidence 2", ...],
    "warnings": ["Warning or caveat 1", "Warning 2", ...]
}}

Provide thorough legal analysis with supporting reasoning."#,
            jurisdiction = jurisdiction_context,
            statute = statute_text,
            query = query
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to interpret statute")
    }

    /// Analyzes legal compliance of a document or action.
    pub async fn analyze_compliance(
        &self,
        document: &str,
        requirements: &[String],
    ) -> Result<AgentResult> {
        let requirements_str = requirements
            .iter()
            .enumerate()
            .map(|(i, req)| format!("{}. {}", i + 1, req))
            .collect::<Vec<_>>()
            .join("\n");

        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" under {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"You are a legal expert. Analyze the following document for compliance{jurisdiction}.

Document:
{document}

Requirements to check:
{requirements}

Provide your compliance analysis in the following JSON format:
{{
    "agent_role": "legal_expert",
    "task": "Compliance analysis",
    "content": "Detailed compliance assessment",
    "confidence": 0.90,
    "evidence": ["Evidence of compliance or non-compliance"],
    "warnings": ["Potential compliance issues or risks"]
}}

Assess whether the document meets all requirements."#,
            jurisdiction = jurisdiction_context,
            document = document,
            requirements = requirements_str
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to analyze compliance")
    }
}

/// Reviewer agent for verification and quality control.
pub struct ReviewerAgent<P> {
    provider: P,
}

impl<P: LLMProvider> ReviewerAgent<P> {
    /// Creates a new reviewer agent.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Reviews a legal document for consistency and correctness.
    pub async fn review_document(&self, document: &str) -> Result<AgentResult> {
        let prompt = format!(
            r#"You are a legal reviewer. Review the following legal document for consistency, correctness, and quality.

Document:
{document}

Provide your review in the following JSON format:
{{
    "agent_role": "reviewer",
    "task": "Document review",
    "content": "Detailed review findings",
    "confidence": 0.88,
    "evidence": ["Issue or observation 1", "Issue 2", ...],
    "warnings": ["Critical issue 1", "Critical issue 2", ...]
}}

Check for:
- Internal consistency
- Logical coherence
- Completeness
- Ambiguities or unclear language
- Potential conflicts or contradictions
- Formatting and structural issues"#,
            document = document
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to review document")
    }

    /// Verifies legal reasoning and arguments.
    pub async fn verify_reasoning(&self, argument: &str) -> Result<AgentResult> {
        let prompt = format!(
            r#"You are a legal reviewer. Verify the following legal reasoning or argument.

Argument:
{argument}

Provide your verification in the following JSON format:
{{
    "agent_role": "reviewer",
    "task": "Reasoning verification",
    "content": "Verification results and assessment",
    "confidence": 0.87,
    "evidence": ["Strong point 1", "Weak point 2", ...],
    "warnings": ["Logical flaw 1", "Unsupported claim 2", ...]
}}

Assess:
- Logical validity
- Factual accuracy
- Supporting evidence
- Potential counter-arguments
- Gaps in reasoning"#,
            argument = argument
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to verify reasoning")
    }
}

/// Drafter agent for creating legal documents.
pub struct DrafterAgent<P> {
    provider: P,
    jurisdiction: Option<Jurisdiction>,
}

impl<P: LLMProvider> DrafterAgent<P> {
    /// Creates a new drafter agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            jurisdiction: None,
        }
    }

    /// Sets the jurisdiction for drafting.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Drafts a statute based on requirements.
    pub async fn draft_statute(
        &self,
        title: &str,
        requirements: &[String],
    ) -> Result<AgentResult> {
        let requirements_str = requirements
            .iter()
            .enumerate()
            .map(|(i, req)| format!("{}. {}", i + 1, req))
            .collect::<Vec<_>>()
            .join("\n");

        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" for {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"You are a legal drafter. Draft a statute{jurisdiction} based on the following requirements.

Title: {title}

Requirements:
{requirements}

Provide your draft in the following JSON format:
{{
    "agent_role": "drafter",
    "task": "Statute drafting",
    "content": "Complete drafted statute text",
    "confidence": 0.85,
    "evidence": ["Drafting decision 1", "Consideration 2", ...],
    "warnings": ["Area needing review 1", "Potential issue 2", ...]
}}

Draft clear, precise, and legally sound language."#,
            jurisdiction = jurisdiction_context,
            title = title,
            requirements = requirements_str
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to draft statute")
    }

    /// Drafts a legal clause or provision.
    pub async fn draft_clause(&self, clause_type: &str, parameters: &str) -> Result<AgentResult> {
        let prompt = format!(
            r#"You are a legal drafter. Draft a {clause_type} clause with the following parameters.

Parameters:
{parameters}

Provide your draft in the following JSON format:
{{
    "agent_role": "drafter",
    "task": "Clause drafting",
    "content": "Complete drafted clause text",
    "confidence": 0.90,
    "evidence": ["Drafting rationale 1", "Consideration 2", ...],
    "warnings": ["Review point 1", "Consideration 2", ...]
}}

Draft clear and enforceable language."#,
            clause_type = clause_type,
            parameters = parameters
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to draft clause")
    }
}

/// Researcher agent for finding case law and precedents.
pub struct ResearcherAgent<P> {
    provider: P,
    jurisdiction: Option<Jurisdiction>,
}

impl<P: LLMProvider> ResearcherAgent<P> {
    /// Creates a new researcher agent.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            jurisdiction: None,
        }
    }

    /// Sets the jurisdiction for research.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Researches case law on a specific legal issue.
    pub async fn research_case_law(&self, legal_issue: &str) -> Result<AgentResult> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"You are a legal researcher. Research case law{jurisdiction} on the following legal issue.

Legal issue: {issue}

Provide your research findings in the following JSON format:
{{
    "agent_role": "researcher",
    "task": "Case law research",
    "content": "Summary of relevant case law and precedents",
    "confidence": 0.82,
    "evidence": [
        "Case citation 1 and its relevance",
        "Case citation 2 and its relevance",
        ...
    ],
    "warnings": ["Limitation 1", "Potential conflict 2", ...]
}}

Identify relevant cases, holdings, and legal principles."#,
            jurisdiction = jurisdiction_context,
            issue = legal_issue
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to research case law")
    }

    /// Finds supporting authority for a legal argument.
    pub async fn find_supporting_authority(
        &self,
        argument: &str,
    ) -> Result<Vec<LegalCitation>> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"You are a legal researcher. Find supporting legal authority{jurisdiction} for the following argument.

Argument: {argument}

Provide your findings in the following JSON format:
{{
    "citations": [
        {{
            "citation": "Full citation",
            "citation_type": "case/statute/regulation",
            "context": "How this supports the argument"
        }}
    ]
}}

Find the most relevant and authoritative sources."#,
            jurisdiction = jurisdiction_context,
            argument = argument
        );

        #[derive(Deserialize)]
        struct CitationsResponse {
            citations: Vec<LegalCitation>,
        }

        let response: CitationsResponse = self
            .provider
            .generate_structured(&prompt)
            .await
            .context("Failed to find supporting authority")?;

        Ok(response.citations)
    }
}

/// Multi-agent orchestrator for coordinating legal analysis tasks.
pub struct AgentOrchestrator<P> {
    expert: Arc<LegalExpertAgent<P>>,
    reviewer: Arc<ReviewerAgent<P>>,
    drafter: Arc<DrafterAgent<P>>,
    researcher: Arc<ResearcherAgent<P>>,
}

impl<P: LLMProvider + Clone> AgentOrchestrator<P> {
    /// Creates a new agent orchestrator.
    pub fn new(provider: P, jurisdiction: Option<Jurisdiction>) -> Self {
        let expert = if let Some(ref j) = jurisdiction {
            LegalExpertAgent::new(provider.clone()).with_jurisdiction(j.clone())
        } else {
            LegalExpertAgent::new(provider.clone())
        };

        let drafter = if let Some(ref j) = jurisdiction {
            DrafterAgent::new(provider.clone()).with_jurisdiction(j.clone())
        } else {
            DrafterAgent::new(provider.clone())
        };

        let researcher = if let Some(j) = jurisdiction {
            ResearcherAgent::new(provider.clone()).with_jurisdiction(j)
        } else {
            ResearcherAgent::new(provider.clone())
        };

        Self {
            expert: Arc::new(expert),
            reviewer: Arc::new(ReviewerAgent::new(provider)),
            drafter: Arc::new(drafter),
            researcher: Arc::new(researcher),
        }
    }

    /// Performs a complete legal analysis workflow.
    ///
    /// 1. Researcher finds relevant case law
    /// 2. Expert interprets and analyzes
    /// 3. Drafter creates a response or document
    /// 4. Reviewer checks quality and consistency
    pub async fn analyze_and_draft(
        &self,
        issue: &str,
        requirements: &[String],
    ) -> Result<Vec<AgentResult>> {
        let mut results = Vec::new();

        // Step 1: Research
        let research_result = self.researcher.research_case_law(issue).await?;
        results.push(research_result);

        // Step 2: Expert analysis
        let statute_text = ""; // Would be provided or retrieved
        let expert_result = self.expert.interpret_statute(statute_text, issue).await?;
        results.push(expert_result);

        // Step 3: Drafting
        let draft_result = self.drafter.draft_statute(issue, requirements).await?;
        results.push(draft_result.clone());

        // Step 4: Review
        let review_result = self.reviewer.review_document(&draft_result.content).await?;
        results.push(review_result);

        Ok(results)
    }

    /// Gets the expert agent.
    #[allow(dead_code)]
    pub fn expert(&self) -> &LegalExpertAgent<P> {
        &self.expert
    }

    /// Gets the reviewer agent.
    #[allow(dead_code)]
    pub fn reviewer(&self) -> &ReviewerAgent<P> {
        &self.reviewer
    }

    /// Gets the drafter agent.
    #[allow(dead_code)]
    pub fn drafter(&self) -> &DrafterAgent<P> {
        &self.drafter
    }

    /// Gets the researcher agent.
    #[allow(dead_code)]
    pub fn researcher(&self) -> &ResearcherAgent<P> {
        &self.researcher
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{providers::MockProvider, legal::Jurisdiction};

    #[tokio::test]
    async fn test_legal_expert_agent() {
        let mock_response = r#"{
            "agent_role": "legal_expert",
            "task": "Test query",
            "content": "Expert interpretation",
            "confidence": 0.85,
            "evidence": ["Evidence 1"],
            "warnings": []
        }"#;

        let provider = MockProvider::new().with_response("legal expert", mock_response);
        let agent = LegalExpertAgent::new(provider).with_jurisdiction(Jurisdiction::UsFederal);

        let result = agent
            .interpret_statute("Test statute", "Test query")
            .await
            .unwrap();

        assert_eq!(result.agent_role, "legal_expert");
        assert_eq!(result.confidence, 0.85);
    }

    #[tokio::test]
    async fn test_reviewer_agent() {
        let mock_response = r#"{
            "agent_role": "reviewer",
            "task": "Document review",
            "content": "Review findings",
            "confidence": 0.88,
            "evidence": ["Issue 1"],
            "warnings": ["Warning 1"]
        }"#;

        let provider = MockProvider::new().with_response("legal reviewer", mock_response);
        let agent = ReviewerAgent::new(provider);

        let result = agent.review_document("Test document").await.unwrap();

        assert_eq!(result.agent_role, "reviewer");
        assert_eq!(result.warnings.len(), 1);
    }

    #[tokio::test]
    async fn test_drafter_agent() {
        let mock_response = r#"{
            "agent_role": "drafter",
            "task": "Statute drafting",
            "content": "Drafted statute text",
            "confidence": 0.85,
            "evidence": ["Decision 1"],
            "warnings": []
        }"#;

        let provider = MockProvider::new().with_response("legal drafter", mock_response);
        let agent = DrafterAgent::new(provider);

        let result = agent
            .draft_statute("Test Statute", &vec!["Requirement 1".to_string()])
            .await
            .unwrap();

        assert_eq!(result.agent_role, "drafter");
    }

    #[test]
    fn test_agent_role_description() {
        assert!(!AgentRole::LegalExpert.description().is_empty());
        assert!(!AgentRole::Reviewer.description().is_empty());
    }
}
