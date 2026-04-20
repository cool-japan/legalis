//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types_3::TemplateVariable;
use super::types_4::HistoricalPeriod;
use super::types_6::{LanguageFamily, LegalConcept};
use super::types_8::LegalDictionary;
use super::types_9::EUMemberStateVariation;
use super::types_10::{DocumentTemplateType, LocalCustom, Locale, ReligiousLawType};
use super::types_12::{ReligiousLawSystem, SearchResult};
use super::types_13::{CustomType, LegalExtensionType, TemplateSection};

/// Term index for fast prefix-based lookups in dictionaries.
/// Enables efficient autocomplete, fuzzy search, and partial matching.
#[derive(Debug, Clone, Default)]
pub struct TermIndex {
    /// Prefix map: prefix -> list of full terms
    pub(super) prefix_map: HashMap<String, Vec<String>>,
    /// Minimum prefix length for indexing
    min_prefix_len: usize,
}
impl TermIndex {
    /// Creates a new term index.
    pub fn new() -> Self {
        Self {
            prefix_map: HashMap::new(),
            min_prefix_len: 2,
        }
    }
    /// Creates a term index with custom minimum prefix length.
    pub fn with_min_prefix_len(min_len: usize) -> Self {
        Self {
            prefix_map: HashMap::new(),
            min_prefix_len: min_len.max(1),
        }
    }
    /// Indexes a term for fast prefix lookups.
    pub fn index_term(&mut self, term: impl Into<String>) {
        let term_str = term.into();
        let term_lower = term_str.to_lowercase();
        for len in self.min_prefix_len..=term_lower.len() {
            if let Some(prefix) = term_lower.get(0..len) {
                self.prefix_map
                    .entry(prefix.to_string())
                    .or_default()
                    .push(term_str.clone());
            }
        }
    }
    /// Finds all terms matching a prefix.
    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&str> {
        let prefix_lower = prefix.to_lowercase();
        self.prefix_map
            .get(&prefix_lower)
            .map(|terms| terms.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    /// Clears all indexed terms.
    pub fn clear(&mut self) {
        self.prefix_map.clear();
    }
    /// Returns the number of unique prefixes indexed.
    pub fn prefix_count(&self) -> usize {
        self.prefix_map.len()
    }
}
/// Types of legal clauses found in documents.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClauseType {
    /// Confidentiality/NDA clause
    Confidentiality,
    /// Indemnification clause
    Indemnification,
    /// Limitation of liability
    LimitationOfLiability,
    /// Termination clause
    Termination,
    /// Governing law clause
    GoverningLaw,
    /// Dispute resolution clause
    DisputeResolution,
    /// Force majeure clause
    ForceMajeure,
    /// Warranty clause
    Warranty,
    /// Payment terms
    Payment,
    /// Intellectual property clause
    IntellectualProperty,
    /// Non-compete clause
    NonCompete,
    /// Assignment clause
    Assignment,
    /// Severability clause
    Severability,
    /// Entire agreement clause
    EntireAgreement,
    /// Amendment clause
    Amendment,
    /// Notice clause
    Notice,
    /// Custom clause type
    Custom(String),
}
/// Etymology information for a legal term.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Etymology {
    /// The modern term
    pub term: String,
    /// Original term or root
    pub root: String,
    /// Language family of origin
    pub language_family: LanguageFamily,
    /// Original language
    pub original_language: String,
    /// Meaning of the root
    pub root_meaning: String,
    /// Historical period of first usage
    pub first_usage: Option<HistoricalPeriod>,
    /// Evolution of the term through time
    pub evolution: Vec<String>,
}
impl Etymology {
    /// Creates a new etymology.
    pub fn new(
        term: impl Into<String>,
        root: impl Into<String>,
        language_family: LanguageFamily,
        original_language: impl Into<String>,
        root_meaning: impl Into<String>,
    ) -> Self {
        Self {
            term: term.into(),
            root: root.into(),
            language_family,
            original_language: original_language.into(),
            root_meaning: root_meaning.into(),
            first_usage: None,
            evolution: Vec::new(),
        }
    }
    /// Adds first usage period.
    pub fn with_first_usage(mut self, period: HistoricalPeriod) -> Self {
        self.first_usage = Some(period);
        self
    }
    /// Adds evolution step.
    pub fn add_evolution(mut self, evolution_step: impl Into<String>) -> Self {
        self.evolution.push(evolution_step.into());
        self
    }
}
/// Legal document template with placeholders and localization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTemplate {
    /// Template identifier
    pub id: String,
    /// Template name
    pub name: String,
    /// Template type
    pub template_type: DocumentTemplateType,
    /// Locale for this template
    pub locale: Locale,
    /// Jurisdiction code (e.g., "US", "GB", "FR")
    pub jurisdiction: String,
    /// Template sections
    pub sections: Vec<TemplateSection>,
    /// Required variables
    pub variables: Vec<TemplateVariable>,
    /// Template metadata
    pub metadata: HashMap<String, String>,
}
impl DocumentTemplate {
    /// Creates a new document template.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        template_type: DocumentTemplateType,
        locale: Locale,
        jurisdiction: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            template_type,
            locale,
            jurisdiction: jurisdiction.into(),
            sections: vec![],
            variables: vec![],
            metadata: HashMap::new(),
        }
    }
    /// Adds a section to the template.
    pub fn add_section(mut self, section: TemplateSection) -> Self {
        self.sections.push(section);
        self
    }
    /// Adds a variable to the template.
    pub fn add_variable(mut self, variable: TemplateVariable) -> Self {
        self.variables.push(variable);
        self
    }
    /// Adds metadata to the template.
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    /// Validates that all required variables are provided.
    pub fn validate_variables(&self, values: &HashMap<String, String>) -> Vec<String> {
        let mut missing = vec![];
        for var in &self.variables {
            if var.required {
                if let Some(value) = values.get(&var.name) {
                    if !var.validate(value) {
                        missing.push(format!(
                            "Invalid value for '{}': expected {:?}",
                            var.name, var.var_type
                        ));
                    }
                } else if var.default_value.is_none() {
                    missing.push(format!("Missing required variable: '{}'", var.name));
                }
            }
        }
        missing
    }
    /// Generates the document by filling in the template with provided values.
    pub fn generate(&self, values: &HashMap<String, String>) -> Result<String, Vec<String>> {
        let errors = self.validate_variables(values);
        if !errors.is_empty() {
            return Err(errors);
        }
        let mut document = String::new();
        for section in &self.sections {
            if !section.should_include(values) {
                continue;
            }
            let mut content = section.content.clone();
            for var in &self.variables {
                let placeholder = format!("{{{{{}}}}}", var.name);
                let value = values
                    .get(&var.name)
                    .or(var.default_value.as_ref())
                    .map(|s| s.as_str())
                    .unwrap_or("");
                content = content.replace(&placeholder, value);
            }
            document.push_str(&content);
            document.push('\n');
        }
        Ok(document)
    }
}
/// Local law terminology entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalLawTerm {
    /// Term in local language.
    pub local_term: String,
    /// English translation.
    pub english_equiv: String,
    /// Legal system context.
    pub legal_system: String,
    /// Jurisdiction.
    pub jurisdiction: String,
    /// Definition in local language.
    pub definition: String,
    /// Usage examples.
    pub examples: Vec<String>,
    /// Related statutes or codes.
    pub related_statutes: Vec<String>,
}
impl LocalLawTerm {
    /// Creates a new local law term.
    pub fn new(
        local_term: impl Into<String>,
        english_equiv: impl Into<String>,
        legal_system: impl Into<String>,
        jurisdiction: impl Into<String>,
        definition: impl Into<String>,
    ) -> Self {
        Self {
            local_term: local_term.into(),
            english_equiv: english_equiv.into(),
            legal_system: legal_system.into(),
            jurisdiction: jurisdiction.into(),
            definition: definition.into(),
            examples: Vec::new(),
            related_statutes: Vec::new(),
        }
    }
    /// Adds an example.
    pub fn add_example(mut self, example: impl Into<String>) -> Self {
        self.examples.push(example.into());
        self
    }
    /// Adds a related statute.
    pub fn add_statute(mut self, statute: impl Into<String>) -> Self {
        self.related_statutes.push(statute.into());
        self
    }
}
/// Braille grade (complexity level).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrailleGrade {
    /// Grade 1: Uncontracted Braille (letter-for-letter)
    Grade1,
    /// Grade 2: Contracted Braille (with abbreviations)
    Grade2,
}
/// Recognized legal entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalEntity {
    /// Entity text as it appears in document
    pub text: String,
    /// Type of entity
    pub entity_type: LegalEntityType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Position in document (character offset)
    pub position: usize,
    /// Normalized form (canonical name)
    pub normalized: Option<String>,
}
impl LegalEntity {
    /// Creates a new legal entity.
    pub fn new(text: impl Into<String>, entity_type: LegalEntityType, position: usize) -> Self {
        Self {
            text: text.into(),
            entity_type,
            confidence: 1.0,
            position,
            normalized: None,
        }
    }
    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
    /// Sets the normalized form.
    pub fn with_normalized(mut self, normalized: impl Into<String>) -> Self {
        self.normalized = Some(normalized.into());
        self
    }
}
/// Unicode CLDR legal extension.
#[derive(Debug, Clone)]
pub struct LegalExtension {
    /// The extension type.
    pub extension_type: LegalExtensionType,
    /// The extension value (e.g., "common", "civil", "bluebook").
    pub value: String,
}
impl LegalExtension {
    /// Creates a new legal extension.
    pub fn new(extension_type: LegalExtensionType, value: &str) -> Self {
        Self {
            extension_type,
            value: value.to_string(),
        }
    }
    /// Formats the extension as a BCP 47 extension string.
    pub fn to_bcp47_extension(&self) -> String {
        match self.extension_type {
            LegalExtensionType::LegalSystem => format!("u-legal-{}", self.value),
            LegalExtensionType::CitationStyle => format!("u-cite-{}", self.value),
            LegalExtensionType::CourtType => format!("u-court-{}", self.value),
            LegalExtensionType::FormalityLevel => format!("u-formality-{}", self.value),
        }
    }
    /// Creates a LegalSystem extension.
    pub fn legal_system(system: &str) -> Self {
        Self::new(LegalExtensionType::LegalSystem, system)
    }
    /// Creates a CitationStyle extension.
    pub fn citation_style(style: &str) -> Self {
        Self::new(LegalExtensionType::CitationStyle, style)
    }
    /// Creates a CourtType extension.
    pub fn court_type(court: &str) -> Self {
        Self::new(LegalExtensionType::CourtType, court)
    }
    /// Creates a FormalityLevel extension.
    pub fn formality_level(level: &str) -> Self {
        Self::new(LegalExtensionType::FormalityLevel, level)
    }
}
/// Religious law registry.
#[derive(Debug, Clone, Default)]
pub struct ReligiousLawRegistry {
    /// Systems indexed by type
    pub(super) systems: HashMap<ReligiousLawType, ReligiousLawSystem>,
}
impl ReligiousLawRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with default systems.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        let mut islamic = ReligiousLawSystem::islamic();
        islamic.add_jurisdiction("Saudi Arabia");
        islamic.add_jurisdiction("Iran");
        islamic.add_jurisdiction("Pakistan");
        islamic.add_jurisdiction("UAE");
        registry.add_system(islamic);
        let mut jewish = ReligiousLawSystem::jewish();
        jewish.add_jurisdiction("Israel");
        registry.add_system(jewish);
        let mut hindu = ReligiousLawSystem::hindu();
        hindu.add_jurisdiction("India");
        hindu.add_jurisdiction("Nepal");
        registry.add_system(hindu);
        registry
    }
    /// Adds a religious law system.
    pub fn add_system(&mut self, system: ReligiousLawSystem) {
        self.systems.insert(system.law_type, system);
    }
    /// Gets a system by type.
    pub fn get_system(&self, law_type: ReligiousLawType) -> Option<&ReligiousLawSystem> {
        self.systems.get(&law_type)
    }
    /// Gets all systems for a jurisdiction.
    pub fn get_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&ReligiousLawSystem> {
        self.systems
            .values()
            .filter(|s| s.jurisdictions.iter().any(|j| j == jurisdiction))
            .collect()
    }
    /// Returns the number of systems.
    pub fn system_count(&self) -> usize {
        self.systems.len()
    }
}
/// Etymology tracker for legal terms.
#[derive(Debug, Clone)]
pub struct EtymologyTracker {
    /// Etymologies indexed by term
    etymologies: HashMap<String, Etymology>,
}
impl EtymologyTracker {
    /// Creates a new etymology tracker.
    pub fn new() -> Self {
        Self {
            etymologies: HashMap::new(),
        }
    }
    /// Creates a tracker with default legal term etymologies.
    pub fn with_defaults() -> Self {
        let mut tracker = Self::new();
        tracker.add_etymology(
            Etymology::new(
                "contract",
                "contractus",
                LanguageFamily::Latin,
                "Latin",
                "drawn together, agreed upon",
            )
            .with_first_usage(HistoricalPeriod::ClassicalLatin)
            .add_evolution("Latin contractus → Old French contract → Middle English contract"),
        );
        tracker.add_etymology(
            Etymology::new(
                "tort",
                "tortus",
                LanguageFamily::Latin,
                "Latin",
                "twisted, wrong",
            )
            .with_first_usage(HistoricalPeriod::MedievalLatin)
            .add_evolution("Latin tortus → Old French tort → Middle English tort"),
        );
        tracker.add_etymology(
            Etymology::new(
                "jury",
                "jurata",
                LanguageFamily::Latin,
                "Latin",
                "sworn (group)",
            )
            .with_first_usage(HistoricalPeriod::MedievalLatin)
            .add_evolution("Latin jurata → Old French juree → Middle English jury"),
        );
        tracker.add_etymology(
            Etymology::new(
                "attorney",
                "atorner",
                LanguageFamily::OldFrench,
                "Old French",
                "to turn over, assign",
            )
            .with_first_usage(HistoricalPeriod::MiddleEnglish)
            .add_evolution("Old French atorner → Anglo-Norman atourne → Middle English attorney"),
        );
        tracker.add_etymology(
            Etymology::new(
                "mortgage",
                "mort + gage",
                LanguageFamily::OldFrench,
                "Old French",
                "dead pledge",
            )
            .with_first_usage(HistoricalPeriod::MiddleEnglish)
            .add_evolution("Old French mort (dead) + gage (pledge) → Middle English mortgage"),
        );
        tracker.add_etymology(
            Etymology::new(
                "habeas corpus",
                "habeas corpus",
                LanguageFamily::Latin,
                "Latin",
                "you shall have the body",
            )
            .with_first_usage(HistoricalPeriod::MedievalLatin)
            .add_evolution("Latin legal phrase preserved in English common law"),
        );
        tracker.add_etymology(
            Etymology::new(
                "bailiff",
                "baillif",
                LanguageFamily::NormanFrench,
                "Norman French",
                "administrator, manager",
            )
            .with_first_usage(HistoricalPeriod::MiddleEnglish)
            .add_evolution(
                "Norman French baillif → Middle English bailif → Modern English bailiff",
            ),
        );
        tracker.add_etymology(
            Etymology::new(
                "equity",
                "aequitas",
                LanguageFamily::Latin,
                "Latin",
                "fairness, equality",
            )
            .with_first_usage(HistoricalPeriod::ClassicalLatin)
            .add_evolution("Latin aequitas → Old French equite → Middle English equity"),
        );
        tracker
    }
    /// Adds an etymology.
    pub fn add_etymology(&mut self, etymology: Etymology) {
        self.etymologies.insert(etymology.term.clone(), etymology);
    }
    /// Gets etymology for a term.
    pub fn get_etymology(&self, term: &str) -> Option<&Etymology> {
        self.etymologies.get(term)
    }
    /// Gets all etymologies by language family.
    pub fn get_by_language_family(&self, family: LanguageFamily) -> Vec<&Etymology> {
        self.etymologies
            .values()
            .filter(|e| e.language_family == family)
            .collect()
    }
    /// Returns the number of tracked etymologies.
    pub fn etymology_count(&self) -> usize {
        self.etymologies.len()
    }
}
/// EU regulation type for language alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EURegulationType {
    /// GDPR (General Data Protection Regulation).
    GDPR,
    /// MiFID II (Markets in Financial Instruments Directive).
    MiFIDII,
    /// REACH (Registration, Evaluation, Authorization of Chemicals).
    REACH,
    /// EUDataAct (EU Data Act).
    EUDataAct,
    /// DigitalMarketsAct (DMA).
    DigitalMarketsAct,
    /// DigitalServicesAct (DSA).
    DigitalServicesAct,
    /// AIAct (Artificial Intelligence Act).
    AIAct,
    /// Custom regulation.
    Custom,
}
/// W3C compliance report.
#[derive(Debug, Clone)]
pub struct W3CComplianceReport {
    /// The locale that was checked.
    pub locale: Locale,
    /// Whether the locale is W3C compliant.
    pub is_compliant: bool,
    /// List of compliance issues.
    pub issues: Vec<String>,
    /// Recommended HTML lang attribute.
    pub lang_attribute: String,
    /// Recommended HTML dir attribute.
    pub dir_attribute: String,
}
impl W3CComplianceReport {
    /// Gets a summary of the compliance check.
    pub fn summary(&self) -> String {
        if self.is_compliant {
            format!("Locale '{}' is W3C compliant", self.locale)
        } else {
            format!(
                "Locale '{}' has {} compliance issue(s): {}",
                self.locale,
                self.issues.len(),
                self.issues.join(", ")
            )
        }
    }
}
/// Legal entity types for recognition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalEntityType {
    /// Court (e.g., "Supreme Court", "Court of Appeals")
    Court,
    /// Company/Corporation (e.g., "Apple Inc.", "Google LLC")
    Company,
    /// Statute/Law (e.g., "Civil Rights Act of 1964")
    Statute,
    /// Legal person (individual in legal context)
    Person,
    /// Government agency (e.g., "SEC", "FTC")
    GovernmentAgency,
    /// Law firm
    LawFirm,
    /// Other entity type
    Other(String),
}
/// Legal domain specializations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalDomain {
    /// General legal terms
    General,
    /// Intellectual Property Law
    IntellectualProperty,
    /// Tax Law
    Tax,
    /// Environmental Law
    Environmental,
    /// Labor and Employment Law
    Labor,
    /// Corporate Law
    Corporate,
    /// Criminal Law
    Criminal,
    /// Civil Procedure
    CivilProcedure,
}
impl LegalDomain {
    /// Creates a specialized dictionary for a given domain and locale.
    pub fn create_dictionary(&self, locale: Locale) -> LegalDictionary {
        let mut dict = LegalDictionary::new(locale.clone());
        match self {
            LegalDomain::General => dict,
            LegalDomain::IntellectualProperty => {
                self.add_ip_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Tax => {
                self.add_tax_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Environmental => {
                self.add_environmental_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Labor => {
                self.add_labor_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Corporate => {
                self.add_corporate_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::Criminal => {
                self.add_criminal_terms(&mut dict, &locale);
                dict
            }
            LegalDomain::CivilProcedure => {
                self.add_civil_procedure_terms(&mut dict, &locale);
                dict
            }
        }
    }
    fn add_ip_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("patent", "patent");
                dict.add_translation("trademark", "trademark");
                dict.add_translation("copyright", "copyright");
                dict.add_translation("trade_secret", "trade secret");
                dict.add_translation("intellectual_property", "intellectual property");
                dict.add_translation("infringement", "infringement");
                dict.add_translation("prior_art", "prior art");
                dict.add_translation("novelty", "novelty");
                dict.add_translation("non_obviousness", "non-obviousness");
                dict.add_translation("fair_use", "fair use");
                dict.add_translation("licensing", "licensing");
                dict.add_translation("royalty", "royalty");
                dict.add_translation("utility_patent", "utility patent");
                dict.add_translation("design_patent", "design patent");
                dict.add_abbreviation("patent", "Pat.");
                dict.add_abbreviation("trademark", "TM");
                dict.add_abbreviation("copyright", "©");
            }
            "ja" => {
                dict.add_translation("patent", "特許");
                dict.add_translation("trademark", "商標");
                dict.add_translation("copyright", "著作権");
                dict.add_translation("trade_secret", "営業秘密");
                dict.add_translation("intellectual_property", "知的財産権");
                dict.add_translation("infringement", "侵害");
                dict.add_translation("prior_art", "先行技術");
                dict.add_translation("novelty", "新規性");
                dict.add_translation("licensing", "ライセンス");
                dict.add_translation("royalty", "ロイヤルティ");
            }
            "de" => {
                dict.add_translation("patent", "Patent");
                dict.add_translation("trademark", "Marke");
                dict.add_translation("copyright", "Urheberrecht");
                dict.add_translation("intellectual_property", "geistiges Eigentum");
                dict.add_translation("infringement", "Verletzung");
            }
            _ => {}
        }
    }
    fn add_tax_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("income_tax", "income tax");
                dict.add_translation("corporate_tax", "corporate tax");
                dict.add_translation("value_added_tax", "value-added tax");
                dict.add_translation("capital_gains", "capital gains");
                dict.add_translation("deduction", "deduction");
                dict.add_translation("exemption", "exemption");
                dict.add_translation("tax_liability", "tax liability");
                dict.add_translation("withholding_tax", "withholding tax");
                dict.add_translation("tax_credit", "tax credit");
                dict.add_translation("taxable_income", "taxable income");
                dict.add_translation("tax_evasion", "tax evasion");
                dict.add_translation("tax_avoidance", "tax avoidance");
                dict.add_translation("fiscal_year", "fiscal year");
                dict.add_abbreviation("value_added_tax", "VAT");
                dict.add_abbreviation("income_tax", "IT");
            }
            "ja" => {
                dict.add_translation("income_tax", "所得税");
                dict.add_translation("corporate_tax", "法人税");
                dict.add_translation("value_added_tax", "消費税");
                dict.add_translation("capital_gains", "キャピタルゲイン");
                dict.add_translation("deduction", "控除");
                dict.add_translation("exemption", "免税");
                dict.add_translation("tax_liability", "納税義務");
                dict.add_translation("withholding_tax", "源泉徴収");
            }
            "de" => {
                dict.add_translation("income_tax", "Einkommensteuer");
                dict.add_translation("corporate_tax", "Körperschaftsteuer");
                dict.add_translation("value_added_tax", "Mehrwertsteuer");
                dict.add_translation("deduction", "Abzug");
                dict.add_abbreviation("value_added_tax", "MwSt");
            }
            _ => {}
        }
    }
    fn add_environmental_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("environmental_impact", "environmental impact");
                dict.add_translation("pollution", "pollution");
                dict.add_translation("emissions", "emissions");
                dict.add_translation("sustainability", "sustainability");
                dict.add_translation(
                    "environmental_assessment",
                    "environmental impact assessment",
                );
                dict.add_translation("climate_change", "climate change");
                dict.add_translation("carbon_footprint", "carbon footprint");
                dict.add_translation("renewable_energy", "renewable energy");
                dict.add_translation("hazardous_waste", "hazardous waste");
                dict.add_translation("conservation", "conservation");
                dict.add_translation("biodiversity", "biodiversity");
                dict.add_translation("environmental_compliance", "environmental compliance");
                dict.add_abbreviation("environmental_assessment", "EIA");
                dict.add_abbreviation("environmental_protection", "EPA");
            }
            "ja" => {
                dict.add_translation("environmental_impact", "環境影響");
                dict.add_translation("pollution", "汚染");
                dict.add_translation("emissions", "排出");
                dict.add_translation("sustainability", "持続可能性");
                dict.add_translation("environmental_assessment", "環境アセスメント");
                dict.add_translation("climate_change", "気候変動");
                dict.add_translation("renewable_energy", "再生可能エネルギー");
            }
            "de" => {
                dict.add_translation("environmental_impact", "Umweltauswirkung");
                dict.add_translation("pollution", "Verschmutzung");
                dict.add_translation("emissions", "Emissionen");
                dict.add_translation("sustainability", "Nachhaltigkeit");
            }
            _ => {}
        }
    }
    fn add_labor_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("employment_contract", "employment contract");
                dict.add_translation("collective_bargaining", "collective bargaining");
                dict.add_translation("wrongful_termination", "wrongful termination");
                dict.add_translation("discrimination", "discrimination");
                dict.add_translation("harassment", "harassment");
                dict.add_translation("minimum_wage", "minimum wage");
                dict.add_translation("overtime", "overtime");
                dict.add_translation("severance_pay", "severance pay");
                dict.add_translation("workers_compensation", "workers' compensation");
                dict.add_translation("occupational_safety", "occupational safety and health");
                dict.add_translation("labor_union", "labor union");
                dict.add_translation("strike", "strike");
                dict.add_translation("lockout", "lockout");
                dict.add_abbreviation("occupational_safety", "OSHA");
            }
            "ja" => {
                dict.add_translation("employment_contract", "雇用契約");
                dict.add_translation("collective_bargaining", "団体交渉");
                dict.add_translation("wrongful_termination", "不当解雇");
                dict.add_translation("discrimination", "差別");
                dict.add_translation("harassment", "ハラスメント");
                dict.add_translation("minimum_wage", "最低賃金");
                dict.add_translation("overtime", "残業");
                dict.add_translation("severance_pay", "退職金");
                dict.add_translation("labor_union", "労働組合");
            }
            "de" => {
                dict.add_translation("employment_contract", "Arbeitsvertrag");
                dict.add_translation("collective_bargaining", "Tarifverhandlungen");
                dict.add_translation("discrimination", "Diskriminierung");
                dict.add_translation("minimum_wage", "Mindestlohn");
                dict.add_translation("overtime", "Überstunden");
            }
            _ => {}
        }
    }
    fn add_corporate_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("merger", "merger");
                dict.add_translation("acquisition", "acquisition");
                dict.add_translation("due_diligence", "due diligence");
                dict.add_translation("shareholder", "shareholder");
                dict.add_translation("board_of_directors", "board of directors");
                dict.add_translation("corporate_governance", "corporate governance");
                dict.add_translation("fiduciary_duty", "fiduciary duty");
                dict.add_abbreviation("merger_and_acquisition", "M&A");
            }
            "ja" => {
                dict.add_translation("merger", "合併");
                dict.add_translation("acquisition", "買収");
                dict.add_translation("due_diligence", "デューデリジェンス");
                dict.add_translation("shareholder", "株主");
                dict.add_translation("board_of_directors", "取締役会");
            }
            _ => {}
        }
    }
    fn add_criminal_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("indictment", "indictment");
                dict.add_translation("arraignment", "arraignment");
                dict.add_translation("plea_bargain", "plea bargain");
                dict.add_translation("miranda_rights", "Miranda rights");
                dict.add_translation("probable_cause", "probable cause");
                dict.add_translation("beyond_reasonable_doubt", "beyond a reasonable doubt");
            }
            "ja" => {
                dict.add_translation("indictment", "起訴");
                dict.add_translation("arraignment", "罪状認否");
                dict.add_translation("probable_cause", "相当な理由");
            }
            _ => {}
        }
    }
    fn add_civil_procedure_terms(&self, dict: &mut LegalDictionary, locale: &Locale) {
        match locale.language.as_str() {
            "en" => {
                dict.add_translation("complaint", "complaint");
                dict.add_translation("summons", "summons");
                dict.add_translation("discovery", "discovery");
                dict.add_translation("deposition", "deposition");
                dict.add_translation("interrogatories", "interrogatories");
                dict.add_translation("summary_judgment", "summary judgment");
                dict.add_translation("motion_to_dismiss", "motion to dismiss");
            }
            "ja" => {
                dict.add_translation("complaint", "訴状");
                dict.add_translation("summons", "召喚状");
                dict.add_translation("discovery", "証拠開示");
            }
            _ => {}
        }
    }
}
/// Regional variation information for a locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalVariation {
    /// The base locale
    pub base_locale: Locale,
    /// Regional locale
    pub regional_locale: Locale,
    /// Description of the regional variation
    pub description: String,
    /// Key differences from the base locale
    pub differences: Vec<String>,
}
impl RegionalVariation {
    /// Creates a new regional variation.
    pub fn new(
        base_locale: Locale,
        regional_locale: Locale,
        description: impl Into<String>,
    ) -> Self {
        Self {
            base_locale,
            regional_locale,
            description: description.into(),
            differences: vec![],
        }
    }
    /// Adds a difference description.
    pub fn add_difference(mut self, difference: impl Into<String>) -> Self {
        self.differences.push(difference.into());
        self
    }
}
/// Screen reader friendly formatter for accessibility.
/// Generates ARIA labels, semantic markup, and screen reader optimized text.
#[derive(Debug)]
pub struct ScreenReaderFormatter {
    #[allow(dead_code)]
    locale: Locale,
}
impl ScreenReaderFormatter {
    /// Creates a new screen reader formatter.
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }
    /// Generates ARIA label for a legal document section.
    pub fn aria_label(&self, section_type: &str, title: &str) -> String {
        match section_type {
            "article" => format!("Article: {}", title),
            "section" => format!("Section: {}", title),
            "chapter" => format!("Chapter: {}", title),
            "clause" => format!("Clause: {}", title),
            "paragraph" => format!("Paragraph: {}", title),
            _ => format!("{}: {}", section_type, title),
        }
    }
    /// Formats legal citation for screen readers.
    pub fn format_citation(&self, citation: &str) -> String {
        let expanded = citation
            .replace("v.", "versus")
            .replace("No.", "Number")
            .replace("§", "Section")
            .replace("¶", "Paragraph")
            .replace("U.S.", "United States")
            .replace("F.2d", "Federal Reporter Second Series")
            .replace("F.3d", "Federal Reporter Third Series")
            .replace("S.Ct.", "Supreme Court Reporter");
        format!("Citation: {}", expanded)
    }
    /// Generates semantic navigation structure.
    pub fn navigation_structure(&self, sections: &[(&str, &str)]) -> String {
        let mut nav = String::from("<nav aria-label=\"Document Navigation\">\n");
        nav.push_str("  <ul>\n");
        for (section_type, title) in sections {
            nav.push_str(&format!(
                "    <li><a href=\"#{}\" aria-label=\"{}\">{}</a></li>\n",
                title.to_lowercase().replace(' ', "-"),
                self.aria_label(section_type, title),
                title
            ));
        }
        nav.push_str("  </ul>\n");
        nav.push_str("</nav>\n");
        nav
    }
    /// Formats table data for screen readers.
    pub fn format_table(&self, caption: &str, headers: &[&str], rows: &[Vec<&str>]) -> String {
        let mut table = format!("<table aria-label=\"{}\">\n", caption);
        table.push_str(&format!("  <caption>{}</caption>\n", caption));
        table.push_str("  <thead>\n    <tr>\n");
        for header in headers {
            table.push_str(&format!("      <th scope=\"col\">{}</th>\n", header));
        }
        table.push_str("    </tr>\n  </thead>\n  <tbody>\n");
        for row in rows {
            table.push_str("    <tr>\n");
            for (i, cell) in row.iter().enumerate() {
                if i == 0 {
                    table.push_str(&format!("      <th scope=\"row\">{}</th>\n", cell));
                } else {
                    table.push_str(&format!("      <td>{}</td>\n", cell));
                }
            }
            table.push_str("    </tr>\n");
        }
        table.push_str("  </tbody>\n</table>\n");
        table
    }
}
/// Legal term extractor for extracting terminology from statutes.
#[derive(Debug, Default)]
pub struct TerminologyExtractor {
    /// Known legal terms
    known_terms: std::collections::HashSet<String>,
    /// Extracted terms with frequencies
    pub(super) extracted: HashMap<String, usize>,
}
impl TerminologyExtractor {
    /// Creates a new terminology extractor.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates an extractor with known legal terms from a dictionary.
    pub fn with_dictionary(dictionary: &LegalDictionary) -> Self {
        let mut extractor = Self::new();
        for (key, _) in &dictionary.translations {
            extractor.known_terms.insert(key.clone());
        }
        extractor
    }
    /// Adds a known legal term.
    pub fn add_known_term(&mut self, term: impl Into<String>) {
        self.known_terms.insert(term.into());
    }
    /// Extracts terminology from statute text.
    pub fn extract_from_text(&mut self, text: &str) {
        let words: Vec<&str> = text
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|w| !w.is_empty())
            .collect();
        for window in words.windows(1) {
            let term = window.join("_").to_lowercase();
            if self.known_terms.contains(&term) {
                *self.extracted.entry(term).or_insert(0) += 1;
            }
        }
        for window_size in 2..=3 {
            for window in words.windows(window_size) {
                let term = window.join("_").to_lowercase();
                if self.known_terms.contains(&term) {
                    *self.extracted.entry(term).or_insert(0) += 1;
                }
            }
        }
    }
    /// Gets extracted terms sorted by frequency.
    pub fn get_terms_by_frequency(&self) -> Vec<(String, usize)> {
        let mut terms: Vec<_> = self
            .extracted
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        terms.sort_by_key(|b| std::cmp::Reverse(b.1));
        terms
    }
    /// Gets the frequency of a specific term.
    pub fn get_frequency(&self, term: &str) -> usize {
        *self.extracted.get(term).unwrap_or(&0)
    }
    /// Gets all extracted terms.
    pub fn extracted_terms(&self) -> &HashMap<String, usize> {
        &self.extracted
    }
    /// Clears all extracted terms.
    pub fn clear(&mut self) {
        self.extracted.clear();
    }
}
/// Address components for legal documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Street address line 1
    pub street1: String,
    /// Street address line 2 (optional)
    pub street2: Option<String>,
    /// City/municipality
    pub city: String,
    /// State/province/prefecture
    pub state: Option<String>,
    /// Postal/ZIP code
    pub postal_code: String,
    /// Country
    pub country: String,
    /// Building/apartment number (for some Asian countries)
    pub building: Option<String>,
}
impl Address {
    /// Creates a new address.
    pub fn new(
        street1: impl Into<String>,
        city: impl Into<String>,
        postal_code: impl Into<String>,
        country: impl Into<String>,
    ) -> Self {
        Self {
            street1: street1.into(),
            street2: None,
            city: city.into(),
            state: None,
            postal_code: postal_code.into(),
            country: country.into(),
            building: None,
        }
    }
    /// Sets the second street line.
    pub fn with_street2(mut self, street2: impl Into<String>) -> Self {
        self.street2 = Some(street2.into());
        self
    }
    /// Sets the state/province.
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }
    /// Sets the building number.
    pub fn with_building(mut self, building: impl Into<String>) -> Self {
        self.building = Some(building.into());
        self
    }
}
/// Registry of EU member state variations.
#[derive(Debug, Default)]
pub struct EUMemberStateRegistry {
    pub(super) variations: Vec<EUMemberStateVariation>,
}
impl EUMemberStateRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with default EU member state variations.
    #[allow(clippy::too_many_arguments)]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("de").with_country("DE"),
                "Germany",
                1958,
                "Civil law (German legal tradition)",
            )
            .add_eu_adaptation("GDPR implementation with national data protection law (BDSG)")
            .add_eu_adaptation("EU Directives transposed into German law")
            .add_specialty("Strong corporate governance (Mitbestimmung)")
            .add_specialty("Federal Constitutional Court (Bundesverfassungsgericht)"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("fr").with_country("FR"),
                "France",
                1958,
                "Civil law (French legal tradition - Napoleonic Code)",
            )
            .add_eu_adaptation("GDPR through French Data Protection Act")
            .add_eu_adaptation("EU competition law integrated into Code de commerce")
            .add_specialty("Administrative law (droit administratif)")
            .add_specialty("Conseil d'État for administrative disputes"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("es").with_country("ES"),
                "Spain",
                1986,
                "Civil law (Spanish legal tradition)",
            )
            .add_eu_adaptation("GDPR through Organic Law 3/2018")
            .add_eu_adaptation("Regional autonomy laws (Catalonia, Basque Country)")
            .add_specialty("Constitutional Court (Tribunal Constitucional)")
            .add_specialty("Regional legal variations"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("it").with_country("IT"),
                "Italy",
                1958,
                "Civil law (Italian legal tradition)",
            )
            .add_eu_adaptation("GDPR implemented through Legislative Decree 101/2018")
            .add_eu_adaptation("EU directives via legislative decrees")
            .add_specialty("Constitutional Court (Corte Costituzionale)")
            .add_specialty("Strong labor law protections"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("nl").with_country("NL"),
                "Netherlands",
                1958,
                "Civil law (Dutch legal tradition)",
            )
            .add_eu_adaptation("GDPR through Dutch Implementation Act (UAVG)")
            .add_eu_adaptation("EU law direct effect recognized")
            .add_specialty("International arbitration hub (The Hague)")
            .add_specialty("Strong commercial law tradition"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("pl").with_country("PL"),
                "Poland",
                2004,
                "Civil law (Polish legal tradition)",
            )
            .add_eu_adaptation("GDPR through Personal Data Protection Act")
            .add_eu_adaptation("EU structural funds legal framework")
            .add_specialty("Constitutional Tribunal (Trybunał Konstytucyjny)")
            .add_specialty("Post-communist legal reforms"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("sv").with_country("SE"),
                "Sweden",
                1995,
                "Civil law (Nordic legal tradition)",
            )
            .add_eu_adaptation("GDPR through Swedish Data Protection Act")
            .add_eu_adaptation("Maintained non-Euro currency (SEK)")
            .add_specialty("Strong transparency laws (Offentlighetsprincipen)")
            .add_specialty("Ombudsman system"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("en").with_country("IE"),
                "Ireland",
                1973,
                "Common law (Irish legal tradition)",
            )
            .add_eu_adaptation("GDPR enforced by Data Protection Commission")
            .add_eu_adaptation("EU tech hub with regulatory enforcement")
            .add_specialty("Common law in EU context")
            .add_specialty("Strong tech regulation enforcement"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("fr").with_country("BE"),
                "Belgium",
                1958,
                "Civil law (Belgian legal tradition)",
            )
            .add_eu_adaptation("GDPR through Belgian Data Protection Authority")
            .add_eu_adaptation("EU institutions headquarters")
            .add_specialty("Multilingual legal system (French, Dutch, German)")
            .add_specialty("Federal and regional court systems"),
        );
        registry.add_variation(
            EUMemberStateVariation::new(
                Locale::new("de").with_country("AT"),
                "Austria",
                1995,
                "Civil law (Austrian legal tradition - ABGB)",
            )
            .add_eu_adaptation("GDPR through Austrian Data Protection Act (DSG)")
            .add_eu_adaptation("EU neutrality adaptations")
            .add_specialty("Austrian Civil Code (ABGB) from 1811")
            .add_specialty("Strong constitutional court"),
        );
        registry
    }
    /// Adds a member state variation to the registry.
    pub fn add_variation(&mut self, variation: EUMemberStateVariation) {
        self.variations.push(variation);
    }
    /// Gets all member state variations.
    pub fn get_all_variations(&self) -> &[EUMemberStateVariation] {
        &self.variations
    }
    /// Finds a specific member state variation.
    pub fn find_variation(&self, country_code: &str) -> Option<&EUMemberStateVariation> {
        self.variations.iter().find(|v| {
            v.member_state_locale
                .country
                .as_ref()
                .map(|c| c == country_code)
                .unwrap_or(false)
        })
    }
}
/// Local custom registry.
#[derive(Debug, Clone, Default)]
pub struct LocalCustomRegistry {
    /// Customs indexed by region
    customs: HashMap<String, Vec<LocalCustom>>,
}
impl LocalCustomRegistry {
    /// Creates a new local custom registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with default customs.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_default_customs();
        registry
    }
    /// Adds a custom.
    pub fn add_custom(&mut self, custom: LocalCustom) {
        self.customs
            .entry(custom.region.clone())
            .or_default()
            .push(custom);
    }
    /// Gets customs for a region.
    pub fn get_customs(&self, region: &str) -> Vec<&LocalCustom> {
        self.customs
            .get(region)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Gets customs by type.
    pub fn get_by_type(&self, region: &str, custom_type: &CustomType) -> Vec<&LocalCustom> {
        self.get_customs(region)
            .into_iter()
            .filter(|c| &c.custom_type == custom_type)
            .collect()
    }
    /// Finds a specific custom by name.
    pub fn find_custom(&self, region: &str, name: &str) -> Option<&LocalCustom> {
        self.get_customs(region)
            .into_iter()
            .find(|c| c.name == name)
    }
    /// Adds default customs.
    fn add_default_customs(&mut self) {
        self.add_custom(
            LocalCustom::new(
                    "Miai marriage",
                    "Japan",
                    Locale::new("ja").with_country("JP"),
                    CustomType::Marriage,
                    "Traditional arranged marriage introduction system with legal implications for family law",
                )
                .with_recognition_level(0.3),
        );
        self.add_custom(
            LocalCustom::new(
                "Ie system",
                "Japan",
                Locale::new("ja").with_country("JP"),
                CustomType::Inheritance,
                "Traditional household system affecting inheritance and family law",
            )
            .with_recognition_level(0.4)
            .with_statutory_basis("Civil Code Article 897 (family grave inheritance)"),
        );
        self.add_custom(
            LocalCustom::new(
                "Red packet custom",
                "China",
                Locale::new("zh").with_script("Hans").with_country("CN"),
                CustomType::Business,
                "Monetary gift custom in business relationships and contracts",
            )
            .with_recognition_level(0.6),
        );
        self.add_custom(
            LocalCustom::new(
                "Hindu Undivided Family",
                "India",
                Locale::new("hi").with_country("IN"),
                CustomType::Property,
                "Joint family property ownership system with tax and inheritance implications",
            )
            .with_recognition_level(1.0)
            .with_statutory_basis("Hindu Succession Act, 1956"),
        );
        self.add_custom(
            LocalCustom::new(
                "Mahr",
                "Saudi Arabia",
                Locale::new("ar").with_country("SA"),
                CustomType::Marriage,
                "Mandatory marriage gift from groom to bride under Islamic law",
            )
            .with_recognition_level(1.0)
            .with_statutory_basis("Sharia law"),
        );
        self.add_custom(
            LocalCustom::new(
                "Tribal sovereignty",
                "United States",
                Locale::new("en").with_country("US"),
                CustomType::DisputeResolution,
                "Tribal courts have jurisdiction over certain matters on reservations",
            )
            .with_recognition_level(1.0)
            .with_statutory_basis("Indian Civil Rights Act of 1968"),
        );
    }
    /// Returns the total number of customs.
    pub fn custom_count(&self) -> usize {
        self.customs.values().map(|v| v.len()).sum()
    }
    /// Returns the number of regions.
    pub fn region_count(&self) -> usize {
        self.customs.len()
    }
}
/// Citation type for validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitationType {
    /// Court case
    Case,
    /// Statute or legislation
    Statute,
    /// Legal journal article
    Article,
    /// Legal book or treatise
    Book,
}
/// Analysis result from legal reasoning engine.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Original query.
    pub query: String,
    /// Query locale.
    pub locale: Locale,
    /// Matched legal concept.
    pub matched_concept: Option<LegalConcept>,
    /// Similar cases found.
    pub similar_cases: Vec<SearchResult>,
    /// Related knowledge graph nodes.
    pub related_nodes: Vec<(String, f32)>,
}
impl AnalysisResult {
    /// Checks if analysis found any results.
    pub fn has_results(&self) -> bool {
        self.matched_concept.is_some()
            || !self.similar_cases.is_empty()
            || !self.related_nodes.is_empty()
    }
    /// Returns the number of similar cases found.
    pub fn case_count(&self) -> usize {
        self.similar_cases.len()
    }
    /// Returns the number of related nodes found.
    pub fn node_count(&self) -> usize {
        self.related_nodes.len()
    }
}
/// Simplification strategy for plain language generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimplificationStrategy {
    /// Replace legal jargon with common terms
    ReplaceJargon,
    /// Break long sentences into shorter ones
    ShortenSentences,
    /// Remove passive voice
    ActiveVoice,
    /// Add explanatory context
    AddContext,
    /// Simplify complex grammatical structures
    SimplifyGrammar,
}
/// Regulatory domain for equivalence mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulatoryDomain {
    /// Data protection and privacy.
    DataProtection,
    /// Financial services regulation.
    FinancialServices,
    /// Environmental regulation.
    Environmental,
    /// Consumer protection.
    ConsumerProtection,
    /// Professional qualifications.
    ProfessionalQualifications,
    /// Product safety standards.
    ProductSafety,
    /// Telecommunications.
    Telecommunications,
}
/// Registry of regional variations for locales.
#[derive(Debug, Default)]
pub struct RegionalVariationRegistry {
    pub(super) variations: Vec<RegionalVariation>,
}
impl RegionalVariationRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with default regional variations.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("US"),
                "American English",
            )
            .add_difference("Uses 'attorney' instead of 'solicitor'")
            .add_difference("Federal system with state and federal courts")
            .add_difference("MM/DD/YYYY date format"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("GB"),
                "British English",
            )
            .add_difference("Uses 'solicitor' and 'barrister'")
            .add_difference("Equity and common law traditions")
            .add_difference("DD/MM/YYYY date format"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("AU"),
                "Australian English",
            )
            .add_difference("Follows UK legal terminology largely")
            .add_difference("Federal system similar to UK")
            .add_difference("DD/MM/YYYY date format"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("en"),
                Locale::new("en").with_country("CA"),
                "Canadian English",
            )
            .add_difference("Mixed common law and civil law (Quebec)")
            .add_difference("Bilingual legal system (English/French)")
            .add_difference("DD/MM/YYYY date format"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("es"),
                Locale::new("es").with_country("ES"),
                "European Spanish",
            )
            .add_difference("Uses 'vosotros' form")
            .add_difference("Civil law system based on Roman law"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("es"),
                Locale::new("es").with_country("MX"),
                "Mexican Spanish",
            )
            .add_difference("Uses 'ustedes' instead of 'vosotros'")
            .add_difference("Civil law influenced by indigenous legal traditions"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("es"),
                Locale::new("es").with_country("AR"),
                "Argentine Spanish",
            )
            .add_difference("Uses 'vos' form")
            .add_difference("Civil law based on Spanish and French codes"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("zh"),
                Locale::new("zh").with_country("CN").with_script("Hans"),
                "Simplified Chinese (Mainland)",
            )
            .add_difference("Simplified characters")
            .add_difference("Socialist legal system")
            .add_difference("Civil law tradition"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("zh"),
                Locale::new("zh").with_country("TW").with_script("Hant"),
                "Traditional Chinese (Taiwan)",
            )
            .add_difference("Traditional characters")
            .add_difference("Civil law based on German law")
            .add_difference("Separate legal system from mainland"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("zh"),
                Locale::new("zh").with_country("HK").with_script("Hant"),
                "Traditional Chinese (Hong Kong)",
            )
            .add_difference("Traditional characters")
            .add_difference("Common law system from British rule")
            .add_difference("Bilingual legal system (Chinese/English)"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("de"),
                Locale::new("de").with_country("DE"),
                "German (Germany)",
            )
            .add_difference("BGB (Civil Code)")
            .add_difference("Federal legal system"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("de"),
                Locale::new("de").with_country("AT"),
                "German (Austria)",
            )
            .add_difference("ABGB (Austrian Civil Code)")
            .add_difference("Similar to German law with variations"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("de"),
                Locale::new("de").with_country("CH"),
                "German (Switzerland)",
            )
            .add_difference("Swiss Civil Code (ZGB)")
            .add_difference("Multilingual legal system")
            .add_difference("Cantonal variations"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("fr"),
                Locale::new("fr").with_country("FR"),
                "French (France)",
            )
            .add_difference("Code Civil (Napoleonic Code)")
            .add_difference("Centralized legal system"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("fr"),
                Locale::new("fr").with_country("CA"),
                "French (Canada/Quebec)",
            )
            .add_difference("Civil law in Quebec, common law elsewhere")
            .add_difference("Bilingual legal system")
            .add_difference("Mix of French and English legal traditions"),
        );
        registry.add_variation(
            RegionalVariation::new(
                Locale::new("fr"),
                Locale::new("fr").with_country("BE"),
                "French (Belgium)",
            )
            .add_difference("Based on French Civil Code")
            .add_difference("Multilingual (French, Dutch, German)"),
        );
        registry
    }
    /// Adds a variation to the registry.
    pub fn add_variation(&mut self, variation: RegionalVariation) {
        self.variations.push(variation);
    }
    /// Gets all variations for a base locale.
    pub fn get_variations(&self, base_locale: &Locale) -> Vec<&RegionalVariation> {
        self.variations
            .iter()
            .filter(|v| v.base_locale.language == base_locale.language)
            .collect()
    }
    /// Finds a specific regional variation.
    pub fn find_variation(&self, regional_locale: &Locale) -> Option<&RegionalVariation> {
        self.variations
            .iter()
            .find(|v| v.regional_locale.tag() == regional_locale.tag())
    }
}
