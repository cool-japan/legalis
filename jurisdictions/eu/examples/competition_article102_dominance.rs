//! Article 102 TFEU - Abuse of Dominant Position Example
//!
//! Demonstrates dominance assessment and various types of abusive conduct
//! (predatory pricing, refusal to deal, tying, margin squeeze, etc.)

use legalis_eu::competition::*;

fn main() {
    println!("=== Article 102 TFEU - Abuse of Dominant Position ===\n");

    // Example 1: Predatory pricing (below average variable cost)
    println!("Example 1: Predatory Pricing - Below AVC (Presumed Abusive)");
    let market1 = RelevantMarket {
        product_market: "Budget airline routes".to_string(),
        geographic_market: GeographicMarket::RegionalMarket(vec![
            MemberState::Spain,
            MemberState::Portugal,
        ]),
        market_share: 0.65,
    };

    let conduct1 = Article102Conduct::new()
        .with_undertaking("Dominant Airline")
        .with_relevant_market(market1.clone())
        .with_abuse(AbuseType::Exclusionary(
            ExclusionaryAbuse::PredatoryPricing {
                price: 25.0,
                average_variable_cost: 40.0, // Pricing below AVC!
            },
        ));

    match conduct1.validate() {
        Ok(result) => {
            println!(
                "   Market share: {:.1}%",
                result.dominance.market_share * 100.0
            );
            println!("   Dominant: {}", result.dominance.is_dominant);
            println!("   Assessment: {}", result.dominance.assessment);
            println!("   Abuse established: {:?}", result.abuse_established);
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 102 TFEU");
                println!("   Predatory pricing below average variable cost");
                println!("   AKZO test: Pricing below AVC presumed abusive");
                println!("   Designed to eliminate competitors, then recoup losses");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Essential facility refusal to deal
    println!("Example 2: Refusal to Deal - Essential Facility (Abusive)");
    let market2 = RelevantMarket {
        product_market: "Port infrastructure".to_string(),
        geographic_market: GeographicMarket::NationalMarket(MemberState::Netherlands),
        market_share: 0.95, // Near monopoly
    };

    let conduct2 = Article102Conduct::new()
        .with_undertaking("Rotterdam Port Authority")
        .with_relevant_market(market2)
        .with_abuse(AbuseType::Exclusionary(ExclusionaryAbuse::RefusalToDeal {
            customer: "New Shipping Line".to_string(),
            essential_facility: true,
        }));

    match conduct2.validate() {
        Ok(result) => {
            println!(
                "   Market share: {:.1}%",
                result.dominance.market_share * 100.0
            );
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 102 TFEU");
                println!("   Essential facility doctrine applies");
                println!("   Port is indispensable for shipping business");
                println!("   Refusal likely to eliminate competition downstream");
                println!("   Bronner test satisfied");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 3: Tying arrangement (Microsoft case pattern)
    println!("Example 3: Tying Arrangement (Abusive)");
    let market3 = RelevantMarket {
        product_market: "Operating systems".to_string(),
        geographic_market: GeographicMarket::EuWide,
        market_share: 0.88,
    };

    let conduct3 = Article102Conduct::new()
        .with_undertaking("OS Monopolist")
        .with_relevant_market(market3)
        .with_abuse(AbuseType::Exclusionary(ExclusionaryAbuse::Tying {
            tying_product: "Operating system".to_string(),
            tied_product: "Media player".to_string(),
        }));

    match conduct3.validate() {
        Ok(result) => {
            println!(
                "   Market share: {:.1}%",
                result.dominance.market_share * 100.0
            );
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 102 TFEU");
                println!("   Microsoft-type tying arrangement");
                println!("   Conditions:");
                println!("   - Dominant in tying product (OS)");
                println!("   - Products are separate");
                println!("   - No consumer choice");
                println!("   - Forecloses competition in tied market (media players)");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: Margin squeeze (telecommunications case)
    println!("Example 4: Margin Squeeze - Negative Margin (Abusive)");
    let market4 = RelevantMarket {
        product_market: "Broadband infrastructure".to_string(),
        geographic_market: GeographicMarket::NationalMarket(MemberState::Germany),
        market_share: 0.72,
    };

    let conduct4 = Article102Conduct::new()
        .with_undertaking("Telecom Incumbent")
        .with_relevant_market(market4)
        .with_abuse(AbuseType::Exclusionary(ExclusionaryAbuse::MarginSqueeze {
            wholesale_price: 40.0,             // Price to competitors
            retail_price: 50.0,                // Own retail price
            downstream_competitor_costs: 15.0, // Competitor's additional costs
        }));

    match conduct4.validate() {
        Ok(result) => {
            println!("   Wholesale price: €40");
            println!("   Retail price: €50");
            println!("   Competitor costs: €15");
            println!("   Competitor margin: €{} (negative!)", 50.0 - 40.0 - 15.0);
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 102 TFEU");
                println!("   Margin squeeze detected");
                println!("   As-efficient competitor cannot operate profitably");
                println!("   Margin: Retail (€50) - Wholesale (€40) - Costs (€15) = €-5");
                println!("   Deutsche Telekom test: Negative margin = abusive");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 5: Excessive pricing (exploitative abuse)
    println!("Example 5: Excessive Pricing (Judicial Discretion)");
    let market5 = RelevantMarket {
        product_market: "Pharmaceutical - life-saving drug".to_string(),
        geographic_market: GeographicMarket::EuWide,
        market_share: 1.0, // Monopoly
    };

    let conduct5 = Article102Conduct::new()
        .with_undertaking("Pharma Monopoly")
        .with_relevant_market(market5)
        .with_abuse(AbuseType::Exploitative(ExploitativeAbuse::UnfairPricing {
            price: 1000.0,
            competitive_price: 700.0,
            excessive_percentage: 0.43, // 43% above competitive level
        }));

    match conduct5.validate() {
        Ok(result) => {
            println!("   Price: €1000");
            println!("   Competitive price: €700");
            println!("   Excessive percentage: 43%");
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 102(a) TFEU");
                println!("   Unfair pricing (exploitative abuse)");
                println!("   United Brands test:");
                println!("   Price significantly exceeds competitive level (>20%)");
                println!("   No reasonable relation to economic value");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Example 6: No dominance (not Article 102 violation)
    println!("Example 6: No Dominant Position (Not Prohibited)");
    let market6 = RelevantMarket {
        product_market: "Smartphones".to_string(),
        geographic_market: GeographicMarket::EuWide,
        market_share: 0.28, // Below 40% threshold
    };

    let conduct6 = Article102Conduct::new()
        .with_undertaking("Phone Maker")
        .with_relevant_market(market6)
        .with_abuse(AbuseType::Exploitative(ExploitativeAbuse::UnfairPricing {
            price: 800.0,
            competitive_price: 600.0,
            excessive_percentage: 0.33,
        }));

    match conduct6.validate() {
        Ok(result) => {
            println!("   Prohibited: {:?}", result.prohibited);
        }
        Err(e) => {
            println!("   Error: {}", e);
            println!("\n   ✅ NO VIOLATION");
            println!("   Market share 28% below dominance threshold (40%)");
            println!("   Article 102 requires dominant position");
            println!("   This is normal competitive pricing");
        }
    }

    println!("\n---\n");

    // Example 7: Exclusive dealing with significant foreclosure
    println!("Example 7: Exclusive Dealing - Long Duration (Abusive)");
    let market7 = RelevantMarket {
        product_market: "Ice cream freezers in retail".to_string(),
        geographic_market: GeographicMarket::NationalMarket(MemberState::Ireland),
        market_share: 0.75,
    };

    let conduct7 = Article102Conduct::new()
        .with_undertaking("Ice Cream Giant")
        .with_relevant_market(market7)
        .with_abuse(AbuseType::Exclusionary(
            ExclusionaryAbuse::ExclusiveDealing {
                duration_months: 36,                 // 3 years
                market_foreclosure_percentage: 0.40, // 40% of outlets
            },
        ));

    match conduct7.validate() {
        Ok(result) => {
            println!("   Duration: 36 months");
            println!("   Market foreclosure: 40%");
            println!("   Prohibited: {:?}", result.prohibited);

            if result.is_prohibited() {
                println!("\n   ❌ VIOLATION: Article 102 TFEU");
                println!("   Van den Bergh Foods pattern (HB Ice Cream case)");
                println!("   Long-term exclusive dealing (>2 years)");
                println!("   Significant market foreclosure (>30%)");
                println!("   Competitors cannot access retail outlets");
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\n---\n");

    // Summary
    println!("=== Article 102 TFEU Key Points ===");
    println!("\n1. Two Elements Required:");
    println!("   a) Dominant position (typically market share >40%)");
    println!("      - >50%: Presumed dominant (AKZO)");
    println!("      - 40-50%: May be dominant");
    println!("      - <40%: Rarely dominant");
    println!("   b) Abuse of that position");
    println!("\n2. Types of Abuse:");
    println!("   Exploitative (Article 102(a)-(b)):");
    println!("   - Unfair pricing (excessive or discriminatory)");
    println!("   - Limiting production/technical development");
    println!("   \n   Exclusionary (Article 102(c)-(d)):");
    println!("   - Predatory pricing (below cost)");
    println!("   - Refusal to deal (essential facilities)");
    println!("   - Tying and bundling");
    println!("   - Exclusive dealing");
    println!("   - Margin squeeze");
    println!("   - Discrimination");
    println!("\n3. Key Tests:");
    println!("   - AKZO: Predatory pricing");
    println!("   - Bronner: Refusal to supply");
    println!("   - United Brands: Excessive pricing");
    println!("   - Microsoft: Tying");
    println!("   - Deutsche Telekom: Margin squeeze");
    println!("\n4. Consequences:");
    println!("   - Fines up to 10% of global turnover");
    println!("   - Structural remedies (divestiture)");
    println!("   - Behavioral remedies (commitments)");
    println!("   - Private damages actions");

    println!("\n=== End of Example ===");
}
