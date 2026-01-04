//! Salesforce Contract format support.
//!
//! Salesforce Contract Management (part of Salesforce CPQ) handles:
//! - Contract lifecycle management
//! - Pricing and quoting
//! - Contract terms and conditions
//! - Approval workflows
//! - Contract amendments and renewals
//!
//! This module provides bidirectional conversion between Salesforce Contract
//! format and legalis_core::Statute format.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Salesforce Contract document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceContract {
    /// Contract metadata
    #[serde(rename = "attributes")]
    pub attributes: SalesforceAttributes,
    /// Contract ID (Salesforce record ID)
    #[serde(rename = "Id")]
    pub id: String,
    /// Contract number
    #[serde(rename = "ContractNumber")]
    pub contract_number: String,
    /// Account ID
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// Status (Draft, InApproval, Activated, Expired)
    #[serde(rename = "Status")]
    pub status: String,
    /// Start date (YYYY-MM-DD)
    #[serde(rename = "StartDate")]
    pub start_date: String,
    /// Contract term (in months)
    #[serde(rename = "ContractTerm")]
    pub contract_term: i32,
    /// Owner ID
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    /// Special terms
    #[serde(rename = "SpecialTerms", skip_serializing_if = "Option::is_none")]
    pub special_terms: Option<String>,
    /// Contract terms (custom)
    #[serde(rename = "ContractTerms__c", skip_serializing_if = "Option::is_none")]
    pub contract_terms: Option<Vec<SalesforceContractTerm>>,
    /// Contract clauses (custom)
    #[serde(rename = "ContractClauses__c", skip_serializing_if = "Option::is_none")]
    pub contract_clauses: Option<Vec<SalesforceContractClause>>,
}

/// Salesforce API attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceAttributes {
    /// Object type
    #[serde(rename = "type")]
    pub object_type: String,
    /// API URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Salesforce contract term (custom object)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceContractTerm {
    /// Term ID
    #[serde(rename = "Id")]
    pub id: String,
    /// Term type (Payment, Delivery, Service Level, etc.)
    #[serde(rename = "TermType__c")]
    pub term_type: String,
    /// Term description
    #[serde(rename = "Description__c")]
    pub description: String,
    /// Obligation party
    #[serde(rename = "ObligationParty__c")]
    pub obligation_party: String,
    /// Trigger condition
    #[serde(
        rename = "TriggerCondition__c",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_condition: Option<String>,
    /// Due date (in days from trigger)
    #[serde(rename = "DueDays__c", skip_serializing_if = "Option::is_none")]
    pub due_days: Option<i32>,
    /// Penalty for breach
    #[serde(rename = "Penalty__c", skip_serializing_if = "Option::is_none")]
    pub penalty: Option<String>,
}

/// Salesforce contract clause (custom object)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceContractClause {
    /// Clause ID
    #[serde(rename = "Id")]
    pub id: String,
    /// Clause type (Indemnity, Warranty, Limitation, etc.)
    #[serde(rename = "ClauseType__c")]
    pub clause_type: String,
    /// Clause title
    #[serde(rename = "Title__c")]
    pub title: String,
    /// Clause text
    #[serde(rename = "ClauseText__c")]
    pub clause_text: String,
    /// Standard clause indicator
    #[serde(rename = "IsStandard__c")]
    pub is_standard: bool,
    /// Risk level (Low, Medium, High)
    #[serde(rename = "RiskLevel__c", skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
}

/// Salesforce Contract importer
pub struct SalesforceContractImporter;

impl SalesforceContractImporter {
    /// Creates a new Salesforce Contract importer
    pub fn new() -> Self {
        Self
    }

    fn parse_salesforce_contract(&self, source: &str) -> InteropResult<SalesforceContract> {
        serde_json::from_str(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse Salesforce Contract JSON: {}", e))
        })
    }

    fn convert_term_to_statute(&self, term: &SalesforceContractTerm) -> Statute {
        let effect_type = match term.term_type.as_str() {
            "Payment" => EffectType::Obligation,
            "Delivery" => EffectType::Grant,
            "Service Level" => EffectType::Obligation,
            "Warranty" => EffectType::Grant,
            "Termination" => EffectType::Revoke,
            _ => EffectType::Obligation,
        };

        let mut effect = Effect::new(effect_type, &term.description);
        effect.parameters.insert(
            "obligation_party".to_string(),
            term.obligation_party.clone(),
        );
        effect
            .parameters
            .insert("term_type".to_string(), term.term_type.clone());

        if let Some(due_days) = term.due_days {
            effect
                .parameters
                .insert("due_days".to_string(), due_days.to_string());
        }

        if let Some(penalty) = &term.penalty {
            effect
                .parameters
                .insert("penalty".to_string(), penalty.clone());
        }

        let mut statute = Statute::new(&term.id, &term.description, effect);

        // Add trigger condition if present
        if let Some(trigger) = &term.trigger_condition {
            statute = statute.with_precondition(Condition::Custom {
                description: trigger.clone(),
            });
        }

        statute
    }

    fn convert_clause_to_statute(&self, clause: &SalesforceContractClause) -> Statute {
        let effect_type = match clause.clause_type.as_str() {
            "Indemnity" | "Warranty" => EffectType::Grant,
            "Limitation" | "Liability" => EffectType::Prohibition,
            "Termination" => EffectType::Revoke,
            _ => EffectType::Grant,
        };

        let mut effect = Effect::new(effect_type, &clause.title);
        effect
            .parameters
            .insert("clause_text".to_string(), clause.clause_text.clone());
        effect
            .parameters
            .insert("clause_type".to_string(), clause.clause_type.clone());
        effect
            .parameters
            .insert("is_standard".to_string(), clause.is_standard.to_string());

        if let Some(risk_level) = &clause.risk_level {
            effect
                .parameters
                .insert("risk_level".to_string(), risk_level.clone());
        }

        Statute::new(&clause.id, &clause.title, effect)
    }
}

impl Default for SalesforceContractImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for SalesforceContractImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SalesforceContract
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let contract = self.parse_salesforce_contract(source)?;
        let mut report =
            ConversionReport::new(LegalFormat::SalesforceContract, LegalFormat::Legalis);
        let mut statutes = Vec::new();

        // Convert contract terms
        if let Some(terms) = &contract.contract_terms {
            for term in terms {
                statutes.push(self.convert_term_to_statute(term));
            }
        }

        // Convert contract clauses
        if let Some(clauses) = &contract.contract_clauses {
            for clause in clauses {
                statutes.push(self.convert_clause_to_statute(clause));
            }
        }

        report.statutes_converted = statutes.len();

        if statutes.is_empty() {
            report.add_warning(
                "No contract terms or clauses found in Salesforce Contract".to_string(),
            );
        }

        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Try to parse as JSON and check for Salesforce Contract specific fields
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(source) {
            if let Some(obj) = value.as_object() {
                return obj.contains_key("Id")
                    && obj.contains_key("ContractNumber")
                    && obj.contains_key("AccountId")
                    && obj
                        .get("attributes")
                        .and_then(|a| a.get("type"))
                        .and_then(|t| t.as_str())
                        .map(|t| t == "Contract")
                        .unwrap_or(false);
            }
        }
        false
    }
}

/// Salesforce Contract exporter
pub struct SalesforceContractExporter;

impl SalesforceContractExporter {
    /// Creates a new Salesforce Contract exporter
    pub fn new() -> Self {
        Self
    }

    fn convert_statute_to_term(&self, statute: &Statute) -> SalesforceContractTerm {
        let term_type = match statute.effect.effect_type {
            EffectType::Grant => "Delivery",
            EffectType::Obligation => "Payment",
            EffectType::Prohibition => "Restriction",
            EffectType::Revoke => "Termination",
            _ => "General",
        };

        let obligation_party = statute
            .effect
            .parameters
            .get("obligation_party")
            .cloned()
            .unwrap_or_else(|| "Customer".to_string());

        let due_days = statute
            .effect
            .parameters
            .get("due_days")
            .and_then(|s| s.parse::<i32>().ok());

        let penalty = statute.effect.parameters.get("penalty").cloned();

        let trigger_condition = if !statute.preconditions.is_empty() {
            Some(format!("{:?}", statute.preconditions))
        } else {
            None
        };

        SalesforceContractTerm {
            id: statute.id.clone(),
            term_type: term_type.to_string(),
            description: statute.title.clone(),
            obligation_party,
            trigger_condition,
            due_days,
            penalty,
        }
    }
}

impl Default for SalesforceContractExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for SalesforceContractExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SalesforceContract
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report =
            ConversionReport::new(LegalFormat::Legalis, LegalFormat::SalesforceContract);

        let terms: Vec<SalesforceContractTerm> = statutes
            .iter()
            .map(|s| self.convert_statute_to_term(s))
            .collect();

        let contract = SalesforceContract {
            attributes: SalesforceAttributes {
                object_type: "Contract".to_string(),
                url: None,
            },
            id: "SFDC-CONTRACT-001".to_string(),
            contract_number: "C-001".to_string(),
            account_id: "ACCT-001".to_string(),
            status: "Draft".to_string(),
            start_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            contract_term: 12,
            owner_id: "OWNER-001".to_string(),
            special_terms: None,
            contract_terms: Some(terms),
            contract_clauses: None,
        };

        let json = serde_json::to_string_pretty(&contract).map_err(|e| {
            InteropError::SerializationError(format!("JSON serialization failed: {}", e))
        })?;

        report.statutes_converted = statutes.len();

        if statutes.len() > 20 {
            report.add_warning(format!(
                "Exported {} statutes as Salesforce contract terms",
                statutes.len()
            ));
        }

        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        // Salesforce Contract can represent most legal concepts
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    #[test]
    fn test_salesforce_contract_validate() {
        let importer = SalesforceContractImporter::new();

        let valid_json = r#"{
            "attributes": {
                "type": "Contract"
            },
            "Id": "800xx000000001",
            "ContractNumber": "C-00001",
            "AccountId": "001xx000000001",
            "Status": "Draft",
            "StartDate": "2024-01-01",
            "ContractTerm": 12,
            "OwnerId": "005xx000000001"
        }"#;

        assert!(importer.validate(valid_json));

        let invalid_json = r#"{"random": "data"}"#;
        assert!(!importer.validate(invalid_json));
    }

    #[test]
    fn test_salesforce_contract_import() {
        let importer = SalesforceContractImporter::new();

        let json = r#"{
            "attributes": {
                "type": "Contract"
            },
            "Id": "800xx000000001",
            "ContractNumber": "C-00001",
            "AccountId": "001xx000000001",
            "Status": "Activated",
            "StartDate": "2024-01-01",
            "ContractTerm": 12,
            "OwnerId": "005xx000000001",
            "ContractTerms__c": [
                {
                    "Id": "TERM-001",
                    "TermType__c": "Payment",
                    "Description__c": "Monthly payment obligation",
                    "ObligationParty__c": "Customer",
                    "DueDays__c": 30
                }
            ]
        }"#;

        let (statutes, report) = importer.import(json).unwrap();
        assert_eq!(statutes.len(), 1);
        assert_eq!(report.statutes_converted, 1);
        assert_eq!(statutes[0].id, "TERM-001");
    }

    #[test]
    fn test_salesforce_contract_export() {
        let exporter = SalesforceContractExporter::new();

        let statute = Statute::new(
            "test-term",
            "Test Payment Term",
            Effect::new(EffectType::Obligation, "Payment due"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();
        assert_eq!(report.statutes_converted, 1);
        assert!(output.contains("SFDC-CONTRACT-001"));
        assert!(output.contains("Test Payment Term"));
    }

    #[test]
    fn test_salesforce_contract_roundtrip() {
        let importer = SalesforceContractImporter::new();
        let exporter = SalesforceContractExporter::new();

        let original_json = r#"{
            "attributes": {
                "type": "Contract"
            },
            "Id": "800xx000000001",
            "ContractNumber": "C-00001",
            "AccountId": "001xx000000001",
            "Status": "Activated",
            "StartDate": "2024-01-01",
            "ContractTerm": 12,
            "OwnerId": "005xx000000001",
            "ContractTerms__c": [
                {
                    "Id": "TERM-001",
                    "TermType__c": "Delivery",
                    "Description__c": "Product delivery",
                    "ObligationParty__c": "Vendor"
                }
            ]
        }"#;

        let (statutes, _) = importer.import(original_json).unwrap();
        let (output, _) = exporter.export(&statutes).unwrap();

        // Verify the exported JSON is valid
        let contract: SalesforceContract = serde_json::from_str(&output).unwrap();
        assert!(contract.contract_terms.is_some());
        assert_eq!(contract.contract_terms.unwrap().len(), 1);
    }
}
