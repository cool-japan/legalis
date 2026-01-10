//! Contract Formation Example (Vertragsschluss)
//!
//! Demonstrates complete contract formation under German BGB (§§145-157 BGB)
//! including offer, acceptance, and validation.

use chrono::Utc;
use legalis_de::bgb::schuldrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Contract Law - Contract Formation Example ===\n");
    println!("BGB Vertragsrecht - Vertragsschluss nach §§145-157 BGB\n");

    // =========================================================================
    // Example 1: Valid Sales Contract Formation
    // =========================================================================
    println!("📋 Example 1: Valid Sales Contract (Kaufvertrag)");
    println!("----------------------------------------\n");

    // Create parties with full legal capacity
    let seller = Party {
        name: "Max Mustermann".to_string(),
        address: "Musterstraße 1, 10115 Berlin".to_string(),
        legal_capacity: LegalCapacity::Full,
        legal_representative: None,
        party_type: PartyType::NaturalPerson,
    };

    let buyer = Party {
        name: "Erika Schmidt".to_string(),
        address: "Beispielweg 5, 80331 München".to_string(),
        legal_capacity: LegalCapacity::Full,
        legal_representative: None,
        party_type: PartyType::NaturalPerson,
    };

    println!("Parties:");
    println!("  Seller: {} ({})", seller.name, seller.address);
    println!("  Buyer:  {} ({})", buyer.name, buyer.address);
    println!();

    // Create offer (Angebot per §145 BGB)
    let offer = Offer {
        offeror: seller.clone(),
        offeree: buyer.clone(),
        terms: ContractTerms {
            subject_matter: "Used car: VW Golf 2020, 50,000 km".to_string(),
            consideration: Some(Capital::from_euros(15_000)),
            essential_terms: vec![
                "Vehicle: VW Golf, 2020 model year".to_string(),
                "Mileage: 50,000 km".to_string(),
                "Price: €15,000".to_string(),
                "Delivery: 14 days after payment".to_string(),
            ],
            additional_terms: vec!["Warranty excluded (sold as-is)".to_string()],
            includes_gtc: false,
        },
        offered_at: Utc::now(),
        acceptance_deadline: Some(Utc::now() + chrono::Duration::days(7)),
        binding: true,
        revoked: false,
    };

    println!("Offer (Angebot §145 BGB):");
    println!("  Subject: {}", offer.terms.subject_matter);
    println!(
        "  Price: €{:.2}",
        offer.terms.consideration.as_ref().unwrap().to_euros()
    );
    println!("  Acceptance deadline: 7 days");
    println!();

    // Validate offer
    match validate_offer(&offer) {
        Ok(()) => println!("✅ Offer valid per §145 BGB"),
        Err(e) => println!("❌ Offer invalid: {}", e),
    }
    println!();

    // Create acceptance (Annahme per §147 BGB)
    let acceptance = Acceptance {
        acceptor: buyer.clone(),
        accepted_at: Utc::now() + chrono::Duration::hours(24),
        modifications: None, // Unconditional acceptance
        timely: true,
    };

    println!("Acceptance (Annahme §147 BGB):");
    println!("  Acceptor: {}", acceptance.acceptor.name);
    println!("  Accepted: 24 hours after offer");
    println!("  Modifications: None (unconditional)");
    println!();

    // Validate acceptance
    match validate_acceptance(&acceptance, &offer) {
        Ok(()) => println!("✅ Acceptance valid per §147 BGB"),
        Err(e) => println!("❌ Acceptance invalid: {}", e),
    }
    println!();

    // Validate contract formation
    match validate_contract_formation(&offer, &acceptance, false, false) {
        Ok(()) => {
            println!("✅ Contract concluded per §§145-157 BGB!");
            println!("   Contract binding upon receipt of acceptance.");
            println!("   Performance obligations arise.");
        }
        Err(e) => println!("❌ Contract not concluded: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Contract with Limited Capacity Party
    // =========================================================================
    println!("📋 Example 2: Contract with Minor (Limited Capacity)");
    println!("----------------------------------------\n");

    let minor = Party {
        name: "Anna Müller".to_string(),
        address: "Hamburg".to_string(),
        legal_capacity: LegalCapacity::Limited, // Age 7-17
        legal_representative: Some("Parent: Thomas Müller".to_string()),
        party_type: PartyType::NaturalPerson,
    };

    println!("Minor party: {} (age 7-17)", minor.name);
    println!(
        "Legal representative: {}",
        minor.legal_representative.as_ref().unwrap()
    );
    println!();

    match validate_party_capacity(&minor) {
        Ok(()) => {
            println!("✅ Minor can enter contract with representative consent (§107 BGB)");
            println!("   Purely beneficial transactions allowed without consent (§107 BGB)");
            println!("   Spending allowances valid (§110 BGB - Taschengeldparagraph)");
        }
        Err(e) => println!("❌ Capacity issue: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 3: Invalid Acceptance (Late Acceptance)
    // =========================================================================
    println!("📋 Example 3: Late Acceptance (Verspätete Annahme)");
    println!("----------------------------------------\n");

    let late_acceptance = Acceptance {
        acceptor: buyer.clone(),
        accepted_at: Utc::now() + chrono::Duration::days(10), // After deadline!
        modifications: None,
        timely: false, // Explicitly marked as late
    };

    println!("Acceptance deadline: 7 days");
    println!("Acceptance received: 10 days after offer");
    println!();

    match validate_acceptance(&late_acceptance, &offer) {
        Ok(()) => println!("✅ Acceptance valid"),
        Err(e) => {
            println!("❌ Late acceptance: {}", e);
            println!("   Effect: Counts as new offer per §150 Abs. 1 BGB");
            println!("   Original offeror can now accept or reject");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 4: Acceptance with Modifications
    // =========================================================================
    println!("📋 Example 4: Acceptance with Modifications");
    println!("----------------------------------------\n");

    let modified_acceptance = Acceptance {
        acceptor: buyer.clone(),
        accepted_at: Utc::now() + chrono::Duration::hours(48),
        modifications: Some(vec![
            "Changed price to €14,000".to_string(),
            "Changed delivery to 7 days".to_string(),
        ]),
        timely: true,
    };

    println!("Original offer: €15,000, 14 days delivery");
    println!("Modified acceptance: €14,000, 7 days delivery");
    println!();

    match validate_acceptance(&modified_acceptance, &offer) {
        Ok(()) => println!("✅ Acceptance valid"),
        Err(e) => {
            println!("❌ Modified acceptance: {}", e);
            println!("   Effect: Rejection + Counter-offer per §150 Abs. 2 BGB");
            println!("   No contract concluded");
            println!("   Modified terms constitute new offer to original offeror");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 5: Service Contract Formation
    // =========================================================================
    println!("📋 Example 5: Service Contract (Dienstvertrag)");
    println!("----------------------------------------\n");

    let consultant = Party {
        name: "Dr. Werner Schmidt (Consultant)".to_string(),
        address: "Frankfurt am Main".to_string(),
        legal_capacity: LegalCapacity::Full,
        legal_representative: None,
        party_type: PartyType::NaturalPerson,
    };

    let company = Party {
        name: "Tech Solutions GmbH".to_string(),
        address: "Berlin".to_string(),
        legal_capacity: LegalCapacity::Full,
        legal_representative: None,
        party_type: PartyType::LegalEntity,
    };

    let service_offer = Offer {
        offeror: consultant.clone(),
        offeree: company.clone(),
        terms: ContractTerms {
            subject_matter: "IT consulting services for 6 months".to_string(),
            consideration: Some(Capital::from_euros(60_000)),
            essential_terms: vec![
                "Service: IT strategy consulting".to_string(),
                "Duration: 6 months".to_string(),
                "Fee: €60,000 (€10,000/month)".to_string(),
                "Start date: Next month".to_string(),
            ],
            additional_terms: vec![
                "Remote work allowed".to_string(),
                "Travel expenses reimbursed".to_string(),
            ],
            includes_gtc: false,
        },
        offered_at: Utc::now(),
        acceptance_deadline: Some(Utc::now() + chrono::Duration::days(14)),
        binding: true,
        revoked: false,
    };

    println!("Service Contract Offer:");
    println!("  Consultant: {}", consultant.name);
    println!("  Client: {}", company.name);
    println!("  Service: {}", service_offer.terms.subject_matter);
    println!(
        "  Fee: €{:.2}",
        service_offer
            .terms
            .consideration
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!();

    let service_acceptance = Acceptance {
        acceptor: company.clone(),
        accepted_at: Utc::now() + chrono::Duration::days(3),
        modifications: None,
        timely: true,
    };

    match validate_contract_formation(&service_offer, &service_acceptance, false, false) {
        Ok(()) => {
            println!("✅ Service contract (Dienstvertrag) concluded!");
            println!("   Contract type: §611 BGB (Dienstvertrag)");
            println!("   Consultant obligated to provide services");
            println!("   Client obligated to pay agreed remuneration");
            println!("   Contract creates continuing obligations");
        }
        Err(e) => println!("❌ Contract error: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary ===");
    println!();
    println!("German Contract Formation (Vertragsschluss) requires:");
    println!("  1. Valid Offer (Angebot) per §145 BGB");
    println!("     - Sufficiently specific terms (essentialia negotii)");
    println!("     - Intent to be bound");
    println!("     - Communication to offeree");
    println!();
    println!("  2. Valid Acceptance (Annahme) per §147 BGB");
    println!("     - Unconditional");
    println!("     - Timely (within deadline or reasonable time)");
    println!("     - By authorized person");
    println!();
    println!("  3. Legal Capacity (Geschäftsfähigkeit) per §§104-115 BGB");
    println!("     - Full capacity: Age 18+");
    println!("     - Limited capacity: Age 7-17 (requires consent)");
    println!("     - No capacity: Under age 7 (void)");
    println!();
    println!("Special Rules:");
    println!("  - Late acceptance = new offer (§150 Abs. 1 BGB)");
    println!("  - Modified acceptance = rejection + counter-offer (§150 Abs. 2 BGB)");
    println!("  - Silence generally not acceptance (§151 BGB)");
    println!("  - Form requirements if specified (§§125-129 BGB)");
    println!();
    println!("✅ All examples demonstrate correct BGB contract formation principles!");
}
