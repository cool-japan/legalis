//! Consumer Rights Act 2015, Part 2 — unfair contract terms examples.
//!
//! Demonstrates the fairness test for consumer contract terms under Part 2 of the
//! Consumer Rights Act 2015 (CRA 2015 ss.62-76), using the
//! `legalis_uk::consumer_rights` library APIs.
//!
//! A term is **unfair** if, contrary to the requirement of good faith, it causes a
//! significant imbalance in the parties' rights and obligations to the detriment of
//! the consumer (s.62). An unfair term is **not binding** on the consumer (s.62(1)).
//! Schedule 2 contains an indicative "grey list" of terms that may be regarded as
//! unfair. The fairness test does not apply to a transparent and prominent term to
//! the extent it specifies the main subject matter or the price (s.64); but
//! transparency does not save a grey-list term.

use legalis_uk::consumer_rights::{GreyListItem, UnfairTermAssessment, validate_unfair_term};

fn main() {
    println!("=== Consumer Rights Act 2015, Part 2 — Unfair Terms (ss.62-76) ===\n");

    assess(
        "Example 1: Exclusion of liability for death/personal injury",
        "We accept no liability for any death or personal injury howsoever caused.",
        UnfairTermAssessment {
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            on_grey_list: Some(GreyListItem::ExcludeLiabilityDeathInjury),
            transparent_and_prominent: false,
        },
    );

    assess(
        "Example 2: Automatic-renewal trap",
        "This subscription renews automatically and may not be cancelled.",
        UnfairTermAssessment {
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            on_grey_list: Some(GreyListItem::AutomaticRenewal),
            transparent_and_prominent: false,
        },
    );

    assess(
        "Example 3: A balanced, transparent term (fair)",
        "Either party may cancel on 30 days' written notice.",
        UnfairTermAssessment {
            contrary_to_good_faith: false,
            significant_imbalance: false,
            detriment_to_consumer: false,
            on_grey_list: None,
            transparent_and_prominent: true,
        },
    );

    assess(
        "Example 4: Imbalanced but transparent non-grey term",
        "Refunds are processed within 60 working days of an approved request.",
        UnfairTermAssessment {
            contrary_to_good_faith: true,
            significant_imbalance: true,
            detriment_to_consumer: true,
            on_grey_list: None,
            transparent_and_prominent: true,
        },
    );
}

/// Assess a single term for fairness and print the statutory outcome.
fn assess(label: &str, term_text: &str, assessment: UnfairTermAssessment) {
    println!("{label}");
    println!("{}", "-".repeat(label.len()));
    println!("  Term: \"{term_text}\"");
    println!(
        "  contrary_to_good_faith:    {}",
        assessment.contrary_to_good_faith
    );
    println!(
        "  significant_imbalance:     {}",
        assessment.significant_imbalance
    );
    println!(
        "  detriment_to_consumer:     {}",
        assessment.detriment_to_consumer
    );
    println!("  on_grey_list:              {:?}", assessment.on_grey_list);
    println!(
        "  transparent_and_prominent: {}",
        assessment.transparent_and_prominent
    );
    println!(
        "  unfairness score (0-100):  {}",
        assessment.unfairness_score()
    );

    if assessment.is_unfair() {
        println!("  => UNFAIR: the term is not binding on the consumer (CRA 2015 s.62(1)).");
    } else {
        println!("  => FAIR: the term is binding.");
    }

    match validate_unfair_term(&assessment, term_text) {
        Ok(()) => println!("  validate_unfair_term: OK (term permitted)\n"),
        Err(e) => println!("  validate_unfair_term: rejected -> {e}\n"),
    }
}
