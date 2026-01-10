//! Case Law Types (判例型定義)
//!
//! This module defines the core types for Japanese case law (判例) database system.

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

/// Court level in the Japanese judicial system (裁判所の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CourtLevel {
    /// Supreme Court (最高裁判所 - Saikō-saibansho)
    Supreme,
    /// High Court (高等裁判所 - Kōtō-saibansho)
    High,
    /// District Court (地方裁判所 - Chihō-saibansho)
    District,
    /// Family Court (家庭裁判所 - Katei-saibansho)
    Family,
    /// Summary Court (簡易裁判所 - Kan'i-saibansho)
    Summary,
}

impl CourtLevel {
    /// Returns the Japanese name of the court level
    pub fn japanese_name(&self) -> &'static str {
        match self {
            CourtLevel::Supreme => "最高裁判所",
            CourtLevel::High => "高等裁判所",
            CourtLevel::District => "地方裁判所",
            CourtLevel::Family => "家庭裁判所",
            CourtLevel::Summary => "簡易裁判所",
        }
    }

    /// Returns the English name of the court level
    pub fn english_name(&self) -> &'static str {
        match self {
            CourtLevel::Supreme => "Supreme Court",
            CourtLevel::High => "High Court",
            CourtLevel::District => "District Court",
            CourtLevel::Family => "Family Court",
            CourtLevel::Summary => "Summary Court",
        }
    }

    /// Returns the binding precedent level (0 = highest authority)
    pub fn precedent_weight(&self) -> u8 {
        match self {
            CourtLevel::Supreme => 0, // Highest authority
            CourtLevel::High => 1,
            CourtLevel::District => 2,
            CourtLevel::Family => 2,
            CourtLevel::Summary => 3, // Lowest authority
        }
    }
}

/// Specific court location (裁判所の所在地)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Court {
    /// Court level
    pub level: CourtLevel,
    /// Court location (e.g., "Tokyo", "Osaka")
    pub location: Option<String>,
    /// Division/Chamber (e.g., "Civil Division", "Criminal Division")
    pub division: Option<String>,
}

impl Court {
    /// Creates a new court
    pub fn new(level: CourtLevel) -> Self {
        Self {
            level,
            location: None,
            division: None,
        }
    }

    /// Sets the court location
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Sets the court division
    pub fn with_division(mut self, division: impl Into<String>) -> Self {
        self.division = Some(division.into());
        self
    }

    /// Returns the full court name
    pub fn full_name(&self) -> String {
        let mut parts = Vec::new();

        if let Some(location) = &self.location {
            parts.push(location.clone());
        }

        parts.push(self.level.japanese_name().to_string());

        if let Some(division) = &self.division {
            parts.push(division.clone());
        }

        parts.join(" ")
    }
}

/// Legal area classification (法分野)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalArea {
    /// Civil law (民事法 - Minji-hō)
    Civil,
    /// Criminal law (刑事法 - Keiji-hō)
    Criminal,
    /// Administrative law (行政法 - Gyōsei-hō)
    Administrative,
    /// Constitutional law (憲法 - Kenpō)
    Constitutional,
    /// Commercial law (商法 - Shōhō)
    Commercial,
    /// Labor law (労働法 - Rōdō-hō)
    Labor,
    /// Intellectual property (知的財産法 - Chiteki-zaisan-hō)
    IntellectualProperty,
    /// Consumer protection (消費者保護法 - Shōhisha-hogo-hō)
    ConsumerProtection,
    /// Tax law (税法 - Zei-hō)
    Tax,
    /// Family law (家族法 - Kazoku-hō)
    Family,
}

impl LegalArea {
    /// Returns the Japanese name
    pub fn japanese_name(&self) -> &'static str {
        match self {
            LegalArea::Civil => "民事法",
            LegalArea::Criminal => "刑事法",
            LegalArea::Administrative => "行政法",
            LegalArea::Constitutional => "憲法",
            LegalArea::Commercial => "商法",
            LegalArea::Labor => "労働法",
            LegalArea::IntellectualProperty => "知的財産法",
            LegalArea::ConsumerProtection => "消費者保護法",
            LegalArea::Tax => "税法",
            LegalArea::Family => "家族法",
        }
    }
}

/// Case outcome/result (判決結果)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaseOutcome {
    /// Plaintiff/Appellant wins (原告勝訴 - Genkoku shōso)
    PlaintiffWins,
    /// Defendant/Appellee wins (被告勝訴 - Hikoku shōso)
    DefendantWins,
    /// Partially granted (一部認容 - Ichibu nin'yō)
    PartiallyGranted,
    /// Appeal dismissed (上告棄却 - Jōkoku kikyaku)
    AppealDismissed,
    /// Appeal granted (上告認容 - Jōkoku nin'yō)
    AppealGranted,
    /// Remanded (差戻し - Sashi-modoshi)
    Remanded,
    /// Settlement (和解 - Wakai)
    Settlement,
    /// Dismissed (却下 - Kyakka)
    Dismissed,
}

/// Court decision metadata (判決メタデータ)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseMetadata {
    /// Case number (事件番号 - Jiken bangō)
    pub case_number: String,
    /// Decision date (判決日 - Hanketsu-bi)
    pub decision_date: DateTime<Utc>,
    /// Court that issued the decision
    pub court: Court,
    /// Legal area
    pub legal_area: LegalArea,
    /// Case outcome
    pub outcome: CaseOutcome,
    /// Keywords for search (キーワード)
    pub keywords: Vec<String>,
    /// Cited statutes (引用法令)
    pub cited_statutes: Vec<String>,
}

impl CaseMetadata {
    /// Creates a new case metadata
    pub fn new(
        case_number: impl Into<String>,
        decision_date: DateTime<Utc>,
        court: Court,
        legal_area: LegalArea,
        outcome: CaseOutcome,
    ) -> Self {
        Self {
            case_number: case_number.into(),
            decision_date,
            court,
            legal_area,
            outcome,
            keywords: Vec::new(),
            cited_statutes: Vec::new(),
        }
    }

    /// Adds a keyword
    pub fn add_keyword(&mut self, keyword: impl Into<String>) {
        self.keywords.push(keyword.into());
    }

    /// Adds a cited statute
    pub fn add_cited_statute(&mut self, statute: impl Into<String>) {
        self.cited_statutes.push(statute.into());
    }
}

/// Party information (当事者情報)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    /// Party type (e.g., "Plaintiff", "Defendant", "Appellant")
    pub party_type: String,
    /// Party name (anonymized if necessary)
    pub name: String,
    /// Representative/Counsel
    pub representative: Option<String>,
}

/// Holding/Rule of law (判旨・法理)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    /// Main legal principle established (判旨 - Hanji)
    pub principle: String,
    /// Detailed reasoning (理由 - Riyū)
    pub reasoning: String,
    /// Related statutes
    pub related_statutes: Vec<String>,
    /// Is this a leading case? (リーディングケース)
    pub is_leading_case: bool,
}

/// Full court decision (裁判例 - Saiban-rei)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtDecision {
    /// Unique identifier
    pub id: String,
    /// Case metadata
    pub metadata: CaseMetadata,
    /// Parties involved
    pub parties: Vec<Party>,
    /// Case summary (事案の概要 - Jian no gaiyō)
    pub summary: String,
    /// Main holdings/legal principles
    pub holdings: Vec<Holding>,
    /// Full text of the decision (判決全文 - Hanketsu zenbun)
    pub full_text: Option<String>,
    /// URL to official source (e.g., courts.go.jp)
    pub source_url: Option<String>,
    /// Additional notes
    pub notes: Option<String>,
}

impl CourtDecision {
    /// Creates a new court decision
    pub fn new(id: impl Into<String>, metadata: CaseMetadata, summary: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            metadata,
            parties: Vec::new(),
            summary: summary.into(),
            holdings: Vec::new(),
            full_text: None,
            source_url: None,
            notes: None,
        }
    }

    /// Adds a party
    pub fn add_party(&mut self, party: Party) {
        self.parties.push(party);
    }

    /// Adds a holding
    pub fn add_holding(&mut self, holding: Holding) {
        self.holdings.push(holding);
    }

    /// Sets the full text
    pub fn with_full_text(mut self, text: impl Into<String>) -> Self {
        self.full_text = Some(text.into());
        self
    }

    /// Sets the source URL
    pub fn with_source_url(mut self, url: impl Into<String>) -> Self {
        self.source_url = Some(url.into());
        self
    }

    /// Returns whether this is a Supreme Court decision
    pub fn is_supreme_court_decision(&self) -> bool {
        self.metadata.court.level == CourtLevel::Supreme
    }

    /// Returns the precedent weight (0 = highest authority)
    pub fn precedent_weight(&self) -> u8 {
        self.metadata.court.level.precedent_weight()
    }

    /// Returns the year of the decision
    pub fn decision_year(&self) -> i32 {
        self.metadata.decision_date.year()
    }

    /// Checks if this decision cites a specific statute
    pub fn cites_statute(&self, statute_name: &str) -> bool {
        self.metadata
            .cited_statutes
            .iter()
            .any(|s| s.contains(statute_name))
    }

    /// Checks if this decision contains a keyword
    pub fn contains_keyword(&self, keyword: &str) -> bool {
        let keyword_lower = keyword.to_lowercase();

        // Check in keywords
        if self
            .metadata
            .keywords
            .iter()
            .any(|k| k.to_lowercase().contains(&keyword_lower))
        {
            return true;
        }

        // Check in summary
        if self.summary.to_lowercase().contains(&keyword_lower) {
            return true;
        }

        // Check in holdings
        if self
            .holdings
            .iter()
            .any(|h| h.principle.to_lowercase().contains(&keyword_lower))
        {
            return true;
        }

        false
    }
}

/// Search query for case law database (判例検索クエリ)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CaseSearchQuery {
    /// Keywords to search for
    pub keywords: Vec<String>,
    /// Court level filter
    pub court_level: Option<CourtLevel>,
    /// Legal area filter
    pub legal_area: Option<LegalArea>,
    /// Start date filter (inclusive)
    pub date_from: Option<DateTime<Utc>>,
    /// End date filter (inclusive)
    pub date_to: Option<DateTime<Utc>>,
    /// Cited statute filter
    pub cited_statute: Option<String>,
    /// Case outcome filter
    pub outcome: Option<CaseOutcome>,
    /// Maximum number of results
    pub limit: Option<usize>,
}

impl CaseSearchQuery {
    /// Creates a new empty search query
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a keyword
    pub fn with_keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keywords.push(keyword.into());
        self
    }

    /// Sets the court level filter
    pub fn with_court_level(mut self, level: CourtLevel) -> Self {
        self.court_level = Some(level);
        self
    }

    /// Sets the legal area filter
    pub fn with_legal_area(mut self, area: LegalArea) -> Self {
        self.legal_area = Some(area);
        self
    }

    /// Sets the date range filter
    pub fn with_date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.date_from = Some(from);
        self.date_to = Some(to);
        self
    }

    /// Sets the cited statute filter
    pub fn with_cited_statute(mut self, statute: impl Into<String>) -> Self {
        self.cited_statute = Some(statute.into());
        self
    }

    /// Sets the case outcome filter
    pub fn with_outcome(mut self, outcome: CaseOutcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    /// Sets the result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Search result with relevance score (検索結果)
#[derive(Debug, Clone)]
pub struct CaseSearchResult {
    /// The court decision
    pub decision: CourtDecision,
    /// Relevance score (0.0 to 1.0)
    pub relevance_score: f64,
    /// Matching keywords found
    pub matching_keywords: Vec<String>,
}

impl CaseSearchResult {
    /// Creates a new search result
    pub fn new(decision: CourtDecision, relevance_score: f64) -> Self {
        Self {
            decision,
            relevance_score,
            matching_keywords: Vec::new(),
        }
    }

    /// Adds a matching keyword
    pub fn add_matching_keyword(&mut self, keyword: impl Into<String>) {
        self.matching_keywords.push(keyword.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_court_level_precedent_weight() {
        assert_eq!(CourtLevel::Supreme.precedent_weight(), 0);
        assert_eq!(CourtLevel::High.precedent_weight(), 1);
        assert_eq!(CourtLevel::District.precedent_weight(), 2);
        assert_eq!(CourtLevel::Summary.precedent_weight(), 3);
    }

    #[test]
    fn test_court_full_name() {
        let court = Court::new(CourtLevel::District)
            .with_location("Tokyo")
            .with_division("Civil Division");

        assert_eq!(court.full_name(), "Tokyo 地方裁判所 Civil Division");
    }

    #[test]
    fn test_court_decision_creation() {
        let metadata = CaseMetadata::new(
            "平成30年(オ)第1234号",
            Utc::now(),
            Court::new(CourtLevel::Supreme),
            LegalArea::Civil,
            CaseOutcome::PlaintiffWins,
        );

        let decision = CourtDecision::new("case-001", metadata, "Test case summary");

        assert_eq!(decision.id, "case-001");
        assert!(decision.is_supreme_court_decision());
        assert_eq!(decision.precedent_weight(), 0);
    }

    #[test]
    fn test_search_query_builder() {
        let query = CaseSearchQuery::new()
            .with_keyword("tort")
            .with_court_level(CourtLevel::Supreme)
            .with_legal_area(LegalArea::Civil)
            .with_limit(10);

        assert_eq!(query.keywords.len(), 1);
        assert_eq!(query.court_level, Some(CourtLevel::Supreme));
        assert_eq!(query.legal_area, Some(LegalArea::Civil));
        assert_eq!(query.limit, Some(10));
    }

    #[test]
    fn test_case_keyword_search() {
        let mut metadata = CaseMetadata::new(
            "令和2年(受)第100号",
            Utc::now(),
            Court::new(CourtLevel::Supreme),
            LegalArea::Civil,
            CaseOutcome::AppealGranted,
        );
        metadata.add_keyword("不法行為");
        metadata.add_keyword("損害賠償");

        let decision = CourtDecision::new("case-002", metadata, "不法行為に基づく損害賠償請求");

        assert!(decision.contains_keyword("不法行為"));
        assert!(decision.contains_keyword("損害賠償"));
        assert!(!decision.contains_keyword("契約違反"));
    }

    #[test]
    fn test_statute_citation() {
        let mut metadata = CaseMetadata::new(
            "平成29年(行ウ)第50号",
            Utc::now(),
            Court::new(CourtLevel::High),
            LegalArea::Administrative,
            CaseOutcome::DefendantWins,
        );
        metadata.add_cited_statute("行政事件訴訟法第3条");
        metadata.add_cited_statute("憲法第14条");

        let decision = CourtDecision::new("case-003", metadata, "行政処分取消訴訟");

        assert!(decision.cites_statute("行政事件訴訟法"));
        assert!(decision.cites_statute("憲法"));
        assert!(!decision.cites_statute("民法"));
    }

    #[test]
    fn test_legal_area_names() {
        assert_eq!(LegalArea::Civil.japanese_name(), "民事法");
        assert_eq!(LegalArea::Labor.japanese_name(), "労働法");
        assert_eq!(
            LegalArea::IntellectualProperty.japanese_name(),
            "知的財産法"
        );
    }
}
