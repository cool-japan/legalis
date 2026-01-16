//! API Playground improvements
//!
//! This module provides enhanced API playground features including:
//! - Interactive API documentation
//! - Request builder with code generation
//! - Response visualization
//! - Collection management
//! - Environment variables

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

/// Error types for playground operations
#[derive(Debug, Error)]
pub enum PlaygroundError {
    #[error("Playground error: {0}")]
    Error(String),

    #[error("Collection not found: {0}")]
    CollectionNotFound(String),

    #[error("Request not found: {0}")]
    RequestNotFound(String),

    #[error("Environment not found: {0}")]
    EnvironmentNotFound(String),
}

/// Result type for playground operations
pub type PlaygroundResult<T> = Result<T, PlaygroundError>;

/// HTTP method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
        }
    }
}

/// Request template for the playground
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestTemplate {
    /// Request ID
    pub id: Uuid,

    /// Request name
    pub name: String,

    /// Description
    pub description: Option<String>,

    /// HTTP method
    pub method: HttpMethod,

    /// URL with variable placeholders
    pub url: String,

    /// Headers
    pub headers: HashMap<String, String>,

    /// Query parameters
    pub query_params: HashMap<String, String>,

    /// Request body
    pub body: Option<serde_json::Value>,

    /// Tags for categorization
    pub tags: Vec<String>,
}

impl RequestTemplate {
    /// Create a new request template
    pub fn new(name: String, method: HttpMethod, url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            method,
            url,
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
            tags: Vec::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Add query parameter
    pub fn with_query_param(mut self, key: String, value: String) -> Self {
        self.query_params.insert(key, value);
        self
    }

    /// Set body
    pub fn with_body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Generate code snippet for this request
    pub fn generate_code(&self, language: CodeLanguage, env: &Environment) -> String {
        let url = self.resolve_variables(&self.url, env);

        match language {
            CodeLanguage::Curl => self.generate_curl(&url, env),
            CodeLanguage::JavaScript => self.generate_javascript(&url, env),
            CodeLanguage::Python => self.generate_python(&url, env),
            CodeLanguage::Rust => self.generate_rust(&url, env),
            CodeLanguage::Go => self.generate_go(&url, env),
        }
    }

    fn resolve_variables(&self, text: &str, env: &Environment) -> String {
        let mut result = text.to_string();
        for (key, value) in &env.variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        result
    }

    fn generate_curl(&self, url: &str, env: &Environment) -> String {
        let mut cmd = format!("curl -X {} '{}'", self.method, url);

        for (key, value) in &self.headers {
            let resolved_value = self.resolve_variables(value, env);
            cmd.push_str(&format!(" \\\n  -H '{}: {}'", key, resolved_value));
        }

        if let Some(body) = &self.body {
            cmd.push_str(&format!(" \\\n  -d '{}'", body));
        }

        cmd
    }

    fn generate_javascript(&self, url: &str, env: &Environment) -> String {
        let mut code = format!("fetch('{}', {{\n", url);
        code.push_str(&format!("  method: '{}',\n", self.method));

        if !self.headers.is_empty() {
            code.push_str("  headers: {\n");
            for (key, value) in &self.headers {
                let resolved_value = self.resolve_variables(value, env);
                code.push_str(&format!("    '{}': '{}',\n", key, resolved_value));
            }
            code.push_str("  },\n");
        }

        if let Some(body) = &self.body {
            code.push_str(&format!("  body: JSON.stringify({}),\n", body));
        }

        code.push_str("})\n  .then(res => res.json())\n  .then(data => console.log(data));");
        code
    }

    fn generate_python(&self, url: &str, env: &Environment) -> String {
        let mut code = String::from("import requests\n\n");

        if !self.headers.is_empty() {
            code.push_str("headers = {\n");
            for (key, value) in &self.headers {
                let resolved_value = self.resolve_variables(value, env);
                code.push_str(&format!("    '{}': '{}',\n", key, resolved_value));
            }
            code.push_str("}\n\n");
        }

        code.push_str(&format!(
            "response = requests.{}('{}',",
            self.method.to_string().to_lowercase(),
            url
        ));

        if !self.headers.is_empty() {
            code.push_str(" headers=headers,");
        }

        if let Some(body) = &self.body {
            code.push_str(&format!(" json={},", body));
        }

        code.push_str(")\nprint(response.json())");
        code
    }

    fn generate_rust(&self, url: &str, env: &Environment) -> String {
        let mut code = String::from(
            "use reqwest;\n\n#[tokio::main]\nasync fn main() -> Result<(), Box<dyn std::error::Error>> {\n",
        );
        code.push_str("    let client = reqwest::Client::new();\n");
        code.push_str(&format!(
            "    let response = client.{}(\"{}\")\n",
            self.method.to_string().to_lowercase(),
            url
        ));

        for (key, value) in &self.headers {
            let resolved_value = self.resolve_variables(value, env);
            code.push_str(&format!(
                "        .header(\"{}\", \"{}\")\n",
                key, resolved_value
            ));
        }

        if let Some(body) = &self.body {
            code.push_str(&format!("        .json(&{})\n", body));
        }

        code.push_str("        .send()\n        .await?;\n\n");
        code.push_str("    let body = response.json::<serde_json::Value>().await?;\n");
        code.push_str("    println!(\"{:?}\", body);\n");
        code.push_str("    Ok(())\n}");
        code
    }

    fn generate_go(&self, url: &str, env: &Environment) -> String {
        let mut code = String::from("package main\n\nimport (\n    \"fmt\"\n    \"net/http\"\n");

        if self.body.is_some() {
            code.push_str("    \"bytes\"\n    \"encoding/json\"\n");
        }

        code.push_str(")\n\nfunc main() {\n");

        if let Some(body) = &self.body {
            code.push_str(&format!("    jsonData, _ := json.Marshal({})\n", body));
            code.push_str(&format!(
                "    req, _ := http.NewRequest(\"{}\", \"{}\", bytes.NewBuffer(jsonData))\n",
                self.method, url
            ));
        } else {
            code.push_str(&format!(
                "    req, _ := http.NewRequest(\"{}\", \"{}\", nil)\n",
                self.method, url
            ));
        }

        for (key, value) in &self.headers {
            let resolved_value = self.resolve_variables(value, env);
            code.push_str(&format!(
                "    req.Header.Set(\"{}\", \"{}\")\n",
                key, resolved_value
            ));
        }

        code.push_str("\n    client := &http.Client{}\n");
        code.push_str("    resp, _ := client.Do(req)\n");
        code.push_str("    defer resp.Body.Close()\n");
        code.push_str("    fmt.Println(resp.Status)\n");
        code.push('}');
        code
    }
}

/// Code language for generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CodeLanguage {
    Curl,
    JavaScript,
    Python,
    Rust,
    Go,
}

/// Collection of requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCollection {
    /// Collection ID
    pub id: Uuid,

    /// Collection name
    pub name: String,

    /// Description
    pub description: Option<String>,

    /// Requests in this collection
    pub requests: Vec<RequestTemplate>,
}

impl RequestCollection {
    /// Create a new collection
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            requests: Vec::new(),
        }
    }

    /// Add request to collection
    pub fn add_request(&mut self, request: RequestTemplate) {
        self.requests.push(request);
    }

    /// Remove request from collection
    pub fn remove_request(&mut self, request_id: Uuid) -> PlaygroundResult<()> {
        let index = self
            .requests
            .iter()
            .position(|r| r.id == request_id)
            .ok_or_else(|| PlaygroundError::RequestNotFound(request_id.to_string()))?;

        self.requests.remove(index);
        Ok(())
    }
}

/// Environment for variable substitution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Environment ID
    pub id: Uuid,

    /// Environment name
    pub name: String,

    /// Variables
    pub variables: HashMap<String, String>,
}

impl Environment {
    /// Create a new environment
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            variables: HashMap::new(),
        }
    }

    /// Set variable
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    /// Get variable
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}

/// Playground manager
pub struct PlaygroundManager {
    collections: Arc<RwLock<HashMap<Uuid, RequestCollection>>>,
    environments: Arc<RwLock<HashMap<Uuid, Environment>>>,
}

impl PlaygroundManager {
    /// Create a new playground manager
    pub fn new() -> Self {
        Self {
            collections: Arc::new(RwLock::new(HashMap::new())),
            environments: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a collection
    pub fn create_collection(&self, collection: RequestCollection) -> PlaygroundResult<Uuid> {
        let id = collection.id;
        let mut collections = self
            .collections
            .write()
            .map_err(|e| PlaygroundError::Error(format!("Failed to acquire write lock: {}", e)))?;

        collections.insert(id, collection);
        Ok(id)
    }

    /// Get a collection
    pub fn get_collection(&self, id: Uuid) -> PlaygroundResult<RequestCollection> {
        let collections = self
            .collections
            .read()
            .map_err(|e| PlaygroundError::Error(format!("Failed to acquire read lock: {}", e)))?;

        collections
            .get(&id)
            .cloned()
            .ok_or_else(|| PlaygroundError::CollectionNotFound(id.to_string()))
    }

    /// List all collections
    pub fn list_collections(&self) -> Vec<RequestCollection> {
        self.collections.read().unwrap().values().cloned().collect()
    }

    /// Delete a collection
    pub fn delete_collection(&self, id: Uuid) -> PlaygroundResult<()> {
        let mut collections = self
            .collections
            .write()
            .map_err(|e| PlaygroundError::Error(format!("Failed to acquire write lock: {}", e)))?;

        collections
            .remove(&id)
            .ok_or_else(|| PlaygroundError::CollectionNotFound(id.to_string()))?;

        Ok(())
    }

    /// Create an environment
    pub fn create_environment(&self, environment: Environment) -> PlaygroundResult<Uuid> {
        let id = environment.id;
        let mut environments = self
            .environments
            .write()
            .map_err(|e| PlaygroundError::Error(format!("Failed to acquire write lock: {}", e)))?;

        environments.insert(id, environment);
        Ok(id)
    }

    /// Get an environment
    pub fn get_environment(&self, id: Uuid) -> PlaygroundResult<Environment> {
        let environments = self
            .environments
            .read()
            .map_err(|e| PlaygroundError::Error(format!("Failed to acquire read lock: {}", e)))?;

        environments
            .get(&id)
            .cloned()
            .ok_or_else(|| PlaygroundError::EnvironmentNotFound(id.to_string()))
    }

    /// List all environments
    pub fn list_environments(&self) -> Vec<Environment> {
        self.environments
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }
}

impl Default for PlaygroundManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_template_creation() {
        let request = RequestTemplate::new(
            "Get Statutes".to_string(),
            HttpMethod::GET,
            "/api/v1/statutes".to_string(),
        );

        assert_eq!(request.name, "Get Statutes");
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.url, "/api/v1/statutes");
    }

    #[test]
    fn test_code_generation_curl() {
        let mut env = Environment::new("test".to_string());
        env.set_variable("API_KEY".to_string(), "secret123".to_string());

        let request = RequestTemplate::new(
            "Get Statutes".to_string(),
            HttpMethod::GET,
            "http://localhost:3000/api/v1/statutes".to_string(),
        )
        .with_header(
            "Authorization".to_string(),
            "Bearer {{API_KEY}}".to_string(),
        );

        let code = request.generate_code(CodeLanguage::Curl, &env);
        assert!(code.contains("curl"));
        assert!(code.contains("Bearer secret123"));
    }

    #[test]
    fn test_collection_management() {
        let mut collection = RequestCollection::new("Test Collection".to_string());

        let request = RequestTemplate::new(
            "Get Statutes".to_string(),
            HttpMethod::GET,
            "/api/v1/statutes".to_string(),
        );

        collection.add_request(request.clone());
        assert_eq!(collection.requests.len(), 1);

        collection.remove_request(request.id).unwrap();
        assert_eq!(collection.requests.len(), 0);
    }

    #[test]
    fn test_playground_manager() {
        let manager = PlaygroundManager::new();

        let collection = RequestCollection::new("Test".to_string());
        let id = manager.create_collection(collection.clone()).unwrap();

        let retrieved = manager.get_collection(id).unwrap();
        assert_eq!(retrieved.name, "Test");

        let collections = manager.list_collections();
        assert_eq!(collections.len(), 1);

        manager.delete_collection(id).unwrap();
        let collections = manager.list_collections();
        assert_eq!(collections.len(), 0);
    }

    #[test]
    fn test_environment_variables() {
        let mut env = Environment::new("production".to_string());
        env.set_variable(
            "BASE_URL".to_string(),
            "https://api.example.com".to_string(),
        );
        env.set_variable("API_KEY".to_string(), "secret".to_string());

        assert_eq!(
            env.get_variable("BASE_URL"),
            Some(&"https://api.example.com".to_string())
        );
        assert_eq!(env.get_variable("API_KEY"), Some(&"secret".to_string()));
    }
}
