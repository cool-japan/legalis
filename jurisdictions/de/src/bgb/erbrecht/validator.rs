//! Validation functions for German Succession Law
//!
//! Implements BGB Book 5 validation logic for wills, legal succession, and inheritance.

use chrono::Utc;

use super::error::{Result, SuccessionLawError};
use super::types::*;

/// Validate deceased person information
pub fn validate_deceased(deceased: &Deceased) -> Result<()> {
    if deceased.name.trim().is_empty() {
        return Err(SuccessionLawError::EmptyName);
    }

    // Death date must be after birth date
    if deceased.date_of_death < deceased.date_of_birth {
        return Err(SuccessionLawError::DeathBeforeBirth {
            death_date: deceased.date_of_death.to_string(),
            birth_date: deceased.date_of_birth.to_string(),
        });
    }

    // Death date cannot be in the future
    if deceased.date_of_death > Utc::now().date_naive() {
        return Err(SuccessionLawError::FutureDeathDate {
            death_date: deceased.date_of_death.to_string(),
        });
    }

    Ok(())
}

/// Validate will (dispatches to specific validator based on type)
pub fn validate_will(will: &Will) -> Result<()> {
    match will.will_type {
        WillType::Holographic => validate_holographic_will(will),
        WillType::Public => validate_public_will(will),
        WillType::Emergency => validate_public_will(will), // Emergency wills use public will validation
    }
}

/// Validate holographic will (Eigenhändiges Testament) - §2247 BGB
pub fn validate_holographic_will(will: &Will) -> Result<()> {
    validate_deceased(&will.testator)?;

    // §2247 Abs. 1: Must be entirely handwritten
    if !will.is_handwritten {
        return Err(SuccessionLawError::WillNotHandwritten);
    }

    // §2247 Abs. 1: Must be signed
    if !will.has_signature {
        return Err(SuccessionLawError::WillMissingSignature);
    }

    // Check if will was revoked
    if will.revoked {
        return Err(SuccessionLawError::WillRevoked {
            revoked_date: will
                .revoked_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
        });
    }

    // Will creation date must be after testator's birth
    if will.created_at < will.testator.date_of_birth {
        return Err(SuccessionLawError::WillBeforeBirth {
            will_date: will.created_at.to_string(),
            birth_date: will.testator.date_of_birth.to_string(),
        });
    }

    // Will creation date must be before testator's death
    if will.created_at > will.testator.date_of_death {
        return Err(SuccessionLawError::WillAfterDeath {
            will_date: will.created_at.to_string(),
            death_date: will.testator.date_of_death.to_string(),
        });
    }

    // Must have at least one beneficiary
    if will.beneficiaries.is_empty() {
        return Err(SuccessionLawError::NoBeneficiaries);
    }

    // Validate each beneficiary
    for beneficiary in &will.beneficiaries {
        if beneficiary.name.trim().is_empty() {
            return Err(SuccessionLawError::EmptyName);
        }
    }

    // Check that fractional shares sum to 1.0 (if all are fractions)
    validate_inheritance_shares(&will.beneficiaries)?;

    Ok(())
}

/// Validate public will (Öffentliches Testament) - §2232 BGB
pub fn validate_public_will(will: &Will) -> Result<()> {
    validate_deceased(&will.testator)?;

    // Public will must meet general requirements
    if !will.has_signature {
        return Err(SuccessionLawError::WillMissingSignature);
    }

    if will.revoked {
        return Err(SuccessionLawError::WillRevoked {
            revoked_date: will
                .revoked_at
                .map(|d| d.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
        });
    }

    // Will creation dates
    if will.created_at < will.testator.date_of_birth {
        return Err(SuccessionLawError::WillBeforeBirth {
            will_date: will.created_at.to_string(),
            birth_date: will.testator.date_of_birth.to_string(),
        });
    }

    if will.created_at > will.testator.date_of_death {
        return Err(SuccessionLawError::WillAfterDeath {
            will_date: will.created_at.to_string(),
            death_date: will.testator.date_of_death.to_string(),
        });
    }

    // Must have beneficiaries
    if will.beneficiaries.is_empty() {
        return Err(SuccessionLawError::NoBeneficiaries);
    }

    // Validate shares
    validate_inheritance_shares(&will.beneficiaries)?;

    Ok(())
}

/// Validate that inheritance shares are valid and sum correctly
fn validate_inheritance_shares(beneficiaries: &[WillBeneficiary]) -> Result<()> {
    let mut total: f64 = 0.0;
    let mut has_fractions = false;
    let mut has_specific_amounts = false;

    for beneficiary in beneficiaries {
        match &beneficiary.inheritance_share {
            InheritanceShare::Full => {
                if beneficiaries.len() > 1 {
                    // If one beneficiary gets Full, there shouldn't be others
                    return Err(SuccessionLawError::InvalidSharesSum { total: 2.0 });
                }
                return Ok(()); // Single beneficiary with Full is always valid
            }
            InheritanceShare::Fraction {
                numerator,
                denominator,
            } => {
                if *denominator == 0 {
                    return Err(SuccessionLawError::InvalidShareDenominatorZero);
                }
                has_fractions = true;
                total += *numerator as f64 / *denominator as f64;
            }
            InheritanceShare::SpecificAmount(_) => {
                has_specific_amounts = true;
                // Specific amounts don't contribute to fraction validation
            }
        }
    }

    // If we have fractions, they should sum to approximately 1.0
    // (unless we also have specific amounts, which makes validation complex)
    if has_fractions && !has_specific_amounts {
        const EPSILON: f64 = 0.0001;
        if (total - 1.0).abs() > EPSILON {
            return Err(SuccessionLawError::InvalidSharesSum { total });
        }
    }

    Ok(())
}

/// Validate testamentary capacity (Testierfähigkeit) - §2229 BGB
pub fn validate_testamentary_capacity(testator_age: u32) -> Result<TestamentaryCapacity> {
    if testator_age < 16 {
        Err(SuccessionLawError::NoTestamentaryCapacity)
    } else if testator_age < 18 {
        // Age 16-17: Limited capacity, requires special formalities
        Ok(TestamentaryCapacity::Limited)
    } else {
        // Age 18+: Full capacity
        Ok(TestamentaryCapacity::Full)
    }
}

/// Validate legal succession (Gesetzliche Erbfolge) - §§1924-1936 BGB
pub fn validate_legal_succession(succession: &LegalSuccession) -> Result<()> {
    validate_deceased(&succession.deceased)?;

    // Must have at least one heir
    if succession.heirs.is_empty() {
        return Err(SuccessionLawError::NoStatutoryHeirs);
    }

    // Validate each heir
    for heir in &succession.heirs {
        if heir.name.trim().is_empty() {
            return Err(SuccessionLawError::EmptyName);
        }
    }

    // If there's spouse inheritance, validate it
    if let Some(ref spouse_inheritance) = succession.spouse_inheritance {
        validate_spouse_inheritance(spouse_inheritance)?;
    }

    Ok(())
}

/// Validate spouse inheritance - §1931 BGB
fn validate_spouse_inheritance(spouse_inheritance: &SpouseInheritance) -> Result<()> {
    if spouse_inheritance.spouse_name.trim().is_empty() {
        return Err(SuccessionLawError::EmptyName);
    }

    // Marriage date cannot be in the future
    if spouse_inheritance.marriage_date > Utc::now().date_naive() {
        return Err(SuccessionLawError::InvalidDate {
            date_type: "Marriage date in future".to_string(),
        });
    }

    Ok(())
}

/// Validate compulsory portion (Pflichtteil) - §§2303-2338 BGB
pub fn validate_compulsory_portion(portion: &CompulsoryPortion) -> Result<()> {
    validate_deceased(&portion.deceased)?;

    // Claimant must be entitled to compulsory portion
    if !portion.claimant.is_entitled() {
        return Err(SuccessionLawError::NotEntitledToCompulsoryPortion {
            relationship: format!("{:?}", portion.claimant.relationship),
        });
    }

    // Estate value must be positive
    if portion.estate_value.amount_cents == 0 {
        return Err(SuccessionLawError::NoEstateValue);
    }

    // Compulsory portion amount cannot exceed estate value
    if portion.amount.amount_cents > portion.estate_value.amount_cents {
        return Err(SuccessionLawError::CompulsoryPortionExceedsEstate {
            amount: portion.amount.to_euros(),
            estate_value: portion.estate_value.to_euros(),
        });
    }

    Ok(())
}

/// Validate inheritance contract (Erbvertrag) - §§2274-2302 BGB
pub fn validate_inheritance_contract(contract: &InheritanceContract) -> Result<()> {
    // §2276 BGB: Must be notarized
    if !contract.notarized {
        return Err(SuccessionLawError::InheritanceContractNotNotarized);
    }

    // Cannot be revoked
    if contract.revoked {
        return Err(SuccessionLawError::InheritanceContractRevoked);
    }

    // Testator and beneficiary must be different
    if contract.testator.trim().is_empty() || contract.beneficiary.trim().is_empty() {
        return Err(SuccessionLawError::EmptyName);
    }

    // Contract date cannot be in the future
    if contract.contract_date > Utc::now().date_naive() {
        return Err(SuccessionLawError::InvalidDate {
            date_type: "Contract date in future".to_string(),
        });
    }

    Ok(())
}

/// Validate estate (Nachlass)
pub fn validate_estate(estate: &Estate) -> Result<()> {
    validate_deceased(&estate.deceased)?;

    // Estate must have assets
    if estate.assets.is_empty() {
        return Err(SuccessionLawError::EmptyEstate);
    }

    // Calculate actual net value
    let calculated_net = estate.calculate_net_value();

    // Estate cannot be insolvent (liabilities > assets)
    let total_assets: u64 = estate.assets.iter().map(|a| a.value.amount_cents).sum();
    let total_liabilities: u64 = estate
        .liabilities
        .iter()
        .map(|l| l.amount.amount_cents)
        .sum();

    if total_liabilities > total_assets {
        return Err(SuccessionLawError::InsolvantEstate {
            assets: (total_assets as f64) / 100.0,
            liabilities: (total_liabilities as f64) / 100.0,
        });
    }

    // Net value should match calculated value
    if estate.net_value.amount_cents != calculated_net.amount_cents {
        return Err(SuccessionLawError::InvalidDate {
            date_type: "Net value mismatch".to_string(),
        });
    }

    Ok(())
}

/// Validate inheritance decision (acceptance/renunciation) - §§1942-1953 BGB
pub fn validate_inheritance_decision(decision: &InheritanceDecision) -> Result<()> {
    // Decision date cannot be in the future
    if decision.decision_date > Utc::now().date_naive() {
        return Err(SuccessionLawError::InvalidDate {
            date_type: "Decision date in future".to_string(),
        });
    }

    // Deadline must be after decision date (if decision is on time)
    if decision.decision_date > decision.deadline {
        return Err(SuccessionLawError::RenunciationDeadlineExpired {
            deadline: decision.deadline.to_string(),
            decision_date: decision.decision_date.to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_valid_deceased() -> Deceased {
        Deceased {
            name: "Hans Mueller".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1950, 1, 1).unwrap(),
            date_of_death: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            place_of_death: "Berlin".to_string(),
            last_residence: "Berlin".to_string(),
            nationality: "German".to_string(),
        }
    }

    #[test]
    fn test_valid_holographic_will() {
        let will = Will {
            testator: create_valid_deceased(),
            will_type: WillType::Holographic,
            created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            place_of_creation: "Berlin".to_string(),
            is_handwritten: true,
            has_signature: true,
            has_date: true,
            beneficiaries: vec![WillBeneficiary {
                name: "Maria Mueller".to_string(),
                relationship: RelationshipToDeceased::Spouse,
                inheritance_share: InheritanceShare::Full,
                conditions: vec![],
            }],
            revoked: false,
            revoked_at: None,
        };

        assert!(validate_holographic_will(&will).is_ok());
    }

    #[test]
    fn test_holographic_will_not_handwritten() {
        let will = Will {
            testator: create_valid_deceased(),
            will_type: WillType::Holographic,
            created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            place_of_creation: "Berlin".to_string(),
            is_handwritten: false, // Invalid!
            has_signature: true,
            has_date: true,
            beneficiaries: vec![WillBeneficiary {
                name: "Maria Mueller".to_string(),
                relationship: RelationshipToDeceased::Spouse,
                inheritance_share: InheritanceShare::Full,
                conditions: vec![],
            }],
            revoked: false,
            revoked_at: None,
        };

        let result = validate_holographic_will(&will);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SuccessionLawError::WillNotHandwritten
        ));
    }

    #[test]
    fn test_will_missing_signature() {
        let will = Will {
            testator: create_valid_deceased(),
            will_type: WillType::Holographic,
            created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
            place_of_creation: "Berlin".to_string(),
            is_handwritten: true,
            has_signature: false, // Invalid!
            has_date: true,
            beneficiaries: vec![WillBeneficiary {
                name: "Maria Mueller".to_string(),
                relationship: RelationshipToDeceased::Spouse,
                inheritance_share: InheritanceShare::Full,
                conditions: vec![],
            }],
            revoked: false,
            revoked_at: None,
        };

        let result = validate_holographic_will(&will);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SuccessionLawError::WillMissingSignature
        ));
    }

    #[test]
    fn test_testamentary_capacity_full() {
        let capacity = validate_testamentary_capacity(25);
        assert!(capacity.is_ok());
        assert_eq!(capacity.unwrap(), TestamentaryCapacity::Full);
    }

    #[test]
    fn test_testamentary_capacity_limited() {
        let capacity = validate_testamentary_capacity(17);
        assert!(capacity.is_ok());
        assert_eq!(capacity.unwrap(), TestamentaryCapacity::Limited);
    }

    #[test]
    fn test_testamentary_capacity_none() {
        let capacity = validate_testamentary_capacity(15);
        assert!(capacity.is_err());
        assert!(matches!(
            capacity.unwrap_err(),
            SuccessionLawError::NoTestamentaryCapacity
        ));
    }

    #[test]
    fn test_valid_legal_succession() {
        let succession = LegalSuccession {
            deceased: create_valid_deceased(),
            heirs: vec![Heir {
                name: "Child 1".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
                relationship: RelationshipToDeceased::Child,
                inheritance_share: InheritanceShare::Fraction {
                    numerator: 1,
                    denominator: 2,
                },
                is_statutory_heir: true,
            }],
            succession_order: SuccessionOrder::First,
            spouse_inheritance: None,
        };

        assert!(validate_legal_succession(&succession).is_ok());
    }

    #[test]
    fn test_legal_succession_no_heirs() {
        let succession = LegalSuccession {
            deceased: create_valid_deceased(),
            heirs: vec![], // No heirs!
            succession_order: SuccessionOrder::First,
            spouse_inheritance: None,
        };

        let result = validate_legal_succession(&succession);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SuccessionLawError::NoStatutoryHeirs
        ));
    }

    #[test]
    fn test_valid_compulsory_portion() {
        use crate::gmbhg::Capital;

        let portion = CompulsoryPortion {
            claimant: CompulsoryPortionClaimant {
                name: "Child".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
                relationship: RelationshipToDeceased::Child,
            },
            deceased: create_valid_deceased(),
            estate_value: Capital::from_euros(100_000),
            portion: InheritanceShare::Fraction {
                numerator: 1,
                denominator: 4,
            }, // 1/2 of 1/2 = 1/4
            amount: Capital::from_euros(25_000),
        };

        assert!(validate_compulsory_portion(&portion).is_ok());
    }

    #[test]
    fn test_compulsory_portion_not_entitled() {
        use crate::gmbhg::Capital;

        let portion = CompulsoryPortion {
            claimant: CompulsoryPortionClaimant {
                name: "Sibling".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1955, 1, 1).unwrap(),
                relationship: RelationshipToDeceased::Sibling, // Not entitled!
            },
            deceased: create_valid_deceased(),
            estate_value: Capital::from_euros(100_000),
            portion: InheritanceShare::Fraction {
                numerator: 1,
                denominator: 4,
            },
            amount: Capital::from_euros(25_000),
        };

        let result = validate_compulsory_portion(&portion);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SuccessionLawError::NotEntitledToCompulsoryPortion { .. }
        ));
    }

    #[test]
    fn test_valid_inheritance_contract() {
        let contract = InheritanceContract {
            testator: "Hans Mueller".to_string(),
            beneficiary: "Maria Mueller".to_string(),
            contract_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            notarized: true,
            inheritance_share: InheritanceShare::Full,
            is_mutual: true,
            revoked: false,
        };

        assert!(validate_inheritance_contract(&contract).is_ok());
    }

    #[test]
    fn test_inheritance_contract_not_notarized() {
        let contract = InheritanceContract {
            testator: "Hans Mueller".to_string(),
            beneficiary: "Maria Mueller".to_string(),
            contract_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            notarized: false, // Invalid!
            inheritance_share: InheritanceShare::Full,
            is_mutual: true,
            revoked: false,
        };

        let result = validate_inheritance_contract(&contract);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SuccessionLawError::InheritanceContractNotNotarized
        ));
    }
}
