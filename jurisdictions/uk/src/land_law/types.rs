//! UK Land Law - Core Types
//!
//! This module provides core types for UK land law (England & Wales), covering:
//! - Estates in land (freehold, leasehold)
//! - Interests in land (easements, covenants, mortgages)
//! - Registration (LRA 2002)
//! - Conveyancing
//!
//! Key statutes:
//! - Law of Property Act 1925 (LPA 1925)
//! - Land Registration Act 2002 (LRA 2002)
//! - Landlord and Tenant Act 1954 (LTA 1954)
//! - Trusts of Land and Appointment of Trustees Act 1996 (TOLATA)

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

// ============================================================================
// Estates in Land
// ============================================================================

/// Type of estate in land (s.1 LPA 1925)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstateType {
    /// Fee simple absolute in possession (freehold)
    FeeSimple,
    /// Term of years absolute (leasehold)
    TermOfYears {
        /// Duration in years (or fraction)
        duration: LeaseDuration,
        /// Whether business tenancy (LTA 1954 protection)
        business_tenancy: bool,
    },
}

/// Duration of a lease
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseDuration {
    /// Fixed term (years, months, weeks)
    Fixed { years: u32, months: u32, weeks: u32 },
    /// Periodic tenancy
    Periodic(PeriodicTenancy),
    /// Tenancy at will
    AtWill,
    /// Tenancy at sufferance
    AtSufferance,
}

/// Type of periodic tenancy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeriodicTenancy {
    /// Weekly
    Weekly,
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Yearly
    Yearly,
}

/// Freehold estate details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeholdEstate {
    /// Title number (if registered)
    pub title_number: Option<String>,
    /// Property address
    pub address: PropertyAddress,
    /// Class of title
    pub title_class: Option<TitleClass>,
    /// Whether registered
    pub registered: bool,
    /// Current owner(s)
    pub owners: Vec<Owner>,
    /// Type of co-ownership (if multiple owners)
    pub co_ownership: Option<CoOwnershipType>,
}

/// Leasehold estate details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseholdEstate {
    /// Title number (if registered)
    pub title_number: Option<String>,
    /// Property address
    pub address: PropertyAddress,
    /// Lease duration
    pub duration: LeaseDuration,
    /// Lease start date (YYYY-MM-DD)
    pub start_date: String,
    /// Ground rent (pence per year)
    pub ground_rent_pence: Option<u64>,
    /// Whether business tenancy
    pub business_tenancy: bool,
    /// LTA 1954 protection status
    pub lta_1954_protected: bool,
    /// Landlord details
    pub landlord: Option<String>,
    /// Whether contracted out of LTA 1954
    pub contracted_out: bool,
}

/// Property address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyAddress {
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub postcode: String,
}

/// Class of title (LRA 2002)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleClass {
    /// Absolute title (best class)
    Absolute,
    /// Good leasehold title
    GoodLeasehold,
    /// Qualified title
    Qualified,
    /// Possessory title
    Possessory,
}

/// Owner of property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub name: String,
    pub owner_type: OwnerType,
}

/// Type of property owner
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnerType {
    /// Individual person
    Individual,
    /// Company
    Company { company_number: String },
    /// Trustee
    Trustee,
    /// Personal representative
    PersonalRepresentative,
}

/// Type of co-ownership
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoOwnershipType {
    /// Joint tenancy (right of survivorship)
    JointTenancy,
    /// Tenancy in common (shares pass to estate)
    TenancyInCommon,
}

// ============================================================================
// Interests in Land
// ============================================================================

/// Interest in land (s.1 LPA 1925)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterestType {
    /// Easement (right over another's land)
    Easement(EasementType),
    /// Restrictive covenant
    RestrictiveCovenant,
    /// Rent charge
    Rentcharge,
    /// Charge by way of legal mortgage
    Mortgage,
    /// Estate contract
    EstateContract,
    /// Option to purchase
    Option,
    /// Right of pre-emption
    RightOfPreEmption,
    /// Beneficial interest under trust
    BeneficialInterest,
    /// Home rights (FLA 1996)
    HomeRights,
}

/// Type of easement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementType {
    /// Right of way
    RightOfWay { foot: bool, vehicle: bool },
    /// Right of light
    RightOfLight,
    /// Right of support
    RightOfSupport,
    /// Right of drainage
    RightOfDrainage,
    /// Right to run services (pipes, cables)
    Services { description: String },
    /// Parking right
    Parking,
    /// Right to storage
    Storage,
    /// Other easement
    Other { description: String },
}

/// Easement details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Easement {
    /// Type of easement
    pub easement_type: EasementType,
    /// Dominant tenement (benefited land)
    pub dominant_tenement: String,
    /// Servient tenement (burdened land)
    pub servient_tenement: String,
    /// How created
    pub creation_method: EasementCreation,
    /// Whether legal or equitable
    pub legal: bool,
    /// Route/extent description
    pub route: Option<String>,
}

/// How easement was created
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementCreation {
    /// Express grant in deed
    ExpressGrant,
    /// Express reservation
    ExpressReservation,
    /// Implied by necessity
    ImpliedNecessity,
    /// Implied by common intention
    ImpliedCommonIntention,
    /// Implied under Wheeldon v Burrows
    WheeldonVBurrows,
    /// Implied under s.62 LPA 1925
    Section62Lpa,
    /// Prescription at common law (20 years)
    PrescriptionCommonLaw,
    /// Prescription under Prescription Act 1832
    PrescriptionAct1832,
    /// Lost modern grant
    LostModernGrant,
}

/// Covenant details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Covenant {
    /// Covenant wording
    pub wording: String,
    /// Whether positive or restrictive
    pub covenant_nature: CovenantNature,
    /// Benefited land
    pub benefited_land: Option<String>,
    /// Burdened land
    pub burdened_land: String,
    /// Whether burden runs with land
    pub burden_runs: bool,
    /// Whether benefit runs with land
    pub benefit_runs: bool,
    /// Building scheme exists
    pub building_scheme: bool,
}

/// Nature of covenant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CovenantNature {
    /// Positive covenant (requires expenditure)
    Positive,
    /// Restrictive covenant (prevents action)
    Restrictive,
    /// Mixed covenant
    Mixed,
}

/// Mortgage details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mortgage {
    /// Lender name
    pub lender: String,
    /// Property charged
    pub property: String,
    /// Amount secured (pence)
    pub amount_pence: u64,
    /// Whether legal charge
    pub legal_charge: bool,
    /// Priority (1 = first charge)
    pub priority: u32,
    /// Whether registered
    pub registered: bool,
}

/// Mortgage remedies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MortgageRemedy {
    /// Action for debt
    ActionForDebt,
    /// Possession
    Possession,
    /// Power of sale
    PowerOfSale,
    /// Appointment of receiver
    AppointmentOfReceiver,
    /// Foreclosure
    Foreclosure,
}

// ============================================================================
// Registration (LRA 2002)
// ============================================================================

/// Registration status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Registered land
    Registered { title_number: String },
    /// Unregistered land
    Unregistered,
    /// First registration pending
    FirstRegistrationPending,
}

/// Trigger for first registration (LRA 2002 s.4)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FirstRegistrationTrigger {
    /// Transfer of freehold
    TransferOfFreehold,
    /// Grant of lease over 7 years
    GrantOfLeaseOver7Years,
    /// Assignment of lease with more than 7 years to run
    AssignmentOfLeaseOver7Years,
    /// First legal mortgage
    FirstLegalMortgage,
    /// Protected first legal mortgage of leasehold
    ProtectedMortgageOfLeasehold,
}

/// Register entry type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisterEntry {
    /// Property register (description)
    PropertyRegister,
    /// Proprietorship register (owners)
    ProprietorshipRegister,
    /// Charges register (burdens)
    ChargesRegister,
}

/// Overriding interest (Sch 3 LRA 2002)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverridingInterest {
    /// Short lease (≤7 years)
    ShortLease,
    /// Interest of person in actual occupation
    ActualOccupation,
    /// Legal easement
    LegalEasement,
    /// Local land charge
    LocalLandCharge,
    /// Mines and minerals
    MinesAndMinerals,
    /// Chancel repair liability
    ChancelRepair,
}

/// Notice or restriction on register
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisterProtection {
    /// Notice (protects priority)
    Notice { agreed: bool },
    /// Restriction (controls dealings)
    Restriction { restriction_type: RestrictionType },
}

/// Type of restriction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestrictionType {
    /// Form A restriction (consents required)
    FormA,
    /// Form B restriction (certificate required)
    FormB,
    /// Trust restriction (two trustees or trust corp)
    TrustRestriction,
    /// Charging order restriction
    ChargingOrder,
    /// Bankruptcy restriction
    Bankruptcy,
    /// Other restriction
    Other { description: String },
}

// ============================================================================
// Unregistered Land
// ============================================================================

/// Land charge class (Land Charges Act 1972)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandChargeClass {
    /// Class A - rentcharges by statute
    ClassA,
    /// Class B - charges by statute
    ClassB,
    /// Class C(i) - puisne mortgage
    ClassCi,
    /// Class C(ii) - limited owner's charge
    ClassCii,
    /// Class C(iii) - general equitable charge
    ClassCiii,
    /// Class C(iv) - estate contract
    ClassCiv,
    /// Class D(i) - Inland Revenue charge
    ClassDi,
    /// Class D(ii) - restrictive covenant
    ClassDii,
    /// Class D(iii) - equitable easement
    ClassDiii,
    /// Class E - annuities
    ClassE,
    /// Class F - home rights
    ClassF,
}

// ============================================================================
// Trusts of Land (TOLATA)
// ============================================================================

/// Trust of land type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustOfLandType {
    /// Express trust of land
    Express,
    /// Resulting trust
    Resulting,
    /// Constructive trust (common intention)
    Constructive,
    /// Statutory trust on co-ownership
    StatutoryCoOwnership,
}

/// TOLATA s.14 application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TolataClaim {
    /// Property subject to trust
    pub property: String,
    /// Applicant
    pub applicant: String,
    /// Order sought
    pub order_sought: TolataOrder,
    /// Section 15 factors considered
    pub section_15_factors: Vec<Section15Factor>,
}

/// Order sought under TOLATA s.14
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TolataOrder {
    /// Order for sale
    Sale,
    /// Postponement of sale
    PostponementOfSale,
    /// Declaration of beneficial interests
    DeclarationOfInterests,
    /// Order regulating occupation
    RegulateOccupation,
    /// Exclusion from occupation
    ExcludeFromOccupation,
    /// Payment for occupation
    OccupationRent,
}

/// Section 15 factors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Section15Factor {
    /// Intentions of settlor
    IntentionsOfSettlor,
    /// Purposes for which property held
    PurposesOfTrust,
    /// Welfare of minors in occupation
    WelfareOfMinors,
    /// Interests of secured creditors
    SecuredCreditors,
    /// Circumstances and wishes of beneficiaries
    BeneficiariesWishes,
}

// ============================================================================
// Conveyancing
// ============================================================================

/// Conveyancing stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConveyancingStage {
    /// Pre-contract (searches, enquiries)
    PreContract,
    /// Exchange of contracts
    Exchange,
    /// Between exchange and completion
    PostExchange,
    /// Completion
    Completion,
    /// Post-completion (registration)
    PostCompletion,
}

/// Pre-contract search
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConveyancingSearch {
    /// Local authority search
    LocalAuthority,
    /// Environmental search
    Environmental,
    /// Chancel repair search
    ChancelRepair,
    /// Water and drainage search
    WaterAndDrainage,
    /// Mining search (coal, tin, brine)
    Mining { search_type: String },
    /// Land Registry search
    LandRegistry,
    /// Bankruptcy search (K16)
    Bankruptcy,
    /// Companies House search
    CompaniesHouse,
}

/// Contract for sale of land
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandContract {
    /// Seller
    pub seller: String,
    /// Buyer
    pub buyer: String,
    /// Property
    pub property: PropertyAddress,
    /// Purchase price (pence)
    pub price_pence: u64,
    /// Deposit amount (pence)
    pub deposit_pence: u64,
    /// Completion date
    pub completion_date: String,
    /// Title guarantee
    pub title_guarantee: TitleGuarantee,
    /// Special conditions
    pub special_conditions: Vec<String>,
}

/// Title guarantee (LP(MP)A 1994)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleGuarantee {
    /// Full title guarantee
    Full,
    /// Limited title guarantee
    Limited,
    /// No title guarantee
    None,
}

/// Transfer deed type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferDeed {
    /// TR1 - registered land transfer
    Tr1,
    /// TP1 - transfer of part
    Tp1,
    /// AS1 - assent (registered)
    As1,
    /// AS3 - assent of whole
    As3,
    /// Conveyance (unregistered)
    Conveyance,
}

// ============================================================================
// Key Case Citations
// ============================================================================

/// Land law case citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LandLawCase {
    pub name: String,
    pub citation: String,
    pub year: u16,
    pub principle: String,
}

impl LandLawCase {
    /// Street v Mountford - lease vs licence distinction
    pub fn street_v_mountford() -> Self {
        Self {
            name: "Street v Mountford".into(),
            citation: "[1985] AC 809".into(),
            year: 1985,
            principle: "Exclusive possession for a term at a rent creates a lease, \
                regardless of label. Substance over form."
                .into(),
        }
    }

    /// Re Ellenborough Park - easement requirements
    pub fn re_ellenborough_park() -> Self {
        Self {
            name: "Re Ellenborough Park".into(),
            citation: "[1956] Ch 131".into(),
            year: 1956,
            principle: "Four requirements for easement: dominant and servient tenement, \
                different owners, accommodation of dominant tenement, capable of grant."
                .into(),
        }
    }

    /// Tulk v Moxhay - restrictive covenant equity
    pub fn tulk_v_moxhay() -> Self {
        Self {
            name: "Tulk v Moxhay".into(),
            citation: "(1848) 2 Ph 774".into(),
            year: 1848,
            principle: "Restrictive covenant binds successor in title who takes with \
                notice, based on equity's intervention."
                .into(),
        }
    }

    /// Williams & Glyn's Bank v Boland - actual occupation
    pub fn williams_and_glyns_v_boland() -> Self {
        Self {
            name: "Williams & Glyn's Bank Ltd v Boland".into(),
            citation: "[1981] AC 487".into(),
            year: 1981,
            principle: "Beneficial interest coupled with actual occupation constitutes \
                overriding interest binding purchaser."
                .into(),
        }
    }

    /// City of London BS v Flegg - overreaching
    pub fn flegg() -> Self {
        Self {
            name: "City of London Building Society v Flegg".into(),
            citation: "[1988] AC 54".into(),
            year: 1988,
            principle: "Beneficial interests under trust of land are overreached when \
                purchase money paid to two trustees. Interest transfers to proceeds."
                .into(),
        }
    }

    /// Lloyds Bank v Rosset - constructive trust
    pub fn lloyds_bank_v_rosset() -> Self {
        Self {
            name: "Lloyds Bank plc v Rosset".into(),
            citation: "[1991] 1 AC 107".into(),
            year: 1991,
            principle: "Common intention constructive trust requires either express \
                agreement plus detriment, or direct financial contribution."
                .into(),
        }
    }

    /// Stack v Dowden - quantification of shares
    pub fn stack_v_dowden() -> Self {
        Self {
            name: "Stack v Dowden".into(),
            citation: "[2007] UKHL 17".into(),
            year: 2007,
            principle: "Joint legal owners presumed joint beneficial owners. \
                Strong evidence needed to rebut. Wide range of factors considered."
                .into(),
        }
    }

    /// Jones v Kernott - imputed intention
    pub fn jones_v_kernott() -> Self {
        Self {
            name: "Jones v Kernott".into(),
            citation: "[2011] UKSC 53".into(),
            year: 2011,
            principle: "Court can impute intention as to shares where common intention \
                cannot be inferred. Fair shares based on whole course of dealing."
                .into(),
        }
    }

    /// Wheeldon v Burrows - implied easements
    pub fn wheeldon_v_burrows() -> Self {
        Self {
            name: "Wheeldon v Burrows".into(),
            citation: "(1879) 12 Ch D 31".into(),
            year: 1879,
            principle: "On sale of part, easements implied for quasi-easements that are \
                continuous, apparent, necessary for reasonable enjoyment, and used at time of grant."
                .into(),
        }
    }

    /// Roake v Chadha - building schemes
    pub fn roake_v_chadha() -> Self {
        Self {
            name: "Roake v Chadha".into(),
            citation: "[1984] 1 WLR 40".into(),
            year: 1984,
            principle: "Express annexation to 'each and every part' of land enables \
                benefit to pass with part sold. Elliston v Reacher requirements for building scheme."
                .into(),
        }
    }
}

// ============================================================================
// Analysis Results
// ============================================================================

/// Result of estate analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstateAnalysisResult {
    pub estate_type: EstateType,
    pub valid: bool,
    pub registration_required: bool,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Result of easement analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasementAnalysisResult {
    pub meets_requirements: bool,
    pub re_ellenborough_satisfied: bool,
    pub dominant_accommodated: bool,
    pub capable_of_grant: bool,
    pub creation_valid: bool,
    pub legal_or_equitable: String,
    pub issues: Vec<String>,
}

/// Result of covenant analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovenantAnalysisResult {
    pub burden_runs: bool,
    pub benefit_runs: bool,
    pub enforceable_at_law: bool,
    pub enforceable_in_equity: bool,
    pub tulk_v_moxhay_satisfied: bool,
    pub building_scheme_exists: bool,
    pub issues: Vec<String>,
}

/// Result of registration analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationAnalysisResult {
    pub registration_required: bool,
    pub trigger: Option<FirstRegistrationTrigger>,
    pub deadline_days: u32,
    pub consequences_of_failure: Vec<String>,
    pub overriding_interests: Vec<OverridingInterest>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estate_type() {
        let freehold = EstateType::FeeSimple;
        assert_eq!(freehold, EstateType::FeeSimple);

        let lease = EstateType::TermOfYears {
            duration: LeaseDuration::Fixed {
                years: 99,
                months: 0,
                weeks: 0,
            },
            business_tenancy: false,
        };
        assert!(matches!(lease, EstateType::TermOfYears { .. }));
    }

    #[test]
    fn test_easement_creation() {
        let easement = Easement {
            easement_type: EasementType::RightOfWay {
                foot: true,
                vehicle: true,
            },
            dominant_tenement: "Plot A".into(),
            servient_tenement: "Plot B".into(),
            creation_method: EasementCreation::ExpressGrant,
            legal: true,
            route: Some("Over the track shown brown on plan".into()),
        };
        assert!(easement.legal);
    }

    #[test]
    fn test_title_class() {
        let absolute = TitleClass::Absolute;
        assert_eq!(absolute, TitleClass::Absolute);

        let possessory = TitleClass::Possessory;
        assert_eq!(possessory, TitleClass::Possessory);
    }

    #[test]
    fn test_covenant_nature() {
        let restrictive = CovenantNature::Restrictive;
        assert_eq!(restrictive, CovenantNature::Restrictive);
    }

    #[test]
    fn test_overriding_interest() {
        let occupation = OverridingInterest::ActualOccupation;
        assert_eq!(occupation, OverridingInterest::ActualOccupation);
    }

    #[test]
    fn test_land_charge_class() {
        let estate_contract = LandChargeClass::ClassCiv;
        assert_eq!(estate_contract, LandChargeClass::ClassCiv);
    }

    #[test]
    fn test_tolata_order() {
        let sale = TolataOrder::Sale;
        assert_eq!(sale, TolataOrder::Sale);
    }

    #[test]
    fn test_key_cases() {
        let street = LandLawCase::street_v_mountford();
        assert!(street.principle.contains("Exclusive possession"));
        assert_eq!(street.year, 1985);

        let ellenborough = LandLawCase::re_ellenborough_park();
        assert!(ellenborough.principle.contains("dominant"));
        assert_eq!(ellenborough.year, 1956);

        let tulk = LandLawCase::tulk_v_moxhay();
        assert!(tulk.principle.contains("Restrictive"));
        assert_eq!(tulk.year, 1848);
    }

    #[test]
    fn test_title_guarantee() {
        let full = TitleGuarantee::Full;
        assert_eq!(full, TitleGuarantee::Full);
    }

    #[test]
    fn test_transfer_deed() {
        let tr1 = TransferDeed::Tr1;
        assert_eq!(tr1, TransferDeed::Tr1);
    }
}
