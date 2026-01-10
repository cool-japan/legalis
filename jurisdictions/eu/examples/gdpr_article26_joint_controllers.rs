//! GDPR Article 26 - Joint Controllers
//!
//! This example demonstrates how to model joint controller arrangements under
//! Article 26, where two or more controllers jointly determine the purposes and
//! means of processing personal data.
//!
//! ## Scenarios
//!
//! 1. **Joint Research Project** - Two universities collaborating on research
//! 2. **Facebook Page Admins** - Based on CJEU C-210/16 Wirtschaftsakademie
//! 3. **Joint Marketing Campaign** - Two companies sharing customer data
//! 4. **Missing Arrangement Documentation** - Non-compliant (Article 26(1))
//! 5. **Missing Contact Point** - Non-compliant (Article 26(1))
//! 6. **Healthcare Data Sharing** - Joint controllers processing Article 9 health data
//!
//! Run with:
//! ```bash
//! cargo run --example gdpr_article26_joint_controllers
//! ```

use chrono::Utc;
use legalis_eu::gdpr::*;

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("GDPR Article 26 - Joint Controllers");
    println!("═══════════════════════════════════════════════════════════════\n");

    scenario1_joint_research_project();
    scenario2_facebook_page_admins();
    scenario3_joint_marketing_campaign();
    scenario4_missing_arrangement_documentation();
    scenario5_missing_contact_point();
    scenario6_healthcare_data_sharing();
}

/// Scenario 1: Joint Research Project
///
/// Two universities running a joint COVID-19 research study, with clear responsibility allocation.
fn scenario1_joint_research_project() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 1: Joint Research Project (Universities)");
    println!("──────────────────────────────────────────────────────────────\n");

    let arrangement = JointControllerArrangement::new()
        .add_controller(
            JointController::new("Ludwig Maximilian University of Munich")
                .with_contact("dpo@lmu.de")
                .add_responsibility(Responsibility::DataCollection)
                .add_responsibility(Responsibility::DataStorage)
                .add_responsibility(Responsibility::SecurityMeasures)
                .add_responsibility(Responsibility::RecordsOfProcessing)
                .as_contact_point()
                .with_notes("Lead institution - manages data infrastructure"),
        )
        .add_controller(
            JointController::new("ETH Zurich")
                .with_contact("privacy@ethz.ch")
                .add_responsibility(Responsibility::DataAnalysis)
                .add_responsibility(Responsibility::DataSubjectRights)
                .add_responsibility(Responsibility::BreachNotification)
                .add_responsibility(Responsibility::DataProtectionImpactAssessment)
                .with_notes("Partner institution - conducts analysis"),
        )
        .with_processing_purpose("Joint COVID-19 epidemiological research study")
        .with_data_categories(vec![
            PersonalDataCategory::Regular("name".to_string()),
            PersonalDataCategory::Regular("email".to_string()),
            PersonalDataCategory::Regular("age".to_string()),
            PersonalDataCategory::Special(SpecialCategory::HealthData),
        ])
        .with_joint_controllership_basis(JointControllershipBasis::ContractualJointVenture {
            contract_type: "Research collaboration agreement".to_string(),
        })
        .with_arrangement_documented(true)
        .with_essence_available_to_data_subjects(true)
        .with_essence_availability_method(
            "Privacy notice on study website: https://covid-study.eu/privacy",
        )
        .with_rights_exercisable_against_each(true)
        .with_arrangement_date(Utc::now() - chrono::Duration::days(30))
        .with_assessment_date(Utc::now())
        .with_notes("DPIA conducted jointly, Article 9(2)(j) research exception applies");

    match arrangement.validate() {
        Ok(validation) => {
            println!("📊 Joint Controller Arrangement Validation:");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Controllers: {}", validation.controllers_count);
            println!(
                "   Contact Point Designated: {}",
                if validation.contact_point_designated {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Arrangement Documented (Article 26(1)): {}",
                if validation.arrangement_documented {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Essence Available to Data Subjects (Article 26(2)): {}",
                if validation.essence_available {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Rights Exercisable Against Each (Article 26(3)): {}",
                if validation.rights_exercisable {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Mandatory Responsibilities Allocated: {}",
                if validation.mandatory_responsibilities_allocated {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Special Categories Processed: {}",
                if validation.has_special_categories {
                    "⚠️  YES (Article 9)"
                } else {
                    "NO"
                }
            );

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!("\n✅ Joint research project demonstrates Article 26 compliance.");
            println!("   Clear responsibility allocation between institutions.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 2: Facebook Page Administrators
///
/// Based on CJEU C-210/16 Wirtschaftsakademie case - Facebook page admin is joint
/// controller with Facebook for page insights processing.
fn scenario2_facebook_page_admins() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 2: Facebook Page Administrators (CJEU C-210/16)");
    println!("──────────────────────────────────────────────────────────────\n");

    let arrangement = JointControllerArrangement::new()
        .add_controller(
            JointController::new("Meta Platforms Ireland Limited")
                .with_contact("https://www.facebook.com/help/contact/540977946302970")
                .add_responsibility(Responsibility::DataCollection)
                .add_responsibility(Responsibility::DataStorage)
                .add_responsibility(Responsibility::SecurityMeasures)
                .add_responsibility(Responsibility::BreachNotification)
                .as_contact_point()
                .with_notes("Platform provider - operates Facebook"),
        )
        .add_controller(
            JointController::new("Acme Marketing GmbH (Page Administrator)")
                .with_contact("privacy@acme-marketing.de")
                .add_responsibility(Responsibility::DataAnalysis)
                .add_responsibility(Responsibility::DataSubjectRights)
                .with_notes("Page administrator - uses Facebook Page Insights for marketing analytics"),
        )
        .with_processing_purpose("Facebook Page Insights - visitor analytics for page administrators")
        .with_data_categories(vec![
            PersonalDataCategory::Regular("Facebook user ID".to_string()),
            PersonalDataCategory::Regular("page interactions (likes, comments, shares)".to_string()),
            PersonalDataCategory::Regular("demographics (age range, gender, location)".to_string()),
        ])
        .with_joint_controllership_basis(JointControllershipBasis::PlatformUser {
            platform: "Facebook".to_string(),
            user: "Page Administrator".to_string(),
            joint_benefit: true,
        })
        .with_arrangement_documented(true)
        .with_essence_available_to_data_subjects(true)
        .with_essence_availability_method("Facebook Page Insights Controller Addendum + Page privacy notice")
        .with_rights_exercisable_against_each(true)
        .with_assessment_date(Utc::now())
        .with_notes("CJEU C-210/16: Page admins are joint controllers with Facebook for Page Insights processing");

    match arrangement.validate() {
        Ok(validation) => {
            println!("📊 Joint Controller Arrangement Validation:");
            println!("   Scenario: Facebook Page Insights (CJEU C-210/16 Wirtschaftsakademie)");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Controllers: {} (Platform + Page Admin)",
                validation.controllers_count
            );

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!("\n✅ Facebook Page administrator arrangement compliant with CJEU case law.");
            println!("   Page admins are joint controllers with Facebook for Page Insights.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 3: Joint Marketing Campaign
///
/// Two companies sharing customer data for a joint marketing campaign.
fn scenario3_joint_marketing_campaign() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 3: Joint Marketing Campaign");
    println!("──────────────────────────────────────────────────────────────\n");

    let arrangement = JointControllerArrangement::new()
        .add_controller(
            JointController::new("TechCorp AG")
                .with_contact("dpo@techcorp.com")
                .add_responsibility(Responsibility::DataCollection)
                .add_responsibility(Responsibility::ConsentManagement)
                .add_responsibility(Responsibility::DataStorage)
                .add_responsibility(Responsibility::SecurityMeasures)
                .as_contact_point()
                .with_notes("Software company - collects customer consent and manages data"),
        )
        .add_controller(
            JointController::new("RetailPro GmbH")
                .with_contact("privacy@retailpro.de")
                .add_responsibility(Responsibility::DataAnalysis)
                .add_responsibility(Responsibility::ThirdPartyDisclosure)
                .add_responsibility(Responsibility::DataSubjectRights)
                .add_responsibility(Responsibility::BreachNotification)
                .with_notes("Retail company - conducts joint marketing"),
        )
        .with_processing_purpose("Joint marketing campaign: Software + Hardware bundle promotion")
        .with_data_categories(vec![
            PersonalDataCategory::Regular("name".to_string()),
            PersonalDataCategory::Regular("email".to_string()),
            PersonalDataCategory::Regular("purchase history".to_string()),
            PersonalDataCategory::Regular("marketing preferences".to_string()),
        ])
        .with_joint_controllership_basis(JointControllershipBasis::CommonPurpose {
            purpose: "Joint marketing to promote bundle sales".to_string(),
            interests_converge: true,
        })
        .with_arrangement_documented(true)
        .with_essence_available_to_data_subjects(true)
        .with_essence_availability_method(
            "Joint privacy notice in marketing emails and campaign website",
        )
        .with_rights_exercisable_against_each(true)
        .with_arrangement_date(Utc::now() - chrono::Duration::days(60))
        .with_assessment_date(Utc::now())
        .with_notes(
            "Lawful basis: Consent (Article 6(1)(a)) - explicit consent for joint marketing",
        );

    match arrangement.validate() {
        Ok(validation) => {
            println!("📊 Joint Controller Arrangement Validation:");
            println!("   Scenario: Joint Marketing Campaign");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Controllers: {}", validation.controllers_count);

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!("\n✅ Joint marketing campaign demonstrates Article 26 compliance.");
            println!("   Clear consent management and responsibility allocation.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 4: Missing Arrangement Documentation
///
/// Non-compliant: Joint controllers without documented arrangement (Article 26(1) violation).
fn scenario4_missing_arrangement_documentation() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 4: Missing Arrangement Documentation (Non-Compliant)");
    println!("──────────────────────────────────────────────────────────────\n");

    let arrangement = JointControllerArrangement::new()
        .add_controller(
            JointController::new("Startup A")
                .add_responsibility(Responsibility::DataCollection)
                .add_responsibility(Responsibility::SecurityMeasures)
                .as_contact_point(),
        )
        .add_controller(
            JointController::new("Startup B")
                .add_responsibility(Responsibility::DataAnalysis)
                .add_responsibility(Responsibility::DataSubjectRights)
                .add_responsibility(Responsibility::BreachNotification),
        )
        .with_processing_purpose("Joint product development")
        .with_data_categories(vec![PersonalDataCategory::Regular("email".to_string())])
        // Article 26(1) violation - no documented arrangement
        .with_arrangement_documented(false)
        .with_essence_available_to_data_subjects(false)
        .with_assessment_date(Utc::now());

    match arrangement.validate() {
        Ok(validation) => {
            println!("📊 Joint Controller Arrangement Validation:");
            println!("   Scenario: Informal collaboration without written arrangement");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Arrangement Documented: {}",
                if validation.arrangement_documented {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Essence Available: {}",
                if validation.essence_available {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );

            println!("\n⚠️  Warnings ({}):", validation.warnings.len());
            for warning in &validation.warnings {
                println!("   - {}", warning);
            }

            println!("\n❌ Non-compliant: Article 26(1) requires a documented arrangement.");
            println!(
                "   Informal collaborations must formalize their joint controller relationship."
            );
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 5: Missing Contact Point
///
/// Non-compliant: No designated contact point for data subjects (Article 26(1) violation).
fn scenario5_missing_contact_point() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 5: Missing Contact Point (Non-Compliant)");
    println!("──────────────────────────────────────────────────────────────\n");

    let arrangement = JointControllerArrangement::new()
        .add_controller(
            JointController::new("Company X")
                .with_contact("legal@company-x.com")
                .add_responsibility(Responsibility::DataCollection)
                .add_responsibility(Responsibility::SecurityMeasures)
                // NOT designated as contact point
                .with_notes("Data collector"),
        )
        .add_controller(
            JointController::new("Company Y")
                .with_contact("privacy@company-y.com")
                .add_responsibility(Responsibility::DataAnalysis)
                .add_responsibility(Responsibility::DataSubjectRights)
                .add_responsibility(Responsibility::BreachNotification)
                // NOT designated as contact point either
                .with_notes("Data processor"),
        )
        .with_processing_purpose("Joint data analytics")
        .with_data_categories(vec![PersonalDataCategory::Regular("user_id".to_string())])
        .with_arrangement_documented(true)
        .with_essence_available_to_data_subjects(true)
        .with_essence_availability_method("Privacy policy")
        .with_assessment_date(Utc::now());

    match arrangement.validate() {
        Ok(validation) => {
            println!("📊 Joint Controller Arrangement Validation:");
            println!("   Scenario: No contact point designated for data subjects");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!(
                "   Contact Point Designated: {}",
                if validation.contact_point_designated {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );

            println!("\n⚠️  Warnings:");
            for warning in &validation.warnings {
                println!("   - {}", warning);
            }

            println!("\n❌ Non-compliant: Article 26(1) requires designating a contact point.");
            println!("   Data subjects must know where to exercise their rights.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}

/// Scenario 6: Healthcare Data Sharing
///
/// Joint controllers processing Article 9 health data (hospitals sharing patient data).
fn scenario6_healthcare_data_sharing() {
    println!("──────────────────────────────────────────────────────────────");
    println!("Scenario 6: Healthcare Data Sharing (Article 9 Special Categories)");
    println!("──────────────────────────────────────────────────────────────\n");

    let arrangement = JointControllerArrangement::new()
        .add_controller(
            JointController::new("Charité - Universitätsmedizin Berlin")
                .with_contact("datenschutz@charite.de")
                .add_responsibility(Responsibility::DataCollection)
                .add_responsibility(Responsibility::DataStorage)
                .add_responsibility(Responsibility::SecurityMeasures)
                .add_responsibility(Responsibility::BreachNotification)
                .add_responsibility(Responsibility::DataProtectionImpactAssessment)
                .as_contact_point()
                .with_notes("Lead hospital - manages patient database"),
        )
        .add_controller(
            JointController::new("University Hospital Munich")
                .with_contact("dpo@klinikum.uni-muenchen.de")
                .add_responsibility(Responsibility::DataAnalysis)
                .add_responsibility(Responsibility::DataSubjectRights)
                .add_responsibility(Responsibility::RecordsOfProcessing)
                .with_notes("Partner hospital - conducts research analysis"),
        )
        .with_processing_purpose("Joint cancer treatment research and patient care coordination")
        .with_data_categories(vec![
            PersonalDataCategory::Regular("patient name".to_string()),
            PersonalDataCategory::Regular("date of birth".to_string()),
            PersonalDataCategory::Special(SpecialCategory::HealthData),
            PersonalDataCategory::Special(SpecialCategory::GeneticData),
        ])
        .with_joint_controllership_basis(JointControllershipBasis::StatutoryRequirement {
            legal_basis: "German Hospital Act (Krankenhausgesetz) - inter-hospital cooperation".to_string(),
        })
        .with_arrangement_documented(true)
        .with_essence_available_to_data_subjects(true)
        .with_essence_availability_method("Patient information leaflet and hospital privacy notice")
        .with_rights_exercisable_against_each(true)
        .with_arrangement_date(Utc::now() - chrono::Duration::days(90))
        .with_assessment_date(Utc::now())
        .with_notes("Lawful basis: Article 6(1)(e) public task + Article 9(2)(h) healthcare. DPIA conducted jointly. ISO 27799 compliant.");

    match arrangement.validate() {
        Ok(validation) => {
            println!("📊 Joint Controller Arrangement Validation:");
            println!("   Scenario: Inter-hospital patient data sharing for treatment and research");
            println!(
                "   Compliant: {}",
                if validation.compliant {
                    "✅ YES"
                } else {
                    "❌ NO"
                }
            );
            println!("   Controllers: {} hospitals", validation.controllers_count);
            println!(
                "   Special Categories: {}",
                if validation.has_special_categories {
                    "⚠️  YES (Article 9 health + genetic data)"
                } else {
                    "NO"
                }
            );
            println!(
                "   Mandatory Responsibilities: {}",
                if validation.mandatory_responsibilities_allocated {
                    "✅ Allocated"
                } else {
                    "❌ Missing"
                }
            );

            if !validation.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &validation.warnings {
                    println!("   - {}", warning);
                }
            }

            if !validation.recommendations.is_empty() {
                println!("\n💡 Recommendations:");
                for rec in &validation.recommendations {
                    println!("   - {}", rec);
                }
            }

            println!(
                "\n✅ Healthcare joint controller arrangement demonstrates Article 26 compliance."
            );
            println!("   Article 9(2)(h) healthcare exception applies for health data processing.");
            println!("   Enhanced security measures appropriate for sensitive health data.");
        }
        Err(e) => println!("❌ Error: {}", e),
    }

    println!();
}
