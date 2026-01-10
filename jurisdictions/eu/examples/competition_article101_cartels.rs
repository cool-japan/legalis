//! Article 101 TFEU - Anti-competitive Agreements Example
//!
//! Demonstrates detection and validation of cartels, price fixing, market sharing,
//! and Article 101(3) exemptions.

use legalis_eu::competition::*;

fn main() {
    println!("=== Article 101 TFEU - Anti-competitive Agreements ===\n");

    // Example 1: Hardcore price fixing cartel (prohibited)
    println!("Example 1: Price Fixing Cartel (Hardcore Restriction)");
    let agreement1 = Article101Agreement::new()
        .with_parties(vec!["Company A", "Company B", "Company C"])
        .with_practice(ConcertedPractice::PriceFixing {
            agreed_minimum_price: Some(100.0),
            market_share_combined: 0.45,
        })
        .with_affected_member_states(vec![
            MemberState::Germany,
            MemberState::France,
            MemberState::Netherlands,
        ]);

    match agreement1.validate() {
        Ok(result) => {
            println!(
                "   Agreement established: {:?}",
                result.agreement_established
            );
            println!("   Appreciable effect: {:?}", result.appreciable_effect);
            println!(
                "   Affects interstate trade: {:?}",
                result.affects_interstate_trade
            );
            println!(
                "   Hardcore restriction: {}",
                result.is_hardcore_restriction
            );
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 101(1) TFEU");
                println!("   This is a hardcore restriction (price fixing)");
                println!("   Agreement is void under Article 101(2)");
                println!("   Parties face potential fines up to 10% of global turnover");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Market sharing agreement (prohibited)
    println!("Example 2: Geographic Market Sharing (Hardcore Restriction)");
    let agreement2 = Article101Agreement::new()
        .with_parties(vec!["Northern Supplier", "Southern Supplier"])
        .with_practice(ConcertedPractice::MarketSharing {
            allocation_type: MarketAllocation::Geographic(vec![
                "Scandinavia".to_string(),
                "Mediterranean".to_string(),
            ]),
            market_share_combined: 0.60,
        })
        .with_affected_member_states(vec![
            MemberState::Sweden,
            MemberState::Denmark,
            MemberState::Spain,
            MemberState::Italy,
            MemberState::Greece,
        ]);

    match agreement2.validate() {
        Ok(result) => {
            println!(
                "   Hardcore restriction: {}",
                result.is_hardcore_restriction
            );
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 101(1)(c) TFEU");
                println!("   Market sharing eliminates intra-brand competition");
                println!("   Consumers in each region lose competitive alternatives");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 3: De minimis - below threshold (not prohibited)
    println!("Example 3: Small Agreement (De Minimis - Not Prohibited)");
    let agreement3 = Article101Agreement::new()
        .with_parties(vec!["Small Co A", "Small Co B"])
        .with_practice(ConcertedPractice::PriceFixing {
            agreed_minimum_price: Some(50.0),
            market_share_combined: 0.08, // 8% - below 10% threshold
        })
        .with_affected_member_states(vec![MemberState::Belgium]);

    match agreement3.validate() {
        Ok(result) => {
            println!("   Combined market share: 8%");
            println!("   Appreciable effect: {:?}", result.appreciable_effect);
            println!("   De minimis: {}", result.is_de_minimis());
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_de_minimis() {
                println!("\n   ✅ DE MINIMIS RULE APPLIES");
                println!("   Agreement does not appreciably restrict competition");
                println!("   Below 10% threshold (De Minimis Notice 2014)");
                println!("   Not caught by Article 101(1)");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: R&D cooperation with valid Article 101(3) exemption
    println!("Example 4: R&D Cooperation (Article 101(3) Exemption)");
    let agreement4 = Article101Agreement::new()
        .with_parties(vec!["Pharma A", "Pharma B"])
        .with_practice(ConcertedPractice::InformationExchange {
            information_type: "Joint R&D for cancer drug".to_string(),
            strategic: false,
        })
        .with_affected_member_states(vec![MemberState::Germany, MemberState::France])
        .with_exemption_claim(
            Article101Exemption::new()
                .with_efficiency_gains("Accelerates drug development by pooling resources")
                .with_consumer_benefit("New cancer treatment available 3 years earlier")
                .with_indispensable(true)
                .with_competition_remains(true),
        );

    match agreement4.validate() {
        Ok(result) => {
            println!("   Exemption claimed: Yes");
            println!("   Exemption valid: {:?}", result.exemption_valid);
            println!("   Prohibited: {:?}", result.prohibited);

            if !result.is_prohibited() {
                println!("\n   ✅ EXEMPTED under Article 101(3)");
                println!("   All four criteria met:");
                println!("   1. Efficiency gains: Faster drug development");
                println!("   2. Consumer benefit: Earlier access to treatment");
                println!(
                    "   3. Restrictions indispensable: Joint R&D requires information sharing"
                );
                println!("   4. Competition remains: Parties still compete in other markets");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 5: Attempted exemption for hardcore restriction (judicial discretion)
    println!("Example 5: Price Fixing with Exemption Claim (Judicial Discretion)");
    let agreement5 = Article101Agreement::new()
        .with_parties(vec!["Airline A", "Airline B"])
        .with_practice(ConcertedPractice::PriceFixing {
            agreed_minimum_price: Some(200.0),
            market_share_combined: 0.55,
        })
        .with_affected_member_states(vec![
            MemberState::Spain,
            MemberState::Italy,
            MemberState::Greece,
        ])
        .with_exemption_claim(
            Article101Exemption::new()
                .with_efficiency_gains("Reduces empty flights")
                .with_consumer_benefit("Lower average fares")
                .with_indispensable(true)
                .with_competition_remains(true),
        );

    match agreement5.validate() {
        Ok(result) => {
            println!(
                "   Hardcore restriction: {}",
                result.is_hardcore_restriction
            );
            println!("   Exemption valid: {:?}", result.exemption_valid);

            if let legalis_core::LegalResult::JudicialDiscretion {
                issue,
                narrative_hint,
                ..
            } = &result.exemption_valid
            {
                println!("\n   ⚖️  JUDICIAL DISCRETION REQUIRED");
                println!("   Issue: {}", issue);
                if let Some(hint) = narrative_hint {
                    println!("   Guidance: {}", hint);
                }
                println!("\n   Hardcore restrictions rarely qualify for Article 101(3)");
                println!("   Requires exceptional circumstances and substantial evidence");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 6: Information exchange (strategic vs non-strategic)
    println!("Example 6: Strategic Information Exchange");
    let agreement6 = Article101Agreement::new()
        .with_parties(vec!["Competitor X", "Competitor Y", "Competitor Z"])
        .with_practice(ConcertedPractice::InformationExchange {
            information_type: "Future pricing strategies".to_string(),
            strategic: true, // Strategic information
        })
        .with_affected_member_states(vec![MemberState::Germany, MemberState::Austria]);

    match agreement6.validate() {
        Ok(result) => {
            println!("   Information type: Strategic (future pricing)");
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ LIKELY VIOLATION: Article 101(1) TFEU");
                println!("   Exchange of strategic information (prices, costs, customers)");
                println!("   Reduces uncertainty and facilitates collusion");
                println!("   Distinguishable from non-strategic info (e.g., safety data)");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Summary
    println!("=== Article 101 TFEU Key Points ===");
    println!("\n1. Prohibited Practices (Article 101(1)):");
    println!("   - Price fixing (hardcore)");
    println!("   - Market sharing (hardcore)");
    println!("   - Limiting production/technical development");
    println!("   - Applying dissimilar conditions (discrimination)");
    println!("   - Tying arrangements");
    println!("\n2. De Minimis Threshold (2014 Notice):");
    println!("   - Horizontal agreements: ≤10% combined market share");
    println!("   - Vertical agreements: ≤15% each party");
    println!("   - Hardcore restrictions: NO threshold");
    println!("\n3. Article 101(3) Exemption (ALL four criteria required):");
    println!("   1. Efficiency gains OR technical/economic progress");
    println!("   2. Consumers receive fair share of benefit");
    println!("   3. Restrictions indispensable");
    println!("   4. Competition not eliminated");
    println!("\n4. Consequences of Violation:");
    println!("   - Agreement void (Article 101(2))");
    println!("   - Fines up to 10% of global turnover");
    println!("   - Private damages actions");
    println!("   - Leniency for whistleblowers");

    println!("\n=== End of Example ===");
}
