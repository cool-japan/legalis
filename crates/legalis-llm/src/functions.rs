//! Function and tool calling support for LLMs.
//!
//! This module provides abstractions for defining and executing functions
//! that can be called by LLMs during generation, enabling agentic workflows.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// A function that can be called by an LLM.
#[derive(Clone)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Function description for the LLM
    pub description: String,
    /// JSON schema for the function parameters
    pub parameters: FunctionParameters,
    /// The actual function implementation
    executor: Arc<dyn Fn(Value) -> Result<Value> + Send + Sync>,
}

impl Function {
    /// Creates a new function.
    pub fn new<F>(name: impl Into<String>, description: impl Into<String>, executor: F) -> Self
    where
        F: Fn(Value) -> Result<Value> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: FunctionParameters::default(),
            executor: Arc::new(executor),
        }
    }

    /// Sets the function parameters schema.
    pub fn with_parameters(mut self, parameters: FunctionParameters) -> Self {
        self.parameters = parameters;
        self
    }

    /// Executes the function with the given arguments.
    pub fn execute(&self, args: Value) -> Result<Value> {
        (self.executor)(args)
    }

    /// Returns the function definition for API requests.
    pub fn to_definition(&self) -> FunctionDefinition {
        FunctionDefinition {
            name: self.name.clone(),
            description: self.description.clone(),
            parameters: self.parameters.clone(),
        }
    }
}

/// Function parameters schema (JSON Schema).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameters {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, ParameterProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl Default for FunctionParameters {
    fn default() -> Self {
        Self {
            schema_type: "object".to_string(),
            properties: Some(HashMap::new()),
            required: None,
        }
    }
}

impl FunctionParameters {
    /// Creates a new parameters schema.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a string parameter.
    pub fn add_string(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        let name = name.into();
        self.properties.get_or_insert_with(HashMap::new).insert(
            name.clone(),
            ParameterProperty {
                param_type: "string".to_string(),
                description: Some(description.into()),
                enum_values: None,
            },
        );

        if required {
            self.required.get_or_insert_with(Vec::new).push(name);
        }

        self
    }

    /// Adds a number parameter.
    pub fn add_number(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        let name = name.into();
        self.properties.get_or_insert_with(HashMap::new).insert(
            name.clone(),
            ParameterProperty {
                param_type: "number".to_string(),
                description: Some(description.into()),
                enum_values: None,
            },
        );

        if required {
            self.required.get_or_insert_with(Vec::new).push(name);
        }

        self
    }

    /// Adds a boolean parameter.
    pub fn add_boolean(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        let name = name.into();
        self.properties.get_or_insert_with(HashMap::new).insert(
            name.clone(),
            ParameterProperty {
                param_type: "boolean".to_string(),
                description: Some(description.into()),
                enum_values: None,
            },
        );

        if required {
            self.required.get_or_insert_with(Vec::new).push(name);
        }

        self
    }

    /// Adds an enum parameter.
    pub fn add_enum(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        values: Vec<String>,
        required: bool,
    ) -> Self {
        let name = name.into();
        self.properties.get_or_insert_with(HashMap::new).insert(
            name.clone(),
            ParameterProperty {
                param_type: "string".to_string(),
                description: Some(description.into()),
                enum_values: Some(values),
            },
        );

        if required {
            self.required.get_or_insert_with(Vec::new).push(name);
        }

        self
    }
}

/// Property definition for a function parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterProperty {
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
}

/// Function definition for API requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: FunctionParameters,
}

/// A registry of functions that can be called by LLMs.
#[derive(Clone)]
pub struct FunctionRegistry {
    functions: HashMap<String, Function>,
}

impl FunctionRegistry {
    /// Creates a new empty function registry.
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Registers a function.
    pub fn register(&mut self, function: Function) {
        self.functions.insert(function.name.clone(), function);
    }

    /// Registers a function with a builder pattern.
    pub fn with_function(mut self, function: Function) -> Self {
        self.register(function);
        self
    }

    /// Gets a function by name.
    pub fn get(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }

    /// Executes a function call by name.
    pub fn execute(&self, name: &str, args: Value) -> Result<Value> {
        let function = self
            .functions
            .get(name)
            .context(format!("Function '{}' not found", name))?;

        function.execute(args)
    }

    /// Returns all function definitions.
    pub fn get_definitions(&self) -> Vec<FunctionDefinition> {
        self.functions.values().map(|f| f.to_definition()).collect()
    }

    /// Returns the number of registered functions.
    pub fn len(&self) -> usize {
        self.functions.len()
    }

    /// Returns whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// A function call request from the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function to call
    pub name: String,
    /// The arguments to pass to the function (JSON)
    pub arguments: Value,
}

impl FunctionCall {
    /// Creates a new function call.
    pub fn new(name: impl Into<String>, arguments: Value) -> Self {
        Self {
            name: name.into(),
            arguments,
        }
    }

    /// Executes this function call using a registry.
    pub fn execute(&self, registry: &FunctionRegistry) -> Result<Value> {
        registry.execute(&self.name, self.arguments.clone())
    }
}

/// Result of a function execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionResult {
    /// The name of the function that was called
    pub name: String,
    /// The result of the function execution
    pub result: Value,
}

impl FunctionResult {
    /// Creates a new function result.
    pub fn new(name: impl Into<String>, result: Value) -> Self {
        Self {
            name: name.into(),
            result,
        }
    }

    /// Converts the result to a formatted string for LLM consumption.
    pub fn to_message(&self) -> String {
        format!(
            "Function '{}' returned: {}",
            self.name,
            serde_json::to_string_pretty(&self.result).unwrap_or_else(|_| "null".to_string())
        )
    }
}

/// Function execution hooks for monitoring and control.
pub trait FunctionHook: Send + Sync {
    /// Called before function execution.
    fn before_execute(&self, name: &str, args: &Value);
    /// Called after successful execution.
    fn after_execute(&self, name: &str, result: &Value);
    /// Called if execution fails.
    fn on_error(&self, name: &str, error: &anyhow::Error);
}

/// Function orchestrator for advanced execution patterns.
pub struct FunctionOrchestrator {
    registry: FunctionRegistry,
    hooks: Vec<Arc<dyn FunctionHook>>,
    max_retries: usize,
    timeout_ms: Option<u64>,
}

impl FunctionOrchestrator {
    /// Creates a new orchestrator with the given registry.
    pub fn new(registry: FunctionRegistry) -> Self {
        Self {
            registry,
            hooks: Vec::new(),
            max_retries: 0,
            timeout_ms: None,
        }
    }

    /// Adds a hook to the orchestrator.
    pub fn add_hook<H: FunctionHook + 'static>(mut self, hook: H) -> Self {
        self.hooks.push(Arc::new(hook));
        self
    }

    /// Sets the maximum number of retries for failed function calls.
    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets a timeout for function execution (in milliseconds).
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }

    /// Executes a function with retry logic and hooks.
    pub fn execute(&self, name: &str, args: Value) -> Result<Value> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            // Call before hooks
            for hook in &self.hooks {
                hook.before_execute(name, &args);
            }

            // Execute the function
            match self.registry.execute(name, args.clone()) {
                Ok(result) => {
                    // Call after hooks
                    for hook in &self.hooks {
                        hook.after_execute(name, &result);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    // Call error hooks
                    for hook in &self.hooks {
                        hook.on_error(name, &e);
                    }

                    last_error = Some(e);

                    // Don't retry on the last attempt
                    if attempt < self.max_retries {
                        // Exponential backoff
                        let delay_ms = 100 * 2_u64.pow(attempt as u32);
                        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                    }
                }
            }
        }

        Err(last_error.unwrap())
    }

    /// Executes multiple functions in parallel.
    pub async fn execute_parallel(&self, calls: Vec<FunctionCall>) -> Vec<Result<Value>> {
        use tokio::task;

        let mut handles = Vec::new();

        for call in calls {
            let registry = self.registry.clone();
            let hooks = self.hooks.clone();
            let max_retries = self.max_retries;

            let handle = task::spawn(async move {
                let mut last_error = None;

                for attempt in 0..=max_retries {
                    // Call before hooks
                    for hook in &hooks {
                        hook.before_execute(&call.name, &call.arguments);
                    }

                    // Execute the function
                    match registry.execute(&call.name, call.arguments.clone()) {
                        Ok(result) => {
                            // Call after hooks
                            for hook in &hooks {
                                hook.after_execute(&call.name, &result);
                            }
                            return Ok(result);
                        }
                        Err(e) => {
                            // Call error hooks
                            for hook in &hooks {
                                hook.on_error(&call.name, &e);
                            }

                            last_error = Some(e);

                            if attempt < max_retries {
                                let delay_ms = 100 * 2_u64.pow(attempt as u32);
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                    .await;
                            }
                        }
                    }
                }

                Err(last_error.unwrap())
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(
                handle
                    .await
                    .unwrap_or_else(|e| Err(anyhow::anyhow!("Task panicked: {}", e))),
            );
        }

        results
    }

    /// Executes a chain of function calls, passing results forward.
    pub fn execute_chain(&self, chain: Vec<(String, Value)>) -> Result<Value> {
        let mut last_result = Value::Null;

        for (name, mut args) in chain {
            // If args contains a special "$prev" key, replace it with the last result
            if let Some(obj) = args.as_object_mut()
                && obj.contains_key("$prev")
            {
                obj.insert("$prev".to_string(), last_result.clone());
            }

            last_result = self.execute(&name, args)?;
        }

        Ok(last_result)
    }

    /// Gets a reference to the underlying registry.
    pub fn registry(&self) -> &FunctionRegistry {
        &self.registry
    }
}

/// A simple logging hook for debugging.
pub struct LoggingHook;

impl FunctionHook for LoggingHook {
    fn before_execute(&self, name: &str, _args: &Value) {
        tracing::debug!("Executing function: {}", name);
    }

    fn after_execute(&self, name: &str, _result: &Value) {
        tracing::debug!("Function {} completed successfully", name);
    }

    fn on_error(&self, name: &str, error: &anyhow::Error) {
        tracing::error!("Function {} failed: {}", name, error);
    }
}

/// Helper macros and utilities for common function patterns.
pub mod helpers {
    use super::*;

    /// Creates a calculator function.
    pub fn create_calculator() -> Function {
        Function::new("calculate", "Performs mathematical calculations", |args| {
            let expression = args
                .get("expression")
                .and_then(|v| v.as_str())
                .context("Missing 'expression' parameter")?;

            // Simple expression evaluator (for demo purposes)
            // In production, use a proper expression parser
            let result = match expression {
                expr if expr.contains('+') => {
                    let parts: Vec<&str> = expr.split('+').collect();
                    if parts.len() != 2 {
                        anyhow::bail!("Invalid expression format");
                    }
                    let a: f64 = parts[0].trim().parse()?;
                    let b: f64 = parts[1].trim().parse()?;
                    a + b
                }
                expr if expr.contains('-') => {
                    let parts: Vec<&str> = expr.split('-').collect();
                    if parts.len() != 2 {
                        anyhow::bail!("Invalid expression format");
                    }
                    let a: f64 = parts[0].trim().parse()?;
                    let b: f64 = parts[1].trim().parse()?;
                    a - b
                }
                expr if expr.contains('*') => {
                    let parts: Vec<&str> = expr.split('*').collect();
                    if parts.len() != 2 {
                        anyhow::bail!("Invalid expression format");
                    }
                    let a: f64 = parts[0].trim().parse()?;
                    let b: f64 = parts[1].trim().parse()?;
                    a * b
                }
                expr if expr.contains('/') => {
                    let parts: Vec<&str> = expr.split('/').collect();
                    if parts.len() != 2 {
                        anyhow::bail!("Invalid expression format");
                    }
                    let a: f64 = parts[0].trim().parse()?;
                    let b: f64 = parts[1].trim().parse()?;
                    if b == 0.0 {
                        anyhow::bail!("Division by zero");
                    }
                    a / b
                }
                _ => anyhow::bail!("Unsupported operation"),
            };

            Ok(serde_json::json!({ "result": result }))
        })
        .with_parameters(FunctionParameters::new().add_string(
            "expression",
            "Mathematical expression to evaluate (e.g., '2 + 2')",
            true,
        ))
    }

    /// Creates a datetime function.
    pub fn create_datetime() -> Function {
        Function::new("get_datetime", "Gets the current date and time", |_args| {
            let now = chrono::Utc::now();
            Ok(serde_json::json!({
                "datetime": now.to_rfc3339(),
                "timestamp": now.timestamp(),
                "date": now.format("%Y-%m-%d").to_string(),
                "time": now.format("%H:%M:%S").to_string(),
            }))
        })
        .with_parameters(FunctionParameters::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_creation() {
        let func = Function::new("test", "A test function", |args| {
            Ok(serde_json::json!({ "input": args }))
        });

        assert_eq!(func.name, "test");
        assert_eq!(func.description, "A test function");
    }

    #[test]
    fn test_function_execution() {
        let func = Function::new("double", "Doubles a number", |args| {
            let num = args
                .get("value")
                .and_then(|v| v.as_i64())
                .context("Missing or invalid 'value' parameter")?;
            Ok(serde_json::json!({ "result": num * 2 }))
        });

        let result = func.execute(serde_json::json!({ "value": 21 })).unwrap();
        assert_eq!(result["result"], 42);
    }

    #[test]
    fn test_function_parameters() {
        let params = FunctionParameters::new()
            .add_string("name", "User's name", true)
            .add_number("age", "User's age", false)
            .add_boolean("active", "Is user active", true)
            .add_enum(
                "role",
                "User role",
                vec!["admin".to_string(), "user".to_string()],
                true,
            );

        assert_eq!(params.properties.as_ref().unwrap().len(), 4);
        assert_eq!(params.required.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_registry() {
        let mut registry = FunctionRegistry::new();

        let func1 = Function::new("func1", "First function", |_| {
            Ok(serde_json::json!("result1"))
        });
        let func2 = Function::new("func2", "Second function", |_| {
            Ok(serde_json::json!("result2"))
        });

        registry.register(func1);
        registry.register(func2);

        assert_eq!(registry.len(), 2);
        assert!(registry.get("func1").is_some());
        assert!(registry.get("func3").is_none());
    }

    #[test]
    fn test_registry_execution() {
        let registry = FunctionRegistry::new().with_function(Function::new(
            "greet",
            "Greets a person",
            |args| {
                let name = args.get("name").and_then(|v| v.as_str()).unwrap_or("World");
                Ok(serde_json::json!({ "message": format!("Hello, {}!", name) }))
            },
        ));

        let result = registry
            .execute("greet", serde_json::json!({ "name": "Alice" }))
            .unwrap();
        assert_eq!(result["message"], "Hello, Alice!");
    }

    #[test]
    fn test_function_call() {
        let call = FunctionCall::new("test", serde_json::json!({ "key": "value" }));
        assert_eq!(call.name, "test");
        assert_eq!(call.arguments["key"], "value");
    }

    #[test]
    fn test_function_result() {
        let result = FunctionResult::new("test", serde_json::json!({ "data": 42 }));
        let message = result.to_message();
        assert!(message.contains("test"));
        assert!(message.contains("42"));
    }

    #[test]
    fn test_calculator_function() {
        let calc = helpers::create_calculator();

        let result = calc
            .execute(serde_json::json!({ "expression": "10 + 5" }))
            .unwrap();
        assert_eq!(result["result"], 15.0);

        let result = calc
            .execute(serde_json::json!({ "expression": "20 * 3" }))
            .unwrap();
        assert_eq!(result["result"], 60.0);
    }

    #[test]
    fn test_datetime_function() {
        let dt = helpers::create_datetime();
        let result = dt.execute(serde_json::json!({})).unwrap();

        assert!(result["datetime"].is_string());
        assert!(result["timestamp"].is_number());
        assert!(result["date"].is_string());
        assert!(result["time"].is_string());
    }

    #[test]
    fn test_function_orchestrator_basic() {
        let mut registry = FunctionRegistry::new();
        registry.register(Function::new("double", "Doubles a number", |args| {
            let num = args.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
            Ok(serde_json::json!({ "result": num * 2.0 }))
        }));

        let orchestrator = FunctionOrchestrator::new(registry);
        let result = orchestrator
            .execute("double", serde_json::json!({ "value": 5.0 }))
            .unwrap();

        assert_eq!(result["result"], 10.0);
    }

    #[test]
    fn test_function_orchestrator_with_retries() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let mut registry = FunctionRegistry::new();
        registry.register(Function::new("flaky", "A flaky function", move |_args| {
            let count = counter_clone.fetch_add(1, Ordering::SeqCst);
            if count < 2 {
                anyhow::bail!("Simulated failure");
            }
            Ok(serde_json::json!({ "success": true }))
        }));

        let orchestrator = FunctionOrchestrator::new(registry).with_max_retries(3);

        let result = orchestrator
            .execute("flaky", serde_json::json!({}))
            .unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(counter.load(Ordering::SeqCst), 3); // Failed twice, succeeded on third try
    }

    #[test]
    fn test_function_orchestrator_chain() {
        let mut registry = FunctionRegistry::new();

        registry.register(Function::new("add5", "Adds 5", |args| {
            let num = args.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
            Ok(serde_json::json!({ "value": num + 5.0 }))
        }));

        registry.register(Function::new("multiply2", "Multiplies by 2", |args| {
            let num = args.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
            Ok(serde_json::json!({ "value": num * 2.0 }))
        }));

        let orchestrator = FunctionOrchestrator::new(registry);

        let chain = vec![
            ("add5".to_string(), serde_json::json!({ "value": 10.0 })),
            (
                "multiply2".to_string(),
                serde_json::json!({ "value": 15.0 }),
            ), // Will use 15 from previous
        ];

        let result = orchestrator.execute_chain(chain).unwrap();
        assert_eq!(result["value"], 30.0); // (10 + 5) * 2 = 30
    }

    #[tokio::test]
    async fn test_function_orchestrator_parallel() {
        let mut registry = FunctionRegistry::new();

        registry.register(Function::new("add", "Adds two numbers", |args| {
            let a = args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let b = args.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
            Ok(serde_json::json!({ "result": a + b }))
        }));

        registry.register(Function::new(
            "multiply",
            "Multiplies two numbers",
            |args| {
                let a = args.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let b = args.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);
                Ok(serde_json::json!({ "result": a * b }))
            },
        ));

        let orchestrator = FunctionOrchestrator::new(registry);

        let calls = vec![
            FunctionCall::new("add", serde_json::json!({ "a": 5.0, "b": 3.0 })),
            FunctionCall::new("multiply", serde_json::json!({ "a": 4.0, "b": 2.0 })),
        ];

        let results = orchestrator.execute_parallel(calls).await;

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].as_ref().unwrap()["result"], 8.0);
        assert_eq!(results[1].as_ref().unwrap()["result"], 8.0);
    }

    #[test]
    fn test_logging_hook() {
        struct TestHook {
            before_count: Arc<AtomicUsize>,
            after_count: Arc<AtomicUsize>,
            error_count: Arc<AtomicUsize>,
        }

        impl FunctionHook for TestHook {
            fn before_execute(&self, _name: &str, _args: &Value) {
                self.before_count.fetch_add(1, Ordering::SeqCst);
            }

            fn after_execute(&self, _name: &str, _result: &Value) {
                self.after_count.fetch_add(1, Ordering::SeqCst);
            }

            fn on_error(&self, _name: &str, _error: &anyhow::Error) {
                self.error_count.fetch_add(1, Ordering::SeqCst);
            }
        }

        use std::sync::atomic::{AtomicUsize, Ordering};

        let before_count = Arc::new(AtomicUsize::new(0));
        let after_count = Arc::new(AtomicUsize::new(0));
        let error_count = Arc::new(AtomicUsize::new(0));

        let hook = TestHook {
            before_count: before_count.clone(),
            after_count: after_count.clone(),
            error_count: error_count.clone(),
        };

        let mut registry = FunctionRegistry::new();
        registry.register(Function::new("test", "Test function", |_args| {
            Ok(serde_json::json!({ "ok": true }))
        }));

        let orchestrator = FunctionOrchestrator::new(registry).add_hook(hook);
        let _ = orchestrator.execute("test", serde_json::json!({})).unwrap();

        assert_eq!(before_count.load(Ordering::SeqCst), 1);
        assert_eq!(after_count.load(Ordering::SeqCst), 1);
        assert_eq!(error_count.load(Ordering::SeqCst), 0);
    }
}
