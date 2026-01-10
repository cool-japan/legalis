//! Constitutional types (Types constitutionnels)
//!
//! Core data structures for the French Constitution of 1958.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Constitutional title (Titre de la Constitution)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstitutionTitle {
    /// Title number (1-16)
    pub number: u8,

    /// French title
    pub title_fr: String,

    /// English translation
    pub title_en: String,

    /// Article range (start, end inclusive)
    pub articles: (u8, u8),

    /// Description in French
    pub description_fr: String,

    /// Description in English
    pub description_en: String,
}

impl ConstitutionTitle {
    /// Create new constitution title
    #[must_use]
    pub fn new(
        number: u8,
        title_fr: impl Into<String>,
        title_en: impl Into<String>,
        articles: (u8, u8),
    ) -> Self {
        Self {
            number,
            title_fr: title_fr.into(),
            title_en: title_en.into(),
            articles,
            description_fr: String::new(),
            description_en: String::new(),
        }
    }

    /// Add French description
    #[must_use]
    pub fn with_description_fr(mut self, desc: impl Into<String>) -> Self {
        self.description_fr = desc.into();
        self
    }

    /// Add English description
    #[must_use]
    pub fn with_description_en(mut self, desc: impl Into<String>) -> Self {
        self.description_en = desc.into();
        self
    }

    /// Get article count
    #[must_use]
    pub fn article_count(&self) -> u8 {
        self.articles.1 - self.articles.0 + 1
    }
}

/// Constitutional article (Article de la Constitution)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConstitutionArticle {
    /// Article number
    pub number: u8,

    /// Title number this article belongs to
    pub title: u8,

    /// French text of the article
    pub text_fr: String,

    /// English translation
    pub text_en: String,

    /// Summary in French
    pub summary_fr: String,

    /// Summary in English
    pub summary_en: String,

    /// Key concepts/tags
    pub tags: Vec<String>,
}

impl ConstitutionArticle {
    /// Create new constitutional article
    #[must_use]
    pub fn new(
        number: u8,
        title: u8,
        text_fr: impl Into<String>,
        text_en: impl Into<String>,
    ) -> Self {
        Self {
            number,
            title,
            text_fr: text_fr.into(),
            text_en: text_en.into(),
            summary_fr: String::new(),
            summary_en: String::new(),
            tags: Vec::new(),
        }
    }

    /// Add French summary
    #[must_use]
    pub fn with_summary_fr(mut self, summary: impl Into<String>) -> Self {
        self.summary_fr = summary.into();
        self
    }

    /// Add English summary
    #[must_use]
    pub fn with_summary_en(mut self, summary: impl Into<String>) -> Self {
        self.summary_en = summary.into();
        self
    }

    /// Add tag
    #[must_use]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Add multiple tags
    #[must_use]
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }
}

/// Fundamental right category (Catégorie de droits fondamentaux)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FundamentalRight {
    /// Equality (Égalité)
    Equality,

    /// Liberty (Liberté)
    Liberty,

    /// Fraternity (Fraternité)
    Fraternity,

    /// Secularism (Laïcité)
    Secularism,

    /// Democracy (Démocratie)
    Democracy,

    /// Social rights (Droits sociaux)
    SocialRights,

    /// Economic rights (Droits économiques)
    EconomicRights,
}

impl FundamentalRight {
    /// Get French name
    #[must_use]
    pub fn french_name(self) -> &'static str {
        match self {
            Self::Equality => "Égalité",
            Self::Liberty => "Liberté",
            Self::Fraternity => "Fraternité",
            Self::Secularism => "Laïcité",
            Self::Democracy => "Démocratie",
            Self::SocialRights => "Droits sociaux",
            Self::EconomicRights => "Droits économiques",
        }
    }

    /// Get English name
    #[must_use]
    pub fn english_name(self) -> &'static str {
        match self {
            Self::Equality => "Equality",
            Self::Liberty => "Liberty",
            Self::Fraternity => "Fraternity",
            Self::Secularism => "Secularism",
            Self::Democracy => "Democracy",
            Self::SocialRights => "Social rights",
            Self::EconomicRights => "Economic rights",
        }
    }
}

/// Government institution (Institution gouvernementale)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Institution {
    /// President of the Republic (Président de la République)
    President,

    /// Prime Minister (Premier ministre)
    PrimeMinister,

    /// Government (Gouvernement)
    Government,

    /// National Assembly (Assemblée nationale)
    NationalAssembly,

    /// Senate (Sénat)
    Senate,

    /// Constitutional Council (Conseil constitutionnel)
    ConstitutionalCouncil,

    /// State Council (Conseil d'État)
    StateCouncil,

    /// Court of Cassation (Cour de cassation)
    CourtOfCassation,
}

impl Institution {
    /// Get French name
    #[must_use]
    pub fn french_name(self) -> &'static str {
        match self {
            Self::President => "Président de la République",
            Self::PrimeMinister => "Premier ministre",
            Self::Government => "Gouvernement",
            Self::NationalAssembly => "Assemblée nationale",
            Self::Senate => "Sénat",
            Self::ConstitutionalCouncil => "Conseil constitutionnel",
            Self::StateCouncil => "Conseil d'État",
            Self::CourtOfCassation => "Cour de cassation",
        }
    }

    /// Get English name
    #[must_use]
    pub fn english_name(self) -> &'static str {
        match self {
            Self::President => "President of the Republic",
            Self::PrimeMinister => "Prime Minister",
            Self::Government => "Government",
            Self::NationalAssembly => "National Assembly",
            Self::Senate => "Senate",
            Self::ConstitutionalCouncil => "Constitutional Council",
            Self::StateCouncil => "State Council",
            Self::CourtOfCassation => "Court of Cassation",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constitution_title_creation() {
        let title = ConstitutionTitle::new(1, "De la souveraineté", "On Sovereignty", (1, 4))
            .with_description_fr("Principes fondamentaux de la République")
            .with_description_en("Fundamental principles of the Republic");

        assert_eq!(title.number, 1);
        assert_eq!(title.articles, (1, 4));
        assert_eq!(title.article_count(), 4);
    }

    #[test]
    fn test_constitution_article_creation() {
        let article = ConstitutionArticle::new(
            1,
            1,
            "La France est une République...",
            "France is a Republic...",
        )
        .with_summary_fr("Définition de la République")
        .with_summary_en("Definition of the Republic")
        .with_tag("république")
        .with_tag("laïcité");

        assert_eq!(article.number, 1);
        assert_eq!(article.title, 1);
        assert_eq!(article.tags.len(), 2);
    }

    #[test]
    fn test_fundamental_rights() {
        assert_eq!(FundamentalRight::Equality.french_name(), "Égalité");
        assert_eq!(FundamentalRight::Equality.english_name(), "Equality");
        assert_eq!(FundamentalRight::Secularism.french_name(), "Laïcité");
    }

    #[test]
    fn test_institutions() {
        assert_eq!(
            Institution::President.french_name(),
            "Président de la République"
        );
        assert_eq!(
            Institution::President.english_name(),
            "President of the Republic"
        );
        assert_eq!(Institution::Senate.french_name(), "Sénat");
    }
}
