//! Consumer Contract Risk Analyzer Example (消費者契約リスク分析例)
//!
//! This example demonstrates consumer contract validation and unfair terms detection
//! according to the Consumer Contract Act (消費者契約法) and Specified Commercial
//! Transactions Act (特定商取引法).
//!
//! # Usage
//! ```bash
//! cargo run --example consumer-contract-risk-analyzer
//! ```

use chrono::{Duration, Utc};
use legalis_jp::consumer_protection::*;

fn main() {
    println!("=== Consumer Contract Risk Analyzer Example ===\n");
    println!("消費者契約リスク分析例 - Consumer Contract Risk Analysis\n");

    // ========================================================================
    // PART 1: CONSUMER CONTRACT ACT (消費者契約法)
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: CONSUMER CONTRACT ACT (消費者契約法)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Example 1: Unfair Terms Detection
    println!("🔍 Example 1: Unfair Terms Detection (不当条項検出)");
    println!("{}", "=".repeat(70));

    let unfair_terms_examples = vec![
        ("当社は一切責任を負いません。", "Full exemption clause"),
        (
            "消費者の負担において一方的に契約を変更できる。",
            "Consumer disadvantage",
        ),
        (
            "解約時には契約金額の100%を違約金として支払う。",
            "Excessive penalty",
        ),
        (
            "故意又は重過失を除き、当社は責任を負いません。",
            "Partial exemption",
        ),
        ("商品は7日以内に配送されます。", "Fair clause"),
    ];

    println!("Analyzing various contract clauses:\n");

    for (clause, description) in unfair_terms_examples {
        let (risk_score, potentially_unfair, unfair_type) = detect_unfair_terms(clause);

        println!("Clause: \"{}\"", clause);
        println!("  Type: {}", description);
        println!("  Risk Score: {} / 100", risk_score);
        println!("  Potentially Unfair: {}", potentially_unfair);
        if let Some(ut) = unfair_type {
            println!("  Unfair Type: {:?}", ut);
        }

        if risk_score >= 70 {
            println!("  ⚠️  HIGH RISK - Likely invalid under Consumer Contract Act");
        } else if risk_score >= 40 {
            println!("  ⚠️  MEDIUM RISK - Requires careful review");
        } else if risk_score >= 30 {
            println!("  ⚡ LOW RISK - May be questionable");
        } else {
            println!("  ✅ ACCEPTABLE RISK");
        }
        println!();
    }

    println!("{}\n", "=".repeat(70));

    // Example 2: Valid Consumer Contract
    println!("📋 Example 2: Valid Consumer Contract (有効な消費者契約)");
    println!("{}", "=".repeat(70));

    let valid_contract = ConsumerContract {
        title: "オンラインサービス利用契約 (Online Service Agreement)".to_string(),
        business_name: "テクノロジーサービス株式会社".to_string(),
        consumer_name: "山田太郎".to_string(),
        contract_date: Utc::now(),
        contract_amount_jpy: 50_000,
        terms: vec![
            ContractTerm {
                term_number: 1,
                text: "サービスは月額5,000円で提供されます。".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 5,
            },
            ContractTerm {
                term_number: 2,
                text: "利用者は個人情報を正確に提供する義務があります。".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 10,
            },
        ],
        cancellation_policy: Some(CancellationPolicy {
            cancellation_fee_jpy: 5_000,
            cancellation_fee_percentage: Some(0.10), // 10%
            notice_period_days: 30,
            description: "30日前の通知で解約可能".to_string(),
        }),
        penalty_clause: None,
    };

    println!("Contract: {}", valid_contract.title);
    println!("Business: {}", valid_contract.business_name);
    println!("Consumer: {}", valid_contract.consumer_name);
    println!("Amount: ¥{}", valid_contract.contract_amount_jpy);
    println!("\nTerms:");
    for term in &valid_contract.terms {
        println!(
            "  {}. {} (Risk: {})",
            term.term_number, term.text, term.risk_score
        );
    }

    if let Some(cancel) = &valid_contract.cancellation_policy {
        println!("\nCancellation Policy:");
        println!(
            "  Fee: ¥{} ({:.0}%)",
            cancel.cancellation_fee_jpy,
            cancel.cancellation_fee_percentage.unwrap_or(0.0) * 100.0
        );
        println!("  Notice: {} days", cancel.notice_period_days);
        println!(
            "  Excessive: {}",
            cancel.is_fee_excessive(valid_contract.contract_amount_jpy)
        );
    }

    match validate_consumer_contract(&valid_contract) {
        Ok(()) => println!("\n✅ Validation: PASSED - Contract is fair and compliant!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 3: Contract with Excessive Cancellation Fee
    println!("❌ Example 3: Excessive Cancellation Fee (過大な解除料)");
    println!("{}", "=".repeat(70));

    let excessive_fee_contract = ConsumerContract {
        title: "高額解約料契約".to_string(),
        business_name: "問題業者".to_string(),
        consumer_name: "消費者".to_string(),
        contract_date: Utc::now(),
        contract_amount_jpy: 100_000,
        terms: vec![ContractTerm {
            term_number: 1,
            text: "Standard term".to_string(),
            potentially_unfair: false,
            unfair_type: None,
            risk_score: 10,
        }],
        cancellation_policy: Some(CancellationPolicy {
            cancellation_fee_jpy: 40_000, // 40% - Excessive!
            cancellation_fee_percentage: Some(0.40),
            notice_period_days: 7,
            description: "解約料40%".to_string(),
        }),
        penalty_clause: None,
    };

    println!(
        "Contract Amount: ¥{}",
        excessive_fee_contract.contract_amount_jpy
    );
    if let Some(cancel) = &excessive_fee_contract.cancellation_policy {
        println!(
            "Cancellation Fee: ¥{} ({:.0}%)",
            cancel.cancellation_fee_jpy,
            cancel.cancellation_fee_percentage.unwrap_or(0.0) * 100.0
        );
    }

    match validate_consumer_contract(&excessive_fee_contract) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!(
            "\n❌ Validation: FAILED (as expected)\n   Error: {}\n   (Article 9-1 violation)",
            e
        ),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 4: Automatic Risk Analysis
    println!("🤖 Example 4: Automatic Risk Analysis (自動リスク分析)");
    println!("{}", "=".repeat(70));

    let risky_contract = ConsumerContract {
        title: "疑わしい契約".to_string(),
        business_name: "業者".to_string(),
        consumer_name: "消費者".to_string(),
        contract_date: Utc::now(),
        contract_amount_jpy: 200_000,
        terms: vec![
            ContractTerm {
                term_number: 1,
                text: "当社は一切責任を負いません。全てのリスクは消費者が負担します。".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 0, // Will be analyzed
            },
            ContractTerm {
                term_number: 2,
                text: "契約内容は当社の裁量により一方的に変更できます。".to_string(),
                potentially_unfair: false,
                unfair_type: None,
                risk_score: 0,
            },
        ],
        cancellation_policy: None,
        penalty_clause: Some(PenaltyClause {
            penalty_amount_jpy: 300_000,    // Exceeds contract value!
            daily_penalty_rate: Some(0.03), // 3% per day!
            description: "違約金".to_string(),
        }),
    };

    println!("Analyzing contract with automatic risk scoring...\n");

    let analyzed_contract = analyze_contract_terms(risky_contract);

    println!("Contract: {}", analyzed_contract.title);
    println!("Analysis Results:\n");

    for term in &analyzed_contract.terms {
        println!("Term {}: \"{}\"", term.term_number, term.text);
        println!("  Risk Score: {} / 100", term.risk_score);
        println!("  Potentially Unfair: {}", term.potentially_unfair);
        if let Some(ut) = &term.unfair_type {
            println!("  Type: {:?}", ut);
        }
        println!();
    }

    if let Some(penalty) = &analyzed_contract.penalty_clause {
        println!("Penalty Clause Analysis:");
        println!("  Amount: ¥{}", penalty.penalty_amount_jpy);
        let risk_multiplier = penalty.risk_multiplier(analyzed_contract.contract_amount_jpy);
        println!("  Risk Multiplier: {:.1}x contract value", risk_multiplier);

        if risk_multiplier > 1.0 {
            println!("  ⚠️  CRITICAL: Penalty exceeds contract value!");
        }
    }

    match validate_consumer_contract(&analyzed_contract) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!(
            "\n❌ Validation: FAILED\n   Multiple violations detected: {}",
            e
        ),
    }

    println!("\n{}\n", "=".repeat(70));

    // ========================================================================
    // PART 2: SPECIFIED COMMERCIAL TRANSACTIONS ACT (特定商取引法)
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: COOLING-OFF RIGHTS (クーリング・オフ)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Example 5: Door-to-Door Sales Cooling-Off
    println!("🚪 Example 5: Door-to-Door Sales (訪問販売)");
    println!("{}", "=".repeat(70));

    let door_to_door = SpecifiedCommercialTransaction {
        transaction_type: TransactionType::DoorToDoor,
        seller_name: "訪問販売業者株式会社".to_string(),
        purchaser_name: "佐藤花子".to_string(),
        contract_date: Utc::now(),
        document_receipt_date: Some(Utc::now()),
        contract_amount_jpy: 150_000,
        product_description: "浄水器システム".to_string(),
        payment_method: "分割払い".to_string(),
        cooling_off_notice_provided: true,
    };

    println!("Transaction: {:?}", door_to_door.transaction_type);
    println!("Seller: {}", door_to_door.seller_name);
    println!("Amount: ¥{}", door_to_door.contract_amount_jpy);
    println!("Product: {}", door_to_door.product_description);
    println!("\nCooling-Off Information:");
    println!(
        "  Period: {} days",
        door_to_door.transaction_type.cooling_off_period_days()
    );
    println!(
        "  Has Cooling-Off: {}",
        door_to_door.transaction_type.has_cooling_off()
    );

    if let Some((deadline, days_remaining, is_valid)) = get_cooling_off_info(&door_to_door) {
        println!("  Deadline: {}", deadline.format("%Y-%m-%d"));
        println!("  Days Remaining: {}", days_remaining);
        println!("  Currently Valid: {}", is_valid);

        if days_remaining > 5 {
            println!("  ✅ Plenty of time to exercise cooling-off!");
        } else if days_remaining > 0 {
            println!("  ⚠️  Limited time remaining!");
        } else {
            println!("  ❌ Cooling-off period expired!");
        }
    }

    match validate_specified_transaction(&door_to_door) {
        Ok(()) => println!("\n✅ Validation: PASSED - Transaction is compliant!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 6: Multi-Level Marketing (20-day cooling-off)
    println!("📊 Example 6: Multi-Level Marketing (連鎖販売取引)");
    println!("{}", "=".repeat(70));

    let mlm_transaction = SpecifiedCommercialTransaction {
        transaction_type: TransactionType::MultiLevelMarketing,
        seller_name: "MLM Company".to_string(),
        purchaser_name: "鈴木一郎".to_string(),
        contract_date: Utc::now(),
        document_receipt_date: Some(Utc::now()),
        contract_amount_jpy: 500_000,
        product_description: "ビジネス参加権と商品".to_string(),
        payment_method: "前払い".to_string(),
        cooling_off_notice_provided: true,
    };

    println!("Transaction: {:?}", mlm_transaction.transaction_type);
    println!("Amount: ¥{}", mlm_transaction.contract_amount_jpy);
    println!(
        "Cooling-Off Period: {} days (longer protection for MLM)",
        mlm_transaction.transaction_type.cooling_off_period_days()
    );

    println!("\n{}\n", "=".repeat(70));

    // Example 7: Mail-Order (No Cooling-Off)
    println!("📧 Example 7: Mail-Order Sales (通信販売)");
    println!("{}", "=".repeat(70));

    let mail_order = SpecifiedCommercialTransaction {
        transaction_type: TransactionType::MailOrder,
        seller_name: "オンラインショップ".to_string(),
        purchaser_name: "高橋美咲".to_string(),
        contract_date: Utc::now(),
        document_receipt_date: None,
        contract_amount_jpy: 30_000,
        product_description: "オンライン商品".to_string(),
        payment_method: "クレジットカード".to_string(),
        cooling_off_notice_provided: false,
    };

    println!("Transaction: {:?}", mail_order.transaction_type);
    println!("Amount: ¥{}", mail_order.contract_amount_jpy);
    println!(
        "Has Cooling-Off: {} (mail-order follows seller's return policy)",
        mail_order.transaction_type.has_cooling_off()
    );

    if let Some((deadline, _days_remaining, _is_valid)) = get_cooling_off_info(&mail_order) {
        println!("Deadline: {}", deadline);
    } else {
        println!("Note: No statutory cooling-off for mail-order");
        println!("      Return policy depends on seller's terms");
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 8: Cooling-Off Exercise
    println!("✉️  Example 8: Exercising Cooling-Off (クーリング・オフ行使)");
    println!("{}", "=".repeat(70));

    let cooling_off_exercise = CoolingOffExercise {
        transaction: door_to_door.clone(),
        exercise_date: Utc::now() + Duration::days(3),
        notification_method: "書面郵送 (Registered mail)".to_string(),
        notification_sent: true,
    };

    println!(
        "Exercise Date: {}",
        cooling_off_exercise.exercise_date.format("%Y-%m-%d")
    );
    println!(
        "Notification Method: {}",
        cooling_off_exercise.notification_method
    );
    println!(
        "Notification Sent: {}",
        cooling_off_exercise.notification_sent
    );
    println!("Is Timely: {}", cooling_off_exercise.is_timely());

    match validate_cooling_off_exercise(&cooling_off_exercise) {
        Ok(()) => {
            println!("\n✅ Validation: PASSED");
            println!("   Consumer can cancel without penalty!");
            println!("   Seller must refund full amount.");
        }
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}", "=".repeat(70));
    println!("\n✨ Consumer Contract Risk Analysis Complete!");
    println!("   All examples demonstrate proper compliance with:");
    println!("   - Consumer Contract Act (消費者契約法)");
    println!("   - Specified Commercial Transactions Act (特定商取引法)\n");
}
