#![cfg(test)]
use super::*;

#[test]
fn test_sub_regional_variation_us_states() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let ca_variation = registry.find_variation("US", "CA");
    assert!(ca_variation.is_some());
    let ca = ca_variation.unwrap();
    assert_eq!(ca.region_name, "California");
    assert_eq!(ca.region_code, "CA");
    assert!(
        ca.legal_differences
            .iter()
            .any(|d| d.contains("Community property state"))
    );
    assert!(ca.legal_differences.iter().any(|d| d.contains("CCPA")));
    let ny_variation = registry.find_variation("US", "NY");
    assert!(ny_variation.is_some());
    let ny = ny_variation.unwrap();
    assert_eq!(ny.region_name, "New York");
    assert!(
        ny.legal_differences
            .iter()
            .any(|d| d.contains("Martin Act"))
    );
    let de_variation = registry.find_variation("US", "DE");
    assert!(de_variation.is_some());
    let de = de_variation.unwrap();
    assert_eq!(de.region_name, "Delaware");
    assert!(de.legal_differences.iter().any(|d| d.contains("DGCL")));
    assert!(
        de.legal_differences
            .iter()
            .any(|d| d.contains("Court of Chancery"))
    );
}
#[test]
fn test_sub_regional_variation_canadian_provinces() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let on_variation = registry.find_variation("CA", "ON");
    assert!(on_variation.is_some());
    let on = on_variation.unwrap();
    assert_eq!(on.region_name, "Ontario");
    assert!(
        on.legal_differences
            .iter()
            .any(|d| d.contains("Common law province"))
    );
    let qc_variation = registry.find_variation("CA", "QC");
    assert!(qc_variation.is_some());
    let qc = qc_variation.unwrap();
    assert_eq!(qc.region_name, "Québec");
    assert!(
        qc.legal_differences
            .iter()
            .any(|d| d.contains("Civil law jurisdiction"))
    );
    assert!(
        qc.legal_differences
            .iter()
            .any(|d| d.contains("Code civil du Québec"))
    );
    let bc_variation = registry.find_variation("CA", "BC");
    assert!(bc_variation.is_some());
    let bc = bc_variation.unwrap();
    assert_eq!(bc.region_name, "British Columbia");
}
#[test]
fn test_sub_regional_variation_get_all_for_country() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let us_variations = registry.get_variations_for_country("US");
    assert!(us_variations.len() >= 6);
    let ca_variations = registry.get_variations_for_country("CA");
    assert!(ca_variations.len() >= 4);
}
#[test]
fn test_eu_member_state_variations() {
    let registry = EUMemberStateRegistry::with_defaults();
    let de = registry.find_variation("DE");
    assert!(de.is_some());
    let germany = de.unwrap();
    assert_eq!(germany.country_name, "Germany");
    assert_eq!(germany.accession_year, 1958);
    assert!(germany.legal_system.contains("Civil law"));
    assert!(germany.eu_adaptations.iter().any(|a| a.contains("GDPR")));
    assert!(
        germany
            .specialties
            .iter()
            .any(|s| s.contains("Mitbestimmung"))
    );
    let fr = registry.find_variation("FR");
    assert!(fr.is_some());
    let france = fr.unwrap();
    assert_eq!(france.country_name, "France");
    assert!(france.legal_system.contains("Napoleonic Code"));
    assert!(
        france
            .specialties
            .iter()
            .any(|s| s.contains("Conseil d'État"))
    );
    let ie = registry.find_variation("IE");
    assert!(ie.is_some());
    let ireland = ie.unwrap();
    assert_eq!(ireland.country_name, "Ireland");
    assert!(ireland.legal_system.contains("Common law"));
    assert!(
        ireland
            .specialties
            .iter()
            .any(|s| s.contains("Common law in EU context"))
    );
}
#[test]
fn test_eu_member_state_all_variations() {
    let registry = EUMemberStateRegistry::with_defaults();
    let all = registry.get_all_variations();
    assert!(all.len() >= 10);
}
#[test]
fn test_dialect_terminology_scottish() {
    let registry = DialectTerminologyRegistry::with_defaults();
    let locale = Locale::new("en").with_country("GB");
    let scottish = registry.find_dialect(&locale, "Scottish Legal");
    assert!(scottish.is_some());
    let dialect = scottish.unwrap();
    assert_eq!(dialect.to_dialect("lawyer"), Some("advocate"));
    assert_eq!(
        dialect.to_dialect("real_estate"),
        Some("heritable property")
    );
    assert_eq!(dialect.to_dialect("mortgage"), Some("standard security"));
    assert_eq!(dialect.to_dialect("plaintiff"), Some("pursuer"));
    assert_eq!(dialect.to_dialect("defendant"), Some("defender"));
    assert_eq!(dialect.from_dialect("advocate"), Some("lawyer"));
    assert_eq!(dialect.from_dialect("pursuer"), Some("plaintiff"));
}
#[test]
fn test_dialect_terminology_louisiana() {
    let registry = DialectTerminologyRegistry::with_defaults();
    let locale = Locale::new("en").with_country("US");
    let louisiana = registry.find_dialect(&locale, "Louisiana Legal");
    assert!(louisiana.is_some());
    let dialect = louisiana.unwrap();
    assert_eq!(dialect.to_dialect("county"), Some("parish"));
    assert_eq!(
        dialect.to_dialect("real_estate"),
        Some("immovable property")
    );
    assert_eq!(dialect.to_dialect("common_law"), Some("civil law"));
    assert_eq!(dialect.to_dialect("deed"), Some("act of sale"));
}
#[test]
fn test_dialect_terminology_quebec() {
    let registry = DialectTerminologyRegistry::with_defaults();
    let locale = Locale::new("fr").with_country("CA");
    let quebec = registry.find_dialect(&locale, "Québec Legal");
    assert!(quebec.is_some());
    let dialect = quebec.unwrap();
    assert_eq!(
        dialect.to_dialect("code_civil"),
        Some("Code civil du Québec")
    );
    assert_eq!(
        dialect.to_dialect("jurisprudence"),
        Some("jurisprudence québécoise")
    );
}
#[test]
fn test_dialect_terminology_get_all_for_locale() {
    let registry = DialectTerminologyRegistry::with_defaults();
    let us_locale = Locale::new("en").with_country("US");
    let us_dialects = registry.get_dialects_for_locale(&us_locale);
    assert!(!us_dialects.is_empty());
    let gb_locale = Locale::new("en").with_country("GB");
    let gb_dialects = registry.get_dialects_for_locale(&gb_locale);
    assert!(!gb_dialects.is_empty());
}
#[test]
fn test_regional_concept_mapping_trust() {
    let mapper = RegionalConceptMapper::with_defaults();
    let mappings = mapper.find_mappings("trust", "GB", "FR");
    assert!(!mappings.is_empty());
    let mapping = mappings[0];
    assert_eq!(mapping.source_concept, "trust");
    assert_eq!(mapping.target_concept, "fiducie");
    assert_eq!(mapping.similarity, 0.7);
    assert!(!mapping.notes.is_empty());
    assert!(mapping.notes.iter().any(|n| n.contains("equity concept")));
}
#[test]
fn test_regional_concept_mapping_llc() {
    let mapper = RegionalConceptMapper::with_defaults();
    let mappings = mapper.find_mappings("LLC", "US", "DE");
    assert!(!mappings.is_empty());
    let mapping = mappings[0];
    assert_eq!(mapping.source_concept, "LLC");
    assert_eq!(mapping.target_concept, "GmbH");
    assert_eq!(mapping.similarity, 0.9);
    assert!(
        mapping
            .notes
            .iter()
            .any(|n| n.contains("limited liability"))
    );
}
#[test]
fn test_regional_concept_mapping_corporation() {
    let mapper = RegionalConceptMapper::with_defaults();
    let mappings = mapper.find_mappings("corporation", "US", "JP");
    assert!(!mappings.is_empty());
    let mapping = mappings[0];
    assert_eq!(mapping.target_concept, "kabushiki kaisha");
    assert_eq!(mapping.similarity, 0.85);
}
#[test]
fn test_regional_concept_mapping_find_all_for_concept() {
    let mapper = RegionalConceptMapper::with_defaults();
    let all_trust_mappings = mapper.find_all_mappings_for_concept("trust");
    assert!(!all_trust_mappings.is_empty());
    let all_corporation_mappings = mapper.find_all_mappings_for_concept("corporation");
    assert!(!all_corporation_mappings.is_empty());
}
#[test]
fn test_cross_regional_term_equivalence_attorney() {
    let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();
    let equiv = registry.find_equivalence("attorney", "US");
    assert!(equiv.is_some());
    let attorney_equiv = equiv.unwrap();
    assert_eq!(attorney_equiv.base_term, "attorney");
    assert_eq!(attorney_equiv.base_jurisdiction, "US");
    let gb_term = attorney_equiv.get_equivalent("GB");
    assert!(gb_term.is_some());
    assert_eq!(gb_term.unwrap().term, "solicitor");
    assert_eq!(
        gb_term.unwrap().equivalence_level,
        EquivalenceLevel::Approximate
    );
    let fr_term = attorney_equiv.get_equivalent("FR");
    assert!(fr_term.is_some());
    assert_eq!(fr_term.unwrap().term, "avocat");
    assert_eq!(fr_term.unwrap().equivalence_level, EquivalenceLevel::Exact);
    let de_term = attorney_equiv.get_equivalent("DE");
    assert!(de_term.is_some());
    assert_eq!(de_term.unwrap().term, "Rechtsanwalt");
}
#[test]
fn test_cross_regional_term_equivalence_corporation() {
    let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();
    let corp_term = registry.get_equivalent_term("corporation", "US", "JP");
    assert!(corp_term.is_some());
    assert_eq!(corp_term.unwrap().term, "kabushiki kaisha");
    assert_eq!(
        corp_term.unwrap().equivalence_level,
        EquivalenceLevel::Exact
    );
    let de_term = registry.get_equivalent_term("corporation", "US", "DE");
    assert!(de_term.is_some());
    assert_eq!(de_term.unwrap().term, "Aktiengesellschaft");
}
#[test]
fn test_cross_regional_term_equivalence_contract() {
    let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();
    let equiv = registry.find_equivalence("contract", "US");
    assert!(equiv.is_some());
    let contract_equiv = equiv.unwrap();
    let gb = contract_equiv.get_equivalent("GB");
    assert_eq!(gb.unwrap().equivalence_level, EquivalenceLevel::Exact);
    let fr = contract_equiv.get_equivalent("FR");
    assert_eq!(fr.unwrap().equivalence_level, EquivalenceLevel::Exact);
    assert_eq!(fr.unwrap().term, "contrat");
    let de = contract_equiv.get_equivalent("DE");
    assert_eq!(de.unwrap().equivalence_level, EquivalenceLevel::Exact);
    assert_eq!(de.unwrap().term, "Vertrag");
}
#[test]
fn test_cross_regional_term_equivalence_trust() {
    let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();
    let equiv = registry.find_equivalence("trust", "GB");
    assert!(equiv.is_some());
    let trust_equiv = equiv.unwrap();
    let fr = trust_equiv.get_equivalent("FR");
    assert!(fr.is_some());
    assert_eq!(fr.unwrap().term, "fiducie");
    assert_eq!(fr.unwrap().equivalence_level, EquivalenceLevel::Approximate);
    assert!(!fr.unwrap().notes.is_empty());
    let de = trust_equiv.get_equivalent("DE");
    assert!(de.is_some());
    assert_eq!(de.unwrap().term, "Treuhand");
    assert_eq!(de.unwrap().equivalence_level, EquivalenceLevel::Loose);
}
#[test]
fn test_cross_regional_term_equivalence_plaintiff() {
    let registry = CrossRegionalTermEquivalenceRegistry::with_defaults();
    let plaintiff_fr = registry.get_equivalent_term("plaintiff", "US", "FR");
    assert!(plaintiff_fr.is_some());
    assert_eq!(plaintiff_fr.unwrap().term, "demandeur");
    let plaintiff_de = registry.get_equivalent_term("plaintiff", "US", "DE");
    assert!(plaintiff_de.is_some());
    assert_eq!(plaintiff_de.unwrap().term, "Kläger");
    let plaintiff_gb = registry.get_equivalent_term("plaintiff", "US", "GB");
    assert!(plaintiff_gb.is_some());
    assert_eq!(plaintiff_gb.unwrap().term, "claimant");
}
#[test]
fn test_equivalence_level_enum() {
    let exact = EquivalenceLevel::Exact;
    let approximate = EquivalenceLevel::Approximate;
    let loose = EquivalenceLevel::Loose;
    let no_equiv = EquivalenceLevel::NoEquivalent;
    assert_eq!(exact, EquivalenceLevel::Exact);
    assert_eq!(approximate, EquivalenceLevel::Approximate);
    assert_eq!(loose, EquivalenceLevel::Loose);
    assert_eq!(no_equiv, EquivalenceLevel::NoEquivalent);
    assert_ne!(exact, approximate);
    assert_ne!(approximate, loose);
    assert_ne!(loose, no_equiv);
}
#[test]
fn test_sub_regional_variation_custom() {
    let mut registry = SubRegionalVariationRegistry::new();
    let custom = SubRegionalVariation::new(
        Locale::new("en").with_country("AU"),
        "NSW",
        "New South Wales",
        "NSW state law",
    )
    .add_legal_difference("Separate state supreme court")
    .add_legal_difference("NSW-specific legislation");
    registry.add_variation(custom);
    let found = registry.find_variation("AU", "NSW");
    assert!(found.is_some());
    assert_eq!(found.unwrap().region_name, "New South Wales");
}
#[test]
fn test_asian_regional_variations() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let maharashtra = registry.find_variation("IN", "MH");
    assert!(maharashtra.is_some());
    assert_eq!(maharashtra.unwrap().region_name, "Maharashtra");
    assert!(
        maharashtra
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Bombay High Court"))
    );
    let delhi = registry.find_variation("IN", "DL");
    assert!(delhi.is_some());
    assert_eq!(delhi.unwrap().region_name, "Delhi");
    let karnataka = registry.find_variation("IN", "KA");
    assert!(karnataka.is_some());
    assert!(
        karnataka
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Tech industry"))
    );
    let singapore = registry.find_variation("SG", "SG");
    assert!(singapore.is_some());
    assert!(
        singapore
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Common law"))
    );
    let kl = registry.find_variation("MY", "WP");
    assert!(kl.is_some());
    assert!(
        kl.unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Islamic law"))
    );
    let bangkok = registry.find_variation("TH", "BKK");
    assert!(bangkok.is_some());
    assert!(
        bangkok
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Civil law"))
    );
    let hanoi = registry.find_variation("VN", "HN");
    assert!(hanoi.is_some());
    assert_eq!(hanoi.unwrap().region_name, "Hanoi");
    let hcmc = registry.find_variation("VN", "SG");
    assert!(hcmc.is_some());
    assert_eq!(hcmc.unwrap().region_name, "Ho Chi Minh City");
    let jakarta = registry.find_variation("ID", "JK");
    assert!(jakarta.is_some());
    assert!(
        jakarta
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Dutch-influenced"))
    );
}
#[test]
fn test_middle_eastern_regional_variations() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let dubai = registry.find_variation("AE", "DU");
    assert!(dubai.is_some());
    assert_eq!(dubai.unwrap().region_name, "Dubai");
    assert!(
        dubai
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("DIFC"))
    );
    let abu_dhabi = registry.find_variation("AE", "AZ");
    assert!(abu_dhabi.is_some());
    assert!(
        abu_dhabi
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("ADGM"))
    );
    let riyadh = registry.find_variation("SA", "RI");
    assert!(riyadh.is_some());
    assert!(
        riyadh
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Sharia law"))
    );
    let tel_aviv = registry.find_variation("IL", "TA");
    assert!(tel_aviv.is_some());
    assert!(
        tel_aviv
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Tech startup"))
    );
}
#[test]
fn test_latin_american_regional_variations() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let sao_paulo = registry.find_variation("BR", "SP");
    assert!(sao_paulo.is_some());
    assert_eq!(sao_paulo.unwrap().region_name, "São Paulo");
    assert!(
        sao_paulo
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Civil law"))
    );
    let rio = registry.find_variation("BR", "RJ");
    assert!(rio.is_some());
    assert!(
        rio.unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Oil and gas"))
    );
    let buenos_aires = registry.find_variation("AR", "BA");
    assert!(buenos_aires.is_some());
    assert!(
        buenos_aires
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Código Civil"))
    );
    let mexico_city = registry.find_variation("MX", "CMX");
    assert!(mexico_city.is_some());
    assert!(
        mexico_city
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Amparo"))
    );
    let santiago = registry.find_variation("CL", "RM");
    assert!(santiago.is_some());
    assert!(
        santiago
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Mining law"))
    );
    let bogota = registry.find_variation("CO", "DC");
    assert!(bogota.is_some());
    assert!(
        bogota
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("tutela"))
    );
}
#[test]
fn test_african_regional_variations() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let gauteng = registry.find_variation("ZA", "GP");
    assert!(gauteng.is_some());
    assert!(
        gauteng
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Roman-Dutch"))
    );
    let western_cape = registry.find_variation("ZA", "WC");
    assert!(western_cape.is_some());
    assert!(
        western_cape
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Wine industry"))
    );
    let lagos = registry.find_variation("NG", "LA");
    assert!(lagos.is_some());
    assert!(
        lagos
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Common law"))
    );
    let cairo = registry.find_variation("EG", "C");
    assert!(cairo.is_some());
    assert!(
        cairo
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("French-influenced"))
    );
    let nairobi = registry.find_variation("KE", "NBO");
    assert!(nairobi.is_some());
    assert!(
        nairobi
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("East African Court"))
    );
}
#[test]
fn test_additional_us_states() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let washington = registry.find_variation("US", "WA");
    assert!(washington.is_some());
    assert!(
        washington
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("tech industry"))
    );
    let massachusetts = registry.find_variation("US", "MA");
    assert!(massachusetts.is_some());
    assert!(
        massachusetts
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("healthcare"))
    );
    let colorado = registry.find_variation("US", "CO");
    assert!(colorado.is_some());
    assert!(
        colorado
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Cannabis"))
    );
    let nevada = registry.find_variation("US", "NV");
    assert!(nevada.is_some());
    assert!(
        nevada
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Gaming"))
    );
    let us_states = registry.get_variations_for_country("US");
    assert!(us_states.len() >= 16);
}
#[test]
fn test_canadian_territories() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    let yukon = registry.find_variation("CA", "YT");
    assert!(yukon.is_some());
    assert_eq!(yukon.unwrap().region_name, "Yukon");
    assert!(
        yukon
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Indigenous"))
    );
    let nwt = registry.find_variation("CA", "NT");
    assert!(nwt.is_some());
    assert_eq!(nwt.unwrap().region_name, "Northwest Territories");
    let nunavut = registry.find_variation("CA", "NU");
    assert!(nunavut.is_some());
    assert!(
        nunavut
            .unwrap()
            .legal_differences
            .iter()
            .any(|d| d.contains("Inuit"))
    );
    let ca_regions = registry.get_variations_for_country("CA");
    assert!(ca_regions.len() >= 7);
}
#[test]
fn test_regional_coverage_count() {
    let registry = SubRegionalVariationRegistry::with_defaults();
    assert!(registry.get_variations_for_country("US").len() >= 16);
    assert!(registry.get_variations_for_country("CA").len() >= 7);
    assert!(registry.get_variations_for_country("IN").len() >= 3);
    assert!(registry.get_variations_for_country("AE").len() >= 2);
    assert!(registry.get_variations_for_country("BR").len() >= 2);
    assert!(registry.get_variations_for_country("ZA").len() >= 2);
    assert!(registry.get_variations_for_country("VN").len() >= 2);
    assert!(registry.find_variation("SG", "SG").is_some());
    assert!(registry.find_variation("MY", "WP").is_some());
    assert!(registry.find_variation("TH", "BKK").is_some());
    assert!(registry.find_variation("ID", "JK").is_some());
    assert!(registry.find_variation("SA", "RI").is_some());
    assert!(registry.find_variation("IL", "TA").is_some());
    assert!(registry.find_variation("AR", "BA").is_some());
    assert!(registry.find_variation("MX", "CMX").is_some());
    assert!(registry.find_variation("CL", "RM").is_some());
    assert!(registry.find_variation("CO", "DC").is_some());
    assert!(registry.find_variation("NG", "LA").is_some());
    assert!(registry.find_variation("EG", "C").is_some());
    assert!(registry.find_variation("KE", "NBO").is_some());
}
#[test]
fn test_eu_member_state_custom() {
    let mut registry = EUMemberStateRegistry::new();
    let custom = EUMemberStateVariation::new(
        Locale::new("fi").with_country("FI"),
        "Finland",
        1995,
        "Civil law (Nordic tradition)",
    )
    .add_eu_adaptation("GDPR through Finnish law")
    .add_specialty("Nordic legal tradition");
    registry.add_variation(custom);
    let found = registry.find_variation("FI");
    assert!(found.is_some());
    assert_eq!(found.unwrap().country_name, "Finland");
}
#[test]
fn test_dialect_terminology_custom() {
    let mut registry = DialectTerminologyRegistry::new();
    let mut custom = DialectTerminology::new(Locale::new("en").with_country("IN"), "Indian Legal");
    custom.add_term("lawyer", "advocate");
    custom.add_term("judge", "honourable justice");
    registry.add_dialect(custom);
    let found = registry.find_dialect(&Locale::new("en").with_country("IN"), "Indian Legal");
    assert!(found.is_some());
    assert_eq!(found.unwrap().to_dialect("lawyer"), Some("advocate"));
}
#[test]
fn test_regional_concept_mapper_custom() {
    let mut mapper = RegionalConceptMapper::new();
    let custom = RegionalConceptMapping::new(
        "limited_partnership",
        "US",
        "kommanditgesellschaft",
        "DE",
        0.85,
    )
    .add_note("Both have limited and general partners");
    mapper.add_mapping(custom);
    let found = mapper.find_mappings("limited_partnership", "US", "DE");
    assert!(!found.is_empty());
    assert_eq!(found[0].target_concept, "kommanditgesellschaft");
}
#[test]
fn test_term_equivalence_custom() {
    let mut registry = CrossRegionalTermEquivalenceRegistry::new();
    let custom = TermEquivalence::new("arbitration", "US")
        .add_equivalent("FR", "arbitrage", EquivalenceLevel::Exact)
        .add_equivalent("DE", "Schiedsverfahren", EquivalenceLevel::Exact)
        .add_note_to_equivalent("FR", "International arbitration hub");
    registry.add_equivalence(custom);
    let found = registry.get_equivalent_term("arbitration", "US", "FR");
    assert!(found.is_some());
    assert_eq!(found.unwrap().term, "arbitrage");
}
#[test]
fn test_template_variable_validation() {
    let text_var = TemplateVariable::new("name", VariableType::Text, true, "Person name");
    assert!(text_var.validate("John Doe"));
    assert!(!text_var.validate(""));
    let number_var = TemplateVariable::new("amount", VariableType::Number, true, "Amount");
    assert!(number_var.validate("123.45"));
    assert!(!number_var.validate("not a number"));
    let email_var = TemplateVariable::new("email", VariableType::Email, true, "Email");
    assert!(email_var.validate("user@example.com"));
    assert!(!email_var.validate("not-an-email"));
    let bool_var = TemplateVariable::new("active", VariableType::Boolean, true, "Active");
    assert!(bool_var.validate("true"));
    assert!(bool_var.validate("false"));
    assert!(bool_var.validate("yes"));
    assert!(bool_var.validate("no"));
    assert!(!bool_var.validate("maybe"));
    let date_var = TemplateVariable::new("date", VariableType::Date, true, "Date");
    assert!(date_var.validate("2024-01-15"));
    assert!(date_var.validate("01/15/2024"));
    assert!(!date_var.validate("invalid"));
}
#[test]
fn test_template_variable_with_default() {
    let var = TemplateVariable::new("state", VariableType::Text, false, "State")
        .with_default("California");
    assert_eq!(var.default_value, Some("California".to_string()));
}
#[test]
fn test_template_section_conditional() {
    let mut context = HashMap::new();
    context.insert("jurisdiction".to_string(), "US".to_string());
    let section_with_condition =
        TemplateSection::new("us_only", "US specific content").with_condition("jurisdiction == US");
    assert!(section_with_condition.should_include(&context));
    context.insert("jurisdiction".to_string(), "GB".to_string());
    assert!(!section_with_condition.should_include(&context));
    let section_not_us =
        TemplateSection::new("non_us", "Non-US content").with_condition("jurisdiction != US");
    assert!(section_not_us.should_include(&context));
    context.insert("jurisdiction".to_string(), "US".to_string());
    assert!(!section_not_us.should_include(&context));
}
#[test]
fn test_template_section_no_condition() {
    let context = HashMap::new();
    let section = TemplateSection::new("always", "Always included");
    assert!(section.should_include(&context));
}
#[test]
fn test_document_template_nda_generation() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let template = registry.get_template("nda_mutual_us").unwrap();
    let mut values = HashMap::new();
    values.insert("party1_name".to_string(), "Acme Corp".to_string());
    values.insert("party2_name".to_string(), "Beta LLC".to_string());
    values.insert("effective_date".to_string(), "2024-01-01".to_string());
    values.insert("state".to_string(), "Delaware".to_string());
    let document = template.generate(&values);
    assert!(document.is_ok());
    let doc_text = document.unwrap();
    assert!(doc_text.contains("MUTUAL NON-DISCLOSURE AGREEMENT"));
    assert!(doc_text.contains("Acme Corp"));
    assert!(doc_text.contains("Beta LLC"));
    assert!(doc_text.contains("2024-01-01"));
    assert!(doc_text.contains("Delaware"));
    assert!(doc_text.contains("CONFIDENTIAL INFORMATION"));
    assert!(doc_text.contains("OBLIGATIONS"));
}
#[test]
fn test_document_template_employment_generation() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let template = registry.get_template("employment_agreement_us").unwrap();
    let mut values = HashMap::new();
    values.insert(
        "company_name".to_string(),
        "Tech Innovations Inc".to_string(),
    );
    values.insert("employee_name".to_string(), "Jane Smith".to_string());
    values.insert("position".to_string(), "Software Engineer".to_string());
    values.insert("start_date".to_string(), "2024-03-15".to_string());
    values.insert("salary".to_string(), "120000".to_string());
    values.insert("state".to_string(), "California".to_string());
    let document = template.generate(&values);
    assert!(document.is_ok());
    let doc_text = document.unwrap();
    assert!(doc_text.contains("EMPLOYMENT AGREEMENT"));
    assert!(doc_text.contains("Tech Innovations Inc"));
    assert!(doc_text.contains("Jane Smith"));
    assert!(doc_text.contains("Software Engineer"));
    assert!(doc_text.contains("$120000"));
    assert!(doc_text.contains("AT-WILL EMPLOYMENT"));
}
#[test]
fn test_document_template_complaint_generation() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let template = registry.get_template("complaint_us").unwrap();
    let mut values = HashMap::new();
    values.insert(
        "court_name".to_string(),
        "UNITED STATES DISTRICT COURT\nSOUTHERN DISTRICT OF NEW YORK".to_string(),
    );
    values.insert("plaintiff_name".to_string(), "Alice Johnson".to_string());
    values.insert("defendant_name".to_string(), "Bob Williams".to_string());
    values.insert("case_number".to_string(), "1:24-cv-12345".to_string());
    values.insert(
        "jurisdiction_facts".to_string(),
        "This Court has jurisdiction pursuant to 28 U.S.C. § 1331.".to_string(),
    );
    values.insert(
        "claim_facts".to_string(),
        "On or about December 1, 2023, Defendant breached the contract.".to_string(),
    );
    values.insert(
        "relief_requested".to_string(),
        "Award Plaintiff damages in the amount of $50,000 plus costs and attorney's fees."
            .to_string(),
    );
    let document = template.generate(&values);
    assert!(document.is_ok());
    let doc_text = document.unwrap();
    assert!(doc_text.contains("COMPLAINT"));
    assert!(doc_text.contains("Alice Johnson"));
    assert!(doc_text.contains("Bob Williams"));
    assert!(doc_text.contains("1:24-cv-12345"));
    assert!(doc_text.contains("JURISDICTION AND VENUE"));
    assert!(doc_text.contains("PRAYER FOR RELIEF"));
}
#[test]
fn test_document_template_articles_generation() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let template = registry.get_template("articles_incorporation_de").unwrap();
    let mut values = HashMap::new();
    values.insert(
        "corporation_name".to_string(),
        "NewCo Technologies, Inc.".to_string(),
    );
    values.insert(
        "registered_agent_name".to_string(),
        "Corporation Service Company".to_string(),
    );
    values.insert(
        "registered_agent_address".to_string(),
        "251 Little Falls Drive, Wilmington, DE 19808".to_string(),
    );
    values.insert("shares_authorized".to_string(), "10000000".to_string());
    values.insert("incorporator_name".to_string(), "John Founder".to_string());
    let document = template.generate(&values);
    assert!(document.is_ok());
    let doc_text = document.unwrap();
    assert!(doc_text.contains("CERTIFICATE OF INCORPORATION"));
    assert!(doc_text.contains("NewCo Technologies, Inc."));
    assert!(doc_text.contains("Corporation Service Company"));
    assert!(doc_text.contains("10000000"));
    assert!(doc_text.contains("John Founder"));
    assert!(doc_text.contains("ARTICLE I - NAME"));
    assert!(doc_text.contains("ARTICLE IV - CAPITAL STOCK"));
}
#[test]
fn test_document_template_missing_required_variable() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let template = registry.get_template("nda_mutual_us").unwrap();
    let mut values = HashMap::new();
    values.insert("party1_name".to_string(), "Acme Corp".to_string());
    let document = template.generate(&values);
    assert!(document.is_err());
    let errors = document.unwrap_err();
    assert!(errors.len() >= 3);
    assert!(
        errors.iter().any(|e| e.contains("party2_name")
            || e.contains("effective_date")
            || e.contains("state"))
    );
}
#[test]
fn test_document_template_invalid_variable_type() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let template = registry.get_template("employment_agreement_us").unwrap();
    let mut values = HashMap::new();
    values.insert("company_name".to_string(), "Tech Corp".to_string());
    values.insert("employee_name".to_string(), "Jane Doe".to_string());
    values.insert("position".to_string(), "Engineer".to_string());
    values.insert("start_date".to_string(), "2024-01-01".to_string());
    values.insert("salary".to_string(), "not-a-number".to_string());
    values.insert("state".to_string(), "CA".to_string());
    let document = template.generate(&values);
    assert!(document.is_err());
    let errors = document.unwrap_err();
    assert!(!errors.is_empty());
    assert!(errors.iter().any(|e| e.contains("salary")));
}
#[test]
fn test_document_template_registry_find_by_type() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let contracts = registry.find_by_type(DocumentTemplateType::Contract);
    assert!(contracts.len() >= 2);
    let court_docs = registry.find_by_type(DocumentTemplateType::CourtFiling);
    assert!(!court_docs.is_empty());
    let corporate_docs = registry.find_by_type(DocumentTemplateType::Corporate);
    assert!(!corporate_docs.is_empty());
}
#[test]
fn test_document_template_registry_find_by_jurisdiction() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let us_templates = registry.find_by_jurisdiction("US");
    assert!(us_templates.len() >= 3);
    let de_templates = registry.find_by_jurisdiction("US-DE");
    assert!(!de_templates.is_empty());
}
#[test]
fn test_document_template_registry_list_templates() {
    let registry = DocumentTemplateRegistry::with_defaults();
    let template_ids = registry.list_templates();
    assert!(template_ids.contains(&"nda_mutual_us"));
    assert!(template_ids.contains(&"employment_agreement_us"));
    assert!(template_ids.contains(&"complaint_us"));
    assert!(template_ids.contains(&"articles_incorporation_de"));
}
#[test]
fn test_document_template_custom() {
    let mut registry = DocumentTemplateRegistry::new();
    let custom_template = DocumentTemplate::new(
        "custom_nda_gb",
        "UK Non-Disclosure Agreement",
        DocumentTemplateType::Contract,
        Locale::new("en").with_country("GB"),
        "GB",
    )
    .add_variable(TemplateVariable::new(
        "party1",
        VariableType::Text,
        true,
        "First Party",
    ))
    .add_variable(TemplateVariable::new(
        "party2",
        VariableType::Text,
        true,
        "Second Party",
    ))
    .add_section(TemplateSection::new("title", "CONFIDENTIALITY AGREEMENT\n"))
    .add_section(TemplateSection::new(
        "parties",
        "This Agreement is made between {{party1}} and {{party2}}.\n",
    ))
    .add_metadata("jurisdiction", "England and Wales");
    registry.add_template(custom_template);
    let retrieved = registry.get_template("custom_nda_gb");
    assert!(retrieved.is_some());
    let template = retrieved.unwrap();
    assert_eq!(template.name, "UK Non-Disclosure Agreement");
    assert_eq!(template.jurisdiction, "GB");
    assert_eq!(
        template.metadata.get("jurisdiction"),
        Some(&"England and Wales".to_string())
    );
}
#[test]
fn test_variable_type_enum() {
    let types = [
        VariableType::Text,
        VariableType::Date,
        VariableType::Number,
        VariableType::Currency,
        VariableType::Boolean,
        VariableType::Email,
        VariableType::Address,
        VariableType::PersonName,
        VariableType::List,
    ];
    assert_eq!(types.len(), 9);
    assert_eq!(types[0], VariableType::Text);
    assert_eq!(types[1], VariableType::Date);
}
#[test]
fn test_document_template_type_enum() {
    let types = [
        DocumentTemplateType::Contract,
        DocumentTemplateType::CourtFiling,
        DocumentTemplateType::Corporate,
        DocumentTemplateType::Compliance,
        DocumentTemplateType::General,
    ];
    assert_eq!(types.len(), 5);
    assert_eq!(types[0], DocumentTemplateType::Contract);
    assert_ne!(types[0], types[1]);
}
#[test]
fn test_citation_parser_bluebook_case() {
    let parser = CitationParser::new(CitationStyle::Bluebook);
    let citation = "Brown v. Board of Education, 347 U.S. 483 (1954)";
    let result = parser.parse_case(citation);
    assert!(result.is_ok());
    let components = result.unwrap();
    assert_eq!(components.title, "Brown v. Board of Education");
    assert_eq!(components.volume, Some("347".to_string()));
    assert_eq!(components.reporter, Some("U.S.".to_string()));
    assert_eq!(components.page, Some("483".to_string()));
    assert_eq!(components.year, Some(1954));
}
#[test]
fn test_citation_parser_oscola_case() {
    let parser = CitationParser::new(CitationStyle::OSCOLA);
    let citation = "R v Smith [2020] EWCA Crim 123";
    let result = parser.parse_case(citation);
    assert!(result.is_ok());
    let components = result.unwrap();
    assert_eq!(components.title, "R v Smith");
    assert_eq!(components.year, Some(2020));
    assert_eq!(components.reporter, Some("EWCA".to_string()));
    assert_eq!(components.page, Some("Crim".to_string()));
}
#[test]
fn test_citation_parser_bluebook_statute() {
    let parser = CitationParser::new(CitationStyle::Bluebook);
    let citation = "42 U.S.C. § 1983";
    let result = parser.parse_statute(citation);
    assert!(result.is_ok());
    let components = result.unwrap();
    assert_eq!(components.title, "42 U.S.C. § 1983");
    assert_eq!(components.reporter, Some("42 U.S.C.".to_string()));
    assert_eq!(components.page, Some("1983".to_string()));
}
#[test]
fn test_citation_parser_oscola_statute() {
    let parser = CitationParser::new(CitationStyle::OSCOLA);
    let citation = "Human Rights Act 1998, s 3";
    let result = parser.parse_statute(citation);
    assert!(result.is_ok());
    let components = result.unwrap();
    assert_eq!(components.title, "Human Rights Act 1998, s 3");
    assert_eq!(components.year, Some(1998));
}
#[test]
fn test_citation_validator_bluebook_case_valid() {
    let validator = CitationValidator::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Brown v. Board of Education".to_string(),
        volume: Some("347".to_string()),
        reporter: Some("U.S.".to_string()),
        page: Some("483".to_string()),
        court: Some("Supreme Court".to_string()),
        year: Some(1954),
        jurisdiction: None,
    };
    let result = validator.validate_case(&components);
    assert!(result.is_ok());
}
#[test]
fn test_citation_validator_bluebook_case_missing_year() {
    let validator = CitationValidator::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Brown v. Board of Education".to_string(),
        volume: Some("347".to_string()),
        reporter: Some("U.S.".to_string()),
        page: Some("483".to_string()),
        court: None,
        year: None,
        jurisdiction: None,
    };
    let result = validator.validate_case(&components);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, CitationError::MissingField { field } if
        field == "year"))
    );
}
#[test]
fn test_citation_validator_oscola_case_valid() {
    let validator = CitationValidator::new(CitationStyle::OSCOLA);
    let components = CitationComponents {
        title: "R v Smith".to_string(),
        volume: None,
        reporter: Some("EWCA".to_string()),
        page: Some("Crim 123".to_string()),
        court: None,
        year: Some(2020),
        jurisdiction: None,
    };
    let result = validator.validate_case(&components);
    assert!(result.is_ok());
}
#[test]
fn test_citation_validator_bluebook_statute_valid() {
    let validator = CitationValidator::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "42 U.S.C.".to_string(),
        volume: None,
        reporter: None,
        page: Some("1983".to_string()),
        court: None,
        year: None,
        jurisdiction: None,
    };
    let result = validator.validate_statute(&components);
    assert!(result.is_ok());
}
#[test]
fn test_citation_normalizer_bluebook_to_oscola() {
    let normalizer = CitationNormalizer::new();
    let components = CitationComponents {
        title: "Brown v. Board of Education".to_string(),
        volume: Some("347".to_string()),
        reporter: Some("U.S.".to_string()),
        page: Some("483".to_string()),
        court: None,
        year: Some(1954),
        jurisdiction: None,
    };
    let result =
        normalizer.convert_case(&components, CitationStyle::Bluebook, CitationStyle::OSCOLA);
    assert!(result.is_ok());
    let converted = result.unwrap();
    assert!(converted.contains("[1954]"));
    assert!(converted.contains("Brown v. Board of Education"));
}
#[test]
fn test_citation_normalizer_parse_and_convert() {
    let normalizer = CitationNormalizer::new();
    let citation = "Brown v. Board of Education, 347 U.S. 483 (1954)";
    let result =
        normalizer.parse_and_convert_case(citation, CitationStyle::Bluebook, CitationStyle::OSCOLA);
    assert!(result.is_ok());
    let converted = result.unwrap();
    assert!(converted.contains("[1954]"));
}
#[test]
fn test_citation_completeness_checker_complete() {
    let checker = CitationCompletenessChecker::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Brown v. Board of Education".to_string(),
        volume: Some("347".to_string()),
        reporter: Some("U.S.".to_string()),
        page: Some("483".to_string()),
        court: Some("Supreme Court".to_string()),
        year: Some(1954),
        jurisdiction: None,
    };
    let report = checker.check_case(&components);
    assert!(report.is_complete());
    assert!(report.completeness_score > 80.0);
    assert!(report.missing_required.is_empty());
}
#[test]
fn test_citation_completeness_checker_incomplete() {
    let checker = CitationCompletenessChecker::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Case Name".to_string(),
        volume: None,
        reporter: None,
        page: None,
        court: None,
        year: None,
        jurisdiction: None,
    };
    let report = checker.check_case(&components);
    assert!(!report.is_complete());
    assert!(report.completeness_score < 50.0);
    assert!(!report.missing_required.is_empty());
    assert!(report.missing_required.contains(&"volume".to_string()));
    assert!(report.missing_required.contains(&"year".to_string()));
}
#[test]
fn test_citation_completeness_report_summary() {
    let checker = CitationCompletenessChecker::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Case Name".to_string(),
        volume: None,
        reporter: None,
        page: None,
        court: None,
        year: None,
        jurisdiction: None,
    };
    let report = checker.check_case(&components);
    let summary = report.summary();
    assert!(summary.contains("incomplete"));
    assert!(summary.contains("missing"));
}
#[test]
fn test_citation_suggester_bluebook_case() {
    let suggester = CitationSuggester::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Case Name".to_string(),
        volume: Some("123".to_string()),
        reporter: Some("F.3d".to_string()),
        page: None,
        court: None,
        year: None,
        jurisdiction: None,
    };
    let suggestions = suggester.suggest_case(&components);
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.contains("page")));
    assert!(suggestions.iter().any(|s| s.contains("year")));
}
#[test]
fn test_citation_suggester_oscola_case() {
    let suggester = CitationSuggester::new(CitationStyle::OSCOLA);
    let components = CitationComponents {
        title: "case name".to_string(),
        volume: None,
        reporter: Some("UKSC".to_string()),
        page: None,
        court: None,
        year: None,
        jurisdiction: None,
    };
    let suggestions = suggester.suggest_case(&components);
    assert!(!suggestions.is_empty());
    assert!(suggestions.iter().any(|s| s.contains("year")));
    assert!(suggestions.iter().any(|s| s.contains("capital")));
}
#[test]
fn test_citation_suggester_statute() {
    let suggester = CitationSuggester::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "42 U.S.C.".to_string(),
        volume: None,
        reporter: None,
        page: None,
        court: None,
        year: None,
        jurisdiction: None,
    };
    let suggestions = suggester.suggest_statute(&components);
    assert!(suggestions.iter().any(|s| s.contains("section")));
}
#[test]
fn test_citation_suggester_validate_and_suggest_case() {
    let suggester = CitationSuggester::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Brown v. Board of Education".to_string(),
        volume: Some("347".to_string()),
        reporter: Some("U.S.".to_string()),
        page: Some("483".to_string()),
        court: None,
        year: Some(1954),
        jurisdiction: None,
    };
    let report = suggester.validate_and_suggest_case(&components);
    assert!(report.is_valid);
    assert!(report.errors.is_empty());
    assert!(report.completeness.is_complete());
}
#[test]
fn test_citation_suggester_validate_and_suggest_invalid() {
    let suggester = CitationSuggester::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Case".to_string(),
        volume: None,
        reporter: None,
        page: None,
        court: None,
        year: None,
        jurisdiction: None,
    };
    let report = suggester.validate_and_suggest_case(&components);
    assert!(!report.is_valid);
    assert!(!report.errors.is_empty());
    assert!(!report.completeness.is_complete());
    assert!(!report.suggestions.is_empty());
}
#[test]
fn test_citation_suggester_style_for_jurisdiction() {
    assert_eq!(
        CitationSuggester::suggest_style_for_jurisdiction("US"),
        CitationStyle::Bluebook
    );
    assert_eq!(
        CitationSuggester::suggest_style_for_jurisdiction("GB"),
        CitationStyle::OSCOLA
    );
    assert_eq!(
        CitationSuggester::suggest_style_for_jurisdiction("AU"),
        CitationStyle::AGLC
    );
    assert_eq!(
        CitationSuggester::suggest_style_for_jurisdiction("CA"),
        CitationStyle::McGill
    );
    assert_eq!(
        CitationSuggester::suggest_style_for_jurisdiction("JP"),
        CitationStyle::Japanese
    );
    assert_eq!(
        CitationSuggester::suggest_style_for_jurisdiction("IN"),
        CitationStyle::Indian
    );
}
#[test]
fn test_validation_report_summary() {
    let suggester = CitationSuggester::new(CitationStyle::Bluebook);
    let components = CitationComponents {
        title: "Test".to_string(),
        volume: None,
        reporter: None,
        page: None,
        court: None,
        year: None,
        jurisdiction: None,
    };
    let report = suggester.validate_and_suggest_case(&components);
    let summary = report.summary();
    assert!(summary.contains("error"));
    assert!(summary.contains("incomplete"));
    assert!(summary.contains("Suggestions"));
}
#[test]
fn test_citation_validation_rule_required() {
    let rule = CitationValidationRule::required("title");
    assert!(rule.required);
    assert_eq!(rule.field, "title");
    let result = rule.validate(None);
    assert!(result.is_err());
    let result = rule.validate(Some(&"Test".to_string()));
    assert!(result.is_ok());
}
#[test]
fn test_citation_validation_rule_optional() {
    let rule = CitationValidationRule::optional("court");
    assert!(!rule.required);
    let result = rule.validate(None);
    assert!(result.is_ok());
    let result = rule.validate(Some(&"Supreme Court".to_string()));
    assert!(result.is_ok());
}
#[test]
fn test_citation_validation_rule_pattern_numeric() {
    let rule = CitationValidationRule::required("volume").with_pattern("numeric");
    let result = rule.validate(Some(&"123".to_string()));
    assert!(result.is_ok());
    let result = rule.validate(Some(&"abc".to_string()));
    assert!(result.is_err());
}
#[test]
fn test_citation_validation_rule_pattern_year() {
    let rule = CitationValidationRule::required("year").with_pattern("year");
    let result = rule.validate(Some(&"2020".to_string()));
    assert!(result.is_ok());
    let result = rule.validate(Some(&"999".to_string()));
    assert!(result.is_err());
    let result = rule.validate(Some(&"10000".to_string()));
    assert!(result.is_err());
}
#[test]
fn test_citation_error_display() {
    let error = CitationError::MissingField {
        field: "year".to_string(),
    };
    assert!(error.to_string().contains("Missing required field"));
    assert!(error.to_string().contains("year"));
    let error = CitationError::InvalidFormat {
        field: "volume".to_string(),
        reason: "Not numeric".to_string(),
    };
    assert!(error.to_string().contains("Invalid format"));
    assert!(error.to_string().contains("volume"));
    let error = CitationError::ParseError {
        reason: "Empty citation".to_string(),
    };
    assert!(error.to_string().contains("Failed to parse"));
}
#[test]
fn test_citation_type_enum() {
    let types = [
        CitationType::Case,
        CitationType::Statute,
        CitationType::Article,
        CitationType::Book,
    ];
    assert_eq!(types.len(), 4);
    assert_eq!(types[0], CitationType::Case);
    assert_ne!(types[0], types[1]);
}
#[test]
fn test_citation_parser_empty_citation() {
    let parser = CitationParser::new(CitationStyle::Bluebook);
    let result = parser.parse_case("");
    assert!(result.is_err());
    if let Err(CitationError::ParseError { reason }) = result {
        assert!(reason.contains("Empty"));
    } else {
        panic!("Expected ParseError");
    }
}
#[test]
fn test_citation_normalizer_default() {
    let normalizer = CitationNormalizer::default();
    let components = CitationComponents::new("Test");
    let _ = normalizer.convert_case(&components, CitationStyle::Bluebook, CitationStyle::OSCOLA);
}
#[test]
fn test_completeness_report_is_complete() {
    let report = CompletenessReport {
        citation_type: CitationType::Case,
        style: CitationStyle::Bluebook,
        completeness_score: 100.0,
        missing_required: vec![],
        missing_optional: vec!["court".to_string()],
        present: vec!["title".to_string(), "year".to_string()],
    };
    assert!(report.is_complete());
    let report = CompletenessReport {
        citation_type: CitationType::Case,
        style: CitationStyle::Bluebook,
        completeness_score: 50.0,
        missing_required: vec!["year".to_string()],
        missing_optional: vec![],
        present: vec!["title".to_string()],
    };
    assert!(!report.is_complete());
}
#[test]
fn test_clause_extractor_confidentiality() {
    let extractor = ClauseExtractor::with_defaults();
    let text = "This agreement contains confidential information that must be protected.";
    let clauses = extractor.extract(text);
    assert!(!clauses.is_empty());
    assert!(
        clauses
            .iter()
            .any(|c| c.clause_type == ClauseType::Confidentiality)
    );
}
#[test]
fn test_clause_extractor_multiple_types() {
    let extractor = ClauseExtractor::with_defaults();
    let text = "The parties agree to indemnify each other. This agreement is governed by the laws of Delaware. Termination may occur with 30 days notice.";
    let clauses = extractor.extract(text);
    assert!(clauses.len() >= 3);
}
#[test]
fn test_clause_extractor_custom_pattern() {
    let mut extractor = ClauseExtractor::new();
    extractor.add_pattern(ClauseType::Custom("Test".to_string()), "test clause");
    let text = "This is a test clause for testing.";
    let clauses = extractor.extract(text);
    assert!(!clauses.is_empty());
}
#[test]
fn test_party_identifier_basic() {
    let identifier = PartyIdentifier::with_defaults();
    let text = "This agreement is between Acme Corporation and Beta LLC.";
    let parties = identifier.identify(text);
    assert!(!parties.is_empty());
}
#[test]
fn test_party_identifier_roles() {
    let identifier = PartyIdentifier::with_defaults();
    let text =
        "The party of the first part, John Smith, and the party of the second part, Jane Doe.";
    let parties = identifier.identify(text);
    assert!(parties.iter().any(|p| p.role == PartyRole::FirstParty));
}
#[test]
fn test_obligation_extractor_mandatory() {
    let extractor = ObligationExtractor::new();
    let text =
        "The Seller shall deliver the goods within 30 days. The Buyer must pay upon delivery.";
    let obligations = extractor.extract(text);
    assert!(!obligations.is_empty());
    assert!(
        obligations
            .iter()
            .any(|o| o.obligation_type == ObligationType::Mandatory)
    );
}
#[test]
fn test_obligation_extractor_prohibition() {
    let extractor = ObligationExtractor::new();
    let text = "The Licensee shall not sublicense the software. The parties must not disclose.";
    let obligations = extractor.extract(text);
    assert!(!obligations.is_empty());
}
#[test]
fn test_obligation_extractor_permissive() {
    let extractor = ObligationExtractor::new();
    let text = "The Tenant may renew the lease for an additional year.";
    let obligations = extractor.extract(text);
    assert!(
        obligations
            .iter()
            .any(|o| o.obligation_type == ObligationType::Permissive)
    );
}
#[test]
fn test_deadline_extractor_date_format() {
    let extractor = DeadlineExtractor::new();
    let text =
        "Payment is due by 12/31/2024. All deliverables must be completed before 06/15/2025.";
    let deadlines = extractor.extract(text);
    assert!(!deadlines.is_empty());
}
#[test]
fn test_deadline_extractor_keywords() {
    let extractor = DeadlineExtractor::new();
    let text = "The deadline for submission is within 30 days of receipt.";
    let deadlines = extractor.extract(text);
    assert!(!deadlines.is_empty());
}
#[test]
fn test_jurisdiction_detector_us() {
    let detector = JurisdictionDetector::with_defaults();
    let text =
        "This agreement shall be governed by the laws of the State of New York, United States.";
    let result = detector.detect(text);
    assert!(result.is_some());
    let (jurisdiction, _confidence) = result.unwrap();
    assert_eq!(jurisdiction, "US");
}
#[test]
fn test_jurisdiction_detector_multiple_indicators() {
    let detector = JurisdictionDetector::with_defaults();
    let text =
        "This agreement references Delaware corporate law and the United States Supreme Court.";
    let result = detector.detect(text);
    assert!(result.is_some());
}
#[test]
fn test_jurisdiction_detector_uk() {
    let detector = JurisdictionDetector::with_defaults();
    let text = "This contract is governed by English law of England and Wales.";
    let result = detector.detect(text);
    assert!(result.is_some());
    let (jurisdiction, _) = result.unwrap();
    assert_eq!(jurisdiction, "GB");
}
#[test]
fn test_legal_risk_scorer_critical() {
    let scorer = LegalRiskScorer::with_defaults();
    let text =
        "The parties agree to unlimited liability with no limitation of liability whatsoever.";
    let (risk_level, factors) = scorer.score(text);
    assert!(risk_level >= RiskLevel::High);
    assert!(!factors.is_empty());
}
#[test]
fn test_legal_risk_scorer_medium() {
    let scorer = LegalRiskScorer::with_defaults();
    let text = "This product is sold as-is with no warranty. All sales are non-refundable.";
    let (risk_level, _) = scorer.score(text);
    assert!(risk_level >= RiskLevel::Medium);
}
#[test]
fn test_legal_risk_scorer_low() {
    let scorer = LegalRiskScorer::with_defaults();
    let text = "Seller provides indemnification and maintains insurance. Liability is limited to contract value.";
    let (risk_level, _) = scorer.score(text);
    assert!(risk_level <= RiskLevel::Medium);
}
#[test]
fn test_legal_risk_scorer_mitigation() {
    let scorer = LegalRiskScorer::with_defaults();
    let text = "This contract includes unlimited liability provisions.";
    let (_risk_level, factors) = scorer.score(text);
    assert!(factors.iter().any(|f| f.mitigation.is_some()));
}
#[test]
fn test_legal_document_analyzer_comprehensive() {
    let analyzer = LegalDocumentAnalyzer::new();
    let text = "This Mutual Non-Disclosure Agreement is between Acme Corp and Beta LLC. \
               The parties shall maintain confidentiality of all proprietary information. \
               The Recipient shall not disclose any confidential information to third parties. \
               This agreement is governed by the laws of Delaware, United States. \
               Payment is due by 12/31/2024. \
               The agreement includes indemnification provisions.";
    let analysis = analyzer.analyze(text);
    assert!(!analysis.clauses.is_empty(), "Should extract clauses");
    assert!(!analysis.parties.is_empty(), "Should identify parties");
    assert!(
        !analysis.obligations.is_empty(),
        "Should extract obligations"
    );
    assert!(!analysis.deadlines.is_empty(), "Should extract deadlines");
    assert!(
        analysis.jurisdiction.is_some(),
        "Should detect jurisdiction"
    );
}
#[test]
fn test_document_analysis_clauses() {
    let analyzer = LegalDocumentAnalyzer::new();
    let text = "This agreement contains confidential information. The parties agree to indemnify each other.";
    let analysis = analyzer.analyze(text);
    assert!(
        analysis
            .clauses
            .iter()
            .any(|c| matches!(c.clause_type, ClauseType::Confidentiality))
    );
    assert!(
        analysis
            .clauses
            .iter()
            .any(|c| matches!(c.clause_type, ClauseType::Indemnification))
    );
}
#[test]
fn test_document_analysis_risk_assessment() {
    let analyzer = LegalDocumentAnalyzer::new();
    let high_risk_text =
        "This agreement includes unlimited liability and personal guarantee clauses.";
    let low_risk_text =
        "This agreement includes limitation of liability and insurance requirements.";
    let high_risk_analysis = analyzer.analyze(high_risk_text);
    let low_risk_analysis = analyzer.analyze(low_risk_text);
    assert!(high_risk_analysis.risk_level >= RiskLevel::High);
    assert!(low_risk_analysis.risk_level <= RiskLevel::Medium);
}
#[test]
fn test_clause_type_display() {
    assert_eq!(ClauseType::Confidentiality.to_string(), "Confidentiality");
    assert_eq!(
        ClauseType::LimitationOfLiability.to_string(),
        "Limitation of Liability"
    );
    assert_eq!(ClauseType::GoverningLaw.to_string(), "Governing Law");
    assert_eq!(
        ClauseType::Custom("MyClause".to_string()).to_string(),
        "MyClause"
    );
}
#[test]
fn test_risk_level_display() {
    assert_eq!(RiskLevel::Low.to_string(), "Low");
    assert_eq!(RiskLevel::Medium.to_string(), "Medium");
    assert_eq!(RiskLevel::High.to_string(), "High");
    assert_eq!(RiskLevel::Critical.to_string(), "Critical");
}
#[test]
fn test_risk_level_ordering() {
    assert!(RiskLevel::Low < RiskLevel::Medium);
    assert!(RiskLevel::Medium < RiskLevel::High);
    assert!(RiskLevel::High < RiskLevel::Critical);
}
#[test]
fn test_obligation_type_variants() {
    let mandatory = ObligationType::Mandatory;
    let permissive = ObligationType::Permissive;
    let prohibition = ObligationType::Prohibition;
    let recommendation = ObligationType::Recommendation;
    assert_ne!(mandatory, permissive);
    assert_ne!(mandatory, prohibition);
    assert_ne!(permissive, recommendation);
}
#[test]
fn test_party_role_variants() {
    let first = PartyRole::FirstParty;
    let second = PartyRole::SecondParty;
    let plaintiff = PartyRole::Plaintiff;
    let defendant = PartyRole::Defendant;
    assert_ne!(first, second);
    assert_ne!(plaintiff, defendant);
}
#[test]
fn test_extracted_clause_confidence() {
    let extractor = ClauseExtractor::with_defaults();
    let text = "The parties shall hereby maintain confidentiality pursuant to this agreement.";
    let clauses = extractor.extract(text);
    assert!(clauses.iter().any(|c| c.confidence > 0.5));
}
#[test]
fn test_deadline_extractor_with_reference_date() {
    let extractor = DeadlineExtractor::new().with_reference_date(2024, 1, 1);
    let text = "Delivery is due by 12/31/2024.";
    let deadlines = extractor.extract(text);
    assert!(!deadlines.is_empty());
}
#[test]
fn test_custom_jurisdiction_indicator() {
    let mut detector = JurisdictionDetector::new();
    detector.add_indicator("CUSTOM", "custom jurisdiction");
    let text = "This agreement is under custom jurisdiction rules.";
    let result = detector.detect(text);
    assert!(result.is_some());
    let (jurisdiction, _) = result.unwrap();
    assert_eq!(jurisdiction, "CUSTOM");
}
#[test]
fn test_custom_risk_indicator() {
    let mut scorer = LegalRiskScorer::new();
    scorer.add_indicator("dangerous clause", RiskLevel::Critical);
    scorer.add_indicator("very risky term", RiskLevel::Critical);
    scorer.add_indicator("extreme hazard", RiskLevel::Critical);
    let text = "This contract contains a dangerous clause and very risky term with extreme hazard.";
    let (risk_level, factors) = scorer.score(text);
    assert_eq!(risk_level, RiskLevel::Critical);
    assert!(!factors.is_empty());
}
#[test]
fn test_analyzer_mutable_access() {
    let mut analyzer = LegalDocumentAnalyzer::new();
    analyzer
        .clause_extractor_mut()
        .add_pattern(ClauseType::Custom("Test".to_string()), "test pattern");
    analyzer
        .jurisdiction_detector_mut()
        .add_indicator("TEST", "test jurisdiction");
    analyzer
        .risk_scorer_mut()
        .add_indicator("test risk", RiskLevel::High);
    let text = "This test pattern is in test jurisdiction with test risk.";
    let analysis = analyzer.analyze(text);
    assert!(analysis.jurisdiction.is_some());
}
