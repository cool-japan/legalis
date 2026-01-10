//! Core types for Competition Law implementation

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// EU Member States (EU27 + EEA)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MemberState {
    // EU27
    Austria,
    Belgium,
    Bulgaria,
    Croatia,
    Cyprus,
    CzechRepublic,
    Denmark,
    Estonia,
    Finland,
    France,
    Germany,
    Greece,
    Hungary,
    Ireland,
    Italy,
    Latvia,
    Lithuania,
    Luxembourg,
    Malta,
    Netherlands,
    Poland,
    Portugal,
    Romania,
    Slovakia,
    Slovenia,
    Spain,
    Sweden,

    // EEA (non-EU)
    Iceland,
    Liechtenstein,
    Norway,
}

/// Geographic scope of relevant market
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GeographicMarket {
    /// Single Member State
    NationalMarket(MemberState),

    /// Multiple Member States
    RegionalMarket(Vec<MemberState>),

    /// Entire EU/EEA
    EuWide,

    /// Global market (for worldwide products/services)
    Global,
}

/// Relevant market definition (product market + geographic market)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelevantMarket {
    /// Product or service market description
    pub product_market: String,

    /// Geographic scope
    pub geographic_market: GeographicMarket,

    /// Market share of undertaking(s) (0.0 to 1.0)
    pub market_share: f64,
}

impl RelevantMarket {
    /// Check if market share indicates dominance (>40% typically)
    pub fn indicates_dominance(&self) -> bool {
        self.market_share > 0.40
    }

    /// Check if market share is very dominant (>50%)
    pub fn is_very_dominant(&self) -> bool {
        self.market_share > 0.50
    }
}

/// Undertaking (enterprise/company in competition law context)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Undertaking {
    pub name: String,
    pub market_share: Option<f64>,
}

impl Undertaking {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            market_share: None,
        }
    }

    pub fn with_market_share(mut self, share: f64) -> Self {
        self.market_share = Some(share);
        self
    }
}

/// Types of abusive conduct under Article 102
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AbuseType {
    /// Exploitative abuse (harming consumers directly)
    Exploitative(ExploitativeAbuse),

    /// Exclusionary abuse (harming competitors/competition structure)
    Exclusionary(ExclusionaryAbuse),
}

/// Exploitative abuse types (Article 102(a)-(b))
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExploitativeAbuse {
    /// Unfair purchase or selling prices
    UnfairPricing {
        price: f64,
        competitive_price: f64,
        excessive_percentage: f64,
    },

    /// Limiting production, markets, or technical development to consumer prejudice
    LimitingProduction { description: String },
}

/// Exclusionary abuse types (Article 102(c)-(d))
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExclusionaryAbuse {
    /// Predatory pricing (below-cost pricing to eliminate competitors)
    PredatoryPricing {
        price: f64,
        average_variable_cost: f64,
    },

    /// Refusal to deal/supply
    RefusalToDeal {
        customer: String,
        essential_facility: bool,
    },

    /// Tying and bundling
    Tying {
        tying_product: String,
        tied_product: String,
    },

    /// Exclusive dealing
    ExclusiveDealing {
        duration_months: u32,
        market_foreclosure_percentage: f64,
    },

    /// Margin squeeze
    MarginSqueeze {
        wholesale_price: f64,
        retail_price: f64,
        downstream_competitor_costs: f64,
    },

    /// Discriminatory treatment (applying dissimilar conditions to equivalent transactions)
    Discrimination { description: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relevant_market_dominance() {
        let market = RelevantMarket {
            product_market: "Smartphones".to_string(),
            geographic_market: GeographicMarket::EuWide,
            market_share: 0.55,
        };

        assert!(market.indicates_dominance());
        assert!(market.is_very_dominant());
    }

    #[test]
    fn test_relevant_market_no_dominance() {
        let market = RelevantMarket {
            product_market: "Laptops".to_string(),
            geographic_market: GeographicMarket::NationalMarket(MemberState::Germany),
            market_share: 0.25,
        };

        assert!(!market.indicates_dominance());
        assert!(!market.is_very_dominant());
    }

    #[test]
    fn test_undertaking_builder() {
        let undertaking = Undertaking::new("Acme Corp").with_market_share(0.45);

        assert_eq!(undertaking.name, "Acme Corp");
        assert_eq!(undertaking.market_share, Some(0.45));
    }
}
