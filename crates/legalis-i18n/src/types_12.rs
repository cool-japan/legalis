//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{ClauseType, LocalLawTerm};
use super::types_3::DialectTerminology;
use super::types_4::{
    ExtractedDeadline, NumberingStyle, PlainLanguageGenerator, QualityMetric, ReadabilityReport,
};
use super::types_5::{AIQualityScore, AdjustedText, QualityEstimationReport};
use super::types_6::RiskFactor;
use super::types_8::RiskLevel;
use super::types_9::{DisambiguationType, ExtractedObligation, IdentifiedParty, WCAGLevel};
use super::types_10::{LegalCase, Locale, ReligiousLawType};
use super::types_11::{DisambiguationContext, HistoricalCalendar};

/// LLM provider type for AI-powered translation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LLMProvider {
    /// OpenAI GPT models.
    OpenAI,
    /// Anthropic Claude models.
    Anthropic,
    /// Google PaLM/Gemini models.
    Google,
    /// Meta Llama models.
    Meta,
    /// Custom LLM provider.
    Custom,
}
/// Local law terminology database.
#[derive(Debug, Clone)]
pub struct LocalLawDatabase {
    /// Terms indexed by local term.
    terms: HashMap<String, LocalLawTerm>,
    /// Terms by jurisdiction.
    by_jurisdiction: HashMap<String, Vec<String>>,
    /// Terms by legal system.
    by_system: HashMap<String, Vec<String>>,
}
impl LocalLawDatabase {
    /// Creates a new database.
    pub fn new() -> Self {
        Self {
            terms: HashMap::new(),
            by_jurisdiction: HashMap::new(),
            by_system: HashMap::new(),
        }
    }
    /// Creates a database with sample terms.
    pub fn with_samples() -> Self {
        let mut db = Self::new();
        db.add_term(
            LocalLawTerm::new("憲法", "Constitution", "Civil Law", "JP", "国の最高法規")
                .add_statute("日本国憲法")
                .add_example("憲法第9条"),
        );
        db.add_term(
            LocalLawTerm::new(
                "民法",
                "Civil Code",
                "Civil Law",
                "JP",
                "私人間の法律関係を定める法律",
            )
            .add_statute("民法典")
            .add_example("民法第709条（不法行為）"),
        );
        db.add_term(
            LocalLawTerm::new(
                "Grundgesetz",
                "Basic Law (Constitution)",
                "Civil Law",
                "DE",
                "Die Verfassung der Bundesrepublik Deutschland",
            )
            .add_statute("GG")
            .add_example("Grundgesetz Artikel 1"),
        );
        db.add_term(
            LocalLawTerm::new(
                "Bürgerliches Gesetzbuch",
                "Civil Code",
                "Civil Law",
                "DE",
                "Das wichtigste Gesetz des deutschen Zivilrechts",
            )
            .add_statute("BGB")
            .add_example("§ 823 BGB (Schadensersatzpflicht)"),
        );
        db.add_term(
            LocalLawTerm::new(
                "Code civil",
                "Civil Code",
                "Civil Law",
                "FR",
                "Code régissant le droit civil en France",
            )
            .add_statute("Code Napoléon")
            .add_example("Article 1240 (responsabilité civile)"),
        );
        db.add_term(
            LocalLawTerm::new(
                "宪法",
                "Constitution",
                "Socialist Law",
                "CN",
                "国家的根本法",
            )
            .add_statute("中华人民共和国宪法")
            .add_example("宪法第一条"),
        );
        db.add_term(
            LocalLawTerm::new(
                "民法典",
                "Civil Code",
                "Socialist Law",
                "CN",
                "调整平等主体之间的民事关系",
            )
            .add_statute("中华人民共和国民法典")
            .add_example("民法典第一编（总则）"),
        );
        db.add_term(
            LocalLawTerm::new(
                "संविधान",
                "Constitution",
                "Common Law",
                "IN",
                "भारत का सर्वोच्च कानून",
            )
            .add_statute("भारतीय संविधान")
            .add_example("अनुच्छेद 14 (समानता का अधिकार)"),
        );
        db.add_term(
            LocalLawTerm::new(
                "Undang-Undang Dasar",
                "Constitution",
                "Civil Law",
                "ID",
                "Konstitusi negara Indonesia",
            )
            .add_statute("UUD 1945")
            .add_example("Pasal 27 (Hak dan Kewajiban Warga Negara)"),
        );
        db.add_term(
            LocalLawTerm::new(
                "الشريعة الإسلامية",
                "Islamic Law (Sharia)",
                "Islamic Law",
                "SA",
                "القانون الإسلامي المستمد من القرآن والسنة",
            )
            .add_statute("القرآن الكريم")
            .add_example("أحكام الأسرة"),
        );
        db
    }
    /// Adds a term to the database.
    pub fn add_term(&mut self, term: LocalLawTerm) {
        self.by_jurisdiction
            .entry(term.jurisdiction.clone())
            .or_default()
            .push(term.local_term.clone());
        self.by_system
            .entry(term.legal_system.clone())
            .or_default()
            .push(term.local_term.clone());
        self.terms.insert(term.local_term.clone(), term);
    }
    /// Gets a term.
    pub fn get_term(&self, local_term: &str) -> Option<&LocalLawTerm> {
        self.terms.get(local_term)
    }
    /// Gets terms by jurisdiction.
    pub fn get_by_jurisdiction(&self, jurisdiction: &str) -> Vec<&LocalLawTerm> {
        self.by_jurisdiction
            .get(jurisdiction)
            .map(|terms| terms.iter().filter_map(|t| self.terms.get(t)).collect())
            .unwrap_or_default()
    }
    /// Gets terms by legal system.
    pub fn get_by_system(&self, system: &str) -> Vec<&LocalLawTerm> {
        self.by_system
            .get(system)
            .map(|terms| terms.iter().filter_map(|t| self.terms.get(t)).collect())
            .unwrap_or_default()
    }
    /// Translates to English.
    pub fn to_english(&self, local_term: &str) -> Option<String> {
        self.terms.get(local_term).map(|t| t.english_equiv.clone())
    }
    /// Gets total term count.
    pub fn term_count(&self) -> usize {
        self.terms.len()
    }
}
/// Cultural context category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextCategory {
    /// Social hierarchy and honorifics
    SocialHierarchy,
    /// Family structure and relationships
    FamilyStructure,
    /// Religious practices
    ReligiousPractice,
    /// Business etiquette
    BusinessEtiquette,
    /// Legal formality levels
    LegalFormality,
    /// Gender roles and expectations
    GenderRoles,
    /// Time perception (monochronic vs polychronic)
    TimePerception,
    /// Communication style (direct vs indirect)
    CommunicationStyle,
    /// Custom category
    Custom(String),
}
/// Legal topic for topic modeling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalTopic {
    /// Topic ID
    pub id: String,
    /// Topic name
    pub name: String,
    /// Key terms for this topic
    pub key_terms: Vec<String>,
    /// Topic weight in document (0.0 to 1.0)
    pub weight: f64,
}
impl LegalTopic {
    /// Creates a new legal topic.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            key_terms: Vec::new(),
            weight: 0.0,
        }
    }
    /// Adds a key term.
    pub fn add_term(mut self, term: impl Into<String>) -> Self {
        self.key_terms.push(term.into());
        self
    }
    /// Sets the topic weight.
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }
}
/// Post-editing action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostEditAction {
    /// Accept translation as-is
    Accept,
    /// Reject and request new translation
    Reject,
    /// Edit specific segments
    Edit,
}
/// Search result with similarity score.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    /// The matched legal case.
    pub case: LegalCase,
    /// Similarity score (0.0 to 1.0).
    pub similarity: f32,
    /// Rank in search results.
    pub rank: usize,
}
impl SearchResult {
    /// Creates a new search result.
    pub fn new(case: LegalCase, similarity: f32, rank: usize) -> Self {
        Self {
            case,
            similarity,
            rank,
        }
    }
}
/// Knowledge graph node representing a legal entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeGraphNode {
    /// Node identifier.
    pub node_id: String,
    /// Node type (concept, case, statute, etc.).
    pub node_type: String,
    /// Node label/name.
    pub label: String,
    /// Node locale.
    pub locale: Locale,
    /// Node properties.
    pub properties: HashMap<String, String>,
    /// Semantic embedding.
    pub embedding: Option<SemanticEmbedding>,
}
impl KnowledgeGraphNode {
    /// Creates a new knowledge graph node.
    pub fn new(
        node_id: impl Into<String>,
        node_type: impl Into<String>,
        label: impl Into<String>,
        locale: Locale,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            node_type: node_type.into(),
            label: label.into(),
            locale,
            properties: HashMap::new(),
            embedding: None,
        }
    }
    /// Adds a property to the node.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
    /// Sets the semantic embedding.
    pub fn with_embedding(mut self, embedding: SemanticEmbedding) -> Self {
        self.embedding = Some(embedding);
        self
    }
}
/// Compliance language normalization level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NormalizationLevel {
    /// Strict normalization (exact terminology).
    Strict,
    /// Standard normalization (common variants allowed).
    Standard,
    /// Flexible normalization (contextual equivalents).
    Flexible,
}
/// Deadline extractor for legal documents with calendar integration.
#[derive(Debug, Default)]
pub struct DeadlineExtractor {
    /// Reference date for relative date calculations
    reference_date: Option<(i32, u32, u32)>,
}
impl DeadlineExtractor {
    /// Creates a new deadline extractor.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets a reference date for relative date calculations.
    pub fn with_reference_date(mut self, year: i32, month: u32, day: u32) -> Self {
        self.reference_date = Some((year, month, day));
        self
    }
    /// Extracts deadlines from document text.
    pub fn extract(&self, text: &str) -> Vec<ExtractedDeadline> {
        let mut deadlines = Vec::new();
        let _date_patterns = [
            r"(\d{1,2})/(\d{1,2})/(\d{2,4})",
            r"(\d{4})-(\d{1,2})-(\d{1,2})",
            r"(\d{1,2})\s+(days?|months?|years?)",
        ];
        let sentences: Vec<&str> = text.split(&['.', ';'][..]).collect();
        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_lower = sentence.to_lowercase();
            if sentence_lower.contains("deadline")
                || sentence_lower.contains("due")
                || sentence_lower.contains("within")
                || sentence_lower.contains("by")
                || sentence_lower.contains("before")
                || sentence_lower.contains("after")
            {
                let date = self.parse_date(sentence);
                deadlines.push(ExtractedDeadline {
                    date,
                    description: sentence.trim().to_string(),
                    position: i * 100,
                    confidence: if date.is_some() { 0.8 } else { 0.5 },
                    context: sentence.trim().to_string(),
                });
            }
        }
        deadlines
    }
    fn parse_date(&self, text: &str) -> Option<(i32, u32, u32)> {
        let parts: Vec<&str> = text.split('/').collect();
        if parts.len() == 3
            && let (Ok(month), Ok(day), Ok(year)) = (
                parts[0].trim().parse::<u32>(),
                parts[1].trim().parse::<u32>(),
                parts[2].trim().parse::<i32>(),
            )
        {
            let full_year = if year < 100 {
                if year > 50 { 1900 + year } else { 2000 + year }
            } else {
                year
            };
            return Some((full_year, month, day));
        }
        None
    }
}
/// Index entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    /// Term
    pub term: String,
    /// Page numbers where this term appears
    pub pages: Vec<usize>,
    /// Sub-entries
    pub sub_entries: Vec<IndexEntry>,
}
impl IndexEntry {
    /// Creates a new index entry.
    pub fn new(term: String) -> Self {
        Self {
            term,
            pages: Vec::new(),
            sub_entries: Vec::new(),
        }
    }
    /// Adds a page reference.
    pub fn add_page(&mut self, page: usize) {
        if !self.pages.contains(&page) {
            self.pages.push(page);
            self.pages.sort();
        }
    }
    /// Adds a sub-entry.
    pub fn add_sub_entry(&mut self, entry: IndexEntry) {
        self.sub_entries.push(entry);
    }
}
/// Contribution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContributionStatus {
    /// Submitted and pending review.
    Pending,
    /// Under review by maintainers.
    InReview,
    /// Approved and merged.
    Approved,
    /// Rejected with reason.
    Rejected,
}
/// Religious law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReligiousLawSystem {
    /// Type of religious law
    pub law_type: ReligiousLawType,
    /// Jurisdictions where this system is recognized
    pub jurisdictions: Vec<String>,
    /// Integration level with civil law (0.0 = separate, 1.0 = fully integrated)
    pub integration_level: f32,
    /// Key principles
    pub principles: Vec<String>,
    /// Sources of authority
    pub sources: Vec<String>,
    /// Civil law equivalents
    pub civil_equivalents: HashMap<String, String>,
}
impl ReligiousLawSystem {
    /// Creates a new religious law system.
    pub fn new(law_type: ReligiousLawType) -> Self {
        Self {
            law_type,
            jurisdictions: Vec::new(),
            integration_level: 0.5,
            principles: Vec::new(),
            sources: Vec::new(),
            civil_equivalents: HashMap::new(),
        }
    }
    /// Adds a jurisdiction.
    pub fn add_jurisdiction(&mut self, jurisdiction: impl Into<String>) {
        self.jurisdictions.push(jurisdiction.into());
    }
    /// Sets integration level.
    pub fn with_integration_level(mut self, level: f32) -> Self {
        self.integration_level = level.clamp(0.0, 1.0);
        self
    }
    /// Adds a principle.
    pub fn with_principle(mut self, principle: impl Into<String>) -> Self {
        self.principles.push(principle.into());
        self
    }
    /// Adds a source.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.sources.push(source.into());
        self
    }
    /// Adds a civil law equivalent.
    pub fn with_equivalent(
        mut self,
        religious_concept: impl Into<String>,
        civil_equivalent: impl Into<String>,
    ) -> Self {
        self.civil_equivalents
            .insert(religious_concept.into(), civil_equivalent.into());
        self
    }
    /// Creates an Islamic law system.
    pub fn islamic() -> Self {
        Self::new(ReligiousLawType::Islamic)
            .with_integration_level(0.9)
            .with_principle("Quran as primary source of law")
            .with_principle("Hadith (Prophet's traditions) as secondary source")
            .with_principle("Ijma (scholarly consensus)")
            .with_principle("Qiyas (analogical reasoning)")
            .with_source("Quran")
            .with_source("Sunnah")
            .with_source("Scholarly interpretations (fiqh)")
            .with_equivalent("mahr", "marriage settlement")
            .with_equivalent("talaq", "divorce")
            .with_equivalent("zakat", "charitable tax")
            .with_equivalent("riba", "usury/interest prohibition")
    }
    /// Creates a Jewish law system.
    pub fn jewish() -> Self {
        Self::new(ReligiousLawType::Jewish)
            .with_integration_level(0.3)
            .with_principle("Torah as divine law")
            .with_principle("Talmudic interpretation")
            .with_principle("Rabbinical authority")
            .with_source("Torah (Written Law)")
            .with_source("Talmud (Oral Law)")
            .with_source("Responsa literature")
            .with_equivalent("get", "religious divorce decree")
            .with_equivalent("ketubah", "marriage contract")
            .with_equivalent("heter iska", "business partnership permitting profit")
    }
    /// Creates a Hindu law system.
    pub fn hindu() -> Self {
        Self::new(ReligiousLawType::Hindu)
            .with_integration_level(0.7)
            .with_principle("Dharma (righteous duty)")
            .with_principle("Karma (action and consequence)")
            .with_principle("Varna (social order)")
            .with_source("Vedas")
            .with_source("Smritis (legal texts)")
            .with_source("Dharmashastra")
            .with_equivalent("vivaha", "marriage")
            .with_equivalent("sampatti", "property")
    }
}
/// Screen reader optimizer with enhanced WCAG compliance.
#[derive(Debug, Clone)]
pub struct ScreenReaderOptimizer {
    /// Target WCAG level
    wcag_level: WCAGLevel,
    /// Include skip links
    include_skip_links: bool,
    /// Add landmark roles
    add_landmarks: bool,
    /// Locale for language-specific optimization
    locale: Locale,
}
impl ScreenReaderOptimizer {
    /// Creates a new screen reader optimizer.
    pub fn new(wcag_level: WCAGLevel, locale: Locale) -> Self {
        Self {
            wcag_level,
            include_skip_links: true,
            add_landmarks: true,
            locale,
        }
    }
    /// Sets whether to include skip links.
    pub fn with_skip_links(mut self, include: bool) -> Self {
        self.include_skip_links = include;
        self
    }
    /// Sets whether to add landmark roles.
    pub fn with_landmarks(mut self, add: bool) -> Self {
        self.add_landmarks = add;
        self
    }
    /// Optimizes HTML for screen readers.
    pub fn optimize_html(&self, html: &str) -> String {
        let mut result = html.to_string();
        if !result.contains("<html") {
            result = format!(
                "<html lang=\"{}\">\n{}\n</html>",
                self.locale.language, result
            );
        }
        if self.include_skip_links {
            let skip_link = self.generate_skip_link();
            result = format!("{}\n{}", skip_link, result);
        }
        if self.add_landmarks {
            result = self.add_landmark_roles(&result);
        }
        result = self.enhance_headings(&result);
        result = self.add_image_alt_reminders(&result);
        result
    }
    fn generate_skip_link(&self) -> String {
        let link_text = match self.locale.language.as_str() {
            "en" => "Skip to main content",
            "ja" => "メインコンテンツへスキップ",
            "es" => "Saltar al contenido principal",
            "fr" => "Passer au contenu principal",
            "de" => "Zum Hauptinhalt springen",
            _ => "Skip to main content",
        };
        format!(
            "<a href=\"#main-content\" class=\"skip-link\">{}</a>",
            link_text
        )
    }
    fn add_landmark_roles(&self, html: &str) -> String {
        html.replace("<nav>", "<nav role=\"navigation\">")
            .replace("<main>", "<main role=\"main\" id=\"main-content\">")
            .replace("<header>", "<header role=\"banner\">")
            .replace("<footer>", "<footer role=\"contentinfo\">")
            .replace("<aside>", "<aside role=\"complementary\">")
            .replace("<form>", "<form role=\"form\">")
    }
    fn enhance_headings(&self, html: &str) -> String {
        html.to_string()
    }
    fn add_image_alt_reminders(&self, html: &str) -> String {
        html.replace("<img ", "<img alt=\"[ADD DESCRIPTION]\" ")
    }
    /// Generates accessible legal document structure.
    pub fn generate_document_structure(&self, title: &str, sections: Vec<(&str, &str)>) -> String {
        let mut html = format!(
            "<!DOCTYPE html>\n<html lang=\"{}\">\n<head>\n<meta charset=\"UTF-8\">\n<title>{}</title>\n</head>\n<body>\n",
            self.locale.language, title
        );
        if self.include_skip_links {
            html.push_str(&self.generate_skip_link());
            html.push('\n');
        }
        html.push_str("<main role=\"main\" id=\"main-content\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", title));
        for (section_title, section_content) in sections {
            html.push_str(&format!(
                "<section>\n<h2>{}</h2>\n<p>{}</p>\n</section>\n",
                section_title, section_content
            ));
        }
        html.push_str("</main>\n</body>\n</html>");
        html
    }
    /// Checks WCAG compliance.
    pub fn check_compliance(&self, html: &str) -> ComplianceReport {
        let mut issues = Vec::new();
        if !html.contains("lang=") {
            issues.push("Missing language attribute on html element".to_string());
        }
        if matches!(self.wcag_level, WCAGLevel::AA | WCAGLevel::AAA)
            && !html.contains("skip-link")
            && !html.contains("Skip to")
        {
            issues.push("Missing skip link (required for AA/AAA)".to_string());
        }
        if !html.contains("<h1") {
            issues.push("Missing h1 heading (main page title)".to_string());
        }
        if self.add_landmarks && !html.contains("role=") {
            issues.push("Missing ARIA landmark roles".to_string());
        }
        let is_compliant = issues.is_empty();
        ComplianceReport {
            wcag_level: self.wcag_level,
            is_compliant,
            issues,
        }
    }
}
/// Historical calendar converter.
#[derive(Debug, Clone)]
pub struct HistoricalCalendarConverter {
    /// Source calendar
    source_calendar: HistoricalCalendar,
}
impl HistoricalCalendarConverter {
    /// Creates a new historical calendar converter.
    pub fn new(source_calendar: HistoricalCalendar) -> Self {
        Self { source_calendar }
    }
    /// Converts a Julian date to Gregorian.
    /// Returns (year, month, day) in Gregorian calendar.
    pub fn julian_to_gregorian(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let a = (14 - month) / 12;
        let y = year + 4800 - a as i32;
        let m = month + 12 * a - 3;
        let jdn = day as i32 + (153 * m as i32 + 2) / 5 + 365 * y + y / 4 - 32083;
        let a = jdn + 32044;
        let b = (4 * a + 3) / 146097;
        let c = a - (146097 * b) / 4;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;
        let greg_day = e - (153 * m + 2) / 5 + 1;
        let greg_month = m + 3 - 12 * (m / 10);
        let greg_year = 100 * b + d - 4800 + m / 10;
        (greg_year, greg_month as u32, greg_day as u32)
    }
    /// Converts a Gregorian date to Julian.
    /// Returns (year, month, day) in Julian calendar.
    pub fn gregorian_to_julian(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let a = (14 - month) / 12;
        let y = year + 4800 - a as i32;
        let m = month + 12 * a - 3;
        let jdn =
            day as i32 + (153 * m as i32 + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
        let c = jdn + 32082;
        let d = (4 * c + 3) / 1461;
        let e = c - (1461 * d) / 4;
        let m = (5 * e + 2) / 153;
        let jul_day = e - (153 * m + 2) / 5 + 1;
        let jul_month = m + 3 - 12 * (m / 10);
        let jul_year = d - 4800 + m / 10;
        (jul_year, jul_month as u32, jul_day as u32)
    }
    /// Calculates the difference in days between Julian and Gregorian calendars.
    pub fn julian_gregorian_offset(&self, year: i32) -> i32 {
        if year < 1582 {
            0
        } else {
            let centuries = (year - 1600) / 100;
            centuries * 3 / 4 + 10
        }
    }
    /// Formats a date in historical calendar notation.
    pub fn format_historical_date(&self, year: i32, month: u32, day: u32) -> String {
        match self.source_calendar {
            HistoricalCalendar::Julian => {
                format!("{} {} {} (O.S.)", day, self.month_name_latin(month), year)
            }
            HistoricalCalendar::Gregorian => {
                format!("{} {} {} (N.S.)", day, self.month_name_latin(month), year)
            }
            HistoricalCalendar::Roman => self.format_roman_date(year, month, day),
            HistoricalCalendar::FrenchRevolutionary => {
                self.format_french_revolutionary_date(year, month, day)
            }
        }
    }
    fn month_name_latin(&self, month: u32) -> &'static str {
        match month {
            1 => "Januarius",
            2 => "Februarius",
            3 => "Martius",
            4 => "Aprilis",
            5 => "Maius",
            6 => "Junius",
            7 => "Julius",
            8 => "Augustus",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
    fn format_roman_date(&self, _year: i32, month: u32, day: u32) -> String {
        let month_name = self.month_name_latin(month);
        format!("a.d. {} {}", day, month_name)
    }
    pub(crate) fn format_french_revolutionary_date(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> String {
        let revolutionary_months = [
            "Vendémiaire",
            "Brumaire",
            "Frimaire",
            "Nivôse",
            "Pluviôse",
            "Ventôse",
            "Germinal",
            "Floréal",
            "Prairial",
            "Messidor",
            "Thermidor",
            "Fructidor",
        ];
        let month_name = if month <= 12 {
            revolutionary_months[(month - 1) as usize]
        } else {
            "Sansculottides"
        };
        format!("{} {} An {}", day, month_name, year)
    }
}
/// Reading level adjuster for adaptive content.
#[derive(Debug, Clone)]
pub struct ReadingLevelAdjuster {
    /// Target reading level
    target_level: TargetReadingLevel,
    /// Plain language generator
    generator: PlainLanguageGenerator,
    /// Maximum iterations for adjustment
    max_iterations: usize,
}
impl ReadingLevelAdjuster {
    /// Creates a new reading level adjuster.
    pub fn new(target_level: TargetReadingLevel, locale: Locale) -> Self {
        let generator = PlainLanguageGenerator::new(target_level.grade_level(), locale);
        Self {
            target_level,
            generator,
            max_iterations: 3,
        }
    }
    /// Sets the maximum iterations for adjustment.
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }
    /// Adds a custom jargon replacement.
    pub fn add_jargon_replacement(
        mut self,
        legal_term: impl Into<String>,
        plain_term: impl Into<String>,
    ) -> Self {
        self.generator = self
            .generator
            .add_jargon_replacement(legal_term, plain_term);
        self
    }
    /// Adjusts text to target reading level.
    pub fn adjust(&self, text: &str) -> AdjustedText {
        let original_level = self.generator.estimate_reading_level(text);
        let mut current_text = text.to_string();
        let mut iterations = 0;
        while iterations < self.max_iterations
            && !self.generator.meets_target(&current_text)
            && iterations < 10
        {
            current_text = self.generator.simplify(&current_text);
            iterations += 1;
        }
        let final_level = self.generator.estimate_reading_level(&current_text);
        AdjustedText {
            original: text.to_string(),
            adjusted: current_text,
            original_level,
            final_level,
            target_level: self.target_level.grade_level(),
            iterations,
            meets_target: final_level <= self.target_level.grade_level(),
        }
    }
}
/// Reading level to adjust to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetReadingLevel {
    /// Elementary (grades 3-5, Flesch-Kincaid 3-5)
    Elementary,
    /// Middle school (grades 6-8, Flesch-Kincaid 6-8)
    MiddleSchool,
    /// High school (grades 9-12, Flesch-Kincaid 9-12)
    HighSchool,
    /// College (undergraduate, Flesch-Kincaid 13-16)
    College,
    /// Professional (graduate+, Flesch-Kincaid 16+)
    Professional,
}
impl TargetReadingLevel {
    /// Returns the Flesch-Kincaid grade level.
    pub fn grade_level(&self) -> f64 {
        match self {
            TargetReadingLevel::Elementary => 4.0,
            TargetReadingLevel::MiddleSchool => 7.0,
            TargetReadingLevel::HighSchool => 10.0,
            TargetReadingLevel::College => 14.0,
            TargetReadingLevel::Professional => 18.0,
        }
    }
}
/// Reading level assessor for legal documents.
/// Calculates readability metrics like Flesch-Kincaid grade level.
#[derive(Debug)]
pub struct ReadingLevelAssessor;
impl ReadingLevelAssessor {
    /// Creates a new reading level assessor.
    pub fn new() -> Self {
        Self
    }
    /// Calculates Flesch Reading Ease score (0-100).
    /// Higher scores indicate easier readability.
    /// 90-100: Very Easy (5th grade)
    /// 80-90: Easy (6th grade)
    /// 70-80: Fairly Easy (7th grade)
    /// 60-70: Standard (8th-9th grade)
    /// 50-60: Fairly Difficult (10th-12th grade)
    /// 30-50: Difficult (College)
    /// 0-30: Very Difficult (College graduate)
    pub fn flesch_reading_ease(&self, text: &str) -> f32 {
        let sentences = self.count_sentences(text);
        let words = self.count_words(text);
        let syllables = self.count_syllables(text);
        if sentences == 0 || words == 0 {
            return 0.0;
        }
        let avg_sentence_length = words as f32 / sentences as f32;
        let avg_syllables_per_word = syllables as f32 / words as f32;
        206.835 - (1.015 * avg_sentence_length) - (84.6 * avg_syllables_per_word)
    }
    /// Calculates Flesch-Kincaid Grade Level.
    /// Returns the U.S. grade level required to understand the text.
    pub fn flesch_kincaid_grade(&self, text: &str) -> f32 {
        let sentences = self.count_sentences(text);
        let words = self.count_words(text);
        let syllables = self.count_syllables(text);
        if sentences == 0 || words == 0 {
            return 0.0;
        }
        let avg_sentence_length = words as f32 / sentences as f32;
        let avg_syllables_per_word = syllables as f32 / words as f32;
        (0.39 * avg_sentence_length) + (11.8 * avg_syllables_per_word) - 15.59
    }
    /// Counts sentences in text.
    fn count_sentences(&self, text: &str) -> usize {
        text.split(['.', '!', '?'])
            .filter(|s| !s.trim().is_empty())
            .count()
    }
    /// Counts words in text.
    fn count_words(&self, text: &str) -> usize {
        text.split_whitespace().count()
    }
    /// Counts syllables in text (simplified heuristic).
    fn count_syllables(&self, text: &str) -> usize {
        let words: Vec<&str> = text.split_whitespace().collect();
        words
            .iter()
            .map(|word| self.count_syllables_in_word(word))
            .sum()
    }
    /// Counts syllables in a single word (simplified algorithm).
    fn count_syllables_in_word(&self, word: &str) -> usize {
        let word = word.to_lowercase();
        let word = word.trim_matches(|c: char| !c.is_alphabetic());
        if word.is_empty() {
            return 0;
        }
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        let mut count = 0;
        let mut prev_was_vowel = false;
        for ch in word.chars() {
            let is_vowel = vowels.contains(&ch);
            if is_vowel && !prev_was_vowel {
                count += 1;
            }
            prev_was_vowel = is_vowel;
        }
        if word.ends_with('e') && count > 1 {
            count -= 1;
        }
        count.max(1)
    }
    /// Provides a readability assessment.
    pub fn assess(&self, text: &str) -> ReadabilityReport {
        let ease = self.flesch_reading_ease(text);
        let grade = self.flesch_kincaid_grade(text);
        let difficulty = if ease >= 90.0 {
            "Very Easy"
        } else if ease >= 80.0 {
            "Easy"
        } else if ease >= 70.0 {
            "Fairly Easy"
        } else if ease >= 60.0 {
            "Standard"
        } else if ease >= 50.0 {
            "Fairly Difficult"
        } else if ease >= 30.0 {
            "Difficult"
        } else {
            "Very Difficult"
        };
        ReadabilityReport {
            flesch_reading_ease: ease,
            flesch_kincaid_grade: grade,
            difficulty: difficulty.to_string(),
            word_count: self.count_words(text),
            sentence_count: self.count_sentences(text),
            syllable_count: self.count_syllables(text),
        }
    }
}
/// WCAG compliance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Target WCAG level
    pub wcag_level: WCAGLevel,
    /// Whether the content is compliant
    pub is_compliant: bool,
    /// List of compliance issues
    pub issues: Vec<String>,
}
/// Legal domain specialization for speech recognition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalSpeechDomain {
    /// Court proceedings and trials.
    CourtProceedings,
    /// Legal depositions and testimonies.
    Depositions,
    /// Legal consultations and advice sessions.
    Consultations,
    /// Contract negotiations.
    ContractNegotiations,
    /// Arbitration and mediation proceedings.
    ArbitrationMediation,
    /// General legal speech.
    General,
}
/// Date/time formatter for legal deadlines.
#[derive(Debug, Clone)]
pub struct DateTimeFormatter {
    locale: Locale,
}
impl DateTimeFormatter {
    /// Creates a new date/time formatter.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_i18n::{DateTimeFormatter, Locale};
    ///
    /// let locale = Locale::new("ja").with_country("JP");
    /// let formatter = DateTimeFormatter::new(locale);
    ///
    /// let date = formatter.format_date(2024, 12, 19);
    /// assert_eq!(date, "2024年12月19日");
    ///
    /// let time = formatter.format_time(14, 30);
    /// assert_eq!(time, "14:30");
    /// ```
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }
    /// Formats a date in the locale's format.
    /// Uses ISO 8601 as input: "YYYY-MM-DD"
    pub fn format_date(&self, year: i32, month: u32, day: u32) -> String {
        match self.locale.language.as_str() {
            "ja" => format!("{}年{}月{}日", year, month, day),
            "zh" => format!("{}年{}月{}日", year, month, day),
            "en" if self.locale.country.as_deref() == Some("US") => {
                format!("{:02}/{:02}/{}", month, day, year)
            }
            "en" => format!("{:02}/{:02}/{}", day, month, year),
            "de" | "fr" | "es" | "it" => format!("{:02}.{:02}.{}", day, month, year),
            _ => format!("{}-{:02}-{:02}", year, month, day),
        }
    }
    /// Formats a time in the locale's format.
    pub fn format_time(&self, hour: u32, minute: u32) -> String {
        match self.locale.language.as_str() {
            "en" if self.locale.country.as_deref() == Some("US") => {
                let (h, ampm) = if hour == 0 {
                    (12, "AM")
                } else if hour < 12 {
                    (hour, "AM")
                } else if hour == 12 {
                    (12, "PM")
                } else {
                    (hour - 12, "PM")
                };
                format!("{:02}:{:02} {}", h, minute, ampm)
            }
            _ => format!("{:02}:{:02}", hour, minute),
        }
    }
    /// Formats a complete datetime.
    pub fn format_datetime(
        &self,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
    ) -> String {
        format!(
            "{} {}",
            self.format_date(year, month, day),
            self.format_time(hour, minute)
        )
    }
}
/// Registry of dialect terminologies.
#[derive(Debug, Default)]
pub struct DialectTerminologyRegistry {
    dialects: Vec<DialectTerminology>,
}
impl DialectTerminologyRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a registry with default dialect terminologies.
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        let mut scottish =
            DialectTerminology::new(Locale::new("en").with_country("GB"), "Scottish Legal");
        scottish.add_term("lawyer", "advocate");
        scottish.add_term("notary_public", "notary public and conveyancer");
        scottish.add_term("real_estate", "heritable property");
        scottish.add_term("personal_property", "moveable property");
        scottish.add_term("mortgage", "standard security");
        scottish.add_term("will", "testament");
        scottish.add_term("plaintiff", "pursuer");
        scottish.add_term("defendant", "defender");
        registry.add_dialect(scottish);
        let mut louisiana =
            DialectTerminology::new(Locale::new("en").with_country("US"), "Louisiana Legal");
        louisiana.add_term("county", "parish");
        louisiana.add_term("real_estate", "immovable property");
        louisiana.add_term("personal_property", "movable property");
        louisiana.add_term("common_law", "civil law");
        louisiana.add_term("deed", "act of sale");
        louisiana.add_term("will", "testament");
        registry.add_dialect(louisiana);
        let mut quebec =
            DialectTerminology::new(Locale::new("fr").with_country("CA"), "Québec Legal");
        quebec.add_term("avocat", "avocat(e)");
        quebec.add_term("notaire", "notaire");
        quebec.add_term("jurisprudence", "jurisprudence québécoise");
        quebec.add_term("code_civil", "Code civil du Québec");
        registry.add_dialect(quebec);
        let mut hong_kong =
            DialectTerminology::new(Locale::new("en").with_country("HK"), "Hong Kong Legal");
        hong_kong.add_term("lawyer", "solicitor or barrister");
        hong_kong.add_term("attorney", "solicitor");
        hong_kong.add_term("court", "Court of Final Appeal / High Court");
        hong_kong.add_term("basic_law", "Basic Law");
        registry.add_dialect(hong_kong);
        let mut australian =
            DialectTerminology::new(Locale::new("en").with_country("AU"), "Australian Legal");
        australian.add_term("lawyer", "solicitor or barrister");
        australian.add_term("attorney", "solicitor");
        australian.add_term("corporation", "company (Pty Ltd)");
        australian.add_term(
            "supreme_court",
            "High Court of Australia (federal) / State Supreme Courts",
        );
        registry.add_dialect(australian);
        registry
    }
    /// Adds a dialect to the registry.
    pub fn add_dialect(&mut self, dialect: DialectTerminology) {
        self.dialects.push(dialect);
    }
    /// Finds a dialect by name and locale.
    pub fn find_dialect(&self, locale: &Locale, dialect_name: &str) -> Option<&DialectTerminology> {
        self.dialects.iter().find(|d| {
            d.base_locale.language == locale.language
                && d.base_locale.country == locale.country
                && d.dialect_name == dialect_name
        })
    }
    /// Gets all dialects for a locale.
    pub fn get_dialects_for_locale(&self, locale: &Locale) -> Vec<&DialectTerminology> {
        self.dialects
            .iter()
            .filter(|d| {
                d.base_locale.language == locale.language && d.base_locale.country == locale.country
            })
            .collect()
    }
}
/// Key clause extractor for legal documents.
#[derive(Debug, Default)]
pub struct ClauseExtractor {
    /// Patterns for identifying clause types
    patterns: HashMap<ClauseType, Vec<String>>,
}
impl ClauseExtractor {
    /// Creates a new clause extractor.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a clause extractor with default patterns.
    pub fn with_defaults() -> Self {
        let mut extractor = Self::new();
        extractor.add_pattern(ClauseType::Confidentiality, "confidential");
        extractor.add_pattern(ClauseType::Confidentiality, "non-disclosure");
        extractor.add_pattern(ClauseType::Confidentiality, "proprietary information");
        extractor.add_pattern(ClauseType::Indemnification, "indemnify");
        extractor.add_pattern(ClauseType::Indemnification, "hold harmless");
        extractor.add_pattern(ClauseType::Indemnification, "defend");
        extractor.add_pattern(ClauseType::LimitationOfLiability, "limitation of liability");
        extractor.add_pattern(ClauseType::LimitationOfLiability, "shall not be liable");
        extractor.add_pattern(ClauseType::LimitationOfLiability, "in no event");
        extractor.add_pattern(ClauseType::Termination, "termination");
        extractor.add_pattern(ClauseType::Termination, "terminate");
        extractor.add_pattern(ClauseType::Termination, "cancellation");
        extractor.add_pattern(ClauseType::GoverningLaw, "governing law");
        extractor.add_pattern(ClauseType::GoverningLaw, "choice of law");
        extractor.add_pattern(ClauseType::GoverningLaw, "governed by");
        extractor.add_pattern(ClauseType::DisputeResolution, "arbitration");
        extractor.add_pattern(ClauseType::DisputeResolution, "mediation");
        extractor.add_pattern(ClauseType::DisputeResolution, "dispute resolution");
        extractor.add_pattern(ClauseType::ForceMajeure, "force majeure");
        extractor.add_pattern(ClauseType::ForceMajeure, "act of god");
        extractor.add_pattern(ClauseType::Warranty, "warranty");
        extractor.add_pattern(ClauseType::Warranty, "warrants");
        extractor.add_pattern(ClauseType::Warranty, "representations");
        extractor.add_pattern(ClauseType::Payment, "payment");
        extractor.add_pattern(ClauseType::Payment, "compensation");
        extractor.add_pattern(ClauseType::Payment, "fee");
        extractor.add_pattern(ClauseType::IntellectualProperty, "intellectual property");
        extractor.add_pattern(ClauseType::IntellectualProperty, "patent");
        extractor.add_pattern(ClauseType::IntellectualProperty, "copyright");
        extractor.add_pattern(ClauseType::IntellectualProperty, "trademark");
        extractor
    }
    /// Adds a pattern for a clause type.
    pub fn add_pattern(&mut self, clause_type: ClauseType, pattern: impl Into<String>) {
        self.patterns
            .entry(clause_type)
            .or_default()
            .push(pattern.into());
    }
    /// Extracts clauses from document text.
    pub fn extract(&self, text: &str) -> Vec<ExtractedClause> {
        let mut clauses = Vec::new();
        let text_lower = text.to_lowercase();
        for (clause_type, patterns) in &self.patterns {
            for pattern in patterns {
                let pattern_lower = pattern.to_lowercase();
                let mut start = 0;
                while let Some(pos) = text_lower[start..].find(&pattern_lower) {
                    let absolute_pos = start + pos;
                    let context_start = absolute_pos.saturating_sub(50);
                    let context_end = (absolute_pos + pattern.len() + 150).min(text.len());
                    let context = &text[context_start..context_end];
                    let confidence = self.calculate_confidence(context, pattern);
                    if confidence > 0.3 {
                        clauses.push(ExtractedClause {
                            clause_type: clause_type.clone(),
                            text: context.to_string(),
                            position: absolute_pos,
                            confidence,
                        });
                    }
                    start = absolute_pos + pattern.len();
                }
            }
        }
        clauses.sort_by_key(|c| c.position);
        clauses
    }
    #[allow(dead_code)]
    fn calculate_confidence(&self, context: &str, pattern: &str) -> f64 {
        let mut score: f64 = 0.5;
        if context
            .trim_start()
            .to_lowercase()
            .starts_with(&pattern.to_lowercase())
        {
            score += 0.2;
        }
        let legal_keywords = ["shall", "hereby", "whereas", "pursuant", "notwithstanding"];
        for keyword in &legal_keywords {
            if context.to_lowercase().contains(keyword) {
                score += 0.05;
            }
        }
        score.min(1.0)
    }
}
/// Quality estimator for AI-powered translations.
#[derive(Debug, Clone)]
pub struct QualityEstimator {
    /// Minimum threshold for acceptable quality (0.0 to 1.0).
    pub min_threshold: f32,
}
impl QualityEstimator {
    /// Creates a new quality estimator.
    pub fn new(min_threshold: f32) -> Self {
        Self {
            min_threshold: min_threshold.clamp(0.0, 1.0),
        }
    }
    /// Creates a quality estimator with default threshold (0.7).
    pub fn with_defaults() -> Self {
        Self::new(0.7)
    }
    /// Estimates quality for a translation (simplified heuristic-based approach).
    pub fn estimate_quality(
        &self,
        source_text: &str,
        translated_text: &str,
        source_locale: Locale,
        target_locale: Locale,
    ) -> QualityEstimationReport {
        let mut report = QualityEstimationReport::new(
            source_text,
            translated_text,
            source_locale,
            target_locale,
        );
        let length_ratio = translated_text.len() as f32 / source_text.len().max(1) as f32;
        let semantic_score = if (0.5..=2.0).contains(&length_ratio) {
            0.8
        } else {
            0.5
        };
        report.add_score(
            AIQualityScore::new(QualityMetric::SemanticAccuracy, semantic_score)
                .with_explanation("Based on length ratio between source and target"),
        );
        let has_legal_terms = translated_text.to_lowercase().contains("law")
            || translated_text.to_lowercase().contains("contract")
            || translated_text.to_lowercase().contains("court");
        let term_score = if has_legal_terms { 0.75 } else { 0.6 };
        report.add_score(
            AIQualityScore::new(QualityMetric::TerminologicalConsistency, term_score)
                .with_explanation("Based on presence of legal terminology"),
        );
        let has_punctuation = translated_text.ends_with('.')
            || translated_text.ends_with('?')
            || translated_text.ends_with('!');
        let grammar_score = if has_punctuation { 0.85 } else { 0.7 };
        report.add_score(
            AIQualityScore::new(QualityMetric::GrammaticalCorrectness, grammar_score)
                .with_explanation("Based on basic sentence structure"),
        );
        let fluency_score = if !translated_text.is_empty() && translated_text.len() > 10 {
            0.8
        } else {
            0.4
        };
        report.add_score(
            AIQualityScore::new(QualityMetric::Fluency, fluency_score)
                .with_explanation("Based on text length and non-emptiness"),
        );
        report
    }
    /// Checks if a translation meets the minimum quality threshold.
    pub fn is_acceptable(&self, report: &QualityEstimationReport) -> bool {
        report.meets_threshold(self.min_threshold)
    }
}
/// Legal document numbering formatter.
#[derive(Debug, Clone)]
pub struct DocumentNumbering {
    style: NumberingStyle,
    #[allow(dead_code)]
    locale: Locale,
}
impl DocumentNumbering {
    /// Creates a new document numbering formatter.
    pub fn new(style: NumberingStyle, locale: Locale) -> Self {
        Self { style, locale }
    }
    /// Formats a hierarchical number (e.g., Article 1, Section 2.1, etc.).
    pub fn format(&self, level: usize, number: usize) -> String {
        match self.style {
            NumberingStyle::Article => match level {
                0 => format!("Article {}", number),
                1 => format!("Section {}", number),
                2 => format!("Paragraph {}", number),
                3 => format!("Clause {}", number),
                _ => format!("Subclause {}", number),
            },
            NumberingStyle::Section => match level {
                0 => format!("Section {}", number),
                1 => self.format_subsection(number),
                2 => self.format_roman_lowercase(number),
                _ => format!("({})", number),
            },
            NumberingStyle::Chapter => match level {
                0 => format!("Chapter {}", number),
                1 => format!("Part {}", self.format_uppercase_letter(number)),
                2 => format!("Subdivision ({})", number),
                _ => format!("({})", self.format_lowercase_letter(number)),
            },
            NumberingStyle::Hierarchical => match level {
                0 => format!("{}.", number),
                1 => format!("{}.", self.format_lowercase_letter(number)),
                2 => format!("{}.", self.format_roman_lowercase(number)),
                _ => format!("({})", number),
            },
            NumberingStyle::Parenthetical => match level {
                0 => format!("({})", number),
                1 => format!("({})", self.format_lowercase_letter(number)),
                2 => format!("({})", self.format_roman_lowercase(number)),
                _ => format!("({})", number),
            },
        }
    }
    fn format_lowercase_letter(&self, n: usize) -> String {
        if n == 0 || n > 26 {
            return n.to_string();
        }
        ((b'a' + (n as u8) - 1) as char).to_string()
    }
    fn format_uppercase_letter(&self, n: usize) -> String {
        if n == 0 || n > 26 {
            return n.to_string();
        }
        ((b'A' + (n as u8) - 1) as char).to_string()
    }
    fn format_subsection(&self, n: usize) -> String {
        format!("Subsection {}", self.format_lowercase_letter(n))
    }
    fn format_roman_lowercase(&self, n: usize) -> String {
        match n {
            1 => "i".to_string(),
            2 => "ii".to_string(),
            3 => "iii".to_string(),
            4 => "iv".to_string(),
            5 => "v".to_string(),
            6 => "vi".to_string(),
            7 => "vii".to_string(),
            8 => "viii".to_string(),
            9 => "ix".to_string(),
            10 => "x".to_string(),
            _ => n.to_string(),
        }
    }
}
/// Level of equivalence between terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquivalenceLevel {
    /// Exact equivalence (same legal meaning)
    Exact,
    /// Approximate equivalence (similar but with differences)
    Approximate,
    /// Loose equivalence (related concept)
    Loose,
    /// No direct equivalent (concept doesn't exist)
    NoEquivalent,
}
/// Extracted clause from a legal document.
#[derive(Debug, Clone)]
pub struct ExtractedClause {
    /// Type of clause
    pub clause_type: ClauseType,
    /// Text of the clause
    pub text: String,
    /// Position in document (character offset)
    pub position: usize,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}
/// Complete analysis of a legal document.
#[derive(Debug)]
pub struct DocumentAnalysis {
    /// Extracted clauses
    pub clauses: Vec<ExtractedClause>,
    /// Identified parties
    pub parties: Vec<IdentifiedParty>,
    /// Extracted obligations
    pub obligations: Vec<ExtractedObligation>,
    /// Extracted deadlines
    pub deadlines: Vec<ExtractedDeadline>,
    /// Detected jurisdiction
    pub jurisdiction: Option<(String, f64)>,
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// Identified risk factors
    pub risk_factors: Vec<RiskFactor>,
}
/// Legal translation prompt template.
#[derive(Debug, Clone)]
pub struct LegalPromptTemplate {
    /// The system prompt for legal translation.
    pub system_prompt: String,
    /// The user prompt template with placeholders.
    pub user_prompt_template: String,
    /// Whether to include legal context in the prompt.
    pub include_legal_context: bool,
    /// Whether to preserve legal citations.
    pub preserve_citations: bool,
    /// Whether to maintain formality level.
    pub maintain_formality: bool,
}
impl LegalPromptTemplate {
    /// Creates a new legal prompt template.
    pub fn new(system_prompt: &str, user_prompt_template: &str) -> Self {
        Self {
            system_prompt: system_prompt.to_string(),
            user_prompt_template: user_prompt_template.to_string(),
            include_legal_context: true,
            preserve_citations: true,
            maintain_formality: true,
        }
    }
    /// Creates a default legal translation prompt template.
    pub fn default_legal_translation() -> Self {
        Self::new(
            "You are a professional legal translator with expertise in multiple legal systems. \
             Translate the following legal text accurately while preserving legal terminology, \
             citations, and formality. Maintain the precise legal meaning and structure.",
            "Translate the following legal text from {source_locale} to {target_locale}:\n\n\
             Text: {text}\n\n\
             Legal Context: {legal_context}\n\n\
             Please provide an accurate legal translation.",
        )
    }
    /// Sets whether to include legal context.
    pub fn with_legal_context(mut self, include: bool) -> Self {
        self.include_legal_context = include;
        self
    }
    /// Sets whether to preserve citations.
    pub fn with_citation_preservation(mut self, preserve: bool) -> Self {
        self.preserve_citations = preserve;
        self
    }
    /// Sets whether to maintain formality.
    pub fn with_formality(mut self, maintain: bool) -> Self {
        self.maintain_formality = maintain;
        self
    }
    /// Renders the prompt with the given parameters.
    pub fn render(
        &self,
        text: &str,
        source_locale: &Locale,
        target_locale: &Locale,
        legal_context: Option<&str>,
    ) -> String {
        let mut prompt = self.user_prompt_template.clone();
        prompt = prompt.replace("{text}", text);
        prompt = prompt.replace("{source_locale}", &source_locale.to_string());
        prompt = prompt.replace("{target_locale}", &target_locale.to_string());
        prompt = prompt.replace(
            "{legal_context}",
            legal_context.unwrap_or("General legal text"),
        );
        prompt
    }
}
/// Multilingual semantic embedding for legal text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticEmbedding {
    /// The embedded text.
    pub text: String,
    /// Language locale of the text.
    pub locale: Locale,
    /// Embedding vector (typically 768 or 1024 dimensions).
    pub vector: Vec<f32>,
    /// Embedding model used.
    pub model: String,
    /// Legal domain context (if applicable).
    pub domain: Option<LegalSpeechDomain>,
}
impl SemanticEmbedding {
    /// Creates a new semantic embedding.
    pub fn new(
        text: impl Into<String>,
        locale: Locale,
        vector: Vec<f32>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            text: text.into(),
            locale,
            vector,
            model: model.into(),
            domain: None,
        }
    }
    /// Sets the legal domain.
    pub fn with_domain(mut self, domain: LegalSpeechDomain) -> Self {
        self.domain = Some(domain);
        self
    }
    /// Computes cosine similarity with another embedding.
    pub fn cosine_similarity(&self, other: &SemanticEmbedding) -> f32 {
        if self.vector.len() != other.vector.len() {
            return 0.0;
        }
        let dot_product: f32 = self
            .vector
            .iter()
            .zip(other.vector.iter())
            .map(|(a, b)| a * b)
            .sum();
        let magnitude_a: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }
        dot_product / (magnitude_a * magnitude_b)
    }
    /// Returns the dimensionality of the embedding vector.
    pub fn dimensions(&self) -> usize {
        self.vector.len()
    }
}
/// Context-aware disambiguator for legal terms.
#[derive(Debug, Clone)]
pub struct ContextDisambiguator {
    /// Map of term to disambiguation contexts.
    contexts: HashMap<String, Vec<DisambiguationContext>>,
}
impl ContextDisambiguator {
    /// Creates a new context disambiguator.
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }
    /// Creates a disambiguator with default legal term contexts.
    pub fn with_defaults() -> Self {
        let mut disambiguator = Self::new();
        disambiguator.add_context(
            "action",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "civil_law", 0.8)
                .with_explanation("In civil law, 'action' typically refers to a lawsuit"),
        );
        disambiguator.add_context(
            "action",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "criminal_law", 0.7)
                .with_explanation("In criminal law, 'action' may refer to prosecution"),
        );
        disambiguator.add_context(
            "consideration",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "contract_law", 0.9)
                .with_explanation(
                    "In contract law, 'consideration' is a requirement for valid contracts",
                ),
        );
        disambiguator.add_context(
            "trust",
            DisambiguationContext::new(DisambiguationType::LegalDomain, "property_law", 0.85)
                .with_explanation("In property law, 'trust' is a fiduciary relationship"),
        );
        disambiguator.add_context(
            "bill",
            DisambiguationContext::new(DisambiguationType::DocumentType, "legislation", 0.8)
                .with_explanation("In legislative context, 'bill' is a proposed law"),
        );
        disambiguator.add_context(
            "bill",
            DisambiguationContext::new(DisambiguationType::DocumentType, "commercial", 0.6)
                .with_explanation("In commercial context, 'bill' may refer to an invoice"),
        );
        disambiguator
    }
    /// Adds a disambiguation context for a term.
    pub fn add_context(&mut self, term: &str, context: DisambiguationContext) {
        self.contexts
            .entry(term.to_lowercase())
            .or_default()
            .push(context);
    }
    /// Gets disambiguation contexts for a term.
    pub fn get_contexts(&self, term: &str) -> Vec<&DisambiguationContext> {
        self.contexts
            .get(&term.to_lowercase())
            .map(|contexts| contexts.iter().collect())
            .unwrap_or_default()
    }
    /// Gets the best disambiguation context for a term given a type.
    pub fn get_best_context(
        &self,
        term: &str,
        disambiguation_type: DisambiguationType,
    ) -> Option<&DisambiguationContext> {
        self.get_contexts(term)
            .into_iter()
            .filter(|ctx| ctx.disambiguation_type == disambiguation_type)
            .max_by(|a, b| {
                a.confidence
                    .partial_cmp(&b.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
    /// Returns the number of terms with disambiguation contexts.
    pub fn term_count(&self) -> usize {
        self.contexts.len()
    }
    /// Returns the total number of disambiguation contexts.
    pub fn context_count(&self) -> usize {
        self.contexts.values().map(|v| v.len()).sum()
    }
}
