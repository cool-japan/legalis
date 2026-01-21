//! Securities Law Types (Securities Act 1933, Securities Exchange Act 1934)
//!
//! This module provides types for US securities regulation under federal securities laws.

#![allow(missing_docs)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A security under the Securities Act of 1933
///
/// Section 2(a)(1) of the Securities Act defines "security" broadly to include:
/// - Stocks, bonds, notes
/// - Investment contracts (Howey Test)
/// - Limited partnership interests
/// - Warrants, options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Security {
    /// Unique identifier (CUSIP, ISIN, or internal ID)
    pub identifier: String,

    /// Security name
    pub name: String,

    /// Type of security
    pub security_type: SecurityType,

    /// Issuer information
    pub issuer: Issuer,

    /// Registration status with SEC
    pub registration_status: RegistrationStatus,

    /// Applicable exemptions from registration
    pub exemptions: Vec<Exemption>,

    /// Date of issuance
    pub issue_date: Option<NaiveDate>,

    /// Offering details if applicable
    pub offering: Option<Offering>,

    /// Whether this is a restricted security (Rule 144)
    pub is_restricted: bool,

    /// Trading restrictions
    pub trading_restrictions: Vec<TradingRestriction>,
}

impl Security {
    /// Check if the security requires SEC registration
    pub fn requires_registration(&self) -> bool {
        matches!(
            self.registration_status,
            RegistrationStatus::Required | RegistrationStatus::Pending { .. }
        ) && self.exemptions.is_empty()
    }

    /// Check if the security is publicly traded
    pub fn is_publicly_traded(&self) -> bool {
        matches!(
            self.registration_status,
            RegistrationStatus::Registered { .. }
        ) && matches!(
            self.security_type,
            SecurityType::CommonStock | SecurityType::PreferredStock | SecurityType::Bond { .. }
        )
    }

    /// Check if the security qualifies as an accredited investor-only security
    pub fn is_accredited_only(&self) -> bool {
        self.exemptions.iter().any(|e| match e {
            Exemption::RegulationD {
                accredited_only, ..
            } => *accredited_only,
            _ => false,
        })
    }
}

/// Types of securities
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SecurityType {
    /// Common stock (equity)
    CommonStock,

    /// Preferred stock (equity with preferences)
    PreferredStock,

    /// Corporate bond
    Bond {
        maturity: Option<NaiveDate>,
        interest_rate: Option<String>,
    },

    /// Convertible note (debt convertible to equity)
    ConvertibleNote { conversion_terms: String },

    /// Simple Agreement for Future Equity (SAFE)
    Safe {
        valuation_cap: Option<f64>,
        discount_rate: Option<f64>,
    },

    /// Warrant (option to purchase stock)
    Warrant {
        exercise_price: f64,
        expiration: Option<NaiveDate>,
    },

    /// Stock option
    StockOption {
        strike_price: f64,
        vesting_schedule: Option<String>,
    },

    /// Investment contract (Howey Test)
    InvestmentContract { description: String },

    /// Limited partnership interest
    LimitedPartnershipInterest,

    /// LLC membership interest
    LlcMembershipInterest,

    /// Asset-backed security
    AssetBackedSecurity { underlying_assets: String },

    /// Mutual fund share
    MutualFundShare,

    /// Exchange-traded fund (ETF) share
    EtfShare,

    /// Real estate investment trust (REIT) share
    ReitShare,

    /// Cryptocurrency token (if deemed a security)
    CryptoToken {
        blockchain: String,
        token_standard: Option<String>,
    },

    /// Other security type
    Other { description: String },
}

/// Issuer of a security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Issuer {
    /// Legal name of issuer
    pub name: String,

    /// Jurisdiction of incorporation
    pub jurisdiction: String,

    /// CIK (Central Index Key) for SEC filers
    pub cik: Option<String>,

    /// Issuer type
    pub issuer_type: IssuerType,

    /// Whether issuer is a reporting company (Exchange Act)
    pub is_reporting_company: bool,

    /// SIC (Standard Industrial Classification) code
    pub sic_code: Option<String>,

    /// Total assets (for materiality determination)
    pub total_assets: Option<f64>,
}

/// Type of issuer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssuerType {
    /// Corporation (C-corp or S-corp)
    Corporation,

    /// Limited liability company (LLC)
    Llc,

    /// Limited partnership (LP)
    LimitedPartnership,

    /// General partnership
    GeneralPartnership,

    /// Investment company (mutual fund, ETF)
    InvestmentCompany,

    /// Real estate investment trust (REIT)
    Reit,

    /// Special purpose vehicle (SPV)
    SpecialPurposeVehicle,

    /// Foreign issuer
    ForeignIssuer { home_country: String },

    /// Other entity type
    Other,
}

/// Registration status with SEC
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Security is registered (Form S-1, S-3, etc. effective)
    Registered {
        registration_number: String,
        effective_date: NaiveDate,
        form_type: RegistrationFormType,
    },

    /// Registration pending (filed but not yet effective)
    Pending {
        filing_date: NaiveDate,
        form_type: RegistrationFormType,
    },

    /// Registration required but not yet filed
    Required,

    /// Exempt from registration
    Exempt { exemption_basis: Vec<Exemption> },

    /// Not a security (does not require registration)
    NotApplicable,
}

/// SEC registration form types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrationFormType {
    /// Form S-1 (general registration)
    S1,

    /// Form S-3 (shelf registration for qualified issuers)
    S3,

    /// Form S-4 (business combinations)
    S4,

    /// Form S-8 (employee benefit plans)
    S8,

    /// Form F-1 (foreign private issuers)
    F1,

    /// Form F-3 (shelf registration for foreign issuers)
    F3,

    /// Form 10 (Exchange Act registration)
    Form10,

    /// Form 8-A (Exchange Act short form)
    Form8A,
}

/// Securities Act exemptions from registration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Exemption {
    /// Section 4(a)(2) private placement exemption
    PrivatePlacement {
        number_of_purchasers: Option<usize>,
        purchaser_sophistication: PurchaserSophistication,
    },

    /// Regulation D exemptions (Rules 504, 506(b), 506(c))
    RegulationD {
        rule: RegulationDRule,
        offering_amount: f64,
        accredited_only: bool,
        general_solicitation: bool,
        filing_form_d: bool,
    },

    /// Regulation A (Tier 1 or Tier 2) - "mini-IPO"
    RegulationA {
        tier: RegulationATier,
        offering_amount: f64,
        offering_circular_qualified: bool,
    },

    /// Regulation S (offshore offerings)
    RegulationS {
        category: RegulationSCategory,
        distribution_compliance_period: Option<u32>,
    },

    /// Regulation Crowdfunding (Section 4(a)(6))
    RegulationCrowdfunding {
        offering_amount: f64,
        funding_portal: String,
    },

    /// Rule 144 (resale of restricted securities)
    Rule144 {
        holding_period: u32,
        public_information_available: bool,
        trading_volume_limits: bool,
    },

    /// Rule 144A (qualified institutional buyers)
    Rule144A { qib_only: bool },

    /// Section 3(a)(11) intrastate exemption
    Intrastate {
        state: String,
        all_purchasers_in_state: bool,
    },

    /// Section 3(a)(9) exchange exemption
    ExchangeExemption {
        exchange_with_existing_security_holders: bool,
    },

    /// Section 4(a)(1)(1/2) exemption for secondary sales
    SecondaryMarketExemption,

    /// Other exemption
    Other {
        description: String,
        legal_basis: String,
    },
}

/// Regulation D rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulationDRule {
    /// Rule 504 (up to $10 million in 12 months)
    Rule504,

    /// Rule 506(b) (unlimited, up to 35 non-accredited, no general solicitation)
    Rule506B,

    /// Rule 506(c) (unlimited, accredited only, general solicitation allowed)
    Rule506C,
}

/// Regulation A tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulationATier {
    /// Tier 1 (up to $20 million in 12 months)
    Tier1,

    /// Tier 2 (up to $75 million in 12 months) - "Reg A+"
    Tier2,
}

/// Regulation S categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulationSCategory {
    /// Category 1 (foreign issuers with no substantial US market interest)
    Category1,

    /// Category 2 (reporting companies or debt securities)
    Category2,

    /// Category 3 (non-reporting equity securities)
    Category3,
}

/// Purchaser sophistication for private placements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurchaserSophistication {
    /// Accredited investor (Reg D definition)
    AccreditedInvestor,

    /// Sophisticated investor (knowledge and experience)
    Sophisticated,

    /// Institutional investor
    Institutional,

    /// Retail investor (generally not eligible for private placements)
    Retail,
}

/// Securities offering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offering {
    /// Type of offering
    pub offering_type: OfferingType,

    /// Total offering size (USD)
    pub offering_size: f64,

    /// Amount raised to date
    pub amount_raised: f64,

    /// Number of investors
    pub number_of_investors: usize,

    /// Offering period
    pub offering_start: NaiveDate,
    pub offering_end: Option<NaiveDate>,

    /// Underwriters (if any)
    pub underwriters: Vec<Underwriter>,

    /// Use of proceeds
    pub use_of_proceeds: Option<String>,

    /// Minimum investment amount
    pub minimum_investment: Option<f64>,
}

/// Type of securities offering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfferingType {
    /// Initial public offering (IPO)
    Ipo,

    /// Follow-on offering (secondary offering)
    FollowOn,

    /// Private placement
    PrivatePlacement,

    /// Rights offering (to existing shareholders)
    RightsOffering,

    /// At-the-market offering (ATM)
    AtTheMarket,

    /// Shelf offering (registered but sold over time)
    ShelfOffering,

    /// Direct listing (no new shares issued)
    DirectListing,

    /// SPAC (special purpose acquisition company)
    Spac,

    /// Regulation A offering
    RegulationA,

    /// Regulation Crowdfunding
    Crowdfunding,
}

/// Underwriter for securities offering
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Underwriter {
    /// Name of underwriter (investment bank)
    pub name: String,

    /// Role in underwriting
    pub role: UnderwriterRole,

    /// Underwriting type
    pub underwriting_type: UnderwritingType,
}

/// Role of underwriter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnderwriterRole {
    /// Lead or book-running underwriter
    Lead,

    /// Co-manager
    CoManager,

    /// Syndicate member
    SyndicateMember,
}

/// Type of underwriting commitment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnderwritingType {
    /// Firm commitment (underwriter buys all shares)
    FirmCommitment,

    /// Best efforts (underwriter markets but doesn't guarantee sale)
    BestEfforts,

    /// All-or-nothing (offering succeeds only if all shares sold)
    AllOrNothing,
}

/// Trading restriction on a security
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradingRestriction {
    /// Rule 144 holding period requirement
    HoldingPeriod { days_remaining: u32 },

    /// Contractual lock-up agreement
    LockUp {
        expiration: NaiveDate,
        parties_bound: String,
    },

    /// Transfer restrictions (legend on certificate)
    TransferRestriction { restriction_text: String },

    /// Market-wide trading halt
    TradingHalt {
        reason: String,
        expected_duration: Option<String>,
    },

    /// Insider trading restriction (blackout period)
    InsiderBlackout { expiration: NaiveDate },
}

/// Accredited investor status (Regulation D definition)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccreditedInvestor {
    /// Whether the investor is accredited
    pub is_accredited: bool,

    /// Basis for accredited status
    pub accreditation_basis: Vec<AccreditationBasis>,

    /// Date of accreditation verification
    pub verification_date: NaiveDate,

    /// Method of verification
    pub verification_method: VerificationMethod,
}

/// Basis for accredited investor status (Rule 501(a))
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccreditationBasis {
    /// Individual income over $200k (or $300k joint) for past 2 years
    IncomeTest {
        individual_income: bool,
        joint_income: bool,
    },

    /// Net worth over $1 million (excluding primary residence)
    NetWorthTest,

    /// Professional certifications (Series 7, 65, 82)
    ProfessionalCertification { certification_type: String },

    /// Director, executive officer, or general partner of issuer
    InsiderStatus,

    /// Entity with assets over $5 million
    EntityAssets,

    /// All equity owners are accredited
    AllOwnersAccredited,

    /// Family office with $5 million in assets under management
    FamilyOffice,

    /// Knowledgeable employee of private fund
    KnowledgeableEmployee,
}

/// Method of verifying accredited investor status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationMethod {
    /// Review of tax returns, W-2s, 1099s
    IncomeDocumentation,

    /// Review of bank statements, brokerage statements, credit reports
    NetWorthDocumentation,

    /// Third-party verification letter
    ThirdPartyVerification,

    /// Self-certification (questionnaire)
    SelfCertification,

    /// Professional certification confirmation
    ProfessionalCertificationConfirmation,
}

/// SEC disclosure document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecDisclosure {
    /// Type of SEC filing
    pub filing_type: SecFilingType,

    /// Filing date
    pub filing_date: NaiveDate,

    /// Accession number
    pub accession_number: Option<String>,

    /// Reporting period (for periodic reports)
    pub period_end: Option<NaiveDate>,

    /// Whether filing is amended
    pub is_amendment: bool,

    /// URL to EDGAR filing
    pub edgar_url: Option<String>,
}

/// SEC filing types (not exhaustive)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecFilingType {
    // Registration statements
    /// Form S-1 (IPO registration)
    S1,

    /// Form S-3 (shelf registration)
    S3,

    /// Form S-4 (mergers/acquisitions)
    S4,

    /// Form S-8 (employee benefit plans)
    S8,

    // Periodic reports (Exchange Act)
    /// Form 10-K (annual report)
    Form10K,

    /// Form 10-Q (quarterly report)
    Form10Q,

    /// Form 8-K (current report - material events)
    Form8K,

    // Proxy statements
    /// Schedule 14A (proxy statement)
    Schedule14A,

    /// Schedule 14C (information statement)
    Schedule14C,

    // Tender offers
    /// Schedule TO (tender offer)
    ScheduleTO,

    /// Schedule 13D (beneficial ownership - active)
    Schedule13D,

    /// Schedule 13G (beneficial ownership - passive)
    Schedule13G,

    // Regulation D
    /// Form D (notice of exempt offering)
    FormD,

    // Regulation A
    /// Form 1-A (offering statement)
    Form1A,

    // Regulation Crowdfunding
    /// Form C (crowdfunding offering)
    FormC,

    /// Form C-U (crowdfunding progress update)
    FormCU,

    /// Form C-AR (crowdfunding annual report)
    FormCAR,

    // Insider trading
    /// Form 3 (initial statement of beneficial ownership)
    Form3,

    /// Form 4 (statement of changes in beneficial ownership)
    Form4,

    /// Form 5 (annual statement of beneficial ownership)
    Form5,

    // Investment companies
    /// Form N-1A (mutual fund registration)
    FormN1A,

    /// Form N-PX (proxy voting record)
    FormNPX,

    // Other
    /// Form 144 (notice of proposed sale of restricted securities)
    Form144,

    /// Other filing type
    Other { form_name: String },
}

/// Blue sky law compliance (state securities laws)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlueSkyCompliance {
    /// State where compliance is assessed
    pub state: String,

    /// Whether offering must be registered with state
    pub registration_required: bool,

    /// Whether offering qualifies for state exemption
    pub state_exemption: Option<StateLawExemption>,

    /// Filing status
    pub filing_status: Option<String>,

    /// Notice filing required (for covered securities)
    pub notice_filing_required: bool,
}

/// State law exemptions from registration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateLawExemption {
    /// Uniform Securities Act exemption
    UniformSecuritiesAct,

    /// Manual exemption (manual review by state)
    Manual,

    /// Covered security (NSMIA preemption)
    CoveredSecurity,

    /// Small offering exemption
    SmallOffering,

    /// Limited offering exemption
    LimitedOffering,
}

/// Qualified institutional buyer (Rule 144A)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualifiedInstitutionalBuyer {
    /// Entity name
    pub name: String,

    /// Type of QIB
    pub qib_type: QibType,

    /// Securities owned and invested (threshold: $100 million)
    pub securities_owned: f64,

    /// Verification date
    pub verification_date: NaiveDate,
}

/// Type of qualified institutional buyer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QibType {
    /// Registered investment company
    InvestmentCompany,

    /// Insurance company
    InsuranceCompany,

    /// Employee benefit plan with assets > $5 million
    EmployeeBenefitPlan,

    /// Bank or savings and loan
    BankOrSavingsAndLoan,

    /// Registered broker-dealer (securities owned > $10 million)
    BrokerDealer,

    /// Business development company
    BusinessDevelopmentCompany,

    /// Other QIB entity
    Other,
}

/// Howey Test for investment contract analysis
///
/// SEC v. W.J. Howey Co., 328 U.S. 293 (1946)
/// An investment contract exists if there is:
/// 1. Investment of money
/// 2. In a common enterprise
/// 3. With expectation of profits
/// 4. Derived from efforts of others
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HoweyTestAnalysis {
    /// Investment of money present
    pub investment_of_money: bool,

    /// Common enterprise present
    pub common_enterprise: CommonEnterpriseType,

    /// Expectation of profits
    pub expectation_of_profits: bool,

    /// Profits derived from efforts of others
    pub efforts_of_others: EffortsOfOthersAnalysis,

    /// Overall conclusion: is it a security?
    pub is_security: bool,

    /// Additional factors considered
    pub additional_factors: Vec<String>,
}

/// Type of common enterprise
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommonEnterpriseType {
    /// Horizontal commonality (pooled funds, pro rata distribution)
    HorizontalCommonality,

    /// Vertical commonality (fortunes tied to promoter's success)
    VerticalCommonality { broad: bool, narrow: bool },

    /// No common enterprise
    NoCommonEnterprise,
}

/// Analysis of "efforts of others" prong
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffortsOfOthersAnalysis {
    /// Whether essential managerial efforts are by others
    pub essential_efforts_by_others: bool,

    /// Investor's role
    pub investor_role: InvestorRole,

    /// Promoter/manager control
    pub promoter_control_level: ControlLevel,
}

/// Investor's role in the enterprise
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestorRole {
    /// Purely passive investor
    Passive,

    /// Some involvement but not essential
    LimitedInvolvement,

    /// Active management role
    ActiveManagement,
}

/// Level of promoter/manager control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlLevel {
    /// Complete control by promoter
    Complete,

    /// Substantial control
    Substantial,

    /// Shared control
    Shared,

    /// Minimal control
    Minimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_requires_registration() {
        let security = Security {
            identifier: "TEST001".to_string(),
            name: "Test Common Stock".to_string(),
            security_type: SecurityType::CommonStock,
            issuer: Issuer {
                name: "Test Corp".to_string(),
                jurisdiction: "Delaware".to_string(),
                cik: None,
                issuer_type: IssuerType::Corporation,
                is_reporting_company: false,
                sic_code: None,
                total_assets: None,
            },
            registration_status: RegistrationStatus::Required,
            exemptions: vec![],
            issue_date: None,
            offering: None,
            is_restricted: false,
            trading_restrictions: vec![],
        };

        assert!(security.requires_registration());
    }

    #[test]
    fn test_security_with_exemption() {
        let security = Security {
            identifier: "TEST002".to_string(),
            name: "Test Preferred Stock".to_string(),
            security_type: SecurityType::PreferredStock,
            issuer: Issuer {
                name: "Test LLC".to_string(),
                jurisdiction: "California".to_string(),
                cik: None,
                issuer_type: IssuerType::Llc,
                is_reporting_company: false,
                sic_code: None,
                total_assets: Some(1_000_000.0),
            },
            registration_status: RegistrationStatus::Exempt {
                exemption_basis: vec![Exemption::RegulationD {
                    rule: RegulationDRule::Rule506B,
                    offering_amount: 5_000_000.0,
                    accredited_only: false,
                    general_solicitation: false,
                    filing_form_d: true,
                }],
            },
            exemptions: vec![Exemption::RegulationD {
                rule: RegulationDRule::Rule506B,
                offering_amount: 5_000_000.0,
                accredited_only: false,
                general_solicitation: false,
                filing_form_d: true,
            }],
            issue_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date")),
            offering: None,
            is_restricted: true,
            trading_restrictions: vec![],
        };

        assert!(!security.requires_registration());
    }

    #[test]
    fn test_accredited_investor_income_test() {
        let investor = AccreditedInvestor {
            is_accredited: true,
            accreditation_basis: vec![AccreditationBasis::IncomeTest {
                individual_income: true,
                joint_income: false,
            }],
            verification_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            verification_method: VerificationMethod::IncomeDocumentation,
        };

        assert!(investor.is_accredited);
        assert_eq!(investor.accreditation_basis.len(), 1);
    }

    #[test]
    fn test_howey_test_is_security() {
        let analysis = HoweyTestAnalysis {
            investment_of_money: true,
            common_enterprise: CommonEnterpriseType::HorizontalCommonality,
            expectation_of_profits: true,
            efforts_of_others: EffortsOfOthersAnalysis {
                essential_efforts_by_others: true,
                investor_role: InvestorRole::Passive,
                promoter_control_level: ControlLevel::Complete,
            },
            is_security: true,
            additional_factors: vec![],
        };

        assert!(analysis.is_security);
    }
}
