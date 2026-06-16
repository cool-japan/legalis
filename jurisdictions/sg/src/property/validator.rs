//! Property Law - Assessment Logic
//!
//! Assessment functions that apply Singapore land law to the typed models in
//! [`super::types`], [`super::leases`] and [`super::conveyancing`]:
//!
//! - [`assess_indefeasibility`] - whether a challenge defeats a registered title
//!   (LTA s. 46).
//! - [`validate_caveat`] - whether a caveat is supported by a caveatable interest
//!   (LTA s. 115).
//! - [`assess_lease_registration`] - the 7-year registration threshold (LTA
//!   s. 45/s. 46(1)).
//! - [`assess_forfeiture`] - re-entry for breach (CLPA s. 18).
//! - [`validate_easement`] - the *Re Ellenborough Park* characteristics.
//! - [`assess_power_of_sale`] - the registered mortgagee's power of sale
//!   (LTA s. 68).
//! - [`validate_land_contract`] / [`assess_option_to_purchase`] - conveyancing
//!   formalities and the option-to-purchase practice.

use super::conveyancing::{OptionToPurchase, SaleAndPurchase};
use super::error::{PropertyError, Result};
use super::leases::{ForfeitureClaim, Lease};
use super::types::{
    Caveat, Easement, IndefeasibilityChallenge, IndefeasibilityException, LandTitle, Mortgage,
};
use serde::{Deserialize, Serialize};

// ============================================================================
// Indefeasibility (LTA s. 46)
// ============================================================================

/// Outcome of an indefeasibility assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndefeasibilityReport {
    /// The lot reference of the title assessed.
    pub lot_reference: String,
    /// Whether the proprietor's title is defeated by the challenge.
    pub title_defeated: bool,
    /// The exception relied on, if any.
    pub exception: Option<IndefeasibilityException>,
    /// Explanatory notes generated during assessment.
    pub notes: Vec<String>,
}

/// Assesses whether a challenge defeats the indefeasibility of a registered
/// title (Land Titles Act s. 46).
///
/// A registered proprietor's title is indefeasible (s. 46(1)). It is defeated
/// only where the proprietor (or agent) was party or privy to fraud or forgery
/// (s. 46(2)), or where the proprietor is bound by an in personam claim
/// (*United Overseas Bank v Bebe* \[2006\] SGCA 30). Overriding interests bind
/// the title without defeating it.
///
/// # Errors
///
/// Returns [`PropertyError::TitleDefeasible`] where the challenge defeats title.
pub fn assess_indefeasibility(
    title: &LandTitle,
    challenge: Option<&IndefeasibilityChallenge>,
) -> Result<()> {
    if let Some(c) = challenge
        && c.defeats_title()
    {
        return Err(PropertyError::TitleDefeasible {
            reason: format!("{} ({})", c.detail, c.exception.statute_reference()),
        });
    }
    let _ = title;
    Ok(())
}

/// Produces a detailed indefeasibility report (records the outcome without
/// short-circuiting).
pub fn assess_indefeasibility_report(
    title: &LandTitle,
    challenge: Option<&IndefeasibilityChallenge>,
) -> IndefeasibilityReport {
    let mut notes = Vec::new();
    let mut title_defeated = false;
    let exception = challenge.map(|c| c.exception);

    if !title.registered {
        notes.push(
            "Parcel is not registered under the Land Titles Act - indefeasibility does not attach (LTA s. 45)"
                .to_string(),
        );
    } else {
        notes.push("Registered title is prima facie indefeasible (LTA s. 46(1))".to_string());
    }

    if let Some(c) = challenge {
        if c.defeats_title() {
            title_defeated = true;
            notes.push(format!(
                "Indefeasibility defeated: {} ({})",
                c.detail,
                c.exception.statute_reference()
            ));
        } else if c.exception.defeats_title() {
            notes.push(format!(
                "Exception raised but does not defeat title (proprietor not party or privy): {} ({})",
                c.detail,
                c.exception.statute_reference()
            ));
        } else {
            notes.push(format!(
                "Overriding/prior interest binds the title without defeating it: {} ({})",
                c.detail,
                c.exception.statute_reference()
            ));
        }
    }

    IndefeasibilityReport {
        lot_reference: title.lot_reference.clone(),
        title_defeated,
        exception,
        notes,
    }
}

// ============================================================================
// Caveats (LTA s. 115)
// ============================================================================

/// Validates a caveat (Land Titles Act s. 115).
///
/// # Errors
///
/// Returns [`PropertyError::NoCaveatableInterest`] where the caveator has no
/// caveatable (proprietary) interest to support the caveat.
pub fn validate_caveat(caveat: &Caveat) -> Result<()> {
    if !caveat.has_caveatable_interest {
        return Err(PropertyError::NoCaveatableInterest {
            detail: caveat.claimed_interest.clone(),
        });
    }
    Ok(())
}

// ============================================================================
// Leases (LTA s. 45/s. 46(1); CLPA s. 18)
// ============================================================================

/// Outcome of a lease registrability assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaseAssessment {
    /// Whether the lease must be registered (term exceeding 7 years).
    pub must_be_registered: bool,
    /// Whether the lease binds as an overriding interest (s. 46(1)).
    pub is_overriding_interest: bool,
    /// Whether the lease creates a legal leasehold estate.
    pub creates_legal_estate: bool,
    /// Explanatory notes generated during assessment.
    pub notes: Vec<String>,
}

/// Assesses whether a lease over registered land has the registration it needs to
/// create a legal leasehold estate (Land Titles Act s. 45/s. 46(1)).
///
/// # Errors
///
/// Returns [`PropertyError::LeaseNotRegistered`] where a lease for a term
/// exceeding 7 years is unregistered.
pub fn assess_lease_registration(lease: &Lease) -> Result<()> {
    if lease.must_be_registered() && !lease.registered {
        return Err(PropertyError::LeaseNotRegistered {
            years: lease.term_years,
        });
    }
    Ok(())
}

/// Produces a detailed lease assessment.
pub fn assess_lease_report(lease: &Lease) -> LeaseAssessment {
    let mut notes = Vec::new();
    let must_be_registered = lease.must_be_registered();
    let is_overriding_interest = lease.is_overriding_interest();
    let creates_legal_estate = lease.creates_legal_estate();

    if must_be_registered {
        if lease.registered {
            notes.push(format!(
                "{}-year lease registered - legal leasehold estate created (LTA s. 45)",
                lease.term_years
            ));
        } else {
            notes.push(format!(
                "{}-year lease unregistered - takes effect only in equity (Walsh v Lonsdale)",
                lease.term_years
            ));
        }
    } else if is_overriding_interest {
        notes.push(
            "Short lease (<= 7 years) with tenant in occupation binds as an overriding interest (LTA s. 46(1))"
                .to_string(),
        );
    } else {
        notes.push(
            "Short lease not protected as an overriding interest (no occupation, or contains an option to purchase)"
                .to_string(),
        );
    }

    LeaseAssessment {
        must_be_registered,
        is_overriding_interest,
        creates_legal_estate,
        notes,
    }
}

/// Assesses whether a lessor may forfeit (re-enter for breach) under CLPA s. 18.
///
/// # Errors
///
/// Returns [`PropertyError::ForfeitureNotAvailable`] where there is no right of
/// re-entry, or where a CLPA s. 18 notice was required (non-rent breach) but not
/// served.
pub fn assess_forfeiture(claim: &ForfeitureClaim) -> Result<()> {
    if claim.is_available() {
        return Ok(());
    }
    let reason = if !claim.right_of_re_entry {
        "no express right of re-entry in the lease"
    } else {
        "a notice under CLPA s. 18 was required for the non-rent breach but was not served"
    };
    Err(PropertyError::ForfeitureNotAvailable {
        reason: reason.to_string(),
    })
}

// ============================================================================
// Easements (Re Ellenborough Park)
// ============================================================================

/// Validates a claimed easement against the *Re Ellenborough Park* \[1956\] Ch
/// 131 characteristics.
///
/// # Errors
///
/// Returns [`PropertyError::InvalidEasement`] where a characteristic is not
/// satisfied.
pub fn validate_easement(easement: &Easement) -> Result<()> {
    if easement.satisfies_ellenborough_park() {
        return Ok(());
    }
    let reason = if easement.dominant_tenement.is_empty() || easement.servient_tenement.is_empty() {
        "a dominant and a servient tenement must both be identified"
    } else if !easement.accommodates_dominant {
        "the right does not accommodate (benefit) the dominant tenement"
    } else if !easement.diversity_of_ownership {
        "no diversity of ownership - the tenements are owned and occupied by the same person"
    } else {
        "the right is not capable of forming the subject matter of a grant"
    };
    Err(PropertyError::InvalidEasement {
        reason: reason.to_string(),
    })
}

// ============================================================================
// Mortgages (LTA s. 68)
// ============================================================================

/// Assesses whether a registered mortgagee's power of sale is exercisable
/// (Land Titles Act s. 68; the charge must be registered and the mortgagor in
/// default).
///
/// # Errors
///
/// Returns [`PropertyError::PowerOfSaleNotAvailable`] where the power of sale is
/// not exercisable.
pub fn assess_power_of_sale(mortgage: &Mortgage) -> Result<()> {
    if mortgage.power_of_sale_exercisable() {
        return Ok(());
    }
    let reason = if !mortgage.registered {
        "the mortgage is not a registered charge"
    } else {
        "the mortgagor is not in default"
    };
    Err(PropertyError::PowerOfSaleNotAvailable {
        reason: reason.to_string(),
    })
}

// ============================================================================
// Conveyancing (Civil Law Act s. 6(d); option to purchase)
// ============================================================================

/// Validates the writing formality for a contract concerning land (Civil Law Act
/// s. 6(d)).
///
/// # Errors
///
/// Returns [`PropertyError::ContractNotInWriting`] where the contract is not
/// evidenced in writing and signed.
pub fn validate_land_contract(in_writing_and_signed: bool) -> Result<()> {
    if in_writing_and_signed {
        Ok(())
    } else {
        Err(PropertyError::ContractNotInWriting)
    }
}

/// Validates a sale and purchase contract's writing formality (Civil Law Act
/// s. 6(d)).
///
/// # Errors
///
/// Returns [`PropertyError::ContractNotInWriting`] where the contract is not
/// evidenced in writing and signed.
pub fn assess_sale_and_purchase(contract: &SaleAndPurchase) -> Result<()> {
    validate_land_contract(contract.in_writing_and_signed)
}

/// Assesses whether an option to purchase was validly exercised.
///
/// # Errors
///
/// Returns [`PropertyError::OptionNotValidlyExercised`] where the option was not
/// exercised, or was exercised outside the option period.
pub fn assess_option_to_purchase(option: &OptionToPurchase) -> Result<()> {
    if option.validly_exercised() {
        return Ok(());
    }
    let reason = match option.exercised_on_day {
        None => "the option was not exercised".to_string(),
        Some(day) => format!(
            "exercise on day {day} was outside the {}-day option period",
            option.option_period_days
        ),
    };
    Err(PropertyError::OptionNotValidlyExercised { reason })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::conveyancing::OptionToPurchase;
    use crate::property::leases::Lease;
    use crate::property::types::{
        Caveat, Easement, EasementKind, IndefeasibilityChallenge, IndefeasibilityException,
        LandTitle, Mortgage, PropertyType, RegisteredProprietor, Tenure,
    };

    fn title() -> LandTitle {
        LandTitle::new(
            "Lot 1234A MK 5",
            "Condominium unit",
            Tenure::Leasehold99,
            PropertyType::Residential,
            RegisteredProprietor::new("Tan Ah Kow"),
        )
    }

    // ---- Indefeasibility ---------------------------------------------------

    #[test]
    fn test_indefeasibility_no_challenge_is_ok() {
        assert!(assess_indefeasibility(&title(), None).is_ok());
    }

    #[test]
    fn test_indefeasibility_fraud_defeats_title() {
        let challenge = IndefeasibilityChallenge::fraud("proprietor registered a forged transfer");
        match assess_indefeasibility(&title(), Some(&challenge)) {
            Err(PropertyError::TitleDefeasible { .. }) => {}
            other => panic!("expected TitleDefeasible, got {other:?}"),
        }
    }

    #[test]
    fn test_indefeasibility_innocent_purchaser_protected() {
        let challenge = IndefeasibilityChallenge::fraud("antecedent forgery").proprietor_innocent();
        assert!(assess_indefeasibility(&title(), Some(&challenge)).is_ok());
    }

    #[test]
    fn test_indefeasibility_report_overriding_interest() {
        let challenge = IndefeasibilityChallenge::new(
            IndefeasibilityException::OverridingShortLease,
            "tenant in occupation under a 2-year lease",
        );
        let report = assess_indefeasibility_report(&title(), Some(&challenge));
        assert!(!report.title_defeated);
        assert!(report.notes.iter().any(|n| n.contains("Overriding")));
    }

    // ---- Caveats -----------------------------------------------------------

    #[test]
    fn test_caveat_with_interest_is_valid() {
        let caveat = Caveat::new(
            "Purchaser",
            "Lot 1234A MK 5",
            "purchaser under an exercised OTP",
        );
        assert!(validate_caveat(&caveat).is_ok());
    }

    #[test]
    fn test_caveat_without_interest_rejected() {
        let caveat = Caveat::new("Neighbour", "Lot 1234A MK 5", "mere personal grievance")
            .without_caveatable_interest();
        match validate_caveat(&caveat) {
            Err(PropertyError::NoCaveatableInterest { .. }) => {}
            other => panic!("expected NoCaveatableInterest, got {other:?}"),
        }
    }

    // ---- Leases ------------------------------------------------------------

    #[test]
    fn test_long_unregistered_lease_rejected() {
        let lease = Lease::new("Landlord", "Tenant", "Office", 30, 5_000_000).unregistered();
        match assess_lease_registration(&lease) {
            Err(PropertyError::LeaseNotRegistered { years }) => assert_eq!(years, 30),
            other => panic!("expected LeaseNotRegistered, got {other:?}"),
        }
    }

    #[test]
    fn test_short_lease_ok_without_registration() {
        let lease = Lease::new("Landlord", "Tenant", "Shop", 5, 500_000).unregistered();
        assert!(assess_lease_registration(&lease).is_ok());
        let report = assess_lease_report(&lease);
        assert!(report.is_overriding_interest);
        assert!(report.creates_legal_estate);
    }

    #[test]
    fn test_forfeiture_assessment() {
        assert!(
            assess_forfeiture(&ForfeitureClaim::for_breach("unauthorised alterations")).is_ok()
        );
        match assess_forfeiture(
            &ForfeitureClaim::for_breach("unauthorised alterations").without_statutory_notice(),
        ) {
            Err(PropertyError::ForfeitureNotAvailable { .. }) => {}
            other => panic!("expected ForfeitureNotAvailable, got {other:?}"),
        }
    }

    // ---- Easements ---------------------------------------------------------

    #[test]
    fn test_valid_easement() {
        let easement = Easement::new("Lot 1", "Lot 2", EasementKind::RightOfWay);
        assert!(validate_easement(&easement).is_ok());
    }

    #[test]
    fn test_easement_without_diversity_rejected() {
        let easement = Easement::new("Lot 1", "Lot 2", EasementKind::RightOfWay).same_owner();
        match validate_easement(&easement) {
            Err(PropertyError::InvalidEasement { reason }) => {
                assert!(reason.contains("diversity of ownership"));
            }
            other => panic!("expected InvalidEasement, got {other:?}"),
        }
    }

    // ---- Mortgages ---------------------------------------------------------

    #[test]
    fn test_power_of_sale_on_default() {
        let mortgage =
            Mortgage::new("Borrower", "DBS Bank Ltd", "Lot 1234A MK 5", 50_000_000).in_default();
        assert!(assess_power_of_sale(&mortgage).is_ok());
    }

    #[test]
    fn test_power_of_sale_without_default_rejected() {
        let mortgage = Mortgage::new("Borrower", "DBS Bank Ltd", "Lot 1234A MK 5", 50_000_000);
        match assess_power_of_sale(&mortgage) {
            Err(PropertyError::PowerOfSaleNotAvailable { reason }) => {
                assert!(reason.contains("not in default"));
            }
            other => panic!("expected PowerOfSaleNotAvailable, got {other:?}"),
        }
    }

    // ---- Conveyancing ------------------------------------------------------

    #[test]
    fn test_oral_land_contract_unenforceable() {
        let sap = SaleAndPurchase::new(
            "Vendor",
            "Purchaser",
            "Condo",
            PropertyType::Residential,
            100_000_000,
        )
        .oral();
        assert_eq!(
            assess_sale_and_purchase(&sap),
            Err(PropertyError::ContractNotInWriting)
        );
    }

    #[test]
    fn test_written_land_contract_ok() {
        let sap = SaleAndPurchase::new(
            "Vendor",
            "Purchaser",
            "Condo",
            PropertyType::Residential,
            100_000_000,
        );
        assert!(assess_sale_and_purchase(&sap).is_ok());
    }

    #[test]
    fn test_option_exercise_validity() {
        let valid = OptionToPurchase::private_resale("Condo", 100_000_000).exercise_on_day(10);
        assert!(assess_option_to_purchase(&valid).is_ok());

        let late = OptionToPurchase::private_resale("Condo", 100_000_000).exercise_on_day(20);
        match assess_option_to_purchase(&late) {
            Err(PropertyError::OptionNotValidlyExercised { .. }) => {}
            other => panic!("expected OptionNotValidlyExercised, got {other:?}"),
        }
    }

    #[test]
    fn test_performance_many_assessments() {
        let mut issues = 0usize;
        for _ in 0..1000 {
            let lease = Lease::new("L", "T", "Unit", 30, 5_000_000).unregistered();
            if assess_lease_registration(&lease).is_err() {
                issues += 1;
            }
            let challenge = IndefeasibilityChallenge::fraud("forgery");
            if assess_indefeasibility(&title(), Some(&challenge)).is_err() {
                issues += 1;
            }
        }
        assert_eq!(issues, 2000);
    }
}
