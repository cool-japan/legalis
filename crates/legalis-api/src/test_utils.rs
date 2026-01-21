//! API Testing Utilities
//!
//! This module provides utilities for testing APIs including:
//! - Test client for making requests
//! - Assertion helpers
//! - Test data generators
//! - Response validators

use axum::http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for testing utilities
#[derive(Debug, Error)]
pub enum TestError {
    #[error("Test error: {0}")]
    Error(String),

    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Result type for testing operations
pub type TestResult<T> = Result<T, TestError>;

/// Test response wrapper
#[derive(Debug, Clone)]
pub struct TestResponse {
    /// HTTP status code
    pub status: StatusCode,

    /// Response headers
    pub headers: HeaderMap,

    /// Response body as JSON
    pub body: serde_json::Value,

    /// Raw body as string
    pub body_string: String,
}

impl TestResponse {
    /// Create a new test response
    pub fn new(status: StatusCode, headers: HeaderMap, body_string: String) -> Self {
        let body = serde_json::from_str(&body_string).unwrap_or(serde_json::Value::Null);

        Self {
            status,
            headers,
            body,
            body_string,
        }
    }

    /// Assert status code
    pub fn assert_status(&self, expected: StatusCode) -> TestResult<&Self> {
        if self.status == expected {
            Ok(self)
        } else {
            Err(TestError::AssertionFailed(format!(
                "Expected status {}, got {}",
                expected, self.status
            )))
        }
    }

    /// Assert status is success (2xx)
    pub fn assert_success(&self) -> TestResult<&Self> {
        if self.status.is_success() {
            Ok(self)
        } else {
            Err(TestError::AssertionFailed(format!(
                "Expected success status, got {}",
                self.status
            )))
        }
    }

    /// Assert header exists
    pub fn assert_header(&self, key: &str) -> TestResult<&Self> {
        if self.headers.contains_key(key) {
            Ok(self)
        } else {
            Err(TestError::AssertionFailed(format!(
                "Expected header '{}' not found",
                key
            )))
        }
    }

    /// Assert header has specific value
    pub fn assert_header_value(&self, key: &str, expected: &str) -> TestResult<&Self> {
        if let Some(value) = self.headers.get(key) {
            let value_str = value.to_str().unwrap_or("");
            if value_str == expected {
                Ok(self)
            } else {
                Err(TestError::AssertionFailed(format!(
                    "Header '{}' expected '{}', got '{}'",
                    key, expected, value_str
                )))
            }
        } else {
            Err(TestError::AssertionFailed(format!(
                "Header '{}' not found",
                key
            )))
        }
    }

    /// Assert JSON body contains field
    pub fn assert_json_field(&self, field: &str) -> TestResult<&Self> {
        if self.body.get(field).is_some() {
            Ok(self)
        } else {
            Err(TestError::AssertionFailed(format!(
                "JSON field '{}' not found",
                field
            )))
        }
    }

    /// Assert JSON field has specific value
    pub fn assert_json_value(
        &self,
        field: &str,
        expected: &serde_json::Value,
    ) -> TestResult<&Self> {
        if let Some(value) = self.body.get(field) {
            if value == expected {
                Ok(self)
            } else {
                Err(TestError::AssertionFailed(format!(
                    "JSON field '{}' expected {:?}, got {:?}",
                    field, expected, value
                )))
            }
        } else {
            Err(TestError::AssertionFailed(format!(
                "JSON field '{}' not found",
                field
            )))
        }
    }

    /// Assert response body matches pattern
    pub fn assert_body_contains(&self, pattern: &str) -> TestResult<&Self> {
        if self.body_string.contains(pattern) {
            Ok(self)
        } else {
            Err(TestError::AssertionFailed(format!(
                "Body does not contain '{}'",
                pattern
            )))
        }
    }

    /// Get JSON field value
    pub fn get_json_field(&self, field: &str) -> Option<&serde_json::Value> {
        self.body.get(field)
    }

    /// Deserialize body to type
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> TestResult<T> {
        serde_json::from_str(&self.body_string)
            .map_err(|e| TestError::Error(format!("Failed to deserialize JSON: {}", e)))
    }
}

/// Test data generator
pub struct TestDataGenerator;

impl TestDataGenerator {
    /// Generate random string
    pub fn random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::rng();

        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Generate random email
    pub fn random_email() -> String {
        format!("{}@test.com", Self::random_string(10))
    }

    /// Generate random UUID
    pub fn random_uuid() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Generate test statute
    pub fn test_statute() -> serde_json::Value {
        serde_json::json!({
            "title": format!("Test Statute {}", Self::random_string(5)),
            "content": "Test content",
            "metadata": {
                "author": "test",
                "version": "1.0"
            }
        })
    }

    /// Generate multiple test statutes
    pub fn test_statutes(count: usize) -> Vec<serde_json::Value> {
        (0..count).map(|_| Self::test_statute()).collect()
    }
}

/// Response validator
pub struct ResponseValidator;

impl ResponseValidator {
    /// Validate JSON schema (simple version)
    pub fn validate_schema(
        response: &serde_json::Value,
        schema: &serde_json::Value,
    ) -> TestResult<()> {
        // Simple schema validation
        if let Some(required) = schema.get("required")
            && let Some(required_fields) = required.as_array()
        {
            for field in required_fields {
                if let Some(field_name) = field.as_str()
                    && response.get(field_name).is_none()
                {
                    return Err(TestError::ValidationFailed(format!(
                        "Missing required field: {}",
                        field_name
                    )));
                }
            }
        }

        Ok(())
    }

    /// Validate response time
    pub fn validate_response_time(duration_ms: u64, max_ms: u64) -> TestResult<()> {
        if duration_ms <= max_ms {
            Ok(())
        } else {
            Err(TestError::ValidationFailed(format!(
                "Response time {}ms exceeds limit {}ms",
                duration_ms, max_ms
            )))
        }
    }

    /// Validate pagination response
    pub fn validate_pagination(response: &serde_json::Value) -> TestResult<PaginationInfo> {
        let total = response
            .get("total")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| TestError::ValidationFailed("Missing 'total' field".to_string()))?;

        let page = response
            .get("page")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| TestError::ValidationFailed("Missing 'page' field".to_string()))?;

        let page_size = response
            .get("page_size")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| TestError::ValidationFailed("Missing 'page_size' field".to_string()))?;

        let items = response
            .get("items")
            .and_then(|v| v.as_array())
            .ok_or_else(|| TestError::ValidationFailed("Missing 'items' field".to_string()))?;

        Ok(PaginationInfo {
            total: total as usize,
            page: page as usize,
            page_size: page_size as usize,
            items_count: items.len(),
        })
    }

    /// Validate error response format
    pub fn validate_error_response(response: &serde_json::Value) -> TestResult<()> {
        if response.get("error").is_none() && response.get("message").is_none() {
            return Err(TestError::ValidationFailed(
                "Error response must have 'error' or 'message' field".to_string(),
            ));
        }

        Ok(())
    }
}

/// Pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub items_count: usize,
}

/// Test suite builder
pub struct TestSuite {
    name: String,
    tests: Vec<TestCase>,
}

impl TestSuite {
    /// Create a new test suite
    pub fn new(name: String) -> Self {
        Self {
            name,
            tests: Vec::new(),
        }
    }

    /// Add a test case
    pub fn add_test(&mut self, test: TestCase) {
        self.tests.push(test);
    }

    /// Get test count
    pub fn test_count(&self) -> usize {
        self.tests.len()
    }

    /// Get suite name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get tests
    pub fn tests(&self) -> &[TestCase] {
        &self.tests
    }
}

/// Test case definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    /// Test name
    pub name: String,

    /// Test description
    pub description: Option<String>,

    /// HTTP method
    pub method: String,

    /// Request path
    pub path: String,

    /// Request headers
    pub headers: HashMap<String, String>,

    /// Request body
    pub body: Option<serde_json::Value>,

    /// Expected status code
    pub expected_status: u16,

    /// Expected response fields
    pub expected_fields: Vec<String>,
}

impl TestCase {
    /// Create a new test case
    pub fn new(name: String, method: String, path: String) -> Self {
        Self {
            name,
            description: None,
            method,
            path,
            headers: HashMap::new(),
            body: None,
            expected_status: 200,
            expected_fields: Vec::new(),
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

    /// Set body
    pub fn with_body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Set expected status
    pub fn expect_status(mut self, status: u16) -> Self {
        self.expected_status = status;
        self
    }

    /// Add expected field
    pub fn expect_field(mut self, field: String) -> Self {
        self.expected_fields.push(field);
        self
    }
}

/// Assertion helper macros would go here in a real implementation
/// For now, we provide basic assertion methods
#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_response_assertions() {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        let body = serde_json::json!({
            "status": "ok",
            "message": "Success"
        });

        let response = TestResponse::new(StatusCode::OK, headers, body.to_string());

        assert!(response.assert_status(StatusCode::OK).is_ok());
        assert!(response.assert_success().is_ok());
        assert!(response.assert_header("content-type").is_ok());
        assert!(response.assert_json_field("status").is_ok());
        assert!(
            response
                .assert_json_value("status", &serde_json::json!("ok"))
                .is_ok()
        );
    }

    #[test]
    fn test_data_generator() {
        let s = TestDataGenerator::random_string(10);
        assert_eq!(s.len(), 10);

        let email = TestDataGenerator::random_email();
        assert!(email.contains("@test.com"));

        let statute = TestDataGenerator::test_statute();
        assert!(statute.get("title").is_some());
        assert!(statute.get("content").is_some());
    }

    #[test]
    fn test_schema_validation() {
        let response = serde_json::json!({
            "id": "123",
            "name": "Test"
        });

        let schema = serde_json::json!({
            "required": ["id", "name"]
        });

        assert!(ResponseValidator::validate_schema(&response, &schema).is_ok());

        let schema = serde_json::json!({
            "required": ["id", "name", "email"]
        });

        assert!(ResponseValidator::validate_schema(&response, &schema).is_err());
    }

    #[test]
    fn test_pagination_validation() {
        let response = serde_json::json!({
            "total": 100,
            "page": 1,
            "page_size": 10,
            "items": [1, 2, 3, 4, 5]
        });

        let info = ResponseValidator::validate_pagination(&response).unwrap();
        assert_eq!(info.total, 100);
        assert_eq!(info.page, 1);
        assert_eq!(info.page_size, 10);
        assert_eq!(info.items_count, 5);
    }

    #[test]
    fn test_suite_builder() {
        let mut suite = TestSuite::new("Statute API Tests".to_string());

        let test1 = TestCase::new(
            "Get all statutes".to_string(),
            "GET".to_string(),
            "/api/v1/statutes".to_string(),
        )
        .expect_status(200)
        .expect_field("items".to_string());

        suite.add_test(test1);

        assert_eq!(suite.test_count(), 1);
        assert_eq!(suite.name(), "Statute API Tests");
    }

    #[test]
    fn test_error_response_validation() {
        let error_response = serde_json::json!({
            "error": "Not found",
            "code": 404
        });

        assert!(ResponseValidator::validate_error_response(&error_response).is_ok());

        let invalid_error = serde_json::json!({
            "status": "failed"
        });

        assert!(ResponseValidator::validate_error_response(&invalid_error).is_err());
    }
}
