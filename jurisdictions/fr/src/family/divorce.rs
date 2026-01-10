//! Divorce law (Articles 229-247).
//!
//! Implements French divorce law from the Code civil, including:
//! - Mutual consent divorce (Article 230)
//! - Divorce by acceptance of the principle (Article 233)
//! - Divorce for definitive alteration (Article 237)
//! - Fault-based divorce (Article 242)

use legalis_core::{Effect, EffectType, Statute};

use super::error::{FamilyLawError, FamilyLawResult};
use super::types::{Divorce, DivorceType};

/// Article 229: Four types of divorce.
///
/// Under French law, divorce can be granted for four reasons:
/// 1. Mutual consent (consentement mutuel)
/// 2. Acceptance of the principle (acceptation du principe)
/// 3. Definitive alteration of the marriage bond (altération définitive du lien conjugal)
/// 4. Fault (faute)
///
/// # Bilingual
/// - FR: Le divorce peut être prononcé pour consentement mutuel, acceptation du principe, altération définitive du lien conjugal, ou faute.
/// - EN: Divorce can be granted for mutual consent, acceptance of the principle, definitive alteration, or fault.
///
/// # Reform 2017
/// Since 2017, mutual consent divorce is simplified and handled by notary without court involvement.
#[must_use]
pub fn article229() -> Statute {
    Statute::new(
        "code-civil-229",
        "Code civil, Article 229 - Four types of divorce",
        Effect::new(
            EffectType::Grant,
            "Divorce can be granted for four reasons: mutual consent, acceptance, alteration, or fault",
        ),
    )
    .with_jurisdiction("FR")
}

/// Article 230: Mutual consent divorce.
///
/// Divorce by mutual consent is based on agreement between spouses.
/// Since 2017 reform, it is handled by notary without court involvement.
///
/// # Bilingual
/// - FR: Le divorce par consentement mutuel est fondé sur un accord entre les époux (Article 230).
/// - EN: Divorce by mutual consent is based on agreement between spouses (Article 230).
///
/// # Requirements
/// - Written agreement signed by both parties
/// - Each party assisted by a lawyer
/// - Notary filing
/// - Children hearing (if requested)
#[must_use]
pub fn article230() -> Statute {
    Statute::new(
        "code-civil-230",
        "Code civil, Article 230 - Mutual consent divorce",
        Effect::new(
            EffectType::Grant,
            "Divorce granted by mutual consent with notary filing",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate mutual consent divorce (Article 230).
///
/// Requirements:
/// - Agreement signed by both parties
/// - Notary filing date set
/// - Children heard if capable of discernment
pub fn validate_mutual_consent_divorce(divorce: &Divorce) -> FamilyLawResult<()> {
    if let DivorceType::MutualConsent {
        agreement_signed,
        notary_filing_date,
        children_heard,
    } = &divorce.divorce_type
    {
        let mut errors = Vec::new();

        if !agreement_signed {
            errors.push(FamilyLawError::AgreementNotSigned);
        }

        if notary_filing_date.is_none() {
            errors.push(FamilyLawError::NotaryFilingRequired);
        }

        // Check if children hearing is required
        let has_minor_children = divorce.children.iter().any(|c| c.age < 18);
        if has_minor_children && !children_heard {
            // Children have right to be heard if capable of discernment
            errors.push(FamilyLawError::ChildHearingRequired);
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(FamilyLawError::MultipleErrors(errors))
        }
    } else {
        Ok(()) // Not a mutual consent divorce
    }
}

/// Article 233: Divorce by acceptance of the principle.
///
/// Both spouses accept the principle of divorce but disagree on consequences.
///
/// # Bilingual
/// - FR: Les époux acceptent le principe de la rupture du mariage mais règlent les conséquences devant le juge (Article 233).
/// - EN: Spouses accept the principle of marriage breakdown but settle consequences before judge (Article 233).
#[must_use]
pub fn article233() -> Statute {
    Statute::new(
        "code-civil-233",
        "Code civil, Article 233 - Divorce by acceptance",
        Effect::new(
            EffectType::Grant,
            "Divorce granted when both parties accept principle despite disagreement on consequences",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate divorce by acceptance of the principle (Article 233).
///
/// Both parties must accept the divorce principle.
pub fn validate_acceptance_principle_divorce(divorce: &Divorce) -> FamilyLawResult<()> {
    if let DivorceType::AcceptancePrinciple {
        both_accept_principle,
        disagreement_on_consequences: _,
    } = &divorce.divorce_type
    {
        if !both_accept_principle {
            Err(FamilyLawError::PrincipleNotAccepted)
        } else {
            Ok(())
        }
    } else {
        Ok(()) // Not an acceptance principle divorce
    }
}

/// Article 237: Divorce for definitive alteration of the marriage bond.
///
/// Divorce can be granted after continuous separation of at least 24 months.
///
/// # Bilingual
/// - FR: Le divorce pour altération définitive du lien conjugal peut être demandé après une séparation de fait de deux ans (Article 237).
/// - EN: Divorce for definitive alteration can be requested after a de facto separation of two years (Article 237).
///
/// # Requirement
/// - At least 24 months of continuous separation
/// - Can be requested by either spouse
#[must_use]
pub fn article237() -> Statute {
    Statute::new(
        "code-civil-237",
        "Code civil, Article 237 - Divorce for definitive alteration",
        Effect::new(
            EffectType::Grant,
            "Divorce granted after 24 months of continuous separation",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate divorce for definitive alteration (Article 237).
///
/// Requires at least 24 months of continuous separation.
pub fn validate_definitive_alteration_divorce(divorce: &Divorce) -> FamilyLawResult<()> {
    if let DivorceType::DefinitiveAlteration {
        separation_duration_months,
    } = &divorce.divorce_type
    {
        if *separation_duration_months < 24 {
            Err(FamilyLawError::InsufficientSeparation {
                months_elapsed: *separation_duration_months,
                required_months: 24,
            })
        } else {
            Ok(())
        }
    } else {
        Ok(()) // Not a definitive alteration divorce
    }
}

/// Article 242: Fault-based divorce.
///
/// Divorce can be granted due to serious breach of marital duties by one spouse.
///
/// # Bilingual
/// - FR: Le divorce pour faute peut être demandé en cas de violation grave ou renouvelée des devoirs du mariage (Article 242).
/// - EN: Fault-based divorce can be requested in case of serious or repeated breach of marital duties (Article 242).
///
/// # Grounds
/// - Violence
/// - Adultery
/// - Severe breach of marital duties
/// - Other serious faults
#[must_use]
pub fn article242() -> Statute {
    Statute::new(
        "code-civil-242",
        "Code civil, Article 242 - Fault-based divorce",
        Effect::new(
            EffectType::Grant,
            "Divorce granted for serious breach of marital duties",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate fault-based divorce (Article 242).
///
/// Requires evidence of fault.
pub fn validate_fault_divorce(divorce: &Divorce) -> FamilyLawResult<()> {
    if let DivorceType::Fault {
        fault_type: _,
        evidence,
    } = &divorce.divorce_type
    {
        if evidence.is_empty() {
            Err(FamilyLawError::NoFaultEvidence)
        } else {
            Ok(())
        }
    } else {
        Ok(()) // Not a fault divorce
    }
}

/// Article 247: Effects of divorce.
///
/// Divorce dissolves the marriage from the date of judgment.
///
/// # Bilingual
/// - FR: Le divorce dissout le mariage à la date du jugement (Article 247).
/// - EN: Divorce dissolves marriage from the date of judgment (Article 247).
#[must_use]
pub fn article247() -> Statute {
    Statute::new(
        "code-civil-247",
        "Code civil, Article 247 - Effects of divorce",
        Effect::new(
            EffectType::Grant,
            "Divorce dissolves marriage from date of judgment",
        ),
    )
    .with_jurisdiction("FR")
}

/// Comprehensive divorce validation.
///
/// Validates divorce proceedings according to the type:
/// - Mutual consent (Article 230)
/// - Acceptance of principle (Article 233)
/// - Definitive alteration (Article 237)
/// - Fault (Article 242)
///
/// Returns all violations found.
pub fn validate_divorce_proceedings(divorce: &Divorce) -> FamilyLawResult<()> {
    match &divorce.divorce_type {
        DivorceType::MutualConsent { .. } => validate_mutual_consent_divorce(divorce),
        DivorceType::AcceptancePrinciple { .. } => validate_acceptance_principle_divorce(divorce),
        DivorceType::DefinitiveAlteration { .. } => validate_definitive_alteration_divorce(divorce),
        DivorceType::Fault { .. } => validate_fault_divorce(divorce),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::family::types::{Child, FaultType, PropertyRegime};
    use chrono::{Duration, Utc};

    #[test]
    fn test_article229_statute_creation() {
        let statute = article229();
        assert_eq!(statute.id, "code-civil-229");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert!(statute.title.contains("229"));
    }

    #[test]
    fn test_validate_mutual_consent_valid() {
        let divorce_type = DivorceType::MutualConsent {
            agreement_signed: true,
            notary_filing_date: Some(Utc::now().naive_utc().date()),
            children_heard: true,
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        assert!(validate_mutual_consent_divorce(&divorce).is_ok());
    }

    #[test]
    fn test_validate_mutual_consent_no_agreement() {
        let divorce_type = DivorceType::MutualConsent {
            agreement_signed: false,
            notary_filing_date: Some(Utc::now().naive_utc().date()),
            children_heard: true,
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        let result = validate_mutual_consent_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::AgreementNotSigned
        ));
    }

    #[test]
    fn test_validate_mutual_consent_no_notary() {
        let divorce_type = DivorceType::MutualConsent {
            agreement_signed: true,
            notary_filing_date: None,
            children_heard: true,
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::SeparationDeBiens {
                marriage_contract: true,
            },
        );

        let result = validate_mutual_consent_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::NotaryFilingRequired
        ));
    }

    #[test]
    fn test_validate_mutual_consent_child_hearing_required() {
        let divorce_type = DivorceType::MutualConsent {
            agreement_signed: true,
            notary_filing_date: Some(Utc::now().naive_utc().date()),
            children_heard: false,
        };

        let child = Child::new("Charlie".to_string(), 12); // Minor child

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        )
        .with_child(child);

        let result = validate_mutual_consent_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::ChildHearingRequired
        ));
    }

    #[test]
    fn test_validate_acceptance_principle_valid() {
        let divorce_type = DivorceType::AcceptancePrinciple {
            both_accept_principle: true,
            disagreement_on_consequences: true,
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        assert!(validate_acceptance_principle_divorce(&divorce).is_ok());
    }

    #[test]
    fn test_validate_acceptance_principle_not_accepted() {
        let divorce_type = DivorceType::AcceptancePrinciple {
            both_accept_principle: false,
            disagreement_on_consequences: true,
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        let result = validate_acceptance_principle_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::PrincipleNotAccepted
        ));
    }

    #[test]
    fn test_validate_definitive_alteration_valid() {
        let divorce_type = DivorceType::DefinitiveAlteration {
            separation_duration_months: 30,
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        assert!(validate_definitive_alteration_divorce(&divorce).is_ok());
    }

    #[test]
    fn test_validate_definitive_alteration_insufficient() {
        let divorce_type = DivorceType::DefinitiveAlteration {
            separation_duration_months: 18,
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        let result = validate_definitive_alteration_divorce(&divorce);
        assert!(result.is_err());
        match result.unwrap_err() {
            FamilyLawError::InsufficientSeparation {
                months_elapsed,
                required_months,
            } => {
                assert_eq!(months_elapsed, 18);
                assert_eq!(required_months, 24);
            }
            _ => panic!("Expected InsufficientSeparation error"),
        }
    }

    #[test]
    fn test_validate_fault_divorce_valid() {
        let divorce_type = DivorceType::Fault {
            fault_type: FaultType::Violence,
            evidence: vec![
                "Police report".to_string(),
                "Medical certificate".to_string(),
            ],
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        assert!(validate_fault_divorce(&divorce).is_ok());
    }

    #[test]
    fn test_validate_fault_divorce_no_evidence() {
        let divorce_type = DivorceType::Fault {
            fault_type: FaultType::Adultery,
            evidence: Vec::new(),
        };

        let divorce = Divorce::new(
            divorce_type,
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        let result = validate_fault_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::NoFaultEvidence
        ));
    }

    #[test]
    fn test_validate_divorce_proceedings_all_types() {
        // Test mutual consent
        let divorce1 = Divorce::new(
            DivorceType::MutualConsent {
                agreement_signed: true,
                notary_filing_date: Some(Utc::now().naive_utc().date()),
                children_heard: true,
            },
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );
        assert!(validate_divorce_proceedings(&divorce1).is_ok());

        // Test acceptance
        let divorce2 = Divorce::new(
            DivorceType::AcceptancePrinciple {
                both_accept_principle: true,
                disagreement_on_consequences: true,
            },
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );
        assert!(validate_divorce_proceedings(&divorce2).is_ok());

        // Test alteration
        let divorce3 = Divorce::new(
            DivorceType::DefinitiveAlteration {
                separation_duration_months: 30,
            },
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );
        assert!(validate_divorce_proceedings(&divorce3).is_ok());

        // Test fault
        let divorce4 = Divorce::new(
            DivorceType::Fault {
                fault_type: FaultType::Violence,
                evidence: vec!["Evidence".to_string()],
            },
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );
        assert!(validate_divorce_proceedings(&divorce4).is_ok());
    }
}
