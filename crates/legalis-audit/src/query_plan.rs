//! Query plan explanation for audit trail queries.
//!
//! This module provides query plan analysis and optimization suggestions
//! for audit trail queries. It helps developers understand query performance
//! and identify optimization opportunities.

use serde::{Deserialize, Serialize};

/// A query execution plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPlan {
    /// Query description
    pub query_description: String,
    /// Execution steps
    pub steps: Vec<ExecutionStep>,
    /// Estimated cost
    pub estimated_cost: QueryCost,
    /// Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
}

/// An execution step in the query plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Step number
    pub step_number: usize,
    /// Operation description
    pub operation: String,
    /// Estimated rows scanned
    pub estimated_rows: usize,
    /// Whether this step uses an index
    pub uses_index: bool,
    /// Selectivity (0.0 to 1.0)
    pub selectivity: f64,
}

/// Query cost estimation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCost {
    /// Total number of records to scan
    pub total_scan_cost: usize,
    /// Expected result set size
    pub result_size: usize,
    /// Complexity level
    pub complexity: QueryComplexity,
}

/// Query complexity levels.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QueryComplexity {
    /// Simple query (single filter)
    Simple,
    /// Moderate query (multiple filters)
    Moderate,
    /// Complex query (many filters with time ranges)
    Complex,
}

/// Optimization suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// Suggestion type
    pub suggestion_type: SuggestionType,
    /// Description
    pub description: String,
    /// Expected improvement
    pub expected_improvement: String,
}

/// Types of optimization suggestions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SuggestionType {
    /// Add an index
    AddIndex,
    /// Narrow time range
    NarrowTimeRange,
    /// Use more specific filters
    UseSpecificFilters,
    /// Reduce result set
    ReduceResultSet,
    /// Consider caching
    ConsiderCaching,
}

/// Query plan generator.
pub struct QueryPlanner {
    /// Total records in the audit trail
    total_records: usize,
}

impl QueryPlanner {
    /// Creates a new query planner.
    pub fn new(total_records: usize) -> Self {
        Self { total_records }
    }

    /// Explains a query and generates an execution plan.
    pub fn explain(
        &self,
        query_description: &str,
        has_indexed_filter: bool,
        has_time_range: bool,
        limit: Option<usize>,
    ) -> QueryPlan {
        let steps = self.generate_execution_steps(has_indexed_filter, has_time_range, limit);
        let estimated_cost = self.estimate_cost(&steps);
        let suggestions =
            self.generate_suggestions(has_indexed_filter, has_time_range, limit, &estimated_cost);

        QueryPlan {
            query_description: query_description.to_string(),
            steps,
            estimated_cost,
            suggestions,
        }
    }

    /// Generates execution steps for a query.
    fn generate_execution_steps(
        &self,
        has_indexed_filter: bool,
        has_time_range: bool,
        limit: Option<usize>,
    ) -> Vec<ExecutionStep> {
        let mut steps = Vec::new();
        let mut step_number = 1;
        let mut current_rows = self.total_records;

        // Step 1: Full table scan or index lookup
        if has_indexed_filter {
            let selectivity = 0.1; // Assume 10% selectivity for indexed fields
            steps.push(ExecutionStep {
                step_number,
                operation: "Index lookup".to_string(),
                estimated_rows: current_rows,
                uses_index: true,
                selectivity,
            });
            current_rows = (current_rows as f64 * selectivity) as usize;
            step_number += 1;
        } else {
            steps.push(ExecutionStep {
                step_number,
                operation: "Full table scan".to_string(),
                estimated_rows: current_rows,
                uses_index: false,
                selectivity: 1.0,
            });
            step_number += 1;
        }

        // Step 2: Time range filter
        if has_time_range {
            let selectivity = 0.5; // Assume 50% for time range
            steps.push(ExecutionStep {
                step_number,
                operation: "Time range filter".to_string(),
                estimated_rows: current_rows,
                uses_index: false,
                selectivity,
            });
            current_rows = (current_rows as f64 * selectivity) as usize;
            step_number += 1;
        }

        // Step 3: Limit/offset
        if let Some(limit_val) = limit {
            steps.push(ExecutionStep {
                step_number,
                operation: format!("Limit to {} records", limit_val),
                estimated_rows: current_rows,
                uses_index: false,
                selectivity: (limit_val.min(current_rows) as f64) / (current_rows as f64).max(1.0),
            });
        }

        steps
    }

    /// Estimates the cost of a query.
    fn estimate_cost(&self, steps: &[ExecutionStep]) -> QueryCost {
        let total_scan_cost: usize = steps.iter().map(|s| s.estimated_rows).sum();
        let result_size = steps
            .last()
            .map(|s| (s.estimated_rows as f64 * s.selectivity) as usize)
            .unwrap_or(self.total_records);

        let complexity = if steps.len() <= 2 {
            QueryComplexity::Simple
        } else if steps.len() <= 4 {
            QueryComplexity::Moderate
        } else {
            QueryComplexity::Complex
        };

        QueryCost {
            total_scan_cost,
            result_size,
            complexity,
        }
    }

    /// Generates optimization suggestions.
    fn generate_suggestions(
        &self,
        has_indexed_filter: bool,
        has_time_range: bool,
        limit: Option<usize>,
        cost: &QueryCost,
    ) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // Suggest adding filters if result set is large
        if cost.result_size > self.total_records / 2 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: SuggestionType::UseSpecificFilters,
                description: "Add more specific filters to reduce result set size".to_string(),
                expected_improvement: "50-90% reduction in scanned records".to_string(),
            });
        }

        // Suggest time range if none specified
        if !has_time_range && self.total_records > 1000 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: SuggestionType::NarrowTimeRange,
                description: "Add a time range filter to limit the search scope".to_string(),
                expected_improvement: "40-80% reduction in scanned records".to_string(),
            });
        }

        // Suggest using indexed fields
        if !has_indexed_filter && self.total_records > 100 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: SuggestionType::AddIndex,
                description: "Use statute_id or subject_id filters for indexed lookup".to_string(),
                expected_improvement: "90% reduction in scan cost".to_string(),
            });
        }

        // Suggest limit if none specified
        if limit.is_none() && cost.result_size > 100 {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: SuggestionType::ReduceResultSet,
                description: "Add a LIMIT clause to reduce memory usage".to_string(),
                expected_improvement: "Reduced memory footprint".to_string(),
            });
        }

        // Suggest caching for complex queries
        if cost.complexity == QueryComplexity::Complex {
            suggestions.push(OptimizationSuggestion {
                suggestion_type: SuggestionType::ConsiderCaching,
                description: "Consider caching results for this complex query".to_string(),
                expected_improvement: "Near-instant subsequent queries".to_string(),
            });
        }

        suggestions
    }
}

/// Query performance analyzer.
pub struct QueryAnalyzer {
    execution_history: Vec<QueryExecution>,
}

/// A recorded query execution.
#[derive(Debug, Clone)]
pub struct QueryExecution {
    pub query_description: String,
    pub execution_time_ms: u64,
    pub records_scanned: usize,
    pub records_returned: usize,
}

impl QueryAnalyzer {
    /// Creates a new query analyzer.
    pub fn new() -> Self {
        Self {
            execution_history: Vec::new(),
        }
    }

    /// Records a query execution.
    pub fn record_execution(&mut self, execution: QueryExecution) {
        self.execution_history.push(execution);
    }

    /// Generates performance statistics.
    pub fn performance_stats(&self) -> QueryPerformanceStats {
        if self.execution_history.is_empty() {
            return QueryPerformanceStats {
                total_queries: 0,
                avg_execution_time_ms: 0.0,
                avg_records_scanned: 0.0,
                avg_selectivity: 0.0,
                slowest_queries: Vec::new(),
            };
        }

        let total_queries = self.execution_history.len();
        let avg_execution_time_ms = self
            .execution_history
            .iter()
            .map(|e| e.execution_time_ms)
            .sum::<u64>() as f64
            / total_queries as f64;

        let avg_records_scanned = self
            .execution_history
            .iter()
            .map(|e| e.records_scanned)
            .sum::<usize>() as f64
            / total_queries as f64;

        let avg_selectivity = self
            .execution_history
            .iter()
            .map(|e| {
                if e.records_scanned > 0 {
                    e.records_returned as f64 / e.records_scanned as f64
                } else {
                    0.0
                }
            })
            .sum::<f64>()
            / total_queries as f64;

        let mut sorted = self.execution_history.clone();
        sorted.sort_by_key(|e| std::cmp::Reverse(e.execution_time_ms));
        let slowest_queries: Vec<String> = sorted
            .iter()
            .take(5)
            .map(|e| format!("{} ({}ms)", e.query_description, e.execution_time_ms))
            .collect();

        QueryPerformanceStats {
            total_queries,
            avg_execution_time_ms,
            avg_records_scanned,
            avg_selectivity,
            slowest_queries,
        }
    }
}

impl Default for QueryAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Query performance statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPerformanceStats {
    pub total_queries: usize,
    pub avg_execution_time_ms: f64,
    pub avg_records_scanned: f64,
    pub avg_selectivity: f64,
    pub slowest_queries: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_planner_creation() {
        let planner = QueryPlanner::new(1000);
        assert_eq!(planner.total_records, 1000);
    }

    #[test]
    fn test_simple_query_plan() {
        let planner = QueryPlanner::new(1000);
        let plan = planner.explain("SELECT * WHERE statute_id='1'", true, false, None);
        assert!(!plan.steps.is_empty());
        assert_eq!(plan.estimated_cost.complexity, QueryComplexity::Simple);
    }

    #[test]
    fn test_complex_query_plan() {
        let planner = QueryPlanner::new(1000);
        let plan = planner.explain("Complex query", true, true, Some(10));
        assert!(plan.steps.len() >= 2);
    }

    #[test]
    fn test_full_scan_query() {
        let planner = QueryPlanner::new(1000);
        let plan = planner.explain("SELECT *", false, false, None);
        assert!(
            plan.steps
                .iter()
                .any(|s| s.operation.contains("Full table"))
        );
    }

    #[test]
    fn test_optimization_suggestions() {
        let planner = QueryPlanner::new(1000);
        let plan = planner.explain("SELECT *", false, false, None);
        assert!(!plan.suggestions.is_empty());
        assert!(
            plan.suggestions
                .iter()
                .any(|s| s.suggestion_type == SuggestionType::AddIndex
                    || s.suggestion_type == SuggestionType::NarrowTimeRange)
        );
    }

    #[test]
    fn test_query_analyzer() {
        let mut analyzer = QueryAnalyzer::new();

        analyzer.record_execution(QueryExecution {
            query_description: "query1".to_string(),
            execution_time_ms: 100,
            records_scanned: 1000,
            records_returned: 50,
        });

        analyzer.record_execution(QueryExecution {
            query_description: "query2".to_string(),
            execution_time_ms: 200,
            records_scanned: 2000,
            records_returned: 100,
        });

        let stats = analyzer.performance_stats();
        assert_eq!(stats.total_queries, 2);
        assert_eq!(stats.avg_execution_time_ms, 150.0);
    }
}
