//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};

use super::functions::I18nResult;
use super::types::{DocumentTemplate, LocalLawTerm};
use super::types_3::TemplateVariable;
use super::types_4::{HistoricalPeriod, QualityMetric, TreatyType, VariableType};
use super::types_6::{
    CitationComponents, CitationStyle, CitationValidationRule, Dialect, LegalConcept, LegalSystem,
    TreatyTerm,
};
use super::types_8::{DialectType, LegalDictionary, SignLanguageType};
use super::types_10::{DocumentTemplateType, Locale};
use super::types_11::{EmbeddingModel, I18nError};
use super::types_12::{ContributionStatus, SemanticEmbedding};
use super::types_13::{ArchaicTerm, TemplateSection};

/// Registry of legal document templates.
#[derive(Debug, Default)]
pub struct DocumentTemplateRegistry {
    templates: HashMap<String, DocumentTemplate>,
}
impl DocumentTemplateRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with default templates.
    #[allow(clippy::too_many_arguments)]
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        let nda_us = DocumentTemplate::new(
                "nda_mutual_us",
                "Mutual Non-Disclosure Agreement",
                DocumentTemplateType::Contract,
                Locale::new("en").with_country("US"),
                "US",
            )
            .add_variable(
                TemplateVariable::new(
                    "party1_name",
                    VariableType::Text,
                    true,
                    "Name of first party",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "party2_name",
                    VariableType::Text,
                    true,
                    "Name of second party",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "effective_date",
                    VariableType::Date,
                    true,
                    "Effective date of the agreement",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "state",
                    VariableType::Text,
                    true,
                    "Governing state law",
                ),
            )
            .add_section(
                TemplateSection::new("title", "MUTUAL NON-DISCLOSURE AGREEMENT\n"),
            )
            .add_section(
                TemplateSection::new(
                    "parties",
                    "This Mutual Non-Disclosure Agreement (\"Agreement\") is entered into as of {{effective_date}}, by and between {{party1_name}} (\"First Party\") and {{party2_name}} (\"Second Party\").\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "recitals",
                    "WHEREAS, the parties wish to explore a business opportunity of mutual interest and in connection with this opportunity, each party may disclose to the other certain confidential technical and business information that the disclosing party desires the receiving party to treat as confidential.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "confidential_info",
                    "1. CONFIDENTIAL INFORMATION\n\n\"Confidential Information\" means any information disclosed by either party to the other party, either directly or indirectly, in writing, orally or by inspection of tangible objects.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "obligations",
                    "2. OBLIGATIONS\n\nEach party agrees to: (a) hold the Confidential Information in strict confidence; (b) not disclose the Confidential Information to third parties; and (c) not use the Confidential Information except for the purpose of evaluating the potential business relationship.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "term",
                    "3. TERM\n\nThis Agreement shall remain in effect for a period of three (3) years from the effective date.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "governing_law",
                    "4. GOVERNING LAW\n\nThis Agreement shall be governed by the laws of the State of {{state}}, without regard to its conflict of laws provisions.\n",
                ),
            )
            .add_metadata("author", "Legalis Document Template System")
            .add_metadata("version", "1.0");
        registry.add_template(nda_us);
        let employment_us = DocumentTemplate::new(
                "employment_agreement_us",
                "Employment Agreement",
                DocumentTemplateType::Contract,
                Locale::new("en").with_country("US"),
                "US",
            )
            .add_variable(
                TemplateVariable::new(
                    "company_name",
                    VariableType::Text,
                    true,
                    "Name of the company",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "employee_name",
                    VariableType::PersonName,
                    true,
                    "Name of the employee",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "position",
                    VariableType::Text,
                    true,
                    "Job title/position",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "start_date",
                    VariableType::Date,
                    true,
                    "Employment start date",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "salary",
                    VariableType::Currency,
                    true,
                    "Annual salary",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "state",
                    VariableType::Text,
                    true,
                    "State law governing the agreement",
                ),
            )
            .add_section(TemplateSection::new("title", "EMPLOYMENT AGREEMENT\n"))
            .add_section(
                TemplateSection::new(
                    "parties",
                    "This Employment Agreement (\"Agreement\") is entered into as of {{start_date}}, by and between {{company_name}} (\"Company\") and {{employee_name}} (\"Employee\").\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "position_duties",
                    "1. POSITION AND DUTIES\n\nCompany hereby employs Employee in the position of {{position}}. Employee accepts such employment and agrees to devote their full business time and attention to the performance of such duties.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "compensation",
                    "2. COMPENSATION\n\nCompany shall pay Employee an annual salary of ${{salary}}, payable in accordance with Company's standard payroll practices.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "at_will",
                    "3. AT-WILL EMPLOYMENT\n\nEmployee's employment with Company is at-will, meaning that either Employee or Company may terminate the employment relationship at any time, with or without cause or notice.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "governing_law",
                    "4. GOVERNING LAW\n\nThis Agreement shall be governed by the laws of the State of {{state}}.\n",
                ),
            );
        registry.add_template(employment_us);
        let complaint_us = DocumentTemplate::new(
                "complaint_us",
                "Civil Complaint",
                DocumentTemplateType::CourtFiling,
                Locale::new("en").with_country("US"),
                "US",
            )
            .add_variable(
                TemplateVariable::new(
                    "court_name",
                    VariableType::Text,
                    true,
                    "Name of the court",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "plaintiff_name",
                    VariableType::PersonName,
                    true,
                    "Name of plaintiff",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "defendant_name",
                    VariableType::PersonName,
                    true,
                    "Name of defendant",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "case_number",
                    VariableType::Text,
                    false,
                    "Case number (if assigned)",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "jurisdiction_facts",
                    VariableType::Text,
                    true,
                    "Facts establishing jurisdiction",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "claim_facts",
                    VariableType::Text,
                    true,
                    "Facts supporting the claim",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "relief_requested",
                    VariableType::Text,
                    true,
                    "Relief requested from the court",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "caption",
                    "{{court_name}}\n\n{{plaintiff_name}},\n    Plaintiff,\nv.\n{{defendant_name}},\n    Defendant.\n\nCase No. {{case_number}}\n\nCOMPLAINT\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "introduction",
                    "Plaintiff {{plaintiff_name}} files this Complaint against Defendant {{defendant_name}} and alleges as follows:\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "jurisdiction",
                    "JURISDICTION AND VENUE\n\n1. {{jurisdiction_facts}}\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "facts",
                    "FACTUAL ALLEGATIONS\n\n2. {{claim_facts}}\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "prayer",
                    "PRAYER FOR RELIEF\n\nWHEREFORE, Plaintiff respectfully requests that the Court:\n\n{{relief_requested}}\n",
                ),
            );
        registry.add_template(complaint_us);
        let articles_de = DocumentTemplate::new(
                "articles_incorporation_de",
                "Certificate of Incorporation",
                DocumentTemplateType::Corporate,
                Locale::new("en").with_country("US"),
                "US-DE",
            )
            .add_variable(
                TemplateVariable::new(
                    "corporation_name",
                    VariableType::Text,
                    true,
                    "Name of the corporation",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "registered_agent_name",
                    VariableType::Text,
                    true,
                    "Name of registered agent",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "registered_agent_address",
                    VariableType::Address,
                    true,
                    "Address of registered agent",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "shares_authorized",
                    VariableType::Number,
                    true,
                    "Number of authorized shares",
                ),
            )
            .add_variable(
                TemplateVariable::new(
                    "incorporator_name",
                    VariableType::PersonName,
                    true,
                    "Name of incorporator",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "title",
                    "CERTIFICATE OF INCORPORATION\nOF\n{{corporation_name}}\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "article1",
                    "ARTICLE I - NAME\n\nThe name of the corporation is {{corporation_name}}.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "article2",
                    "ARTICLE II - REGISTERED OFFICE AND AGENT\n\nThe address of the corporation's registered office in the State of Delaware is {{registered_agent_address}}, and the name of its registered agent at such address is {{registered_agent_name}}.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "article3",
                    "ARTICLE III - PURPOSE\n\nThe purpose of the corporation is to engage in any lawful act or activity for which corporations may be organized under the General Corporation Law of Delaware.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "article4",
                    "ARTICLE IV - CAPITAL STOCK\n\nThe total number of shares of stock which the corporation shall have authority to issue is {{shares_authorized}} shares of Common Stock, par value $0.001 per share.\n",
                ),
            )
            .add_section(
                TemplateSection::new(
                    "signature",
                    "IN WITNESS WHEREOF, the undersigned incorporator has executed this Certificate of Incorporation this _____ day of __________, 20__.\n\n_________________________\n{{incorporator_name}}\nIncorporator\n",
                ),
            );
        registry.add_template(articles_de);
        registry
    }
    /// Adds a template to the registry.
    pub fn add_template(&mut self, template: DocumentTemplate) {
        self.templates.insert(template.id.clone(), template);
    }
    /// Gets a template by ID.
    pub fn get_template(&self, id: &str) -> Option<&DocumentTemplate> {
        self.templates.get(id)
    }
    /// Finds templates by type.
    pub fn find_by_type(&self, template_type: DocumentTemplateType) -> Vec<&DocumentTemplate> {
        self.templates
            .values()
            .filter(|t| t.template_type == template_type)
            .collect()
    }
    /// Finds templates by jurisdiction.
    pub fn find_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&DocumentTemplate> {
        self.templates
            .values()
            .filter(|t| t.jurisdiction == jurisdiction)
            .collect()
    }
    /// Lists all available template IDs.
    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }
}
/// Treaty language standardizer.
#[derive(Debug, Clone)]
pub struct TreatyStandardizer {
    /// Terms indexed by treaty name.
    pub(super) treaties: HashMap<String, Vec<TreatyTerm>>,
    /// Terms indexed by canonical term.
    pub(super) term_index: HashMap<String, Vec<TreatyTerm>>,
}
impl TreatyStandardizer {
    /// Creates a new treaty standardizer.
    pub fn new() -> Self {
        Self {
            treaties: HashMap::new(),
            term_index: HashMap::new(),
        }
    }
    /// Creates a standardizer with default UN treaty terms.
    pub fn with_un_defaults() -> Self {
        let mut standardizer = Self::new();
        standardizer.add_term(
            TreatyTerm::new("ICCPR", TreatyType::UNTreaty, "civil and political rights")
                .add_translation("fr", "droits civils et politiques")
                .add_translation("es", "derechos civiles y políticos")
                .add_translation("ru", "гражданские и политические права")
                .add_translation("zh", "公民权利和政治权利")
                .add_translation("ar", "الحقوق المدنية والسياسية")
                .with_article("Preamble")
                .add_country("US")
                .add_country("GB")
                .add_country("FR"),
        );
        standardizer.add_term(
            TreatyTerm::new("UNCLOS", TreatyType::UNTreaty, "territorial sea")
                .add_translation("fr", "mer territoriale")
                .add_translation("es", "mar territorial")
                .add_translation("ru", "территориальное море")
                .add_translation("zh", "领海")
                .with_article("Article 2")
                .add_country("US")
                .add_country("CN")
                .add_country("JP"),
        );
        standardizer.add_term(
            TreatyTerm::new(
                "Paris Agreement",
                TreatyType::Environmental,
                "climate change",
            )
            .add_translation("fr", "changement climatique")
            .add_translation("es", "cambio climático")
            .add_translation("de", "Klimawandel")
            .add_translation("zh", "气候变化")
            .with_article("Article 2")
            .add_country("US")
            .add_country("EU")
            .add_country("CN"),
        );
        standardizer
    }
    /// Adds a treaty term.
    pub fn add_term(&mut self, term: TreatyTerm) {
        self.treaties
            .entry(term.treaty_name.clone())
            .or_default()
            .push(term.clone());
        self.term_index
            .entry(term.canonical_term.clone())
            .or_default()
            .push(term);
    }
    /// Gets all terms for a specific treaty.
    pub fn get_treaty_terms(&self, treaty_name: &str) -> Vec<&TreatyTerm> {
        self.treaties
            .get(treaty_name)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Translates a canonical term to a specific language.
    pub fn translate_term(&self, canonical_term: &str, language: &str) -> Vec<String> {
        self.term_index
            .get(canonical_term)
            .map(|terms| {
                terms
                    .iter()
                    .filter_map(|t| t.translations.get(language).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Gets the total number of treaties.
    pub fn treaty_count(&self) -> usize {
        self.treaties.len()
    }
    /// Gets the total number of terms.
    pub fn term_count(&self) -> usize {
        self.term_index.len()
    }
}
/// Dialect handler.
#[derive(Debug, Clone)]
pub struct DialectHandler {
    /// Dialects indexed by ID.
    pub(super) dialects: HashMap<String, Dialect>,
    /// Dialects by base language.
    pub(super) by_language: HashMap<String, Vec<String>>,
}
impl DialectHandler {
    /// Creates a new dialect handler.
    pub fn new() -> Self {
        Self {
            dialects: HashMap::new(),
            by_language: HashMap::new(),
        }
    }
    /// Creates a handler with default dialects.
    pub fn with_defaults() -> Self {
        let mut handler = Self::new();
        handler.add_dialect(
            Dialect::new(
                "en-GB-legal",
                "en",
                "British Legal English",
                DialectType::Occupational,
            )
            .with_region("GB")
            .add_variation("attorney", "solicitor")
            .add_variation("lawsuit", "legal action")
            .add_variation("corporation", "company"),
        );
        handler.add_dialect(
            Dialect::new(
                "en-US-legal",
                "en",
                "American Legal English",
                DialectType::Occupational,
            )
            .with_region("US")
            .add_variation("solicitor", "attorney")
            .add_variation("barrister", "trial lawyer")
            .add_variation("lorry", "truck"),
        );
        handler.add_dialect(
            Dialect::new("es-MX", "es", "Mexican Spanish", DialectType::Regional)
                .with_region("MX")
                .add_variation("coche", "carro")
                .add_variation("ordenador", "computadora"),
        );
        handler.add_dialect(
            Dialect::new("es-AR", "es", "Argentine Spanish", DialectType::Regional)
                .with_region("AR")
                .add_variation("tú", "vos")
                .add_variation("coche", "auto"),
        );
        handler.add_dialect(
            Dialect::new("ar-EG", "ar", "Egyptian Arabic", DialectType::Regional)
                .with_region("EG")
                .add_variation("كيف حالك", "إزيك"),
        );
        handler.add_dialect(
            Dialect::new("ar-SA", "ar", "Saudi Arabic", DialectType::Regional)
                .with_region("SA")
                .add_variation("شنو", "ايش"),
        );
        handler.add_dialect(
            Dialect::new("zh-CN", "zh", "Simplified Chinese", DialectType::Regional)
                .with_region("CN")
                .add_variation("電腦", "电脑")
                .add_variation("軟體", "软件"),
        );
        handler.add_dialect(
            Dialect::new("zh-TW", "zh", "Traditional Chinese", DialectType::Regional)
                .with_region("TW")
                .add_variation("电脑", "電腦")
                .add_variation("软件", "軟體"),
        );
        handler
    }
    /// Adds a dialect.
    pub fn add_dialect(&mut self, dialect: Dialect) {
        self.by_language
            .entry(dialect.base_language.clone())
            .or_default()
            .push(dialect.dialect_id.clone());
        self.dialects.insert(dialect.dialect_id.clone(), dialect);
    }
    /// Gets a dialect by ID.
    pub fn get_dialect(&self, dialect_id: &str) -> Option<&Dialect> {
        self.dialects.get(dialect_id)
    }
    /// Gets all dialects for a language.
    pub fn get_by_language(&self, language_code: &str) -> Vec<&Dialect> {
        self.by_language
            .get(language_code)
            .map(|ids| ids.iter().filter_map(|id| self.dialects.get(id)).collect())
            .unwrap_or_default()
    }
    /// Normalizes a dialect term to standard form.
    pub fn normalize(&self, dialect_id: &str, term: &str) -> Option<String> {
        self.dialects
            .get(dialect_id)
            .and_then(|d| d.to_standard(term))
    }
    /// Converts standard term to dialect.
    pub fn to_dialect(&self, dialect_id: &str, standard_term: &str) -> Option<String> {
        self.dialects
            .get(dialect_id)
            .and_then(|d| d.to_dialect(standard_term))
    }
    /// Gets total dialect count.
    pub fn dialect_count(&self) -> usize {
        self.dialects.len()
    }
}
/// Community contribution workflow manager.
#[derive(Debug, Clone)]
pub struct ContributionWorkflow {
    /// Contributions by ID.
    contributions: HashMap<String, Contribution>,
    /// Contributions by status.
    by_status: HashMap<ContributionStatus, Vec<String>>,
    /// Contributions by language.
    by_language: HashMap<String, Vec<String>>,
}
impl ContributionWorkflow {
    /// Creates a new workflow manager.
    pub fn new() -> Self {
        Self {
            contributions: HashMap::new(),
            by_status: HashMap::new(),
            by_language: HashMap::new(),
        }
    }
    /// Submits a contribution.
    pub fn submit(&mut self, contribution: Contribution) {
        self.by_status
            .entry(contribution.status)
            .or_default()
            .push(contribution.contribution_id.clone());
        self.by_language
            .entry(contribution.language_code.clone())
            .or_default()
            .push(contribution.contribution_id.clone());
        self.contributions
            .insert(contribution.contribution_id.clone(), contribution);
    }
    /// Gets a contribution by ID.
    pub fn get_contribution(&self, id: &str) -> Option<&Contribution> {
        self.contributions.get(id)
    }
    /// Gets all contributions with a status.
    pub fn get_by_status(&self, status: ContributionStatus) -> Vec<&Contribution> {
        self.by_status
            .get(&status)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.contributions.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Gets contributions for a language.
    pub fn get_by_language(&self, language_code: &str) -> Vec<&Contribution> {
        self.by_language
            .get(language_code)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.contributions.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Approves a contribution.
    pub fn approve(&mut self, id: &str) -> Result<(), String> {
        if let Some(contrib) = self.contributions.get_mut(id) {
            let old_status = contrib.status;
            contrib.approve();
            let new_status = contrib.status;
            if let Some(vec) = self.by_status.get_mut(&old_status) {
                vec.retain(|cid| cid != id);
            }
            self.by_status
                .entry(new_status)
                .or_default()
                .push(id.to_string());
            Ok(())
        } else {
            Err(format!("Contribution {} not found", id))
        }
    }
    /// Rejects a contribution.
    pub fn reject(&mut self, id: &str, reason: impl Into<String>) -> Result<(), String> {
        if let Some(contrib) = self.contributions.get_mut(id) {
            let old_status = contrib.status;
            contrib.reject(reason);
            let new_status = contrib.status;
            if let Some(vec) = self.by_status.get_mut(&old_status) {
                vec.retain(|cid| cid != id);
            }
            self.by_status
                .entry(new_status)
                .or_default()
                .push(id.to_string());
            Ok(())
        } else {
            Err(format!("Contribution {} not found", id))
        }
    }
    /// Gets total contribution count.
    pub fn contribution_count(&self) -> usize {
        self.contributions.len()
    }
    /// Gets count by status.
    pub fn count_by_status(&self, status: ContributionStatus) -> usize {
        self.by_status.get(&status).map(|v| v.len()).unwrap_or(0)
    }
}
/// Term frequency-inverse document frequency (TF-IDF) score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfIdfScore {
    /// Term
    pub term: String,
    /// TF-IDF score
    pub score: f64,
    /// Term frequency in document
    pub term_frequency: f64,
    /// Inverse document frequency
    pub idf: f64,
}
impl TfIdfScore {
    /// Creates a new TF-IDF score.
    pub fn new(term: impl Into<String>, tf: f64, idf: f64) -> Self {
        Self {
            term: term.into(),
            score: tf * idf,
            term_frequency: tf,
            idf,
        }
    }
}
/// Archaic term dictionary for historical legal language.
#[derive(Debug, Clone)]
pub struct ArchaicTermDictionary {
    /// Terms indexed by period
    pub(super) terms_by_period: HashMap<HistoricalPeriod, Vec<ArchaicTerm>>,
    /// Terms indexed by archaic term
    pub(super) terms_by_name: HashMap<String, Vec<ArchaicTerm>>,
}
impl ArchaicTermDictionary {
    /// Creates a new archaic term dictionary.
    pub fn new() -> Self {
        Self {
            terms_by_period: HashMap::new(),
            terms_by_name: HashMap::new(),
        }
    }
    /// Creates a dictionary with default archaic legal terms.
    pub fn with_defaults() -> Self {
        let mut dict = Self::new();
        dict.add_term(
            ArchaicTerm::new(
                "folcriht",
                HistoricalPeriod::OldEnglish,
                "common law",
                "The law of the people, customary law",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Under folcriht, disputes were settled by the community"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "wergild",
                HistoricalPeriod::OldEnglish,
                "blood money",
                "Compensation paid to the family of a slain person",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The wergild for a thane was 1200 shillings"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "moot",
                HistoricalPeriod::OldEnglish,
                "assembly",
                "A judicial assembly or court",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The shire moot met twice yearly"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "feoffment",
                HistoricalPeriod::MiddleEnglish,
                "grant of land",
                "The grant of a fief or fee; transfer of property",
                Locale::new("en").with_country("GB"),
            )
            .with_example("A feoffment required livery of seisin"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "frankpledge",
                HistoricalPeriod::MiddleEnglish,
                "mutual surety",
                "System of collective responsibility for law and order",
                Locale::new("en").with_country("GB"),
            )
            .with_example("All freemen were organized into frankpledge groups"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "assize",
                HistoricalPeriod::MiddleEnglish,
                "court session",
                "A session of a court; also a statute or ordinance",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The assize of clarendon established procedures for criminal justice"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "attainder",
                HistoricalPeriod::EarlyModern,
                "forfeiture",
                "Loss of civil rights and property upon conviction of treason",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Bills of attainder were abolished in 1870"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "praemunire",
                HistoricalPeriod::EarlyModern,
                "usurpation of royal authority",
                "Offense of appealing to foreign authority over the Crown",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Praemunire was used against those asserting papal authority"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "ius civile",
                HistoricalPeriod::ClassicalLatin,
                "civil law",
                "The law applicable to Roman citizens",
                Locale::new("la"),
            )
            .with_example("Ius civile governed property and contract matters"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "lex aquilia",
                HistoricalPeriod::ClassicalLatin,
                "tort law",
                "Roman law governing damages to property",
                Locale::new("la"),
            )
            .with_example("The lex aquilia provided for compensation for wrongful damage"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "mancipatio",
                HistoricalPeriod::ClassicalLatin,
                "formal transfer",
                "Formal procedure for transferring ownership of property",
                Locale::new("la"),
            )
            .with_example("Mancipatio required five witnesses and a scale bearer"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "mainour",
                HistoricalPeriod::MedievalLatin,
                "stolen goods",
                "Stolen property found in the possession of a thief",
                Locale::new("la"),
            )
            .with_example("A thief taken with mainour could be summarily tried"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "essoign",
                HistoricalPeriod::MedievalLatin,
                "excuse",
                "An excuse for non-appearance in court",
                Locale::new("la"),
            )
            .with_example("Illness was a valid essoign for missing court"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "mesne profits",
                HistoricalPeriod::Victorian,
                "interim profits",
                "Profits from land wrongfully withheld from the rightful owner",
                Locale::new("en").with_country("GB"),
            )
            .with_example("The tenant was liable for mesne profits during the wrongful occupation"),
        );
        dict.add_term(
            ArchaicTerm::new(
                "copyhold",
                HistoricalPeriod::Victorian,
                "tenure by copy",
                "Land held by copy of the manorial court roll",
                Locale::new("en").with_country("GB"),
            )
            .with_example("Copyhold was abolished in 1925"),
        );
        dict
    }
    /// Adds an archaic term.
    pub fn add_term(&mut self, term: ArchaicTerm) {
        self.terms_by_period
            .entry(term.period)
            .or_default()
            .push(term.clone());
        self.terms_by_name
            .entry(term.term.clone())
            .or_default()
            .push(term);
    }
    /// Gets terms by historical period.
    pub fn get_by_period(&self, period: HistoricalPeriod) -> Vec<&ArchaicTerm> {
        self.terms_by_period
            .get(&period)
            .map(|terms| terms.iter().collect())
            .unwrap_or_default()
    }
    /// Gets terms by archaic name.
    pub fn get_by_name(&self, name: &str) -> Vec<&ArchaicTerm> {
        self.terms_by_name
            .get(name)
            .map(|terms| terms.iter().collect())
            .unwrap_or_default()
    }
    /// Translates archaic term to modern equivalent.
    pub fn translate_to_modern(&self, archaic_term: &str) -> Option<String> {
        self.terms_by_name
            .get(archaic_term)
            .and_then(|terms| terms.first())
            .map(|term| term.modern_equivalent.clone())
    }
    /// Returns the number of terms in the dictionary.
    pub fn term_count(&self) -> usize {
        self.terms_by_name.len()
    }
    /// Returns the number of periods represented.
    pub fn period_count(&self) -> usize {
        self.terms_by_period.len()
    }
}
/// Community contribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contribution {
    /// Contribution ID.
    pub contribution_id: String,
    /// Contributor identifier.
    pub contributor: String,
    /// Language code.
    pub language_code: String,
    /// Term or translation being contributed.
    pub content: LocalLawTerm,
    /// Current status.
    pub status: ContributionStatus,
    /// Submission timestamp.
    pub submitted_at: String,
    /// Review comments.
    pub comments: Vec<String>,
    /// Rejection reason (if rejected).
    pub rejection_reason: Option<String>,
}
impl Contribution {
    /// Creates a new contribution.
    pub fn new(
        contribution_id: impl Into<String>,
        contributor: impl Into<String>,
        language_code: impl Into<String>,
        content: LocalLawTerm,
    ) -> Self {
        Self {
            contribution_id: contribution_id.into(),
            contributor: contributor.into(),
            language_code: language_code.into(),
            content,
            status: ContributionStatus::Pending,
            submitted_at: "2024-01-01T00:00:00Z".to_string(),
            comments: Vec::new(),
            rejection_reason: None,
        }
    }
    /// Sets submission timestamp.
    pub fn with_timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.submitted_at = timestamp.into();
        self
    }
    /// Adds a review comment.
    pub fn add_comment(mut self, comment: impl Into<String>) -> Self {
        self.comments.push(comment.into());
        self
    }
    /// Approves the contribution.
    pub fn approve(&mut self) {
        self.status = ContributionStatus::Approved;
    }
    /// Rejects the contribution.
    pub fn reject(&mut self, reason: impl Into<String>) {
        self.status = ContributionStatus::Rejected;
        self.rejection_reason = Some(reason.into());
    }
    /// Moves to review.
    pub fn start_review(&mut self) {
        self.status = ContributionStatus::InReview;
    }
}
/// Document similarity score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore {
    /// Document 1 ID
    pub doc1_id: String,
    /// Document 2 ID
    pub doc2_id: String,
    /// Similarity score (0.0 to 1.0)
    pub score: f64,
    /// Similarity method used
    pub method: String,
}
impl SimilarityScore {
    /// Creates a new similarity score.
    pub fn new(
        doc1_id: impl Into<String>,
        doc2_id: impl Into<String>,
        score: f64,
        method: impl Into<String>,
    ) -> Self {
        Self {
            doc1_id: doc1_id.into(),
            doc2_id: doc2_id.into(),
            score: score.clamp(0.0, 1.0),
            method: method.into(),
        }
    }
    /// Checks if documents are highly similar (>= 0.8).
    pub fn is_highly_similar(&self) -> bool {
        self.score >= 0.8
    }
    /// Checks if documents are moderately similar (>= 0.5).
    pub fn is_moderately_similar(&self) -> bool {
        self.score >= 0.5
    }
}
/// AI quality score for a specific metric.
#[derive(Debug, Clone)]
pub struct AIQualityScore {
    /// The quality metric.
    pub metric: QualityMetric,
    /// The score (0.0 to 1.0).
    pub score: f32,
    /// Explanation of the score.
    pub explanation: Option<String>,
}
impl AIQualityScore {
    /// Creates a new quality score.
    pub fn new(metric: QualityMetric, score: f32) -> Self {
        Self {
            metric,
            score: score.clamp(0.0, 1.0),
            explanation: None,
        }
    }
    /// Adds an explanation.
    pub fn with_explanation(mut self, explanation: &str) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }
}
/// Sign language reference for video/image linking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignLanguageReference {
    /// Term or phrase in spoken/written language
    pub term: String,
    /// Sign language type
    pub sign_language: SignLanguageType,
    /// URL to video demonstrating the sign
    pub video_url: Option<String>,
    /// URL to image/diagram of the sign
    pub image_url: Option<String>,
    /// Description of how to perform the sign
    pub description: Option<String>,
    /// Locale of the term
    pub locale: Locale,
}
impl SignLanguageReference {
    /// Creates a new sign language reference.
    pub fn new(term: impl Into<String>, sign_language: SignLanguageType, locale: Locale) -> Self {
        Self {
            term: term.into(),
            sign_language,
            video_url: None,
            image_url: None,
            description: None,
            locale,
        }
    }
    /// Adds a video URL.
    pub fn with_video(mut self, url: impl Into<String>) -> Self {
        self.video_url = Some(url.into());
        self
    }
    /// Adds an image URL.
    pub fn with_image(mut self, url: impl Into<String>) -> Self {
        self.image_url = Some(url.into());
        self
    }
    /// Adds a description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}
/// Quality estimation report for AI translation.
#[derive(Debug, Clone)]
pub struct QualityEstimationReport {
    /// Overall quality score (0.0 to 1.0).
    pub overall_score: f32,
    /// Individual metric scores.
    pub metric_scores: HashMap<QualityMetric, AIQualityScore>,
    /// Source text.
    pub source_text: String,
    /// Translated text.
    pub translated_text: String,
    /// Source locale.
    pub source_locale: Locale,
    /// Target locale.
    pub target_locale: Locale,
}
impl QualityEstimationReport {
    /// Creates a new quality estimation report.
    pub fn new(
        source_text: &str,
        translated_text: &str,
        source_locale: Locale,
        target_locale: Locale,
    ) -> Self {
        Self {
            overall_score: 0.0,
            metric_scores: HashMap::new(),
            source_text: source_text.to_string(),
            translated_text: translated_text.to_string(),
            source_locale,
            target_locale,
        }
    }
    /// Adds a quality score for a metric.
    pub fn add_score(&mut self, score: AIQualityScore) {
        self.metric_scores.insert(score.metric, score);
        self.recalculate_overall_score();
    }
    /// Recalculates the overall score based on metric scores.
    fn recalculate_overall_score(&mut self) {
        if self.metric_scores.is_empty() {
            self.overall_score = 0.0;
            return;
        }
        let sum: f32 = self.metric_scores.values().map(|s| s.score).sum();
        self.overall_score = sum / self.metric_scores.len() as f32;
    }
    /// Gets the quality level (Low, Medium, High, Excellent).
    pub fn get_quality_level(&self) -> &str {
        match self.overall_score {
            s if s >= 0.9 => "Excellent",
            s if s >= 0.75 => "High",
            s if s >= 0.5 => "Medium",
            _ => "Low",
        }
    }
    /// Checks if the translation meets a minimum quality threshold.
    pub fn meets_threshold(&self, threshold: f32) -> bool {
        self.overall_score >= threshold
    }
    /// Generates a summary of the quality estimation.
    pub fn summary(&self) -> String {
        format!(
            "Translation from {} to {} - Overall Quality: {:.2}% ({})\n\
             Metric Scores: {}",
            self.source_locale,
            self.target_locale,
            self.overall_score * 100.0,
            self.get_quality_level(),
            self.metric_scores.len()
        )
    }
}
/// Mapping between legal concepts across different legal systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalConceptMapping {
    /// The legal system this concept belongs to
    pub legal_system: LegalSystem,
    /// The concept identifier
    pub concept: String,
    /// Equivalent concepts in other legal systems
    pub equivalents: HashMap<LegalSystem, Vec<String>>,
    /// Notes on differences or caveats
    pub notes: Option<String>,
}
impl LegalConceptMapping {
    /// Creates a new concept mapping.
    pub fn new(legal_system: LegalSystem, concept: impl Into<String>) -> Self {
        Self {
            legal_system,
            concept: concept.into(),
            equivalents: HashMap::new(),
            notes: None,
        }
    }
    /// Adds an equivalent concept in another legal system.
    pub fn add_equivalent(mut self, system: LegalSystem, equivalent: impl Into<String>) -> Self {
        self.equivalents
            .entry(system)
            .or_default()
            .push(equivalent.into());
        self
    }
    /// Adds a note about the mapping.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes = Some(note.into());
        self
    }
    /// Gets equivalent concepts for a target legal system.
    pub fn get_equivalents(&self, target: LegalSystem) -> Option<&Vec<String>> {
        self.equivalents.get(&target)
    }
}
/// Key term extractor using TF-IDF.
pub struct KeyTermExtractor {
    /// Document corpus for IDF calculation
    corpus: Vec<String>,
    /// Stop words to exclude
    pub(crate) stop_words: std::collections::HashSet<String>,
}
impl KeyTermExtractor {
    /// Creates a new key term extractor.
    pub fn new() -> Self {
        let stop_words = vec![
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with",
            "by", "from", "as", "is", "was", "are", "were", "be", "been", "being", "have", "has",
            "had", "do", "does", "did", "will", "would", "should", "could", "may", "might",
            "shall", "must", "can",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        Self {
            corpus: Vec::new(),
            stop_words,
        }
    }
    /// Adds a document to the corpus.
    pub fn add_document(&mut self, text: impl Into<String>) {
        self.corpus.push(text.into());
    }
    /// Calculates term frequency for a document.
    fn calculate_tf(&self, text: &str) -> HashMap<String, f64> {
        let words: Vec<String> = text
            .to_lowercase()
            .split_whitespace()
            .filter(|w| !self.stop_words.contains(*w))
            .map(|s| s.to_string())
            .collect();
        let total = words.len() as f64;
        let mut tf = HashMap::new();
        for word in words {
            *tf.entry(word).or_insert(0.0) += 1.0;
        }
        for count in tf.values_mut() {
            *count /= total;
        }
        tf
    }
    /// Calculates inverse document frequency.
    fn calculate_idf(&self, term: &str) -> f64 {
        let docs_with_term = self
            .corpus
            .iter()
            .filter(|doc| doc.to_lowercase().contains(term))
            .count();
        if docs_with_term == 0 {
            0.0
        } else {
            (self.corpus.len() as f64 / docs_with_term as f64).ln()
        }
    }
    /// Extracts key terms from a document using TF-IDF.
    pub fn extract_key_terms(&self, text: &str, top_n: usize) -> Vec<TfIdfScore> {
        let tf = self.calculate_tf(text);
        let mut scores = Vec::new();
        for (term, tf_val) in tf {
            let idf = self.calculate_idf(&term);
            scores.push(TfIdfScore::new(term, tf_val, idf));
        }
        scores.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scores.into_iter().take(top_n).collect()
    }
    /// Gets the corpus size.
    pub fn corpus_size(&self) -> usize {
        self.corpus.len()
    }
    /// Adds a custom stop word.
    pub fn add_stop_word(&mut self, word: impl Into<String>) {
        self.stop_words.insert(word.into());
    }
}
/// Multi-locale translation manager with LRU caching support.
#[derive(Debug)]
pub struct TranslationManager {
    dictionaries: HashMap<String, LegalDictionary>,
    fallback_locale: Option<Locale>,
    /// LRU cache for translation lookups: (key, locale_tag) -> translation
    /// Uses RwLock for thread-safe access in parallel operations
    pub(super) cache: Arc<RwLock<LruCache<(String, String), String>>>,
}
impl TranslationManager {
    /// Creates a new translation manager with default LRU cache size (1000 entries).
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{TranslationManager, LegalDictionary, Locale};
    ///
    /// let mut manager = TranslationManager::new();
    ///
    /// // Add Japanese dictionary
    /// let mut ja_dict = LegalDictionary::new(Locale::new("ja").with_country("JP"));
    /// ja_dict.add_translation("contract", "契約");
    /// manager.add_dictionary(ja_dict);
    ///
    /// // Translate
    /// let locale = Locale::new("ja").with_country("JP");
    /// let translation = manager.translate("contract", &locale).unwrap();
    /// assert_eq!(translation, "契約");
    /// ```
    pub fn new() -> Self {
        Self::with_cache_size(1000)
    }
    /// Creates a new translation manager with custom LRU cache size.
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            dictionaries: HashMap::new(),
            fallback_locale: None,
            cache: Arc::new(RwLock::new(LruCache::new(
                NonZeroUsize::new(cache_size)
                    .unwrap_or(NonZeroUsize::new(1000).expect("invariant: 1000 is non-zero")),
            ))),
        }
    }
    /// Sets the fallback locale.
    pub fn with_fallback(mut self, locale: Locale) -> Self {
        self.fallback_locale = Some(locale);
        self
    }
    /// Adds a dictionary.
    pub fn add_dictionary(&mut self, dict: LegalDictionary) {
        self.dictionaries.insert(dict.locale.tag(), dict);
    }
    /// Translates a key for a locale with caching.
    pub fn translate(&self, key: &str, locale: &Locale) -> I18nResult<String> {
        let cache_key = (key.to_string(), locale.tag());
        {
            if let Ok(mut cache) = self.cache.write()
                && let Some(cached) = cache.get(&cache_key)
            {
                return Ok(cached.clone());
            }
        }
        let result = self.translate_uncached(key, locale);
        if let Ok(ref translation) = result
            && let Ok(mut cache) = self.cache.write()
        {
            cache.put(cache_key, translation.clone());
        }
        result
    }
    /// Translates a key for a locale without using cache.
    fn translate_uncached(&self, key: &str, locale: &Locale) -> I18nResult<String> {
        if let Some(dict) = self.dictionaries.get(&locale.tag())
            && let Some(translation) = dict.translate(key)
        {
            return Ok(translation.to_string());
        }
        if let Some(dict) = self.dictionaries.get(&locale.language)
            && let Some(translation) = dict.translate(key)
        {
            return Ok(translation.to_string());
        }
        if let Some(ref fallback) = self.fallback_locale
            && let Some(dict) = self.dictionaries.get(&fallback.tag())
            && let Some(translation) = dict.translate(key)
        {
            return Ok(translation.to_string());
        }
        Err(I18nError::TranslationMissing {
            key: key.to_string(),
            locale: locale.tag(),
        })
    }
    /// Clears the translation cache.
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }
    /// Gets the current cache size.
    pub fn cache_size(&self) -> usize {
        self.cache.read().map(|cache| cache.len()).unwrap_or(0)
    }
    /// Resizes the LRU cache.
    /// Note: This creates a new cache, so all existing cached entries will be lost.
    pub fn resize_cache(&self, new_size: usize) {
        if let Ok(mut cache) = self.cache.write() {
            *cache = LruCache::new(
                NonZeroUsize::new(new_size)
                    .unwrap_or(NonZeroUsize::new(1000).expect("invariant: 1000 is non-zero")),
            );
        }
    }
}
/// Adjusted text with reading level information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustedText {
    /// Original text
    pub original: String,
    /// Adjusted text
    pub adjusted: String,
    /// Original reading level
    pub original_level: f64,
    /// Final reading level after adjustment
    pub final_level: f64,
    /// Target reading level
    pub target_level: f64,
    /// Number of iterations performed
    pub iterations: usize,
    /// Whether the adjusted text meets the target level
    pub meets_target: bool,
}
impl AdjustedText {
    /// Returns improvement in grade levels.
    pub fn improvement(&self) -> f64 {
        self.original_level - self.final_level
    }
}
/// Multilingual semantic embedder for legal text.
#[derive(Debug, Clone)]
pub struct MultilingualEmbedder {
    /// Embedding model to use.
    pub model: EmbeddingModel,
    /// Embedding dimension size.
    pub dimension: usize,
    /// Whether to normalize embeddings.
    pub normalize: bool,
    /// Legal dictionary for domain adaptation.
    pub dictionary: Option<LegalDictionary>,
}
impl MultilingualEmbedder {
    /// Creates a new multilingual embedder.
    pub fn new(model: EmbeddingModel, dimension: usize) -> Self {
        Self {
            model,
            dimension,
            normalize: true,
            dictionary: None,
        }
    }
    /// Creates an embedder with LaBSE (768 dimensions).
    pub fn labse() -> Self {
        Self::new(EmbeddingModel::LaBSE, 768)
    }
    /// Creates an embedder with XLM-RoBERTa (1024 dimensions).
    pub fn xlm_roberta() -> Self {
        Self::new(EmbeddingModel::XLMRoBERTa, 1024)
    }
    /// Creates an embedder for legal domain (768 dimensions).
    pub fn legal_domain() -> Self {
        Self::new(EmbeddingModel::LegalMultilingual, 768)
    }
    /// Sets whether to normalize embeddings.
    pub fn with_normalization(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }
    /// Sets the legal dictionary for domain adaptation.
    pub fn with_dictionary(mut self, dictionary: LegalDictionary) -> Self {
        self.dictionary = Some(dictionary);
        self
    }
    /// Embeds text into a semantic vector (placeholder - would use actual model).
    pub fn embed(&self, text: &str, locale: Locale) -> SemanticEmbedding {
        let mut vector = vec![0.0f32; self.dimension];
        for (i, byte) in text.bytes().enumerate() {
            let idx = (byte as usize + i) % self.dimension;
            vector[idx] += 1.0;
        }
        if self.normalize {
            let magnitude: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
            if magnitude > 0.0 {
                for v in &mut vector {
                    *v /= magnitude;
                }
            }
        }
        SemanticEmbedding::new(text, locale, vector, self.model.to_string())
    }
    /// Embeds multiple texts in batch.
    pub fn embed_batch(&self, texts: &[(String, Locale)]) -> Vec<SemanticEmbedding> {
        texts
            .iter()
            .map(|(text, locale)| self.embed(text, locale.clone()))
            .collect()
    }
}
/// Cross-lingual concept mapper.
#[derive(Debug, Clone)]
pub struct ConceptMapper {
    /// Legal concepts indexed by ID.
    pub concepts: HashMap<String, LegalConcept>,
    /// Multilingual embedder.
    pub embedder: MultilingualEmbedder,
}
impl ConceptMapper {
    /// Creates a new concept mapper.
    pub fn new(embedder: MultilingualEmbedder) -> Self {
        Self {
            concepts: HashMap::new(),
            embedder,
        }
    }
    /// Creates a mapper with default legal concepts.
    pub fn with_defaults(embedder: MultilingualEmbedder) -> Self {
        let mut mapper = Self::new(embedder);
        mapper.add_concept(
            LegalConcept::new(
                "contract",
                "Contract",
                "A legally binding agreement between two or more parties",
            )
            .add_localized_name(Locale::new("es").with_country("ES"), "Contrato")
            .add_localized_name(Locale::new("fr").with_country("FR"), "Contrat")
            .add_localized_name(Locale::new("de").with_country("DE"), "Vertrag")
            .add_localized_name(Locale::new("ja").with_country("JP"), "契約"),
        );
        mapper.add_concept(
            LegalConcept::new(
                "tort",
                "Tort",
                "A civil wrong that causes harm or loss to another person",
            )
            .add_localized_name(Locale::new("fr").with_country("FR"), "Délit civil")
            .add_localized_name(Locale::new("de").with_country("DE"), "Delikt")
            .add_localized_name(Locale::new("ja").with_country("JP"), "不法行為"),
        );
        mapper.add_concept(
            LegalConcept::new(
                "jurisdiction",
                "Jurisdiction",
                "The official power to make legal decisions and judgments",
            )
            .add_localized_name(Locale::new("es").with_country("ES"), "Jurisdicción")
            .add_localized_name(Locale::new("fr").with_country("FR"), "Juridiction")
            .add_localized_name(Locale::new("de").with_country("DE"), "Gerichtsbarkeit")
            .add_localized_name(Locale::new("ja").with_country("JP"), "管轄権"),
        );
        mapper
    }
    /// Adds a concept to the mapper.
    pub fn add_concept(&mut self, mut concept: LegalConcept) {
        if concept.embedding.is_none() {
            let embedding = self
                .embedder
                .embed(&concept.definition, Locale::new("en").with_country("US"));
            concept.embedding = Some(embedding);
        }
        self.concepts.insert(concept.concept_id.clone(), concept);
    }
    /// Finds the most similar concept to a query.
    pub fn find_concept(&self, query: &str, locale: Locale) -> Option<LegalConcept> {
        let query_embedding = self.embedder.embed(query, locale);
        self.concepts
            .values()
            .filter_map(|concept| {
                concept.embedding.as_ref().map(|emb| {
                    let similarity = query_embedding.cosine_similarity(emb);
                    (concept.clone(), similarity)
                })
            })
            .max_by(|(_, sim_a), (_, sim_b)| {
                sim_a
                    .partial_cmp(sim_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(concept, _)| concept)
    }
    /// Maps a term from one language to equivalent terms in other languages.
    pub fn map_term_across_languages(
        &self,
        term: &str,
        source_locale: Locale,
    ) -> HashMap<String, String> {
        if let Some(concept) = self.find_concept(term, source_locale) {
            concept.localized_names.clone()
        } else {
            HashMap::new()
        }
    }
    /// Returns the total number of concepts.
    pub fn concept_count(&self) -> usize {
        self.concepts.len()
    }
}
/// Citation validation errors.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CitationError {
    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    /// Invalid field format
    #[error("Invalid format for field {field}: {reason}")]
    InvalidFormat { field: String, reason: String },
    /// Style-specific violation
    #[error("Style violation for {style}: {reason}")]
    StyleViolation { style: String, reason: String },
    /// Parse error
    #[error("Failed to parse citation: {reason}")]
    ParseError { reason: String },
    /// Unsupported conversion
    #[error("Cannot convert from {from} to {to}: {reason}")]
    UnsupportedConversion {
        from: String,
        to: String,
        reason: String,
    },
}
/// Citation validator for checking citations against style rules.
#[derive(Debug, Clone)]
pub struct CitationValidator {
    style: CitationStyle,
}
impl CitationValidator {
    /// Creates a new citation validator for a specific style.
    pub fn new(style: CitationStyle) -> Self {
        Self { style }
    }
    /// Validates a case citation.
    pub fn validate_case(&self, components: &CitationComponents) -> Result<(), Vec<CitationError>> {
        let rules = self.get_case_rules();
        self.validate_with_rules(components, &rules)
    }
    /// Validates a statute citation.
    pub fn validate_statute(
        &self,
        components: &CitationComponents,
    ) -> Result<(), Vec<CitationError>> {
        let rules = self.get_statute_rules();
        self.validate_with_rules(components, &rules)
    }
    /// Validates components against a set of rules.
    fn validate_with_rules(
        &self,
        components: &CitationComponents,
        rules: &[CitationValidationRule],
    ) -> Result<(), Vec<CitationError>> {
        let mut errors = Vec::new();
        let year_str = components.year.map(|y| y.to_string());
        for rule in rules {
            let value = match rule.field.as_str() {
                "title" => Some(&components.title),
                "volume" => components.volume.as_ref(),
                "reporter" => components.reporter.as_ref(),
                "page" => components.page.as_ref(),
                "court" => components.court.as_ref(),
                "year" => year_str.as_ref(),
                "jurisdiction" => components.jurisdiction.as_ref(),
                _ => None,
            };
            if let Err(e) = rule.validate(value) {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    /// Gets validation rules for case citations based on style.
    pub(crate) fn get_case_rules(&self) -> Vec<CitationValidationRule> {
        match &self.style {
            CitationStyle::Bluebook => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::required("volume").with_pattern("numeric"),
                    CitationValidationRule::required("reporter"),
                    CitationValidationRule::required("page"),
                    CitationValidationRule::optional("court"),
                    CitationValidationRule::required("year").with_pattern("year"),
                ]
            }
            CitationStyle::OSCOLA => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::required("year").with_pattern("year"),
                    CitationValidationRule::required("reporter"),
                    CitationValidationRule::optional("page"),
                ]
            }
            CitationStyle::AGLC => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::required("year").with_pattern("year"),
                    CitationValidationRule::required("reporter"),
                    CitationValidationRule::optional("volume"),
                ]
            }
            CitationStyle::McGill => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::required("year").with_pattern("year"),
                    CitationValidationRule::optional("reporter"),
                ]
            }
            CitationStyle::Japanese => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::optional("year"),
                ]
            }
            CitationStyle::Indian => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::required("year").with_pattern("year"),
                    CitationValidationRule::optional("reporter"),
                    CitationValidationRule::optional("court"),
                ]
            }
            _ => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::optional("year"),
                ]
            }
        }
    }
    /// Gets validation rules for statute citations based on style.
    pub(crate) fn get_statute_rules(&self) -> Vec<CitationValidationRule> {
        match &self.style {
            CitationStyle::Bluebook => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::optional("page"),
                ]
            }
            CitationStyle::OSCOLA => {
                vec![
                    CitationValidationRule::required("title"),
                    CitationValidationRule::optional("year").with_pattern("year"),
                ]
            }
            _ => vec![CitationValidationRule::required("title")],
        }
    }
}
