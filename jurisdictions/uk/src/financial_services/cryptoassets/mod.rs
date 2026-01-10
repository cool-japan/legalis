//! Cryptoassets Module (FSMA 2023, FCA PS19/22, MLR 2017 Reg 14A)
//!
//! Comprehensive implementation of UK cryptoassets regulation.
//!
//! # Key Legislation
//!
//! ## Financial Services and Markets Act 2023 (FSMA 2023)
//!
//! FSMA 2023 brought cryptoassets into UK regulatory perimeter with two major changes:
//!
//! ### Part 5: Stablecoin Regulation
//!
//! HM Treasury can designate stablecoins as "regulated payment arrangements" under systemic payment
//! system regulation. Designated stablecoins subject to:
//!
//! **Reserve backing requirements**:
//! - 1:1 backing with high-quality liquid assets (cash, central bank reserves, short-term government bonds)
//! - Reserves segregated from issuer's own assets
//! - Held on trust for token holders
//! - Cannot be used for issuer's business activities
//!
//! **Redemption rights**:
//! - Token holders must have enforceable claim to underlying reserves
//! - Redemption at par value (1:1) without excessive delay
//! - Clear redemption process disclosed to users
//!
//! **Operational resilience**:
//! - Systems and controls to ensure continuity
//! - Disaster recovery and business continuity plans
//! - Cyber security measures
//!
//! **Prudential requirements**:
//! - Minimum capital requirements
//! - Liquidity buffers
//! - Ongoing supervision by FCA/Bank of England
//!
//! **Independent audits**:
//! - Regular audits by qualified auditor
//! - Verify reserve assets exist and match tokens in circulation
//! - Audit reports published to token holders
//!
//! ### Section 21: Financial Promotions (Effective October 8, 2023)
//!
//! **Major milestone**: From October 8, 2023, cryptoasset promotions subject to FSMA 2000 s.21
//! financial promotions regime.
//!
//! **Promotion requirements**:
//! - Must be made by FCA-authorized person, OR
//! - Content approved by FCA-authorized person, OR
//! - Exemption applies (e.g., one-off non-solicited, high net worth, sophisticated investors)
//!
//! **FCA Policy Statement PS23/6** (June 2023) sets detailed rules:
//!
//! **Risk warnings** (mandatory):
//! - "Don't invest unless you're prepared to lose all the money you invest. This is a high-risk
//!   investment and you are unlikely to be protected if something goes wrong."
//! - Warning must be as prominent as any information about benefits
//! - Cannot be hidden in terms and conditions
//!
//! **Direct offer financial promotions** (specific investment):
//! - 24-hour cooling-off period before first-time buyers can respond
//! - Personalized risk warning based on client's financial situation
//! - FCA can ban firms from approving promotions
//!
//! **Influencer marketing**:
//! - Celebrity endorsements must disclose if paid
//! - Influencers must not approve financial promotions (only authorized firms)
//! - Influencer responsibility if promotion unapproved
//!
//! ## FCA Policy Statement PS19/22: Guidance on Cryptoassets
//!
//! FCA's definitive guidance on cryptoasset classification (July 2019, updated regularly).
//!
//! ### Three-Category Taxonomy
//!
//! #### 1. Security Tokens
//!
//! **Definition**: Cryptoassets that are "specified investments" under RAO 2001.
//!
//! **Characteristics**:
//! - Provide equity-like rights (voting, dividends, residual claim)
//! - Provide debt-like rights (interest, principal repayment, fixed maturity)
//! - Units in collective investment scheme
//! - Derivative contracts
//!
//! **Howey Test** (adapted from US law):
//! FCA uses modified Howey Test to assess if token is investment:
//! 1. **Investment of money**: Capital contributed by token holders
//! 2. **Common enterprise**: Pooled funds or shared profits
//! 3. **Expectation of profit**: Holders expect financial return
//! 4. **From efforts of others**: Profit depends on issuer/third-party efforts, not holder's own work
//!
//! If all four elements present: Likely a security token.
//!
//! **Regulation**:
//! - Full FCA authorization required (FSMA 2000 s.19)
//! - Prospectus required for public offers (UK Prospectus Regulation)
//! - MiFID II applies (transaction reporting, best execution, conduct rules)
//! - UK MAR applies (insider dealing, market manipulation prohibitions)
//! - COBS applies (client categorization, suitability, appropriateness)
//!
//! **Examples**:
//! - Tokenized shares (equity ownership in company)
//! - Tokenized bonds (debt instrument with interest and maturity)
//! - Security token offerings (STOs) replacing traditional IPOs
//! - Fund tokens (units in tokenized investment fund)
//!
//! #### 2. E-Money Tokens
//!
//! **Definition**: Cryptoassets meeting e-money definition under EMR 2011 Regulation 2.
//!
//! **E-money characteristics**:
//! 1. **Electronically stored monetary value**
//! 2. **Issued on receipt of funds** (fiat currency)
//! 3. **Accepted by third parties** as means of payment
//! 4. **Redeemable at par** (1:1 with fiat)
//!
//! **Regulation**:
//! - E-money institution (EMI) authorization required
//! - Electronic Money Regulations 2011 (EMR 2011) applies
//! - Safeguarding requirements (segregate client funds)
//! - Redemption rights (at par, without excessive delay)
//! - Prudential requirements (minimum capital, liquidity)
//!
//! **Examples**:
//! - USDC (USD Coin) - if UK issuer
//! - GBPT (GBP-pegged stablecoin)
//! - E-money stablecoins from regulated issuers
//!
//! **Not e-money** (key exclusions):
//! - Limited network tokens (only accepted by issuer, e.g., gift cards)
//! - Single-purpose tokens (only for issuer's goods/services)
//!
//! #### 3. Utility Tokens (Unregulated)
//!
//! **Definition**: Cryptoassets providing access to goods/services on platform.
//!
//! **Characteristics**:
//! - No investment characteristics
//! - Used to access specific application/platform
//! - Value primarily from utility, not speculation
//!
//! **Regulation**:
//! - Generally unregulated by FCA (unless disguised security token)
//! - BUT: Cryptoasset exchange providers regulated under MLR 2017 Reg 14A
//! - Consumer protection laws apply (Consumer Rights Act 2015)
//! - Common law fraud/misrepresentation
//!
//! **Examples**:
//! - Filecoin (file storage access)
//! - BAT (Basic Attention Token - advertising platform)
//! - In-game tokens (access to gaming features)
//!
//! **Warning**: Many "utility tokens" are disguised security tokens. FCA will look through
//! label to substance. If token has investment characteristics despite utility label,
//! treated as security.
//!
//! ## Money Laundering Regulations 2017 Regulation 14A
//!
//! **Major change**: Regulation 14A inserted January 10, 2020, bringing cryptoassets into
//! UK AML/CTF regime.
//!
//! ### Cryptoasset Definitions (Reg 14A)
//!
//! **Cryptoasset**: Cryptographically secured digital representation of value or contractual
//! rights that can be transferred, stored, or traded electronically.
//!
//! **Cryptoasset exchange provider**: Business that exchanges cryptoassets for money or
//! other cryptoassets.
//!
//! **Custodian wallet provider**: Business that safeguards private cryptographic keys on
//! behalf of customers.
//!
//! ### FCA Registration Requirement
//!
//! **Who must register**:
//! - Cryptoasset exchange providers
//! - Custodian wallet providers
//! - Operating in/from UK
//!
//! **Registration process**:
//! - Apply to FCA via Connect portal
//! - Demonstrate fit and proper (directors, beneficial owners, compliance officers)
//! - Provide evidence of AML/CTF systems and controls
//! - Appoint Money Laundering Reporting Officer (MLRO)
//! - Pay registration fee (£2,000-10,000 depending on size)
//!
//! **Fit and proper test**:
//! FCA conducts rigorous assessment:
//! - Criminal record checks
//! - Financial crime history
//! - Business model sustainability
//! - Competence and capability
//!
//! **Rejection rate**: ~80% of applications rejected (2020-2022). FCA has very high standards.
//!
//! ### AML/CTF Requirements (MLR 2017 Part 2-3)
//!
//! **Customer Due Diligence (CDD)**:
//! - Identity verification (name, date of birth, address)
//! - Source of funds assessment
//! - Purpose of business relationship
//! - Ongoing monitoring
//!
//! **Enhanced Due Diligence (EDD)** required for:
//! - Politically Exposed Persons (PEPs)
//! - High-risk countries (FATF list)
//! - Large/complex transactions
//! - Anonymous/privacy coins (Monero, Zcash)
//!
//! **Suspicious Activity Reports (SARs)**:
//! - File to National Crime Agency (NCA)
//! - When knowledge/suspicion of money laundering
//! - Common red flags:
//!   - Structuring (multiple small transactions to avoid reporting threshold)
//!   - P2P trades avoiding exchanges
//!   - Use of mixers/tumblers
//!   - Offshore exchanges in high-risk jurisdictions
//!   - No clear source of funds
//!
//! ### Travel Rule (MLR 2017 Reg 14A)
//!
//! **FATF Recommendation 16** implemented in UK:
//!
//! **Threshold**: ≥£1,000 (or equivalent)
//!
//! **Information required**:
//! - **Originator**: Name, account/wallet address
//! - **Beneficiary**: Name, account/wallet address
//!
//! **Transmission**: Must accompany cryptoasset transfer
//!
//! **Purpose**: Ensure crypto transfers traceable like traditional bank transfers.
//!
//! **Challenges**:
//! - Technical implementation (on-chain vs off-chain)
//! - Unhosted wallets (self-custody)
//! - Cross-border coordination
//! - Privacy vs compliance balance
//!
//! ## Tax Treatment (HMRC Cryptoassets Manual)
//!
//! ### HMRC Position
//!
//! **Cryptoassets are property** (not currency) for UK tax purposes.
//!
//! ### Capital Gains Tax (CGT)
//!
//! **Chargeable disposal events**:
//! - Selling cryptoassets for fiat
//! - Exchanging one cryptoasset for another
//! - Using cryptoassets to buy goods/services
//! - Gifting cryptoassets (other than to spouse)
//!
//! **CGT calculation**:
//! - Acquisition cost (including fees)
//! - Disposal proceeds (including fees)
//! - Gain = Proceeds - Cost
//! - Annual exempt amount: £3,000 (2024/25)
//! - CGT rate: 10% (basic rate) or 20% (higher rate)
//!
//! **Pooling rules** (same-day and 30-day matching):
//! - Same-day: Disposals matched with acquisitions on same day
//! - 30-day: Disposals matched with acquisitions within next 30 days
//! - Section 104 pool: Remaining tokens pooled with average cost
//!
//! ### Income Tax
//!
//! **Taxable income from cryptoassets**:
//! - Mining rewards (when received)
//! - Staking rewards
//! - Airdrops (if actively sought)
//! - Employment income paid in crypto
//! - Trading profits (if trading activity)
//!
//! **Income Tax rates** (2024/25):
//! - Basic rate (£12,571-£50,270): 20%
//! - Higher rate (£50,271-£125,140): 40%
//! - Additional rate (>£125,140): 45%
//!
//! ### Record-Keeping
//!
//! **HMRC requirements**:
//! - Type of cryptoasset
//! - Date of transaction
//! - Quantity
//! - Value in GBP at time of transaction
//! - Transaction fees
//! - Counterparty details (if known)
//!
//! **Retention**: 6 years from end of tax year
//!
//! ## International Context
//!
//! ### UK's Approach vs Other Jurisdictions
//!
//! **UK: Pragmatic regulation**
//! - Sector-specific rules (not blanket bans)
//! - FCA PS19/22 classification provides clarity
//! - High AML/CTF standards (80% rejection rate)
//! - Embracing innovation (sandbox, TechSprints)
//!
//! **USA: Fragmented regulation**
//! - SEC (securities), CFTC (commodities), FinCEN (AML)
//! - Howey Test for securities
//! - Aggressive enforcement (e.g., vs Ripple, Binance)
//!
//! **EU: MiCA Regulation**
//! - Markets in Crypto-Assets Regulation (MiCA) effective 2024-2025
//! - Comprehensive crypto framework
//! - Stablecoin reserve requirements
//! - Crypto-asset service provider licensing
//!
//! **Singapore: Innovation-friendly**
//! - Payment Services Act 2019
//! - MAS licensing for crypto services
//! - Pragmatic AML/CTF approach
//!
//! ### FATF Recommendations
//!
//! **Financial Action Task Force** (FATF) sets global AML/CTF standards.
//!
//! **Recommendation 15**: Virtual assets and VASPs
//! - Virtual Asset Service Providers (VASPs) = crypto exchanges, custodians
//! - Must be licensed/registered
//! - AML/CTF requirements equivalent to traditional finance
//!
//! **Recommendation 16**: Travel Rule
//! - ≥$1,000 (or equivalent) threshold
//! - Originator and beneficiary information must accompany transfer
//!
//! **UK compliance**: UK fully implements FATF Recommendations via MLR 2017 Reg 14A.
//!
//! # Compliance Checklist
//!
//! ## For Cryptoasset Issuers
//!
//! - [ ] Classify token correctly (security, e-money, utility, stablecoin)
//! - [ ] If security token: Obtain FCA authorization, publish prospectus if public offer
//! - [ ] If e-money token: Obtain EMI authorization
//! - [ ] If stablecoin: Ensure 1:1 reserve backing, segregation, audit, redemption rights
//! - [ ] Financial promotions: Obtain FCA-authorized approval (from Oct 8, 2023)
//! - [ ] Include prominent risk warnings
//! - [ ] Fair, clear and not misleading
//!
//! ## For Cryptoasset Exchange Providers
//!
//! - [ ] Register with FCA under MLR 2017 Reg 14A
//! - [ ] Implement AML/CTF policies and procedures
//! - [ ] Appoint Money Laundering Reporting Officer (MLRO)
//! - [ ] Perform Customer Due Diligence (CDD) for all customers
//! - [ ] Enhanced Due Diligence (EDD) for PEPs and high-risk customers
//! - [ ] Implement Travel Rule (≥£1,000 transfers)
//! - [ ] Sanctions screening against OFSI/UN lists
//! - [ ] File Suspicious Activity Reports (SARs) to NCA when suspicious
//! - [ ] Maintain records for 5 years
//! - [ ] Staff training on financial crime
//!
//! ## For Individual Crypto Investors
//!
//! - [ ] Keep detailed records of all transactions (HMRC requirement)
//! - [ ] Calculate Capital Gains Tax on disposals
//! - [ ] Report income from mining, staking, airdrops
//! - [ ] Self Assessment tax return if gains exceed £3,000 annual exempt amount
//! - [ ] Beware of fraudulent promotions (check FCA Warning List)
//! - [ ] Only use FCA-registered exchanges for UK operations
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::financial_services::cryptoassets::*;
//! use chrono::NaiveDate;
//!
//! // Classify token using Howey Test
//! let assessment = SecurityTokenAssessment {
//!     token_name: "InvestToken".to_string(),
//!     investment_of_money: true,
//!     common_enterprise: true,
//!     expectation_of_profit: true,
//!     profit_from_others_efforts: true,
//!     equity_rights: false,
//!     debt_rights: false,
//!     derivative_characteristics: false,
//! };
//!
//! if assessment.is_likely_security() {
//!     println!("Token is likely a security - FCA authorization required");
//! }
//!
//! // Validate stablecoin compliance (FSMA 2023)
//! let stablecoin = Stablecoin {
//!     token_name: "GBPT".to_string(),
//!     issuer: "UK Stablecoin Ltd".to_string(),
//!     peg: StablecoinPeg::FiatCurrency {
//!         currency: "GBP".to_string(),
//!     },
//!     reserve_backing: ReserveBacking {
//!         backing_ratio: 1.0,
//!         assets_description: "Bank deposits at UK banks".to_string(),
//!         custody_arrangements: "Segregated account".to_string(),
//!         segregated: true,
//!     },
//!     reserve_audited: true,
//!     audit_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
//!     redemption_rights: RedemptionRights {
//!         redeemable: true,
//!         redemption_fee: None,
//!         redemption_timeframe: Some("1 business day".to_string()),
//!     },
//!     fca_authorized: true,
//!     authorization_type: Some("E-Money Institution".to_string()),
//! };
//!
//! validate_stablecoin(&stablecoin)?;
//!
//! // Validate cryptoasset promotion (post-Oct 8, 2023)
//! let promotion = CryptoassetPromotion {
//!     promotion_content: "Invest in Bitcoin - Warning: High risk investment".to_string(),
//!     promotion_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     medium: PromotionMedium::SocialMedia,
//!     approved_by_authorized_person: true,
//!     approver_frn: Some("123456".to_string()),
//!     risk_warning_included: true,
//!     risk_warning_prominent: true,
//!     fair_clear_not_misleading: true,
//!     target_audience: PromotionAudience::Retail,
//! };
//!
//! validate_cryptoasset_promotion(&promotion)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{CryptoassetsError, Result};
pub use types::{
    CryptoassetClassification, CryptoassetExchangeProvider, CryptoassetPromotion,
    PromotionAudience, PromotionMedium, RedemptionRights, ReserveBacking, SecurityTokenAssessment,
    SecurityTokenType, Stablecoin, StablecoinPeg,
};
pub use validator::{
    validate_cryptoasset_classification, validate_cryptoasset_promotion,
    validate_exchange_provider, validate_security_token_assessment, validate_stablecoin,
};
