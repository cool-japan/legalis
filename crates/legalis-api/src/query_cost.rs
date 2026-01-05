//! Query cost analysis for GraphQL.
//!
//! This module provides cost analysis for GraphQL queries to prevent
//! expensive or abusive queries from overwhelming the server.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cost analysis result for a GraphQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCost {
    /// Total cost of the query
    pub total_cost: u64,
    /// Breakdown by field
    pub field_costs: HashMap<String, FieldCost>,
    /// Maximum depth reached
    pub max_depth: u32,
    /// Total number of fields requested
    pub field_count: u32,
    /// Whether the query exceeds the allowed cost
    pub exceeds_limit: bool,
    /// Cost limit that was applied
    pub cost_limit: u64,
}

/// Cost information for a specific field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldCost {
    /// Name of the field
    pub field_name: String,
    /// Base cost of the field
    pub base_cost: u64,
    /// Multiplier based on arguments (e.g., first: 100 adds multiplier)
    pub multiplier: f64,
    /// Total cost (base_cost * multiplier)
    pub total_cost: u64,
    /// Depth level of this field
    pub depth: u32,
    /// Child fields
    pub children: Vec<String>,
}

/// Configuration for query cost analysis.
#[derive(Debug, Clone)]
pub struct CostConfig {
    /// Maximum allowed total cost
    pub max_cost: u64,
    /// Cost per field (default)
    pub default_field_cost: u64,
    /// Cost per depth level
    pub depth_cost: u64,
    /// Maximum allowed depth
    pub max_depth: u32,
    /// Custom field costs
    pub field_costs: HashMap<String, u64>,
    /// Cost multiplier for list fields
    pub list_multiplier: f64,
    /// Cost multiplier for connection fields
    pub connection_multiplier: f64,
}

impl Default for CostConfig {
    fn default() -> Self {
        let mut field_costs = HashMap::new();

        // Expensive operations
        field_costs.insert("statutes".to_string(), 10);
        field_costs.insert("searchStatutes".to_string(), 50);
        field_costs.insert("statutesByJurisdiction".to_string(), 20);
        field_costs.insert("verifyStatutes".to_string(), 100);
        field_costs.insert("statutesConnection".to_string(), 15);

        // Moderate operations
        field_costs.insert("statute".to_string(), 5);
        field_costs.insert("statuteCount".to_string(), 3);

        // Cheap operations
        field_costs.insert("id".to_string(), 1);
        field_costs.insert("title".to_string(), 1);
        field_costs.insert("version".to_string(), 1);
        field_costs.insert("jurisdiction".to_string(), 1);

        Self {
            max_cost: 1000,
            default_field_cost: 2,
            depth_cost: 5,
            max_depth: 15,
            field_costs,
            list_multiplier: 10.0,
            connection_multiplier: 5.0,
        }
    }
}

/// Query cost analyzer.
pub struct CostAnalyzer {
    config: CostConfig,
}

impl CostAnalyzer {
    /// Create a new cost analyzer with default configuration.
    pub fn new() -> Self {
        Self {
            config: CostConfig::default(),
        }
    }

    /// Create a new cost analyzer with custom configuration.
    pub fn with_config(config: CostConfig) -> Self {
        Self { config }
    }

    /// Analyze a GraphQL query and calculate its cost.
    ///
    /// This is a simplified implementation that uses heuristics.
    /// For production use, integrate with async-graphql's complexity analyzer.
    pub fn analyze_query(&self, query: &str) -> QueryCost {
        let mut total_cost = 0u64;
        let mut field_costs = HashMap::new();
        let mut max_depth = 0u32;
        let mut field_count = 0u32;

        // Parse query into fields (simplified)
        let fields = self.extract_fields(query);

        for (field_name, depth) in fields {
            field_count += 1;
            max_depth = max_depth.max(depth);

            let base_cost = self
                .config
                .field_costs
                .get(&field_name)
                .copied()
                .unwrap_or(self.config.default_field_cost);

            // Calculate multiplier based on query characteristics
            let multiplier = self.calculate_multiplier(&field_name, query);

            let field_total_cost = (base_cost as f64 * multiplier) as u64;
            total_cost += field_total_cost;

            // Add depth cost
            total_cost += (depth as u64) * self.config.depth_cost;

            field_costs.insert(
                field_name.clone(),
                FieldCost {
                    field_name: field_name.clone(),
                    base_cost,
                    multiplier,
                    total_cost: field_total_cost,
                    depth,
                    children: vec![],
                },
            );
        }

        let exceeds_limit = total_cost > self.config.max_cost || max_depth > self.config.max_depth;

        QueryCost {
            total_cost,
            field_costs,
            max_depth,
            field_count,
            exceeds_limit,
            cost_limit: self.config.max_cost,
        }
    }

    /// Validate a query against cost limits.
    pub fn validate_query(&self, query: &str) -> Result<QueryCost, CostError> {
        let cost = self.analyze_query(query);

        if cost.total_cost > self.config.max_cost {
            return Err(CostError::ExceedsCostLimit {
                cost: cost.total_cost,
                limit: self.config.max_cost,
            });
        }

        if cost.max_depth > self.config.max_depth {
            return Err(CostError::ExceedsDepthLimit {
                depth: cost.max_depth,
                limit: self.config.max_depth,
            });
        }

        Ok(cost)
    }

    /// Get the configuration.
    pub fn config(&self) -> &CostConfig {
        &self.config
    }

    /// Extract fields from query (simplified heuristic).
    fn extract_fields(&self, query: &str) -> Vec<(String, u32)> {
        let mut fields = Vec::new();
        let mut depth = 0u32;

        // Remove all newlines and extra whitespace for easier parsing
        let cleaned = query.replace('\n', " ").replace('\r', "");
        let mut chars = cleaned.chars().peekable();
        let mut current_field = String::new();
        let mut in_args = false;

        while let Some(ch) = chars.next() {
            match ch {
                '{' => {
                    // Before opening brace, we might have a field name
                    if !current_field.is_empty() && !in_args {
                        let field = current_field.trim().to_string();
                        if !field.is_empty()
                            && field != "query"
                            && field != "mutation"
                            && field != "subscription"
                        {
                            fields.push((field, depth));
                        }
                        current_field.clear();
                    }
                    depth += 1;
                }
                '}' => {
                    // Add any pending field before closing
                    if !current_field.is_empty() {
                        let field = current_field.trim().to_string();
                        if !field.is_empty()
                            && field != "query"
                            && field != "mutation"
                            && field != "subscription"
                        {
                            fields.push((field, depth));
                        }
                        current_field.clear();
                    }
                    depth = depth.saturating_sub(1);
                }
                '(' => {
                    in_args = true;
                    // Field name before args
                    if !current_field.is_empty() {
                        let field = current_field.trim().to_string();
                        if !field.is_empty()
                            && field != "query"
                            && field != "mutation"
                            && field != "subscription"
                        {
                            fields.push((field, depth));
                        }
                        current_field.clear();
                    }
                }
                ')' => {
                    in_args = false;
                }
                ' ' | '\t' => {
                    if !current_field.is_empty() && !in_args && depth > 0 {
                        let field = current_field.trim().to_string();
                        if !field.is_empty()
                            && field != "query"
                            && field != "mutation"
                            && field != "subscription"
                        {
                            fields.push((field, depth));
                        }
                        current_field.clear();
                    }
                }
                _ => {
                    if !in_args && depth > 0 {
                        current_field.push(ch);
                    }
                }
            }
        }

        // Handle last field
        if !current_field.is_empty() && depth > 0 {
            let field = current_field.trim().to_string();
            if !field.is_empty()
                && field != "query"
                && field != "mutation"
                && field != "subscription"
            {
                fields.push((field, depth));
            }
        }

        fields
    }

    /// Calculate multiplier based on query characteristics.
    fn calculate_multiplier(&self, field_name: &str, query: &str) -> f64 {
        let mut multiplier = 1.0;

        // Check if it's a list or connection field
        if field_name.ends_with("s") || field_name.contains("Connection") {
            if field_name.contains("Connection") {
                multiplier *= self.config.connection_multiplier;
            } else {
                multiplier *= self.config.list_multiplier;
            }
        }

        // Check for pagination arguments that might reduce cost
        if query.contains("first:") {
            if let Some(first_str) = query.split("first:").nth(1) {
                if let Some(num_str) = first_str.split(|c: char| !c.is_ascii_digit()).next() {
                    if let Ok(num) = num_str.parse::<u32>() {
                        if num > 0 && num < 100 {
                            multiplier *= (num as f64) / 10.0;
                        }
                    }
                }
            }
        }

        multiplier
    }
}

impl Default for CostAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during cost analysis.
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum CostError {
    #[error("Query cost {cost} exceeds limit {limit}")]
    ExceedsCostLimit { cost: u64, limit: u64 },

    #[error("Query depth {depth} exceeds limit {limit}")]
    ExceedsDepthLimit { depth: u32, limit: u32 },

    #[error("Query is too complex: {reason}")]
    TooComplex { reason: String },
}

/// Cost reporting and analysis.
pub struct CostReporter {
    analyzer: CostAnalyzer,
}

impl CostReporter {
    /// Create a new cost reporter.
    pub fn new(analyzer: CostAnalyzer) -> Self {
        Self { analyzer }
    }

    /// Generate a detailed cost report for a query.
    pub fn generate_report(&self, query: &str) -> CostReport {
        let cost = self.analyzer.analyze_query(query);

        let most_expensive_field = cost
            .field_costs
            .values()
            .max_by_key(|f| f.total_cost)
            .cloned();

        let recommendations = self.generate_recommendations(&cost);

        CostReport {
            cost,
            most_expensive_field,
            recommendations,
        }
    }

    /// Generate optimization recommendations.
    fn generate_recommendations(&self, cost: &QueryCost) -> Vec<String> {
        let mut recommendations = Vec::new();

        if cost.max_depth > 10 {
            recommendations.push(
                "Consider reducing query depth by splitting into multiple queries".to_string(),
            );
        }

        if cost.field_count > 50 {
            recommendations
                .push("Consider using field selection to request only needed fields".to_string());
        }

        // Find expensive fields
        for (field_name, field_cost) in &cost.field_costs {
            if field_cost.total_cost > 100 {
                recommendations.push(format!(
                    "Field '{}' is expensive (cost: {}). Consider adding pagination or filters",
                    field_name, field_cost.total_cost
                ));
            }
        }

        recommendations
    }
}

/// Detailed cost report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostReport {
    /// Cost analysis
    pub cost: QueryCost,
    /// Most expensive field
    pub most_expensive_field: Option<FieldCost>,
    /// Optimization recommendations
    pub recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_config_default() {
        let config = CostConfig::default();
        assert_eq!(config.max_cost, 1000);
        assert_eq!(config.default_field_cost, 2);
        assert_eq!(config.depth_cost, 5);
        assert_eq!(config.max_depth, 15);
    }

    #[test]
    fn test_cost_analyzer_creation() {
        let analyzer = CostAnalyzer::new();
        assert_eq!(analyzer.config.max_cost, 1000);
    }

    #[test]
    fn test_analyze_simple_query() {
        let analyzer = CostAnalyzer::new();
        let query = "{ statuteCount }";
        let cost = analyzer.analyze_query(query);

        eprintln!("Query: {}", query);
        eprintln!("Total cost: {}", cost.total_cost);
        eprintln!("Field count: {}", cost.field_count);
        eprintln!("Fields: {:?}", cost.field_costs.keys().collect::<Vec<_>>());

        assert!(
            cost.total_cost > 0,
            "Cost should be greater than 0, got {}",
            cost.total_cost
        );
        assert!(!cost.exceeds_limit);
    }

    #[test]
    fn test_analyze_complex_query() {
        let analyzer = CostAnalyzer::new();
        let query = r#"
            {
                statutes {
                    id
                    title
                    version
                    jurisdiction
                }
            }
        "#;
        let cost = analyzer.analyze_query(query);

        assert!(cost.total_cost > 0);
        assert!(cost.field_count >= 5); // statutes, id, title, version, jurisdiction
        assert!(cost.max_depth > 0);
    }

    #[test]
    fn test_validate_query_within_limits() {
        let analyzer = CostAnalyzer::new();
        let query = "{ statuteCount }";
        let result = analyzer.validate_query(query);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_query_exceeds_limit() {
        let mut config = CostConfig::default();
        config.max_cost = 1; // Very low limit
        let analyzer = CostAnalyzer::with_config(config);

        let query = r#"
            {
                statutes {
                    id
                    title
                }
            }
        "#;
        let result = analyzer.validate_query(query);

        assert!(result.is_err());
        if let Err(CostError::ExceedsCostLimit { cost, limit }) = result {
            assert!(cost > limit);
        }
    }

    #[test]
    fn test_extract_fields() {
        let analyzer = CostAnalyzer::new();
        let query = r#"
            {
                statutes {
                    id
                    title
                }
            }
        "#;
        let fields = analyzer.extract_fields(query);

        assert!(!fields.is_empty());
        assert!(fields.iter().any(|(name, _)| name == "statutes"));
        assert!(fields.iter().any(|(name, _)| name == "id"));
        assert!(fields.iter().any(|(name, _)| name == "title"));
    }

    #[test]
    fn test_calculate_multiplier_simple() {
        let analyzer = CostAnalyzer::new();
        let multiplier = analyzer.calculate_multiplier("statute", "{ statute { id } }");
        assert!(multiplier >= 1.0);
    }

    #[test]
    fn test_calculate_multiplier_list() {
        let analyzer = CostAnalyzer::new();
        let multiplier = analyzer.calculate_multiplier("statutes", "{ statutes { id } }");
        assert!(multiplier > 1.0);
    }

    #[test]
    fn test_calculate_multiplier_with_pagination() {
        let analyzer = CostAnalyzer::new();
        let query = "{ statutes(first: 10) { id } }";
        let multiplier = analyzer.calculate_multiplier("statutes", query);
        assert!(multiplier > 0.0);
    }

    #[test]
    fn test_cost_reporter_generation() {
        let analyzer = CostAnalyzer::new();
        let reporter = CostReporter::new(analyzer);
        let query = r#"
            {
                statutes {
                    id
                    title
                }
            }
        "#;
        let report = reporter.generate_report(query);

        assert!(report.cost.total_cost > 0);
        assert!(report.most_expensive_field.is_some());
    }

    #[test]
    fn test_cost_reporter_recommendations() {
        let analyzer = CostAnalyzer::new();
        let reporter = CostReporter::new(analyzer);

        // Create a query with many fields
        let mut query = String::from("{ statutes {");
        for i in 0..60 {
            query.push_str(&format!(" field{}", i));
        }
        query.push_str(" } }");

        let report = reporter.generate_report(&query);
        eprintln!("Query: {}", query);
        eprintln!("Field count: {}", report.cost.field_count);
        eprintln!("Max depth: {}", report.cost.max_depth);
        eprintln!("Recommendations: {:?}", report.recommendations);
        assert!(
            !report.recommendations.is_empty(),
            "Expected recommendations for query with {} fields",
            report.cost.field_count
        );
    }

    #[test]
    fn test_field_cost_serialization() {
        let field_cost = FieldCost {
            field_name: "test".to_string(),
            base_cost: 10,
            multiplier: 2.0,
            total_cost: 20,
            depth: 1,
            children: vec!["child1".to_string(), "child2".to_string()],
        };

        let json = serde_json::to_string(&field_cost).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("child1"));
    }

    #[test]
    fn test_custom_field_costs() {
        let mut config = CostConfig::default();
        config
            .field_costs
            .insert("expensive_field".to_string(), 500);
        let analyzer = CostAnalyzer::with_config(config);

        let query = "{ expensive_field }";
        let cost = analyzer.analyze_query(query);

        assert!(cost.total_cost >= 500);
    }
}
