//! Case Citation Formatting (判例引用形式)
//!
//! This module provides citation formatting for Japanese court decisions.

use super::error::{CaseLawError, Result};
use super::types::{CourtDecision, CourtLevel};
use chrono::Datelike;

/// Citation format style (引用形式スタイル)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationStyle {
    /// Short citation with case number only (短縮引用)
    Short,
    /// Standard Japanese legal citation (標準引用)
    Standard,
    /// Full citation with all details (完全引用)
    Full,
    /// Blue Book style (American legal citation)
    BlueBook,
}

/// Citation formatter (引用形式フォーマッター)
pub struct CitationFormatter;

impl CitationFormatter {
    /// Formats a court decision citation
    pub fn format(decision: &CourtDecision, style: CitationStyle) -> Result<String> {
        match style {
            CitationStyle::Short => Self::format_short(decision),
            CitationStyle::Standard => Self::format_standard(decision),
            CitationStyle::Full => Self::format_full(decision),
            CitationStyle::BlueBook => Self::format_bluebook(decision),
        }
    }

    /// Short citation: Case number only
    /// Example: "最判令和2年1月10日"
    fn format_short(decision: &CourtDecision) -> Result<String> {
        let date = decision.metadata.decision_date;
        let court_abbrev = Self::court_abbreviation(decision.metadata.court.level);

        Ok(format!(
            "{}{}年{}月{}日",
            court_abbrev,
            Self::japanese_year(date.year()),
            date.month(),
            date.day()
        ))
    }

    /// Standard Japanese legal citation
    /// Example: "最高裁判所令和2年1月10日判決 令和元年(受)第1234号"
    fn format_standard(decision: &CourtDecision) -> Result<String> {
        let date = decision.metadata.decision_date;
        let court_name = decision.metadata.court.level.japanese_name();

        Ok(format!(
            "{}{}年{}月{}日判決 {}",
            court_name,
            Self::japanese_year(date.year()),
            date.month(),
            date.day(),
            decision.metadata.case_number
        ))
    }

    /// Full citation with URL
    /// Example: "最高裁判所令和2年1月10日判決 令和元年(受)第1234号 https://courts.go.jp/..."
    fn format_full(decision: &CourtDecision) -> Result<String> {
        let mut citation = Self::format_standard(decision)?;

        if let Some(url) = &decision.source_url {
            citation.push_str(&format!(" {}", url));
        }

        Ok(citation)
    }

    /// Blue Book style citation (American legal format)
    /// Example: "[Case Name], Supreme Court of Japan, Jan. 10, 2020, Reiwa 1 (Ju) No. 1234"
    fn format_bluebook(decision: &CourtDecision) -> Result<String> {
        let date = decision.metadata.decision_date;
        let court_name = decision.metadata.court.level.english_name();

        let month_name = match date.month() {
            1 => "Jan.",
            2 => "Feb.",
            3 => "Mar.",
            4 => "Apr.",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "Aug.",
            9 => "Sept.",
            10 => "Oct.",
            11 => "Nov.",
            12 => "Dec.",
            _ => "Unknown",
        };

        Ok(format!(
            "{}, {}, {} {}, {}",
            decision.summary.lines().next().unwrap_or("Untitled"),
            court_name,
            month_name,
            date.day(),
            date.year()
        ))
    }

    /// Returns court abbreviation for short citations
    fn court_abbreviation(level: CourtLevel) -> &'static str {
        match level {
            CourtLevel::Supreme => "最判",  // 最高裁判決
            CourtLevel::High => "高判",     // 高等裁判決
            CourtLevel::District => "地判", // 地方裁判決
            CourtLevel::Family => "家判",   // 家庭裁判決
            CourtLevel::Summary => "簡判",  // 簡易裁判決
        }
    }

    /// Converts Western year to Japanese era year format
    /// This is a simplified version - full implementation would need era handling
    fn japanese_year(year: i32) -> String {
        if year >= 2019 {
            format!("令和{}", year - 2018)
        } else if year >= 1989 {
            format!("平成{}", year - 1988)
        } else if year >= 1926 {
            format!("昭和{}", year - 1925)
        } else {
            format!("{}", year)
        }
    }
}

/// Parses a Japanese case number
/// Example: "令和2年(受)第1234号" -> ("令和2年", "受", "1234")
pub fn parse_case_number(case_number: &str) -> Result<(String, String, String)> {
    // This is a simplified parser
    // Real implementation would handle various formats

    if !case_number.contains("年") || !case_number.contains("号") {
        return Err(CaseLawError::InvalidCaseNumber {
            case_number: case_number.to_string(),
        });
    }

    // Extract components using simple string manipulation
    let parts: Vec<&str> = case_number.split(['(', ')']).collect();

    if parts.len() < 3 {
        return Err(CaseLawError::InvalidCaseNumber {
            case_number: case_number.to_string(),
        });
    }

    let era_year = parts[0].replace("年", "").trim().to_string();
    let case_type = parts[1].trim().to_string();
    let number = parts[2]
        .replace("第", "")
        .replace("号", "")
        .trim()
        .to_string();

    Ok((era_year, case_type, number))
}

/// Generates a citation link (if URL is available)
pub fn citation_link(decision: &CourtDecision) -> Option<String> {
    decision.source_url.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::case_law::types::{CaseMetadata, CaseOutcome, Court, CourtDecision, LegalArea};
    use chrono::Utc;

    #[test]
    fn test_format_short() {
        let metadata = CaseMetadata::new(
            "令和2年(受)第1234号",
            Utc::now(),
            Court::new(CourtLevel::Supreme),
            LegalArea::Civil,
            CaseOutcome::PlaintiffWins,
        );

        let decision = CourtDecision::new("test-001", metadata, "Test summary");

        let citation = CitationFormatter::format(&decision, CitationStyle::Short).unwrap();
        assert!(citation.contains("最判"));
    }

    #[test]
    fn test_format_standard() {
        let metadata = CaseMetadata::new(
            "令和2年(受)第1234号",
            Utc::now(),
            Court::new(CourtLevel::Supreme),
            LegalArea::Civil,
            CaseOutcome::PlaintiffWins,
        );

        let decision = CourtDecision::new("test-002", metadata, "Test summary");

        let citation = CitationFormatter::format(&decision, CitationStyle::Standard).unwrap();
        assert!(citation.contains("最高裁判所"));
        assert!(citation.contains("令和2年(受)第1234号"));
    }

    #[test]
    fn test_parse_case_number() {
        let result = parse_case_number("令和2年(受)第1234号");
        assert!(result.is_ok());

        let (_era, case_type, _number) = result.unwrap();
        assert_eq!(case_type, "受");
    }

    #[test]
    fn test_parse_invalid_case_number() {
        let result = parse_case_number("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_court_abbreviation() {
        assert_eq!(
            CitationFormatter::court_abbreviation(CourtLevel::Supreme),
            "最判"
        );
        assert_eq!(
            CitationFormatter::court_abbreviation(CourtLevel::High),
            "高判"
        );
        assert_eq!(
            CitationFormatter::court_abbreviation(CourtLevel::District),
            "地判"
        );
    }
}
