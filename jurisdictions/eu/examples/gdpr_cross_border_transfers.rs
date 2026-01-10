//! GDPR Cross-Border Transfer Scenarios
//!
//! This example demonstrates how to validate cross-border data transfers under GDPR Chapter V.
//!
//! ## Scenarios Covered
//!
//! 1. Transfer to adequate country (Switzerland)
//! 2. Transfer to US with SCCs (Schrems II implications)
//! 3. Transfer with BCRs
//! 4. Transfer using derogation (explicit consent)
//! 5. Invalid transfer (no legal basis)
//! 6. Old SCC version (fails validation)
//! 7. Compelling legitimate interests (Article 49(1)(g))

use chrono::Utc;
use legalis_eu::gdpr::cross_border::*;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::shared::member_states::MemberState;

fn main() -> Result<(), GdprError> {
    println!("=== GDPR Cross-Border Transfer Validation Examples ===\n");

    scenario_1_adequate_country()?;
    scenario_2_us_with_sccs()?;
    scenario_3_bcrs()?;
    scenario_4_derogation_consent()?;
    scenario_5_no_legal_basis();
    scenario_6_old_scc_version();
    scenario_7_compelling_legitimate_interests()?;

    println!("\n✅ All cross-border transfer scenarios completed");
    Ok(())
}

/// Scenario 1: Transfer to Country with Adequacy Decision (Article 45)
///
/// Switzerland has an adequacy decision, so no additional safeguards needed.
fn scenario_1_adequate_country() -> Result<(), GdprError> {
    println!("## Scenario 1: Transfer to Adequate Country (Switzerland)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Switzerland")
        .with_adequate_destination(AdequateCountry::Switzerland)
        .add_data_category("customer names")
        .add_data_category("email addresses")
        .with_purpose("Cloud storage");

    let validation = transfer.validate()?;

    println!("Transfer to: Switzerland");
    println!(
        "Legal basis: Adequacy decision (granted {})",
        AdequateCountry::Switzerland.adequacy_year()
    );
    println!("Transfer permitted: {:?}", validation.transfer_permitted);
    println!(
        "Additional measures required: {}",
        validation.additional_measures_required
    );
    println!(
        "Risk assessment required: {}",
        validation.risk_assessment_required
    );

    println!("\n✅ Transfer approved - Switzerland has adequacy decision\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 2: Transfer to US with SCCs (Schrems II Implications)
///
/// After Schrems II (C-311/18), transfers to US require Transfer Impact Assessment
/// even with SCCs, due to US government surveillance laws (FISA 702, EO 12333).
fn scenario_2_us_with_sccs() -> Result<(), GdprError> {
    println!("## Scenario 2: Transfer to US with Standard Contractual Clauses\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("US")
        .with_safeguard(TransferSafeguard::StandardContractualClauses {
            version: "2021".to_string(),
            clauses_signed: true,
        })
        .add_data_category("user profiles")
        .with_purpose("Data analytics");

    let validation = transfer.validate()?;

    println!("Transfer to: United States");
    println!("Legal basis: Standard Contractual Clauses (2021 version)");
    println!("Transfer permitted: {:?}", validation.transfer_permitted);
    println!(
        "Additional measures required: {}",
        validation.additional_measures_required
    );
    println!(
        "Risk assessment required: {}",
        validation.risk_assessment_required
    );

    println!("\n⚠️ SCHREMS II IMPLICATIONS:");
    println!("   Controller must perform Transfer Impact Assessment:");
    println!("   1. Assess US surveillance laws (FISA 702, EO 12333)");
    println!("   2. Evaluate if recipient is subject to government access");
    println!("   3. Consider supplementary measures:");
    println!("      - End-to-end encryption");
    println!("      - Pseudonymization");
    println!("      - Legal guarantees from US recipient");
    println!("   4. Document assessment and decision");

    println!("\n⚠️ Transfer requires judicial discretion (see LegalResult)\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 3: Transfer with Binding Corporate Rules (BCRs)
///
/// BCRs allow multinational corporations to transfer data within their group.
fn scenario_3_bcrs() -> Result<(), GdprError> {
    println!("## Scenario 3: Transfer with Binding Corporate Rules (BCRs)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Singapore")
        .with_safeguard(TransferSafeguard::BindingCorporateRules {
            approved_by: MemberState::Ireland,
            approval_date: Utc::now() - chrono::Duration::days(365),
        })
        .add_data_category("employee data")
        .with_purpose("HR management");

    let validation = transfer.validate()?;

    println!("Transfer to: Singapore");
    println!("Legal basis: Binding Corporate Rules");
    println!("Approved by: Irish Data Protection Commission");
    println!("Transfer permitted: {:?}", validation.transfer_permitted);
    println!(
        "Additional measures required: {}",
        validation.additional_measures_required
    );

    println!("\n✅ Transfer approved - BCRs provide adequate safeguards\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 4: Transfer Using Derogation (Explicit Consent)
///
/// Article 49 derogations are for specific situations, not regular transfers.
fn scenario_4_derogation_consent() -> Result<(), GdprError> {
    println!("## Scenario 4: Transfer Using Derogation (Explicit Consent)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Brazil")
        .with_derogation(TransferDerogation::ExplicitConsent)
        .add_data_category("booking information")
        .with_purpose("Hotel reservation");

    let validation = transfer.validate()?;

    println!("Transfer to: Brazil");
    println!("Legal basis: Explicit consent derogation (Article 49(1)(a))");
    println!("Transfer permitted: {:?}", validation.transfer_permitted);

    println!("\n⚠️ IMPORTANT:");
    println!("   Derogations are for SPECIFIC SITUATIONS, not regular transfers");
    println!("   Data subject must be informed of risks (no adequacy, no safeguards)");
    println!("   Cannot be used for repetitive/mass transfers");

    println!("\n✅ Transfer approved under derogation\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 5: Invalid Transfer (No Legal Basis)
///
/// Transfer without adequacy, safeguards, or derogations is prohibited.
fn scenario_5_no_legal_basis() {
    println!("## Scenario 5: Invalid Transfer (No Legal Basis)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Unknown Country")
        .add_data_category("personal data");
    // No safeguard, derogation, or adequacy decision

    match transfer.validate() {
        Ok(_) => println!("❌ Should have failed!"),
        Err(e) => {
            println!("Transfer to: Unknown Country");
            println!("Legal basis: NONE");
            println!("\n❌ TRANSFER REJECTED:");
            println!("   {}", e);
            println!("\n   Reason: No adequacy decision, appropriate safeguards, or derogation");
        }
    }

    println!();
    println!("{}", "=".repeat(70));
    println!();
}

/// Scenario 6: Old SCC Version (Validation Failure)
///
/// Old SCCs expired in June 2022 - must use 2021 version.
fn scenario_6_old_scc_version() {
    println!("## Scenario 6: Old SCC Version (Validation Failure)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("Singapore")
        .with_safeguard(TransferSafeguard::StandardContractualClauses {
            version: "2010".to_string(), // Old version!
            clauses_signed: true,
        });

    match transfer.validate() {
        Ok(_) => println!("❌ Should have failed!"),
        Err(e) => {
            println!("Transfer to: Singapore");
            println!("Legal basis: Standard Contractual Clauses (2010 version)");
            println!("\n❌ VALIDATION FAILED:");
            println!("   {}", e);
            println!("\n   Old SCC versions (2001/2004/2010) expired June 27, 2022");
            println!("   Must use 2021 version (Commission Implementing Decision 2021/914)");
        }
    }

    println!();
    println!("{}", "=".repeat(70));
    println!();
}

/// Scenario 7: Compelling Legitimate Interests (Article 49(1)(g))
///
/// Limited derogation for occasional, non-repetitive transfers.
fn scenario_7_compelling_legitimate_interests() -> Result<(), GdprError> {
    println!("## Scenario 7: Compelling Legitimate Interests (Article 49(1)(g))\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("India")
        .with_derogation(TransferDerogation::CompellingLegitimateInterests {
            affected_data_subjects: 5, // Small number
            is_repetitive: false,
        })
        .add_data_category("contract details")
        .with_purpose("One-time legal claim");

    let validation = transfer.validate()?;

    println!("Transfer to: India");
    println!("Legal basis: Compelling legitimate interests");
    println!("Affected data subjects: 5");
    println!("Is repetitive: false");
    println!("Transfer permitted: {:?}", validation.transfer_permitted);

    println!("\n⚠️ STRICT CONDITIONS:");
    println!("   - Only for OCCASIONAL transfers");
    println!("   - Cannot be repetitive");
    println!("   - Limited number of data subjects");
    println!("   - Recital 113: Should not be used for systematic transfers");

    // Example of invalid use (too many subjects)
    println!("\n❌ Counter-example (100 data subjects):");
    let invalid_transfer = CrossBorderTransfer::new()
        .with_origin("EU")
        .with_destination_country("India")
        .with_derogation(TransferDerogation::CompellingLegitimateInterests {
            affected_data_subjects: 100, // Too many
            is_repetitive: false,
        });

    match invalid_transfer.validate() {
        Ok(_) => println!("   Should have failed!"),
        Err(e) => println!("   Rejected: {}", e),
    }

    println!("\n✅ Original transfer (5 subjects) approved under strict conditions\n");
    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}
