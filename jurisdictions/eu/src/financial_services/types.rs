//! Core types for EU Financial Services regulation
//!
//! This module covers MiFID II and PSD2 regulations.

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

use crate::shared::MemberState;

// ============================================================================
// MiFID II (Markets in Financial Instruments Directive II)
// ============================================================================

/// Investment services under MiFID II (Annex I Section A)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum InvestmentService {
    /// Reception and transmission of orders
    ReceptionAndTransmission,
    /// Execution of orders on behalf of clients
    OrderExecution,
    /// Dealing on own account
    DealingOnOwnAccount,
    /// Portfolio management
    PortfolioManagement,
    /// Investment advice
    InvestmentAdvice,
    /// Underwriting of financial instruments
    Underwriting,
    /// Placing of financial instruments
    Placing,
    /// Operation of multilateral trading facility (MTF)
    MtfOperation,
    /// Operation of organized trading facility (OTF)
    OtfOperation,
}

/// Financial instruments under MiFID II (Annex I Section C)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum FinancialInstrument {
    /// Transferable securities
    TransferableSecurities,
    /// Money market instruments
    MoneyMarketInstruments,
    /// Units in collective investment undertakings
    CollectiveInvestmentUnits,
    /// Options, futures, swaps, and other derivatives
    Derivatives {
        /// Underlying asset type
        underlying: String,
    },
    /// Emission allowances
    EmissionAllowances,
}

/// Client categorization under MiFID II (Article 4)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ClientCategory {
    /// Retail client - highest protection
    Retail,
    /// Professional client - reduced protection
    Professional {
        /// Whether client requested professional treatment
        elective: bool,
    },
    /// Eligible counterparty - minimal protection
    EligibleCounterparty,
}

/// Best execution criteria (Article 27)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct BestExecutionPolicy {
    /// Execution factors considered
    pub execution_factors: Vec<ExecutionFactor>,
    /// Execution venues used
    pub execution_venues: Vec<String>,
    /// Relative importance of factors
    pub factor_weighting: String,
    /// How policy achieves best execution
    pub methodology: String,
    /// Monitoring and review process
    pub monitoring_process: String,
}

/// Execution factors for best execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ExecutionFactor {
    /// Price
    Price,
    /// Costs
    Costs,
    /// Speed of execution
    Speed,
    /// Likelihood of execution and settlement
    Likelihood,
    /// Size of order
    Size,
    /// Nature of order
    Nature,
}

/// Product governance under MiFID II (Article 16)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ProductGovernance {
    /// Target market definition
    pub target_market: TargetMarket,
    /// Distribution strategy
    pub distribution_strategy: String,
    /// Product testing and scenario analysis
    pub product_testing: bool,
    /// Monitoring and review process
    pub monitoring_review: String,
}

/// Target market for financial product
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TargetMarket {
    /// Client category
    pub client_category: ClientCategory,
    /// Knowledge and experience level
    pub knowledge_experience: KnowledgeLevel,
    /// Financial situation (ability to bear losses)
    pub financial_situation: FinancialSituation,
    /// Risk tolerance
    pub risk_tolerance: RiskTolerance,
    /// Investment objectives
    pub objectives: Vec<InvestmentObjective>,
}

/// Client knowledge and experience level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum KnowledgeLevel {
    /// Basic or no knowledge
    Basic,
    /// Informed - some understanding
    Informed,
    /// Advanced - sophisticated understanding
    Advanced,
}

/// Financial situation assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum FinancialSituation {
    /// Can bear limited losses only
    LimitedLossTolerance,
    /// Can bear moderate losses
    ModerateLossTolerance,
    /// Can bear full capital loss
    FullCapitalLoss,
}

/// Risk tolerance level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RiskTolerance {
    /// Risk-averse
    Low,
    /// Neutral
    Medium,
    /// Risk-seeking
    High,
}

/// Investment objectives
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum InvestmentObjective {
    /// Capital preservation
    CapitalPreservation,
    /// Income generation
    IncomeGeneration,
    /// Growth
    Growth,
    /// Hedging
    Hedging,
    /// Speculation
    Speculation,
}

/// Inducements and conflicts of interest (Article 23-24)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InducementPolicy {
    /// Whether inducements accepted/paid
    pub inducements_accepted: bool,
    /// Disclosure to clients
    pub disclosure: String,
    /// Quality enhancement test passed
    pub quality_enhancement: bool,
    /// Conflicts of interest identification
    pub conflicts_identified: Vec<String>,
    /// Conflicts mitigation measures
    pub conflicts_mitigation: Vec<String>,
}

// ============================================================================
// PSD2 (Payment Services Directive 2)
// ============================================================================

/// Payment services under PSD2 (Annex I)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PaymentService {
    /// Services enabling cash deposits/withdrawals
    CashServices,
    /// Execution of payment transactions
    PaymentExecution {
        /// Type of payment
        payment_type: PaymentType,
    },
    /// Issuing payment instruments
    PaymentInstrumentIssuing,
    /// Acquiring payment transactions
    Acquiring,
    /// Money remittance
    MoneyRemittance,
    /// Payment initiation services (PIS)
    PaymentInitiation,
    /// Account information services (AIS)
    AccountInformation,
}

/// Payment transaction types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PaymentType {
    /// Credit transfer
    CreditTransfer,
    /// Direct debit
    DirectDebit,
    /// Card payment
    CardPayment,
}

/// Strong Customer Authentication (SCA) requirements (Article 97)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StrongCustomerAuthentication {
    /// Authentication based on 2+ elements from different categories
    pub authentication_elements: Vec<AuthenticationElement>,
    /// Dynamic linking for payment transactions
    pub dynamic_linking: bool,
    /// Whether exemption applies
    pub exemption: Option<ScaExemption>,
}

/// Authentication element categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum AuthenticationElement {
    /// Knowledge - something only user knows (password, PIN)
    Knowledge,
    /// Possession - something only user has (token, mobile device)
    Possession,
    /// Inherence - something user is (biometric)
    Inherence,
}

/// SCA exemptions (RTS on SCA)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ScaExemption {
    /// Low-value transaction (< €30)
    LowValue,
    /// Recurring transaction with same amount/payee
    RecurringTransaction,
    /// Trusted beneficiary
    TrustedBeneficiary,
    /// Transaction risk analysis indicates low risk
    LowRisk,
    /// Unattended terminal for transport/parking
    UnattendedTerminal,
}

/// Payment initiation service provider (PISP)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct PaymentInitiationProvider {
    /// Provider name
    pub name: String,
    /// Authorization by competent authority
    pub authorized: bool,
    /// Member State of authorization
    pub home_member_state: MemberState,
    /// Whether passporting into other Member States
    pub passporting: Vec<MemberState>,
    /// API access to account servicing payment service providers
    pub api_access: bool,
}

/// Account information service provider (AISP)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AccountInformationProvider {
    /// Provider name
    pub name: String,
    /// Authorization by competent authority
    pub authorized: bool,
    /// Member State of authorization
    pub home_member_state: MemberState,
    /// Whether passporting into other Member States
    pub passporting: Vec<MemberState>,
    /// API access to account servicing payment service providers
    pub api_access: bool,
    /// User consent obtained
    pub user_consent: bool,
}

/// Passporting rights (Article 28 MiFID II / Article 28 PSD2)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Passport {
    /// Home Member State (where authorized)
    pub home_member_state: MemberState,
    /// Host Member States (where passporting)
    pub host_member_states: Vec<MemberState>,
    /// Services covered by passport
    pub services: Vec<String>,
    /// Notification to competent authority
    pub notification_date: DateTime<Utc>,
    /// Whether establishment of branch or freedom to provide services
    pub branch_established: bool,
}

/// Transaction reporting under MiFID II (Article 26)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TransactionReport {
    /// Unique transaction identifier
    pub transaction_id: String,
    /// Trading date and time
    pub trading_datetime: DateTime<Utc>,
    /// Financial instrument
    pub instrument: FinancialInstrument,
    /// Buy/sell indicator
    pub buy_sell: BuySellIndicator,
    /// Quantity
    pub quantity: f64,
    /// Price
    pub price: f64,
    /// Client identification
    pub client_id: String,
    /// Venue of execution
    pub venue: String,
}

/// Buy or sell indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum BuySellIndicator {
    Buy,
    Sell,
}

/// Open banking / API access (PSD2 Article 67)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct OpenBankingApi {
    /// API endpoint URL
    pub api_endpoint: String,
    /// API documentation publicly available
    pub documentation_available: bool,
    /// Testing facility provided
    pub testing_facility: bool,
    /// Obstacle reporting mechanism
    pub obstacle_reporting: bool,
    /// Dedicated interface for TPPs
    pub dedicated_interface: bool,
    /// Performance statistics published
    pub performance_stats_published: bool,
}

/// Third-party provider (TPP) under PSD2
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ThirdPartyProvider {
    /// Payment initiation service provider
    PaymentInitiation(PaymentInitiationProvider),
    /// Account information service provider
    AccountInformation(AccountInformationProvider),
}

/// Conduct of business rules (MiFID II Article 24-25)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ConductOfBusiness {
    /// Acting honestly, fairly, and professionally
    pub act_in_best_interest: bool,
    /// Information to clients (fair, clear, not misleading)
    pub information_quality: InformationQuality,
    /// Suitability/appropriateness assessment
    pub assessment: ClientAssessment,
    /// Best execution obligation
    pub best_execution: Option<BestExecutionPolicy>,
    /// Client order handling
    pub order_handling_policy: String,
}

/// Quality of information provided to clients
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InformationQuality {
    /// Fair
    pub fair: bool,
    /// Clear
    pub clear: bool,
    /// Not misleading
    pub not_misleading: bool,
}

/// Client assessment for suitability/appropriateness
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ClientAssessment {
    /// Suitability assessment (for investment advice/portfolio management)
    Suitability {
        /// Knowledge and experience assessed
        knowledge_experience: bool,
        /// Financial situation assessed
        financial_situation: bool,
        /// Investment objectives assessed
        investment_objectives: bool,
    },
    /// Appropriateness assessment (for other services)
    Appropriateness {
        /// Knowledge and experience assessed
        knowledge_experience: bool,
    },
    /// No assessment (execution-only for non-complex instruments)
    ExecutionOnly,
}
