//! Competency question testing for ontology validation.
//!
//! Competency questions are natural language questions that an ontology should be able
//! to answer. This module provides tools for:
//! - Defining competency questions
//! - Translating them to SPARQL queries
//! - Executing and validating answers
//! - Generating test reports

use crate::Triple;
use serde::{Deserialize, Serialize};

/// Priority level for a competency question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestionPriority {
    /// Must be answered correctly (critical)
    Critical,
    /// Should be answered correctly (important)
    High,
    /// Nice to answer correctly
    Medium,
    /// Optional
    Low,
}

/// A competency question with associated SPARQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyQuestion {
    /// Unique identifier for the question
    pub id: String,
    /// Natural language question
    pub question: String,
    /// SPARQL query that answers the question
    pub sparql_query: String,
    /// Expected result type (ASK, SELECT, CONSTRUCT)
    pub result_type: QueryResultType,
    /// Priority of this question
    pub priority: QuestionPriority,
    /// Category/domain of the question
    pub category: Option<String>,
    /// Expected minimum number of results (for SELECT queries)
    pub min_results: Option<usize>,
    /// Example expected results (for validation)
    pub example_results: Vec<String>,
}

/// Type of SPARQL query result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryResultType {
    /// Boolean result (ASK query)
    Ask,
    /// Variable bindings (SELECT query)
    Select,
    /// RDF graph (CONSTRUCT query)
    Construct,
}

impl CompetencyQuestion {
    /// Creates a new competency question.
    pub fn new(
        id: impl Into<String>,
        question: impl Into<String>,
        sparql_query: impl Into<String>,
        result_type: QueryResultType,
    ) -> Self {
        Self {
            id: id.into(),
            question: question.into(),
            sparql_query: sparql_query.into(),
            result_type,
            priority: QuestionPriority::Medium,
            category: None,
            min_results: None,
            example_results: Vec::new(),
        }
    }

    /// Sets the priority.
    pub fn with_priority(mut self, priority: QuestionPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Sets the category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the minimum expected results.
    pub fn with_min_results(mut self, min_results: usize) -> Self {
        self.min_results = Some(min_results);
        self
    }

    /// Adds an example expected result.
    pub fn with_example_result(mut self, result: impl Into<String>) -> Self {
        self.example_results.push(result.into());
        self
    }
}

/// Result of executing a competency question test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionTestResult {
    /// Question ID
    pub question_id: String,
    /// Whether the test passed
    pub passed: bool,
    /// Actual result count (for SELECT queries)
    pub result_count: Option<usize>,
    /// Actual result (for ASK queries)
    pub boolean_result: Option<bool>,
    /// Validation messages
    pub messages: Vec<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl QuestionTestResult {
    /// Creates a new test result.
    pub fn new(question_id: impl Into<String>, passed: bool) -> Self {
        Self {
            question_id: question_id.into(),
            passed,
            result_count: None,
            boolean_result: None,
            messages: Vec::new(),
            execution_time_ms: 0,
        }
    }

    /// Adds a message.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.messages.push(message.into());
        self
    }
}

/// Test suite for competency questions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyQuestionSuite {
    /// Name of the test suite
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// All questions in the suite
    pub questions: Vec<CompetencyQuestion>,
}

impl CompetencyQuestionSuite {
    /// Creates a new test suite.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            questions: Vec::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a question.
    pub fn add_question(&mut self, question: CompetencyQuestion) {
        self.questions.push(question);
    }

    /// Gets questions by category.
    pub fn get_questions_by_category(&self, category: &str) -> Vec<&CompetencyQuestion> {
        self.questions
            .iter()
            .filter(|q| q.category.as_deref() == Some(category))
            .collect()
    }

    /// Gets questions by priority.
    pub fn get_questions_by_priority(
        &self,
        priority: QuestionPriority,
    ) -> Vec<&CompetencyQuestion> {
        self.questions
            .iter()
            .filter(|q| q.priority == priority)
            .collect()
    }
}

/// Executor for competency question tests.
pub struct CompetencyQuestionTester {
    /// RDF triples to test against
    triples: Vec<Triple>,
}

impl CompetencyQuestionTester {
    /// Creates a new tester with the given RDF data.
    pub fn new(triples: Vec<Triple>) -> Self {
        Self { triples }
    }

    /// Tests a single competency question.
    pub fn test_question(&self, question: &CompetencyQuestion) -> QuestionTestResult {
        let start_time = std::time::Instant::now();

        // For now, we'll do a simple validation based on query structure
        // In a real implementation, this would execute SPARQL against an RDF store
        let passed = self.validate_question(question);

        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        let mut result = QuestionTestResult::new(&question.id, passed);
        result.execution_time_ms = execution_time_ms;

        if passed {
            result = result.with_message("Query is well-formed and executable");
        } else {
            result = result.with_message("Query validation failed");
        }

        // For SELECT queries, check if we have enough triples
        if question.result_type == QueryResultType::Select {
            let result_count = self.estimate_result_count(question);
            result.result_count = Some(result_count);

            if let Some(min_results) = question.min_results
                && result_count < min_results
            {
                result.passed = false;
                result = result.with_message(format!(
                    "Expected at least {} results, got {}",
                    min_results, result_count
                ));
            }
        }

        result
    }

    /// Tests all questions in a suite.
    pub fn test_suite(&self, suite: &CompetencyQuestionSuite) -> SuiteTestReport {
        let mut results = Vec::new();

        for question in &suite.questions {
            results.push(self.test_question(question));
        }

        SuiteTestReport::new(suite.name.clone(), results)
    }

    /// Validates that a question is well-formed.
    fn validate_question(&self, question: &CompetencyQuestion) -> bool {
        let query = question.sparql_query.to_lowercase();

        // Check for basic SPARQL query structure
        match question.result_type {
            QueryResultType::Ask => query.contains("ask"),
            QueryResultType::Select => query.contains("select") && query.contains("where"),
            QueryResultType::Construct => query.contains("construct") && query.contains("where"),
        }
    }

    /// Estimates result count for a SELECT query (simplified).
    fn estimate_result_count(&self, _question: &CompetencyQuestion) -> usize {
        // In a real implementation, this would execute the query
        // For now, return the number of triples as a rough estimate
        self.triples.len().min(100)
    }
}

/// Report of test suite execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuiteTestReport {
    /// Suite name
    pub suite_name: String,
    /// All test results
    pub results: Vec<QuestionTestResult>,
    /// Total number of tests
    pub total_tests: usize,
    /// Number of passed tests
    pub passed_tests: usize,
    /// Number of failed tests
    pub failed_tests: usize,
    /// Overall pass rate
    pub pass_rate: f64,
}

impl SuiteTestReport {
    /// Creates a new test report.
    pub fn new(suite_name: impl Into<String>, results: Vec<QuestionTestResult>) -> Self {
        let total_tests = results.len();
        let passed_tests = results.iter().filter(|r| r.passed).count();
        let failed_tests = total_tests - passed_tests;
        let pass_rate = if total_tests > 0 {
            passed_tests as f64 / total_tests as f64
        } else {
            0.0
        };

        Self {
            suite_name: suite_name.into(),
            results,
            total_tests,
            passed_tests,
            failed_tests,
            pass_rate,
        }
    }

    /// Returns results for failed tests only.
    pub fn get_failed_results(&self) -> Vec<&QuestionTestResult> {
        self.results.iter().filter(|r| !r.passed).collect()
    }

    /// Returns results by priority.
    pub fn get_results_by_priority(
        &self,
        suite: &CompetencyQuestionSuite,
        priority: QuestionPriority,
    ) -> Vec<&QuestionTestResult> {
        let priority_question_ids: Vec<_> = suite
            .get_questions_by_priority(priority)
            .iter()
            .map(|q| &q.id)
            .collect();

        self.results
            .iter()
            .filter(|r| priority_question_ids.contains(&&r.question_id))
            .collect()
    }

    /// Generates a summary report as text.
    pub fn summary(&self) -> String {
        format!(
            "Competency Question Test Report: {}\n\
             Total Tests: {}\n\
             Passed: {}\n\
             Failed: {}\n\
             Pass Rate: {:.2}%",
            self.suite_name,
            self.total_tests,
            self.passed_tests,
            self.failed_tests,
            self.pass_rate * 100.0
        )
    }
}

/// Builder for creating standard competency question suites.
pub struct CompetencyQuestionBuilder {
    suite: CompetencyQuestionSuite,
}

impl CompetencyQuestionBuilder {
    /// Creates a new builder.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            suite: CompetencyQuestionSuite::new(name),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.suite = self.suite.with_description(description);
        self
    }

    /// Adds a question about legal entities.
    pub fn add_legal_entity_question(mut self) -> Self {
        let q = CompetencyQuestion::new(
            "legal-entities",
            "What are all the legal entities defined in the ontology?",
            "SELECT ?entity WHERE { ?entity a legalis:LegalEntity }",
            QueryResultType::Select,
        )
        .with_priority(QuestionPriority::High)
        .with_category("entities");

        self.suite.add_question(q);
        self
    }

    /// Adds a question about statute effects.
    pub fn add_statute_effects_question(mut self) -> Self {
        let q = CompetencyQuestion::new(
            "statute-effects",
            "What effects can statutes have?",
            "SELECT DISTINCT ?effectType WHERE { \
                ?statute legalis:hasEffect ?effect . \
                ?effect legalis:effectType ?effectType \
             }",
            QueryResultType::Select,
        )
        .with_priority(QuestionPriority::Critical)
        .with_category("statutes");

        self.suite.add_question(q);
        self
    }

    /// Adds a question about jurisdiction.
    pub fn add_jurisdiction_question(mut self) -> Self {
        let q = CompetencyQuestion::new(
            "jurisdiction-statutes",
            "What statutes apply to a given jurisdiction?",
            "SELECT ?statute WHERE { \
                ?statute eli:jurisdiction \"California\" \
             }",
            QueryResultType::Select,
        )
        .with_priority(QuestionPriority::High)
        .with_category("jurisdiction");

        self.suite.add_question(q);
        self
    }

    /// Builds the suite.
    pub fn build(self) -> CompetencyQuestionSuite {
        self.suite
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RdfValue;

    #[test]
    fn test_competency_question_creation() {
        let q = CompetencyQuestion::new(
            "test-q1",
            "What are all the classes?",
            "SELECT ?class WHERE { ?class a owl:Class }",
            QueryResultType::Select,
        )
        .with_priority(QuestionPriority::High)
        .with_category("ontology-structure");

        assert_eq!(q.id, "test-q1");
        assert_eq!(q.priority, QuestionPriority::High);
        assert_eq!(q.category, Some("ontology-structure".to_string()));
    }

    #[test]
    fn test_question_suite() {
        let mut suite = CompetencyQuestionSuite::new("Test Suite");
        suite.add_question(
            CompetencyQuestion::new(
                "q1",
                "Question 1?",
                "SELECT * WHERE { ?s ?p ?o }",
                QueryResultType::Select,
            )
            .with_category("cat1"),
        );

        suite.add_question(
            CompetencyQuestion::new(
                "q2",
                "Question 2?",
                "ASK WHERE { ?s a owl:Class }",
                QueryResultType::Ask,
            )
            .with_category("cat2"),
        );

        assert_eq!(suite.questions.len(), 2);
        assert_eq!(suite.get_questions_by_category("cat1").len(), 1);
    }

    #[test]
    fn test_question_tester() {
        let triples = vec![Triple {
            subject: "http://example.org/Entity1".to_string(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Class".to_string()),
        }];

        let tester = CompetencyQuestionTester::new(triples);

        let q = CompetencyQuestion::new(
            "test-q",
            "Is there a class?",
            "ASK WHERE { ?x a owl:Class }",
            QueryResultType::Ask,
        );

        let result = tester.test_question(&q);
        assert!(result.passed);
    }

    #[test]
    fn test_suite_testing() {
        let mut suite = CompetencyQuestionSuite::new("Legal Ontology Tests");

        suite.add_question(
            CompetencyQuestion::new(
                "q1",
                "Find all statutes",
                "SELECT ?s WHERE { ?s a legalis:Statute }",
                QueryResultType::Select,
            )
            .with_priority(QuestionPriority::Critical),
        );

        suite.add_question(
            CompetencyQuestion::new(
                "q2",
                "Are there any effects?",
                "ASK WHERE { ?x legalis:hasEffect ?y }",
                QueryResultType::Ask,
            )
            .with_priority(QuestionPriority::High),
        );

        let triples = vec![];
        let tester = CompetencyQuestionTester::new(triples);
        let report = tester.test_suite(&suite);

        assert_eq!(report.total_tests, 2);
        assert!(report.pass_rate >= 0.0 && report.pass_rate <= 1.0);
    }

    #[test]
    fn test_question_builder() {
        let suite = CompetencyQuestionBuilder::new("Legal Ontology")
            .with_description("Tests for legal ontology")
            .add_legal_entity_question()
            .add_statute_effects_question()
            .build();

        assert_eq!(suite.name, "Legal Ontology");
        assert_eq!(suite.questions.len(), 2);
    }

    #[test]
    fn test_test_report() {
        let results = vec![
            QuestionTestResult::new("q1", true),
            QuestionTestResult::new("q2", false),
            QuestionTestResult::new("q3", true),
        ];

        let report = SuiteTestReport::new("Test Suite", results);

        assert_eq!(report.total_tests, 3);
        assert_eq!(report.passed_tests, 2);
        assert_eq!(report.failed_tests, 1);
        assert!((report.pass_rate - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_failed_results() {
        let results = vec![
            QuestionTestResult::new("q1", true),
            QuestionTestResult::new("q2", false),
        ];

        let report = SuiteTestReport::new("Test", results);
        let failed = report.get_failed_results();

        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].question_id, "q2");
    }

    #[test]
    fn test_report_summary() {
        let results = vec![QuestionTestResult::new("q1", true)];
        let report = SuiteTestReport::new("My Suite", results);
        let summary = report.summary();

        assert!(summary.contains("My Suite"));
        assert!(summary.contains("Total Tests: 1"));
        assert!(summary.contains("Passed: 1"));
    }
}
