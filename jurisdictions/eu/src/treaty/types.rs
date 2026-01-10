//! Core types for Treaty Framework

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Type of EU treaty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TreatyType {
    /// Treaty on European Union (TEU)
    TEU,

    /// Treaty on the Functioning of the European Union (TFEU)
    TFEU,

    /// Charter of Fundamental Rights
    Charter,
}

/// Treaty article reference
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TreatyArticle {
    /// Treaty type
    pub treaty: TreatyType,

    /// Article number
    pub article: u32,

    /// Paragraph (optional)
    pub paragraph: Option<u32>,
}

impl TreatyArticle {
    /// Create new treaty article reference
    pub fn new(treaty: TreatyType, article: u32) -> Self {
        Self {
            treaty,
            article,
            paragraph: None,
        }
    }

    /// Add paragraph number
    pub fn with_paragraph(mut self, paragraph: u32) -> Self {
        self.paragraph = Some(paragraph);
        self
    }

    /// Format citation
    pub fn format(&self) -> String {
        let treaty_str = match self.treaty {
            TreatyType::TEU => "TEU",
            TreatyType::TFEU => "TFEU",
            TreatyType::Charter => "Charter",
        };

        if let Some(para) = self.paragraph {
            format!("Article {}({}) {}", self.article, para, treaty_str)
        } else {
            format!("Article {} {}", self.article, treaty_str)
        }
    }
}

/// Treaty provision (skeleton)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TreatyProvision {
    /// Article reference
    pub article: TreatyArticle,

    /// Brief description
    pub description: String,
}

impl TreatyProvision {
    pub fn new(article: TreatyArticle, description: impl Into<String>) -> Self {
        Self {
            article,
            description: description.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treaty_article_formatting() {
        let art34 = TreatyArticle::new(TreatyType::TFEU, 34);
        assert_eq!(art34.format(), "Article 34 TFEU");

        let art6_1 = TreatyArticle::new(TreatyType::TEU, 6).with_paragraph(1);
        assert_eq!(art6_1.format(), "Article 6(1) TEU");
    }

    #[test]
    fn test_charter_article() {
        let art8 = TreatyArticle::new(TreatyType::Charter, 8);
        assert_eq!(art8.format(), "Article 8 Charter");
    }
}
