//! Charter of Fundamental Rights

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Charter of Fundamental Rights articles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CharterArticle {
    /// Article 7 - Respect for private and family life
    RespectForPrivateLife,

    /// Article 8 - Protection of personal data
    DataProtection,

    /// Article 11 - Freedom of expression and information
    FreedomOfExpression,

    /// Article 16 - Freedom to conduct a business
    FreedomToConductBusiness,

    /// Article 47 - Right to an effective remedy and fair trial
    EffectiveRemedy,
}

impl CharterArticle {
    /// Get article number
    pub fn article_number(&self) -> u32 {
        match self {
            CharterArticle::RespectForPrivateLife => 7,
            CharterArticle::DataProtection => 8,
            CharterArticle::FreedomOfExpression => 11,
            CharterArticle::FreedomToConductBusiness => 16,
            CharterArticle::EffectiveRemedy => 47,
        }
    }

    /// Get article title
    pub fn title(&self) -> &str {
        match self {
            CharterArticle::RespectForPrivateLife => "Respect for private and family life",
            CharterArticle::DataProtection => "Protection of personal data",
            CharterArticle::FreedomOfExpression => "Freedom of expression and information",
            CharterArticle::FreedomToConductBusiness => "Freedom to conduct a business",
            CharterArticle::EffectiveRemedy => "Right to an effective remedy and to a fair trial",
        }
    }

    /// Format citation
    pub fn format(&self) -> String {
        format!("Article {} Charter", self.article_number())
    }
}

/// Fundamental right (skeleton)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundamentalRight {
    /// Charter article
    pub article: CharterArticle,

    /// Description of the right
    pub description: String,

    /// Related GDPR articles (if applicable)
    pub gdpr_connection: Option<String>,
}

impl FundamentalRight {
    pub fn new(article: CharterArticle) -> Self {
        Self {
            article,
            description: article.title().to_string(),
            gdpr_connection: None,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_gdpr_connection(mut self, connection: impl Into<String>) -> Self {
        self.gdpr_connection = Some(connection.into());
        self
    }
}

/// Well-known Charter rights related to digital/business law
pub mod common_rights {
    use super::*;

    /// Article 8 Charter - Protection of personal data
    /// (Foundation for GDPR)
    pub fn data_protection() -> FundamentalRight {
        FundamentalRight::new(CharterArticle::DataProtection)
            .with_description(
                "Everyone has the right to the protection of personal data concerning them",
            )
            .with_gdpr_connection("GDPR implements this fundamental right")
    }

    /// Article 7 Charter - Respect for private life
    /// (Closely related to data protection)
    pub fn privacy() -> FundamentalRight {
        FundamentalRight::new(CharterArticle::RespectForPrivateLife)
            .with_description("Everyone has the right to respect for private and family life, home and communications")
    }

    /// Article 16 Charter - Freedom to conduct business
    /// (Must be balanced against data protection)
    pub fn business_freedom() -> FundamentalRight {
        FundamentalRight::new(CharterArticle::FreedomToConductBusiness)
            .with_description("The freedom to conduct a business in accordance with Union law")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charter_article_numbers() {
        assert_eq!(CharterArticle::DataProtection.article_number(), 8);
        assert_eq!(CharterArticle::RespectForPrivateLife.article_number(), 7);
        assert_eq!(CharterArticle::FreedomOfExpression.article_number(), 11);
    }

    #[test]
    fn test_charter_formatting() {
        let art8 = CharterArticle::DataProtection;
        assert_eq!(art8.format(), "Article 8 Charter");
    }

    #[test]
    fn test_fundamental_right_builder() {
        let right = common_rights::data_protection();
        assert_eq!(right.article, CharterArticle::DataProtection);
        assert!(right.gdpr_connection.is_some());
    }
}
