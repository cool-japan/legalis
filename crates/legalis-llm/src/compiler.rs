//! Law compiler using LLM for natural language processing.

use crate::LLMProvider;
use anyhow::Result;
use legalis_core::Statute;

/// Compiles natural language legal text into structured Statute objects.
pub struct LawCompiler<P: LLMProvider> {
    provider: P,
}

impl<P: LLMProvider> LawCompiler<P> {
    /// Creates a new LawCompiler with the given LLM provider.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Compiles natural language statute text into a structured Statute.
    pub async fn compile(&self, raw_text: &str) -> Result<Statute> {
        let system_prompt = r#"You are a 'Legal Compiler'. Convert natural language statute text into Rust structures.
Mark any interpretive or discretionary parts as 'JudicialDiscretion'.
Respond with valid JSON matching this structure:
{
    "id": "statute-id",
    "title": "Statute Title",
    "preconditions": [],
    "effect": {
        "effect_type": "Grant|Revoke|Obligation|Prohibition|MonetaryTransfer|StatusChange|Custom",
        "description": "Effect description",
        "parameters": {}
    },
    "discretion_logic": null or "description of discretionary element"
}"#;

        let prompt = format!(
            "{}\n\nParse the following statute:\n\n{}",
            system_prompt, raw_text
        );

        self.provider.generate_structured(&prompt).await
    }

    /// Analyzes a statute for potential issues and ambiguities.
    pub async fn analyze(&self, statute: &Statute) -> Result<AnalysisReport> {
        let statute_json = serde_json::to_string_pretty(statute)?;

        let prompt = format!(
            r#"Analyze the following statute for:
1. Logical consistency
2. Ambiguous language that might require judicial interpretation
3. Potential conflicts with common legal principles
4. Missing conditions or edge cases

Statute:
{}

Respond with JSON:
{{
    "issues": ["list of identified issues"],
    "ambiguities": ["list of ambiguous terms or phrases"],
    "recommendations": ["list of recommendations"],
    "discretion_points": ["areas requiring human judgment"]
}}"#,
            statute_json
        );

        self.provider.generate_structured(&prompt).await
    }

    /// Generates a human-readable explanation of a statute.
    pub async fn explain(&self, statute: &Statute) -> Result<String> {
        let statute_json = serde_json::to_string_pretty(statute)?;

        let prompt = format!(
            r#"Explain the following statute in plain language that a non-lawyer can understand.
Include:
1. Who this law applies to
2. What conditions must be met
3. What happens when conditions are met
4. Any areas where human judgment is required

Statute:
{}"#,
            statute_json
        );

        self.provider.generate_text(&prompt).await
    }
}

/// Report from statute analysis.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalysisReport {
    /// Identified issues
    pub issues: Vec<String>,
    /// Ambiguous terms or phrases
    pub ambiguities: Vec<String>,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
    /// Points requiring human judgment
    pub discretion_points: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;
    use legalis_core::{Effect, EffectType};

    #[tokio::test]
    async fn test_law_compiler_with_mock() {
        let mock_response = r#"{
            "id": "test-statute-1",
            "title": "Test Statute",
            "preconditions": [],
            "effect": {
                "effect_type": "Grant",
                "description": "Test effect",
                "parameters": {}
            },
            "discretion_logic": null,
            "temporal_validity": {
                "effective_date": null,
                "expiry_date": null,
                "enacted_at": null,
                "amended_at": null
            },
            "version": 1,
            "jurisdiction": null,
            "relations": [],
            "amendments": []
        }"#;

        let provider = MockProvider::new().with_response("Parse", mock_response);
        let compiler = LawCompiler::new(provider);

        let result = compiler.compile("Test statute text").await;
        assert!(result.is_ok());

        let statute = result.unwrap();
        assert_eq!(statute.id, "test-statute-1");
    }

    #[tokio::test]
    async fn test_analysis_report() {
        let mock_response = r#"{
            "issues": ["No expiration date specified"],
            "ambiguities": ["'reasonable time' is undefined"],
            "recommendations": ["Add specific time limits"],
            "discretion_points": ["Determining what constitutes 'reasonable'"]
        }"#;

        let provider = MockProvider::new().with_response("Analyze", mock_response);
        let compiler = LawCompiler::new(provider);

        let statute = Statute::new(
            "test-1",
            "Test",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let report = compiler.analyze(&statute).await;
        assert!(report.is_ok());

        let report = report.unwrap();
        assert!(!report.issues.is_empty());
    }
}
