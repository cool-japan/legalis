//! Securities and Futures Act 2001 - Offers of Investments (Part 13)
//!
//! Models the **prospectus regime** for offers of securities, securities-based
//! derivatives and units in collective investment schemes under **Part 13** of
//! the Securities and Futures Act 2001.
//!
//! ## The prospectus requirement
//!
//! By SFA s. 240, a person must not make an offer of securities unless the offer
//! is made in or accompanied by a prospectus that has been **registered by MAS**
//! (s. 246), unless an exemption applies.
//!
//! ## The principal exemptions
//!
//! - **Small offers** (s. 272A): personal offers where the total amount raised
//!   within any 12-month period does not exceed
//!   [`SMALL_OFFER_CAP_CENTS`] (SGD 5 million).
//! - **Private placement** (s. 272B): offers made to no more than
//!   [`PRIVATE_PLACEMENT_MAX_PERSONS`] (50) persons within any 12-month period.
//! - **Institutional investors** (s. 274): offers made only to institutional
//!   investors.
//! - **Accredited investors** (s. 275): offers made only to accredited investors
//!   (and certain relevant persons).
//!
//! Monetary values are stored as **SGD cents** (`u64`).

use super::types::{CapitalMarketsProduct, InvestorClass};
use serde::{Deserialize, Serialize};

// ============================================================================
// Statutory thresholds (SFA Part 13)
// ============================================================================

/// Small-offers exemption ceiling: the total amount raised from personal offers
/// within any 12-month period must not exceed this amount (SFA s. 272A).
/// SGD 5,000,000, in cents.
pub const SMALL_OFFER_CAP_CENTS: u64 = 500_000_000;

/// Private-placement exemption ceiling: the number of persons to whom the offer
/// is made within any 12-month period must not exceed this figure (SFA s. 272B).
pub const PRIVATE_PLACEMENT_MAX_PERSONS: u32 = 50;

// ============================================================================
// Offering exemptions
// ============================================================================

/// An exemption from the Part 13 prospectus requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferingExemption {
    /// Small offers exemption: total raised within a 12-month period does not
    /// exceed SGD 5 million (SFA s. 272A).
    SmallOffer,
    /// Private placement: offer made to no more than 50 persons within a 12-month
    /// period (SFA s. 272B).
    PrivatePlacement,
    /// Offer made only to institutional investors (SFA s. 274).
    InstitutionalInvestors,
    /// Offer made only to accredited investors and other relevant persons
    /// (SFA s. 275).
    AccreditedInvestors,
}

impl OfferingExemption {
    /// Returns the statutory reference for this exemption.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            OfferingExemption::SmallOffer => "SFA s. 272A",
            OfferingExemption::PrivatePlacement => "SFA s. 272B",
            OfferingExemption::InstitutionalInvestors => "SFA s. 274",
            OfferingExemption::AccreditedInvestors => "SFA s. 275",
        }
    }

    /// Returns a plain-language description of the exemption.
    pub fn description(&self) -> &'static str {
        match self {
            OfferingExemption::SmallOffer => {
                "Small offers - total raised within any 12-month period not exceeding SGD 5 million"
            }
            OfferingExemption::PrivatePlacement => {
                "Private placement - offer to no more than 50 persons within any 12-month period"
            }
            OfferingExemption::InstitutionalInvestors => {
                "Offer made only to institutional investors"
            }
            OfferingExemption::AccreditedInvestors => {
                "Offer made only to accredited investors and other relevant persons"
            }
        }
    }
}

// ============================================================================
// Prospectus
// ============================================================================

/// A prospectus for an offer of securities under Part 13.
///
/// To support a public offer a prospectus must be **registered by MAS**
/// (s. 246) and must not contain a false or misleading statement, nor omit
/// information that investors and their advisers would reasonably require
/// (s. 243, s. 253).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prospectus {
    /// Whether the prospectus has been registered by MAS (s. 246).
    pub registered_with_mas: bool,
    /// Whether the prospectus contains a false or misleading statement
    /// (s. 253(1)).
    pub has_false_or_misleading_statement: bool,
    /// Whether the prospectus omits information required by s. 243.
    pub omits_required_information: bool,
}

impl Prospectus {
    /// Creates a registered, defect-free prospectus.
    pub fn registered() -> Self {
        Self {
            registered_with_mas: true,
            has_false_or_misleading_statement: false,
            omits_required_information: false,
        }
    }

    /// Creates an unregistered (lodged but not yet registered) prospectus.
    pub fn unregistered() -> Self {
        Self {
            registered_with_mas: false,
            has_false_or_misleading_statement: false,
            omits_required_information: false,
        }
    }

    /// Records that the prospectus contains a false or misleading statement.
    pub fn with_false_statement(mut self) -> Self {
        self.has_false_or_misleading_statement = true;
        self
    }

    /// Records that the prospectus omits required information.
    pub fn with_omission(mut self) -> Self {
        self.omits_required_information = true;
        self
    }

    /// Whether the prospectus is defective (false/misleading statement or a
    /// material omission) under s. 253.
    pub fn is_defective(&self) -> bool {
        self.has_false_or_misleading_statement || self.omits_required_information
    }
}

// ============================================================================
// Securities offering
// ============================================================================

/// An offer of capital markets products being assessed against the Part 13
/// prospectus regime.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecuritiesOffering {
    /// Identifier for the offering.
    pub offering_id: String,
    /// The class of capital markets product being offered.
    pub product: CapitalMarketsProduct,
    /// Total amount to be raised by this offer, in SGD cents.
    pub amount_cents: u64,
    /// Total amount raised from comparable personal offers within the preceding
    /// 12 months, in SGD cents (relevant to the small-offers ceiling).
    pub raised_in_12_months_cents: u64,
    /// Number of persons to whom the offer is made (relevant to private
    /// placement).
    pub number_of_offerees: u32,
    /// The class of the offerees.
    pub offeree_class: InvestorClass,
    /// An exemption claimed, if any.
    pub exemption: Option<OfferingExemption>,
    /// A prospectus, where one has been prepared.
    pub prospectus: Option<Prospectus>,
}

impl SecuritiesOffering {
    /// Creates a new offering record (retail offerees, no exemption, no
    /// prospectus by default).
    pub fn new(
        offering_id: impl Into<String>,
        product: CapitalMarketsProduct,
        amount_cents: u64,
    ) -> Self {
        Self {
            offering_id: offering_id.into(),
            product,
            amount_cents,
            raised_in_12_months_cents: amount_cents,
            number_of_offerees: 0,
            offeree_class: InvestorClass::Retail,
            exemption: None,
            prospectus: None,
        }
    }

    /// Sets the total amount raised within the preceding 12 months (defaults to
    /// the offer amount).
    pub fn with_raised_in_12_months(mut self, cents: u64) -> Self {
        self.raised_in_12_months_cents = cents;
        self
    }

    /// Sets the number of offerees.
    pub fn with_offerees(mut self, count: u32) -> Self {
        self.number_of_offerees = count;
        self
    }

    /// Sets the class of the offerees.
    pub fn with_offeree_class(mut self, class: InvestorClass) -> Self {
        self.offeree_class = class;
        self
    }

    /// Records an exemption claimed for this offering.
    pub fn with_exemption(mut self, exemption: OfferingExemption) -> Self {
        self.exemption = Some(exemption);
        self
    }

    /// Records a prospectus prepared for this offering.
    pub fn with_prospectus(mut self, prospectus: Prospectus) -> Self {
        self.prospectus = Some(prospectus);
        self
    }

    /// Whether a claimed exemption is, on the recorded facts, actually made out.
    ///
    /// - Small offers (s. 272A): the amount raised within 12 months must not
    ///   exceed [`SMALL_OFFER_CAP_CENTS`].
    /// - Private placement (s. 272B): the number of offerees must not exceed
    ///   [`PRIVATE_PLACEMENT_MAX_PERSONS`].
    /// - Institutional (s. 274): every offeree must be an institutional investor.
    /// - Accredited (s. 275): every offeree must be an accredited (or
    ///   institutional) investor.
    pub fn exemption_made_out(&self) -> bool {
        match self.exemption {
            None => false,
            Some(OfferingExemption::SmallOffer) => {
                self.raised_in_12_months_cents <= SMALL_OFFER_CAP_CENTS
            }
            Some(OfferingExemption::PrivatePlacement) => {
                self.number_of_offerees <= PRIVATE_PLACEMENT_MAX_PERSONS
            }
            Some(OfferingExemption::InstitutionalInvestors) => {
                self.offeree_class == InvestorClass::Institutional
            }
            Some(OfferingExemption::AccreditedInvestors) => self.offeree_class.is_sophisticated(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemption_references() {
        assert_eq!(
            OfferingExemption::SmallOffer.statute_reference(),
            "SFA s. 272A"
        );
        assert_eq!(
            OfferingExemption::PrivatePlacement.statute_reference(),
            "SFA s. 272B"
        );
        assert_eq!(
            OfferingExemption::InstitutionalInvestors.statute_reference(),
            "SFA s. 274"
        );
        assert_eq!(
            OfferingExemption::AccreditedInvestors.statute_reference(),
            "SFA s. 275"
        );
    }

    #[test]
    fn test_prospectus_defect() {
        assert!(!Prospectus::registered().is_defective());
        assert!(
            Prospectus::registered()
                .with_false_statement()
                .is_defective()
        );
        assert!(Prospectus::registered().with_omission().is_defective());
        assert!(!Prospectus::unregistered().registered_with_mas);
    }

    #[test]
    fn test_small_offer_at_cap_is_within() {
        let offering =
            SecuritiesOffering::new("of-1", CapitalMarketsProduct::Securities, 500_000_000)
                .with_exemption(OfferingExemption::SmallOffer);
        // Exactly SGD 5m is within the ceiling.
        assert!(offering.exemption_made_out());
    }

    #[test]
    fn test_small_offer_over_cap_fails() {
        let offering =
            SecuritiesOffering::new("of-2", CapitalMarketsProduct::Securities, 500_000_001)
                .with_exemption(OfferingExemption::SmallOffer);
        assert!(!offering.exemption_made_out());
    }

    #[test]
    fn test_private_placement_boundary() {
        let at_limit = SecuritiesOffering::new("of-3", CapitalMarketsProduct::Securities, 1)
            .with_offerees(50)
            .with_exemption(OfferingExemption::PrivatePlacement);
        assert!(at_limit.exemption_made_out());

        let over_limit = SecuritiesOffering::new("of-4", CapitalMarketsProduct::Securities, 1)
            .with_offerees(51)
            .with_exemption(OfferingExemption::PrivatePlacement);
        assert!(!over_limit.exemption_made_out());
    }

    #[test]
    fn test_institutional_exemption_requires_institutional_offerees() {
        let ok = SecuritiesOffering::new("of-5", CapitalMarketsProduct::Securities, 100_000_000)
            .with_offeree_class(InvestorClass::Institutional)
            .with_exemption(OfferingExemption::InstitutionalInvestors);
        assert!(ok.exemption_made_out());

        let retail =
            SecuritiesOffering::new("of-6", CapitalMarketsProduct::Securities, 100_000_000)
                .with_offeree_class(InvestorClass::Retail)
                .with_exemption(OfferingExemption::InstitutionalInvestors);
        assert!(!retail.exemption_made_out());
    }

    #[test]
    fn test_accredited_exemption_accepts_institutional_and_accredited() {
        let accredited =
            SecuritiesOffering::new("of-7", CapitalMarketsProduct::Securities, 100_000_000)
                .with_offeree_class(InvestorClass::Accredited)
                .with_exemption(OfferingExemption::AccreditedInvestors);
        assert!(accredited.exemption_made_out());

        let retail =
            SecuritiesOffering::new("of-8", CapitalMarketsProduct::Securities, 100_000_000)
                .with_offeree_class(InvestorClass::Retail)
                .with_exemption(OfferingExemption::AccreditedInvestors);
        assert!(!retail.exemption_made_out());
    }

    #[test]
    fn test_no_exemption_is_not_made_out() {
        let offering =
            SecuritiesOffering::new("of-9", CapitalMarketsProduct::Securities, 100_000_000);
        assert!(!offering.exemption_made_out());
    }

    #[test]
    fn test_offering_serde_roundtrip() {
        let offering =
            SecuritiesOffering::new("of-10", CapitalMarketsProduct::Securities, 200_000_000)
                .with_offerees(20)
                .with_offeree_class(InvestorClass::Accredited)
                .with_exemption(OfferingExemption::AccreditedInvestors)
                .with_prospectus(Prospectus::registered());
        let json = serde_json::to_string(&offering).expect("serialize");
        let back: SecuritiesOffering = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(offering, back);
    }
}
