//! Contract law — the doctrine of consideration.
//!
//! Demonstrates the common-law doctrine of consideration, using the
//! `legalis_uk::contract` library APIs.
//!
//! Consideration is the "price" for which a promise is bought. For a simple
//! (non-deed) contract to be enforceable, each party must provide consideration.
//! The key rules modelled here are:
//!
//! - Consideration must be **sufficient but need not be adequate** — the courts do
//!   not weigh the bargain (*Chappell & Co Ltd v Nestlé Co Ltd* [1960] AC 87:
//!   chocolate wrappers were part of the consideration);
//! - **Past consideration is not good consideration** — an act already performed
//!   before the promise was made cannot support it (*Re McArdle* [1951] Ch 669;
//!   cf. *Lampleigh v Brathwait* (1615) where the act was requested);
//! - Consideration must **move from the promisee** (*Tweddle v Atkinson* (1861)
//!   1 B&S 393), though a third party may now sometimes enforce a term under the
//!   Contracts (Rights of Third Parties) Act 1999;
//! - A pre-existing duty can nonetheless furnish a **practical benefit**
//!   (*Williams v Roffey Bros & Nicholls (Contractors) Ltd* [1991] 1 QB 1).

use legalis_uk::contract::{
    Consideration, ConsiderationType, Party, PartyType, validate_consideration,
};

fn main() {
    println!("=== Contract Law — Doctrine of Consideration ===\n");

    let buyer = Party {
        name: "B. Buyer".to_string(),
        party_type: PartyType::Individual,
        age: Some(34),
    };

    assess(
        "Example 1: Payment of money (good consideration)",
        Consideration {
            description: "Payment of £500 for a second-hand bicycle".to_string(),
            provided_by: buyer.clone(),
            consideration_type: ConsiderationType::Money,
            sufficient: true,
            is_past: false,
        },
    );

    assess(
        "Example 2: Nominal but sufficient (Chappell v Nestlé)",
        Consideration {
            description: "Three chocolate-bar wrappers plus 1s 6d".to_string(),
            provided_by: buyer.clone(),
            consideration_type: ConsiderationType::Act,
            sufficient: true,
            is_past: false,
        },
    );

    assess(
        "Example 3: Past consideration (Re McArdle)",
        Consideration {
            description: "A promise to pay £488 for home improvements already completed"
                .to_string(),
            provided_by: buyer.clone(),
            consideration_type: ConsiderationType::Act,
            sufficient: true,
            is_past: true,
        },
    );

    assess(
        "Example 4: Practical benefit (Williams v Roffey)",
        Consideration {
            description: "Completing carpentry on time, averting a penalty clause".to_string(),
            provided_by: buyer,
            consideration_type: ConsiderationType::PracticalBenefit,
            sufficient: true,
            is_past: false,
        },
    );
}

/// Assess one item of consideration and print the doctrinal outcome.
fn assess(label: &str, consideration: Consideration) {
    println!("{label}");
    println!("{}", "-".repeat(label.len()));
    println!("  Consideration: {}", consideration.description);
    println!("  Type: {:?}", consideration.consideration_type);
    println!("  Sufficient: {}", consideration.sufficient);
    println!("  Past: {}", consideration.is_past);
    println!("  is_valid(): {}", consideration.is_valid());

    match validate_consideration(&consideration) {
        Ok(()) => println!("  validate_consideration: OK — good consideration\n"),
        Err(e) => println!("  validate_consideration: not good consideration -> {e}\n"),
    }
}
