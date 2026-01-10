//! Intellectual Property Types (知的財産権型定義)
//!
//! Type definitions for Japanese intellectual property law, including:
//! - Patent Act (特許法 - Tokkyo-hō)
//! - Copyright Act (著作権法 - Chosakuken-hō)
//! - Trademark Act (商標法 - Shōhyō-hō)
//! - Design Act (意匠法 - Ishō-hō)
//!
//! # Legal References
//! - Patent Act (Act No. 121 of 1959) - 特許法
//! - Copyright Act (Act No. 48 of 1970) - 著作権法
//! - Trademark Act (Act No. 127 of 1959) - 商標法
//! - Design Act (Act No. 125 of 1959) - 意匠法

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ============================================================================
// Constants - Intellectual Property Law (知的財産法)
// ============================================================================

/// Patent protection period (Article 67)
/// 特許権存続期間 - 出願日から20年
pub const PATENT_PROTECTION_YEARS: u32 = 20;

/// Copyright protection period (Article 51)
/// 著作権保護期間 - 著作者の死後70年
pub const COPYRIGHT_PROTECTION_YEARS: u32 = 70;

/// Trademark renewal period (Article 19)
/// 商標権更新期間 - 10年
pub const TRADEMARK_RENEWAL_YEARS: u32 = 10;

/// Design protection period (Article 21)
/// 意匠権存続期間 - 登録日から25年
pub const DESIGN_PROTECTION_YEARS: u32 = 25;

// ============================================================================
// Patent Act Types (特許法型)
// ============================================================================

/// Invention category (発明の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InventionCategory {
    /// Product invention (物の発明 - Mono no hatsumei)
    Product,

    /// Method invention (方法の発明 - Hōhō no hatsumei)
    Method,

    /// Production method invention (物を生産する方法の発明)
    ProductionMethod,
}

/// Patentability requirements (特許要件)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatentRequirement {
    /// Industrial applicability (産業上の利用可能性 - Article 29-1 preamble)
    IndustrialApplicability,

    /// Novelty (新規性 - Article 29-1)
    Novelty,

    /// Inventive step (進歩性 - Article 29-2)
    InventiveStep,

    /// First to file (先願 - Article 39)
    FirstToFile,
}

/// Patent application (特許出願)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PatentApplication {
    /// Application number (出願番号 - Shutsugan bangō)
    pub application_number: String,

    /// Filing date (出願日 - Shutsugan-bi)
    pub filing_date: DateTime<Utc>,

    /// Invention title (発明の名称 - Hatsumei no meishō)
    pub title: String,

    /// Inventor name(s) (発明者名 - Hatsumei-sha mei)
    pub inventors: Vec<String>,

    /// Applicant name(s) (出願人名 - Shutsugan-nin mei)
    pub applicants: Vec<String>,

    /// Invention category (発明の種類)
    pub category: InventionCategory,

    /// Claims (請求項 - Seikyū-kō)
    pub claims: Vec<String>,

    /// Abstract (要約 - Yōyaku)
    pub abstract_text: String,

    /// Priority claim date (優先日 - Yūsen-bi)
    pub priority_date: Option<DateTime<Utc>>,

    /// Examination requested (審査請求済み - Shinsa seikyū-zumi)
    pub examination_requested: bool,
}

impl PatentApplication {
    /// Check if patent protection has expired (特許権存続期間満了判定)
    pub fn is_protection_expired(&self, _grant_date: DateTime<Utc>) -> bool {
        let expiry_date =
            self.filing_date + chrono::Duration::days((PATENT_PROTECTION_YEARS * 365) as i64);
        Utc::now() > expiry_date
    }

    /// Calculate years since filing (出願経過年数計算)
    pub fn years_since_filing(&self) -> f64 {
        (Utc::now() - self.filing_date).num_days() as f64 / 365.0
    }
}

/// Patent grant (特許査定)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PatentGrant {
    /// Patent number (特許番号 - Tokkyo bangō)
    pub patent_number: String,

    /// Grant date (設定登録日 - Settei tōroku-bi)
    pub grant_date: DateTime<Utc>,

    /// Application (出願情報 - Shutsugan jōhō)
    pub application: PatentApplication,

    /// Annual fees paid up to year (年金納付年度)
    pub annual_fees_paid_until_year: u32,
}

impl PatentGrant {
    /// Check if annual fees are current (年金納付状態確認)
    pub fn are_annual_fees_current(&self) -> bool {
        let years_since_grant = (Utc::now() - self.grant_date).num_days() as f64 / 365.0;
        self.annual_fees_paid_until_year as f64 >= years_since_grant
    }

    /// Check if patent is still valid (特許有効性確認)
    pub fn is_valid(&self) -> bool {
        !self.application.is_protection_expired(self.grant_date) && self.are_annual_fees_current()
    }
}

/// Patent infringement type (特許権侵害形態)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PatentInfringementType {
    /// Direct infringement (直接侵害 - Chokusetsu shingai)
    Direct,

    /// Indirect infringement (間接侵害 - Kansetsu shingai - Article 101)
    Indirect,

    /// Contributory infringement (寄与侵害 - Kiyo shingai)
    Contributory,
}

// ============================================================================
// Copyright Act Types (著作権法型)
// ============================================================================

/// Work category (著作物の種類 - Article 10)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WorkCategory {
    /// Literary work (言語の著作物 - Gengo no chosakubutsu)
    Literary,

    /// Musical work (音楽の著作物 - Ongaku no chosakubutsu)
    Musical,

    /// Choreographic work (舞踊の著作物 - Buyō no chosakubutsu)
    Choreographic,

    /// Artistic work (美術の著作物 - Bijutsu no chosakubutsu)
    Artistic,

    /// Architectural work (建築の著作物 - Kenchiku no chosakubutsu)
    Architectural,

    /// Photographic work (写真の著作物 - Shashin no chosakubutsu)
    Photographic,

    /// Cinematographic work (映画の著作物 - Eiga no chosakubutsu)
    Cinematographic,

    /// Program work (プログラムの著作物 - Puroguramu no chosakubutsu)
    Program,

    /// Database work (データベースの著作物)
    Database,
}

/// Economic rights (財産権 - Articles 21-28)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EconomicRight {
    /// Reproduction right (複製権 - Fukusei-ken - Article 21)
    Reproduction,

    /// Performance right (上演・演奏権 - Jōen/ensō-ken - Article 22)
    Performance,

    /// Public transmission right (公衆送信権 - Kōshū sōshin-ken - Article 23)
    PublicTransmission,

    /// Exhibition right (展示権 - Tenji-ken - Article 25)
    Exhibition,

    /// Distribution right (頒布権 - Hanpu-ken - Article 26)
    Distribution,

    /// Transfer right (譲渡権 - Jōto-ken - Article 26-2)
    Transfer,

    /// Rental right (貸与権 - Taiyo-ken - Article 26-3)
    Rental,

    /// Translation/adaptation right (翻訳・翻案権 - Article 27)
    TranslationAdaptation,

    /// Original author's right (二次的著作物利用権 - Article 28)
    OriginalAuthor,
}

/// Moral rights (著作者人格権 - Articles 18-20)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MoralRight {
    /// Right to make public (公表権 - Kōhyō-ken - Article 18)
    MakePublic,

    /// Right to name (氏名表示権 - Shimei hyōji-ken - Article 19)
    Name,

    /// Right to integrity (同一性保持権 - Dōitsu-sei hoji-ken - Article 20)
    Integrity,
}

/// Copyrighted work (著作物)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyrightedWork {
    /// Work title (著作物の題号 - Chosakubutsu no daigō)
    pub title: String,

    /// Author name(s) (著作者名 - Chosakusha mei)
    pub authors: Vec<String>,

    /// Work category (著作物の種類)
    pub category: WorkCategory,

    /// Creation date (創作日 - Sōsaku-bi)
    pub creation_date: DateTime<Utc>,

    /// First publication date (最初の公表日 - Saisho no kōhyō-bi)
    pub first_publication_date: Option<DateTime<Utc>>,

    /// Copyright holder (著作権者 - Chosakuken-sha)
    pub copyright_holder: String,

    /// Is work for hire (職務著作 - Shokumu chosakubutsu - Article 15)
    pub is_work_for_hire: bool,

    /// Derivative work source (二次的著作物の原著作物)
    pub derivative_source: Option<String>,
}

impl CopyrightedWork {
    /// Check if copyright has expired (著作権保護期間満了判定)
    pub fn is_copyright_expired(&self, author_death_date: Option<DateTime<Utc>>) -> bool {
        if let Some(death_date) = author_death_date {
            let expiry_date =
                death_date + chrono::Duration::days((COPYRIGHT_PROTECTION_YEARS * 365) as i64);
            Utc::now() > expiry_date
        } else {
            // For works without known death date, cannot determine expiration
            false
        }
    }
}

/// Fair use type (権利制限 - Articles 30-47-8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FairUseType {
    /// Private use (私的使用 - Shiteki shiyō - Article 30)
    PrivateUse,

    /// Quotation (引用 - In'yō - Article 32)
    Quotation,

    /// Educational use (教育目的 - Kyōiku mokuteki - Article 35)
    EducationalUse,

    /// News reporting (報道目的 - Hōdō mokuteki - Article 41)
    NewsReporting,

    /// Parody/criticism (批評・研究 - Hihyō/kenkyū)
    Criticism,
}

/// Copyright infringement claim (著作権侵害主張)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyrightInfringement {
    /// Original work (原著作物)
    pub original_work: CopyrightedWork,

    /// Alleged infringing work description (侵害疑義著作物)
    pub alleged_infringing_work: String,

    /// Rights infringed (侵害された権利)
    pub rights_infringed: Vec<EconomicRight>,

    /// Date of infringement (侵害日)
    pub infringement_date: DateTime<Utc>,

    /// Claimed fair use defense (権利制限の主張)
    pub fair_use_claim: Option<FairUseType>,

    /// Estimated damages (推定損害額)
    pub estimated_damages_jpy: Option<u64>,
}

// ============================================================================
// Trademark Act Types (商標法型)
// ============================================================================

/// Trademark type (商標の種類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TrademarkType {
    /// Word mark (文字商標 - Moji shōhyō)
    Word,

    /// Logo mark (図形商標 - Zukei shōhyō)
    Logo,

    /// Combined mark (結合商標 - Ketsugō shōhyō)
    Combined,

    /// Three-dimensional mark (立体商標 - Rittai shōhyō)
    ThreeDimensional,

    /// Color mark (色彩商標 - Shikisai shōhyō)
    Color,

    /// Sound mark (音商標 - Oto shōhyō)
    Sound,

    /// Motion mark (動き商標 - Ugoki shōhyō)
    Motion,

    /// Hologram mark (ホログラム商標)
    Hologram,

    /// Position mark (位置商標 - Ichi shōhyō)
    Position,
}

/// Nice Classification (ニース分類)
/// International trademark classification system (1-45)
pub type NiceClass = u8;

/// Trademark application (商標出願)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrademarkApplication {
    /// Application number (出願番号)
    pub application_number: String,

    /// Filing date (出願日)
    pub filing_date: DateTime<Utc>,

    /// Trademark representation (商標の表示)
    pub trademark_representation: String,

    /// Trademark type (商標の種類)
    pub trademark_type: TrademarkType,

    /// Applicant name (出願人名)
    pub applicant: String,

    /// Designated goods/services classes (指定商品・役務の区分)
    pub designated_classes: Vec<NiceClass>,

    /// Designated goods/services (指定商品・役務)
    pub designated_goods_services: Vec<String>,
}

/// Trademark registration (商標登録)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TrademarkRegistration {
    /// Registration number (登録番号)
    pub registration_number: String,

    /// Registration date (登録日)
    pub registration_date: DateTime<Utc>,

    /// Application (出願情報)
    pub application: TrademarkApplication,

    /// Renewal count (更新回数)
    pub renewal_count: u32,

    /// Last renewal date (最終更新日)
    pub last_renewal_date: Option<DateTime<Utc>>,
}

impl TrademarkRegistration {
    /// Check if trademark registration is still valid (商標権有効性確認)
    pub fn is_valid(&self) -> bool {
        let last_renewal = self.last_renewal_date.unwrap_or(self.registration_date);
        let expiry_date =
            last_renewal + chrono::Duration::days((TRADEMARK_RENEWAL_YEARS * 365) as i64);
        Utc::now() <= expiry_date
    }

    /// Calculate years until renewal required (更新までの年数)
    pub fn years_until_renewal(&self) -> f64 {
        let last_renewal = self.last_renewal_date.unwrap_or(self.registration_date);
        let expiry_date =
            last_renewal + chrono::Duration::days((TRADEMARK_RENEWAL_YEARS * 365) as i64);
        (expiry_date - Utc::now()).num_days() as f64 / 365.0
    }
}

/// Trademark similarity assessment (商標類似性評価)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SimilarityLevel {
    /// Identical (同一 - Dōitsu)
    Identical,

    /// Highly similar (高度に類似 - Kōdo ni ruiji)
    HighlySimilar,

    /// Similar (類似 - Ruiji)
    Similar,

    /// Somewhat similar (やや類似 - Yaya ruiji)
    SomewhatSimilar,

    /// Not similar (非類似 - Hi-ruiji)
    NotSimilar,
}

// ============================================================================
// Design Act Types (意匠法型)
// ============================================================================

/// Design category (意匠の分類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DesignCategory {
    /// Product design (物品の意匠 - Buppin no ishō)
    Product,

    /// Partial design (部分意匠 - Bubun ishō)
    Partial,

    /// Related design (関連意匠 - Kanren ishō)
    Related,

    /// Secret design (秘密意匠 - Himitsu ishō)
    Secret,
}

/// Design application (意匠出願)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DesignApplication {
    /// Application number (出願番号)
    pub application_number: String,

    /// Filing date (出願日)
    pub filing_date: DateTime<Utc>,

    /// Design title (意匠に係る物品)
    pub article_title: String,

    /// Designer name(s) (創作者名)
    pub designers: Vec<String>,

    /// Applicant name (出願人名)
    pub applicant: String,

    /// Design category (意匠の分類)
    pub category: DesignCategory,

    /// Description (意匠の説明)
    pub description: String,

    /// Related design application number (関連意匠出願番号)
    pub related_design_application: Option<String>,
}

/// Design registration (意匠登録)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DesignRegistration {
    /// Registration number (登録番号)
    pub registration_number: String,

    /// Registration date (登録日)
    pub registration_date: DateTime<Utc>,

    /// Application (出願情報)
    pub application: DesignApplication,
}

impl DesignRegistration {
    /// Check if design protection has expired (意匠権存続期間満了判定)
    pub fn is_protection_expired(&self) -> bool {
        let expiry_date =
            self.registration_date + chrono::Duration::days((DESIGN_PROTECTION_YEARS * 365) as i64);
        Utc::now() > expiry_date
    }

    /// Calculate years of remaining protection (残存保護期間)
    pub fn years_of_protection_remaining(&self) -> f64 {
        let expiry_date =
            self.registration_date + chrono::Duration::days((DESIGN_PROTECTION_YEARS * 365) as i64);
        let remaining_days = (expiry_date - Utc::now()).num_days();
        if remaining_days > 0 {
            remaining_days as f64 / 365.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_application_years_since_filing() {
        let app = PatentApplication {
            application_number: "2020-123456".to_string(),
            filing_date: Utc::now() - chrono::Duration::days(730), // 2 years ago
            title: "Test Invention".to_string(),
            inventors: vec!["Inventor".to_string()],
            applicants: vec!["Applicant".to_string()],
            category: InventionCategory::Product,
            claims: vec!["Claim 1".to_string()],
            abstract_text: "Abstract".to_string(),
            priority_date: None,
            examination_requested: false,
        };

        assert!(app.years_since_filing() >= 1.9 && app.years_since_filing() <= 2.1);
    }

    #[test]
    fn test_trademark_registration_validity() {
        let registration = TrademarkRegistration {
            registration_number: "6000000".to_string(),
            registration_date: Utc::now() - chrono::Duration::days(365 * 5),
            application: TrademarkApplication {
                application_number: "2015-123456".to_string(),
                filing_date: Utc::now() - chrono::Duration::days(365 * 6),
                trademark_representation: "TEST™".to_string(),
                trademark_type: TrademarkType::Word,
                applicant: "Test Corp".to_string(),
                designated_classes: vec![9, 42],
                designated_goods_services: vec!["Software".to_string()],
            },
            renewal_count: 0,
            last_renewal_date: None,
        };

        assert!(registration.is_valid());
        assert!(registration.years_until_renewal() > 4.0);
    }

    #[test]
    fn test_design_registration_protection() {
        let registration = DesignRegistration {
            registration_number: "1500000".to_string(),
            registration_date: Utc::now() - chrono::Duration::days(365 * 3),
            application: DesignApplication {
                application_number: "2020-001234".to_string(),
                filing_date: Utc::now() - chrono::Duration::days(365 * 4),
                article_title: "Chair".to_string(),
                designers: vec!["Designer".to_string()],
                applicant: "Furniture Co.".to_string(),
                category: DesignCategory::Product,
                description: "Ergonomic chair design".to_string(),
                related_design_application: None,
            },
        };

        assert!(!registration.is_protection_expired());
        assert!(registration.years_of_protection_remaining() > 20.0);
    }

    #[test]
    fn test_patent_constants() {
        assert_eq!(PATENT_PROTECTION_YEARS, 20);
        assert_eq!(COPYRIGHT_PROTECTION_YEARS, 70);
        assert_eq!(TRADEMARK_RENEWAL_YEARS, 10);
        assert_eq!(DESIGN_PROTECTION_YEARS, 25);
    }
}
