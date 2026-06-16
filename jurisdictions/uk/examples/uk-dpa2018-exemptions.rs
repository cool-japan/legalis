//! Data Protection Act 2018 exemption examples.
//!
//! Demonstrates the DPA 2018 exemptions from UK GDPR obligations, using the
//! `legalis_uk::data_protection::exemptions` library APIs.
//!
//! DPA 2018 (principally Schedule 2, with further provisions in s.26, s.36, s.37
//! and Schedule 3) provides exemptions from certain UK GDPR rights and obligations
//! for specific purposes — for example the prevention/detection of crime, the
//! special purposes (journalism), academic research, legal professional privilege,
//! and confidential references. Exemptions must be applied **narrowly** and only
//! to the extent that compliance would prejudice the relevant purpose; they never
//! provide blanket immunity.

use legalis_uk::data_protection::exemptions::{
    validate_academic_research_exemption, validate_journalism_exemption,
};
use legalis_uk::data_protection::{Dpa2018Exemption, exemptions::CrimeTaxPurpose};

fn main() {
    println!("=== DPA 2018 Exemptions from UK GDPR ===\n");

    describe_exemptions();
    journalism_exemption_examples();
    academic_research_exemption_examples();
}

/// Print the statutory basis, type and narrowness flag for a range of exemptions.
fn describe_exemptions() {
    println!("Selected exemptions (statutory basis and application)");
    println!("-----------------------------------------------------\n");

    let exemptions = [
        Dpa2018Exemption::NationalSecurity {
            ministerial_certificate: true,
        },
        Dpa2018Exemption::CrimeTaxation {
            purpose: CrimeTaxPurpose::CrimePrevention,
        },
        Dpa2018Exemption::LegalPrivilege,
        Dpa2018Exemption::ConfidentialReferences,
        Dpa2018Exemption::ExamScripts,
    ];

    for exemption in &exemptions {
        println!("{:?}", exemption.exemption_type());
        println!("  Statutory basis: {}", exemption.statutory_provision());
        println!(
            "  Requires narrow application: {}",
            exemption.requires_narrow_application()
        );
        println!("  ICO guidance: {}\n", exemption.ico_guidance());
    }
}

/// DPA 2018 Schedule 2 Part 5 — the journalism (special purposes) exemption.
fn journalism_exemption_examples() {
    println!("Journalism exemption (DPA 2018 Sch 2 Part 5 para 26)");
    println!("----------------------------------------------------\n");

    println!("  Public-interest investigation, all conditions met:");
    print_result(validate_journalism_exemption(true, true, true));

    println!("  Processing not in the public interest:");
    print_result(validate_journalism_exemption(false, true, true));

    println!("  Compliance with UK GDPR not incompatible with the journalism:");
    print_result(validate_journalism_exemption(true, true, false));
    println!();
}

/// DPA 2018 Schedule 2 Part 6 — the academic research exemption.
fn academic_research_exemption_examples() {
    println!("Academic research exemption (DPA 2018 Sch 2 Part 6 para 27)");
    println!("----------------------------------------------------------\n");

    println!("  Safeguards in place, no individual decisions, no substantial harm:");
    print_result(validate_academic_research_exemption(true, true, true));

    println!("  Processing used for decisions about particular individuals:");
    print_result(validate_academic_research_exemption(true, false, true));
    println!();
}

/// Print the result of an exemption validation.
fn print_result(result: Result<(), String>) {
    match result {
        Ok(()) => println!("    -> exemption available\n"),
        Err(reason) => println!("    -> exemption NOT available: {reason}\n"),
    }
}
