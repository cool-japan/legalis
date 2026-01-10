//! Consumer Protection Types (消費者保護法型定義)
//!
//! Type definitions for Japanese consumer protection law, including:
//! - Consumer Contract Act (消費者契約法 - Shōhisha Keiyaku-hō)
//! - Specified Commercial Transactions Act (特定商取引法 - Tokutei Shō Torihiki-hō)
//!
//! # Legal References
//! - Consumer Contract Act (Act No. 61 of 2000) - 消費者契約法
//! - Specified Commercial Transactions Act (Act No. 57 of 1976) - 特定商取引法

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Consumer Protection Law (消費者保護法)
// ============================================================================

/// Standard cooling-off period for door-to-door sales (Article 9)
/// 訪問販売のクーリング・オフ期間 - 8日
pub const COOLING_OFF_DOOR_TO_DOOR_DAYS: u32 = 8;

/// Cooling-off period for multi-level marketing (Article 40)
/// 連鎖販売取引のクーリング・オフ期間 - 20日
pub const COOLING_OFF_MLM_DAYS: u32 = 20;

/// Cooling-off period for business opportunity sales (Article 58)
/// 業務提供誘引販売のクーリング・オフ期間 - 20日
pub const COOLING_OFF_BUSINESS_OPP_DAYS: u32 = 20;

/// Maximum penalty rate (Article 9, Consumer Contract Act)
/// 損害賠償額予定の上限 - 平均的損害額
pub const MAX_PENALTY_RATE_AVERAGE_DAMAGE: f64 = 1.0;

// ============================================================================
// Consumer Contract Act Types (消費者契約法型)
// ============================================================================

/// Party role in contract (契約当事者)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PartyRole {
    /// Consumer (消費者 - Shōhisha)
    Consumer,

    /// Business operator (事業者 - Jigyō-sha)
    Business,
}

/// Unfair contract term type (不当条項の種類 - Article 8-10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnfairTermType {
    /// Full exemption from liability (全部免責条項 - Article 8-1-1)
    FullExemption,

    /// Partial exemption from liability (一部免責条項 - Article 8-1-2/3)
    PartialExemption,

    /// Excessive penalty clause (過大な損害賠償予定 - Article 9-1)
    ExcessivePenalty,

    /// Excessive cancellation fee (過大な解除料 - Article 9-1)
    ExcessiveCancellationFee,

    /// Consumer disadvantage clause (消費者の利益を一方的に害する条項 - Article 10)
    ConsumerDisadvantage,

    /// Unreasonable burden on consumer (不当に重い義務)
    UnreasonableBurden,
}

/// Rescission ground (取消事由 - Article 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RescissionGround {
    /// Misrepresentation of important facts (不実告知 - Article 4-1-1)
    Misrepresentation,

    ///断定的判断の提供 (Provision of definite judgment - Article 4-1-2)
    DefiniteJudgment,

    /// Non-disclosure of disadvantageous facts (不利益事実の不告知 - Article 4-2)
    NonDisclosure,

    /// Undue influence (過量な契約 - Article 4-3)
    UndueInfluence,

    /// Threatening behavior (威迫 - Article 4-3-3)
    Threat,

    /// Obstruction of leaving (退去妨害 - Article 4-3-1)
    ObstructionOfLeaving,

    /// Refusal to leave (不退去 - Article 4-3-2)
    RefusalToLeave,
}

/// Consumer contract (消費者契約)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConsumerContract {
    /// Contract title (契約名称 - Keiyaku meishō)
    pub title: String,

    /// Business operator name (事業者名 - Jigyō-sha mei)
    pub business_name: String,

    /// Consumer name (消費者名 - Shōhisha mei)
    pub consumer_name: String,

    /// Contract date (契約日 - Keiyaku-bi)
    pub contract_date: DateTime<Utc>,

    /// Contract amount (契約金額 - Keiyaku kingaku)
    pub contract_amount_jpy: u64,

    /// Contract terms (契約条項 - Keiyaku jōkō)
    pub terms: Vec<ContractTerm>,

    /// Cancellation policy (解約規定 - Kaiyaku kitei)
    pub cancellation_policy: Option<CancellationPolicy>,

    /// Penalty clause (損害賠償予定条項)
    pub penalty_clause: Option<PenaltyClause>,
}

/// Contract term (契約条項)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContractTerm {
    /// Term number (条項番号 - Jōkō bangō)
    pub term_number: u32,

    /// Term text (条項内容 - Jōkō naiyō)
    pub text: String,

    /// Is potentially unfair (不当条項の疑い)
    pub potentially_unfair: bool,

    /// Unfair term type if detected (不当条項種別)
    pub unfair_type: Option<UnfairTermType>,

    /// Risk score (0-100, higher = more risky)
    /// リスクスコア (0-100、高いほど危険)
    pub risk_score: u32,
}

/// Cancellation policy (解約規定)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancellationPolicy {
    /// Cancellation fee amount (解約料 - Kaiyaku-ryō)
    pub cancellation_fee_jpy: u64,

    /// Cancellation fee percentage (解約料率)
    pub cancellation_fee_percentage: Option<f64>,

    /// Notice period required (予告期間 - Yokoku kikan)
    pub notice_period_days: u32,

    /// Description (説明 - Setsumei)
    pub description: String,
}

impl CancellationPolicy {
    /// Check if cancellation fee is excessive (解約料過大判定)
    pub fn is_fee_excessive(&self, contract_amount: u64) -> bool {
        // Typically, cancellation fees should not exceed 10-20% of contract value
        // or average damages
        if let Some(percentage) = self.cancellation_fee_percentage {
            percentage > 0.20 // Over 20% is generally excessive
        } else {
            let fee_ratio = self.cancellation_fee_jpy as f64 / contract_amount as f64;
            fee_ratio > 0.20
        }
    }
}

/// Penalty clause (損害賠償予定条項)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PenaltyClause {
    /// Penalty amount (違約金額 - Iyaku kingaku)
    pub penalty_amount_jpy: u64,

    /// Daily penalty rate (日割違約金率)
    pub daily_penalty_rate: Option<f64>,

    /// Description (説明 - Setsumei)
    pub description: String,
}

impl PenaltyClause {
    /// Check if penalty is excessive (Article 9)
    /// 違約金過大判定
    pub fn is_penalty_excessive(&self, average_damages_jpy: u64) -> bool {
        self.penalty_amount_jpy > average_damages_jpy
    }

    /// Calculate risk multiplier (リスク倍率計算)
    pub fn risk_multiplier(&self, average_damages_jpy: u64) -> f64 {
        if average_damages_jpy == 0 {
            return 0.0;
        }
        self.penalty_amount_jpy as f64 / average_damages_jpy as f64
    }
}

/// Rescission claim (取消権の行使)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RescissionClaim {
    /// Contract being rescinded (対象契約)
    pub contract: ConsumerContract,

    /// Ground for rescission (取消事由)
    pub ground: RescissionGround,

    /// Date of rescission (取消日 - Torikeshi-bi)
    pub rescission_date: DateTime<Utc>,

    /// Description of grounds (事由の説明)
    pub description: String,

    /// Evidence of grounds (証拠)
    pub evidence_description: Option<String>,
}

impl RescissionClaim {
    /// Check if rescission period is still valid (取消期間内判定)
    /// Article 7: 6 months from knowledge, 5 years from contract
    pub fn is_within_rescission_period(&self) -> bool {
        let months_since_contract =
            (self.rescission_date - self.contract.contract_date).num_days() / 30;

        // Simplified check: within 5 years of contract
        months_since_contract < 60 // 5 years = 60 months
    }
}

// ============================================================================
// Specified Commercial Transactions Act Types (特定商取引法型)
// ============================================================================

/// Transaction type under SCTA (特定商取引の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TransactionType {
    /// Door-to-door sales (訪問販売 - Hōmon hanbai - Article 2-1)
    DoorToDoor,

    /// Telemarketing (電話勧誘販売 - Denwa kan'yū hanbai - Article 2-3)
    Telemarketing,

    /// Mail-order sales (通信販売 - Tsūshin hanbai - Article 2-2)
    MailOrder,

    /// Multi-level marketing (連鎖販売取引 - Rensa hanbai torihiki - Article 33)
    MultiLevelMarketing,

    /// Business opportunity sales (業務提供誘引販売 - Gyōmu teikyō yūin hanbai)
    BusinessOpportunity,

    /// Continuing services (特定継続的役務 - Tokutei keizoku-teki ekimu)
    ContinuingServices,
}

impl TransactionType {
    /// Get cooling-off period for this transaction type (クーリング・オフ期間取得)
    pub fn cooling_off_period_days(&self) -> u32 {
        match self {
            TransactionType::DoorToDoor => COOLING_OFF_DOOR_TO_DOOR_DAYS,
            TransactionType::Telemarketing => COOLING_OFF_DOOR_TO_DOOR_DAYS,
            TransactionType::MailOrder => 0, // No cooling-off for mail-order
            TransactionType::MultiLevelMarketing => COOLING_OFF_MLM_DAYS,
            TransactionType::BusinessOpportunity => COOLING_OFF_BUSINESS_OPP_DAYS,
            TransactionType::ContinuingServices => COOLING_OFF_DOOR_TO_DOOR_DAYS,
        }
    }

    /// Check if cooling-off applies (クーリング・オフ適用判定)
    pub fn has_cooling_off(&self) -> bool {
        self.cooling_off_period_days() > 0
    }
}

/// Specified commercial transaction (特定商取引)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpecifiedCommercialTransaction {
    /// Transaction type (取引種別 - Torihiki shubetsu)
    pub transaction_type: TransactionType,

    /// Seller/provider name (販売業者名 - Hanbai gyōsha mei)
    pub seller_name: String,

    /// Purchaser name (購入者名 - Kōnyū-sha mei)
    pub purchaser_name: String,

    /// Contract date (契約日 - Keiyaku-bi)
    pub contract_date: DateTime<Utc>,

    /// Receipt of contract documents date (契約書面交付日)
    pub document_receipt_date: Option<DateTime<Utc>>,

    /// Contract amount (契約金額 - Keiyaku kingaku)
    pub contract_amount_jpy: u64,

    /// Product/service description (商品・役務の内容)
    pub product_description: String,

    /// Payment method (支払方法 - Shiharai hōhō)
    pub payment_method: String,

    /// Cooling-off notice provided (クーリング・オフ告知済み)
    pub cooling_off_notice_provided: bool,
}

impl SpecifiedCommercialTransaction {
    /// Calculate cooling-off deadline (クーリング・オフ期限計算)
    pub fn cooling_off_deadline(&self) -> Option<DateTime<Utc>> {
        if !self.transaction_type.has_cooling_off() {
            return None;
        }

        let start_date = self.document_receipt_date.unwrap_or(self.contract_date);
        let days = self.transaction_type.cooling_off_period_days();

        Some(start_date + chrono::Duration::days(days as i64))
    }

    /// Check if still within cooling-off period (クーリング・オフ期間内判定)
    pub fn is_within_cooling_off_period(&self) -> bool {
        if let Some(deadline) = self.cooling_off_deadline() {
            Utc::now() <= deadline
        } else {
            false
        }
    }

    /// Days remaining in cooling-off period (クーリング・オフ残日数)
    pub fn cooling_off_days_remaining(&self) -> Option<i64> {
        if let Some(deadline) = self.cooling_off_deadline() {
            let remaining = (deadline - Utc::now()).num_days();
            Some(remaining.max(0))
        } else {
            None
        }
    }
}

/// Cooling-off exercise (クーリング・オフの行使)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CoolingOffExercise {
    /// Transaction being cancelled (対象取引)
    pub transaction: SpecifiedCommercialTransaction,

    /// Exercise date (行使日 - Kōshi-bi)
    pub exercise_date: DateTime<Utc>,

    /// Method of notification (通知方法 - Tsūchi hōhō)
    pub notification_method: String,

    /// Notification sent (通知済み - Tsūchi-zumi)
    pub notification_sent: bool,
}

impl CoolingOffExercise {
    /// Check if cooling-off was exercised in time (期間内行使判定)
    pub fn is_timely(&self) -> bool {
        if let Some(deadline) = self.transaction.cooling_off_deadline() {
            self.exercise_date <= deadline
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_type_cooling_off_periods() {
        assert_eq!(TransactionType::DoorToDoor.cooling_off_period_days(), 8);
        assert_eq!(
            TransactionType::MultiLevelMarketing.cooling_off_period_days(),
            20
        );
        assert_eq!(TransactionType::MailOrder.cooling_off_period_days(), 0);
        assert!(!TransactionType::MailOrder.has_cooling_off());
        assert!(TransactionType::DoorToDoor.has_cooling_off());
    }

    #[test]
    fn test_cancellation_policy_excessive() {
        let policy = CancellationPolicy {
            cancellation_fee_jpy: 30_000,
            cancellation_fee_percentage: Some(0.30), // 30%
            notice_period_days: 30,
            description: "Test".to_string(),
        };

        assert!(policy.is_fee_excessive(100_000));

        let reasonable_policy = CancellationPolicy {
            cancellation_fee_jpy: 10_000,
            cancellation_fee_percentage: Some(0.10), // 10%
            notice_period_days: 30,
            description: "Test".to_string(),
        };

        assert!(!reasonable_policy.is_fee_excessive(100_000));
    }

    #[test]
    fn test_penalty_clause_excessive() {
        let penalty = PenaltyClause {
            penalty_amount_jpy: 200_000,
            daily_penalty_rate: None,
            description: "Test".to_string(),
        };

        assert!(penalty.is_penalty_excessive(100_000)); // 2x average damages
        assert!(!penalty.is_penalty_excessive(300_000)); // Less than average

        assert_eq!(penalty.risk_multiplier(100_000), 2.0);
    }

    #[test]
    fn test_cooling_off_deadline() {
        let transaction = SpecifiedCommercialTransaction {
            transaction_type: TransactionType::DoorToDoor,
            seller_name: "Seller".to_string(),
            purchaser_name: "Buyer".to_string(),
            contract_date: Utc::now(),
            document_receipt_date: Some(Utc::now()),
            contract_amount_jpy: 100_000,
            product_description: "Product".to_string(),
            payment_method: "Credit card".to_string(),
            cooling_off_notice_provided: true,
        };

        assert!(transaction.cooling_off_deadline().is_some());
        assert!(transaction.is_within_cooling_off_period());
        assert!(transaction.cooling_off_days_remaining().unwrap() > 0);
    }

    #[test]
    fn test_mail_order_no_cooling_off() {
        let transaction = SpecifiedCommercialTransaction {
            transaction_type: TransactionType::MailOrder,
            seller_name: "Online Store".to_string(),
            purchaser_name: "Customer".to_string(),
            contract_date: Utc::now(),
            document_receipt_date: None,
            contract_amount_jpy: 50_000,
            product_description: "Online product".to_string(),
            payment_method: "Credit card".to_string(),
            cooling_off_notice_provided: false,
        };

        assert!(transaction.cooling_off_deadline().is_none());
        assert!(!transaction.is_within_cooling_off_period());
    }
}
