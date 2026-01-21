//! Securities Law Module (Securities Act 1933, Securities Exchange Act 1934)
//!
//! Comprehensive implementation of US federal securities regulation.
//!
//! # Key Legislation
//!
//! ## Securities Act of 1933 ("Securities Act" or "33 Act")
//!
//! The Securities Act of 1933 is the primary federal law regulating the **offer and sale** of securities.
//!
//! ### Core Purpose
//!
//! The 1933 Act has two main goals:
//! 1. **Disclosure**: Require issuers to provide material information to investors
//! 2. **Prohibition of fraud**: Prohibit misstatements and omissions in securities offerings
//!
//! ### Section 5: The Central Provision
//!
//! **Section 5(a): Registration Requirement**
//!
//! "Unless a registration statement is in effect as to a security, it shall be unlawful for any person:
//! (1) to sell such security through the use of interstate commerce, or
//! (2) to carry or transmit any prospectus relating to such security..."
//!
//! Violation of Section 5 is a **strict liability** offense.
//!
//! ### Timeline of an Offering
//!
//! ```text
//! Pre-Filing Period    |  Waiting Period       |  Post-Effective Period
//! (No offers allowed)  |  (Offers, no sales)   |  (Offers and sales OK)
//!                      |                       |
//!       ├──────────────┼───────────────────────┼─────────────────────────►
//!                      │                       │
//!                Filing Date           Effective Date
//!                (Registration         (SEC declares
//!                 Statement filed)      registration
//!                                       effective)
//!
//! Pre-Filing:
//! - Generally no offers permitted
//! - Exceptions: "Test the waters" (Reg A, Reg CF, EGC)
//!
//! Waiting Period:
//! - Oral offers permitted
//! - Written offers only via statutory prospectus or free writing prospectus
//! - No sales
//!
//! Post-Effective:
//! - Offers and sales permitted
//! - Prospectus delivery required
//! ```
//!
//! ### Exemptions from Registration
//!
//! The 1933 Act provides several exemptions from registration:
//!
//! #### Section 4(a)(2): Private Placement Exemption
//!
//! Exempts "transactions by an issuer not involving any public offering."
//!
//! Requirements (judicially developed):
//! - Limited number of offerees
//! - Offerees are sophisticated or have access to information
//! - No general solicitation or advertising
//! - Restricted securities (resale limitations)
//!
//! #### Regulation D: Safe Harbors for Private Placements
//!
//! **Rule 504**: Up to $10 million in 12 months
//! - State registration may be required
//! - General solicitation allowed if state-registered
//!
//! **Rule 506(b)**: Unlimited offering amount
//! - Up to 35 non-accredited investors (unlimited accredited)
//! - No general solicitation
//! - Non-accredited investors must be sophisticated
//!
//! **Rule 506(c)**: Unlimited offering amount
//! - Accredited investors only
//! - General solicitation allowed
//! - Must take reasonable steps to verify accredited status
//!
//! #### Regulation A: "Mini-IPO"
//!
//! **Tier 1**: Up to $20 million in 12 months
//! - State blue sky registration required
//! - Ongoing reporting requirements
//!
//! **Tier 2**: Up to $75 million in 12 months ("Reg A+")
//! - Preempts state registration (covered security)
//! - Audited financials required
//! - Ongoing reporting to SEC
//! - Investment limits for non-accredited investors (10% of income/net worth)
//!
//! #### Regulation Crowdfunding: Section 4(a)(6)
//!
//! - Up to $5 million in 12 months
//! - Must use registered funding portal or broker-dealer
//! - Investment limits based on investor income/net worth
//! - Ongoing reporting requirements
//!
//! #### Regulation S: Offshore Offerings
//!
//! - Offerings outside the United States
//! - No directed selling efforts in US
//! - Distribution compliance period (40 days to 1 year)
//!
//! ### Accredited Investor Definition (Rule 501(a))
//!
//! **Natural Persons**:
//! - Income over $200,000 (individual) or $300,000 (joint) for past 2 years with expectation of same
//! - Net worth over $1 million (excluding primary residence)
//! - Professional certifications: Series 7, 65, or 82
//!
//! **Entities**:
//! - Assets over $5 million (if not formed for specific investment)
//! - All equity owners are accredited investors
//! - Banks, insurance companies, investment companies, employee benefit plans
//!
//! ## Securities Exchange Act of 1934 ("Exchange Act" or "34 Act")
//!
//! The Exchange Act of 1934 regulates **secondary trading** of securities and ongoing disclosure.
//!
//! ### Section 12: Registration of Securities
//!
//! Companies must register under Section 12 if:
//! - Listed on national securities exchange, OR
//! - Assets > $10 million AND either:
//!   - 2,000+ shareholders of record, OR
//!   - 500+ non-accredited shareholders
//!
//! ### Periodic Reporting Requirements
//!
//! Registered companies must file:
//! - **Form 10-K**: Annual report (audited financials) - within 60-90 days of fiscal year-end
//! - **Form 10-Q**: Quarterly report (unaudited) - within 40-45 days of quarter-end
//! - **Form 8-K**: Current report for material events - within 4 business days
//!
//! ### Section 10(b) and Rule 10b-5: Antifraud Provision
//!
//! Rule 10b-5 makes it unlawful to:
//! - Employ any device, scheme, or artifice to defraud
//! - Make any untrue statement of material fact or omit to state a material fact
//! - Engage in any act, practice, or course of business which operates as a fraud
//!
//! #### Insider Trading
//!
//! Rule 10b-5 prohibits trading on **material non-public information** (MNPI).
//!
//! **Elements** (for insider trading liability):
//! 1. Material information (reasonable investor would consider important)
//! 2. Non-public (not generally disseminated)
//! 3. Breach of duty (fiduciary duty or misappropriation)
//! 4. Trading (or tipping)
//!
//! **Famous Cases**:
//! - *Chiarella v. United States* (1980): Duty to disclose or abstain
//! - *Dirks v. SEC* (1983): Tippee liability requires tipper benefit
//! - *United States v. O'Hagan* (1997): Misappropriation theory
//!
//! ### Section 16: Insider Reporting and Short-Swing Profits
//!
//! **Section 16(a)**: Officers, directors, and 10%+ shareholders must report transactions
//! - Form 3: Initial statement of beneficial ownership
//! - Form 4: Statement of changes (within 2 business days)
//! - Form 5: Annual statement
//!
//! **Section 16(b)**: Short-swing profit rule
//! - Insiders must disgorge profits from matching purchases and sales within 6 months
//! - Strict liability (no scienter requirement)
//! - Company or shareholder can sue to recover profits
//!
//! ### Section 13(d): Beneficial Ownership Reporting (5% shareholders)
//!
//! - **Schedule 13D**: Active investors (seeking control or influence) - within 10 days
//! - **Schedule 13G**: Passive investors - within 45 days of year-end
//!
//! ## Other Key Securities Laws
//!
//! ### Investment Company Act of 1940 ("40 Act")
//!
//! Regulates mutual funds, ETFs, and other investment companies.
//!
//! **Section 3(a)(1)**: Definition of investment company
//! - Primarily engaged in investing, reinvesting in securities
//! - > 40% of assets in investment securities
//!
//! **Exemptions**:
//! - Section 3(c)(1): ≤ 100 beneficial owners
//! - Section 3(c)(7): Qualified purchasers only
//!
//! ### Investment Advisers Act of 1940
//!
//! Regulates investment advisers.
//!
//! ### Trust Indenture Act of 1939
//!
//! Regulates debt securities (bonds).
//!
//! ### Sarbanes-Oxley Act of 2002 ("SOX")
//!
//! Enacted after Enron/WorldCom scandals to improve corporate governance and financial disclosure.
//!
//! **Key Provisions**:
//! - Section 302: CEO/CFO certification of financial statements
//! - Section 404: Internal control assessment
//! - Section 906: Criminal penalties for false certifications
//! - Auditor independence requirements
//!
//! ### Dodd-Frank Wall Street Reform Act of 2010
//!
//! Enacted after 2008 financial crisis.
//!
//! **Key Provisions**:
//! - Title VII: Derivatives regulation (swaps)
//! - Section 953: Pay ratio disclosure
//! - Section 1502: Conflict minerals disclosure
//! - Section 922: Whistleblower protections and bounties
//! - Volcker Rule: Proprietary trading restrictions for banks
//!
//! ### JOBS Act of 2012
//!
//! Jumpstart Our Business Startups Act - eased capital formation for emerging companies.
//!
//! **Key Provisions**:
//! - Emerging Growth Company (EGC) reduced disclosure requirements
//! - Regulation A+ (Tier 2 up to $75M)
//! - Regulation Crowdfunding
//! - General solicitation in Rule 506(c)
//! - Increased Section 12(g) thresholds (2,000 shareholders)
//!
//! ## Blue Sky Laws (State Securities Laws)
//!
//! Every state has its own securities laws ("blue sky laws").
//!
//! ### National Securities Markets Improvement Act of 1996 (NSMIA)
//!
//! NSMIA preempts state registration for **covered securities**:
//! - Securities listed on national exchanges (NYSE, Nasdaq)
//! - Securities offered under Regulation D (Rule 506)
//! - Securities offered under Regulation A (Tier 2)
//! - Securities issued by registered investment companies
//!
//! States can still:
//! - Require notice filings and fees
//! - Enforce antifraud provisions
//!
//! ## Rule 144: Resale of Restricted and Control Securities
//!
//! Rule 144 provides a safe harbor for resale of:
//! - **Restricted securities**: Acquired in unregistered offering
//! - **Control securities**: Held by affiliates
//!
//! **Requirements**:
//! 1. **Holding period**: 6 months (reporting company) or 1 year (non-reporting)
//! 2. **Current information**: Issuer current in Exchange Act reporting
//! 3. **Trading volume formula**: Greater of 1% of outstanding or 4-week average volume
//! 4. **Ordinary brokerage transactions**: No solicitation of buy orders
//! 5. **Form 144 notice**: If > 5,000 shares or $50,000 in 3 months
//!
//! ## Rule 144A: Resale to Qualified Institutional Buyers (QIBs)
//!
//! **Qualified Institutional Buyer** (QIB): Entity that owns/invests ≥ $100 million in securities
//! (≥ $10 million for broker-dealers).
//!
//! Rule 144A allows resale of restricted securities to QIBs without registration.
//!
//! Creates a **private placement secondary market** for institutional investors.
//!
//! ## The Howey Test: What is a Security?
//!
//! *SEC v. W.J. Howey Co.*, 328 U.S. 293 (1946)
//!
//! An **investment contract** is a security if it involves:
//! 1. **Investment of money**
//! 2. **In a common enterprise**
//! 3. **With expectation of profits**
//! 4. **Derived from the efforts of others**
//!
//! The Howey Test applies to non-traditional investments:
//! - Orange groves (*Howey*)
//! - Condominium hotel units (*United Housing Foundation*)
//! - Cryptocurrency tokens (*SEC v. Ripple*, *SEC v. Telegram*)
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_us::securities::*;
//! use chrono::NaiveDate;
//!
//! // Validate Regulation D offering
//! let exemption = Exemption::RegulationD {
//!     rule: RegulationDRule::Rule506B,
//!     offering_amount: 5_000_000.0,
//!     accredited_only: false,
//!     general_solicitation: false,
//!     filing_form_d: true,
//! };
//!
//! let offering = Offering {
//!     offering_type: OfferingType::PrivatePlacement,
//!     offering_size: 5_000_000.0,
//!     amount_raised: 3_000_000.0,
//!     number_of_investors: 25,
//!     offering_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     offering_end: None,
//!     underwriters: vec![],
//!     use_of_proceeds: Some("Working capital".to_string()),
//!     minimum_investment: Some(100_000.0),
//! };
//!
//! validate_regulation_d(&offering, &exemption, 20, 5)?;
//!
//! // Validate accredited investor
//! let investor = AccreditedInvestor {
//!     is_accredited: true,
//!     accreditation_basis: vec![AccreditationBasis::IncomeTest {
//!         individual_income: true,
//!         joint_income: false,
//!     }],
//!     verification_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     verification_method: VerificationMethod::IncomeDocumentation,
//! };
//!
//! validate_accredited_investor(&investor)?;
//!
//! // Analyze Howey Test for crypto token
//! let analysis = HoweyTestAnalysis {
//!     investment_of_money: true,
//!     common_enterprise: CommonEnterpriseType::HorizontalCommonality,
//!     expectation_of_profits: true,
//!     efforts_of_others: EffortsOfOthersAnalysis {
//!         essential_efforts_by_others: true,
//!         investor_role: InvestorRole::Passive,
//!         promoter_control_level: ControlLevel::Complete,
//!     },
//!     is_security: true,
//!     additional_factors: vec![],
//! };
//!
//! let is_security = validate_howey_test(&analysis)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export key types
pub use error::{Result, SecuritiesError};
pub use types::{
    AccreditationBasis, AccreditedInvestor, BlueSkyCompliance, CommonEnterpriseType, ControlLevel,
    EffortsOfOthersAnalysis, Exemption, HoweyTestAnalysis, InvestorRole, Issuer, IssuerType,
    Offering, OfferingType, PurchaserSophistication, QibType, QualifiedInstitutionalBuyer,
    RegistrationFormType, RegistrationStatus, RegulationATier, RegulationDRule,
    RegulationSCategory, SecDisclosure, SecFilingType, Security, SecurityType, StateLawExemption,
    TradingRestriction, Underwriter, UnderwriterRole, UnderwritingType, VerificationMethod,
};

// Re-export validator functions
pub use validator::{
    validate_3c1_exemption, validate_accredited_investor, validate_beneficial_ownership_reporting,
    validate_blue_sky_compliance, validate_crowdfunding, validate_exchange_act_registration,
    validate_form_4_timeliness, validate_howey_test, validate_offering_integration,
    validate_periodic_reporting, validate_qib_status, validate_registration, validate_regulation_a,
    validate_regulation_d, validate_rule_144_holding_period, validate_rule_144_volume,
    validate_section_16b,
};
