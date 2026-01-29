//! Korean Legal Citation System
//!
//! Implements citation formats for Korean laws and regulations.
//!
//! # 법률 인용 형식 / Citation Format
//!
//! Standard format: 법률명 제X조 제Y항 제Z호
//!
//! Examples:
//! - 민법 제1조
//! - 개인정보 보호법 제15조 제1항
//! - 근로기준법 제50조 제1항 제1호

use crate::i18n::BilingualText;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Korean law citation
///
/// # 법률 인용
///
/// Format: 법률명 제X조 제Y항 제Z호
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    /// Law name in Korean
    pub law_name: BilingualText,
    /// Article number (조)
    pub article: u32,
    /// Paragraph number (항), optional
    pub paragraph: Option<u32>,
    /// Subparagraph number (호), optional
    pub subparagraph: Option<u32>,
}

impl Citation {
    /// Create a new citation
    pub fn new(law_name: BilingualText, article: u32) -> Self {
        Self {
            law_name,
            article,
            paragraph: None,
            subparagraph: None,
        }
    }

    /// Create citation with full reference
    pub fn full(
        law_name: BilingualText,
        article: u32,
        paragraph: Option<u32>,
        subparagraph: Option<u32>,
    ) -> Self {
        Self {
            law_name,
            article,
            paragraph,
            subparagraph,
        }
    }

    /// Add paragraph reference
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Add subparagraph reference
    pub fn with_subparagraph(mut self, subparagraph: u32) -> Self {
        self.subparagraph = Some(subparagraph);
        self
    }

    /// Format citation in Korean
    ///
    /// Example: 민법 제1조 제2항 제3호
    pub fn format_korean(&self) -> String {
        let mut result = format!("{} 제{}조", self.law_name.ko, self.article);

        if let Some(p) = self.paragraph {
            result.push_str(&format!(" 제{}항", p));
        }

        if let Some(s) = self.subparagraph {
            result.push_str(&format!(" 제{}호", s));
        }

        result
    }

    /// Format citation in English
    ///
    /// Example: Civil Code, Art. 1, Para. 2, Subpara. 3
    pub fn format_english(&self) -> String {
        let law_name = if self.law_name.en.is_empty() {
            &self.law_name.ko
        } else {
            &self.law_name.en
        };

        let mut result = format!("{}, Art. {}", law_name, self.article);

        if let Some(p) = self.paragraph {
            result.push_str(&format!(", Para. {}", p));
        }

        if let Some(s) = self.subparagraph {
            result.push_str(&format!(", Subpara. {}", s));
        }

        result
    }

    /// Format bilingual citation
    pub fn format_bilingual(&self) -> String {
        format!("{} / {}", self.format_korean(), self.format_english())
    }
}

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_korean())
    }
}

/// Major Korean laws with their official names
pub mod laws {
    use super::*;

    /// 민법 / Civil Code
    pub fn civil_code() -> BilingualText {
        BilingualText::new("민법", "Civil Code")
    }

    /// 형법 / Criminal Code
    pub fn criminal_code() -> BilingualText {
        BilingualText::new("형법", "Criminal Code")
    }

    /// 상법 / Commercial Code
    pub fn commercial_code() -> BilingualText {
        BilingualText::new("상법", "Commercial Code")
    }

    /// 근로기준법 / Labor Standards Act
    pub fn labor_standards_act() -> BilingualText {
        BilingualText::new("근로기준법", "Labor Standards Act")
    }

    /// 개인정보 보호법 / Personal Information Protection Act
    pub fn pipa() -> BilingualText {
        BilingualText::new("개인정보 보호법", "Personal Information Protection Act")
    }

    /// 고용보험법 / Employment Insurance Act
    pub fn employment_insurance_act() -> BilingualText {
        BilingualText::new("고용보험법", "Employment Insurance Act")
    }

    /// 산업재해보상보험법 / Industrial Accident Compensation Insurance Act
    pub fn workers_compensation_act() -> BilingualText {
        BilingualText::new(
            "산업재해보상보험법",
            "Industrial Accident Compensation Insurance Act",
        )
    }

    /// 소득세법 / Income Tax Act
    pub fn income_tax_act() -> BilingualText {
        BilingualText::new("소득세법", "Income Tax Act")
    }

    /// 법인세법 / Corporate Tax Act
    pub fn corporate_tax_act() -> BilingualText {
        BilingualText::new("법인세법", "Corporate Tax Act")
    }

    /// 부가가치세법 / Value-Added Tax Act
    pub fn vat_act() -> BilingualText {
        BilingualText::new("부가가치세법", "Value-Added Tax Act")
    }

    /// 독점규제 및 공정거래에 관한 법률 / Monopoly Regulation and Fair Trade Act
    pub fn fair_trade_act() -> BilingualText {
        BilingualText::new(
            "독점규제 및 공정거래에 관한 법률",
            "Monopoly Regulation and Fair Trade Act",
        )
    }

    /// 특허법 / Patent Act
    pub fn patent_act() -> BilingualText {
        BilingualText::new("특허법", "Patent Act")
    }

    /// 저작권법 / Copyright Act
    pub fn copyright_act() -> BilingualText {
        BilingualText::new("저작권법", "Copyright Act")
    }

    /// 상표법 / Trademark Act
    pub fn trademark_act() -> BilingualText {
        BilingualText::new("상표법", "Trademark Act")
    }

    /// 금융소비자 보호에 관한 법률 / Financial Consumer Protection Act
    pub fn financial_consumer_protection_act() -> BilingualText {
        BilingualText::new(
            "금융소비자 보호에 관한 법률",
            "Financial Consumer Protection Act",
        )
    }

    /// 자본시장과 금융투자업에 관한 법률 / Financial Investment Services and Capital Markets Act
    pub fn capital_markets_act() -> BilingualText {
        BilingualText::new(
            "자본시장과 금융투자업에 관한 법률",
            "Financial Investment Services and Capital Markets Act",
        )
    }
}

/// Create citation helpers
pub mod cite {
    use super::*;

    /// Create Civil Code citation
    pub fn civil_code(article: u32) -> Citation {
        Citation::new(laws::civil_code(), article)
    }

    /// Create Criminal Code citation
    pub fn criminal_code(article: u32) -> Citation {
        Citation::new(laws::criminal_code(), article)
    }

    /// Create Commercial Code citation
    pub fn commercial_code(article: u32) -> Citation {
        Citation::new(laws::commercial_code(), article)
    }

    /// Create Labor Standards Act citation
    pub fn labor_standards(article: u32) -> Citation {
        Citation::new(laws::labor_standards_act(), article)
    }

    /// Create PIPA citation
    pub fn pipa(article: u32) -> Citation {
        Citation::new(laws::pipa(), article)
    }

    /// Create Employment Insurance Act citation
    pub fn employment_insurance(article: u32) -> Citation {
        Citation::new(laws::employment_insurance_act(), article)
    }

    /// Create Workers' Compensation Act citation
    pub fn workers_compensation(article: u32) -> Citation {
        Citation::new(laws::workers_compensation_act(), article)
    }

    /// Create Income Tax Act citation
    pub fn income_tax(article: u32) -> Citation {
        Citation::new(laws::income_tax_act(), article)
    }

    /// Create Corporate Tax Act citation
    pub fn corporate_tax(article: u32) -> Citation {
        Citation::new(laws::corporate_tax_act(), article)
    }

    /// Create VAT Act citation
    pub fn vat(article: u32) -> Citation {
        Citation::new(laws::vat_act(), article)
    }

    /// Create Fair Trade Act citation
    pub fn fair_trade(article: u32) -> Citation {
        Citation::new(laws::fair_trade_act(), article)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_citation() {
        let cite = Citation::new(laws::civil_code(), 1);
        assert_eq!(cite.format_korean(), "민법 제1조");
    }

    #[test]
    fn test_citation_with_paragraph() {
        let cite = Citation::new(laws::pipa(), 15).with_paragraph(1);
        assert_eq!(cite.format_korean(), "개인정보 보호법 제15조 제1항");
    }

    #[test]
    fn test_citation_with_subparagraph() {
        let cite = Citation::new(laws::labor_standards_act(), 50)
            .with_paragraph(1)
            .with_subparagraph(1);
        assert_eq!(cite.format_korean(), "근로기준법 제50조 제1항 제1호");
    }

    #[test]
    fn test_english_citation() {
        let cite = Citation::new(laws::civil_code(), 509).with_paragraph(2);
        assert_eq!(cite.format_english(), "Civil Code, Art. 509, Para. 2");
    }

    #[test]
    fn test_bilingual_citation() {
        let cite = cite::pipa(15);
        let bilingual = cite.format_bilingual();
        assert!(bilingual.contains("개인정보 보호법"));
        assert!(bilingual.contains("Personal Information Protection Act"));
    }

    #[test]
    fn test_cite_helpers() {
        let cite = cite::civil_code(1000);
        assert_eq!(cite.article, 1000);
        assert_eq!(cite.law_name.ko, "민법");
    }

    #[test]
    fn test_display_trait() {
        let cite = cite::commercial_code(169);
        let display = format!("{}", cite);
        assert_eq!(display, "상법 제169조");
    }

    #[test]
    fn test_full_citation() {
        let cite = Citation::full(laws::commercial_code(), 147, Some(1), Some(2));
        assert_eq!(cite.format_korean(), "상법 제147조 제1항 제2호");
    }
}
