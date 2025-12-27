//! Legal-specific prompt engineering techniques.
//!
//! This module provides advanced prompting strategies tailored for legal analysis,
//! including chain-of-law reasoning, multi-step workflows, citation-grounded generation,
//! and statutory interpretation.

use crate::{LLMProvider, legal::{Jurisdiction, LegalCitation}};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Chain-of-law reasoning step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalReasoningStep {
    /// Step number
    pub step: usize,
    /// Legal principle or rule applied
    pub principle: String,
    /// Analysis or reasoning
    pub reasoning: String,
    /// Supporting citations
    pub citations: Vec<String>,
    /// Conclusion reached at this step
    pub conclusion: String,
}

/// Complete chain-of-law reasoning result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainOfLawResult {
    /// Original legal question
    pub question: String,
    /// Reasoning steps
    pub steps: Vec<LegalReasoningStep>,
    /// Final conclusion
    pub final_conclusion: String,
    /// Overall confidence (0.0 - 1.0)
    pub confidence: f64,
}

/// Multi-step legal analysis workflow result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalAnalysisWorkflow {
    /// Issue identification
    pub issues: Vec<String>,
    /// Relevant law identification
    pub relevant_laws: Vec<String>,
    /// Application of law to facts
    pub application: String,
    /// Conclusion
    pub conclusion: String,
    /// Alternative arguments
    pub alternatives: Vec<String>,
}

/// Citation-grounded generation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationGroundedResult {
    /// Generated content
    pub content: String,
    /// Citations supporting each claim
    pub supporting_citations: Vec<CitationSupport>,
    /// Confidence in citation accuracy
    pub citation_confidence: f64,
}

/// Citation support for a specific claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationSupport {
    /// The claim being supported
    pub claim: String,
    /// Citations supporting this claim
    pub citations: Vec<LegalCitation>,
}

/// Legal precedent match result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecedentMatch {
    /// Case citation
    pub citation: String,
    /// Similarity score (0.0 - 1.0)
    pub similarity: f64,
    /// Key facts that match
    pub matching_facts: Vec<String>,
    /// Holdings relevant to the query
    pub relevant_holdings: Vec<String>,
    /// How this precedent applies
    pub application: String,
}

/// Statutory interpretation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatutoryInterpretation {
    /// The statute being interpreted
    pub statute: String,
    /// Plain meaning interpretation
    pub plain_meaning: String,
    /// Legislative intent (if discernible)
    pub legislative_intent: Option<String>,
    /// Canons of construction applied
    pub canons_applied: Vec<String>,
    /// Interpretation conclusion
    pub interpretation: String,
    /// Ambiguities identified
    pub ambiguities: Vec<String>,
}

/// Chain-of-law prompter for step-by-step legal reasoning.
pub struct ChainOfLawPrompter<P> {
    provider: P,
    jurisdiction: Option<Jurisdiction>,
}

impl<P: LLMProvider> ChainOfLawPrompter<P> {
    /// Creates a new chain-of-law prompter.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            jurisdiction: None,
        }
    }

    /// Sets the jurisdiction for legal reasoning.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Performs chain-of-law reasoning on a legal question.
    pub async fn reason(&self, question: &str, facts: &[String]) -> Result<ChainOfLawResult> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in the context of {}", j.description()))
            .unwrap_or_default();

        let facts_str = facts
            .iter()
            .enumerate()
            .map(|(i, fact)| format!("{}. {}", i + 1, fact))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"Perform step-by-step legal reasoning (chain-of-law) to answer the following legal question{jurisdiction}.

Question: {question}

Facts:
{facts}

Use the following chain-of-law reasoning format:

Step 1: Identify the relevant legal principle or rule
Step 2: Analyze how the principle applies to the facts
Step 3: Consider any exceptions or limitations
Step 4: Draw intermediate conclusions
Step 5: Apply additional principles if needed
Step N: Reach final conclusion

Provide your reasoning in the following JSON format:
{{
    "question": "{question}",
    "steps": [
        {{
            "step": 1,
            "principle": "Legal principle or rule",
            "reasoning": "Detailed analysis",
            "citations": ["Citation 1", "Citation 2"],
            "conclusion": "Intermediate conclusion"
        }}
    ],
    "final_conclusion": "Final conclusion based on all steps",
    "confidence": 0.85
}}

Show your work clearly at each step with supporting legal authority."#,
            jurisdiction = jurisdiction_context,
            question = question,
            facts = facts_str
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to perform chain-of-law reasoning")
    }
}

/// Multi-step legal analysis workflow prompter (IRAC method).
pub struct LegalAnalysisWorkflowPrompter<P> {
    provider: P,
}

impl<P: LLMProvider> LegalAnalysisWorkflowPrompter<P> {
    /// Creates a new legal analysis workflow prompter.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Performs IRAC (Issue, Rule, Application, Conclusion) analysis.
    pub async fn analyze_irac(&self, scenario: &str) -> Result<LegalAnalysisWorkflow> {
        let prompt = format!(
            r#"Perform a comprehensive legal analysis using the IRAC method (Issue, Rule, Application, Conclusion).

Scenario:
{scenario}

Provide your analysis in the following JSON format:
{{
    "issues": ["Legal issue 1", "Legal issue 2", ...],
    "relevant_laws": ["Relevant law 1", "Relevant law 2", ...],
    "application": "Detailed application of law to facts",
    "conclusion": "Legal conclusion",
    "alternatives": ["Alternative argument 1", "Alternative argument 2", ...]
}}

Be thorough in identifying all issues and applying relevant law."#,
            scenario = scenario
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to perform IRAC analysis")
    }

    /// Performs multi-step analysis with custom workflow.
    pub async fn analyze_custom(
        &self,
        scenario: &str,
        steps: &[String],
    ) -> Result<serde_json::Value> {
        let steps_str = steps
            .iter()
            .enumerate()
            .map(|(i, step)| format!("Step {}: {}", i + 1, step))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"Perform a multi-step legal analysis following these steps:

{steps}

Scenario:
{scenario}

Provide a structured JSON response with results for each step."#,
            steps = steps_str,
            scenario = scenario
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to perform custom workflow analysis")
    }
}

/// Citation-grounded generation prompter.
pub struct CitationGroundedPrompter<P> {
    provider: P,
}

impl<P: LLMProvider> CitationGroundedPrompter<P> {
    /// Creates a new citation-grounded prompter.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Generates content with citation support for each claim.
    pub async fn generate_with_citations(
        &self,
        topic: &str,
        requirements: &[String],
    ) -> Result<CitationGroundedResult> {
        let requirements_str = requirements
            .iter()
            .enumerate()
            .map(|(i, req)| format!("{}. {}", i + 1, req))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"Generate content on the following legal topic, ensuring every significant claim is supported by legal citations.

Topic: {topic}

Requirements:
{requirements}

Provide your response in the following JSON format:
{{
    "content": "Generated content with inline citation markers",
    "supporting_citations": [
        {{
            "claim": "Specific claim made",
            "citations": [
                {{
                    "citation": "Full legal citation",
                    "citation_type": "case/statute/regulation",
                    "context": "How this citation supports the claim"
                }}
            ]
        }}
    ],
    "citation_confidence": 0.90
}}

Every factual or legal claim must be supported by appropriate authority."#,
            topic = topic,
            requirements = requirements_str
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to generate citation-grounded content")
    }
}

/// Legal precedent matching prompter.
pub struct PrecedentMatchPrompter<P> {
    provider: P,
    jurisdiction: Option<Jurisdiction>,
}

impl<P: LLMProvider> PrecedentMatchPrompter<P> {
    /// Creates a new precedent match prompter.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            jurisdiction: None,
        }
    }

    /// Sets the jurisdiction for precedent matching.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Finds matching precedents for a given scenario.
    pub async fn find_precedents(
        &self,
        scenario: &str,
        key_facts: &[String],
    ) -> Result<Vec<PrecedentMatch>> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in {}", j.description()))
            .unwrap_or_default();

        let facts_str = key_facts
            .iter()
            .enumerate()
            .map(|(i, fact)| format!("{}. {}", i + 1, fact))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"Find legal precedents{jurisdiction} that match the following scenario.

Scenario:
{scenario}

Key facts:
{facts}

Provide matching precedents in the following JSON format:
{{
    "precedents": [
        {{
            "citation": "Full case citation",
            "similarity": 0.85,
            "matching_facts": ["Matching fact 1", "Matching fact 2"],
            "relevant_holdings": ["Holding 1", "Holding 2"],
            "application": "How this precedent applies to the current scenario"
        }}
    ]
}}

Focus on precedents with strong factual similarity and relevant legal holdings."#,
            jurisdiction = jurisdiction_context,
            scenario = scenario,
            facts = facts_str
        );

        #[derive(Deserialize)]
        struct PrecedentsResponse {
            precedents: Vec<PrecedentMatch>,
        }

        let response: PrecedentsResponse = self
            .provider
            .generate_structured(&prompt)
            .await
            .context("Failed to find precedents")?;

        Ok(response.precedents)
    }
}

/// Statutory interpretation prompter.
pub struct StatutoryInterpretationPrompter<P> {
    provider: P,
    jurisdiction: Option<Jurisdiction>,
}

impl<P: LLMProvider> StatutoryInterpretationPrompter<P> {
    /// Creates a new statutory interpretation prompter.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            jurisdiction: None,
        }
    }

    /// Sets the jurisdiction for interpretation.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Interprets a statute using various interpretation canons.
    pub async fn interpret(
        &self,
        statute_text: &str,
        question: &str,
    ) -> Result<StatutoryInterpretation> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" under {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"Interpret the following statute{jurisdiction} to answer the question, using established canons of statutory construction.

Statute:
{statute}

Question: {question}

Apply the following interpretation methods:
1. Plain meaning rule
2. Legislative intent (if discernible)
3. Canons of construction (e.g., ejusdem generis, expressio unius, noscitur a sociis)
4. Contextual analysis
5. Purpose-based interpretation

Provide your interpretation in the following JSON format:
{{
    "statute": "{statute}",
    "plain_meaning": "Plain language interpretation",
    "legislative_intent": "Legislative intent if discernible",
    "canons_applied": ["Canon 1", "Canon 2", ...],
    "interpretation": "Final interpretation",
    "ambiguities": ["Ambiguity 1", "Ambiguity 2", ...]
}}

Be thorough in considering all reasonable interpretations."#,
            jurisdiction = jurisdiction_context,
            statute = statute_text,
            question = question
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to interpret statute")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::MockProvider;

    #[tokio::test]
    async fn test_chain_of_law_prompting() {
        let mock_response = r#"{
            "question": "Test question",
            "steps": [
                {
                    "step": 1,
                    "principle": "Test principle",
                    "reasoning": "Test reasoning",
                    "citations": ["Citation 1"],
                    "conclusion": "Step conclusion"
                }
            ],
            "final_conclusion": "Final conclusion",
            "confidence": 0.85
        }"#;

        let provider = MockProvider::new().with_response("chain-of-law", mock_response);
        let prompter = ChainOfLawPrompter::new(provider)
            .with_jurisdiction(Jurisdiction::UsFederal);

        let result = prompter
            .reason("Test question", &vec!["Fact 1".to_string()])
            .await
            .unwrap();

        assert_eq!(result.steps.len(), 1);
        assert_eq!(result.confidence, 0.85);
    }

    #[tokio::test]
    async fn test_irac_analysis() {
        let mock_response = r#"{
            "issues": ["Issue 1"],
            "relevant_laws": ["Law 1"],
            "application": "Application text",
            "conclusion": "Conclusion",
            "alternatives": ["Alternative 1"]
        }"#;

        let provider = MockProvider::new().with_response("IRAC", mock_response);
        let prompter = LegalAnalysisWorkflowPrompter::new(provider);

        let result = prompter.analyze_irac("Test scenario").await.unwrap();

        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.relevant_laws.len(), 1);
    }

    #[tokio::test]
    async fn test_citation_grounded_generation() {
        let mock_response = r#"{
            "content": "Generated content",
            "supporting_citations": [
                {
                    "claim": "Test claim",
                    "citations": [
                        {
                            "citation": "Test citation",
                            "citation_type": "case",
                            "context": "Test context"
                        }
                    ]
                }
            ],
            "citation_confidence": 0.90
        }"#;

        let provider = MockProvider::new().with_response("citation", mock_response);
        let prompter = CitationGroundedPrompter::new(provider);

        let result = prompter
            .generate_with_citations("Test topic", &vec!["Requirement 1".to_string()])
            .await
            .unwrap();

        assert_eq!(result.citation_confidence, 0.90);
        assert_eq!(result.supporting_citations.len(), 1);
    }

    #[tokio::test]
    async fn test_precedent_matching() {
        let mock_response = r#"{
            "precedents": [
                {
                    "citation": "Test v. Example, 123 U.S. 456",
                    "similarity": 0.85,
                    "matching_facts": ["Fact 1"],
                    "relevant_holdings": ["Holding 1"],
                    "application": "Application"
                }
            ]
        }"#;

        let provider = MockProvider::new().with_response("precedent", mock_response);
        let prompter = PrecedentMatchPrompter::new(provider);

        let precedents = prompter
            .find_precedents("Test scenario", &vec!["Fact 1".to_string()])
            .await
            .unwrap();

        assert_eq!(precedents.len(), 1);
        assert_eq!(precedents[0].similarity, 0.85);
    }

    #[tokio::test]
    async fn test_statutory_interpretation() {
        let mock_response = r#"{
            "statute": "Test statute",
            "plain_meaning": "Plain meaning",
            "legislative_intent": "Intent",
            "canons_applied": ["Canon 1"],
            "interpretation": "Interpretation",
            "ambiguities": ["Ambiguity 1"]
        }"#;

        let provider = MockProvider::new().with_response("interpret", mock_response);
        let prompter = StatutoryInterpretationPrompter::new(provider);

        let result = prompter
            .interpret("Test statute", "Test question")
            .await
            .unwrap();

        assert_eq!(result.plain_meaning, "Plain meaning");
        assert_eq!(result.canons_applied.len(), 1);
    }
}
