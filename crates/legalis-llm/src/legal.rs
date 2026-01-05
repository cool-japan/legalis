//! Legal-specific LLM features for document analysis and processing.
//!
//! This module provides specialized functionality for legal document summarization,
//! case law analysis, contract clause extraction, and jurisdiction-aware processing.

use crate::LLMProvider;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Legal jurisdiction for context-aware analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Jurisdiction {
    /// United States federal law
    UsFederal,
    /// United States state law
    UsState(String),
    /// United Kingdom
    Uk,
    /// European Union
    Eu,
    /// Canada federal
    CanadaFederal,
    /// Canada provincial
    CanadaProvince(String),
    /// Australia
    Australia,
    /// Japan
    Japan,
    /// Custom jurisdiction
    Custom(String),
}

impl Jurisdiction {
    /// Returns a description of the jurisdiction for prompts.
    pub fn description(&self) -> String {
        match self {
            Jurisdiction::UsFederal => "United States federal law".to_string(),
            Jurisdiction::UsState(state) => format!("{} state law, United States", state),
            Jurisdiction::Uk => "United Kingdom law".to_string(),
            Jurisdiction::Eu => "European Union law".to_string(),
            Jurisdiction::CanadaFederal => "Canadian federal law".to_string(),
            Jurisdiction::CanadaProvince(province) => {
                format!("{} provincial law, Canada", province)
            }
            Jurisdiction::Australia => "Australian law".to_string(),
            Jurisdiction::Japan => "Japanese law".to_string(),
            Jurisdiction::Custom(name) => name.clone(),
        }
    }
}

/// Legal document type for specialized processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalDocumentType {
    /// Court case or decision
    CaseLaw,
    /// Legislative statute or act
    Statute,
    /// Contract or agreement
    Contract,
    /// Legal brief or memorandum
    Brief,
    /// Regulation or rule
    Regulation,
    /// Legal opinion
    Opinion,
}

/// Citation extracted from a legal document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalCitation {
    /// Citation text (e.g., "Brown v. Board of Education, 347 U.S. 483 (1954)")
    pub citation: String,
    /// Type of citation (case, statute, regulation, etc.)
    pub citation_type: String,
    /// Context where the citation appears
    pub context: Option<String>,
}

/// Summary of a legal document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalDocumentSummary {
    /// Brief summary (1-2 sentences)
    pub brief: String,
    /// Detailed summary
    pub detailed: String,
    /// Key points or holdings
    pub key_points: Vec<String>,
    /// Citations found in the document
    pub citations: Vec<LegalCitation>,
    /// Document type
    pub document_type: String,
}

/// Case law analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseLawAnalysis {
    /// Case name
    pub case_name: String,
    /// Court that decided the case
    pub court: String,
    /// Decision date
    pub date: Option<String>,
    /// Legal issues addressed
    pub issues: Vec<String>,
    /// Holdings (legal conclusions)
    pub holdings: Vec<String>,
    /// Reasoning (rationale for the decision)
    pub reasoning: String,
    /// Important citations referenced
    pub citations: Vec<LegalCitation>,
    /// Key legal principles established
    pub legal_principles: Vec<String>,
}

/// Contract clause information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClause {
    /// Clause title or type
    pub title: String,
    /// Clause text
    pub text: String,
    /// Clause category (e.g., "Payment Terms", "Liability", "Termination")
    pub category: String,
    /// Risk assessment (Low, Medium, High)
    pub risk_level: Option<String>,
    /// Analysis or notes
    pub analysis: Option<String>,
}

/// Contract analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAnalysis {
    /// Contract title or type
    pub title: String,
    /// Parties involved
    pub parties: Vec<String>,
    /// Extracted clauses
    pub clauses: Vec<ContractClause>,
    /// Key dates (effective date, termination date, etc.)
    pub key_dates: Vec<String>,
    /// Overall risk assessment
    pub risk_assessment: String,
    /// Recommendations or concerns
    pub recommendations: Vec<String>,
}

/// Legal argument structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalArgument {
    /// Argument conclusion or thesis
    pub conclusion: String,
    /// Supporting premises or reasons
    pub premises: Vec<String>,
    /// Supporting authority (cases, statutes, etc.)
    pub authority: Vec<LegalCitation>,
    /// Counter-arguments to address
    pub counter_arguments: Vec<String>,
    /// Rebuttals to counter-arguments
    pub rebuttals: Vec<String>,
}

/// Legal document summarizer.
pub struct LegalDocumentSummarizer<P> {
    provider: P,
    jurisdiction: Option<Jurisdiction>,
}

impl<P: LLMProvider> LegalDocumentSummarizer<P> {
    /// Creates a new legal document summarizer.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            jurisdiction: None,
        }
    }

    /// Sets the jurisdiction for context-aware analysis.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Summarizes a legal document with citation extraction.
    pub async fn summarize(
        &self,
        document: &str,
        document_type: LegalDocumentType,
    ) -> Result<LegalDocumentSummary> {
        let doc_type_str = match document_type {
            LegalDocumentType::CaseLaw => "case law",
            LegalDocumentType::Statute => "statute",
            LegalDocumentType::Contract => "contract",
            LegalDocumentType::Brief => "legal brief",
            LegalDocumentType::Regulation => "regulation",
            LegalDocumentType::Opinion => "legal opinion",
        };

        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in the context of {}", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"Analyze the following legal {} document{} and provide a structured summary.

Document:
{}

Please provide your analysis in the following JSON format:
{{
    "brief": "1-2 sentence summary",
    "detailed": "Detailed summary (2-3 paragraphs)",
    "key_points": ["Key point 1", "Key point 2", ...],
    "citations": [
        {{
            "citation": "Full citation text",
            "citation_type": "case/statute/regulation/other",
            "context": "Context where citation appears"
        }}
    ],
    "document_type": "{}"
}}

Extract all legal citations accurately and provide comprehensive key points."#,
            doc_type_str, jurisdiction_context, document, doc_type_str
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to generate legal document summary")
    }

    /// Analyzes case law and extracts key legal information.
    pub async fn analyze_case_law(&self, case_text: &str) -> Result<CaseLawAnalysis> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" Analyze in the context of {}.", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"Analyze the following case law and extract key legal information.{}

Case text:
{}

Provide your analysis in the following JSON format:
{{
    "case_name": "Case name",
    "court": "Court name",
    "date": "Decision date (if available)",
    "issues": ["Legal issue 1", "Legal issue 2", ...],
    "holdings": ["Holding 1", "Holding 2", ...],
    "reasoning": "Detailed reasoning and rationale",
    "citations": [
        {{
            "citation": "Citation text",
            "citation_type": "case/statute/other",
            "context": "Context"
        }}
    ],
    "legal_principles": ["Principle 1", "Principle 2", ...]
}}

Extract all relevant information accurately."#,
            jurisdiction_context, case_text
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to analyze case law")
    }

    /// Extracts and analyzes contract clauses.
    pub async fn analyze_contract(&self, contract_text: &str) -> Result<ContractAnalysis> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" Analyze in the context of {}.", j.description()))
            .unwrap_or_default();

        let prompt = format!(
            r#"Analyze the following contract and extract key information and clauses.{}

Contract text:
{}

Provide your analysis in the following JSON format:
{{
    "title": "Contract title or type",
    "parties": ["Party 1", "Party 2", ...],
    "clauses": [
        {{
            "title": "Clause title",
            "text": "Clause text",
            "category": "Payment Terms/Liability/Termination/etc.",
            "risk_level": "Low/Medium/High",
            "analysis": "Brief analysis of the clause"
        }}
    ],
    "key_dates": ["Date 1", "Date 2", ...],
    "risk_assessment": "Overall risk assessment",
    "recommendations": ["Recommendation 1", "Recommendation 2", ...]
}}

Focus on identifying potential risks and important provisions."#,
            jurisdiction_context, contract_text
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to analyze contract")
    }

    /// Generates a legal argument on a given issue.
    pub async fn generate_argument(
        &self,
        issue: &str,
        position: &str,
        supporting_facts: &[String],
    ) -> Result<LegalArgument> {
        let jurisdiction_context = self
            .jurisdiction
            .as_ref()
            .map(|j| format!(" in the context of {}", j.description()))
            .unwrap_or_default();

        let facts_str = supporting_facts
            .iter()
            .enumerate()
            .map(|(i, fact)| format!("{}. {}", i + 1, fact))
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"Generate a legal argument{} for the following issue.

Issue: {}
Position: {}

Supporting facts:
{}

Provide the argument in the following JSON format:
{{
    "conclusion": "Main conclusion or thesis",
    "premises": ["Premise 1", "Premise 2", ...],
    "authority": [
        {{
            "citation": "Supporting authority citation",
            "citation_type": "case/statute/other",
            "context": "How this authority supports the argument"
        }}
    ],
    "counter_arguments": ["Counter-argument 1", "Counter-argument 2", ...],
    "rebuttals": ["Rebuttal to counter-argument 1", "Rebuttal to counter-argument 2", ...]
}}

Provide well-reasoned arguments with appropriate legal authority."#,
            jurisdiction_context, issue, position, facts_str
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to generate legal argument")
    }
}

/// Legal citation extractor.
pub struct CitationExtractor<P> {
    provider: P,
}

impl<P: LLMProvider> CitationExtractor<P> {
    /// Creates a new citation extractor.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Extracts all legal citations from text.
    pub async fn extract_citations(&self, text: &str) -> Result<Vec<LegalCitation>> {
        let prompt = format!(
            r#"Extract all legal citations from the following text.

Text:
{}

Provide the citations in the following JSON format:
{{
    "citations": [
        {{
            "citation": "Full citation text",
            "citation_type": "case/statute/regulation/other",
            "context": "Surrounding context or sentence"
        }}
    ]
}}

Extract all citations accurately, including case citations, statute references, and regulatory citations."#,
            text
        );

        #[derive(Deserialize)]
        struct CitationsResponse {
            citations: Vec<LegalCitation>,
        }

        let response: CitationsResponse = self
            .provider
            .generate_structured(&prompt)
            .await
            .context("Failed to extract citations")?;

        Ok(response.citations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::MockProvider;

    #[test]
    fn test_jurisdiction_description() {
        assert_eq!(
            Jurisdiction::UsFederal.description(),
            "United States federal law"
        );
        assert_eq!(
            Jurisdiction::UsState("California".to_string()).description(),
            "California state law, United States"
        );
        assert_eq!(
            Jurisdiction::Custom("International Law".to_string()).description(),
            "International Law"
        );
    }

    #[tokio::test]
    async fn test_legal_document_summarizer() {
        let mock_response = r#"{
            "brief": "Brief summary of the case",
            "detailed": "Detailed summary with more information",
            "key_points": ["Point 1", "Point 2"],
            "citations": [
                {
                    "citation": "Test v. Example, 123 U.S. 456 (2020)",
                    "citation_type": "case",
                    "context": "Referenced in the analysis"
                }
            ],
            "document_type": "case law"
        }"#;

        let provider = MockProvider::new().with_response("Analyze", mock_response);
        let summarizer =
            LegalDocumentSummarizer::new(provider).with_jurisdiction(Jurisdiction::UsFederal);

        let summary = summarizer
            .summarize("Test case text", LegalDocumentType::CaseLaw)
            .await
            .unwrap();

        assert_eq!(summary.brief, "Brief summary of the case");
        assert_eq!(summary.key_points.len(), 2);
        assert_eq!(summary.citations.len(), 1);
    }

    #[tokio::test]
    async fn test_citation_extractor() {
        let mock_response = r#"{
            "citations": [
                {
                    "citation": "Brown v. Board of Education, 347 U.S. 483 (1954)",
                    "citation_type": "case",
                    "context": "Landmark civil rights case"
                }
            ]
        }"#;

        let provider = MockProvider::new().with_response("Extract", mock_response);
        let extractor = CitationExtractor::new(provider);

        let citations = extractor
            .extract_citations("Test text with citations")
            .await
            .unwrap();

        assert_eq!(citations.len(), 1);
        assert_eq!(citations[0].citation_type, "case");
    }

    #[test]
    fn test_legal_document_type() {
        assert_eq!(LegalDocumentType::CaseLaw, LegalDocumentType::CaseLaw);
        assert_ne!(LegalDocumentType::CaseLaw, LegalDocumentType::Statute);
    }

    #[test]
    fn test_legal_citation_creation() {
        let citation = LegalCitation {
            citation: "Test citation".to_string(),
            citation_type: "case".to_string(),
            context: Some("Test context".to_string()),
        };

        assert_eq!(citation.citation, "Test citation");
        assert_eq!(citation.citation_type, "case");
        assert!(citation.context.is_some());
    }
}
