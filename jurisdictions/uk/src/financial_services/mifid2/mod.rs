//! MiFID II Module (UK Markets in Financial Instruments Directive II)
//!
//! Comprehensive implementation of UK MiFID II regulation for investment services.
//!
//! # Key Legislation
//!
//! ## UK MiFID II (Retained EU Law)
//!
//! UK MiFID II is the Markets in Financial Instruments Directive II (2014/65/EU) as retained
//! in UK law post-Brexit. It regulates investment services and activities.
//!
//! ### Scope of MiFID II
//!
//! MiFID II applies to:
//! - **Investment firms**: Firms providing investment services (dealing, portfolio management, advice)
//! - **Credit institutions**: Banks providing investment services
//! - **Trading venues**: Regulated Markets (RM), Multilateral Trading Facilities (MTF), Organized Trading Facilities (OTF)
//! - **Data reporting services providers**: Approved Publication Arrangements (APA), Consolidated Tape Providers (CTP)
//!
//! ### MiFID II vs MiFID I (2007)
//!
//! Major enhancements in MiFID II:
//! 1. **Expanded scope**: Includes commodities, derivatives, new trading venues (OTF)
//! 2. **Transparency**: Pre/post-trade transparency for equity and non-equity instruments
//! 3. **Transaction reporting**: Enhanced reporting to regulators (65 fields)
//! 4. **Product governance**: Manufacturers/distributors must define target markets
//! 5. **Research unbundling**: Investment research must be paid separately from execution
//! 6. **Best execution**: Detailed reporting on execution venues
//! 7. **Algorithmic trading**: Requirements for high-frequency trading firms
//!
//! ## FCA COBS (Conduct of Business Sourcebook)
//!
//! COBS implements MiFID II conduct of business requirements in UK.
//!
//! ### COBS 3: Client Categorization
//!
//! Three client categories with different protections:
//!
//! **1. Retail Client** (highest protection):
//! - Default category for individuals and small businesses
//! - Full MiFID II protections apply
//! - Suitability assessment required for personal recommendations (COBS 9)
//! - Appropriateness assessment for non-advised sales of complex products (COBS 10)
//!
//! **2. Professional Client** (intermediate protection):
//! - Per se professional: Authorized firms, large undertakings (balance sheet >€20m, turnover >€40m, own funds >€2m)
//! - Elective professional: Clients who opt up (must meet criteria and receive warning)
//! - Reduced protections compared to retail
//!
//! **3. Eligible Counterparty** (minimal protection):
//! - Authorized firms, central banks, governments, international organizations
//! - Minimal COBS protections (mainly transaction reporting)
//!
//! ### COBS 9: Suitability (Personal Recommendations)
//!
//! When providing **personal recommendations** or managing portfolios for retail clients:
//!
//! **1. Information gathering** (COBS 9.2):
//! - **Knowledge and experience**: Investments types, volume, frequency, education
//! - **Financial situation**: Income, assets, regular financial commitments
//! - **Investment objectives**: Time horizon, risk tolerance, purpose (growth/income)
//!
//! **2. Suitability assessment** (COBS 9.4):
//! - Assess whether investment is suitable given client information
//! - Must consider diversification (single product may be unsuitable even if client can afford it)
//!
//! **3. Suitability report** (COBS 9.4):
//! - Must explain why recommendation is suitable
//! - Highlight risks
//! - Provide before transaction execution (or immediately after for portfolio management)
//!
//! ### COBS 10: Appropriateness (Non-Advised Sales)
//!
//! For **non-advised sales** of **complex products** to retail clients:
//!
//! **Complex products** include:
//! - Derivatives
//! - Structured products
//! - UCITS with complex features
//! - Investment trusts (closed-end funds)
//!
//! **Non-complex products** (appropriateness not required):
//! - Shares admitted to regulated market
//! - Money market instruments
//! - Bonds (non-convertible, non-subordinated)
//! - Mainstream UCITS
//!
//! **Appropriateness assessment** (COBS 10.2):
//! - Assess client's knowledge and experience
//! - If client lacks understanding, must warn client
//! - If client insists after warning, can proceed (documented)
//!
//! ### COBS 11.2A: Best Execution
//!
//! Firms must take **all sufficient steps** to obtain best possible result for clients when
//! executing orders (MiFID II Article 27).
//!
//! **Best execution factors** (COBS 11.2A.7R):
//! 1. **Price**: Most important for retail clients
//! 2. **Costs**: Execution fees, clearing, settlement
//! 3. **Speed**: Execution speed
//! 4. **Likelihood of execution and settlement**
//! 5. **Size**: Order size
//! 6. **Nature**: Order characteristics
//! 7. **Any other relevant consideration**
//!
//! **Best execution policy** (COBS 11.2A.2R):
//! - Firms must establish and implement execution policy
//! - Must identify venues where firm places significant reliance
//! - Must obtain prior consent from clients
//!
//! **Annual reporting** (COBS 11.2A.28R):
//! - Firms must publish **top 5 execution venues** by volume for each asset class
//! - Must include quality of execution assessment
//! - Report published on website by 30 April following year
//!
//! ### COBS 16A: Product Governance
//!
//! MiFID II Article 16(3) introduced product governance requirements.
//!
//! #### Manufacturers (COBS 16A.1)
//!
//! **Product approval process** (COBS 16A.1.5R):
//! - Must establish product approval committee
//! - Products designed to meet needs of identified target market
//! - Distribution strategy compatible with target market
//!
//! **Target market definition** (COBS 16A.1.4R):
//! Must specify for each product:
//! 1. **Type of clients**: Retail, professional, eligible counterparty
//! 2. **Knowledge and experience**: Basic, informed, advanced
//! 3. **Financial situation**: Ability to bear losses (none, limited, full)
//! 4. **Risk tolerance**: Low, medium, high
//! 5. **Objectives and needs**: Capital preservation, growth, income, etc.
//! 6. **Time horizon**: Short (<3y), medium (3-7y), long (>7y)
//!
//! **Notification to distributors** (COBS 16A.1.9R):
//! - Manufacturers must provide distributors with all information on product and target market
//! - Enable distributors to understand and recommend product appropriately
//!
//! #### Distributors (COBS 16A.2)
//!
//! **Distribution strategy** (COBS 16A.2.1R):
//! - Distributors must obtain target market information from manufacturers
//! - Distribution strategy must be compatible with target market
//! - Sales outside target market require justification
//!
//! **Product review** (COBS 16A.2.5R):
//! - Distributors must regularly review products they offer
//! - Identify events that could materially affect risk/return
//! - Consider whether product remains consistent with target market
//!
//! ### COBS 2.3B: Research Unbundling (Inducements)
//!
//! MiFID II Article 24(8) prohibits investment firms managing portfolios from accepting
//! inducements from third parties (including bundled research).
//!
//! **Inducements ban** (COBS 2.3A.2R):
//! - Firms managing portfolios/providing independent advice cannot accept:
//!   - Fees, commissions, or monetary benefits from third parties
//!   - In relation to investment services to clients
//! - **Exception**: Minor non-monetary benefits (market commentary, research on macro-economics)
//!
//! **Research payment accounts** (COBS 2.3B.4R):
//! Investment research must be paid either:
//! 1. From firm's own resources (P&L charge), or
//! 2. From separate **research payment account** (RPA) funded by:
//!    - Specific research charge to client (separate from execution commission), or
//!    - Payments from firm's own resources
//!
//! **Research budget** (COBS 2.3B.5R):
//! - Firm must set and review total research budget at least annually
//! - Based on reasonable assessment of need for third-party research
//! - Controlled by firm, not portfolio managers
//! - Not linked to volume/value of transactions
//!
//! **Client disclosure** (COBS 2.3B.9R):
//! - Firms must provide clients with summary information on RPA at least annually:
//!   - Total costs paid from RPA
//!   - Proportion paid by each client/group
//!   - Amount budgeted for coming year
//!
//! ## UK MAR (Market Abuse Regulation)
//!
//! UK MAR is the Market Abuse Regulation (EU) 596/2014 as retained in UK law.
//!
//! ### Article 14: Insider Dealing Prohibition
//!
//! **Insider dealing** is prohibited:
//! - Using inside information to acquire/dispose of financial instruments
//! - Recommending/inducing another to trade on inside information
//! - Unlawfully disclosing inside information
//!
//! **Inside information** (Article 7):
//! - Specific, precise information
//! - Not publicly available
//! - Relating to issuer/financial instruments
//! - Which, if public, would likely have significant effect on price
//!
//! **Penalties**:
//! - Criminal: Up to 7 years imprisonment (Criminal Justice Act 1993)
//! - Civil: FCA can impose unlimited fine (FSMA 2000 s.123)
//!
//! ### Article 15: Market Manipulation Prohibition
//!
//! **Market manipulation** is prohibited:
//! - Transactions giving false/misleading signals on supply/demand/price
//! - Artificial price levels
//! - Disseminating false/misleading information
//! - Benchmark manipulation (e.g., LIBOR rigging)
//!
//! ### Article 16: Suspicious Transaction Reporting (STR)
//!
//! **Reporting obligation** (Article 16(2)):
//! - Persons professionally arranging/executing transactions must notify FCA without delay
//! - When they reasonably suspect transaction might constitute:
//!   - Insider dealing (Article 14)
//!   - Market manipulation (Article 15)
//!
//! **Detection systems** (Article 16(1)):
//! - Firms must establish effective arrangements, systems and procedures
//! - To detect and report suspicious orders and transactions
//!
//! ### Article 18: Insider Lists
//!
//! **Insider list requirement**:
//! - Issuers (or persons acting on their behalf) must draw up lists
//! - Of persons with access to inside information
//! - Relating directly or indirectly to issuer
//!
//! **Content** (Article 18(3)):
//! - Identity of person with access
//! - Reason for being on list
//! - Date/time obtained access
//! - Date/time list was drawn up
//!
//! **Updates**:
//! - Must be promptly updated when circumstances change
//! - Submitted to FCA upon request
//!
//! **Retention**: Retain for at least 5 years after drawn up or updated
//!
//! ## FCA SUP 17: Transaction Reporting
//!
//! SUP 17 implements MiFID II Article 26 transaction reporting in UK.
//!
//! ### Reporting Obligation (SUP 17.1)
//!
//! **Who must report**:
//! - Investment firms executing transactions in:
//!   - Financial instruments admitted to trading on UK trading venue
//!   - Or for which prospectus has been published
//!
//! **Reporting deadline**: T+1
//! - By close of following working day after execution
//!
//! ### Transaction Report Content (MiFID II RTS 22)
//!
//! **65 fields** required, including:
//!
//! **Instrument identification**:
//! - ISIN (International Securities Identification Number)
//! - Classification (CFI code)
//!
//! **Counterparties**:
//! - Buyer identification: LEI (Legal Entity Identifier) or national ID
//! - Seller identification: LEI or national ID
//! - Investment decision: LEI or national ID (who made decision)
//! - Executing trader: National ID (who executed)
//!
//! **Trade details**:
//! - Quantity
//! - Price
//! - Notional amount
//! - Venue: MIC (Market Identifier Code) or "XOFF" (off-venue)
//! - Country of branch
//!
//! **Timing**:
//! - Trading date/time (UTC)
//! - Trading capacity (principal/agent/matched principal)
//!
//! ### Legal Entity Identifier (LEI)
//!
//! **LEI format**:
//! - 20-character alphanumeric code
//! - Issued by GLEIF (Global Legal Entity Identifier Foundation)
//! - Example: 213800EBPD2GY84SVP41
//!
//! **LEI requirement**:
//! - All legal entities executing transactions must have LEI
//! - Must be renewed annually
//! - Non-compliance: Cannot execute transactions
//!
//! ### Market Identifier Code (MIC)
//!
//! **MIC format**:
//! - 4-character code identifying trading venue
//! - ISO 10383 standard
//! - Examples:
//!   - XLON: London Stock Exchange
//!   - BATE: Cboe Europe (formerly BATS)
//!   - CHIX: Cboe Europe (Chi-X)
//!   - XOFF: Off-venue (OTC)
//!
//! ## International Context
//!
//! ### London as Global Financial Centre
//!
//! London remains a leading global financial centre despite Brexit:
//! - Over 250 foreign banks
//! - 40% of global foreign exchange trading
//! - Leading derivatives trading centre
//! - Major asset management hub (£11 trillion AUM)
//!
//! ### Post-Brexit Changes
//!
//! **Regulatory divergence**:
//! - UK can now modify MiFID II (no longer bound by EU law)
//! - FCA's Wholesale Markets Review (2021-2024): Proposed reforms
//! - Share Trading Obligation (STO): Removed (allows UK shares to trade on EU venues)
//! - Derivative Trading Obligation (DTO): Modified
//!
//! **Equivalence**:
//! - UK-EU equivalence decisions for investment services
//! - Not comprehensive (unlike passporting)
//! - Allows some cross-border activities
//!
//! ### ESMA Guidelines (Retained)
//!
//! European Securities and Markets Authority (ESMA) guidelines mostly retained:
//! - MiFID II Q&A: Interpretive guidance
//! - RTS/ITS: Regulatory/Implementing Technical Standards
//! - UK may diverge over time
//!
//! ## Compliance Checklist
//!
//! ### Transaction Reporting (T+1)
//! - [ ] All transactions in scope reported to FCA by T+1
//! - [ ] All 65 fields complete (ISIN, LEI, quantity, price, MIC)
//! - [ ] LEI obtained and renewed annually
//! - [ ] Transaction reporting system tested
//!
//! ### Product Governance
//! - [ ] Product approval committee established
//! - [ ] Target market defined for each product (6 dimensions)
//! - [ ] Distribution strategy compatible with target market
//! - [ ] Distributors notified of target market
//! - [ ] Product reviews performed regularly
//!
//! ### Research Unbundling
//! - [ ] Investment research paid from RPA or own P&L (not client commissions)
//! - [ ] Research budget set and reviewed annually
//! - [ ] Client disclosure provided at least annually
//! - [ ] Research charges separated from execution costs
//!
//! ### Best Execution
//! - [ ] Best execution policy established and implemented
//! - [ ] Client consent to execution policy obtained
//! - [ ] Top 5 execution venues report published by 30 April
//! - [ ] Quality of execution monitored
//!
//! ### Market Abuse
//! - [ ] Suspicious transaction reporting systems in place
//! - [ ] STRs submitted to FCA without delay when suspicion arises
//! - [ ] Insider lists maintained for inside information events
//! - [ ] Market abuse training provided to staff
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::financial_services::mifid2::*;
//! use chrono::NaiveDate;
//!
//! // Transaction reporting (T+1 deadline)
//! let report = TransactionReport {
//!     report_id: "TR123456".to_string(),
//!     transaction_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
//!     instrument_isin: "GB0002374006".to_string(),
//!     buyer_lei: "213800EBPD2GY84SVP41".to_string(),
//!     seller_lei: "529900T8BM49AURSDO55".to_string(),
//!     executing_entity_lei: "213800EBPD2GY84SVP41".to_string(),
//!     quantity: 10000.0,
//!     price: 125.50,
//!     currency: "GBP".to_string(),
//!     venue_mic: "XLON".to_string(),
//!     reported_to_fca: true,
//!     reporting_deadline: NaiveDate::from_ymd_opt(2024, 1, 16).unwrap(),
//! };
//!
//! let current_date = NaiveDate::from_ymd_opt(2024, 1, 16).unwrap();
//! validate_transaction_report(&report, current_date)?;
//!
//! // Product governance - target market definition
//! let governance = ProductGovernance {
//!     product_name: "UK Equity Growth Fund".to_string(),
//!     manufacturer: "Asset Management Ltd".to_string(),
//!     target_market: TargetMarket {
//!         client_categories: vec![ClientCategory::Retail],
//!         knowledge_level: KnowledgeLevel::Informed,
//!         risk_tolerance: RiskTolerance::Medium,
//!         min_investment_amount: Some(1000.0),
//!         ability_to_bear_losses: AbilityToBearLosses::Limited,
//!         time_horizon: TimeHorizon::MediumTerm,
//!     },
//!     approved_by_committee: true,
//!     approval_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
//!     distribution_channels: vec!["Platform".to_string(), "Direct".to_string()],
//!     distributor_notifications_sent: true,
//! };
//!
//! validate_product_governance(&governance)?;
//!
//! // Research unbundling
//! let research_payment = ResearchPayment {
//!     research_provider: "Independent Research Ltd".to_string(),
//!     payment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     amount_gbp: 25000.0,
//!     paid_from_research_account: true,
//!     research_budget_approved: true,
//!     disclosed_to_clients: true,
//! };
//!
//! validate_research_payment(&research_payment)?;
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{Mifid2Error, Result};
pub use types::{
    AbilityToBearLosses, BestExecutionReport, ClientCategory, ExecutionVenue, KnowledgeLevel,
    MifidFirmType, ProductGovernance, ResearchPayment, RiskTolerance, TargetMarket, TimeHorizon,
    TransactionReport,
};
pub use validator::{
    validate_best_execution_report, validate_product_governance, validate_research_payment,
    validate_target_market_match, validate_transaction_report,
};
