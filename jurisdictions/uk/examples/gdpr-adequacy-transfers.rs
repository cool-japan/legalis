//! UK GDPR International Data Transfer Examples
//!
//! Demonstrates UK adequacy decisions and international data transfer mechanisms
//! under UK GDPR Articles 45-49.

use legalis_uk::data_protection::{
    adequacy::{Article49Derogation, TransferMechanism, UkAdequacyDecision},
    is_adequate_country_uk,
};

fn main() {
    println!("=== UK GDPR International Data Transfers ===\n");
    println!("UK adequacy decisions and transfer mechanisms under Articles 45-49\n");
    println!("================================================\n");

    // Example 1: Test adequacy for various countries
    example_1_adequacy_checks();

    // Example 2: UK-specific adequacy decisions
    example_2_uk_specific_decisions();

    // Example 3: Transfer mechanisms when adequacy is absent
    example_3_transfer_mechanisms();

    // Example 4: Article 49 derogations
    example_4_article49_derogations();
}

fn example_1_adequacy_checks() {
    println!("Example 1: UK Adequacy Decisions by Country");
    println!("============================================\n");

    let test_countries = vec![
        ("France", "EU member state"),
        ("Germany", "EU member state"),
        ("Norway", "EEA member (not EU)"),
        ("Switzerland", "Retained EU adequacy"),
        ("Japan", "Retained EU adequacy"),
        ("South Korea", "UK-specific adequacy (2021)"),
        ("United States", "UK-US Data Bridge (2023)"),
        ("Canada", "Retained EU adequacy (PIPEDA only)"),
        ("Australia", "No adequacy decision"),
        ("Brazil", "No adequacy decision"),
        ("India", "No adequacy decision"),
        ("China", "No adequacy decision"),
    ];

    println!("Testing adequacy for {} countries:\n", test_countries.len());

    for (country, note) in test_countries {
        let has_adequacy = is_adequate_country_uk(country);

        if has_adequacy {
            println!("✅ {} - ADEQUATE", country);
            println!("   {}", note);
            println!("   Transfer permitted under UK GDPR Article 45");
        } else {
            println!("❌ {} - NO ADEQUACY", country);
            println!("   {}", note);
            println!("   Requires safeguards under Articles 46-49");
        }
        println!();
    }
}

fn example_2_uk_specific_decisions() {
    println!("Example 2: UK-Specific Adequacy Decisions");
    println!("==========================================\n");

    let decisions = vec![
        UkAdequacyDecision::SouthKorea,
        UkAdequacyDecision::UnitedStates,
    ];

    for decision in decisions {
        println!("Country: {:?}", decision);
        println!("Decision date: {:?}", decision.decision_date());
        println!("Legal basis: {}", decision.legal_basis());
        println!("Valid: {}", decision.is_valid());

        if let Some(conditions) = decision.conditions() {
            println!("Conditions:\n  {}", conditions);
        } else {
            println!("Conditions: None");
        }
        println!();
    }

    println!("Post-Brexit UK Adequacy Landscape:");
    println!("===================================");
    println!();
    println!("1. UK retains EU adequacy decisions made before Brexit");
    println!("   - All EEA countries (Norway, Iceland, Liechtenstein)");
    println!("   - All EU member states (27 countries)");
    println!("   - Countries with EU adequacy: Japan, Switzerland, Canada, etc.");
    println!();
    println!("2. UK makes new adequacy decisions post-Brexit");
    println!("   - South Korea (December 2021)");
    println!("   - United States (October 2023 - UK-US Data Bridge)");
    println!();
    println!("3. UK does NOT automatically adopt new EU adequacy decisions");
    println!("   - UK must make its own determinations for countries");
    println!("   - Secretary of State makes UK adequacy regulations");
    println!();
}

fn example_3_transfer_mechanisms() {
    println!("Example 3: Alternative Transfer Mechanisms (No Adequacy)");
    println!("=========================================================\n");

    println!("Scenario: Transferring data to Brazil (no UK adequacy decision)");
    println!();

    let mechanisms = [
        TransferMechanism::UkIdta,
        TransferMechanism::EuSccsWithAddendum,
        TransferMechanism::BindingCorporateRules {
            approval_reference: "ICO/BCR/2024/001".to_string(),
        },
        TransferMechanism::CodeOfConduct {
            code_reference: "ICO/COC/2024/005".to_string(),
        },
        TransferMechanism::CertificationMechanism {
            certificate_reference: "ICO/CERT/2024/010".to_string(),
        },
    ];

    println!("Available Transfer Mechanisms (Articles 46-47):\n");

    for (i, mechanism) in mechanisms.iter().enumerate() {
        println!("{}. {:?}", i + 1, mechanism);
        println!();

        match mechanism {
            TransferMechanism::UkIdta => {
                println!("   UK International Data Transfer Agreement");
                println!("   - Post-Brexit UK replacement for EU SCCs");
                println!("   - Published by ICO (2022)");
                println!("   - Standard contractual clauses approved by ICO");
                println!("   - Article 46(2)(c) UK GDPR");
            }
            TransferMechanism::EuSccsWithAddendum => {
                println!("   EU Standard Contractual Clauses with UK Addendum");
                println!("   - Allows continued use of EU SCCs for UK transfers");
                println!("   - Must use ICO's UK Addendum (International Data Transfer Addendum)");
                println!("   - Article 46(2)(c) UK GDPR");
            }
            TransferMechanism::BindingCorporateRules { .. } => {
                println!("   Binding Corporate Rules (BCRs)");
                println!("   - Internal policies for multinational groups");
                println!("   - Requires ICO approval");
                println!("   - Article 47 UK GDPR");
            }
            TransferMechanism::CodeOfConduct { .. } => {
                println!("   Approved Code of Conduct");
                println!("   - Sector-specific codes");
                println!("   - Article 40 UK GDPR");
            }
            TransferMechanism::CertificationMechanism { .. } => {
                println!("   Approved Certification Mechanism");
                println!("   - Data protection certification");
                println!("   - Article 42 UK GDPR");
            }
            _ => {}
        }
        println!();
    }

    println!("ICO Recommendation:");
    println!("  • UK IDTA or EU SCCs with UK Addendum for most transfers");
    println!("  • BCRs for multinational group companies");
    println!("  • Derogations (Article 49) only for occasional transfers");
    println!();
}

fn example_4_article49_derogations() {
    println!("Example 4: Article 49 Derogations (Limited Use)");
    println!("================================================\n");

    println!("⚠️  WARNING: Derogations should ONLY be used for occasional,");
    println!("   non-repetitive transfers. NOT for regular business transfers.\n");
    println!();

    let derogations = vec![
        (
            Article49Derogation::ExplicitConsent,
            "Data subject explicitly consented to transfer, informed of risks",
            "One-off transfer of employee data to US office for relocation",
        ),
        (
            Article49Derogation::ContractPerformance,
            "Transfer necessary for performance of contract with data subject",
            "Hotel booking requires sending passenger data to foreign hotel",
        ),
        (
            Article49Derogation::LegalClaims,
            "Transfer necessary for establishment, exercise or defense of legal claims",
            "Sending data to foreign lawyer for litigation",
        ),
        (
            Article49Derogation::PublicInterest,
            "Transfer necessary for important reasons of public interest",
            "Transferring health data for public health emergency response",
        ),
        (
            Article49Derogation::VitalInterests,
            "Transfer necessary to protect vital interests of data subject",
            "Emergency medical data transfer to save life",
        ),
        (
            Article49Derogation::PublicRegister,
            "Transfer from a public register",
            "Data from Companies House public register",
        ),
        (
            Article49Derogation::CompellingLegitimateInterests,
            "Compelling legitimate interests (very restrictive)",
            "One-off transfer, limited data subjects, no high risk",
        ),
    ];

    for (derogation, description, example) in derogations {
        println!("{:?}", derogation);
        println!("  Description: {}", description);
        println!("  Example: {}", example);
        println!();
    }

    println!("Article 49(1) Requirements:");
    println!("============================");
    println!();
    println!("1. Explicit consent derogation:");
    println!("   • Data subject must be informed of possible risks");
    println!("   • Risks include: lack of adequacy decision, no safeguards");
    println!("   • Consent must be freely given, specific, informed");
    println!();
    println!("2. Compelling legitimate interests derogation:");
    println!("   • ONLY when no other derogation applies");
    println!("   • NOT for repetitive transfers");
    println!("   • LIMITED number of data subjects");
    println!("   • Controller must assess circumstances");
    println!("   • Notify ICO (and inform data subject)");
    println!();
    println!("3. General restrictions:");
    println!("   • Cannot use for regular, systematic transfers");
    println!("   • Should document why transfer is exceptional");
    println!("   • Consider if alternative mechanism (IDTA, SCCs) more appropriate");
    println!();
}
