//! Compliance matrix generation for regulatory analysis.
//!
//! This module generates compliance matrices that map requirements, controls,
//! and regulations for auditing and compliance verification.

use crate::ast::{LegalDocument, StatuteNode};
#[cfg(test)]
use crate::ast::EffectNode;
use std::collections::{HashMap, HashSet};

/// A compliance matrix showing relationships between requirements and controls.
#[derive(Debug, Clone)]
pub struct ComplianceMatrix {
    /// All jurisdictions found
    pub jurisdictions: Vec<String>,
    /// All requirement types found
    pub requirement_types: Vec<String>,
    /// Matrix entries mapping (jurisdiction, requirement) to statutes
    pub entries: HashMap<(String, String), Vec<String>>,
    /// Coverage statistics
    pub stats: ComplianceStats,
}

/// Statistics about compliance coverage.
#[derive(Debug, Clone, Default)]
pub struct ComplianceStats {
    /// Total number of statutes
    pub total_statutes: usize,
    /// Number of statutes with jurisdiction specified
    pub statutes_with_jurisdiction: usize,
    /// Number of statutes with requirements
    pub statutes_with_requirements: usize,
    /// Number of independent statutes (no dependencies)
    pub independent_statutes: usize,
    /// Coverage percentage by jurisdiction
    pub jurisdiction_coverage: HashMap<String, f64>,
}

impl ComplianceMatrix {
    /// Generates a compliance matrix from a legal document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut jurisdictions = HashSet::new();
        let mut requirement_types = HashSet::new();
        let mut entries: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut stats = ComplianceStats {
            total_statutes: doc.statutes.len(),
            ..Default::default()
        };

        for statute in &doc.statutes {
            // Extract jurisdiction
            let jurisdiction = extract_jurisdiction(&statute.id)
                .unwrap_or_else(|| "UNKNOWN".to_string());
            jurisdictions.insert(jurisdiction.clone());

            if jurisdiction != "UNKNOWN" {
                stats.statutes_with_jurisdiction += 1;
            }

            // Extract requirement types from effect types
            for effect in &statute.effects {
                let req_type = normalize_requirement_type(&effect.effect_type);
                requirement_types.insert(req_type.clone());

                // Add to matrix
                let key = (jurisdiction.clone(), req_type);
                entries.entry(key).or_default().push(statute.id.clone());
            }

            // Check for dependencies
            if !statute.requires.is_empty() {
                stats.statutes_with_requirements += 1;
            } else {
                stats.independent_statutes += 1;
            }
        }

        // Calculate jurisdiction coverage
        let mut jurisdiction_coverage = HashMap::new();
        for jurisdiction in &jurisdictions {
            let count = doc
                .statutes
                .iter()
                .filter(|s| {
                    extract_jurisdiction(&s.id)
                        .map(|j| j == *jurisdiction)
                        .unwrap_or(false)
                })
                .count();
            let coverage = if stats.total_statutes > 0 {
                (count as f64 / stats.total_statutes as f64) * 100.0
            } else {
                0.0
            };
            jurisdiction_coverage.insert(jurisdiction.clone(), coverage);
        }
        stats.jurisdiction_coverage = jurisdiction_coverage;

        let mut jurisdictions: Vec<_> = jurisdictions.into_iter().collect();
        jurisdictions.sort();

        let mut requirement_types: Vec<_> = requirement_types.into_iter().collect();
        requirement_types.sort();

        Self {
            jurisdictions,
            requirement_types,
            entries,
            stats,
        }
    }

    /// Generates a text-based matrix report.
    pub fn to_text_report(&self) -> String {
        let mut report = String::new();

        report.push_str("COMPLIANCE MATRIX\n");
        report.push_str("=================\n\n");

        // Statistics
        report.push_str("Statistics:\n");
        report.push_str(&format!("  Total Statutes: {}\n", self.stats.total_statutes));
        report.push_str(&format!(
            "  With Jurisdiction: {}\n",
            self.stats.statutes_with_jurisdiction
        ));
        report.push_str(&format!(
            "  With Requirements: {}\n",
            self.stats.statutes_with_requirements
        ));
        report.push_str(&format!(
            "  Independent: {}\n",
            self.stats.independent_statutes
        ));
        report.push('\n');

        // Jurisdiction coverage
        report.push_str("Jurisdiction Coverage:\n");
        for (jurisdiction, coverage) in &self.stats.jurisdiction_coverage {
            report.push_str(&format!("  {}: {:.1}%\n", jurisdiction, coverage));
        }
        report.push('\n');

        // Matrix
        report.push_str("Matrix:\n");
        report.push_str(&format!(
            "{:<20} | {}\n",
            "Jurisdiction",
            self.requirement_types.join(" | ")
        ));
        report.push_str(&"-".repeat(80));
        report.push('\n');

        for jurisdiction in &self.jurisdictions {
            let mut row = format!("{:<20} |", jurisdiction);
            for req_type in &self.requirement_types {
                let key = (jurisdiction.clone(), req_type.clone());
                let count = self.entries.get(&key).map(|v| v.len()).unwrap_or(0);
                row.push_str(&format!(" {:^10} |", if count > 0 {
                    count.to_string()
                } else {
                    "-".to_string()
                }));
            }
            report.push_str(&row);
            report.push('\n');
        }

        report
    }

    /// Generates a CSV representation of the matrix.
    pub fn to_csv(&self) -> String {
        let mut csv = String::new();

        // Header
        csv.push_str("Jurisdiction,");
        csv.push_str(&self.requirement_types.join(","));
        csv.push('\n');

        // Rows
        for jurisdiction in &self.jurisdictions {
            csv.push_str(jurisdiction);
            csv.push(',');

            let row: Vec<String> = self
                .requirement_types
                .iter()
                .map(|req_type| {
                    let key = (jurisdiction.clone(), req_type.clone());
                    self.entries
                        .get(&key)
                        .map(|v| v.len().to_string())
                        .unwrap_or_else(|| "0".to_string())
                })
                .collect();

            csv.push_str(&row.join(","));
            csv.push('\n');
        }

        csv
    }

    /// Gets all statutes for a specific jurisdiction and requirement type.
    pub fn get_statutes(&self, jurisdiction: &str, requirement_type: &str) -> Vec<&String> {
        let key = (jurisdiction.to_string(), requirement_type.to_string());
        self.entries
            .get(&key)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Identifies gaps where a jurisdiction has no coverage for a requirement type.
    pub fn find_gaps(&self) -> Vec<(String, String)> {
        let mut gaps = Vec::new();

        for jurisdiction in &self.jurisdictions {
            for req_type in &self.requirement_types {
                let key = (jurisdiction.clone(), req_type.clone());
                if !self.entries.contains_key(&key)
                    || self.entries[&key].is_empty()
                {
                    gaps.push((jurisdiction.clone(), req_type.clone()));
                }
            }
        }

        gaps
    }
}

/// Extracts jurisdiction from statute ID (e.g., "US-CA-law-1" -> "US-CA").
fn extract_jurisdiction(statute_id: &str) -> Option<String> {
    let parts: Vec<&str> = statute_id.split('-').collect();
    if parts.len() >= 2 {
        Some(format!("{}-{}", parts[0], parts[1]))
    } else {
        None
    }
}

/// Normalizes requirement type names.
fn normalize_requirement_type(effect_type: &str) -> String {
    effect_type.to_uppercase().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::EffectNode;

    fn create_test_statute(id: &str, effect_type: &str) -> StatuteNode {
        StatuteNode {
            id: id.to_string(),
            title: "Test".to_string(),
            conditions: vec![],
            effects: vec![EffectNode {
                effect_type: effect_type.to_string(),
                description: "test".to_string(),
                parameters: vec![],
            }],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec![],
        }
    }

    #[test]
    fn test_compliance_matrix_generation() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                create_test_statute("US-CA-law-1", "GRANT"),
                create_test_statute("US-CA-law-2", "REVOKE"),
                create_test_statute("US-NY-law-1", "GRANT"),
            ],
        };

        let matrix = ComplianceMatrix::from_document(&doc);

        assert_eq!(matrix.jurisdictions.len(), 2);
        assert!(matrix.jurisdictions.contains(&"US-CA".to_string()));
        assert!(matrix.jurisdictions.contains(&"US-NY".to_string()));

        assert_eq!(matrix.requirement_types.len(), 2);
        assert!(matrix.requirement_types.contains(&"GRANT".to_string()));
        assert!(matrix.requirement_types.contains(&"REVOKE".to_string()));
    }

    #[test]
    fn test_compliance_stats() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                create_test_statute("US-CA-law-1", "GRANT"),
                create_test_statute("US-CA-law-2", "GRANT"),
            ],
        };

        let matrix = ComplianceMatrix::from_document(&doc);

        assert_eq!(matrix.stats.total_statutes, 2);
        assert_eq!(matrix.stats.statutes_with_jurisdiction, 2);
        assert_eq!(matrix.stats.independent_statutes, 2);
    }

    #[test]
    fn test_csv_export() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                create_test_statute("US-CA-law-1", "GRANT"),
                create_test_statute("US-NY-law-1", "REVOKE"),
            ],
        };

        let matrix = ComplianceMatrix::from_document(&doc);
        let csv = matrix.to_csv();

        assert!(csv.contains("Jurisdiction"));
        assert!(csv.contains("GRANT"));
        assert!(csv.contains("REVOKE"));
        assert!(csv.contains("US-CA"));
        assert!(csv.contains("US-NY"));
    }

    #[test]
    fn test_find_gaps() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![
                create_test_statute("US-CA-law-1", "GRANT"),
                create_test_statute("US-NY-law-1", "REVOKE"),
                // US-CA has no REVOKE, US-NY has no GRANT
            ],
        };

        let matrix = ComplianceMatrix::from_document(&doc);
        let gaps = matrix.find_gaps();

        // Should have gaps: US-CA missing REVOKE, US-NY missing GRANT
        assert!(!gaps.is_empty());
        assert!(gaps.len() >= 2);
    }

    #[test]
    fn test_text_report() {
        let doc = LegalDocument {
            imports: vec![],
            statutes: vec![create_test_statute("US-CA-law-1", "GRANT")],
        };

        let matrix = ComplianceMatrix::from_document(&doc);
        let report = matrix.to_text_report();

        assert!(report.contains("COMPLIANCE MATRIX"));
        assert!(report.contains("Statistics:"));
        assert!(report.contains("Jurisdiction Coverage:"));
    }
}
