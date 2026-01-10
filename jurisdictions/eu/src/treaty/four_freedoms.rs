//! Four Freedoms - Internal Market Provisions (Articles 28-66 TFEU)

use super::types::TreatyArticle;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The Four Freedoms of the EU internal market
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FourFreedom {
    /// Free movement of goods (Articles 28-37 TFEU)
    Goods,

    /// Free movement of persons (Articles 45-48 TFEU)
    Persons,

    /// Freedom to provide services (Articles 56-62 TFEU)
    Services,

    /// Free movement of capital (Articles 63-66 TFEU)
    Capital,
}

impl FourFreedom {
    /// Get treaty article range for this freedom
    pub fn article_range(&self) -> &str {
        match self {
            FourFreedom::Goods => "28-37",
            FourFreedom::Persons => "45-48",
            FourFreedom::Services => "56-62",
            FourFreedom::Capital => "63-66",
        }
    }

    /// Get key prohibition article
    pub fn prohibition_article(&self) -> TreatyArticle {
        use super::types::TreatyType;
        match self {
            FourFreedom::Goods => TreatyArticle::new(TreatyType::TFEU, 34),
            FourFreedom::Persons => TreatyArticle::new(TreatyType::TFEU, 45),
            FourFreedom::Services => TreatyArticle::new(TreatyType::TFEU, 56),
            FourFreedom::Capital => TreatyArticle::new(TreatyType::TFEU, 63),
        }
    }
}

/// Type of freedom restriction
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FreedomType {
    /// Quantitative restriction (explicit limit)
    QuantitativeRestriction { measure: String },

    /// Measure having equivalent effect (MEQR)
    EquivalentEffect { measure: String },

    /// Discriminatory measure
    Discrimination { basis: String },
}

/// Restriction on freedom (skeleton)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Restriction {
    /// Which freedom is restricted
    pub freedom: FourFreedom,

    /// Type of restriction
    pub restriction_type: FreedomType,

    /// Member state imposing restriction
    pub member_state: String,
}

impl Restriction {
    pub fn new(
        freedom: FourFreedom,
        restriction_type: FreedomType,
        member_state: impl Into<String>,
    ) -> Self {
        Self {
            freedom,
            restriction_type,
            member_state: member_state.into(),
        }
    }
}

/// Justification grounds for restrictions (Article 36 TFEU pattern)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JustificationGround {
    /// Public morality
    PublicMorality,

    /// Public policy
    PublicPolicy,

    /// Public security
    PublicSecurity,

    /// Protection of health and life (humans, animals, plants)
    HealthProtection,

    /// Protection of national treasures
    NationalTreasures,

    /// Protection of industrial/commercial property
    IntellectualProperty,

    /// Environmental protection (mandatory requirement)
    EnvironmentalProtection,

    /// Consumer protection (mandatory requirement)
    ConsumerProtection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_freedoms_articles() {
        assert_eq!(FourFreedom::Goods.article_range(), "28-37");
        assert_eq!(FourFreedom::Persons.article_range(), "45-48");
        assert_eq!(FourFreedom::Services.article_range(), "56-62");
        assert_eq!(FourFreedom::Capital.article_range(), "63-66");
    }

    #[test]
    fn test_prohibition_articles() {
        let goods = FourFreedom::Goods.prohibition_article();
        assert_eq!(goods.format(), "Article 34 TFEU");

        let persons = FourFreedom::Persons.prohibition_article();
        assert_eq!(persons.format(), "Article 45 TFEU");
    }
}
