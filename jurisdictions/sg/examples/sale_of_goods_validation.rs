//! Sale of Goods Validation Example
//!
//! This example demonstrates validation of sales under the Sale of Goods Act (Cap. 393):
//! 1. Implied terms (s. 13, 14(2), 14(3), 15)
//! 2. Lemon Law applicability
//! 3. Defect reporting and remedies
//! 4. Merchantable quality assessment
//!
//! ## Legal Context
//!
//! Sale of Goods Act (Cap. 393) Implied Terms:
//! - **s. 13**: Goods must correspond to description
//! - **s. 14(2)**: Merchantable quality (seller in business)
//! - **s. 14(3)**: Fitness for particular purpose (if purpose made known)
//! - **s. 15**: Sale by sample (bulk corresponds to sample)
//!
//! Lemon Law (2012):
//! - Applies to defective goods within 6 months
//! - Remedies: Repair, replacement, price reduction, refund

use legalis_sg::consumer::*;

fn print_header(title: &str) {
    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║ {:<65} ║", title);
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");
}

fn print_sale_summary(sale: &SaleOfGoods) {
    println!("Contract ID: {}", sale.contract_id);
    println!("Goods: {}", sale.goods_description);
    println!(
        "Seller in Business: {}",
        if sale.seller_in_business { "Yes" } else { "No" }
    );
    if let Some(ref purpose) = sale.particular_purpose {
        println!("Particular Purpose: {}", purpose);
    }
    println!(
        "Sale by Sample: {}",
        if sale.sale_by_sample { "Yes" } else { "No" }
    );
    println!(
        "Lemon Law Applicable: {}",
        if sale.is_lemon_law_applicable() {
            "Yes (within 6 months)"
        } else {
            "No (beyond 6 months)"
        }
    );
}

fn print_implied_terms(sale: &SaleOfGoods) {
    println!("\nImplied Terms:");
    for (idx, term) in sale.implied_terms.iter().enumerate() {
        println!(
            "  {}. [{}] {}",
            idx + 1,
            term.statute_reference(),
            term.description()
        );
    }
}

fn print_defect_status(sale: &SaleOfGoods) {
    println!("\nDefect Status:");
    if sale.is_defective {
        println!("  ❌ DEFECTIVE");
        if let Some(ref defect) = sale.defect_description {
            println!("  Description: {}", defect);
        }
    } else {
        println!("  ✅ No defects reported");
    }
}

fn print_recommended_remedies(sale: &SaleOfGoods) {
    let remedies = recommend_remedy(sale);

    println!("\nAvailable Remedies:");
    if remedies.is_empty() {
        println!("  N/A (no defects)");
    } else {
        for (idx, remedy) in remedies.iter().enumerate() {
            let description = match remedy {
                ConsumerRemedy::Repair => "Repair - Supplier must fix the defect",
                ConsumerRemedy::Replacement => "Replacement - Provide a new identical item",
                ConsumerRemedy::PriceReduction => "Price Reduction - Partial refund for defect",
                ConsumerRemedy::Refund => "Full Refund - Return goods and get money back",
                ConsumerRemedy::Rescission => "Rescission - Cancel the contract",
                ConsumerRemedy::Damages => "Damages - Monetary compensation for loss",
            };
            println!("  {}. {:?} - {}", idx + 1, remedy, description);
        }
    }
}

fn print_validation_result(sale: &SaleOfGoods) {
    println!("\nValidation Result:");
    match validate_sale_of_goods(sale) {
        Ok(()) => println!("  ✅ Sale passes validation"),
        Err(e) => {
            println!("  ❌ Validation failed:");
            println!("     {}", e);
            if let Some(statute) = e.statute_reference() {
                println!("     Statute: {}", statute);
            }
        }
    }
}

fn main() {
    print_header("SALE OF GOODS VALIDATION - SINGAPORE");

    // Scenario 1: Business sale with merchantable quality
    print_header("Scenario 1: Standard Business Sale (All Terms Apply)");

    let sale1 = SaleOfGoods::new(
        "S001",
        true, // Seller in business
        "Samsung 55\" 4K Smart TV",
    );

    print_sale_summary(&sale1);
    print_implied_terms(&sale1);
    print_defect_status(&sale1);
    print_recommended_remedies(&sale1);
    print_validation_result(&sale1);

    // Scenario 2: Sale with particular purpose - fitness for purpose applies
    print_header("Scenario 2: Sale with Particular Purpose (s. 14(3) Applies)");

    let mut sale2 = SaleOfGoods::new("S002", true, "Industrial paint - 20 liters");
    sale2.particular_purpose =
        Some("Coating outdoor metal structures in marine environment".to_string());

    print_sale_summary(&sale2);
    print_implied_terms(&sale2);
    print_defect_status(&sale2);
    print_recommended_remedies(&sale2);
    print_validation_result(&sale2);

    // Scenario 3: Defective goods within Lemon Law period
    print_header("Scenario 3: Defective Goods (Lemon Law Applies)");

    let mut sale3 = SaleOfGoods::new("S003", true, "Dyson V15 Vacuum Cleaner");
    sale3.report_defect("Motor fails to start after 2 weeks of use");

    print_sale_summary(&sale3);
    print_implied_terms(&sale3);
    print_defect_status(&sale3);
    print_recommended_remedies(&sale3);
    print_validation_result(&sale3);

    // Scenario 4: Not merchantable quality - goods defective
    print_header("Scenario 4: Not Merchantable Quality (SOGA s. 14(2) Breach)");

    let mut sale4 = SaleOfGoods::new("S004", true, "Apple MacBook Pro 16\" laptop");
    sale4.report_defect("Screen has dead pixels and keyboard keys are sticking");

    print_sale_summary(&sale4);
    print_implied_terms(&sale4);
    print_defect_status(&sale4);
    print_recommended_remedies(&sale4);
    print_validation_result(&sale4);

    // Scenario 5: Sale by sample
    print_header("Scenario 5: Sale by Sample (SOGA s. 15 Applies)");

    let mut sale5 = SaleOfGoods::new("S005", true, "Office chairs - bulk order of 50 units");
    sale5.sale_by_sample = true;

    print_sale_summary(&sale5);
    print_implied_terms(&sale5);
    print_defect_status(&sale5);
    print_recommended_remedies(&sale5);
    print_validation_result(&sale5);

    // Scenario 6: Private sale (not in business) - limited implied terms
    print_header("Scenario 6: Private Sale (s. 14(2) Does Not Apply)");

    let sale6 = SaleOfGoods::new(
        "S006",
        false, // Private seller, not in business
        "Used iPhone 12 Pro",
    );

    print_sale_summary(&sale6);
    print_implied_terms(&sale6);
    println!("\n⚠️  Note: s. 14(2) merchantable quality does not apply to private sales!");
    println!("   Only s. 13 (corresponds to description) applies.");
    print_defect_status(&sale6);
    print_recommended_remedies(&sale6);
    print_validation_result(&sale6);

    // Scenario 7: Fitness for purpose breach
    print_header("Scenario 7: Not Fit for Particular Purpose (s. 14(3) Breach)");

    let mut sale7 = SaleOfGoods::new("S007", true, "Laptop computer - budget model");
    sale7.particular_purpose = Some("Professional video editing and 3D rendering".to_string());
    sale7.report_defect(
        "Computer freezes when running video editing software - insufficient RAM and GPU",
    );

    print_sale_summary(&sale7);
    print_implied_terms(&sale7);
    print_defect_status(&sale7);
    print_recommended_remedies(&sale7);
    print_validation_result(&sale7);

    println!("\n⚠️  Note: This would trigger ConsumerError::NotFitForPurpose if we validated it");
    println!(
        "   The seller should have warned that the budget model is unsuitable for professional use."
    );

    // Summary and practical guidance
    print_header("SUMMARY AND PRACTICAL GUIDANCE");

    println!("Implied Terms Summary:\n");
    println!("┌──────────┬────────────────────────────┬────────────────────────────┐");
    println!("│ Section  │ Implied Term               │ When It Applies            │");
    println!("├──────────┼────────────────────────────┼────────────────────────────┤");
    println!("│ s. 13    │ Corresponds to description │ ALWAYS                     │");
    println!("│ s. 14(2) │ Merchantable quality       │ Seller in business         │");
    println!("│ s. 14(3) │ Fit for particular purpose │ Purpose made known         │");
    println!("│ s. 15    │ Sale by sample             │ Explicitly by sample       │");
    println!("└──────────┴────────────────────────────┴────────────────────────────┘\n");

    println!("Lemon Law Remedies (within 6 months):\n");
    println!("1. REPAIR - Supplier must fix the defect (reasonable time)");
    println!("2. REPLACEMENT - Supplier must provide new identical goods");
    println!("3. PRICE REDUCTION - Consumer gets partial refund for defect");
    println!("4. FULL REFUND - Consumer returns goods and gets full refund");
    println!("\nConsumer must:");
    println!("  - Report defect within 6 months of delivery");
    println!("  - Give supplier reasonable opportunity to remedy");
    println!("  - Choose remedy (not all remedies may be available)\n");

    println!("What to Do if Goods are Defective:\n");
    println!("Step 1: Document the defect (photos, videos, written description)");
    println!("Step 2: Contact seller immediately - give chance to remedy");
    println!("Step 3: If within 6 months, invoke Lemon Law rights");
    println!("Step 4: If seller refuses, file complaint with CASE");
    println!("Step 5: Consider Small Claims Tribunal (≤ SGD 20,000)\n");

    println!("Small Claims Tribunal:");
    println!("  - Fast, low-cost dispute resolution");
    println!("  - Claims up to SGD 20,000 (or SGD 30,000 with consent)");
    println!("  - No lawyers required");
    println!("  - Filing fee: SGD 10");
    println!("  - Website: https://www.judiciary.gov.sg/sct\n");

    println!("Consumer Association of Singapore (CASE):");
    println!("  - Free mediation for disputes");
    println!("  - Consumer education and advice");
    println!("  - Hotline: 6100 0315");
    println!("  - Website: https://www.case.org.sg\n");
}
