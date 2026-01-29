//! Saudi Arabian Legal Citation System
//!
//! Supports both Arabic and English citation formats for:
//! - Royal Decrees (المراسيم الملكية)
//! - Council of Ministers Resolutions (قرارات مجلس الوزراء)
//! - Ministerial Decisions (القرارات الوزارية)
//! - Circulars (التعاميم)
//!
//! ## Citation Format
//!
//! Royal Decree: `Royal Decree No. [number] dated [Hijri date]`
//! Arabic: `المرسوم الملكي رقم [number] بتاريخ [Hijri date]`

use serde::{Deserialize, Serialize};
use std::fmt;

/// Types of Saudi legal instruments
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecreeType {
    /// Royal Decree (مرسوم ملكي)
    RoyalDecree,
    /// Royal Order (أمر ملكي)
    RoyalOrder,
    /// Council of Ministers Resolution (قرار مجلس الوزراء)
    CouncilResolution,
    /// Ministerial Decision (قرار وزاري)
    MinisterialDecision,
    /// Circular (تعميم)
    Circular,
    /// Regulation (نظام)
    Regulation,
}

impl DecreeType {
    /// Get Arabic name
    pub fn name_ar(&self) -> &'static str {
        match self {
            Self::RoyalDecree => "المرسوم الملكي",
            Self::RoyalOrder => "الأمر الملكي",
            Self::CouncilResolution => "قرار مجلس الوزراء",
            Self::MinisterialDecision => "القرار الوزاري",
            Self::Circular => "التعميم",
            Self::Regulation => "النظام",
        }
    }

    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::RoyalDecree => "Royal Decree",
            Self::RoyalOrder => "Royal Order",
            Self::CouncilResolution => "Council of Ministers Resolution",
            Self::MinisterialDecision => "Ministerial Decision",
            Self::Circular => "Circular",
            Self::Regulation => "Regulation",
        }
    }
}

/// Saudi Arabian legal citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaudiCitation {
    /// Type of decree
    pub decree_type: DecreeType,
    /// Decree number (e.g., "M/3", "51")
    pub decree_number: String,
    /// Hijri date (e.g., "28/1/1437")
    pub hijri_date: String,
    /// Law title in Arabic
    pub title_ar: Option<String>,
    /// Law title in English
    pub title_en: Option<String>,
    /// Article number
    pub article: Option<u32>,
    /// Paragraph/section
    pub paragraph: Option<String>,
}

impl SaudiCitation {
    /// Create a new Royal Decree citation
    pub fn royal_decree(number: impl Into<String>, hijri_date: impl Into<String>) -> Self {
        Self {
            decree_type: DecreeType::RoyalDecree,
            decree_number: number.into(),
            hijri_date: hijri_date.into(),
            title_ar: None,
            title_en: None,
            article: None,
            paragraph: None,
        }
    }

    /// Create a new Royal Order citation
    pub fn royal_order(number: impl Into<String>, hijri_date: impl Into<String>) -> Self {
        Self {
            decree_type: DecreeType::RoyalOrder,
            decree_number: number.into(),
            hijri_date: hijri_date.into(),
            title_ar: None,
            title_en: None,
            article: None,
            paragraph: None,
        }
    }

    /// Create a Council Resolution citation
    pub fn council_resolution(number: impl Into<String>, hijri_date: impl Into<String>) -> Self {
        Self {
            decree_type: DecreeType::CouncilResolution,
            decree_number: number.into(),
            hijri_date: hijri_date.into(),
            title_ar: None,
            title_en: None,
            article: None,
            paragraph: None,
        }
    }

    /// Add Arabic title
    pub fn with_title_ar(mut self, title: impl Into<String>) -> Self {
        self.title_ar = Some(title.into());
        self
    }

    /// Add English title
    pub fn with_title_en(mut self, title: impl Into<String>) -> Self {
        self.title_en = Some(title.into());
        self
    }

    /// Add article number
    pub fn with_article(mut self, article: u32) -> Self {
        self.article = Some(article);
        self
    }

    /// Add paragraph/section
    pub fn with_paragraph(mut self, para: impl Into<String>) -> Self {
        self.paragraph = Some(para.into());
        self
    }

    /// Format as English citation
    pub fn format_en(&self) -> String {
        let mut parts = vec![
            format!("{} No. {}", self.decree_type.name_en(), self.decree_number),
            format!("dated {}", self.hijri_date),
        ];

        if let Some(ref title) = self.title_en {
            parts.insert(1, format!("({})", title));
        }

        if let Some(article) = self.article {
            parts.push(format!("Article {}", article));
        }

        if let Some(ref para) = self.paragraph {
            parts.push(format!("¶ {}", para));
        }

        parts.join(", ")
    }

    /// Format as Arabic citation
    pub fn format_ar(&self) -> String {
        let mut parts = vec![
            format!("{} رقم {}", self.decree_type.name_ar(), self.decree_number),
            format!("بتاريخ {}", self.hijri_date),
        ];

        if let Some(ref title) = self.title_ar {
            parts.insert(1, format!("({})", title));
        }

        if let Some(article) = self.article {
            parts.push(format!("المادة {}", article));
        }

        if let Some(ref para) = self.paragraph {
            parts.push(format!("الفقرة {}", para));
        }

        parts.join("، ")
    }
}

impl fmt::Display for SaudiCitation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_en())
    }
}

/// Common Saudi law citations
pub mod common_citations {
    use super::*;

    /// Basic Law of Governance (1992)
    pub fn basic_law_1992() -> SaudiCitation {
        SaudiCitation::royal_decree("A/90", "27/8/1412")
            .with_title_en("Basic Law of Governance")
            .with_title_ar("النظام الأساسي للحكم")
    }

    /// Companies Law (2015)
    pub fn companies_law_2015() -> SaudiCitation {
        SaudiCitation::royal_decree("M/3", "28/1/1437")
            .with_title_en("Companies Law")
            .with_title_ar("نظام الشركات")
    }

    /// Labor Law (2005)
    pub fn labor_law_2005() -> SaudiCitation {
        SaudiCitation::royal_decree("M/51", "23/8/1426")
            .with_title_en("Labor Law")
            .with_title_ar("نظام العمل")
    }

    /// Capital Market Law (2003)
    pub fn capital_market_law_2003() -> SaudiCitation {
        SaudiCitation::royal_decree("M/30", "2/6/1424")
            .with_title_en("Capital Market Law")
            .with_title_ar("نظام السوق المالية")
    }

    /// Personal Data Protection Law (2021)
    pub fn pdpl_2021() -> SaudiCitation {
        SaudiCitation::royal_decree("M/19", "9/2/1443")
            .with_title_en("Personal Data Protection Law")
            .with_title_ar("نظام حماية البيانات الشخصية")
    }

    /// VAT Law (2017)
    pub fn vat_law_2017() -> SaudiCitation {
        SaudiCitation::royal_decree("M/113", "2/11/1438")
            .with_title_en("Value Added Tax Law")
            .with_title_ar("نظام ضريبة القيمة المضافة")
    }

    /// Income Tax Law (2004)
    pub fn income_tax_law_2004() -> SaudiCitation {
        SaudiCitation::royal_decree("M/1", "15/1/1425")
            .with_title_en("Income Tax Law")
            .with_title_ar("نظام ضريبة الدخل")
    }

    /// Zakat Law (1951, updated)
    pub fn zakat_law() -> SaudiCitation {
        SaudiCitation::royal_decree("17/2/28/3321", "29/6/1370")
            .with_title_en("Zakat and Income Tax Collection Law")
            .with_title_ar("نظام جباية الزكاة والدخل")
    }

    /// Arbitration Law (2012)
    pub fn arbitration_law_2012() -> SaudiCitation {
        SaudiCitation::royal_decree("M/34", "24/5/1433")
            .with_title_en("Arbitration Law")
            .with_title_ar("نظام التحكيم")
    }

    /// Commercial Court Law (2020)
    pub fn commercial_court_law_2020() -> SaudiCitation {
        SaudiCitation::royal_decree("M/93", "14/7/1441")
            .with_title_en("Commercial Court Law")
            .with_title_ar("نظام المحاكم التجارية")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_royal_decree_citation() {
        let citation = SaudiCitation::royal_decree("M/3", "28/1/1437")
            .with_title_en("Companies Law")
            .with_title_ar("نظام الشركات")
            .with_article(3);

        let en = citation.format_en();
        assert!(en.contains("Royal Decree No. M/3"));
        assert!(en.contains("Companies Law"));
        assert!(en.contains("Article 3"));

        let ar = citation.format_ar();
        assert!(ar.contains("المرسوم الملكي"));
        assert!(ar.contains("نظام الشركات"));
        assert!(ar.contains("المادة 3"));
    }

    #[test]
    fn test_decree_types() {
        assert_eq!(DecreeType::RoyalDecree.name_en(), "Royal Decree");
        assert_eq!(DecreeType::RoyalDecree.name_ar(), "المرسوم الملكي");
        assert_eq!(
            DecreeType::CouncilResolution.name_en(),
            "Council of Ministers Resolution"
        );
    }

    #[test]
    fn test_common_citations() {
        let basic_law = common_citations::basic_law_1992();
        assert_eq!(basic_law.decree_number, "A/90");
        assert!(basic_law.title_en.is_some());

        let companies = common_citations::companies_law_2015();
        assert_eq!(companies.decree_number, "M/3");

        let labor = common_citations::labor_law_2005();
        assert_eq!(labor.decree_number, "M/51");

        let vat = common_citations::vat_law_2017();
        assert_eq!(vat.decree_number, "M/113");
    }

    #[test]
    fn test_citation_with_paragraph() {
        let citation = SaudiCitation::royal_decree("M/51", "23/8/1426")
            .with_article(80)
            .with_paragraph("1");

        let formatted = citation.format_en();
        assert!(formatted.contains("Article 80"));
        assert!(formatted.contains("¶ 1"));
    }

    #[test]
    fn test_display_trait() {
        let citation =
            SaudiCitation::royal_decree("M/3", "28/1/1437").with_title_en("Companies Law");

        let display = format!("{}", citation);
        assert!(display.contains("M/3"));
        assert!(display.contains("Companies Law"));
    }
}
