//! Immovable Property Transfer Example (Grundstuecke)
//!
//! Demonstrates transfer of immovable property (land) under German BGB §§873-902.

use chrono::{NaiveDate, Utc};
use legalis_de::bgb::sachenrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Property Law - Immovable Transfers ===\n");
    println!("BGB Sachenrecht - Grundstuecksuebertragung\n");

    // =========================================================================
    // Example 1: Valid Land Transfer (§873 BGB)
    // =========================================================================
    println!("Example 1: Residential Property Transfer");
    println!("----------------------------------------\n");

    let land_transfer = ImmovableTransfer {
        transferor: PropertyParty {
            name: "Hans Mueller".to_string(),
            address: Some("Berlin".to_string()),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1975, 3, 15).unwrap()),
            is_natural_person: true,
        },
        transferee: PropertyParty {
            name: "Erika Schmidt".to_string(),
            address: Some("Munich".to_string()),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1982, 7, 22).unwrap()),
            is_natural_person: true,
        },
        land_parcel: LandParcel {
            parcel_number: "123/45".to_string(),
            land_registry_district: "Berlin-Mitte".to_string(),
            size_square_meters: 500,
            location: "Alexanderplatz 1, 10178 Berlin".to_string(),
            description: "Residential property with house".to_string(),
            value: Capital::from_euros(750_000),
        },
        agreement: TransferAgreement {
            agreement_reached: true,
            agreed_at: Utc::now(),
            transfer_intent: true,
            acceptance_intent: true,
        },
        registration: LandRegistryEntry {
            registered: true,
            registration_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            registry_office: "Amtsgericht Berlin-Mitte".to_string(),
            section: LandRegistrySection::SectionI,
            entry_number: Some("Blatt 1234".to_string()),
        },
        consideration: Some(Capital::from_euros(750_000)),
        transferred_at: Utc::now(),
    };

    println!("Property Details:");
    println!(
        "  Parcel Number: {}",
        land_transfer.land_parcel.parcel_number
    );
    println!(
        "  District: {}",
        land_transfer.land_parcel.land_registry_district
    );
    println!("  Location: {}", land_transfer.land_parcel.location);
    println!(
        "  Size: {} sqm",
        land_transfer.land_parcel.size_square_meters
    );
    println!(
        "  Value: EUR {:.2}",
        land_transfer.land_parcel.value.to_euros()
    );
    println!();
    println!("Transfer Parties:");
    println!("  Transferor: {}", land_transfer.transferor.name);
    println!("  Transferee: {}", land_transfer.transferee.name);
    println!(
        "  Purchase Price: EUR {:.2}",
        land_transfer.consideration.as_ref().unwrap().to_euros()
    );
    println!();

    match validate_immovable_transfer(&land_transfer) {
        Ok(()) => {
            println!("✅ Land transfer valid per §873 BGB!");
            println!("   Requirements met:");
            println!("   ✓ Agreement (Einigung) between parties");
            println!("   ✓ Land registry entry (Grundbucheintragung)");
            println!("   ✓ Valid parcel specification");
            println!();
            println!("   §873 Abs. 1: Ownership transfers upon registration");
            println!("   Registration at: Amtsgericht Berlin-Mitte");
            println!(
                "   Entry: Section I (Owner), {}",
                land_transfer.registration.entry_number.as_ref().unwrap()
            );
        }
        Err(e) => println!("❌ Transfer invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 2: Commercial Property with Mortgage
    // =========================================================================
    println!("Example 2: Commercial Property with Mortgage");
    println!("----------------------------------------\n");

    let commercial_parcel = LandParcel {
        parcel_number: "456/78".to_string(),
        land_registry_district: "Frankfurt-Stadt".to_string(),
        size_square_meters: 2_000,
        location: "Zeil 100, 60313 Frankfurt".to_string(),
        description: "Commercial building - retail space".to_string(),
        value: Capital::from_euros(3_000_000),
    };

    println!("Property: {}", commercial_parcel.description);
    println!("  Location: {}", commercial_parcel.location);
    println!("  Size: {} sqm", commercial_parcel.size_square_meters);
    println!("  Value: EUR {:.2}", commercial_parcel.value.to_euros());
    println!();

    let mortgage = Mortgage {
        land_parcel: commercial_parcel.clone(),
        creditor: PropertyParty {
            name: "Deutsche Bank AG".to_string(),
            address: Some("Frankfurt".to_string()),
            date_of_birth: None,
            is_natural_person: false,
        },
        debtor: PropertyParty {
            name: "Retail Company GmbH".to_string(),
            address: Some("Frankfurt".to_string()),
            date_of_birth: None,
            is_natural_person: false,
        },
        secured_claim: SecuredClaim {
            claim_description: "Commercial property loan".to_string(),
            claim_amount: Capital::from_euros(2_000_000),
            interest_rate: Some(3.5),
            maturity_date: Some(NaiveDate::from_ymd_opt(2044, 1, 1).unwrap()),
            claim_exists: true,
        },
        mortgage_amount: Capital::from_euros(2_000_000),
        priority_rank: 1,
        registered_at: Utc::now(),
        registry_entry: LandRegistryEntry {
            registered: true,
            registration_date: Some(NaiveDate::from_ymd_opt(2024, 1, 20).unwrap()),
            registry_office: "Amtsgericht Frankfurt".to_string(),
            section: LandRegistrySection::SectionIII,
            entry_number: Some("III-1".to_string()),
        },
    };

    println!("Mortgage Details:");
    println!("  Creditor: {}", mortgage.creditor.name);
    println!("  Debtor: {}", mortgage.debtor.name);
    println!("  Amount: EUR {:.2}", mortgage.mortgage_amount.to_euros());
    println!(
        "  Interest: {}%",
        mortgage.secured_claim.interest_rate.unwrap()
    );
    println!("  Priority Rank: {}", mortgage.priority_rank);
    println!();

    match validate_mortgage(&mortgage) {
        Ok(()) => {
            println!("✅ Mortgage valid per §1113 BGB!");
            println!("   Characteristics:");
            println!("   - Accessory to loan claim (akzessorisch)");
            println!("   - Registered in Section III of land registry");
            println!("   - Priority rank 1 (highest priority)");
            println!("   - Secures EUR 2,000,000 loan");
            println!();
            println!("   If debtor defaults:");
            println!("   - Creditor may foreclose (Zwangsversteigerung)");
            println!("   - Mortgage holder paid from sale proceeds");
            println!("   - Priority rank determines payment order");
        }
        Err(e) => println!("❌ Mortgage invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 3: Land Charge (Grundschuld)
    // =========================================================================
    println!("Example 3: Land Charge (Grundschuld §1191 BGB)");
    println!("----------------------------------------\n");

    let land_charge = LandCharge {
        land_parcel: LandParcel {
            parcel_number: "789/12".to_string(),
            land_registry_district: "Munich-Nord".to_string(),
            size_square_meters: 800,
            location: "Leopoldstrasse 50, 80802 Munich".to_string(),
            description: "Residential apartment building".to_string(),
            value: Capital::from_euros(1_500_000),
        },
        creditor: PropertyParty {
            name: "Bayerische Landesbank".to_string(),
            address: Some("Munich".to_string()),
            date_of_birth: None,
            is_natural_person: false,
        },
        debtor: PropertyParty {
            name: "Property Owner GmbH".to_string(),
            address: Some("Munich".to_string()),
            date_of_birth: None,
            is_natural_person: false,
        },
        charge_amount: Capital::from_euros(1_000_000),
        purpose: "Financing for property acquisition".to_string(),
        priority_rank: 1,
        registered_at: Utc::now(),
        registry_entry: LandRegistryEntry {
            registered: true,
            registration_date: Some(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()),
            registry_office: "Amtsgericht Munich".to_string(),
            section: LandRegistrySection::SectionIII,
            entry_number: Some("III-1".to_string()),
        },
        is_owner_land_charge: false,
    };

    println!("Land Charge Details:");
    println!("  Amount: EUR {:.2}", land_charge.charge_amount.to_euros());
    println!("  Purpose: {}", land_charge.purpose);
    println!("  Priority Rank: {}", land_charge.priority_rank);
    println!();

    match validate_land_charge(&land_charge) {
        Ok(()) => {
            println!("✅ Land charge valid per §1191 BGB!");
            println!("   Key difference from Mortgage:");
            println!("   - Non-accessory (nicht akzessorisch)");
            println!("   - Independent of underlying claim");
            println!("   - More flexible than mortgage");
            println!("   - Commonly used in German practice");
            println!();
            println!("   Advantages:");
            println!("   - Can secure future/changing claims");
            println!("   - Survives repayment of original loan");
            println!("   - Can be reused for new loans");
        }
        Err(e) => println!("❌ Land charge invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 4: Easement (Dienstbarkeit)
    // =========================================================================
    println!("Example 4: Right of Way Easement (Wegerecht)");
    println!("----------------------------------------\n");

    let easement = Easement {
        easement_type: EasementType::RightOfWay {
            path_description: "Access path across parcel 123/45 to reach public road".to_string(),
        },
        servient_land: LandParcel {
            parcel_number: "123/45".to_string(),
            land_registry_district: "Hamburg-Mitte".to_string(),
            size_square_meters: 1_000,
            location: "Elbchaussee 100".to_string(),
            description: "Land parcel serving easement".to_string(),
            value: Capital::from_euros(800_000),
        },
        dominant_land: Some(LandParcel {
            parcel_number: "123/46".to_string(),
            land_registry_district: "Hamburg-Mitte".to_string(),
            size_square_meters: 500,
            location: "Elbchaussee 102".to_string(),
            description: "Land parcel benefiting from easement".to_string(),
            value: Capital::from_euros(600_000),
        }),
        beneficiary: None,
        established_at: Utc::now(),
        establishment_method: EasementEstablishment::Agreement,
        registered: true,
    };

    println!("Easement Details:");
    println!("  Type: Right of Way (Wegerecht)");
    println!(
        "  Servient land: Parcel {}",
        easement.servient_land.parcel_number
    );
    println!(
        "  Dominant land: Parcel {}",
        easement.dominant_land.as_ref().unwrap().parcel_number
    );
    println!("  Established by: Agreement (Vertrag)");
    println!();

    match validate_easement(&easement) {
        Ok(()) => {
            println!("✅ Easement valid per §1018 BGB!");
            println!("   Predial easement (Grunddienstbarkeit):");
            println!("   - Burdens servient land (dienendes Grundstueck)");
            println!("   - Benefits dominant land (herrschendes Grundstueck)");
            println!("   - Runs with the land (property right)");
            println!("   - Registered in Section II of land registry");
            println!();
            println!("   Effect:");
            println!("   - Owner of dominant land may use path");
            println!("   - Servient land owner must tolerate use");
            println!("   - Easement binds future owners");
        }
        Err(e) => println!("❌ Easement invalid: {}", e),
    }
    println!("\n");

    // =========================================================================
    // Example 5: Invalid Transfer - No Registration
    // =========================================================================
    println!("Example 5: Invalid Transfer - Missing Registration");
    println!("----------------------------------------\n");

    let invalid_transfer = ImmovableTransfer {
        transferor: PropertyParty {
            name: "Seller".to_string(),
            address: Some("Berlin".to_string()),
            date_of_birth: None,
            is_natural_person: true,
        },
        transferee: PropertyParty {
            name: "Buyer".to_string(),
            address: Some("Munich".to_string()),
            date_of_birth: None,
            is_natural_person: true,
        },
        land_parcel: LandParcel {
            parcel_number: "999/99".to_string(),
            land_registry_district: "Berlin".to_string(),
            size_square_meters: 300,
            location: "Test Street 1".to_string(),
            description: "Test property".to_string(),
            value: Capital::from_euros(400_000),
        },
        agreement: TransferAgreement {
            agreement_reached: true,
            agreed_at: Utc::now(),
            transfer_intent: true,
            acceptance_intent: true,
        },
        registration: LandRegistryEntry {
            registered: false, // Not registered!
            registration_date: None,
            registry_office: "Amtsgericht Berlin".to_string(),
            section: LandRegistrySection::SectionI,
            entry_number: None,
        },
        consideration: Some(Capital::from_euros(400_000)),
        transferred_at: Utc::now(),
    };

    println!("Scenario: Agreement reached but not yet registered");
    println!();

    match validate_immovable_transfer(&invalid_transfer) {
        Ok(()) => println!("✅ Transfer valid (unexpected)"),
        Err(e) => {
            println!("❌ Transfer fails: {}", e);
            println!();
            println!("Explanation:");
            println!("  §873 Abs. 1 requires:");
            println!("  1. Agreement (Einigung) ✓");
            println!("  2. Land registry entry (Eintragung) ✗");
            println!();
            println!("  Without registration, ownership does NOT transfer!");
            println!("  This is the 'registration principle' (Eintragungsprinzip)");
            println!("  Agreement alone creates only obligation, not property right");
        }
    }
    println!("\n");

    // =========================================================================
    // Summary
    // =========================================================================
    println!("=== Summary: German Immovable Property Law ===");
    println!();
    println!("§873 - Land Transfer:");
    println!("  Requirements: Agreement + Land Registry Entry");
    println!("  Registration principle (Eintragungsprinzip)");
    println!("  Ownership transfers only upon registration");
    println!();
    println!("§1113 - Mortgage (Hypothek):");
    println!("  Accessory to claim (akzessorisch)");
    println!("  Secures specific loan");
    println!("  Registered in Section III");
    println!();
    println!("§1191 - Land Charge (Grundschuld):");
    println!("  Non-accessory (nicht akzessorisch)");
    println!("  Independent of underlying claim");
    println!("  More flexible, commonly used");
    println!();
    println!("§1018 - Predial Easement (Grunddienstbarkeit):");
    println!("  Burdens servient land");
    println!("  Benefits dominant land");
    println!("  Runs with the land");
    println!();
    println!("All examples demonstrate correct BGB immovable property law!");
}
