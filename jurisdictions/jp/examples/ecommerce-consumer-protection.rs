//! E-Commerce Consumer Protection Example
//!
//! Demonstrates e-commerce transaction validation and subscription service
//! compliance checking under the Specified Commercial Transactions Act.
//!
//! Run with:
//! ```bash
//! cargo run --example ecommerce-consumer-protection
//! ```

use chrono::Utc;
use legalis_jp::consumer_protection::ecommerce::*;
use legalis_jp::consumer_protection::ecommerce_validator::*;

fn main() {
    println!("=== E-Commerce Consumer Protection Example ===\n");

    // Example 1: Valid e-commerce transaction
    println!("📦 Example 1: Valid E-Commerce Transaction");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let valid_transaction = create_valid_transaction();
    validate_and_print_transaction(&valid_transaction);
    println!();

    // Example 2: Invalid transaction (incomplete disclosure)
    println!("❌ Example 2: Invalid Transaction (Incomplete Disclosure)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let invalid_transaction = create_invalid_transaction();
    validate_and_print_transaction(&invalid_transaction);
    println!();

    // Example 3: Digital content transaction
    println!("📱 Example 3: Digital Content Transaction");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let digital_transaction = create_digital_transaction();
    validate_and_print_transaction(&digital_transaction);
    println!();

    // Example 4: Subscription service validation
    println!("🔄 Example 4: Subscription Service Validation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let subscription = create_subscription_service();
    validate_and_print_subscription(&subscription);
    println!();

    // Example 5: Return policy analysis
    println!("↩️  Example 5: Return Policy Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    analyze_return_policies();
}

fn create_valid_transaction() -> EcommerceTransaction {
    EcommerceTransaction {
        transaction_id: "ORDER-20260109-001".to_string(),
        platform_type: PlatformType::DirectStore,
        seller_name: "Tokyo Electronics Co., Ltd.".to_string(),
        customer_name: "Tanaka Taro".to_string(),
        order_date: Utc::now(),
        product_description: "Wireless Headphones - Premium Model".to_string(),
        is_digital_content: false,
        digital_content_type: None,
        order_amount_jpy: 15_000,
        payment_method: PaymentMethod::CreditCard,
        return_policy: ReturnPolicy {
            returns_accepted: true,
            return_period_days: 14,
            conditions: vec!["Unopened packaging".to_string()],
            return_shipping_by_customer: false,
            refund_processing_days: 7,
        },
        shipping_info: Some(ShippingInfo {
            estimated_delivery_days: 2,
            shipping_cost_jpy: 0,
            free_shipping_threshold_jpy: Some(10_000),
            tracking_available: true,
        }),
        legal_disclosure: LegalDisclosure {
            business_name: "Tokyo Electronics Co., Ltd.".to_string(),
            representative_name: "Yamada Ichiro".to_string(),
            business_address: "1-2-3 Shibuya, Shibuya-ku, Tokyo 150-0001".to_string(),
            contact_phone: "03-1234-5678".to_string(),
            contact_email: "support@tokyo-electronics.jp".to_string(),
            business_hours: "9:00-18:00 (Mon-Fri)".to_string(),
            return_policy_disclosed: true,
            delivery_timeframe_disclosed: true,
            payment_methods_disclosed: true,
        },
        terms_url: Some("https://tokyo-electronics.jp/terms".to_string()),
        privacy_policy_url: Some("https://tokyo-electronics.jp/privacy".to_string()),
    }
}

fn create_invalid_transaction() -> EcommerceTransaction {
    EcommerceTransaction {
        transaction_id: "ORDER-20260109-002".to_string(),
        platform_type: PlatformType::Marketplace,
        seller_name: "Small Shop".to_string(),
        customer_name: "Sato Hanako".to_string(),
        order_date: Utc::now(),
        product_description: "Handmade Accessories".to_string(),
        is_digital_content: false,
        digital_content_type: None,
        order_amount_jpy: 8_000,
        payment_method: PaymentMethod::BankTransfer,
        return_policy: ReturnPolicy {
            returns_accepted: false,
            return_period_days: 0,
            conditions: vec![],
            return_shipping_by_customer: true,
            refund_processing_days: 0,
        },
        shipping_info: Some(ShippingInfo {
            estimated_delivery_days: 7,
            shipping_cost_jpy: 500,
            free_shipping_threshold_jpy: None,
            tracking_available: false,
        }),
        legal_disclosure: LegalDisclosure {
            business_name: String::new(),       // Missing!
            representative_name: String::new(), // Missing!
            business_address: String::new(),    // Missing!
            contact_phone: String::new(),       // Missing!
            contact_email: "shop@example.com".to_string(),
            business_hours: String::new(),
            return_policy_disclosed: false,
            delivery_timeframe_disclosed: false,
            payment_methods_disclosed: false,
        },
        terms_url: None,
        privacy_policy_url: None,
    }
}

fn create_digital_transaction() -> EcommerceTransaction {
    EcommerceTransaction {
        transaction_id: "DIG-20260109-001".to_string(),
        platform_type: PlatformType::DirectStore,
        seller_name: "Digital Content Platform".to_string(),
        customer_name: "Suzuki Yuki".to_string(),
        order_date: Utc::now(),
        product_description: "Premium E-book: Learn Rust Programming".to_string(),
        is_digital_content: true,
        digital_content_type: Some(DigitalContentType::Ebook),
        order_amount_jpy: 2_980,
        payment_method: PaymentMethod::CreditCard,
        return_policy: ReturnPolicy {
            returns_accepted: false,
            return_period_days: 0,
            conditions: vec![
                "No returns for digital content after download".to_string(),
                "Preview available before purchase".to_string(),
            ],
            return_shipping_by_customer: false,
            refund_processing_days: 0,
        },
        shipping_info: None, // Digital content - no shipping
        legal_disclosure: LegalDisclosure {
            business_name: "Digital Content Platform KK".to_string(),
            representative_name: "Kobayashi Kenji".to_string(),
            business_address: "5-6-7 Roppongi, Minato-ku, Tokyo 106-0032".to_string(),
            contact_phone: "03-9876-5432".to_string(),
            contact_email: "info@digitalcontent.jp".to_string(),
            business_hours: "24/7 (Online Support)".to_string(),
            return_policy_disclosed: true,
            delivery_timeframe_disclosed: true,
            payment_methods_disclosed: true,
        },
        terms_url: Some("https://digitalcontent.jp/terms".to_string()),
        privacy_policy_url: Some("https://digitalcontent.jp/privacy".to_string()),
    }
}

fn create_subscription_service() -> SubscriptionService {
    SubscriptionService {
        service_name: "Premium Video Streaming Service".to_string(),
        provider_name: "Stream Japan KK".to_string(),
        start_date: Utc::now(),
        billing_cycle: BillingCycle::Monthly,
        monthly_fee_jpy: 1_980,
        cancellation_notice_days: 14,
        auto_renewal: true,
        free_trial_days: Some(30),
    }
}

fn validate_and_print_transaction(transaction: &EcommerceTransaction) {
    println!("Transaction ID: {}", transaction.transaction_id);
    println!(
        "Platform: {} ({:?})",
        transaction.platform_type.name_ja(),
        transaction.platform_type
    );
    println!("Seller: {}", transaction.seller_name);
    println!("Amount: ¥{}", transaction.order_amount_jpy);
    println!(
        "Payment: {} ({:?})",
        transaction.payment_method.name_ja(),
        transaction.payment_method
    );

    if transaction.is_digital_content {
        println!(
            "Type: Digital Content ({:?})",
            transaction.digital_content_type
        );
    } else {
        println!("Type: Physical Goods");
    }

    println!("\n📋 Validation Results:");
    match validate_ecommerce_transaction(transaction) {
        Ok(report) => {
            if report.is_valid() {
                println!("✅ Transaction is VALID");
                if !report.warnings.is_empty() {
                    println!("\n⚠️  Warnings:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            } else {
                println!("❌ Transaction is INVALID");
                println!("\n🚫 Errors:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
                if !report.warnings.is_empty() {
                    println!("\n⚠️  Warnings:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Validation error: {}", e);
        }
    }
}

fn validate_and_print_subscription(subscription: &SubscriptionService) {
    println!("Service: {}", subscription.service_name);
    println!("Provider: {}", subscription.provider_name);
    println!(
        "Billing: {} (¥{}/month)",
        subscription.billing_cycle.name_ja(),
        subscription.monthly_fee_jpy
    );
    println!(
        "Cancellation Notice: {} days",
        subscription.cancellation_notice_days
    );
    println!(
        "Auto-Renewal: {}",
        if subscription.auto_renewal {
            "Yes"
        } else {
            "No"
        }
    );

    if let Some(trial_days) = subscription.free_trial_days {
        println!("Free Trial: {} days", trial_days);
    }

    println!("\n📋 Validation Results:");
    match validate_subscription_service(subscription) {
        Ok(report) => {
            if report.is_valid() {
                println!("✅ Subscription service is VALID");
                if !report.warnings.is_empty() {
                    println!("\n⚠️  Warnings:");
                    for warning in &report.warnings {
                        println!("  • {}", warning);
                    }
                }
            } else {
                println!("❌ Subscription service is INVALID");
                println!("\n🚫 Errors:");
                for error in &report.errors {
                    println!("  • {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation error: {}", e);
        }
    }
}

fn analyze_return_policies() {
    let policies = vec![
        (
            "Consumer-Friendly Policy",
            ReturnPolicy {
                returns_accepted: true,
                return_period_days: 30,
                conditions: vec!["Original packaging preferred".to_string()],
                return_shipping_by_customer: false,
                refund_processing_days: 5,
            },
            50_000,
        ),
        (
            "Minimal Policy",
            ReturnPolicy {
                returns_accepted: true,
                return_period_days: 3,
                conditions: vec!["Unopened only".to_string()],
                return_shipping_by_customer: true,
                refund_processing_days: 30,
            },
            50_000,
        ),
        (
            "No Returns",
            ReturnPolicy {
                returns_accepted: false,
                return_period_days: 0,
                conditions: vec!["Final sale".to_string()],
                return_shipping_by_customer: true,
                refund_processing_days: 0,
            },
            10_000,
        ),
    ];

    for (name, policy, amount) in policies {
        println!("\n{}", name);
        println!("  Returns Accepted: {}", policy.returns_accepted);
        println!("  Return Period: {} days", policy.return_period_days);
        println!(
            "  Refund Processing: {} days",
            policy.refund_processing_days
        );
        println!(
            "  Customer-Friendly: {}",
            if policy.is_consumer_friendly() {
                "✅ Yes"
            } else {
                "❌ No"
            }
        );

        match validate_return_policy(&policy, amount) {
            Ok(report) => {
                if !report.warnings.is_empty() {
                    println!("  Warnings:");
                    for warning in &report.warnings {
                        println!("    • {}", warning);
                    }
                }
            }
            Err(e) => {
                println!("  Error: {}", e);
            }
        }
    }
}
