//! Vietnamese Criminal Code 2015 (Bộ luật Hình sự 2015) - Law No. 100/2015/QH13
//!
//! Vietnam's comprehensive criminal law code, effective from January 1, 2018.
//! Amended by Laws 12/2017, 27/2018, and 37/2019.
//!
//! ## Structure
//!
//! The Criminal Code contains 26 chapters (Chương) and 426 articles:
//!
//! - **General Part** (Phần chung): Chapters I-XIII (Điều 1-104)
//! - **Specific Part** (Phần các tội phạm): Chapters XIV-XXVI (Điều 105-426)
//!
//! ## Penalties
//!
//! - Death penalty (Tử hình) - Article 40
//! - Life imprisonment (Tù chung thân) - Article 41
//! - Fixed-term imprisonment (Tù có thời hạn): 3 months - 20 years - Article 42
//! - Reform without detention (Cải tạo không giam giữ) - Article 43
//! - Fine (Phạt tiền) - Article 33

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Criminal penalty types (Các hình phạt) - Article 32-44
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Penalty {
    /// Death penalty (Tử hình) - Article 40
    Death,
    /// Life imprisonment (Tù chung thân) - Article 41
    LifeImprisonment,
    /// Fixed-term imprisonment (Tù có thời hạn) - Article 42
    FixedTermImprisonment {
        /// Months of imprisonment (3-240 months = 3 months to 20 years)
        months: u16,
    },
    /// Reform without detention (Cải tạo không giam giữ) - Article 43
    ReformWithoutDetention {
        /// Months of reform (6-36 months)
        months: u8,
    },
    /// Fine (Phạt tiền) - Article 33
    Fine {
        /// Amount in VND (1-1,000,000,000)
        amount: i64,
    },
}

impl Penalty {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> String {
        match self {
            Self::Death => "Tử hình".to_string(),
            Self::LifeImprisonment => "Tù chung thân".to_string(),
            Self::FixedTermImprisonment { months } => {
                format!("Tù có thời hạn {} tháng", months)
            }
            Self::ReformWithoutDetention { months } => {
                format!("Cải tạo không giam giữ {} tháng", months)
            }
            Self::Fine { amount } => format!("Phạt tiền {} VND", amount),
        }
    }

    /// Get English name
    pub fn name_en(&self) -> String {
        match self {
            Self::Death => "Death penalty".to_string(),
            Self::LifeImprisonment => "Life imprisonment".to_string(),
            Self::FixedTermImprisonment { months } => {
                format!("Imprisonment for {} months", months)
            }
            Self::ReformWithoutDetention { months } => {
                format!("Reform without detention for {} months", months)
            }
            Self::Fine { amount } => format!("Fine of {} VND", amount),
        }
    }

    /// Validate if penalty is within legal limits
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Death | Self::LifeImprisonment => true,
            Self::FixedTermImprisonment { months } => (3..=240).contains(months),
            Self::ReformWithoutDetention { months } => (6..=36).contains(months),
            Self::Fine { amount } => (1..=1_000_000_000).contains(amount),
        }
    }
}

/// Supplementary penalties (Hình phạt bổ sung) - Article 45-50
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupplementaryPenalty {
    /// Prohibition from holding certain posts (Cấm đảm nhiệm chức vụ) - Article 45
    PostProhibition {
        /// Duration in years (1-5)
        years: u8,
    },
    /// Prohibition from practicing profession (Cấm hành nghề) - Article 46
    ProfessionProhibition {
        /// Duration in years (1-5)
        years: u8,
    },
    /// Ban from residence (Cấm cư trú) - Article 47
    ResidenceBan {
        /// Duration in years (1-5)
        years: u8,
    },
    /// Probation (Quản chế) - Article 48
    Probation {
        /// Duration in years (1-3)
        years: u8,
    },
    /// Confiscation of property (Tịch thu tài sản) - Article 49
    PropertyConfiscation {
        /// Partial or full confiscation
        partial: bool,
    },
    /// Fine (Phạt tiền) - Article 33-34
    Fine {
        /// Amount in VND
        amount: i64,
    },
    /// Expulsion (Trục xuất) - Article 50
    Expulsion,
}

/// Crime severity levels (Mức độ nghiêm trọng) - Article 9
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrimeSeverity {
    /// Less serious crime (Tội phạm ít nghiêm trọng) - max 3 years imprisonment
    LessSerious,
    /// Serious crime (Tội phạm nghiêm trọng) - 3-7 years imprisonment
    Serious,
    /// Very serious crime (Tội phạm rất nghiêm trọng) - 7-15 years imprisonment
    VerySerious,
    /// Extremely serious crime (Tội phạm đặc biệt nghiêm trọng) - over 15 years, life, or death
    ExtremelySerious,
}

impl CrimeSeverity {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::LessSerious => "Tội phạm ít nghiêm trọng",
            Self::Serious => "Tội phạm nghiêm trọng",
            Self::VerySerious => "Tội phạm rất nghiêm trọng",
            Self::ExtremelySerious => "Tội phạm đặc biệt nghiêm trọng",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::LessSerious => "Less serious crime",
            Self::Serious => "Serious crime",
            Self::VerySerious => "Very serious crime",
            Self::ExtremelySerious => "Extremely serious crime",
        }
    }

    /// Get maximum imprisonment period in months
    pub fn max_imprisonment_months(&self) -> Option<u16> {
        match self {
            Self::LessSerious => Some(36),  // 3 years
            Self::Serious => Some(84),      // 7 years
            Self::VerySerious => Some(180), // 15 years
            Self::ExtremelySerious => None, // Can be life or death
        }
    }
}

/// Circumstances that aggravate criminal liability (Tình tiết tăng nặng) - Article 52
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AggravatingCircumstance {
    /// Organized crime (Phạm tội có tổ chức)
    OrganizedCrime,
    /// Abuse of position (Lợi dụng chức vụ, quyền hạn)
    AbuseOfPosition,
    /// Recidivism (Tái phạm nguy hiểm)
    Recidivism,
    /// Against vulnerable victims (Phạm tội đối với người già, trẻ em, người khuyết tật)
    VulnerableVictim,
    /// Using weapons (Sử dụng vũ khí)
    UsingWeapons,
    /// During disaster (Lợi dụng thiên tai, dịch bệnh)
    DuringDisaster,
    /// Other
    Other(String),
}

impl AggravatingCircumstance {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::OrganizedCrime => "Phạm tội có tổ chức".to_string(),
            Self::AbuseOfPosition => "Lợi dụng chức vụ, quyền hạn".to_string(),
            Self::Recidivism => "Tái phạm nguy hiểm".to_string(),
            Self::VulnerableVictim => {
                "Phạm tội đối với người già, trẻ em, người khuyết tật".to_string()
            }
            Self::UsingWeapons => "Sử dụng vũ khí, phương tiện nguy hiểm".to_string(),
            Self::DuringDisaster => "Lợi dụng thiên tai, dịch bệnh, chiến tranh".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }
}

/// Circumstances that mitigate criminal liability (Tình tiết giảm nhẹ) - Article 51
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MitigatingCircumstance {
    /// Voluntary surrender (Tự thú)
    VoluntarySurrender,
    /// Active cooperation (Tích cực hợp tác)
    ActiveCooperation,
    /// Restitution (Khắc phục hậu quả)
    Restitution,
    /// Self-defense (Phòng vệ chính đáng)
    SelfDefense,
    /// Under duress (Bị ép buộc)
    UnderDuress,
    /// First-time offender (Phạm tội lần đầu)
    FirstTimeOffender,
    /// Other
    Other(String),
}

impl MitigatingCircumstance {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> String {
        match self {
            Self::VoluntarySurrender => "Tự nguyện đầu thú, thành khẩn khai báo".to_string(),
            Self::ActiveCooperation => "Tích cực giúp đỡ cơ quan điều tra".to_string(),
            Self::Restitution => "Tích cực khắc phục hậu quả".to_string(),
            Self::SelfDefense => "Phòng vệ chính đáng".to_string(),
            Self::UnderDuress => "Phạm tội do bị ép buộc về vật chất hoặc tinh thần".to_string(),
            Self::FirstTimeOffender => "Phạm tội lần đầu, không nghiêm trọng".to_string(),
            Self::Other(desc) => desc.clone(),
        }
    }
}

/// Criminal liability age (Tuổi chịu trách nhiệm hình sự) - Article 12
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriminalAge;

impl CriminalAge {
    /// General criminal liability age (16 years) - Article 12.1
    pub const GENERAL: u8 = 16;

    /// Serious crimes age (14 years) - Article 12.2
    pub const SERIOUS_CRIMES: u8 = 14;

    /// Check if person bears criminal liability for crime
    pub fn is_liable(age: u8, severity: CrimeSeverity) -> bool {
        match severity {
            CrimeSeverity::VerySerious | CrimeSeverity::ExtremelySerious => {
                age >= Self::SERIOUS_CRIMES
            }
            _ => age >= Self::GENERAL,
        }
    }
}

/// Result type for criminal code operations
pub type CriminalCodeResult<T> = Result<T, CriminalCodeError>;

/// Errors related to Criminal Code
#[derive(Debug, Error)]
pub enum CriminalCodeError {
    /// Below criminal liability age
    #[error("Dưới tuổi chịu trách nhiệm hình sự (Điều 12): {age} tuổi")]
    BelowCriminalAge { age: u8 },

    /// Invalid penalty
    #[error("Hình phạt không hợp lệ (Điều {article}): {reason}")]
    InvalidPenalty { article: u32, reason: String },

    /// Penalty exceeds statutory limits
    #[error("Hình phạt vượt quá khung hình phạt (Điều {article}): {reason}")]
    PenaltyExceedsLimits { article: u32, reason: String },

    /// Invalid circumstances
    #[error("Tình tiết không hợp lệ: {reason}")]
    InvalidCircumstances { reason: String },
}

/// Validate criminal liability based on age
pub fn validate_criminal_liability(age: u8, severity: CrimeSeverity) -> CriminalCodeResult<()> {
    if CriminalAge::is_liable(age, severity) {
        Ok(())
    } else {
        Err(CriminalCodeError::BelowCriminalAge { age })
    }
}

/// Validate penalty is within statutory limits
pub fn validate_penalty(penalty: &Penalty) -> CriminalCodeResult<()> {
    if penalty.is_valid() {
        Ok(())
    } else {
        Err(CriminalCodeError::InvalidPenalty {
            article: 32,
            reason: format!("Hình phạt không đúng khung: {:?}", penalty),
        })
    }
}

/// Calculate adjusted penalty based on circumstances (Article 51-54)
pub fn calculate_adjusted_penalty(
    base_months: u16,
    mitigating: &[MitigatingCircumstance],
    aggravating: &[AggravatingCircumstance],
) -> u16 {
    let mut adjusted = base_months as f64;

    // Mitigating circumstances can reduce by up to 50%
    let mitigating_reduction = (mitigating.len() as f64 * 0.1).min(0.5);
    adjusted *= 1.0 - mitigating_reduction;

    // Aggravating circumstances can increase by up to 50%
    let aggravating_increase = (aggravating.len() as f64 * 0.1).min(0.5);
    adjusted *= 1.0 + aggravating_increase;

    // Minimum 3 months for imprisonment
    (adjusted as u16).max(3)
}

/// Get Criminal Code checklist
pub fn get_criminal_code_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Tuổi chịu trách nhiệm hình sự",
            "Criminal liability age",
            "Điều 12",
        ),
        ("Tội phạm cố ý", "Intentional crime", "Điều 13"),
        ("Tội phạm vô ý", "Negligent crime", "Điều 14"),
        ("Tình tiết giảm nhẹ", "Mitigating circumstances", "Điều 51"),
        (
            "Tình tiết tăng nặng",
            "Aggravating circumstances",
            "Điều 52",
        ),
        ("Khung hình phạt", "Penalty framework", "Điều 32-44"),
        ("Tái phạm", "Recidivism", "Điều 58"),
        ("Đồng phạm", "Accomplices", "Điều 17-18"),
        ("Chuẩn bị phạm tội", "Preparation of crime", "Điều 15"),
        ("Phạm tội chưa đạt", "Attempted crime", "Điều 16"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penalty_validation() {
        let valid_imprisonment = Penalty::FixedTermImprisonment { months: 120 };
        assert!(valid_imprisonment.is_valid());
        assert!(validate_penalty(&valid_imprisonment).is_ok());

        let invalid_imprisonment = Penalty::FixedTermImprisonment { months: 300 };
        assert!(!invalid_imprisonment.is_valid());

        let valid_fine = Penalty::Fine { amount: 50_000_000 };
        assert!(valid_fine.is_valid());

        let valid_reform = Penalty::ReformWithoutDetention { months: 12 };
        assert!(valid_reform.is_valid());
    }

    #[test]
    fn test_criminal_age() {
        // 17-year-old for less serious crime
        assert!(validate_criminal_liability(17, CrimeSeverity::LessSerious).is_ok());

        // 15-year-old for less serious crime - not liable
        assert!(validate_criminal_liability(15, CrimeSeverity::LessSerious).is_err());

        // 14-year-old for extremely serious crime - liable
        assert!(validate_criminal_liability(14, CrimeSeverity::ExtremelySerious).is_ok());

        // 13-year-old for any crime - not liable
        assert!(validate_criminal_liability(13, CrimeSeverity::ExtremelySerious).is_err());
    }

    #[test]
    fn test_crime_severity() {
        assert_eq!(
            CrimeSeverity::LessSerious.max_imprisonment_months(),
            Some(36)
        );
        assert_eq!(
            CrimeSeverity::ExtremelySerious.max_imprisonment_months(),
            None
        );
    }

    #[test]
    fn test_adjusted_penalty() {
        let base = 120; // 10 years

        // No circumstances - stays the same
        let adjusted = calculate_adjusted_penalty(base, &[], &[]);
        assert_eq!(adjusted, base);

        // One mitigating circumstance - reduces by ~10%
        let mitigating = vec![MitigatingCircumstance::VoluntarySurrender];
        let adjusted = calculate_adjusted_penalty(base, &mitigating, &[]);
        assert!(adjusted < base);
        assert_eq!(adjusted, 108); // 120 * 0.9

        // One aggravating circumstance - increases by ~10%
        let aggravating = vec![AggravatingCircumstance::OrganizedCrime];
        let adjusted = calculate_adjusted_penalty(base, &[], &aggravating);
        assert!(adjusted > base);
        assert_eq!(adjusted, 132); // 120 * 1.1

        // Multiple circumstances
        let adjusted = calculate_adjusted_penalty(base, &mitigating, &aggravating);
        assert_eq!(adjusted, 118); // 120 * 0.9 * 1.1 = 118.8 -> 118
    }

    #[test]
    fn test_penalty_display() {
        let death = Penalty::Death;
        assert_eq!(death.name_vi(), "Tử hình");
        assert_eq!(death.name_en(), "Death penalty");

        let imprisonment = Penalty::FixedTermImprisonment { months: 60 };
        assert!(imprisonment.name_vi().contains("60 tháng"));
    }

    #[test]
    fn test_criminal_code_checklist() {
        let checklist = get_criminal_code_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 10);
    }

    #[test]
    fn test_supplementary_penalties() {
        let prohibition = SupplementaryPenalty::PostProhibition { years: 3 };
        assert!(matches!(
            prohibition,
            SupplementaryPenalty::PostProhibition { years: 3 }
        ));

        let confiscation = SupplementaryPenalty::PropertyConfiscation { partial: false };
        assert!(matches!(
            confiscation,
            SupplementaryPenalty::PropertyConfiscation { partial: false }
        ));
    }
}
