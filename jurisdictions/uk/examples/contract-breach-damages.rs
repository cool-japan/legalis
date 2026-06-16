//! Contract law — breach, remoteness and the measure of damages.
//!
//! Demonstrates the assessment of damages for breach of contract, using the
//! `legalis_uk::contract` library APIs.
//!
//! Two questions are modelled:
//!
//! 1. **Classification of the term breached.** Breach of a *condition* (a term
//!    going to the root of the contract) entitles the innocent party to terminate
//!    and claim damages; breach of a mere *warranty* sounds only in damages; an
//!    *innominate term* depends on the seriousness of the consequences
//!    (*Hong Kong Fir Shipping v Kawasaki Kisen Kaisha* [1962] 2 QB 26).
//!
//! 2. **The recoverable amount.** Damages aim to put the claimant in the position
//!    they would have been in had the contract been performed (*Robinson v Harman*
//!    (1848)). Recovery is limited by:
//!    - **remoteness** — loss must either arise naturally (limb 1) or have been in
//!      the parties' reasonable contemplation (limb 2) at formation
//!      (*Hadley v Baxendale* (1854) 9 Ex 341; *Victoria Laundry v Newman* [1949]);
//!    - the **duty to mitigate** — the claimant cannot recover for avoidable loss
//!      (*British Westinghouse v Underground Electric Railways* [1912]).

use chrono::{TimeZone, Utc};
use legalis_uk::contract::{
    BreachType, ContractBreach, ContractTerm, DamagesCalculation, DamagesMeasure,
    MitigationAnalysis, Party, PartyType, RemotenessAnalysis, TermClassification, TermSource,
    validate_breach,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Contract Law — Breach and Damages ===\n");

    breach_classification()?;
    damages_assessment();

    Ok(())
}

/// Classify breaches of a condition, a warranty and an innominate term.
fn breach_classification() -> Result<(), Box<dyn std::error::Error>> {
    println!("Classification of the term breached");
    println!("-----------------------------------\n");

    let breaching_party = Party {
        name: "Supplier Ltd".to_string(),
        party_type: PartyType::Company,
        age: None,
    };
    let breach_date = Utc
        .with_ymd_and_hms(2024, 5, 1, 9, 0, 0)
        .single()
        .ok_or("invalid breach date")?;

    let condition = ContractBreach {
        breaching_party: breaching_party.clone(),
        term_breached: ContractTerm {
            text: "Goods shall be delivered by the agreed date (time of the essence)".to_string(),
            classification: TermClassification::Condition,
            term_source: TermSource::Express,
        },
        breach_type: BreachType::Fundamental,
        breach_date,
        description: "Delivery 6 weeks late after time was made of the essence".to_string(),
    };
    classify(&condition);

    let warranty = ContractBreach {
        breaching_party: breaching_party.clone(),
        term_breached: ContractTerm {
            text: "The seller will service the equipment annually".to_string(),
            classification: TermClassification::Warranty,
            term_source: TermSource::Express,
        },
        breach_type: BreachType::Minor,
        breach_date,
        description: "One annual service missed".to_string(),
    };
    classify(&warranty);

    let innominate = ContractBreach {
        breaching_party,
        term_breached: ContractTerm {
            text: "The vessel shall be seaworthy".to_string(),
            classification: TermClassification::InnominateTerm,
            term_source: TermSource::ImpliedInLaw,
        },
        breach_type: BreachType::Fundamental,
        breach_date,
        description:
            "Serious unseaworthiness depriving the charterer of substantially the whole benefit"
                .to_string(),
    };
    classify(&innominate);

    Ok(())
}

/// Run `validate_breach` and report whether termination is available.
fn classify(breach: &ContractBreach) {
    println!(
        "  {:?} — {}",
        breach.term_breached.classification, breach.term_breached.text
    );
    match validate_breach(breach) {
        Ok(()) => println!("    -> no actionable breach\n"),
        Err(e) => println!("    -> {e}\n"),
    }
}

/// Compute recoverable damages with Hadley remoteness and a mitigation deduction.
fn damages_assessment() {
    println!("Measure of damages (expectation), remoteness and mitigation");
    println!("-----------------------------------------------------------\n");

    // Limb 1: loss of resale profit arising naturally from late delivery.
    let lost_profit = RemotenessAnalysis::analyze(
        "Lost ordinary resale profit on the consignment",
        12_000.0,
        true, // arises naturally (limb 1)
        true,
        None,
    );

    // Limb 2: an exceptionally lucrative sub-contract, recoverable only if the
    // special circumstances were communicated at formation.
    let lucrative_contract = RemotenessAnalysis::analyze(
        "Profit on an exceptionally lucrative government sub-contract",
        40_000.0,
        false, // does not arise naturally
        true,  // but was in contemplation (special knowledge communicated)
        Some("Buyer told seller at formation of the time-critical government contract"),
    );

    // Too remote: a freak loss neither natural nor contemplated.
    let freak_loss = RemotenessAnalysis::analyze(
        "Loss of an unrelated speculative property deal",
        80_000.0,
        false,
        false,
        None,
    );

    // The claimant reasonably mitigated, saving £5,000 it might otherwise claim.
    let mitigation = MitigationAnalysis::analyze(
        vec!["Sourced replacement goods from an alternative supplier".to_string()],
        true,
        5_000.0,
        5_000.0,
    );

    let calculation = DamagesCalculation::calculate(
        DamagesMeasure::Expectation,
        vec![lost_profit, lucrative_contract, freak_loss],
        mitigation,
    );

    println!("  Measure: {:?}", calculation.measure);
    println!(
        "  Gross losses claimed:    £{:.2}",
        calculation.gross_amount
    );
    println!(
        "  Limb 1 recovery:         £{:.2}",
        calculation.limb1_recovery
    );
    println!(
        "  Limb 2 recovery:         £{:.2}",
        calculation.limb2_recovery
    );
    println!(
        "  Total deductions:        £{:.2}",
        calculation.total_deductions
    );
    println!(
        "  NET recoverable damages: £{:.2}\n",
        calculation.net_damages
    );

    println!("  Per-head remoteness analysis:");
    for head in &calculation.remoteness {
        let status = if head.is_remote {
            "TOO REMOTE"
        } else if head.arises_naturally {
            "recoverable (limb 1)"
        } else {
            "recoverable (limb 2)"
        };
        println!(
            "    - {} (£{:.2}): {status}",
            head.loss_description, head.loss_amount
        );
    }
}
