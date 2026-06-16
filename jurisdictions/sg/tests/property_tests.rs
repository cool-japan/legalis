//! Integration tests for the Singapore property-law module.
//!
//! These exercise the public API of [`legalis_sg::property`] across the Torrens
//! title system (Land Titles Act 1993), conveyancing, leases and security
//! interests through realistic Singapore property scenarios.

use legalis_sg::property::*;

fn sample_title() -> LandTitle {
    LandTitle::new(
        "Lot 1234A MK 5",
        "Condominium unit #12-34",
        Tenure::Leasehold99,
        PropertyType::Residential,
        RegisteredProprietor::new("Tan Ah Kow").with_identifier("S1234567A"),
    )
}

/// Fraud to which the registered proprietor was party defeats indefeasibility
/// (LTA s. 46(2)); a bona fide purchaser for value takes free (immediate
/// indefeasibility, *UOB v Bebe*).
#[test]
fn fraud_defeats_title_but_innocent_purchaser_is_protected() {
    let fraud = IndefeasibilityChallenge::fraud("proprietor registered a forged transfer");
    match assess_indefeasibility(&sample_title(), Some(&fraud)) {
        Err(PropertyError::TitleDefeasible { .. }) => {}
        other => panic!("expected TitleDefeasible, got {other:?}"),
    }

    let innocent = fraud.proprietor_innocent();
    assert!(assess_indefeasibility(&sample_title(), Some(&innocent)).is_ok());
}

/// An in personam claim binds the proprietor regardless (*UOB v Bebe*).
#[test]
fn in_personam_claim_binds_proprietor() {
    let challenge = IndefeasibilityChallenge::new(
        IndefeasibilityException::InPersonam,
        "proprietor bound by a prior contract to convey",
    );
    assert!(assess_indefeasibility(&sample_title(), Some(&challenge)).is_err());
}

/// An overriding interest (short lease, easement) binds but does not defeat title.
#[test]
fn overriding_interest_binds_without_defeating() {
    let challenge = IndefeasibilityChallenge::new(
        IndefeasibilityException::OverridingShortLease,
        "tenant in occupation under a 2-year lease",
    );
    assert!(assess_indefeasibility(&sample_title(), Some(&challenge)).is_ok());
    let report = assess_indefeasibility_report(&sample_title(), Some(&challenge));
    assert!(!report.title_defeated);
}

/// A caveat needs a caveatable (proprietary) interest (LTA s. 115).
#[test]
fn caveat_needs_caveatable_interest() {
    let valid = Caveat::new(
        "Purchaser",
        "Lot 1234A MK 5",
        "purchaser under an exercised OTP",
    );
    assert!(validate_caveat(&valid).is_ok());
    assert!(valid.prohibits_registration());

    let invalid = Caveat::new("Neighbour", "Lot 1234A MK 5", "personal grievance")
        .without_caveatable_interest();
    assert!(validate_caveat(&invalid).is_err());
}

/// A lease for a term exceeding 7 years must be registered to create a legal
/// estate (LTA s. 45); a short lease binds as an overriding interest (s. 46(1)).
#[test]
fn lease_seven_year_registration_threshold() {
    let long_unregistered =
        Lease::new("Landlord", "Tenant", "Office floor", 30, 5_000_000).unregistered();
    match assess_lease_registration(&long_unregistered) {
        Err(PropertyError::LeaseNotRegistered { years }) => assert_eq!(years, 30),
        other => panic!("expected LeaseNotRegistered, got {other:?}"),
    }

    let short = Lease::new("Landlord", "Tenant", "Shop", 5, 500_000).unregistered();
    assert!(assess_lease_registration(&short).is_ok());
    let report = assess_lease_report(&short);
    assert!(report.is_overriding_interest);
    assert!(report.creates_legal_estate);

    // Exactly 7 years does not require registration; 8 years does.
    assert!(!Lease::new("L", "T", "U", 7, 1).must_be_registered());
    assert!(Lease::new("L", "T", "U", 8, 1).must_be_registered());
}

/// Lease covenants: quiet enjoyment binds the lessor; payment of rent the lessee.
#[test]
fn lease_covenants_classification() {
    assert_eq!(LeaseCovenant::QuietEnjoyment.party(), CovenantParty::Lessor);
    assert_eq!(LeaseCovenant::PayRent.party(), CovenantParty::Lessee);
    assert!(LeaseCovenant::QuietEnjoyment.implied_by_default());
    assert_eq!(
        LeaseDetermination::Forfeiture.statute_reference(),
        Some("Conveyancing and Law of Property Act s. 18")
    );
}

/// Forfeiture requires a right of re-entry and (for a non-rent breach) a CLPA
/// s. 18 notice.
#[test]
fn forfeiture_requires_re_entry_and_notice() {
    assert!(assess_forfeiture(&ForfeitureClaim::for_breach("unauthorised subletting")).is_ok());
    assert!(assess_forfeiture(&ForfeitureClaim::for_rent_arrears()).is_ok());

    let no_notice =
        ForfeitureClaim::for_breach("unauthorised subletting").without_statutory_notice();
    assert!(assess_forfeiture(&no_notice).is_err());

    let no_clause = ForfeitureClaim::for_rent_arrears().without_re_entry_clause();
    assert!(assess_forfeiture(&no_clause).is_err());
}

/// An easement must satisfy the *Re Ellenborough Park* characteristics.
#[test]
fn easement_ellenborough_park() {
    let valid = Easement::new(
        "Lot 1 (dominant)",
        "Lot 2 (servient)",
        EasementKind::RightOfWay,
    );
    assert!(validate_easement(&valid).is_ok());

    assert!(validate_easement(&valid.clone().same_owner()).is_err());
    assert!(validate_easement(&valid.not_accommodating()).is_err());
}

/// A registered mortgagee's power of sale arises on default (LTA s. 68).
#[test]
fn mortgage_power_of_sale_on_default() {
    let mortgage =
        Mortgage::new("Borrower", "DBS Bank Ltd", "Lot 1234A MK 5", 80_000_000).in_default();
    assert!(assess_power_of_sale(&mortgage).is_ok());

    let not_defaulted = Mortgage::new("Borrower", "DBS Bank Ltd", "Lot 1234A MK 5", 80_000_000);
    assert!(assess_power_of_sale(&not_defaulted).is_err());

    let unregistered = Mortgage::new("Borrower", "Lender", "Lot 9", 10_000_000)
        .unregistered()
        .in_default();
    assert!(assess_power_of_sale(&unregistered).is_err());
}

/// A contract for the disposition of land must be in writing (Civil Law Act
/// s. 6(d)).
#[test]
fn land_contract_writing_requirement() {
    assert!(validate_land_contract(true).is_ok());
    assert_eq!(
        validate_land_contract(false),
        Err(PropertyError::ContractNotInWriting)
    );

    let oral = SaleAndPurchase::new(
        "Vendor",
        "Purchaser",
        "Condo",
        PropertyType::Residential,
        150_000_000,
    )
    .oral();
    assert!(assess_sale_and_purchase(&oral).is_err());
}

/// The option-to-purchase practice: exercise must be within the option period.
#[test]
fn option_to_purchase_exercise() {
    let otp = OptionToPurchase::private_resale("Condo #10-11", 200_000_000);
    // 1% option fee = SGD 20,000; conventional 14-day period.
    assert_eq!(otp.option_fee_cents, 2_000_000);
    assert_eq!(otp.option_period_days, 14);

    let exercised = otp.exercise_on_day(10);
    assert!(assess_option_to_purchase(&exercised).is_ok());

    let late = OptionToPurchase::private_resale("Condo", 200_000_000).exercise_on_day(20);
    assert!(assess_option_to_purchase(&late).is_err());
}

/// Buyer's Stamp Duty is computed on the marginal residential/non-residential
/// scales (Stamp Duties Act 1929; rates as at 2023).
#[test]
fn buyers_stamp_duty_scales() {
    // Residential SGD 1,000,000 -> SGD 24,600.
    assert_eq!(
        compute_buyers_stamp_duty_cents(100_000_000, PropertyType::Residential),
        2_460_000
    );
    // Residential SGD 5,000,000 -> SGD 239,600.
    assert_eq!(
        compute_buyers_stamp_duty_cents(500_000_000, PropertyType::Residential),
        23_960_000
    );
    // Non-residential SGD 2,000,000 -> SGD 69,600.
    assert_eq!(
        compute_buyers_stamp_duty_cents(200_000_000, PropertyType::Commercial),
        6_960_000
    );
}

/// A full conveyancing flow: OTP exercised, written contract, completion.
#[test]
fn full_conveyancing_flow() {
    let otp = OptionToPurchase::private_resale("Condo #10-11", 200_000_000).exercise_on_day(12);
    assert!(assess_option_to_purchase(&otp).is_ok());

    let sap = SaleAndPurchase::new(
        "Vendor",
        "Purchaser",
        "Condo #10-11",
        PropertyType::Residential,
        200_000_000,
    );
    assert!(assess_sale_and_purchase(&sap).is_ok());
    assert_eq!(sap.balance_on_completion_cents(), 190_000_000);

    let completion = Completion::pending()
        .with_balance_paid()
        .with_transfer_executed()
        .with_vacant_possession();
    assert!(completion.is_complete());
}

/// Serialization roundtrips for the aggregate land-title type.
#[test]
fn land_title_serde_roundtrip() {
    let title = sample_title();
    let json = serde_json::to_string(&title).expect("serialize");
    let restored: LandTitle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(title, restored);
}

/// Serialization roundtrips for the lease and option aggregates.
#[test]
fn lease_and_option_serde_roundtrip() {
    let lease = Lease::new(
        "Landlord Pte Ltd",
        "Tenant Pte Ltd",
        "Warehouse",
        30,
        8_000_000,
    )
    .with_option_to_purchase();
    let lease_json = serde_json::to_string(&lease).expect("serialize");
    assert_eq!(
        lease,
        serde_json::from_str(&lease_json).expect("deserialize")
    );

    let otp = OptionToPurchase::private_resale("Condo", 150_000_000).exercise_on_day(7);
    let otp_json = serde_json::to_string(&otp).expect("serialize");
    assert_eq!(otp, serde_json::from_str(&otp_json).expect("deserialize"));
}

/// A typical lease registrability assessment completes well under 1 ms.
#[test]
fn lease_assessment_is_fast() {
    use std::time::Instant;

    let lease = Lease::new("Landlord", "Tenant", "Office floor", 30, 5_000_000).unregistered();
    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = assess_lease_registration(&lease);
    }
    let per_call = start.elapsed() / iterations;
    assert!(
        per_call.as_micros() < 1_000,
        "assessment too slow: {per_call:?} per call"
    );
}
