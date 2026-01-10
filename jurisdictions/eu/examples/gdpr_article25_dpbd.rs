//! GDPR Article 25 - Data Protection by Design and by Default Example
//!
//! This example demonstrates data protection by design (DPbD) and data protection
//! by default (DPbDefault) under Article 25 GDPR.
//!
//! ## Scenarios Covered
//!
//! 1. Complete e-commerce website (fully compliant)
//! 2. Social media platform with privacy-preserving defaults
//! 3. Healthcare system with enhanced privacy technologies
//! 4. Non-compliant system missing data minimisation
//! 5. System missing default settings (Article 25(2))
//! 6. Legacy system retrofitted with privacy measures

use chrono::Utc;
use legalis_eu::gdpr::*;

fn main() -> Result<(), GdprError> {
    println!("=== GDPR Article 25 - Data Protection by Design and by Default Examples ===\n");

    scenario_1_complete_ecommerce_website()?;
    scenario_2_social_media_privacy_preserving()?;
    scenario_3_healthcare_enhanced_privacy()?;
    scenario_4_missing_data_minimisation()?;
    scenario_5_missing_default_settings()?;
    scenario_6_legacy_system_retrofitted()?;

    println!("\n✅ All Article 25 scenarios completed");
    Ok(())
}

/// Scenario 1: Complete E-Commerce Website (Fully Compliant)
fn scenario_1_complete_ecommerce_website() -> Result<(), GdprError> {
    println!("## Scenario 1: E-Commerce Website (Fully Compliant DPbD)\\n");

    let dpbd = DataProtectionByDesign::new()
        .with_system_name("ShopEU E-Commerce Platform")
        .with_processing_purpose("Online retail - customer orders and account management")
        // Article 25(1) - Design Principles (Article 5)
        .add_design_principle(DesignPrinciple::DataMinimisation {
            only_necessary_data: true,
            justification: "Only collect: name, email, shipping address, payment method. No tracking cookies without consent.".to_string(),
        })
        .add_design_principle(DesignPrinciple::PurposeLimitation {
            limited_to_purpose: true,
            documented: true,
        })
        .add_design_principle(DesignPrinciple::StorageLimitation {
            retention_period_defined: true,
            automated_deletion: true,
        })
        .add_design_principle(DesignPrinciple::Accuracy {
            validation_implemented: true,
            correction_mechanism: true,
        })
        .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
            security_measures_integrated: true,
            access_controls: true,
        })
        .add_design_principle(DesignPrinciple::Transparency {
            privacy_notice: true,
            clear_language: true,
        })
        .add_design_principle(DesignPrinciple::LawfulnessFairness {
            lawful_basis_identified: true,
            no_deceptive_practices: true,
        })
        .add_design_principle(DesignPrinciple::Accountability {
            documentation: true,
            compliance_monitoring: true,
        })
        // Article 25(2) - Default Settings
        .add_default_setting(DefaultSetting::MinimalDataCollection {
            optional_fields_opt_in: true,
            no_pre_ticked_boxes: true,
        })
        .add_default_setting(DefaultSetting::LimitedProcessing {
            purpose_specific: true,
            no_excessive_processing: true,
        })
        .add_default_setting(DefaultSetting::LimitedStorage {
            shortest_necessary_period: true,
            automatic_deletion: true,
        })
        .add_default_setting(DefaultSetting::LimitedAccessibility {
            need_to_know_basis: true,
            role_based_access: true,
        })
        .add_default_setting(DefaultSetting::PrivacyPreservingDefaults {
            strictest_privacy_settings: true,
            user_must_opt_out: false,
        })
        .add_default_setting(DefaultSetting::MinimalThirdPartyDisclosure {
            no_default_sharing: true,
            explicit_consent_required: true,
        })
        // Privacy-Enhancing Technologies
        .add_privacy_technology(PrivacyEnhancingTechnology::Encryption {
            algorithm: "AES-256-GCM for payment data, TLS 1.3 in transit".to_string(),
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::Pseudonymisation {
            method: "Order IDs pseudonymised, analytics data anonymised".to_string(),
        })
        // Article 25(1) Considerations
        .with_state_of_art_considered(true)
        .with_costs_considered(true)
        .with_processing_context_considered(true)
        .with_risks_assessed(true)
        .with_assessment_date(Utc::now())
        .with_responsible_party("Chief Privacy Officer")
        .with_notes("Design reviewed by data protection team. Privacy by design principles applied from initial architecture.");

    let validation = dpbd.validate()?;

    println!("System: ShopEU E-Commerce Platform");
    println!("Purpose: Online retail and account management");
    println!();

    println!("ARTICLE 25(1) - DATA PROTECTION BY DESIGN:");
    println!(
        "Design Principles Implemented: {}",
        validation.design_principles_count
    );
    println!("  ✓ Data minimisation (Article 5(1)(c))");
    println!("  ✓ Purpose limitation (Article 5(1)(b))");
    println!("  ✓ Storage limitation (Article 5(1)(e))");
    println!("  ✓ Accuracy (Article 5(1)(d))");
    println!("  ✓ Integrity and confidentiality (Article 5(1)(f))");
    println!("  ✓ Transparency (Article 5(1)(a))");
    println!("  ✓ Lawfulness and fairness (Article 5(1)(a))");
    println!("  ✓ Accountability (Article 5(2))");
    println!();

    println!("ARTICLE 25(2) - DATA PROTECTION BY DEFAULT:");
    println!(
        "Default Settings Configured: {}",
        validation.default_settings_count
    );
    println!("  ✓ Minimal data collection (optional fields opt-in)");
    println!("  ✓ Limited processing (purpose-specific)");
    println!("  ✓ Limited storage (automatic deletion)");
    println!("  ✓ Limited accessibility (need-to-know basis)");
    println!("  ✓ Privacy-preserving defaults (strictest settings)");
    println!("  ✓ Minimal third-party disclosure (no default sharing)");
    println!();

    println!("PRIVACY-ENHANCING TECHNOLOGIES:");
    println!("  ✓ Encryption: AES-256-GCM + TLS 1.3");
    println!("  ✓ Pseudonymisation: Order IDs and analytics");
    println!();

    println!("ARTICLE 25(1) CONSIDERATIONS:");
    println!("  ✓ State of the art considered");
    println!("  ✓ Implementation costs considered");
    println!("  ✓ Processing context considered");
    println!("  ✓ Risks to rights and freedoms assessed");
    println!();

    println!(
        "OVERALL COMPLIANCE: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Article 25 requires privacy to be 'baked in' from the design stage:");
    println!("   - DPbD (25(1)): Implement Article 5 principles in system design");
    println!("   - DPbDefault (25(2)): Most privacy-protective settings by default");
    println!("   - Users should NOT have to configure privacy - it's the default\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 2: Social Media Platform with Privacy-Preserving Defaults
fn scenario_2_social_media_privacy_preserving() -> Result<(), GdprError> {
    println!("## Scenario 2: Social Media Platform (Privacy-Preserving Defaults)\\n");

    let dpbd = DataProtectionByDesign::new()
        .with_system_name("EuroSocial - Privacy-First Social Network")
        .with_processing_purpose("Social networking and user-generated content sharing")
        .add_design_principle(DesignPrinciple::DataMinimisation {
            only_necessary_data: true,
            justification: "Username, email, optional profile info. No location tracking, no friend scraping.".to_string(),
        })
        .add_design_principle(DesignPrinciple::PurposeLimitation {
            limited_to_purpose: true,
            documented: true,
        })
        .add_design_principle(DesignPrinciple::Transparency {
            privacy_notice: true,
            clear_language: true,
        })
        .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
            security_measures_integrated: true,
            access_controls: true,
        })
        // CRITICAL: Article 25(2) Defaults for Social Media
        .add_default_setting(DefaultSetting::MinimalDataCollection {
            optional_fields_opt_in: true,
            no_pre_ticked_boxes: true,
        })
        .add_default_setting(DefaultSetting::PrivacyPreservingDefaults {
            strictest_privacy_settings: true,
            user_must_opt_out: false,
        })
        .add_default_setting(DefaultSetting::MinimalThirdPartyDisclosure {
            no_default_sharing: true,
            explicit_consent_required: true,
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::Encryption {
            algorithm: "End-to-end encryption for direct messages".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_costs_considered(true)
        .with_processing_context_considered(true)
        .with_risks_assessed(true)
        .with_notes("Default settings: Profile private, posts friends-only, no location sharing, no behavioral advertising without consent.");

    let validation = dpbd.validate()?;

    println!("System: EuroSocial - Privacy-First Social Network");
    println!("Context: High-risk processing (profiling, behavioral analysis)");
    println!();

    println!("PRIVACY-PRESERVING DEFAULTS (Article 25(2)):");
    println!("  ✓ Profile visibility: PRIVATE by default (user must opt-in to public)");
    println!("  ✓ Posts visibility: FRIENDS-ONLY by default");
    println!("  ✓ Location sharing: OFF by default");
    println!("  ✓ Behavioral advertising: OFF by default (requires explicit consent)");
    println!("  ✓ Third-party data sharing: OFF by default");
    println!("  ✓ Friend suggestions: OFF by default (no contact scraping)");
    println!();

    println!("CONTRAST WITH NON-COMPLIANT DEFAULTS:");
    println!("  ❌ Profile public by default");
    println!("  ❌ Location tracking enabled by default");
    println!("  ❌ Behavioral ads with pre-ticked consent boxes");
    println!("  ❌ Third-party data sharing unless user opts out");
    println!();

    println!(
        "COMPLIANCE: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Article 25(2) is CRITICAL for social media:");
    println!("   - Users must OPT-IN to data sharing, not OPT-OUT");
    println!("   - Most restrictive privacy settings must be DEFAULT");
    println!("   - Pre-ticked consent boxes violate Article 25(2)");
    println!("   - Recital 78: \"The controller should [...] ensure that by default");
    println!("     personal data are not made accessible without the individual's");
    println!("     intervention to an indefinite number of natural persons.\"\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 3: Healthcare System with Enhanced Privacy Technologies
fn scenario_3_healthcare_enhanced_privacy() -> Result<(), GdprError> {
    println!("## Scenario 3: Healthcare System (Enhanced Privacy Technologies)\\n");

    let dpbd = DataProtectionByDesign::new()
        .with_system_name("MedSecure - Electronic Health Records")
        .with_processing_purpose("Patient electronic health records and medical research")
        .add_design_principle(DesignPrinciple::DataMinimisation {
            only_necessary_data: true,
            justification: "Only clinical data necessary for treatment and care coordination".to_string(),
        })
        .add_design_principle(DesignPrinciple::PurposeLimitation {
            limited_to_purpose: true,
            documented: true,
        })
        .add_design_principle(DesignPrinciple::Accuracy {
            validation_implemented: true,
            correction_mechanism: true,
        })
        .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
            security_measures_integrated: true,
            access_controls: true,
        })
        .add_design_principle(DesignPrinciple::Accountability {
            documentation: true,
            compliance_monitoring: true,
        })
        .add_default_setting(DefaultSetting::MinimalDataCollection {
            optional_fields_opt_in: true,
            no_pre_ticked_boxes: true,
        })
        .add_default_setting(DefaultSetting::LimitedAccessibility {
            need_to_know_basis: true,
            role_based_access: true,
        })
        .add_default_setting(DefaultSetting::LimitedStorage {
            shortest_necessary_period: true,
            automatic_deletion: true,
        })
        // Advanced Privacy-Enhancing Technologies
        .add_privacy_technology(PrivacyEnhancingTechnology::Encryption {
            algorithm: "FIPS 140-2 validated encryption modules".to_string(),
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::Pseudonymisation {
            method: "Clinical trial patient pseudonymisation with separate key storage".to_string(),
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::Anonymisation {
            technique: "k-anonymity for research datasets (k=5)".to_string(),
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::DifferentialPrivacy {
            epsilon: 0.1,
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::HomomorphicEncryption {
            scheme: "Partially homomorphic encryption for encrypted analytics".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_costs_considered(true)
        .with_processing_context_considered(true)
        .with_risks_assessed(true)
        .with_notes("Article 9 special category data requires enhanced privacy measures. Research uses differential privacy and homomorphic encryption.");

    let validation = dpbd.validate()?;

    println!("System: MedSecure - Electronic Health Records");
    println!("Data Type: Article 9 special category (health data)");
    println!("Context: High-risk processing requiring enhanced privacy");
    println!();

    println!("PRIVACY-ENHANCING TECHNOLOGIES (State of the Art):");
    println!("  ✓ FIPS 140-2 validated encryption");
    println!("  ✓ Pseudonymisation for clinical trials");
    println!("  ✓ k-anonymity (k=5) for research datasets");
    println!("  ✓ Differential privacy (ε=0.1) for aggregate queries");
    println!("  ✓ Homomorphic encryption for encrypted analytics");
    println!();

    println!("DEFAULT SETTINGS:");
    println!("  ✓ Access: Need-to-know basis (doctor can only see assigned patients)");
    println!("  ✓ Storage: Medical records auto-archived after regulatory retention period");
    println!("  ✓ Research: Patient data pseudonymised by default for research");
    println!();

    println!(
        "COMPLIANCE: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Article 25 + Article 9 special categories:");
    println!("   - Enhanced privacy technologies required (state of the art)");
    println!("   - Differential privacy for research analytics");
    println!("   - Homomorphic encryption allows computation on encrypted data");
    println!("   - k-anonymity prevents re-identification in datasets");
    println!("   - Article 25 = proactive privacy, not reactive compliance\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 4: Non-Compliant System (Missing Data Minimisation)
fn scenario_4_missing_data_minimisation() -> Result<(), GdprError> {
    println!("## Scenario 4: Non-Compliant System (Missing Data Minimisation)\\n");

    let dpbd = DataProtectionByDesign::new()
        .with_system_name("DataHoarder CRM")
        .with_processing_purpose("Customer relationship management")
        // Missing: DataMinimisation principle
        .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
            security_measures_integrated: true,
            access_controls: true,
        })
        .add_design_principle(DesignPrinciple::Transparency {
            privacy_notice: true,
            clear_language: true,
        })
        .add_default_setting(DefaultSetting::LimitedAccessibility {
            need_to_know_basis: true,
            role_based_access: true,
        })
        .with_state_of_art_considered(true)
        .with_costs_considered(true)
        .with_processing_context_considered(true)
        .with_risks_assessed(true)
        .with_notes("System collects excessive data: birthdate, gender, income, education, browsing history, social media profiles - NOT necessary for CRM purpose.");

    let validation = dpbd.validate()?;

    println!("System: DataHoarder CRM");
    println!("Issue: Collects excessive data not necessary for stated purpose");
    println!();

    println!("DATA COLLECTED:");
    println!("  ✓ Name, email, phone (NECESSARY for CRM)");
    println!("  ❌ Birthdate (NOT necessary)");
    println!("  ❌ Gender (NOT necessary)");
    println!("  ❌ Income level (NOT necessary)");
    println!("  ❌ Education (NOT necessary)");
    println!("  ❌ Browsing history (NOT necessary)");
    println!("  ❌ Social media profiles (NOT necessary)");
    println!();

    println!("COMPLIANCE: ❌ NON-COMPLIANT");
    println!();

    println!("⚠️ VIOLATIONS:");
    for warning in &validation.warnings {
        println!("  - {}", warning);
    }

    println!("\n💡 KEY POINT:");
    println!("   Article 25(1) + Article 5(1)(c) Data Minimisation:");
    println!("   - Collect ONLY data necessary for the stated purpose");
    println!("   - 'Just in case' data collection violates GDPR");
    println!("   - Ask: 'Can we achieve the purpose WITHOUT this data?'");
    println!("   - If yes, DON'T collect it");
    println!();
    println!("   EDPB Guidelines: 'The controller should not collect personal data");
    println!("   on the off-chance that it might be useful in the future.'\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 5: System Missing Default Settings (Article 25(2))
fn scenario_5_missing_default_settings() -> Result<(), GdprError> {
    println!("## Scenario 5: System Missing Default Settings (Article 25(2))\\n");

    let dpbd = DataProtectionByDesign::new()
        .with_system_name("OnlineAds Platform")
        .with_processing_purpose("Online advertising and user profiling")
        .add_design_principle(DesignPrinciple::DataMinimisation {
            only_necessary_data: true,
            justification: "Only collect data necessary for ad delivery".to_string(),
        })
        .add_design_principle(DesignPrinciple::Transparency {
            privacy_notice: true,
            clear_language: true,
        })
        .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
            security_measures_integrated: true,
            access_controls: true,
        })
        // PROBLEM: No default settings configured
        // System requires users to manually configure all privacy settings
        .with_state_of_art_considered(true)
        .with_costs_considered(true)
        .with_processing_context_considered(true)
        .with_risks_assessed(true)
        .with_notes("Platform requires users to manually opt-out of behavioral tracking, third-party sharing, and profiling. Pre-ticked consent boxes used.");

    let validation = dpbd.validate()?;

    println!("System: OnlineAds Platform");
    println!("Issue: No privacy-protective default settings");
    println!();

    println!("CURRENT DEFAULTS (NON-COMPLIANT):");
    println!("  ❌ Behavioral tracking: ENABLED by default");
    println!("  ❌ Third-party data sharing: ENABLED by default");
    println!("  ❌ Profiling: ENABLED by default");
    println!("  ❌ Consent: Pre-ticked checkboxes");
    println!("  ❌ User must manually OPT-OUT of everything");
    println!();

    println!("COMPLIANCE: ❌ NON-COMPLIANT");
    println!();

    println!("⚠️ VIOLATIONS:");
    for warning in &validation.warnings {
        println!("  - {}", warning);
    }

    println!("\nREQUIRED DEFAULTS (Article 25(2)):");
    println!("  ✓ Behavioral tracking: DISABLED by default (user must opt-in)");
    println!("  ✓ Third-party sharing: DISABLED by default");
    println!("  ✓ Profiling: DISABLED by default (or minimal profiling only)");
    println!("  ✓ Consent: NOT pre-ticked");
    println!("  ✓ Privacy-first settings by default");

    println!("\n💡 KEY POINT:");
    println!("   Article 25(2) violations are COMMON in adtech:");
    println!("   - Pre-ticked consent boxes = NOT valid consent (Article 7)");
    println!("   - Pre-ticked boxes also violate Article 25(2) defaults");
    println!("   - Users should NOT have to 'hunt' for privacy settings");
    println!("   - Default should be MOST privacy-protective, not LEAST");
    println!();
    println!("   WP29 Opinion 5/2018: 'Pre-ticked boxes do not constitute");
    println!("   freely given, specific, informed consent.'\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}

/// Scenario 6: Legacy System Retrofitted with Privacy Measures
fn scenario_6_legacy_system_retrofitted() -> Result<(), GdprError> {
    println!(
        "## Scenario 6: Legacy System Retrofitted with Privacy (Article 25 Applied Retroactively)\\n"
    );

    let dpbd = DataProtectionByDesign::new()
        .with_system_name("Legacy ERP System (Post-GDPR Retrofit)")
        .with_processing_purpose("Enterprise resource planning - employee and customer data")
        .add_design_principle(DesignPrinciple::DataMinimisation {
            only_necessary_data: true,
            justification: "Conducted data audit, removed 40% of collected fields as unnecessary".to_string(),
        })
        .add_design_principle(DesignPrinciple::StorageLimitation {
            retention_period_defined: true,
            automated_deletion: true,
        })
        .add_design_principle(DesignPrinciple::IntegrityConfidentiality {
            security_measures_integrated: true,
            access_controls: true,
        })
        .add_design_principle(DesignPrinciple::Accountability {
            documentation: true,
            compliance_monitoring: true,
        })
        .add_default_setting(DefaultSetting::MinimalDataCollection {
            optional_fields_opt_in: true,
            no_pre_ticked_boxes: true,
        })
        .add_default_setting(DefaultSetting::LimitedAccessibility {
            need_to_know_basis: true,
            role_based_access: true,
        })
        .add_default_setting(DefaultSetting::LimitedStorage {
            shortest_necessary_period: true,
            automatic_deletion: true,
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::Encryption {
            algorithm: "Retrofitted field-level encryption for sensitive data".to_string(),
        })
        .add_privacy_technology(PrivacyEnhancingTechnology::Pseudonymisation {
            method: "Employee IDs pseudonymised in reporting modules".to_string(),
        })
        .with_state_of_art_considered(true)
        .with_costs_considered(true)
        .with_processing_context_considered(true)
        .with_risks_assessed(true)
        .with_notes("Pre-GDPR legacy system retrofitted with Article 25 measures. Data audit reduced data collection by 40%. Automated retention policies added.");

    let validation = dpbd.validate()?;

    println!("System: Legacy ERP (Pre-GDPR) → Retrofitted");
    println!("Scenario: Applying Article 25 to existing system");
    println!();

    println!("RETROFIT MEASURES:");
    println!("  ✓ Data audit: Removed 40% of collected fields");
    println!("  ✓ Field-level encryption added for sensitive data");
    println!("  ✓ Pseudonymisation for reporting modules");
    println!("  ✓ Automated retention and deletion policies");
    println!("  ✓ Role-based access controls (RBAC) implemented");
    println!("  ✓ Optional fields changed from mandatory to opt-in");
    println!();

    println!("BEFORE RETROFIT:");
    println!("  ❌ Collected excessive data 'just in case'");
    println!("  ❌ No retention policies (data kept indefinitely)");
    println!("  ❌ All users could access all data");
    println!("  ❌ No encryption");
    println!();

    println!("AFTER RETROFIT:");
    println!("  ✓ Data minimisation applied");
    println!("  ✓ Automated deletion after retention period");
    println!("  ✓ Need-to-know access (RBAC)");
    println!("  ✓ Encryption and pseudonymisation");
    println!();

    println!(
        "COMPLIANCE: {}",
        if validation.compliant {
            "✅ COMPLIANT"
        } else {
            "❌ NON-COMPLIANT"
        }
    );

    println!("\n💡 KEY POINT:");
    println!("   Article 25 is NOT just for new systems:");
    println!("   - Existing systems must be retrofitted to comply");
    println!("   - Conduct data audit: What do you collect? Is it necessary?");
    println!("   - Remove unnecessary data fields");
    println!("   - Add privacy-protective defaults");
    println!("   - Implement retention and deletion policies");
    println!();
    println!("   EDPB: 'The principle of data protection by design and by default");
    println!("   should also apply to organisational processes and business practices.'\n");

    println!("{}", "=".repeat(70));
    println!();

    Ok(())
}
