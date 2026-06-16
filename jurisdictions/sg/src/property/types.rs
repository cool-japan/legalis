//! Property Law - Torrens Title and Interests in Land
//!
//! Type-safe models of Singapore land law under the **Torrens system** of title
//! by registration (Land Titles Act 1993):
//!
//! - **Registered title** and the **indefeasibility** of a proprietor's title
//!   (LTA s. 46), together with the statutory exceptions (fraud/forgery,
//!   s. 46(2); the overriding interests of s. 46(1)) and the in personam
//!   exception (*United Overseas Bank v Bebe* \[2006\] SGCA 30).
//! - **Caveats** lodged to protect unregistered interests (LTA s. 115).
//! - **Interests in land**: mortgages/charges (LTA s. 68) and easements
//!   (*Re Ellenborough Park* \[1956\] Ch 131).
//!
//! Monetary values are stored as **SGD cents** (`u64`).

use serde::{Deserialize, Serialize};

// ============================================================================
// Tenure and property type
// ============================================================================

/// The tenure (duration of the estate) by which land is held.
///
/// Singapore land is predominantly held on freehold, 999-year and 99-year
/// leasehold tenure (the latter typical of HDB flats and many condominiums).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tenure {
    /// Freehold (estate in fee simple) - the most extensive tenure.
    Freehold,
    /// 999-year leasehold (treated as near-freehold in practice).
    Leasehold999,
    /// 99-year leasehold (typical of HDB flats and many private developments).
    Leasehold99,
    /// Some other leasehold tenure (e.g. 60- or 103-year grants).
    LeaseholdOther,
}

impl Tenure {
    /// Returns a plain-language description of the tenure.
    pub fn description(&self) -> &'static str {
        match self {
            Tenure::Freehold => "Freehold (estate in fee simple)",
            Tenure::Leasehold999 => "999-year leasehold",
            Tenure::Leasehold99 => "99-year leasehold",
            Tenure::LeaseholdOther => "Other leasehold tenure",
        }
    }

    /// Whether the tenure is freehold.
    pub fn is_freehold(&self) -> bool {
        matches!(self, Tenure::Freehold)
    }
}

/// The use class of a property, relevant (among other things) to the rate of
/// Buyer's Stamp Duty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Residential property (e.g. a flat, condominium unit or landed house).
    Residential,
    /// Commercial property (e.g. office or retail).
    Commercial,
    /// Industrial property (e.g. a factory or warehouse).
    Industrial,
    /// Mixed-use property.
    MixedUse,
}

impl PropertyType {
    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            PropertyType::Residential => "Residential property",
            PropertyType::Commercial => "Commercial property",
            PropertyType::Industrial => "Industrial property",
            PropertyType::MixedUse => "Mixed-use property",
        }
    }

    /// Whether the property is residential (the residential Buyer's Stamp Duty
    /// scale applies).
    pub fn is_residential(&self) -> bool {
        matches!(self, PropertyType::Residential)
    }
}

// ============================================================================
// Registered title (Torrens)
// ============================================================================

/// The registered proprietor of land (or of a registered interest).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegisteredProprietor {
    /// Name of the proprietor.
    pub name: String,
    /// NRIC (for an individual) or UEN (for an entity), if recorded.
    pub identifier: Option<String>,
}

impl RegisteredProprietor {
    /// Creates a new registered proprietor.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            identifier: None,
        }
    }

    /// Records the proprietor's NRIC/UEN.
    pub fn with_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.identifier = Some(identifier.into());
        self
    }
}

/// A parcel of registered land and its title.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandTitle {
    /// The lot / land-register folio reference (e.g. "Lot 1234A MK 5").
    pub lot_reference: String,
    /// A short description of the property.
    pub description: String,
    /// Tenure of the estate.
    pub tenure: Tenure,
    /// Use class of the property.
    pub property_type: PropertyType,
    /// The registered proprietor.
    pub proprietor: RegisteredProprietor,
    /// Whether the parcel is brought under the Land Titles Act (registered land).
    pub registered: bool,
}

impl LandTitle {
    /// Creates a new registered land title.
    pub fn new(
        lot_reference: impl Into<String>,
        description: impl Into<String>,
        tenure: Tenure,
        property_type: PropertyType,
        proprietor: RegisteredProprietor,
    ) -> Self {
        Self {
            lot_reference: lot_reference.into(),
            description: description.into(),
            tenure,
            property_type,
            proprietor,
            registered: true,
        }
    }

    /// Marks the parcel as not yet brought under the Land Titles Act.
    pub fn unregistered(mut self) -> Self {
        self.registered = false;
        self
    }
}

// ============================================================================
// Indefeasibility (LTA s. 46)
// ============================================================================

/// An exception to the indefeasibility of a registered proprietor's title
/// (Land Titles Act s. 46).
///
/// The exceptions divide into those that **defeat** the proprietor's title
/// (fraud, forgery, an in personam claim) and **overriding interests** that
/// **bind** the title even though unregistered (short leases, easements).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndefeasibilityException {
    /// Fraud to which the proprietor (or the proprietor's agent) was a party or
    /// privy (s. 46(2)(a)).
    Fraud,
    /// Forgery - the instrument under which the proprietor took was forged
    /// (s. 46(2)).
    Forgery,
    /// An in personam claim against the proprietor based on a known legal or
    /// equitable obligation (*United Overseas Bank v Bebe* \[2006\] SGCA 30).
    InPersonam,
    /// The overriding interest of a tenant in occupation under a short lease
    /// (not exceeding 7 years) - s. 46(1).
    OverridingShortLease,
    /// An easement subsisting over the land as an overriding interest
    /// (s. 46(1)).
    OverridingEasement,
    /// A subsisting prior registered interest notified on the land-register.
    PriorRegisteredInterest,
}

impl IndefeasibilityException {
    /// Returns the statutory reference / authority for this exception.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            IndefeasibilityException::Fraud => "Land Titles Act s. 46(2)(a)",
            IndefeasibilityException::Forgery => "Land Titles Act s. 46(2)",
            IndefeasibilityException::InPersonam => {
                "United Overseas Bank v Bebe [2006] SGCA 30 (in personam exception)"
            }
            IndefeasibilityException::OverridingShortLease => "Land Titles Act s. 46(1)",
            IndefeasibilityException::OverridingEasement => "Land Titles Act s. 46(1)",
            IndefeasibilityException::PriorRegisteredInterest => "Land Titles Act s. 46(1)",
        }
    }

    /// Whether this exception **defeats** the proprietor's title (as opposed to
    /// merely binding it as an overriding/registered interest).
    ///
    /// Fraud, forgery and an in personam claim defeat title; overriding and
    /// prior registered interests bind it without defeating it.
    pub fn defeats_title(&self) -> bool {
        matches!(
            self,
            IndefeasibilityException::Fraud
                | IndefeasibilityException::Forgery
                | IndefeasibilityException::InPersonam
        )
    }
}

/// A challenge to the indefeasibility of a registered title.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndefeasibilityChallenge {
    /// The exception relied on.
    pub exception: IndefeasibilityException,
    /// For fraud/forgery: whether the registered proprietor was a party or privy
    /// to it. Indefeasibility is only displaced where the proprietor (or agent)
    /// was party or privy - a bona fide purchaser for value takes free of an
    /// antecedent fraud (immediate indefeasibility, *UOB v Bebe*).
    pub proprietor_party_or_privy: bool,
    /// Description of the challenge.
    pub detail: String,
}

impl IndefeasibilityChallenge {
    /// Creates a fraud challenge (proprietor party or privy by default).
    pub fn fraud(detail: impl Into<String>) -> Self {
        Self {
            exception: IndefeasibilityException::Fraud,
            proprietor_party_or_privy: true,
            detail: detail.into(),
        }
    }

    /// Creates a challenge based on the given exception.
    pub fn new(exception: IndefeasibilityException, detail: impl Into<String>) -> Self {
        Self {
            exception,
            proprietor_party_or_privy: exception.defeats_title(),
            detail: detail.into(),
        }
    }

    /// Records that the proprietor was not a party or privy to the fraud/forgery
    /// (a bona fide purchaser for value), which preserves indefeasibility.
    pub fn proprietor_innocent(mut self) -> Self {
        self.proprietor_party_or_privy = false;
        self
    }

    /// Whether the challenge defeats the proprietor's title.
    ///
    /// Fraud/forgery defeat title only where the proprietor was party or privy;
    /// an in personam claim binds the proprietor regardless; overriding and prior
    /// registered interests bind but do not defeat title.
    pub fn defeats_title(&self) -> bool {
        match self.exception {
            IndefeasibilityException::Fraud | IndefeasibilityException::Forgery => {
                self.proprietor_party_or_privy
            }
            IndefeasibilityException::InPersonam => true,
            _ => false,
        }
    }
}

// ============================================================================
// Caveats (LTA s. 115)
// ============================================================================

/// A caveat lodged against a registered title to protect an unregistered
/// interest (Land Titles Act s. 115).
///
/// A caveat operates as a statutory injunction: while it subsists, the Registrar
/// must not register a dealing inconsistent with the caveator's claimed interest.
/// A caveat may only be lodged by a person who has a **caveatable interest** - a
/// proprietary interest in the land, not a mere personal or contractual right.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Caveat {
    /// The person lodging the caveat.
    pub caveator: String,
    /// The lot reference of the land caveated.
    pub lot_reference: String,
    /// Description of the interest claimed.
    pub claimed_interest: String,
    /// Whether the claimed interest is a proprietary (caveatable) interest.
    pub has_caveatable_interest: bool,
    /// Whether the caveat is currently lodged (in force).
    pub lodged: bool,
}

impl Caveat {
    /// Creates a new lodged caveat supported by a caveatable interest.
    pub fn new(
        caveator: impl Into<String>,
        lot_reference: impl Into<String>,
        claimed_interest: impl Into<String>,
    ) -> Self {
        Self {
            caveator: caveator.into(),
            lot_reference: lot_reference.into(),
            claimed_interest: claimed_interest.into(),
            has_caveatable_interest: true,
            lodged: true,
        }
    }

    /// Records that the asserted interest is not a caveatable (proprietary)
    /// interest.
    pub fn without_caveatable_interest(mut self) -> Self {
        self.has_caveatable_interest = false;
        self
    }

    /// Whether the caveat presently prohibits the registration of inconsistent
    /// dealings (it is lodged and supported by a caveatable interest).
    pub fn prohibits_registration(&self) -> bool {
        self.lodged && self.has_caveatable_interest
    }
}

// ============================================================================
// Mortgages / charges (LTA s. 68)
// ============================================================================

/// A remedy available to a registered mortgagee on the mortgagor's default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MortgageeRemedy {
    /// Exercising the statutory power of sale.
    PowerOfSale,
    /// Obtaining an order for foreclosure.
    Foreclosure,
    /// Taking possession of the mortgaged land.
    Possession,
    /// Appointing a receiver of the income of the land.
    AppointReceiver,
}

impl MortgageeRemedy {
    /// Returns a plain-language description of the remedy.
    pub fn description(&self) -> &'static str {
        match self {
            MortgageeRemedy::PowerOfSale => "Exercise of the statutory power of sale",
            MortgageeRemedy::Foreclosure => "Order for foreclosure",
            MortgageeRemedy::Possession => "Taking possession of the mortgaged land",
            MortgageeRemedy::AppointReceiver => "Appointment of a receiver",
        }
    }
}

/// A mortgage of registered land.
///
/// Under the Torrens system a mortgage **takes effect as a charge** and does not
/// transfer the legal estate to the mortgagee (Land Titles Act s. 68); the
/// mortgagee obtains a registered security interest with the statutory remedies
/// on default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mortgage {
    /// The mortgagor (borrower / registered proprietor).
    pub mortgagor: String,
    /// The mortgagee (lender).
    pub mortgagee: String,
    /// The lot reference of the mortgaged land.
    pub lot_reference: String,
    /// Principal sum secured, in SGD cents.
    pub principal_cents: u64,
    /// Whether the mortgage (charge) is registered (LTA s. 45).
    pub registered: bool,
    /// Whether the mortgagor is in default.
    pub in_default: bool,
}

impl Mortgage {
    /// Creates a new registered mortgage that is not in default.
    pub fn new(
        mortgagor: impl Into<String>,
        mortgagee: impl Into<String>,
        lot_reference: impl Into<String>,
        principal_cents: u64,
    ) -> Self {
        Self {
            mortgagor: mortgagor.into(),
            mortgagee: mortgagee.into(),
            lot_reference: lot_reference.into(),
            principal_cents,
            registered: true,
            in_default: false,
        }
    }

    /// Marks the mortgage as unregistered.
    pub fn unregistered(mut self) -> Self {
        self.registered = false;
        self
    }

    /// Marks the mortgagor as in default.
    pub fn in_default(mut self) -> Self {
        self.in_default = true;
        self
    }

    /// Whether the statutory power of sale is exercisable: the mortgage must be a
    /// registered charge and the mortgagor must be in default.
    pub fn power_of_sale_exercisable(&self) -> bool {
        self.registered && self.in_default
    }
}

// ============================================================================
// Easements (Re Ellenborough Park)
// ============================================================================

/// The kind of easement (a non-exhaustive set of common easements).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementKind {
    /// A right of way over the servient land.
    RightOfWay,
    /// A right of drainage / to run services through the servient land.
    RightOfDrainage,
    /// A right of support for a building.
    RightOfSupport,
    /// A right to light through a defined aperture.
    RightOfLight,
    /// A right to park or some other recognised easement.
    Other,
}

impl EasementKind {
    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            EasementKind::RightOfWay => "Right of way",
            EasementKind::RightOfDrainage => "Right of drainage / services",
            EasementKind::RightOfSupport => "Right of support",
            EasementKind::RightOfLight => "Right of light",
            EasementKind::Other => "Other recognised easement",
        }
    }
}

/// An easement: a right enjoyed by the owner of one parcel of land (the dominant
/// tenement) over another parcel (the servient tenement).
///
/// To be a valid easement the four characteristics in *Re Ellenborough Park*
/// \[1956\] Ch 131 must be satisfied: (1) there must be a dominant and a servient
/// tenement; (2) the easement must accommodate the dominant tenement; (3) the
/// dominant and servient tenements must not be owned and occupied by the same
/// person; and (4) the right must be capable of forming the subject matter of a
/// grant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Easement {
    /// The dominant tenement (benefited land).
    pub dominant_tenement: String,
    /// The servient tenement (burdened land).
    pub servient_tenement: String,
    /// Kind of easement.
    pub kind: EasementKind,
    /// Whether the easement accommodates (benefits) the dominant tenement.
    pub accommodates_dominant: bool,
    /// Whether the dominant and servient tenements are separately owned/occupied
    /// (diversity of ownership).
    pub diversity_of_ownership: bool,
    /// Whether the right is capable of forming the subject matter of a grant
    /// (sufficiently definite, not mere recreation, no new negative burden).
    pub capable_of_grant: bool,
    /// Whether the easement is registered (LTA).
    pub registered: bool,
}

impl Easement {
    /// Creates a new easement satisfying the *Re Ellenborough Park*
    /// characteristics, registered by default.
    pub fn new(
        dominant_tenement: impl Into<String>,
        servient_tenement: impl Into<String>,
        kind: EasementKind,
    ) -> Self {
        Self {
            dominant_tenement: dominant_tenement.into(),
            servient_tenement: servient_tenement.into(),
            kind,
            accommodates_dominant: true,
            diversity_of_ownership: true,
            capable_of_grant: true,
            registered: true,
        }
    }

    /// Marks the easement as not accommodating the dominant tenement.
    pub fn not_accommodating(mut self) -> Self {
        self.accommodates_dominant = false;
        self
    }

    /// Marks the dominant and servient tenements as held by the same person.
    pub fn same_owner(mut self) -> Self {
        self.diversity_of_ownership = false;
        self
    }

    /// Whether the four *Re Ellenborough Park* characteristics are satisfied.
    ///
    /// A dominant and servient tenement are present by construction (both are
    /// named); the remaining three characteristics are checked.
    pub fn satisfies_ellenborough_park(&self) -> bool {
        !self.dominant_tenement.is_empty()
            && !self.servient_tenement.is_empty()
            && self.accommodates_dominant
            && self.diversity_of_ownership
            && self.capable_of_grant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proprietor() -> RegisteredProprietor {
        RegisteredProprietor::new("Tan Ah Kow").with_identifier("S1234567A")
    }

    #[test]
    fn test_tenure_and_property_type() {
        assert!(Tenure::Freehold.is_freehold());
        assert!(!Tenure::Leasehold99.is_freehold());
        assert!(PropertyType::Residential.is_residential());
        assert!(!PropertyType::Commercial.is_residential());
    }

    #[test]
    fn test_land_title_registered_by_default() {
        let title = LandTitle::new(
            "Lot 1234A MK 5",
            "Condominium unit #12-34",
            Tenure::Leasehold99,
            PropertyType::Residential,
            proprietor(),
        );
        assert!(title.registered);
        assert!(!title.unregistered().registered);
    }

    #[test]
    fn test_indefeasibility_exception_classification() {
        assert!(IndefeasibilityException::Fraud.defeats_title());
        assert!(IndefeasibilityException::Forgery.defeats_title());
        assert!(IndefeasibilityException::InPersonam.defeats_title());
        assert!(!IndefeasibilityException::OverridingShortLease.defeats_title());
        assert!(!IndefeasibilityException::PriorRegisteredInterest.defeats_title());
    }

    #[test]
    fn test_fraud_challenge_requires_proprietor_party() {
        let fraud = IndefeasibilityChallenge::fraud("forged transfer registered by the proprietor");
        assert!(fraud.defeats_title());

        // A bona fide purchaser for value who was not party/privy takes free
        // (immediate indefeasibility, UOB v Bebe).
        let innocent = fraud.proprietor_innocent();
        assert!(!innocent.defeats_title());
    }

    #[test]
    fn test_in_personam_binds_regardless() {
        let challenge = IndefeasibilityChallenge::new(
            IndefeasibilityException::InPersonam,
            "proprietor bound by a prior contract to convey",
        );
        assert!(challenge.defeats_title());
    }

    #[test]
    fn test_overriding_interest_does_not_defeat_title() {
        let challenge = IndefeasibilityChallenge::new(
            IndefeasibilityException::OverridingShortLease,
            "tenant in occupation under a 3-year lease",
        );
        assert!(!challenge.defeats_title());
    }

    #[test]
    fn test_caveat_prohibits_registration() {
        let caveat = Caveat::new("Purchaser Pte Ltd", "Lot 1234A MK 5", "purchaser under OTP");
        assert!(caveat.prohibits_registration());

        let no_interest = caveat.without_caveatable_interest();
        assert!(!no_interest.prohibits_registration());
    }

    #[test]
    fn test_mortgage_power_of_sale() {
        let mortgage = Mortgage::new("Borrower", "DBS Bank Ltd", "Lot 1234A MK 5", 50_000_000);
        // Registered but not in default - no power of sale.
        assert!(!mortgage.power_of_sale_exercisable());

        let defaulted = mortgage.in_default();
        assert!(defaulted.power_of_sale_exercisable());

        // Unregistered charge in default - still no statutory power of sale.
        let unregistered = Mortgage::new("Borrower", "Lender", "Lot 9", 10_000_000)
            .unregistered()
            .in_default();
        assert!(!unregistered.power_of_sale_exercisable());
    }

    #[test]
    fn test_easement_ellenborough_park() {
        let valid = Easement::new(
            "Lot 1 (dominant)",
            "Lot 2 (servient)",
            EasementKind::RightOfWay,
        );
        assert!(valid.satisfies_ellenborough_park());

        // No accommodation of the dominant tenement.
        assert!(
            !valid
                .clone()
                .not_accommodating()
                .satisfies_ellenborough_park()
        );
        // Unity of ownership and occupation.
        assert!(!valid.same_owner().satisfies_ellenborough_park());
    }

    #[test]
    fn test_types_serde_roundtrip() {
        let title = LandTitle::new(
            "Lot 555X MK 1",
            "Landed house",
            Tenure::Freehold,
            PropertyType::Residential,
            proprietor(),
        );
        let json = serde_json::to_string(&title).expect("serialize");
        let back: LandTitle = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(title, back);
    }
}
