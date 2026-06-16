//! Securities and Futures Act 2001 - Capital Markets Products and Licensing
//!
//! Type-safe models of the building blocks of Singapore's capital markets under
//! the **Securities and Futures Act 2001 (SFA)**, administered by the Monetary
//! Authority of Singapore (MAS):
//!
//! - **Capital markets products** (s. 2(1)): securities, units in a collective
//!   investment scheme, derivatives contracts and spot foreign exchange
//!   contracts for leveraged foreign exchange trading.
//! - **Investor classification** (s. 4A): institutional, accredited and retail
//!   investors - the gateway to the Part 13 offering exemptions.
//! - **Licensing** (Part 4): the Capital Markets Services (CMS) licence (s. 82),
//!   the regulated activities in the Second Schedule, and the appointment of
//!   representatives (s. 99B).
//!
//! Monetary values are stored as **SGD cents** (`u64`), matching the convention
//! used across `legalis-sg`.

use serde::{Deserialize, Serialize};

// ============================================================================
// Statutory thresholds (SFA s. 4A - definition of "accredited investor")
// ============================================================================

/// Accredited-investor threshold: an individual whose net personal assets
/// exceed this amount (SFA s. 4A(1)(a)(i); SF(CMP) Regulations). SGD 2,000,000,
/// in cents.
pub const ACCREDITED_NET_PERSONAL_ASSETS_CENTS: u64 = 200_000_000;

/// For the net personal assets test, the value of an individual's primary
/// residence may only contribute up to this amount (SF(CMP) Regulations).
/// SGD 1,000,000, in cents.
pub const ACCREDITED_PRIMARY_RESIDENCE_CAP_CENTS: u64 = 100_000_000;

/// Accredited-investor threshold: an individual whose net financial assets
/// exceed this amount (SFA s. 4A(1)(a)(i)). SGD 1,000,000, in cents.
pub const ACCREDITED_NET_FINANCIAL_ASSETS_CENTS: u64 = 100_000_000;

/// Accredited-investor threshold: an individual whose income in the preceding 12
/// months is not less than this amount (SFA s. 4A(1)(a)(i)). SGD 300,000, in
/// cents.
pub const ACCREDITED_ANNUAL_INCOME_CENTS: u64 = 30_000_000;

/// Accredited-investor threshold: a corporation with net assets exceeding this
/// amount (SFA s. 4A(1)(b)). SGD 10,000,000, in cents.
pub const ACCREDITED_CORPORATION_NET_ASSETS_CENTS: u64 = 1_000_000_000;

// ============================================================================
// Capital markets products (SFA s. 2(1))
// ============================================================================

/// The classes of **capital markets product** defined by SFA s. 2(1).
///
/// The SFA regulates capital markets products; the class determines which
/// regulatory regimes (offering, licensing, market conduct) apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapitalMarketsProduct {
    /// Securities: debentures, stocks or shares, and units of a business trust
    /// (and certain securities-based derivatives).
    Securities,
    /// Units in a collective investment scheme.
    CollectiveInvestmentSchemeUnits,
    /// Derivatives contracts (futures, options, swaps and the like) - the SFA was
    /// amended in 2017 to regulate "derivatives contracts" in place of the former
    /// "futures contracts".
    DerivativesContract,
    /// Spot foreign exchange contracts for the purposes of leveraged foreign
    /// exchange trading.
    SpotForexLeveraged,
}

impl CapitalMarketsProduct {
    /// Returns the defining statutory reference (all classes are defined in
    /// s. 2(1)).
    pub fn statute_reference(&self) -> &'static str {
        "SFA s. 2(1)"
    }

    /// Returns a plain-language description of the product class.
    pub fn description(&self) -> &'static str {
        match self {
            CapitalMarketsProduct::Securities => {
                "Securities - debentures, stocks or shares, and units of a business trust"
            }
            CapitalMarketsProduct::CollectiveInvestmentSchemeUnits => {
                "Units in a collective investment scheme"
            }
            CapitalMarketsProduct::DerivativesContract => {
                "Derivatives contracts - futures, options, swaps and the like"
            }
            CapitalMarketsProduct::SpotForexLeveraged => {
                "Spot foreign exchange contracts for leveraged foreign exchange trading"
            }
        }
    }

    /// Whether an offer of this product to the public engages the Part 13
    /// prospectus regime. Securities and units in a collective investment scheme
    /// are offered under Part 13; pure derivatives and leveraged spot FX are not
    /// offered under the prospectus regime.
    pub fn engages_prospectus_regime(&self) -> bool {
        matches!(
            self,
            CapitalMarketsProduct::Securities
                | CapitalMarketsProduct::CollectiveInvestmentSchemeUnits
        )
    }
}

/// The kind of security (a subset of capital markets products).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityKind {
    /// Ordinary or preference shares in a corporation.
    Share,
    /// A debenture (including bonds and notes evidencing indebtedness).
    Debenture,
    /// A unit in a business trust.
    BusinessTrustUnit,
    /// A securities-based derivatives contract (e.g. an option over shares).
    SecuritiesBasedDerivative,
}

impl SecurityKind {
    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            SecurityKind::Share => "Shares in a corporation",
            SecurityKind::Debenture => "Debenture (bond or note evidencing indebtedness)",
            SecurityKind::BusinessTrustUnit => "Unit in a business trust",
            SecurityKind::SecuritiesBasedDerivative => "Securities-based derivatives contract",
        }
    }
}

/// A security being issued or dealt in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Security {
    /// Name / description of the security.
    pub name: String,
    /// Kind of security.
    pub kind: SecurityKind,
    /// Name of the issuer.
    pub issuer: String,
}

impl Security {
    /// Creates a new security.
    pub fn new(name: impl Into<String>, kind: SecurityKind, issuer: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind,
            issuer: issuer.into(),
        }
    }

    /// Returns the capital markets product class of this security (always
    /// [`CapitalMarketsProduct::Securities`]).
    pub fn product_class(&self) -> CapitalMarketsProduct {
        CapitalMarketsProduct::Securities
    }
}

/// The kind of derivatives contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivativeKind {
    /// An exchange-traded or OTC futures contract.
    Futures,
    /// An option (call or put).
    Option,
    /// A swap (interest rate, currency, etc.).
    Swap,
    /// A contract for differences (CFD).
    ContractForDifference,
    /// A leveraged spot foreign exchange contract.
    LeveragedForex,
}

impl DerivativeKind {
    /// Returns a plain-language description.
    pub fn description(&self) -> &'static str {
        match self {
            DerivativeKind::Futures => "Futures contract",
            DerivativeKind::Option => "Option (call or put)",
            DerivativeKind::Swap => "Swap (interest-rate, currency or other)",
            DerivativeKind::ContractForDifference => "Contract for differences (CFD)",
            DerivativeKind::LeveragedForex => "Leveraged spot foreign exchange contract",
        }
    }

    /// Returns the capital markets product class for this derivative kind.
    pub fn product_class(&self) -> CapitalMarketsProduct {
        match self {
            DerivativeKind::LeveragedForex => CapitalMarketsProduct::SpotForexLeveraged,
            _ => CapitalMarketsProduct::DerivativesContract,
        }
    }
}

/// A derivatives contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DerivativesContract {
    /// Description of the underlying (e.g. "STI index", "USD/SGD").
    pub underlying: String,
    /// Kind of derivative.
    pub kind: DerivativeKind,
}

impl DerivativesContract {
    /// Creates a new derivatives contract.
    pub fn new(underlying: impl Into<String>, kind: DerivativeKind) -> Self {
        Self {
            underlying: underlying.into(),
            kind,
        }
    }

    /// Returns the capital markets product class of this contract.
    pub fn product_class(&self) -> CapitalMarketsProduct {
        self.kind.product_class()
    }
}

// ============================================================================
// Collective investment schemes (SFA Part 13 Division 2; s. 286/s. 287)
// ============================================================================

/// The authorisation/recognition status of a collective investment scheme (CIS).
///
/// A CIS may only be offered to the public in Singapore if it is **authorised**
/// (constituted in Singapore - SFA s. 286) or **recognised** (constituted
/// outside Singapore - SFA s. 287). Restricted schemes may be offered only to
/// limited classes of investors without authorisation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CisAuthorisationStatus {
    /// Authorised scheme constituted in Singapore (SFA s. 286).
    Authorised,
    /// Recognised scheme constituted outside Singapore (SFA s. 287).
    Recognised,
    /// A restricted scheme offered only to relevant persons / accredited or
    /// institutional investors, not to the retail public.
    Restricted,
    /// Not authorised, recognised or otherwise exempt.
    NotAuthorised,
}

impl CisAuthorisationStatus {
    /// Returns the statutory reference for this status.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            CisAuthorisationStatus::Authorised => "SFA s. 286",
            CisAuthorisationStatus::Recognised => "SFA s. 287",
            CisAuthorisationStatus::Restricted => "SFA s. 305/s. 305A",
            CisAuthorisationStatus::NotAuthorised => "SFA s. 286/s. 287",
        }
    }

    /// Whether a scheme with this status may be offered to the retail public.
    pub fn may_offer_to_public(&self) -> bool {
        matches!(
            self,
            CisAuthorisationStatus::Authorised | CisAuthorisationStatus::Recognised
        )
    }
}

/// A collective investment scheme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectiveInvestmentScheme {
    /// Name of the scheme.
    pub name: String,
    /// Whether the scheme is constituted in Singapore.
    pub constituted_in_singapore: bool,
    /// Authorisation/recognition status.
    pub status: CisAuthorisationStatus,
}

impl CollectiveInvestmentScheme {
    /// Creates a new collective investment scheme record.
    pub fn new(
        name: impl Into<String>,
        constituted_in_singapore: bool,
        status: CisAuthorisationStatus,
    ) -> Self {
        Self {
            name: name.into(),
            constituted_in_singapore,
            status,
        }
    }

    /// Whether the scheme may be offered to the retail public in Singapore.
    pub fn may_offer_to_public(&self) -> bool {
        self.status.may_offer_to_public()
    }
}

// ============================================================================
// Investor classification (SFA s. 4A)
// ============================================================================

/// The class of an investor, which controls the availability of the Part 13
/// offering exemptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestorClass {
    /// An institutional investor (banks, MAS-regulated entities, the Government,
    /// statutory bodies, etc.) - SFA s. 4A(1)(c).
    Institutional,
    /// An accredited investor meeting the wealth/income thresholds - SFA
    /// s. 4A(1)(a)/(b).
    Accredited,
    /// A retail investor (the general public), entitled to the full protection of
    /// the prospectus regime.
    Retail,
}

impl InvestorClass {
    /// Returns the statutory reference for this class.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            InvestorClass::Institutional => "SFA s. 4A(1)(c)",
            InvestorClass::Accredited => "SFA s. 4A(1)(a)/(b)",
            InvestorClass::Retail => "SFA Part 13 (general public)",
        }
    }

    /// Whether an investor of this class is a "sophisticated" investor for whom
    /// the prospectus protections may be disapplied (institutional or
    /// accredited).
    pub fn is_sophisticated(&self) -> bool {
        matches!(
            self,
            InvestorClass::Institutional | InvestorClass::Accredited
        )
    }
}

/// Whether an **individual** qualifies as an accredited investor under SFA
/// s. 4A(1)(a), applying the wealth and income thresholds.
///
/// An individual is accredited if **any** of the following is satisfied:
/// - net personal assets exceed [`ACCREDITED_NET_PERSONAL_ASSETS_CENTS`]
///   (SGD 2m), where the primary residence contributes at most
///   [`ACCREDITED_PRIMARY_RESIDENCE_CAP_CENTS`] (SGD 1m); or
/// - net financial assets exceed [`ACCREDITED_NET_FINANCIAL_ASSETS_CENTS`]
///   (SGD 1m); or
/// - income in the preceding 12 months is at least
///   [`ACCREDITED_ANNUAL_INCOME_CENTS`] (SGD 300k).
///
/// All amounts are in SGD cents.
pub fn is_accredited_individual(
    net_financial_assets_cents: u64,
    net_other_assets_cents: u64,
    primary_residence_net_equity_cents: u64,
    income_last_12_months_cents: u64,
) -> bool {
    // Financial assets test.
    if net_financial_assets_cents > ACCREDITED_NET_FINANCIAL_ASSETS_CENTS {
        return true;
    }
    // Income test.
    if income_last_12_months_cents >= ACCREDITED_ANNUAL_INCOME_CENTS {
        return true;
    }
    // Net personal assets test, capping the primary residence contribution.
    let capped_residence =
        primary_residence_net_equity_cents.min(ACCREDITED_PRIMARY_RESIDENCE_CAP_CENTS);
    let net_personal_assets = net_financial_assets_cents
        .saturating_add(net_other_assets_cents)
        .saturating_add(capped_residence);
    net_personal_assets > ACCREDITED_NET_PERSONAL_ASSETS_CENTS
}

/// Whether a **corporation** qualifies as an accredited investor under SFA
/// s. 4A(1)(b): net assets exceeding [`ACCREDITED_CORPORATION_NET_ASSETS_CENTS`]
/// (SGD 10m). The amount is in SGD cents.
pub fn is_accredited_corporation(net_assets_cents: u64) -> bool {
    net_assets_cents > ACCREDITED_CORPORATION_NET_ASSETS_CENTS
}

// ============================================================================
// Licensing (SFA Part 4)
// ============================================================================

/// A **regulated activity** for which a Capital Markets Services (CMS) licence is
/// required (SFA Second Schedule).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatedActivity {
    /// Dealing in capital markets products.
    DealingInCapitalMarketsProducts,
    /// Advising on corporate finance.
    AdvisingOnCorporateFinance,
    /// Fund management.
    FundManagement,
    /// Real estate investment trust management.
    RealEstateInvestmentTrustManagement,
    /// Product financing.
    ProductFinancing,
    /// Providing credit rating services.
    ProvidingCreditRatingServices,
    /// Providing custodial services.
    ProvidingCustodialServices,
}

impl RegulatedActivity {
    /// Returns the statutory reference (all regulated activities are listed in
    /// the Second Schedule).
    pub fn statute_reference(&self) -> &'static str {
        "SFA Second Schedule"
    }

    /// Returns a plain-language description of the activity.
    pub fn description(&self) -> &'static str {
        match self {
            RegulatedActivity::DealingInCapitalMarketsProducts => {
                "Dealing in capital markets products"
            }
            RegulatedActivity::AdvisingOnCorporateFinance => "Advising on corporate finance",
            RegulatedActivity::FundManagement => "Fund management",
            RegulatedActivity::RealEstateInvestmentTrustManagement => {
                "Real estate investment trust management"
            }
            RegulatedActivity::ProductFinancing => "Product financing",
            RegulatedActivity::ProvidingCreditRatingServices => "Providing credit rating services",
            RegulatedActivity::ProvidingCustodialServices => "Providing custodial services",
        }
    }
}

/// The status of a Capital Markets Services (CMS) licence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CmsLicenceStatus {
    /// The licence has been granted and is in force (SFA s. 83).
    Granted,
    /// The licence has been suspended by MAS (SFA s. 95).
    Suspended,
    /// The licence has been revoked by MAS (SFA s. 95).
    Revoked,
    /// The licence has lapsed.
    Lapsed,
    /// No licence is held, but the person is exempt (e.g. a bank - SFA s. 99).
    Exempt,
    /// No licence is held and no exemption applies.
    NotLicensed,
}

impl CmsLicenceStatus {
    /// Whether the holder may lawfully carry on regulated activities under this
    /// status (a granted licence or an applicable exemption).
    pub fn permits_regulated_activity(&self) -> bool {
        matches!(self, CmsLicenceStatus::Granted | CmsLicenceStatus::Exempt)
    }

    /// Returns the statutory reference for this status.
    pub fn statute_reference(&self) -> &'static str {
        match self {
            CmsLicenceStatus::Granted => "SFA s. 83",
            CmsLicenceStatus::Suspended | CmsLicenceStatus::Revoked => "SFA s. 95",
            CmsLicenceStatus::Lapsed => "SFA s. 82",
            CmsLicenceStatus::Exempt => "SFA s. 99",
            CmsLicenceStatus::NotLicensed => "SFA s. 82",
        }
    }
}

/// A Capital Markets Services (CMS) licence (SFA s. 82-83).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapitalMarketsServicesLicence {
    /// Name of the licence holder.
    pub holder: String,
    /// The regulated activities authorised by the licence.
    pub activities: Vec<RegulatedActivity>,
    /// Current status of the licence.
    pub status: CmsLicenceStatus,
}

impl CapitalMarketsServicesLicence {
    /// Creates a new licence record (granted by default).
    pub fn new(holder: impl Into<String>, activities: Vec<RegulatedActivity>) -> Self {
        Self {
            holder: holder.into(),
            activities,
            status: CmsLicenceStatus::Granted,
        }
    }

    /// Sets the status of the licence.
    pub fn with_status(mut self, status: CmsLicenceStatus) -> Self {
        self.status = status;
        self
    }

    /// Whether the licence authorises the given regulated activity (and is in
    /// force).
    pub fn authorises(&self, activity: RegulatedActivity) -> bool {
        self.status.permits_regulated_activity() && self.activities.contains(&activity)
    }
}

/// An appointed representative acting for a principal (a CMS licensee or an
/// exempt financial institution) in a regulated activity (SFA s. 99B).
///
/// A person must not act as a representative unless their name appears in the
/// public register of representatives maintained by MAS (s. 99D), under the
/// Representative Notification Framework.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppointedRepresentative {
    /// Name of the representative.
    pub name: String,
    /// Name of the principal for whom the representative acts.
    pub principal: String,
    /// The regulated activities the representative is appointed to conduct.
    pub activities: Vec<RegulatedActivity>,
    /// Whether the representative's name is on the MAS public register (s. 99D).
    pub on_public_register: bool,
}

impl AppointedRepresentative {
    /// Creates a new representative record (on the public register by default).
    pub fn new(
        name: impl Into<String>,
        principal: impl Into<String>,
        activities: Vec<RegulatedActivity>,
    ) -> Self {
        Self {
            name: name.into(),
            principal: principal.into(),
            activities,
            on_public_register: true,
        }
    }

    /// Records that the representative is not on the MAS public register.
    pub fn not_on_register(mut self) -> Self {
        self.on_public_register = false;
        self
    }

    /// Whether the representative may lawfully act for the given activity.
    pub fn may_act(&self, activity: RegulatedActivity) -> bool {
        self.on_public_register && self.activities.contains(&activity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_class_metadata() {
        assert_eq!(
            CapitalMarketsProduct::Securities.statute_reference(),
            "SFA s. 2(1)"
        );
        assert!(CapitalMarketsProduct::Securities.engages_prospectus_regime());
        assert!(!CapitalMarketsProduct::DerivativesContract.engages_prospectus_regime());
    }

    #[test]
    fn test_security_product_class() {
        let s = Security::new("ABC Ordinary Shares", SecurityKind::Share, "ABC Pte Ltd");
        assert_eq!(s.product_class(), CapitalMarketsProduct::Securities);
    }

    #[test]
    fn test_derivative_product_class() {
        let fx = DerivativesContract::new("USD/SGD", DerivativeKind::LeveragedForex);
        assert_eq!(
            fx.product_class(),
            CapitalMarketsProduct::SpotForexLeveraged
        );

        let fut = DerivativesContract::new("STI index", DerivativeKind::Futures);
        assert_eq!(
            fut.product_class(),
            CapitalMarketsProduct::DerivativesContract
        );
    }

    #[test]
    fn test_cis_offer_to_public() {
        let authorised = CollectiveInvestmentScheme::new(
            "SG Equity Fund",
            true,
            CisAuthorisationStatus::Authorised,
        );
        assert!(authorised.may_offer_to_public());

        let restricted = CollectiveInvestmentScheme::new(
            "PE Restricted Fund",
            true,
            CisAuthorisationStatus::Restricted,
        );
        assert!(!restricted.may_offer_to_public());
        assert_eq!(restricted.status.statute_reference(), "SFA s. 305/s. 305A");
    }

    #[test]
    fn test_accredited_individual_financial_assets() {
        // Net financial assets just over SGD 1m -> accredited.
        assert!(is_accredited_individual(100_000_001, 0, 0, 0));
        // Exactly SGD 1m financial assets is NOT over the threshold.
        assert!(!is_accredited_individual(100_000_000, 0, 0, 0));
    }

    #[test]
    fn test_accredited_individual_income() {
        // Income of exactly SGD 300k qualifies (>= threshold).
        assert!(is_accredited_individual(0, 0, 0, 30_000_000));
        assert!(!is_accredited_individual(0, 0, 0, 29_999_999));
    }

    #[test]
    fn test_accredited_individual_net_personal_assets_residence_cap() {
        // SGD 1.5m financial + SGD 5m residence: residence is capped at SGD 1m,
        // so net personal assets = 1.5m + 1m = 2.5m > 2m -> accredited.
        assert!(is_accredited_individual(150_000_000, 0, 500_000_000, 0));

        // SGD 0.5m financial + SGD 5m residence: capped residence 1m,
        // net personal assets = 0.5m + 1m = 1.5m, not over 2m, and financial
        // assets do not exceed 1m -> NOT accredited.
        assert!(!is_accredited_individual(50_000_000, 0, 500_000_000, 0));
    }

    #[test]
    fn test_accredited_corporation() {
        assert!(is_accredited_corporation(1_000_000_001));
        assert!(!is_accredited_corporation(1_000_000_000));
    }

    #[test]
    fn test_investor_class_sophistication() {
        assert!(InvestorClass::Institutional.is_sophisticated());
        assert!(InvestorClass::Accredited.is_sophisticated());
        assert!(!InvestorClass::Retail.is_sophisticated());
    }

    #[test]
    fn test_cms_licence_authorises() {
        let licence = CapitalMarketsServicesLicence::new(
            "Alpha Capital Pte Ltd",
            vec![
                RegulatedActivity::FundManagement,
                RegulatedActivity::DealingInCapitalMarketsProducts,
            ],
        );
        assert!(licence.authorises(RegulatedActivity::FundManagement));
        assert!(!licence.authorises(RegulatedActivity::AdvisingOnCorporateFinance));

        let revoked = licence.with_status(CmsLicenceStatus::Revoked);
        assert!(!revoked.authorises(RegulatedActivity::FundManagement));
    }

    #[test]
    fn test_licence_status_exempt_permits_activity() {
        assert!(CmsLicenceStatus::Granted.permits_regulated_activity());
        assert!(CmsLicenceStatus::Exempt.permits_regulated_activity());
        assert!(!CmsLicenceStatus::Suspended.permits_regulated_activity());
        assert_eq!(CmsLicenceStatus::Exempt.statute_reference(), "SFA s. 99");
    }

    #[test]
    fn test_representative_may_act() {
        let rep = AppointedRepresentative::new(
            "Jane Tan",
            "Alpha Capital Pte Ltd",
            vec![RegulatedActivity::FundManagement],
        );
        assert!(rep.may_act(RegulatedActivity::FundManagement));
        assert!(!rep.may_act(RegulatedActivity::ProductFinancing));

        let off_register = rep.not_on_register();
        assert!(!off_register.may_act(RegulatedActivity::FundManagement));
    }

    #[test]
    fn test_types_serde_roundtrip() {
        let licence = CapitalMarketsServicesLicence::new(
            "Beta Securities Pte Ltd",
            vec![RegulatedActivity::DealingInCapitalMarketsProducts],
        )
        .with_status(CmsLicenceStatus::Granted);
        let json = serde_json::to_string(&licence).expect("serialize");
        let back: CapitalMarketsServicesLicence = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(licence, back);
    }
}
