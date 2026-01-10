//! Good Faith Acquisition Example (Gutglaeubiger Erwerb)
//!
//! Demonstrates good faith acquisition of movables under German BGB §§932-936.

use chrono::Utc;
use legalis_de::bgb::sachenrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Property Law - Good Faith Acquisition ===\n");
    println!("BGB Sachenrecht - Gutglaeubiger Erwerb §§932-936\n");

    // =========================================================================
    // Example 1: Valid Good Faith Acquisition (§932 BGB)
    // =========================================================================
    println!("Example 1: Good Faith Purchase from Non-Owner");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  1. Anna owns a laptop");
    println!("  2. Anna lends laptop to Boris");
    println!("  3. Boris (non-owner) sells laptop to Clara");
    println!("  4. Clara believes Boris is the owner (good faith)");
    println!("  5. Clara acquires ownership despite Boris not being owner");
    println!();

    let transfer = MovableTransferBuilder::new()
        .transferor("Boris (Non-Owner)", "Berlin")
        .transferee("Clara (Good Faith Buyer)", "Munich")
        .thing("Laptop - Dell XPS 15", Capital::from_euros(1_500))
        .transfer_type(MovableTransferType::ActualDelivery)
        .agreement(Utc::now())
        .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
        .consideration(Capital::from_euros(1_500))
        .good_faith(true)
        .build()
        .unwrap();

    let acquisition = GoodFaithAcquisition {
        transfer,
        transferor_not_owner: true,
        good_faith: true,
        no_gross_negligence: true,
        acquired_through_voluntary_transfer: true,
        acquisition_valid: true,
    };

    println!("Good Faith Acquisition Analysis:");
    println!(
        "  Transferor: {} (not the owner!)",
        acquisition.transfer.transferor.name
    );
    println!("  Acquirer: {}", acquisition.transfer.transferee.name);
    println!("  Object: {}", acquisition.transfer.thing.description);
    println!(
        "  Price: EUR {:.2}",
        acquisition
            .transfer
            .consideration
            .as_ref()
            .unwrap()
            .to_euros()
    );
    println!();

    match validate_good_faith_acquisition(&acquisition) {
        Ok(()) => {
            println!("✅ Good faith acquisition valid per §932 BGB!");
            println!();
            println!("   Requirements met:");
            println!("   ✓ Transferor not the owner");
            println!("   ✓ Acquirer in good faith (guter Glaube)");
            println!("   ✓ No gross negligence (keine grobe Fahrlaessigkeit)");
            println!("   ✓ Voluntary transfer by possessor");
            println!();
            println!("   Result: Clara becomes owner!");
            println!("   Original owner (Anna) loses ownership");
            println!("   §932 protects good faith acquirer over owner");
            println!();
            println!("   Policy rationale:");
            println!("   - Protects commercial transactions");
            println!("   - Promotes confidence in possession");
            println!("   - \"Possession equals ownership in movables\"");
        }
        Err(e) => println!("❌ Acquisition invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Failed - Gross Negligence (§932 Abs. 2 BGB)
    // =========================================================================
    println!("Example 2: Failed Acquisition - Grossly Negligent");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  - Seller offers expensive watch for very low price");
    println!("  - Buyer asks no questions about ownership");
    println!("  - Buyer grossly negligent in not recognizing suspicious circumstances");
    println!("  - No good faith acquisition");
    println!();

    let grossly_negligent_transfer = MovableTransferBuilder::new()
        .transferor("Suspicious Seller", "Berlin")
        .transferee("Negligent Buyer", "Hamburg")
        .thing(
            "Rolex watch (EUR 10,000 value)",
            Capital::from_euros(10_000),
        )
        .transfer_type(MovableTransferType::ActualDelivery)
        .agreement(Utc::now())
        .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
        .consideration(Capital::from_euros(500)) // Suspiciously low!
        .good_faith(true)
        .build()
        .unwrap();

    let failed_acquisition = GoodFaithAcquisition {
        transfer: grossly_negligent_transfer,
        transferor_not_owner: true,
        good_faith: true,
        no_gross_negligence: false, // Grossly negligent!
        acquired_through_voluntary_transfer: true,
        acquisition_valid: false,
    };

    println!("Suspicious Transaction:");
    println!("  Watch value: EUR 10,000");
    println!("  Sale price: EUR 500 (5% of value!)");
    println!("  Buyer asked no questions about ownership");
    println!();

    match validate_good_faith_acquisition(&failed_acquisition) {
        Ok(()) => println!("✅ Acquisition valid (unexpected)"),
        Err(e) => {
            println!("❌ Acquisition fails: {}", e);
            println!();
            println!("   §932 Abs. 2 excludes good faith if:");
            println!("   - Acquirer grossly negligent (grob fahrlaessig)");
            println!("   - Should have known transferor not owner");
            println!();
            println!("   Suspicious circumstances:");
            println!("   - Price far below market value");
            println!("   - No documentation provided");
            println!("   - Unusual sale circumstances");
            println!();
            println!("   Result: Original owner retains ownership");
            println!("   Buyer has claim against seller for breach");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 3: Failed - Stolen Property (§935 BGB)
    // =========================================================================
    println!("Example 3: Stolen Property - §935 BGB Exception");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  - Bicycle stolen from owner");
    println!("  - Thief sells to good faith buyer");
    println!("  - §935 BGB: No good faith acquisition of stolen property");
    println!("  - Original owner can reclaim (Eigentumsherausgabeanspruch)");
    println!();

    let stolen_transfer = MovableTransferBuilder::new()
        .transferor("Thief", "Berlin")
        .transferee("Innocent Buyer", "Munich")
        .thing("Bicycle - stolen from owner", Capital::from_euros(800))
        .transfer_type(MovableTransferType::ActualDelivery)
        .agreement(Utc::now())
        .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
        .consideration(Capital::from_euros(800))
        .good_faith(true)
        .build()
        .unwrap();

    let stolen_acquisition = GoodFaithAcquisition {
        transfer: stolen_transfer,
        transferor_not_owner: true,
        good_faith: true,
        no_gross_negligence: true,
        acquired_through_voluntary_transfer: false, // Stolen!
        acquisition_valid: false,
    };

    println!("Transaction:");
    println!("  Object: Stolen bicycle");
    println!("  Buyer: In good faith, no negligence");
    println!("  Price: Market value");
    println!();

    match validate_good_faith_acquisition(&stolen_acquisition) {
        Ok(()) => println!("✅ Acquisition valid (unexpected)"),
        Err(e) => {
            println!("❌ Acquisition fails: {}", e);
            println!();
            println!("   §935 BGB: Lost or stolen property exception");
            println!("   'Abhanden gekommene Sachen'");
            println!();
            println!("   Rule: Cannot acquire ownership of:");
            println!("   - Stolen property (gestohlen)");
            println!("   - Lost property (verloren)");
            println!("   - Property taken without owner's will");
            println!();
            println!("   Exception to exception (§935 Abs. 2):");
            println!("   - Money (Geld)");
            println!("   - Bearer instruments (Inhaberpapiere)");
            println!("   CAN be acquired in good faith even if stolen");
            println!();
            println!("   Result: Original owner retains ownership");
            println!("   Buyer must return bicycle");
            println!("   Buyer has claim against seller");
        }
    }
    println!("\n");

    // =========================================================================
    // Example 4: Money Exception (§935 Abs. 2 BGB)
    // =========================================================================
    println!("Example 4: Money - Exception to §935");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  - EUR 1,000 cash stolen from owner");
    println!("  - Thief uses cash to buy goods");
    println!("  - Seller receives cash in good faith");
    println!("  - §935 Abs. 2: Money CAN be acquired in good faith");
    println!("  - Seller becomes owner despite money being stolen");
    println!();

    println!("Legal Analysis:");
    println!("  Object: Money (EUR 1,000 cash)");
    println!("  Transferor: Thief (non-owner)");
    println!("  Acquirer: Seller of goods (good faith)");
    println!();
    println!("✅ Acquisition valid per §935 Abs. 2 BGB!");
    println!();
    println!("   §935 Abs. 2 exception for:");
    println!("   - Money (Geld)");
    println!("   - Bearer instruments (Inhaberpapiere)");
    println!();
    println!("   Rationale:");
    println!("   - Money must circulate freely");
    println!("   - Impossible to trace ownership of cash");
    println!("   - Commercial necessity");
    println!();
    println!("   Result: Seller owns the money");
    println!("   Original owner loses ownership");
    println!("   Owner's remedy: Sue thief for damages");
    println!("\n");

    // =========================================================================
    // Example 5: Public Auction Exception (§935 Abs. 2 BGB)
    // =========================================================================
    println!("Example 5: Public Auction - Another §935 Exception");
    println!("----------------------------------------\n");

    println!("Scenario:");
    println!("  - Painting stolen from owner");
    println!("  - Sold at public auction");
    println!("  - Buyer acquires in good faith");
    println!("  - §935 Abs. 2: Public auction exception");
    println!();

    println!("✅ Acquisition valid per §935 Abs. 2 BGB!");
    println!();
    println!("   §935 Abs. 2 allows good faith acquisition of");
    println!("   stolen property purchased at:");
    println!("   - Public auction (oeffentliche Versteigerung)");
    println!("   - Merchant dealing in such goods");
    println!();
    println!("   Requirements:");
    println!("   - Acquired at public auction");
    println!("   - Buyer in good faith");
    println!("   - Not grossly negligent");
    println!();
    println!("   Rationale:");
    println!("   - Protects auction commerce");
    println!("   - Public auctions presumed legitimate");
    println!();
    println!("   Owner's remedy:");
    println!("   - Within 1 year: Can reclaim against compensation");
    println!("   - After 1 year: Lost ownership completely");
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary: Good Faith Acquisition (§§932-936 BGB) ===");
    println!();
    println!("§932 - Basic Rule:");
    println!("  Requirements:");
    println!("  1. Transferor not the owner");
    println!("  2. Acquirer in good faith");
    println!("  3. No gross negligence (§932 Abs. 2)");
    println!("  4. Valid transfer transaction (agreement + delivery)");
    println!();
    println!("§932 Abs. 2 - Gross Negligence:");
    println!("  No protection if acquirer:");
    println!("  - Knew or should have known transferor not owner");
    println!("  - Grossly negligent (grob fahrlaessig)");
    println!();
    println!("§935 - Stolen/Lost Property:");
    println!("  General rule: NO good faith acquisition");
    println!("  Exception: Money and bearer instruments (§935 Abs. 2)");
    println!("  Exception: Public auctions (§935 Abs. 2)");
    println!();
    println!("Policy Balance:");
    println!("  - §932: Protects good faith acquirer (transaction security)");
    println!("  - §935: Protects original owner (property security)");
    println!("  - Money exception: Commercial necessity");
    println!();
    println!("Comparison: Immovables (Land):");
    println!("  - Land: Good faith acquisition per §892 BGB");
    println!("  - Based on land registry, not possession");
    println!("  - NO stolen property exception (§935 only for movables)");
    println!();
    println!("All examples demonstrate correct BGB good faith acquisition!");
}
