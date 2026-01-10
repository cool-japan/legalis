//! Movable Property Transfer Example (Bewegliche Sachen)
//!
//! Demonstrates transfer of movable property under German BGB §§929-936.

use chrono::Utc;
use legalis_de::bgb::sachenrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Property Law - Movable Transfers ===\n");
    println!("BGB Sachenrecht - Uebertragung beweglicher Sachen\n");

    // =========================================================================
    // Example 1: Standard Transfer with Delivery (§929 S. 1 BGB)
    // =========================================================================
    println!("Example 1: Car Sale with Physical Delivery");
    println!("----------------------------------------\n");

    let transfer = MovableTransferBuilder::new()
        .transferor("Max Mustermann", "Berlin")
        .transferee("Erika Schmidt", "Munich")
        .thing(
            "Used car - VW Golf 2020, 50,000 km",
            Capital::from_euros(15_000),
        )
        .transfer_type(MovableTransferType::ActualDelivery)
        .agreement(Utc::now())
        .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
        .consideration(Capital::from_euros(15_000))
        .good_faith(true)
        .build()
        .unwrap();

    println!("Transfer Details:");
    println!("  Transferor: {}", transfer.transferor.name);
    println!("  Transferee: {}", transfer.transferee.name);
    println!("  Object: {}", transfer.thing.description);
    println!(
        "  Price: EUR {:.2}",
        transfer.consideration.as_ref().unwrap().to_euros()
    );
    println!("  Method: Actual Delivery (§929 S. 1 BGB)");
    println!();

    match validate_movable_transfer(&transfer) {
        Ok(()) => {
            println!("✅ Transfer valid per §929 S. 1 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Agreement (Einigung) between parties");
            println!("   ✓ Delivery (Uebergabe) completed");
            println!("   ✓ Transferor has authority");
            println!("   Result: Ownership transferred to Erika Schmidt");
        }
        Err(e) => println!("❌ Transfer invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Constructive Possession Transfer (§930 BGB)
    // =========================================================================
    println!("Example 2: Leased Property Transfer (§930 BGB)");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  - Hans owns an apartment currently leased to tenant");
    println!("  - Hans sells apartment to buyer");
    println!("  - Tenant continues living there");
    println!("  - No physical delivery needed (§930 BGB)");
    println!();

    let constructive_transfer = MovableTransferBuilder::new()
        .transferor("Hans Mueller", "Frankfurt")
        .transferee("Anna Weber", "Hamburg")
        .thing("Furniture in leased apartment", Capital::from_euros(5_000))
        .transfer_type(MovableTransferType::ConstructivePossession)
        .agreement(Utc::now())
        .consideration(Capital::from_euros(5_000))
        .good_faith(true)
        .build()
        .unwrap();

    println!("Transfer Method: Constructive Possession (Besitzkonstitut)");
    println!("  Transferor: {}", constructive_transfer.transferor.name);
    println!("  Transferee: {}", constructive_transfer.transferee.name);
    println!();

    match validate_movable_transfer(&constructive_transfer) {
        Ok(()) => {
            println!("✅ Transfer valid per §930 BGB!");
            println!("   §930: Transfer without delivery when transferee");
            println!("   already has indirect possession through lease");
            println!("   Ownership transfers despite tenant having possession");
        }
        Err(e) => println!("❌ Transfer invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 3: Assignment of Claim (§931 BGB)
    // =========================================================================
    println!("Example 3: Third Party Possession (§931 BGB)");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  - Seller stored goods at warehouse");
    println!("  - Seller transfers ownership to buyer");
    println!("  - Warehouse keeps possession");
    println!("  - Seller assigns claim for return to buyer (§931 BGB)");
    println!();

    let assignment_transfer = MovableTransferBuilder::new()
        .transferor("Company A GmbH", "Berlin")
        .transferee("Company B AG", "Munich")
        .thing(
            "Industrial machinery at warehouse",
            Capital::from_euros(50_000),
        )
        .transfer_type(MovableTransferType::AssignmentOfClaim)
        .agreement(Utc::now())
        .consideration(Capital::from_euros(50_000))
        .good_faith(true)
        .build()
        .unwrap();

    println!("Transfer Details:");
    println!("  Object: {}", assignment_transfer.thing.description);
    println!("  Method: Assignment of herausgabe claim (§931 BGB)");
    println!();

    match validate_movable_transfer(&assignment_transfer) {
        Ok(()) => {
            println!("✅ Transfer valid per §931 BGB!");
            println!("   When third party has possession:");
            println!("   - Agreement between transferor and transferee");
            println!("   - Assignment of claim for return (Herausgabeanspruch)");
            println!("   - No physical delivery required");
        }
        Err(e) => println!("❌ Transfer invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 4: Invalid Transfer - No Delivery
    // =========================================================================
    println!("Example 4: Invalid Transfer - Missing Delivery");
    println!("----------------------------------------\n");

    let mut invalid_transfer = MovableTransferBuilder::new()
        .transferor("Seller", "Berlin")
        .transferee("Buyer", "Munich")
        .thing("Bicycle", Capital::from_euros(500))
        .transfer_type(MovableTransferType::ActualDelivery)
        .agreement(Utc::now())
        .consideration(Capital::from_euros(500))
        .good_faith(true)
        .build()
        .unwrap();

    // Remove delivery to make it invalid
    invalid_transfer.delivery = None;

    println!("Scenario: Agreement reached but no delivery");
    println!();

    match validate_movable_transfer(&invalid_transfer) {
        Ok(()) => println!("✅ Transfer valid (unexpected)"),
        Err(e) => {
            println!("❌ Transfer fails: {}", e);
            println!();
            println!("Explanation:");
            println!("  §929 S. 1 requires BOTH:");
            println!("  1. Agreement (Einigung) ✓");
            println!("  2. Delivery (Uebergabe) ✗");
            println!("  Without delivery, ownership does not transfer");
            println!("  Only obligation to transfer exists (sales contract)");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 5: Possession Analysis
    // =========================================================================
    println!("Example 5: Possession (Besitz §854 BGB)");
    println!("----------------------------------------\n");

    let possession = Possession {
        possessor: PropertyParty {
            name: "Thomas Schmidt".to_string(),
            address: Some("Hamburg".to_string()),
            date_of_birth: None,
            is_natural_person: true,
        },
        thing: Thing {
            description: "Bicycle".to_string(),
            property_type: PropertyType::Movable,
            value: Capital::from_euros(800),
            is_consumable: false,
            is_fungible: false,
            location: Some("Hamburg".to_string()),
        },
        possession_type: PossessionType::DirectPossession,
        acquired_at: Utc::now(),
        factual_control: true,
        possession_will: true,
    };

    println!("Possession Requirements:");
    println!("  Possessor: {}", possession.possessor.name);
    println!("  Object: {}", possession.thing.description);
    println!("  Type: Direct possession (unmittelbarer Besitz)");
    println!();

    match validate_possession(&possession) {
        Ok(()) => {
            println!("✅ Possession valid per §854 BGB!");
            println!("   §854 Requirements:");
            println!("   ✓ Factual control (tatsaechliche Gewalt)");
            println!("   ✓ Possession will (Besitzwille)");
            println!();
            println!("   Possession protection:");
            println!("   - §861 BGB: Return claim if dispossessed");
            println!("   - §862 BGB: Cessation claim if disturbed");
            println!("   - §864 BGB: One-year limitation period");
        }
        Err(e) => println!("❌ Possession invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 6: Pledge (Pfandrecht §1204 BGB)
    // =========================================================================
    println!("Example 6: Movable Pledge (Pfandrecht)");
    println!("----------------------------------------\n");

    let pledge = MovablePledge {
        pledgor: PropertyParty {
            name: "Debtor GmbH".to_string(),
            address: Some("Berlin".to_string()),
            date_of_birth: None,
            is_natural_person: false,
        },
        pledgee: PropertyParty {
            name: "Creditor Bank AG".to_string(),
            address: Some("Frankfurt".to_string()),
            date_of_birth: None,
            is_natural_person: false,
        },
        pledged_thing: Thing {
            description: "Industrial equipment".to_string(),
            property_type: PropertyType::Movable,
            value: Capital::from_euros(100_000),
            is_consumable: false,
            is_fungible: false,
            location: Some("Berlin warehouse".to_string()),
        },
        secured_claim: SecuredClaim {
            claim_description: "Business loan".to_string(),
            claim_amount: Capital::from_euros(80_000),
            interest_rate: Some(5.5),
            maturity_date: None,
            claim_exists: true,
        },
        possession_transferred: true,
        established_at: Utc::now(),
    };

    println!("Pledge Details:");
    println!("  Pledgor: {}", pledge.pledgor.name);
    println!("  Pledgee: {}", pledge.pledgee.name);
    println!("  Object: {}", pledge.pledged_thing.description);
    println!(
        "  Secured amount: EUR {:.2}",
        pledge.secured_claim.claim_amount.to_euros()
    );
    println!();

    match validate_movable_pledge(&pledge) {
        Ok(()) => {
            println!("✅ Pledge valid per §1204 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Agreement between pledgor and pledgee");
            println!("   ✓ Possession transferred to pledgee (§1205 BGB)");
            println!("   ✓ Secured claim exists");
            println!();
            println!("   Effect: Creditor has security interest");
            println!("   If debtor defaults, creditor may sell pledged item");
        }
        Err(e) => println!("❌ Pledge invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary: German Movable Transfer Law ===");
    println!();
    println!("§929 S. 1 - Standard Transfer:");
    println!("  Requirements: Agreement + Delivery + Authority");
    println!("  Most common for everyday transactions");
    println!();
    println!("§930 - Constructive Possession:");
    println!("  Transfer without delivery when acquirer");
    println!("  already has indirect possession");
    println!("  Example: Leased property sales");
    println!();
    println!("§931 - Assignment of Claim:");
    println!("  When third party has possession");
    println!("  Transferor assigns herausgabe claim to acquirer");
    println!();
    println!("§854 - Possession:");
    println!("  Requires: Factual control + Possession will");
    println!("  Protected by §§861-862 BGB");
    println!();
    println!("§1204 - Pledge:");
    println!("  Security interest in movable property");
    println!("  Requires possession transfer to creditor");
    println!();
    println!("All examples demonstrate correct BGB property law!");
}
