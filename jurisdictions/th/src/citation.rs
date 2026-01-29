//! Thai Legal Citation System (การอ้างอิงกฎหมายไทย)
//!
//! This module provides types and utilities for representing Thai legal citations.
//! Thai legal citations use Buddhist Era (พ.ศ.) years and follow specific formats
//! for different types of legal sources.
//!
//! ## Citation Formats
//!
//! ### Acts of Parliament (พระราชบัญญัติ - พ.ร.บ.)
//!
//! Thai acts use Buddhist Era years in their citations:
//! - Full Thai: "พระราชบัญญัติคุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562 มาตรา 26"
//! - Short Thai: "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562 มาตรา 26"
//! - English: "Personal Data Protection Act B.E. 2562 (2019), Section 26"
//!
//! ### Constitution (รัฐธรรมนูญ)
//!
//! - "รัฐธรรมนูญแห่งราชอาณาจักรไทย พ.ศ. 2560 มาตรา 26"
//! - "Constitution of the Kingdom of Thailand B.E. 2560 (2017), Section 26"
//!
//! ### Royal Decrees (พระราชกำหนด - พ.ร.ก.)
//!
//! - "พระราชกำหนดการบริหารราชการในสถานการณ์ฉุกเฉิน พ.ศ. 2548"
//! - "Emergency Decree on Public Administration in Emergency Situation B.E. 2548 (2005)"
//!
//! ### Ministerial Regulations (กฎกระทรวง)
//!
//! - "กฎกระทรวง ฉบับที่ 1 พ.ศ. 2563"
//! - "Ministerial Regulation No. 1 B.E. 2563 (2020)"
//!
//! ### Court Decisions
//!
//! Thai court decisions are cited by decision number and year:
//! - Supreme Court: "คำพิพากษาศาลฎีกาที่ 1234/2567"
//! - Court of Appeal: "คำพิพากษาศาลอุทธรณ์ที่ 5678/2567"
//!
//! ## Examples
//!
//! ```
//! use legalis_th::citation::*;
//! use legalis_th::calendar::BuddhistYear;
//!
//! // PDPA citation
//! let pdpa = ThaiAct::new(
//!     "คุ้มครองข้อมูลส่วนบุคคล",
//!     "Personal Data Protection Act",
//!     BuddhistYear::from_be(2562),
//! );
//! assert_eq!(pdpa.format_th(), "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562");
//!
//! // Section citation
//! let section = pdpa.section(26);
//! assert_eq!(section.format_th(), "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562 มาตรา 26");
//! ```

use crate::calendar::BuddhistYear;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a Thai Act of Parliament (พระราชบัญญัติ - พ.ร.บ.)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThaiAct {
    /// Thai name of the act (e.g., "คุ้มครองข้อมูลส่วนบุคคล")
    pub name_th: String,

    /// English name of the act (e.g., "Personal Data Protection Act")
    pub name_en: String,

    /// Buddhist Era year of enactment
    pub year: BuddhistYear,

    /// Short Thai abbreviation (optional, e.g., "พ.ร.บ. คุ้มครองข้อมูล")
    pub short_name_th: Option<String>,

    /// Short English abbreviation (optional, e.g., "PDPA")
    pub short_name_en: Option<String>,
}

impl ThaiAct {
    /// Creates a new ThaiAct
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::citation::ThaiAct;
    /// use legalis_th::calendar::BuddhistYear;
    ///
    /// let pdpa = ThaiAct::new(
    ///     "คุ้มครองข้อมูลส่วนบุคคล",
    ///     "Personal Data Protection Act",
    ///     BuddhistYear::from_be(2562),
    /// );
    /// ```
    pub fn new(name_th: impl Into<String>, name_en: impl Into<String>, year: BuddhistYear) -> Self {
        Self {
            name_th: name_th.into(),
            name_en: name_en.into(),
            year,
            short_name_th: None,
            short_name_en: None,
        }
    }

    /// Creates a ThaiAct from Common Era year
    pub fn from_ce(name_th: impl Into<String>, name_en: impl Into<String>, ce_year: i32) -> Self {
        Self::new(name_th, name_en, BuddhistYear::from_ce(ce_year))
    }

    /// Sets Thai short name
    pub fn with_short_name_th(mut self, short_name: impl Into<String>) -> Self {
        self.short_name_th = Some(short_name.into());
        self
    }

    /// Sets English short name
    pub fn with_short_name_en(mut self, short_name: impl Into<String>) -> Self {
        self.short_name_en = Some(short_name.into());
        self
    }

    /// Creates a reference to a specific section
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_th::citation::ThaiAct;
    /// use legalis_th::calendar::BuddhistYear;
    ///
    /// let pdpa = ThaiAct::new(
    ///     "คุ้มครองข้อมูลส่วนบุคคล",
    ///     "Personal Data Protection Act",
    ///     BuddhistYear::from_be(2562),
    /// );
    /// let section = pdpa.section(26);
    /// ```
    pub fn section(&self, section: u32) -> ThaiActSection {
        ThaiActSection {
            act: self.clone(),
            section,
            paragraph: None,
            subparagraph: None,
        }
    }

    /// Formats the act in Thai format: "พ.ร.บ. \[name\] พ.ศ. \[year\]"
    pub fn format_th(&self) -> String {
        format!("พ.ร.บ. {} พ.ศ. {}", self.name_th, self.year.be_year)
    }

    /// Formats the act in English format: "\[name\] B.E. \[year\] (\[CE year\])"
    pub fn format_en(&self) -> String {
        format!(
            "{} B.E. {} ({})",
            self.name_en,
            self.year.be_year,
            self.year.ce_year()
        )
    }

    /// Formats using short name if available
    pub fn format_short_th(&self) -> String {
        if let Some(short) = &self.short_name_th {
            format!("{} พ.ศ. {}", short, self.year.be_year)
        } else {
            self.format_th()
        }
    }

    /// Formats using English short name if available
    pub fn format_short_en(&self) -> String {
        if let Some(short) = &self.short_name_en {
            format!("{} B.E. {}", short, self.year.be_year)
        } else {
            self.format_en()
        }
    }
}

impl fmt::Display for ThaiAct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_th())
    }
}

/// Represents a specific section within a Thai Act
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThaiActSection {
    /// The act this section belongs to
    pub act: ThaiAct,

    /// Section number (มาตรา)
    pub section: u32,

    /// Paragraph number (วรรค) - optional
    pub paragraph: Option<u32>,

    /// Subparagraph letter (e.g., "(1)", "(2)") - optional
    pub subparagraph: Option<String>,
}

impl ThaiActSection {
    /// Creates a new section reference
    pub fn new(act: ThaiAct, section: u32) -> Self {
        Self {
            act,
            section,
            paragraph: None,
            subparagraph: None,
        }
    }

    /// Sets the paragraph number
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Sets the subparagraph
    pub fn with_subparagraph(mut self, subparagraph: impl Into<String>) -> Self {
        self.subparagraph = Some(subparagraph.into());
        self
    }

    /// Formats the section in Thai format
    ///
    /// # Examples
    ///
    /// - "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562 มาตรา 26"
    /// - "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562 มาตรา 26 วรรค 2"
    pub fn format_th(&self) -> String {
        let mut result = format!("{} มาตรา {}", self.act.format_th(), self.section);

        if let Some(para) = self.paragraph {
            result.push_str(&format!(" วรรค {}", para));
        }

        if let Some(subpara) = &self.subparagraph {
            result.push_str(&format!(" ({})", subpara));
        }

        result
    }

    /// Formats the section in English format
    pub fn format_en(&self) -> String {
        let mut result = format!("{}, Section {}", self.act.format_en(), self.section);

        if let Some(para) = self.paragraph {
            result.push_str(&format!(", Paragraph {}", para));
        }

        if let Some(subpara) = &self.subparagraph {
            result.push_str(&format!(" ({})", subpara));
        }

        result
    }

    /// Returns a short citation format in Thai
    pub fn short_citation_th(&self) -> String {
        format!("มาตรา {}", self.section)
    }

    /// Returns a short citation format in English
    pub fn short_citation_en(&self) -> String {
        format!("Section {}", self.section)
    }
}

impl fmt::Display for ThaiActSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_th())
    }
}

/// Types of Thai legal instruments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThaiLegalInstrumentType {
    /// Constitution (รัฐธรรมนูญ)
    Constitution,

    /// Act of Parliament (พระราชบัญญัติ - พ.ร.บ.)
    Act,

    /// Royal Decree (พระราชกำหนด - พ.ร.ก.)
    RoyalDecree,

    /// Emergency Decree (พระราชกำหนด ฉุกเฉิน)
    EmergencyDecree,

    /// Ministerial Regulation (กฎกระทรวง)
    MinisterialRegulation,

    /// Ministerial Notification (ประกาศกระทรวง)
    MinisterialNotification,

    /// Royal Command (พระราชโองการ)
    RoyalCommand,

    /// Regulation (ระเบียบ)
    Regulation,

    /// Notification (ประกาศ)
    Notification,
}

impl ThaiLegalInstrumentType {
    /// Returns the Thai name of the instrument type
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::Constitution => "รัฐธรรมนูญ",
            Self::Act => "พระราชบัญญัติ",
            Self::RoyalDecree => "พระราชกำหนด",
            Self::EmergencyDecree => "พระราชกำหนด ฉุกเฉิน",
            Self::MinisterialRegulation => "กฎกระทรวง",
            Self::MinisterialNotification => "ประกาศกระทรวง",
            Self::RoyalCommand => "พระราชโองการ",
            Self::Regulation => "ระเบียบ",
            Self::Notification => "ประกาศ",
        }
    }

    /// Returns the English name of the instrument type
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Constitution => "Constitution",
            Self::Act => "Act",
            Self::RoyalDecree => "Royal Decree",
            Self::EmergencyDecree => "Emergency Decree",
            Self::MinisterialRegulation => "Ministerial Regulation",
            Self::MinisterialNotification => "Ministerial Notification",
            Self::RoyalCommand => "Royal Command",
            Self::Regulation => "Regulation",
            Self::Notification => "Notification",
        }
    }

    /// Returns the Thai abbreviation
    pub fn abbreviation_th(&self) -> &'static str {
        match self {
            Self::Constitution => "รธน.",
            Self::Act => "พ.ร.บ.",
            Self::RoyalDecree => "พ.ร.ก.",
            Self::EmergencyDecree => "พ.ร.ก. ฉุกเฉิน",
            Self::MinisterialRegulation => "กฎกระทรวง",
            Self::MinisterialNotification => "ประกาศกระทรวง",
            Self::RoyalCommand => "พ.ร.อ.",
            Self::Regulation => "ระเบียบ",
            Self::Notification => "ประกาศ",
        }
    }
}

impl fmt::Display for ThaiLegalInstrumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_th())
    }
}

/// Represents the Thai Constitution (รัฐธรรมนูญแห่งราชอาณาจักรไทย)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThaiConstitution {
    /// Buddhist Era year of enactment
    pub year: BuddhistYear,
}

impl ThaiConstitution {
    /// Creates a reference to the Thai Constitution
    pub fn new(year: BuddhistYear) -> Self {
        Self { year }
    }

    /// Creates a Constitution reference from Common Era year
    pub fn from_ce(ce_year: i32) -> Self {
        Self {
            year: BuddhistYear::from_ce(ce_year),
        }
    }

    /// Current constitution (2017 / B.E. 2560)
    pub fn current() -> Self {
        Self::from_ce(2017)
    }

    /// 1997 Constitution (B.E. 2540) - "People's Constitution"
    pub fn be_2540() -> Self {
        Self::from_ce(1997)
    }

    /// Creates a reference to a specific section
    pub fn section(&self, section: u32) -> ThaiConstitutionSection {
        ThaiConstitutionSection {
            constitution: self.clone(),
            section,
        }
    }

    /// Formats the constitution in Thai format
    pub fn format_th(&self) -> String {
        format!("รัฐธรรมนูญแห่งราชอาณาจักรไทย พ.ศ. {}", self.year.be_year)
    }

    /// Formats the constitution in English format
    pub fn format_en(&self) -> String {
        format!(
            "Constitution of the Kingdom of Thailand B.E. {} ({})",
            self.year.be_year,
            self.year.ce_year()
        )
    }
}

impl fmt::Display for ThaiConstitution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_th())
    }
}

/// Represents a specific section within the Thai Constitution
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThaiConstitutionSection {
    /// The constitution this section belongs to
    pub constitution: ThaiConstitution,

    /// Section number (มาตรา)
    pub section: u32,
}

impl ThaiConstitutionSection {
    /// Formats the section in Thai format
    pub fn format_th(&self) -> String {
        format!("{} มาตรา {}", self.constitution.format_th(), self.section)
    }

    /// Formats the section in English format
    pub fn format_en(&self) -> String {
        format!(
            "{}, Section {}",
            self.constitution.format_en(),
            self.section
        )
    }
}

impl fmt::Display for ThaiConstitutionSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_th())
    }
}

/// Thai court levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThaiCourtLevel {
    /// Supreme Court (ศาลฎีกา)
    SupremeCourt,

    /// Supreme Court Dika Region (ศาลฎีกาภูมิภาค)
    SupremeCourtRegion,

    /// Court of Appeal (ศาลอุทธรณ์)
    CourtOfAppeal,

    /// Court of Appeal Region (ศาลอุทธรณ์ภูมิภาค)
    CourtOfAppealRegion,

    /// Civil Court (ศาลแพ่ง)
    CivilCourt,

    /// Criminal Court (ศาลอาญา)
    CriminalCourt,

    /// Labour Court (ศาลแรงงาน)
    LabourCourt,

    /// Central Bankruptcy Court (ศาลล้มละลายกลาง)
    BankruptcyCourt,

    /// Tax Court (ศาลภาษีอากร)
    TaxCourt,

    /// Intellectual Property and International Trade Court (ศาลทรัพย์สินทางปัญญาและการค้าระหว่างประเทศ)
    IntellectualPropertyCourt,

    /// Juvenile and Family Court (ศาลเยาวชนและครอบครัว)
    JuvenileFamilyCourt,

    /// Administrative Court (ศาลปกครอง)
    AdministrativeCourt,

    /// Constitutional Court (ศาลรัฐธรรมนูญ)
    ConstitutionalCourt,

    /// Military Court (ศาลทหาร)
    MilitaryCourt,
}

impl ThaiCourtLevel {
    /// Returns the Thai name of the court
    pub fn name_th(&self) -> &'static str {
        match self {
            Self::SupremeCourt => "ศาลฎีกา",
            Self::SupremeCourtRegion => "ศาลฎีกาภูมิภาค",
            Self::CourtOfAppeal => "ศาลอุทธรณ์",
            Self::CourtOfAppealRegion => "ศาลอุทธรณ์ภูมิภาค",
            Self::CivilCourt => "ศาลแพ่ง",
            Self::CriminalCourt => "ศาลอาญา",
            Self::LabourCourt => "ศาลแรงงาน",
            Self::BankruptcyCourt => "ศาลล้มละลายกลาง",
            Self::TaxCourt => "ศาลภาษีอากร",
            Self::IntellectualPropertyCourt => "ศาลทรัพย์สินทางปัญญาและการค้าระหว่างประเทศ",
            Self::JuvenileFamilyCourt => "ศาลเยาวชนและครอบครัว",
            Self::AdministrativeCourt => "ศาลปกครอง",
            Self::ConstitutionalCourt => "ศาลรัฐธรรมนูญ",
            Self::MilitaryCourt => "ศาลทหาร",
        }
    }

    /// Returns the English name of the court
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::SupremeCourt => "Supreme Court",
            Self::SupremeCourtRegion => "Supreme Court Dika Region",
            Self::CourtOfAppeal => "Court of Appeal",
            Self::CourtOfAppealRegion => "Court of Appeal Region",
            Self::CivilCourt => "Civil Court",
            Self::CriminalCourt => "Criminal Court",
            Self::LabourCourt => "Labour Court",
            Self::BankruptcyCourt => "Central Bankruptcy Court",
            Self::TaxCourt => "Tax Court",
            Self::IntellectualPropertyCourt => {
                "Intellectual Property and International Trade Court"
            }
            Self::JuvenileFamilyCourt => "Juvenile and Family Court",
            Self::AdministrativeCourt => "Administrative Court",
            Self::ConstitutionalCourt => "Constitutional Court",
            Self::MilitaryCourt => "Military Court",
        }
    }
}

impl fmt::Display for ThaiCourtLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name_th())
    }
}

/// Represents a Thai court decision
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThaiCourtDecision {
    /// Court level
    pub court: ThaiCourtLevel,

    /// Decision number
    pub number: u32,

    /// Buddhist Era year
    pub year: BuddhistYear,

    /// Optional case name
    pub case_name: Option<String>,
}

impl ThaiCourtDecision {
    /// Creates a new court decision citation
    pub fn new(court: ThaiCourtLevel, number: u32, year: BuddhistYear) -> Self {
        Self {
            court,
            number,
            year,
            case_name: None,
        }
    }

    /// Sets the case name
    pub fn with_case_name(mut self, case_name: impl Into<String>) -> Self {
        self.case_name = Some(case_name.into());
        self
    }

    /// Formats the decision in Thai format
    pub fn format_th(&self) -> String {
        let mut result = format!(
            "คำพิพากษา{}ที่ {}/{}",
            self.court.name_th(),
            self.number,
            self.year.be_year
        );

        if let Some(name) = &self.case_name {
            result.push_str(&format!(" ({})", name));
        }

        result
    }

    /// Formats the decision in English format
    pub fn format_en(&self) -> String {
        let mut result = format!(
            "{} Decision No. {}/B.E. {}",
            self.court.name_en(),
            self.number,
            self.year.be_year
        );

        if let Some(name) = &self.case_name {
            result.push_str(&format!(" ({})", name));
        }

        result
    }
}

impl fmt::Display for ThaiCourtDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_th())
    }
}

/// Complete Thai legal citation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThaiCitation {
    /// Constitution citation
    Constitution(ThaiConstitutionSection),

    /// Act citation
    Act(ThaiActSection),

    /// Court decision citation
    CourtDecision(ThaiCourtDecision),
}

impl ThaiCitation {
    /// Formats the citation in Thai
    pub fn format_th(&self) -> String {
        match self {
            Self::Constitution(section) => section.format_th(),
            Self::Act(section) => section.format_th(),
            Self::CourtDecision(decision) => decision.format_th(),
        }
    }

    /// Formats the citation in English
    pub fn format_en(&self) -> String {
        match self {
            Self::Constitution(section) => section.format_en(),
            Self::Act(section) => section.format_en(),
            Self::CourtDecision(decision) => decision.format_en(),
        }
    }
}

impl fmt::Display for ThaiCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_th())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thai_act() {
        let pdpa = ThaiAct::new(
            "คุ้มครองข้อมูลส่วนบุคคล",
            "Personal Data Protection Act",
            BuddhistYear::from_be(2562),
        );

        assert_eq!(pdpa.format_th(), "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562");
        assert_eq!(
            pdpa.format_en(),
            "Personal Data Protection Act B.E. 2562 (2019)"
        );
    }

    #[test]
    fn test_thai_act_section() {
        let pdpa = ThaiAct::new(
            "คุ้มครองข้อมูลส่วนบุคคล",
            "Personal Data Protection Act",
            BuddhistYear::from_be(2562),
        );

        let section = pdpa.section(26);
        assert_eq!(
            section.format_th(),
            "พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562 มาตรา 26"
        );
        assert_eq!(
            section.format_en(),
            "Personal Data Protection Act B.E. 2562 (2019), Section 26"
        );
    }

    #[test]
    fn test_thai_act_section_with_paragraph() {
        let act = ThaiAct::new("ทดสอบ", "Test Act", BuddhistYear::from_be(2567));
        let section = act.section(10).with_paragraph(2);

        assert_eq!(
            section.format_th(),
            "พ.ร.บ. ทดสอบ พ.ศ. 2567 มาตรา 10 วรรค 2"
        );
    }

    #[test]
    fn test_thai_constitution() {
        let constitution = ThaiConstitution::current();
        assert_eq!(constitution.year.be_year, 2560);
        assert_eq!(
            constitution.format_th(),
            "รัฐธรรมนูญแห่งราชอาณาจักรไทย พ.ศ. 2560"
        );
        assert_eq!(
            constitution.format_en(),
            "Constitution of the Kingdom of Thailand B.E. 2560 (2017)"
        );
    }

    #[test]
    fn test_thai_constitution_section() {
        let constitution = ThaiConstitution::current();
        let section = constitution.section(26);

        assert_eq!(
            section.format_th(),
            "รัฐธรรมนูญแห่งราชอาณาจักรไทย พ.ศ. 2560 มาตรา 26"
        );
    }

    #[test]
    fn test_thai_legal_instrument_types() {
        assert_eq!(ThaiLegalInstrumentType::Act.name_th(), "พระราชบัญญัติ");
        assert_eq!(ThaiLegalInstrumentType::Act.abbreviation_th(), "พ.ร.บ.");
        assert_eq!(ThaiLegalInstrumentType::Constitution.name_th(), "รัฐธรรมนูญ");
        assert_eq!(
            ThaiLegalInstrumentType::RoyalDecree.name_th(),
            "พระราชกำหนด"
        );
    }

    #[test]
    fn test_thai_court_levels() {
        assert_eq!(ThaiCourtLevel::SupremeCourt.name_th(), "ศาลฎีกา");
        assert_eq!(ThaiCourtLevel::SupremeCourt.name_en(), "Supreme Court");
        assert_eq!(ThaiCourtLevel::LabourCourt.name_th(), "ศาลแรงงาน");
        assert_eq!(ThaiCourtLevel::LabourCourt.name_en(), "Labour Court");
    }

    #[test]
    fn test_thai_court_decision() {
        let decision = ThaiCourtDecision::new(
            ThaiCourtLevel::SupremeCourt,
            1234,
            BuddhistYear::from_be(2567),
        );

        assert_eq!(decision.format_th(), "คำพิพากษาศาลฎีกาที่ 1234/2567");
        assert_eq!(
            decision.format_en(),
            "Supreme Court Decision No. 1234/B.E. 2567"
        );
    }

    #[test]
    fn test_thai_act_short_names() {
        let pdpa = ThaiAct::new(
            "คุ้มครองข้อมูลส่วนบุคคล",
            "Personal Data Protection Act",
            BuddhistYear::from_be(2562),
        )
        .with_short_name_th("พ.ร.บ. คุ้มครองข้อมูล")
        .with_short_name_en("PDPA");

        assert_eq!(pdpa.format_short_th(), "พ.ร.บ. คุ้มครองข้อมูล พ.ศ. 2562");
        assert_eq!(pdpa.format_short_en(), "PDPA B.E. 2562");
    }

    #[test]
    fn test_major_thai_acts() {
        // Civil and Commercial Code
        let ccc = ThaiAct::from_ce("ประมวลกฎหมายแพ่งและพาณิชย์", "Civil and Commercial Code", 1992);
        assert_eq!(ccc.year.be_year, 2535);

        // Foreign Business Act
        let fba = ThaiAct::from_ce("ประกอบธุรกิจของคนต่างด้าว", "Foreign Business Act", 1999);
        assert_eq!(fba.year.be_year, 2542);

        // Labour Protection Act
        let lpa = ThaiAct::from_ce("คุ้มครองแรงงาน", "Labour Protection Act", 1998);
        assert_eq!(lpa.year.be_year, 2541);
    }
}
