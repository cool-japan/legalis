//! Securities and Futures Act 2001 (Singapore)
//!
//! Type-safe modelling of Singapore's capital markets regime under the
//! **Securities and Futures Act 2001 (SFA)**, administered and enforced by the
//! **Monetary Authority of Singapore (MAS)**.
//!
//! # Overview
//!
//! The SFA is the principal statute for Singapore's capital markets. This module
//! models four pillars of the regime:
//!
//! ## 1. Capital markets products (s. 2(1))
//!
//! A *capital markets product* is any securities, units in a collective
//! investment scheme, derivatives contract, or spot foreign exchange contract
//! for leveraged foreign exchange trading. See [`CapitalMarketsProduct`],
//! [`Security`], [`DerivativesContract`] and [`CollectiveInvestmentScheme`].
//!
//! ## 2. Offers of investments - the prospectus regime (Part 13)
//!
//! By [SFA s. 240], an offer of securities or units in a collective investment
//! scheme must be made in or accompanied by a prospectus **registered by MAS**
//! (s. 246), unless an exemption applies. The principal exemptions are modelled
//! by [`OfferingExemption`]:
//!
//! - **Small offers** (s. 272A) - total raised within any 12-month period does
//!   not exceed [`SMALL_OFFER_CAP_CENTS`] (SGD 5 million);
//! - **Private placement** (s. 272B) - offer to no more than
//!   [`PRIVATE_PLACEMENT_MAX_PERSONS`] (50) persons within any 12-month period;
//! - **Institutional investors** (s. 274);
//! - **Accredited investors** (s. 275).
//!
//! ## 3. Market conduct (Part 12)
//!
//! The market-misconduct prohibitions, enforced criminally and through the civil
//! penalty regime (s. 232):
//!
//! - **Insider trading** - by a connected person (s. 218) and by any other person
//!   (s. 219): [`InsiderTradingClaim`];
//! - **False trading and market rigging** (s. 197): [`FalseTradingClaim`];
//! - **Employment of manipulative or deceptive devices** (s. 201):
//!   [`MarketManipulationClaim`];
//! - **False or misleading statements** (s. 199): [`MisleadingStatementClaim`];
//! - **Fraudulent inducement to deal** (s. 200): [`FraudulentInducementClaim`].
//!
//! ## 4. Licensing (Part 4)
//!
//! A person must not carry on business in a regulated activity (Second Schedule)
//! without a **Capital Markets Services (CMS) licence** (s. 82):
//! [`CapitalMarketsServicesLicence`]. Representatives must be appointed and
//! entered on the MAS public register (s. 99B): [`AppointedRepresentative`].
//!
//! # Example
//!
//! ```rust
//! use legalis_sg::securities::*;
//!
//! // A public offer of shares with no exemption needs a registered prospectus.
//! let offering =
//!     SecuritiesOffering::new("ipo-1", CapitalMarketsProduct::Securities, 5_000_000_000);
//! assert!(matches!(
//!     assess_prospectus_requirement(&offering),
//!     Err(SecuritiesError::ProspectusRequired { .. })
//! ));
//!
//! // A private placement to no more than 50 persons is exempt (s. 272B).
//! let placement =
//!     SecuritiesOffering::new("pp-1", CapitalMarketsProduct::Securities, 5_000_000_000)
//!         .with_offerees(30)
//!         .with_exemption(OfferingExemption::PrivatePlacement);
//! assert!(assess_prospectus_requirement(&placement).is_ok());
//!
//! // Insider trading by a connected director infringes s. 218.
//! let insider = InsiderTradingClaim::new("it-1", true);
//! assert!(assess_insider_trading(&insider).is_err());
//!
//! // The civil penalty cap is three times the profit gained (s. 232).
//! assert_eq!(max_civil_penalty_cents(100_000_000, true), 300_000_000);
//! ```
//!
//! # Statute references
//!
//! - `SFA s. 2(1)` - definition of capital markets products
//! - `SFA s. 4A` - institutional and accredited investors
//! - `SFA s. 82-83, s. 99B` - CMS licence and representatives (Part 4)
//! - `SFA s. 197, s. 199-201` - false trading, manipulation, misleading statements
//! - `SFA s. 218-219` - insider trading (Part 12 Division 3)
//! - `SFA s. 232` - civil penalty regime
//! - `SFA s. 240, s. 246, s. 272A-275` - prospectus and exemptions (Part 13)
//! - `SFA s. 286-287` - authorised / recognised collective investment schemes
//!
//! [SFA s. 240]: https://sso.agc.gov.sg/Act/SFA2001
//!
//! # Module structure
//!
//! - [`error`] - [`SecuritiesError`] with statute references and severity
//! - [`types`] - capital markets products, investor classes and licensing
//! - [`offerings`] - Part 13 offers, prospectus and exemptions
//! - [`misconduct`] - Part 12 market-conduct claims and the civil penalty cap
//! - [`validator`] - assessment functions and report structs

pub mod error;
pub mod misconduct;
pub mod offerings;
pub mod types;
pub mod validator;

pub use error::*;
pub use misconduct::*;
pub use offerings::*;
pub use types::*;
pub use validator::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_to_end_ipo_requires_prospectus() {
        // A retail IPO of shares with no exemption and no prospectus.
        let offering =
            SecuritiesOffering::new("ipo-e2e", CapitalMarketsProduct::Securities, 10_000_000_000);
        let report = assess_offering_report(&offering);
        assert!(!report.compliant);
        assert!(report.prospectus_required);
        assert!(assess_prospectus_requirement(&offering).is_err());
    }

    #[test]
    fn test_end_to_end_accredited_placement_is_compliant() {
        let offering =
            SecuritiesOffering::new("ap-e2e", CapitalMarketsProduct::Securities, 2_000_000_000)
                .with_offeree_class(InvestorClass::Accredited)
                .with_offerees(15)
                .with_exemption(OfferingExemption::AccreditedInvestors);
        assert!(assess_prospectus_requirement(&offering).is_ok());
    }

    #[test]
    fn test_offering_json_roundtrip() {
        let offering = SecuritiesOffering::new(
            "of-rt",
            CapitalMarketsProduct::CollectiveInvestmentSchemeUnits,
            800_000_000,
        )
        .with_offerees(40)
        .with_offeree_class(InvestorClass::Accredited)
        .with_exemption(OfferingExemption::PrivatePlacement)
        .with_prospectus(Prospectus::registered());
        let json = serde_json::to_string(&offering).expect("serialize");
        let decoded: SecuritiesOffering = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(offering, decoded);
    }

    #[test]
    fn test_licence_json_roundtrip() {
        let licence = CapitalMarketsServicesLicence::new(
            "Omega Asset Management Pte Ltd",
            vec![
                RegulatedActivity::FundManagement,
                RegulatedActivity::DealingInCapitalMarketsProducts,
            ],
        );
        let json = serde_json::to_string(&licence).expect("serialize");
        let decoded: CapitalMarketsServicesLicence =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(licence, decoded);
    }

    #[test]
    fn test_insider_claim_json_roundtrip() {
        let claim =
            InsiderTradingClaim::new("it-rt", false).with_conduct(InsiderConduct::Communicated);
        let json = serde_json::to_string(&claim).expect("serialize");
        let decoded: InsiderTradingClaim = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(claim, decoded);
    }

    #[test]
    fn test_offering_report_json_roundtrip() {
        let offering =
            SecuritiesOffering::new("rep-rt", CapitalMarketsProduct::Securities, 100_000_000)
                .with_exemption(OfferingExemption::SmallOffer);
        let report = assess_offering_report(&offering);
        let json = serde_json::to_string(&report).expect("serialize");
        let decoded: OfferingReport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(report, decoded);
    }
}
