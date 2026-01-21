//! Indonesian Legal Citation System
//!
//! Indonesia uses a hierarchical citation system based on the type of legal instrument:
//!
//! ## Citation Format
//!
//! - **Undang-Undang (UU)**: `UU No. [number] Tahun [year], Pasal [article] ayat ([paragraph])`
//! - **Peraturan Pemerintah (PP)**: `PP No. [number] Tahun [year]`
//! - **Peraturan Presiden (Perpres)**: `Perpres No. [number] Tahun [year]`
//! - **Peraturan Menteri (Permen)**: `Permen[ministry] No. [number] Tahun [year]`
//! - **Peraturan Daerah (Perda)**: `Perda [region] No. [number] Tahun [year]`
//!
//! ## Examples
//!
//! - `UU No. 13 Tahun 2003, Pasal 1 ayat (1)` - Labor Law, Article 1 paragraph 1
//! - `UU No. 27 Tahun 2022, Pasal 5 ayat (2)` - Personal Data Protection
//! - `UU No. 25 Tahun 2007, Pasal 3` - Investment Law
//! - `PP No. 71 Tahun 2019` - Government Regulation on Electronic Systems

use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of Indonesian legal instrument
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalInstrumentType {
    /// Undang-Undang (Law/Act)
    UndangUndang,
    /// Undang-Undang Dasar (Constitution)
    UndangUndangDasar,
    /// Peraturan Pemerintah Pengganti Undang-Undang (Government Regulation in Lieu of Law)
    Perppu,
    /// Peraturan Pemerintah (Government Regulation)
    PeraturanPemerintah,
    /// Peraturan Presiden (Presidential Regulation)
    PeraturanPresiden,
    /// Keputusan Presiden (Presidential Decree)
    KeputusanPresiden,
    /// Peraturan Menteri (Ministerial Regulation)
    PeraturanMenteri(Ministry),
    /// Peraturan Daerah (Regional Regulation)
    PeraturanDaerah(String),
    /// Kitab Undang-Undang (Code/Codified Law)
    KitabUndangUndang(CodeType),
}

impl fmt::Display for LegalInstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndangUndang => write!(f, "UU"),
            Self::UndangUndangDasar => write!(f, "UUD"),
            Self::Perppu => write!(f, "Perppu"),
            Self::PeraturanPemerintah => write!(f, "PP"),
            Self::PeraturanPresiden => write!(f, "Perpres"),
            Self::KeputusanPresiden => write!(f, "Keppres"),
            Self::PeraturanMenteri(ministry) => write!(f, "Permen{}", ministry.abbreviation()),
            Self::PeraturanDaerah(region) => write!(f, "Perda {}", region),
            Self::KitabUndangUndang(code) => write!(f, "{}", code),
        }
    }
}

/// Indonesian ministries that issue regulations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ministry {
    /// Kementerian Hukum dan HAM (Law and Human Rights)
    Kumham,
    /// Kementerian Tenaga Kerja (Manpower/Labor)
    Naker,
    /// Kementerian Keuangan (Finance)
    Keuangan,
    /// Kementerian Perdagangan (Trade)
    Perdagangan,
    /// Kementerian Komunikasi dan Informatika (Communications and Information)
    Kominfo,
    /// Kementerian Kesehatan (Health)
    Kesehatan,
    /// Kementerian Perindustrian (Industry)
    Perindustrian,
    /// Kementerian Investasi/BKPM
    Investasi,
    /// Other ministries
    Other(String),
}

impl Ministry {
    /// Get ministry abbreviation for citation
    pub fn abbreviation(&self) -> &str {
        match self {
            Self::Kumham => "kumham",
            Self::Naker => "naker",
            Self::Keuangan => "keu",
            Self::Perdagangan => "dag",
            Self::Kominfo => "kominfo",
            Self::Kesehatan => "kes",
            Self::Perindustrian => "perin",
            Self::Investasi => "BKPM",
            Self::Other(abbr) => abbr,
        }
    }

    /// Get full ministry name in Indonesian
    pub fn full_name_id(&self) -> &str {
        match self {
            Self::Kumham => "Kementerian Hukum dan HAM",
            Self::Naker => "Kementerian Ketenagakerjaan",
            Self::Keuangan => "Kementerian Keuangan",
            Self::Perdagangan => "Kementerian Perdagangan",
            Self::Kominfo => "Kementerian Komunikasi dan Informatika",
            Self::Kesehatan => "Kementerian Kesehatan",
            Self::Perindustrian => "Kementerian Perindustrian",
            Self::Investasi => "Kementerian Investasi/BKPM",
            Self::Other(name) => name,
        }
    }

    /// Get full ministry name in English
    pub fn full_name_en(&self) -> &str {
        match self {
            Self::Kumham => "Ministry of Law and Human Rights",
            Self::Naker => "Ministry of Manpower",
            Self::Keuangan => "Ministry of Finance",
            Self::Perdagangan => "Ministry of Trade",
            Self::Kominfo => "Ministry of Communications and Information Technology",
            Self::Kesehatan => "Ministry of Health",
            Self::Perindustrian => "Ministry of Industry",
            Self::Investasi => "Ministry of Investment/BKPM",
            Self::Other(name) => name,
        }
    }
}

/// Type of codified law (Kitab)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CodeType {
    /// KUHPerdata - Kitab Undang-Undang Hukum Perdata (Civil Code)
    KUHPerdata,
    /// KUHPidana - Kitab Undang-Undang Hukum Pidana (Criminal Code)
    KUHPidana,
    /// KUHAP - Kitab Undang-Undang Hukum Acara Pidana (Criminal Procedure Code)
    KUHAP,
    /// KUHDagang - Kitab Undang-Undang Hukum Dagang (Commercial Code)
    KUHDagang,
    /// KHI - Kompilasi Hukum Islam (Compilation of Islamic Law)
    KHI,
}

impl fmt::Display for CodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KUHPerdata => write!(f, "KUHPerdata"),
            Self::KUHPidana => write!(f, "KUHPidana"),
            Self::KUHAP => write!(f, "KUHAP"),
            Self::KUHDagang => write!(f, "KUHDagang"),
            Self::KHI => write!(f, "KHI"),
        }
    }
}

/// Indonesian legal citation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndonesianCitation {
    /// Type of legal instrument
    pub instrument_type: LegalInstrumentType,
    /// Number of the instrument
    pub number: u32,
    /// Year of enactment
    pub year: u32,
    /// Article number (Pasal)
    pub article: Option<u32>,
    /// Paragraph number within article (ayat)
    pub paragraph: Option<u32>,
    /// Point/clause within paragraph (huruf)
    pub point: Option<char>,
    /// Sub-point (angka)
    pub sub_point: Option<u32>,
    /// Title/name of the law in Indonesian
    pub title_id: Option<String>,
    /// Title/name of the law in English
    pub title_en: Option<String>,
}

impl IndonesianCitation {
    /// Create a new citation for an Undang-Undang (Law)
    pub fn undang_undang(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::UndangUndang,
            number,
            year,
            article: None,
            paragraph: None,
            point: None,
            sub_point: None,
            title_id: None,
            title_en: None,
        }
    }

    /// Create a new citation for the Constitution (UUD 1945)
    pub fn constitution() -> Self {
        Self {
            instrument_type: LegalInstrumentType::UndangUndangDasar,
            number: 1945,
            year: 1945,
            article: None,
            paragraph: None,
            point: None,
            sub_point: None,
            title_id: Some("Undang-Undang Dasar Negara Republik Indonesia Tahun 1945".to_string()),
            title_en: Some("Constitution of the Republic of Indonesia 1945".to_string()),
        }
    }

    /// Create a new citation for a Peraturan Pemerintah (Government Regulation)
    pub fn peraturan_pemerintah(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::PeraturanPemerintah,
            number,
            year,
            article: None,
            paragraph: None,
            point: None,
            sub_point: None,
            title_id: None,
            title_en: None,
        }
    }

    /// Create a citation for KUHPerdata (Civil Code)
    pub fn kuh_perdata() -> Self {
        Self {
            instrument_type: LegalInstrumentType::KitabUndangUndang(CodeType::KUHPerdata),
            number: 23,
            year: 1847,
            article: None,
            paragraph: None,
            point: None,
            sub_point: None,
            title_id: Some("Kitab Undang-Undang Hukum Perdata".to_string()),
            title_en: Some("Indonesian Civil Code".to_string()),
        }
    }

    /// Create a citation for KHI (Compilation of Islamic Law)
    pub fn khi() -> Self {
        Self {
            instrument_type: LegalInstrumentType::KitabUndangUndang(CodeType::KHI),
            number: 1,
            year: 1991,
            article: None,
            paragraph: None,
            point: None,
            sub_point: None,
            title_id: Some("Kompilasi Hukum Islam".to_string()),
            title_en: Some("Compilation of Islamic Law".to_string()),
        }
    }

    /// Set article number (Pasal)
    pub fn with_article(mut self, article: u32) -> Self {
        self.article = Some(article);
        self
    }

    /// Set paragraph number (ayat)
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Set point/clause (huruf)
    pub fn with_point(mut self, point: char) -> Self {
        self.point = Some(point);
        self
    }

    /// Set sub-point (angka)
    pub fn with_sub_point(mut self, sub_point: u32) -> Self {
        self.sub_point = Some(sub_point);
        self
    }

    /// Set Indonesian title
    pub fn with_title_id(mut self, title: impl Into<String>) -> Self {
        self.title_id = Some(title.into());
        self
    }

    /// Set English title
    pub fn with_title_en(mut self, title: impl Into<String>) -> Self {
        self.title_en = Some(title.into());
        self
    }

    /// Get the formal Indonesian citation string
    pub fn citation_id(&self) -> String {
        format!("{}", self)
    }

    /// Get the English-style citation string
    pub fn citation_en(&self) -> String {
        let mut citation = match &self.instrument_type {
            LegalInstrumentType::UndangUndang => {
                format!("Law No. {} of {}", self.number, self.year)
            }
            LegalInstrumentType::UndangUndangDasar => "Constitution 1945".to_string(),
            LegalInstrumentType::Perppu => {
                format!(
                    "Government Regulation in Lieu of Law No. {} of {}",
                    self.number, self.year
                )
            }
            LegalInstrumentType::PeraturanPemerintah => {
                format!("Government Regulation No. {} of {}", self.number, self.year)
            }
            LegalInstrumentType::PeraturanPresiden => {
                format!(
                    "Presidential Regulation No. {} of {}",
                    self.number, self.year
                )
            }
            LegalInstrumentType::KeputusanPresiden => {
                format!("Presidential Decree No. {} of {}", self.number, self.year)
            }
            LegalInstrumentType::PeraturanMenteri(ministry) => {
                format!(
                    "Ministerial Regulation ({}) No. {} of {}",
                    ministry.full_name_en(),
                    self.number,
                    self.year
                )
            }
            LegalInstrumentType::PeraturanDaerah(region) => {
                format!(
                    "Regional Regulation {} No. {} of {}",
                    region, self.number, self.year
                )
            }
            LegalInstrumentType::KitabUndangUndang(code) => match code {
                CodeType::KUHPerdata => "Indonesian Civil Code (KUHPerdata)".to_string(),
                CodeType::KUHPidana => "Indonesian Criminal Code (KUHPidana)".to_string(),
                CodeType::KUHAP => "Indonesian Criminal Procedure Code (KUHAP)".to_string(),
                CodeType::KUHDagang => "Indonesian Commercial Code (KUHDagang)".to_string(),
                CodeType::KHI => "Compilation of Islamic Law (KHI)".to_string(),
            },
        };

        if let Some(article) = self.article {
            citation.push_str(&format!(", Article {}", article));
            if let Some(paragraph) = self.paragraph {
                citation.push_str(&format!("({})", paragraph));
            }
            if let Some(point) = self.point {
                citation.push_str(&format!(" letter {}", point));
            }
            if let Some(sub_point) = self.sub_point {
                citation.push_str(&format!(" number {}", sub_point));
            }
        }

        citation
    }
}

impl fmt::Display for IndonesianCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.instrument_type {
            LegalInstrumentType::UndangUndangDasar => {
                write!(f, "UUD NRI Tahun 1945")?;
            }
            LegalInstrumentType::KitabUndangUndang(code) => {
                write!(f, "{}", code)?;
            }
            _ => {
                write!(
                    f,
                    "{} No. {} Tahun {}",
                    self.instrument_type, self.number, self.year
                )?;
            }
        }

        if let Some(article) = self.article {
            write!(f, ", Pasal {}", article)?;
            if let Some(paragraph) = self.paragraph {
                write!(f, " ayat ({})", paragraph)?;
            }
            if let Some(point) = self.point {
                write!(f, " huruf {}", point)?;
            }
            if let Some(sub_point) = self.sub_point {
                write!(f, " angka {}", sub_point)?;
            }
        }

        Ok(())
    }
}

/// Common Indonesian legal citations
pub mod common_citations {
    use super::*;

    /// UU No. 13 Tahun 2003 - Ketenagakerjaan (Labor Law)
    pub fn labor_law() -> IndonesianCitation {
        IndonesianCitation::undang_undang(13, 2003)
            .with_title_id("Ketenagakerjaan")
            .with_title_en("Manpower/Labor")
    }

    /// UU No. 27 Tahun 2022 - Perlindungan Data Pribadi (Personal Data Protection)
    pub fn pdp_law() -> IndonesianCitation {
        IndonesianCitation::undang_undang(27, 2022)
            .with_title_id("Perlindungan Data Pribadi")
            .with_title_en("Personal Data Protection")
    }

    /// UU No. 25 Tahun 2007 - Penanaman Modal (Investment)
    pub fn investment_law() -> IndonesianCitation {
        IndonesianCitation::undang_undang(25, 2007)
            .with_title_id("Penanaman Modal")
            .with_title_en("Investment")
    }

    /// UU No. 11 Tahun 2020 - Cipta Kerja (Omnibus Law/Job Creation)
    pub fn omnibus_law() -> IndonesianCitation {
        IndonesianCitation::undang_undang(11, 2020)
            .with_title_id("Cipta Kerja")
            .with_title_en("Job Creation (Omnibus Law)")
    }

    /// UU No. 6 Tahun 2023 - Penetapan Perppu Cipta Kerja
    pub fn omnibus_law_2023() -> IndonesianCitation {
        IndonesianCitation::undang_undang(6, 2023)
            .with_title_id("Penetapan Peraturan Pemerintah Pengganti Undang-Undang Nomor 2 Tahun 2022 tentang Cipta Kerja")
            .with_title_en("Stipulation of Omnibus Law as Law")
    }

    /// UU No. 40 Tahun 2007 - Perseroan Terbatas (Limited Liability Company)
    pub fn company_law() -> IndonesianCitation {
        IndonesianCitation::undang_undang(40, 2007)
            .with_title_id("Perseroan Terbatas")
            .with_title_en("Limited Liability Company")
    }

    /// UU No. 21 Tahun 2008 - Perbankan Syariah (Islamic Banking)
    pub fn islamic_banking_law() -> IndonesianCitation {
        IndonesianCitation::undang_undang(21, 2008)
            .with_title_id("Perbankan Syariah")
            .with_title_en("Islamic Banking")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undang_undang_citation() {
        let citation = IndonesianCitation::undang_undang(13, 2003)
            .with_article(1)
            .with_paragraph(1);

        assert_eq!(
            citation.to_string(),
            "UU No. 13 Tahun 2003, Pasal 1 ayat (1)"
        );
        assert_eq!(citation.citation_en(), "Law No. 13 of 2003, Article 1(1)");
    }

    #[test]
    fn test_constitution_citation() {
        let citation = IndonesianCitation::constitution().with_article(28);

        assert_eq!(citation.to_string(), "UUD NRI Tahun 1945, Pasal 28");
        assert_eq!(citation.citation_en(), "Constitution 1945, Article 28");
    }

    #[test]
    fn test_government_regulation_citation() {
        let citation = IndonesianCitation::peraturan_pemerintah(71, 2019)
            .with_article(3)
            .with_paragraph(2);

        assert_eq!(
            citation.to_string(),
            "PP No. 71 Tahun 2019, Pasal 3 ayat (2)"
        );
    }

    #[test]
    fn test_civil_code_citation() {
        let citation = IndonesianCitation::kuh_perdata().with_article(1320);

        assert_eq!(citation.to_string(), "KUHPerdata, Pasal 1320");
        assert_eq!(
            citation.citation_en(),
            "Indonesian Civil Code (KUHPerdata), Article 1320"
        );
    }

    #[test]
    fn test_khi_citation() {
        let citation = IndonesianCitation::khi().with_article(85);

        assert_eq!(citation.to_string(), "KHI, Pasal 85");
        assert_eq!(
            citation.citation_en(),
            "Compilation of Islamic Law (KHI), Article 85"
        );
    }

    #[test]
    fn test_full_citation_with_points() {
        let citation = IndonesianCitation::undang_undang(27, 2022)
            .with_article(5)
            .with_paragraph(1)
            .with_point('a')
            .with_sub_point(1);

        assert_eq!(
            citation.to_string(),
            "UU No. 27 Tahun 2022, Pasal 5 ayat (1) huruf a angka 1"
        );
    }

    #[test]
    fn test_common_citations() {
        let labor = common_citations::labor_law();
        assert_eq!(labor.number, 13);
        assert_eq!(labor.year, 2003);

        let pdp = common_citations::pdp_law();
        assert_eq!(pdp.number, 27);
        assert_eq!(pdp.year, 2022);

        let investment = common_citations::investment_law();
        assert_eq!(investment.number, 25);
        assert_eq!(investment.year, 2007);
    }

    #[test]
    fn test_ministry_names() {
        assert_eq!(
            Ministry::Naker.full_name_id(),
            "Kementerian Ketenagakerjaan"
        );
        assert_eq!(Ministry::Naker.full_name_en(), "Ministry of Manpower");
        assert_eq!(Ministry::Kominfo.abbreviation(), "kominfo");
    }
}
