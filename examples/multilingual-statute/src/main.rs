//! Multilingual Statute Display Example
//!
//! This example demonstrates how to use `legalis-i18n` for displaying
//! legal statutes in multiple languages with proper localization.
//!
//! ## Features
//!
//! - **Locale Support**: 20+ jurisdictions (JP, US, GB, DE, FR, etc.)
//! - **Legal Dictionaries**: Pre-built translations for legal terminology
//! - **Citation Formatting**: Bluebook, Japanese, OSCOLA, and more
//! - **Cultural Parameters**: Age of majority, protected classes by country

use legalis_i18n::{
    CitationComponents, CitationFormatter, CitationStyle, CulturalParams, Jurisdiction,
    LegalDictionary, LegalSystem, Locale, TranslationManager,
};

fn setup_translation_manager() -> TranslationManager {
    let mut manager = TranslationManager::new();

    // Add standard dictionaries
    manager.add_dictionary(LegalDictionary::english_us());
    manager.add_dictionary(LegalDictionary::japanese());
    manager.add_dictionary(LegalDictionary::german());
    manager.add_dictionary(LegalDictionary::french());
    manager.add_dictionary(LegalDictionary::spanish());
    manager.add_dictionary(LegalDictionary::chinese_simplified());

    manager
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "=".repeat(80));
    println!("   MULTILINGUAL STATUTE DISPLAY - Legalis-i18n Demo");
    println!("   多言語法令表示システム");
    println!("{}", "=".repeat(80));
    println!();

    // =========================================================================
    // Step 1: Locale Creation
    // =========================================================================
    println!("Step 1: Locale Creation\n");

    let locales = vec![
        Locale::new("ja").with_country("JP"),
        Locale::new("en").with_country("US"),
        Locale::new("de").with_country("DE"),
        Locale::new("fr").with_country("FR"),
        Locale::new("es").with_country("ES"),
        Locale::new("zh").with_country("CN"),
    ];

    for locale in &locales {
        println!("   {} - {}", locale.tag(), locale.language);
    }
    println!();

    // =========================================================================
    // Step 2: Legal Term Translation
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 2: Legal Term Translation\n");

    let manager = setup_translation_manager();

    let terms = vec![
        "statute",
        "contract",
        "obligation",
        "liability",
        "plaintiff",
        "defendant",
        "court",
        "judgment",
    ];

    println!("   | Term       | Japanese  | German        | French        | Spanish       |");
    println!("   |------------|-----------|---------------|---------------|---------------|");

    let ja_locale = Locale::new("ja").with_country("JP");
    let de_locale = Locale::new("de").with_country("DE");
    let fr_locale = Locale::new("fr").with_country("FR");
    let es_locale = Locale::new("es").with_country("ES");

    for term in &terms {
        let ja = manager
            .translate(term, &ja_locale)
            .unwrap_or_else(|_| term.to_string());
        let de = manager
            .translate(term, &de_locale)
            .unwrap_or_else(|_| term.to_string());
        let fr = manager
            .translate(term, &fr_locale)
            .unwrap_or_else(|_| term.to_string());
        let es = manager
            .translate(term, &es_locale)
            .unwrap_or_else(|_| term.to_string());
        println!(
            "   | {:10} | {:9} | {:13} | {:13} | {:13} |",
            term, ja, de, fr, es
        );
    }
    println!();

    // =========================================================================
    // Step 3: Citation Formatting
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 3: Citation Style Comparison\n");

    let citation = CitationComponents::new("Civil Code")
        .with_year(2020)
        .with_jurisdiction("Japan");

    let styles = vec![
        (CitationStyle::Japanese, "Japanese Style"),
        (CitationStyle::Bluebook, "Bluebook (US)"),
        (CitationStyle::OSCOLA, "OSCOLA (UK)"),
        (CitationStyle::European, "European Style"),
    ];

    for (style, name) in &styles {
        let formatter = CitationFormatter::new(style.clone(), ja_locale.clone());
        let formatted = formatter.format_statute(&citation);
        println!("   {}: {}", name, formatted);
    }
    println!();

    // Case citation example
    println!("   Case Citation Examples:\n");

    let case_citation = CitationComponents::new("Brown v. Board of Education")
        .with_volume("347")
        .with_reporter("U.S.")
        .with_page("483")
        .with_year(1954)
        .with_court("Supreme Court");

    let us_locale = Locale::new("en").with_country("US");
    let bluebook_formatter = CitationFormatter::new(CitationStyle::Bluebook, us_locale.clone());
    println!(
        "   Bluebook: {}",
        bluebook_formatter.format_case(&case_citation)
    );

    let oscola_formatter = CitationFormatter::new(CitationStyle::OSCOLA, us_locale);
    println!(
        "   OSCOLA:   {}",
        oscola_formatter.format_case(&case_citation)
    );
    println!();

    // =========================================================================
    // Step 4: Cultural Parameters by Country
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 4: Cultural Parameters by Country\n");

    let countries = vec![
        ("JP", "Japan"),
        ("US", "United States"),
        ("GB", "United Kingdom"),
        ("DE", "Germany"),
        ("FR", "France"),
    ];

    println!("   | Country        | Age of Majority | Protected Classes                  |");
    println!("   |----------------|-----------------|-------------------------------------|");

    for (code, name) in &countries {
        let params = CulturalParams::for_country(code);
        let age = params
            .age_of_majority
            .map(|a| a.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        let classes = if params.protected_classes.is_empty() {
            "N/A".to_string()
        } else {
            params
                .protected_classes
                .iter()
                .take(3)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
                + if params.protected_classes.len() > 3 {
                    "..."
                } else {
                    ""
                }
        };
        println!("   | {:14} | {:15} | {:35} |", name, age, classes);
    }
    println!();

    // =========================================================================
    // Step 5: Jurisdiction Setup
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("Step 5: Jurisdiction Configuration\n");

    let jp_jurisdiction = Jurisdiction::new("JP", "Japan", ja_locale.clone())
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::for_country("JP"));

    let us_jurisdiction =
        Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(CulturalParams::for_country("US"));

    let de_jurisdiction = Jurisdiction::new("DE", "Germany", de_locale.clone())
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::for_country("DE"));

    let jurisdictions = vec![jp_jurisdiction, us_jurisdiction, de_jurisdiction];

    println!("   Configured Jurisdictions:\n");
    for j in &jurisdictions {
        println!("   {} ({}):", j.name, j.id);
        println!("      Legal System: {:?}", j.legal_system);
        println!(
            "      Age of Majority: {}",
            j.cultural_params
                .age_of_majority
                .map(|a| a.to_string())
                .unwrap_or_else(|| "N/A".to_string())
        );
        println!("      Locale: {}", j.locale.tag());
        println!();
    }

    // =========================================================================
    // Summary
    // =========================================================================
    println!("{}", "=".repeat(80));
    println!("   INTERNATIONALIZATION SUMMARY");
    println!("{}", "=".repeat(80));
    println!();
    println!("   Supported Languages:");
    println!("   | Language   | Code | Dictionaries Available |");
    println!("   |------------|------|------------------------|");
    println!("   | English    | en   | US, GB, AU, CA, IN     |");
    println!("   | Japanese   | ja   | JP                     |");
    println!("   | German     | de   | DE, AT, CH             |");
    println!("   | French     | fr   | FR, CA, BE, CH         |");
    println!("   | Spanish    | es   | ES, MX, AR             |");
    println!("   | Chinese    | zh   | CN, TW                 |");
    println!();
    println!("   Citation Styles:");
    println!("   - Bluebook (US legal citation)");
    println!("   - OSCOLA (UK legal citation)");
    println!("   - Japanese (日本法令引用)");
    println!("   - AGLC (Australian)");
    println!("   - McGill (Canadian)");
    println!("   - European, Harvard, APA, Chicago");
    println!();
    println!("   Legal Systems:");
    println!("   - Civil Law (大陸法): Japan, Germany, France");
    println!("   - Common Law (英米法): US, UK, Australia");
    println!("   - Mixed Systems: South Africa, Louisiana");
    println!();
    println!("   Use Cases:");
    println!("   - Multilingual legal databases");
    println!("   - Cross-border legal research");
    println!("   - International law firm tools");
    println!("   - Comparative law studies");
    println!();

    Ok(())
}
