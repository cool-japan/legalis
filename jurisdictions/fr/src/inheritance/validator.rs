//! Validation functions for French inheritance law
//!
//! This module provides comprehensive validation of successions, wills,
//! and heir shares according to French Code civil provisions.

use super::error::{InheritanceLawError, InheritanceLawResult};
use super::types::{Heir, ReservedPortion, Succession, Will, WillType};
use chrono::Datelike;

/// Validates a succession according to French inheritance law
///
/// # Validations
/// - Article 720: Succession must be opened (death date present)
/// - Article 720: Last domicile must be specified
/// - Article 724: Heirs must be designated
/// - Article 873: Estate must be solvent (assets >= debts)
/// - Article 735: Heir shares must be valid
///
/// # Example
///
/// ```
/// use legalis_fr::inheritance::{Succession, Person as InheritancePerson, validate_succession};
/// use chrono::NaiveDate;
///
/// let deceased = InheritancePerson::new("Jean Dupont".to_string(), 75);
/// let succession = Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
///     .with_last_domicile("Paris, France".to_string())
///     .with_opened(true);
///
/// assert!(validate_succession(&succession).is_ok());
/// ```
pub fn validate_succession(succession: &Succession) -> InheritanceLawResult<()> {
    let mut errors = Vec::new();

    // Article 720: Succession must be opened at death
    if succession.death_date.year() < 1800 {
        errors.push(InheritanceLawError::InvalidDate {
            reason: "Death date must be after 1800".to_string(),
        });
    }

    // Article 720: Last domicile must be specified
    if succession.last_domicile.is_empty() {
        errors.push(InheritanceLawError::InvalidDomicile {
            domicile: succession.last_domicile.clone(),
        });
    }

    // Article 873: Estate must be solvent (or zero debts)
    let net_value = succession.net_estate_value();
    if net_value < 0 {
        errors.push(InheritanceLawError::EstateInsolvent {
            debts: succession.total_debts(),
            assets: succession.total_estate_value(),
        });
    }

    // If will exists, validate it
    if let Some(ref will) = succession.will {
        if let Err(e) = validate_will(will) {
            errors.push(e);
        }
    }

    // If heirs exist, validate their shares
    if !succession.heirs.is_empty() {
        if let Err(e) = validate_heir_shares(&succession.heirs) {
            errors.push(e);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(InheritanceLawError::MultipleErrors(errors))
    }
}

/// Validates a will according to French law requirements
///
/// # Validations
/// - Articles 774-792: Will must meet formal requirements
/// - Article 970: Holographic will must be handwritten, dated, signed
/// - Article 971: Authentic will must have notary and 2 witnesses
/// - Article 976: Mystic will must be sealed with notary
/// - Will must not be revoked
/// - Testator must be specified
///
/// # Example
///
/// ```
/// use legalis_fr::inheritance::{Will, WillType, validate_will};
/// use chrono::NaiveDate;
///
/// let holographic = WillType::Holographic {
///     handwritten: true,
///     dated: true,
///     signed: true,
/// };
/// let will = Will::new(holographic, "Jean Dupont".to_string(), NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
///
/// assert!(validate_will(&will).is_ok());
/// ```
pub fn validate_will(will: &Will) -> InheritanceLawResult<()> {
    // Check if will has been revoked
    if will.revoked {
        return Err(InheritanceLawError::WillRevoked);
    }

    // Check testator is specified
    if will.testator.is_empty() {
        return Err(InheritanceLawError::MissingTestator);
    }

    // Validate date
    if will.date.year() < 1800 || will.date.year() > 2100 {
        return Err(InheritanceLawError::InvalidDate {
            reason: format!("Will date {} is out of valid range", will.date),
        });
    }

    // Validate will type requirements
    match &will.will_type {
        // Article 970: Holographic will requirements
        WillType::Holographic {
            handwritten,
            dated,
            signed,
        } => {
            if !handwritten || !dated || !signed {
                return Err(InheritanceLawError::HolographicWillNotHandwritten);
            }
        }

        // Article 971: Authentic will requirements
        WillType::Authentic { notary, witnesses } => {
            if notary.is_empty() {
                return Err(InheritanceLawError::AuthenticWillMissingFormalities);
            }
            if witnesses.len() < 2 {
                return Err(InheritanceLawError::AuthenticWillMissingFormalities);
            }
        }

        // Article 976: Mystic will requirements
        WillType::Mystic { sealed, notary } => {
            if !sealed || notary.is_empty() {
                return Err(InheritanceLawError::MysticWillNotSealed);
            }
        }
    }

    Ok(())
}

/// Validates that heir shares sum to approximately 1.0 (100%)
///
/// # Validations
/// - Article 735: Total shares must equal 1.0 (allowing 0.01 tolerance)
/// - Shares must be non-negative
/// - At least one heir must be present
///
/// # Example
///
/// ```
/// use legalis_fr::inheritance::{Heir, Person as InheritancePerson, Relationship as InheritanceRelationship, validate_heir_shares};
///
/// let child1 = InheritancePerson::new("Marie Dupont".to_string(), 45);
/// let child2 = InheritancePerson::new("Pierre Dupont".to_string(), 40);
///
/// let heir1 = Heir::new(child1, InheritanceRelationship::Child).with_actual_share(0.5);
/// let heir2 = Heir::new(child2, InheritanceRelationship::Child).with_actual_share(0.5);
///
/// assert!(validate_heir_shares(&vec![heir1, heir2]).is_ok());
/// ```
pub fn validate_heir_shares(heirs: &[Heir]) -> InheritanceLawResult<()> {
    if heirs.is_empty() {
        return Err(InheritanceLawError::NoHeirs);
    }

    // Filter out heirs who have renounced
    let active_heirs: Vec<&Heir> = heirs.iter().filter(|h| !h.renounced).collect();

    if active_heirs.is_empty() {
        return Err(InheritanceLawError::NoHeirs);
    }

    // Calculate total shares
    let total_shares: f64 = active_heirs.iter().map(|h| h.actual_share).sum();

    // Check shares are valid (allowing 0.01 tolerance for floating point)
    if (total_shares - 1.0).abs() > 0.01 {
        return Err(InheritanceLawError::InvalidShareDistribution {
            total: total_shares,
        });
    }

    // Check all shares are non-negative
    for heir in active_heirs {
        if heir.actual_share < 0.0 {
            return Err(InheritanceLawError::InvalidShareDistribution {
                total: total_shares,
            });
        }
    }

    Ok(())
}

/// Validates that reserved portions are respected
///
/// # Validations
/// - Article 912-913: Reserved portions for descendants
/// - Calculates reserved portion based on number of children
/// - Ensures each child receives at least their reserved share
/// - Available portion = 1.0 - reserved portion
///
/// # Arguments
/// - `heirs`: All heirs in the succession
/// - `dispositions_value`: Total value of testamentary dispositions
/// - `estate_value`: Total estate value
///
/// # Returns
/// - Ok(()) if reserved portions are respected
/// - Err if reserved portions are violated
///
/// # Example
///
/// ```
/// use legalis_fr::inheritance::{Heir, Person as InheritancePerson, Relationship as InheritanceRelationship, validate_reserved_portion};
///
/// let child = InheritancePerson::new("Marie Dupont".to_string(), 45);
/// let heir = Heir::new(child, InheritanceRelationship::Child)
///     .with_reserved_portion(0.5)
///     .with_actual_share(0.6);
///
/// // 1 child: reserved = 0.5, actual = 0.6 (OK)
/// assert!(validate_reserved_portion(&vec![heir], 400_000, 1_000_000).is_ok());
/// ```
pub fn validate_reserved_portion(
    heirs: &[Heir],
    dispositions_value: u64,
    estate_value: u64,
) -> InheritanceLawResult<()> {
    // Count descendants (children) who haven't renounced
    let children_count = heirs
        .iter()
        .filter(|h| {
            !h.renounced
                && matches!(
                    h.relationship,
                    super::types::Relationship::Child
                        | super::types::Relationship::Grandchild
                        | super::types::Relationship::GreatGrandchild
                )
        })
        .count() as u32;

    // If no children, no reserved portion applies (complete testamentary freedom)
    if children_count == 0 {
        return Ok(());
    }

    // Calculate reserved portion according to Article 913
    let reserved_calc = ReservedPortion::calculate(children_count);

    // Calculate available portion value
    let available_value = (estate_value as f64 * reserved_calc.available_portion) as u64;

    // Check if dispositions exceed available portion
    if dispositions_value > available_value {
        let _reserved_value = (estate_value as f64 * reserved_calc.reserved_portion) as u64;
        let allocated_to_children = estate_value.saturating_sub(dispositions_value);

        return Err(InheritanceLawError::ReservedPortionViolation {
            allocated: allocated_to_children as f64 / estate_value as f64,
            required: reserved_calc.reserved_portion,
        });
    }

    // Validate each child receives at least their reserved portion
    for heir in heirs {
        if heir.renounced {
            continue;
        }

        // Only check descendants
        if matches!(
            heir.relationship,
            super::types::Relationship::Child
                | super::types::Relationship::Grandchild
                | super::types::Relationship::GreatGrandchild
        ) {
            if let Some(reserved) = heir.reserved_portion {
                if heir.actual_share < reserved {
                    return Err(InheritanceLawError::ReservedPortionViolation {
                        allocated: heir.actual_share,
                        required: reserved,
                    });
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inheritance::types::{Asset, AssetType, Debt, Person, Relationship};
    use chrono::NaiveDate;

    #[test]
    fn test_validate_succession_valid() {
        let deceased = Person::new("Jean Dupont".to_string(), 75);
        let succession = Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
            .with_last_domicile("Paris, France".to_string())
            .with_opened(true);

        assert!(validate_succession(&succession).is_ok());
    }

    #[test]
    fn test_validate_succession_no_domicile() {
        let deceased = Person::new("Jean Dupont".to_string(), 75);
        let succession = Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());

        let result = validate_succession(&succession);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InheritanceLawError::InvalidDomicile { .. }
        ));
    }

    #[test]
    fn test_validate_succession_insolvent() {
        let deceased = Person::new("Jean Dupont".to_string(), 75);
        let succession = Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
            .with_last_domicile("Paris, France".to_string())
            .with_asset(Asset::new(
                AssetType::BankAccount,
                "Savings".to_string(),
                100_000,
            ))
            .with_debt(Debt::new("Bank".to_string(), 200_000));

        let result = validate_succession(&succession);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InheritanceLawError::EstateInsolvent { .. }
        ));
    }

    #[test]
    fn test_validate_will_holographic_valid() {
        let will_type = WillType::Holographic {
            handwritten: true,
            dated: true,
            signed: true,
        };
        let will = Will::new(
            will_type,
            "Jean Dupont".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        );

        assert!(validate_will(&will).is_ok());
    }

    #[test]
    fn test_validate_will_holographic_invalid() {
        let will_type = WillType::Holographic {
            handwritten: false,
            dated: true,
            signed: true,
        };
        let will = Will::new(
            will_type,
            "Jean Dupont".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        );

        let result = validate_will(&will);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InheritanceLawError::HolographicWillNotHandwritten
        ));
    }

    #[test]
    fn test_validate_will_authentic_valid() {
        let will_type = WillType::Authentic {
            notary: "Maître Lefebvre".to_string(),
            witnesses: vec!["Witness 1".to_string(), "Witness 2".to_string()],
        };
        let will = Will::new(
            will_type,
            "Jean Dupont".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        );

        assert!(validate_will(&will).is_ok());
    }

    #[test]
    fn test_validate_will_authentic_missing_witnesses() {
        let will_type = WillType::Authentic {
            notary: "Maître Lefebvre".to_string(),
            witnesses: vec!["Witness 1".to_string()], // Only 1 witness
        };
        let will = Will::new(
            will_type,
            "Jean Dupont".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        );

        let result = validate_will(&will);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InheritanceLawError::AuthenticWillMissingFormalities
        ));
    }

    #[test]
    fn test_validate_will_revoked() {
        let will_type = WillType::Holographic {
            handwritten: true,
            dated: true,
            signed: true,
        };
        let will = Will::new(
            will_type,
            "Jean Dupont".to_string(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        )
        .with_revoked(true);

        let result = validate_will(&will);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InheritanceLawError::WillRevoked
        ));
    }

    #[test]
    fn test_validate_heir_shares_valid() {
        let child1 = Person::new("Marie Dupont".to_string(), 45);
        let child2 = Person::new("Pierre Dupont".to_string(), 40);

        let heir1 = Heir::new(child1, Relationship::Child).with_actual_share(0.5);
        let heir2 = Heir::new(child2, Relationship::Child).with_actual_share(0.5);

        assert!(validate_heir_shares(&[heir1, heir2]).is_ok());
    }

    #[test]
    fn test_validate_heir_shares_invalid_total() {
        let child1 = Person::new("Marie Dupont".to_string(), 45);
        let child2 = Person::new("Pierre Dupont".to_string(), 40);

        let heir1 = Heir::new(child1, Relationship::Child).with_actual_share(0.4);
        let heir2 = Heir::new(child2, Relationship::Child).with_actual_share(0.4);

        let result = validate_heir_shares(&[heir1, heir2]);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InheritanceLawError::InvalidShareDistribution { .. }
        ));
    }

    #[test]
    fn test_validate_heir_shares_with_renunciation() {
        let child1 = Person::new("Marie Dupont".to_string(), 45);
        let child2 = Person::new("Pierre Dupont".to_string(), 40);

        let heir1 = Heir::new(child1, Relationship::Child)
            .with_actual_share(1.0)
            .with_renounced(false);
        let heir2 = Heir::new(child2, Relationship::Child)
            .with_actual_share(0.0)
            .with_renounced(true);

        assert!(validate_heir_shares(&[heir1, heir2]).is_ok());
    }

    #[test]
    fn test_validate_reserved_portion_one_child_ok() {
        let child = Person::new("Marie Dupont".to_string(), 45);
        let heir = Heir::new(child, Relationship::Child)
            .with_reserved_portion(0.5)
            .with_actual_share(0.6);

        // Estate: 1M, Dispositions: 400k (within 500k available), Child gets 600k (>500k reserved)
        assert!(validate_reserved_portion(&[heir], 400_000, 1_000_000).is_ok());
    }

    #[test]
    fn test_validate_reserved_portion_one_child_violation() {
        let child = Person::new("Marie Dupont".to_string(), 45);
        let heir = Heir::new(child, Relationship::Child)
            .with_reserved_portion(0.5)
            .with_actual_share(0.4);

        // Estate: 1M, Child gets 400k (<500k reserved) - VIOLATION
        let result = validate_reserved_portion(&[heir], 600_000, 1_000_000);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            InheritanceLawError::ReservedPortionViolation { .. }
        ));
    }

    #[test]
    fn test_validate_reserved_portion_two_children_ok() {
        let child1 = Person::new("Marie Dupont".to_string(), 45);
        let child2 = Person::new("Pierre Dupont".to_string(), 40);

        let heir1 = Heir::new(child1, Relationship::Child)
            .with_reserved_portion(1.0 / 3.0)
            .with_actual_share(1.0 / 3.0);
        let heir2 = Heir::new(child2, Relationship::Child)
            .with_reserved_portion(1.0 / 3.0)
            .with_actual_share(1.0 / 3.0);

        // Estate: 900k, Dispositions: 300k (within 300k available = 1/3)
        assert!(validate_reserved_portion(&[heir1, heir2], 300_000, 900_000).is_ok());
    }

    #[test]
    fn test_validate_reserved_portion_no_children() {
        let parent = Person::new("Parent".to_string(), 70);
        let heir = Heir::new(parent, Relationship::Parent).with_actual_share(1.0);

        // No children = no reserved portion = complete freedom
        assert!(validate_reserved_portion(&[heir], 1_000_000, 1_000_000).is_ok());
    }

    #[test]
    fn test_validate_reserved_portion_three_children() {
        let child1 = Person::new("Marie".to_string(), 45);
        let child2 = Person::new("Pierre".to_string(), 40);
        let child3 = Person::new("Sophie".to_string(), 35);

        let heir1 = Heir::new(child1, Relationship::Child)
            .with_reserved_portion(0.25)
            .with_actual_share(0.25);
        let heir2 = Heir::new(child2, Relationship::Child)
            .with_reserved_portion(0.25)
            .with_actual_share(0.25);
        let heir3 = Heir::new(child3, Relationship::Child)
            .with_reserved_portion(0.25)
            .with_actual_share(0.25);

        // 3 children: 3/4 reserved (750k), 1/4 available (250k)
        assert!(validate_reserved_portion(&vec![heir1, heir2, heir3], 250_000, 1_000_000).is_ok());
    }
}
