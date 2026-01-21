//! Temporal consistency checking for time-aware legal knowledge graphs.
//!
//! This module provides validation and consistency checking for temporal data,
//! including:
//! - Temporal constraint validation
//! - Conflict detection between time periods
//! - Integrity checking for bitemporal data
//! - Gap detection in temporal coverage

use crate::temporal_rdf::{TemporalGraph, TemporalTriple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Temporal consistency checker
#[derive(Debug, Clone)]
pub struct TemporalConsistencyChecker {
    /// Validation rules
    rules: Vec<ConsistencyRule>,
    /// Detected violations
    violations: Vec<Violation>,
}

impl Default for TemporalConsistencyChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl TemporalConsistencyChecker {
    /// Creates a new consistency checker with default rules
    pub fn new() -> Self {
        let mut checker = Self {
            rules: Vec::new(),
            violations: Vec::new(),
        };

        // Add default rules
        checker.add_rule(ConsistencyRule::NoOverlappingValidTimes);
        checker.add_rule(ConsistencyRule::ValidTimeBeforeTransactionTime);
        checker.add_rule(ConsistencyRule::ChronologicalOrder);
        checker.add_rule(ConsistencyRule::NoGapsInCoverage);

        checker
    }

    /// Adds a consistency rule
    pub fn add_rule(&mut self, rule: ConsistencyRule) {
        self.rules.push(rule);
    }

    /// Validates a temporal graph
    pub fn validate(&mut self, graph: &TemporalGraph) -> ConsistencyReport {
        self.violations.clear();

        let rules = self.rules.clone();
        for rule in &rules {
            match rule {
                ConsistencyRule::NoOverlappingValidTimes => {
                    self.check_no_overlapping_valid_times(graph);
                }
                ConsistencyRule::ValidTimeBeforeTransactionTime => {
                    self.check_valid_before_transaction(graph);
                }
                ConsistencyRule::ChronologicalOrder => {
                    self.check_chronological_order(graph);
                }
                ConsistencyRule::NoGapsInCoverage => {
                    self.check_no_gaps(graph);
                }
            }
        }

        ConsistencyReport {
            total_triples: graph.triples.len(),
            violations: self.violations.clone(),
            is_consistent: self.violations.is_empty(),
        }
    }

    fn check_no_overlapping_valid_times(&mut self, graph: &TemporalGraph) {
        // Group triples by subject+predicate
        let mut groups: HashMap<String, Vec<&TemporalTriple>> = HashMap::new();

        for triple in &graph.triples {
            let key = format!("{}::{}", triple.triple.subject, triple.triple.predicate);
            groups.entry(key).or_default().push(triple);
        }

        // Check for overlaps within each group
        for (key, triples) in groups {
            for i in 0..triples.len() {
                for j in (i + 1)..triples.len() {
                    if let (Some(vt1), Some(vt2)) = (&triples[i].valid_time, &triples[j].valid_time)
                        && vt1.overlaps(vt2)
                    {
                        self.violations.push(Violation {
                            rule: ConsistencyRule::NoOverlappingValidTimes,
                            description: format!(
                                "Overlapping valid times for {} with values {:?} and {:?}",
                                key, triples[i].triple.object, triples[j].triple.object
                            ),
                            severity: Severity::Error,
                        });
                    }
                }
            }
        }
    }

    fn check_valid_before_transaction(&mut self, graph: &TemporalGraph) {
        for triple in &graph.triples {
            if let (Some(vt), Some(tt)) = (&triple.valid_time, &triple.transaction_time) {
                // Valid time start should typically not be after transaction time start
                if vt.start > tt.start {
                    self.violations.push(Violation {
                        rule: ConsistencyRule::ValidTimeBeforeTransactionTime,
                        description: format!(
                            "Valid time starts after transaction time for triple: {} {} {:?}",
                            triple.triple.subject, triple.triple.predicate, triple.triple.object
                        ),
                        severity: Severity::Warning,
                    });
                }
            }
        }
    }

    fn check_chronological_order(&mut self, graph: &TemporalGraph) {
        let mut sorted_triples = graph.triples.clone();
        sorted_triples.sort_by(|a, b| {
            let a_time = a.transaction_time.as_ref().map(|t| t.start);
            let b_time = b.transaction_time.as_ref().map(|t| t.start);
            a_time.cmp(&b_time)
        });

        // Check that transaction times are non-decreasing
        for i in 1..sorted_triples.len() {
            if let (Some(tt_prev), Some(tt_curr)) = (
                &sorted_triples[i - 1].transaction_time,
                &sorted_triples[i].transaction_time,
            ) && tt_curr.start < tt_prev.start
            {
                self.violations.push(Violation {
                    rule: ConsistencyRule::ChronologicalOrder,
                    description: "Transaction times are not in chronological order".to_string(),
                    severity: Severity::Error,
                });
            }
        }
    }

    fn check_no_gaps(&mut self, graph: &TemporalGraph) {
        // Group by subject+predicate
        let mut groups: HashMap<String, Vec<&TemporalTriple>> = HashMap::new();

        for triple in &graph.triples {
            let key = format!("{}::{}", triple.triple.subject, triple.triple.predicate);
            groups.entry(key).or_default().push(triple);
        }

        // Check for gaps in each group
        for (key, mut triples) in groups {
            // Sort by valid time start
            triples.sort_by(|a, b| {
                let a_time = a.valid_time.as_ref().map(|t| t.start);
                let b_time = b.valid_time.as_ref().map(|t| t.start);
                a_time.cmp(&b_time)
            });

            // Check consecutive periods
            for i in 1..triples.len() {
                if let (Some(vt_prev), Some(vt_curr)) =
                    (&triples[i - 1].valid_time, &triples[i].valid_time)
                    && let Some(prev_end) = vt_prev.end
                    && vt_curr.start > prev_end
                {
                    self.violations.push(Violation {
                        rule: ConsistencyRule::NoGapsInCoverage,
                        description: format!("Gap detected in temporal coverage for {}", key),
                        severity: Severity::Warning,
                    });
                }
            }
        }
    }
}

/// Consistency rules for temporal data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsistencyRule {
    /// No overlapping valid time periods for same subject+predicate
    NoOverlappingValidTimes,
    /// Valid time should not start after transaction time
    ValidTimeBeforeTransactionTime,
    /// Transaction times should be in chronological order
    ChronologicalOrder,
    /// No gaps in temporal coverage
    NoGapsInCoverage,
}

/// Severity of a violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Critical error
    Error,
    /// Warning
    Warning,
    /// Informational
    Info,
}

/// A consistency violation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Violation {
    /// The rule that was violated
    pub rule: ConsistencyRule,
    /// Description of the violation
    pub description: String,
    /// Severity level
    pub severity: Severity,
}

/// Consistency check report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyReport {
    /// Total number of temporal triples checked
    pub total_triples: usize,
    /// List of violations found
    pub violations: Vec<Violation>,
    /// Whether the graph is consistent
    pub is_consistent: bool,
}

impl ConsistencyReport {
    /// Gets violations by severity
    pub fn violations_by_severity(&self, severity: Severity) -> Vec<&Violation> {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .collect()
    }

    /// Counts violations by rule
    pub fn count_by_rule(&self, rule: ConsistencyRule) -> usize {
        self.violations.iter().filter(|v| v.rule == rule).count()
    }

    /// Gets a summary string
    pub fn summary(&self) -> String {
        if self.is_consistent {
            format!("All {} temporal triples are consistent", self.total_triples)
        } else {
            format!(
                "{} violations found in {} temporal triples ({} errors, {} warnings)",
                self.violations.len(),
                self.total_triples,
                self.violations_by_severity(Severity::Error).len(),
                self.violations_by_severity(Severity::Warning).len()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::temporal_rdf::TimePeriod;
    use crate::{RdfValue, Triple};
    use chrono::Utc;

    fn sample_triple(value: &str) -> Triple {
        Triple {
            subject: "ex:law1".to_string(),
            predicate: "ex:hasStatus".to_string(),
            object: RdfValue::string(value),
        }
    }

    #[test]
    fn test_no_violations() {
        let mut graph = TemporalGraph::new("https://example.org/");

        let now = Utc::now();
        let period1 = TimePeriod::new(now, Some(now + chrono::Duration::hours(1)));

        let triple = TemporalTriple::new(sample_triple("active")).with_valid_time(period1);

        graph.add_triple(triple);

        let mut checker = TemporalConsistencyChecker::new();
        let report = checker.validate(&graph);

        assert!(report.is_consistent);
        assert_eq!(report.violations.len(), 0);
    }

    #[test]
    fn test_overlapping_valid_times() {
        let mut graph = TemporalGraph::new("https://example.org/");

        let now = Utc::now();
        let period1 = TimePeriod::new(now, Some(now + chrono::Duration::hours(2)));
        let period2 = TimePeriod::new(
            now + chrono::Duration::hours(1),
            Some(now + chrono::Duration::hours(3)),
        );

        let triple1 = TemporalTriple::new(sample_triple("active")).with_valid_time(period1);
        let triple2 = TemporalTriple::new(sample_triple("inactive")).with_valid_time(period2);

        graph.add_triple(triple1);
        graph.add_triple(triple2);

        let mut checker = TemporalConsistencyChecker::new();
        let report = checker.validate(&graph);

        assert!(!report.is_consistent);
        assert!(report.count_by_rule(ConsistencyRule::NoOverlappingValidTimes) > 0);
    }

    #[test]
    fn test_valid_after_transaction() {
        let mut graph = TemporalGraph::new("https://example.org/");

        let now = Utc::now();
        let valid_period = TimePeriod::new(now + chrono::Duration::days(1), None);
        let trans_period = TimePeriod::new(now, None);

        let triple = TemporalTriple::new(sample_triple("active"))
            .with_valid_time(valid_period)
            .with_transaction_time(trans_period);

        graph.add_triple(triple);

        let mut checker = TemporalConsistencyChecker::new();
        let report = checker.validate(&graph);

        // This might be a warning, not necessarily an error
        let warnings = report.violations_by_severity(Severity::Warning);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_report_summary() {
        let report = ConsistencyReport {
            total_triples: 10,
            violations: vec![
                Violation {
                    rule: ConsistencyRule::NoOverlappingValidTimes,
                    description: "Test error".to_string(),
                    severity: Severity::Error,
                },
                Violation {
                    rule: ConsistencyRule::NoGapsInCoverage,
                    description: "Test warning".to_string(),
                    severity: Severity::Warning,
                },
            ],
            is_consistent: false,
        };

        let summary = report.summary();
        assert!(summary.contains("2 violations"));
        assert!(summary.contains("1 errors"));
        assert!(summary.contains("1 warnings"));
    }

    #[test]
    fn test_violations_by_severity() {
        let report = ConsistencyReport {
            total_triples: 5,
            violations: vec![
                Violation {
                    rule: ConsistencyRule::NoOverlappingValidTimes,
                    description: "Error 1".to_string(),
                    severity: Severity::Error,
                },
                Violation {
                    rule: ConsistencyRule::NoGapsInCoverage,
                    description: "Warning 1".to_string(),
                    severity: Severity::Warning,
                },
                Violation {
                    rule: ConsistencyRule::NoOverlappingValidTimes,
                    description: "Error 2".to_string(),
                    severity: Severity::Error,
                },
            ],
            is_consistent: false,
        };

        let errors = report.violations_by_severity(Severity::Error);
        assert_eq!(errors.len(), 2);

        let warnings = report.violations_by_severity(Severity::Warning);
        assert_eq!(warnings.len(), 1);
    }
}
