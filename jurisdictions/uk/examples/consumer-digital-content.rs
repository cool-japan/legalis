//! Consumer Rights Act 2015 — digital content examples.
//!
//! Demonstrates the statutory rights that apply to the supply of digital content
//! to a consumer under Part 1, Chapter 3 of the Consumer Rights Act 2015
//! (CRA 2015 ss.33-47), using the `legalis_uk::consumer_rights` library APIs.
//!
//! Digital content (software, apps, games, music, video, ebooks) supplied to a
//! consumer must be:
//!
//! - of **satisfactory quality** (s.34);
//! - **fit for a particular purpose** made known to the trader (s.35); and
//! - **as described** (s.36).
//!
//! Where digital content does not conform, the consumer's remedies (ss.42-44)
//! are the right to repair or replacement and, failing that, a price reduction;
//! the trader is also liable for damage caused to a device or other digital
//! content (s.46). The right to satisfactory quality cannot be excluded (s.47).

use chrono::NaiveDate;
use legalis_uk::consumer_rights::{
    Consumer, DigitalContentContract, DigitalContentStatutoryRight, DigitalContentType, Trader,
    validate_digital_content_contract, validate_satisfactory_quality,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Consumer Rights Act 2015 — Digital Content (ss.33-47) ===\n");

    contract_examples()?;
    conformity_examples()?;

    Ok(())
}

/// Build and validate digital-content contracts (the statutory rights that attach).
fn contract_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("Statutory rights attaching to supplied digital content");
    println!("------------------------------------------------------\n");

    let supply_date = NaiveDate::from_ymd_opt(2024, 3, 1).ok_or("invalid supply date")?;

    let trader = Trader {
        name: "Pixel Games Ltd".to_string(),
        address: "10 Studio Row, Brighton, BN1 1AA".to_string(),
        contact: "support@pixelgames.example".to_string(),
        company_number: Some("09876543".to_string()),
    };

    let consumer = Consumer {
        name: "Jordan Lee".to_string(),
        address: "5 Hillside, Leeds, LS1 2BB".to_string(),
        contact: "jordan.lee@example.com".to_string(),
    };

    let game = DigitalContentContract {
        description: "Downloadable strategy game, version 2.1".to_string(),
        price_gbp: 49.99,
        supply_date,
        trader: trader.clone(),
        consumer: consumer.clone(),
        statutory_rights: vec![
            DigitalContentStatutoryRight::SatisfactoryQuality,
            DigitalContentStatutoryRight::FitForPurpose,
            DigitalContentStatutoryRight::AsDescribed,
        ],
        content_type: DigitalContentType::Games,
    };

    println!("Paid game download:");
    print_contract(&game);
    match validate_digital_content_contract(&game) {
        Ok(()) => println!("  validate_digital_content_contract: OK\n"),
        Err(e) => println!("  validate_digital_content_contract: rejected -> {e}\n"),
    }

    // Free digital content (price £0) still carries the statutory rights where it
    // is supplied under a contract for which the consumer gives other consideration.
    let free_app = DigitalContentContract {
        description: "Companion mobile app (free download)".to_string(),
        price_gbp: 0.0,
        supply_date,
        trader,
        consumer,
        statutory_rights: vec![DigitalContentStatutoryRight::SatisfactoryQuality],
        content_type: DigitalContentType::Software,
    };

    println!("Free companion app:");
    print_contract(&free_app);
    match validate_digital_content_contract(&free_app) {
        Ok(()) => println!("  validate_digital_content_contract: OK\n"),
        Err(e) => println!("  validate_digital_content_contract: rejected -> {e}\n"),
    }

    Ok(())
}

/// Print the key fields of a digital-content contract.
fn print_contract(contract: &DigitalContentContract) {
    println!("  Description: {}", contract.description);
    println!("  Price: £{:.2}", contract.price_gbp);
    println!("  Content type: {:?}", contract.content_type);
    print!("  Statutory rights:");
    for right in &contract.statutory_rights {
        print!(" {right:?}");
    }
    println!();
}

/// Demonstrate the s.34 satisfactory-quality conformity test.
fn conformity_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("Section 34 — satisfactory quality conformity test");
    println!("-------------------------------------------------\n");

    println!("  Game runs as expected (satisfactory):");
    print_conformity(validate_satisfactory_quality(
        "Downloadable strategy game",
        "",
        49.99,
        true,
    ));

    println!("  Game crashes on launch (not satisfactory):");
    print_conformity(validate_satisfactory_quality(
        "Downloadable strategy game",
        "Crashes to desktop on every launch; unplayable",
        49.99,
        false,
    ));

    Ok(())
}

/// Print the result of a conformity check.
fn print_conformity(result: Result<(), legalis_uk::consumer_rights::ConsumerRightsError>) {
    match result {
        Ok(()) => println!("    -> conforms: no remedy required\n"),
        Err(e) => println!("    -> non-conforming: {e}\n"),
    }
}
