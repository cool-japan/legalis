//! Contract Template Error Types (契約テンプレートエラー型)
//!
//! Error types for contract template generation operations.

use thiserror::Error;

/// Contract template errors (契約テンプレートエラー)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum TemplateError {
    /// Template not found
    /// (テンプレート未発見 - Tenpurēto mikakken)
    #[error("Template not found: {template_id}")]
    TemplateNotFound { template_id: String },

    /// Missing required variable
    /// (必須変数欠如 - Hissu hensū ketsujō)
    #[error("Missing required variable(s): {variables:?}")]
    MissingRequiredVariables { variables: Vec<String> },

    /// Missing single variable
    /// (変数欠如 - Hensū ketsujō)
    #[error("Missing required variable: {variable}")]
    MissingVariable { variable: String },

    /// Invalid variable value
    /// (無効な変数値 - Mukō na hensū chi)
    #[error("Invalid variable value for '{variable}': {reason}")]
    InvalidVariableValue { variable: String, reason: String },

    /// Template parsing error
    /// (テンプレートパースエラー - Tenpurēto pāsu erā)
    #[error("Failed to parse template: {reason}")]
    TemplateParsingError { reason: String },

    /// Template rendering error
    /// (テンプレートレンダリングエラー - Tenpurēto rendaringu erā)
    #[error("Failed to render template: {reason}")]
    TemplateRenderingError { reason: String },

    /// Clause not found
    /// (条項未発見 - Jōkō mikakken)
    #[error("Clause not found: {clause_id}")]
    ClauseNotFound { clause_id: String },

    /// Invalid template structure
    /// (無効なテンプレート構造 - Mukō na tenpurēto kōzō)
    #[error("Invalid template structure: {reason}")]
    InvalidTemplateStructure { reason: String },

    /// Conflicting clauses
    /// (条項の競合 - Jōkō no kyōgō)
    #[error("Conflicting clauses: {clause1} and {clause2}")]
    ConflictingClauses { clause1: String, clause2: String },

    /// Invalid date format
    /// (無効な日付形式 - Mukō na hizuke keishiki)
    #[error("Invalid date format for '{field}': expected YYYY-MM-DD")]
    InvalidDateFormat { field: String },

    /// Template validation error
    /// (テンプレート検証エラー - Tenpurēto kenshō erā)
    #[error("Template validation failed: {reason}")]
    ValidationError { reason: String },

    /// Contract validation failed
    /// (契約検証失敗 - Keiyaku kenshō shippai)
    #[error("Contract validation failed: {reason}")]
    ValidationFailed { reason: String },

    /// Generic error
    /// (汎用エラー - Han'yō erā)
    #[error("Template error: {message}")]
    GenericError { message: String },
}

/// Result type for template operations
pub type Result<T> = std::result::Result<T, TemplateError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = TemplateError::TemplateNotFound {
            template_id: "employment_001".to_string(),
        };
        assert!(error.to_string().contains("employment_001"));

        let error = TemplateError::MissingRequiredVariables {
            variables: vec!["employee_name".to_string(), "start_date".to_string()],
        };
        assert!(error.to_string().contains("employee_name"));
        assert!(error.to_string().contains("start_date"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = TemplateError::TemplateNotFound {
            template_id: "test".to_string(),
        };
        let error2 = TemplateError::TemplateNotFound {
            template_id: "test".to_string(),
        };
        assert_eq!(error1, error2);
    }
}
