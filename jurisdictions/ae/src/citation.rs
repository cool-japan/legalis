//! UAE Legal Citation System
//!
//! UAE uses both Arabic (authoritative) and English for legal documents.
//!
//! ## Citation Format
//!
//! - Federal Law: `Federal Law No. [number]/[year]`
//! - Federal Decree-Law: `Federal Decree-Law No. [number]/[year]`
//! - Cabinet Resolution: `Cabinet Resolution No. [number]/[year]`
//! - Ministerial Decision: `Ministerial Decision No. [number]/[year]`
//!
//! Arabic format: `القانون الاتحادي رقم [number] لسنة [year]`

use serde::{Deserialize, Serialize};
use std::fmt;

/// Types of UAE legal instruments
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalInstrumentType {
    /// Federal Law (القانون الاتحادي)
    FederalLaw,
    /// Federal Decree-Law (مرسوم بقانون اتحادي)
    FederalDecreeLaw,
    /// Cabinet Resolution (قرار مجلس الوزراء)
    CabinetResolution,
    /// Ministerial Decision (قرار وزاري)
    MinisterialDecision,
    /// Federal Decree (مرسوم اتحادي)
    FederalDecree,
    /// Emirate Law (قانون محلي)
    EmirateLaw { emirate: Emirate },
    /// DIFC Law (DIFC specific)
    DifcLaw,
    /// ADGM Regulation (ADGM specific)
    AdgmRegulation,
}

/// Emirates (Imarat)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Emirate {
    /// Abu Dhabi (أبوظبي)
    AbuDhabi,
    /// Dubai (دبي)
    Dubai,
    /// Sharjah (الشارقة)
    Sharjah,
    /// Ajman (عجمان)
    Ajman,
    /// Umm Al Quwain (أم القيوين)
    UmmAlQuwain,
    /// Ras Al Khaimah (رأس الخيمة)
    RasAlKhaimah,
    /// Fujairah (الفجيرة)
    Fujairah,
}

impl Emirate {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::AbuDhabi => "أبوظبي",
            Self::Dubai => "دبي",
            Self::Sharjah => "الشارقة",
            Self::Ajman => "عجمان",
            Self::UmmAlQuwain => "أم القيوين",
            Self::RasAlKhaimah => "رأس الخيمة",
            Self::Fujairah => "الفجيرة",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::AbuDhabi => "Abu Dhabi",
            Self::Dubai => "Dubai",
            Self::Sharjah => "Sharjah",
            Self::Ajman => "Ajman",
            Self::UmmAlQuwain => "Umm Al Quwain",
            Self::RasAlKhaimah => "Ras Al Khaimah",
            Self::Fujairah => "Fujairah",
        }
    }
}

impl LegalInstrumentType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::FederalLaw => "القانون الاتحادي",
            Self::FederalDecreeLaw => "مرسوم بقانون اتحادي",
            Self::CabinetResolution => "قرار مجلس الوزراء",
            Self::MinisterialDecision => "قرار وزاري",
            Self::FederalDecree => "مرسوم اتحادي",
            Self::EmirateLaw { .. } => "قانون محلي",
            Self::DifcLaw => "قانون مركز دبي المالي العالمي",
            Self::AdgmRegulation => "لائحة سوق أبوظبي العالمي",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::FederalLaw => "Federal Law",
            Self::FederalDecreeLaw => "Federal Decree-Law",
            Self::CabinetResolution => "Cabinet Resolution",
            Self::MinisterialDecision => "Ministerial Decision",
            Self::FederalDecree => "Federal Decree",
            Self::EmirateLaw { .. } => "Emirate Law",
            Self::DifcLaw => "DIFC Law",
            Self::AdgmRegulation => "ADGM Regulation",
        }
    }

    /// Get English abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::FederalLaw => "FL",
            Self::FederalDecreeLaw => "FDL",
            Self::CabinetResolution => "CR",
            Self::MinisterialDecision => "MD",
            Self::FederalDecree => "FD",
            Self::EmirateLaw { .. } => "EL",
            Self::DifcLaw => "DIFC",
            Self::AdgmRegulation => "ADGM",
        }
    }
}

/// UAE Legal Citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UaeCitation {
    /// Type of legal instrument
    pub instrument_type: LegalInstrumentType,
    /// Number of the law/decree
    pub number: u32,
    /// Year of issuance
    pub year: u32,
    /// Article number (if specific)
    pub article: Option<u32>,
    /// Paragraph/clause number
    pub paragraph: Option<u32>,
    /// Title in Arabic
    pub title_ar: Option<String>,
    /// Title in English
    pub title_en: Option<String>,
}

impl UaeCitation {
    /// Create a new Federal Law citation
    pub fn federal_law(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::FederalLaw,
            number,
            year,
            article: None,
            paragraph: None,
            title_ar: None,
            title_en: None,
        }
    }

    /// Create a new Federal Decree-Law citation
    pub fn federal_decree_law(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::FederalDecreeLaw,
            number,
            year,
            article: None,
            paragraph: None,
            title_ar: None,
            title_en: None,
        }
    }

    /// Create a Cabinet Resolution citation
    pub fn cabinet_resolution(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::CabinetResolution,
            number,
            year,
            article: None,
            paragraph: None,
            title_ar: None,
            title_en: None,
        }
    }

    /// Create a DIFC Law citation
    pub fn difc_law(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::DifcLaw,
            number,
            year,
            article: None,
            paragraph: None,
            title_ar: None,
            title_en: None,
        }
    }

    /// Create an ADGM Regulation citation
    pub fn adgm_regulation(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::AdgmRegulation,
            number,
            year,
            article: None,
            paragraph: None,
            title_ar: None,
            title_en: None,
        }
    }

    /// Add specific article
    pub fn with_article(mut self, article: u32) -> Self {
        self.article = Some(article);
        self
    }

    /// Add paragraph
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Add Arabic title
    pub fn with_title_ar(mut self, title: impl Into<String>) -> Self {
        self.title_ar = Some(title.into());
        self
    }

    /// Add English title
    pub fn with_title_en(mut self, title: impl Into<String>) -> Self {
        self.title_en = Some(title.into());
        self
    }

    /// Format as English citation
    pub fn format_en(&self) -> String {
        let mut s = format!(
            "{} No. {}/{}",
            self.instrument_type.name_en(),
            self.number,
            self.year
        );

        if let Some(ref title) = self.title_en {
            s = format!("{} ({})", s, title);
        }

        if let Some(article) = self.article {
            s.push_str(&format!(", Article {}", article));
        }

        if let Some(paragraph) = self.paragraph {
            s.push_str(&format!("({})", paragraph));
        }

        s
    }

    /// Format as Arabic citation
    pub fn format_ar(&self) -> String {
        let mut s = format!(
            "{} رقم {} لسنة {}",
            self.instrument_type.name_ar(),
            self.number,
            self.year
        );

        if let Some(ref title) = self.title_ar {
            s = format!("{} بشأن {}", s, title);
        }

        if let Some(article) = self.article {
            s.push_str(&format!("، المادة {}", article));
        }

        if let Some(paragraph) = self.paragraph {
            s.push_str(&format!("({})", paragraph));
        }

        s
    }
}

impl fmt::Display for UaeCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_en())
    }
}

/// Common UAE law citations
pub mod common_citations {
    use super::*;

    /// Federal Decree-Law No. 33/2021 on Labour Relations
    pub fn labor_law_2021() -> UaeCitation {
        UaeCitation::federal_decree_law(33, 2021)
            .with_title_ar("تنظيم علاقات العمل")
            .with_title_en("Regulation of Labour Relations")
    }

    /// Federal Law No. 5/1985 on Civil Transactions
    pub fn civil_transactions_law() -> UaeCitation {
        UaeCitation::federal_law(5, 1985)
            .with_title_ar("المعاملات المدنية")
            .with_title_en("Civil Transactions")
    }

    /// Federal Decree-Law No. 32/2021 on Commercial Companies
    pub fn commercial_companies_law() -> UaeCitation {
        UaeCitation::federal_decree_law(32, 2021)
            .with_title_ar("الشركات التجارية")
            .with_title_en("Commercial Companies")
    }

    /// Federal Decree-Law No. 45/2021 on Data Protection
    pub fn data_protection_law() -> UaeCitation {
        UaeCitation::federal_decree_law(45, 2021)
            .with_title_ar("حماية البيانات الشخصية")
            .with_title_en("Protection of Personal Data")
    }

    /// Federal Decree-Law No. 2/2015 on Combating Discrimination
    pub fn anti_discrimination_law() -> UaeCitation {
        UaeCitation::federal_decree_law(2, 2015)
            .with_title_ar("مكافحة التمييز والكراهية")
            .with_title_en("Combating Discrimination and Hatred")
    }

    /// Federal Law No. 2/2014 on SMEs (Small and Medium Enterprises)
    pub fn sme_law() -> UaeCitation {
        UaeCitation::federal_law(2, 2014)
            .with_title_ar("المنشآت الصغيرة والمتوسطة")
            .with_title_en("Small and Medium Enterprises")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federal_law_citation() {
        let citation = UaeCitation::federal_law(5, 1985)
            .with_title_en("Civil Transactions")
            .with_article(18);

        let formatted = citation.format_en();
        assert!(formatted.contains("Federal Law No. 5/1985"));
        assert!(formatted.contains("Article 18"));
    }

    #[test]
    fn test_federal_decree_law_citation() {
        let citation = UaeCitation::federal_decree_law(33, 2021).with_title_ar("تنظيم علاقات العمل");

        let ar = citation.format_ar();
        assert!(ar.contains("مرسوم بقانون اتحادي"));
        assert!(ar.contains("33"));
        assert!(ar.contains("2021"));
    }

    #[test]
    fn test_common_citations() {
        let labor = common_citations::labor_law_2021();
        assert_eq!(labor.number, 33);
        assert_eq!(labor.year, 2021);

        let civil = common_citations::civil_transactions_law();
        assert_eq!(civil.year, 1985);
    }

    #[test]
    fn test_emirates() {
        assert_eq!(Emirate::Dubai.name_ar(), "دبي");
        assert_eq!(Emirate::AbuDhabi.name_en(), "Abu Dhabi");
    }

    #[test]
    fn test_difc_citation() {
        let citation = UaeCitation::difc_law(6, 2021).with_title_en("Companies Law");

        assert!(citation.format_en().contains("DIFC Law No. 6/2021"));
    }

    #[test]
    fn test_instrument_types() {
        assert_eq!(LegalInstrumentType::FederalLaw.abbreviation(), "FL");
        assert_eq!(LegalInstrumentType::FederalDecreeLaw.abbreviation(), "FDL");
        assert_eq!(LegalInstrumentType::DifcLaw.name_en(), "DIFC Law");
    }
}
