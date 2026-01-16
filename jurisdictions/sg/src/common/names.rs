//! Singapore multi-ethnic name formatting utilities.
//!
//! Singapore's multi-ethnic society requires handling of various name formats:
//!
//! # Ethnic Groups and Name Formats
//!
//! ## Chinese Names (华人)
//! - Format: Family name + Given name (no space)
//! - Example: 李伟明 (Lǐ Wěimíng) → Lee Wei Ming (romanized)
//! - Legal documents: Family name followed by given name
//!
//! ## Malay Names (马来人)
//! - Format: Given name + bin/binti + Father's name
//! - Example: Ahmad bin Ibrahim (son of Ibrahim)
//! - Example: Siti binti Abdullah (daughter of Abdullah)
//! - No family name in traditional format
//!
//! ## Indian Names (印度人)
//! - South Indian: Given name + s/o or d/o + Father's name
//! - Example: Ravi s/o Krishnan (son of Krishnan)
//! - Example: Lakshmi d/o Raman (daughter of Raman)
//! - North Indian: Given name + Family name (similar to Western)
//!
//! ## Western Names
//! - Format: Given name + Middle name(s) + Family name
//! - Example: John David Smith
//!
//! # NRIC Name Format
//!
//! Singapore NRIC shows name in the following format:
//! - Chinese: FAMILY NAME GIVEN NAME (romanized, all caps)
//! - Malay: GIVEN NAME BIN/BINTI FATHER'S NAME (all caps)
//! - Indian: GIVEN NAME S/O or D/O FATHER'S NAME (all caps)

use legalis_i18n::{Locale, NameFormatter, PersonName};
use serde::{Deserialize, Serialize};

/// Ethnic groups in Singapore for name formatting purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EthnicGroup {
    /// Chinese ethnicity (华人)
    Chinese,
    /// Malay ethnicity (马来人)
    Malay,
    /// Indian ethnicity (印度人)
    Indian,
    /// Eurasian or Western
    Western,
    /// Other ethnic groups
    Other,
}

impl EthnicGroup {
    /// Returns the Chinese name for this ethnic group.
    #[must_use]
    pub fn chinese_name(&self) -> &'static str {
        match self {
            Self::Chinese => "华人",
            Self::Malay => "马来人",
            Self::Indian => "印度人",
            Self::Western => "欧裔",
            Self::Other => "其他",
        }
    }

    /// Returns the Malay name for this ethnic group.
    #[must_use]
    pub fn malay_name(&self) -> &'static str {
        match self {
            Self::Chinese => "Cina",
            Self::Malay => "Melayu",
            Self::Indian => "India",
            Self::Western => "Eropah",
            Self::Other => "Lain-lain",
        }
    }
}

/// Singapore person name with ethnic-specific formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingaporePersonName {
    /// Primary given name
    pub given_name: String,
    /// Family name (for Chinese and Western)
    pub family_name: Option<String>,
    /// Middle name or second given name
    pub middle_name: Option<String>,
    /// Father's name (for Malay and Indian)
    pub father_name: Option<String>,
    /// Ethnic group for formatting
    pub ethnic_group: EthnicGroup,
    /// Gender for bin/binti, s/o/d/o determination
    pub is_male: Option<bool>,
    /// Chinese characters name (if applicable)
    pub chinese_name: Option<String>,
    /// Honorific prefix (Encik, Puan, Mr., Mrs., etc.)
    pub prefix: Option<String>,
    /// Suffix (Jr., PhD, etc.)
    pub suffix: Option<String>,
}

impl SingaporePersonName {
    /// Creates a new Chinese name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporePersonName;
    ///
    /// let name = SingaporePersonName::chinese("Wei Ming", "Lee")
    ///     .with_chinese_name("李伟明");
    /// assert_eq!(name.format_full(), "Lee Wei Ming");
    /// ```
    #[must_use]
    pub fn chinese(given_name: impl Into<String>, family_name: impl Into<String>) -> Self {
        Self {
            given_name: given_name.into(),
            family_name: Some(family_name.into()),
            middle_name: None,
            father_name: None,
            ethnic_group: EthnicGroup::Chinese,
            is_male: None,
            chinese_name: None,
            prefix: None,
            suffix: None,
        }
    }

    /// Creates a new Malay name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporePersonName;
    ///
    /// let name = SingaporePersonName::malay("Ahmad", "Ibrahim", true);
    /// assert_eq!(name.format_full(), "Ahmad bin Ibrahim");
    ///
    /// let name = SingaporePersonName::malay("Siti", "Abdullah", false);
    /// assert_eq!(name.format_full(), "Siti binti Abdullah");
    /// ```
    #[must_use]
    pub fn malay(
        given_name: impl Into<String>,
        father_name: impl Into<String>,
        is_male: bool,
    ) -> Self {
        Self {
            given_name: given_name.into(),
            family_name: None,
            middle_name: None,
            father_name: Some(father_name.into()),
            ethnic_group: EthnicGroup::Malay,
            is_male: Some(is_male),
            chinese_name: None,
            prefix: None,
            suffix: None,
        }
    }

    /// Creates a new Indian (South Indian) name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporePersonName;
    ///
    /// let name = SingaporePersonName::indian("Ravi", "Krishnan", true);
    /// assert_eq!(name.format_full(), "Ravi s/o Krishnan");
    ///
    /// let name = SingaporePersonName::indian("Lakshmi", "Raman", false);
    /// assert_eq!(name.format_full(), "Lakshmi d/o Raman");
    /// ```
    #[must_use]
    pub fn indian(
        given_name: impl Into<String>,
        father_name: impl Into<String>,
        is_male: bool,
    ) -> Self {
        Self {
            given_name: given_name.into(),
            family_name: None,
            middle_name: None,
            father_name: Some(father_name.into()),
            ethnic_group: EthnicGroup::Indian,
            is_male: Some(is_male),
            chinese_name: None,
            prefix: None,
            suffix: None,
        }
    }

    /// Creates a new Western-style name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporePersonName;
    ///
    /// let name = SingaporePersonName::western("John", "Smith")
    ///     .with_middle_name("David");
    /// assert_eq!(name.format_full(), "John David Smith");
    /// ```
    #[must_use]
    pub fn western(given_name: impl Into<String>, family_name: impl Into<String>) -> Self {
        Self {
            given_name: given_name.into(),
            family_name: Some(family_name.into()),
            middle_name: None,
            father_name: None,
            ethnic_group: EthnicGroup::Western,
            is_male: None,
            chinese_name: None,
            prefix: None,
            suffix: None,
        }
    }

    /// Sets the Chinese characters name.
    #[must_use]
    pub fn with_chinese_name(mut self, name: impl Into<String>) -> Self {
        self.chinese_name = Some(name.into());
        self
    }

    /// Sets the middle name.
    #[must_use]
    pub fn with_middle_name(mut self, name: impl Into<String>) -> Self {
        self.middle_name = Some(name.into());
        self
    }

    /// Sets the prefix/honorific.
    #[must_use]
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the suffix.
    #[must_use]
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Formats the full name according to ethnic conventions.
    #[must_use]
    pub fn format_full(&self) -> String {
        match self.ethnic_group {
            EthnicGroup::Chinese => self.format_chinese(),
            EthnicGroup::Malay => self.format_malay(),
            EthnicGroup::Indian => self.format_indian(),
            EthnicGroup::Western | EthnicGroup::Other => self.format_western(),
        }
    }

    /// Formats name in Chinese style (Family Given).
    fn format_chinese(&self) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &self.prefix {
            parts.push(prefix.clone());
        }

        if let Some(family) = &self.family_name {
            parts.push(family.clone());
        }

        parts.push(self.given_name.clone());

        if let Some(middle) = &self.middle_name {
            parts.push(middle.clone());
        }

        if let Some(suffix) = &self.suffix {
            parts.push(suffix.clone());
        }

        parts.join(" ")
    }

    /// Formats name in Malay style (Given bin/binti Father).
    fn format_malay(&self) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &self.prefix {
            parts.push(prefix.clone());
        }

        parts.push(self.given_name.clone());

        if let Some(father) = &self.father_name {
            let connector = if self.is_male.unwrap_or(true) {
                "bin"
            } else {
                "binti"
            };
            parts.push(connector.to_string());
            parts.push(father.clone());
        }

        if let Some(suffix) = &self.suffix {
            parts.push(suffix.clone());
        }

        parts.join(" ")
    }

    /// Formats name in Indian style (Given s/o or d/o Father).
    fn format_indian(&self) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &self.prefix {
            parts.push(prefix.clone());
        }

        parts.push(self.given_name.clone());

        if let Some(father) = &self.father_name {
            let connector = if self.is_male.unwrap_or(true) {
                "s/o"
            } else {
                "d/o"
            };
            parts.push(connector.to_string());
            parts.push(father.clone());
        }

        if let Some(suffix) = &self.suffix {
            parts.push(suffix.clone());
        }

        parts.join(" ")
    }

    /// Formats name in Western style (Given Middle Family).
    fn format_western(&self) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &self.prefix {
            parts.push(prefix.clone());
        }

        parts.push(self.given_name.clone());

        if let Some(middle) = &self.middle_name {
            parts.push(middle.clone());
        }

        if let Some(family) = &self.family_name {
            parts.push(family.clone());
        }

        if let Some(suffix) = &self.suffix {
            parts.push(suffix.clone());
        }

        parts.join(" ")
    }

    /// Formats name for NRIC style (all caps, specific format).
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporePersonName;
    ///
    /// let name = SingaporePersonName::chinese("Wei Ming", "Lee");
    /// assert_eq!(name.format_nric(), "LEE WEI MING");
    ///
    /// let name = SingaporePersonName::malay("Ahmad", "Ibrahim", true);
    /// assert_eq!(name.format_nric(), "AHMAD BIN IBRAHIM");
    /// ```
    #[must_use]
    pub fn format_nric(&self) -> String {
        self.format_full().to_uppercase()
    }

    /// Formats name with Chinese characters (if available).
    ///
    /// # Example
    ///
    /// ```rust
    /// use legalis_sg::common::SingaporePersonName;
    ///
    /// let name = SingaporePersonName::chinese("Wei Ming", "Lee")
    ///     .with_chinese_name("李伟明");
    /// assert_eq!(name.format_with_chinese(), "Lee Wei Ming (李伟明)");
    /// ```
    #[must_use]
    pub fn format_with_chinese(&self) -> String {
        if let Some(chinese) = &self.chinese_name {
            format!("{} ({})", self.format_full(), chinese)
        } else {
            self.format_full()
        }
    }

    /// Returns just the given name.
    #[must_use]
    pub fn given_name(&self) -> &str {
        &self.given_name
    }

    /// Returns the family name if present.
    #[must_use]
    pub fn family_name(&self) -> Option<&str> {
        self.family_name.as_deref()
    }

    /// Returns the father's name if present.
    #[must_use]
    pub fn father_name(&self) -> Option<&str> {
        self.father_name.as_deref()
    }
}

/// Singapore name formatter with support for multiple ethnic formats.
#[derive(Debug, Clone)]
pub struct SingaporeNameFormatter {
    default_ethnic_group: EthnicGroup,
}

impl Default for SingaporeNameFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl SingaporeNameFormatter {
    /// Creates a new Singapore name formatter.
    #[must_use]
    pub fn new() -> Self {
        Self {
            default_ethnic_group: EthnicGroup::Chinese,
        }
    }

    /// Creates a formatter with a specific default ethnic group.
    #[must_use]
    pub fn with_default_ethnic_group(ethnic_group: EthnicGroup) -> Self {
        Self {
            default_ethnic_group: ethnic_group,
        }
    }

    /// Formats a PersonName using Singapore conventions.
    ///
    /// Uses the default ethnic group for formatting.
    #[must_use]
    pub fn format(&self, name: &PersonName) -> String {
        match self.default_ethnic_group {
            EthnicGroup::Chinese => {
                format!("{} {}", name.family_name, name.given_name)
            }
            EthnicGroup::Western | EthnicGroup::Other => {
                let locale = Locale::new("en").with_country("SG");
                let formatter = NameFormatter::new(locale);
                formatter.format_full_name(name)
            }
            EthnicGroup::Malay | EthnicGroup::Indian => {
                // For Malay/Indian, use given name only (requires father's name for full format)
                name.given_name.clone()
            }
        }
    }

    /// Formats a name in citation style (Family, Given).
    #[must_use]
    pub fn format_citation(&self, name: &PersonName) -> String {
        let locale = Locale::new("en").with_country("SG");
        let formatter = NameFormatter::new(locale);
        formatter.format_citation(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Chinese name tests
    // ========================================================================

    #[test]
    fn test_chinese_name_format() {
        let name = SingaporePersonName::chinese("Wei Ming", "Lee");
        assert_eq!(name.format_full(), "Lee Wei Ming");
        assert_eq!(name.format_nric(), "LEE WEI MING");
    }

    #[test]
    fn test_chinese_name_with_chinese_chars() {
        let name = SingaporePersonName::chinese("Wei Ming", "Lee").with_chinese_name("李伟明");
        assert_eq!(name.format_with_chinese(), "Lee Wei Ming (李伟明)");
    }

    #[test]
    fn test_chinese_name_with_prefix() {
        let name = SingaporePersonName::chinese("Wei Ming", "Lee").with_prefix("Mr.");
        assert_eq!(name.format_full(), "Mr. Lee Wei Ming");
    }

    // ========================================================================
    // Malay name tests
    // ========================================================================

    #[test]
    fn test_malay_male_name() {
        let name = SingaporePersonName::malay("Ahmad", "Ibrahim", true);
        assert_eq!(name.format_full(), "Ahmad bin Ibrahim");
        assert_eq!(name.format_nric(), "AHMAD BIN IBRAHIM");
    }

    #[test]
    fn test_malay_female_name() {
        let name = SingaporePersonName::malay("Siti", "Abdullah", false);
        assert_eq!(name.format_full(), "Siti binti Abdullah");
    }

    #[test]
    fn test_malay_name_with_prefix() {
        let name = SingaporePersonName::malay("Ahmad", "Ibrahim", true).with_prefix("Encik");
        assert_eq!(name.format_full(), "Encik Ahmad bin Ibrahim");
    }

    // ========================================================================
    // Indian name tests
    // ========================================================================

    #[test]
    fn test_indian_male_name() {
        let name = SingaporePersonName::indian("Ravi", "Krishnan", true);
        assert_eq!(name.format_full(), "Ravi s/o Krishnan");
        assert_eq!(name.format_nric(), "RAVI S/O KRISHNAN");
    }

    #[test]
    fn test_indian_female_name() {
        let name = SingaporePersonName::indian("Lakshmi", "Raman", false);
        assert_eq!(name.format_full(), "Lakshmi d/o Raman");
    }

    // ========================================================================
    // Western name tests
    // ========================================================================

    #[test]
    fn test_western_name() {
        let name = SingaporePersonName::western("John", "Smith");
        assert_eq!(name.format_full(), "John Smith");
    }

    #[test]
    fn test_western_name_with_middle() {
        let name = SingaporePersonName::western("John", "Smith").with_middle_name("David");
        assert_eq!(name.format_full(), "John David Smith");
    }

    #[test]
    fn test_western_name_with_prefix_suffix() {
        let name = SingaporePersonName::western("John", "Smith")
            .with_prefix("Dr.")
            .with_suffix("PhD");
        assert_eq!(name.format_full(), "Dr. John Smith PhD");
    }

    // ========================================================================
    // Ethnic group tests
    // ========================================================================

    #[test]
    fn test_ethnic_group_names() {
        assert_eq!(EthnicGroup::Chinese.chinese_name(), "华人");
        assert_eq!(EthnicGroup::Malay.malay_name(), "Melayu");
        assert_eq!(EthnicGroup::Indian.chinese_name(), "印度人");
    }

    // ========================================================================
    // Formatter tests
    // ========================================================================

    #[test]
    fn test_singapore_name_formatter() {
        let formatter = SingaporeNameFormatter::new();
        let name = PersonName::new("Wei Ming", "Lee");
        assert_eq!(formatter.format(&name), "Lee Wei Ming");
    }

    #[test]
    fn test_singapore_name_formatter_citation() {
        let formatter = SingaporeNameFormatter::new();
        let name = PersonName::new("Wei Ming", "Lee");
        assert_eq!(formatter.format_citation(&name), "Lee, Wei Ming");
    }
}
