//! Data Protection Officer (DPO) designation and ICO registration examples.
//!
//! Demonstrates the UK GDPR rules on DPO designation (Article 37), the tasks of
//! the DPO (Article 39), the position/independence requirements (Article 38) and
//! the duty to publish and notify the DPO's contact details to the ICO
//! (Article 37(7)), using the `legalis_uk::data_protection::dpo` library APIs.

use legalis_uk::data_protection::dpo::{
    DpoAssessment, DpoContactDetails, DpoPosition, DpoTask, ICO_DPO_NOTIFICATION_URL,
    MonitoringScale, OrganisationType,
};

fn main() {
    println!("=== UK GDPR Data Protection Officer (Articles 37-39) ===\n");

    designation_examples();
    list_dpo_tasks();
    position_example();
    notification_example();
}

/// Article 37(1) — when designation of a DPO is mandatory.
fn designation_examples() {
    println!("Article 37(1) — is designation of a DPO mandatory?");
    println!("--------------------------------------------------\n");

    // (a) A local authority (public authority).
    assess(
        "Local council (public authority)",
        DpoAssessment {
            organisation_type: OrganisationType::PublicAuthority,
            court_acting_judicially: false,
            regular_systematic_monitoring_is_core: false,
            monitoring_scale: MonitoringScale::NotApplicable,
            special_category_processing_is_core: false,
            special_category_scale: MonitoringScale::NotApplicable,
            criminal_data_processing_is_core: false,
            criminal_data_scale: MonitoringScale::NotApplicable,
            competent_authority_law_enforcement: false,
        },
    );

    // (b) An ad-tech firm whose core activity is large-scale behavioural tracking.
    assess(
        "Ad-tech firm: large-scale behavioural tracking (core activity)",
        DpoAssessment {
            organisation_type: OrganisationType::PrivateSector,
            court_acting_judicially: false,
            regular_systematic_monitoring_is_core: true,
            monitoring_scale: MonitoringScale::LargeScale,
            special_category_processing_is_core: false,
            special_category_scale: MonitoringScale::NotApplicable,
            criminal_data_processing_is_core: false,
            criminal_data_scale: MonitoringScale::NotApplicable,
            competent_authority_law_enforcement: false,
        },
    );

    // (c) A private hospital group: large-scale special-category (health) data.
    assess(
        "Private hospital group: large-scale health data (core activity)",
        DpoAssessment {
            organisation_type: OrganisationType::PrivateSector,
            court_acting_judicially: false,
            regular_systematic_monitoring_is_core: false,
            monitoring_scale: MonitoringScale::NotApplicable,
            special_category_processing_is_core: true,
            special_category_scale: MonitoringScale::LargeScale,
            criminal_data_processing_is_core: false,
            criminal_data_scale: MonitoringScale::NotApplicable,
            competent_authority_law_enforcement: false,
        },
    );

    // A small accountancy practice: no mandatory ground.
    assess(
        "Small accountancy practice (no mandatory ground)",
        DpoAssessment {
            organisation_type: OrganisationType::PrivateSector,
            court_acting_judicially: false,
            regular_systematic_monitoring_is_core: false,
            monitoring_scale: MonitoringScale::NotApplicable,
            special_category_processing_is_core: false,
            special_category_scale: MonitoringScale::NotApplicable,
            criminal_data_processing_is_core: false,
            criminal_data_scale: MonitoringScale::NotApplicable,
            competent_authority_law_enforcement: false,
        },
    );

    // A charity unsure whether its special-category processing is "large scale".
    assess(
        "Charity: special-category processing of unclear scale",
        DpoAssessment {
            organisation_type: OrganisationType::NotForProfit,
            court_acting_judicially: false,
            regular_systematic_monitoring_is_core: false,
            monitoring_scale: MonitoringScale::NotApplicable,
            special_category_processing_is_core: true,
            special_category_scale: MonitoringScale::Unclear,
            criminal_data_processing_is_core: false,
            criminal_data_scale: MonitoringScale::NotApplicable,
            competent_authority_law_enforcement: false,
        },
    );
}

/// Run one designation assessment and print the outcome.
fn assess(label: &str, assessment: DpoAssessment) {
    let outcome = assessment.assess();
    println!("{label}:");
    if outcome.is_mandatory() {
        println!("  => Designation MANDATORY. Grounds:");
        for ground in &outcome.mandatory_grounds {
            println!(
                "     - {} ({})",
                ground.explanation(),
                ground.statutory_provision()
            );
        }
    } else if outcome.is_recommended() {
        println!("  => Not strictly mandatory, but RECOMMENDED (borderline scale):");
        for ground in &outcome.borderline_grounds {
            println!(
                "     - {} ({})",
                ground.explanation(),
                ground.statutory_provision()
            );
        }
    } else {
        println!("  => Designation NOT mandatory (a voluntary DPO may still be appointed).");
    }
    println!();
}

/// Article 39(1) — the statutory tasks of the DPO.
fn list_dpo_tasks() {
    println!("Article 39(1) — tasks of the DPO");
    println!("--------------------------------\n");
    for task in DpoTask::all() {
        println!("  [{}] {}", task.statutory_provision(), task.description());
    }
    println!();
}

/// Article 38 — the position and independence of the DPO.
fn position_example() {
    println!("Article 38 — position and independence of the DPO");
    println!("-------------------------------------------------\n");

    let compliant = DpoPosition {
        involved_in_all_issues: true,
        provided_with_resources: true,
        operationally_independent: true,
        protected_from_dismissal: true,
        reports_to_highest_management: true,
        free_of_conflict_of_interest: true,
    };
    println!(
        "  Properly resourced, independent DPO: compliant = {}",
        compliant.is_compliant()
    );

    // A common defect: the Head of Marketing is also named as DPO.
    let conflicted = DpoPosition {
        involved_in_all_issues: true,
        provided_with_resources: true,
        operationally_independent: false,
        protected_from_dismissal: true,
        reports_to_highest_management: false,
        free_of_conflict_of_interest: false,
    };
    println!(
        "  Head of Marketing appointed as DPO: compliant = {}",
        conflicted.is_compliant()
    );
    for failure in conflicted.compliance_failures() {
        println!("     - {} ({})", failure.reason, failure.provision);
    }
    println!();
}

/// Article 37(7) — publishing and notifying the DPO's contact details to the ICO.
fn notification_example() {
    println!("Article 37(7) — publication and ICO notification of contact details");
    println!("-------------------------------------------------------------------\n");

    let compliant = DpoContactDetails {
        name_or_title: "Data Protection Officer".to_string(),
        postal_address: "DPO, 1 Privacy Way, London, EC1A 1AA".to_string(),
        email: "dpo@example.org".to_string(),
        telephone: Some("+44 20 7000 0000".to_string()),
        published: true,
        notified_to_ico: true,
    };
    println!(
        "  Published and notified to the ICO: compliant = {}",
        compliant.is_compliant()
    );

    let incomplete = DpoContactDetails {
        name_or_title: "Data Protection Officer".to_string(),
        postal_address: "DPO, 1 Privacy Way, London, EC1A 1AA".to_string(),
        email: "dpo@example.org".to_string(),
        telephone: None,
        published: true,
        notified_to_ico: false,
    };
    println!(
        "  Published but not notified to the ICO: compliant = {}",
        incomplete.is_compliant()
    );
    for failure in incomplete.validate_notification() {
        println!(
            "     - {} ({})",
            failure.message(),
            failure.statutory_provision()
        );
    }

    println!("\n  Notify the ICO via: {ICO_DPO_NOTIFICATION_URL}");
}
