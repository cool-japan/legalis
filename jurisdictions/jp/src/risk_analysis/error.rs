//! Risk Analysis Error Types (リスク分析エラー型)
//!
//! Error types for risk analysis operations.

use thiserror::Error;

/// Risk analysis errors (リスク分析エラー)
#[derive(Debug, Error, Clone, PartialEq)]
pub enum RiskAnalysisError {
    /// Document parsing error
    /// (文書パースエラー - Bunsho pāsu erā)
    #[error("Failed to parse document: {reason}")]
    DocumentParsingError { reason: String },

    /// Rule execution error
    /// (ルール実行エラー - Rūru jikkō erā)
    #[error("Rule execution failed: {rule_id} - {reason}")]
    RuleExecutionError { rule_id: String, reason: String },

    /// Invalid risk score
    /// (無効なリスクスコア - Mukō na risuku sukoa)
    #[error("Invalid risk score: {score} (must be 0-100)")]
    InvalidRiskScore { score: i32 },

    /// Missing required data
    /// (必須データ欠如 - Hissu dēta ketsuj)
    #[error("Missing required data: {field_name}")]
    MissingRequiredData { field_name: String },

    /// Unsupported contract type
    /// (未対応の契約種別 - Mitaiō no keiyaku shubetsu)
    #[error("Unsupported contract type: {contract_type}")]
    UnsupportedContractType { contract_type: String },

    /// Report generation error
    /// (レポート生成エラー - Repōto seisei erā)
    #[error("Failed to generate report: {reason}")]
    ReportGenerationError { reason: String },

    /// Compliance check error
    /// (コンプライアンスチェックエラー - Konpuraiansu chekku erā)
    #[error("Compliance check failed: {legal_area} - {reason}")]
    ComplianceCheckError { legal_area: String, reason: String },

    /// Configuration error
    /// (設定エラー - Settei erā)
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    /// Generic error
    /// (汎用エラー - Han'yō erā)
    #[error("Risk analysis error: {message}")]
    GenericError { message: String },
}

/// Result type for risk analysis operations
pub type Result<T> = std::result::Result<T, RiskAnalysisError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = RiskAnalysisError::InvalidRiskScore { score: 150 };
        assert!(error.to_string().contains("150"));

        let error = RiskAnalysisError::MissingRequiredData {
            field_name: "contract_text".to_string(),
        };
        assert!(error.to_string().contains("contract_text"));
    }

    #[test]
    fn test_error_equality() {
        let error1 = RiskAnalysisError::InvalidRiskScore { score: 150 };
        let error2 = RiskAnalysisError::InvalidRiskScore { score: 150 };
        assert_eq!(error1, error2);
    }
}
