//! Internationalized Error Messages for GDPR Compliance
//!
//! This example demonstrates how to display GDPR compliance error messages
//! in 11 languages for better user experience across the European Union.
//!
//! Supported Languages: EN, DE, FR, ES, IT, PL, NL, PT, SV, CS, EL
//!
//! Run with: cargo run --example gdpr_i18n_errors

use legalis_eu::gdpr::article6::DataProcessing;
use legalis_eu::gdpr::error::GdprError;
use legalis_eu::gdpr::types::{LawfulBasis, PersonalDataCategory, ProcessingOperation};

fn main() {
    println!("=== GDPR Internationalized Error Messages ===\n");

    // Scenario 1: Missing controller
    println!("Scenario 1: Missing Data Controller");
    println!("{}", "=".repeat(60));
    let processing = DataProcessing::new()
        .with_purpose("Customer analytics")
        .add_data_category(PersonalDataCategory::Regular("email".to_string()))
        .with_operation(ProcessingOperation::Collection);

    if let Err(error) = processing.validate() {
        println!("🇬🇧 English:  {}", error.message("en"));
        println!("🇩🇪 Deutsch:  {}", error.message("de"));
        println!("🇫🇷 Français: {}", error.message("fr"));
    }

    // Scenario 2: Missing lawful basis (ALL LANGUAGES)
    println!("\n\nScenario 2: Missing Lawful Basis - All 11 Languages");
    println!("{}", "=".repeat(60));
    let processing = DataProcessing::new()
        .with_controller("Acme Corp")
        .with_purpose("Marketing")
        .add_data_category(PersonalDataCategory::Regular("name".to_string()));

    if let Err(error) = processing.validate() {
        println!("🇬🇧 English:    {}", error.message("en"));
        println!("🇩🇪 Deutsch:    {}", error.message("de"));
        println!("🇫🇷 Français:   {}", error.message("fr"));
        println!("🇪🇸 Español:    {}", error.message("es"));
        println!("🇮🇹 Italiano:   {}", error.message("it"));
        println!("🇵🇱 Polski:     {}", error.message("pl"));
        println!("🇳🇱 Nederlands: {}", error.message("nl"));
        println!("🇵🇹 Português:  {}", error.message("pt"));
        println!("🇸🇪 Svenska:    {}", error.message("sv"));
        println!("🇨🇿 Čeština:    {}", error.message("cs"));
        println!("🇬🇷 Ελληνικά:   {}", error.message("el"));
    }

    // Scenario 3: Invalid consent
    println!("\n\nScenario 3: Invalid Consent (not freely given)");
    println!("{}", "=".repeat(60));
    let processing = DataProcessing::new()
        .with_controller("Social Network GmbH")
        .with_purpose("User profiling")
        .add_data_category(PersonalDataCategory::Regular(
            "browsing history".to_string(),
        ))
        .with_operation(ProcessingOperation::Use)
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: false, // ❌ Not freely given
            specific: true,
            informed: true,
            unambiguous: true,
        });

    if let Err(error) = processing.validate() {
        println!("🇬🇧 English:  {}", error.message("en"));
        println!("🇩🇪 Deutsch:  {}", error.message("de"));
        println!("🇫🇷 Français: {}", error.message("fr"));
    }

    // Scenario 4: Special categories without Article 9 exception
    println!("\n\nScenario 4: Processing Special Categories (Health Data)");
    println!("{}", "=".repeat(60));
    println!("Note: This triggers a warning, not an error");
    let processing = DataProcessing::new()
        .with_controller("Healthcare App")
        .with_purpose("Symptom tracking")
        .add_data_category(PersonalDataCategory::Special(
            legalis_eu::gdpr::types::SpecialCategory::HealthData,
        ))
        .with_operation(ProcessingOperation::Collection)
        .with_lawful_basis(LawfulBasis::Consent {
            freely_given: true,
            specific: true,
            informed: true,
            unambiguous: true,
        });

    match processing.validate() {
        Ok(validation) => {
            if validation.requires_article9_exception {
                let error = GdprError::SpecialCategoryWithoutException;
                println!("⚠️  Warning:");
                println!("🇬🇧 English:  {}", error.message("en"));
                println!("🇩🇪 Deutsch:  {}", error.message("de"));
                println!("🇫🇷 Français: {}", error.message("fr"));
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Scenario 5: Breach notification late
    println!("\n\nScenario 5: Late Breach Notification");
    println!("{}", "=".repeat(60));
    let error = GdprError::BreachNotificationLate { hours: 96 };
    println!("🇬🇧 English:  {}", error.message("en"));
    println!("🇩🇪 Deutsch:  {}", error.message("de"));
    println!("🇫🇷 Français: {}", error.message("fr"));

    // Scenario 6: Child consent required
    println!("\n\nScenario 6: Child Data Subject");
    println!("{}", "=".repeat(60));
    let error = GdprError::ChildConsentRequired;
    println!("🇬🇧 English:  {}", error.message("en"));
    println!("🇩🇪 Deutsch:  {}", error.message("de"));
    println!("🇫🇷 Français: {}", error.message("fr"));

    // Scenario 7: Custom user preferences
    println!("\n\n=== User Preference Example ===\n");
    println!("Simulating user language preferences across EU:");
    println!("{}", "=".repeat(60));

    let user_languages = vec![
        ("en", "🇬🇧 British user"),
        ("de", "🇩🇪 German user"),
        ("fr", "🇫🇷 French user"),
        ("es", "🇪🇸 Spanish user"),
        ("it", "🇮🇹 Italian user"),
        ("pl", "🇵🇱 Polish user"),
        ("nl", "🇳🇱 Dutch user"),
        ("pt", "🇵🇹 Portuguese user"),
        ("sv", "🇸🇪 Swedish user"),
        ("cs", "🇨🇿 Czech user"),
        ("el", "🇬🇷 Greek user"),
        ("ja", "🇯🇵 Japanese user (fallback to EN)"),
    ];

    let error = GdprError::NoDataCategories;

    for (lang, desc) in user_languages {
        println!("{}: {}", desc, error.message(lang));
    }

    // Scenario 8: API Integration Example
    println!("\n\n=== API Integration Pattern ===\n");
    println!("Example JSON error response with i18n (11 languages):");
    println!("{}", "=".repeat(60));

    let error = GdprError::invalid_consent("Consent not specific enough");

    // Simulate API response with all languages
    let response = serde_json::json!({
        "error": {
            "code": "INVALID_CONSENT",
            "messages": {
                "en": error.message("en"),
                "de": error.message("de"),
                "fr": error.message("fr"),
                "es": error.message("es"),
                "it": error.message("it"),
                "pl": error.message("pl"),
                "nl": error.message("nl"),
                "pt": error.message("pt"),
                "sv": error.message("sv"),
                "cs": error.message("cs"),
                "el": error.message("el"),
            },
            "timestamp": "2026-01-10T12:00:00Z"
        }
    });

    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    println!("\n=== Summary ===\n");
    println!("✅ All error messages are available in 11 EU languages:");
    println!("   • English (EN)    - GDPR (Default)");
    println!("   • German (DE)     - DSGVO");
    println!("   • French (FR)     - RGPD");
    println!("   • Spanish (ES)    - RGPD");
    println!("   • Italian (IT)    - GDPR");
    println!("   • Polish (PL)     - RODO");
    println!("   • Dutch (NL)      - AVG");
    println!("   • Portuguese (PT) - RGPD");
    println!("   • Swedish (SV)    - GDPR");
    println!("   • Czech (CS)      - GDPR");
    println!("   • Greek (EL)      - GDPR");
    println!("\n📝 Usage:");
    println!("   error.message(\"en\") // English (default)");
    println!("   error.message(\"de\") // German");
    println!("   error.message(\"pl\") // Polish");
    println!("   error.message(\"ja\") // Unsupported → fallback to English");
    println!("\n💡 Perfect for:");
    println!("   • Multi-language web applications");
    println!("   • REST APIs serving EU clients");
    println!("   • Compliance dashboards");
    println!("   • User-facing error messages");
    println!("   • GDPR compliance tools");
    println!("\n🌍 Coverage:");
    println!("   These 11 languages cover ~420 million EU citizens");
    println!("   (Major markets: DE, FR, ES, IT, PL, NL represent ~80% of EU GDP)");
}
