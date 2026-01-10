//! Workflow automation system for Legalis CLI.
//!
//! This module provides:
//! - Workflow definition files (YAML)
//! - Task pipelines with dependencies
//! - Conditional execution
//! - Parallel task execution
//! - Workflow templates library

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// A workflow definition containing tasks and execution rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Workflow name
    pub name: String,
    /// Workflow description
    pub description: Option<String>,
    /// Workflow version
    pub version: String,
    /// Global workflow variables
    pub variables: HashMap<String, String>,
    /// List of tasks to execute
    pub tasks: Vec<Task>,
    /// Execution mode (sequential or parallel)
    #[serde(default = "default_execution_mode")]
    pub execution_mode: ExecutionMode,
}

fn default_execution_mode() -> ExecutionMode {
    ExecutionMode::Sequential
}

/// Execution mode for workflows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    /// Execute tasks sequentially in order
    Sequential,
    /// Execute tasks in parallel where possible
    Parallel,
}

/// A single task in a workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Task ID (unique within workflow)
    pub id: String,
    /// Task name
    pub name: String,
    /// Task description
    pub description: Option<String>,
    /// Command to execute (Legalis CLI command)
    pub command: String,
    /// Task arguments
    pub args: Vec<String>,
    /// Tasks this task depends on (must complete before this runs)
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// Condition for executing this task
    pub condition: Option<Condition>,
    /// Continue workflow on task failure
    #[serde(default)]
    pub continue_on_error: bool,
    /// Task timeout in seconds
    pub timeout: Option<u64>,
    /// Task retry policy
    pub retry: Option<RetryPolicy>,
    /// Environment variables for this task
    #[serde(default)]
    pub env: HashMap<String, String>,
}

/// Condition for conditional task execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Condition {
    /// Always execute (default)
    Always,
    /// Execute only if variable equals value
    VarEquals { var: String, value: String },
    /// Execute only if variable does not equal value
    VarNotEquals { var: String, value: String },
    /// Execute only if file exists
    FileExists { path: String },
    /// Execute only if file does not exist
    FileNotExists { path: String },
    /// Execute only if previous task succeeded
    PrevSuccess { task_id: String },
    /// Execute only if previous task failed
    PrevFailed { task_id: String },
    /// Execute if any of the conditions are true
    Or { conditions: Vec<Condition> },
    /// Execute if all of the conditions are true
    And { conditions: Vec<Condition> },
    /// Execute if the condition is false
    Not { condition: Box<Condition> },
}

/// Retry policy for tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Delay between retries in seconds
    pub delay_seconds: u64,
    /// Exponential backoff multiplier
    #[serde(default = "default_backoff_multiplier")]
    pub backoff_multiplier: f64,
}

fn default_backoff_multiplier() -> f64 {
    1.0
}

/// Result of task execution.
#[derive(Debug, Clone)]
pub struct TaskResult {
    /// Task ID
    pub task_id: String,
    /// Whether the task succeeded
    pub success: bool,
    /// Task output
    pub output: String,
    /// Task error message (if failed)
    pub error: Option<String>,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Number of retry attempts
    pub retry_count: u32,
}

/// Result of workflow execution.
#[derive(Debug, Clone)]
pub struct WorkflowResult {
    /// Workflow name
    pub workflow_name: String,
    /// Whether the workflow succeeded
    pub success: bool,
    /// Individual task results
    pub task_results: Vec<TaskResult>,
    /// Total execution duration in milliseconds
    pub total_duration_ms: u64,
    /// Workflow error message (if failed)
    pub error: Option<String>,
}

/// Workflow executor.
pub struct WorkflowExecutor {
    /// Workflow to execute
    workflow: Workflow,
    /// Task results collected during execution
    task_results: HashMap<String, TaskResult>,
    /// Global variables (workflow variables + runtime variables)
    variables: HashMap<String, String>,
}

impl WorkflowExecutor {
    /// Create a new workflow executor.
    pub fn new(workflow: Workflow) -> Self {
        let variables = workflow.variables.clone();
        Self {
            workflow,
            task_results: HashMap::new(),
            variables,
        }
    }

    /// Execute the workflow.
    pub fn execute(&mut self) -> WorkflowResult {
        let start_time = std::time::Instant::now();
        let workflow_name = self.workflow.name.clone();

        let success = match self.workflow.execution_mode {
            ExecutionMode::Sequential => self.execute_sequential(),
            ExecutionMode::Parallel => self.execute_parallel(),
        };

        let total_duration_ms = start_time.elapsed().as_millis() as u64;

        WorkflowResult {
            workflow_name,
            success,
            task_results: self.task_results.values().cloned().collect(),
            total_duration_ms,
            error: if success {
                None
            } else {
                Some("Workflow execution failed".to_string())
            },
        }
    }

    /// Execute tasks sequentially.
    fn execute_sequential(&mut self) -> bool {
        for task in self.workflow.tasks.clone() {
            if !self.should_execute_task(&task) {
                continue;
            }

            let result = self.execute_task(&task);
            self.task_results.insert(task.id.clone(), result.clone());

            if !result.success && !task.continue_on_error {
                return false;
            }
        }
        true
    }

    /// Execute tasks in parallel where possible.
    fn execute_parallel(&mut self) -> bool {
        use std::collections::HashSet;

        let mut completed: HashSet<String> = HashSet::new();
        let mut pending: Vec<Task> = self.workflow.tasks.clone();

        while !pending.is_empty() {
            // Find tasks that can be executed (all dependencies completed)
            let ready_tasks: Vec<Task> = pending
                .iter()
                .filter(|task| {
                    task.depends_on.iter().all(|dep| completed.contains(dep))
                        && self.should_execute_task(task)
                })
                .cloned()
                .collect();

            if ready_tasks.is_empty() && !pending.is_empty() {
                // Circular dependency or unmet conditions
                return false;
            }

            // Execute ready tasks in parallel (simulated here with sequential execution)
            // In a real implementation, this would use threads/async
            for task in ready_tasks {
                let result = self.execute_task(&task);
                self.task_results.insert(task.id.clone(), result.clone());

                if result.success {
                    completed.insert(task.id.clone());
                } else if !task.continue_on_error {
                    return false;
                }

                // Remove from pending
                pending.retain(|t| t.id != task.id);
            }
        }

        true
    }

    /// Check if a task should be executed based on its condition.
    fn should_execute_task(&self, task: &Task) -> bool {
        match &task.condition {
            None => true,
            Some(condition) => self.evaluate_condition(condition),
        }
    }

    /// Evaluate a condition.
    fn evaluate_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Always => true,
            Condition::VarEquals { var, value } => self.variables.get(var) == Some(value),
            Condition::VarNotEquals { var, value } => self.variables.get(var) != Some(value),
            Condition::FileExists { path } => Path::new(path).exists(),
            Condition::FileNotExists { path } => !Path::new(path).exists(),
            Condition::PrevSuccess { task_id } => {
                self.task_results.get(task_id).is_some_and(|r| r.success)
            }
            Condition::PrevFailed { task_id } => {
                self.task_results.get(task_id).is_some_and(|r| !r.success)
            }
            Condition::Or { conditions } => conditions.iter().any(|c| self.evaluate_condition(c)),
            Condition::And { conditions } => conditions.iter().all(|c| self.evaluate_condition(c)),
            Condition::Not { condition } => !self.evaluate_condition(condition),
        }
    }

    /// Execute a single task with retry logic.
    fn execute_task(&self, task: &Task) -> TaskResult {
        let start_time = std::time::Instant::now();
        let mut retry_count = 0;
        let max_attempts = task.retry.as_ref().map_or(1, |r| r.max_attempts + 1);

        loop {
            let result = self.execute_task_once(task);

            if result.success || retry_count >= max_attempts - 1 {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                return TaskResult {
                    task_id: task.id.clone(),
                    success: result.success,
                    output: result.output,
                    error: result.error,
                    duration_ms,
                    retry_count,
                };
            }

            retry_count += 1;

            // Apply retry delay with exponential backoff
            if let Some(retry) = &task.retry {
                let delay = retry.delay_seconds as f64
                    * retry.backoff_multiplier.powi(retry_count as i32 - 1);
                std::thread::sleep(std::time::Duration::from_secs_f64(delay));
            }
        }
    }

    /// Execute a task once (no retry).
    #[allow(dead_code)]
    fn execute_task_once(&self, task: &Task) -> TaskResult {
        // In a real implementation, this would execute the actual command
        // For now, we simulate task execution
        TaskResult {
            task_id: task.id.clone(),
            success: true,
            output: format!("Executed: {} {:?}", task.command, task.args),
            error: None,
            duration_ms: 100,
            retry_count: 0,
        }
    }
}

/// Workflow template library.
pub struct WorkflowTemplates;

impl WorkflowTemplates {
    /// Get all available workflow templates.
    pub fn list_templates() -> Vec<WorkflowTemplate> {
        vec![
            WorkflowTemplate {
                name: "verify-and-test".to_string(),
                description: "Verify statutes and run tests".to_string(),
                category: TemplateCategory::Testing,
            },
            WorkflowTemplate {
                name: "format-lint-verify".to_string(),
                description: "Format, lint, and verify statutes".to_string(),
                category: TemplateCategory::Quality,
            },
            WorkflowTemplate {
                name: "build-and-export".to_string(),
                description: "Build and export statutes to multiple formats".to_string(),
                category: TemplateCategory::Build,
            },
            WorkflowTemplate {
                name: "ci-pipeline".to_string(),
                description: "Complete CI pipeline (format, lint, verify, test, build)".to_string(),
                category: TemplateCategory::CI,
            },
            WorkflowTemplate {
                name: "batch-processing".to_string(),
                description: "Batch process multiple statute files".to_string(),
                category: TemplateCategory::Batch,
            },
        ]
    }

    /// Generate a workflow from a template.
    pub fn generate_from_template(template_name: &str) -> Option<Workflow> {
        match template_name {
            "verify-and-test" => Some(Self::verify_and_test_template()),
            "format-lint-verify" => Some(Self::format_lint_verify_template()),
            "build-and-export" => Some(Self::build_and_export_template()),
            "ci-pipeline" => Some(Self::ci_pipeline_template()),
            "batch-processing" => Some(Self::batch_processing_template()),
            _ => None,
        }
    }

    fn verify_and_test_template() -> Workflow {
        Workflow {
            name: "verify-and-test".to_string(),
            description: Some("Verify statutes and run tests".to_string()),
            version: "1.0.0".to_string(),
            variables: HashMap::from([
                ("INPUT_DIR".to_string(), "./statutes".to_string()),
                ("TEST_FILE".to_string(), "./tests.yaml".to_string()),
            ]),
            tasks: vec![
                Task {
                    id: "verify".to_string(),
                    name: "Verify Statutes".to_string(),
                    description: Some("Verify all statute files".to_string()),
                    command: "verify".to_string(),
                    args: vec!["--input".to_string(), "${INPUT_DIR}/*.ldsl".to_string()],
                    depends_on: vec![],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "test".to_string(),
                    name: "Run Tests".to_string(),
                    description: Some("Run test suite".to_string()),
                    command: "test".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--tests".to_string(),
                        "${TEST_FILE}".to_string(),
                    ],
                    depends_on: vec!["verify".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(600),
                    retry: None,
                    env: HashMap::new(),
                },
            ],
            execution_mode: ExecutionMode::Sequential,
        }
    }

    fn format_lint_verify_template() -> Workflow {
        Workflow {
            name: "format-lint-verify".to_string(),
            description: Some("Format, lint, and verify statutes".to_string()),
            version: "1.0.0".to_string(),
            variables: HashMap::from([("INPUT_DIR".to_string(), "./statutes".to_string())]),
            tasks: vec![
                Task {
                    id: "format".to_string(),
                    name: "Format Files".to_string(),
                    description: Some("Format all statute files".to_string()),
                    command: "format".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--inplace".to_string(),
                    ],
                    depends_on: vec![],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "lint".to_string(),
                    name: "Lint Files".to_string(),
                    description: Some("Lint all statute files".to_string()),
                    command: "lint".to_string(),
                    args: vec!["--input".to_string(), "${INPUT_DIR}/*.ldsl".to_string()],
                    depends_on: vec!["format".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "verify".to_string(),
                    name: "Verify Files".to_string(),
                    description: Some("Verify all statute files".to_string()),
                    command: "verify".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--strict".to_string(),
                    ],
                    depends_on: vec!["lint".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
            ],
            execution_mode: ExecutionMode::Sequential,
        }
    }

    fn build_and_export_template() -> Workflow {
        Workflow {
            name: "build-and-export".to_string(),
            description: Some("Build and export statutes to multiple formats".to_string()),
            version: "1.0.0".to_string(),
            variables: HashMap::from([
                ("INPUT_FILE".to_string(), "./statute.ldsl".to_string()),
                ("OUTPUT_DIR".to_string(), "./build".to_string()),
            ]),
            tasks: vec![
                Task {
                    id: "verify".to_string(),
                    name: "Verify Statute".to_string(),
                    description: Some("Verify statute before export".to_string()),
                    command: "verify".to_string(),
                    args: vec!["--input".to_string(), "${INPUT_FILE}".to_string()],
                    depends_on: vec![],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "export-json".to_string(),
                    name: "Export to JSON".to_string(),
                    description: Some("Export statute to JSON".to_string()),
                    command: "export".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_FILE}".to_string(),
                        "--output".to_string(),
                        "${OUTPUT_DIR}/statute.json".to_string(),
                        "--export-format".to_string(),
                        "json".to_string(),
                    ],
                    depends_on: vec!["verify".to_string()],
                    condition: None,
                    continue_on_error: true,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "export-yaml".to_string(),
                    name: "Export to YAML".to_string(),
                    description: Some("Export statute to YAML".to_string()),
                    command: "export".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_FILE}".to_string(),
                        "--output".to_string(),
                        "${OUTPUT_DIR}/statute.yaml".to_string(),
                        "--export-format".to_string(),
                        "yaml".to_string(),
                    ],
                    depends_on: vec!["verify".to_string()],
                    condition: None,
                    continue_on_error: true,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
            ],
            execution_mode: ExecutionMode::Parallel,
        }
    }

    fn ci_pipeline_template() -> Workflow {
        Workflow {
            name: "ci-pipeline".to_string(),
            description: Some(
                "Complete CI pipeline (format, lint, verify, test, build)".to_string(),
            ),
            version: "1.0.0".to_string(),
            variables: HashMap::from([
                ("INPUT_DIR".to_string(), "./statutes".to_string()),
                ("OUTPUT_DIR".to_string(), "./build".to_string()),
                ("TEST_FILE".to_string(), "./tests.yaml".to_string()),
            ]),
            tasks: vec![
                Task {
                    id: "format".to_string(),
                    name: "Format Check".to_string(),
                    description: Some("Check formatting".to_string()),
                    command: "format".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--dry-run".to_string(),
                    ],
                    depends_on: vec![],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "lint".to_string(),
                    name: "Lint".to_string(),
                    description: Some("Run linter".to_string()),
                    command: "lint".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--strict".to_string(),
                    ],
                    depends_on: vec!["format".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "verify".to_string(),
                    name: "Verify".to_string(),
                    description: Some("Verify statutes".to_string()),
                    command: "verify".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--strict".to_string(),
                    ],
                    depends_on: vec!["lint".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "test".to_string(),
                    name: "Test".to_string(),
                    description: Some("Run tests".to_string()),
                    command: "test".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--tests".to_string(),
                        "${TEST_FILE}".to_string(),
                    ],
                    depends_on: vec!["verify".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(600),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "build".to_string(),
                    name: "Build".to_string(),
                    description: Some("Build artifacts".to_string()),
                    command: "export".to_string(),
                    args: vec![
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--output".to_string(),
                        "${OUTPUT_DIR}".to_string(),
                        "--export-format".to_string(),
                        "json".to_string(),
                    ],
                    depends_on: vec!["test".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(300),
                    retry: None,
                    env: HashMap::new(),
                },
            ],
            execution_mode: ExecutionMode::Sequential,
        }
    }

    fn batch_processing_template() -> Workflow {
        Workflow {
            name: "batch-processing".to_string(),
            description: Some("Batch process multiple statute files".to_string()),
            version: "1.0.0".to_string(),
            variables: HashMap::from([
                ("INPUT_DIR".to_string(), "./statutes".to_string()),
                ("OUTPUT_DIR".to_string(), "./processed".to_string()),
            ]),
            tasks: vec![
                Task {
                    id: "batch-verify".to_string(),
                    name: "Batch Verify".to_string(),
                    description: Some("Verify all files in parallel".to_string()),
                    command: "batch".to_string(),
                    args: vec![
                        "verify".to_string(),
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                    ],
                    depends_on: vec![],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(600),
                    retry: None,
                    env: HashMap::new(),
                },
                Task {
                    id: "batch-export".to_string(),
                    name: "Batch Export".to_string(),
                    description: Some("Export all files in parallel".to_string()),
                    command: "batch".to_string(),
                    args: vec![
                        "export".to_string(),
                        "--input".to_string(),
                        "${INPUT_DIR}/*.ldsl".to_string(),
                        "--output".to_string(),
                        "${OUTPUT_DIR}".to_string(),
                        "--export-format".to_string(),
                        "json".to_string(),
                    ],
                    depends_on: vec!["batch-verify".to_string()],
                    condition: None,
                    continue_on_error: false,
                    timeout: Some(600),
                    retry: None,
                    env: HashMap::new(),
                },
            ],
            execution_mode: ExecutionMode::Sequential,
        }
    }
}

/// Workflow template metadata.
#[derive(Debug, Clone)]
pub struct WorkflowTemplate {
    pub name: String,
    pub description: String,
    pub category: TemplateCategory,
}

/// Template category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateCategory {
    Testing,
    Quality,
    Build,
    CI,
    Batch,
}

impl std::fmt::Display for TemplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateCategory::Testing => write!(f, "Testing"),
            TemplateCategory::Quality => write!(f, "Quality"),
            TemplateCategory::Build => write!(f, "Build"),
            TemplateCategory::CI => write!(f, "CI/CD"),
            TemplateCategory::Batch => write!(f, "Batch Processing"),
        }
    }
}

/// Load a workflow from a YAML file.
pub fn load_workflow(path: &Path) -> anyhow::Result<Workflow> {
    let contents = std::fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("Failed to read workflow file: {}", e))?;

    serde_yaml::from_str(&contents)
        .map_err(|e| anyhow::anyhow!("Failed to parse workflow YAML: {}", e))
}

/// Save a workflow to a YAML file.
pub fn save_workflow(workflow: &Workflow, path: &Path) -> anyhow::Result<()> {
    let yaml = serde_yaml::to_string(workflow)
        .map_err(|e| anyhow::anyhow!("Failed to serialize workflow: {}", e))?;

    std::fs::write(path, yaml).map_err(|e| anyhow::anyhow!("Failed to write workflow file: {}", e))
}

/// Validate a workflow definition.
pub fn validate_workflow(workflow: &Workflow) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Check for duplicate task IDs
    let mut task_ids = std::collections::HashSet::new();
    for task in &workflow.tasks {
        if !task_ids.insert(&task.id) {
            errors.push(format!("Duplicate task ID: {}", task.id));
        }
    }

    // Check for invalid dependencies
    for task in &workflow.tasks {
        for dep in &task.depends_on {
            if !task_ids.contains(dep) {
                errors.push(format!(
                    "Task '{}' depends on non-existent task '{}'",
                    task.id, dep
                ));
            }
        }
    }

    // Check for circular dependencies
    if has_circular_dependencies(workflow) {
        errors.push("Circular dependency detected in workflow".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Check for circular dependencies in workflow.
fn has_circular_dependencies(workflow: &Workflow) -> bool {
    use std::collections::{HashMap, HashSet};

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for task in &workflow.tasks {
        graph.insert(
            &task.id,
            task.depends_on.iter().map(|s| s.as_str()).collect(),
        );
    }

    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for task in &workflow.tasks {
        if has_cycle(&graph, &task.id, &mut visited, &mut rec_stack) {
            return true;
        }
    }

    false
}

fn has_cycle<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    visited: &mut HashSet<&'a str>,
    rec_stack: &mut HashSet<&'a str>,
) -> bool {
    if rec_stack.contains(node) {
        return true;
    }

    if visited.contains(node) {
        return false;
    }

    visited.insert(node);
    rec_stack.insert(node);

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if has_cycle(graph, neighbor, visited, rec_stack) {
                return true;
            }
        }
    }

    rec_stack.remove(node);
    false
}
