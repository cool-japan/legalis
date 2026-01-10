//! Validation functions for French property law
//!
//! This module provides comprehensive validation of properties, easements,
//! and transactions according to French Code civil provisions.

use super::error::{PropertyLawError, PropertyLawResult};
use super::types::{Easement, EasementType, Encumbrance, Property};

/// Validates a property according to French property law
pub fn validate_property(property: &Property) -> PropertyLawResult<()> {
    let mut errors = Vec::new();

    if property.value == 0 {
        errors.push(PropertyLawError::InvalidPropertyValue {
            value: property.value,
        });
    }

    if property.owner.is_empty() {
        errors.push(PropertyLawError::OwnershipViolation {
            reason: "Owner must be specified".to_string(),
        });
    }

    if property.location.is_empty() {
        errors.push(PropertyLawError::InvalidPropertyType {
            property_type: "Location must be specified".to_string(),
        });
    }

    for easement in &property.easements {
        if let Err(e) = validate_easement(easement) {
            errors.push(e);
        }
    }

    for encumbrance in &property.encumbrances {
        if let Err(e) = validate_encumbrance(encumbrance) {
            errors.push(e);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(PropertyLawError::MultipleErrors(errors))
    }
}

pub fn validate_easement(easement: &Easement) -> PropertyLawResult<()> {
    if easement.servient_estate.is_empty() {
        return Err(PropertyLawError::InvalidEasement {
            reason: "Servient estate must be specified".to_string(),
        });
    }

    match easement.easement_type {
        EasementType::RightOfWay | EasementType::LandlockedAccess | EasementType::WaterRights => {
            if easement.dominant_estate.is_none() {
                return Err(PropertyLawError::InvalidEasement {
                    reason: format!(
                        "{:?} easement should have dominant estate specified",
                        easement.easement_type
                    ),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn validate_encumbrance(encumbrance: &Encumbrance) -> PropertyLawResult<()> {
    if encumbrance.beneficiary.is_empty() {
        return Err(PropertyLawError::InvalidEncumbrance {
            reason: "Beneficiary must be specified".to_string(),
        });
    }

    match encumbrance.encumbrance_type {
        super::types::EncumbranceType::Mortgage | super::types::EncumbranceType::Lien => {
            if encumbrance.amount.is_none() {
                return Err(PropertyLawError::InvalidEncumbrance {
                    reason: format!(
                        "{:?} encumbrance should have amount specified",
                        encumbrance.encumbrance_type
                    ),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn validate_ownership(property: &Property) -> PropertyLawResult<()> {
    if property.owner.is_empty() {
        return Err(PropertyLawError::OwnershipViolation {
            reason: "Article 544 requires owner to be specified".to_string(),
        });
    }

    if property.value == 0 {
        return Err(PropertyLawError::InvalidPropertyValue {
            value: property.value,
        });
    }

    Ok(())
}

pub fn validate_transaction(property: &Property) -> PropertyLawResult<()> {
    if property.is_immovable() {
        if property.owner.is_empty() {
            return Err(PropertyLawError::InvalidTransaction {
                reason: "Seller must be specified for real estate transaction".to_string(),
            });
        }

        if property.value == 0 {
            return Err(PropertyLawError::InvalidTransaction {
                reason: "Property value must be specified for sale".to_string(),
            });
        }
    }

    Ok(())
}
