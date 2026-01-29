//! Code of Civil Procedure (CPC) 1908 Types
//!
//! Types for India's civil procedure law that governs procedures in civil courts.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Type of civil suit under CPC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuitType {
    /// Money suit for recovery of debt or damages
    Money,
    /// Specific performance of contract (Section 9)
    SpecificPerformance,
    /// Declaratory suit (Section 9, 34)
    Declaratory,
    /// Injunction suit (Order 39)
    Injunction,
    /// Partition suit
    Partition,
    /// Possession suit
    Possession,
    /// Probate and administration
    Probate,
    /// Insolvency petition
    Insolvency,
    /// Matrimonial relief
    Matrimonial,
    /// Other civil matter
    Other,
}

/// Territorial jurisdiction basis (Section 15-20)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JurisdictionBasis {
    /// Where defendant resides/works (Section 20)
    DefendantResidence,
    /// Where cause of action arose (Section 20)
    CauseOfAction,
    /// Where property situated (Section 16-19)
    PropertyLocation,
    /// Where contract performed (Section 20)
    ContractPerformance,
    /// Court's territorial limits (Section 15)
    TerritorialLimits,
}

/// Pecuniary jurisdiction limits
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PecuniaryJurisdiction {
    /// Court type
    pub court_type: CourtType,
    /// Minimum suit value
    pub min_value: Option<f64>,
    /// Maximum suit value
    pub max_value: Option<f64>,
}

/// Type of civil court
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourtType {
    /// Supreme Court (Article 136)
    SupremeCourt,
    /// High Court (Article 226, Section 15)
    HighCourt,
    /// District Court (Principal seat)
    DistrictCourt,
    /// Sub-Judge Court
    SubJudge,
    /// Munsif Court (Small causes)
    Munsif,
    /// Small Causes Court (special jurisdiction)
    SmallCauses,
}

/// Pleading type (Order 6-7)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PleadingType {
    /// Plaint (Order 7)
    Plaint,
    /// Written statement (Order 8)
    WrittenStatement,
    /// Replication
    Replication,
    /// Rejoinder
    Rejoinder,
    /// Interlocutory application
    InterlocutoryApplication,
    /// Amendment application (Order 6 Rule 17)
    Amendment,
}

/// Civil suit structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CivilSuit {
    /// Suit number
    pub suit_number: String,
    /// Suit type
    pub suit_type: SuitType,
    /// Court where filed
    pub court: CourtType,
    /// Plaintiff details
    pub plaintiff: String,
    /// Defendant details
    pub defendant: String,
    /// Suit value (Rs.)
    pub suit_value: f64,
    /// Date of filing
    pub filing_date: NaiveDate,
    /// Jurisdiction basis
    pub jurisdiction_basis: Vec<JurisdictionBasis>,
    /// Court fees paid
    pub court_fees_paid: f64,
    /// Is limitation complied
    pub within_limitation: bool,
    /// Current status
    pub status: SuitStatus,
}

/// Status of civil suit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SuitStatus {
    /// Filed
    Filed,
    /// Plaint admitted (Order 7 Rule 11 check passed)
    Admitted,
    /// Plaint rejected (Order 7 Rule 11)
    Rejected,
    /// Written statement filed
    WrittenStatementFiled,
    /// Issues framed (Order 14)
    IssuesFramed,
    /// Evidence stage
    Evidence,
    /// Arguments
    Arguments,
    /// Judgment reserved
    JudgmentReserved,
    /// Judgment delivered
    Judged,
    /// Decree passed
    Decreed,
    /// Execution proceedings (Order 21)
    Execution,
    /// Appeal filed
    OnAppeal,
    /// Disposed
    Disposed,
}

/// Order type under CPC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    /// Temporary injunction (Order 39 Rule 1-2)
    TemporaryInjunction,
    /// Attachment before judgment (Order 38)
    AttachmentBeforeJudgment,
    /// Appointment of receiver (Order 40)
    AppointmentOfReceiver,
    /// Interim maintenance (Order 39 Rule 2A)
    InterimMaintenance,
    /// Stay of execution (Order 21 Rule 30)
    StayOfExecution,
    /// Discovery and inspection (Order 11)
    Discovery,
    /// Summoning witness (Order 16)
    SummoningWitness,
    /// Amendment of pleadings (Order 6 Rule 17)
    Amendment,
}

/// Appeal type (Section 96-110, Order 41-43)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealType {
    /// First Appeal (Section 96) - from original decree
    FirstAppeal,
    /// Second Appeal (Section 100) - on substantial question of law
    SecondAppeal,
    /// Appeal from Order (Section 104)
    AppealFromOrder,
    /// Revision (Section 115) - not an appeal but supervisory jurisdiction
    Revision,
    /// Review (Order 47) - before same court
    Review,
}

/// Appeal structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Appeal {
    /// Appeal number
    pub appeal_number: String,
    /// Appeal type
    pub appeal_type: AppealType,
    /// Lower court suit number
    pub lower_court_suit: String,
    /// Appellant name
    pub appellant: String,
    /// Respondent name
    pub respondent: String,
    /// Date of decree/order appealed against
    pub decree_date: NaiveDate,
    /// Date of filing appeal
    pub filing_date: NaiveDate,
    /// Is filed within limitation (30/60/90 days)
    pub within_limitation: bool,
    /// Is court fee paid
    pub court_fee_paid: bool,
    /// Is security deposited (Order 41 Rule 1)
    pub security_deposited: bool,
    /// Status
    pub status: AppealStatus,
}

/// Appeal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppealStatus {
    /// Filed
    Filed,
    /// Admitted
    Admitted,
    /// Dismissed for delay
    DismissedForDelay,
    /// Dismissed for non-prosecution
    DismissedForNonProsecution,
    /// Pending hearing
    Pending,
    /// Reserved for orders
    Reserved,
    /// Allowed
    Allowed,
    /// Dismissed
    Dismissed,
    /// Partly allowed
    PartlyAllowed,
}

/// Execution proceedings (Order 21)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionProceeding {
    /// Execution petition number
    pub petition_number: String,
    /// Decree holder (applicant)
    pub decree_holder: String,
    /// Judgment debtor (respondent)
    pub judgment_debtor: String,
    /// Decree amount (Rs.)
    pub decree_amount: f64,
    /// Date of decree
    pub decree_date: NaiveDate,
    /// Date of filing execution
    pub filing_date: NaiveDate,
    /// Is within limitation (12 years from decree)
    pub within_limitation: bool,
    /// Mode of execution
    pub execution_mode: Vec<ExecutionMode>,
    /// Status
    pub status: ExecutionStatus,
}

/// Mode of execution (Order 21)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Attachment and sale of property (Order 21 Rule 54)
    AttachmentAndSale,
    /// Arrest and detention (Order 21 Rule 37)
    ArrestAndDetention,
    /// Attachment of salary (Order 21 Rule 48)
    SalaryAttachment,
    /// Delivery of possession
    DeliveryOfPossession,
    /// Appointment of receiver (Order 21 Rule 4)
    Receiver,
}

/// Execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionStatus {
    /// Filed
    Filed,
    /// Notice issued
    NoticeIssued,
    /// Property attached
    PropertyAttached,
    /// Proclamation sale pending
    SalePending,
    /// Satisfied
    Satisfied,
    /// Partly satisfied
    PartlySatisfied,
    /// Withdrawn
    Withdrawn,
}

/// Court fees calculation (Court Fees Act 1870)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CourtFees {
    /// Suit value (Rs.)
    pub suit_value: f64,
    /// Ad valorem fees (percentage-based)
    pub ad_valorem_fees: f64,
    /// Fixed fees (if applicable)
    pub fixed_fees: Option<f64>,
    /// Process fees
    pub process_fees: f64,
    /// Total court fees
    pub total_fees: f64,
}

impl CourtFees {
    /// Calculate court fees for money suit
    ///
    /// # रुपये (Rupees) Calculation
    /// - Up to Rs. 500: Rs. 5
    /// - Rs. 500 - 1,000: Rs. 5 + 4% of excess
    /// - Rs. 1,000 - 5,000: Rs. 25 + 3% of excess
    /// - Rs. 5,000 - 20,000: Rs. 145 + 2.5% of excess
    /// - Above Rs. 20,000: Rs. 520 + 2% of excess
    pub fn calculate_money_suit(suit_value: f64) -> Self {
        let ad_valorem = if suit_value <= 500.0 {
            5.0
        } else if suit_value <= 1000.0 {
            5.0 + (suit_value - 500.0) * 0.04
        } else if suit_value <= 5000.0 {
            25.0 + (suit_value - 1000.0) * 0.03
        } else if suit_value <= 20000.0 {
            145.0 + (suit_value - 5000.0) * 0.025
        } else {
            520.0 + (suit_value - 20000.0) * 0.02
        };

        Self {
            suit_value,
            ad_valorem_fees: ad_valorem,
            fixed_fees: None,
            process_fees: 100.0, // Typical process fees
            total_fees: ad_valorem + 100.0,
        }
    }
}

/// Limitation period for civil suits (Limitation Act 1963)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitationPeriod {
    /// Suit type
    pub suit_type: SuitType,
    /// Limitation period in years
    pub years: u32,
    /// Starting point of limitation
    pub starting_point: String,
}

impl LimitationPeriod {
    /// Get limitation period for suit type
    pub fn for_suit_type(suit_type: SuitType) -> Self {
        match suit_type {
            SuitType::Money => Self {
                suit_type,
                years: 3,
                starting_point: "When debt/liability accrued".to_string(),
            },
            SuitType::SpecificPerformance => Self {
                suit_type,
                years: 3,
                starting_point: "When contract ought to be performed".to_string(),
            },
            SuitType::Declaratory => Self {
                suit_type,
                years: 3,
                starting_point: "When right to sue accrued".to_string(),
            },
            SuitType::Possession => Self {
                suit_type,
                years: 12,
                starting_point: "When dispossession occurred".to_string(),
            },
            _ => Self {
                suit_type,
                years: 3,
                starting_point: "When cause of action arose".to_string(),
            },
        }
    }
}
