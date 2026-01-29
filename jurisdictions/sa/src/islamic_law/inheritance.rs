//! Islamic Inheritance Law (المواريث)
//!
//! Inheritance distribution according to Islamic Sharia following Hanbali school.
//! The inheritance shares are specified in the Quran.

use crate::common::Sar;
use serde::{Deserialize, Serialize};

/// Types of inheritance shares under Islamic law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InheritanceShare {
    /// Fixed share (e.g., 1/2, 1/4, 1/8)
    Fixed { numerator: u32, denominator: u32 },
    /// Residuary heir (عصبة) - gets remaining after fixed shares
    Residuary,
    /// No inheritance (legally barred or excluded)
    None,
}

impl InheritanceShare {
    /// Calculate actual amount from estate
    pub fn calculate_amount(&self, total_estate: Sar) -> Sar {
        match self {
            Self::Fixed {
                numerator,
                denominator,
            } => {
                let halalas = total_estate.halalas() * (*numerator as i64) / (*denominator as i64);
                Sar::from_halalas(halalas)
            }
            Self::Residuary | Self::None => Sar::from_halalas(0),
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

/// Relationship to deceased for inheritance purposes
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
    /// Paternal Grandmother (جدة لأب)
    PaternalGrandmother,
    /// Maternal Grandmother (جدة لأم)
    MaternalGrandmother,
    /// Grandson through son (ابن الابن)
    GrandsonThroughSon,
    /// Granddaughter through son (بنت الابن)
    GranddaughterThroughSon,
}

impl HeritageRelationship {
    /// Get typical inheritance share based on Quranic rules
    ///
    /// Note: Actual shares can be complex and depend on presence of other heirs.
    pub fn typical_share(&self, has_children: bool) -> InheritanceShare {
        match self {
            Self::Husband => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 4,
                    } // 1/4 if children exist
                } else {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 2,
                    } // 1/2 if no children
                }
            }
            Self::Wife => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 8,
                    } // 1/8 if children exist
                } else {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 4,
                    } // 1/4 if no children
                }
            }
            Self::Father => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 6,
                    } // 1/6 if children exist
                } else {
                    InheritanceShare::Residuary // Gets remainder if no children
                }
            }
            Self::Mother => {
                if has_children {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 6,
                    } // 1/6 if children exist
                } else {
                    InheritanceShare::Fixed {
                        numerator: 1,
                        denominator: 3,
                    } // 1/3 if no children
                }
            }
            Self::Son => InheritanceShare::Residuary, // Sons are residuary heirs (twice daughter's share)
            Self::Daughter => InheritanceShare::Fixed {
                // Varies: 1/2 if sole daughter, 2/3 if 2+ daughters
                numerator: 1,
                denominator: 2,
            },
            Self::FullBrother | Self::FullSister => {
                if has_children {
                    InheritanceShare::None // Blocked by children
                } else {
                    InheritanceShare::Residuary
                }
            }
            Self::PaternalGrandfather => InheritanceShare::Fixed {
                numerator: 1,
                denominator: 6,
            },
            Self::PaternalGrandmother | Self::MaternalGrandmother => InheritanceShare::Fixed {
                numerator: 1,
                denominator: 6,
            },
            Self::GrandsonThroughSon => InheritanceShare::Residuary,
            Self::GranddaughterThroughSon => InheritanceShare::Fixed {
                numerator: 1,
                denominator: 2,
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
            Self::PaternalGrandmother => "جدة لأب",
            Self::MaternalGrandmother => "جدة لأم",
            Self::GrandsonThroughSon => "ابن الابن",
            Self::GranddaughterThroughSon => "بنت الابن",
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
            Self::PaternalGrandmother => "Paternal Grandmother",
            Self::MaternalGrandmother => "Maternal Grandmother",
            Self::GrandsonThroughSon => "Grandson (through son)",
            Self::GranddaughterThroughSon => "Granddaughter (through son)",
        }
    }
}

/// Calculate inheritance distribution
///
/// Note: This is a simplified calculation. Real Islamic inheritance
/// can be very complex and requires qualified Islamic scholars (علماء).
pub fn calculate_inheritance(
    estate: Sar,
    heirs: &[(HeritageRelationship, u32)],
) -> Vec<(String, InheritanceShare, Sar)> {
    let has_children = heirs.iter().any(|(rel, _)| {
        matches!(
            rel,
            HeritageRelationship::Son
                | HeritageRelationship::Daughter
                | HeritageRelationship::GrandsonThroughSon
                | HeritageRelationship::GranddaughterThroughSon
        )
    });

    let mut distribution = Vec::new();

    for (relationship, count) in heirs {
        let share = relationship.typical_share(has_children);
        let amount = share.calculate_amount(estate);
        let total_amount = Sar::from_halalas(amount.halalas() * (*count as i64));

        distribution.push((relationship.name_en().to_string(), share, total_amount));
    }

    distribution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inheritance_share_calculation() {
        let estate = Sar::from_riyals(100_000);
        let share = InheritanceShare::Fixed {
            numerator: 1,
            denominator: 2,
        };

        let amount = share.calculate_amount(estate);
        assert_eq!(amount.riyals(), 50_000);
    }

    #[test]
    fn test_husband_share_with_children() {
        let rel = HeritageRelationship::Husband;
        let share = rel.typical_share(true);

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
    fn test_husband_share_no_children() {
        let rel = HeritageRelationship::Husband;
        let share = rel.typical_share(false);

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
    fn test_wife_share_with_children() {
        let rel = HeritageRelationship::Wife;
        let share = rel.typical_share(true);

        match share {
            InheritanceShare::Fixed {
                numerator,
                denominator,
            } => {
                assert_eq!(numerator, 1);
                assert_eq!(denominator, 8); // 1/8 with children
            }
            _ => panic!("Expected fixed share"),
        }
    }

    #[test]
    fn test_inheritance_calculation() {
        let estate = Sar::from_riyals(100_000);
        let heirs = vec![
            (HeritageRelationship::Husband, 1),
            (HeritageRelationship::Daughter, 2),
        ];

        let distribution = calculate_inheritance(estate, &heirs);
        assert!(!distribution.is_empty());
        assert_eq!(distribution.len(), 2);
    }

    #[test]
    fn test_share_as_fraction() {
        let share = InheritanceShare::Fixed {
            numerator: 1,
            denominator: 4,
        };
        assert_eq!(share.as_fraction(), "1/4");

        let residuary = InheritanceShare::Residuary;
        assert!(residuary.as_fraction().contains("Residuary"));
    }

    #[test]
    fn test_relationship_names() {
        assert_eq!(HeritageRelationship::Husband.name_ar(), "زوج");
        assert_eq!(HeritageRelationship::Daughter.name_en(), "Daughter");
    }
}
