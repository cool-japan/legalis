//! Taint analysis for tracking security-sensitive attributes in legal documents.
//!
//! This module provides taint analysis to track how sensitive data flows through
//! the statute dependency graph and identify potential security issues.

use crate::ast::{ConditionNode, LegalDocument, StatuteNode};
use std::collections::{HashMap, HashSet};

/// Categories of sensitive data that can be tracked
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaintCategory {
    /// Personal identification (SSN, passport, etc.)
    PersonalIdentity,
    /// Financial information (account numbers, cards, etc.)
    Financial,
    /// Medical/health records
    Medical,
    /// Privileged communications (attorney-client, etc.)
    Privileged,
    /// Authentication credentials
    Credentials,
    /// Location/tracking data
    Location,
    /// Biometric data
    Biometric,
    /// Custom category for user-defined sensitive data
    Custom,
}

/// Represents a tainted field with its sensitivity level
#[derive(Debug, Clone, PartialEq)]
pub struct TaintInfo {
    /// The field name that is tainted
    pub field: String,
    /// Categories of taint that apply to this field
    pub categories: HashSet<TaintCategory>,
    /// Statutes that introduced this taint
    pub sources: HashSet<String>,
}

/// Configuration for taint analysis
#[derive(Debug, Clone)]
pub struct TaintConfig {
    /// Fields that are considered tainted by default
    tainted_fields: HashMap<String, HashSet<TaintCategory>>,
    /// Whether to propagate taint through REQUIRES relationships
    propagate_through_requires: bool,
    /// Whether to propagate taint through DELEGATE relationships
    propagate_through_delegates: bool,
}

impl Default for TaintConfig {
    fn default() -> Self {
        let mut tainted_fields = HashMap::new();

        // Common sensitive fields in legal documents
        tainted_fields.insert("ssn".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::PersonalIdentity);
            set
        });
        tainted_fields.insert("social_security_number".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::PersonalIdentity);
            set
        });
        tainted_fields.insert("passport_number".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::PersonalIdentity);
            set
        });
        tainted_fields.insert("account_number".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Financial);
            set
        });
        tainted_fields.insert("credit_card".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Financial);
            set
        });
        tainted_fields.insert("medical_record".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Medical);
            set
        });
        tainted_fields.insert("health_information".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Medical);
            set
        });
        tainted_fields.insert("password".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Credentials);
            set
        });
        tainted_fields.insert("api_key".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Credentials);
            set
        });
        tainted_fields.insert("gps_location".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Location);
            set
        });
        tainted_fields.insert("fingerprint".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Biometric);
            set
        });
        tainted_fields.insert("biometric_data".to_string(), {
            let mut set = HashSet::new();
            set.insert(TaintCategory::Biometric);
            set
        });

        Self {
            tainted_fields,
            propagate_through_requires: true,
            propagate_through_delegates: true,
        }
    }
}

impl TaintConfig {
    /// Creates a new taint configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a tainted field with specified categories
    pub fn add_tainted_field(&mut self, field: String, categories: HashSet<TaintCategory>) {
        self.tainted_fields.insert(field, categories);
    }

    /// Sets whether to propagate taint through REQUIRES relationships
    pub fn set_propagate_requires(&mut self, propagate: bool) {
        self.propagate_through_requires = propagate;
    }

    /// Sets whether to propagate taint through DELEGATE relationships
    pub fn set_propagate_delegates(&mut self, propagate: bool) {
        self.propagate_through_delegates = propagate;
    }
}

/// Taint analyzer for legal documents
pub struct TaintAnalyzer {
    config: TaintConfig,
    /// Map from statute ID to taint info
    taint_map: HashMap<String, Vec<TaintInfo>>,
}

impl TaintAnalyzer {
    /// Creates a new taint analyzer with default configuration
    pub fn new() -> Self {
        Self {
            config: TaintConfig::default(),
            taint_map: HashMap::new(),
        }
    }

    /// Creates a new taint analyzer with custom configuration
    pub fn with_config(config: TaintConfig) -> Self {
        Self {
            config,
            taint_map: HashMap::new(),
        }
    }

    /// Analyzes a legal document for taint propagation
    pub fn analyze(&mut self, doc: &LegalDocument) {
        // First pass: identify tainted fields in each statute
        for statute in &doc.statutes {
            let taints = self.analyze_statute(statute);
            if !taints.is_empty() {
                self.taint_map.insert(statute.id.clone(), taints);
            }
        }

        // Second pass: propagate taint through dependencies
        if self.config.propagate_through_requires || self.config.propagate_through_delegates {
            self.propagate_taint(doc);
        }
    }

    /// Analyzes a single statute for tainted fields
    fn analyze_statute(&self, statute: &StatuteNode) -> Vec<TaintInfo> {
        let mut taints = Vec::new();
        let mut seen_fields = HashSet::new();

        // Check conditions for tainted fields
        for condition in &statute.conditions {
            self.collect_tainted_fields(condition, &mut taints, &mut seen_fields, &statute.id);
        }

        // Check exception conditions
        for exception in &statute.exceptions {
            for condition in &exception.conditions {
                self.collect_tainted_fields(condition, &mut taints, &mut seen_fields, &statute.id);
            }
        }

        // Check delegate conditions
        for delegate in &statute.delegates {
            for condition in &delegate.conditions {
                self.collect_tainted_fields(condition, &mut taints, &mut seen_fields, &statute.id);
            }
        }

        // Check scope conditions
        if let Some(scope) = &statute.scope {
            for condition in &scope.conditions {
                self.collect_tainted_fields(condition, &mut taints, &mut seen_fields, &statute.id);
            }
        }

        // Check constraint conditions
        for constraint in &statute.constraints {
            self.collect_tainted_fields(&constraint.condition, &mut taints, &mut seen_fields, &statute.id);
        }

        taints
    }

    /// Collects tainted fields from a condition
    fn collect_tainted_fields(
        &self,
        condition: &ConditionNode,
        taints: &mut Vec<TaintInfo>,
        seen: &mut HashSet<String>,
        statute_id: &str,
    ) {
        match condition {
            ConditionNode::Comparison { field, .. }
            | ConditionNode::Between { field, .. }
            | ConditionNode::In { field, .. }
            | ConditionNode::Like { field, .. }
            | ConditionNode::Matches { field, .. }
            | ConditionNode::InRange { field, .. }
            | ConditionNode::NotInRange { field, .. } => {
                if let Some(categories) = self.config.tainted_fields.get(field) {
                    if seen.insert(field.clone()) {
                        let mut sources = HashSet::new();
                        sources.insert(statute_id.to_string());
                        taints.push(TaintInfo {
                            field: field.clone(),
                            categories: categories.clone(),
                            sources,
                        });
                    }
                }
            }
            ConditionNode::HasAttribute { key } => {
                if let Some(categories) = self.config.tainted_fields.get(key) {
                    if seen.insert(key.clone()) {
                        let mut sources = HashSet::new();
                        sources.insert(statute_id.to_string());
                        taints.push(TaintInfo {
                            field: key.clone(),
                            categories: categories.clone(),
                            sources,
                        });
                    }
                }
            }
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                self.collect_tainted_fields(left, taints, seen, statute_id);
                self.collect_tainted_fields(right, taints, seen, statute_id);
            }
            ConditionNode::Not(inner) => {
                self.collect_tainted_fields(inner, taints, seen, statute_id);
            }
            _ => {}
        }
    }

    /// Propagates taint through statute dependencies
    fn propagate_taint(&mut self, doc: &LegalDocument) {
        let mut changed = true;

        while changed {
            changed = false;

            for statute in &doc.statutes {
                let mut new_taints = Vec::new();

                // Propagate through REQUIRES
                if self.config.propagate_through_requires {
                    for req_id in &statute.requires {
                        if let Some(req_taints) = self.taint_map.get(req_id) {
                            for taint in req_taints {
                                let mut new_taint = taint.clone();
                                new_taint.sources.insert(statute.id.clone());
                                new_taints.push(new_taint);
                            }
                        }
                    }
                }

                // Propagate through DELEGATES
                if self.config.propagate_through_delegates {
                    for delegate in &statute.delegates {
                        if let Some(delegate_taints) = self.taint_map.get(&delegate.target_id) {
                            for taint in delegate_taints {
                                let mut new_taint = taint.clone();
                                new_taint.sources.insert(statute.id.clone());
                                new_taints.push(new_taint);
                            }
                        }
                    }
                }

                // Merge new taints if any
                if !new_taints.is_empty() {
                    let existing = self.taint_map.entry(statute.id.clone()).or_default();
                    for new_taint in new_taints {
                        // Check if we already have this field
                        if let Some(existing_taint) = existing.iter_mut().find(|t| t.field == new_taint.field) {
                            // Merge categories and sources
                            let old_len = existing_taint.sources.len();
                            existing_taint.categories.extend(new_taint.categories);
                            existing_taint.sources.extend(new_taint.sources);
                            if existing_taint.sources.len() > old_len {
                                changed = true;
                            }
                        } else {
                            // Add new taint
                            existing.push(new_taint);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    /// Gets taint information for a statute
    pub fn get_taint(&self, statute_id: &str) -> Option<&Vec<TaintInfo>> {
        self.taint_map.get(statute_id)
    }

    /// Checks if a statute has any tainted fields
    pub fn is_tainted(&self, statute_id: &str) -> bool {
        self.taint_map.get(statute_id).is_some_and(|t| !t.is_empty())
    }

    /// Gets all statutes that use a specific tainted field
    pub fn find_users_of_field(&self, field: &str) -> Vec<String> {
        self.taint_map
            .iter()
            .filter(|(_, taints)| taints.iter().any(|t| t.field == field))
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Gets all tainted fields across all statutes
    pub fn all_tainted_fields(&self) -> HashSet<String> {
        self.taint_map
            .values()
            .flat_map(|taints| taints.iter().map(|t| t.field.clone()))
            .collect()
    }

    /// Generates a report of taint propagation
    pub fn generate_report(&self) -> TaintReport {
        let mut categories_count = HashMap::new();
        let mut total_tainted_statutes = 0;
        let mut total_taints = 0;

        for taints in self.taint_map.values() {
            if !taints.is_empty() {
                total_tainted_statutes += 1;
            }
            for taint in taints {
                total_taints += 1;
                for category in &taint.categories {
                    *categories_count.entry(*category).or_insert(0) += 1;
                }
            }
        }

        TaintReport {
            total_tainted_statutes,
            total_taints,
            categories_count,
            taint_map: self.taint_map.clone(),
        }
    }
}

impl Default for TaintAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Report of taint analysis results
#[derive(Debug, Clone)]
pub struct TaintReport {
    /// Number of statutes with tainted data
    pub total_tainted_statutes: usize,
    /// Total number of tainted fields
    pub total_taints: usize,
    /// Count of taints by category
    pub categories_count: HashMap<TaintCategory, usize>,
    /// Full taint map
    pub taint_map: HashMap<String, Vec<TaintInfo>>,
}

impl TaintReport {
    /// Prints a human-readable report
    pub fn print(&self) {
        println!("Taint Analysis Report");
        println!("====================");
        println!("Total tainted statutes: {}", self.total_tainted_statutes);
        println!("Total tainted fields: {}", self.total_taints);
        println!("\nTaints by category:");
        for (category, count) in &self.categories_count {
            println!("  {:?}: {}", category, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ConditionNode, ConditionValue, EffectNode, LegalDocument, StatuteNode};

    #[test]
    fn test_taint_detection_basic() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "statute1".to_string(),
                title: "SSN Statute".to_string(),
                conditions: vec![ConditionNode::Comparison {
                    field: "ssn".to_string(),
                    operator: "==".to_string(),
                    value: ConditionValue::String("123-45-6789".to_string()),
                }],
                effects: vec![EffectNode {
                    effect_type: "grant".to_string(),
                    description: "Access".to_string(),
                    parameters: vec![],
                }],
                ..Default::default()
            }],
        };

        let mut analyzer = TaintAnalyzer::new();
        analyzer.analyze(&doc);

        assert!(analyzer.is_tainted("statute1"));
        let taints = analyzer.get_taint("statute1").unwrap();
        assert_eq!(taints.len(), 1);
        assert_eq!(taints[0].field, "ssn");
        assert!(taints[0].categories.contains(&TaintCategory::PersonalIdentity));
    }

    #[test]
    fn test_taint_propagation_requires() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "base".to_string(),
                    title: "Base".to_string(),
                    conditions: vec![ConditionNode::Comparison {
                        field: "credit_card".to_string(),
                        operator: "!=".to_string(),
                        value: ConditionValue::String("".to_string()),
                    }],
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Payment".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "derived".to_string(),
                    title: "Derived".to_string(),
                    requires: vec!["base".to_string()],
                    effects: vec![EffectNode {
                        effect_type: "process".to_string(),
                        description: "Process payment".to_string(),
                        parameters: vec![],
                    }],
                    ..Default::default()
                },
            ],
        };

        let mut analyzer = TaintAnalyzer::new();
        analyzer.analyze(&doc);

        // Both should be tainted
        assert!(analyzer.is_tainted("base"));
        assert!(analyzer.is_tainted("derived"));

        // Derived should have taint propagated from base
        let derived_taints = analyzer.get_taint("derived").unwrap();
        assert!(!derived_taints.is_empty());
        assert!(derived_taints.iter().any(|t| t.field == "credit_card"));
    }

    #[test]
    fn test_taint_report() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "statute1".to_string(),
                    title: "Statute 1".to_string(),
                    conditions: vec![ConditionNode::Comparison {
                        field: "ssn".to_string(),
                        operator: "==".to_string(),
                        value: ConditionValue::String("123".to_string()),
                    }],
                    ..Default::default()
                },
                StatuteNode {
                    id: "statute2".to_string(),
                    title: "Statute 2".to_string(),
                    conditions: vec![ConditionNode::Comparison {
                        field: "medical_record".to_string(),
                        operator: "!=".to_string(),
                        value: ConditionValue::String("".to_string()),
                    }],
                    ..Default::default()
                },
            ],
        };

        let mut analyzer = TaintAnalyzer::new();
        analyzer.analyze(&doc);

        let report = analyzer.generate_report();
        assert_eq!(report.total_tainted_statutes, 2);
        assert_eq!(report.total_taints, 2);
        assert!(report.categories_count.contains_key(&TaintCategory::PersonalIdentity));
        assert!(report.categories_count.contains_key(&TaintCategory::Medical));
    }

    #[test]
    fn test_no_taint() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![StatuteNode {
                id: "statute1".to_string(),
                title: "Clean Statute".to_string(),
                conditions: vec![ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">".to_string(),
                    value: ConditionValue::Number(18),
                }],
                ..Default::default()
            }],
        };

        let mut analyzer = TaintAnalyzer::new();
        analyzer.analyze(&doc);

        assert!(!analyzer.is_tainted("statute1"));
        let report = analyzer.generate_report();
        assert_eq!(report.total_tainted_statutes, 0);
    }
}
