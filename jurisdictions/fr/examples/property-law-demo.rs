//! Property Law Example
//!
//! Demonstrates French property law (Code civil - Droit des biens)
//! including ownership rights, easements, and property encumbrances.

use legalis_fr::property::*;

fn main() {
    println!("=== French Property Law Example ===\n");
    println!("Code civil - Droit des biens (Book II)\n");

    // Example 1: Immovable Property
    println!("📋 Example 1: Immovable Property (Bien immeuble)");
    println!("────────────────────────────────────────────────\n");

    let property = Property::new(
        PropertyType::Immovable {
            land_area: 1_000.0,         // 1,000 m²
            building_area: Some(200.0), // 200 m² building
        },
        "Jean Dupont".to_string(),
        "15 rue de la République, 75011 Paris".to_string(),
        500_000, // €500,000
    );

    match validate_property(&property) {
        Ok(()) => {
            println!("✅ Property: VALID");
            println!("   Owner: {}", property.owner);
            println!("   Location: {}", property.location);
            println!("   Value: €{}", property.value);
            if let PropertyType::Immovable {
                land_area,
                building_area,
            } = property.property_type
            {
                println!("\n   Property Details:");
                println!("      Land area: {} m²", land_area);
                if let Some(building) = building_area {
                    println!("      Building area: {} m²", building);
                }
            }
            println!("\n   📖 Legal Basis:");
            println!("      Article 544: Absolute ownership right");
            println!("      Owner can use, enjoy, and dispose of property");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 2: Movable Property
    println!("📋 Example 2: Movable Property (Bien meuble)");
    println!("─────────────────────────────────────────────\n");

    let movable = Property::new(
        PropertyType::Movable {
            description: "2020 Renault Clio".to_string(),
        },
        "Marie Martin".to_string(),
        "Vehicle registration Paris".to_string(),
        15_000, // €15,000
    );

    match validate_property(&movable) {
        Ok(()) => {
            println!("✅ Movable Property: VALID");
            println!("   Owner: {}", movable.owner);
            println!("   Value: €{}", movable.value);
            if let PropertyType::Movable { description } = &movable.property_type {
                println!("   Description: {}", description);
            }
            println!("\n   📖 Legal Classification:");
            println!("      Article 527: All property not immovable is movable");
            println!("      Includes vehicles, furniture, securities");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 3: Right of Way Easement
    println!("📋 Example 3: Easement - Right of Way (Servitude de passage)");
    println!("────────────────────────────────────────────────────────────\n");

    let easement = Easement::new(
        EasementType::RightOfWay,
        "Property at 17 rue de Paris (servient)".to_string(),
    )
    .with_dominant_estate("Property at 15 rue de Paris (dominant)".to_string())
    .with_description("3-meter wide passage for vehicle access to public road".to_string());

    match validate_easement(&easement) {
        Ok(()) => {
            println!("✅ Easement: VALID");
            println!("   Type: {:?}", easement.easement_type);
            println!("   Servient estate: {}", easement.servient_estate);
            if let Some(dominant) = &easement.dominant_estate {
                println!("   Dominant estate: {}", dominant);
            }
            println!("   Description: {}", easement.description);
            println!("\n   📖 Legal Basis:");
            println!("      Article 637: Easement is charge on land");
            println!("      Article 682: Right of way for landlocked property");
            println!("      Servient estate is burdened, dominant estate benefits");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 4: Light Easement
    println!("📋 Example 4: Easement - Light and View (Servitude de vue)");
    println!("──────────────────────────────────────────────────────────\n");

    let light_easement = Easement::new(
        EasementType::Light,
        "15 Avenue des Champs-Élysées, Paris".to_string(),
    )
    .with_description("Windows must maintain minimum distance from property line".to_string());

    match validate_easement(&light_easement) {
        Ok(()) => {
            println!("✅ Light Easement: VALID");
            println!("   Type: {:?}", light_easement.easement_type);
            println!("   Servient estate: {}", light_easement.servient_estate);
            println!("   Description: {}", light_easement.description);
            println!("\n   📖 Legal Requirements (Article 675-680):");
            println!("      - Straight views: 1.90m from property line");
            println!("      - Angled views: 0.60m from property line");
            println!("      - Purpose: Privacy protection for neighbors");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 5: Water Rights Easement
    println!("📋 Example 5: Easement - Water Rights (Servitude d'eau)");
    println!("─────────────────────────────────────────────────────\n");

    let water_easement = Easement::new(
        EasementType::WaterRights,
        "Lower adjacent property".to_string(),
    )
    .with_dominant_estate("Rural property in Provence (upper land)".to_string())
    .with_description("Natural drainage from upper to lower land".to_string());

    match validate_easement(&water_easement) {
        Ok(()) => {
            println!("✅ Water Easement: VALID");
            println!("   Type: {:?}", water_easement.easement_type);
            println!("   Servient: {}", water_easement.servient_estate);
            if let Some(dominant) = &water_easement.dominant_estate {
                println!("   Dominant: {}", dominant);
            }
            println!("\n   📖 Legal Basis:");
            println!("      Article 640: Natural water flow easement");
            println!("      Lower land must accept water from higher land");
            println!("      Natural servitude - no compensation required");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 6: Property with Mortgage Encumbrance
    println!("📋 Example 6: Property with Mortgage Encumbrance");
    println!("────────────────────────────────────────────────\n");

    let mortgage_enc = Encumbrance::new(EncumbranceType::Mortgage, "Crédit Agricole".to_string())
        .with_amount(300_000) // €300,000 mortgage
        .with_description("First-ranking mortgage on property".to_string());

    let encumbered_property = Property::new(
        PropertyType::Immovable {
            land_area: 800.0,
            building_area: Some(150.0),
        },
        "Pierre Leroy".to_string(),
        "28 Boulevard Voltaire, 75011 Paris".to_string(),
        400_000,
    )
    .with_encumbrance(mortgage_enc.clone());

    match validate_property(&encumbered_property) {
        Ok(()) => {
            println!("✅ Property with Encumbrance: VALID");
            println!("   Owner: {}", encumbered_property.owner);
            println!("   Value: €{}", encumbered_property.value);
            println!("\n   Encumbrance:");
            println!("      Type: {:?}", mortgage_enc.encumbrance_type);
            println!("      Beneficiary: {}", mortgage_enc.beneficiary);
            if let Some(amount) = mortgage_enc.amount {
                println!("      Amount: €{}", amount);
                println!("      Equity: €{}", encumbered_property.value - amount);
            }
            println!("      Description: {}", mortgage_enc.description);
            println!("\n   📖 Legal Effect:");
            println!("      Mortgage gives creditor security interest");
            println!("      Property can be seized if loan defaults");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 7: Property with Usufruct
    println!("📋 Example 7: Property with Usufruct (Usufruit)");
    println!("───────────────────────────────────────────────\n");

    let usufruct_enc =
        Encumbrance::new(EncumbranceType::UsufructRights, "Widow Bernard".to_string())
            .with_description("Lifetime usufruct for surviving spouse".to_string());

    let usufruct_property = Property::new(
        PropertyType::Immovable {
            land_area: 1_200.0,
            building_area: Some(180.0),
        },
        "Sophie Bernard".to_string(), // Bare owner (nu-propriétaire)
        "10 rue de la Paix, 06000 Nice".to_string(),
        600_000,
    )
    .with_encumbrance(usufruct_enc.clone());

    match validate_property(&usufruct_property) {
        Ok(()) => {
            println!("✅ Property with Usufruct: VALID");
            println!(
                "   Bare owner: {} (nu-propriétaire)",
                usufruct_property.owner
            );
            println!("   Property value: €{}", usufruct_property.value);
            println!("\n   Usufruct Details:");
            println!("      Type: {:?}", usufruct_enc.encumbrance_type);
            println!("      Usufructuary: {}", usufruct_enc.beneficiary);
            println!("      Description: {}", usufruct_enc.description);
            println!("\n   📖 Legal Framework (Articles 578-624):");
            println!("      - Usufructuary: Right to use and enjoy (usus, fructus)");
            println!("      - Bare owner: Right to dispose (abusus)");
            println!("      - Common in inheritance planning");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 8: Asset Classification
    println!("📋 Example 8: Asset Classification (Article 490)");
    println!("────────────────────────────────────────────────\n");

    let assets = vec![
        Asset::new(
            "Agricultural land".to_string(),
            AssetType::ImmovableByNature,
        )
        .with_value(200_000),
        Asset::new(
            "Industrial machinery attached to building".to_string(),
            AssetType::ImmovableByDestination,
        )
        .with_value(50_000),
        Asset::new("Vehicle".to_string(), AssetType::MovableByNature).with_value(25_000),
        Asset::new(
            "Standing timber to be harvested".to_string(),
            AssetType::MovableByAnticipation,
        )
        .with_value(10_000),
    ];

    println!("Asset Classifications:");
    for asset in &assets {
        println!("   - {} ({:?})", asset.description, asset.asset_type);
        if let Some(value) = asset.value {
            println!("     Value: €{}", value);
        }
    }

    println!("\n   📖 Classification Rules:");
    println!("      - Immovable by nature: Land, buildings");
    println!("      - Immovable by destination: Movables attached to immovables");
    println!("      - Movable by nature: All other property");
    println!("      - Movable by anticipation: Immovables to be detached");

    println!("\n");

    // Summary
    println!("════════════════════════════════════════════════════════");
    println!("📊 Summary: French Property Law");
    println!("════════════════════════════════════════════════════════\n");

    println!("🏡 Property Types (Articles 516-543):");
    println!("   - Immovable (Immeubles): Land, buildings, attached fixtures");
    println!("   - Movable (Meubles): All other property\n");

    println!("👤 Ownership (Articles 544-577):");
    println!("   Article 544: Absolute ownership right");
    println!("   - Use (usus): Right to use");
    println!("   - Enjoyment (fructus): Right to fruits/income");
    println!("   - Disposal (abusus): Right to transfer/destroy\n");

    println!("🛣️  Easements (Servitudes - Articles 637-710):");
    println!("   Types:");
    println!("   - Right of Way: Access rights (Article 682)");
    println!("   - Water Rights: Natural drainage (Article 640)");
    println!("   - Light: Distance requirements (Articles 675-680)");
    println!("   - Support: Structural support obligations\n");

    println!("⚖️  Encumbrances:");
    println!("   - Mortgage: Security interest for creditor");
    println!("   - Usufruct: Split ownership (use vs. disposal)");
    println!("   - Lien: Creditor's claim");
    println!("   - Use and Habitation: Limited use rights\n");

    println!("🔑 Key Principles:");
    println!("   - Absolute ownership (Article 544)");
    println!("   - Numerus clausus: Limited property rights");
    println!("   - Registration for real estate (land registry)");
    println!("   - Strong neighbor protection (servitudes)");
}
