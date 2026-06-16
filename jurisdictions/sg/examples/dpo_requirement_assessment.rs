//! PDPA Data Protection Officer (DPO) Assessment Example
//!
//! Demonstrates the Accountability Obligation under s. 11 of the PDPA:
//!
//! - Designation of at least one DPO is **mandatory** for every organisation
//!   (s. 11(3)) — this is NOT merely advisory
//! - The DPO's business contact information must be made available to the public
//!   (s. 11(5))
//! - On top of the mandatory designation, the *scale* of the DPO function should
//!   reflect the organisation's data-handling profile (advisory recommendation)
//! - Maximum financial penalties (s. 48J(3)): SGD 1M, or 10% of Singapore
//!   turnover where turnover exceeds SGD 10M
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example dpo_requirement_assessment
//! ```

use legalis_sg::pdpa::*;

fn main() {
    println!("== Singapore PDPA Data Protection Officer Assessment (PDPA 2012, s. 11) ==\n");

    mandatory_designation();
    println!("\n{}\n", "-".repeat(68));
    advisory_staffing_recommendation();
    println!("\n{}\n", "-".repeat(68));
    financial_penalties();
}

/// Section 11(3): every organisation must designate at least one DPO; s. 11(5)
/// requires its business contact information to be public.
fn mandatory_designation() {
    println!("Mandatory DPO designation (s. 11(3)) and public contact (s. 11(5))\n");

    // No DPO designated -> contravenes the mandatory duty.
    let no_dpo = PdpaOrganisation::new("Startup Pte Ltd", OrganisationType::Private);
    print_accountability(&no_dpo);

    // DPO designated but contact not published -> still non-compliant (s. 11(5)).
    let dpo_unpublished = DpoContact::new("Data Protection Officer", "dpo@firm.sg", "+6561110000");
    let unpublished = PdpaOrganisation::new("Mid Co Pte Ltd", OrganisationType::Private)
        .with_dpo(dpo_unpublished)
        .with_privacy_policy("https://midco.sg/privacy");
    print_accountability(&unpublished);

    // Fully compliant: DPO designated, contact published, policy published.
    let mut dpo = DpoContact::new("Data Protection Officer", "dpo@bigco.sg", "+6562220000");
    dpo.publish();
    let compliant = PdpaOrganisation::new("Big Co Pte Ltd", OrganisationType::Private)
        .with_dpo(dpo)
        .with_privacy_policy("https://bigco.sg/privacy");
    print_accountability(&compliant);
}

fn print_accountability(org: &PdpaOrganisation) {
    let report = validate_organisation_accountability(org);
    println!(
        "  {} -> compliant={} (DPO designated? {})",
        org.name,
        report.is_compliant,
        org.has_designated_dpo()
    );
    for e in &report.errors {
        println!("      error: {}", first_line(e));
    }
}

/// The designation is always mandatory; only the *scale* of resourcing varies
/// with the organisation's data-handling profile.
fn advisory_staffing_recommendation() {
    println!("Advisory DPO staffing recommendation (designation always mandatory)\n");

    let profiles = [
        ("Corner shop", 100u64, false),
        ("Growing SaaS", 12_000, false),
        ("Private clinic", 800, true),
        ("National platform", 200_000, true),
    ];
    for (name, subjects, sensitive) in profiles {
        let org = PdpaOrganisation::new(name, OrganisationType::Private)
            .with_data_profile(subjects, sensitive);
        println!(
            "  {:<20} ({:>7} subjects, sensitive={}) -> {:?}",
            name,
            subjects,
            sensitive,
            org.dpo_staffing_recommendation()
        );
    }
}

/// Section 48J(3): maximum financial penalty is SGD 1M, or 10% of annual
/// Singapore turnover for organisations whose turnover exceeds SGD 10M.
fn financial_penalties() {
    println!("Maximum financial penalties (s. 48J(3))\n");

    for turnover in [2_000_000u64, 10_000_000, 25_000_000, 100_000_000] {
        println!(
            "  annual SG turnover SGD {:>11} -> max penalty SGD {}",
            turnover,
            max_financial_penalty_sgd(turnover)
        );
    }
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or(s)
}
