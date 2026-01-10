//! Consumer Protection - Error Types
//!
//! This module defines error types for Singapore consumer protection violations with
//! trilingual messages in Singapore's primary languages:
//! - English (business and administration)
//! - Chinese/华语 (Chinese community, 74% of population)
//! - Malay/Bahasa Melayu (national language, 13% of population)
//!
//! Tamil support is optional as it's less commonly used in legal/business contexts.

use thiserror::Error;

/// Result type for consumer protection operations
pub type Result<T> = std::result::Result<T, ConsumerError>;

/// Consumer protection error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ConsumerError {
    /// Unfair practice detected (CPFTA s. 4-7)
    #[error(
        "Unfair practice detected: {practice_type:?} (CPFTA {statute})\n\
         检测到不公平交易: {practice_type:?} (消费者保护(公平交易)法{statute}条)"
    )]
    UnfairPractice {
        practice_type: String,
        statute: String,
    },

    /// Goods not merchantable (SOGA s. 14(2))
    #[error(
        "Goods are not of merchantable quality (SOGA s. 14(2))\n\
         商品不符合可售品质标准 (买卖法第14(2)条)\n\
         Barangan tidak mempunyai kualiti yang boleh dijual (SOGA s. 14(2))"
    )]
    NotMerchantable { description: String },

    /// Goods do not correspond to description (SOGA s. 13)
    #[error(
        "Goods do not correspond to description (SOGA s. 13)\n\
         商品与描述不符 (买卖法第13条)"
    )]
    DescriptionMismatch { description: String },

    /// Goods not fit for particular purpose (SOGA s. 14(3))
    #[error(
        "Goods not fit for particular purpose: {purpose} (SOGA s. 14(3))\n\
         商品不适合特定用途: {purpose} (买卖法第14(3)条)"
    )]
    NotFitForPurpose { purpose: String },

    /// Sale by sample mismatch (SOGA s. 15)
    #[error(
        "Goods do not correspond with sample (SOGA s. 15)\n\
         商品与样品不符 (买卖法第15条)"
    )]
    SampleMismatch,

    /// Defect discovered (Lemon Law)
    #[error(
        "Defect discovered within 6 months: {description} (Lemon Law)\n\
         6个月内发现缺陷: {description} (柠檬法)\n\
         Kecacatan ditemui dalam tempoh 6 bulan: {description} (Undang-undang Lemon)"
    )]
    DefectDiscovered { description: String },

    /// False representation (CPFTA s. 4)
    #[error(
        "False or misleading representation (CPFTA s. 4)\n\
         虚假或误导性陈述 (消费者保护(公平交易)法第4条)"
    )]
    FalseRepresentation { details: String },

    /// Unconscionable conduct (CPFTA s. 5)
    #[error(
        "Unconscionable conduct detected (CPFTA s. 5)\n\
         检测到不合情理行为 (消费者保护(公平交易)法第5条)"
    )]
    UnconscionableConduct { details: String },

    /// Bait advertising (CPFTA s. 6)
    #[error(
        "Bait advertising detected (CPFTA s. 6)\n\
         检测到诱饵广告 (消费者保护(公平交易)法第6条)"
    )]
    BaitAdvertising { details: String },

    /// Harassment or coercion (CPFTA s. 7)
    #[error(
        "Harassment or coercion detected (CPFTA s. 7)\n\
         检测到骚扰或胁迫 (消费者保护(公平交易)法第7条)"
    )]
    Harassment { details: String },

    /// Exceeds Small Claims Tribunal limit
    #[error(
        "Claim amount SGD {amount} exceeds SCT limit of SGD 20,000\n\
         索赔金额新币{amount}元超过小额索偿庭限额新币20,000元"
    )]
    ExceedsSctLimit { amount: u64 },

    /// Warranty expired
    #[error(
        "Warranty expired (expired {days_ago} days ago)\n\
         保修期已过期 ({days_ago}天前过期)"
    )]
    WarrantyExpired { days_ago: u32 },

    /// Contractterm potentially unfair
    #[error(
        "Contract term potentially unfair: {term_description}\n\
         合同条款可能不公平: {term_description}"
    )]
    UnfairTerm { term_description: String },

    /// High risk contract
    #[error(
        "Contract has high risk score: {risk_score}/100\n\
         合同风险评分过高: {risk_score}/100"
    )]
    HighRiskContract { risk_score: u32 },

    /// Generic validation error
    #[error(
        "Consumer protection validation error: {message}\n\
         消费者保护验证错误: {message}"
    )]
    ValidationError { message: String },
}

impl ConsumerError {
    /// Returns the statute reference
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            ConsumerError::UnfairPractice { .. } => Some("CPFTA s. 4-7"),
            ConsumerError::NotMerchantable { .. } => Some("SOGA s. 14(2)"),
            ConsumerError::DescriptionMismatch { .. } => Some("SOGA s. 13"),
            ConsumerError::NotFitForPurpose { .. } => Some("SOGA s. 14(3)"),
            ConsumerError::SampleMismatch => Some("SOGA s. 15"),
            ConsumerError::DefectDiscovered { .. } => Some("Lemon Law"),
            ConsumerError::FalseRepresentation { .. } => Some("CPFTA s. 4"),
            ConsumerError::UnconscionableConduct { .. } => Some("CPFTA s. 5"),
            ConsumerError::BaitAdvertising { .. } => Some("CPFTA s. 6"),
            ConsumerError::Harassment { .. } => Some("CPFTA s. 7"),
            _ => None,
        }
    }

    /// Returns severity level (1-5)
    pub fn severity(&self) -> u8 {
        match self {
            ConsumerError::UnfairPractice { .. } => 4,
            ConsumerError::NotMerchantable { .. } => 3,
            ConsumerError::DescriptionMismatch { .. } => 3,
            ConsumerError::NotFitForPurpose { .. } => 3,
            ConsumerError::SampleMismatch => 3,
            ConsumerError::DefectDiscovered { .. } => 3,
            ConsumerError::FalseRepresentation { .. } => 4,
            ConsumerError::UnconscionableConduct { .. } => 5,
            ConsumerError::BaitAdvertising { .. } => 4,
            ConsumerError::Harassment { .. } => 5,
            ConsumerError::ExceedsSctLimit { .. } => 2,
            ConsumerError::WarrantyExpired { .. } => 2,
            ConsumerError::UnfairTerm { .. } => 3,
            ConsumerError::HighRiskContract { .. } => 4,
            ConsumerError::ValidationError { .. } => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statute_reference() {
        let error = ConsumerError::NotMerchantable {
            description: "Defective".to_string(),
        };
        assert_eq!(error.statute_reference(), Some("SOGA s. 14(2)"));

        let error2 = ConsumerError::FalseRepresentation {
            details: "Misleading ad".to_string(),
        };
        assert_eq!(error2.statute_reference(), Some("CPFTA s. 4"));
    }

    #[test]
    fn test_error_severity() {
        let error1 = ConsumerError::UnconscionableConduct {
            details: "Took advantage".to_string(),
        };
        assert_eq!(error1.severity(), 5);

        let error2 = ConsumerError::WarrantyExpired { days_ago: 30 };
        assert_eq!(error2.severity(), 2);
    }

    #[test]
    fn test_error_display() {
        let error = ConsumerError::DefectDiscovered {
            description: "Battery fails to charge".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("Defect discovered within 6 months"));
        assert!(display.contains("Battery fails to charge"));
        assert!(display.contains("Lemon Law"));
    }
}
