//! UK GDPR consent validation examples.
//!
//! Demonstrates the conditions for valid consent under UK GDPR, using the
//! `ConsentQuality` type re-exported by `legalis_uk::data_protection` (the UK GDPR
//! reuses the EU GDPR consent model).
//!
//! Consent is one of the six lawful bases for processing (UK GDPR Article 6(1)(a)).
//! To be valid, consent must be (Article 4(11) and Article 7):
//!
//! - **freely given** — a genuine, free choice, with no detriment for refusal and
//!   no imbalance of power that vitiates choice;
//! - **specific** — to distinct, separately consented purposes;
//! - **informed** — the data subject knows the controller's identity and the
//!   purposes of processing;
//! - **unambiguous** — given by a clear affirmative act (not pre-ticked boxes or
//!   silence); and
//! - **easily withdrawable** — it must be as easy to withdraw consent as to give it
//!   (Article 7(3)).
//!
//! All five conditions must be satisfied. `ConsentQuality::is_valid()` returns
//! true only when every condition holds.

use legalis_uk::data_protection::ConsentQuality;

fn main() {
    println!("=== UK GDPR Consent Validation (Articles 4(11) & 7) ===\n");

    assess(
        "Example 1: Valid consent (clear opt-in, easy withdrawal)",
        ConsentQuality {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
            easily_withdrawable: true,
        },
    );

    assess(
        "Example 2: Pre-ticked box (not unambiguous)",
        ConsentQuality {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: false,
            easily_withdrawable: true,
        },
    );

    assess(
        "Example 3: Bundled consent (not specific)",
        ConsentQuality {
            freely_given: true,
            specific: false,
            informed: true,
            unambiguous: true,
            easily_withdrawable: true,
        },
    );

    assess(
        "Example 4: Consent as a condition of service (not freely given)",
        ConsentQuality {
            freely_given: false,
            specific: true,
            informed: true,
            unambiguous: true,
            easily_withdrawable: true,
        },
    );

    assess(
        "Example 5: No easy withdrawal mechanism (Article 7(3) breach)",
        ConsentQuality {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
            easily_withdrawable: false,
        },
    );
}

/// Print the five conditions and the overall validity for one consent record.
fn assess(label: &str, consent: ConsentQuality) {
    println!("{label}");
    println!("{}", "-".repeat(label.len()));
    println!("  freely_given:        {}", consent.freely_given);
    println!("  specific:            {}", consent.specific);
    println!("  informed:            {}", consent.informed);
    println!("  unambiguous:         {}", consent.unambiguous);
    println!("  easily_withdrawable: {}", consent.easily_withdrawable);

    if consent.is_valid() {
        println!("  => VALID consent: a lawful basis under Article 6(1)(a).\n");
    } else {
        println!(
            "  => INVALID consent: at least one Article 4(11)/Article 7 condition is not met; \
             consent cannot be relied upon.\n"
        );
    }
}
