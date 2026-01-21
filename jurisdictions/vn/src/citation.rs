//! Vietnamese Legal Citation System
//!
//! Vietnam uses a hierarchical citation system based on the type of legal instrument:
//!
//! ## Citation Format
//!
//! - **Luật (Law)**: `Luật [name] số [number]/[year]/QH[session], Điều [article]`
//! - **Nghị định (Decree)**: `Nghị định số [number]/[year]/NĐ-CP`
//! - **Thông tư (Circular)**: `Thông tư số [number]/[year]/TT-[ministry]`
//! - **Quyết định (Decision)**: `Quyết định số [number]/[year]/QĐ-[issuer]`
//!
//! ## Examples
//!
//! - `Luật Lao động số 45/2019/QH14, Điều 1` - Labor Code 2019, Article 1
//! - `Luật Doanh nghiệp số 59/2020/QH14` - Enterprise Law 2020
//! - `Nghị định số 145/2020/NĐ-CP` - Decree on Labor Code implementation

use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of Vietnamese legal instrument
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalInstrumentType {
    /// Hiến pháp (Constitution)
    HienPhap,
    /// Luật (Law) - passed by National Assembly
    Luat,
    /// Pháp lệnh (Ordinance) - passed by Standing Committee
    PhapLenh,
    /// Nghị quyết của Quốc hội (National Assembly Resolution)
    NghiQuyetQh,
    /// Nghị định (Decree) - issued by Government
    NghiDinh,
    /// Thông tư (Circular) - issued by Ministries
    ThongTu(Ministry),
    /// Quyết định (Decision)
    QuyetDinh(Issuer),
    /// Bộ luật (Code)
    BoLuat(CodeType),
}

impl fmt::Display for LegalInstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HienPhap => write!(f, "Hiến pháp"),
            Self::Luat => write!(f, "Luật"),
            Self::PhapLenh => write!(f, "Pháp lệnh"),
            Self::NghiQuyetQh => write!(f, "Nghị quyết"),
            Self::NghiDinh => write!(f, "Nghị định"),
            Self::ThongTu(ministry) => write!(f, "Thông tư ({})", ministry.abbreviation()),
            Self::QuyetDinh(issuer) => write!(f, "Quyết định ({})", issuer.abbreviation()),
            Self::BoLuat(code) => write!(f, "{}", code),
        }
    }
}

/// Vietnamese ministries that issue circulars
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ministry {
    /// Bộ Lao động - Thương binh và Xã hội (Labor)
    LdTbXh,
    /// Bộ Tài chính (Finance)
    Btc,
    /// Bộ Kế hoạch và Đầu tư (Planning and Investment)
    BkhDt,
    /// Bộ Công Thương (Industry and Trade)
    Bct,
    /// Bộ Thông tin và Truyền thông (Information and Communications)
    BttTt,
    /// Bộ Tư pháp (Justice)
    Btp,
    /// Bộ Công an (Public Security)
    Bca,
    /// Bộ Y tế (Health)
    Byt,
    /// Other ministries
    Other(String),
}

impl Ministry {
    /// Get ministry abbreviation for citation
    pub fn abbreviation(&self) -> &str {
        match self {
            Self::LdTbXh => "BLĐTBXH",
            Self::Btc => "BTC",
            Self::BkhDt => "BKHĐT",
            Self::Bct => "BCT",
            Self::BttTt => "BTTTT",
            Self::Btp => "BTP",
            Self::Bca => "BCA",
            Self::Byt => "BYT",
            Self::Other(abbr) => abbr,
        }
    }

    /// Get full ministry name in Vietnamese
    pub fn full_name_vi(&self) -> &str {
        match self {
            Self::LdTbXh => "Bộ Lao động - Thương binh và Xã hội",
            Self::Btc => "Bộ Tài chính",
            Self::BkhDt => "Bộ Kế hoạch và Đầu tư",
            Self::Bct => "Bộ Công Thương",
            Self::BttTt => "Bộ Thông tin và Truyền thông",
            Self::Btp => "Bộ Tư pháp",
            Self::Bca => "Bộ Công an",
            Self::Byt => "Bộ Y tế",
            Self::Other(name) => name,
        }
    }

    /// Get full ministry name in English
    pub fn full_name_en(&self) -> &str {
        match self {
            Self::LdTbXh => "Ministry of Labor, Invalids and Social Affairs",
            Self::Btc => "Ministry of Finance",
            Self::BkhDt => "Ministry of Planning and Investment",
            Self::Bct => "Ministry of Industry and Trade",
            Self::BttTt => "Ministry of Information and Communications",
            Self::Btp => "Ministry of Justice",
            Self::Bca => "Ministry of Public Security",
            Self::Byt => "Ministry of Health",
            Self::Other(name) => name,
        }
    }
}

/// Issuer for decisions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Issuer {
    /// Thủ tướng Chính phủ (Prime Minister)
    ThuTuong,
    /// Chính phủ (Government)
    ChinhPhu,
    /// Bộ (Ministry)
    Bo(Ministry),
    /// UBND tỉnh/thành phố (Provincial People's Committee)
    UbndProvince(String),
    /// Other issuer
    Other(String),
}

impl Issuer {
    /// Get abbreviation for citation
    pub fn abbreviation(&self) -> &str {
        match self {
            Self::ThuTuong => "TTg",
            Self::ChinhPhu => "CP",
            Self::Bo(ministry) => ministry.abbreviation(),
            Self::UbndProvince(province) => province,
            Self::Other(abbr) => abbr,
        }
    }
}

/// Type of codified law (Bộ luật)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CodeType {
    /// Bộ luật Dân sự (Civil Code)
    DanSu,
    /// Bộ luật Hình sự (Criminal Code)
    HinhSu,
    /// Bộ luật Lao động (Labor Code)
    LaoDong,
    /// Bộ luật Tố tụng dân sự (Civil Procedure Code)
    ToTungDanSu,
    /// Bộ luật Tố tụng hình sự (Criminal Procedure Code)
    ToTungHinhSu,
}

impl fmt::Display for CodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DanSu => write!(f, "Bộ luật Dân sự"),
            Self::HinhSu => write!(f, "Bộ luật Hình sự"),
            Self::LaoDong => write!(f, "Bộ luật Lao động"),
            Self::ToTungDanSu => write!(f, "Bộ luật Tố tụng dân sự"),
            Self::ToTungHinhSu => write!(f, "Bộ luật Tố tụng hình sự"),
        }
    }
}

/// Vietnamese legal citation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VietnameseCitation {
    /// Type of legal instrument
    pub instrument_type: LegalInstrumentType,
    /// Document number
    pub number: u32,
    /// Year of issuance
    pub year: u32,
    /// National Assembly session (for laws) - e.g., QH14, QH15
    pub na_session: Option<u32>,
    /// Article number (Điều)
    pub article: Option<u32>,
    /// Clause number within article (khoản)
    pub clause: Option<u32>,
    /// Point within clause (điểm)
    pub point: Option<char>,
    /// Title/name of the law in Vietnamese
    pub title_vi: Option<String>,
    /// Title/name of the law in English
    pub title_en: Option<String>,
}

impl VietnameseCitation {
    /// Create a new citation for a Luật (Law)
    pub fn luat(number: u32, year: u32, na_session: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::Luat,
            number,
            year,
            na_session: Some(na_session),
            article: None,
            clause: None,
            point: None,
            title_vi: None,
            title_en: None,
        }
    }

    /// Create a new citation for the Constitution (Hiến pháp)
    pub fn constitution(year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::HienPhap,
            number: 0,
            year,
            na_session: None,
            article: None,
            clause: None,
            point: None,
            title_vi: Some(format!(
                "Hiến pháp nước Cộng hòa xã hội chủ nghĩa Việt Nam năm {}",
                year
            )),
            title_en: Some(format!(
                "Constitution of the Socialist Republic of Vietnam {}",
                year
            )),
        }
    }

    /// Create a new citation for a Nghị định (Decree)
    pub fn nghi_dinh(number: u32, year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::NghiDinh,
            number,
            year,
            na_session: None,
            article: None,
            clause: None,
            point: None,
            title_vi: None,
            title_en: None,
        }
    }

    /// Create a citation for the Labor Code
    pub fn labor_code(year: u32, na_session: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::BoLuat(CodeType::LaoDong),
            number: 45,
            year,
            na_session: Some(na_session),
            article: None,
            clause: None,
            point: None,
            title_vi: Some("Bộ luật Lao động".to_string()),
            title_en: Some("Labor Code".to_string()),
        }
    }

    /// Create a citation for the Civil Code
    pub fn civil_code(year: u32) -> Self {
        Self {
            instrument_type: LegalInstrumentType::BoLuat(CodeType::DanSu),
            number: 91,
            year,
            na_session: Some(13),
            article: None,
            clause: None,
            point: None,
            title_vi: Some("Bộ luật Dân sự".to_string()),
            title_en: Some("Civil Code".to_string()),
        }
    }

    /// Set article number (Điều)
    pub fn with_article(mut self, article: u32) -> Self {
        self.article = Some(article);
        self
    }

    /// Set clause number (khoản)
    pub fn with_clause(mut self, clause: u32) -> Self {
        self.clause = Some(clause);
        self
    }

    /// Set point (điểm)
    pub fn with_point(mut self, point: char) -> Self {
        self.point = Some(point);
        self
    }

    /// Set Vietnamese title
    pub fn with_title_vi(mut self, title: impl Into<String>) -> Self {
        self.title_vi = Some(title.into());
        self
    }

    /// Set English title
    pub fn with_title_en(mut self, title: impl Into<String>) -> Self {
        self.title_en = Some(title.into());
        self
    }

    /// Get the formal Vietnamese citation string
    pub fn citation_vi(&self) -> String {
        format!("{}", self)
    }

    /// Get the English-style citation string
    pub fn citation_en(&self) -> String {
        let mut citation = match &self.instrument_type {
            LegalInstrumentType::Luat => {
                let session = self
                    .na_session
                    .map_or(String::new(), |s| format!("/QH{}", s));
                format!("Law No. {}/{}{}", self.number, self.year, session)
            }
            LegalInstrumentType::HienPhap => format!("Constitution {}", self.year),
            LegalInstrumentType::NghiDinh => {
                format!("Decree No. {}/{}/ND-CP", self.number, self.year)
            }
            LegalInstrumentType::ThongTu(ministry) => {
                format!(
                    "Circular No. {}/{}/TT-{}",
                    self.number,
                    self.year,
                    ministry.abbreviation()
                )
            }
            LegalInstrumentType::BoLuat(code) => match code {
                CodeType::DanSu => format!("Civil Code {}", self.year),
                CodeType::HinhSu => format!("Criminal Code {}", self.year),
                CodeType::LaoDong => format!("Labor Code {}", self.year),
                CodeType::ToTungDanSu => format!("Civil Procedure Code {}", self.year),
                CodeType::ToTungHinhSu => format!("Criminal Procedure Code {}", self.year),
            },
            _ => format!("Document No. {}/{}", self.number, self.year),
        };

        if let Some(article) = self.article {
            citation.push_str(&format!(", Article {}", article));
            if let Some(clause) = self.clause {
                citation.push_str(&format!(".{}", clause));
            }
            if let Some(point) = self.point {
                citation.push_str(&format!("({})", point));
            }
        }

        citation
    }
}

impl fmt::Display for VietnameseCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.instrument_type {
            LegalInstrumentType::HienPhap => {
                write!(f, "Hiến pháp năm {}", self.year)?;
            }
            LegalInstrumentType::Luat => {
                let session = self
                    .na_session
                    .map_or(String::new(), |s| format!("/QH{}", s));
                if let Some(title) = &self.title_vi {
                    write!(
                        f,
                        "Luật {} số {}/{}{}",
                        title, self.number, self.year, session
                    )?;
                } else {
                    write!(f, "Luật số {}/{}{}", self.number, self.year, session)?;
                }
            }
            LegalInstrumentType::NghiDinh => {
                write!(f, "Nghị định số {}/{}/NĐ-CP", self.number, self.year)?;
            }
            LegalInstrumentType::ThongTu(ministry) => {
                write!(
                    f,
                    "Thông tư số {}/{}/TT-{}",
                    self.number,
                    self.year,
                    ministry.abbreviation()
                )?;
            }
            LegalInstrumentType::BoLuat(code) => {
                let session = self
                    .na_session
                    .map_or(String::new(), |s| format!("/QH{}", s));
                write!(f, "{} số {}/{}{}", code, self.number, self.year, session)?;
            }
            _ => {
                write!(
                    f,
                    "{} số {}/{}",
                    self.instrument_type, self.number, self.year
                )?;
            }
        }

        if let Some(article) = self.article {
            write!(f, ", Điều {}", article)?;
            if let Some(clause) = self.clause {
                write!(f, " khoản {}", clause)?;
            }
            if let Some(point) = self.point {
                write!(f, " điểm {}", point)?;
            }
        }

        Ok(())
    }
}

/// Common Vietnamese legal citations
pub mod common_citations {
    use super::*;

    /// Labor Code 2019 (Bộ luật Lao động 2019)
    pub fn labor_code_2019() -> VietnameseCitation {
        VietnameseCitation::labor_code(2019, 14)
    }

    /// Enterprise Law 2020 (Luật Doanh nghiệp 2020)
    pub fn enterprise_law_2020() -> VietnameseCitation {
        VietnameseCitation::luat(59, 2020, 14)
            .with_title_vi("Doanh nghiệp")
            .with_title_en("Enterprise")
    }

    /// Investment Law 2020 (Luật Đầu tư 2020)
    pub fn investment_law_2020() -> VietnameseCitation {
        VietnameseCitation::luat(61, 2020, 14)
            .with_title_vi("Đầu tư")
            .with_title_en("Investment")
    }

    /// Civil Code 2015 (Bộ luật Dân sự 2015)
    pub fn civil_code_2015() -> VietnameseCitation {
        VietnameseCitation::civil_code(2015)
    }

    /// Constitution 2013 (Hiến pháp 2013)
    pub fn constitution_2013() -> VietnameseCitation {
        VietnameseCitation::constitution(2013)
    }

    /// Land Law 2013 (Luật Đất đai 2013)
    pub fn land_law_2013() -> VietnameseCitation {
        VietnameseCitation::luat(45, 2013, 13)
            .with_title_vi("Đất đai")
            .with_title_en("Land")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_law_citation() {
        let citation = VietnameseCitation::luat(45, 2019, 14)
            .with_title_vi("Lao động")
            .with_article(1)
            .with_clause(1);

        assert_eq!(
            citation.to_string(),
            "Luật Lao động số 45/2019/QH14, Điều 1 khoản 1"
        );
    }

    #[test]
    fn test_constitution_citation() {
        let citation = VietnameseCitation::constitution(2013).with_article(14);

        assert_eq!(citation.to_string(), "Hiến pháp năm 2013, Điều 14");
        assert_eq!(citation.citation_en(), "Constitution 2013, Article 14");
    }

    #[test]
    fn test_decree_citation() {
        let citation = VietnameseCitation::nghi_dinh(145, 2020)
            .with_article(3)
            .with_clause(2);

        assert_eq!(
            citation.to_string(),
            "Nghị định số 145/2020/NĐ-CP, Điều 3 khoản 2"
        );
    }

    #[test]
    fn test_labor_code_citation() {
        let citation = VietnameseCitation::labor_code(2019, 14).with_article(90);

        assert_eq!(
            citation.to_string(),
            "Bộ luật Lao động số 45/2019/QH14, Điều 90"
        );
    }

    #[test]
    fn test_common_citations() {
        let labor = common_citations::labor_code_2019();
        assert_eq!(labor.year, 2019);
        assert_eq!(labor.na_session, Some(14));

        let enterprise = common_citations::enterprise_law_2020();
        assert_eq!(enterprise.number, 59);
        assert_eq!(enterprise.year, 2020);
    }

    #[test]
    fn test_ministry_names() {
        assert_eq!(Ministry::LdTbXh.abbreviation(), "BLĐTBXH");
        assert_eq!(
            Ministry::LdTbXh.full_name_en(),
            "Ministry of Labor, Invalids and Social Affairs"
        );
    }
}
