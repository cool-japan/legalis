//! Russian legal citation formatting system.
//!
//! Supports citation of:
//! - Federal Laws (Федеральные законы)
//! - Codes (Кодексы)
//! - Government Decrees (Постановления Правительства)
//! - Presidential Decrees (Указы Президента)
//! - Court Decisions (Судебные решения)

use serde::{Deserialize, Serialize};

/// Type of legal document in Russian legal system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// Federal Law (Федеральный закон)
    FederalLaw,
    /// Code (Кодекс)
    Code,
    /// Presidential Decree (Указ Президента)
    PresidentialDecree,
    /// Government Decree (Постановление Правительства)
    GovernmentDecree,
    /// Constitutional Court Decision (Постановление Конституционного Суда)
    ConstitutionalCourtDecision,
    /// Supreme Court Decision (Постановление Верховного Суда)
    SupremeCourtDecision,
    /// Arbitration Court Decision (Постановление Арбитражного Суда)
    ArbitrationCourtDecision,
}

/// Russian legal document citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalDocument {
    /// Type of document
    pub document_type: DocumentType,
    /// Document number (e.g., "152-FZ")
    pub number: String,
    /// Date of adoption
    pub date: chrono::NaiveDate,
    /// Title in Russian
    pub title_ru: String,
    /// Title in English
    pub title_en: Option<String>,
    /// Article or section reference
    pub article: Option<String>,
    /// Paragraph reference
    pub paragraph: Option<String>,
    /// Subparagraph reference
    pub subparagraph: Option<String>,
}

impl LegalDocument {
    /// Creates a new Federal Law document
    pub fn federal_law(
        number: impl Into<String>,
        date: chrono::NaiveDate,
        title_ru: impl Into<String>,
    ) -> Self {
        Self {
            document_type: DocumentType::FederalLaw,
            number: number.into(),
            date,
            title_ru: title_ru.into(),
            title_en: None,
            article: None,
            paragraph: None,
            subparagraph: None,
        }
    }

    /// Creates a new Code document
    pub fn code(
        name: impl Into<String>,
        date: chrono::NaiveDate,
        title_ru: impl Into<String>,
    ) -> Self {
        Self {
            document_type: DocumentType::Code,
            number: name.into(),
            date,
            title_ru: title_ru.into(),
            title_en: None,
            article: None,
            paragraph: None,
            subparagraph: None,
        }
    }

    /// Sets the English title
    pub fn with_title_en(mut self, title_en: impl Into<String>) -> Self {
        self.title_en = Some(title_en.into());
        self
    }

    /// Sets the article reference
    pub fn with_article(mut self, article: impl Into<String>) -> Self {
        self.article = Some(article.into());
        self
    }

    /// Sets the paragraph reference
    pub fn with_paragraph(mut self, paragraph: impl Into<String>) -> Self {
        self.paragraph = Some(paragraph.into());
        self
    }

    /// Sets the subparagraph reference
    pub fn with_subparagraph(mut self, subparagraph: impl Into<String>) -> Self {
        self.subparagraph = Some(subparagraph.into());
        self
    }
}

/// Citation style for Russian legal documents
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationStyle {
    /// Full citation with complete title
    Full,
    /// Short citation with abbreviated reference
    Short,
    /// Official gazette style
    Official,
}

/// Russian legal citation formatter
pub struct CitationFormatter;

impl CitationFormatter {
    /// Formats a legal document according to the specified style
    pub fn format(document: &LegalDocument, style: CitationStyle) -> String {
        match style {
            CitationStyle::Full => Self::format_full(document),
            CitationStyle::Short => Self::format_short(document),
            CitationStyle::Official => Self::format_official(document),
        }
    }

    fn format_full(doc: &LegalDocument) -> String {
        let mut result = String::new();

        // Document type and number
        result.push_str(&Self::format_document_type(&doc.document_type));
        result.push(' ');
        result.push_str(&doc.number);

        // Date
        result.push_str(&format!(" от {} г.", doc.date.format("%d.%m.%Y")));

        // Title
        result.push_str(&format!(" \"{}\"", doc.title_ru));

        // Article reference
        if let Some(ref article) = doc.article {
            result.push_str(&format!(" ст. {}", article));
        }

        // Paragraph reference
        if let Some(ref paragraph) = doc.paragraph {
            result.push_str(&format!(", п. {}", paragraph));
        }

        // Subparagraph reference
        if let Some(ref subparagraph) = doc.subparagraph {
            result.push_str(&format!(", пп. {}", subparagraph));
        }

        result
    }

    fn format_short(doc: &LegalDocument) -> String {
        let mut result = String::new();

        // Short document type
        result.push_str(&Self::format_short_type(&doc.document_type));
        result.push(' ');
        result.push_str(&doc.number);

        // Article reference if present
        if let Some(ref article) = doc.article {
            result.push_str(&format!(", ст. {}", article));
        }

        result
    }

    fn format_official(doc: &LegalDocument) -> String {
        let mut result = String::new();

        // Official document reference
        result.push_str(&Self::format_document_type(&doc.document_type));
        result.push(' ');
        result.push_str(&doc.number);
        result.push_str(&format!(" от {} года", doc.date.format("%d.%m.%Y")));

        // Article and paragraph references
        if let Some(ref article) = doc.article {
            result.push_str(&format!(", статья {}", article));

            if let Some(ref paragraph) = doc.paragraph {
                result.push_str(&format!(", пункт {}", paragraph));

                if let Some(ref subparagraph) = doc.subparagraph {
                    result.push_str(&format!(", подпункт {}", subparagraph));
                }
            }
        }

        result
    }

    fn format_document_type(doc_type: &DocumentType) -> String {
        match doc_type {
            DocumentType::FederalLaw => "Федеральный закон".to_string(),
            DocumentType::Code => "Кодекс".to_string(),
            DocumentType::PresidentialDecree => "Указ Президента РФ".to_string(),
            DocumentType::GovernmentDecree => "Постановление Правительства РФ".to_string(),
            DocumentType::ConstitutionalCourtDecision => {
                "Постановление Конституционного Суда РФ".to_string()
            }
            DocumentType::SupremeCourtDecision => "Постановление Верховного Суда РФ".to_string(),
            DocumentType::ArbitrationCourtDecision => {
                "Постановление Арбитражного Суда РФ".to_string()
            }
        }
    }

    fn format_short_type(doc_type: &DocumentType) -> String {
        match doc_type {
            DocumentType::FederalLaw => "ФЗ".to_string(),
            DocumentType::Code => "Кодекс".to_string(),
            DocumentType::PresidentialDecree => "Указ Президента".to_string(),
            DocumentType::GovernmentDecree => "Пост. Правительства".to_string(),
            DocumentType::ConstitutionalCourtDecision => "Пост. КС РФ".to_string(),
            DocumentType::SupremeCourtDecision => "Пост. ВС РФ".to_string(),
            DocumentType::ArbitrationCourtDecision => "Пост. АС РФ".to_string(),
        }
    }
}

/// Type alias for Citation
pub type Citation = LegalDocument;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federal_law_citation() {
        let doc = LegalDocument::federal_law(
            "152-FZ",
            chrono::NaiveDate::from_ymd_opt(2006, 7, 27).expect("Valid date"),
            "О персональных данных",
        )
        .with_article("3");

        let full = CitationFormatter::format(&doc, CitationStyle::Full);
        assert!(full.contains("Федеральный закон 152-FZ"));
        assert!(full.contains("ст. 3"));

        let short = CitationFormatter::format(&doc, CitationStyle::Short);
        assert!(short.contains("ФЗ 152-FZ"));
    }

    #[test]
    fn test_code_citation() {
        let doc = LegalDocument::code(
            "ГК РФ",
            chrono::NaiveDate::from_ymd_opt(1994, 11, 30).expect("Valid date"),
            "Гражданский кодекс Российской Федерации",
        )
        .with_article("128")
        .with_paragraph("1");

        let full = CitationFormatter::format(&doc, CitationStyle::Full);
        assert!(full.contains("Кодекс ГК РФ"));
        assert!(full.contains("ст. 128"));
        assert!(full.contains("п. 1"));
    }
}
