//! Structured output generation for legal statutes and conditions.
//!
//! This module provides functionality to convert natural language legal text
//! into structured schemas and abstract syntax trees (ASTs).

use crate::LLMProvider;
use anyhow::{Context, Result};
use legalis_core::{Condition, Effect, Statute};
use serde::{Deserialize, Serialize};

/// Schema for a statute structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteSchema {
    /// Statute title or name
    pub title: String,
    /// Statute identifier (e.g., "17 U.S.C. § 107")
    pub identifier: Option<String>,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Sections within the statute
    pub sections: Vec<StatuteSection>,
    /// Effective date
    pub effective_date: Option<String>,
    /// Metadata
    pub metadata: Option<serde_json::Value>,
}

/// Section within a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteSection {
    /// Section number or identifier
    pub section_id: String,
    /// Section title
    pub title: String,
    /// Section content
    pub content: String,
    /// Subsections
    pub subsections: Vec<StatuteSubsection>,
    /// Rules derived from this section
    pub rules: Vec<RuleSchema>,
}

/// Subsection within a statute section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteSubsection {
    /// Subsection identifier (e.g., "(a)", "(1)", etc.)
    pub subsection_id: String,
    /// Subsection content
    pub content: String,
}

/// Schema for a rule extracted from legal text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSchema {
    /// Rule name or identifier
    pub name: String,
    /// Conditions that trigger the rule
    pub conditions: Vec<ConditionSchema>,
    /// Effects or consequences of the rule
    pub effects: Vec<EffectSchema>,
    /// Priority or precedence
    pub priority: Option<i32>,
}

/// Schema for a condition in a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionSchema {
    /// Condition type (e.g., "comparison", "boolean", "existence")
    pub condition_type: String,
    /// Entity or subject of the condition
    pub entity: String,
    /// Operator (e.g., "equals", "greater_than", "exists")
    pub operator: String,
    /// Value to compare against
    pub value: Option<String>,
    /// Additional parameters
    pub parameters: Option<serde_json::Value>,
}

/// Schema for an effect in a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectSchema {
    /// Effect type (e.g., "obligation", "prohibition", "permission")
    pub effect_type: String,
    /// Subject affected by the effect
    pub subject: String,
    /// Action or outcome
    pub action: String,
    /// Additional parameters
    pub parameters: Option<serde_json::Value>,
}

/// Entity extracted from legal text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalEntity {
    /// Entity type (e.g., "person", "organization", "date", "amount")
    pub entity_type: String,
    /// Entity value or name
    pub value: String,
    /// Context or role in the legal text
    pub role: Option<String>,
}

/// Relationship between legal entities or statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalRelationship {
    /// Source entity or statute
    pub source: String,
    /// Relationship type (e.g., "depends_on", "modifies", "repeals")
    pub relationship_type: String,
    /// Target entity or statute
    pub target: String,
    /// Description of the relationship
    pub description: Option<String>,
}

/// Statute schema generator.
pub struct StatuteSchemaGenerator<P> {
    provider: P,
}

impl<P: LLMProvider> StatuteSchemaGenerator<P> {
    /// Creates a new statute schema generator.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Generates a statute schema from natural language text.
    pub async fn generate_schema(&self, statute_text: &str) -> Result<StatuteSchema> {
        let prompt = format!(
            r#"Analyze the following legal statute and extract its structure into a schema.

Statute text:
{}

Provide the statute schema in the following JSON format:
{{
    "title": "Statute title",
    "identifier": "Statute identifier (e.g., '17 U.S.C. § 107')",
    "jurisdiction": "Jurisdiction",
    "sections": [
        {{
            "section_id": "Section ID",
            "title": "Section title",
            "content": "Section content",
            "subsections": [
                {{
                    "subsection_id": "(a)",
                    "content": "Subsection content"
                }}
            ],
            "rules": [
                {{
                    "name": "Rule name",
                    "conditions": [
                        {{
                            "condition_type": "comparison",
                            "entity": "Entity name",
                            "operator": "equals",
                            "value": "Value"
                        }}
                    ],
                    "effects": [
                        {{
                            "effect_type": "obligation",
                            "subject": "Subject",
                            "action": "Action"
                        }}
                    ],
                    "priority": 1
                }}
            ]
        }}
    ],
    "effective_date": "Effective date if available"
}}

Extract all sections, subsections, and derive rules with their conditions and effects."#,
            statute_text
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to generate statute schema")
    }

    /// Extracts rules from legal text.
    pub async fn extract_rules(&self, legal_text: &str) -> Result<Vec<RuleSchema>> {
        let prompt = format!(
            r#"Extract all rules, conditions, and effects from the following legal text.

Legal text:
{}

Provide the rules in the following JSON format:
{{
    "rules": [
        {{
            "name": "Rule name or description",
            "conditions": [
                {{
                    "condition_type": "comparison/boolean/existence",
                    "entity": "Entity or variable name",
                    "operator": "equals/greater_than/less_than/exists",
                    "value": "Value to compare"
                }}
            ],
            "effects": [
                {{
                    "effect_type": "obligation/prohibition/permission",
                    "subject": "Who is affected",
                    "action": "What happens"
                }}
            ],
            "priority": 1
        }}
    ]
}}

Extract all conditions (IF clauses) and effects (THEN outcomes) from the text."#,
            legal_text
        );

        #[derive(Deserialize)]
        struct RulesResponse {
            rules: Vec<RuleSchema>,
        }

        let response: RulesResponse = self
            .provider
            .generate_structured(&prompt)
            .await
            .context("Failed to extract rules")?;

        Ok(response.rules)
    }

    /// Converts a condition schema to a legalis-core Condition.
    pub fn schema_to_condition(&self, schema: &ConditionSchema) -> Condition {
        // This is a simplified conversion using custom conditions
        // A real implementation could parse specific condition types
        let description = if let Some(ref value) = schema.value {
            format!("{} {} {}", schema.entity, schema.operator, value)
        } else {
            format!("{} {}", schema.entity, schema.operator)
        };
        Condition::custom(description)
    }

    /// Converts an effect schema to a legalis-core Effect.
    pub fn schema_to_effect(&self, schema: &EffectSchema) -> Effect {
        use legalis_core::EffectType;

        let effect_type = match schema.effect_type.to_lowercase().as_str() {
            "obligation" => EffectType::Obligation,
            "prohibition" => EffectType::Prohibition,
            "permission" | "grant" => EffectType::Grant,
            "revoke" => EffectType::Revoke,
            "monetary_transfer" | "monetary" => EffectType::MonetaryTransfer,
            "status_change" | "status" => EffectType::StatusChange,
            _ => EffectType::Custom,
        };

        let description = format!("{}: {}", schema.subject, schema.action);
        Effect::new(effect_type, description)
    }

    /// Converts a rule schema to preconditions and an effect.
    pub fn schema_to_preconditions_and_effect(
        &self,
        schema: &RuleSchema,
    ) -> (Vec<Condition>, Effect) {
        let conditions: Vec<Condition> = schema
            .conditions
            .iter()
            .map(|c| self.schema_to_condition(c))
            .collect();

        // For simplicity, use the first effect or create a default one
        let effect = if let Some(first_effect) = schema.effects.first() {
            self.schema_to_effect(first_effect)
        } else {
            Effect::grant(&schema.name)
        };

        (conditions, effect)
    }

    /// Generates a complete Statute from natural language.
    pub async fn generate_statute(&self, statute_text: &str) -> Result<Statute> {
        let schema = self.generate_schema(statute_text).await?;

        // Get all rules from all sections
        let mut all_conditions = Vec::new();
        let mut primary_effect = None;

        for section in &schema.sections {
            for rule_schema in &section.rules {
                let (conditions, effect) = self.schema_to_preconditions_and_effect(rule_schema);
                all_conditions.extend(conditions);
                if primary_effect.is_none() {
                    primary_effect = Some(effect);
                }
            }
        }

        // Create statute with the collected preconditions and primary effect
        let effect = primary_effect.unwrap_or_else(|| Effect::grant(&schema.title));
        let mut statute = Statute::new(
            schema.identifier.as_deref().unwrap_or(&schema.title),
            &schema.title,
            effect,
        );

        statute.preconditions = all_conditions;
        statute.jurisdiction = Some(schema.jurisdiction);

        Ok(statute)
    }
}

/// Entity extractor for legal documents.
pub struct EntityExtractor<P> {
    provider: P,
}

impl<P: LLMProvider> EntityExtractor<P> {
    /// Creates a new entity extractor.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Extracts legal entities from text.
    pub async fn extract_entities(&self, text: &str) -> Result<Vec<LegalEntity>> {
        let prompt = format!(
            r#"Extract all legal entities from the following text.

Text:
{}

Provide the entities in the following JSON format:
{{
    "entities": [
        {{
            "entity_type": "person/organization/date/amount/location/statute/other",
            "value": "Entity value or name",
            "role": "Role or context in the legal text"
        }}
    ]
}}

Extract all relevant legal entities including parties, dates, amounts, and statutory references."#,
            text
        );

        #[derive(Deserialize)]
        struct EntitiesResponse {
            entities: Vec<LegalEntity>,
        }

        let response: EntitiesResponse = self
            .provider
            .generate_structured(&prompt)
            .await
            .context("Failed to extract entities")?;

        Ok(response.entities)
    }
}

/// Relationship extractor for legal dependencies.
pub struct RelationshipExtractor<P> {
    provider: P,
}

impl<P: LLMProvider> RelationshipExtractor<P> {
    /// Creates a new relationship extractor.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Extracts relationships between statutes or entities.
    pub async fn extract_relationships(&self, text: &str) -> Result<Vec<LegalRelationship>> {
        let prompt = format!(
            r#"Extract all relationships between legal statutes, entities, or provisions from the following text.

Text:
{}

Provide the relationships in the following JSON format:
{{
    "relationships": [
        {{
            "source": "Source statute or entity",
            "relationship_type": "depends_on/modifies/repeals/references/supersedes",
            "target": "Target statute or entity",
            "description": "Description of the relationship"
        }}
    ]
}}

Identify all dependencies, modifications, repeals, and references."#,
            text
        );

        #[derive(Deserialize)]
        struct RelationshipsResponse {
            relationships: Vec<LegalRelationship>,
        }

        let response: RelationshipsResponse = self
            .provider
            .generate_structured(&prompt)
            .await
            .context("Failed to extract relationships")?;

        Ok(response.relationships)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::MockProvider;

    #[tokio::test]
    async fn test_statute_schema_generator() {
        let mock_response = r#"{
            "title": "Test Statute",
            "identifier": "Test § 1",
            "jurisdiction": "Test Jurisdiction",
            "sections": [
                {
                    "section_id": "1",
                    "title": "General Provisions",
                    "content": "This section defines general provisions",
                    "subsections": [],
                    "rules": []
                }
            ],
            "effective_date": "2024-01-01"
        }"#;

        let provider = MockProvider::new().with_response("Analyze", mock_response);
        let generator = StatuteSchemaGenerator::new(provider);

        let schema = generator
            .generate_schema("Test statute text")
            .await
            .unwrap();

        assert_eq!(schema.title, "Test Statute");
        assert_eq!(schema.sections.len(), 1);
    }

    #[tokio::test]
    async fn test_extract_rules() {
        let mock_response = r#"{
            "rules": [
                {
                    "name": "Test Rule",
                    "conditions": [
                        {
                            "condition_type": "comparison",
                            "entity": "age",
                            "operator": "greater_than",
                            "value": "18"
                        }
                    ],
                    "effects": [
                        {
                            "effect_type": "permission",
                            "subject": "person",
                            "action": "vote"
                        }
                    ],
                    "priority": 1
                }
            ]
        }"#;

        let provider = MockProvider::new().with_response("Extract", mock_response);
        let generator = StatuteSchemaGenerator::new(provider);

        let rules = generator.extract_rules("Test legal text").await.unwrap();

        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].name, "Test Rule");
        assert_eq!(rules[0].conditions.len(), 1);
        assert_eq!(rules[0].effects.len(), 1);
    }

    #[tokio::test]
    async fn test_entity_extractor() {
        let mock_response = r#"{
            "entities": [
                {
                    "entity_type": "person",
                    "value": "John Doe",
                    "role": "plaintiff"
                }
            ]
        }"#;

        let provider = MockProvider::new().with_response("Extract", mock_response);
        let extractor = EntityExtractor::new(provider);

        let entities = extractor
            .extract_entities("Test text with entities")
            .await
            .unwrap();

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, "person");
        assert_eq!(entities[0].value, "John Doe");
    }

    #[test]
    fn test_schema_to_condition() {
        let provider = MockProvider::new();
        let generator = StatuteSchemaGenerator::new(provider);

        let schema = ConditionSchema {
            condition_type: "comparison".to_string(),
            entity: "age".to_string(),
            operator: "greater_than".to_string(),
            value: Some("18".to_string()),
            parameters: None,
        };

        let _condition = generator.schema_to_condition(&schema);
        // Condition is now created as a custom condition
    }

    #[test]
    fn test_schema_to_effect() {
        let provider = MockProvider::new();
        let generator = StatuteSchemaGenerator::new(provider);

        let schema = EffectSchema {
            effect_type: "permission".to_string(),
            subject: "person".to_string(),
            action: "vote".to_string(),
            parameters: None,
        };

        let effect = generator.schema_to_effect(&schema);
        assert!(effect.description.contains("person"));
    }
}

// ============================================================================
// Structured Output Generation (v0.2.6)
// ============================================================================

/// JSON schema for constrained generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchemaConstraint {
    /// Schema title
    pub title: String,
    /// Schema type (object, array, string, etc.)
    pub schema_type: String,
    /// Required properties
    pub required: Vec<String>,
    /// Property definitions
    pub properties: serde_json::Value,
    /// Additional schema constraints
    pub additional_constraints: Option<serde_json::Value>,
}

impl JsonSchemaConstraint {
    /// Creates a new JSON schema constraint.
    pub fn new(title: impl Into<String>, schema_type: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            schema_type: schema_type.into(),
            required: Vec::new(),
            properties: serde_json::json!({}),
            additional_constraints: None,
        }
    }

    /// Adds a required property.
    pub fn add_required(mut self, property: impl Into<String>) -> Self {
        self.required.push(property.into());
        self
    }

    /// Sets property definitions.
    pub fn with_properties(mut self, properties: serde_json::Value) -> Self {
        self.properties = properties;
        self
    }

    /// Converts to JSON schema format.
    pub fn to_json_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "title": self.title,
            "type": self.schema_type,
            "required": self.required,
            "properties": self.properties,
            "additionalProperties": false
        })
    }
}

/// JSON schema-constrained generator.
pub struct JsonSchemaGenerator<P> {
    provider: P,
    strict_mode: bool,
}

impl<P: LLMProvider> JsonSchemaGenerator<P> {
    /// Creates a new JSON schema generator.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            strict_mode: true,
        }
    }

    /// Sets strict mode (reject responses that don't match schema).
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }

    /// Generates structured output constrained by JSON schema.
    pub async fn generate_with_schema(
        &self,
        prompt: &str,
        schema: &JsonSchemaConstraint,
    ) -> Result<serde_json::Value> {
        let schema_json = schema.to_json_schema();

        let full_prompt = format!(
            r#"{prompt}

You must respond with valid JSON that conforms to this exact schema:

{schema}

Ensure all required fields are present and all types match the schema."#,
            prompt = prompt,
            schema = serde_json::to_string_pretty(&schema_json)?
        );

        let response = self.provider.generate_text(&full_prompt).await?;

        // Try to parse and validate the response
        let parsed: serde_json::Value =
            serde_json::from_str(&response).context("Failed to parse response as JSON")?;

        if self.strict_mode {
            self.validate_against_schema(&parsed, schema)?;
        }

        Ok(parsed)
    }

    /// Validates JSON against schema.
    fn validate_against_schema(
        &self,
        value: &serde_json::Value,
        schema: &JsonSchemaConstraint,
    ) -> Result<()> {
        // Basic validation - check required fields
        if let serde_json::Value::Object(obj) = value {
            for required_field in &schema.required {
                if !obj.contains_key(required_field) {
                    anyhow::bail!("Missing required field: {}", required_field);
                }
            }
        }
        Ok(())
    }
}

/// Grammar rule for guided decoding.
#[derive(Debug, Clone)]
pub struct GrammarRule {
    /// Rule name
    pub name: String,
    /// Rule pattern (regex or BNF)
    pub pattern: String,
    /// Rule type (regex, bnf, json)
    pub rule_type: GrammarRuleType,
}

/// Type of grammar rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrammarRuleType {
    /// Regular expression
    Regex,
    /// Backus-Naur Form
    BNF,
    /// JSON schema
    Json,
}

/// Grammar-guided decoder.
pub struct GrammarGuidedDecoder<P> {
    provider: P,
    grammar: Vec<GrammarRule>,
}

impl<P: LLMProvider> GrammarGuidedDecoder<P> {
    /// Creates a new grammar-guided decoder.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            grammar: Vec::new(),
        }
    }

    /// Adds a grammar rule.
    pub fn add_rule(mut self, rule: GrammarRule) -> Self {
        self.grammar.push(rule);
        self
    }

    /// Generates text following the grammar rules.
    pub async fn generate_guided(&self, prompt: &str) -> Result<String> {
        let grammar_description = self
            .grammar
            .iter()
            .map(|r| format!("{}: {}", r.name, r.pattern))
            .collect::<Vec<_>>()
            .join("\n");

        let full_prompt = format!(
            r#"{prompt}

Your response must follow these grammar rules:
{grammar}

Generate output that strictly adheres to these rules."#,
            prompt = prompt,
            grammar = grammar_description
        );

        let response = self.provider.generate_text(&full_prompt).await?;

        // Validate against grammar rules
        for rule in &self.grammar {
            if rule.rule_type == GrammarRuleType::Regex {
                let re = regex::Regex::new(&rule.pattern)?;
                if !re.is_match(&response) {
                    anyhow::bail!("Response does not match grammar rule: {}", rule.name);
                }
            }
        }

        Ok(response)
    }
}

/// Legal form template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalFormTemplate {
    /// Form name
    pub name: String,
    /// Form description
    pub description: String,
    /// Form fields
    pub fields: Vec<FormField>,
    /// Validation rules
    pub validation_rules: Vec<ValidationRule>,
}

/// Form field definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    /// Field name
    pub name: String,
    /// Field label
    pub label: String,
    /// Field type (text, number, date, select, etc.)
    pub field_type: String,
    /// Whether field is required
    pub required: bool,
    /// Default value
    pub default_value: Option<String>,
    /// Help text
    pub help_text: Option<String>,
}

/// Validation rule for form fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Field name to validate
    pub field_name: String,
    /// Validation type (required, min_length, max_length, pattern, etc.)
    pub validation_type: String,
    /// Validation parameters
    pub parameters: Option<serde_json::Value>,
    /// Error message
    pub error_message: String,
}

/// Legal form filler using AI.
pub struct LegalFormFiller<P> {
    provider: P,
}

impl<P: LLMProvider> LegalFormFiller<P> {
    /// Creates a new legal form filler.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Fills a legal form from natural language input.
    pub async fn fill_form(
        &self,
        template: &LegalFormTemplate,
        input_text: &str,
    ) -> Result<serde_json::Value> {
        let fields_description = template
            .fields
            .iter()
            .map(|f| {
                format!(
                    "- {} ({}): {}{}",
                    f.label,
                    f.field_type,
                    f.help_text.as_deref().unwrap_or(""),
                    if f.required { " [REQUIRED]" } else { "" }
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"Fill out this legal form based on the provided information.

Form: {form_name}
{form_description}

Fields to fill:
{fields}

Information provided:
{input}

Extract the relevant information and fill the form fields. Respond with JSON:
{{
    "filled_fields": {{
        "field_name_1": "value1",
        "field_name_2": "value2",
        ...
    }},
    "confidence": 0.95,
    "missing_required": ["field_name_3"],
    "notes": "Any additional notes or clarifications"
}}"#,
            form_name = template.name,
            form_description = template.description,
            fields = fields_description,
            input = input_text
        );

        let response = self.provider.generate_text(&prompt).await?;
        let parsed: serde_json::Value = serde_json::from_str(&response)?;

        Ok(parsed)
    }

    /// Validates filled form data.
    pub fn validate_form(
        &self,
        template: &LegalFormTemplate,
        filled_data: &serde_json::Value,
    ) -> Result<Vec<String>> {
        let mut errors = Vec::new();

        if let Some(filled_fields) = filled_data["filled_fields"].as_object() {
            // Check required fields
            for field in &template.fields {
                if field.required && !filled_fields.contains_key(&field.name) {
                    errors.push(format!("Required field '{}' is missing", field.label));
                }
            }

            // Apply validation rules
            for rule in &template.validation_rules {
                if let Some(value) = filled_fields.get(&rule.field_name) {
                    if !self.validate_field(value, rule) {
                        errors.push(rule.error_message.clone());
                    }
                }
            }
        }

        Ok(errors)
    }

    fn validate_field(&self, _value: &serde_json::Value, _rule: &ValidationRule) -> bool {
        // Simplified validation - in production would implement full validation logic
        true
    }
}

/// Structured case analysis output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseAnalysisOutput {
    /// Case name
    pub case_name: String,
    /// Citation
    pub citation: String,
    /// Court
    pub court: String,
    /// Decision date
    pub date: String,
    /// Parties involved
    pub parties: CaseParties,
    /// Facts summary
    pub facts: String,
    /// Legal issues
    pub issues: Vec<String>,
    /// Court's holding
    pub holding: String,
    /// Reasoning
    pub reasoning: Vec<String>,
    /// Outcome
    pub outcome: String,
    /// Precedents cited
    pub precedents_cited: Vec<String>,
    /// Legal principles
    pub legal_principles: Vec<String>,
    /// Dissenting opinions (if any)
    pub dissenting_opinions: Option<String>,
}

/// Parties in a case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseParties {
    /// Plaintiff(s) or Appellant(s)
    pub plaintiffs: Vec<String>,
    /// Defendant(s) or Appellee(s)
    pub defendants: Vec<String>,
}

/// Case analyzer that produces structured output.
pub struct CaseAnalyzer<P> {
    provider: P,
}

impl<P: LLMProvider> CaseAnalyzer<P> {
    /// Creates a new case analyzer.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Analyzes a case and produces structured output.
    pub async fn analyze_case(&self, case_text: &str) -> Result<CaseAnalysisOutput> {
        let prompt = format!(
            r#"Analyze this legal case and provide structured output.

Case Text:
{case_text}

Provide a comprehensive analysis in JSON format:
{{
    "case_name": "Plaintiff v. Defendant",
    "citation": "123 F.3d 456 (9th Cir. 2020)",
    "court": "United States Court of Appeals for the Ninth Circuit",
    "date": "2020-01-15",
    "parties": {{
        "plaintiffs": ["..."],
        "defendants": ["..."]
    }},
    "facts": "Summary of relevant facts...",
    "issues": ["Legal issue 1", "Legal issue 2"],
    "holding": "Court's decision...",
    "reasoning": ["Reason 1", "Reason 2"],
    "outcome": "Final outcome...",
    "precedents_cited": ["Case 1", "Case 2"],
    "legal_principles": ["Principle 1", "Principle 2"],
    "dissenting_opinions": "Summary of dissent (if any)"
}}"#,
            case_text = case_text
        );

        let response = self.provider.generate_text(&prompt).await?;
        let analysis: CaseAnalysisOutput =
            serde_json::from_str(&response).context("Failed to parse case analysis")?;

        Ok(analysis)
    }
}

/// Table cell data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    /// Cell value
    pub value: String,
    /// Cell data type
    pub data_type: String,
    /// Confidence score
    pub confidence: f64,
}

/// Extracted table data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableData {
    /// Table title or description
    pub title: Option<String>,
    /// Column headers
    pub headers: Vec<String>,
    /// Row data
    pub rows: Vec<Vec<TableCell>>,
    /// Table metadata
    pub metadata: Option<serde_json::Value>,
}

/// Tabular data extractor.
pub struct TabularDataExtractor<P> {
    provider: P,
}

impl<P: LLMProvider> TabularDataExtractor<P> {
    /// Creates a new tabular data extractor.
    pub fn new(provider: P) -> Self {
        Self { provider }
    }

    /// Extracts tabular data from text.
    pub async fn extract_table(&self, text: &str) -> Result<Vec<TableData>> {
        let prompt = format!(
            r#"Extract all tabular data from this text and format as JSON.

Text:
{text}

For each table found, provide:
{{
    "title": "Table title (if any)",
    "headers": ["Column 1", "Column 2", ...],
    "rows": [
        [
            {{"value": "cell1", "data_type": "text", "confidence": 0.95}},
            {{"value": "cell2", "data_type": "number", "confidence": 0.90}}
        ]
    ]
}}

Respond with an array of table objects."#,
            text = text
        );

        let response = self.provider.generate_text(&prompt).await?;
        let tables: Vec<TableData> =
            serde_json::from_str(&response).context("Failed to parse table data")?;

        Ok(tables)
    }

    /// Extracts specific columns from text.
    pub async fn extract_columns(&self, text: &str, column_names: &[String]) -> Result<TableData> {
        let columns_list = column_names.join(", ");

        let prompt = format!(
            r#"Extract the following columns from this text: {columns}

Text:
{text}

Format as a table with these exact column headers.
Provide as JSON with headers and rows arrays."#,
            columns = columns_list,
            text = text
        );

        let response = self.provider.generate_text(&prompt).await?;
        let table: TableData =
            serde_json::from_str(&response).context("Failed to parse column data")?;

        Ok(table)
    }

    /// Converts table to CSV format.
    pub fn table_to_csv(&self, table: &TableData) -> String {
        let mut csv = String::new();

        // Headers
        csv.push_str(&table.headers.join(","));
        csv.push('\n');

        // Rows
        for row in &table.rows {
            let row_values: Vec<String> = row.iter().map(|cell| cell.value.clone()).collect();
            csv.push_str(&row_values.join(","));
            csv.push('\n');
        }

        csv
    }
}

#[cfg(test)]
mod v2_6_tests {
    use super::*;

    #[test]
    fn test_json_schema_constraint() {
        let schema = JsonSchemaConstraint::new("TestSchema", "object")
            .add_required("field1")
            .add_required("field2")
            .with_properties(serde_json::json!({
                "field1": {"type": "string"},
                "field2": {"type": "number"}
            }));

        let json_schema = schema.to_json_schema();
        assert_eq!(json_schema["title"], "TestSchema");
        assert_eq!(json_schema["type"], "object");
        assert_eq!(json_schema["required"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_grammar_rule_creation() {
        let rule = GrammarRule {
            name: "email".to_string(),
            pattern: r"^\w+@\w+\.\w+$".to_string(),
            rule_type: GrammarRuleType::Regex,
        };

        assert_eq!(rule.name, "email");
        assert_eq!(rule.rule_type, GrammarRuleType::Regex);
    }

    #[test]
    fn test_legal_form_template() {
        let template = LegalFormTemplate {
            name: "Simple Contract".to_string(),
            description: "A simple contract template".to_string(),
            fields: vec![FormField {
                name: "party1".to_string(),
                label: "First Party".to_string(),
                field_type: "text".to_string(),
                required: true,
                default_value: None,
                help_text: Some("Enter the name of the first party".to_string()),
            }],
            validation_rules: vec![],
        };

        assert_eq!(template.fields.len(), 1);
        assert!(template.fields[0].required);
    }

    #[test]
    fn test_case_parties() {
        let parties = CaseParties {
            plaintiffs: vec!["John Doe".to_string()],
            defendants: vec!["Jane Smith".to_string()],
        };

        assert_eq!(parties.plaintiffs.len(), 1);
        assert_eq!(parties.defendants.len(), 1);
    }

    #[test]
    fn test_table_cell() {
        let cell = TableCell {
            value: "100".to_string(),
            data_type: "number".to_string(),
            confidence: 0.95,
        };

        assert_eq!(cell.value, "100");
        assert!((cell.confidence - 0.95).abs() < f64::EPSILON);
    }

    #[test]
    fn test_table_data() {
        let table = TableData {
            title: Some("Test Table".to_string()),
            headers: vec!["Name".to_string(), "Age".to_string()],
            rows: vec![vec![
                TableCell {
                    value: "Alice".to_string(),
                    data_type: "text".to_string(),
                    confidence: 0.99,
                },
                TableCell {
                    value: "30".to_string(),
                    data_type: "number".to_string(),
                    confidence: 0.95,
                },
            ]],
            metadata: None,
        };

        assert_eq!(table.headers.len(), 2);
        assert_eq!(table.rows.len(), 1);
    }
}
