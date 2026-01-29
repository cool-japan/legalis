//! Islamic Law (Sharia) Integration for Saudi Arabia
//!
//! Saudi Arabia follows the **Hanbali school** (المذهب الحنبلي) of Islamic jurisprudence
//! as the primary madhab, though reference is made to other schools.
//!
//! This module covers:
//! - Commercial transactions (المعاملات)
//! - Family law (الأحوال الشخصية)
//! - Inheritance (المواريث)

pub mod commercial_transactions;
pub mod family_law;
pub mod inheritance;

pub use commercial_transactions::{
    CommercialTransaction, CommercialTransactionError, CommercialTransactionResult,
    IslamicFinanceContract, check_sharia_compliance,
};

pub use family_law::{
    DivorceType, FamilyLaw, FamilyLawError, FamilyLawResult, GuardianshipType, MarriageContract,
};

pub use inheritance::{HeritageRelationship, InheritanceShare, calculate_inheritance};

use serde::{Deserialize, Serialize};

/// Hanbali jurisprudence principles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HanbaliPrinciple {
    /// Strict adherence to Quran and Sunnah
    StrictTextualAdherence,
    /// Limited use of analogy (Qiyas)
    LimitedQiyas,
    /// Preference for hadith over other sources
    HadithPreference,
    /// Rejection of speculative reasoning
    RejectionOfSpeculation,
    /// Strict prohibition of interest (Riba)
    StrictRibaProhibition,
}

impl HanbaliPrinciple {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::StrictTextualAdherence => "التمسك بالنص",
            Self::LimitedQiyas => "تحديد القياس",
            Self::HadithPreference => "تفضيل الحديث",
            Self::RejectionOfSpeculation => "رفض الرأي المحض",
            Self::StrictRibaProhibition => "تحريم الربا",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::StrictTextualAdherence => "Strict Textual Adherence",
            Self::LimitedQiyas => "Limited Analogy",
            Self::HadithPreference => "Hadith Preference",
            Self::RejectionOfSpeculation => "Rejection of Speculation",
            Self::StrictRibaProhibition => "Strict Riba Prohibition",
        }
    }

    /// Get description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::StrictTextualAdherence => {
                "Hanbali school emphasizes strict adherence to Quranic and Hadith texts"
            }
            Self::LimitedQiyas => "Limited use of analogical reasoning compared to other schools",
            Self::HadithPreference => {
                "Preference given to hadith reports over other juristic methods"
            }
            Self::RejectionOfSpeculation => "Rejection of purely speculative reasoning (Ra'y)",
            Self::StrictRibaProhibition => "Very strict interpretation of interest prohibition",
        }
    }
}

/// Get Hanbali jurisprudence principles
pub fn get_hanbali_principles() -> Vec<HanbaliPrinciple> {
    vec![
        HanbaliPrinciple::StrictTextualAdherence,
        HanbaliPrinciple::LimitedQiyas,
        HanbaliPrinciple::HadithPreference,
        HanbaliPrinciple::RejectionOfSpeculation,
        HanbaliPrinciple::StrictRibaProhibition,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanbali_principles() {
        let principles = get_hanbali_principles();
        assert!(!principles.is_empty());
        assert_eq!(principles.len(), 5);
    }

    #[test]
    fn test_principle_names() {
        let principle = HanbaliPrinciple::StrictTextualAdherence;
        assert!(principle.name_ar().contains("النص"));
        assert!(principle.name_en().contains("Textual"));
    }

    #[test]
    fn test_principle_descriptions() {
        let principle = HanbaliPrinciple::StrictRibaProhibition;
        let desc = principle.description_en();
        assert!(desc.contains("interest"));
    }
}
