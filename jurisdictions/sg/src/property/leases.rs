//! Property Law - Leases
//!
//! Models leasehold interests in registered land under the Land Titles Act 1993,
//! supplemented by the Conveyancing and Law of Property Act 1886 (CLPA) and the
//! common law:
//!
//! - **Creation and registration.** A lease for a term **exceeding 7 years**
//!   must be registered to create a legal leasehold estate (LTA s. 45). A lease
//!   **not exceeding 7 years** under which the tenant is in occupation binds the
//!   registered proprietor as an overriding interest even if unregistered
//!   (LTA s. 46(1)). An unregistered lease over 7 years takes effect, if at all,
//!   only in equity (an agreement for a lease; *Walsh v Lonsdale* (1882) 21 Ch D
//!   9).
//! - **Covenants.** Express and implied covenants of landlord and tenant,
//!   including the covenant for quiet enjoyment and the obligation to pay rent.
//! - **Determination.** By effluxion of time, surrender, merger, notice to quit
//!   (periodic tenancies) and **forfeiture** (re-entry for breach), the latter
//!   subject to the notice and relief regime in CLPA s. 18.
//!
//! Monetary values are stored as **SGD cents** (`u64`).

use serde::{Deserialize, Serialize};

/// The threshold term (in years) above which a lease of registered land must be
/// registered to create a legal leasehold estate, and at or below which a lease
/// under which the tenant is in occupation is an overriding interest
/// (Land Titles Act s. 46(1)).
pub const LEASE_REGISTRATION_THRESHOLD_YEARS: u32 = 7;

// ============================================================================
// Lease
// ============================================================================

/// A lease of registered land.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lease {
    /// The lessor (landlord).
    pub lessor: String,
    /// The lessee (tenant).
    pub lessee: String,
    /// Description / reference of the demised premises.
    pub premises: String,
    /// Term of the lease, in years.
    pub term_years: u32,
    /// Rent reserved, in SGD cents per month.
    pub rent_cents_per_month: u64,
    /// Whether the lease is registered (LTA s. 45).
    pub registered: bool,
    /// Whether the tenant is in actual occupation (relevant to the s. 46(1)
    /// overriding-interest exception for short leases).
    pub tenant_in_occupation: bool,
    /// Whether the lease contains an option to purchase (which is excluded from
    /// the short-lease overriding-interest protection).
    pub contains_option_to_purchase: bool,
}

impl Lease {
    /// Creates a new registered lease (tenant in occupation, no option to
    /// purchase by default).
    pub fn new(
        lessor: impl Into<String>,
        lessee: impl Into<String>,
        premises: impl Into<String>,
        term_years: u32,
        rent_cents_per_month: u64,
    ) -> Self {
        Self {
            lessor: lessor.into(),
            lessee: lessee.into(),
            premises: premises.into(),
            term_years,
            rent_cents_per_month,
            registered: true,
            tenant_in_occupation: true,
            contains_option_to_purchase: false,
        }
    }

    /// Marks the lease as unregistered.
    pub fn unregistered(mut self) -> Self {
        self.registered = false;
        self
    }

    /// Marks that the tenant is not in occupation.
    pub fn not_in_occupation(mut self) -> Self {
        self.tenant_in_occupation = false;
        self
    }

    /// Records that the lease contains an option to purchase.
    pub fn with_option_to_purchase(mut self) -> Self {
        self.contains_option_to_purchase = true;
        self
    }

    /// Whether the lease must be registered to create a legal leasehold estate
    /// (term exceeding 7 years).
    pub fn must_be_registered(&self) -> bool {
        self.term_years > LEASE_REGISTRATION_THRESHOLD_YEARS
    }

    /// Whether the lease takes effect as an **overriding interest** binding the
    /// registered proprietor without registration (LTA s. 46(1)): a term not
    /// exceeding 7 years, the tenant in occupation, and no option to purchase.
    pub fn is_overriding_interest(&self) -> bool {
        !self.must_be_registered() && self.tenant_in_occupation && !self.contains_option_to_purchase
    }

    /// Whether the lease creates a legal leasehold estate.
    ///
    /// A lease over 7 years does so only when registered; a short lease does so
    /// where it binds as an overriding interest.
    pub fn creates_legal_estate(&self) -> bool {
        if self.must_be_registered() {
            self.registered
        } else {
            self.registered || self.is_overriding_interest()
        }
    }
}

// ============================================================================
// Covenants
// ============================================================================

/// The party bound by a lease covenant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CovenantParty {
    /// A covenant binding the lessor (landlord).
    Lessor,
    /// A covenant binding the lessee (tenant).
    Lessee,
}

/// A covenant in a lease.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseCovenant {
    /// Lessee's covenant to pay rent.
    PayRent,
    /// Lessor's covenant for quiet enjoyment.
    QuietEnjoyment,
    /// Lessee's covenant to keep the premises in repair.
    Repair,
    /// Lessee's covenant not to assign or sublet without consent.
    NotToAssignWithoutConsent,
    /// Lessee's covenant not to commit waste.
    NotToCommitWaste,
    /// Lessee's covenant to insure the premises.
    Insure,
    /// Lessee's covenant to use the premises only for a permitted use.
    PermittedUseOnly,
}

impl LeaseCovenant {
    /// Returns the party bound by the covenant.
    pub fn party(&self) -> CovenantParty {
        match self {
            LeaseCovenant::QuietEnjoyment => CovenantParty::Lessor,
            _ => CovenantParty::Lessee,
        }
    }

    /// Whether the covenant is implied by law in the absence of express
    /// agreement.
    ///
    /// The lessor's covenant for quiet enjoyment and the lessee's obligation to
    /// pay the reserved rent and not to commit waste are implied; the others are
    /// typically express.
    pub fn implied_by_default(&self) -> bool {
        matches!(
            self,
            LeaseCovenant::QuietEnjoyment
                | LeaseCovenant::PayRent
                | LeaseCovenant::NotToCommitWaste
        )
    }

    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            LeaseCovenant::PayRent => "Lessee to pay the reserved rent",
            LeaseCovenant::QuietEnjoyment => "Lessor's covenant for quiet enjoyment",
            LeaseCovenant::Repair => "Lessee to keep the premises in repair",
            LeaseCovenant::NotToAssignWithoutConsent => {
                "Lessee not to assign or sublet without consent"
            }
            LeaseCovenant::NotToCommitWaste => "Lessee not to commit waste",
            LeaseCovenant::Insure => "Lessee to insure the premises",
            LeaseCovenant::PermittedUseOnly => {
                "Lessee to use the premises for the permitted use only"
            }
        }
    }
}

// ============================================================================
// Determination
// ============================================================================

/// The manner in which a lease is, or is said to be, determined (brought to an
/// end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseDetermination {
    /// By effluxion of time - the term expires.
    EffluxionOfTime,
    /// By surrender - the lessee yields the term to the lessor.
    Surrender,
    /// By merger - the term and the reversion vest in the same person.
    Merger,
    /// By notice to quit - terminating a periodic tenancy.
    NoticeToQuit,
    /// By forfeiture - the lessor re-enters for breach of covenant.
    Forfeiture,
}

impl LeaseDetermination {
    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            LeaseDetermination::EffluxionOfTime => "Determination by effluxion of time",
            LeaseDetermination::Surrender => "Determination by surrender",
            LeaseDetermination::Merger => "Determination by merger of term and reversion",
            LeaseDetermination::NoticeToQuit => {
                "Determination of a periodic tenancy by notice to quit"
            }
            LeaseDetermination::Forfeiture => "Determination by forfeiture (re-entry for breach)",
        }
    }

    /// Returns the controlling statute/authority, where one applies.
    pub fn statute_reference(&self) -> Option<&'static str> {
        match self {
            LeaseDetermination::Forfeiture => Some("Conveyancing and Law of Property Act s. 18"),
            _ => None,
        }
    }
}

/// A claim by the lessor to forfeit the lease (re-enter for breach of covenant).
///
/// Forfeiture requires an express right of re-entry. For a breach **other than**
/// non-payment of rent, the lessor must first serve a notice under CLPA s. 18
/// specifying the breach, requiring it to be remedied (if capable of remedy) and
/// requiring compensation; the lessee may seek relief against forfeiture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForfeitureClaim {
    /// Description of the breach relied on.
    pub breach: String,
    /// Whether the breach is non-payment of rent (for which the CLPA s. 18 notice
    /// is not required, though a formal demand may be).
    pub breach_is_non_payment_of_rent: bool,
    /// Whether the lease contains an express right of re-entry / forfeiture.
    pub right_of_re_entry: bool,
    /// Whether a CLPA s. 18 notice has been served (required for a non-rent
    /// breach).
    pub statutory_notice_served: bool,
}

impl ForfeitureClaim {
    /// Creates a forfeiture claim for a non-rent breach (with a re-entry clause
    /// and a s. 18 notice served by default).
    pub fn for_breach(breach: impl Into<String>) -> Self {
        Self {
            breach: breach.into(),
            breach_is_non_payment_of_rent: false,
            right_of_re_entry: true,
            statutory_notice_served: true,
        }
    }

    /// Creates a forfeiture claim for non-payment of rent.
    pub fn for_rent_arrears() -> Self {
        Self {
            breach: "non-payment of rent".to_string(),
            breach_is_non_payment_of_rent: true,
            right_of_re_entry: true,
            statutory_notice_served: false,
        }
    }

    /// Records that the lease contains no express right of re-entry.
    pub fn without_re_entry_clause(mut self) -> Self {
        self.right_of_re_entry = false;
        self
    }

    /// Records that no CLPA s. 18 notice was served.
    pub fn without_statutory_notice(mut self) -> Self {
        self.statutory_notice_served = false;
        self
    }

    /// Whether forfeiture is lawfully available.
    ///
    /// There must be an express right of re-entry; and, for a breach other than
    /// non-payment of rent, a CLPA s. 18 notice must have been served.
    pub fn is_available(&self) -> bool {
        if !self.right_of_re_entry {
            return false;
        }
        self.breach_is_non_payment_of_rent || self.statutory_notice_served
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_lease_overriding_interest() {
        let lease = Lease::new("Landlord", "Tenant", "Shop unit", 3, 500_000);
        assert!(!lease.must_be_registered());
        assert!(lease.is_overriding_interest());
        assert!(lease.creates_legal_estate());
    }

    #[test]
    fn test_short_lease_with_option_loses_override() {
        let lease = Lease::new("Landlord", "Tenant", "Shop unit", 5, 500_000)
            .unregistered()
            .with_option_to_purchase();
        // The option to purchase is excluded from the s. 46(1) protection.
        assert!(!lease.is_overriding_interest());
    }

    #[test]
    fn test_long_lease_must_be_registered() {
        let lease = Lease::new("Landlord", "Tenant", "Office floor", 30, 5_000_000);
        assert!(lease.must_be_registered());
        assert!(lease.creates_legal_estate()); // registered by default

        let unregistered = lease.unregistered();
        assert!(!unregistered.creates_legal_estate());
    }

    #[test]
    fn test_boundary_seven_years_does_not_require_registration() {
        let lease = Lease::new("Landlord", "Tenant", "Unit", 7, 500_000);
        assert!(!lease.must_be_registered());
        let eight = Lease::new("Landlord", "Tenant", "Unit", 8, 500_000);
        assert!(eight.must_be_registered());
    }

    #[test]
    fn test_covenant_classification() {
        assert_eq!(LeaseCovenant::QuietEnjoyment.party(), CovenantParty::Lessor);
        assert_eq!(LeaseCovenant::PayRent.party(), CovenantParty::Lessee);
        assert!(LeaseCovenant::QuietEnjoyment.implied_by_default());
        assert!(LeaseCovenant::PayRent.implied_by_default());
        assert!(!LeaseCovenant::Insure.implied_by_default());
    }

    #[test]
    fn test_determination_authority() {
        assert_eq!(
            LeaseDetermination::Forfeiture.statute_reference(),
            Some("Conveyancing and Law of Property Act s. 18")
        );
        assert_eq!(LeaseDetermination::Surrender.statute_reference(), None);
    }

    #[test]
    fn test_forfeiture_requires_re_entry_and_notice() {
        let claim = ForfeitureClaim::for_breach("alterations without consent");
        assert!(claim.is_available());

        // No re-entry clause -> no forfeiture.
        assert!(!claim.clone().without_re_entry_clause().is_available());
        // Non-rent breach with no s. 18 notice -> not available.
        assert!(!claim.without_statutory_notice().is_available());
    }

    #[test]
    fn test_forfeiture_for_rent_arrears_needs_no_s18_notice() {
        let claim = ForfeitureClaim::for_rent_arrears();
        assert!(claim.is_available());
    }

    #[test]
    fn test_lease_serde_roundtrip() {
        let lease = Lease::new(
            "Landlord Pte Ltd",
            "Tenant Pte Ltd",
            "Warehouse",
            10,
            8_000_000,
        )
        .with_option_to_purchase();
        let json = serde_json::to_string(&lease).expect("serialize");
        let back: Lease = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(lease, back);
    }
}
