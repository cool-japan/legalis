//! Family Law demonstration program.
//!
//! This example demonstrates the French family law module, showing:
//! - Marriage validation (Articles 143-180)
//! - Divorce proceedings (Articles 229-247)
//! - Property regimes (Articles 1387-1536)
//! - PACS (Civil solidarity pacts)
//!
//! # Usage
//!
//! ```bash
//! cargo run --example family-law-demo --features=serde
//! ```

use chrono::{Duration, Utc};
use legalis_fr::family::*;

fn main() {
    println!("\n🏛️  French Family Law - Comprehensive Demo");
    println!("═══════════════════════════════════════════════════════\n");

    // Demo 1: Valid Marriage
    println!("📋 Demo 1: Valid Marriage");
    println!("─────────────────────────────────────────\n");
    demo_valid_marriage();

    // Demo 2: Invalid Marriage (Age Violation)
    println!("\n📋 Demo 2: Invalid Marriage - Age Violation");
    println!("────────────────────────────────────────────────\n");
    demo_invalid_marriage_age();

    // Demo 3: Marriage with Consanguinity Violation
    println!("\n📋 Demo 3: Marriage - Consanguinity Violation");
    println!("──────────────────────────────────────────────────\n");
    demo_consanguinity_violation();

    // Demo 4: Mutual Consent Divorce
    println!("\n👔 Demo 4: Mutual Consent Divorce");
    println!("───────────────────────────────────────\n");
    demo_mutual_consent_divorce();

    // Demo 5: Divorce for Definitive Alteration
    println!("\n👔 Demo 5: Divorce for Definitive Alteration");
    println!("────────────────────────────────────────────────\n");
    demo_definitive_alteration_divorce();

    // Demo 6: Fault-Based Divorce
    println!("\n👔 Demo 6: Fault-Based Divorce");
    println!("───────────────────────────────────────\n");
    demo_fault_divorce();

    // Demo 7: Property Regimes
    println!("\n🏢 Demo 7: Property Regimes");
    println!("───────────────────────────────────\n");
    demo_property_regimes();

    // Demo 8: PACS Formation and Dissolution
    println!("\n🤝 Demo 8: PACS Formation and Dissolution");
    println!("──────────────────────────────────────────\n");
    demo_pacs();

    println!("\n═══════════════════════════════════════════════════════");
    println!("✅ Family Law Demo Complete!");
    println!("═══════════════════════════════════════════════════════\n");
}

fn demo_valid_marriage() {
    let person1 = Person::new(
        "Alice Dupont".to_string(),
        28,
        Nationality::French,
        MaritalStatus::Single,
    );
    let person2 = Person::new(
        "Bob Martin".to_string(),
        30,
        Nationality::French,
        MaritalStatus::Single,
    );

    let pub_date = Utc::now().naive_utc().date() - Duration::days(15);
    let marriage_date = Utc::now().naive_utc().date();

    let marriage = Marriage::new(person1, person2)
        .with_consent([true, true])
        .with_banns_published(true)
        .with_banns_publication_date(pub_date)
        .with_marriage_date(marriage_date);

    println!("Parties: Alice Dupont (28) & Bob Martin (30)");
    println!("Consent: Both parties consent");
    println!("Banns: Published 15 days ago");
    println!("Validation: ");

    match validate_marriage(&marriage) {
        Ok(()) => {
            println!("  ✅ Marriage is VALID");
            println!("  ✅ All requirements met (Articles 143-180)");
            println!("  ✅ Minimum age: 18+ (Article 144)");
            println!("  ✅ Consent: Yes (Article 146)");
            println!("  ✅ No bigamy (Article 147)");
            println!("  ✅ Banns published 10+ days (Article 161)");
        }
        Err(e) => {
            println!("  ❌ Validation Error: {}", e);
        }
    }
}

fn demo_invalid_marriage_age() {
    let person1 = Person::new(
        "Charlie".to_string(),
        17, // Too young!
        Nationality::French,
        MaritalStatus::Single,
    );
    let person2 = Person::new(
        "Diana".to_string(),
        25,
        Nationality::French,
        MaritalStatus::Single,
    );

    let marriage = Marriage::new(person1, person2).with_consent([true, true]);

    println!("Parties: Charlie (17) & Diana (25)");
    println!("❌ Problem: Charlie is only 17 years old");
    println!("Validation: ");

    match validate_marriage(&marriage) {
        Ok(()) => {
            println!("  ✅ Marriage is valid");
        }
        Err(e) => {
            println!("  ❌ VIOLATION DETECTED:");
            println!("     Article 144: Minimum age is 18 years");
            println!("     Error: {}", e.description_en());
            println!("     FR: {}", e.description_fr());
        }
    }
}

fn demo_consanguinity_violation() {
    let person1 = Person::new(
        "Eve".to_string(),
        25,
        Nationality::French,
        MaritalStatus::Single,
    )
    .with_relationship(Relationship::Sibling);

    let person2 = Person::new(
        "Frank".to_string(),
        27,
        Nationality::French,
        MaritalStatus::Single,
    );

    let marriage = Marriage::new(person1, person2).with_consent([true, true]);

    println!("Parties: Eve & Frank");
    println!("❌ Problem: Siblings attempting to marry");
    println!("Validation: ");

    match validate_marriage(&marriage) {
        Ok(()) => {
            println!("  ✅ Marriage is valid");
        }
        Err(e) => {
            println!("  ❌ VIOLATION DETECTED:");
            println!("     Article 180: Consanguinity prohibition");
            println!("     Absolute nullity - marriage prohibited between siblings");
            println!("     Error: {}", e.description_en());
        }
    }
}

fn demo_mutual_consent_divorce() {
    let divorce_type = DivorceType::MutualConsent {
        agreement_signed: true,
        notary_filing_date: Some(Utc::now().naive_utc().date()),
        children_heard: true,
    };

    let marriage_date = Utc::now().naive_utc().date() - Duration::days(3650);

    let divorce = Divorce::new(
        divorce_type,
        marriage_date,
        "Alice Dupont".to_string(),
        "Bob Martin".to_string(),
        PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        },
    );

    println!("Type: Divorce by Mutual Consent (Article 230)");
    println!("Marriage Date: 10 years ago");
    println!("Agreement: Signed by both parties");
    println!("Notary: Filing date set");
    println!("Property Regime: Communauté réduite aux acquêts (default)");
    println!("Validation: ");

    match validate_divorce(&divorce) {
        Ok(()) => {
            println!("  ✅ Divorce proceedings are VALID");
            println!("  ✅ Reform 2017: Simplified procedure with notary");
            println!("  ✅ No court involvement required");
            println!("  ✅ Agreement signed");
            println!("  ✅ Children heard (if applicable)");
        }
        Err(e) => {
            println!("  ❌ Validation Error: {}", e);
        }
    }
}

fn demo_definitive_alteration_divorce() {
    let divorce_type = DivorceType::DefinitiveAlteration {
        separation_duration_months: 30,
    };

    let marriage_date = Utc::now().naive_utc().date() - Duration::days(3650);
    let separation_date = Utc::now().naive_utc().date() - Duration::days(900);

    let divorce = Divorce::new(
        divorce_type,
        marriage_date,
        "Grace".to_string(),
        "Henry".to_string(),
        PropertyRegime::SeparationDeBiens {
            marriage_contract: true,
        },
    )
    .with_separation_date(separation_date);

    println!("Type: Divorce for Definitive Alteration (Article 237)");
    println!("Marriage Date: 10 years ago");
    println!("Separation: 30 months (exceeds 24-month requirement)");
    println!("Property Regime: Séparation de biens");
    println!("Validation: ");

    match validate_divorce(&divorce) {
        Ok(()) => {
            println!("  ✅ Divorce proceedings are VALID");
            println!("  ✅ Article 237: Separation ≥ 24 months");
            println!("  ✅ Can be requested by either spouse");
            println!("  ✅ No fault determination needed");
        }
        Err(e) => {
            println!("  ❌ Validation Error: {}", e);
        }
    }
}

fn demo_fault_divorce() {
    let divorce_type = DivorceType::Fault {
        fault_type: FaultType::Violence,
        evidence: vec![
            "Police report dated 2024-03-15".to_string(),
            "Medical certificate".to_string(),
            "Witness statements".to_string(),
        ],
    };

    let marriage_date = Utc::now().naive_utc().date() - Duration::days(2000);

    let divorce = Divorce::new(
        divorce_type,
        marriage_date,
        "Isabelle".to_string(),
        "Jacques".to_string(),
        PropertyRegime::CommunauteReduite {
            marriage_contract: false,
            acquets: Vec::new(),
            biens_propres: Vec::new(),
        },
    );

    println!("Type: Fault-Based Divorce (Article 242)");
    println!("Fault: Violence");
    println!("Evidence: 3 pieces submitted");
    println!("  - Police report");
    println!("  - Medical certificate");
    println!("  - Witness statements");
    println!("Validation: ");

    match validate_divorce(&divorce) {
        Ok(()) => {
            println!("  ✅ Divorce proceedings are VALID");
            println!("  ✅ Article 242: Serious breach of marital duties");
            println!("  ✅ Evidence provided");
            println!("  ⚖️  Fault divorce may affect property division");
        }
        Err(e) => {
            println!("  ❌ Validation Error: {}", e);
        }
    }
}

fn demo_property_regimes() {
    println!("French Matrimonial Property Regimes:\n");

    // Default regime
    let regime1 = PropertyRegime::CommunauteReduite {
        marriage_contract: false,
        acquets: Vec::new(),
        biens_propres: Vec::new(),
    };

    println!("1. Communauté réduite aux acquêts (Default since 1966)");
    println!("   Article 1400: Default regime");
    println!("   FR: {}", regime_name_fr(&regime1));
    println!("   EN: {}", regime_name_en(&regime1));
    println!("   Contract Required: No");
    println!("   Property acquired during marriage: Joint ownership");
    println!("   Property owned before marriage: Separate");
    println!(
        "   Validation: {}",
        if validate_property_regime(&regime1).is_ok() {
            "✅ Valid"
        } else {
            "❌ Invalid"
        }
    );

    println!();

    // Separation of property
    let regime2 = PropertyRegime::SeparationDeBiens {
        marriage_contract: true,
    };

    println!("2. Séparation de biens");
    println!("   Article 1536: Separation of property");
    println!("   FR: {}", regime_name_fr(&regime2));
    println!("   EN: {}", regime_name_en(&regime2));
    println!("   Contract Required: YES");
    println!("   Each spouse: Retains ownership of their own property");
    println!(
        "   Validation: {}",
        if validate_property_regime(&regime2).is_ok() {
            "✅ Valid"
        } else {
            "❌ Invalid"
        }
    );

    println!();

    // Universal community
    let regime3 = PropertyRegime::CommunauteUniverselle {
        marriage_contract: true,
    };

    println!("3. Communauté universelle");
    println!("   Contract Required: YES");
    println!("   All property: Joint ownership (past and future)");
    println!(
        "   Validation: {}",
        if validate_property_regime(&regime3).is_ok() {
            "✅ Valid"
        } else {
            "❌ Invalid"
        }
    );
}

fn demo_pacs() {
    // PACS registration
    let registration_date = Utc::now().naive_utc().date() - Duration::days(730);

    let pacs = PACS::new("Léa Dubois".to_string(), "Marie Lefebvre".to_string())
        .with_registration_date(registration_date)
        .with_property_regime(PACSPropertyRegime::Separation);

    println!("PACS: Pacte civil de solidarité (Article 515-1, since 1999)");
    println!("Parties: Léa Dubois & Marie Lefebvre");
    println!("Registration: 2 years ago");
    println!("Property Regime: Separation (default for PACS)");
    println!("Validation: ");

    match validate_pacs(&pacs) {
        Ok(()) => {
            println!("  ✅ PACS is VALID");
            println!("  ✅ Properly registered");
            println!("  📝 Available to same-sex and different-sex couples");
            println!("  📝 Provides legal framework without marriage");
        }
        Err(e) => {
            println!("  ❌ Validation Error: {}", e);
        }
    }

    println!("\nDissolution Scenario:");

    let notice_date = Utc::now().naive_utc().date() - Duration::days(60);
    let dissolution_date = Utc::now().naive_utc().date();

    let dissolved_pacs = pacs
        .with_dissolution_notice_date(notice_date)
        .with_dissolution_date(dissolution_date);

    println!("  Notice given: 60 days ago");
    println!("  Dissolution: Today");
    println!("  Validation: ");

    match validate_pacs_dissolution(&dissolved_pacs) {
        Ok(()) => {
            println!("    ✅ Dissolution is VALID");
            println!("    ✅ Proper notice period");
            println!("    📝 PACS can be dissolved unilaterally or mutually");
        }
        Err(e) => {
            println!("    ❌ Validation Error: {}", e);
        }
    }
}
