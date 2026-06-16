//! Share Issuance & Dilution Example (Companies Act 1967)
//!
//! Demonstrates share allotment, paid-up capital, ownership percentages and the
//! dilution effect of issuing new shares. Singapore companies issue shares with
//! no par value (Companies Act 1967 s. 62A), which this example models.
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example share_issuance
//! ```

use chrono::Utc;
use legalis_sg::companies::*;

fn main() {
    println!("== Singapore Share Issuance & Dilution (Companies Act 1967) ==\n");

    // Founders hold 100,000 no-par-value ordinary shares; SGD 100,000 paid up.
    let mut capital = ShareCapital::no_par_value(10_000_000, 100_000);
    capital.add_share_class(ShareClass::ordinary(100_000, None));

    let founders = vec![
        shareholder("Founder A", "S1111111A", 60_000),
        shareholder("Founder B", "S2222222B", 40_000),
    ];

    println!(
        "Initial cap table ({} shares issued):",
        capital.issued_shares
    );
    print_cap_table(&founders, capital.issued_shares);
    match validate_share_capital(&capital) {
        Ok(()) => println!("  share capital structure valid\n"),
        Err(e) => println!("  invalid: {}\n", first_line(&e.to_string())),
    }

    // A new investor subscribes for 50,000 new ordinary shares at SGD 2.00 each.
    let new_shares: u64 = 50_000;
    let issue_price_cents: u64 = 200;
    let mut diluted = founders.clone();
    diluted.push(shareholder("Investor C", "202301234B", new_shares));

    let post_money_shares = capital.issued_shares + new_shares;
    println!(
        "After issuing {} new shares to Investor C ({} shares total):",
        new_shares, post_money_shares
    );
    print_cap_table(&diluted, post_money_shares);

    // Dilution effect on Founder A.
    let before = ownership_pct(60_000, capital.issued_shares);
    let after = ownership_pct(60_000, post_money_shares);
    println!(
        "  Founder A diluted from {:.2}% to {:.2}% ({:+.2} pp)",
        before,
        after,
        after - before
    );

    match validate_shareholder_ownership(&diluted, post_money_shares) {
        Ok(()) => println!("  ownership totals are consistent"),
        Err(e) => println!("  inconsistency: {}", first_line(&e.to_string())),
    }

    let raised = new_shares * issue_price_cents;
    println!("  New capital raised: SGD {:.2}", raised as f64 / 100.0);
}

fn shareholder(name: &str, id: &str, shares: u64) -> Shareholder {
    Shareholder {
        name: name.to_string(),
        identification: id.to_string(),
        nationality_or_jurisdiction: "Singapore".to_string(),
        address: Address::singapore("1 Raffles Place", "048616"),
        share_allocation: ShareAllocation::new("Ordinary", shares, 100),
        acquisition_date: Utc::now(),
    }
}

fn print_cap_table(shareholders: &[Shareholder], total_shares: u64) {
    for sh in shareholders {
        println!(
            "  {:<12} {:>7} shares  {:>6.2}%",
            sh.name,
            sh.share_allocation.number_of_shares,
            sh.share_allocation.ownership_percentage(total_shares)
        );
    }
}

fn ownership_pct(shares: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (shares as f64 / total as f64) * 100.0
    }
}

fn first_line(message: &str) -> &str {
    message.lines().next().unwrap_or(message)
}
