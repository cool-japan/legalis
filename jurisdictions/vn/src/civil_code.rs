//! Vietnamese Civil Code 2015 (Bộ luật Dân sự 2015) - Law No. 91/2015/QH13
//!
//! Vietnam's comprehensive civil law code, effective from January 1, 2017.
//! Amended by Law 42/2020 (taking effect January 1, 2021).
//!
//! ## Structure
//!
//! The Civil Code contains 7 chapters (called "Parts" - Phần) and 689 articles:
//!
//! 1. **Part I: General Provisions** (Điều 1-36)
//! 2. **Part II: Individuals** (Điều 37-97)
//! 3. **Part III: Legal Persons** (Điều 98-170)
//! 4. **Part IV: Property, Ownership, Possession** (Điều 171-289)
//! 5. **Part V: Civil Transactions and Representation** (Điều 290-492)
//! 6. **Part VI: Statute of Limitations** (Điều 493-503)
//! 7. **Part VII: Inheritance** (Điều 604-689)
//!
//! ## Key Concepts
//!
//! - Legal capacity (Năng lực pháp luật dân sự)
//! - Civil transactions (Giao dịch dân sự)
//! - Property rights (Quyền sở hữu)
//! - Contracts (Hợp đồng)
//! - Torts (Trách nhiệm bồi thường thiệt hại ngoài hợp đồng)
//! - Inheritance (Thừa kế)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Parts of the Civil Code (7 main parts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CivilCodePart {
    /// Part I: General Provisions (Điều 1-36)
    GeneralProvisions,
    /// Part II: Individuals (Điều 37-97)
    Individuals,
    /// Part III: Legal Persons (Điều 98-170)
    LegalPersons,
    /// Part IV: Property, Ownership, Possession (Điều 171-289)
    PropertyAndOwnership,
    /// Part V: Civil Transactions and Representation (Điều 290-492)
    CivilTransactions,
    /// Part VI: Statute of Limitations (Điều 493-503)
    StatuteOfLimitations,
    /// Part VII: Inheritance (Điều 604-689)
    Inheritance,
}

impl CivilCodePart {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::GeneralProvisions => "Phần Thứ nhất: Những quy định chung",
            Self::Individuals => "Phần Thứ hai: Cá nhân",
            Self::LegalPersons => "Phần Thứ ba: Pháp nhân",
            Self::PropertyAndOwnership => "Phần Thứ tư: Tài sản, quyền sở hữu, quyền chiếm hữu",
            Self::CivilTransactions => "Phần Thứ năm: Giao dịch dân sự và đại diện",
            Self::StatuteOfLimitations => "Phần Thứ sáu: Thời hiệu",
            Self::Inheritance => "Phần Thứ bảy: Thừa kế",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::GeneralProvisions => "Part I: General Provisions",
            Self::Individuals => "Part II: Individuals",
            Self::LegalPersons => "Part III: Legal Persons",
            Self::PropertyAndOwnership => "Part IV: Property, Ownership, Possession",
            Self::CivilTransactions => "Part V: Civil Transactions and Representation",
            Self::StatuteOfLimitations => "Part VI: Statute of Limitations",
            Self::Inheritance => "Part VII: Inheritance",
        }
    }

    /// Get article range for this part
    pub fn article_range(&self) -> (u32, u32) {
        match self {
            Self::GeneralProvisions => (1, 36),
            Self::Individuals => (37, 97),
            Self::LegalPersons => (98, 170),
            Self::PropertyAndOwnership => (171, 289),
            Self::CivilTransactions => (290, 492),
            Self::StatuteOfLimitations => (493, 503),
            Self::Inheritance => (604, 689),
        }
    }
}

/// Civil capacity levels (Năng lực hành vi dân sự) - Article 19-21
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CivilCapacity {
    /// Full civil capacity (Có đầy đủ năng lực hành vi dân sự) - 18 years or older
    Full,
    /// Limited civil capacity (Hạn chế năng lực hành vi dân sự) - 6-18 years
    Limited,
    /// No civil capacity (Không có năng lực hành vi dân sự) - Under 6 years
    None,
}

impl CivilCapacity {
    /// Determine civil capacity by age
    pub fn from_age(age: u8) -> Self {
        if age >= 18 {
            Self::Full
        } else if age >= 6 {
            Self::Limited
        } else {
            Self::None
        }
    }

    /// Check if can enter into transactions independently
    pub fn can_transact_independently(&self) -> bool {
        matches!(self, Self::Full)
    }

    /// Check if requires guardian consent
    pub fn requires_guardian_consent(&self) -> bool {
        matches!(self, Self::Limited | Self::None)
    }
}

/// Types of property (Tài sản) - Article 105-107
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyType {
    /// Movable property (Động sản)
    Movable,
    /// Immovable property (Bất động sản) - land, buildings
    Immovable,
    /// Intellectual property (Tài sản trí tuệ)
    Intellectual,
    /// Money (Tiền)
    Money,
    /// Securities (Giấy tờ có giá)
    Securities,
}

impl PropertyType {
    /// Get Vietnamese name
    pub fn name_vi(&self) -> &'static str {
        match self {
            Self::Movable => "Động sản",
            Self::Immovable => "Bất động sản",
            Self::Intellectual => "Tài sản trí tuệ",
            Self::Money => "Tiền",
            Self::Securities => "Giấy tờ có giá",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Movable => "Movable property",
            Self::Immovable => "Immovable property",
            Self::Intellectual => "Intellectual property",
            Self::Money => "Money",
            Self::Securities => "Securities",
        }
    }

    /// Check if requires registration for ownership transfer
    pub fn requires_registration(&self) -> bool {
        matches!(self, Self::Immovable | Self::Intellectual)
    }
}

/// Contract types (Hợp đồng) - Article 385
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale contract (Hợp đồng mua bán)
    Sale,
    /// Gift contract (Hợp đồng tặng cho)
    Gift,
    /// Loan contract (Hợp đồng vay)
    Loan,
    /// Lease contract (Hợp đồng thuê tài sản)
    Lease,
    /// Service contract (Hợp đồng dịch vụ)
    Service,
    /// Construction contract (Hợp đồng xây dựng)
    Construction,
    /// Other
    Other(String),
}

/// Statute of limitations periods (Thời hiệu) - Article 155-159
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatuteOfLimitations {
    /// Period in years
    pub years: u8,
}

impl StatuteOfLimitations {
    /// General statute of limitations (3 years) - Article 155.1
    pub fn general() -> Self {
        Self { years: 3 }
    }

    /// Real property related (10 years) - Article 155.3
    pub fn real_property() -> Self {
        Self { years: 10 }
    }

    /// Contract disputes (3 years) - Article 155.1
    pub fn contract_disputes() -> Self {
        Self { years: 3 }
    }

    /// Tort claims (3 years) - Article 155.1
    pub fn tort_claims() -> Self {
        Self { years: 3 }
    }

    /// Environmental damage (10 years) - Article 155.3
    pub fn environmental_damage() -> Self {
        Self { years: 10 }
    }
}

/// Inheritance types (Thừa kế) - Article 605-606
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InheritanceType {
    /// Testamentary inheritance (Thừa kế theo di chúc)
    Testamentary,
    /// Statutory inheritance (Thừa kế theo pháp luật)
    Statutory,
}

/// Inheritance priority groups (Hàng thừa kế) - Article 651
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InheritancePriority {
    /// First priority: spouse, parents, children
    First,
    /// Second priority: siblings, paternal grandparents, maternal grandparents
    Second,
    /// Third priority: uncles, aunts
    Third,
}

impl InheritancePriority {
    /// Get Vietnamese description
    pub fn description_vi(&self) -> &'static str {
        match self {
            Self::First => "Vợ, chồng, cha đẻ, mẹ đẻ, cha nuôi, mẹ nuôi, con đẻ, con nuôi",
            Self::Second => "Ông nội, bà nội, ông ngoại, bà ngoại, anh, chị, em ruột",
            Self::Third => "Cô, dì, cậu, chú, bác ruột",
        }
    }

    /// Get English description
    pub fn description_en(&self) -> &'static str {
        match self {
            Self::First => {
                "Spouse, biological parents, adoptive parents, biological children, adopted children"
            }
            Self::Second => "Paternal grandparents, maternal grandparents, siblings",
            Self::Third => "Uncles, aunts",
        }
    }
}

/// Result type for civil code operations
pub type CivilCodeResult<T> = Result<T, CivilCodeError>;

/// Errors related to Civil Code
#[derive(Debug, Error)]
pub enum CivilCodeError {
    /// Lack of civil capacity
    #[error("Thiếu năng lực hành vi dân sự (Điều {article}): {reason}")]
    LackOfCivilCapacity { article: u32, reason: String },

    /// Invalid contract
    #[error("Hợp đồng không hợp lệ (Điều {article}): {reason}")]
    InvalidContract { article: u32, reason: String },

    /// Property rights violation
    #[error("Vi phạm quyền sở hữu (Điều {article}): {reason}")]
    PropertyRightsViolation { article: u32, reason: String },

    /// Statute of limitations expired
    #[error("Hết thời hiệu (Điều 155): đã quá {years} năm")]
    StatuteExpired { years: u8 },

    /// Invalid inheritance claim
    #[error("Yêu cầu thừa kế không hợp lệ (Điều {article}): {reason}")]
    InvalidInheritanceClaim { article: u32, reason: String },
}

/// Validate if a person can enter into a transaction
pub fn validate_civil_capacity(age: u8, transaction_value: i64) -> CivilCodeResult<()> {
    let capacity = CivilCapacity::from_age(age);

    match capacity {
        CivilCapacity::Full => Ok(()),
        CivilCapacity::Limited => {
            // Limited capacity persons can do small transactions (Article 21)
            // Typically under 10 million VND for minors 6-18
            if transaction_value > 10_000_000 {
                Err(CivilCodeError::LackOfCivilCapacity {
                    article: 21,
                    reason: format!(
                        "Người từ 6-18 tuổi cần sự đồng ý của người đại diện cho giao dịch trên 10 triệu đồng. Giá trị giao dịch: {} VND",
                        transaction_value
                    ),
                })
            } else {
                Ok(())
            }
        }
        CivilCapacity::None => Err(CivilCodeError::LackOfCivilCapacity {
            article: 20,
            reason: "Người dưới 6 tuổi không có năng lực hành vi dân sự".to_string(),
        }),
    }
}

/// Validate statute of limitations
pub fn validate_statute_of_limitations(
    years_elapsed: u8,
    limitation: StatuteOfLimitations,
) -> CivilCodeResult<()> {
    if years_elapsed > limitation.years {
        Err(CivilCodeError::StatuteExpired {
            years: limitation.years,
        })
    } else {
        Ok(())
    }
}

/// Calculate inheritance share for statutory inheritance (Article 651-652)
pub fn calculate_statutory_inheritance_share(
    total_heirs_in_priority: u32,
    is_minor_or_disabled: bool,
) -> Result<f64, CivilCodeError> {
    if total_heirs_in_priority == 0 {
        return Err(CivilCodeError::InvalidInheritanceClaim {
            article: 651,
            reason: "Không có người thừa kế".to_string(),
        });
    }

    let base_share = 1.0 / f64::from(total_heirs_in_priority);

    // Minors and disabled persons get additional share (Article 652)
    if is_minor_or_disabled {
        Ok(base_share * 1.5)
    } else {
        Ok(base_share)
    }
}

/// Get Civil Code checklist
pub fn get_civil_code_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Năng lực pháp luật dân sự",
            "Civil legal capacity",
            "Điều 16",
        ),
        (
            "Năng lực hành vi dân sự",
            "Civil act capacity",
            "Điều 19-21",
        ),
        (
            "Điều kiện hợp lệ giao dịch dân sự",
            "Valid civil transaction conditions",
            "Điều 117",
        ),
        (
            "Hình thức giao dịch dân sự",
            "Civil transaction form",
            "Điều 119-122",
        ),
        ("Quyền sở hữu tài sản", "Property ownership", "Điều 222-229"),
        ("Thời hiệu khởi kiện", "Statute of limitations", "Điều 155"),
        ("Di chúc hợp lệ", "Valid will", "Điều 629-645"),
        (
            "Quyền thừa kế theo pháp luật",
            "Statutory inheritance rights",
            "Điều 651-655",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_capacity_from_age() {
        assert_eq!(CivilCapacity::from_age(25), CivilCapacity::Full);
        assert_eq!(CivilCapacity::from_age(18), CivilCapacity::Full);
        assert_eq!(CivilCapacity::from_age(15), CivilCapacity::Limited);
        assert_eq!(CivilCapacity::from_age(5), CivilCapacity::None);
    }

    #[test]
    fn test_civil_capacity_validation() {
        // Adult can do any transaction
        assert!(validate_civil_capacity(25, 1_000_000_000).is_ok());

        // Minor can do small transaction
        assert!(validate_civil_capacity(15, 5_000_000).is_ok());

        // Minor cannot do large transaction without consent
        assert!(validate_civil_capacity(15, 50_000_000).is_err());

        // Child under 6 cannot transact
        assert!(validate_civil_capacity(5, 1_000).is_err());
    }

    #[test]
    fn test_statute_of_limitations() {
        let general = StatuteOfLimitations::general();
        assert_eq!(general.years, 3);

        let real_property = StatuteOfLimitations::real_property();
        assert_eq!(real_property.years, 10);

        // Within limitation
        assert!(validate_statute_of_limitations(2, general).is_ok());

        // Exceeded limitation
        assert!(validate_statute_of_limitations(5, general).is_err());
    }

    #[test]
    fn test_property_types() {
        assert!(PropertyType::Immovable.requires_registration());
        assert!(PropertyType::Intellectual.requires_registration());
        assert!(!PropertyType::Movable.requires_registration());
    }

    #[test]
    fn test_inheritance_calculation() {
        // 3 heirs, equal share
        let share = calculate_statutory_inheritance_share(3, false).ok();
        assert!(share.is_some());
        assert!((share.unwrap() - 0.333).abs() < 0.01);

        // Minor gets 1.5x share
        let minor_share = calculate_statutory_inheritance_share(3, true).ok();
        assert!(minor_share.is_some());
        assert!((minor_share.unwrap() - 0.5).abs() < 0.01);

        // No heirs should error
        assert!(calculate_statutory_inheritance_share(0, false).is_err());
    }

    #[test]
    fn test_civil_code_parts() {
        let part1 = CivilCodePart::GeneralProvisions;
        assert_eq!(part1.article_range(), (1, 36));

        let part7 = CivilCodePart::Inheritance;
        assert_eq!(part7.article_range(), (604, 689));
    }

    #[test]
    fn test_inheritance_priority() {
        let first = InheritancePriority::First;
        assert!(first.description_vi().contains("Vợ, chồng"));
        assert!(first.description_en().contains("Spouse"));
    }

    #[test]
    fn test_civil_code_checklist() {
        let checklist = get_civil_code_checklist();
        assert!(!checklist.is_empty());
        assert_eq!(checklist.len(), 8);
    }
}
