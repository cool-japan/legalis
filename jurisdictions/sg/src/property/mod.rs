//! Property Law (Singapore)
//!
//! Type-safe modelling of Singapore land law. Land in Singapore is held under the
//! **Torrens system** of title by registration, administered by the Singapore
//! Land Authority under the **Land Titles Act 1993 (LTA)**, supplemented by the
//! **Conveyancing and Law of Property Act 1886 (CLPA)**, the **Civil Law Act
//! 1909** and equitable doctrine.
//!
//! # Overview
//!
//! ## 1. Registered title and indefeasibility (LTA s. 46)
//!
//! Registration confers an **indefeasible** title - one free of unregistered
//! interests (s. 46(1)). The cardinal exceptions are **fraud or forgery** to
//! which the proprietor was a party or privy (s. 46(2)), and the **in personam**
//! exception recognised in *United Overseas Bank v Bebe* \[2006\] SGCA 30.
//! Certain unregistered **overriding interests** (short leases, easements) bind
//! the title without defeating it. See [`LandTitle`],
//! [`IndefeasibilityException`] and [`IndefeasibilityChallenge`].
//!
//! ## 2. Caveats (LTA s. 115)
//!
//! A person with a **caveatable interest** - a proprietary interest, not a mere
//! personal right - may lodge a [`Caveat`], which operates as a statutory
//! injunction against the registration of inconsistent dealings.
//!
//! ## 3. Conveyancing
//!
//! A contract for the disposition of land must be evidenced in writing and signed
//! (Civil Law Act s. 6(d)). The standard private-property mechanism is the
//! **Option to Purchase** ([`OptionToPurchase`]): an option fee (conventionally
//! 1%), an option period (conventionally 14 days), exercise by payment of the
//! balance deposit (a further 4%), and [`Completion`]. **Buyer's Stamp Duty** is
//! computed by [`compute_buyers_stamp_duty_cents`] (Stamp Duties Act 1929).
//!
//! ## 4. Leases
//!
//! A lease for a term **exceeding 7 years** must be registered to create a legal
//! leasehold estate (LTA s. 45); a short lease with the tenant in occupation
//! binds as an overriding interest (s. 46(1)). See [`Lease`], [`LeaseCovenant`],
//! [`LeaseDetermination`] and [`ForfeitureClaim`] (forfeiture under CLPA s. 18).
//!
//! ## 5. Interests
//!
//! A [`Mortgage`] of registered land takes effect as a **charge** (LTA s. 68); an
//! [`Easement`] must satisfy the *Re Ellenborough Park* \[1956\] Ch 131
//! characteristics.
//!
//! # Example
//!
//! ```rust
//! use legalis_sg::property::*;
//!
//! // A lease for a term exceeding 7 years must be registered (LTA s. 45).
//! let lease = Lease::new("Landlord", "Tenant", "Office floor", 30, 5_000_000).unregistered();
//! assert!(matches!(
//!     assess_lease_registration(&lease),
//!     Err(PropertyError::LeaseNotRegistered { years: 30 })
//! ));
//!
//! // Fraud by the proprietor defeats indefeasibility (LTA s. 46(2)).
//! let title = LandTitle::new(
//!     "Lot 1234A MK 5",
//!     "Condominium unit",
//!     Tenure::Leasehold99,
//!     PropertyType::Residential,
//!     RegisteredProprietor::new("Tan Ah Kow"),
//! );
//! let fraud = IndefeasibilityChallenge::fraud("proprietor registered a forged transfer");
//! assert!(assess_indefeasibility(&title, Some(&fraud)).is_err());
//!
//! // Buyer's Stamp Duty on a SGD 1,000,000 residential purchase is SGD 24,600.
//! assert_eq!(
//!     compute_buyers_stamp_duty_cents(100_000_000, PropertyType::Residential),
//!     2_460_000
//! );
//! ```
//!
//! # Statute references
//!
//! - `Land Titles Act s. 45` - registration of instruments
//! - `Land Titles Act s. 46` - indefeasibility and its exceptions
//! - `Land Titles Act s. 68` - mortgage takes effect as a charge
//! - `Land Titles Act s. 115` - caveats
//! - `Conveyancing and Law of Property Act s. 18` - forfeiture of leases
//! - `Civil Law Act s. 6(d)` - writing requirement for land contracts
//! - `Stamp Duties Act 1929` - Buyer's Stamp Duty
//! - `Re Ellenborough Park [1956] Ch 131` - characteristics of an easement
//! - `United Overseas Bank v Bebe [2006] SGCA 30` - indefeasibility / in personam
//!
//! # Module structure
//!
//! - [`error`] - [`PropertyError`] with statute references and severity
//! - [`types`] - Torrens title, indefeasibility, caveats, mortgages, easements
//! - [`leases`] - leases, covenants, determination and forfeiture
//! - [`conveyancing`] - sale and purchase, option to purchase, completion, BSD
//! - [`validator`] - assessment functions and report structs

pub mod conveyancing;
pub mod error;
pub mod leases;
pub mod types;
pub mod validator;

pub use conveyancing::*;
pub use error::*;
pub use leases::*;
pub use types::*;
pub use validator::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_to_end_conveyancing_flow() {
        // OTP granted and validly exercised, then completion.
        let otp = OptionToPurchase::private_resale("Condo #10-11", 200_000_000).exercise_on_day(10);
        assert!(assess_option_to_purchase(&otp).is_ok());

        let sap = SaleAndPurchase::new(
            "Vendor",
            "Purchaser",
            "Condo #10-11",
            PropertyType::Residential,
            200_000_000,
        );
        assert!(assess_sale_and_purchase(&sap).is_ok());
        // BSD on SGD 2,000,000 residential = SGD 69,600.
        assert_eq!(sap.buyers_stamp_duty_cents(), 6_960_000);

        let completion = Completion::pending()
            .with_balance_paid()
            .with_transfer_executed()
            .with_vacant_possession();
        assert!(completion.is_complete());
    }

    #[test]
    fn test_end_to_end_torrens_fraud() {
        let title = LandTitle::new(
            "Lot 9999Z MK 2",
            "Landed house",
            Tenure::Freehold,
            PropertyType::Residential,
            RegisteredProprietor::new("Fraudster Pte Ltd"),
        );
        let fraud = IndefeasibilityChallenge::fraud("registered proprietor forged the transfer");
        let report = assess_indefeasibility_report(&title, Some(&fraud));
        assert!(report.title_defeated);
        assert!(assess_indefeasibility(&title, Some(&fraud)).is_err());
    }

    #[test]
    fn test_land_title_json_roundtrip() {
        let title = LandTitle::new(
            "Lot 1234A MK 5",
            "Condominium unit #12-34",
            Tenure::Leasehold99,
            PropertyType::Residential,
            RegisteredProprietor::new("Tan Ah Kow").with_identifier("S1234567A"),
        );
        let json = serde_json::to_string(&title).expect("serialize");
        let decoded: LandTitle = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(title, decoded);
    }

    #[test]
    fn test_lease_json_roundtrip() {
        let lease = Lease::new(
            "Landlord Pte Ltd",
            "Tenant Pte Ltd",
            "Office",
            30,
            5_000_000,
        );
        let json = serde_json::to_string(&lease).expect("serialize");
        let decoded: Lease = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(lease, decoded);
    }

    #[test]
    fn test_option_json_roundtrip() {
        let otp = OptionToPurchase::private_resale("Condo #10-11", 150_000_000).exercise_on_day(7);
        let json = serde_json::to_string(&otp).expect("serialize");
        let decoded: OptionToPurchase = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(otp, decoded);
    }

    #[test]
    fn test_lease_assessment_json_roundtrip() {
        let lease = Lease::new("L", "T", "Shop", 5, 500_000);
        let report = assess_lease_report(&lease);
        let json = serde_json::to_string(&report).expect("serialize");
        let decoded: LeaseAssessment = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(report, decoded);
    }

    #[test]
    fn test_indefeasibility_report_json_roundtrip() {
        let title = LandTitle::new(
            "Lot 1",
            "Unit",
            Tenure::Freehold,
            PropertyType::Residential,
            RegisteredProprietor::new("Owner"),
        );
        let report = assess_indefeasibility_report(&title, None);
        let json = serde_json::to_string(&report).expect("serialize");
        let decoded: IndefeasibilityReport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(report, decoded);
    }
}
