//! Internationalization Support for Korean Legal Texts
//!
//! Provides bilingual text structures for Korean (한국어) and English.
//! Korean text is authoritative in Korean law.
//!
//! # 이중 언어 지원 / Bilingual Support
//!
//! All legal texts are provided in both Korean (authoritative) and English (translation).

use serde::{Deserialize, Serialize};
use std::fmt;

/// Bilingual text with Korean as authoritative
///
/// # 이중 언어 텍스트 구조
///
/// Korean text (한국어) is legally authoritative.
/// English text is provided for reference only.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BilingualText {
    /// Korean text (한국어) - authoritative / 권위 있는 텍스트
    pub ko: String,
    /// English text - translation / 영문 번역
    pub en: String,
}

impl BilingualText {
    /// Create a new bilingual text
    ///
    /// # Arguments
    /// * `ko` - Korean text (authoritative)
    /// * `en` - English translation
    pub fn new(ko: impl Into<String>, en: impl Into<String>) -> Self {
        Self {
            ko: ko.into(),
            en: en.into(),
        }
    }

    /// Create from Korean only (English empty)
    pub fn korean_only(ko: impl Into<String>) -> Self {
        Self {
            ko: ko.into(),
            en: String::new(),
        }
    }

    /// Get the authoritative text (Korean)
    pub fn authoritative(&self) -> &str {
        &self.ko
    }

    /// Get the translation (English)
    pub fn translation(&self) -> &str {
        &self.en
    }

    /// Check if translation is available
    pub fn has_translation(&self) -> bool {
        !self.en.is_empty()
    }

    /// Format as "Korean (English)"
    pub fn format_both(&self) -> String {
        if self.has_translation() {
            format!("{} ({})", self.ko, self.en)
        } else {
            self.ko.clone()
        }
    }
}

impl fmt::Display for BilingualText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ko)
    }
}

/// Locale preference for display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Locale {
    /// Korean (한국어) - default
    #[default]
    Korean,
    /// English
    English,
    /// Both languages
    Both,
}

impl Locale {
    /// Get text based on locale preference
    pub fn select<'a>(&self, text: &'a BilingualText) -> &'a str {
        match self {
            Locale::Korean => &text.ko,
            Locale::English => {
                if text.en.is_empty() {
                    &text.ko
                } else {
                    &text.en
                }
            }
            Locale::Both => &text.ko, // Use format_both() for both
        }
    }
}

/// Common legal terms in Korean law
pub mod terms {
    use super::BilingualText;

    /// 법률 / Law
    pub fn law() -> BilingualText {
        BilingualText::new("법률", "Law")
    }

    /// 규정 / Regulation
    pub fn regulation() -> BilingualText {
        BilingualText::new("규정", "Regulation")
    }

    /// 조례 / Ordinance
    pub fn ordinance() -> BilingualText {
        BilingualText::new("조례", "Ordinance")
    }

    /// 규칙 / Rules
    pub fn rules() -> BilingualText {
        BilingualText::new("규칙", "Rules")
    }

    /// 제...조 / Article ...
    pub fn article(num: u32) -> BilingualText {
        BilingualText::new(format!("제{}조", num), format!("Article {}", num))
    }

    /// 제...항 / Paragraph ...
    pub fn paragraph(num: u32) -> BilingualText {
        BilingualText::new(format!("제{}항", num), format!("Paragraph {}", num))
    }

    /// 제...호 / Subparagraph ...
    pub fn subparagraph(num: u32) -> BilingualText {
        BilingualText::new(format!("제{}호", num), format!("Subparagraph {}", num))
    }

    /// 개인정보 / Personal Information
    pub fn personal_information() -> BilingualText {
        BilingualText::new("개인정보", "Personal Information")
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bilingual_text_creation() {
        let text = BilingualText::new("민법", "Civil Code");
        assert_eq!(text.ko, "민법");
        assert_eq!(text.en, "Civil Code");
        assert!(text.has_translation());
    }

    #[test]
    fn test_korean_only() {
        let text = BilingualText::korean_only("대한민국");
        assert_eq!(text.ko, "대한민국");
        assert!(text.en.is_empty());
        assert!(!text.has_translation());
    }

    #[test]
    fn test_format_both() {
        let text = BilingualText::new("개인정보 보호법", "PIPA");
        assert_eq!(text.format_both(), "개인정보 보호법 (PIPA)");
    }

    #[test]
    fn test_locale_selection() {
        let text = BilingualText::new("계약", "Contract");
        assert_eq!(Locale::Korean.select(&text), "계약");
        assert_eq!(Locale::English.select(&text), "Contract");
    }

    #[test]
    fn test_locale_fallback() {
        let text = BilingualText::korean_only("불법행위");
        assert_eq!(Locale::English.select(&text), "불법행위"); // Falls back to Korean
    }

    #[test]
    fn test_terms() {
        let article = terms::article(5);
        assert_eq!(article.ko, "제5조");
        assert_eq!(article.en, "Article 5");
    }

    #[test]
    fn test_display() {
        let text = BilingualText::new("상법", "Commercial Code");
        assert_eq!(format!("{}", text), "상법");
    }
}
