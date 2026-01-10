//! BGB Property Law Validators (Sachenrecht)
//!
//! Validation functions implementing BGB property law requirements.

use crate::bgb::sachenrecht::error::{PropertyError, Result};
use crate::bgb::sachenrecht::types::*;

/// Validate movable transfer under §929 BGB
///
/// Requirements per §929 BGB:
/// 1. Agreement (Einigung) between transferor and transferee
/// 2. Delivery (Übergabe) for §929 S. 1, or alternative for §930/§931
/// 3. Transferor has authority to transfer (Verfügungsbefugnis)
pub fn validate_movable_transfer(transfer: &MovableTransfer) -> Result<()> {
    // Requirement 1: Parties must exist
    validate_party_exists(&transfer.transferor, "Transferor")?;
    validate_party_exists(&transfer.transferee, "Transferee")?;

    // Requirement 2: Thing must be specified
    if transfer.thing.description.trim().is_empty() {
        return Err(PropertyError::ThingMissing);
    }

    // Requirement 3: Agreement must be reached
    if !transfer.agreement.agreement_reached {
        return Err(PropertyError::NoAgreement);
    }

    if !transfer.agreement.transfer_intent || !transfer.agreement.acceptance_intent {
        return Err(PropertyError::NoAgreement);
    }

    // Requirement 4: Delivery (for §929 S. 1) or alternative
    match transfer.transfer_type {
        MovableTransferType::ActualDelivery => {
            // §929 S. 1: Requires actual delivery
            if transfer.delivery.is_none() || !transfer.delivery.as_ref().unwrap().delivered {
                return Err(PropertyError::NoDelivery);
            }
        }
        // §930, §931: Alternative transfer methods, no physical delivery required
        MovableTransferType::ConstructivePossession
        | MovableTransferType::AssignmentOfClaim
        | MovableTransferType::BriefHandDelivery => {
            // Agreement sufficient, delivery not required
        }
    }

    // Requirement 5: Value should be positive
    if transfer.thing.value.amount_cents == 0 {
        return Err(PropertyError::InvalidValue);
    }

    Ok(())
}

/// Validate immovable transfer under §873 BGB
///
/// Requirements per §873 BGB:
/// 1. Agreement (Einigung) between transferor and transferee
/// 2. Land registry entry (Grundbucheintragung)
/// 3. Valid land parcel specification
pub fn validate_immovable_transfer(transfer: &ImmovableTransfer) -> Result<()> {
    // Requirement 1: Parties must exist
    validate_party_exists(&transfer.transferor, "Transferor")?;
    validate_party_exists(&transfer.transferee, "Transferee")?;

    // Requirement 2: Agreement must be reached
    if !transfer.agreement.agreement_reached {
        return Err(PropertyError::NoAgreement);
    }

    // Requirement 3: Land registry entry required (§873 Abs. 1 BGB)
    if !transfer.registration.registered {
        return Err(PropertyError::NoLandRegistryEntry);
    }

    // Requirement 4: Valid land parcel
    validate_land_parcel(&transfer.land_parcel)?;

    Ok(())
}

/// Validate land parcel specification
pub fn validate_land_parcel(parcel: &LandParcel) -> Result<()> {
    // Parcel number required
    if parcel.parcel_number.trim().is_empty() {
        return Err(PropertyError::InvalidParcelNumber);
    }

    // Land registry district required
    if parcel.land_registry_district.trim().is_empty() {
        return Err(PropertyError::LandRegistryDistrictMissing);
    }

    // Size must be positive
    if parcel.size_square_meters == 0 {
        return Err(PropertyError::InvalidParcelSize);
    }

    // Description required
    if parcel.description.len() < 3 {
        return Err(PropertyError::InvalidDescription);
    }

    // Value should be positive
    if parcel.value.amount_cents == 0 {
        return Err(PropertyError::InvalidValue);
    }

    Ok(())
}

/// Validate possession under §854 BGB
///
/// Requirements per §854 BGB:
/// 1. Factual control (tatsächliche Gewalt)
/// 2. Possession will (Besitzwille)
pub fn validate_possession(possession: &Possession) -> Result<()> {
    // Requirement 1: Factual control
    if !possession.factual_control {
        return Err(PropertyError::NoFactualControl);
    }

    // Requirement 2: Possession will
    if !possession.possession_will {
        return Err(PropertyError::NoPossessionWill);
    }

    // Possessor must be specified
    validate_party_exists(&possession.possessor, "Possessor")?;

    // Thing must be specified
    if possession.thing.description.trim().is_empty() {
        return Err(PropertyError::ThingMissing);
    }

    Ok(())
}

/// Validate possession protection claim under §§861-862 BGB
pub fn validate_possession_protection_claim(claim: &PossessionProtectionClaim) -> Result<()> {
    // Parties must exist
    validate_party_exists(&claim.claimant, "Claimant")?;
    validate_party_exists(&claim.respondent, "Respondent")?;

    // Thing must be specified
    if claim.thing.description.trim().is_empty() {
        return Err(PropertyError::ThingMissing);
    }

    // Unlawful interference required
    if !claim.unlawful_interference {
        return Err(PropertyError::NoInterference);
    }

    // One-year limitation (§864 BGB)
    if !claim.within_one_year {
        return Err(PropertyError::OneyearLimitationExceeded);
    }

    Ok(())
}

/// Validate easement under §§1018-1093 BGB
pub fn validate_easement(easement: &Easement) -> Result<()> {
    // Servient land required
    validate_land_parcel(&easement.servient_land)?;

    // For predial easements (Grunddienstbarkeit), dominant land required
    if easement.dominant_land.is_none() && easement.beneficiary.is_none() {
        return Err(PropertyError::DominantLandMissing);
    }

    // Registration required for enforceability
    if !easement.registered {
        return Err(PropertyError::EasementNotRegistered);
    }

    Ok(())
}

/// Validate mortgage under §1113 BGB
pub fn validate_mortgage(mortgage: &Mortgage) -> Result<()> {
    // Land parcel required
    validate_land_parcel(&mortgage.land_parcel)?;

    // Parties required
    validate_party_exists(&mortgage.creditor, "Creditor")?;
    validate_party_exists(&mortgage.debtor, "Debtor")?;

    // Mortgage amount must be positive
    if mortgage.mortgage_amount.amount_cents == 0 {
        return Err(PropertyError::InvalidMortgageAmount);
    }

    // Secured claim required
    validate_secured_claim(&mortgage.secured_claim)?;

    // Priority rank must be ≥ 1
    if mortgage.priority_rank == 0 {
        return Err(PropertyError::InvalidPriorityRank);
    }

    // Registration required
    if !mortgage.registry_entry.registered {
        return Err(PropertyError::NoLandRegistryEntry);
    }

    Ok(())
}

/// Validate land charge under §1191 BGB
pub fn validate_land_charge(charge: &LandCharge) -> Result<()> {
    // Land parcel required
    validate_land_parcel(&charge.land_parcel)?;

    // Parties required
    validate_party_exists(&charge.creditor, "Creditor")?;
    validate_party_exists(&charge.debtor, "Debtor")?;

    // Charge amount must be positive
    if charge.charge_amount.amount_cents == 0 {
        return Err(PropertyError::InvalidLandChargeAmount);
    }

    // Priority rank must be ≥ 1
    if charge.priority_rank == 0 {
        return Err(PropertyError::InvalidPriorityRank);
    }

    // Registration required
    if !charge.registry_entry.registered {
        return Err(PropertyError::NoLandRegistryEntry);
    }

    Ok(())
}

/// Validate secured claim
fn validate_secured_claim(claim: &SecuredClaim) -> Result<()> {
    if claim.claim_description.trim().is_empty() {
        return Err(PropertyError::SecuredClaimMissing);
    }

    if claim.claim_amount.amount_cents == 0 {
        return Err(PropertyError::InvalidValue);
    }

    // For mortgages, claim must exist
    if !claim.claim_exists {
        return Err(PropertyError::SecuredClaimNonexistent);
    }

    Ok(())
}

/// Validate movable pledge under §1204 BGB
pub fn validate_movable_pledge(pledge: &MovablePledge) -> Result<()> {
    // Parties required
    validate_party_exists(&pledge.pledgor, "Pledgor")?;
    validate_party_exists(&pledge.pledgee, "Pledgee")?;

    // Pledged thing required
    if pledge.pledged_thing.description.trim().is_empty() {
        return Err(PropertyError::PledgedThingMissing);
    }

    // Possession transfer required (§1205 BGB)
    if !pledge.possession_transferred {
        return Err(PropertyError::PledgePossessionNotTransferred);
    }

    // Secured claim required
    validate_secured_claim(&pledge.secured_claim)?;

    Ok(())
}

/// Validate good faith acquisition under §§932-936 BGB
pub fn validate_good_faith_acquisition(acquisition: &GoodFaithAcquisition) -> Result<()> {
    // Base transfer must be valid
    validate_movable_transfer(&acquisition.transfer)?;

    // Transferor must not be owner (otherwise not good faith acquisition)
    if !acquisition.transferor_not_owner {
        return Err(PropertyError::ValidationFailed {
            reason: "Transferor is owner - not a good faith acquisition case".to_string(),
        });
    }

    // Good faith required (§932 Abs. 1 BGB)
    if !acquisition.good_faith {
        return Err(PropertyError::NoGoodFaith);
    }

    // No gross negligence (§932 Abs. 2 BGB)
    if !acquisition.no_gross_negligence {
        return Err(PropertyError::GrosslyNegligentLackOfKnowledge);
    }

    // Voluntary transfer required - §935 BGB excludes stolen/lost things
    if !acquisition.acquired_through_voluntary_transfer {
        return Err(PropertyError::ThingLostOrStolen);
    }

    Ok(())
}

/// Validate party exists and has valid name
fn validate_party_exists(party: &PropertyParty, role: &str) -> Result<()> {
    if party.name.trim().is_empty() {
        return Err(PropertyError::InvalidParty {
            party: role.to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gmbhg::Capital;
    use chrono::Utc;

    fn create_valid_movable_transfer() -> MovableTransfer {
        MovableTransferBuilder::new()
            .transferor("Max Mustermann", "Berlin")
            .transferee("Erika Schmidt", "Munich")
            .thing("Used car", Capital::from_euros(15_000))
            .transfer_type(MovableTransferType::ActualDelivery)
            .agreement(Utc::now())
            .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
            .consideration(Capital::from_euros(15_000))
            .good_faith(true)
            .build()
            .unwrap()
    }

    #[test]
    fn test_validate_movable_transfer_valid() {
        let transfer = create_valid_movable_transfer();
        assert!(validate_movable_transfer(&transfer).is_ok());
    }

    #[test]
    fn test_validate_movable_transfer_no_delivery() {
        let mut transfer = create_valid_movable_transfer();
        transfer.delivery = None;

        let result = validate_movable_transfer(&transfer);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PropertyError::NoDelivery));
    }

    #[test]
    fn test_validate_movable_transfer_no_agreement() {
        let mut transfer = create_valid_movable_transfer();
        transfer.agreement.agreement_reached = false;

        let result = validate_movable_transfer(&transfer);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PropertyError::NoAgreement));
    }

    #[test]
    fn test_validate_possession_valid() {
        let possession = Possession {
            possessor: PropertyParty {
                name: "Owner".to_string(),
                address: Some("Berlin".to_string()),
                date_of_birth: None,
                is_natural_person: true,
            },
            thing: Thing {
                description: "Bicycle".to_string(),
                property_type: PropertyType::Movable,
                value: Capital::from_euros(500),
                is_consumable: false,
                is_fungible: false,
                location: None,
            },
            possession_type: PossessionType::DirectPossession,
            acquired_at: Utc::now(),
            factual_control: true,
            possession_will: true,
        };

        assert!(validate_possession(&possession).is_ok());
    }

    #[test]
    fn test_validate_possession_no_factual_control() {
        let possession = Possession {
            possessor: PropertyParty {
                name: "Owner".to_string(),
                address: None,
                date_of_birth: None,
                is_natural_person: true,
            },
            thing: Thing {
                description: "Book".to_string(),
                property_type: PropertyType::Movable,
                value: Capital::from_euros(20),
                is_consumable: false,
                is_fungible: true,
                location: None,
            },
            possession_type: PossessionType::DirectPossession,
            acquired_at: Utc::now(),
            factual_control: false,
            possession_will: true,
        };

        let result = validate_possession(&possession);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PropertyError::NoFactualControl
        ));
    }

    #[test]
    fn test_validate_land_parcel_valid() {
        let parcel = LandParcel {
            parcel_number: "123/45".to_string(),
            land_registry_district: "Berlin-Mitte".to_string(),
            size_square_meters: 500,
            location: "Alexanderplatz 1".to_string(),
            description: "Residential property".to_string(),
            value: Capital::from_euros(500_000),
        };

        assert!(validate_land_parcel(&parcel).is_ok());
    }

    #[test]
    fn test_validate_land_parcel_invalid_size() {
        let parcel = LandParcel {
            parcel_number: "123/45".to_string(),
            land_registry_district: "Berlin-Mitte".to_string(),
            size_square_meters: 0,
            location: "Alexanderplatz 1".to_string(),
            description: "Residential property".to_string(),
            value: Capital::from_euros(500_000),
        };

        let result = validate_land_parcel(&parcel);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PropertyError::InvalidParcelSize
        ));
    }

    #[test]
    fn test_validate_good_faith_acquisition_valid() {
        let transfer = create_valid_movable_transfer();
        let acquisition = GoodFaithAcquisition {
            transfer,
            transferor_not_owner: true,
            good_faith: true,
            no_gross_negligence: true,
            acquired_through_voluntary_transfer: true,
            acquisition_valid: true,
        };

        assert!(validate_good_faith_acquisition(&acquisition).is_ok());
    }

    #[test]
    fn test_validate_good_faith_acquisition_no_good_faith() {
        let transfer = create_valid_movable_transfer();
        let acquisition = GoodFaithAcquisition {
            transfer,
            transferor_not_owner: true,
            good_faith: false,
            no_gross_negligence: true,
            acquired_through_voluntary_transfer: true,
            acquisition_valid: false,
        };

        let result = validate_good_faith_acquisition(&acquisition);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PropertyError::NoGoodFaith));
    }

    #[test]
    fn test_validate_good_faith_acquisition_thing_stolen() {
        let transfer = create_valid_movable_transfer();
        let acquisition = GoodFaithAcquisition {
            transfer,
            transferor_not_owner: true,
            good_faith: true,
            no_gross_negligence: true,
            acquired_through_voluntary_transfer: false, // Stolen!
            acquisition_valid: false,
        };

        let result = validate_good_faith_acquisition(&acquisition);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PropertyError::ThingLostOrStolen
        ));
    }
}
