//! Chinese Legal Citation System
//!
//! Implements citation formats for Chinese laws and regulations.
//!
//! # 法律引用格式 / Citation Format
//!
//! Standard format: 《法律名称》第X条第Y款第Z项
//!
//! Examples:
//! - 《中华人民共和国民法典》第1条
//! - 《个人信息保护法》第13条第1款
//! - 《网络安全法》第21条第1款第3项

use crate::i18n::BilingualText;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Chinese law citation
///
/// # 法律引用
///
/// Format: 《法律名称》第X条第Y款第Z项
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    /// Law name in Chinese (with 《》 brackets)
    pub law_name: BilingualText,
    /// Article number (条)
    pub article: u32,
    /// Paragraph number (款), optional
    pub paragraph: Option<u32>,
    /// Item number (项), optional
    pub item: Option<u32>,
    /// Sub-item number (目), optional
    pub sub_item: Option<u32>,
}

impl Citation {
    /// Create a new citation
    pub fn new(law_name: BilingualText, article: u32) -> Self {
        Self {
            law_name,
            article,
            paragraph: None,
            item: None,
            sub_item: None,
        }
    }

    /// Create citation with full reference
    pub fn full(
        law_name: BilingualText,
        article: u32,
        paragraph: Option<u32>,
        item: Option<u32>,
    ) -> Self {
        Self {
            law_name,
            article,
            paragraph,
            item,
            sub_item: None,
        }
    }

    /// Add paragraph reference
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Add item reference
    pub fn with_item(mut self, item: u32) -> Self {
        self.item = Some(item);
        self
    }

    /// Add sub-item reference
    pub fn with_sub_item(mut self, sub_item: u32) -> Self {
        self.sub_item = Some(sub_item);
        self
    }

    /// Format citation in Chinese
    ///
    /// Example: 《民法典》第1条第2款第3项
    pub fn format_chinese(&self) -> String {
        let mut result = format!("《{}》第{}条", self.law_name.zh, self.article);

        if let Some(p) = self.paragraph {
            result.push_str(&format!("第{}款", p));
        }

        if let Some(i) = self.item {
            result.push_str(&format!("第{}项", i));
        }

        if let Some(s) = self.sub_item {
            result.push_str(&format!("第{}目", s));
        }

        result
    }

    /// Format citation in English
    ///
    /// Example: Civil Code, Art. 1, Para. 2, Item 3
    pub fn format_english(&self) -> String {
        let law_name = if self.law_name.en.is_empty() {
            &self.law_name.zh
        } else {
            &self.law_name.en
        };

        let mut result = format!("{}, Art. {}", law_name, self.article);

        if let Some(p) = self.paragraph {
            result.push_str(&format!(", Para. {}", p));
        }

        if let Some(i) = self.item {
            result.push_str(&format!(", Item {}", i));
        }

        if let Some(s) = self.sub_item {
            result.push_str(&format!(", Sub-item {}", s));
        }

        result
    }

    /// Format bilingual citation
    pub fn format_bilingual(&self) -> String {
        format!("{} / {}", self.format_chinese(), self.format_english())
    }
}

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_chinese())
    }
}

/// Major Chinese laws with their official names
pub mod laws {
    use super::*;

    /// 中华人民共和国民法典 / Civil Code of the PRC
    pub fn civil_code() -> BilingualText {
        BilingualText::new("中华人民共和国民法典", "Civil Code of the PRC")
    }

    /// 中华人民共和国个人信息保护法 / Personal Information Protection Law
    pub fn pipl() -> BilingualText {
        BilingualText::new(
            "中华人民共和国个人信息保护法",
            "Personal Information Protection Law of the PRC",
        )
    }

    /// 中华人民共和国网络安全法 / Cybersecurity Law
    pub fn cybersecurity_law() -> BilingualText {
        BilingualText::new("中华人民共和国网络安全法", "Cybersecurity Law of the PRC")
    }

    /// 中华人民共和国数据安全法 / Data Security Law
    pub fn data_security_law() -> BilingualText {
        BilingualText::new("中华人民共和国数据安全法", "Data Security Law of the PRC")
    }

    /// 中华人民共和国公司法 / Company Law
    pub fn company_law() -> BilingualText {
        BilingualText::new("中华人民共和国公司法", "Company Law of the PRC")
    }

    /// 中华人民共和国劳动合同法 / Labor Contract Law
    pub fn labor_contract_law() -> BilingualText {
        BilingualText::new("中华人民共和国劳动合同法", "Labor Contract Law of the PRC")
    }

    /// 中华人民共和国外商投资法 / Foreign Investment Law
    pub fn foreign_investment_law() -> BilingualText {
        BilingualText::new(
            "中华人民共和国外商投资法",
            "Foreign Investment Law of the PRC",
        )
    }

    /// 中华人民共和国反垄断法 / Anti-Monopoly Law
    pub fn anti_monopoly_law() -> BilingualText {
        BilingualText::new("中华人民共和国反垄断法", "Anti-Monopoly Law of the PRC")
    }

    /// 中华人民共和国合同法 / Contract Law (superseded by Civil Code)
    pub fn contract_law() -> BilingualText {
        BilingualText::new("中华人民共和国合同法", "Contract Law of the PRC")
    }

    /// 中华人民共和国侵权责任法 / Tort Liability Law (superseded by Civil Code)
    pub fn tort_law() -> BilingualText {
        BilingualText::new("中华人民共和国侵权责任法", "Tort Liability Law of the PRC")
    }

    /// 中华人民共和国物权法 / Property Law (superseded by Civil Code)
    pub fn property_law() -> BilingualText {
        BilingualText::new("中华人民共和国物权法", "Property Law of the PRC")
    }

    /// 中华人民共和国刑法 / Criminal Law
    pub fn criminal_law() -> BilingualText {
        BilingualText::new("中华人民共和国刑法", "Criminal Law of the PRC")
    }

    /// 中华人民共和国宪法 / Constitution
    pub fn constitution() -> BilingualText {
        BilingualText::new("中华人民共和国宪法", "Constitution of the PRC")
    }

    /// 中华人民共和国电子商务法 / E-Commerce Law
    pub fn ecommerce_law() -> BilingualText {
        BilingualText::new("中华人民共和国电子商务法", "E-Commerce Law of the PRC")
    }

    /// 中华人民共和国消费者权益保护法 / Consumer Protection Law
    pub fn consumer_protection_law() -> BilingualText {
        BilingualText::new(
            "中华人民共和国消费者权益保护法",
            "Consumer Protection Law of the PRC",
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

    /// Create PIPL citation
    pub fn pipl(article: u32) -> Citation {
        Citation::new(laws::pipl(), article)
    }

    /// Create Cybersecurity Law citation
    pub fn cybersecurity(article: u32) -> Citation {
        Citation::new(laws::cybersecurity_law(), article)
    }

    /// Create Data Security Law citation
    pub fn data_security(article: u32) -> Citation {
        Citation::new(laws::data_security_law(), article)
    }

    /// Create Company Law citation
    pub fn company(article: u32) -> Citation {
        Citation::new(laws::company_law(), article)
    }

    /// Create Company Law citation (alias)
    pub fn company_law(article: u32) -> Citation {
        Citation::new(laws::company_law(), article)
    }

    /// Create Labor Contract Law citation
    pub fn labor_contract(article: u32) -> Citation {
        Citation::new(laws::labor_contract_law(), article)
    }

    /// Create Foreign Investment Law citation
    pub fn foreign_investment(article: u32) -> Citation {
        Citation::new(laws::foreign_investment_law(), article)
    }

    /// Create Anti-Monopoly Law citation
    pub fn anti_monopoly(article: u32) -> Citation {
        Citation::new(laws::anti_monopoly_law(), article)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_citation() {
        let cite = Citation::new(laws::civil_code(), 1);
        assert_eq!(cite.format_chinese(), "《中华人民共和国民法典》第1条");
    }

    #[test]
    fn test_citation_with_paragraph() {
        let cite = Citation::new(laws::pipl(), 13).with_paragraph(1);
        assert_eq!(
            cite.format_chinese(),
            "《中华人民共和国个人信息保护法》第13条第1款"
        );
    }

    #[test]
    fn test_citation_with_item() {
        let cite = Citation::new(laws::cybersecurity_law(), 21)
            .with_paragraph(1)
            .with_item(3);
        assert_eq!(
            cite.format_chinese(),
            "《中华人民共和国网络安全法》第21条第1款第3项"
        );
    }

    #[test]
    fn test_english_citation() {
        let cite = Citation::new(laws::civil_code(), 509).with_paragraph(2);
        assert_eq!(
            cite.format_english(),
            "Civil Code of the PRC, Art. 509, Para. 2"
        );
    }

    #[test]
    fn test_bilingual_citation() {
        let cite = cite::pipl(4);
        let bilingual = cite.format_bilingual();
        assert!(bilingual.contains("《中华人民共和国个人信息保护法》"));
        assert!(bilingual.contains("Personal Information Protection Law"));
    }

    #[test]
    fn test_cite_helpers() {
        let cite = cite::civil_code(1000);
        assert_eq!(cite.article, 1000);
        assert_eq!(cite.law_name.zh, "中华人民共和国民法典");
    }

    #[test]
    fn test_display_trait() {
        let cite = cite::data_security(21);
        let display = format!("{}", cite);
        assert_eq!(display, "《中华人民共和国数据安全法》第21条");
    }

    #[test]
    fn test_full_citation() {
        let cite = Citation::full(laws::company_law(), 147, Some(1), Some(2));
        assert_eq!(
            cite.format_chinese(),
            "《中华人民共和国公司法》第147条第1款第2项"
        );
    }
}
