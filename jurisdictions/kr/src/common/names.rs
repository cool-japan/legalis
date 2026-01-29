//! Korean Name Utilities
//!
//! Handles Korean personal and company names
//!
//! # 이름 유틸리티 / Name Utilities

use serde::{Deserialize, Serialize};
use std::fmt;

/// Korean personal name
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KoreanName {
    /// Family name (성)
    pub family_name: String,
    /// Given name (이름)
    pub given_name: String,
    /// Full name in English romanization (optional)
    pub romanized: Option<String>,
}

impl KoreanName {
    /// Create new Korean name
    pub fn new(family_name: impl Into<String>, given_name: impl Into<String>) -> Self {
        Self {
            family_name: family_name.into(),
            given_name: given_name.into(),
            romanized: None,
        }
    }

    /// Create with romanization
    pub fn with_romanization(
        family_name: impl Into<String>,
        given_name: impl Into<String>,
        romanized: impl Into<String>,
    ) -> Self {
        Self {
            family_name: family_name.into(),
            given_name: given_name.into(),
            romanized: Some(romanized.into()),
        }
    }

    /// Get full name in Korean (family name + given name)
    pub fn full_name_korean(&self) -> String {
        format!("{}{}", self.family_name, self.given_name)
    }

    /// Get full name with romanization if available
    pub fn full_name_with_romanization(&self) -> String {
        if let Some(ref roman) = self.romanized {
            format!("{} ({})", self.full_name_korean(), roman)
        } else {
            self.full_name_korean()
        }
    }
}

impl fmt::Display for KoreanName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name_korean())
    }
}

/// Company organization form
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganizationForm {
    /// 주식회사 / Stock Company (Jusikhoesa)
    StockCompany,
    /// 유한회사 / Limited Company (Yuhanhoesa)
    LimitedCompany,
    /// 유한책임회사 / Limited Liability Company (Yuhan Chaegimhoesa)
    Llc,
    /// 합명회사 / General Partnership Company (Hapmyeonghoesa)
    GeneralPartnership,
    /// 합자회사 / Limited Partnership Company (Hapjahoesa)
    LimitedPartnership,
}

impl OrganizationForm {
    /// Get Korean name
    pub fn name_ko(&self) -> &'static str {
        match self {
            OrganizationForm::StockCompany => "주식회사",
            OrganizationForm::LimitedCompany => "유한회사",
            OrganizationForm::Llc => "유한책임회사",
            OrganizationForm::GeneralPartnership => "합명회사",
            OrganizationForm::LimitedPartnership => "합자회사",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            OrganizationForm::StockCompany => "Stock Company",
            OrganizationForm::LimitedCompany => "Limited Company",
            OrganizationForm::Llc => "Limited Liability Company",
            OrganizationForm::GeneralPartnership => "General Partnership Company",
            OrganizationForm::LimitedPartnership => "Limited Partnership Company",
        }
    }

    /// Get abbreviated form in Korean
    pub fn abbreviation_ko(&self) -> &'static str {
        match self {
            OrganizationForm::StockCompany => "(주)",
            OrganizationForm::LimitedCompany => "(유)",
            OrganizationForm::Llc => "(유한)",
            OrganizationForm::GeneralPartnership => "(합명)",
            OrganizationForm::LimitedPartnership => "(합자)",
        }
    }

    /// Get abbreviated form in English
    pub fn abbreviation_en(&self) -> &'static str {
        match self {
            OrganizationForm::StockCompany => "Co., Ltd.",
            OrganizationForm::LimitedCompany => "Ltd.",
            OrganizationForm::Llc => "LLC",
            OrganizationForm::GeneralPartnership => "GP",
            OrganizationForm::LimitedPartnership => "LP",
        }
    }
}

/// Korean company name
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanyName {
    /// Trade name (상호)
    pub trade_name: String,
    /// Organization form
    pub organization_form: OrganizationForm,
    /// English name (optional)
    pub english_name: Option<String>,
}

impl CompanyName {
    /// Create new company name
    pub fn new(trade_name: impl Into<String>, organization_form: OrganizationForm) -> Self {
        Self {
            trade_name: trade_name.into(),
            organization_form,
            english_name: None,
        }
    }

    /// Create with English name
    pub fn with_english(
        trade_name: impl Into<String>,
        organization_form: OrganizationForm,
        english_name: impl Into<String>,
    ) -> Self {
        Self {
            trade_name: trade_name.into(),
            organization_form,
            english_name: Some(english_name.into()),
        }
    }

    /// Get full company name in Korean (org form + trade name)
    pub fn full_name_korean(&self) -> String {
        format!("{} {}", self.organization_form.name_ko(), self.trade_name)
    }

    /// Get full company name with abbreviation (trade name + abbr)
    pub fn name_with_abbreviation(&self) -> String {
        format!(
            "{}{}",
            self.trade_name,
            self.organization_form.abbreviation_ko()
        )
    }

    /// Get English company name if available
    pub fn full_name_english(&self) -> String {
        if let Some(ref _en) = self.english_name {
            format!("{}, {}", _en, self.organization_form.abbreviation_en())
        } else {
            self.full_name_korean()
        }
    }

    /// Get bilingual name
    pub fn full_name_bilingual(&self) -> String {
        if self.english_name.is_some() {
            format!("{} / {}", self.full_name_korean(), self.full_name_english())
        } else {
            self.full_name_korean()
        }
    }
}

impl fmt::Display for CompanyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name_korean())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_korean_name() {
        let name = KoreanName::new("김", "철수");
        assert_eq!(name.full_name_korean(), "김철수");
    }

    #[test]
    fn test_korean_name_with_romanization() {
        let name = KoreanName::with_romanization("박", "영희", "Park Young-hee");
        assert_eq!(
            name.full_name_with_romanization(),
            "박영희 (Park Young-hee)"
        );
    }

    #[test]
    fn test_organization_form() {
        let stock = OrganizationForm::StockCompany;
        assert_eq!(stock.name_ko(), "주식회사");
        assert_eq!(stock.name_en(), "Stock Company");
        assert_eq!(stock.abbreviation_ko(), "(주)");
        assert_eq!(stock.abbreviation_en(), "Co., Ltd.");
    }

    #[test]
    fn test_company_name() {
        let company = CompanyName::new("삼성전자", OrganizationForm::StockCompany);
        assert_eq!(company.full_name_korean(), "주식회사 삼성전자");
        assert_eq!(company.name_with_abbreviation(), "삼성전자(주)");
    }

    #[test]
    fn test_company_name_with_english() {
        let company = CompanyName::with_english(
            "네이버",
            OrganizationForm::StockCompany,
            "NAVER Corporation",
        );
        assert_eq!(company.full_name_english(), "NAVER Corporation, Co., Ltd.");
        assert!(company.full_name_bilingual().contains("주식회사 네이버"));
        assert!(company.full_name_bilingual().contains("NAVER Corporation"));
    }

    #[test]
    fn test_display_traits() {
        let name = KoreanName::new("이", "민호");
        assert_eq!(format!("{}", name), "이민호");

        let company = CompanyName::new("카카오", OrganizationForm::StockCompany);
        assert_eq!(format!("{}", company), "주식회사 카카오");
    }
}
