//! E-Commerce Validation
//!
//! Validation logic for e-commerce transactions under
//! Specified Commercial Transactions Act.

use crate::egov::ValidationReport;

use super::ecommerce::*;
use super::error::{ConsumerProtectionError, Result};

/// Validate e-commerce transaction
pub fn validate_ecommerce_transaction(
    transaction: &EcommerceTransaction,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Article 11: Required legal disclosures
    if !transaction.has_complete_disclosure() {
        let missing = transaction.legal_disclosure.missing_items();
        report.add_error(format!(
            "Incomplete legal disclosure (特定商取引法第11条). Missing: {}",
            missing.join(", ")
        ));
    }

    // Check return policy
    if !transaction.has_appropriate_return_policy() {
        report.add_warning("Return policy may not meet consumer expectations".to_string());
    }

    // Digital content special rules
    if transaction.is_digital_content && transaction.return_policy.returns_accepted {
        report.add_warning(
            "Digital content returns typically not allowed after download/access".to_string(),
        );
    }

    // Check shipping information for physical goods
    if !transaction.is_digital_content && transaction.shipping_info.is_none() {
        report.add_error("Shipping information required for physical goods".to_string());
    }

    // Check terms and privacy policy URLs
    if transaction.terms_url.is_none() {
        report.add_warning("Terms and conditions URL should be provided".to_string());
    }

    if transaction.privacy_policy_url.is_none() {
        report.add_warning("Privacy policy URL should be provided (個人情報保護法)".to_string());
    }

    // Check transaction ID
    if transaction.transaction_id.is_empty() {
        report.add_error("Transaction ID is required".to_string());
    }

    Ok(report)
}

/// Validate subscription service
pub fn validate_subscription_service(
    subscription: &SubscriptionService,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Check service name
    if subscription.service_name.is_empty() {
        report.add_error("Service name is required".to_string());
    }

    // Check provider name
    if subscription.provider_name.is_empty() {
        report.add_error("Provider name is required".to_string());
    }

    // Check cancellation notice period
    if !subscription.has_reasonable_cancellation_notice() {
        report.add_warning(format!(
            "Cancellation notice period of {} days may be excessive. Recommend 30 days or less.",
            subscription.cancellation_notice_days
        ));
    }

    // Auto-renewal warning
    if subscription.auto_renewal && !subscription.has_free_trial() {
        report.add_warning(
            "Auto-renewal without free trial should be clearly disclosed to consumers".to_string(),
        );
    }

    // Check billing cycle reasonableness
    if subscription.monthly_fee_jpy == 0 {
        report.add_error("Monthly fee must be specified".to_string());
    }

    Ok(report)
}

/// Validate return policy
pub fn validate_return_policy(
    policy: &ReturnPolicy,
    order_amount_jpy: u64,
) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    if policy.returns_accepted {
        // Check return period
        if policy.return_period_days < 7 {
            report.add_warning(
                "Return period less than 7 days may not be consumer-friendly".to_string(),
            );
        }

        // Check refund processing time
        if policy.refund_processing_days > 14 {
            report.add_warning(format!(
                "Refund processing time of {} days is long. Recommend 7-14 days.",
                policy.refund_processing_days
            ));
        }

        // Check shipping cost burden
        if policy.return_shipping_by_customer && order_amount_jpy > 10_000 {
            report.add_warning(
                "Customer bearing return shipping cost may be unfair for high-value orders"
                    .to_string(),
            );
        }
    } else {
        // No returns policy
        report.add_warning(
            "No returns policy may not be consumer-friendly. Consider allowing returns within a reasonable period.".to_string(),
        );
    }

    Ok(report)
}

/// Quick validation helper for e-commerce transaction
pub fn quick_validate_ecommerce(transaction: &EcommerceTransaction) -> Result<()> {
    let report = validate_ecommerce_transaction(transaction)?;
    if !report.is_valid() {
        Err(ConsumerProtectionError::ValidationError {
            message: format!("{} validation errors", report.errors.len()),
        })
    } else {
        Ok(())
    }
}

/// Quick validation helper for subscription service
pub fn quick_validate_subscription(subscription: &SubscriptionService) -> Result<()> {
    let report = validate_subscription_service(subscription)?;
    if !report.is_valid() {
        Err(ConsumerProtectionError::ValidationError {
            message: format!("{} validation errors", report.errors.len()),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_valid_legal_disclosure() -> LegalDisclosure {
        LegalDisclosure {
            business_name: "Test Shop Co., Ltd.".to_string(),
            representative_name: "Taro Tanaka".to_string(),
            business_address: "1-1-1 Shibuya, Tokyo".to_string(),
            contact_phone: "03-1234-5678".to_string(),
            contact_email: "info@testshop.com".to_string(),
            business_hours: "9:00-18:00".to_string(),
            return_policy_disclosed: true,
            delivery_timeframe_disclosed: true,
            payment_methods_disclosed: true,
        }
    }

    #[test]
    fn test_validate_complete_ecommerce_transaction() {
        let transaction = EcommerceTransaction {
            transaction_id: "ORDER-001".to_string(),
            platform_type: PlatformType::DirectStore,
            seller_name: "Test Shop".to_string(),
            customer_name: "Customer Name".to_string(),
            order_date: Utc::now(),
            product_description: "Test Product".to_string(),
            is_digital_content: false,
            digital_content_type: None,
            order_amount_jpy: 10_000,
            payment_method: PaymentMethod::CreditCard,
            return_policy: ReturnPolicy {
                returns_accepted: true,
                return_period_days: 14,
                conditions: vec![],
                return_shipping_by_customer: false,
                refund_processing_days: 7,
            },
            shipping_info: Some(ShippingInfo {
                estimated_delivery_days: 3,
                shipping_cost_jpy: 500,
                free_shipping_threshold_jpy: Some(5000),
                tracking_available: true,
            }),
            legal_disclosure: create_valid_legal_disclosure(),
            terms_url: Some("https://example.com/terms".to_string()),
            privacy_policy_url: Some("https://example.com/privacy".to_string()),
        };

        let report = validate_ecommerce_transaction(&transaction).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_incomplete_disclosure() {
        let transaction = EcommerceTransaction {
            transaction_id: "ORDER-002".to_string(),
            platform_type: PlatformType::Marketplace,
            seller_name: "Test Seller".to_string(),
            customer_name: "Customer".to_string(),
            order_date: Utc::now(),
            product_description: "Product".to_string(),
            is_digital_content: false,
            digital_content_type: None,
            order_amount_jpy: 5000,
            payment_method: PaymentMethod::BankTransfer,
            return_policy: ReturnPolicy {
                returns_accepted: false,
                return_period_days: 0,
                conditions: vec![],
                return_shipping_by_customer: true,
                refund_processing_days: 0,
            },
            shipping_info: Some(ShippingInfo {
                estimated_delivery_days: 5,
                shipping_cost_jpy: 800,
                free_shipping_threshold_jpy: None,
                tracking_available: false,
            }),
            legal_disclosure: LegalDisclosure {
                business_name: String::new(),
                representative_name: String::new(),
                business_address: String::new(),
                contact_phone: String::new(),
                contact_email: String::new(),
                business_hours: String::new(),
                return_policy_disclosed: false,
                delivery_timeframe_disclosed: false,
                payment_methods_disclosed: false,
            },
            terms_url: None,
            privacy_policy_url: None,
        };

        let report = validate_ecommerce_transaction(&transaction).unwrap();
        assert!(!report.is_valid());
        assert!(!report.errors.is_empty());
    }

    #[test]
    fn test_validate_digital_content() {
        let transaction = EcommerceTransaction {
            transaction_id: "DIG-001".to_string(),
            platform_type: PlatformType::DirectStore,
            seller_name: "Digital Store".to_string(),
            customer_name: "Customer".to_string(),
            order_date: Utc::now(),
            product_description: "E-book".to_string(),
            is_digital_content: true,
            digital_content_type: Some(DigitalContentType::Ebook),
            order_amount_jpy: 1500,
            payment_method: PaymentMethod::CreditCard,
            return_policy: ReturnPolicy {
                returns_accepted: false,
                return_period_days: 0,
                conditions: vec!["No returns for digital content".to_string()],
                return_shipping_by_customer: false,
                refund_processing_days: 0,
            },
            shipping_info: None,
            legal_disclosure: create_valid_legal_disclosure(),
            terms_url: Some("https://example.com/terms".to_string()),
            privacy_policy_url: Some("https://example.com/privacy".to_string()),
        };

        let report = validate_ecommerce_transaction(&transaction).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_subscription() {
        let subscription = SubscriptionService {
            service_name: "Streaming Service".to_string(),
            provider_name: "Stream Co.".to_string(),
            start_date: Utc::now(),
            billing_cycle: BillingCycle::Monthly,
            monthly_fee_jpy: 980,
            cancellation_notice_days: 14,
            auto_renewal: true,
            free_trial_days: Some(7),
        };

        let report = validate_subscription_service(&subscription).unwrap();
        assert!(report.is_valid());
    }

    #[test]
    fn test_validate_subscription_excessive_notice() {
        let subscription = SubscriptionService {
            service_name: "Service".to_string(),
            provider_name: "Provider".to_string(),
            start_date: Utc::now(),
            billing_cycle: BillingCycle::Annual,
            monthly_fee_jpy: 500,
            cancellation_notice_days: 60,
            auto_renewal: true,
            free_trial_days: None,
        };

        let report = validate_subscription_service(&subscription).unwrap();
        assert!(report.is_valid()); // Still valid but has warnings
        assert!(!report.warnings.is_empty());
    }

    #[test]
    fn test_validate_return_policy() {
        let good_policy = ReturnPolicy {
            returns_accepted: true,
            return_period_days: 14,
            conditions: vec![],
            return_shipping_by_customer: false,
            refund_processing_days: 7,
        };

        let report = validate_return_policy(&good_policy, 10_000).unwrap();
        assert!(report.is_valid());
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn test_validate_poor_return_policy() {
        let poor_policy = ReturnPolicy {
            returns_accepted: true,
            return_period_days: 3,
            conditions: vec![],
            return_shipping_by_customer: true,
            refund_processing_days: 30,
        };

        let report = validate_return_policy(&poor_policy, 50_000).unwrap();
        assert!(report.is_valid()); // Valid but has warnings
        assert!(report.warnings.len() >= 2);
    }

    #[test]
    fn test_quick_validate() {
        let transaction = EcommerceTransaction {
            transaction_id: "QUICK-001".to_string(),
            platform_type: PlatformType::DirectStore,
            seller_name: "Shop".to_string(),
            customer_name: "Customer".to_string(),
            order_date: Utc::now(),
            product_description: "Product".to_string(),
            is_digital_content: false,
            digital_content_type: None,
            order_amount_jpy: 5000,
            payment_method: PaymentMethod::CreditCard,
            return_policy: ReturnPolicy {
                returns_accepted: true,
                return_period_days: 7,
                conditions: vec![],
                return_shipping_by_customer: false,
                refund_processing_days: 7,
            },
            shipping_info: Some(ShippingInfo {
                estimated_delivery_days: 2,
                shipping_cost_jpy: 300,
                free_shipping_threshold_jpy: None,
                tracking_available: true,
            }),
            legal_disclosure: create_valid_legal_disclosure(),
            terms_url: Some("https://example.com/terms".to_string()),
            privacy_policy_url: Some("https://example.com/privacy".to_string()),
        };

        assert!(quick_validate_ecommerce(&transaction).is_ok());
    }
}
