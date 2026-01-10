//! E-Commerce Consumer Protection
//!
//! E-commerce specific provisions under Specified Commercial Transactions Act
//! and related regulations (特定商取引法 - 通信販売).

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// E-commerce platform type (ECプラットフォームの種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PlatformType {
    /// Direct online store (自社EC)
    DirectStore,
    /// Marketplace (ECマーケットプレイス)
    Marketplace,
    /// Social commerce (ソーシャルコマース)
    SocialCommerce,
    /// Auction site (オークションサイト)
    Auction,
}

impl PlatformType {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::DirectStore => "自社ECサイト",
            Self::Marketplace => "マーケットプレイス",
            Self::SocialCommerce => "ソーシャルコマース",
            Self::Auction => "オークションサイト",
        }
    }
}

/// Digital content type (デジタルコンテンツの種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DigitalContentType {
    /// Software (ソフトウェア)
    Software,
    /// E-book (電子書籍)
    Ebook,
    /// Music (音楽)
    Music,
    /// Video (動画)
    Video,
    /// Game (ゲーム)
    Game,
    /// Subscription service (サブスクリプション)
    Subscription,
}

/// Return policy (返品・返金ポリシー)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReturnPolicy {
    /// Whether returns are accepted (返品可否)
    pub returns_accepted: bool,
    /// Return period in days (返品期限)
    pub return_period_days: u32,
    /// Conditions for return (返品条件)
    pub conditions: Vec<String>,
    /// Who bears return shipping cost (返送料負担)
    pub return_shipping_by_customer: bool,
    /// Refund processing days (返金処理日数)
    pub refund_processing_days: u32,
}

impl ReturnPolicy {
    /// Check if return policy is consumer-friendly
    pub fn is_consumer_friendly(&self) -> bool {
        self.returns_accepted && self.return_period_days >= 7 && !self.return_shipping_by_customer
    }

    /// Check if policy meets minimum requirements
    pub fn meets_minimum_requirements(&self) -> bool {
        // For digital content, returns may not be applicable
        // For physical goods, at least basic return policy should exist
        self.returns_accepted || !self.conditions.is_empty()
    }
}

/// Payment method (支払方法)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PaymentMethod {
    /// Credit card (クレジットカード)
    CreditCard,
    /// Bank transfer (銀行振込)
    BankTransfer,
    /// Cash on delivery (代金引換)
    CashOnDelivery,
    /// Electronic money (電子マネー)
    ElectronicMoney,
    /// Mobile payment (モバイル決済)
    MobilePayment,
    /// Convenience store payment (コンビニ決済)
    ConvenienceStore,
}

impl PaymentMethod {
    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::CreditCard => "クレジットカード",
            Self::BankTransfer => "銀行振込",
            Self::CashOnDelivery => "代金引換",
            Self::ElectronicMoney => "電子マネー",
            Self::MobilePayment => "モバイル決済",
            Self::ConvenienceStore => "コンビニ決済",
        }
    }

    /// Check if payment method requires immediate payment
    pub fn requires_immediate_payment(&self) -> bool {
        matches!(
            self,
            Self::CreditCard | Self::ElectronicMoney | Self::MobilePayment
        )
    }
}

/// Shipping information (配送情報)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShippingInfo {
    /// Estimated delivery days (配送予定日数)
    pub estimated_delivery_days: u32,
    /// Shipping cost (配送料)
    pub shipping_cost_jpy: u64,
    /// Free shipping threshold (送料無料基準額)
    pub free_shipping_threshold_jpy: Option<u64>,
    /// Tracking available (追跡可能)
    pub tracking_available: bool,
}

/// Required disclosure items for e-commerce (特定商取引法に基づく表記)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LegalDisclosure {
    /// Business name (事業者名)
    pub business_name: String,
    /// Representative name (代表者名)
    pub representative_name: String,
    /// Business address (所在地)
    pub business_address: String,
    /// Contact phone (電話番号)
    pub contact_phone: String,
    /// Contact email (メールアドレス)
    pub contact_email: String,
    /// Business hours (営業時間)
    pub business_hours: String,
    /// Return policy disclosed (返品ポリシー記載)
    pub return_policy_disclosed: bool,
    /// Delivery timeframe disclosed (配送時期記載)
    pub delivery_timeframe_disclosed: bool,
    /// Payment methods disclosed (支払方法記載)
    pub payment_methods_disclosed: bool,
}

impl LegalDisclosure {
    /// Check if all required items are disclosed (Article 11)
    pub fn is_complete(&self) -> bool {
        !self.business_name.is_empty()
            && !self.business_address.is_empty()
            && !self.contact_phone.is_empty()
            && !self.contact_email.is_empty()
            && self.return_policy_disclosed
            && self.delivery_timeframe_disclosed
            && self.payment_methods_disclosed
    }

    /// Get missing disclosure items
    pub fn missing_items(&self) -> Vec<String> {
        let mut missing = Vec::new();

        if self.business_name.is_empty() {
            missing.push("事業者名 (Business name)".to_string());
        }
        if self.business_address.is_empty() {
            missing.push("所在地 (Business address)".to_string());
        }
        if self.contact_phone.is_empty() {
            missing.push("電話番号 (Contact phone)".to_string());
        }
        if self.contact_email.is_empty() {
            missing.push("メールアドレス (Contact email)".to_string());
        }
        if !self.return_policy_disclosed {
            missing.push("返品ポリシー (Return policy)".to_string());
        }
        if !self.delivery_timeframe_disclosed {
            missing.push("配送時期 (Delivery timeframe)".to_string());
        }
        if !self.payment_methods_disclosed {
            missing.push("支払方法 (Payment methods)".to_string());
        }

        missing
    }
}

/// E-commerce transaction (通信販売取引)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EcommerceTransaction {
    /// Transaction ID (取引ID)
    pub transaction_id: String,
    /// Platform type (プラットフォーム)
    pub platform_type: PlatformType,
    /// Seller information (販売者情報)
    pub seller_name: String,
    /// Customer name (顧客名)
    pub customer_name: String,
    /// Order date (注文日)
    pub order_date: DateTime<Utc>,
    /// Product description (商品説明)
    pub product_description: String,
    /// Is digital content (デジタルコンテンツか)
    pub is_digital_content: bool,
    /// Digital content type (デジタルコンテンツ種別)
    pub digital_content_type: Option<DigitalContentType>,
    /// Order amount (注文金額)
    pub order_amount_jpy: u64,
    /// Payment method (支払方法)
    pub payment_method: PaymentMethod,
    /// Return policy (返品ポリシー)
    pub return_policy: ReturnPolicy,
    /// Shipping info (配送情報)
    pub shipping_info: Option<ShippingInfo>,
    /// Legal disclosure (特定商取引法表記)
    pub legal_disclosure: LegalDisclosure,
    /// Terms and conditions URL (利用規約URL)
    pub terms_url: Option<String>,
    /// Privacy policy URL (プライバシーポリシーURL)
    pub privacy_policy_url: Option<String>,
}

impl EcommerceTransaction {
    /// Check if transaction has complete legal disclosures
    pub fn has_complete_disclosure(&self) -> bool {
        self.legal_disclosure.is_complete()
    }

    /// Check if return policy is appropriate
    pub fn has_appropriate_return_policy(&self) -> bool {
        // Digital content may have different rules
        if self.is_digital_content {
            // Digital content may not allow returns after download
            true
        } else {
            self.return_policy.meets_minimum_requirements()
        }
    }

    /// Check if transaction is for physical goods requiring shipping
    pub fn requires_shipping(&self) -> bool {
        !self.is_digital_content && self.shipping_info.is_some()
    }

    /// Get estimated delivery date
    pub fn estimated_delivery_date(&self) -> Option<DateTime<Utc>> {
        self.shipping_info.as_ref().map(|shipping| {
            self.order_date + chrono::Duration::days(shipping.estimated_delivery_days as i64)
        })
    }
}

/// Subscription service (サブスクリプションサービス)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubscriptionService {
    /// Service name (サービス名)
    pub service_name: String,
    /// Provider name (提供事業者)
    pub provider_name: String,
    /// Subscription start date (契約開始日)
    pub start_date: DateTime<Utc>,
    /// Billing cycle (請求サイクル)
    pub billing_cycle: BillingCycle,
    /// Monthly fee (月額料金)
    pub monthly_fee_jpy: u64,
    /// Cancellation policy (解約ポリシー)
    pub cancellation_notice_days: u32,
    /// Auto-renewal (自動更新)
    pub auto_renewal: bool,
    /// Free trial period (無料トライアル期間)
    pub free_trial_days: Option<u32>,
}

/// Billing cycle (請求サイクル)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BillingCycle {
    /// Monthly (月額)
    Monthly,
    /// Quarterly (四半期)
    Quarterly,
    /// Annual (年額)
    Annual,
}

impl BillingCycle {
    /// Get cycle in months
    pub fn months(&self) -> u32 {
        match self {
            Self::Monthly => 1,
            Self::Quarterly => 3,
            Self::Annual => 12,
        }
    }

    /// Get Japanese name
    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Monthly => "月額",
            Self::Quarterly => "四半期",
            Self::Annual => "年額",
        }
    }
}

impl SubscriptionService {
    /// Calculate next billing date
    pub fn next_billing_date(&self) -> DateTime<Utc> {
        self.start_date + chrono::Duration::days((self.billing_cycle.months() * 30) as i64)
    }

    /// Check if cancellation notice period is reasonable
    pub fn has_reasonable_cancellation_notice(&self) -> bool {
        // Typically, 1 month notice should be sufficient
        self.cancellation_notice_days <= 30
    }

    /// Check if free trial is offered
    pub fn has_free_trial(&self) -> bool {
        self.free_trial_days.is_some() && self.free_trial_days.unwrap() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_type_names() {
        assert_eq!(PlatformType::DirectStore.name_ja(), "自社ECサイト");
        assert_eq!(PlatformType::Marketplace.name_ja(), "マーケットプレイス");
    }

    #[test]
    fn test_return_policy_consumer_friendly() {
        let friendly = ReturnPolicy {
            returns_accepted: true,
            return_period_days: 14,
            conditions: vec![],
            return_shipping_by_customer: false,
            refund_processing_days: 7,
        };
        assert!(friendly.is_consumer_friendly());

        let unfriendly = ReturnPolicy {
            returns_accepted: true,
            return_period_days: 3,
            conditions: vec![],
            return_shipping_by_customer: true,
            refund_processing_days: 7,
        };
        assert!(!unfriendly.is_consumer_friendly());
    }

    #[test]
    fn test_payment_method_immediate() {
        assert!(PaymentMethod::CreditCard.requires_immediate_payment());
        assert!(!PaymentMethod::BankTransfer.requires_immediate_payment());
    }

    #[test]
    fn test_legal_disclosure_complete() {
        let complete = LegalDisclosure {
            business_name: "Test Shop".to_string(),
            representative_name: "Taro Tanaka".to_string(),
            business_address: "Tokyo".to_string(),
            contact_phone: "03-1234-5678".to_string(),
            contact_email: "info@test.com".to_string(),
            business_hours: "9:00-18:00".to_string(),
            return_policy_disclosed: true,
            delivery_timeframe_disclosed: true,
            payment_methods_disclosed: true,
        };
        assert!(complete.is_complete());
        assert!(complete.missing_items().is_empty());
    }

    #[test]
    fn test_legal_disclosure_incomplete() {
        let incomplete = LegalDisclosure {
            business_name: String::new(),
            representative_name: "Taro Tanaka".to_string(),
            business_address: String::new(),
            contact_phone: String::new(),
            contact_email: "info@test.com".to_string(),
            business_hours: "9:00-18:00".to_string(),
            return_policy_disclosed: false,
            delivery_timeframe_disclosed: true,
            payment_methods_disclosed: false,
        };
        assert!(!incomplete.is_complete());
        assert_eq!(incomplete.missing_items().len(), 5);
    }

    #[test]
    fn test_billing_cycle_months() {
        assert_eq!(BillingCycle::Monthly.months(), 1);
        assert_eq!(BillingCycle::Quarterly.months(), 3);
        assert_eq!(BillingCycle::Annual.months(), 12);
    }

    #[test]
    fn test_subscription_cancellation_notice() {
        let reasonable = SubscriptionService {
            service_name: "Video Streaming".to_string(),
            provider_name: "Test Provider".to_string(),
            start_date: Utc::now(),
            billing_cycle: BillingCycle::Monthly,
            monthly_fee_jpy: 1000,
            cancellation_notice_days: 14,
            auto_renewal: true,
            free_trial_days: Some(7),
        };
        assert!(reasonable.has_reasonable_cancellation_notice());
        assert!(reasonable.has_free_trial());
    }
}
