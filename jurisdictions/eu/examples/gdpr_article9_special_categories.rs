//! GDPR Article 9 - Special Categories Processing Example
//!
//! Demonstrates the 10 exceptions under Article 9(2) for processing special categories

use legalis_eu::gdpr::*;

fn main() {
    println!("=== GDPR Article 9 - Special Categories Processing ===\n");

    // Example 1: Explicit consent for health research
    println!("Example 1: Article 9(2)(a) - Explicit Consent for Medical Research");
    let research = Article9Processing::new()
        .with_controller("University Medical Research Center")
        .with_purpose("COVID-19 vaccine efficacy study")
        .add_special_category(SpecialCategory::HealthData)
        .add_special_category(SpecialCategory::GeneticData)
        .with_exception(Article9Exception::ExplicitConsent {
            purposes: vec![
                "Analyze immune response to vaccine".to_string(),
                "Identify genetic markers affecting efficacy".to_string(),
            ],
            consent_documented: true,
        });

    match research.validate() {
        Ok(validation) => {
            if validation.is_compliant() {
                println!("   ✅ Processing is GDPR compliant");
                println!("   Exception: Article 9(2)(a) - Explicit consent");
                println!("   Special categories: {:?}", validation.special_categories);
            }
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 2: Healthcare - Medical diagnosis
    println!("Example 2: Article 9(2)(h) - Healthcare/Medical Diagnosis");
    let diagnosis = Article9Processing::new()
        .with_controller("City Hospital")
        .with_purpose("Patient diagnosis and treatment")
        .add_special_category(SpecialCategory::HealthData)
        .with_exception(Article9Exception::Healthcare {
            purpose: HealthcarePurpose::MedicalDiagnosis,
            professional_secrecy: true,
        });

    match diagnosis.validate() {
        Ok(validation) => {
            println!("   ✅ Valid healthcare processing");
            println!("   Note: Must be processed by/under professional secrecy obligation");
            println!("   Compliance: {:?}", validation.compliance_status);
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 3: Vital interests (emergency - unable to consent)
    println!("Example 3: Article 9(2)(c) - Vital Interests (Emergency)");
    let emergency = Article9Processing::new()
        .with_controller("Emergency Services")
        .with_purpose("Life-saving treatment for unconscious patient")
        .add_special_category(SpecialCategory::HealthData)
        .with_exception(Article9Exception::VitalInterestsUnableToConsent {
            life_threatening: true,
            unable_to_consent: true,
        });

    match emergency.validate() {
        Ok(_validation) => {
            println!("   ✅ Emergency processing authorized");
            println!("   Conditions:");
            println!("      - Life-threatening situation: YES");
            println!("      - Data subject unable to consent: YES");
            println!("   Note: Only applies when patient cannot give consent");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 4: Scientific research with safeguards
    println!("Example 4: Article 9(2)(j) - Scientific Research with Safeguards");
    let science = Article9Processing::new()
        .with_controller("National Research Institute")
        .with_purpose("Epidemiological study on chronic diseases")
        .add_special_category(SpecialCategory::HealthData)
        .add_special_category(SpecialCategory::GeneticData)
        .with_exception(Article9Exception::ArchivingResearchStatistics {
            purpose: ResearchPurpose::ScientificResearch,
            legal_basis: "EU Research Framework Regulation".to_string(),
            technical_organizational_measures: vec![
                "Pseudonymization of all personal data".to_string(),
                "Data minimization (only necessary data)".to_string(),
                "Access controls and encryption".to_string(),
                "Regular audits and oversight".to_string(),
            ],
        });

    match science.validate() {
        Ok(validation) => {
            println!("   ✅ Research processing authorized");
            println!("   Legal basis: EU Research Framework Regulation");
            println!("   Safeguards implemented:");
            if let Article9Exception::ArchivingResearchStatistics {
                technical_organizational_measures,
                ..
            } = &validation.exception
            {
                for measure in technical_organizational_measures {
                    println!("      - {}", measure);
                }
            }
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 5: Employment law
    println!("Example 5: Article 9(2)(b) - Employment/Social Security Law");
    let employment = Article9Processing::new()
        .with_controller("Company HR Department")
        .with_purpose("Disability accommodation under employment law")
        .add_special_category(SpecialCategory::HealthData)
        .with_exception(Article9Exception::EmploymentSocialSecurityLaw {
            legal_basis: "EU Employment Equality Directive 2000/78/EC".to_string(),
            authorized_by_union_or_member_state_law: true,
        });

    match employment.validate() {
        Ok(_validation) => {
            println!("   ✅ Employment law exception valid");
            println!("   Legal basis: EU Employment Equality Directive");
            println!("   Purpose: Providing reasonable accommodations");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 6: Public health
    println!("Example 6: Article 9(2)(i) - Public Health");
    let public_health = Article9Processing::new()
        .with_controller("National Public Health Agency")
        .with_purpose("Epidemic monitoring and control")
        .add_special_category(SpecialCategory::HealthData)
        .with_exception(Article9Exception::PublicHealth {
            public_health_purpose: "Monitoring COVID-19 spread, cross-border health threats"
                .to_string(),
            authorized_by_union_or_member_state_law: true,
            professional_secrecy: true,
        });

    match public_health.validate() {
        Ok(_validation) => {
            println!("   ✅ Public health processing authorized");
            println!("   Purpose: Epidemic monitoring (serious cross-border threat)");
            println!("   Processed under professional secrecy obligation");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 7: Data manifestly made public (requires human judgment)
    println!("Example 7: Article 9(2)(e) - Data Manifestly Made Public");
    let public_data = Article9Processing::new()
        .with_controller("News Organization")
        .with_purpose("Reporting on public figure's political campaign")
        .add_special_category(SpecialCategory::PoliticalOpinions)
        .with_exception(Article9Exception::DataManifestlyMadePublic {
            public_source: "Candidate's official campaign website and public speeches".to_string(),
        });

    match public_data.validate() {
        Ok(validation) => {
            use legalis_core::LegalResult;
            println!("   ⚖️  Requires human judgment");
            if let LegalResult::JudicialDiscretion {
                issue,
                narrative_hint,
                ..
            } = &validation.exception_valid
            {
                println!("   Issue: {}", issue);
                if let Some(hint) = narrative_hint {
                    println!("   Guidance: {}", hint);
                }
            }
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 8: Foundation/association
    println!("Example 8: Article 9(2)(d) - Legitimate Activities of Foundation");
    let foundation = Article9Processing::new()
        .with_controller("Religious Charity Foundation")
        .with_purpose("Member support services")
        .add_special_category(SpecialCategory::ReligiousBeliefs)
        .with_exception(Article9Exception::LegitimateActivitiesFoundation {
            organization_type: "Non-profit religious foundation".to_string(),
            limited_to_members: true,
            no_disclosure_without_consent: true,
        });

    match foundation.validate() {
        Ok(_validation) => {
            println!("   ✅ Foundation processing authorized");
            println!("   Conditions:");
            println!("      - Processing limited to members/former members");
            println!("      - No disclosure outside organization without consent");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 9: Legal claims
    println!("Example 9: Article 9(2)(f) - Legal Claims");
    let legal = Article9Processing::new()
        .with_controller("Law Firm")
        .with_purpose("Defense of employment discrimination lawsuit")
        .add_special_category(SpecialCategory::HealthData)
        .add_special_category(SpecialCategory::RacialEthnicOrigin)
        .with_exception(Article9Exception::LegalClaims {
            claim_description: "Defending client against discrimination claims".to_string(),
        });

    match legal.validate() {
        Ok(_validation) => {
            println!("   ✅ Legal claims exception applies");
            println!("   Necessary for establishment, exercise, or defense of legal claims");
        }
        Err(e) => println!("   ❌ Error: {}", e),
    }

    println!("\n---\n");

    // Example 10: Invalid - missing exception
    println!("Example 10: Invalid - Processing Special Categories Without Exception");
    let invalid = Article9Processing::new()
        .with_controller("Marketing Company")
        .with_purpose("Targeted advertising")
        .add_special_category(SpecialCategory::HealthData);
    // Note: No exception provided

    match invalid.validate() {
        Ok(_) => println!("   Unexpectedly valid"),
        Err(e) => {
            println!("   ❌ Processing PROHIBITED");
            println!("   Error: {}", e);
            println!("\n   Article 9(1) prohibits processing special categories");
            println!("   unless one of the 10 exceptions under Article 9(2) applies.");
        }
    }

    println!("\n---\n");

    // Summary
    println!("=== Article 9 Key Takeaways ===");
    println!("1. Special categories are PROHIBITED by default (Article 9(1))");
    println!("2. 10 exceptions under Article 9(2):");
    println!("   (a) Explicit consent");
    println!("   (b) Employment/social security law");
    println!("   (c) Vital interests (unable to consent)");
    println!("   (d) Legitimate activities of foundations");
    println!("   (e) Data manifestly made public");
    println!("   (f) Legal claims");
    println!("   (g) Substantial public interest");
    println!("   (h) Healthcare/medical diagnosis");
    println!("   (i) Public health");
    println!("   (j) Archiving/research/statistics");
    println!("\n3. Article 9 exception is ADDITIONAL to Article 6 lawful basis");
    println!("4. Higher bar than regular processing (explicit consent, not just consent)");
    println!("5. Many exceptions require Union/Member State law authorization");

    println!("\n=== End of Example ===");
}
