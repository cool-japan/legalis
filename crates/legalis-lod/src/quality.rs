//! Data quality metrics for RDF/LOD data.
//!
//! This module provides utilities to measure and report on the quality
//! of RDF data, including completeness, consistency, accuracy, and timeliness.

use crate::{LodResult, Triple};
use std::collections::{HashMap, HashSet};

/// Data quality dimensions following ISO/IEC 25012 and W3C best practices.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QualityDimension {
    /// Completeness - extent to which data is present
    Completeness,
    /// Consistency - adherence to semantic rules
    Consistency,
    /// Accuracy - degree of correctness
    Accuracy,
    /// Timeliness - currency of the data
    Timeliness,
    /// Accessibility - ease of access
    Accessibility,
    /// Conformity - adherence to standards
    Conformity,
}

/// Quality metric score (0.0 to 1.0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QualityScore {
    /// Score value between 0.0 and 1.0
    pub score: f64,
    /// Weight for aggregation
    pub weight: f64,
}

impl QualityScore {
    /// Creates a new quality score.
    pub fn new(score: f64, weight: f64) -> Self {
        Self {
            score: score.clamp(0.0, 1.0),
            weight: weight.max(0.0),
        }
    }

    /// Creates a perfect score (1.0).
    pub fn perfect() -> Self {
        Self::new(1.0, 1.0)
    }

    /// Creates a failing score (0.0).
    pub fn fail() -> Self {
        Self::new(0.0, 1.0)
    }
}

/// Data quality report.
#[derive(Debug, Clone)]
pub struct QualityReport {
    /// Scores by quality dimension
    pub dimension_scores: HashMap<QualityDimension, QualityScore>,
    /// Overall quality score (weighted average)
    pub overall_score: f64,
    /// Specific issues found
    pub issues: Vec<QualityIssue>,
    /// Number of triples analyzed
    pub triple_count: usize,
    /// Number of unique subjects
    pub subject_count: usize,
    /// Number of unique properties
    pub property_count: usize,
}

/// Data quality issue.
#[derive(Debug, Clone)]
pub struct QualityIssue {
    /// Issue type
    pub issue_type: QualityIssueType,
    /// Severity level
    pub severity: Severity,
    /// Description of the issue
    pub description: String,
    /// Subject URI (if applicable)
    pub subject: Option<String>,
    /// Property URI (if applicable)
    pub property: Option<String>,
}

/// Types of quality issues.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualityIssueType {
    /// Missing required property
    MissingProperty,
    /// Empty literal value
    EmptyValue,
    /// Invalid URI format
    InvalidUri,
    /// Orphan resource (no incoming links)
    OrphanResource,
    /// Duplicate triple
    Duplicate,
    /// Inconsistent datatype
    InconsistentDatatype,
    /// Missing language tag
    MissingLanguageTag,
    /// Deprecated property
    DeprecatedProperty,
    /// Outdated data
    OutdatedData,
}

/// Issue severity level.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Low severity - minor issue
    Low,
    /// Medium severity - should be fixed
    Medium,
    /// High severity - must be fixed
    High,
    /// Critical severity - data is invalid
    Critical,
}

/// Data quality analyzer.
#[derive(Debug)]
pub struct QualityAnalyzer {
    /// Required properties for statutes
    required_properties: HashSet<String>,
    /// Recommended properties
    recommended_properties: HashSet<String>,
    /// Deprecated properties
    deprecated_properties: HashSet<String>,
}

impl Default for QualityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl QualityAnalyzer {
    /// Creates a new quality analyzer with default rules.
    pub fn new() -> Self {
        let mut required = HashSet::new();
        required.insert("rdf:type".to_string());
        required.insert("eli:title".to_string());
        required.insert("dcterms:identifier".to_string());

        let mut recommended = HashSet::new();
        recommended.insert("dcterms:title".to_string());
        recommended.insert("eli:jurisdiction".to_string());
        recommended.insert("eli:date_document".to_string());
        recommended.insert("dcterms:license".to_string());
        recommended.insert("prov:wasAttributedTo".to_string());

        Self {
            required_properties: required,
            recommended_properties: recommended,
            deprecated_properties: HashSet::new(),
        }
    }

    /// Adds a required property.
    pub fn add_required_property(&mut self, property: impl Into<String>) {
        self.required_properties.insert(property.into());
    }

    /// Adds a recommended property.
    pub fn add_recommended_property(&mut self, property: impl Into<String>) {
        self.recommended_properties.insert(property.into());
    }

    /// Adds a deprecated property.
    pub fn add_deprecated_property(&mut self, property: impl Into<String>) {
        self.deprecated_properties.insert(property.into());
    }

    /// Analyzes the quality of RDF triples.
    pub fn analyze(&self, triples: &[Triple]) -> LodResult<QualityReport> {
        let mut issues = Vec::new();
        let mut dimension_scores = HashMap::new();

        // Group triples by subject
        let by_subject = self.group_by_subject(triples);

        // Calculate completeness
        let completeness = self.calculate_completeness(&by_subject, &mut issues);
        dimension_scores.insert(QualityDimension::Completeness, completeness);

        // Calculate consistency
        let consistency = self.calculate_consistency(triples, &mut issues);
        dimension_scores.insert(QualityDimension::Consistency, consistency);

        // Calculate accuracy
        let accuracy = self.calculate_accuracy(triples, &mut issues);
        dimension_scores.insert(QualityDimension::Accuracy, accuracy);

        // Calculate conformity
        let conformity = self.calculate_conformity(triples, &mut issues);
        dimension_scores.insert(QualityDimension::Conformity, conformity);

        // Calculate overall score (weighted average)
        let overall_score = self.calculate_overall_score(&dimension_scores);

        // Count statistics
        let triple_count = triples.len();
        let subject_count = by_subject.len();
        let property_count = triples
            .iter()
            .map(|t| &t.predicate)
            .collect::<HashSet<_>>()
            .len();

        Ok(QualityReport {
            dimension_scores,
            overall_score,
            issues,
            triple_count,
            subject_count,
            property_count,
        })
    }

    fn group_by_subject<'a>(&self, triples: &'a [Triple]) -> HashMap<&'a str, Vec<&'a Triple>> {
        let mut by_subject: HashMap<&str, Vec<&Triple>> = HashMap::new();
        for triple in triples {
            by_subject.entry(&triple.subject).or_default().push(triple);
        }
        by_subject
    }

    fn calculate_completeness(
        &self,
        by_subject: &HashMap<&str, Vec<&Triple>>,
        issues: &mut Vec<QualityIssue>,
    ) -> QualityScore {
        let mut total_required = 0;
        let mut found_required = 0;
        let mut total_recommended = 0;
        let mut found_recommended = 0;

        for (subject, subject_triples) in by_subject {
            let properties: HashSet<&str> = subject_triples
                .iter()
                .map(|t| t.predicate.as_str())
                .collect();

            // Check required properties
            for req in &self.required_properties {
                total_required += 1;
                if properties.contains(req.as_str()) {
                    found_required += 1;
                } else {
                    issues.push(QualityIssue {
                        issue_type: QualityIssueType::MissingProperty,
                        severity: Severity::Critical,
                        description: format!("Missing required property: {}", req),
                        subject: Some(subject.to_string()),
                        property: Some(req.clone()),
                    });
                }
            }

            // Check recommended properties
            for rec in &self.recommended_properties {
                total_recommended += 1;
                if properties.contains(rec.as_str()) {
                    found_recommended += 1;
                } else {
                    issues.push(QualityIssue {
                        issue_type: QualityIssueType::MissingProperty,
                        severity: Severity::Low,
                        description: format!("Missing recommended property: {}", rec),
                        subject: Some(subject.to_string()),
                        property: Some(rec.clone()),
                    });
                }
            }
        }

        // Weighted score: required (70%), recommended (30%)
        let required_score = if total_required > 0 {
            found_required as f64 / total_required as f64
        } else {
            1.0
        };

        let recommended_score = if total_recommended > 0 {
            found_recommended as f64 / total_recommended as f64
        } else {
            1.0
        };

        let score = (required_score * 0.7) + (recommended_score * 0.3);
        QualityScore::new(score, 1.0)
    }

    fn calculate_consistency(
        &self,
        triples: &[Triple],
        issues: &mut Vec<QualityIssue>,
    ) -> QualityScore {
        let mut consistent_count = 0;
        let total = triples.len();

        // Check for duplicate triples
        let mut seen = HashSet::new();
        for triple in triples {
            let key = (
                &triple.subject,
                &triple.predicate,
                format!("{:?}", triple.object),
            );
            if !seen.insert(key) {
                issues.push(QualityIssue {
                    issue_type: QualityIssueType::Duplicate,
                    severity: Severity::Medium,
                    description: "Duplicate triple found".to_string(),
                    subject: Some(triple.subject.clone()),
                    property: Some(triple.predicate.clone()),
                });
            } else {
                consistent_count += 1;
            }
        }

        let score = if total > 0 {
            consistent_count as f64 / total as f64
        } else {
            1.0
        };

        QualityScore::new(score, 1.0)
    }

    fn calculate_accuracy(
        &self,
        triples: &[Triple],
        issues: &mut Vec<QualityIssue>,
    ) -> QualityScore {
        let mut accurate_count = 0;
        let total = triples.len();

        for triple in triples {
            let mut is_accurate = true;

            // Check for empty literals
            if let crate::RdfValue::Literal(s, _) = &triple.object
                && s.trim().is_empty()
            {
                is_accurate = false;
                issues.push(QualityIssue {
                    issue_type: QualityIssueType::EmptyValue,
                    severity: Severity::High,
                    description: "Empty literal value".to_string(),
                    subject: Some(triple.subject.clone()),
                    property: Some(triple.predicate.clone()),
                });
            }

            // Check for invalid URIs
            if let crate::RdfValue::Uri(uri) = &triple.object
                && !self.is_valid_uri(uri)
            {
                is_accurate = false;
                issues.push(QualityIssue {
                    issue_type: QualityIssueType::InvalidUri,
                    severity: Severity::High,
                    description: format!("Invalid URI: {}", uri),
                    subject: Some(triple.subject.clone()),
                    property: Some(triple.predicate.clone()),
                });
            }

            if is_accurate {
                accurate_count += 1;
            }
        }

        let score = if total > 0 {
            accurate_count as f64 / total as f64
        } else {
            1.0
        };

        QualityScore::new(score, 1.0)
    }

    fn calculate_conformity(
        &self,
        triples: &[Triple],
        issues: &mut Vec<QualityIssue>,
    ) -> QualityScore {
        let mut conforming_count = 0;
        let total = triples.len();

        for triple in triples {
            let mut is_conforming = true;

            // Check for deprecated properties
            if self.deprecated_properties.contains(&triple.predicate) {
                is_conforming = false;
                issues.push(QualityIssue {
                    issue_type: QualityIssueType::DeprecatedProperty,
                    severity: Severity::Medium,
                    description: format!("Use of deprecated property: {}", triple.predicate),
                    subject: Some(triple.subject.clone()),
                    property: Some(triple.predicate.clone()),
                });
            }

            if is_conforming {
                conforming_count += 1;
            }
        }

        let score = if total > 0 {
            conforming_count as f64 / total as f64
        } else {
            1.0
        };

        QualityScore::new(score, 1.0)
    }

    fn calculate_overall_score(
        &self,
        dimension_scores: &HashMap<QualityDimension, QualityScore>,
    ) -> f64 {
        let total_weight: f64 = dimension_scores.values().map(|s| s.weight).sum();

        if total_weight == 0.0 {
            return 0.0;
        }

        let weighted_sum: f64 = dimension_scores.values().map(|s| s.score * s.weight).sum();

        weighted_sum / total_weight
    }

    fn is_valid_uri(&self, uri: &str) -> bool {
        // Basic URI validation
        !uri.is_empty()
            && (uri.starts_with("http://") || uri.starts_with("https://") || uri.contains(':'))
    }
}

impl QualityReport {
    /// Returns issues by severity.
    pub fn issues_by_severity(&self, severity: Severity) -> Vec<&QualityIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == severity)
            .collect()
    }

    /// Returns issues by type.
    pub fn issues_by_type(&self, issue_type: QualityIssueType) -> Vec<&QualityIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.issue_type == issue_type)
            .collect()
    }

    /// Returns the score for a specific dimension.
    pub fn dimension_score(&self, dimension: QualityDimension) -> Option<f64> {
        self.dimension_scores.get(&dimension).map(|s| s.score)
    }

    /// Checks if the data meets a minimum quality threshold.
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.overall_score >= threshold
    }

    /// Generates a summary report as text.
    pub fn summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str(&format!(
            "Overall Quality Score: {:.2}%\n",
            self.overall_score * 100.0
        ));
        summary.push_str(&format!("Triples Analyzed: {}\n", self.triple_count));
        summary.push_str(&format!("Unique Subjects: {}\n", self.subject_count));
        summary.push_str(&format!("Unique Properties: {}\n\n", self.property_count));

        summary.push_str("Dimension Scores:\n");
        for (dimension, score) in &self.dimension_scores {
            summary.push_str(&format!("  {:?}: {:.2}%\n", dimension, score.score * 100.0));
        }

        summary.push_str(&format!("\nTotal Issues: {}\n", self.issues.len()));
        summary.push_str(&format!(
            "  Critical: {}\n",
            self.issues_by_severity(Severity::Critical).len()
        ));
        summary.push_str(&format!(
            "  High: {}\n",
            self.issues_by_severity(Severity::High).len()
        ));
        summary.push_str(&format!(
            "  Medium: {}\n",
            self.issues_by_severity(Severity::Medium).len()
        ));
        summary.push_str(&format!(
            "  Low: {}\n",
            self.issues_by_severity(Severity::Low).len()
        ));

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfValue;

    fn sample_triples() -> Vec<Triple> {
        vec![
            Triple {
                subject: "statute:1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("eli:LegalResource".to_string()),
            },
            Triple {
                subject: "statute:1".to_string(),
                predicate: "eli:title".to_string(),
                object: RdfValue::string("Test Statute"),
            },
            Triple {
                subject: "statute:1".to_string(),
                predicate: "dcterms:identifier".to_string(),
                object: RdfValue::string("statute-1"),
            },
        ]
    }

    #[test]
    fn test_quality_analyzer_basic() {
        let analyzer = QualityAnalyzer::new();
        let triples = sample_triples();
        let report = analyzer.analyze(&triples).unwrap();

        assert_eq!(report.triple_count, 3);
        assert_eq!(report.subject_count, 1);
        assert!(report.overall_score > 0.0);
    }

    #[test]
    fn test_quality_score() {
        let score = QualityScore::new(0.85, 1.0);
        assert_eq!(score.score, 0.85);
        assert_eq!(score.weight, 1.0);

        let perfect = QualityScore::perfect();
        assert_eq!(perfect.score, 1.0);

        let fail = QualityScore::fail();
        assert_eq!(fail.score, 0.0);
    }

    #[test]
    fn test_completeness_calculation() {
        let analyzer = QualityAnalyzer::new();
        let triples = sample_triples();
        let report = analyzer.analyze(&triples).unwrap();

        // Should have high completeness as all required properties are present
        let completeness = report
            .dimension_score(QualityDimension::Completeness)
            .unwrap();
        assert!(completeness >= 0.7);
    }

    #[test]
    fn test_missing_required_property() {
        let analyzer = QualityAnalyzer::new();
        let mut triples = sample_triples();
        // Remove the required eli:title property
        triples.retain(|t| t.predicate != "eli:title");

        let report = analyzer.analyze(&triples).unwrap();

        // Should have critical issues
        let critical_issues = report.issues_by_severity(Severity::Critical);
        assert!(!critical_issues.is_empty());
    }

    #[test]
    fn test_duplicate_detection() {
        let analyzer = QualityAnalyzer::new();
        let mut triples = sample_triples();
        // Add a duplicate
        triples.push(triples[0].clone());

        let report = analyzer.analyze(&triples).unwrap();

        // Should detect duplicate
        let duplicate_issues = report.issues_by_type(QualityIssueType::Duplicate);
        assert!(!duplicate_issues.is_empty());
    }

    #[test]
    fn test_empty_value_detection() {
        let analyzer = QualityAnalyzer::new();
        let mut triples = sample_triples();
        triples.push(Triple {
            subject: "statute:1".to_string(),
            predicate: "dcterms:description".to_string(),
            object: RdfValue::string("   "),
        });

        let report = analyzer.analyze(&triples).unwrap();

        let empty_issues = report.issues_by_type(QualityIssueType::EmptyValue);
        assert!(!empty_issues.is_empty());
    }

    #[test]
    fn test_quality_report_summary() {
        let analyzer = QualityAnalyzer::new();
        let triples = sample_triples();
        let report = analyzer.analyze(&triples).unwrap();

        let summary = report.summary();
        assert!(summary.contains("Overall Quality Score"));
        assert!(summary.contains("Triples Analyzed"));
        assert!(summary.contains("Dimension Scores"));
    }

    #[test]
    fn test_meets_threshold() {
        let analyzer = QualityAnalyzer::new();
        let triples = sample_triples();
        let report = analyzer.analyze(&triples).unwrap();

        assert!(report.meets_threshold(0.5));
    }

    #[test]
    fn test_custom_properties() {
        let mut analyzer = QualityAnalyzer::new();
        analyzer.add_required_property("custom:required");
        analyzer.add_recommended_property("custom:recommended");
        analyzer.add_deprecated_property("old:property");

        assert!(analyzer.required_properties.contains("custom:required"));
        assert!(
            analyzer
                .recommended_properties
                .contains("custom:recommended")
        );
        assert!(analyzer.deprecated_properties.contains("old:property"));
    }

    #[test]
    fn test_deprecated_property_detection() {
        let mut analyzer = QualityAnalyzer::new();
        analyzer.add_deprecated_property("old:property");

        let mut triples = sample_triples();
        triples.push(Triple {
            subject: "statute:1".to_string(),
            predicate: "old:property".to_string(),
            object: RdfValue::string("value"),
        });

        let report = analyzer.analyze(&triples).unwrap();

        let deprecated_issues = report.issues_by_type(QualityIssueType::DeprecatedProperty);
        assert!(!deprecated_issues.is_empty());
    }

    #[test]
    fn test_dimension_scores() {
        let analyzer = QualityAnalyzer::new();
        let triples = sample_triples();
        let report = analyzer.analyze(&triples).unwrap();

        assert!(
            report
                .dimension_score(QualityDimension::Completeness)
                .is_some()
        );
        assert!(
            report
                .dimension_score(QualityDimension::Consistency)
                .is_some()
        );
        assert!(report.dimension_score(QualityDimension::Accuracy).is_some());
        assert!(
            report
                .dimension_score(QualityDimension::Conformity)
                .is_some()
        );
    }
}
