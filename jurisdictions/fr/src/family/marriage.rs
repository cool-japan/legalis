//! Marriage law (Articles 143-180).
//!
//! Implements French marriage law from the Code civil, including:
//! - Marriage requirements (age, consent, capacity)
//! - Banns publication (Article 161)
//! - Marriage oppositions (Article 165)
//! - Prohibitions (consanguinity, affinity)
//! - Marriage nullity (absolute and relative)

use legalis_core::{Effect, EffectType, Statute};

use super::error::{FamilyLawError, FamilyLawResult};
use super::types::{Marriage, Nationality, Relationship};

/// Article 143: Marriage can be contracted between two persons of different sex or of the same sex.
///
/// Since the law of May 17, 2013, same-sex marriage is legal in France.
///
/// # Bilingual
/// - FR: Le mariage peut être contracté entre deux personnes de sexe différent ou de même sexe.
/// - EN: Marriage can be contracted between two persons of different sex or of the same sex.
#[must_use]
pub fn article143() -> Statute {
    Statute::new(
        "code-civil-143",
        "Code civil, Article 143 - Marriage capacity",
        Effect::new(
            EffectType::Grant,
            "Marriage can be contracted between persons of any sex",
        ),
    )
    .with_jurisdiction("FR")
}

/// Article 144: Minimum age for marriage is 18 years.
///
/// Previously, the minimum age was 15 for women and 18 for men. Since 2006, it is 18 for everyone.
///
/// # Bilingual
/// - FR: L'âge minimum pour se marier est de dix-huit ans.
/// - EN: The minimum age for marriage is eighteen years.
///
/// # Example
/// ```rust,ignore
/// use legalis_fr::family::{Person, Nationality, MaritalStatus};
///
/// let person = Person::new("Alice".to_string(), 17, Nationality::French, MaritalStatus::Single);
/// // This would violate Article 144
/// ```
#[must_use]
pub fn article144() -> Statute {
    Statute::new(
        "code-civil-144",
        "Code civil, Article 144 - Minimum marriage age",
        Effect::new(
            EffectType::Prohibition,
            "Marriage prohibited if either party is under 18 years old",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate minimum age requirement (Article 144).
///
/// Both parties must be at least 18 years old.
pub fn validate_minimum_age(marriage: &Marriage) -> FamilyLawResult<()> {
    let mut errors = Vec::new();

    for party in &marriage.parties {
        if party.age < 18 {
            errors.push(FamilyLawError::InsufficientAge {
                actual_age: party.age,
                required_age: 18,
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(FamilyLawError::MultipleErrors(errors))
    }
}

/// Article 146: There is no marriage without consent.
///
/// Free and full consent of both spouses is essential for marriage validity.
///
/// # Bilingual
/// - FR: Il n'y a pas de mariage sans consentement.
/// - EN: There is no marriage without consent.
///
/// # Nullity
/// Lack of consent leads to relative nullity (Article 180).
#[must_use]
pub fn article146() -> Statute {
    Statute::new(
        "code-civil-146",
        "Code civil, Article 146 - Marriage consent",
        Effect::new(
            EffectType::Prohibition,
            "Marriage prohibited without free and full consent of both parties",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate consent requirement (Article 146).
///
/// Both parties must give free and full consent.
pub fn validate_consent(marriage: &Marriage) -> FamilyLawResult<()> {
    let mut errors = Vec::new();

    for (i, consent_given) in marriage.consent_given.iter().enumerate() {
        if !consent_given {
            errors.push(FamilyLawError::NoConsent {
                party: marriage.parties[i].name.clone(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(FamilyLawError::MultipleErrors(errors))
    }
}

/// Article 146-1: Marriage must be contracted with personal presence (for French nationals).
///
/// Since 2006, proxy marriages are prohibited for French nationals.
///
/// # Bilingual
/// - FR: Le mariage d'un Français, même contracté à l'étranger, requiert sa présence (Article 146-1).
/// - EN: Marriage of a French national, even contracted abroad, requires their presence (Article 146-1).
#[must_use]
pub fn article146_1() -> Statute {
    Statute::new(
        "code-civil-146-1",
        "Code civil, Article 146-1 - Personal presence",
        Effect::new(
            EffectType::Prohibition,
            "Proxy marriage prohibited for French nationals",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate personal presence requirement (Article 146-1).
///
/// Proxy marriage is prohibited if either party is a French national.
pub fn validate_personal_presence(marriage: &Marriage) -> FamilyLawResult<()> {
    if !marriage.proxy_used {
        return Ok(());
    }

    // Check if either party is French
    let has_french_national = marriage
        .parties
        .iter()
        .any(|p| matches!(p.nationality, Nationality::French));

    if has_french_national {
        Err(FamilyLawError::ProxyMarriageProhibited)
    } else {
        Ok(())
    }
}

/// Article 147: Prohibition of bigamy.
///
/// One cannot contract a second marriage before dissolution of the first.
///
/// # Bilingual
/// - FR: On ne peut contracter un second mariage avant la dissolution du premier (Article 147).
/// - EN: One cannot contract a second marriage before dissolution of the first (Article 147).
///
/// # Nullity
/// Bigamy leads to absolute nullity (Article 180, 184).
#[must_use]
pub fn article147() -> Statute {
    Statute::new(
        "code-civil-147",
        "Code civil, Article 147 - Prohibition of bigamy",
        Effect::new(
            EffectType::Prohibition,
            "Marriage prohibited if either party is already married",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate bigamy prohibition (Article 147).
///
/// Neither party can be already married.
pub fn validate_no_bigamy(marriage: &Marriage) -> FamilyLawResult<()> {
    use super::types::MaritalStatus;

    for party in &marriage.parties {
        if matches!(
            party.marital_status,
            MaritalStatus::Married | MaritalStatus::PACS
        ) {
            return Err(FamilyLawError::Bigamy {
                existing_marriage_date: "unknown".to_string(),
            });
        }
    }

    Ok(())
}

/// Article 161: Publication of banns.
///
/// Marriage must be preceded by publication of banns at least 10 days before the ceremony.
///
/// # Bilingual
/// - FR: Le mariage doit être précédé d'une publication des bans (Article 161).
/// - EN: Marriage must be preceded by publication of banns (Article 161).
///
/// # Requirement
/// Banns must be published at least 10 days before the ceremony.
#[must_use]
pub fn article161() -> Statute {
    Statute::new(
        "code-civil-161",
        "Code civil, Article 161 - Publication of banns",
        Effect::new(
            EffectType::Obligation,
            "Marriage must be preceded by publication of banns at least 10 days before ceremony",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate banns publication (Article 161).
///
/// Banns must be published, and at least 10 days must elapse before marriage.
pub fn validate_banns_publication(marriage: &Marriage) -> FamilyLawResult<()> {
    if !marriage.banns_published {
        return Err(FamilyLawError::BannsNotPublished);
    }

    if let (Some(pub_date), Some(marriage_date)) =
        (marriage.banns_publication_date, marriage.marriage_date)
    {
        let duration = marriage_date.signed_duration_since(pub_date);
        let days_elapsed = duration.num_days();

        if days_elapsed < 10 {
            return Err(FamilyLawError::BannsPublishedTooRecently {
                days_elapsed: days_elapsed as u32,
            });
        }
    }

    Ok(())
}

/// Article 165: Opposition to marriage.
///
/// Persons with legitimate interest can file opposition to marriage.
///
/// # Bilingual
/// - FR: Le droit de former opposition à la célébration du mariage appartient aux personnes ayant un intérêt légitime (Article 165).
/// - EN: The right to file opposition to the celebration of marriage belongs to persons with legitimate interest (Article 165).
///
/// # Grounds
/// Grounds for opposition include:
/// - Existing marriage (bigamy)
/// - Insufficient age
/// - Lack of consent
/// - Consanguinity or affinity
#[must_use]
pub fn article165() -> Statute {
    Statute::new(
        "code-civil-165",
        "Code civil, Article 165 - Opposition to marriage",
        Effect::new(
            EffectType::Grant,
            "Right to file opposition to marriage for persons with legitimate interest",
        ),
    )
    .with_jurisdiction("FR")
}

/// Check for marriage oppositions (Article 165).
///
/// Returns error if unresolved oppositions exist.
pub fn check_oppositions(marriage: &Marriage) -> FamilyLawResult<()> {
    if !marriage.oppositions.is_empty() {
        let grounds: Vec<String> = marriage
            .oppositions
            .iter()
            .map(|o| o.ground.to_string())
            .collect();

        Err(FamilyLawError::MarriageOpposition { grounds })
    } else {
        Ok(())
    }
}

/// Article 180: Marriage nullity.
///
/// Marriage can be declared null in certain cases:
/// - Absolute nullity: Bigamy, incest, lack of minimum age without dispensation
/// - Relative nullity: Vice of consent, error about person, violence
///
/// # Bilingual
/// - FR: Le mariage peut être déclaré nul dans certains cas (Article 180).
/// - EN: Marriage can be declared null in certain cases (Article 180).
///
/// # Distinction
/// - Absolute nullity: Can be invoked by anyone, no time limit
/// - Relative nullity: Only affected spouse can invoke, within time limit
#[must_use]
pub fn article180() -> Statute {
    Statute::new(
        "code-civil-180",
        "Code civil, Article 180 - Marriage nullity",
        Effect::new(
            EffectType::Prohibition,
            "Marriage can be declared null for absolute or relative causes",
        ),
    )
    .with_jurisdiction("FR")
}

/// Validate consanguinity prohibition.
///
/// Marriage is prohibited between:
/// - Direct ascendants/descendants
/// - Siblings
///
/// This leads to absolute nullity.
pub fn validate_no_consanguinity(marriage: &Marriage) -> FamilyLawResult<()> {
    for party in &marriage.parties {
        if let Some(rel) = &party.relationship_to_other {
            match rel {
                Relationship::DirectAscendant
                | Relationship::DirectDescendant
                | Relationship::Sibling => {
                    return Err(FamilyLawError::Incest {
                        relationship: rel.to_string(),
                    });
                }
                _ => {}
            }
        }
    }

    Ok(())
}

/// Comprehensive marriage validation.
///
/// Checks all requirements from Articles 143-180:
/// - Minimum age (Article 144)
/// - Consent (Article 146)
/// - Personal presence (Article 146-1)
/// - No bigamy (Article 147)
/// - Banns publication (Article 161)
/// - No oppositions (Article 165)
/// - No consanguinity (Article 180)
///
/// Returns all violations found.
pub fn validate_marriage_conditions(marriage: &Marriage) -> FamilyLawResult<()> {
    let mut errors = Vec::new();

    // Article 144: Minimum age
    if let Err(e) = validate_minimum_age(marriage) {
        match e {
            FamilyLawError::MultipleErrors(errs) => errors.extend(errs),
            other => errors.push(other),
        }
    }

    // Article 146: Consent
    if let Err(e) = validate_consent(marriage) {
        match e {
            FamilyLawError::MultipleErrors(errs) => errors.extend(errs),
            other => errors.push(other),
        }
    }

    // Article 146-1: Personal presence
    if let Err(e) = validate_personal_presence(marriage) {
        errors.push(e);
    }

    // Article 147: No bigamy
    if let Err(e) = validate_no_bigamy(marriage) {
        errors.push(e);
    }

    // Article 161: Banns publication (only if marriage date is set)
    if marriage.marriage_date.is_some() {
        if let Err(e) = validate_banns_publication(marriage) {
            errors.push(e);
        }
    }

    // Article 165: No oppositions
    if let Err(e) = check_oppositions(marriage) {
        errors.push(e);
    }

    // Consanguinity prohibition
    if let Err(e) = validate_no_consanguinity(marriage) {
        errors.push(e);
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(FamilyLawError::MultipleErrors(errors))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::family::types::{MaritalStatus, MarriageOpposition, OppositionGround, Person};
    use chrono::Duration;

    #[test]
    fn test_article144_statute_creation() {
        let statute = article144();
        assert_eq!(statute.id, "code-civil-144");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert!(statute.title.contains("144"));
    }

    #[test]
    fn test_validate_minimum_age_valid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2);

        assert!(validate_minimum_age(&marriage).is_ok());
    }

    #[test]
    fn test_validate_minimum_age_invalid() {
        let person1 = Person::new(
            "Alice".to_string(),
            17,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2);

        let result = validate_minimum_age(&marriage);
        assert!(result.is_err());
        match result.unwrap_err() {
            FamilyLawError::InsufficientAge {
                actual_age,
                required_age,
            } => {
                assert_eq!(actual_age, 17);
                assert_eq!(required_age, 18);
            }
            _ => panic!("Expected InsufficientAge error"),
        }
    }

    #[test]
    fn test_validate_consent_valid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2).with_consent([true, true]);

        assert!(validate_consent(&marriage).is_ok());
    }

    #[test]
    fn test_validate_consent_invalid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2).with_consent([true, false]);

        let result = validate_consent(&marriage);
        assert!(result.is_err());
        match result.unwrap_err() {
            FamilyLawError::NoConsent { party } => {
                assert_eq!(party, "Bob");
            }
            _ => panic!("Expected NoConsent error"),
        }
    }

    #[test]
    fn test_validate_personal_presence_valid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2).with_proxy_used(false);

        assert!(validate_personal_presence(&marriage).is_ok());
    }

    #[test]
    fn test_validate_personal_presence_proxy_foreign_ok() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::Foreign("Belgium".to_string()),
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::Foreign("Germany".to_string()),
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2).with_proxy_used(true);

        // Proxy allowed for foreign nationals
        assert!(validate_personal_presence(&marriage).is_ok());
    }

    #[test]
    fn test_validate_personal_presence_proxy_french_prohibited() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::Foreign("Germany".to_string()),
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2).with_proxy_used(true);

        let result = validate_personal_presence(&marriage);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::ProxyMarriageProhibited
        ));
    }

    #[test]
    fn test_validate_no_bigamy_valid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Divorced,
        );

        let marriage = Marriage::new(person1, person2);

        assert!(validate_no_bigamy(&marriage).is_ok());
    }

    #[test]
    fn test_validate_no_bigamy_invalid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Married,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2);

        let result = validate_no_bigamy(&marriage);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FamilyLawError::Bigamy { .. }));
    }

    #[test]
    fn test_validate_banns_publication_valid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let pub_date = chrono::Utc::now().naive_utc().date() - Duration::days(15);
        let marriage_date = chrono::Utc::now().naive_utc().date();

        let marriage = Marriage::new(person1, person2)
            .with_banns_published(true)
            .with_banns_publication_date(pub_date)
            .with_marriage_date(marriage_date);

        assert!(validate_banns_publication(&marriage).is_ok());
    }

    #[test]
    fn test_validate_banns_publication_not_published() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2).with_banns_published(false);

        let result = validate_banns_publication(&marriage);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::BannsNotPublished
        ));
    }

    #[test]
    fn test_validate_banns_publication_too_recent() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let pub_date = chrono::Utc::now().naive_utc().date() - Duration::days(5);
        let marriage_date = chrono::Utc::now().naive_utc().date();

        let marriage = Marriage::new(person1, person2)
            .with_banns_published(true)
            .with_banns_publication_date(pub_date)
            .with_marriage_date(marriage_date);

        let result = validate_banns_publication(&marriage);
        assert!(result.is_err());
        match result.unwrap_err() {
            FamilyLawError::BannsPublishedTooRecently { days_elapsed } => {
                assert_eq!(days_elapsed, 5);
            }
            _ => panic!("Expected BannsPublishedTooRecently error"),
        }
    }

    #[test]
    fn test_check_oppositions_none() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2);

        assert!(check_oppositions(&marriage).is_ok());
    }

    #[test]
    fn test_check_oppositions_exists() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let opposition = MarriageOpposition {
            ground: OppositionGround::InsufficientAge,
            filed_by: "Parent".to_string(),
            filed_date: chrono::Utc::now().naive_utc().date(),
        };

        let marriage = Marriage::new(person1, person2).with_opposition(opposition);

        let result = check_oppositions(&marriage);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::MarriageOpposition { .. }
        ));
    }

    #[test]
    fn test_validate_no_consanguinity_valid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2);

        assert!(validate_no_consanguinity(&marriage).is_ok());
    }

    #[test]
    fn test_validate_no_consanguinity_siblings_prohibited() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        )
        .with_relationship(Relationship::Sibling);
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2);

        let result = validate_no_consanguinity(&marriage);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FamilyLawError::Incest { .. }));
    }

    #[test]
    fn test_validate_marriage_conditions_fully_valid() {
        let person1 = Person::new(
            "Alice".to_string(),
            25,
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let pub_date = chrono::Utc::now().naive_utc().date() - Duration::days(15);
        let marriage_date = chrono::Utc::now().naive_utc().date();

        let marriage = Marriage::new(person1, person2)
            .with_consent([true, true])
            .with_banns_published(true)
            .with_banns_publication_date(pub_date)
            .with_marriage_date(marriage_date);

        assert!(validate_marriage_conditions(&marriage).is_ok());
    }

    #[test]
    fn test_validate_marriage_conditions_multiple_errors() {
        let person1 = Person::new(
            "Alice".to_string(),
            17, // Too young
            Nationality::French,
            MaritalStatus::Single,
        );
        let person2 = Person::new(
            "Bob".to_string(),
            27,
            Nationality::French,
            MaritalStatus::Single,
        );

        let marriage = Marriage::new(person1, person2)
            .with_consent([true, false]) // Bob doesn't consent
            .with_proxy_used(true); // Proxy prohibited for French

        let result = validate_marriage_conditions(&marriage);
        assert!(result.is_err());
        match result.unwrap_err() {
            FamilyLawError::MultipleErrors(errors) => {
                assert!(errors.len() >= 2); // At least age and consent errors
            }
            _ => panic!("Expected MultipleErrors"),
        }
    }
}
