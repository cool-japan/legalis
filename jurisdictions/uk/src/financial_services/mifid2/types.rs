//! MiFID II Types (UK MiFID II, COBS, MAR)

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// MiFID firm type
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MifidFirmType {
    /// Investment firm
    InvestmentFirm,
    /// Credit institution providing investment services
    CreditInstitution,
    /// Systematic Internaliser (SI)
    SystematicInternaliser,
    /// Regulated Market (RM)
    RegulatedMarket,
    /// Multilateral Trading Facility (MTF)
    MultilateralTradingFacility,
    /// Organized Trading Facility (OTF)
    OrganizedTradingFacility,
}

/// Transaction report (MiFID II Article 26, FCA SUP 17)
///
/// Investment firms must report transactions to FCA by end of following working day (T+1).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransactionReport {
    /// Report identifier
    pub report_id: String,
    /// Transaction date
    pub transaction_date: NaiveDate,
    /// Instrument identification (ISIN)
    pub instrument_isin: String,
    /// Buyer identification (LEI)
    pub buyer_lei: String,
    /// Seller identification (LEI)
    pub seller_lei: String,
    /// Executing entity (LEI)
    pub executing_entity_lei: String,
    /// Quantity
    pub quantity: f64,
    /// Price
    pub price: f64,
    /// Currency (ISO 4217)
    pub currency: String,
    /// Trading venue (MIC code)
    pub venue_mic: String,
    /// Whether reported to FCA
    pub reported_to_fca: bool,
    /// Reporting deadline (T+1)
    pub reporting_deadline: NaiveDate,
}

impl TransactionReport {
    /// Check if reported within T+1 deadline
    pub fn is_timely_reported(&self, current_date: NaiveDate) -> bool {
        self.reported_to_fca && current_date <= self.reporting_deadline
    }

    /// Calculate reporting deadline (T+1)
    pub fn calculate_deadline(transaction_date: NaiveDate) -> NaiveDate {
        // Add 1 business day (simplified - real implementation needs business day calendar)
        transaction_date
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap_or(transaction_date)
    }
}

/// Product governance (COBS 16A, MiFID II Article 16(3))
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProductGovernance {
    /// Product name
    pub product_name: String,
    /// Manufacturer firm
    pub manufacturer: String,
    /// Target market definition
    pub target_market: TargetMarket,
    /// Product approval committee approval
    pub approved_by_committee: bool,
    /// Approval date
    pub approval_date: Option<NaiveDate>,
    /// Distribution channels
    pub distribution_channels: Vec<String>,
    /// Distributors notified
    pub distributor_notifications_sent: bool,
}

/// Target market definition (COBS 16A.1)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TargetMarket {
    /// Client categories
    pub client_categories: Vec<ClientCategory>,
    /// Knowledge and experience level
    pub knowledge_level: KnowledgeLevel,
    /// Risk tolerance
    pub risk_tolerance: RiskTolerance,
    /// Minimum investment amount
    pub min_investment_amount: Option<f64>,
    /// Ability to bear losses
    pub ability_to_bear_losses: AbilityToBearLosses,
    /// Investment time horizon
    pub time_horizon: TimeHorizon,
}

/// Client category for target market
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClientCategory {
    /// Retail clients
    Retail,
    /// Professional clients
    Professional,
    /// Eligible counterparties
    EligibleCounterparty,
}

/// Knowledge and experience level
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KnowledgeLevel {
    /// Basic - little or no knowledge
    Basic,
    /// Informed - some knowledge and experience
    Informed,
    /// Advanced - extensive knowledge
    Advanced,
}

/// Risk tolerance level
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskTolerance {
    /// Low risk tolerance
    Low,
    /// Medium risk tolerance
    Medium,
    /// High risk tolerance
    High,
}

/// Ability to bear losses
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AbilityToBearLosses {
    /// No capacity for loss
    NoCapacity,
    /// Limited capacity for loss
    Limited,
    /// Full capacity for loss beyond initial investment
    Full,
}

/// Investment time horizon
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TimeHorizon {
    /// Short term (< 3 years)
    ShortTerm,
    /// Medium term (3-7 years)
    MediumTerm,
    /// Long term (> 7 years)
    LongTerm,
}

/// Research unbundling payment (MiFID II Article 24(8), COBS 2.3B)
///
/// Investment research must be paid separately from execution services.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResearchPayment {
    /// Research provider
    pub research_provider: String,
    /// Payment date
    pub payment_date: NaiveDate,
    /// Amount in GBP
    pub amount_gbp: f64,
    /// Paid from research payment account (not client dealing commission)
    pub paid_from_research_account: bool,
    /// Research budget approved
    pub research_budget_approved: bool,
    /// Disclosed to clients
    pub disclosed_to_clients: bool,
}

impl ResearchPayment {
    /// Check if unbundling compliant
    pub fn is_unbundled(&self) -> bool {
        self.paid_from_research_account && self.research_budget_approved
    }
}

/// Best execution report (COBS 11.2)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BestExecutionReport {
    /// Reporting period
    pub period_start: NaiveDate,
    /// Period end
    pub period_end: NaiveDate,
    /// Top 5 execution venues by volume
    pub top_venues: Vec<ExecutionVenue>,
    /// Quality assessment
    pub quality_assessment: String,
    /// Published to clients
    pub published_to_clients: bool,
}

/// Execution venue
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecutionVenue {
    /// Venue name
    pub venue_name: String,
    /// MIC code
    pub mic_code: String,
    /// Volume percentage
    pub volume_percentage: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_report_timely() {
        let report = TransactionReport {
            report_id: "TR001".to_string(),
            transaction_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            instrument_isin: "GB0002374006".to_string(),
            buyer_lei: "213800EBPD2GY84SVP41".to_string(),
            seller_lei: "529900T8BM49AURSDO55".to_string(),
            executing_entity_lei: "213800EBPD2GY84SVP41".to_string(),
            quantity: 1000.0,
            price: 100.0,
            currency: "GBP".to_string(),
            venue_mic: "XLON".to_string(),
            reported_to_fca: true,
            reporting_deadline: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
        };

        let current = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        assert!(report.is_timely_reported(current));
    }

    #[test]
    fn test_research_payment_unbundled() {
        let payment = ResearchPayment {
            research_provider: "Research House Ltd".to_string(),
            payment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 10000.0,
            paid_from_research_account: true,
            research_budget_approved: true,
            disclosed_to_clients: true,
        };

        assert!(payment.is_unbundled());
    }

    #[test]
    fn test_research_payment_bundled() {
        let payment = ResearchPayment {
            research_provider: "Research House Ltd".to_string(),
            payment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            amount_gbp: 10000.0,
            paid_from_research_account: false, // Bundled with execution
            research_budget_approved: false,
            disclosed_to_clients: false,
        };

        assert!(!payment.is_unbundled());
    }
}
