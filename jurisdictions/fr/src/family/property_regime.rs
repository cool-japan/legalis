//! Property regime law (Articles 1387-1536).
//!
//! Implements French matrimonial property regimes from the Code civil, including:
//! - Communauté réduite aux acquêts (default regime)
//! - Séparation de biens (separation of property)
//! - Communauté universelle (universal community)

use legalis_core::{Effect, EffectType, Statute};

use super::error::{FamilyLawError, FamilyLawResult};
use super::types::{PACSPropertyRegime, PropertyRegime};

/// Article 1387: Freedom to choose property regime.
///
/// Spouses can choose their property regime by marriage contract.
///
/// # Bilingual
/// - FR: Les époux peuvent régler leurs droits et devoirs matrimoniaux par des conventions matrimoniales (Article 1387).
/// - EN: Spouses can regulate their matrimonial rights and duties by marriage contracts (Article 1387).
#[must_use]
pub fn article1387() -> Statute {
    Statute::new(
        "code-civil-1387",
        "Code civil, Article 1387 - Freedom of marriage contract",
        Effect::new(
            EffectType::Grant,
            "Spouses may choose their property regime by marriage contract",
        ),
    )
    .with_jurisdiction("FR")
}

/// Article 1400: Default property regime (Communauté réduite aux acquêts).
///
/// Since 1966, the default regime is communauté réduite aux acquêts.
/// Property acquired during marriage is jointly owned (communauté).
/// Property owned before marriage or inherited remains separate (biens propres).
///
/// # Bilingual
/// - FR: À défaut de contrat de mariage, les époux sont soumis au régime de la communauté réduite aux acquêts (Article 1400).
/// - EN: In the absence of marriage contract, spouses are subject to the regime of reduced community to acquests (Article 1400).
///
/// # Default since 1966
/// This regime became the default on February 1, 1966, replacing the previous regime.
#[must_use]
pub fn article1400() -> Statute {
    Statute::new(
        "code-civil-1400",
        "Code civil, Article 1400 - Default property regime",
        Effect::new(
            EffectType::Grant,
            "Default regime is communauté réduite aux acquêts (community property)",
        ),
    )
    .with_jurisdiction("FR")
}

/// Article 1401: Composition of community property (acquêts).
///
/// Community property includes all property acquired during marriage by either spouse.
///
/// # Bilingual
/// - FR: La communauté se compose des acquêts faits par les époux ensemble ou séparément durant le mariage (Article 1401).
/// - EN: Community property comprises acquisitions made by spouses together or separately during marriage (Article 1401).
#[must_use]
pub fn article1401() -> Statute {
    Statute::new(
        "code-civil-1401",
        "Code civil, Article 1401 - Community property composition",
        Effect::new(
            EffectType::Grant,
            "Property acquired during marriage forms community property",
        ),
    )
    .with_jurisdiction("FR")
}

/// Article 1404: Separate property (biens propres).
///
/// Separate property includes:
/// - Property owned before marriage
/// - Property inherited or received as gifts during marriage
/// - Property of a personal nature
///
/// # Bilingual
/// - FR: Forment des biens propres les biens dont les époux avaient la propriété avant le mariage (Article 1404).
/// - EN: Separate property comprises property owned by spouses before marriage (Article 1404).
#[must_use]
pub fn article1404() -> Statute {
    Statute::new(
        "code-civil-1404",
        "Code civil, Article 1404 - Separate property definition",
        Effect::new(
            EffectType::Grant,
            "Property owned before marriage or inherited remains separate",
        ),
    )
    .with_jurisdiction("FR")
}

/// Article 1536: Separation of property regime.
///
/// Under separation of property, each spouse retains ownership and management of their own property.
///
/// # Bilingual
/// - FR: Lorsque les époux ont stipulé dans leur contrat de mariage qu'ils seraient séparés de biens (Article 1536).
/// - EN: When spouses have stipulated in their marriage contract that they will be separate as to property (Article 1536).
///
/// # Requirement
/// Requires a marriage contract.
#[must_use]
pub fn article1536() -> Statute {
    Statute::new(
        "code-civil-1536",
        "Code civil, Article 1536 - Separation of property",
        Effect::new(
            EffectType::Grant,
            "Each spouse retains ownership and management of their own property",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate that a property regime requiring marriage contract has one.
///
/// Separation of property and universal community require a marriage contract.
pub fn validate_property_regime_contract(regime: &PropertyRegime) -> FamilyLawResult<()> {
    match regime {
        PropertyRegime::SeparationDeBiens { marriage_contract }
        | PropertyRegime::CommunauteUniverselle { marriage_contract }
        | PropertyRegime::Custom {
            marriage_contract, ..
        } => {
            if !marriage_contract {
                Err(FamilyLawError::MarriageContractRequired {
                    regime: match regime {
                        PropertyRegime::SeparationDeBiens { .. } => {
                            "Séparation de biens".to_string()
                        }
                        PropertyRegime::CommunauteUniverselle { .. } => {
                            "Communauté universelle".to_string()
                        }
                        PropertyRegime::Custom { description, .. } => description.clone(),
                        _ => unreachable!(),
                    },
                })
            } else {
                Ok(())
            }
        }
        PropertyRegime::CommunauteReduite { .. } => {
            // Default regime doesn't require contract
            Ok(())
        }
    }
}

/// Validate PACS property regime.
///
/// PACS has simpler property rules than marriage.
/// Default is separation of property.
pub fn validate_pacs_property_regime(_regime: &PACSPropertyRegime) -> FamilyLawResult<()> {
    // PACS property regimes are always valid
    // Default is separation, joint ownership is allowed by agreement
    Ok(())
}

/// Check if property regime is the default regime.
#[must_use]
pub fn is_default_regime(regime: &PropertyRegime) -> bool {
    matches!(
        regime,
        PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            ..
        }
    )
}

/// Get regime name in French.
#[must_use]
pub fn regime_name_fr(regime: &PropertyRegime) -> String {
    match regime {
        PropertyRegime::CommunauteReduite { .. } => "Communauté réduite aux acquêts".to_string(),
        PropertyRegime::SeparationDeBiens { .. } => "Séparation de biens".to_string(),
        PropertyRegime::CommunauteUniverselle { .. } => "Communauté universelle".to_string(),
        PropertyRegime::Custom { description, .. } => description.clone(),
    }
}

/// Get regime name in English.
#[must_use]
pub fn regime_name_en(regime: &PropertyRegime) -> String {
    match regime {
        PropertyRegime::CommunauteReduite { .. } => "Community of acquisitions".to_string(),
        PropertyRegime::SeparationDeBiens { .. } => "Separation of property".to_string(),
        PropertyRegime::CommunauteUniverselle { .. } => "Universal community".to_string(),
        PropertyRegime::Custom { description, .. } => format!("Custom regime: {}", description),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::family::types::Asset;
    use chrono::Utc;

    #[test]
    fn test_article1387_statute_creation() {
        let statute = article1387();
        assert_eq!(statute.id, "code-civil-1387");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert!(statute.title.contains("1387"));
    }

    #[test]
    fn test_article1400_statute_creation() {
        let statute = article1400();
        assert_eq!(statute.id, "code-civil-1400");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert!(statute.title.contains("Default"));
    }

    #[test]
    fn test_validate_default_regime_no_contract_required() {
        let regime = PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        };

        assert!(validate_property_regime_contract(&regime).is_ok());
    }

    #[test]
    fn test_validate_separation_requires_contract() {
        let regime = PropertyRegime::SeparationDeBiens {
            marriage_contract: false,
        };

        let result = validate_property_regime_contract(&regime);
        assert!(result.is_err());
        match result.unwrap_err() {
            FamilyLawError::MarriageContractRequired { regime: r } => {
                assert!(r.contains("Séparation"));
            }
            _ => panic!("Expected MarriageContractRequired error"),
        }
    }

    #[test]
    fn test_validate_separation_with_contract() {
        let regime = PropertyRegime::SeparationDeBiens {
            marriage_contract: true,
        };

        assert!(validate_property_regime_contract(&regime).is_ok());
    }

    #[test]
    fn test_validate_universal_community_requires_contract() {
        let regime = PropertyRegime::CommunauteUniverselle {
            marriage_contract: false,
        };

        let result = validate_property_regime_contract(&regime);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::MarriageContractRequired { .. }
        ));
    }

    #[test]
    fn test_validate_custom_regime_requires_contract() {
        let regime = PropertyRegime::Custom {
            description: "Custom regime".to_string(),
            marriage_contract: false,
        };

        let result = validate_property_regime_contract(&regime);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_pacs_property_regime() {
        assert!(validate_pacs_property_regime(&PACSPropertyRegime::Separation).is_ok());
        assert!(validate_pacs_property_regime(&PACSPropertyRegime::Joint).is_ok());
    }

    #[test]
    fn test_is_default_regime() {
        let default_regime = PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        };
        assert!(is_default_regime(&default_regime));

        let explicit_regime = PropertyRegime::CommunauteReduite {
            marriage_contract: true,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        };
        assert!(!is_default_regime(&explicit_regime));

        let separation = PropertyRegime::SeparationDeBiens {
            marriage_contract: true,
        };
        assert!(!is_default_regime(&separation));
    }

    #[test]
    fn test_regime_name_fr() {
        let communaute = PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        };
        assert_eq!(
            regime_name_fr(&communaute),
            "Communauté réduite aux acquêts"
        );

        let separation = PropertyRegime::SeparationDeBiens {
            marriage_contract: true,
        };
        assert_eq!(regime_name_fr(&separation), "Séparation de biens");
    }

    #[test]
    fn test_regime_name_en() {
        let communaute = PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        };
        assert_eq!(regime_name_en(&communaute), "Community of acquisitions");

        let separation = PropertyRegime::SeparationDeBiens {
            marriage_contract: true,
        };
        assert_eq!(regime_name_en(&separation), "Separation of property");
    }

    #[test]
    fn test_property_classification() {
        let asset1 = Asset::new(
            "Apartment bought during marriage".to_string(),
            250_000,
            Utc::now().naive_utc().date(),
        );

        let asset2 = Asset::new(
            "Family heirloom".to_string(),
            50_000,
            Utc::now().naive_utc().date(),
        );

        let regime = PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: vec![asset1],
            biens_propres: vec![("Alice".to_string(), asset2)],
        };

        match regime {
            PropertyRegime::CommunauteReduite {
                acquets,
                biens_propres,
                ..
            } => {
                assert_eq!(acquets.len(), 1);
                assert_eq!(biens_propres.len(), 1);
                assert_eq!(acquets[0].value, 250_000);
                assert_eq!(biens_propres[0].1.value, 50_000);
            }
            _ => panic!("Expected CommunauteReduite"),
        }
    }
}
