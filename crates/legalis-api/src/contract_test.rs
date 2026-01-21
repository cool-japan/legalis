//! API contract testing utilities.
//!
//! This module provides utilities for API contract testing,
//! ensuring API responses match expected schemas and contracts.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Contract test result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTestResult {
    /// Test name
    pub test_name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Error messages if failed
    pub errors: Vec<String>,
    /// Warnings (non-breaking issues)
    pub warnings: Vec<String>,
}

/// Contract violation type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    /// Missing required field
    MissingField(String),
    /// Unexpected field (not in schema)
    UnexpectedField(String),
    /// Type mismatch
    TypeMismatch {
        field: String,
        expected: String,
        actual: String,
    },
    /// Value constraint violation
    ConstraintViolation { field: String, constraint: String },
    /// Status code mismatch
    StatusCodeMismatch { expected: u16, actual: u16 },
    /// Header missing
    MissingHeader(String),
}

/// JSON schema field definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    /// Field name
    pub name: String,
    /// Expected type (string, number, boolean, object, array, null)
    pub field_type: String,
    /// Whether the field is required
    pub required: bool,
    /// Nested schema for objects
    pub nested_schema: Option<Box<ContractSchema>>,
    /// Item schema for arrays
    pub array_item_schema: Option<Box<FieldSchema>>,
}

/// Contract schema definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSchema {
    /// Schema fields
    pub fields: Vec<FieldSchema>,
    /// Whether to allow additional fields
    pub allow_additional_fields: bool,
}

/// HTTP response contract.
#[derive(Debug, Clone)]
pub struct ResponseContract {
    /// Expected status code
    pub status_code: u16,
    /// Expected response schema
    pub schema: ContractSchema,
    /// Required headers
    pub required_headers: Vec<String>,
}

/// Contract validator.
pub struct ContractValidator;

impl ContractValidator {
    /// Creates a new contract validator.
    pub fn new() -> Self {
        Self
    }

    /// Validates a response against a contract.
    pub fn validate_response(
        &self,
        test_name: &str,
        actual_status: u16,
        actual_body: &Value,
        actual_headers: &HashMap<String, String>,
        contract: &ResponseContract,
    ) -> ContractTestResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate status code
        if actual_status != contract.status_code {
            errors.push(format!(
                "Status code mismatch: expected {}, got {}",
                contract.status_code, actual_status
            ));
        }

        // Validate headers
        for required_header in &contract.required_headers {
            if !actual_headers.contains_key(required_header) {
                errors.push(format!("Missing required header: {}", required_header));
            }
        }

        // Validate response body schema
        let schema_violations = self.validate_schema(actual_body, &contract.schema);
        for violation in schema_violations {
            match violation {
                ViolationType::MissingField(field) => {
                    errors.push(format!("Missing required field: {}", field));
                }
                ViolationType::UnexpectedField(field) => {
                    warnings.push(format!("Unexpected field: {}", field));
                }
                ViolationType::TypeMismatch {
                    field,
                    expected,
                    actual,
                } => {
                    errors.push(format!(
                        "Type mismatch for field '{}': expected {}, got {}",
                        field, expected, actual
                    ));
                }
                ViolationType::ConstraintViolation { field, constraint } => {
                    errors.push(format!(
                        "Constraint violation for field '{}': {}",
                        field, constraint
                    ));
                }
                _ => {}
            }
        }

        ContractTestResult {
            test_name: test_name.to_string(),
            passed: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Validates a JSON value against a schema.
    fn validate_schema(&self, value: &Value, schema: &ContractSchema) -> Vec<ViolationType> {
        let mut violations = Vec::new();

        if !value.is_object() {
            violations.push(ViolationType::TypeMismatch {
                field: "root".to_string(),
                expected: "object".to_string(),
                actual: self.get_type_name(value),
            });
            return violations;
        }

        let obj = value.as_object().unwrap();

        // Check required fields
        for field_schema in &schema.fields {
            if field_schema.required && !obj.contains_key(&field_schema.name) {
                violations.push(ViolationType::MissingField(field_schema.name.clone()));
            }
        }

        // Check field types and nested schemas
        for (field_name, field_value) in obj {
            if let Some(field_schema) = schema.fields.iter().find(|f| &f.name == field_name) {
                // Validate type
                if !self.matches_type(field_value, &field_schema.field_type) {
                    violations.push(ViolationType::TypeMismatch {
                        field: field_name.clone(),
                        expected: field_schema.field_type.clone(),
                        actual: self.get_type_name(field_value),
                    });
                }

                // Validate nested object
                if let Some(nested_schema) = &field_schema.nested_schema
                    && field_value.is_object()
                {
                    let nested_violations = self.validate_schema(field_value, nested_schema);
                    violations.extend(nested_violations);
                }

                // Validate array items
                if let Some(item_schema) = &field_schema.array_item_schema
                    && let Some(arr) = field_value.as_array()
                {
                    for item in arr {
                        if !self.matches_type(item, &item_schema.field_type) {
                            violations.push(ViolationType::TypeMismatch {
                                field: format!("{}[]", field_name),
                                expected: item_schema.field_type.clone(),
                                actual: self.get_type_name(item),
                            });
                        }
                    }
                }
            } else if !schema.allow_additional_fields {
                violations.push(ViolationType::UnexpectedField(field_name.clone()));
            }
        }

        violations
    }

    /// Checks if a value matches an expected type.
    fn matches_type(&self, value: &Value, expected_type: &str) -> bool {
        match expected_type {
            "string" => value.is_string(),
            "number" => value.is_number(),
            "boolean" => value.is_boolean(),
            "object" => value.is_object(),
            "array" => value.is_array(),
            "null" => value.is_null(),
            _ => false,
        }
    }

    /// Gets the type name of a JSON value.
    fn get_type_name(&self, value: &Value) -> String {
        if value.is_string() {
            "string".to_string()
        } else if value.is_number() {
            "number".to_string()
        } else if value.is_boolean() {
            "boolean".to_string()
        } else if value.is_object() {
            "object".to_string()
        } else if value.is_array() {
            "array".to_string()
        } else if value.is_null() {
            "null".to_string()
        } else {
            "unknown".to_string()
        }
    }
}

impl Default for ContractValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-defined contract schemas for common endpoints.
pub mod schemas {
    use super::*;

    /// Schema for statute response.
    pub fn statute_schema() -> ContractSchema {
        ContractSchema {
            fields: vec![
                FieldSchema {
                    name: "id".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    nested_schema: None,
                    array_item_schema: None,
                },
                FieldSchema {
                    name: "title".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    nested_schema: None,
                    array_item_schema: None,
                },
                FieldSchema {
                    name: "effect".to_string(),
                    field_type: "object".to_string(),
                    required: true,
                    nested_schema: Some(Box::new(effect_schema())),
                    array_item_schema: None,
                },
            ],
            allow_additional_fields: true,
        }
    }

    /// Schema for effect object.
    pub fn effect_schema() -> ContractSchema {
        ContractSchema {
            fields: vec![
                FieldSchema {
                    name: "effect_type".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    nested_schema: None,
                    array_item_schema: None,
                },
                FieldSchema {
                    name: "target".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    nested_schema: None,
                    array_item_schema: None,
                },
            ],
            allow_additional_fields: true,
        }
    }

    /// Schema for list response.
    pub fn list_schema(item_schema: ContractSchema) -> ContractSchema {
        ContractSchema {
            fields: vec![FieldSchema {
                name: "items".to_string(),
                field_type: "array".to_string(),
                required: true,
                nested_schema: None,
                array_item_schema: Some(Box::new(FieldSchema {
                    name: "item".to_string(),
                    field_type: "object".to_string(),
                    required: true,
                    nested_schema: Some(Box::new(item_schema)),
                    array_item_schema: None,
                })),
            }],
            allow_additional_fields: true,
        }
    }

    /// Schema for error response.
    pub fn error_schema() -> ContractSchema {
        ContractSchema {
            fields: vec![FieldSchema {
                name: "error".to_string(),
                field_type: "string".to_string(),
                required: true,
                nested_schema: None,
                array_item_schema: None,
            }],
            allow_additional_fields: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_contract_validator_success() {
        let validator = ContractValidator::new();

        let schema = ContractSchema {
            fields: vec![
                FieldSchema {
                    name: "id".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    nested_schema: None,
                    array_item_schema: None,
                },
                FieldSchema {
                    name: "count".to_string(),
                    field_type: "number".to_string(),
                    required: true,
                    nested_schema: None,
                    array_item_schema: None,
                },
            ],
            allow_additional_fields: false,
        };

        let contract = ResponseContract {
            status_code: 200,
            schema,
            required_headers: vec!["content-type".to_string()],
        };

        let response_body = json!({
            "id": "test-123",
            "count": 42
        });

        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let result =
            validator.validate_response("test_success", 200, &response_body, &headers, &contract);

        assert!(result.passed);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_contract_validator_missing_field() {
        let validator = ContractValidator::new();

        let schema = ContractSchema {
            fields: vec![FieldSchema {
                name: "required_field".to_string(),
                field_type: "string".to_string(),
                required: true,
                nested_schema: None,
                array_item_schema: None,
            }],
            allow_additional_fields: false,
        };

        let contract = ResponseContract {
            status_code: 200,
            schema,
            required_headers: vec![],
        };

        let response_body = json!({});

        let result = validator.validate_response(
            "test_missing",
            200,
            &response_body,
            &HashMap::new(),
            &contract,
        );

        assert!(!result.passed);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].contains("required_field"));
    }

    #[test]
    fn test_contract_validator_type_mismatch() {
        let validator = ContractValidator::new();

        let schema = ContractSchema {
            fields: vec![FieldSchema {
                name: "value".to_string(),
                field_type: "number".to_string(),
                required: true,
                nested_schema: None,
                array_item_schema: None,
            }],
            allow_additional_fields: false,
        };

        let contract = ResponseContract {
            status_code: 200,
            schema,
            required_headers: vec![],
        };

        let response_body = json!({
            "value": "not_a_number"
        });

        let result = validator.validate_response(
            "test_type",
            200,
            &response_body,
            &HashMap::new(),
            &contract,
        );

        assert!(!result.passed);
        assert!(result.errors.iter().any(|e| e.contains("Type mismatch")));
    }

    #[test]
    fn test_contract_validator_status_mismatch() {
        let validator = ContractValidator::new();

        let schema = ContractSchema {
            fields: vec![],
            allow_additional_fields: true,
        };

        let contract = ResponseContract {
            status_code: 200,
            schema,
            required_headers: vec![],
        };

        let response_body = json!({});

        let result = validator.validate_response(
            "test_status",
            404,
            &response_body,
            &HashMap::new(),
            &contract,
        );

        assert!(!result.passed);
        assert!(result.errors.iter().any(|e| e.contains("Status code")));
    }

    #[test]
    fn test_contract_validator_nested_schema() {
        let validator = ContractValidator::new();

        let nested_schema = ContractSchema {
            fields: vec![FieldSchema {
                name: "nested_value".to_string(),
                field_type: "string".to_string(),
                required: true,
                nested_schema: None,
                array_item_schema: None,
            }],
            allow_additional_fields: false,
        };

        let schema = ContractSchema {
            fields: vec![FieldSchema {
                name: "nested".to_string(),
                field_type: "object".to_string(),
                required: true,
                nested_schema: Some(Box::new(nested_schema)),
                array_item_schema: None,
            }],
            allow_additional_fields: false,
        };

        let contract = ResponseContract {
            status_code: 200,
            schema,
            required_headers: vec![],
        };

        let response_body = json!({
            "nested": {
                "nested_value": "test"
            }
        });

        let result = validator.validate_response(
            "test_nested",
            200,
            &response_body,
            &HashMap::new(),
            &contract,
        );

        assert!(result.passed);
    }

    #[test]
    fn test_statute_schema() {
        let schema = schemas::statute_schema();
        assert_eq!(schema.fields.len(), 3);
        assert!(schema.allow_additional_fields);
    }

    #[test]
    fn test_error_schema() {
        let schema = schemas::error_schema();
        assert_eq!(schema.fields.len(), 1);
        assert!(!schema.allow_additional_fields);
    }
}
