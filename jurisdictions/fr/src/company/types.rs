//! Company law types (Types de droit des sociétés)
//!
//! This module provides type definitions for French company law under the Code de commerce.
//!
//! # Overview of French Company Forms
//!
//! The Code de commerce recognizes multiple company types, but three dominate commercial practice:
//! SA (Société Anonyme), SARL (Société à Responsabilité Limitée), and SAS (Société par Actions Simplifiée).
//!
//! ## Historical Evolution of Company Forms
//!
//! ### Napoleonic Era to 1867
//!
//! - **1807**: Code de commerce establishes société en nom collectif (general partnership) and
//!   société en commandite (limited partnership). No true limited liability corporations yet.
//! - **1863-1867**: Industrial revolution demands capital mobilization. Government authorizes
//!   individual SAs (Crédit Lyonnais, Compagnie Générale Transatlantique) on case-by-case basis.
//! - **1867**: Landmark liberalization—SA formation becomes registration-based (no prior authorization).
//!   This unleashes industrial financing (railroads, mines, steel).
//!
//! ### 20th Century: Democratization of Company Forms
//!
//! - **1925**: SARL created to provide limited liability for small businesses. Germany's GmbH (1892)
//!   inspired French lawmakers. Initial minimum capital: 20,000 francs.
//! - **1966**: Comprehensive company law reform (Loi sur les sociétés commerciales). Modernizes SA,
//!   SARL structures. Introduces clearer governance rules.
//! - **1994**: SAS created for flexibility. Targets family businesses and private equity. No board
//!   requirement—governance defined entirely by statuts (articles of incorporation).
//! - **1999**: SAS capital minimum reduced to 1 franc (then €1 in 2002 euro conversion). Explosive growth.
//! - **2003**: SARL capital minimum abolished (from €7,500 to €1). EURL (single-member SARL) popularized.
//!
//! ### 21st Century: SAS Dominance
//!
//! By 2020, SAS accounts for 65% of new incorporations, SARL 30%, SA only 2%. Despite low formation
//! numbers, SA dominates large enterprises (CAC 40: 38/40 companies are SA or SCA variant).
//!
//! ## Comparative Table: SA vs. SARL vs. SAS
//!
//! | Feature | SA | SARL | SAS |
//! |---------|----|----|-----|
//! | **Year introduced** | 1867 | 1925 | 1994 |
//! | **Code articles** | L225-1 to L225-270 | L223-1 to L223-43 | L227-1 to L227-20 |
//! | **Minimum capital** | €37,000 | €1 | €1 |
//! | **Maximum partners** | Unlimited | 100 | Unlimited |
//! | **Shares/parts** | Actions (shares) | Parts sociales (partnership interests) | Actions (shares) |
//! | **Transferability** | Freely transferable (unless restricted in statuts) | Restricted (spouse/heirs/partners priority) | Defined in statuts (usually restricted) |
//! | **Governance** | Mandatory board (3-18) or directoire/conseil | Gérant(s) - manager(s) | Président (required); otherwise free |
//! | **Public offering** | Yes (required for Euronext) | No | No (except rare SAS-listed) |
//! | **Audit requirement** | Always (commissaire aux comptes) | If exceeds thresholds | If exceeds thresholds |
//! | **Typical users** | Large corps, IPO candidates | SMEs, family firms | Startups, PE/VC, family holdcos |
//! | **2023 incorporations** | 2,500 (~2%) | 32,000 (~30%) | 70,000 (~65%) |
//! | **CAC 40 presence** | 38/40 (including SCA) | 0/40 | 2/40 (Michelin family holdco) |
//!
//! ## Key Design Differences
//!
//! ### SA: Prestige and Public Markets
//!
//! - **Rationale**: SA designed for large-scale capital raising, especially public offerings.
//!   €37,000 minimum signals seriousness to investors.
//! - **Governance**: Rigid but predictable. Board of 3-18 directors with fixed rules appeals
//!   to institutional investors (pension funds, insurance companies).
//! - **Regulation**: Heavily regulated (AMF oversight if listed). Transparency requirements
//!   (annual reports, AGM disclosure) exceed SARL/SAS.
//!
//! ### SARL: Middle-Class Entrepreneurship
//!
//! - **Rationale**: 1925 creation aimed to give small merchants/artisans limited liability
//!   without SA's complexity and cost. "Société de personnes à responsabilité limitée"
//!   (personal company with limited liability).
//! - **Governance**: Flexible manager (gérant) system. Can have single gérant (like sole
//!   proprietorship) or multiple gérants (partnership-style).
//! - **Transfer restrictions**: Parts sociales harder to sell than actions. Spouses, heirs,
//!   and existing partners have pre-emptive rights. Protects family/small business character.
//!
//! ### SAS: Contractual Freedom
//!
//! - **Rationale**: 1994 introduction sought to attract businesses that found SA too rigid
//!   and SARL unsuitable for outside investors. Inspired by US LLC and Delaware corporations.
//! - **Governance**: Total freedom in statuts. Can mimic SA board, adopt US-style CEO+officers,
//!   or create bespoke governance (conseil stratégique, comité d'investissement, etc.).
//! - **Use cases**:
//!   - **Startups**: VCs demand SAS for liquidation preferences, anti-dilution, drag-along rights
//!   - **Private equity**: SAS allows complex governance (board observers, reserved matters)
//!   - **Family offices**: Holdcos use SAS to customize succession/voting arrangements
//!   - **Joint ventures**: Partners design bespoke governance reflecting their agreement
//!
//! ## International Equivalents
//!
//! | France | Germany | Japan | USA | UK | Netherlands | Switzerland | China |
//! |--------|---------|-------|-----|----|-----------|-----------|----|
//! | **SA** | AG (Aktiengesellschaft) | KK (株式会社) | C Corp (Delaware) | PLC | NV | AG | 股份有限公司 |
//! | **SARL** | GmbH | GK (合同会社) | LLC | Ltd (private) | BV | GmbH/Sàrl | 有限责任公司 |
//! | **SAS** | GmbH & Co. KG (hybrid) | - | LLC (flexible) | Ltd (flexible articles) | BV (flexible) | - | - |
//!
//! **Notes**:
//! - German AG requires two-tier board (Aufsichtsrat + Vorstand); French SA allows choice (monist/dualist)
//! - Japanese KK abolished ¥10M capital minimum in 2006 (now ¥1 like France), but cultural preference
//!   for higher capital persists (prestige signal)
//! - US C Corp has no capital minimum; par value is nominal ($0.01 common). "Thin capitalization"
//!   accepted unless piercing corporate veil.
//! - UK PLC requires £50,000 capital (EU directive compliance); Ltd requires £1 (similar to French evolution)
//! - Chinese 股份有限公司 (stock company) requires CNY 5M minimum historically, abolished 2013 but re-introduced
//!   sector-specific minimums (banking, insurance)
//!
//! ## Modern Policy Debates
//!
//! ### Should SA Capital Minimum Be Reduced?
//!
//! **Arguments for reduction** (€37,000 → €1, like SARL/SAS):
//! - Encourages entrepreneurship (fewer barriers to incorporation)
//! - Harmonizes with EU trend (many states reduced minimums post-2008 crisis)
//! - Capital maintenance rules (distributions restrictions) protect creditors more than fixed minimum
//! - Most SAs have far more than €37,000 anyway (median CAC 40 capital: €1.5 billion)
//!
//! **Arguments against reduction**:
//! - Preserves SA prestige (€37,000 signals serious enterprise to investors, creditors)
//! - Creditor protection (small buffer against insolvency, especially for non-listed SAs)
//! - Public offering readiness (companies planning IPO should demonstrate capital adequacy)
//! - Discourages "SA shopping" (entrepreneurs shouldn't use SA just for name prestige without substance)
//!
//! **Current consensus**: Maintain €37,000. Policymakers satisfied with three-tier system (SA for
//! large, SARL/SAS for small). No legislative momentum for change as of 2024.
//!
//! ### SAS vs. SA for IPOs?
//!
//! Euronext Paris historically required SA (or SCA - société en commandite par actions). 2024 regulations
//! still strongly prefer SA:
//! - **Governance transparency**: SA's fixed board structure (3-18) more transparent than SAS's
//!   bespoke governance (investors must read statuts carefully)
//! - **Minority protection**: SA has detailed rules for AGO/AGE (quorum, voting thresholds). SAS
//!   statuts can vary widely, potentially disadvantaging minorities.
//! - **International comparability**: Foreign investors understand SA (analogous to AG/PLC/KK).
//!   SAS is uniquely French, requiring education.
//!
//! **Rare SAS listings**: A few SAS have listed (e.g., certain ETFs structured as SAS), but major
//! companies convert to SA before IPO (BlaBlaCar, Criteo converted SA pre-IPO discussions).
//!
//! ## Specialized Variants (Not Covered in This Module)
//!
//! - **SCA (Société en Commandite par Actions)**: Hybrid with general partners (unlimited liability)
//!   and limited partners (shares). Used by families for control (Hermès, Lagardère, Michelin).
//!   General partners cannot be removed by shareholders, providing anti-takeover protection.
//! - **SELAS/SELARL**: Liberal profession companies (lawyers, doctors, architects). Similar to SAS/SARL
//!   but for regulated professions.
//! - **SE (Societas Europaea)**: European Company. Supranational form governed by EU regulation.
//!   Rare in France (Airbus SE is prominent example).
//! - **Coopératives (SCOP, SCIC)**: Worker cooperatives. Different governance (democratic, one person
//!   one vote). Growing in social economy sector.

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Company type (Type de société)
///
/// The three main types of French commercial companies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompanyType {
    /// Société Anonyme (Stock Company) - Articles L225-1+
    ///
    /// - Minimum capital: €37,000
    /// - Board of directors: 3-18 members
    /// - Suitable for large corporations
    /// - Can be publicly traded
    SA,

    /// Société à Responsabilité Limitée (Limited Liability Company) - Articles L223-1+
    ///
    /// - Minimum capital: €1
    /// - Maximum partners: 100
    /// - Most common for SMEs
    /// - Flexible management structure
    SARL,

    /// Société par Actions Simplifiée (Simplified Joint-Stock Company) - Articles L227-1+
    ///
    /// - Minimum capital: €1
    /// - Very flexible governance (defined by statuts)
    /// - No board requirement
    /// - Popular for startups
    SAS,
}

impl CompanyType {
    /// Get the French legal name
    #[must_use]
    pub fn french_name(self) -> &'static str {
        match self {
            Self::SA => "Société Anonyme",
            Self::SARL => "Société à Responsabilité Limitée",
            Self::SAS => "Société par Actions Simplifiée",
        }
    }

    /// Get the abbreviation
    #[must_use]
    pub fn abbreviation(self) -> &'static str {
        match self {
            Self::SA => "SA",
            Self::SARL => "SARL",
            Self::SAS => "SAS",
        }
    }

    /// Get minimum capital requirement in euros
    #[must_use]
    pub fn minimum_capital(self) -> u64 {
        match self {
            Self::SA => Capital::SA_MINIMUM,
            Self::SARL | Self::SAS => Capital::SARL_SAS_MINIMUM,
        }
    }
}

/// Share capital (Capital social)
///
/// Represents the capital of a company in euros.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Capital {
    /// Amount in euros
    pub amount_eur: u64,
}

impl Capital {
    /// Minimum capital for SA (Article L225-1)
    pub const SA_MINIMUM: u64 = 37_000;

    /// Minimum capital for SARL and SAS (since 2003 reform)
    pub const SARL_SAS_MINIMUM: u64 = 1;

    /// Create a new Capital
    #[must_use]
    pub const fn new(amount_eur: u64) -> Self {
        Self { amount_eur }
    }

    /// Check if capital is valid for a given company type
    #[must_use]
    pub fn is_valid_for(self, company_type: CompanyType) -> bool {
        self.amount_eur >= company_type.minimum_capital()
    }

    /// Check if this is a small company (≤ €100,000 capital)
    #[must_use]
    pub fn is_small_company(self) -> bool {
        self.amount_eur <= 100_000
    }

    /// Check if this is a large company (> €500,000 capital)
    #[must_use]
    pub fn is_large_company(self) -> bool {
        self.amount_eur > 500_000
    }
}

/// Shareholder or partner (Associé / Actionnaire)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Shareholder {
    /// Name of shareholder (individual or company)
    pub name: String,

    /// Number of shares or parts owned
    pub shares: u64,

    /// Contribution amount in euros (apport)
    pub contribution_eur: u64,

    /// Date of entry (optional)
    pub entry_date: Option<NaiveDate>,
}

impl Shareholder {
    /// Create a new shareholder
    #[must_use]
    pub fn new(name: String, shares: u64, contribution_eur: u64) -> Self {
        Self {
            name,
            shares,
            contribution_eur,
            entry_date: None,
        }
    }

    /// Set entry date
    #[must_use]
    pub fn with_entry_date(mut self, date: NaiveDate) -> Self {
        self.entry_date = Some(date);
        self
    }
}

/// Articles of incorporation (Statuts)
///
/// The founding document of a French company.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArticlesOfIncorporation {
    /// Company name (Dénomination sociale)
    pub company_name: String,

    /// Company type
    pub company_type: CompanyType,

    /// Business purpose (Objet social)
    pub business_purpose: Vec<String>,

    /// Registered office (Siège social)
    pub head_office: String,

    /// Share capital
    pub capital: Capital,

    /// Fiscal year end (1-12 for January-December)
    pub fiscal_year_end: u8,

    /// Shareholders/partners
    pub shareholders: Vec<Shareholder>,

    /// Duration in years (max 99)
    pub duration_years: u8,

    /// Date of incorporation
    pub incorporation_date: Option<NaiveDate>,
}

impl ArticlesOfIncorporation {
    /// Create new articles of incorporation
    #[must_use]
    pub fn new(company_name: String, company_type: CompanyType, capital: Capital) -> Self {
        Self {
            company_name,
            company_type,
            business_purpose: Vec::new(),
            head_office: String::new(),
            capital,
            fiscal_year_end: 12, // December by default
            shareholders: Vec::new(),
            duration_years: 99, // Maximum duration
            incorporation_date: None,
        }
    }

    /// Add a business purpose
    #[must_use]
    pub fn with_business_purpose(mut self, purpose: String) -> Self {
        self.business_purpose.push(purpose);
        self
    }

    /// Set head office
    #[must_use]
    pub fn with_head_office(mut self, address: String) -> Self {
        self.head_office = address;
        self
    }

    /// Add a shareholder
    #[must_use]
    pub fn with_shareholder(mut self, shareholder: Shareholder) -> Self {
        self.shareholders.push(shareholder);
        self
    }

    /// Set fiscal year end
    #[must_use]
    pub fn with_fiscal_year_end(mut self, month: u8) -> Self {
        self.fiscal_year_end = month;
        self
    }

    /// Set incorporation date
    #[must_use]
    pub fn with_incorporation_date(mut self, date: NaiveDate) -> Self {
        self.incorporation_date = Some(date);
        self
    }

    /// Get total number of shares
    #[must_use]
    pub fn total_shares(&self) -> u64 {
        self.shareholders.iter().map(|s| s.shares).sum()
    }

    /// Get total contributions
    #[must_use]
    pub fn total_contributions(&self) -> u64 {
        self.shareholders.iter().map(|s| s.contribution_eur).sum()
    }

    /// Check if company name includes required suffix
    #[must_use]
    pub fn has_valid_name_suffix(&self) -> bool {
        let abbr = self.company_type.abbreviation();
        let full = self.company_type.french_name();
        self.company_name.contains(abbr) || self.company_name.contains(full)
    }
}

/// Director (Administrateur / Directeur)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Director {
    /// Name of director
    pub name: String,

    /// Appointment date
    pub appointed_date: NaiveDate,

    /// Term length in years (max 6 for SA)
    pub term_years: u8,

    /// Position type
    pub position: DirectorPosition,
}

impl Director {
    /// Create a new director
    #[must_use]
    pub fn new(name: String, appointed_date: NaiveDate, term_years: u8) -> Self {
        Self {
            name,
            appointed_date,
            term_years,
            position: DirectorPosition::Director,
        }
    }

    /// Set position
    #[must_use]
    pub fn with_position(mut self, position: DirectorPosition) -> Self {
        self.position = position;
        self
    }
}

/// Director position (Fonction)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DirectorPosition {
    /// Chairman of the board (Président du conseil)
    Chairman,
    /// CEO (Directeur général)
    CEO,
    /// Deputy CEO (Directeur général délégué)
    DeputyCEO,
    /// Regular director (Administrateur)
    Director,
    /// Outside director (Administrateur indépendant)
    OutsideDirector,
}

/// Board of directors (Conseil d'administration)
///
/// Required for SA companies (Article L225-17).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoardOfDirectors {
    /// Board members (3-18 for SA)
    pub members: Vec<Director>,

    /// Chairman name (optional - can be designated later)
    pub chairman: Option<String>,
}

impl BoardOfDirectors {
    /// Minimum board size for SA
    pub const SA_MIN_DIRECTORS: usize = 3;

    /// Maximum board size for SA
    pub const SA_MAX_DIRECTORS: usize = 18;

    /// Create a new board
    #[must_use]
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            chairman: None,
        }
    }

    /// Add a director
    #[must_use]
    pub fn with_director(mut self, director: Director) -> Self {
        self.members.push(director);
        self
    }

    /// Set chairman
    #[must_use]
    pub fn with_chairman(mut self, name: String) -> Self {
        self.chairman = Some(name);
        self
    }

    /// Check if board size is valid for SA
    #[must_use]
    pub fn is_valid_size_for_sa(&self) -> bool {
        self.members.len() >= Self::SA_MIN_DIRECTORS && self.members.len() <= Self::SA_MAX_DIRECTORS
    }

    /// Get number of directors
    #[must_use]
    pub fn size(&self) -> usize {
        self.members.len()
    }
}

impl Default for BoardOfDirectors {
    fn default() -> Self {
        Self::new()
    }
}

/// Meeting type (Type d'assemblée)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MeetingType {
    /// Ordinary general meeting (Assemblée générale ordinaire - AGO)
    OrdinaryGeneralMeeting,

    /// Extraordinary general meeting (Assemblée générale extraordinaire - AGE)
    ExtraordinaryGeneralMeeting,
}

/// Resolution type (Type de résolution)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResolutionType {
    /// Ordinary resolution (simple majority)
    Ordinary,

    /// Special resolution (2/3 majority for AGE)
    Special,
}

/// Shareholders meeting (Assemblée générale)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShareholdersMeeting {
    /// Meeting type
    pub meeting_type: MeetingType,

    /// Meeting date
    pub date: NaiveDate,

    /// Total shares represented
    pub shares_represented: u64,

    /// Total shares in company
    pub total_shares: u64,

    /// Votes in favor
    pub votes_for: u64,

    /// Votes against
    pub votes_against: u64,

    /// Abstentions
    pub abstentions: u64,
}

impl ShareholdersMeeting {
    /// Create a new shareholders meeting
    #[must_use]
    pub fn new(meeting_type: MeetingType, date: NaiveDate, total_shares: u64) -> Self {
        Self {
            meeting_type,
            date,
            shares_represented: 0,
            total_shares,
            votes_for: 0,
            votes_against: 0,
            abstentions: 0,
        }
    }

    /// Set votes
    #[must_use]
    pub fn with_votes(
        mut self,
        shares_represented: u64,
        votes_for: u64,
        votes_against: u64,
        abstentions: u64,
    ) -> Self {
        self.shares_represented = shares_represented;
        self.votes_for = votes_for;
        self.votes_against = votes_against;
        self.abstentions = abstentions;
        self
    }

    /// Calculate quorum percentage
    #[must_use]
    pub fn quorum_percentage(&self) -> f64 {
        if self.total_shares == 0 {
            0.0
        } else {
            (self.shares_represented as f64 / self.total_shares as f64) * 100.0
        }
    }

    /// Calculate approval percentage (of votes cast, excluding abstentions)
    #[must_use]
    pub fn approval_percentage(&self) -> f64 {
        let votes_cast = self.votes_for + self.votes_against;
        if votes_cast == 0 {
            0.0
        } else {
            (self.votes_for as f64 / votes_cast as f64) * 100.0
        }
    }

    /// Check if quorum is met (typically 20% for SA ordinary, 25% for extraordinary)
    #[must_use]
    pub fn has_quorum(&self, required_percentage: f64) -> bool {
        self.quorum_percentage() >= required_percentage
    }

    /// Check if resolution passed
    #[must_use]
    pub fn is_approved(&self, resolution_type: ResolutionType) -> bool {
        let required = match resolution_type {
            ResolutionType::Ordinary => 50.0, // Simple majority
            ResolutionType::Special => 66.67, // 2/3 majority
        };
        self.approval_percentage() > required
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_type_names() {
        assert_eq!(CompanyType::SA.french_name(), "Société Anonyme");
        assert_eq!(CompanyType::SARL.abbreviation(), "SARL");
        assert_eq!(CompanyType::SAS.abbreviation(), "SAS");
    }

    #[test]
    fn test_capital_minimum() {
        assert_eq!(CompanyType::SA.minimum_capital(), 37_000);
        assert_eq!(CompanyType::SARL.minimum_capital(), 1);
        assert_eq!(CompanyType::SAS.minimum_capital(), 1);
    }

    #[test]
    fn test_capital_validation() {
        let sa_capital = Capital::new(50_000);
        assert!(sa_capital.is_valid_for(CompanyType::SA));

        let insufficient_sa_capital = Capital::new(30_000);
        assert!(!insufficient_sa_capital.is_valid_for(CompanyType::SA));

        let sarl_capital = Capital::new(1);
        assert!(sarl_capital.is_valid_for(CompanyType::SARL));
    }

    #[test]
    fn test_capital_classification() {
        let small = Capital::new(50_000);
        assert!(small.is_small_company());
        assert!(!small.is_large_company());

        let large = Capital::new(1_000_000);
        assert!(!large.is_small_company());
        assert!(large.is_large_company());
    }

    #[test]
    fn test_shareholder_creation() {
        let shareholder = Shareholder::new("Fondateur 1".to_string(), 1000, 50_000);
        assert_eq!(shareholder.shares, 1000);
        assert_eq!(shareholder.contribution_eur, 50_000);
    }

    #[test]
    fn test_articles_builder() {
        let articles = ArticlesOfIncorporation::new(
            "TechCorp SA".to_string(),
            CompanyType::SA,
            Capital::new(100_000),
        )
        .with_business_purpose("Software development".to_string())
        .with_head_office("Paris".to_string())
        .with_shareholder(Shareholder::new("Founder".to_string(), 1000, 100_000));

        assert_eq!(articles.company_name, "TechCorp SA");
        assert_eq!(articles.shareholders.len(), 1);
        assert_eq!(articles.total_shares(), 1000);
    }

    #[test]
    fn test_name_suffix_validation() {
        let valid_sa = ArticlesOfIncorporation::new(
            "MyCompany SA".to_string(),
            CompanyType::SA,
            Capital::new(100_000),
        );
        assert!(valid_sa.has_valid_name_suffix());

        let invalid = ArticlesOfIncorporation::new(
            "MyCompany".to_string(),
            CompanyType::SA,
            Capital::new(100_000),
        );
        assert!(!invalid.has_valid_name_suffix());
    }

    #[test]
    fn test_board_size_validation() {
        let mut board = BoardOfDirectors::new();
        assert!(!board.is_valid_size_for_sa()); // Too few

        // Add 3 directors
        for i in 1..=3 {
            board = board.with_director(Director::new(
                format!("Director {}", i),
                chrono::Utc::now().naive_utc().date(),
                6,
            ));
        }
        assert!(board.is_valid_size_for_sa());

        assert_eq!(board.size(), 3);
    }

    #[test]
    fn test_shareholders_meeting_quorum() {
        let meeting = ShareholdersMeeting::new(
            MeetingType::OrdinaryGeneralMeeting,
            chrono::Utc::now().naive_utc().date(),
            10_000,
        )
        .with_votes(3_000, 2_000, 500, 500);

        assert_eq!(meeting.quorum_percentage(), 30.0);
        assert!(meeting.has_quorum(20.0)); // 20% required
        assert!(meeting.has_quorum(25.0)); // 25% required
    }

    #[test]
    fn test_resolution_approval() {
        let meeting = ShareholdersMeeting::new(
            MeetingType::OrdinaryGeneralMeeting,
            chrono::Utc::now().naive_utc().date(),
            10_000,
        )
        .with_votes(5_000, 3_000, 1_500, 500);

        // 3000 / (3000 + 1500) = 66.67%
        assert!(meeting.is_approved(ResolutionType::Ordinary)); // > 50%
        assert!(!meeting.is_approved(ResolutionType::Special)); // Need > 66.67%
    }
}
