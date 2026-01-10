//! Consumer Contract Analysis Example
//!
//! This example demonstrates comprehensive consumer contract analysis including:
//! 1. Creating consumer contracts with terms
//! 2. Detecting unfair practices (CPFTA s. 4-7)
//! 3. Risk scoring based on detected practices
//! 4. Contract validation and SCT eligibility
//!
//! ## Legal Context
//!
//! Under the Consumer Protection (Fair Trading) Act (Cap. 52A):
//! - s. 4: False or misleading representation
//! - s. 5: Unconscionable conduct
//! - s. 6: Bait advertising
//! - s. 7: Harassment or coercion
//!
//! Small Claims Tribunal handles claims up to SGD 20,000 (or 30,000 with consent)

use legalis_sg::consumer::*;

fn print_header(title: &str) {
    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║ {:<65} ║", title);
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");
}

fn print_contract_summary(contract: &ConsumerContract) {
    println!("Contract ID: {}", contract.contract_id);
    println!("Seller: {}", contract.seller_name);
    println!("Consumer: {}", contract.consumer_name);
    println!("Amount: SGD {:.2}", contract.amount_cents as f64 / 100.0);
    println!("Transaction Type: {:?}", contract.transaction_type);
    println!("Description: {}", contract.description);
    println!("\nContract Terms: {}", contract.terms.len());
    for (idx, term) in contract.terms.iter().enumerate() {
        let status = if term.is_potentially_unfair {
            "⚠️  POTENTIALLY UNFAIR"
        } else {
            "✓"
        };
        println!(
            "  {}. [{}] {:?} - {}",
            idx + 1,
            status,
            term.category,
            term.description
        );
        if term.is_potentially_unfair {
            for indicator in &term.risk_indicators {
                println!("       → {}", indicator);
            }
        }
    }
}

fn print_unfair_practices(contract: &ConsumerContract) {
    println!(
        "\nUnfair Practices Detected: {}",
        contract.unfair_practices.len()
    );
    if contract.unfair_practices.is_empty() {
        println!("  ✅ No unfair practices detected");
    } else {
        for (idx, practice) in contract.unfair_practices.iter().enumerate() {
            println!(
                "  {}. {} (Severity: {}/10)",
                idx + 1,
                practice.statute_reference,
                practice.severity
            );
            println!("     Type: {:?}", practice.practice_type);
            println!("     Description: {}", practice.description);
            if !practice.evidence.is_empty() {
                println!("     Evidence:");
                for evidence in &practice.evidence {
                    println!("       - {}", evidence);
                }
            }
        }
    }
}

fn print_risk_assessment(contract: &ConsumerContract) {
    println!("\n╭───────────────────────────────────────────────────────────────────╮");
    println!("│ RISK ASSESSMENT                                                   │");
    println!("├───────────────────────────────────────────────────────────────────┤");
    println!(
        "│ Risk Score: {}/100                                               │",
        contract.risk_score
    );

    let risk_level = if contract.risk_score < 30 {
        "LOW ✓"
    } else if contract.risk_score < 70 {
        "MEDIUM ⚠️"
    } else {
        "HIGH ❌"
    };

    println!("│ Risk Level: {:<55}│", risk_level);
    println!("│                                                                   │");

    let sct_status = if contract.is_sct_eligible() {
        "YES (claim can be filed at Small Claims Tribunal)"
    } else {
        "NO (exceeds SGD 20,000 limit, requires civil court)"
    };
    println!("│ SCT Eligible: {:<51}│", sct_status);

    if contract.transaction_type == TransactionType::SaleOfGoods {
        let lemon_status = if contract.is_lemon_law_applicable() {
            "YES (within 6-month window)"
        } else {
            "NO (beyond 6-month window)"
        };
        println!("│ Lemon Law Applicable: {:<43}│", lemon_status);
    }

    println!("╰───────────────────────────────────────────────────────────────────╯");
}

fn print_validation_result(result: std::result::Result<(), ConsumerError>) {
    println!("\nValidation Result:");
    match result {
        Ok(()) => println!("  ✅ Contract passes validation"),
        Err(e) => {
            println!("  ❌ Validation failed: {}", e);
            if let Some(statute) = e.statute_reference() {
                println!("     Statute: {}", statute);
            }
        }
    }
}

fn main() {
    print_header("CONSUMER CONTRACT ANALYSIS - SINGAPORE");

    // Scenario 1: Low-risk legitimate contract
    print_header("Scenario 1: Legitimate Contract (Low Risk)");

    let mut contract1 = ConsumerContract::new(
        "C001",
        "ABC Electronics Pte Ltd",
        "John Tan Wei Ming",
        TransactionType::SaleOfGoods,
        150_000, // SGD 1,500
        "Laptop computer - Dell Inspiron 15",
    );
    contract1.seller_uen = Some("201012345A".to_string());

    // Add reasonable warranty
    let mut warranty = WarrantyTerms::new(
        365,
        WarrantyType::Manufacturer,
        "1-year manufacturer warranty for defects in materials and workmanship",
    );
    warranty.add_exclusion("Physical damage from drops or liquid");
    warranty.add_exclusion("Software issues");
    contract1.warranty_terms = Some(warranty);

    // Add legitimate terms
    let term1 = ContractTerm::new(
        "T1",
        "Payment due within 7 days of invoice date",
        TermCategory::Payment,
    );
    contract1.add_term(term1);

    let term2 = ContractTerm::new(
        "T2",
        "14-day return policy for unopened items",
        TermCategory::ReturnRefund,
    );
    contract1.add_term(term2);

    // Detect practices and calculate risk
    let practices1 = detect_unfair_practices(&contract1);
    for practice in practices1 {
        contract1.add_unfair_practice(practice);
    }
    contract1.calculate_risk_score();

    print_contract_summary(&contract1);
    print_unfair_practices(&contract1);
    print_risk_assessment(&contract1);
    print_validation_result(validate_consumer_contract(&contract1));

    // Scenario 2: Medium-risk contract with potentially unfair terms
    print_header("Scenario 2: Medium Risk - Potentially Unfair Terms");

    let mut contract2 = ConsumerContract::new(
        "C002",
        "QuickFix Renovation Pte Ltd",
        "Mary Lim Hui Ling",
        TransactionType::Services,
        1_200_000, // SGD 12,000
        "Home renovation - kitchen and bathroom",
    );

    // Add potentially unfair terms
    let mut term3 = ContractTerm::new(
        "T3",
        "Company not responsible for any delays or damages whatsoever",
        TermCategory::LiabilityLimitation,
    );
    term3.mark_unfair("Overly broad liability exclusion");
    contract2.add_term(term3);

    let mut term4 = ContractTerm::new(
        "T4",
        "All payments are non-refundable once work commences",
        TermCategory::ReturnRefund,
    );
    term4.mark_unfair("No refund policy may be unconscionable");
    contract2.add_term(term4);

    // Detect practices
    let practices2 = detect_unfair_practices(&contract2);
    for practice in practices2 {
        contract2.add_unfair_practice(practice);
    }
    contract2.calculate_risk_score();

    print_contract_summary(&contract2);
    print_unfair_practices(&contract2);
    print_risk_assessment(&contract2);
    print_validation_result(validate_consumer_contract(&contract2));

    // Scenario 3: High-risk contract with multiple violations
    print_header("Scenario 3: High Risk - Multiple Unfair Practices");

    let mut contract3 = ConsumerContract::new(
        "C003",
        "MiracleCure Health Products",
        "Ali Rahman bin Hassan",
        TransactionType::SaleOfGoods,
        350_000, // SGD 3,500
        "GUARANTEED miracle weight loss supplement - 100% effective!",
    );

    // Add suspicious terms
    let mut term5 = ContractTerm::new(
        "T5",
        "No refunds under any circumstances",
        TermCategory::ReturnRefund,
    );
    term5.mark_unfair("Absolute no-refund policy");
    contract3.add_term(term5);

    let mut term6 = ContractTerm::new(
        "T6",
        "Company has no liability for any health issues or side effects",
        TermCategory::LiabilityLimitation,
    );
    term6.mark_unfair("Health product with no liability - unconscionable");
    contract3.add_term(term6);

    // Manually add specific unfair practices
    let mut practice1 = UnfairPractice::new(
        "UP1",
        UnfairPracticeType::FalseRepresentation,
        "Making unsubstantiated health claims",
    );
    practice1.add_evidence("Description claims '100% effective' without scientific backing");
    practice1.add_evidence("Use of word 'GUARANTEED' and 'miracle'");
    contract3.add_unfair_practice(practice1);

    let mut practice2 = UnfairPractice::new(
        "UP2",
        UnfairPracticeType::UnconscionableConduct,
        "No refunds combined with exaggerated claims",
    );
    practice2.add_evidence("Absolute no-refund policy for health product");
    practice2.add_evidence("No liability for health issues");
    contract3.add_unfair_practice(practice2);

    // Auto-detect additional practices
    let practices3 = detect_unfair_practices(&contract3);
    for practice in practices3 {
        contract3.add_unfair_practice(practice);
    }

    contract3.calculate_risk_score();

    print_contract_summary(&contract3);
    print_unfair_practices(&contract3);
    print_risk_assessment(&contract3);
    print_validation_result(validate_consumer_contract(&contract3));

    // Scenario 4: Contract exceeding SCT limit
    print_header("Scenario 4: High-Value Contract (Exceeds SCT Limit)");

    let mut contract4 = ConsumerContract::new(
        "C004",
        "Luxury Motors Pte Ltd",
        "David Ng Boon Kiat",
        TransactionType::SaleOfGoods,
        3_500_000, // SGD 35,000
        "Used luxury vehicle - Mercedes E-Class 2020",
    );

    let term7 = ContractTerm::new(
        "T7",
        "Vehicle sold as-is with no warranties",
        TermCategory::WarrantyDisclaimer,
    );
    contract4.add_term(term7);

    // Detect practices
    let practices4 = detect_unfair_practices(&contract4);
    for practice in practices4 {
        contract4.add_unfair_practice(practice);
    }
    contract4.calculate_risk_score();

    print_contract_summary(&contract4);
    print_unfair_practices(&contract4);
    print_risk_assessment(&contract4);
    print_validation_result(validate_consumer_contract(&contract4));

    // Summary and Recommendations
    print_header("SUMMARY AND NEXT STEPS");

    println!("Contract Analysis Complete\n");
    println!("┌─────────────┬───────────────┬───────────────────┬──────────────┐");
    println!("│ Contract ID │ Risk Score    │ Risk Level        │ SCT Eligible │");
    println!("├─────────────┼───────────────┼───────────────────┼──────────────┤");
    println!(
        "│ {:11} │ {:>13} │ {:17} │ {:12} │",
        contract1.contract_id,
        format!("{}/100", contract1.risk_score),
        "LOW",
        if contract1.is_sct_eligible() {
            "YES"
        } else {
            "NO"
        }
    );
    println!(
        "│ {:11} │ {:>13} │ {:17} │ {:12} │",
        contract2.contract_id,
        format!("{}/100", contract2.risk_score),
        "MEDIUM",
        if contract2.is_sct_eligible() {
            "YES"
        } else {
            "NO"
        }
    );
    println!(
        "│ {:11} │ {:>13} │ {:17} │ {:12} │",
        contract3.contract_id,
        format!("{}/100", contract3.risk_score),
        "HIGH",
        if contract3.is_sct_eligible() {
            "YES"
        } else {
            "NO"
        }
    );
    println!(
        "│ {:11} │ {:>13} │ {:17} │ {:12} │",
        contract4.contract_id,
        format!("{}/100", contract4.risk_score),
        "LOW",
        if contract4.is_sct_eligible() {
            "YES"
        } else {
            "NO"
        }
    );
    println!("└─────────────┴───────────────┴───────────────────┴──────────────┘\n");

    println!("Recommendations:\n");
    println!("1. LOW RISK (0-29): Contract appears reasonable, proceed with normal caution");
    println!("2. MEDIUM RISK (30-69): Review unfair terms, consider negotiation or legal advice");
    println!("3. HIGH RISK (70-100): High likelihood of unfair practices, seek legal advice");
    println!("\nFor disputes:");
    println!("  - Claims ≤ SGD 20,000: File at Small Claims Tribunal (fast, low-cost)");
    println!("  - Claims > SGD 20,000: Consider District/High Court or mediation");
    println!("  - Unfair practices: Report to CASE (Consumers Association of Singapore)");
    println!("\nFor defective goods:");
    println!(
        "  - Within 6 months: Lemon Law applies (repair, replacement, refund, price reduction)"
    );
    println!("  - Beyond 6 months: Sale of Goods Act implied terms may still apply");
}
