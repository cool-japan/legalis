#![cfg(test)]
use super::*;
#[test]
fn test_locale_parsing() {
    let locale = Locale::parse("ja-JP").unwrap();
    assert_eq!(locale.language, "ja");
    assert_eq!(locale.country, Some("JP".to_string()));
}
#[test]
fn test_locale_tag() {
    let locale = Locale::new("en").with_country("US");
    assert_eq!(locale.tag(), "en-US");
}
#[test]
fn test_translation_manager() {
    let mut manager = TranslationManager::new();
    let mut ja_dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
    ja_dict.add_translation("adult", "成人");
    ja_dict.add_translation("statute", "法律");
    manager.add_dictionary(ja_dict);
    let locale = Locale::new("ja").with_country("JP");
    assert_eq!(manager.translate("adult", &locale).unwrap(), "成人");
}
#[test]
fn test_jurisdiction_registry() {
    let registry = JurisdictionRegistry::with_defaults();
    let japan = registry.get("JP").unwrap();
    assert_eq!(japan.name, "Japan");
    assert_eq!(japan.legal_system, LegalSystem::CivilLaw);
}
#[test]
fn test_cultural_params() {
    let params = CulturalParams::japan();
    assert_eq!(params.age_of_majority, Some(18));
}
#[test]
fn test_plural_rules_english() {
    let rules = PluralRules::new(Locale::new("en"));
    assert_eq!(rules.category(1), PluralCategory::One);
    assert_eq!(rules.category(2), PluralCategory::Other);
    assert_eq!(rules.category(0), PluralCategory::Other);
}
#[test]
fn test_plural_rules_japanese() {
    let rules = PluralRules::new(Locale::new("ja"));
    assert_eq!(rules.category(1), PluralCategory::Other);
    assert_eq!(rules.category(2), PluralCategory::Other);
}
#[test]
fn test_plural_rules_russian() {
    let rules = PluralRules::new(Locale::new("ru"));
    assert_eq!(rules.category(1), PluralCategory::One);
    assert_eq!(rules.category(2), PluralCategory::Few);
    assert_eq!(rules.category(5), PluralCategory::Many);
}
#[test]
fn test_plural_rules_arabic() {
    let rules = PluralRules::new(Locale::new("ar"));
    assert_eq!(rules.category(0), PluralCategory::Zero);
    assert_eq!(rules.category(1), PluralCategory::One);
    assert_eq!(rules.category(2), PluralCategory::Two);
    assert_eq!(rules.category(5), PluralCategory::Few);
}
#[test]
fn test_message_formatter() {
    let formatter = MessageFormatter::new(Locale::new("en"));
    let mut args = HashMap::new();
    args.insert("name".to_string(), "John".to_string());
    args.insert("age".to_string(), "30".to_string());
    let result = formatter.format("Hello {name}, you are {age} years old", &args);
    assert_eq!(result, "Hello John, you are 30 years old");
}
#[test]
fn test_message_formatter_plural() {
    let formatter = MessageFormatter::new(Locale::new("en"));
    assert_eq!(formatter.format_plural(1, "1 item", "items"), "1 item");
    assert_eq!(formatter.format_plural(2, "1 item", "items"), "items");
}
#[test]
fn test_datetime_formatter_japanese() {
    let formatter = DateTimeFormatter::new(Locale::new("ja").with_country("JP"));
    assert_eq!(formatter.format_date(2024, 12, 14), "2024年12月14日");
    assert_eq!(formatter.format_time(15, 30), "15:30");
}
#[test]
fn test_datetime_formatter_us() {
    let formatter = DateTimeFormatter::new(Locale::new("en").with_country("US"));
    assert_eq!(formatter.format_date(2024, 12, 14), "12/14/2024");
    assert_eq!(formatter.format_time(15, 30), "03:30 PM");
    assert_eq!(formatter.format_time(9, 15), "09:15 AM");
}
#[test]
fn test_datetime_formatter_german() {
    let formatter = DateTimeFormatter::new(Locale::new("de").with_country("DE"));
    assert_eq!(formatter.format_date(2024, 12, 14), "14.12.2024");
    assert_eq!(formatter.format_time(15, 30), "15:30");
}
#[test]
fn test_currency_formatter_usd() {
    let formatter = CurrencyFormatter::new(Locale::new("en").with_country("US"));
    assert_eq!(formatter.format(1000.50, "USD"), "$1000.50");
    assert_eq!(formatter.format(100.0, "USD"), "$100");
}
#[test]
fn test_currency_formatter_eur() {
    let formatter = CurrencyFormatter::new(Locale::new("de").with_country("DE"));
    assert_eq!(formatter.format(1000.50, "EUR"), "1000,50 €");
}
#[test]
fn test_currency_formatter_jpy() {
    let formatter = CurrencyFormatter::new(Locale::new("ja").with_country("JP"));
    assert_eq!(formatter.format(1000.0, "JPY"), "¥1000");
}
#[test]
fn test_number_formatter_english() {
    let formatter = NumberFormatter::new(Locale::new("en"));
    assert_eq!(formatter.format_integer(1000), "1,000");
    assert_eq!(formatter.format_integer(1000000), "1,000,000");
    assert_eq!(formatter.format_percentage(50.5), "50.5%");
}
#[test]
fn test_number_formatter_german() {
    let formatter = NumberFormatter::new(Locale::new("de"));
    assert_eq!(formatter.format_integer(1000), "1.000");
    assert_eq!(formatter.format_integer(1000000), "1.000.000");
    assert_eq!(formatter.format_percentage(50.5), "50,5 %");
}
#[test]
fn test_number_formatter_french() {
    let formatter = NumberFormatter::new(Locale::new("fr"));
    assert_eq!(formatter.format_integer(1000), "1 000");
    assert_eq!(formatter.format_percentage(50.5), "50,5 %");
}
#[test]
fn test_number_formatter_japanese() {
    let formatter = NumberFormatter::new(Locale::new("ja"));
    assert_eq!(formatter.format_integer(1000), "1000");
    assert_eq!(formatter.format_integer(1000000), "1000000");
}
#[test]
fn test_legal_dictionary_japanese() {
    let dict = LegalDictionary::japanese();
    assert_eq!(dict.translate("statute"), Some("法律"));
    assert_eq!(dict.translate("contract"), Some("契約"));
    assert_eq!(dict.translate("court"), Some("裁判所"));
}
#[test]
fn test_legal_dictionary_german() {
    let dict = LegalDictionary::german();
    assert_eq!(dict.translate("statute"), Some("Gesetz"));
    assert_eq!(dict.translate("contract"), Some("Vertrag"));
    assert_eq!(dict.translate("court"), Some("Gericht"));
}
#[test]
fn test_legal_dictionary_french() {
    let dict = LegalDictionary::french();
    assert_eq!(dict.translate("statute"), Some("loi"));
    assert_eq!(dict.translate("contract"), Some("contrat"));
    assert_eq!(dict.translate("court"), Some("tribunal"));
}
#[test]
fn test_legal_dictionary_spanish() {
    let dict = LegalDictionary::spanish();
    assert_eq!(dict.translate("statute"), Some("estatuto"));
    assert_eq!(dict.translate("contract"), Some("contrato"));
    assert_eq!(dict.translate("court"), Some("tribunal"));
}
#[test]
fn test_legal_dictionary_chinese() {
    let dict = LegalDictionary::chinese_simplified();
    assert_eq!(dict.translate("statute"), Some("法规"));
    assert_eq!(dict.translate("contract"), Some("合同"));
    assert_eq!(dict.translate("court"), Some("法院"));
}
#[test]
fn test_translation_manager_with_dictionaries() {
    let mut manager = TranslationManager::new();
    manager.add_dictionary(LegalDictionary::japanese());
    manager.add_dictionary(LegalDictionary::german());
    let ja_locale = Locale::new("ja").with_country("JP");
    let de_locale = Locale::new("de").with_country("DE");
    assert_eq!(manager.translate("statute", &ja_locale).unwrap(), "法律");
    assert_eq!(manager.translate("statute", &de_locale).unwrap(), "Gesetz");
}
#[test]
fn test_latin_dictionary() {
    let dict = LegalDictionary::latin();
    assert_eq!(dict.translate("guilty_mind"), Some("mens rea"));
    assert_eq!(dict.translate("guilty_act"), Some("actus reus"));
    assert_eq!(dict.translate("good_faith"), Some("bona fide"));
    assert_eq!(dict.translate("in_fact"), Some("de facto"));
    assert!(dict.define("mens rea").is_some());
}
#[test]
fn test_legal_concept_mapping() {
    let registry = LegalConceptRegistry::with_defaults();
    let mapping = registry
        .find_mapping(LegalSystem::CommonLaw, "tort")
        .unwrap();
    assert_eq!(mapping.concept, "tort");
    let civil_equivalents = mapping.get_equivalents(LegalSystem::CivilLaw).unwrap();
    assert!(civil_equivalents.contains(&"delict".to_string()));
}
#[test]
fn test_legal_concept_system_mappings() {
    let registry = LegalConceptRegistry::with_defaults();
    let mappings = registry.get_system_mappings(LegalSystem::CommonLaw, LegalSystem::CivilLaw);
    assert!(!mappings.is_empty());
    let tort_mapping = mappings.iter().find(|(concept, _)| *concept == "tort");
    assert!(tort_mapping.is_some());
}
#[test]
fn test_calendar_converter_japanese() {
    let converter = CalendarConverter::new(Locale::new("ja").with_country("JP"));
    let date = converter.from_gregorian(2024, 12, 14);
    assert_eq!(date.system, CalendarSystem::Japanese);
    assert_eq!(date.year, 6);
    assert_eq!(date.era, Some("Reiwa".to_string()));
    let date = converter.from_gregorian(2018, 5, 1);
    assert_eq!(date.system, CalendarSystem::Japanese);
    assert_eq!(date.year, 30);
    assert_eq!(date.era, Some("Heisei".to_string()));
}
#[test]
fn test_calendar_converter_buddhist() {
    let converter = CalendarConverter::new(Locale::new("th").with_country("TH"));
    let date = converter.from_gregorian(2024, 12, 14);
    assert_eq!(date.system, CalendarSystem::Buddhist);
    assert_eq!(date.year, 2567);
}
#[test]
fn test_calendar_date_formatting() {
    let converter = CalendarConverter::new(Locale::new("ja").with_country("JP"));
    let date = CalendarDate::new(CalendarSystem::Japanese, 6, 12, 14).with_era("Reiwa");
    let formatted = converter.format_date(&date);
    assert_eq!(formatted, "Reiwa6年12月14日");
}
#[test]
fn test_working_days_japan() {
    let config = WorkingDaysConfig::japan();
    assert!(!config.is_working_day(2024, 12, 14));
    assert!(!config.is_working_day(2024, 12, 15));
    assert!(config.is_working_day(2024, 12, 16));
    assert!(!config.is_working_day(2024, 1, 1));
}
#[test]
fn test_working_days_saudi_arabia() {
    let config = WorkingDaysConfig::saudi_arabia();
    assert!(!config.weekend.contains(&DayOfWeek::Sunday));
    assert!(config.weekend.contains(&DayOfWeek::Friday));
    assert!(config.weekend.contains(&DayOfWeek::Saturday));
}
#[test]
fn test_add_working_days() {
    let config = WorkingDaysConfig::new("TEST");
    let (year, month, day) = config.add_working_days(2024, 12, 13, 3);
    assert_eq!(year, 2024);
    assert_eq!(month, 12);
    assert_eq!(day, 18);
}
#[test]
fn test_day_of_week_calculation() {
    let config = WorkingDaysConfig::new("TEST");
    let day = config.calculate_day_of_week(2024, 12, 14);
    assert_eq!(day, DayOfWeek::Saturday);
    let day = config.calculate_day_of_week(2024, 12, 16);
    assert_eq!(day, DayOfWeek::Monday);
}
#[test]
fn test_translation_roundtrip_japanese() {
    let dict_ja = LegalDictionary::japanese();
    let dict_en = LegalDictionary::english_us();
    assert_eq!(dict_ja.translate("statute"), Some("法律"));
    assert_eq!(dict_en.translate("statute"), Some("statute"));
}
#[test]
fn test_all_locale_dictionaries() {
    let _en = LegalDictionary::english_us();
    let _ja = LegalDictionary::japanese();
    let _de = LegalDictionary::german();
    let _fr = LegalDictionary::french();
    let _es = LegalDictionary::spanish();
    let _zh = LegalDictionary::chinese_simplified();
    let _la = LegalDictionary::latin();
}
#[test]
fn test_jurisdiction_cultural_params() {
    let registry = JurisdictionRegistry::with_defaults();
    let japan = registry.get("JP").unwrap();
    assert_eq!(japan.cultural_params.age_of_majority, Some(18));
    let saudi = registry.get("SA").unwrap();
    assert_eq!(saudi.legal_system, LegalSystem::ReligiousLaw);
    assert!(
        saudi
            .cultural_params
            .religious_considerations
            .contains(&"islam".to_string())
    );
}
#[test]
fn test_locale_variations() {
    let us = Locale::new("en").with_country("US");
    let gb = Locale::new("en").with_country("GB");
    assert_eq!(us.tag(), "en-US");
    assert_eq!(gb.tag(), "en-GB");
    assert_eq!(us.language, gb.language);
    assert_ne!(us.country, gb.country);
}
#[test]
fn test_jurisdiction_glossaries() {
    let jp_glossary = LegalDictionary::glossary_japan();
    assert_eq!(jp_glossary.translate("civil_code"), Some("民法"));
    assert_eq!(jp_glossary.translate("family_register"), Some("戸籍"));
    assert_eq!(jp_glossary.translate("kabushiki_kaisha"), Some("株式会社"));
    let us_glossary = LegalDictionary::glossary_united_states();
    assert_eq!(us_glossary.translate("due_process"), Some("due process"));
    assert_eq!(
        us_glossary.translate("supreme_court"),
        Some("Supreme Court")
    );
    assert_eq!(us_glossary.translate("class_action"), Some("class action"));
    let uk_glossary = LegalDictionary::glossary_united_kingdom();
    assert_eq!(uk_glossary.translate("barrister"), Some("barrister"));
    assert_eq!(uk_glossary.translate("freehold"), Some("freehold"));
    assert_eq!(uk_glossary.translate("trust"), Some("trust"));
    let de_glossary = LegalDictionary::glossary_germany();
    assert_eq!(de_glossary.translate("bgb"), Some("BGB"));
    assert_eq!(
        de_glossary.translate("bundesgerichtshof"),
        Some("Bundesgerichtshof")
    );
    let fr_glossary = LegalDictionary::glossary_france();
    assert_eq!(fr_glossary.translate("code_civil"), Some("Code civil"));
    assert_eq!(
        fr_glossary.translate("cour_de_cassation"),
        Some("Cour de cassation")
    );
    let cn_glossary = LegalDictionary::glossary_china();
    assert_eq!(cn_glossary.translate("civil_law"), Some("民法"));
    assert_eq!(cn_glossary.translate("peoples_court"), Some("人民法院"));
}
#[test]
fn test_glossary_for_jurisdiction() {
    let jp_glossary = LegalDictionary::glossary_for_jurisdiction("JP");
    assert_eq!(jp_glossary.locale.country, Some("JP".to_string()));
    let us_glossary = LegalDictionary::glossary_for_jurisdiction("US");
    assert_eq!(us_glossary.locale.country, Some("US".to_string()));
}
#[test]
fn test_locale_matches() {
    let en = Locale::new("en");
    let en_us = Locale::new("en").with_country("US");
    let en_gb = Locale::new("en").with_country("GB");
    let ja = Locale::new("ja");
    assert!(en.matches(&en_us));
    assert!(en_us.matches(&en));
    assert!(!en_us.matches(&en_gb));
    assert!(!en.matches(&ja));
}
#[test]
fn test_locale_parent() {
    let zh_hans_cn = Locale::new("zh").with_script("Hans").with_country("CN");
    let parent1 = zh_hans_cn.parent().unwrap();
    assert_eq!(parent1.language, "zh");
    assert_eq!(parent1.script, Some("Hans".to_string()));
    assert_eq!(parent1.country, None);
    let parent2 = parent1.parent().unwrap();
    assert_eq!(parent2.language, "zh");
    assert_eq!(parent2.script, None);
    assert_eq!(parent2.country, None);
    assert!(parent2.parent().is_none());
}
#[test]
fn test_locale_fallback_chain() {
    let zh_hans_cn = Locale::new("zh").with_script("Hans").with_country("CN");
    let chain = zh_hans_cn.fallback_chain();
    assert_eq!(chain.len(), 3);
    assert_eq!(chain[0].tag(), "zh-Hans-CN");
    assert_eq!(chain[1].tag(), "zh-Hans");
    assert_eq!(chain[2].tag(), "zh");
}
#[test]
fn test_regional_variation_registry() {
    let registry = RegionalVariationRegistry::with_defaults();
    let en_variations = registry.get_variations(&Locale::new("en"));
    assert!(en_variations.len() >= 4);
    let us_locale = Locale::new("en").with_country("US");
    let us_variation = registry.find_variation(&us_locale);
    assert!(us_variation.is_some());
    assert_eq!(us_variation.unwrap().description, "American English");
}
#[test]
fn test_regional_variation_differences() {
    let registry = RegionalVariationRegistry::with_defaults();
    let us_locale = Locale::new("en").with_country("US");
    let us_variation = registry.find_variation(&us_locale).unwrap();
    assert!(!us_variation.differences.is_empty());
    assert!(
        us_variation
            .differences
            .iter()
            .any(|d| d.contains("attorney"))
    );
}
#[test]
fn test_chinese_script_variations() {
    let registry = RegionalVariationRegistry::with_defaults();
    let cn_locale = Locale::new("zh").with_country("CN").with_script("Hans");
    let cn_variation = registry.find_variation(&cn_locale);
    assert!(cn_variation.is_some());
    assert!(cn_variation.unwrap().description.contains("Simplified"));
    let tw_locale = Locale::new("zh").with_country("TW").with_script("Hant");
    let tw_variation = registry.find_variation(&tw_locale);
    assert!(tw_variation.is_some());
    assert!(tw_variation.unwrap().description.contains("Traditional"));
}
#[test]
fn test_spanish_regional_variations() {
    let registry = RegionalVariationRegistry::with_defaults();
    let es_variations = registry.get_variations(&Locale::new("es"));
    assert!(es_variations.len() >= 3);
    let mx_locale = Locale::new("es").with_country("MX");
    let mx_variation = registry.find_variation(&mx_locale);
    assert!(mx_variation.is_some());
    assert!(
        mx_variation
            .unwrap()
            .differences
            .iter()
            .any(|d| d.contains("ustedes"))
    );
}
#[test]
fn test_german_regional_variations() {
    let registry = RegionalVariationRegistry::with_defaults();
    let de_variations = registry.get_variations(&Locale::new("de"));
    assert!(de_variations.len() >= 3);
    let ch_locale = Locale::new("de").with_country("CH");
    let ch_variation = registry.find_variation(&ch_locale);
    assert!(ch_variation.is_some());
    assert!(
        ch_variation
            .unwrap()
            .differences
            .iter()
            .any(|d| d.contains("Swiss"))
    );
}
#[test]
fn test_french_regional_variations() {
    let registry = RegionalVariationRegistry::with_defaults();
    let fr_variations = registry.get_variations(&Locale::new("fr"));
    assert!(fr_variations.len() >= 3);
    let ca_locale = Locale::new("fr").with_country("CA");
    let ca_variation = registry.find_variation(&ca_locale);
    assert!(ca_variation.is_some());
    assert!(
        ca_variation
            .unwrap()
            .differences
            .iter()
            .any(|d| d.contains("Quebec") || d.contains("Bilingual"))
    );
}
#[test]
fn test_mock_translation_service() {
    let service = MockTranslationService::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let result = service.translate("contract", &en, &ja).unwrap();
    assert_eq!(result, "[ja] contract");
    assert_eq!(service.service_name(), "MockTranslationService");
    assert!(service.is_available());
}
#[test]
fn test_mock_translation_service_unavailable() {
    let mut service = MockTranslationService::new();
    service.set_available(false);
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let result = service.translate("contract", &en, &ja);
    assert!(result.is_err());
    assert!(!service.is_available());
}
#[test]
fn test_translation_memory_exact_match() {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    memory.add_translation("contract", en.clone(), "契約", ja.clone());
    let matches = memory.find_exact("contract", &en, &ja);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].target_text, "契約");
}
#[test]
fn test_translation_memory_no_match() {
    let memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let matches = memory.find_exact("contract", &en, &ja);
    assert_eq!(matches.len(), 0);
}
#[test]
fn test_translation_memory_fuzzy_match() {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    memory.add_translation("employment contract", en.clone(), "雇用契約", ja.clone());
    let matches = memory.find_fuzzy("employment contract agreement", &en, &ja, 0.5);
    assert!(!matches.is_empty());
    assert!(matches[0].1 > 0.5);
}
#[test]
fn test_translation_memory_entry() {
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let entry = TranslationMemoryEntry::new("contract", en, "契約", ja)
        .with_quality(0.95)
        .with_metadata("translator", "human");
    assert_eq!(entry.source_text, "contract");
    assert_eq!(entry.target_text, "契約");
    assert_eq!(entry.quality_score, 0.95);
    assert_eq!(entry.metadata.get("translator").unwrap(), "human");
}
#[test]
fn test_machine_translation_fallback_memory_hit() {
    let mut fallback = MachineTranslationFallback::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    fallback
        .memory_mut()
        .add_translation("contract", en.clone(), "契約", ja.clone());
    let result = fallback.translate("contract", &en, &ja).unwrap();
    assert_eq!(result, "契約");
}
#[test]
fn test_machine_translation_fallback_service() {
    let mut fallback = MachineTranslationFallback::new();
    fallback.add_service(Box::new(MockTranslationService::new()));
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let result = fallback.translate("contract", &en, &ja).unwrap();
    assert_eq!(result, "[ja] contract");
    assert_eq!(fallback.memory().len(), 1);
}
#[test]
fn test_machine_translation_fallback_no_service() {
    let mut fallback = MachineTranslationFallback::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let result = fallback.translate("contract", &en, &ja);
    assert!(result.is_err());
}
#[test]
fn test_terminology_extractor() {
    let mut extractor = TerminologyExtractor::new();
    extractor.add_known_term("contract");
    extractor.add_known_term("employment");
    extractor.add_known_term("statute");
    let text = "This contract governs employment. The statute requires a written contract.";
    extractor.extract_from_text(text);
    assert_eq!(extractor.get_frequency("contract"), 2);
    assert_eq!(extractor.get_frequency("employment"), 1);
    assert_eq!(extractor.get_frequency("statute"), 1);
    let terms = extractor.get_terms_by_frequency();
    assert_eq!(terms[0].0, "contract");
    assert_eq!(terms[0].1, 2);
}
#[test]
fn test_terminology_extractor_with_dictionary() {
    let mut dict = LegalDictionary::new(Locale::new("en"));
    dict.add_translation("contract", "contract");
    dict.add_translation("statute", "statute");
    let mut extractor = TerminologyExtractor::with_dictionary(&dict);
    let text = "The contract requires compliance with the statute.";
    extractor.extract_from_text(text);
    assert_eq!(extractor.get_frequency("contract"), 1);
    assert_eq!(extractor.get_frequency("statute"), 1);
}
#[test]
fn test_terminology_extractor_clear() {
    let mut extractor = TerminologyExtractor::new();
    extractor.add_known_term("contract");
    let text = "This is a contract.";
    extractor.extract_from_text(text);
    assert_eq!(extractor.get_frequency("contract"), 1);
    extractor.clear();
    assert_eq!(extractor.get_frequency("contract"), 0);
    assert!(extractor.extracted_terms().is_empty());
}
#[test]
fn test_translation_memory_levenshtein_similarity() {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    memory.add_translation("contract", en.clone(), "契約", ja.clone());
    memory.add_translation("contractor", en.clone(), "請負業者", ja.clone());
    let matches = memory.find_fuzzy_levenshtein("contracts", &en, &ja, 0.7);
    assert!(!matches.is_empty());
    assert!(matches[0].1 >= 0.7);
}
#[test]
fn test_translation_memory_context_aware() {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let mut entry1 = TranslationMemoryEntry::new("right", en.clone(), "権利", ja.clone());
    entry1
        .metadata
        .insert("context".to_string(), "contract_law".to_string());
    memory.add_entry(entry1);
    let mut entry2 = TranslationMemoryEntry::new("right", en.clone(), "右", ja.clone());
    entry2
        .metadata
        .insert("context".to_string(), "directions".to_string());
    memory.add_entry(entry2);
    let contract_matches = memory.find_with_context("right", &en, &ja, Some("contract_law"), 0.9);
    assert_eq!(contract_matches.len(), 1);
    assert_eq!(contract_matches[0].0.target_text, "権利");
    let direction_matches = memory.find_with_context("right", &en, &ja, Some("directions"), 0.9);
    assert_eq!(direction_matches.len(), 1);
    assert_eq!(direction_matches[0].0.target_text, "右");
}
#[test]
fn test_translation_memory_save_load() {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    memory.add_translation("contract", en.clone(), "契約", ja.clone());
    memory.add_translation("statute", en.clone(), "法令", ja.clone());
    let temp_dir = tempfile::TempDir::new().expect("tempdir");
    let temp_path = temp_dir.path().join("test_translation_memory.json");
    memory.save_to_file(&temp_path).unwrap();
    let mut loaded_memory = TranslationMemory::new();
    loaded_memory.load_from_file(&temp_path).unwrap();
    assert_eq!(loaded_memory.len(), 2);
    let matches = loaded_memory.find_exact("contract", &en, &ja);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].target_text, "契約");
}
#[test]
fn test_translation_memory_tmx_export_import() {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    memory.add_translation("contract", en.clone(), "契約", ja.clone());
    memory.add_translation("employment", en.clone(), "雇用", ja.clone());
    let temp_dir = tempfile::TempDir::new().expect("tempdir");
    let temp_path = temp_dir.path().join("test_translation_memory.tmx");
    memory.export_to_tmx(&temp_path).unwrap();
    let mut imported_memory = TranslationMemory::new();
    imported_memory.import_from_tmx(&temp_path).unwrap();
    assert_eq!(imported_memory.len(), 2);
    let matches = imported_memory.find_exact("contract", &en, &ja);
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].target_text, "契約");
}
#[test]
fn test_translation_memory_merge() {
    let mut memory1 = TranslationMemory::new();
    let mut memory2 = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    memory1.add_translation("contract", en.clone(), "契約", ja.clone());
    memory2.add_translation("statute", en.clone(), "法令", ja.clone());
    memory1.merge(&memory2);
    assert_eq!(memory1.len(), 2);
    assert!(memory1.find_exact("contract", &en, &ja).len() == 1);
    assert!(memory1.find_exact("statute", &en, &ja).len() == 1);
}
#[test]
fn test_translation_memory_xml_escape() {
    let mut memory = TranslationMemory::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    memory.add_translation("A & B < C > \"D\"", en.clone(), "A と B", ja.clone());
    let temp_dir = tempfile::TempDir::new().expect("tempdir");
    let temp_path = temp_dir.path().join("test_xml_escape.tmx");
    memory.export_to_tmx(&temp_path).unwrap();
    let tmx_content = std::fs::read_to_string(&temp_path).unwrap();
    assert!(tmx_content.contains("&amp;"));
    assert!(tmx_content.contains("&lt;"));
    assert!(tmx_content.contains("&gt;"));
    assert!(tmx_content.contains("&quot;"));
}
#[test]
fn test_translation_service_batch() {
    let service = MockTranslationService::new();
    let en = Locale::new("en");
    let ja = Locale::new("ja");
    let texts = vec!["contract", "statute", "employment"];
    let results = service.translate_batch(&texts, &en, &ja).unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "[ja] contract");
    assert_eq!(results[1], "[ja] statute");
    assert_eq!(results[2], "[ja] employment");
}
#[test]
fn test_screen_reader_aria_label() {
    let formatter = ScreenReaderFormatter::new(Locale::new("en"));
    assert_eq!(
        formatter.aria_label("article", "Contract Formation"),
        "Article: Contract Formation"
    );
    assert_eq!(
        formatter.aria_label("section", "Definitions"),
        "Section: Definitions"
    );
}
#[test]
fn test_screen_reader_citation_formatting() {
    let formatter = ScreenReaderFormatter::new(Locale::new("en"));
    let citation = "Brown v. Board of Education, 347 U.S. 483 (1954)";
    let formatted = formatter.format_citation(citation);
    assert!(formatted.contains("versus"));
    assert!(formatted.contains("United States"));
    assert!(!formatted.contains("v."));
    assert!(!formatted.contains("U.S."));
}
#[test]
fn test_screen_reader_navigation() {
    let formatter = ScreenReaderFormatter::new(Locale::new("en"));
    let sections = vec![
        ("article", "Introduction"),
        ("section", "Definitions"),
        ("chapter", "Enforcement"),
    ];
    let nav = formatter.navigation_structure(&sections);
    assert!(nav.contains("<nav"));
    assert!(nav.contains("aria-label"));
    assert!(nav.contains("Introduction"));
    assert!(nav.contains("Definitions"));
    assert!(nav.contains("Enforcement"));
}
#[test]
fn test_screen_reader_table_formatting() {
    let formatter = ScreenReaderFormatter::new(Locale::new("en"));
    let headers = vec!["Name", "Role", "Jurisdiction"];
    let rows = vec![
        vec!["John Doe", "Judge", "Federal"],
        vec!["Jane Smith", "Attorney", "State"],
    ];
    let table = formatter.format_table("Legal Personnel", &headers, &rows);
    assert!(table.contains("<table"));
    assert!(table.contains("aria-label"));
    assert!(table.contains("<caption>Legal Personnel</caption>"));
    assert!(table.contains("scope=\"col\""));
    assert!(table.contains("scope=\"row\""));
}
#[test]
fn test_plain_language_converter() {
    let converter = PlainLanguageConverter::new(Locale::new("en"));
    let legal_text = "The plaintiff hereby files this complaint pursuant to federal law.";
    let plain = converter.convert(legal_text);
    assert!(plain.contains("person who filed the lawsuit") || plain.contains("plaintiff"));
    assert!(plain.contains("by this document") || plain.contains("hereby"));
}
#[test]
fn test_plain_language_custom_conversion() {
    let mut converter = PlainLanguageConverter::new(Locale::new("en"));
    converter.add_conversion("escheat", "revert to the state");
    assert_eq!(
        converter.get_plain_alternative("escheat"),
        Some(&"revert to the state".to_string())
    );
}
#[test]
fn test_reading_level_flesch_reading_ease() {
    let assessor = ReadingLevelAssessor::new();
    let simple_text = "The cat sat on the mat. It was a nice day.";
    let ease = assessor.flesch_reading_ease(simple_text);
    assert!(ease > 60.0);
    let complex_text = "Notwithstanding the aforementioned jurisdictional complications, the defendant's constitutional rights remain inviolate pursuant to established jurisprudence.";
    let ease_complex = assessor.flesch_reading_ease(complex_text);
    assert!(ease_complex < ease);
}
#[test]
fn test_reading_level_flesch_kincaid_grade() {
    let assessor = ReadingLevelAssessor::new();
    let text = "The law requires clear documentation. All parties must sign the agreement.";
    let grade = assessor.flesch_kincaid_grade(text);
    assert!(grade >= 0.0);
    assert!(grade < 20.0);
}
#[test]
fn test_reading_level_assessment_report() {
    let assessor = ReadingLevelAssessor::new();
    let text = "Contract law governs agreements. Each party has rights and duties.";
    let report = assessor.assess(text);
    assert!(report.word_count > 0);
    assert!(report.sentence_count > 0);
    assert!(report.syllable_count > 0);
    assert!(!report.difficulty.is_empty());
    assert!(report.flesch_reading_ease >= 0.0 && report.flesch_reading_ease <= 206.835);
}
#[test]
fn test_braille_formatter_basic() {
    let formatter = BrailleFormatter::new(BrailleGrade::Grade1);
    let braille = formatter.to_braille("law");
    assert!(!braille.is_empty());
    assert!(
        braille
            .chars()
            .all(|c| ('\u{2800}'..='\u{28FF}').contains(&c) || c == ' ')
    );
}
#[test]
fn test_braille_section_number() {
    let formatter = BrailleFormatter::new(BrailleGrade::Grade1);
    let section = formatter.format_section_number("abc");
    assert!(section.starts_with('§'));
    assert!(section.contains(char::from_u32(0x2801).unwrap()));
}
#[test]
fn test_audio_description_flowchart() {
    let generator = AudioDescriptionGenerator::new(Locale::new("en"));
    let elements = vec!["File complaint", "Serve defendant", "Discovery", "Trial"];
    let description = generator.describe_diagram("flowchart", &elements);
    assert!(description.contains("Flowchart"));
    assert!(description.contains("4 steps"));
    assert!(description.contains("File complaint"));
    assert!(description.contains("then"));
}
#[test]
fn test_audio_description_chart() {
    let generator = AudioDescriptionGenerator::new(Locale::new("en"));
    let data = vec![
        ("Criminal".to_string(), 45.0),
        ("Civil".to_string(), 35.0),
        ("Family".to_string(), 20.0),
    ];
    let bar_chart = generator.describe_chart("bar", &data);
    assert!(bar_chart.contains("Bar chart"));
    assert!(bar_chart.contains("3 data points"));
    let pie_chart = generator.describe_chart("pie", &data);
    assert!(pie_chart.contains("Pie chart"));
    assert!(pie_chart.contains("%"));
}
#[test]
fn test_audio_description_table() {
    let generator = AudioDescriptionGenerator::new(Locale::new("en"));
    let description = generator.describe_table("Case Statistics", 10, 5);
    assert!(description.contains("Table"));
    assert!(description.contains("Case Statistics"));
    assert!(description.contains("10 rows"));
    assert!(description.contains("5 columns"));
}
#[test]
fn test_abbreviations() {
    let mut dict = LegalDictionary::new(Locale::new("en").with_country("US"));
    dict.add_abbreviation("corporation", "Corp.");
    dict.add_abbreviation("incorporated", "Inc.");
    dict.add_abbreviation("attorney", "Atty.");
    assert_eq!(dict.get_abbreviation("corporation"), Some("Corp."));
    assert_eq!(dict.get_abbreviation("incorporated"), Some("Inc."));
    assert_eq!(dict.get_abbreviation("attorney"), Some("Atty."));
    assert_eq!(dict.expand_abbreviation("Corp."), Some("corporation"));
    assert_eq!(dict.expand_abbreviation("Inc."), Some("incorporated"));
    assert!(dict.is_abbreviation("Corp."));
    assert!(!dict.is_abbreviation("corporation"));
}
#[test]
fn test_contextual_translation() {
    let mut dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
    dict.add_translation("right", "権利");
    dict.add_contextual_translation("right", "direction", "右");
    dict.add_contextual_translation("right", "legal", "権利");
    assert_eq!(dict.translate("right"), Some("権利"));
    assert_eq!(
        dict.translate_with_context("right", "direction"),
        Some("右")
    );
    assert_eq!(dict.translate_with_context("right", "legal"), Some("権利"));
    assert_eq!(dict.translate_with_context("right", "other"), Some("権利"));
    let contexts = dict.get_contexts_for_term("right");
    assert_eq!(contexts.len(), 2);
    assert!(contexts.contains(&"direction"));
    assert!(contexts.contains(&"legal"));
}
#[test]
fn test_validation_helpers() {
    assert!(is_valid_language_code("en"));
    assert!(is_valid_language_code("ja"));
    assert!(!is_valid_language_code("EN"));
    assert!(!is_valid_language_code("eng"));
    assert!(is_valid_country_code("US"));
    assert!(is_valid_country_code("JP"));
    assert!(!is_valid_country_code("us"));
    assert!(!is_valid_country_code("USA"));
    assert!(is_valid_script_code("Hans"));
    assert!(is_valid_script_code("Hant"));
    assert!(!is_valid_script_code("hans"));
    assert!(!is_valid_script_code("HANS"));
    assert!(is_valid_locale_tag("en-US"));
    assert!(is_valid_locale_tag("zh-Hans-CN"));
    assert!(!is_valid_locale_tag("EN-US"));
}
#[test]
fn test_ordinal_formatting() {
    let en_formatter = NumberFormatter::new(Locale::new("en").with_country("US"));
    assert_eq!(en_formatter.format_ordinal(1), "1st");
    assert_eq!(en_formatter.format_ordinal(2), "2nd");
    assert_eq!(en_formatter.format_ordinal(3), "3rd");
    assert_eq!(en_formatter.format_ordinal(4), "4th");
    assert_eq!(en_formatter.format_ordinal(11), "11th");
    assert_eq!(en_formatter.format_ordinal(21), "21st");
    assert_eq!(en_formatter.format_ordinal(42), "42nd");
    assert_eq!(en_formatter.format_ordinal(113), "113th");
    let fr_formatter = NumberFormatter::new(Locale::new("fr").with_country("FR"));
    assert_eq!(fr_formatter.format_ordinal(1), "1er");
    assert_eq!(fr_formatter.format_ordinal(2), "2e");
    let ja_formatter = NumberFormatter::new(Locale::new("ja").with_country("JP"));
    assert_eq!(ja_formatter.format_ordinal(1), "第1");
    assert_eq!(ja_formatter.format_ordinal(5), "第5");
}
#[test]
fn test_number_to_words() {
    let en_formatter = NumberFormatter::new(Locale::new("en").with_country("US"));
    assert_eq!(en_formatter.number_to_words(0), "zero");
    assert_eq!(en_formatter.number_to_words(1), "one");
    assert_eq!(en_formatter.number_to_words(15), "fifteen");
    assert_eq!(en_formatter.number_to_words(20), "twenty");
    assert_eq!(en_formatter.number_to_words(42), "forty-two");
    assert_eq!(en_formatter.number_to_words(100), "one hundred");
    assert_eq!(en_formatter.number_to_words(101), "one hundred and one");
    assert_eq!(en_formatter.number_to_words(1000), "one thousand");
    assert_eq!(
        en_formatter.number_to_words(1234),
        "one thousand two hundred and thirty-four"
    );
    let ja_formatter = NumberFormatter::new(Locale::new("ja").with_country("JP"));
    assert_eq!(ja_formatter.number_to_words(0), "零");
    assert_eq!(ja_formatter.number_to_words(1), "一");
    assert_eq!(ja_formatter.number_to_words(10), "十");
    assert_eq!(ja_formatter.number_to_words(11), "十一");
    assert_eq!(ja_formatter.number_to_words(100), "百");
    assert_eq!(ja_formatter.number_to_words(123), "百二十三");
}
#[test]
fn test_text_collator() {
    let en_collator = TextCollator::new(Locale::new("en").with_country("US"));
    assert_eq!(
        en_collator.compare("apple", "BANANA"),
        std::cmp::Ordering::Less
    );
    assert_eq!(
        en_collator.compare("zebra", "Apple"),
        std::cmp::Ordering::Greater
    );
    let mut items = vec![
        "Zebra".to_string(),
        "apple".to_string(),
        "Banana".to_string(),
    ];
    en_collator.sort(&mut items);
    assert_eq!(items, vec!["apple", "Banana", "Zebra"]);
    assert!(en_collator.starts_with("Contract", "con"));
    assert!(en_collator.starts_with("STATUTE", "stat"));
    let de_collator = TextCollator::new(Locale::new("de").with_country("DE"));
    assert_eq!(de_collator.normalize("äöü"), "aeoeue");
    assert_eq!(de_collator.normalize("Straße"), "strasse");
}
#[test]
fn test_dictionary_export_import() {
    let mut dict = LegalDictionary::new(Locale::new("en").with_country("US"));
    dict.add_translation("contract", "contract");
    dict.add_translation("statute", "statute");
    dict.add_abbreviation("corporation", "Corp.");
    dict.add_contextual_translation("right", "legal", "legal right");
    let json = dict.to_json().unwrap();
    assert!(json.contains("contract"));
    assert!(json.contains("statute"));
    let imported = LegalDictionary::from_json(&json).unwrap();
    assert_eq!(imported.translate("contract"), Some("contract"));
    assert_eq!(imported.translate("statute"), Some("statute"));
    assert_eq!(imported.get_abbreviation("corporation"), Some("Corp."));
    assert_eq!(
        imported.translate_with_context("right", "legal"),
        Some("legal right")
    );
}
#[test]
fn test_dictionary_merge() {
    let mut dict1 = LegalDictionary::new(Locale::new("en").with_country("US"));
    dict1.add_translation("contract", "contract");
    dict1.add_translation("statute", "statute");
    let mut dict2 = LegalDictionary::new(Locale::new("en").with_country("US"));
    dict2.add_translation("statute", "law");
    dict2.add_translation("court", "court");
    dict1.merge(&dict2);
    assert_eq!(dict1.translate("contract"), Some("contract"));
    assert_eq!(dict1.translate("statute"), Some("statute"));
    assert_eq!(dict1.translate("court"), Some("court"));
}
#[test]
fn test_dictionary_counts() {
    let mut dict = LegalDictionary::new(Locale::new("en").with_country("US"));
    dict.add_translation("contract", "contract");
    dict.add_translation("statute", "statute");
    dict.add_definition("contract", "A legally binding agreement");
    dict.add_abbreviation("corporation", "Corp.");
    dict.add_contextual_translation("right", "legal", "legal right");
    assert_eq!(dict.translation_count(), 2);
    assert_eq!(dict.definition_count(), 1);
    assert_eq!(dict.abbreviation_count(), 1);
    assert_eq!(dict.contextual_translation_count(), 1);
}
#[test]
fn test_suggest_best_locale() {
    let available = vec![
        Locale::new("en").with_country("US"),
        Locale::new("en").with_country("GB"),
        Locale::new("fr").with_country("FR"),
        Locale::new("ja"),
    ];
    let requested = Locale::new("en").with_country("US");
    let suggested = suggest_best_locale(&requested, &available).unwrap();
    assert_eq!(suggested, &Locale::new("en").with_country("US"));
    let requested = Locale::new("ja").with_country("JP");
    let suggested = suggest_best_locale(&requested, &available).unwrap();
    assert_eq!(suggested.language, "ja");
    let requested = Locale::new("de").with_country("DE");
    let suggested = suggest_best_locale(&requested, &available);
    assert!(suggested.is_none());
}
#[test]
fn test_common_legal_locales() {
    let locales = common_legal_locales();
    assert!(locales.len() >= 10);
    assert!(locales.iter().any(|l| l.tag() == "en-US"));
    assert!(locales.iter().any(|l| l.tag() == "ja-JP"));
    assert!(locales.iter().any(|l| l.tag() == "zh-Hans-CN"));
}
#[test]
fn test_timezone_utc_to_local() {
    let jst = TimeZone::new("Asia/Tokyo", 540, "Japan Standard Time (JST)", false);
    let (y, m, d, h, min) = jst.utc_to_local(2024, 1, 1, 0, 0);
    assert_eq!((y, m, d, h, min), (2024, 1, 1, 9, 0));
    let (y, m, d, h, min) = jst.utc_to_local(2024, 1, 1, 23, 0);
    assert_eq!((y, m, d, h, min), (2024, 1, 2, 8, 0));
}
#[test]
fn test_timezone_local_to_utc() {
    let est = TimeZone::new(
        "America/New_York",
        -300,
        "Eastern Standard Time (EST)",
        true,
    );
    let (y, m, d, h, min) = est.local_to_utc(2024, 1, 1, 9, 0);
    assert_eq!((y, m, d, h, min), (2024, 1, 1, 14, 0));
    let (y, m, d, h, min) = est.local_to_utc(2024, 1, 1, 2, 0);
    assert_eq!((y, m, d, h, min), (2024, 1, 1, 7, 0));
}
#[test]
fn test_timezone_format_offset() {
    let jst = TimeZone::new("Asia/Tokyo", 540, "Japan Standard Time (JST)", false);
    assert_eq!(jst.format_offset(), "+09:00");
    let est = TimeZone::new(
        "America/New_York",
        -300,
        "Eastern Standard Time (EST)",
        true,
    );
    assert_eq!(est.format_offset(), "-05:00");
    let utc = TimeZone::new("UTC", 0, "Coordinated Universal Time (UTC)", false);
    assert_eq!(utc.format_offset(), "+00:00");
}
#[test]
fn test_timezone_registry() {
    let registry = TimeZoneRegistry::with_defaults();
    assert!(registry.get_zone("Asia/Tokyo").is_some());
    assert!(registry.get_zone("America/New_York").is_some());
    assert!(registry.get_zone("Europe/London").is_some());
    assert!(registry.get_zone("UTC").is_some());
    let jp_tz = registry.zone_for_jurisdiction("JP").unwrap();
    assert_eq!(jp_tz.identifier, "Asia/Tokyo");
    let us_tz = registry.zone_for_jurisdiction("US").unwrap();
    assert_eq!(us_tz.identifier, "America/New_York");
}
#[test]
fn test_deadline_calculator_basic() {
    let jp_config = WorkingDaysConfig::japan();
    let calculator = DeadlineCalculator::new(jp_config);
    let (y, m, d) = calculator.calculate_deadline(2024, 1, 1, 5);
    assert_eq!((y, m, d), (2024, 1, 8));
}
#[test]
fn test_deadline_calculator_with_time() {
    let us_config = WorkingDaysConfig::united_states();
    let calculator = DeadlineCalculator::new(us_config);
    let (_y, _m, _d, h, min) = calculator.calculate_deadline_with_time(2024, 1, 1, 9, 30, 3);
    assert_eq!(h, 9);
    assert_eq!(min, 30);
}
#[test]
fn test_deadline_calculator_timezone_conversion() {
    let jp_config = WorkingDaysConfig::japan();
    let calculator = DeadlineCalculator::new(jp_config);
    let jst = TimeZone::new("Asia/Tokyo", 540, "Japan Standard Time (JST)", false);
    let est = TimeZone::new(
        "America/New_York",
        -300,
        "Eastern Standard Time (EST)",
        true,
    );
    let (y, m, d, h, min) = calculator.convert_timezone(2024, 1, 1, 9, 0, &jst, &est);
    assert_eq!((y, m, d, h, min), (2023, 12, 31, 19, 0));
}
#[test]
fn test_deadline_calculator_is_deadline_passed() {
    let jp_config = WorkingDaysConfig::japan();
    let calculator = DeadlineCalculator::new(jp_config);
    assert!(calculator.is_deadline_passed(2023, 12, 31, 2024, 1, 1));
    assert!(!calculator.is_deadline_passed(2024, 1, 2, 2024, 1, 1));
    assert!(!calculator.is_deadline_passed(2024, 1, 1, 2024, 1, 1));
}
#[test]
fn test_citation_bluebook_case() {
    let formatter = CitationFormatter::new(
        CitationStyle::Bluebook,
        Locale::new("en").with_country("US"),
    );
    let citation = CitationComponents::new("Brown v. Board of Education")
        .with_volume("347")
        .with_reporter("U.S.")
        .with_page("483")
        .with_year(1954);
    let formatted = formatter.format_case(&citation);
    assert!(formatted.contains("Brown v. Board of Education"));
    assert!(formatted.contains("347 U.S. 483"));
    assert!(formatted.contains("1954"));
}
#[test]
fn test_citation_oscola_case() {
    let formatter =
        CitationFormatter::new(CitationStyle::OSCOLA, Locale::new("en").with_country("GB"));
    let citation = CitationComponents::new("Donoghue v Stevenson")
        .with_volume("1932")
        .with_reporter("AC")
        .with_page("562")
        .with_year(1932);
    let formatted = formatter.format_case(&citation);
    assert!(formatted.contains("Donoghue v Stevenson"));
    assert!(formatted.contains("[1932]"));
    assert!(formatted.contains("1932 AC"));
}
#[test]
fn test_citation_bluebook_statute() {
    let formatter = CitationFormatter::new(
        CitationStyle::Bluebook,
        Locale::new("en").with_country("US"),
    );
    let citation = CitationComponents::new("Civil Rights Act")
        .with_reporter("U.S.C.")
        .with_page("2000a")
        .with_year(1964);
    let formatted = formatter.format_statute(&citation);
    assert!(formatted.contains("Civil Rights Act"));
    assert!(formatted.contains("U.S.C."));
    assert!(formatted.contains("§ 2000a"));
    assert!(formatted.contains("(1964)"));
}
#[test]
fn test_citation_japanese() {
    let formatter = CitationFormatter::new(
        CitationStyle::Japanese,
        Locale::new("ja").with_country("JP"),
    );
    let citation = CitationComponents::new("最高裁判所判決")
        .with_court("最高裁")
        .with_volume("123")
        .with_page("45")
        .with_year(2020);
    let formatted = formatter.format_case(&citation);
    assert!(formatted.contains("最高裁判所判決"));
    assert!(formatted.contains("最高裁"));
    assert!(formatted.contains("2020"));
    assert!(formatted.contains("123号"));
    assert!(formatted.contains("45頁"));
}
#[test]
fn test_citation_style_for_jurisdiction() {
    assert_eq!(
        CitationFormatter::style_for_jurisdiction("US"),
        CitationStyle::Bluebook
    );
    assert_eq!(
        CitationFormatter::style_for_jurisdiction("GB"),
        CitationStyle::OSCOLA
    );
    assert_eq!(
        CitationFormatter::style_for_jurisdiction("AU"),
        CitationStyle::AGLC
    );
    assert_eq!(
        CitationFormatter::style_for_jurisdiction("CA"),
        CitationStyle::McGill
    );
    assert_eq!(
        CitationFormatter::style_for_jurisdiction("JP"),
        CitationStyle::Japanese
    );
    assert_eq!(
        CitationFormatter::style_for_jurisdiction("DE"),
        CitationStyle::European
    );
}
#[test]
fn test_text_direction_detection() {
    let arabic = Locale::new("ar");
    assert_eq!(
        BidirectionalText::detect_direction(&arabic),
        TextDirection::RTL
    );
    let hebrew = Locale::new("he");
    assert_eq!(
        BidirectionalText::detect_direction(&hebrew),
        TextDirection::RTL
    );
    let english = Locale::new("en");
    assert_eq!(
        BidirectionalText::detect_direction(&english),
        TextDirection::LTR
    );
    let persian = Locale::new("fa");
    assert_eq!(
        BidirectionalText::detect_direction(&persian),
        TextDirection::RTL
    );
}
#[test]
fn test_bidirectional_text_rtl() {
    let arabic_locale = Locale::new("ar");
    let bidi = BidirectionalText::new(arabic_locale);
    assert!(bidi.is_rtl());
    assert_eq!(bidi.direction(), TextDirection::RTL);
}
#[test]
fn test_bidirectional_text_ltr() {
    let english_locale = Locale::new("en");
    let bidi = BidirectionalText::new(english_locale);
    assert!(!bidi.is_rtl());
    assert_eq!(bidi.direction(), TextDirection::LTR);
}
#[test]
fn test_direction_markers() {
    let arabic_locale = Locale::new("ar");
    let bidi = BidirectionalText::new(arabic_locale);
    let wrapped = bidi.wrap_with_direction_markers("نص عربي");
    assert!(wrapped.contains('\u{202B}'));
    assert!(wrapped.contains('\u{202C}'));
}
#[test]
fn test_arabic_numerals() {
    let arabic_locale = Locale::new("ar");
    let bidi = BidirectionalText::new(arabic_locale);
    let formatted = bidi.format_number(123);
    assert_eq!(formatted, "١٢٣");
    let formatted_year = bidi.format_number(2024);
    assert_eq!(formatted_year, "٢٠٢٤");
}
#[test]
fn test_persian_numerals() {
    let persian_locale = Locale::new("fa");
    let bidi = BidirectionalText::new(persian_locale);
    let formatted = bidi.format_number(123);
    assert_eq!(formatted, "۱۲۳");
}
#[test]
fn test_rtl_date_formatting() {
    let arabic_locale = Locale::new("ar");
    let bidi = BidirectionalText::new(arabic_locale);
    let date = bidi.format_date_rtl(2024, 1, 15);
    assert!(date.contains('١'));
    assert!(date.contains('٥'));
}
#[test]
fn test_paragraph_formatting() {
    let arabic_locale = Locale::new("ar");
    let bidi = BidirectionalText::new(arabic_locale);
    let paragraph = bidi.format_paragraph("هذا نص عربي");
    assert!(paragraph.contains("dir=\"rtl\""));
    assert!(paragraph.starts_with("<p"));
    assert!(paragraph.ends_with("</p>"));
}
#[test]
fn test_list_formatting() {
    let hebrew_locale = Locale::new("he");
    let bidi = BidirectionalText::new(hebrew_locale);
    let items = vec!["פריט 1".to_string(), "פריט 2".to_string()];
    let list = bidi.format_list(&items);
    assert!(list.contains("dir=\"rtl\""));
    assert!(list.contains("<ul"));
    assert!(list.contains("<li>"));
}
#[test]
fn test_name_order_detection() {
    let japanese = Locale::new("ja");
    assert_eq!(
        NameFormatter::detect_name_order(&japanese),
        NameOrder::FamilyFirst
    );
    let korean = Locale::new("ko");
    assert_eq!(
        NameFormatter::detect_name_order(&korean),
        NameOrder::FamilyFirst
    );
    let english = Locale::new("en");
    assert_eq!(
        NameFormatter::detect_name_order(&english),
        NameOrder::GivenFirst
    );
    let chinese = Locale::new("zh");
    assert_eq!(
        NameFormatter::detect_name_order(&chinese),
        NameOrder::FamilyFirst
    );
}
#[test]
fn test_western_name_formatting() {
    let formatter = NameFormatter::new(Locale::new("en").with_country("US"));
    let name = PersonName::new("John", "Smith")
        .with_middle_name("David")
        .with_prefix("Dr.")
        .with_suffix("Jr.");
    let formatted = formatter.format_full_name(&name);
    assert_eq!(formatted, "Dr. John David Smith Jr.");
}
#[test]
fn test_japanese_name_formatting() {
    let formatter = NameFormatter::new(Locale::new("ja").with_country("JP"));
    let name = PersonName::new("太郎", "山田");
    let formatted = formatter.format_full_name(&name);
    assert_eq!(formatted, "山田 太郎");
}
#[test]
fn test_korean_name_formatting() {
    let formatter = NameFormatter::new(Locale::new("ko").with_country("KR"));
    let name = PersonName::new("민수", "김");
    let formatted = formatter.format_full_name(&name);
    assert_eq!(formatted, "김민수");
}
#[test]
fn test_chinese_name_formatting() {
    let cn_formatter = NameFormatter::new(Locale::new("zh").with_country("CN"));
    let name = PersonName::new("伟", "李");
    let formatted = cn_formatter.format_full_name(&name);
    assert_eq!(formatted, "李伟");
    let tw_formatter = NameFormatter::new(Locale::new("zh").with_country("TW"));
    let formatted_tw = tw_formatter.format_full_name(&name);
    assert_eq!(formatted_tw, "李 伟");
}
#[test]
fn test_russian_name_formatting() {
    let formatter = NameFormatter::new(Locale::new("ru").with_country("RU"));
    let name = PersonName::new("Иван", "Иванов").with_patronymic("Иванович");
    let formatted = formatter.format_full_name(&name);
    assert_eq!(formatted, "Иванов Иван Иванович");
}
#[test]
fn test_arabic_name_formatting() {
    let formatter = NameFormatter::new(Locale::new("ar"));
    let name = PersonName::new("محمد", "الأحمد").with_patronymic("بن علي");
    let formatted = formatter.format_full_name(&name);
    assert_eq!(formatted, "محمد بن علي الأحمد");
}
#[test]
fn test_name_citation_format() {
    let formatter = NameFormatter::new(Locale::new("en").with_country("US"));
    let name = PersonName::new("John", "Smith").with_middle_name("David");
    let citation = formatter.format_citation(&name);
    assert_eq!(citation, "Smith, John David");
}
#[test]
fn test_name_initials() {
    let formatter = NameFormatter::new(Locale::new("en").with_country("US"));
    let name = PersonName::new("John", "Smith").with_middle_name("David");
    let initials = formatter.format_initials(&name);
    assert_eq!(initials, "J. D. S.");
    let name_no_middle = PersonName::new("John", "Smith");
    let initials_no_middle = formatter.format_initials(&name_no_middle);
    assert_eq!(initials_no_middle, "J. S.");
}
#[test]
fn test_us_address_formatting() {
    let formatter = AddressFormatter::new(Locale::new("en").with_country("US"));
    let address = Address::new("123 Main St", "New York", "10001", "USA").with_state("NY");
    let formatted = formatter.format(&address);
    assert!(formatted.contains("123 Main St"));
    assert!(formatted.contains("New York, NY 10001"));
    assert!(formatted.contains("USA"));
}
#[test]
fn test_uk_address_formatting() {
    let formatter = AddressFormatter::new(Locale::new("en").with_country("GB"));
    let address = Address::new("10 Downing Street", "London", "SW1A 2AA", "United Kingdom");
    let formatted = formatter.format(&address);
    assert!(formatted.contains("10 Downing Street"));
    assert!(formatted.contains("London"));
    assert!(formatted.contains("SW1A 2AA"));
}
#[test]
fn test_japanese_address_formatting() {
    let formatter = AddressFormatter::new(Locale::new("ja").with_country("JP"));
    let address = Address::new("1-1-1", "千代田区", "100-0001", "日本")
        .with_state("東京都")
        .with_building("ビル101");
    let formatted = formatter.format(&address);
    assert!(formatted.contains("〒100-0001"));
    assert!(formatted.contains("東京都"));
    assert!(formatted.contains("千代田区"));
    assert!(formatted.contains("ビル101"));
}
#[test]
fn test_european_address_formatting() {
    let formatter = AddressFormatter::new(Locale::new("de").with_country("DE"));
    let address = Address::new("Hauptstraße 1", "Berlin", "10115", "Germany");
    let formatted = formatter.format(&address);
    assert!(formatted.contains("Hauptstraße 1"));
    assert!(formatted.contains("10115 Berlin"));
    assert!(formatted.contains("Germany"));
}
#[test]
fn test_address_single_line() {
    let formatter = AddressFormatter::new(Locale::new("en").with_country("US"));
    let address = Address::new("123 Main St", "New York", "10001", "USA").with_state("NY");
    let single_line = formatter.format_single_line(&address);
    assert!(!single_line.contains('\n'));
    assert!(single_line.contains(", "));
}
