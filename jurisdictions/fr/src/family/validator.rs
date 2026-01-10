//! Comprehensive validators for family law.
//!
//! Provides high-level validation functions that combine multiple checks.

use super::divorce::validate_divorce_proceedings;
use super::error::{FamilyLawError, FamilyLawResult};
use super::marriage::validate_marriage_conditions;
use super::property_regime::validate_property_regime_contract;
use super::types::{Divorce, Marriage, PACS, PropertyRegime};

/// Validate all aspects of a marriage.
///
/// This is a convenience function that performs comprehensive marriage validation.
///
/// # Validation includes
/// - All marriage requirements (age, consent, banns, etc.)
/// - Consanguinity checks
/// - Bigamy prohibition
/// - Personal presence requirements
///
/// # Example
/// ```rust,ignore
/// use legalis_fr::family::{Marriage, Person, Nationality, MaritalStatus, validate_marriage};
///
/// let marriage = Marriage::new(
///     Person::new("Alice".to_string(), 25, Nationality::French, MaritalStatus::Single),
///     Person::new("Bob".to_string(), 27, Nationality::French, MaritalStatus::Single),
/// )
/// .with_consent([true, true])
/// .with_banns_published(true);
///
/// match validate_marriage(&marriage) {
///     Ok(()) => println!("Marriage is valid"),
///     Err(e) => println!("Marriage validation failed: {}", e),
/// }
/// ```
pub fn validate_marriage(marriage: &Marriage) -> FamilyLawResult<()> {
    validate_marriage_conditions(marriage)
}

/// Validate all aspects of a divorce.
///
/// This is a convenience function that performs comprehensive divorce validation
/// based on the divorce type.
///
/// # Validation includes
/// - Type-specific requirements (mutual consent, acceptance, alteration, fault)
/// - Children hearing requirements
/// - Separation duration for alteration divorce
/// - Evidence for fault divorce
///
/// # Example
/// ```rust,ignore
/// use legalis_fr::family::{Divorce, DivorceType, PropertyRegime, validate_divorce};
///
/// let divorce = Divorce::new(
///     DivorceType::MutualConsent {
///         agreement_signed: true,
///         notary_filing_date: Some(chrono::Utc::now().naive_utc().date()),
///         children_heard: true,
///     },
///     chrono::Utc::now().naive_utc().date(),
///     "Alice".to_string(),
///     "Bob".to_string(),
///     PropertyRegime::CommunauteReduite {
///         marriage_contract: false,
///         acquets: Vec::new(),
///         biens_propres: Vec::new(),
///     },
/// );
///
/// match validate_divorce(&divorce) {
///     Ok(()) => println!("Divorce proceedings are valid"),
///     Err(e) => println!("Divorce validation failed: {}", e),
/// }
/// ```
pub fn validate_divorce(divorce: &Divorce) -> FamilyLawResult<()> {
    // Validate divorce proceedings
    validate_divorce_proceedings(divorce)?;

    // Validate property regime
    validate_property_regime_contract(&divorce.property_regime)?;

    Ok(())
}

/// Validate property regime.
///
/// Ensures that the property regime has the required marriage contract if needed.
///
/// # Example
/// ```rust,ignore
/// use legalis_fr::family::{PropertyRegime, validate_property_regime};
///
/// let regime = PropertyRegime::SeparationDeBiens {
///     marriage_contract: true,
/// };
///
/// assert!(validate_property_regime(&regime).is_ok());
/// ```
pub fn validate_property_regime(regime: &PropertyRegime) -> FamilyLawResult<()> {
    validate_property_regime_contract(regime)
}

/// Validate PACS registration.
///
/// Ensures PACS has been properly registered.
///
/// # Example
/// ```rust,ignore
/// use legalis_fr::family::{PACS, validate_pacs};
///
/// let pacs = PACS::new("Alice".to_string(), "Bob".to_string())
///     .with_registration_date(chrono::Utc::now().naive_utc().date());
///
/// assert!(validate_pacs(&pacs).is_ok());
/// ```
pub fn validate_pacs(pacs: &PACS) -> FamilyLawResult<()> {
    if pacs.registration_date.is_none() {
        return Err(FamilyLawError::PacsRegistrationIncomplete);
    }

    Ok(())
}

/// Validate PACS dissolution.
///
/// For unilateral dissolution, ensures proper notice period.
///
/// # Notice period
/// Unilateral dissolution requires notice to the other party.
pub fn validate_pacs_dissolution(pacs: &PACS) -> FamilyLawResult<()> {
    if pacs.dissolution_date.is_none() {
        return Ok(()); // Not dissolved yet
    }

    // Check if proper notice was given for unilateral dissolution
    if let Some(notice_days) = pacs.dissolution_notice_days() {
        // Typically requires at least some notice period
        // For simplicity, we'll just check it's positive
        if notice_days == 0 {
            return Err(FamilyLawError::PacsDissolutionNoticeInsufficient {
                days_elapsed: notice_days,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::family::types::{DivorceType, MaritalStatus, Nationality, Person, PropertyRegime};
    use chrono::{Duration, Utc};

    #[test]
    fn test_validate_marriage_valid() {
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

        let pub_date = Utc::now().naive_utc().date() - Duration::days(15);
        let marriage_date = Utc::now().naive_utc().date();

        let marriage = Marriage::new(person1, person2)
            .with_consent([true, true])
            .with_banns_published(true)
            .with_banns_publication_date(pub_date)
            .with_marriage_date(marriage_date);

        assert!(validate_marriage(&marriage).is_ok());
    }

    #[test]
    fn test_validate_marriage_invalid_age() {
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

        let marriage = Marriage::new(person1, person2).with_consent([true, true]);

        assert!(validate_marriage(&marriage).is_err());
    }

    #[test]
    fn test_validate_divorce_mutual_consent_valid() {
        let divorce = Divorce::new(
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

        assert!(validate_divorce(&divorce).is_ok());
    }

    #[test]
    fn test_validate_divorce_invalid_separation() {
        let divorce = Divorce::new(
            DivorceType::DefinitiveAlteration {
                separation_duration_months: 18, // Less than required 24
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

        let result = validate_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::InsufficientSeparation { .. }
        ));
    }

    #[test]
    fn test_validate_divorce_with_invalid_property_regime() {
        let divorce = Divorce::new(
            DivorceType::MutualConsent {
                agreement_signed: true,
                notary_filing_date: Some(Utc::now().naive_utc().date()),
                children_heard: true,
            },
            Utc::now().naive_utc().date() - Duration::days(3650),
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::SeparationDeBiens {
                marriage_contract: false, // Requires contract!
            },
        );

        let result = validate_divorce(&divorce);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::MarriageContractRequired { .. }
        ));
    }

    #[test]
    fn test_validate_property_regime_default() {
        let regime = PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        };

        assert!(validate_property_regime(&regime).is_ok());
    }

    #[test]
    fn test_validate_property_regime_separation_no_contract() {
        let regime = PropertyRegime::SeparationDeBiens {
            marriage_contract: false,
        };

        let result = validate_property_regime(&regime);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::MarriageContractRequired { .. }
        ));
    }

    #[test]
    fn test_validate_pacs_valid() {
        let pacs = PACS::new("Alice".to_string(), "Bob".to_string())
            .with_registration_date(Utc::now().naive_utc().date());

        assert!(validate_pacs(&pacs).is_ok());
    }

    #[test]
    fn test_validate_pacs_not_registered() {
        let pacs = PACS::new("Alice".to_string(), "Bob".to_string());

        let result = validate_pacs(&pacs);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FamilyLawError::PacsRegistrationIncomplete
        ));
    }

    #[test]
    fn test_validate_pacs_dissolution_with_notice() {
        let notice_date = Utc::now().naive_utc().date() - Duration::days(30);
        let dissolution_date = Utc::now().naive_utc().date();

        let pacs = PACS::new("Alice".to_string(), "Bob".to_string())
            .with_registration_date(Utc::now().naive_utc().date() - Duration::days(365))
            .with_dissolution_notice_date(notice_date)
            .with_dissolution_date(dissolution_date);

        assert!(validate_pacs_dissolution(&pacs).is_ok());
    }

    #[test]
    fn test_validate_pacs_not_dissolved() {
        let pacs = PACS::new("Alice".to_string(), "Bob".to_string())
            .with_registration_date(Utc::now().naive_utc().date());

        // Not dissolved, so validation should pass
        assert!(validate_pacs_dissolution(&pacs).is_ok());
    }

    #[test]
    fn test_comprehensive_marriage_divorce_flow() {
        // Step 1: Valid marriage
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

        let pub_date = Utc::now().naive_utc().date() - Duration::days(15);
        let marriage_date = Utc::now().naive_utc().date();

        let marriage = Marriage::new(person1, person2)
            .with_consent([true, true])
            .with_banns_published(true)
            .with_banns_publication_date(pub_date)
            .with_marriage_date(marriage_date);

        assert!(validate_marriage(&marriage).is_ok());

        // Step 2: Years later, divorce
        let divorce = Divorce::new(
            DivorceType::MutualConsent {
                agreement_signed: true,
                notary_filing_date: Some(Utc::now().naive_utc().date()),
                children_heard: true,
            },
            marriage_date,
            "Alice".to_string(),
            "Bob".to_string(),
            PropertyRegime::CommunauteReduite {
                marriage_contract: false,
                acquets: Vec::new(),
                biens_propres: Vec::new(),
            },
        );

        assert!(validate_divorce(&divorce).is_ok());
    }
}
