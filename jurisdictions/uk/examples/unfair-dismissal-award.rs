//! Unfair Dismissal Award Calculation Examples
//!
//! Demonstrates the two monetary components of a successful unfair-dismissal claim
//! under the Employment Rights Act 1996:
//!
//! - **Basic award (s.119)** — calculated like a statutory redundancy payment using the
//!   age-banded reckoning of s.162, on a week's pay capped at £700 (s.227, April 2024),
//!   over at most 20 years.
//! - **Compensatory award (ss.123-124)** — the tribunal's assessment of the claimant's
//!   financial loss, limited to the lower of 52 weeks' gross pay or the statutory maximum
//!   of £115,115 (April 2024).

use legalis_uk::employment::*;

fn main() {
    println!("=== UK Unfair Dismissal Award Calculator ===\n");
    println!("ERA 1996 ss.118-124\n");
    println!("============================================\n");

    example_typical_claim();
    example_high_earner_capped();
    example_long_serving_older_worker();
}

/// A mid-career claimant with a modest assessed loss below every cap.
fn example_typical_claim() {
    println!("Example 1: Typical claim (loss below all caps)");
    println!("==============================================\n");

    let award = UnfairDismissalAward {
        // s.119: aged 45 with 10 years' service => 4 years @1.5 + 6 years @1.0 = 12 weeks.
        basic_award: BasicAward {
            age: 45,
            years_of_service: 10,
            weekly_pay_gbp: 600.0,
        },
        // s.123/124: tribunal assesses £18,000 of loss; 52 × £600 = £31,200 cap does not bite.
        compensatory_award: CompensatoryAward {
            assessed_loss_gbp: 18_000.0,
            gross_weekly_pay_gbp: 600.0,
        },
    };

    print_award(&award);
}

/// A very high earner whose compensatory award is limited by the statutory maximum.
fn example_high_earner_capped() {
    println!("Example 2: High earner (compensatory award hits statutory maximum)");
    println!("=================================================================\n");

    let award = UnfairDismissalAward {
        basic_award: BasicAward {
            age: 52,
            years_of_service: 12,
            weekly_pay_gbp: 3_000.0, // capped to £700 for the basic award
        },
        compensatory_award: CompensatoryAward {
            // 52 × £3,000 = £156,000, so the £115,115 statutory maximum binds.
            assessed_loss_gbp: 250_000.0,
            gross_weekly_pay_gbp: 3_000.0,
        },
    };

    print_award(&award);
    println!(
        "  Note: compensatory award limited to the statutory maximum £{:.2} (s.124).\n",
        CompensatoryAward::STATUTORY_MAXIMUM_GBP
    );
}

/// A long-serving older worker hitting the maximum basic award.
fn example_long_serving_older_worker() {
    println!("Example 3: Long-serving older worker (maximum basic award)");
    println!("==========================================================\n");

    let award = UnfairDismissalAward {
        // Aged 63 with 25 years (20 reckoned, all at 1.5×) => 30 weeks × £700 = £21,000.
        basic_award: BasicAward {
            age: 63,
            years_of_service: 25,
            weekly_pay_gbp: 900.0,
        },
        compensatory_award: CompensatoryAward {
            // 52 × £900 = £46,800 cap; assessed loss is lower and is paid in full.
            assessed_loss_gbp: 40_000.0,
            gross_weekly_pay_gbp: 900.0,
        },
    };

    print_award(&award);
    println!(
        "  Note: basic award reaches the statutory maximum £{:.2} (s.119).\n",
        BasicAward::statutory_maximum()
    );
}

/// Print the breakdown of an unfair-dismissal award.
fn print_award(award: &UnfairDismissalAward) {
    let reckoning = award.basic_award.reckoning();

    println!("Basic award (ERA 1996 s.119):");
    println!(
        "  Reckoning: {} yr @1.5× + {} yr @1.0× + {} yr @0.5× = {:.1} weeks",
        reckoning.years_at_one_and_half,
        reckoning.years_at_one,
        reckoning.years_at_half,
        reckoning.weeks_due()
    );
    println!(
        "  Week's pay (capped): £{:.2}",
        award.basic_award.capped_weekly_pay()
    );
    println!("  Basic award: £{:.2}\n", award.basic());

    println!("Compensatory award (ERA 1996 ss.123-124):");
    println!(
        "  Assessed loss: £{:.2}",
        award.compensatory_award.assessed_loss_gbp
    );
    println!(
        "  Statutory cap: £{:.2}",
        award.compensatory_award.statutory_cap()
    );
    println!("  Compensatory award: £{:.2}\n", award.compensatory());

    println!("Total compensation: £{:.2}\n", award.total());
}
