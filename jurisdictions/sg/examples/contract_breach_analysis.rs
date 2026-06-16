//! Contract breach analysis (Singapore common law).
//!
//! Demonstrates the `legalis_sg::contract` module end to end: validating
//! formation, classifying a breach, applying *Hadley v Baxendale* remoteness and
//! the mitigation principle to a damages claim, and testing the availability of
//! specific performance.
//!
//! Run with:
//!
//! ```bash
//! cargo run -p legalis-sg --example contract_breach_analysis
//! ```

use legalis_sg::contract::*;

fn main() {
    println!("=== Singapore Contract Law: Breach Analysis ===\n");

    // 1. Formation -----------------------------------------------------------
    let offer = Offer::new(
        "o-2026-77",
        "Equator Machinery Pte Ltd",
        "Tampines Fabrication Pte Ltd",
        "supply and installation of one CNC milling machine",
    );
    let mut contract = Contract::new("k-2026-77", offer, AgreementContext::Commercial)
        .with_acceptance(Acceptance::new(
            "o-2026-77",
            "Tampines Fabrication Pte Ltd",
            AcceptanceMode::Electronic,
        ));
    contract.add_consideration(Consideration::promise(
        "Equator Machinery Pte Ltd",
        "supply and install the machine",
    ));
    contract.add_consideration(Consideration::promise(
        "Tampines Fabrication Pte Ltd",
        "pay SGD 180,000",
    ));

    match validate_formation(&contract) {
        Ok(()) => println!("[Formation] Valid (Gay Choon Ing v Loh Sze Ti [2009] SGCA 3)."),
        Err(e) => println!("[Formation] FAILED: {e}"),
    }

    // 2. Terms and breach ----------------------------------------------------
    let condition = ContractTerm::new(
        "t-cond",
        "machine to be brand new (not refurbished)",
        TermClassification::Condition,
    );
    let consequence = classify_breach(&condition, false);
    println!(
        "\n[Term] Breach of condition -> may terminate: {} ({})",
        consequence.may_terminate,
        condition.classification.authority()
    );

    let innominate = ContractTerm::new(
        "t-innom",
        "machine to be delivered within 6 weeks",
        TermClassification::Innominate,
    );
    let minor = classify_breach(&innominate, false);
    let serious = classify_breach(&innominate, true);
    println!(
        "[Term] Innominate (minor delay) -> may terminate: {}; (substantial deprivation) -> {}",
        minor.may_terminate, serious.may_terminate
    );

    // 3. Damages with remoteness + mitigation --------------------------------
    let heads = vec![
        HeadOfLoss::ordinary("cost of hiring a replacement machine", 2_400_000),
        // The lost downstream contract was never disclosed to the seller, so it
        // is too remote under the second limb of Hadley v Baxendale.
        HeadOfLoss::special("profit on an undisclosed export contract", 9_000_000, false),
        // Storage charges the buyer could have avoided by mitigating.
        HeadOfLoss::ordinary("avoidable demurrage", 600_000).avoidable(),
    ];
    let award = assess_damages(DamagesMeasure::Expectation, &heads).expect("valid heads");
    println!(
        "\n[Damages] Claimed:     SGD {:.2}",
        award.claimed_cents as f64 / 100.0
    );
    println!("[Damages] Recoverable: SGD {:.2}", award.recoverable_sgd());
    println!("[Damages] Excluded as too remote: {:?}", award.remote_heads);
    println!(
        "[Damages] Excluded for failure to mitigate: {:?}",
        award.unmitigated_heads
    );

    // 4. Specific performance ------------------------------------------------
    let goods = SpecificPerformanceFactors::new();
    let land = SpecificPerformanceFactors::new().unique_subject_matter();
    println!(
        "\n[Specific performance] Ordinary goods available: {} | Unique subject matter (e.g. land) available: {}",
        assess_specific_performance(&goods).is_ok(),
        assess_specific_performance(&land).is_ok()
    );

    println!("\nAnalysis complete.");
}
