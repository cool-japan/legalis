//! Cross-border Data Transfer Module
//!
//! # 数据出境 / Cross-border Data Transfer
//!
//! Implements requirements for cross-border transfer of personal information
//! under PIPL Articles 38-43 and CAC Data Export Security Assessment Measures.
//!
//! ## Transfer Mechanisms
//!
//! 1. CAC Security Assessment (安全评估)
//! 2. Standard Contract with CAC filing (标准合同)
//! 3. Personal Information Protection Certification (认证)

#![allow(missing_docs)]
//!
//! ## When Security Assessment Required
//!
//! - CII operators
//! - Processing PI of 1M+ individuals
//! - Cumulative cross-border of 100k+ individuals since Jan 1 of previous year
//! - Cumulative cross-border of 10k+ sensitive PI individuals

use crate::i18n::BilingualText;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::error::{PiplError, PiplResult};
use super::types::*;

/// Cross-border transfer mechanism
///
/// # 数据出境机制
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferMechanism {
    /// CAC Security Assessment (安全评估)
    /// Required for CII operators and large-scale processors
    SecurityAssessment {
        /// Assessment application date
        application_date: NaiveDate,
        /// Assessment approval date
        approval_date: Option<NaiveDate>,
        /// Assessment validity (2 years)
        valid_until: Option<NaiveDate>,
        /// Assessment result
        result: Option<AssessmentResult>,
    },

    /// Standard Contract with CAC Filing (标准合同备案)
    /// For non-CII operators meeting thresholds
    StandardContract {
        /// Contract signing date
        signing_date: NaiveDate,
        /// Filing date with CAC
        filing_date: Option<NaiveDate>,
        /// Filing number
        filing_number: Option<String>,
    },

    /// Personal Information Protection Certification (认证)
    CertificationMechanism {
        /// Certification body
        certifier: String,
        /// Certification date
        certification_date: NaiveDate,
        /// Certification validity
        valid_until: NaiveDate,
        /// Certification number
        certificate_number: String,
    },

    /// International Treaty or Agreement
    /// Between PRC and recipient country
    InternationalAgreement {
        /// Treaty/agreement name
        agreement_name: String,
        /// Recipient country
        country: String,
    },

    /// Individual Consent (for limited scenarios)
    IndividualConsent {
        /// Consent obtained
        consent_obtained: bool,
        /// Information provided about overseas recipient
        recipient_info_provided: bool,
    },
}

impl TransferMechanism {
    /// Get Chinese name
    pub fn name_zh(&self) -> &str {
        match self {
            Self::SecurityAssessment { .. } => "安全评估",
            Self::StandardContract { .. } => "标准合同",
            Self::CertificationMechanism { .. } => "个人信息保护认证",
            Self::InternationalAgreement { .. } => "国际条约或协定",
            Self::IndividualConsent { .. } => "个人同意",
        }
    }

    /// Check if mechanism is valid/active
    pub fn is_valid(&self, as_of: NaiveDate) -> bool {
        match self {
            Self::SecurityAssessment {
                valid_until,
                result,
                ..
            } => {
                result.as_ref().is_some_and(|r| r.is_approved())
                    && valid_until.is_some_and(|d| d >= as_of)
            }
            Self::StandardContract { filing_date, .. } => filing_date.is_some(),
            Self::CertificationMechanism { valid_until, .. } => *valid_until >= as_of,
            Self::InternationalAgreement { .. } => true,
            Self::IndividualConsent {
                consent_obtained,
                recipient_info_provided,
            } => *consent_obtained && *recipient_info_provided,
        }
    }
}

/// Security assessment result
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssessmentResult {
    /// 通过 / Approved
    Approved,
    /// 附条件通过 / Approved with Conditions
    ApprovedWithConditions { conditions: Vec<String> },
    /// 不通过 / Rejected
    Rejected { reasons: Vec<String> },
    /// 待审 / Pending
    Pending,
}

impl AssessmentResult {
    pub fn is_approved(&self) -> bool {
        matches!(self, Self::Approved | Self::ApprovedWithConditions { .. })
    }
}

/// Cross-border transfer record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossBorderTransferRecord {
    /// Transfer ID
    pub id: String,
    /// Domestic handler
    pub handler: String,
    /// Overseas recipient
    pub recipient: OverseasRecipient,
    /// Transfer mechanism used
    pub mechanism: TransferMechanism,
    /// PI categories transferred
    pub pi_categories: Vec<PersonalInformationCategory>,
    /// Sensitive PI categories
    pub sensitive_categories: Vec<SensitivePersonalInformation>,
    /// Number of individuals affected
    pub individuals_count: u64,
    /// Transfer purpose
    pub purpose: String,
    /// Transfer start date
    pub start_date: NaiveDate,
    /// Individual notification provided
    pub individual_notified: bool,
    /// Separate consent obtained (if required)
    pub separate_consent: bool,
}

/// Overseas recipient information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverseasRecipient {
    /// Recipient name
    pub name: BilingualText,
    /// Country/region
    pub country: String,
    /// Contact information
    pub contact: String,
    /// Processing purpose
    pub purpose: String,
    /// Recipient's data protection measures
    pub protection_measures: Vec<String>,
    /// Contractual commitments
    pub commitments: Vec<String>,
}

/// Determine required transfer mechanism
pub fn determine_transfer_mechanism(
    is_cii_operator: bool,
    total_individuals: u64,
    cross_border_count: u64,
    sensitive_cross_border_count: u64,
) -> RequiredMechanism {
    // CII operators must use security assessment
    if is_cii_operator {
        return RequiredMechanism::SecurityAssessmentRequired {
            reason: BilingualText::new(
                "关键信息基础设施运营者必须进行安全评估",
                "CII operators must undergo security assessment",
            ),
        };
    }

    // Large-scale processors (1M+ individuals)
    if total_individuals >= 1_000_000 {
        return RequiredMechanism::SecurityAssessmentRequired {
            reason: BilingualText::new(
                "处理100万人以上个人信息的处理者必须进行安全评估",
                "Processors handling PI of 1M+ individuals must undergo security assessment",
            ),
        };
    }

    // Cumulative cross-border 100k+ individuals
    if cross_border_count >= 100_000 {
        return RequiredMechanism::SecurityAssessmentRequired {
            reason: BilingualText::new(
                "累计向境外提供10万人以上个人信息的必须进行安全评估",
                "Cumulative cross-border transfer of 100k+ individuals requires security assessment",
            ),
        };
    }

    // Sensitive PI cross-border 10k+
    if sensitive_cross_border_count >= 10_000 {
        return RequiredMechanism::SecurityAssessmentRequired {
            reason: BilingualText::new(
                "累计向境外提供1万人以上敏感个人信息的必须进行安全评估",
                "Cumulative cross-border of 10k+ sensitive PI requires security assessment",
            ),
        };
    }

    // Standard contract allowed
    RequiredMechanism::StandardContractAllowed {
        note: BilingualText::new(
            "可以采用标准合同方式并向网信部门备案",
            "Standard contract with CAC filing is permitted",
        ),
    }
}

/// Required mechanism determination result
#[derive(Debug, Clone)]
pub enum RequiredMechanism {
    /// Security assessment required
    SecurityAssessmentRequired { reason: BilingualText },
    /// Standard contract allowed
    StandardContractAllowed { note: BilingualText },
    /// Certification allowed
    CertificationAllowed { note: BilingualText },
}

impl RequiredMechanism {
    /// Check if security assessment is required
    pub fn requires_security_assessment(&self) -> bool {
        matches!(self, Self::SecurityAssessmentRequired { .. })
    }
}

/// Validate cross-border transfer compliance
pub fn validate_cross_border_transfer(
    transfer: &CrossBorderTransferRecord,
    handler_is_cii: bool,
    total_individuals: u64,
) -> PiplResult<()> {
    // Check if individual was notified (Article 39)
    if !transfer.individual_notified {
        return Err(PiplError::CrossBorderViolation {
            violation: "个人未被告知境外接收方信息 / Individual not notified of overseas recipient"
                .to_string(),
        });
    }

    // Check separate consent for cross-border (Article 39)
    if !transfer.separate_consent {
        return Err(PiplError::CrossBorderViolation {
            violation: "未取得个人单独同意 / Separate consent not obtained".to_string(),
        });
    }

    // Determine required mechanism
    let required = determine_transfer_mechanism(
        handler_is_cii,
        total_individuals,
        transfer.individuals_count,
        transfer.sensitive_categories.len() as u64 * transfer.individuals_count,
    );

    // Validate mechanism matches requirements
    if required.requires_security_assessment() {
        if !matches!(
            transfer.mechanism,
            TransferMechanism::SecurityAssessment { .. }
        ) {
            return Err(PiplError::SecurityAssessmentRequired {
                individuals: total_individuals,
            });
        }

        // Check if assessment is valid
        let today = chrono::Utc::now().date_naive();
        if !transfer.mechanism.is_valid(today) {
            return Err(PiplError::CrossBorderViolation {
                violation: "安全评估未通过或已过期 / Security assessment not approved or expired"
                    .to_string(),
            });
        }
    }

    // Check recipient protection measures
    if transfer.recipient.protection_measures.is_empty() {
        return Err(PiplError::OverseasRecipientViolation {
            recipient: transfer.recipient.name.zh.clone(),
        });
    }

    Ok(())
}

/// Standard contract key clauses (Article 38)
pub fn standard_contract_required_clauses() -> Vec<BilingualText> {
    vec![
        BilingualText::new(
            "境外接收方处理个人信息的目的、方式",
            "Purpose and method of overseas recipient's PI processing",
        ),
        BilingualText::new(
            "境外接收方处理个人信息的类型和保存期限",
            "Types of PI and retention period",
        ),
        BilingualText::new(
            "个人行使权利的方式和程序",
            "Methods and procedures for individuals to exercise rights",
        ),
        BilingualText::new(
            "境外接收方保护个人信息的技术和管理措施",
            "Technical and management measures for PI protection",
        ),
        BilingualText::new(
            "发生个人信息安全事件的处理措施",
            "Measures for handling PI security incidents",
        ),
        BilingualText::new(
            "个人信息保护影响评估情况",
            "PI protection impact assessment results",
        ),
        BilingualText::new(
            "境外接收方所在国家或地区的法律对合同履行的影响",
            "Impact of recipient country laws on contract performance",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_mechanism_cii() {
        let result = determine_transfer_mechanism(true, 10_000, 1_000, 100);
        assert!(result.requires_security_assessment());
    }

    #[test]
    fn test_determine_mechanism_large_processor() {
        let result = determine_transfer_mechanism(false, 2_000_000, 50_000, 5_000);
        assert!(result.requires_security_assessment());
    }

    #[test]
    fn test_determine_mechanism_standard_contract() {
        let result = determine_transfer_mechanism(false, 500_000, 50_000, 5_000);
        assert!(!result.requires_security_assessment());
    }

    #[test]
    fn test_transfer_mechanism_validity() {
        let today = chrono::Utc::now().date_naive();
        let future = today + chrono::Duration::days(365);

        let mechanism = TransferMechanism::SecurityAssessment {
            application_date: today - chrono::Duration::days(30),
            approval_date: Some(today - chrono::Duration::days(10)),
            valid_until: Some(future),
            result: Some(AssessmentResult::Approved),
        };

        assert!(mechanism.is_valid(today));
    }

    #[test]
    fn test_standard_contract_clauses() {
        let clauses = standard_contract_required_clauses();
        assert_eq!(clauses.len(), 7);
        assert!(clauses[0].zh.contains("目的"));
    }
}
