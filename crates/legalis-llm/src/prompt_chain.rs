//! Advanced prompt chaining for complex multi-step workflows.
//!
//! This module provides a sophisticated prompt chaining system that allows
//! building complex workflows with dependencies, conditional execution,
//! and result aggregation.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::LLMProvider;

/// A node in a prompt chain representing a single step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainNode {
    /// Unique identifier for this node
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Prompt template (supports variable substitution with {{var}})
    pub prompt_template: String,
    /// Dependencies (node IDs that must complete first)
    pub dependencies: Vec<String>,
    /// Whether this node is conditional
    pub conditional: Option<ConditionalExecution>,
    /// Result processing
    pub result_processor: Option<ResultProcessor>,
}

impl ChainNode {
    /// Creates a new chain node.
    pub fn new(id: impl Into<String>, name: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            prompt_template: prompt.into(),
            dependencies: Vec::new(),
            conditional: None,
            result_processor: None,
        }
    }

    /// Adds a dependency.
    pub fn depends_on(mut self, node_id: impl Into<String>) -> Self {
        self.dependencies.push(node_id.into());
        self
    }

    /// Sets conditional execution.
    pub fn when(mut self, condition: ConditionalExecution) -> Self {
        self.conditional = Some(condition);
        self
    }

    /// Sets result processor.
    pub fn process_with(mut self, processor: ResultProcessor) -> Self {
        self.result_processor = Some(processor);
        self
    }

    /// Renders the prompt with variable substitution.
    pub fn render_prompt(&self, variables: &HashMap<String, String>) -> String {
        let mut result = self.prompt_template.clone();

        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }
}

/// Conditional execution rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalExecution {
    /// Variable to check
    pub variable: String,
    /// Condition type
    pub condition: Condition,
}

/// Types of conditions for conditional execution.
#[derive(Clone, Serialize, Deserialize)]
pub enum Condition {
    /// Execute if variable equals value
    Equals(String),
    /// Execute if variable contains substring
    Contains(String),
    /// Execute if variable matches regex
    Matches(String),
    /// Execute if variable is not empty
    NotEmpty,
    /// Execute if variable is empty
    IsEmpty,
    /// Custom predicate (not serializable, runtime only)
    #[serde(skip)]
    Custom(Arc<dyn Fn(&str) -> bool + Send + Sync>),
}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::Equals(s) => f.debug_tuple("Equals").field(s).finish(),
            Condition::Contains(s) => f.debug_tuple("Contains").field(s).finish(),
            Condition::Matches(s) => f.debug_tuple("Matches").field(s).finish(),
            Condition::NotEmpty => write!(f, "NotEmpty"),
            Condition::IsEmpty => write!(f, "IsEmpty"),
            Condition::Custom(_) => write!(f, "Custom(<function>)"),
        }
    }
}

impl Condition {
    /// Evaluates the condition against a value.
    pub fn evaluate(&self, value: &str) -> bool {
        match self {
            Condition::Equals(expected) => value == expected,
            Condition::Contains(substring) => value.contains(substring),
            Condition::Matches(pattern) => regex::Regex::new(pattern)
                .map(|re| re.is_match(value))
                .unwrap_or(false),
            Condition::NotEmpty => !value.is_empty(),
            Condition::IsEmpty => value.is_empty(),
            Condition::Custom(predicate) => predicate(value),
        }
    }
}

/// Result processing options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultProcessor {
    /// Extract JSON field
    ExtractJson { field: String },
    /// Apply regex extraction
    ExtractRegex { pattern: String },
    /// Trim whitespace
    Trim,
    /// Convert to uppercase
    ToUpperCase,
    /// Convert to lowercase
    ToLowerCase,
    /// Take first N characters
    TakeFirst { count: usize },
    /// Take last N characters
    TakeLast { count: usize },
    /// Split by delimiter and take index
    Split { delimiter: String, index: usize },
}

impl ResultProcessor {
    /// Processes a result string.
    pub fn process(&self, input: &str) -> Result<String> {
        match self {
            ResultProcessor::ExtractJson { field } => {
                let json: serde_json::Value =
                    serde_json::from_str(input).context("Failed to parse JSON")?;

                json.get(field)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .context("Field not found or not a string")
            }
            ResultProcessor::ExtractRegex { pattern } => {
                let re = regex::Regex::new(pattern).context("Invalid regex pattern")?;

                re.captures(input)
                    .and_then(|cap| cap.get(1))
                    .map(|m| m.as_str().to_string())
                    .context("No match found")
            }
            ResultProcessor::Trim => Ok(input.trim().to_string()),
            ResultProcessor::ToUpperCase => Ok(input.to_uppercase()),
            ResultProcessor::ToLowerCase => Ok(input.to_lowercase()),
            ResultProcessor::TakeFirst { count } => Ok(input.chars().take(*count).collect()),
            ResultProcessor::TakeLast { count } => {
                let chars: Vec<char> = input.chars().collect();
                let start = chars.len().saturating_sub(*count);
                Ok(chars[start..].iter().collect())
            }
            ResultProcessor::Split { delimiter, index } => input
                .split(delimiter)
                .nth(*index)
                .map(|s| s.to_string())
                .context("Index out of bounds"),
        }
    }
}

/// A complete prompt chain workflow.
#[derive(Debug, Clone)]
pub struct PromptChain {
    /// All nodes in the chain
    nodes: Vec<ChainNode>,
    /// Initial variables
    initial_variables: HashMap<String, String>,
}

impl PromptChain {
    /// Creates a new prompt chain.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            initial_variables: HashMap::new(),
        }
    }

    /// Adds a node to the chain.
    pub fn add_node(mut self, node: ChainNode) -> Self {
        self.nodes.push(node);
        self
    }

    /// Sets an initial variable.
    pub fn with_variable(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.initial_variables.insert(key.into(), value.into());
        self
    }

    /// Executes the entire chain.
    pub async fn execute<P: LLMProvider>(&self, provider: &P) -> Result<ChainExecutionResult> {
        let mut results = HashMap::new();
        let mut variables = self.initial_variables.clone();
        let mut execution_order = Vec::new();

        // Topological sort for dependency resolution
        let sorted_nodes = self.topological_sort()?;

        for node in sorted_nodes {
            // Check if node should execute based on conditions
            if let Some(ref conditional) = node.conditional {
                if let Some(var_value) = variables.get(&conditional.variable) {
                    if !conditional.condition.evaluate(var_value) {
                        continue; // Skip this node
                    }
                } else {
                    continue; // Variable not found, skip
                }
            }

            // Render prompt with current variables
            let prompt = node.render_prompt(&variables);

            // Execute the prompt
            let result = provider
                .generate_text(&prompt)
                .await
                .with_context(|| format!("Failed to execute node: {}", node.id))?;

            // Process result if processor is specified
            let processed_result = if let Some(ref processor) = node.result_processor {
                processor.process(&result)?
            } else {
                result
            };

            // Store result
            results.insert(node.id.clone(), processed_result.clone());
            variables.insert(node.id.clone(), processed_result);
            execution_order.push(node.id.clone());
        }

        Ok(ChainExecutionResult {
            results,
            execution_order,
        })
    }

    /// Performs topological sort to determine execution order.
    fn topological_sort(&self) -> Result<Vec<ChainNode>> {
        let mut sorted = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp_visited = std::collections::HashSet::new();

        fn visit(
            node_id: &str,
            nodes: &[ChainNode],
            visited: &mut std::collections::HashSet<String>,
            temp_visited: &mut std::collections::HashSet<String>,
            sorted: &mut Vec<ChainNode>,
        ) -> Result<()> {
            if visited.contains(node_id) {
                return Ok(());
            }

            if temp_visited.contains(node_id) {
                anyhow::bail!("Circular dependency detected");
            }

            temp_visited.insert(node_id.to_string());

            let node = nodes
                .iter()
                .find(|n| n.id == node_id)
                .context("Node not found")?;

            for dep in &node.dependencies {
                visit(dep, nodes, visited, temp_visited, sorted)?;
            }

            temp_visited.remove(node_id);
            visited.insert(node_id.to_string());
            sorted.push(node.clone());

            Ok(())
        }

        for node in &self.nodes {
            if !visited.contains(&node.id) {
                visit(
                    &node.id,
                    &self.nodes,
                    &mut visited,
                    &mut temp_visited,
                    &mut sorted,
                )?;
            }
        }

        Ok(sorted)
    }
}

impl Default for PromptChain {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of executing a prompt chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainExecutionResult {
    /// Results by node ID
    pub results: HashMap<String, String>,
    /// Order in which nodes were executed
    pub execution_order: Vec<String>,
}

impl ChainExecutionResult {
    /// Gets the result of a specific node.
    pub fn get(&self, node_id: &str) -> Option<&String> {
        self.results.get(node_id)
    }

    /// Gets the final result (last executed node).
    pub fn final_result(&self) -> Option<&String> {
        self.execution_order
            .last()
            .and_then(|id| self.results.get(id))
    }
}

/// Builder for creating legal analysis chains.
pub struct LegalAnalysisChainBuilder {
    chain: PromptChain,
}

impl LegalAnalysisChainBuilder {
    /// Creates a new legal analysis chain builder.
    pub fn new() -> Self {
        Self {
            chain: PromptChain::new(),
        }
    }

    /// Adds a contract review step.
    pub fn contract_review(mut self) -> Self {
        self.chain = self.chain.add_node(ChainNode::new(
            "contract_review",
            "Contract Review",
            "Review the following contract and identify key clauses:\n\n{{contract_text}}",
        ));
        self
    }

    /// Adds a risk assessment step.
    pub fn risk_assessment(mut self) -> Self {
        self.chain = self.chain.add_node(
            ChainNode::new(
                "risk_assessment",
                "Risk Assessment",
                "Based on the contract review:\n\n{{contract_review}}\n\nIdentify potential legal risks."
            ).depends_on("contract_review")
        );
        self
    }

    /// Adds a compliance check step.
    pub fn compliance_check(mut self, jurisdiction: impl Into<String>) -> Self {
        let jurisdiction = jurisdiction.into();
        self.chain = self.chain.add_node(
            ChainNode::new(
                "compliance_check",
                "Compliance Check",
                format!(
                    "Review this contract for compliance with {} law:\n\n{{{{contract_review}}}}",
                    jurisdiction
                ),
            )
            .depends_on("contract_review"),
        );
        self
    }

    /// Adds a recommendation step.
    pub fn recommendations(mut self) -> Self {
        self.chain = self.chain.add_node(
            ChainNode::new(
                "recommendations",
                "Recommendations",
                "Based on the risk assessment:\n\n{{risk_assessment}}\n\nAnd compliance check:\n\n{{compliance_check}}\n\nProvide specific recommendations."
            )
            .depends_on("risk_assessment")
            .depends_on("compliance_check")
        );
        self
    }

    /// Sets the contract text.
    pub fn with_contract(mut self, contract_text: impl Into<String>) -> Self {
        self.chain = self.chain.with_variable("contract_text", contract_text);
        self
    }

    /// Builds the chain.
    pub fn build(self) -> PromptChain {
        self.chain
    }
}

impl Default for LegalAnalysisChainBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_node_creation() {
        let node = ChainNode::new("step1", "First Step", "Analyze: {{input}}").depends_on("step0");

        assert_eq!(node.id, "step1");
        assert_eq!(node.name, "First Step");
        assert_eq!(node.dependencies.len(), 1);
    }

    #[test]
    fn test_prompt_rendering() {
        let node = ChainNode::new("test", "Test", "Hello {{name}}, you are {{age}} years old");

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), "Alice".to_string());
        variables.insert("age".to_string(), "30".to_string());

        let rendered = node.render_prompt(&variables);
        assert_eq!(rendered, "Hello Alice, you are 30 years old");
    }

    #[test]
    fn test_condition_equals() {
        let condition = Condition::Equals("yes".to_string());
        assert!(condition.evaluate("yes"));
        assert!(!condition.evaluate("no"));
    }

    #[test]
    fn test_condition_contains() {
        let condition = Condition::Contains("error".to_string());
        assert!(condition.evaluate("This is an error message"));
        assert!(!condition.evaluate("All is well"));
    }

    #[test]
    fn test_condition_not_empty() {
        let condition = Condition::NotEmpty;
        assert!(condition.evaluate("some text"));
        assert!(!condition.evaluate(""));
    }

    #[test]
    fn test_result_processor_trim() {
        let processor = ResultProcessor::Trim;
        let result = processor.process("  hello world  ").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_result_processor_uppercase() {
        let processor = ResultProcessor::ToUpperCase;
        let result = processor.process("hello").unwrap();
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_result_processor_take_first() {
        let processor = ResultProcessor::TakeFirst { count: 5 };
        let result = processor.process("hello world").unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_result_processor_split() {
        let processor = ResultProcessor::Split {
            delimiter: ",".to_string(),
            index: 1,
        };
        let result = processor.process("apple,banana,cherry").unwrap();
        assert_eq!(result, "banana");
    }

    #[test]
    fn test_topological_sort() {
        let chain = PromptChain::new()
            .add_node(ChainNode::new("a", "A", "{{input}}"))
            .add_node(ChainNode::new("b", "B", "{{a}}").depends_on("a"))
            .add_node(ChainNode::new("c", "C", "{{b}}").depends_on("b"));

        let sorted = chain.topological_sort().unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0].id, "a");
        assert_eq!(sorted[1].id, "b");
        assert_eq!(sorted[2].id, "c");
    }

    #[test]
    fn test_circular_dependency_detection() {
        let chain = PromptChain::new()
            .add_node(ChainNode::new("a", "A", "{{b}}").depends_on("b"))
            .add_node(ChainNode::new("b", "B", "{{a}}").depends_on("a"));

        let result = chain.topological_sort();
        assert!(result.is_err());
    }

    #[test]
    fn test_legal_analysis_chain_builder() {
        let chain = LegalAnalysisChainBuilder::new()
            .contract_review()
            .risk_assessment()
            .compliance_check("California")
            .recommendations()
            .with_contract("Sample contract text")
            .build();

        assert_eq!(chain.nodes.len(), 4);
        assert!(chain.initial_variables.contains_key("contract_text"));
    }
}
