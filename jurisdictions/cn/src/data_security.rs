//! Data Security Law Module (数据安全法)
//!
//! # 中华人民共和国数据安全法 / Data Security Law of the PRC
//!
//! Implements the Data Security Law effective September 1, 2021.
//!
//! ## Key Concepts
//!
//! - **数据 (Data)**: Records in electronic or other forms
//! - **数据处理 (Data Processing)**: Collection, storage, use, processing, transmission, provision, disclosure
//! - **数据安全 (Data Security)**: Measures to ensure effective protection and lawful use
//! - **重要数据 (Important Data)**: Data affecting national security, economy, public interest
//! - **核心数据 (Core Data)**: Data critical to national security
//!
//! ## Data Classification System
//!
//! Article 21: National data classification and hierarchical protection system
//!
//! ### Classification Levels
//!
//! 1. **Core Data (核心数据)**: Critical to national security
//! 2. **Important Data (重要数据)**: Affecting security, economy, public interest
//! 3. **General Data (一般数据)**: Other data
//!
//! ## Cross-Border Transfer
//!
//! Article 31: Important data cross-border transfer requires security assessment
//!
//! ## National Security Provisions
//!
//! Article 24-26: Data security review mechanism for activities affecting national security

use crate::i18n::BilingualText;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Types
// ============================================================================

/// Data classification level (数据分类级别)
///
/// Article 21
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DataClassification {
    /// General data (一般数据)
    General,
    /// Important data (重要数据)
    Important,
    /// Core data (核心数据)
    Core,
}

impl DataClassification {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::General => BilingualText::new("一般数据", "General data"),
            Self::Important => BilingualText::new("重要数据", "Important data"),
            Self::Core => BilingualText::new("核心数据", "Core data"),
        }
    }

    /// Check if requires security assessment for cross-border transfer
    ///
    /// Article 31
    pub fn requires_cross_border_assessment(&self) -> bool {
        matches!(self, Self::Important | Self::Core)
    }

    /// Check if requires security review
    ///
    /// Articles 24-26
    pub fn requires_security_review(&self) -> bool {
        matches!(self, Self::Core)
    }
}

/// Data processing activity (数据处理活动)
///
/// Article 3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataProcessingActivity {
    /// Collection (收集)
    Collection,
    /// Storage (存储)
    Storage,
    /// Use (使用)
    Use,
    /// Processing (加工)
    Processing,
    /// Transmission (传输)
    Transmission,
    /// Provision (提供)
    Provision,
    /// Disclosure (公开)
    Disclosure,
}

impl DataProcessingActivity {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Collection => BilingualText::new("收集", "Collection"),
            Self::Storage => BilingualText::new("存储", "Storage"),
            Self::Use => BilingualText::new("使用", "Use"),
            Self::Processing => BilingualText::new("加工", "Processing"),
            Self::Transmission => BilingualText::new("传输", "Transmission"),
            Self::Provision => BilingualText::new("提供", "Provision"),
            Self::Disclosure => BilingualText::new("公开", "Disclosure"),
        }
    }
}

/// Data processor (数据处理者)
///
/// Article 27
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessor {
    /// Name
    pub name: String,
    /// Organization type
    pub organization_type: BilingualText,
    /// Data security officer appointed
    pub has_security_officer: bool,
    /// Security officer name
    pub security_officer: Option<String>,
}

/// Data processing record (数据处理记录)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessingRecord {
    /// Processor
    pub processor: String,
    /// Activity type
    pub activity: DataProcessingActivity,
    /// Data classification
    pub data_classification: DataClassification,
    /// Data description
    pub data_description: BilingualText,
    /// Processing date
    pub processing_date: DateTime<Utc>,
    /// Purpose
    pub purpose: BilingualText,
    /// Legal basis
    pub legal_basis: BilingualText,
}

/// Cross-border data transfer (数据出境)
///
/// Article 31
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossBorderDataTransfer {
    /// Data exporter (Chinese entity)
    pub exporter: String,
    /// Recipient (overseas entity)
    pub recipient: String,
    /// Recipient jurisdiction
    pub recipient_jurisdiction: String,
    /// Data classification
    pub data_classification: DataClassification,
    /// Data description
    pub data_description: BilingualText,
    /// Transfer date
    pub transfer_date: DateTime<Utc>,
    /// Security assessment completed
    pub security_assessment_completed: bool,
    /// Assessment date
    pub assessment_date: Option<DateTime<Utc>>,
}

impl CrossBorderDataTransfer {
    /// Check if security assessment is required
    ///
    /// Article 31
    pub fn requires_security_assessment(&self) -> bool {
        self.data_classification.requires_cross_border_assessment()
    }

    /// Check if transfer is compliant
    pub fn is_compliant(&self) -> bool {
        if self.requires_security_assessment() {
            self.security_assessment_completed
        } else {
            true
        }
    }
}

/// Data security review (数据安全审查)
///
/// Articles 24-26
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSecurityReview {
    /// Entity under review
    pub entity: String,
    /// Activity description
    pub activity: BilingualText,
    /// Review initiated date
    pub review_initiated: DateTime<Utc>,
    /// Review status
    pub status: ReviewStatus,
    /// Review result
    pub result: Option<ReviewResult>,
}

/// Review status (审查状态)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    /// Pending (待审查)
    Pending,
    /// Under review (审查中)
    UnderReview,
    /// Completed (已完成)
    Completed,
}

impl ReviewStatus {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Pending => BilingualText::new("待审查", "Pending"),
            Self::UnderReview => BilingualText::new("审查中", "Under review"),
            Self::Completed => BilingualText::new("已完成", "Completed"),
        }
    }
}

/// Review result (审查结果)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewResult {
    /// Approved (批准)
    Approved,
    /// Approved with conditions (附条件批准)
    ApprovedWithConditions,
    /// Denied (禁止)
    Denied,
}

impl ReviewResult {
    /// Get bilingual description
    pub fn description(&self) -> BilingualText {
        match self {
            Self::Approved => BilingualText::new("批准", "Approved"),
            Self::ApprovedWithConditions => {
                BilingualText::new("附条件批准", "Approved with conditions")
            }
            Self::Denied => BilingualText::new("禁止", "Denied"),
        }
    }
}

/// Data security obligation (数据安全义务)
///
/// Articles 27-29
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSecurityObligation {
    /// Obligation description
    pub description: BilingualText,
    /// Applicable to data classification
    pub applicable_classification: DataClassification,
    /// Legal basis (article)
    pub legal_basis: String,
}

// ============================================================================
// Validators
// ============================================================================

/// Validate data processor compliance
///
/// Article 27: Data processors must establish data security management system
pub fn validate_data_processor(processor: &DataProcessor) -> Result<(), DataSecurityError> {
    // Article 27: Data processors must appoint security officer
    if !processor.has_security_officer {
        return Err(DataSecurityError::NoSecurityOfficer {
            processor: processor.name.clone(),
        });
    }

    Ok(())
}

/// Validate cross-border data transfer
///
/// Article 31
pub fn validate_cross_border_transfer(
    transfer: &CrossBorderDataTransfer,
) -> Result<(), DataSecurityError> {
    // Check if security assessment is required and completed
    if transfer.requires_security_assessment() && !transfer.security_assessment_completed {
        return Err(DataSecurityError::SecurityAssessmentRequired {
            exporter: transfer.exporter.clone(),
            recipient: transfer.recipient.clone(),
            data_classification: transfer.data_classification.description(),
        });
    }

    Ok(())
}

/// Check if data security review is required
///
/// Articles 24-26
pub fn requires_security_review(
    data_classification: DataClassification,
    affects_national_security: bool,
) -> bool {
    // Article 24: Review required for activities affecting national security
    data_classification.requires_security_review() || affects_national_security
}

/// Determine data classification
///
/// Article 21: Classification based on impact on national security, economy, public interest
pub fn determine_data_classification(
    affects_national_security: bool,
    critical_to_national_security: bool,
    large_scale: bool,
    sector: Option<&str>,
) -> DataClassification {
    if critical_to_national_security {
        return DataClassification::Core;
    }

    if affects_national_security || large_scale {
        return DataClassification::Important;
    }

    // Critical sectors (CII, finance, etc.) may have Important Data
    if let Some(s) = sector
        && matches!(
            s,
            "finance" | "energy" | "telecommunications" | "transportation"
        )
    {
        return DataClassification::Important;
    }

    DataClassification::General
}

// ============================================================================
// Errors
// ============================================================================

/// Errors for Data Security Law
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum DataSecurityError {
    /// No security officer appointed
    #[error("Data processor {processor} must appoint data security officer")]
    NoSecurityOfficer {
        /// Processor name
        processor: String,
    },

    /// Security assessment required
    #[error(
        "Cross-border transfer from {exporter} to {recipient} requires security assessment for {data_classification}"
    )]
    SecurityAssessmentRequired {
        /// Exporter
        exporter: String,
        /// Recipient
        recipient: String,
        /// Data classification
        data_classification: BilingualText,
    },

    /// Security review required
    #[error("Data security review required: {activity}")]
    SecurityReviewRequired {
        /// Activity description
        activity: BilingualText,
    },

    /// Data classification violation
    #[error("Data classification violation: {description}")]
    ClassificationViolation {
        /// Description
        description: BilingualText,
    },
}

/// Result type for Data Security operations
pub type DataSecurityResult<T> = Result<T, DataSecurityError>;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_classification_ordering() {
        assert!(DataClassification::Core > DataClassification::Important);
        assert!(DataClassification::Important > DataClassification::General);
    }

    #[test]
    fn test_cross_border_assessment_requirements() {
        assert!(DataClassification::Core.requires_cross_border_assessment());
        assert!(DataClassification::Important.requires_cross_border_assessment());
        assert!(!DataClassification::General.requires_cross_border_assessment());
    }

    #[test]
    fn test_security_review_requirements() {
        assert!(DataClassification::Core.requires_security_review());
        assert!(!DataClassification::Important.requires_security_review());
        assert!(!DataClassification::General.requires_security_review());
    }

    #[test]
    fn test_determine_data_classification() {
        let core = determine_data_classification(true, true, false, None);
        assert_eq!(core, DataClassification::Core);

        let important = determine_data_classification(true, false, false, None);
        assert_eq!(important, DataClassification::Important);

        let general = determine_data_classification(false, false, false, None);
        assert_eq!(general, DataClassification::General);

        let financial_important =
            determine_data_classification(false, false, false, Some("finance"));
        assert_eq!(financial_important, DataClassification::Important);
    }

    #[test]
    fn test_cross_border_transfer_compliance() {
        let transfer = CrossBorderDataTransfer {
            exporter: "中国公司".to_string(),
            recipient: "Overseas Corp".to_string(),
            recipient_jurisdiction: "US".to_string(),
            data_classification: DataClassification::Important,
            data_description: BilingualText::new("重要数据", "Important data"),
            transfer_date: Utc::now(),
            security_assessment_completed: true,
            assessment_date: Some(Utc::now()),
        };

        assert!(transfer.requires_security_assessment());
        assert!(transfer.is_compliant());
        assert!(validate_cross_border_transfer(&transfer).is_ok());
    }

    #[test]
    fn test_cross_border_transfer_without_assessment() {
        let transfer = CrossBorderDataTransfer {
            exporter: "中国公司".to_string(),
            recipient: "Overseas Corp".to_string(),
            recipient_jurisdiction: "US".to_string(),
            data_classification: DataClassification::Important,
            data_description: BilingualText::new("重要数据", "Important data"),
            transfer_date: Utc::now(),
            security_assessment_completed: false,
            assessment_date: None,
        };

        assert!(!transfer.is_compliant());
        assert!(validate_cross_border_transfer(&transfer).is_err());
    }

    #[test]
    fn test_data_processor_validation() {
        let processor = DataProcessor {
            name: "某公司".to_string(),
            organization_type: BilingualText::new("有限责任公司", "Limited Liability Company"),
            has_security_officer: true,
            security_officer: Some("张三".to_string()),
        };

        assert!(validate_data_processor(&processor).is_ok());
    }

    #[test]
    fn test_data_processor_no_security_officer() {
        let processor = DataProcessor {
            name: "某公司".to_string(),
            organization_type: BilingualText::new("有限责任公司", "Limited Liability Company"),
            has_security_officer: false,
            security_officer: None,
        };

        assert!(validate_data_processor(&processor).is_err());
    }

    #[test]
    fn test_requires_security_review() {
        assert!(requires_security_review(DataClassification::Core, false));
        assert!(requires_security_review(DataClassification::General, true));
        assert!(!requires_security_review(
            DataClassification::General,
            false
        ));
    }
}
