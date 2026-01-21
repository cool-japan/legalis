//! Internationalization Support for Chinese Legal Texts
//!
//! Provides bilingual text structures for Chinese (中文) and English.
//! Chinese text is authoritative in Chinese law.
//!
//! # 双语支持 / Bilingual Support
//!
//! All legal texts are provided in both Chinese (authoritative) and English (translation).

use serde::{Deserialize, Serialize};
use std::fmt;

/// Bilingual text with Chinese as authoritative
///
/// # 双语文本结构
///
/// Chinese text (中文) is legally authoritative.
/// English text is provided for reference only.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BilingualText {
    /// Chinese text (中文) - authoritative / 权威文本
    pub zh: String,
    /// English text - translation / 英文翻译
    pub en: String,
}

impl BilingualText {
    /// Create a new bilingual text
    ///
    /// # Arguments
    /// * `zh` - Chinese text (authoritative)
    /// * `en` - English translation
    pub fn new(zh: impl Into<String>, en: impl Into<String>) -> Self {
        Self {
            zh: zh.into(),
            en: en.into(),
        }
    }

    /// Create from Chinese only (English empty)
    pub fn chinese_only(zh: impl Into<String>) -> Self {
        Self {
            zh: zh.into(),
            en: String::new(),
        }
    }

    /// Get the authoritative text (Chinese)
    pub fn authoritative(&self) -> &str {
        &self.zh
    }

    /// Get the translation (English)
    pub fn translation(&self) -> &str {
        &self.en
    }

    /// Check if translation is available
    pub fn has_translation(&self) -> bool {
        !self.en.is_empty()
    }

    /// Format as "Chinese (English)"
    pub fn format_both(&self) -> String {
        if self.has_translation() {
            format!("{} ({})", self.zh, self.en)
        } else {
            self.zh.clone()
        }
    }
}

impl fmt::Display for BilingualText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.zh)
    }
}

/// Locale preference for display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Locale {
    /// Chinese (中文) - default
    #[default]
    Chinese,
    /// English
    English,
    /// Both languages
    Both,
}

impl Locale {
    /// Get text based on locale preference
    pub fn select<'a>(&self, text: &'a BilingualText) -> &'a str {
        match self {
            Locale::Chinese => &text.zh,
            Locale::English => {
                if text.en.is_empty() {
                    &text.zh
                } else {
                    &text.en
                }
            }
            Locale::Both => &text.zh, // Use format_both() for both
        }
    }
}

/// Common legal terms in Chinese law
pub mod terms {
    use super::BilingualText;

    /// 法律 / Law
    pub fn law() -> BilingualText {
        BilingualText::new("法律", "Law")
    }

    /// 法规 / Regulation
    pub fn regulation() -> BilingualText {
        BilingualText::new("法规", "Regulation")
    }

    /// 条例 / Ordinance
    pub fn ordinance() -> BilingualText {
        BilingualText::new("条例", "Ordinance")
    }

    /// 规章 / Rules
    pub fn rules() -> BilingualText {
        BilingualText::new("规章", "Rules")
    }

    /// 第...条 / Article ...
    pub fn article(num: u32) -> BilingualText {
        BilingualText::new(format!("第{}条", num), format!("Article {}", num))
    }

    /// 第...款 / Paragraph ...
    pub fn paragraph(num: u32) -> BilingualText {
        BilingualText::new(format!("第{}款", num), format!("Paragraph {}", num))
    }

    /// 第...项 / Item ...
    pub fn item(num: u32) -> BilingualText {
        BilingualText::new(format!("第{}项", num), format!("Item {}", num))
    }

    /// 个人信息 / Personal Information
    pub fn personal_information() -> BilingualText {
        BilingualText::new("个人信息", "Personal Information")
    }

    /// 敏感个人信息 / Sensitive Personal Information
    pub fn sensitive_personal_information() -> BilingualText {
        BilingualText::new("敏感个人信息", "Sensitive Personal Information")
    }

    /// 个人信息处理者 / Personal Information Handler
    pub fn pi_handler() -> BilingualText {
        BilingualText::new("个人信息处理者", "Personal Information Handler")
    }

    /// 数据处理 / Data Processing
    pub fn data_processing() -> BilingualText {
        BilingualText::new("数据处理", "Data Processing")
    }

    /// 知情同意 / Informed Consent
    pub fn informed_consent() -> BilingualText {
        BilingualText::new("知情同意", "Informed Consent")
    }

    /// 关键信息基础设施 / Critical Information Infrastructure
    pub fn cii() -> BilingualText {
        BilingualText::new("关键信息基础设施", "Critical Information Infrastructure")
    }

    /// 网络运营者 / Network Operator
    pub fn network_operator() -> BilingualText {
        BilingualText::new("网络运营者", "Network Operator")
    }

    /// 重要数据 / Important Data
    pub fn important_data() -> BilingualText {
        BilingualText::new("重要数据", "Important Data")
    }

    /// 数据出境 / Cross-border Data Transfer
    pub fn cross_border_transfer() -> BilingualText {
        BilingualText::new("数据出境", "Cross-border Data Transfer")
    }

    /// 安全评估 / Security Assessment
    pub fn security_assessment() -> BilingualText {
        BilingualText::new("安全评估", "Security Assessment")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_text_creation() {
        let text = BilingualText::new("民法典", "Civil Code");
        assert_eq!(text.zh, "民法典");
        assert_eq!(text.en, "Civil Code");
        assert!(text.has_translation());
    }

    #[test]
    fn test_chinese_only() {
        let text = BilingualText::chinese_only("中华人民共和国");
        assert_eq!(text.zh, "中华人民共和国");
        assert!(text.en.is_empty());
        assert!(!text.has_translation());
    }

    #[test]
    fn test_format_both() {
        let text = BilingualText::new("个人信息保护法", "PIPL");
        assert_eq!(text.format_both(), "个人信息保护法 (PIPL)");
    }

    #[test]
    fn test_locale_selection() {
        let text = BilingualText::new("合同", "Contract");
        assert_eq!(Locale::Chinese.select(&text), "合同");
        assert_eq!(Locale::English.select(&text), "Contract");
    }

    #[test]
    fn test_locale_fallback() {
        let text = BilingualText::chinese_only("侵权");
        assert_eq!(Locale::English.select(&text), "侵权"); // Falls back to Chinese
    }

    #[test]
    fn test_terms() {
        let article = terms::article(5);
        assert_eq!(article.zh, "第5条");
        assert_eq!(article.en, "Article 5");
    }

    #[test]
    fn test_display() {
        let text = BilingualText::new("数据安全法", "Data Security Law");
        assert_eq!(format!("{}", text), "数据安全法");
    }
}
