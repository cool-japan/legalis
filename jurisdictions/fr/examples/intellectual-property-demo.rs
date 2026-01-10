//! Intellectual Property Law Example
//!
//! Demonstrates French IP law (Code de la propriété intellectuelle)
//! including patents, copyright, trademarks, and designs.

use chrono::NaiveDate;
use legalis_fr::intellectual_property::*;

fn main() {
    println!("=== French Intellectual Property Law Example ===\n");
    println!("Code de la propriété intellectuelle (CPI)\n");

    let current_date = NaiveDate::from_ymd_opt(2024, 11, 10).unwrap();

    // Example 1: Patent (Brevet d'invention)
    println!("📋 Example 1: Patent Application");
    println!("─────────────────────────────────\n");

    let patent = Patent::builder()
        .title("Novel Water Filtration System".to_string())
        .inventor("Dr. Marie Curie".to_string())
        .filing_date(NaiveDate::from_ymd_opt(2022, 1, 15).unwrap())
        .novelty(true)
        .inventive_step(true)
        .industrial_applicability(true)
        .build()
        .expect("Valid patent");

    match validate_patent(&patent, current_date) {
        Ok(()) => {
            println!("✅ Patent: VALID");
            println!("   Title: {}", patent.title);
            println!("   Inventor: {}", patent.inventor);
            println!("   Filing date: {}", patent.filing_date);
            println!("\n   ✅ Patentability Requirements (Article L611-10):");
            println!("      1. Novelty (nouveauté): {}", patent.novelty);
            println!(
                "      2. Inventive step (activité inventive): {}",
                patent.inventive_step
            );
            println!(
                "      3. Industrial applicability: {}",
                patent.industrial_applicability
            );
            println!("\n   Duration: 20 years from filing (Article L611-11)");
            println!("   Expiry date: {}", patent.expiry_date());
            println!(
                "   Status: {}",
                if patent.is_expired(current_date) {
                    "Expired"
                } else {
                    "Active"
                }
            );
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 2: Copyright (Droit d'auteur)
    println!("📋 Example 2: Copyright Protection");
    println!("──────────────────────────────────\n");

    let copyright = Copyright::builder()
        .work_title("Les Misérables Modernes".to_string())
        .author("Victor Nouveau".to_string())
        .creation_date(NaiveDate::from_ymd_opt(2020, 5, 15).unwrap())
        .work_type(WorkType::Literary)
        .build()
        .expect("Valid copyright");

    match validate_copyright(&copyright, current_date) {
        Ok(()) => {
            println!("✅ Copyright: VALID");
            println!("   Title: {}", copyright.work_title);
            println!("   Author: {}", copyright.author);
            println!("   Work type: {:?}", copyright.work_type);
            println!("   Creation date: {}", copyright.creation_date);
            println!("\n   📖 Protection (Articles L122-1 to L123-1):");
            println!("      - Moral rights: Perpetual and inalienable");
            println!("        • Right of disclosure (divulgation)");
            println!("        • Right of attribution (paternité)");
            println!("        • Right of integrity (respect de l'œuvre)");
            println!("      - Economic rights: Life + 70 years");
            println!("        • Reproduction right");
            println!("        • Public performance right");
            println!(
                "\n   Status: {}",
                if copyright.is_expired(current_date) {
                    "Expired"
                } else {
                    "Protected"
                }
            );
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 3: Trademark (Marque)
    println!("📋 Example 3: Trademark Registration");
    println!("────────────────────────────────────\n");

    let trademark = Trademark::builder()
        .mark("LAFONTAINE™".to_string())
        .owner("Lafontaine SA".to_string())
        .registration_date(NaiveDate::from_ymd_opt(2020, 6, 1).unwrap())
        .classes(vec![9, 35, 42]) // Nice Classification
        .distinctiveness(true)
        .build()
        .expect("Valid trademark");

    match validate_trademark(&trademark, current_date) {
        Ok(()) => {
            println!("✅ Trademark: VALID");
            println!("   Mark: {}", trademark.mark);
            println!("   Owner: {}", trademark.owner);
            println!("   Registration: {}", trademark.registration_date);
            println!("\n   Nice Classification Classes:");
            for class in &trademark.classes {
                let description = match class {
                    9 => "Scientific and technological apparatus",
                    35 => "Advertising; business management",
                    42 => "Scientific and technological services",
                    _ => "Other",
                };
                println!("      Class {}: {}", class, description);
            }
            println!("\n   ✅ Requirements (Article L711-1):");
            println!("      - Distinctive sign: {}", trademark.distinctiveness);
            println!("      - Valid classes: {}", trademark.has_valid_classes());
            println!("\n   Duration: 10 years renewable (Article L712-1)");
            println!("   Expiry date: {}", trademark.expiry_date());
            println!("   Renewals: Indefinite");
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 4: Industrial Design (Dessin et modèle)
    println!("📋 Example 4: Industrial Design Protection");
    println!("──────────────────────────────────────────\n");

    let design = Design::builder()
        .title("Ergonomic Office Chair".to_string())
        .creator("Studio Design Paris".to_string())
        .filing_date(NaiveDate::from_ymd_opt(2024, 2, 10).unwrap())
        .novelty(true)
        .individual_character(true)
        .protection_years(25)
        .build()
        .expect("Valid design");

    match validate_design(&design, current_date) {
        Ok(()) => {
            println!("✅ Design: VALID");
            println!("   Title: {}", design.title);
            println!("   Creator: {}", design.creator);
            println!("   Filing date: {}", design.filing_date);
            println!("\n   ✅ Protection Requirements (Article L511-1):");
            println!("      1. Novelty (nouveauté): {}", design.novelty);
            println!(
                "      2. Individual character (caractère propre): {}",
                design.individual_character
            );
            println!("\n   Duration: 5 years renewable up to 25 years (Article L513-1)");
            println!("   Current protection: {} years", design.protection_years);
            println!("   Expiry date: {}", design.expiry_date());
        }
        Err(e) => println!("❌ Invalid: {}", e),
    }

    println!("\n");

    // Example 5: Expired Patent
    println!("📋 Example 5: Expired Patent");
    println!("────────────────────────────\n");

    let old_patent = Patent::builder()
        .title("Ancient Invention".to_string())
        .inventor("Historical Inventor".to_string())
        .filing_date(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
        .novelty(true)
        .inventive_step(true)
        .industrial_applicability(true)
        .build()
        .expect("Valid patent");

    println!("Patent: {}", old_patent.title);
    println!("Filing date: {}", old_patent.filing_date);
    println!("Expiry date: {}", old_patent.expiry_date());
    println!(
        "Status: {}",
        if old_patent.is_expired(current_date) {
            "❌ EXPIRED (20 years elapsed)"
        } else {
            "✅ Active"
        }
    );
    println!("\nConsequence: Invention is now in public domain");
    println!("Anyone can use this invention without permission");

    println!("\n");

    // Example 6: Invalid Patent - Lacks Novelty
    println!("📋 Example 6: Invalid Patent - Prior Art");
    println!("────────────────────────────────────────\n");

    let invalid_patent = Patent::builder()
        .title("Wheel for Vehicles".to_string())
        .inventor("Late Inventor".to_string())
        .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        .novelty(false) // Prior art exists!
        .inventive_step(false)
        .industrial_applicability(true)
        .build()
        .expect("Constructed patent");

    match validate_patent(&invalid_patent, current_date) {
        Ok(()) => println!("✅ Unexpectedly valid"),
        Err(e) => {
            println!("❌ Patent Rejected:");
            println!("   {}", e);
            println!("\n   Reason: Lacks novelty (Article L611-10)");
            println!("   Prior art: Invention is already known");
            println!("   Consequence: INPI (Institut National) rejects application");
        }
    }

    println!("\n");

    // Summary
    println!("════════════════════════════════════════════════════════");
    println!("📊 Summary: French Intellectual Property Law");
    println!("════════════════════════════════════════════════════════\n");

    println!("🔬 Patents (Brevets - Book VI):");
    println!("   Requirements: Novelty + Inventive step + Industrial applicability");
    println!("   Duration: 20 years from filing (Article L611-11)");
    println!("   Registration: INPI (Institut National de la Propriété Industrielle)\n");

    println!("📚 Copyright (Droit d'auteur - Books I-III):");
    println!("   Automatic protection upon creation");
    println!("   Moral rights: Perpetual and inalienable");
    println!("   Economic rights: Author's life + 70 years (Article L123-1)");
    println!("   No registration required\n");

    println!("™️  Trademarks (Marques - Book VII):");
    println!("   Requirements: Distinctive sign (Article L711-1)");
    println!("   Duration: 10 years renewable indefinitely (Article L712-1)");
    println!("   Registration: INPI required\n");

    println!("🎨 Designs (Dessins et modèles - Book V):");
    println!("   Requirements: Novelty + Individual character (Article L511-1)");
    println!("   Duration: 5 years renewable up to 25 years (Article L513-1)");
    println!("   Registration: INPI required\n");

    println!("🔑 Key Principles:");
    println!("   - Strong moral rights for copyright (unique to civil law)");
    println!("   - Industrial property requires registration");
    println!("   - INPI centralized management");
    println!("   - Harmonized with EU directives");
}
