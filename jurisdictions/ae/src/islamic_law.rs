//! UAE Islamic Law (Sharia) Integration
//!
//! The UAE legal system integrates Islamic Sharia principles, particularly in:
//! - Personal status matters (family law, inheritance)
//! - Islamic finance and banking
//! - Religious endowments (Waqf)
//!
//! ## Sources of Islamic Law
//!
//! 1. **القرآن الكريم** - Quran (Primary source)
//! 2. **السنة النبوية** - Sunnah (Prophetic tradition)
//! 3. **الإجماع** - Ijma (Scholarly consensus)
//! 4. **القياس** - Qiyas (Analogical reasoning)
//!
//! ## Applicable Schools (Madhahib)
//!
//! UAE primarily follows the **Maliki school** (المذهب المالكي) but also
//! references Hanafi, Shafi'i, and Hanbali schools.

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for Islamic law operations
pub type IslamicLawResult<T> = Result<T, IslamicLawError>;

/// Islamic schools of jurisprudence (Madhahib)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MadhabSchool {
    /// Hanafi (حنفي) - Followed by Abu Hanifa
    Hanafi,
    /// Maliki (مالكي) - Followed by Malik ibn Anas (UAE primary)
    Maliki,
    /// Shafi'i (شافعي) - Followed by Al-Shafi'i
    Shafii,
    /// Hanbali (حنبلي) - Followed by Ahmad ibn Hanbal
    Hanbali,
}

impl MadhabSchool {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Hanafi => "المذهب الحنفي",
            Self::Maliki => "المذهب المالكي",
            Self::Shafii => "المذهب الشافعي",
            Self::Hanbali => "المذهب الحنبلي",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Hanafi => "Hanafi School",
            Self::Maliki => "Maliki School",
            Self::Shafii => "Shafi'i School",
            Self::Hanbali => "Hanbali School",
        }
    }
}

/// Types of inheritance shares under Islamic law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InheritanceShare {
    /// Fixed share (e.g., 1/2, 1/4, 1/8)
    Fixed { numerator: u32, denominator: u32 },
    /// Residuary heir (gets remaining after fixed shares)
    Residuary,
    /// No inheritance (legally barred)
    None,
}

impl InheritanceShare {
    /// Calculate actual amount from estate
    pub fn calculate_amount(&self, total_estate: Aed) -> Aed {
        match self {
            Self::Fixed {
                numerator,
                denominator,
            } => {
                let fils = total_estate.fils() * (*numerator as i64) / (*denominator as i64);
                Aed::from_fils(fils)
            }
            Self::Residuary | Self::None => Aed::from_fils(0),
        }
    }

    /// Get fraction as string
    pub fn as_fraction(&self) -> String {
        match self {
            Self::Fixed {
                numerator,
                denominator,
            } => format!("{}/{}", numerator, denominator),
            Self::Residuary => "Residuary (عصبة)".to_string(),
            Self::None => "None".to_string(),
        }
    }
}

/// Relationship to deceased for inheritance
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeritageRelationship {
    /// Husband (زوج)
    Husband,
    /// Wife (زوجة)
    Wife,
    /// Father (أب)
    Father,
    /// Mother (أم)
    Mother,
    /// Son (ابن)
    Son,
    /// Daughter (بنت)
    Daughter,
    /// Full Brother (أخ شقيق)
    FullBrother,
    /// Full Sister (أخت شقيقة)
    FullSister,
    /// Paternal Grandfather (جد)
    PaternalGrandfather,
    /// Maternal Grandmother (جدة)
    MaternalGrandmother,
}

impl HeritageRelationship {
    /// Get standard inheritance share
    ///
    /// Note: Actual shares depend on presence of other heirs.
    /// This returns typical shares in common scenarios.
    pub fn typical_share(&self, has_children: bool, spouse_exists: bool) -> InheritanceShare {
        match self {
            Self::Husband => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 4,
                    } // 1/4
                } else {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 2,
                    } // 1/2
                }
            }
            Self::Wife => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 8,
                    } // 1/8
                } else {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 4,
                    } // 1/4
                }
            }
            Self::Father => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 6,
                    } // 1/6
                } else {
                    InheritanceShare::Residuary // Gets remainder
                }
            }
            Self::Mother => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 6,
                    } // 1/6
                } else {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 3,
                    } // 1/3
                }
            }
            Self::Son => InheritanceShare::Residuary, // Sons are residuary heirs
            Self::Daughter => InheritanceShare::Fixed {
                // Varies: 1/2 if sole, 2/3 if 2+
                numerator: 1,
                denominator: 2,
            },
            Self::FullBrother | Self::FullSister => {
                if has_children || spouse_exists {
                    InheritanceShare::None // Blocked by children
                } else {
                    InheritanceShare::Residuary
                }
            }
            Self::PaternalGrandfather => InheritanceShare::Fixed {
                numerator: 1,
                denominator: 6,
            },
            Self::MaternalGrandmother => InheritanceShare::Fixed {
                numerator: 1,
                denominator: 6,
            },
        }
    }

    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Husband => "زوج",
            Self::Wife => "زوجة",
            Self::Father => "أب",
            Self::Mother => "أم",
            Self::Son => "ابن",
            Self::Daughter => "بنت",
            Self::FullBrother => "أخ شقيق",
            Self::FullSister => "أخت شقيقة",
            Self::PaternalGrandfather => "جد",
            Self::MaternalGrandmother => "جدة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Husband => "Husband",
            Self::Wife => "Wife",
            Self::Father => "Father",
            Self::Mother => "Mother",
            Self::Son => "Son",
            Self::Daughter => "Daughter",
            Self::FullBrother => "Full Brother",
            Self::FullSister => "Full Sister",
            Self::PaternalGrandfather => "Paternal Grandfather",
            Self::MaternalGrandmother => "Maternal Grandmother",
        }
    }
}

/// Types of Islamic finance contracts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IslamicFinanceContract {
    /// Murabaha (مرابحة) - Cost-plus financing
    Murabaha,
    /// Ijara (إجارة) - Leasing
    Ijara,
    /// Mudaraba (مضاربة) - Profit-sharing partnership
    Mudaraba,
    /// Musharaka (مشاركة) - Joint venture
    Musharaka,
    /// Istisna'a (استصناع) - Manufacturing/construction financing
    Istisna,
    /// Sukuk (صكوك) - Islamic bonds
    Sukuk,
    /// Takaful (تكافل) - Islamic insurance
    Takaful,
    /// Qard Hasan (قرض حسن) - Benevolent loan
    QardHasan,
}

impl IslamicFinanceContract {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Murabaha => "مرابحة",
            Self::Ijara => "إجارة",
            Self::Mudaraba => "مضاربة",
            Self::Musharaka => "مشاركة",
            Self::Istisna => "استصناع",
            Self::Sukuk => "صكوك",
            Self::Takaful => "تكافل",
            Self::QardHasan => "قرض حسن",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Murabaha => "Murabaha (Cost-Plus Financing)",
            Self::Ijara => "Ijara (Leasing)",
            Self::Mudaraba => "Mudaraba (Profit-Sharing Partnership)",
            Self::Musharaka => "Musharaka (Joint Venture)",
            Self::Istisna => "Istisna'a (Manufacturing Finance)",
            Self::Sukuk => "Sukuk (Islamic Bonds)",
            Self::Takaful => "Takaful (Islamic Insurance)",
            Self::QardHasan => "Qard Hasan (Benevolent Loan)",
        }
    }

    /// Check if contract is Sharia-compliant
    pub fn is_sharia_compliant(&self) -> bool {
        // All defined contracts are Sharia-compliant by definition
        true
    }

    /// Get prohibited elements (Riba, Gharar, Maysir)
    pub fn prohibited_in_conventional(&self) -> Vec<&'static str> {
        match self {
            Self::Murabaha | Self::Ijara | Self::Istisna => {
                vec!["Interest (Riba)", "Excessive uncertainty (Gharar)"]
            }
            Self::Mudaraba | Self::Musharaka => vec!["Guaranteed returns", "Fixed interest"],
            Self::Sukuk => vec!["Interest-bearing debt", "Non-asset backed"],
            Self::Takaful => vec!["Gambling (Maysir)", "Interest (Riba)"],
            Self::QardHasan => vec!["Interest (Riba)"],
        }
    }
}

/// Types of religious endowments (Waqf)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WaqfType {
    /// Public waqf (وقف عام) - For general public benefit
    Public,
    /// Family waqf (وقف أهلي) - For family descendants
    Family,
    /// Mixed waqf (وقف مشترك) - Combination of public and family
    Mixed,
}

impl WaqfType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::Public => "وقف عام",
            Self::Family => "وقف أهلي",
            Self::Mixed => "وقف مشترك",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Public => "Public Waqf (Charitable Endowment)",
            Self::Family => "Family Waqf (Private Endowment)",
            Self::Mixed => "Mixed Waqf",
        }
    }
}

/// Waqf (Religious Endowment) details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waqf {
    /// Type of waqf
    pub waqf_type: WaqfType,
    /// Endowed property value
    pub property_value: Aed,
    /// Beneficiary description
    pub beneficiaries: String,
    /// Is perpetual (most waqfs are perpetual)
    pub is_perpetual: bool,
    /// Managing authority
    pub manager: String,
}

impl Waqf {
    /// Check if waqf is valid under Islamic law
    pub fn is_valid(&self) -> IslamicLawResult<()> {
        if self.property_value.fils() <= 0 {
            return Err(IslamicLawError::InvalidWaqf {
                reason: "Property value must be positive".to_string(),
            });
        }

        if self.beneficiaries.is_empty() {
            return Err(IslamicLawError::InvalidWaqf {
                reason: "Beneficiaries must be specified".to_string(),
            });
        }

        Ok(())
    }
}

/// Islamic law errors
#[derive(Debug, Error)]
pub enum IslamicLawError {
    /// Inheritance calculation error
    #[error("خطأ في حساب الميراث: {reason}")]
    InheritanceError { reason: String },

    /// Non-Sharia compliant transaction
    #[error("المعاملة غير متوافقة مع الشريعة الإسلامية: {element}")]
    NonShariaCompliant { element: String },

    /// Invalid waqf
    #[error("الوقف غير صالح: {reason}")]
    InvalidWaqf { reason: String },

    /// Prohibited element (Riba, Gharar, Maysir)
    #[error("عنصر محظور في الشريعة الإسلامية: {element} ({description})")]
    ProhibitedElement {
        element: String,
        description: String,
    },
}

/// Check if financial transaction contains Riba (interest)
pub fn check_riba(interest_rate: Option<f64>) -> IslamicLawResult<()> {
    if let Some(rate) = interest_rate
        && rate > 0.0
    {
        return Err(IslamicLawError::ProhibitedElement {
            element: "Riba (الربا)".to_string(),
            description: "Interest-based transactions are prohibited".to_string(),
        });
    }
    Ok(())
}

/// Calculate simple inheritance distribution
///
/// Note: This is a simplified calculation. Real Islamic inheritance
/// can be complex and requires qualified Islamic jurists (فقهاء).
pub fn calculate_simple_inheritance(
    estate: Aed,
    relationships: &[(HeritageRelationship, u32)],
) -> Vec<(String, InheritanceShare, Aed)> {
    let has_children = relationships.iter().any(|(rel, _)| {
        matches!(
            rel,
            HeritageRelationship::Son | HeritageRelationship::Daughter
        )
    });

    let spouse_exists = relationships.iter().any(|(rel, _)| {
        matches!(
            rel,
            HeritageRelationship::Husband | HeritageRelationship::Wife
        )
    });

    let mut distribution = Vec::new();

    for (relationship, count) in relationships {
        let share = relationship.typical_share(has_children, spouse_exists);
        let amount = share.calculate_amount(estate);
        let total_amount = Aed::from_fils(amount.fils() * (*count as i64));

        distribution.push((relationship.name_en().to_string(), share, total_amount));
    }

    distribution
}

/// Get Islamic finance compliance checklist
pub fn get_sharia_compliance_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("عدم وجود الربا", "No interest (Riba)"),
        ("عدم الغرر المفرط", "No excessive uncertainty (Gharar)"),
        ("عدم الميسر", "No gambling (Maysir)"),
        ("ملكية أصول حقيقية", "Real asset ownership"),
        ("مشاركة الأرباح والخسائر", "Profit and loss sharing"),
        ("نشاط حلال", "Halal business activity"),
        ("شفافية", "Transparency"),
        ("عقد صحيح", "Valid Islamic contract"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_madhab_schools() {
        assert_eq!(MadhabSchool::Maliki.name_ar(), "المذهب المالكي");
        assert_eq!(MadhabSchool::Hanafi.name_en(), "Hanafi School");
    }

    #[test]
    fn test_inheritance_share_calculation() {
        let estate = Aed::from_dirhams(100_000);
        let share = InheritanceShare::Fixed {
            numerator: 1,
            denominator: 2,
        };

        let amount = share.calculate_amount(estate);
        assert_eq!(amount.dirhams(), 50_000);
    }

    #[test]
    fn test_husband_inheritance_with_children() {
        let rel = HeritageRelationship::Husband;
        let share = rel.typical_share(true, false);

        match share {
            InheritanceShare::Fixed {
                numerator,
                denominator,
            } => {
                assert_eq!(numerator, 1);
                assert_eq!(denominator, 4); // 1/4 with children
            }
            _ => panic!("Expected fixed share"),
        }
    }

    #[test]
    fn test_husband_inheritance_no_children() {
        let rel = HeritageRelationship::Husband;
        let share = rel.typical_share(false, false);

        match share {
            InheritanceShare::Fixed {
                numerator,
                denominator,
            } => {
                assert_eq!(numerator, 1);
                assert_eq!(denominator, 2); // 1/2 without children
            }
            _ => panic!("Expected fixed share"),
        }
    }

    #[test]
    fn test_islamic_finance_contracts() {
        let murabaha = IslamicFinanceContract::Murabaha;
        assert!(murabaha.is_sharia_compliant());
        assert_eq!(murabaha.name_ar(), "مرابحة");

        let prohibited = murabaha.prohibited_in_conventional();
        assert!(prohibited.contains(&"Interest (Riba)"));
    }

    #[test]
    fn test_riba_check() {
        assert!(check_riba(None).is_ok());
        assert!(check_riba(Some(0.0)).is_ok());
        assert!(check_riba(Some(5.0)).is_err());
    }

    #[test]
    fn test_waqf_validation() {
        let valid_waqf = Waqf {
            waqf_type: WaqfType::Public,
            property_value: Aed::from_dirhams(1_000_000),
            beneficiaries: "Mosque and school".to_string(),
            is_perpetual: true,
            manager: "Awqaf Department".to_string(),
        };

        assert!(valid_waqf.is_valid().is_ok());

        let invalid_waqf = Waqf {
            waqf_type: WaqfType::Family,
            property_value: Aed::from_fils(0),
            beneficiaries: "".to_string(),
            is_perpetual: true,
            manager: "".to_string(),
        };

        assert!(invalid_waqf.is_valid().is_err());
    }

    #[test]
    fn test_simple_inheritance_calculation() {
        let estate = Aed::from_dirhams(100_000);
        let heirs = vec![
            (HeritageRelationship::Husband, 1),
            (HeritageRelationship::Daughter, 2),
        ];

        let distribution = calculate_simple_inheritance(estate, &heirs);
        assert!(!distribution.is_empty());
        assert_eq!(distribution.len(), 2);
    }

    #[test]
    fn test_waqf_types() {
        assert_eq!(WaqfType::Public.name_ar(), "وقف عام");
        assert_eq!(
            WaqfType::Family.name_en(),
            "Family Waqf (Private Endowment)"
        );
    }

    #[test]
    fn test_sharia_compliance_checklist() {
        let checklist = get_sharia_compliance_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.len() >= 8);
    }

    #[test]
    fn test_inheritance_share_fraction() {
        let share = InheritanceShare::Fixed {
            numerator: 1,
            denominator: 4,
        };
        assert_eq!(share.as_fraction(), "1/4");

        let residuary = InheritanceShare::Residuary;
        assert!(residuary.as_fraction().contains("Residuary"));
    }
}
