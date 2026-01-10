//! Commercial Law Types (商法・会社法)
//!
//! This module provides type-safe representations of Japanese commercial law entities
//! including company types, corporate governance structures, and share management.
//!
//! # Covered Laws
//! - Companies Act (会社法 - Kaisha-hō)
//! - Commercial Code (商法 - Shōhō)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// Company Types (会社の種類) - Article 26
// ============================================================================

/// Company type as defined in Companies Act Article 26
/// (会社法第26条 - 会社の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompanyType {
    /// Stock company (株式会社 - Kabushiki-gaisha)
    /// The most common company type in Japan
    StockCompany,

    /// Limited liability company (合同会社 - Gōdō-gaisha)
    /// Popular for small businesses and foreign subsidiaries
    LLC,

    /// Limited partnership company (合資会社 - Gōshi-gaisha)
    /// Rare, with both limited and unlimited liability partners
    LimitedPartnership,

    /// General partnership company (合名会社 - Gōmei-gaisha)
    /// Rare, all partners have unlimited liability
    GeneralPartnership,
}

impl fmt::Display for CompanyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StockCompany => write!(f, "株式会社 (Kabushiki-gaisha / Stock Company)"),
            Self::LLC => write!(f, "合同会社 (Gōdō-gaisha / LLC)"),
            Self::LimitedPartnership => write!(f, "合資会社 (Gōshi-gaisha / Limited Partnership)"),
            Self::GeneralPartnership => write!(f, "合名会社 (Gōmei-gaisha / General Partnership)"),
        }
    }
}

// ============================================================================
// Capital Requirements (資本金) - Article 27
// ============================================================================

/// Capital amount in Japanese Yen
/// (資本金 - Shihon-kin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Capital {
    /// Amount in JPY (円)
    pub amount_jpy: u64,
}

impl Capital {
    /// Minimum capital requirement (1 yen as of 2006 amendment)
    /// (最低資本金 - Saitei shihon-kin)
    pub const MINIMUM: u64 = 1;

    /// Creates a new capital amount
    pub fn new(amount_jpy: u64) -> Self {
        Self { amount_jpy }
    }

    /// Checks if capital meets minimum requirements
    pub fn is_valid(&self) -> bool {
        self.amount_jpy >= Self::MINIMUM
    }

    /// Checks if this is a "small company" (資本金1億円以下)
    /// Used for various regulatory thresholds
    pub fn is_small_company(&self) -> bool {
        self.amount_jpy <= 100_000_000
    }

    /// Checks if this is a "large company" (大会社)
    /// Capital over 500 million yen (Article 2, Item 6)
    pub fn is_large_company(&self) -> bool {
        self.amount_jpy > 500_000_000
    }
}

impl fmt::Display for Capital {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "¥{}", self.amount_jpy)
    }
}

// ============================================================================
// Articles of Incorporation (定款) - Article 38
// ============================================================================

/// Articles of incorporation (定款 - Teikan)
/// Required document for company formation (Article 26)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticlesOfIncorporation {
    /// Company name (商号 - Shōgō)
    /// Must include company type suffix (e.g., "株式会社")
    pub company_name: String,

    /// Business purpose (事業目的 - Jigyō mokuteki)
    /// Required by Article 27, Item 1
    pub business_purposes: Vec<String>,

    /// Head office location (本店所在地 - Honten shozaichi)
    /// Required by Article 27, Item 2
    pub head_office_location: String,

    /// Total authorized shares (発行可能株式総数 - Hakkō kanō kabushiki sōsū)
    /// Required for stock companies (Article 37)
    pub authorized_shares: Option<u64>,

    /// Capital amount (資本金 - Shihon-kin)
    pub capital: Capital,

    /// Fiscal year end (決算期 - Kessan-ki)
    /// Month (1-12)
    pub fiscal_year_end_month: u8,

    /// Incorporators (発起人 - Hokki-nin)
    /// At least one required (Article 25)
    pub incorporators: Vec<Incorporator>,

    /// Date of establishment (設立日 - Setsuritsu-bi)
    pub establishment_date: Option<DateTime<Utc>>,
}

/// Incorporator (発起人 - Hokki-nin)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Incorporator {
    /// Name (氏名 - Shimei)
    pub name: String,

    /// Address (住所 - Jūsho)
    pub address: String,

    /// Number of shares subscribed (引受株式数 - Hikiuke kabushiki-sū)
    pub shares_subscribed: Option<u64>,

    /// Investment amount (出資額 - Shusshi-gaku)
    pub investment_amount_jpy: u64,
}

// ============================================================================
// Corporate Governance (コーポレートガバナンス)
// ============================================================================

/// Shareholders meeting (株主総会 - Kabunushi sōkai) - Article 295
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareholdersMeeting {
    /// Meeting type
    pub meeting_type: MeetingType,

    /// Meeting date
    pub meeting_date: DateTime<Utc>,

    /// Agenda items (議案 - Gian)
    pub agenda_items: Vec<AgendaItem>,

    /// Quorum met (定足数 - Teisoku-sū)
    pub quorum_met: bool,

    /// Total voting rights present
    pub voting_rights_present: u64,

    /// Total voting rights issued
    pub voting_rights_total: u64,
}

/// Meeting type (総会の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeetingType {
    /// Ordinary general meeting (定時株主総会 - Teiji kabunushi sōkai)
    /// Must be held within 3 months after fiscal year end (Article 296)
    OrdinaryGeneralMeeting,

    /// Extraordinary general meeting (臨時株主総会 - Rinji kabunushi sōkai)
    ExtraordinaryGeneralMeeting,
}

/// Agenda item (議案 - Gian)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgendaItem {
    /// Item number
    pub item_number: u32,

    /// Description (議案内容 - Gian naiyō)
    pub description: String,

    /// Resolution type required
    pub resolution_type: ResolutionType,

    /// Votes in favor (賛成票 - Sansei-hyō)
    pub votes_favor: u64,

    /// Votes against (反対票 - Hantai-hyō)
    pub votes_against: u64,

    /// Abstentions (棄権 - Kiken)
    pub abstentions: u64,

    /// Result
    pub result: Option<ResolutionResult>,
}

/// Resolution type (決議要件 - Ketsugi yōken)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionType {
    /// Ordinary resolution (普通決議 - Futsū ketsugi)
    /// Majority of voting rights present (Article 309-1)
    OrdinaryResolution,

    /// Special resolution (特別決議 - Tokubetsu ketsugi)
    /// 2/3 of voting rights present (Article 309-2)
    SpecialResolution,

    /// Extraordinary resolution (特殊決議 - Tokushu ketsugi)
    /// Higher thresholds for specific matters (Article 309-3, 309-4)
    ExtraordinaryResolution,
}

/// Resolution result (決議結果 - Ketsugi kekka)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionResult {
    /// Approved (可決 - Kaketsu)
    Approved,

    /// Rejected (否決 - Hiketsu)
    Rejected,
}

/// Board of directors (取締役会 - Torishimari-yaku kai) - Article 362
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardOfDirectors {
    /// Directors (取締役 - Torishimari-yaku)
    /// Minimum 3 for companies with board (Article 331-5)
    pub directors: Vec<Director>,

    /// Board meeting frequency (開催頻度 - Kaisai hindō)
    pub meeting_frequency: Option<String>,
}

/// Director (取締役 - Torishimari-yaku)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Director {
    /// Name (氏名 - Shimei)
    pub name: String,

    /// Position (役職 - Yakushoku)
    pub position: DirectorPosition,

    /// Term start date (就任日 - Shūnin-bi)
    pub term_start: DateTime<Utc>,

    /// Term end date (退任日 - Tainin-bi)
    /// Maximum 2 years for stock companies (Article 332-1)
    pub term_end: Option<DateTime<Utc>>,
}

/// Director position (取締役の役職)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DirectorPosition {
    /// Representative director (代表取締役 - Daihyō torishimari-yaku)
    RepresentativeDirector,

    /// President (社長 - Shachō)
    President,

    /// Executive director (専務取締役 - Senmu torishimari-yaku)
    ExecutiveDirector,

    /// Managing director (常務取締役 - Jōmu torishimari-yaku)
    ManagingDirector,

    /// Director (取締役 - Torishimari-yaku)
    Director,

    /// Outside director (社外取締役 - Shagai torishimari-yaku)
    OutsideDirector,
}

/// Corporate auditor system (監査役会 - Kansa-yaku kai) - Article 381
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorporateAuditors {
    /// Auditors (監査役 - Kansa-yaku)
    /// Minimum 3 for audit board (Article 335-3)
    pub auditors: Vec<CorporateAuditor>,
}

/// Corporate auditor (監査役 - Kansa-yaku)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorporateAuditor {
    /// Name (氏名 - Shimei)
    pub name: String,

    /// Full-time or part-time (常勤・非常勤 - Jōkin/hi-jōkin)
    pub is_full_time: bool,

    /// Outside auditor (社外監査役 - Shagai kansa-yaku)
    /// At least half must be outside for large companies (Article 335-3)
    pub is_outside: bool,

    /// Term start date (就任日 - Shūnin-bi)
    pub term_start: DateTime<Utc>,

    /// Term end date (退任日 - Tainin-bi)
    /// Maximum 4 years (Article 336-1)
    pub term_end: Option<DateTime<Utc>>,
}

// ============================================================================
// Shares and Capital (株式・資本)
// ============================================================================

/// Share type (株式の種類 - Kabushiki no shurui) - Article 107
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareType {
    /// Share class name (種類株式の名称 - Shurui kabushiki no meishō)
    pub class_name: String,

    /// Voting rights (議決権 - Giketsu-ken)
    pub has_voting_rights: bool,

    /// Dividend preference (配当優先権 - Haitō yūsen-ken)
    pub dividend_preference: Option<DividendPreference>,

    /// Transfer restriction (譲渡制限 - Jōto seigen)
    pub transfer_restricted: bool,

    /// Conversion rights (転換権 - Tenkan-ken)
    pub conversion_rights: Option<ConversionRights>,
}

/// Dividend preference (配当優先権)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DividendPreference {
    /// Preferred dividend rate (%)
    pub preferred_rate_percent: f64,

    /// Cumulative (累積的 - Ruiseki-teki)
    pub is_cumulative: bool,
}

/// Conversion rights (転換権)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionRights {
    /// Target share class
    pub target_class: String,

    /// Conversion ratio
    pub conversion_ratio: f64,
}

/// Share transfer (株式譲渡 - Kabushiki jōto) - Article 113
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareTransfer {
    /// Transferor (譲渡人 - Jōto-nin)
    pub transferor: String,

    /// Transferee (譲受人 - Jōju-nin)
    pub transferee: String,

    /// Number of shares (株式数 - Kabushiki-sū)
    pub number_of_shares: u64,

    /// Share class
    pub share_class: String,

    /// Transfer date (譲渡日 - Jōto-bi)
    pub transfer_date: DateTime<Utc>,

    /// Board approval required (取締役会承認要 - Torishimari-yaku kai shōnin-yō)
    pub requires_board_approval: bool,

    /// Board approval obtained (承認済み - Shōnin-zumi)
    pub board_approval_obtained: bool,
}

/// Share issuance (募集株式の発行 - Boshū kabushiki no hakkō) - Article 199
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareIssuance {
    /// Number of shares to issue (発行株式数 - Hakkō kabushiki-sū)
    pub shares_to_issue: u64,

    /// Issue price per share (発行価額 - Hakkō kagaku)
    pub price_per_share_jpy: u64,

    /// Payment deadline (払込期日 - Haraikomi kijitsu)
    pub payment_deadline: DateTime<Utc>,

    /// Shareholders' preemptive rights (株主の優先引受権 - Kabunushi no yūsen hikiuke-ken)
    pub preemptive_rights_offered: bool,
}

// ============================================================================
// Commercial Code (商法) Essentials
// ============================================================================

/// Commercial transaction (商行為 - Shōkōi) - Article 501
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommercialTransaction {
    /// Transaction type
    pub transaction_type: CommercialTransactionType,

    /// Transaction date
    pub transaction_date: DateTime<Utc>,

    /// Amount (金額 - Kingaku)
    pub amount_jpy: u64,

    /// Parties involved (当事者 - Tōjisha)
    pub parties: Vec<String>,
}

/// Commercial transaction type (商行為の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommercialTransactionType {
    /// Purchase for resale (転売目的の購入 - Tenbai mokuteki no kōnyū)
    PurchaseForResale,

    /// Sale of goods (物品の販売 - Buppin no hanbai)
    SaleOfGoods,

    /// Commercial loan (商業貸付 - Shōgyō kashitsuke)
    CommercialLoan,

    /// Brokerage (仲立 - Nakadachi)
    Brokerage,

    /// Agency (代理 - Dairi)
    Agency,
}

/// Merchant (商人 - Shōnin) - Article 503
/// A person who conducts commercial transactions as a business
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Merchant {
    /// Name or company name (氏名・商号 - Shimei/Shōgō)
    pub name: String,

    /// Business type (営業種類 - Eigyō shurui)
    pub business_type: String,

    /// Registration number (登記番号 - Tōki bangō)
    pub registration_number: Option<String>,
}

/// Statutory interest rate (法定利率 - Hōtei riritsu) - Article 515
/// 6% per annum for commercial transactions (commercial statutory rate)
/// Changed from 5% to 6% as of April 1, 2020
pub const COMMERCIAL_STATUTORY_INTEREST_RATE: f64 = 0.06;

/// Calculate statutory interest for commercial transactions
/// (商事法定利息の計算 - Shōji hōtei risoku no keisan)
pub fn calculate_commercial_statutory_interest(principal_jpy: u64, days: u64) -> u64 {
    let daily_rate = COMMERCIAL_STATUTORY_INTEREST_RATE / 365.0;
    let interest = principal_jpy as f64 * daily_rate * days as f64;
    interest.round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capital_validation() {
        let valid_capital = Capital::new(1_000_000);
        assert!(valid_capital.is_valid());

        let minimum_capital = Capital::new(1);
        assert!(minimum_capital.is_valid());

        let zero_capital = Capital::new(0);
        assert!(!zero_capital.is_valid());
    }

    #[test]
    fn test_capital_classification() {
        let small_company = Capital::new(50_000_000);
        assert!(small_company.is_small_company());
        assert!(!small_company.is_large_company());

        let large_company = Capital::new(600_000_000);
        assert!(!large_company.is_small_company());
        assert!(large_company.is_large_company());
    }

    #[test]
    fn test_commercial_interest_calculation() {
        // ¥1,000,000 for 365 days at 6% = ¥60,000
        let interest = calculate_commercial_statutory_interest(1_000_000, 365);
        assert_eq!(interest, 60_000);

        // ¥1,000,000 for 30 days
        let interest_30_days = calculate_commercial_statutory_interest(1_000_000, 30);
        assert!(interest_30_days > 4_900 && interest_30_days < 5_000);
    }

    #[test]
    fn test_company_type_display() {
        assert_eq!(
            CompanyType::StockCompany.to_string(),
            "株式会社 (Kabushiki-gaisha / Stock Company)"
        );
    }
}
