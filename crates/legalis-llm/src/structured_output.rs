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
