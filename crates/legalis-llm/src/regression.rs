//! Regression testing for prompt changes.
//!
//! This module provides tools for testing how prompt changes affect
//! LLM outputs, helping detect unwanted regressions in quality.

use crate::LLMProvider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A test case for regression testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTestCase {
    /// Test case name
    pub name: String,
    /// The prompt to test
    pub prompt: String,
    /// Expected output patterns (regex patterns)
    pub expected_patterns: Vec<String>,
    /// Baseline output (from previous version)
    pub baseline_output: Option<String>,
    /// Metadata for the test case
    pub metadata: HashMap<String, String>,
}

impl RegressionTestCase {
    /// Creates a new regression test case.
    pub fn new(name: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            prompt: prompt.into(),
            expected_patterns: Vec::new(),
            baseline_output: None,
            metadata: HashMap::new(),
        }
    }

    /// Adds an expected pattern that should be present in the output.
    pub fn expect_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.expected_patterns.push(pattern.into());
        self
    }

    /// Sets the baseline output from a previous prompt version.
    pub fn with_baseline(mut self, baseline: impl Into<String>) -> Self {
        self.baseline_output = Some(baseline.into());
        self
    }

    /// Adds metadata to the test case.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Result of running a regression test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTestResult {
    /// Test case name
    pub test_name: String,
    /// Generated output
    pub output: String,
    /// Whether all expected patterns were found
    pub patterns_matched: bool,
    /// Patterns that matched
    pub matched_patterns: Vec<String>,
    /// Patterns that didn't match
    pub missing_patterns: Vec<String>,
    /// Similarity to baseline (0.0 - 1.0)
    pub baseline_similarity: Option<f64>,
    /// Whether the test passed
    pub passed: bool,
    /// Failure reason if failed
    pub failure_reason: Option<String>,
}

impl RegressionTestResult {
    /// Creates a new regression test result.
    pub fn new(test_name: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            test_name: test_name.into(),
            output: output.into(),
            patterns_matched: true,
            matched_patterns: Vec::new(),
            missing_patterns: Vec::new(),
            baseline_similarity: None,
            passed: true,
            failure_reason: None,
        }
    }

    /// Marks the test as failed with a reason.
    pub fn fail(mut self, reason: impl Into<String>) -> Self {
        self.passed = false;
        self.failure_reason = Some(reason.into());
        self
    }

    /// Sets pattern matching results.
    pub fn with_pattern_results(mut self, matched: Vec<String>, missing: Vec<String>) -> Self {
        self.patterns_matched = missing.is_empty();
        self.matched_patterns = matched;
        self.missing_patterns = missing;
        self
    }

    /// Sets baseline similarity score.
    pub fn with_baseline_similarity(mut self, similarity: f64) -> Self {
        self.baseline_similarity = Some(similarity.clamp(0.0, 1.0));
        self
    }
}

/// Regression test suite.
pub struct RegressionTestSuite {
    test_cases: Vec<RegressionTestCase>,
    baseline_threshold: f64,
}

impl RegressionTestSuite {
    /// Creates a new regression test suite.
    pub fn new() -> Self {
        Self {
            test_cases: Vec::new(),
            baseline_threshold: 0.7,
        }
    }

    /// Sets the similarity threshold for baseline comparison (0.0 - 1.0).
    pub fn with_baseline_threshold(mut self, threshold: f64) -> Self {
        self.baseline_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Adds a test case to the suite.
    pub fn add_test(mut self, test: RegressionTestCase) -> Self {
        self.test_cases.push(test);
        self
    }

    /// Adds multiple test cases to the suite.
    pub fn add_tests(mut self, tests: Vec<RegressionTestCase>) -> Self {
        self.test_cases.extend(tests);
        self
    }

    /// Runs the regression test suite on a provider.
    pub async fn run<P: LLMProvider>(&self, provider: &P) -> RegressionTestReport {
        let mut results = Vec::new();

        for test in &self.test_cases {
            let result = self.run_test(provider, test).await;
            results.push(result);
        }

        RegressionTestReport::from_results(results)
    }

    /// Runs a single test case.
    async fn run_test<P: LLMProvider>(
        &self,
        provider: &P,
        test: &RegressionTestCase,
    ) -> RegressionTestResult {
        // Generate output
        let output = match provider.generate_text(&test.prompt).await {
            Ok(o) => o,
            Err(e) => {
                return RegressionTestResult::new(test.name.clone(), "")
                    .fail(format!("Generation failed: {}", e));
            }
        };

        let mut result = RegressionTestResult::new(test.name.clone(), output.clone());

        // Check expected patterns
        if !test.expected_patterns.is_empty() {
            let (matched, missing) = check_patterns(&output, &test.expected_patterns);

            if !missing.is_empty() {
                result = result.fail(format!("Missing patterns: {}", missing.join(", ")));
            }

            result = result.with_pattern_results(matched, missing);
        }

        // Compare with baseline if available
        if let Some(baseline) = &test.baseline_output {
            let similarity = compute_text_similarity(&output, baseline);
            result = result.with_baseline_similarity(similarity);

            if similarity < self.baseline_threshold {
                result = result.fail(format!(
                    "Output differs significantly from baseline (similarity: {:.2})",
                    similarity
                ));
            }
        }

        result
    }

    /// Returns the number of test cases.
    pub fn test_count(&self) -> usize {
        self.test_cases.len()
    }
}

impl Default for RegressionTestSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Report summarizing regression test results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTestReport {
    /// Individual test results
    pub results: Vec<RegressionTestResult>,
    /// Total number of tests
    pub total_tests: usize,
    /// Number of tests that passed
    pub passed_tests: usize,
    /// Number of tests that failed
    pub failed_tests: usize,
    /// Overall pass rate (0.0 - 1.0)
    pub pass_rate: f64,
}

impl RegressionTestReport {
    /// Creates a report from test results.
    pub fn from_results(results: Vec<RegressionTestResult>) -> Self {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        let pass_rate = if total_tests > 0 {
            passed_tests as f64 / total_tests as f64
        } else {
            0.0
        };

        Self {
            results,
            total_tests,
            passed_tests,
            failed_tests,
            pass_rate,
        }
    }

    /// Returns true if all tests passed.
    pub fn all_passed(&self) -> bool {
        self.failed_tests == 0
    }

    /// Returns failed test results.
    pub fn failed_tests_iter(&self) -> impl Iterator<Item = &RegressionTestResult> {
        self.results.iter().filter(|r| !r.passed)
    }

    /// Generates a summary string.
    pub fn summary(&self) -> String {
        format!(
            "Regression Tests: {}/{} passed ({:.1}%)",
            self.passed_tests,
            self.total_tests,
            self.pass_rate * 100.0
        )
    }
}

/// Checks which patterns match in the output.
fn check_patterns(output: &str, patterns: &[String]) -> (Vec<String>, Vec<String>) {
    let mut matched = Vec::new();
    let mut missing = Vec::new();

    for pattern in patterns {
        // Simple substring matching (could be extended to regex)
        if output.contains(pattern) {
            matched.push(pattern.clone());
        } else {
            missing.push(pattern.clone());
        }
    }

    (matched, missing)
}

/// Computes similarity between two texts (word overlap based).
fn compute_text_similarity(text1: &str, text2: &str) -> f64 {
    let text1_lower = text1.to_lowercase();
    let text2_lower = text2.to_lowercase();

    let words1: std::collections::HashSet<&str> = text1_lower.split_whitespace().collect();

    let words2: std::collections::HashSet<&str> = text2_lower.split_whitespace().collect();

    if words1.is_empty() && words2.is_empty() {
        return 1.0;
    }

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[test]
    fn test_regression_test_case_creation() {
        let test = RegressionTestCase::new("Test", "What is 2+2?")
            .expect_pattern("4")
            .with_baseline("The answer is 4")
            .with_metadata("category", "math");

        assert_eq!(test.name, "Test");
        assert_eq!(test.expected_patterns.len(), 1);
        assert!(test.baseline_output.is_some());
        assert_eq!(test.metadata.get("category"), Some(&"math".to_string()));
    }

    #[test]
    fn test_check_patterns() {
        let output = "The answer is 42 and it's correct";
        let patterns = vec!["answer".to_string(), "42".to_string(), "wrong".to_string()];

        let (matched, missing) = check_patterns(output, &patterns);

        assert_eq!(matched.len(), 2);
        assert_eq!(missing.len(), 1);
        assert!(matched.contains(&"answer".to_string()));
        assert!(matched.contains(&"42".to_string()));
        assert!(missing.contains(&"wrong".to_string()));
    }

    #[test]
    fn test_compute_text_similarity() {
        assert_eq!(compute_text_similarity("hello world", "hello world"), 1.0);
        assert!(compute_text_similarity("hello world", "hello there") > 0.3);
        assert!(compute_text_similarity("hello world", "hello there") < 0.7);
        assert!(compute_text_similarity("completely different", "unrelated text") < 0.3);
    }

    #[test]
    fn test_regression_test_result() {
        let result = RegressionTestResult::new("Test", "Output")
            .with_pattern_results(vec!["pattern1".to_string()], vec!["pattern2".to_string()])
            .with_baseline_similarity(0.85);

        assert!(!result.patterns_matched);
        assert_eq!(result.matched_patterns.len(), 1);
        assert_eq!(result.missing_patterns.len(), 1);
        assert_eq!(result.baseline_similarity, Some(0.85));
    }

    #[test]
    fn test_regression_test_suite_creation() {
        let suite = RegressionTestSuite::new()
            .with_baseline_threshold(0.8)
            .add_test(RegressionTestCase::new("Test1", "Prompt1"))
            .add_test(RegressionTestCase::new("Test2", "Prompt2"));

        assert_eq!(suite.test_count(), 2);
        assert_eq!(suite.baseline_threshold, 0.8);
    }

    #[tokio::test]
    async fn test_regression_test_run() {
        let provider = MockProvider::default();
        let suite = RegressionTestSuite::new().add_test(RegressionTestCase::new("Test", "Hello"));

        let report = suite.run(&provider).await;

        assert_eq!(report.total_tests, 1);
        // Test completes successfully even if no patterns are specified
        assert_eq!(report.passed_tests, 1);
    }

    #[test]
    fn test_regression_test_report() {
        let results = vec![
            RegressionTestResult::new("Test1", "Output1"),
            RegressionTestResult::new("Test2", "Output2").fail("Pattern missing"),
            RegressionTestResult::new("Test3", "Output3"),
        ];

        let report = RegressionTestReport::from_results(results);

        assert_eq!(report.total_tests, 3);
        assert_eq!(report.passed_tests, 2);
        assert_eq!(report.failed_tests, 1);
        assert!((report.pass_rate - 0.6666).abs() < 0.01);
        assert!(!report.all_passed());
    }

    #[test]
    fn test_regression_report_summary() {
        let results = vec![
            RegressionTestResult::new("Test1", "Output1"),
            RegressionTestResult::new("Test2", "Output2"),
        ];

        let report = RegressionTestReport::from_results(results);
        let summary = report.summary();

        assert!(summary.contains("2/2"));
        assert!(summary.contains("100"));
    }
}
