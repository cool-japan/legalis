//! Japanese law types and structures.
//!
//! Provides types for representing Japanese legislation according to
//! the official classification system.

use serde::{Deserialize, Serialize};

use crate::era::{Era, JapaneseDate};

/// A Japanese law with its official metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JapaneseLaw {
    /// Law number (法令番号)
    pub law_number: LawNumber,
    /// Law type
    pub law_type: LawType,
    /// Official title
    pub title: String,
    /// Alternative/abbreviated title
    pub short_title: Option<String>,
    /// Promulgation date
    pub promulgation_date: JapaneseDate,
    /// Effective date (may differ from promulgation)
    pub effective_date: Option<JapaneseDate>,
    /// Last amendment date
    pub last_amended: Option<JapaneseDate>,
    /// Administering ministry/agency
    pub ministry: Option<String>,
    /// Whether the law is currently in force
    pub is_active: bool,
}

/// Types of Japanese laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LawType {
    /// 憲法 (Constitution)
    Constitution,
    /// 法律 (Act/Law passed by Diet)
    Act,
    /// 政令 (Cabinet Order)
    CabinetOrder,
    /// 府令 (Prime Minister's Office Order)
    PrimeMinisterOrder,
    /// 省令 (Ministerial Ordinance)
    MinisterialOrdinance,
    /// 規則 (Rules/Regulations)
    Rule,
    /// 条約 (Treaty)
    Treaty,
    /// 条例 (Local Ordinance)
    LocalOrdinance,
}

impl LawType {
    /// Returns the Japanese name of the law type.
    #[must_use]
    pub fn japanese_name(&self) -> &'static str {
        match self {
            LawType::Constitution => "憲法",
            LawType::Act => "法律",
            LawType::CabinetOrder => "政令",
            LawType::PrimeMinisterOrder => "府令",
            LawType::MinisterialOrdinance => "省令",
            LawType::Rule => "規則",
            LawType::Treaty => "条約",
            LawType::LocalOrdinance => "条例",
        }
    }

    /// Returns the hierarchy level (lower = higher authority).
    #[must_use]
    pub fn hierarchy_level(&self) -> u8 {
        match self {
            LawType::Constitution => 1,
            LawType::Treaty => 2,
            LawType::Act => 3,
            LawType::CabinetOrder => 4,
            LawType::PrimeMinisterOrder => 5,
            LawType::MinisterialOrdinance => 5,
            LawType::Rule => 6,
            LawType::LocalOrdinance => 7,
        }
    }
}

impl std::fmt::Display for LawType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.japanese_name())
    }
}

/// Japanese law number format.
///
/// Example: "令和元年法律第一号" (Act No. 1 of Reiwa 1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LawNumber {
    /// Era
    pub era: Era,
    /// Year within era
    pub year: u32,
    /// Law type
    pub law_type: LawType,
    /// Number within that year/type
    pub number: u32,
}

impl LawNumber {
    /// Creates a new law number.
    #[must_use]
    pub fn new(era: Era, year: u32, law_type: LawType, number: u32) -> Self {
        Self {
            era,
            year,
            law_type,
            number,
        }
    }

    /// Parses a law number from string.
    ///
    /// Supports formats like:
    /// - "令和元年法律第一号"
    /// - "昭和二十二年法律第百三十一号"
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();

        // Detect era
        let era = if s.starts_with("明治") {
            Era::Meiji
        } else if s.starts_with("大正") {
            Era::Taisho
        } else if s.starts_with("昭和") {
            Era::Showa
        } else if s.starts_with("平成") {
            Era::Heisei
        } else if s.starts_with("令和") {
            Era::Reiwa
        } else {
            return None;
        };

        let rest = &s[era.japanese_name().len()..];

        // Parse year
        let year_end = rest.find('年')?;
        let year_str = &rest[..year_end];
        let year = if year_str == "元" {
            1
        } else {
            Self::parse_japanese_number(year_str)?
        };

        let rest = &rest[year_end + "年".len()..];

        // Parse law type
        let (law_type, rest) = if let Some(stripped) = rest.strip_prefix("憲法") {
            (LawType::Constitution, stripped)
        } else if let Some(stripped) = rest.strip_prefix("法律") {
            (LawType::Act, stripped)
        } else if let Some(stripped) = rest.strip_prefix("政令") {
            (LawType::CabinetOrder, stripped)
        } else if let Some(stripped) = rest.strip_prefix("省令") {
            (LawType::MinisterialOrdinance, stripped)
        } else if let Some(stripped) = rest.strip_prefix("条約") {
            (LawType::Treaty, stripped)
        } else {
            return None;
        };

        // Parse number (第X号)
        let rest = rest.trim_start_matches('第');
        let num_end = rest.find('号')?;
        let num_str = &rest[..num_end];
        let number = Self::parse_japanese_number(num_str)?;

        Some(Self::new(era, year, law_type, number))
    }

    /// Parses a Japanese numeral string to u32.
    ///
    /// Handles numbers like:
    /// - "一" = 1
    /// - "十" = 10
    /// - "二十二" = 22
    /// - "百三十一" = 131
    /// - "千二百三十四" = 1234
    fn parse_japanese_number(s: &str) -> Option<u32> {
        // Try parsing as Arabic numerals first
        if let Ok(n) = s.parse::<u32>() {
            return Some(n);
        }

        // Parse Japanese numerals
        // Algorithm: accumulate digits, apply multipliers, sum up
        let mut result = 0u32;
        let mut current = 0u32; // Current digit being built

        for c in s.chars() {
            match c {
                '一' => current = 1,
                '二' => current = 2,
                '三' => current = 3,
                '四' => current = 4,
                '五' => current = 5,
                '六' => current = 6,
                '七' => current = 7,
                '八' => current = 8,
                '九' => current = 9,
                '十' => {
                    // "十" alone = 10, "三十" = 30
                    if current == 0 {
                        current = 1;
                    }
                    result += current * 10;
                    current = 0;
                }
                '百' => {
                    // "百" alone = 100, "三百" = 300
                    if current == 0 {
                        current = 1;
                    }
                    result += current * 100;
                    current = 0;
                }
                '千' => {
                    // "千" alone = 1000, "三千" = 3000
                    if current == 0 {
                        current = 1;
                    }
                    result += current * 1000;
                    current = 0;
                }
                _ => return None,
            }
        }

        // Add any remaining digit (ones place)
        result += current;

        if result == 0 { None } else { Some(result) }
    }

    /// Formats as Japanese string.
    #[must_use]
    pub fn to_japanese_string(&self) -> String {
        let year_str = if self.year == 1 {
            "元".to_string()
        } else {
            Self::to_japanese_number(self.year)
        };

        format!(
            "{}{}年{}第{}号",
            self.era.japanese_name(),
            year_str,
            self.law_type.japanese_name(),
            Self::to_japanese_number(self.number)
        )
    }

    /// Converts a number to Japanese numerals.
    fn to_japanese_number(n: u32) -> String {
        if n == 0 {
            return "零".to_string();
        }

        let mut result = String::new();
        let mut remaining = n;

        if remaining >= 1000 {
            let thousands = remaining / 1000;
            if thousands > 1 {
                result.push(Self::digit_to_kanji(thousands));
            }
            result.push('千');
            remaining %= 1000;
        }

        if remaining >= 100 {
            let hundreds = remaining / 100;
            if hundreds > 1 {
                result.push(Self::digit_to_kanji(hundreds));
            }
            result.push('百');
            remaining %= 100;
        }

        if remaining >= 10 {
            let tens = remaining / 10;
            if tens > 1 {
                result.push(Self::digit_to_kanji(tens));
            }
            result.push('十');
            remaining %= 10;
        }

        if remaining > 0 {
            result.push(Self::digit_to_kanji(remaining));
        }

        result
    }

    fn digit_to_kanji(d: u32) -> char {
        match d {
            1 => '一',
            2 => '二',
            3 => '三',
            4 => '四',
            5 => '五',
            6 => '六',
            7 => '七',
            8 => '八',
            9 => '九',
            _ => '?',
        }
    }
}

impl std::fmt::Display for LawNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_japanese_string())
    }
}

impl JapaneseLaw {
    /// Creates a new Japanese law.
    #[must_use]
    pub fn new(law_number: LawNumber, title: &str, promulgation_date: JapaneseDate) -> Self {
        Self {
            law_type: law_number.law_type,
            law_number,
            title: title.to_string(),
            short_title: None,
            promulgation_date,
            effective_date: None,
            last_amended: None,
            ministry: None,
            is_active: true,
        }
    }

    /// Sets the short title.
    #[must_use]
    pub fn with_short_title(mut self, title: &str) -> Self {
        self.short_title = Some(title.to_string());
        self
    }

    /// Sets the effective date.
    #[must_use]
    pub fn with_effective_date(mut self, date: JapaneseDate) -> Self {
        self.effective_date = Some(date);
        self
    }

    /// Sets the ministry.
    #[must_use]
    pub fn with_ministry(mut self, ministry: &str) -> Self {
        self.ministry = Some(ministry.to_string());
        self
    }

    /// Marks the law as inactive/repealed.
    #[must_use]
    pub fn repealed(mut self) -> Self {
        self.is_active = false;
        self
    }

    /// Converts to a Legalis statute.
    #[must_use]
    pub fn to_statute(&self) -> legalis_core::Statute {
        legalis_core::Statute::new(
            format!("jp-law-{}", self.law_number),
            &self.title,
            legalis_core::Effect::new(
                legalis_core::EffectType::Grant,
                format!("{} ({})", self.title, self.law_number),
            ),
        )
        .with_jurisdiction("JP")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_law_number_parse() {
        let law_num = LawNumber::parse("令和元年法律第一号").unwrap();
        assert_eq!(law_num.era, Era::Reiwa);
        assert_eq!(law_num.year, 1);
        assert_eq!(law_num.law_type, LawType::Act);
        assert_eq!(law_num.number, 1);

        let law_num = LawNumber::parse("昭和二十二年法律第百三十一号").unwrap();
        assert_eq!(law_num.era, Era::Showa);
        assert_eq!(law_num.year, 22);
        assert_eq!(law_num.number, 131);
    }

    #[test]
    fn test_law_number_format() {
        let law_num = LawNumber::new(Era::Reiwa, 1, LawType::Act, 1);
        assert_eq!(law_num.to_japanese_string(), "令和元年法律第一号");

        let law_num = LawNumber::new(Era::Showa, 22, LawType::Act, 131);
        assert_eq!(law_num.to_japanese_string(), "昭和二十二年法律第百三十一号");
    }

    #[test]
    fn test_japanese_number_parsing() {
        assert_eq!(LawNumber::parse_japanese_number("一"), Some(1));
        assert_eq!(LawNumber::parse_japanese_number("十"), Some(10));
        assert_eq!(LawNumber::parse_japanese_number("二十二"), Some(22));
        assert_eq!(LawNumber::parse_japanese_number("百三十一"), Some(131));
        assert_eq!(LawNumber::parse_japanese_number("千二百三十四"), Some(1234));
    }

    #[test]
    fn test_law_type_hierarchy() {
        assert!(LawType::Constitution.hierarchy_level() < LawType::Act.hierarchy_level());
        assert!(LawType::Act.hierarchy_level() < LawType::CabinetOrder.hierarchy_level());
        assert!(
            LawType::CabinetOrder.hierarchy_level() < LawType::LocalOrdinance.hierarchy_level()
        );
    }

    #[test]
    fn test_japanese_law() {
        let law_num = LawNumber::new(Era::Reiwa, 5, LawType::Act, 42);
        let promulgation = JapaneseDate::new(Era::Reiwa, 5, 6, 14).unwrap();

        let law = JapaneseLaw::new(law_num, "テスト法", promulgation)
            .with_short_title("テスト")
            .with_ministry("法務省");

        assert_eq!(law.title, "テスト法");
        assert_eq!(law.short_title, Some("テスト".to_string()));
        assert_eq!(law.ministry, Some("法務省".to_string()));
        assert!(law.is_active);
    }
}
