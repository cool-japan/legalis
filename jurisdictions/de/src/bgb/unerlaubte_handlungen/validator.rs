//! BGB Tort Law Validators (Unerlaubte Handlungen)
//!
//! Multi-stage validation implementing BGB tort law requirements.

use crate::bgb::unerlaubte_handlungen::error::{Result, TortError};
use crate::bgb::unerlaubte_handlungen::types::{
    Justification, TortClaim823_1, TortClaim826, Verschulden,
};

/// Validate a tort claim under §823 Abs. 1 BGB
///
/// Requirements per §823 Abs. 1 BGB:
/// 1. Violation of protected interest (Leben, Körper, Gesundheit, Freiheit, Eigentum, sonstiges Recht)
/// 2. Intent (Vorsatz) OR Negligence (Fahrlässigkeit)
/// 3. Unlawfulness (Widerrechtlichkeit) - presumed unless justified
/// 4. Causation (Kausalität)
/// 5. Damage (Schaden)
pub fn validate_tort_claim_823_1(claim: &TortClaim823_1) -> Result<()> {
    // Requirement 1: Protected interest must be violated
    // (Already validated by type system - protected_interest field exists)

    // Requirement 2: Fault (Verschulden) must exist
    if matches!(claim.verschulden, Verschulden::KeinVerschulden) {
        return Err(TortError::NoFaultProven);
    }

    // Requirement 3: Unlawfulness (Widerrechtlichkeit)
    if !claim.widerrechtlich {
        return Err(TortError::NoProtectedInterestViolated);
    }

    // Check if unlawfulness is negated by justification
    if let Some(ref justification) = claim.justification {
        let grund = match justification {
            Justification::SelfDefense => "Notwehr",
            Justification::Necessity => "Notstand",
            Justification::Consent => "Einwilligung",
            Justification::LegalAuthorization => "Gesetzliche Befugnis",
            Justification::ExerciseOfRights => "Rechtausübung",
        };
        return Err(TortError::UnlawfulnessNegated {
            grund: grund.to_string(),
        });
    }

    // Requirement 4: Causation
    if !claim.causation_established {
        return Err(TortError::CausationNotProven);
    }

    // Requirement 5: Damage
    if claim.damages.total.amount_cents == 0 {
        return Err(TortError::ZeroDamage);
    }

    Ok(())
}

/// Validate a tort claim under §826 BGB
///
/// Requirements per §826 BGB:
/// 1. Intent to cause damage (Schädigungsvorsatz)
/// 2. Conduct contrary to good morals (Sittenwidrigkeit)
/// 3. Damage (Schaden)
/// 4. Causation (Kausalität)
pub fn validate_tort_claim_826(claim: &TortClaim826) -> Result<()> {
    // Requirement 1: Intent to cause damage
    if !claim.schadigungsvorsatz {
        return Err(TortError::NoIntentToHarm);
    }

    // Requirement 2: Contrary to good morals
    if !claim.sittenwidrig {
        return Err(TortError::NotContraryToGoodMorals);
    }

    // Requirement 3: Causation
    if !claim.causation_established {
        return Err(TortError::CausationNotProven);
    }

    // Requirement 4: Damage
    if claim.damages.total.amount_cents == 0 {
        return Err(TortError::ZeroDamage);
    }

    Ok(())
}

/// Validate parties are specified
pub fn validate_parties_exist(tortfeasor_name: &str, injured_party_name: &str) -> Result<()> {
    if tortfeasor_name.trim().is_empty() {
        return Err(TortError::TortfeasorMissing);
    }

    if injured_party_name.trim().is_empty() {
        return Err(TortError::InjuredPartyMissing);
    }

    Ok(())
}

/// Validate damage amount
pub fn validate_damage_amount(amount_cents: u64) -> Result<()> {
    if amount_cents == 0 {
        Err(TortError::ZeroDamage)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bgb::unerlaubte_handlungen::types::{
        DamageClaim, ProtectedInterest, TortClaim823_1Builder, TortParty,
    };
    use crate::gmbhg::Capital;
    use chrono::Utc;

    fn create_valid_claim_823_1() -> TortClaim823_1 {
        TortClaim823_1Builder::new()
            .tortfeasor("Max Mustermann", "Berlin")
            .injured_party("Erika Schmidt", "Munich")
            .protected_interest(ProtectedInterest::Body)
            .violation_direct_injury("Broken arm", "Moderate")
            .verschulden(Verschulden::EinfacheFahrlassigkeit)
            .widerrechtlich(true)
            .incident_date(Utc::now())
            .damages_medical(Capital::from_euros(5_000))
            .causation_established(true)
            .build()
            .unwrap()
    }

    #[test]
    fn test_validate_tort_claim_823_1_valid() {
        let claim = create_valid_claim_823_1();
        assert!(validate_tort_claim_823_1(&claim).is_ok());
    }

    #[test]
    fn test_validate_tort_claim_823_1_no_fault() {
        let mut claim = create_valid_claim_823_1();
        claim.verschulden = Verschulden::KeinVerschulden;

        let result = validate_tort_claim_823_1(&claim);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TortError::NoFaultProven));
    }

    #[test]
    fn test_validate_tort_claim_823_1_not_unlawful() {
        let mut claim = create_valid_claim_823_1();
        claim.widerrechtlich = false;

        let result = validate_tort_claim_823_1(&claim);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tort_claim_823_1_justified() {
        let mut claim = create_valid_claim_823_1();
        claim.justification = Some(Justification::SelfDefense);

        let result = validate_tort_claim_823_1(&claim);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TortError::UnlawfulnessNegated { .. }
        ));
    }

    #[test]
    fn test_validate_tort_claim_823_1_no_causation() {
        let mut claim = create_valid_claim_823_1();
        claim.causation_established = false;

        let result = validate_tort_claim_823_1(&claim);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TortError::CausationNotProven));
    }

    #[test]
    fn test_validate_tort_claim_823_1_zero_damage() {
        let mut claim = create_valid_claim_823_1();
        claim.damages.total = Capital { amount_cents: 0 };

        let result = validate_tort_claim_823_1(&claim);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TortError::ZeroDamage));
    }

    #[test]
    fn test_validate_tort_claim_826_valid() {
        let claim = TortClaim826 {
            tortfeasor: TortParty {
                name: "Bad Actor".to_string(),
                address: Some("Berlin".to_string()),
                is_natural_person: true,
            },
            injured_party: TortParty {
                name: "Victim".to_string(),
                address: Some("Munich".to_string()),
                is_natural_person: true,
            },
            conduct: "Fraudulent misrepresentation".to_string(),
            sittenwidrig: true,
            schadigungsvorsatz: true,
            incident_date: Utc::now(),
            damages: DamageClaim {
                property_damage: Some(Capital::from_euros(10_000)),
                personal_injury: None,
                pain_and_suffering: None,
                lost_income: None,
                medical_expenses: None,
                consequential_damages: None,
                total: Capital::from_euros(10_000),
            },
            causation_established: true,
            notes: None,
        };

        assert!(validate_tort_claim_826(&claim).is_ok());
    }

    #[test]
    fn test_validate_tort_claim_826_no_intent() {
        let claim = TortClaim826 {
            tortfeasor: TortParty {
                name: "Bad Actor".to_string(),
                address: None,
                is_natural_person: true,
            },
            injured_party: TortParty {
                name: "Victim".to_string(),
                address: None,
                is_natural_person: true,
            },
            conduct: "Negligent conduct".to_string(),
            sittenwidrig: true,
            schadigungsvorsatz: false, // No intent!
            incident_date: Utc::now(),
            damages: DamageClaim {
                property_damage: Some(Capital::from_euros(5_000)),
                personal_injury: None,
                pain_and_suffering: None,
                lost_income: None,
                medical_expenses: None,
                consequential_damages: None,
                total: Capital::from_euros(5_000),
            },
            causation_established: true,
            notes: None,
        };

        let result = validate_tort_claim_826(&claim);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TortError::NoIntentToHarm));
    }

    #[test]
    fn test_validate_parties_exist_valid() {
        assert!(validate_parties_exist("Max Mustermann", "Erika Schmidt").is_ok());
    }

    #[test]
    fn test_validate_parties_exist_missing_tortfeasor() {
        let result = validate_parties_exist("", "Erika Schmidt");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TortError::TortfeasorMissing));
    }

    #[test]
    fn test_validate_parties_exist_missing_injured_party() {
        let result = validate_parties_exist("Max Mustermann", "");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TortError::InjuredPartyMissing
        ));
    }

    #[test]
    fn test_validate_damage_amount_valid() {
        assert!(validate_damage_amount(1000).is_ok());
        assert!(validate_damage_amount(10000000).is_ok());
    }

    #[test]
    fn test_validate_damage_amount_zero() {
        let result = validate_damage_amount(0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TortError::ZeroDamage));
    }
}
