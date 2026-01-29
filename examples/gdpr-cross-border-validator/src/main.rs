//! GDPR Cross-Border Data Transfer Validator
//!
//! Automated validation of international data transfers under GDPR Chapter V (Articles 44-49).
//!
//! This tool demonstrates "Compliance as Code" - automatically determining whether
//! cross-border data transfers comply with GDPR without manual legal review.
//!
//! Key Features:
//! - Article 45: Adequacy decision validation
//! - Article 46: Appropriate safeguards verification
//! - Article 49: Derogations check
//! - Schrems II compliance (USA special handling)
//! - Transfer Impact Assessment (TIA) recommendations

use anyhow::Result;
use legalis_core::LegalResult;
use legalis_eu::gdpr::cross_border::{AdequateCountry, CrossBorderTransfer, TransferSafeguard};

fn main() -> Result<()> {
    println!("🌍 GDPR Cross-Border Data Transfer Validator");
    println!("   Compliance as Code - Chapter V Automated Validation\n");

    // Scenario 1: EU → Japan (Adequacy Decision)
    scenario_eu_to_japan()?;

    // Scenario 2: EU → Laos (No Protection)
    scenario_eu_to_laos()?;

    // Scenario 3: EU → USA (DPF Certified - SCCs)
    scenario_eu_to_usa_with_scc()?;

    // Scenario 4: EU → USA (No Safeguards)
    scenario_eu_to_usa_no_safeguards()?;

    // Scenario 5: EU → UK (Adequacy Decision)
    scenario_eu_to_uk()?;

    println!("\n✅ All validation scenarios completed!");
    println!("\n💡 Key Insight:");
    println!("   GDPR compliance → Automated validation (no lawyer needed)");
    println!("   ¥1M legal fees → ¥0 instant verification");
    println!("   Weeks of review → Milliseconds of computation");
    println!("\n🚀 This is Compliance as Code.");

    Ok(())
}

/// Scenario 1: EU → Japan (Adequacy Decision - Article 45)
fn scenario_eu_to_japan() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("  Scenario 1: EU → Japan (Adequacy Decision)");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("📋 Transfer Details:");
    println!("   Origin: EU (Germany)");
    println!("   Destination: Japan (Tokyo data center)");
    println!("   Data Type: Customer personal data");
    println!("   Safeguards: None (adequacy decision exists)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU-Germany")
        .with_destination_country("Japan")
        .with_adequate_destination(AdequateCountry::Japan)
        .add_data_category("Customer personal data")
        .with_purpose("Service provision");

    println!("🔍 Validating against GDPR Chapter V...");
    let validation = transfer.validate()?;

    println!("\n📊 Validation Result:");
    match &validation.transfer_permitted {
        LegalResult::Deterministic(true) => {
            println!("   Status: ✅ ALLOWED");
            println!("   Legal Basis: GDPR Article 45 (Adequacy Decision)");
            println!("   Details: Japan received EU adequacy decision in 2019");
            println!("   Additional Measures: None required");
            println!("   TIA Required: No");
        }
        _ => println!("   Status: ❌ NOT ALLOWED"),
    }

    println!("\n💰 Cost Comparison:");
    println!("   Traditional: Lawyer consultation ¥500,000 + 2 weeks");
    println!("   Legalis-RS:  Instant automated validation ¥0\n");

    println!("═══════════════════════════════════════════════════════════\n");
    Ok(())
}

/// Scenario 2: EU → Laos (No Adequacy, No Safeguards)
fn scenario_eu_to_laos() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("  Scenario 2: EU → Laos (No Protection)");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("📋 Transfer Details:");
    println!("   Origin: EU (France)");
    println!("   Destination: Laos (no adequacy decision)");
    println!("   Data Type: User profiles");
    println!("   Safeguards: None\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU-France")
        .with_destination_country("Laos")
        .add_data_category("User profiles");

    println!("🔍 Validating against GDPR Chapter V...");
    let validation_result = transfer.validate();

    println!("\n📊 Validation Result:");
    match validation_result {
        Ok(_) => println!("   Status: ✅ ALLOWED (unexpected)"),
        Err(e) => {
            println!("   Status: ❌ FORBIDDEN");
            println!("   Reason: {}", e);
            println!("   Legal Basis: GDPR Chapter V violation");
            println!("\n⚠️  WARNING:");
            println!("   - Laos has NO EU adequacy decision (Article 45)");
            println!("   - NO appropriate safeguards detected (Article 46)");
            println!("   - Potential GDPR fine: Up to 4% of global annual turnover (Article 83)");
            println!("\n✅ Recommended Actions:");
            println!("   1. Use EU data center instead");
            println!("   2. Or implement Standard Contractual Clauses (SCCs)");
            println!("   3. Or obtain explicit user consent (Article 49 derogation)");
        }
    }

    println!("\n💰 Risk Avoidance:");
    println!("   GDPR Violation Fine: Up to 4% of global turnover");
    println!("   Legalis-RS Prevention: Automated validation before deployment\n");

    println!("═══════════════════════════════════════════════════════════\n");
    Ok(())
}

/// Scenario 3: EU → USA (Standard Contractual Clauses - Schrems II)
fn scenario_eu_to_usa_with_scc() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("  Scenario 3: EU → USA (SCCs + Schrems II TIA)");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("📋 Transfer Details:");
    println!("   Origin: EU (Netherlands)");
    println!("   Destination: USA (AWS us-east-1)");
    println!("   Data Type: Customer data");
    println!("   Safeguards: Standard Contractual Clauses (2021 version)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU-Netherlands")
        .with_destination_country("US")
        .with_safeguard(TransferSafeguard::StandardContractualClauses {
            version: "2021".to_string(),
            clauses_signed: true,
        })
        .add_data_category("Customer data")
        .with_purpose("Cloud storage");

    println!("🔍 Validating against GDPR Chapter V...");
    let validation = transfer.validate()?;

    println!("\n📊 Validation Result:");
    match &validation.transfer_permitted {
        LegalResult::JudicialDiscretion {
            issue,
            narrative_hint,
            ..
        } => {
            println!("   Status: 🔶 CONDITIONAL (Human Judgment Required)");
            println!("   Legal Basis: GDPR Article 46(2)(c) - Standard Contractual Clauses");
            println!("   Additional Requirements: {}", issue);
            if let Some(hint) = narrative_hint {
                println!("\n📝 Guidance:");
                for line in hint.lines() {
                    println!("   {}", line.trim());
                }
            }
            println!("\n⚠️  Schrems II Impact (CJEU C-311/18):");
            println!("   - USA lacks adequacy decision (invalidated July 2020)");
            println!("   - SCCs alone may not suffice for USA transfers");
            println!("   - Transfer Impact Assessment (TIA) REQUIRED");
            println!("   - Assess: US government surveillance laws (FISA 702, EO 12333)");
            println!("\n✅ Compliance Checklist:");
            println!("   □ Conduct Transfer Impact Assessment (TIA)");
            println!("   □ Implement supplementary measures (encryption, pseudonymization)");
            println!("   □ Verify AWS Data Privacy Framework certification");
            println!("   □ Document assessment results");
            println!("   □ Inform data subjects of transfer risks");
        }
        LegalResult::Deterministic(true) => {
            println!("   Status: ✅ ALLOWED");
        }
        _ => println!("   Status: ❌ FORBIDDEN"),
    }

    println!("\n💰 Value Proposition:");
    println!("   Manual legal review: ¥1,000,000 + 3 weeks");
    println!("   Legalis-RS validation: ¥0 + instant + comprehensive checklist\n");

    println!("═══════════════════════════════════════════════════════════\n");
    Ok(())
}

/// Scenario 4: EU → USA (No Safeguards - Forbidden)
fn scenario_eu_to_usa_no_safeguards() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("  Scenario 4: EU → USA (No Safeguards)");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("📋 Transfer Details:");
    println!("   Origin: EU (Spain)");
    println!("   Destination: USA (non-certified cloud provider)");
    println!("   Data Type: Employee data");
    println!("   Safeguards: None\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU-Spain")
        .with_destination_country("US")
        .add_data_category("Employee data");

    println!("🔍 Validating against GDPR Chapter V...");
    let validation_result = transfer.validate();

    println!("\n📊 Validation Result:");
    match validation_result {
        Ok(_) => println!("   Status: ✅ ALLOWED (unexpected)"),
        Err(e) => {
            println!("   Status: ❌ FORBIDDEN");
            println!("   Reason: {}", e);
            println!("\n⚠️  Critical Compliance Issue:");
            println!("   - USA has NO adequacy decision (post-Schrems II)");
            println!("   - NO Standard Contractual Clauses (SCCs)");
            println!("   - NO Binding Corporate Rules (BCRs)");
            println!("   - NO Data Privacy Framework certification");
            println!("\n💥 Potential Penalties:");
            println!("   - GDPR Article 83(5)(c): Up to €20M or 4% of global turnover");
            println!("   - Supervisory authority corrective measures");
            println!("   - Immediate transfer suspension order");
            println!("\n✅ Required Actions:");
            println!("   1. Implement Standard Contractual Clauses (2021 version)");
            println!("   2. Conduct Transfer Impact Assessment");
            println!("   3. Apply supplementary measures (encryption, etc.)");
            println!("   4. OR: Migrate to EU/UK data center");
        }
    }

    println!("\n💡 Alternative: Use EU Data Center");
    println!("   AWS eu-central-1 (Frankfurt) → No GDPR transfer restrictions\n");

    println!("═══════════════════════════════════════════════════════════\n");
    Ok(())
}

/// Scenario 5: EU → UK (Post-Brexit Adequacy)
fn scenario_eu_to_uk() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("  Scenario 5: EU → UK (Post-Brexit Adequacy)");
    println!("═══════════════════════════════════════════════════════════\n");

    println!("📋 Transfer Details:");
    println!("   Origin: EU (Ireland)");
    println!("   Destination: UK (London)");
    println!("   Data Type: Healthcare records");
    println!("   Safeguards: None (adequacy decision exists)\n");

    let transfer = CrossBorderTransfer::new()
        .with_origin("EU-Ireland")
        .with_destination_country("UK")
        .with_adequate_destination(AdequateCountry::UnitedKingdom)
        .add_data_category("Healthcare records")
        .with_purpose("Medical research");

    println!("🔍 Validating against GDPR Chapter V...");
    let validation = transfer.validate()?;

    println!("\n📊 Validation Result:");
    match &validation.transfer_permitted {
        LegalResult::Deterministic(true) => {
            println!("   Status: ✅ ALLOWED");
            println!("   Legal Basis: GDPR Article 45 (Adequacy Decision)");
            println!("   Details: UK received EU adequacy decision in 2021 (post-Brexit)");
            println!("   Validity: Subject to review (sunset clause consideration)");
            println!("   Additional Measures: None required");
        }
        _ => println!("   Status: ❌ NOT ALLOWED"),
    }

    println!("\n📝 Note:");
    println!("   UK adequacy decision is subject to periodic review.");
    println!("   Monitor: European Commission adequacy decision updates.\n");

    println!("═══════════════════════════════════════════════════════════\n");
    Ok(())
}
