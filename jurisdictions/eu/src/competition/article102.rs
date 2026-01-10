//! Article 102 TFEU - Abuse of dominant position
//!
//! Article 102: Prohibits abuse by one or more undertakings of a dominant position
//! within the internal market or in a substantial part of it, insofar as it may
//! affect trade between Member States.
//!
//! Two elements required:
//! 1. Dominant position in relevant market
//! 2. Abuse of that position
//!
//! Dominance typically established with market share >40% (United Brands, Hoffmann-La Roche)

use super::error::CompetitionError;
use super::types::{AbuseType, ExclusionaryAbuse, ExploitativeAbuse, RelevantMarket};
use legalis_core::LegalResult;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Dominance assessment result
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DominanceAssessment {
    /// Market share of undertaking
    pub market_share: f64,

    /// Whether undertaking is dominant (typically >40%)
    pub is_dominant: bool,

    /// Barriers to entry (high barriers strengthen dominance)
    pub barriers_to_entry: Vec<String>,

    /// Assessment narrative
    pub assessment: String,
}

impl DominanceAssessment {
    /// Assess dominance based on market share
    ///
    /// Guidelines (case law):
    /// - >50%: Presumed dominant (AKZO)
    /// - 40-50%: May be dominant depending on other factors
    /// - <40%: Rarely dominant unless special circumstances
    pub fn assess(market_share: f64, barriers: Vec<String>) -> Self {
        let is_dominant = market_share > 0.40;

        let assessment = if market_share > 0.50 {
            format!(
                "Market share {:.1}% - presumed dominant position (>50%)",
                market_share * 100.0
            )
        } else if market_share > 0.40 {
            format!(
                "Market share {:.1}% - likely dominant depending on other factors",
                market_share * 100.0
            )
        } else {
            format!(
                "Market share {:.1}% - below dominance threshold",
                market_share * 100.0
            )
        };

        Self {
            market_share,
            is_dominant,
            barriers_to_entry: barriers,
            assessment,
        }
    }
}

/// Article 102 conduct validation
#[derive(Debug, Clone)]
pub struct Article102Conduct {
    pub undertaking: Option<String>,
    pub relevant_market: Option<RelevantMarket>,
    pub abuse: Option<AbuseType>,
}

impl Article102Conduct {
    pub fn new() -> Self {
        Self {
            undertaking: None,
            relevant_market: None,
            abuse: None,
        }
    }

    pub fn with_undertaking(mut self, name: impl Into<String>) -> Self {
        self.undertaking = Some(name.into());
        self
    }

    pub fn with_relevant_market(mut self, market: RelevantMarket) -> Self {
        self.relevant_market = Some(market);
        self
    }

    pub fn with_abuse(mut self, abuse: AbuseType) -> Self {
        self.abuse = Some(abuse);
        self
    }

    /// Validate Article 102 compliance
    pub fn validate(&self) -> Result<Article102Validation, CompetitionError> {
        // Check required fields
        let _undertaking = self
            .undertaking
            .as_ref()
            .ok_or_else(|| CompetitionError::missing_field("undertaking"))?;

        let market = self
            .relevant_market
            .as_ref()
            .ok_or_else(|| CompetitionError::missing_field("relevant_market"))?;

        let abuse_type = self
            .abuse
            .as_ref()
            .ok_or_else(|| CompetitionError::missing_field("abuse"))?;

        // Element 1: Dominant position
        if market.market_share <= 0.40 {
            return Err(CompetitionError::no_dominant_position(
                market.market_share * 100.0,
            ));
        }

        let dominance = DominanceAssessment::assess(market.market_share, Vec::new());

        // Element 2: Abuse of dominant position
        let abuse_established = self.assess_abuse(abuse_type, market)?;

        // Determine prohibition
        let is_abuse = matches!(abuse_established, LegalResult::Deterministic(true));
        let prohibited = if dominance.is_dominant && is_abuse {
            LegalResult::Deterministic(true)
        } else {
            LegalResult::Deterministic(false)
        };

        Ok(Article102Validation {
            dominance,
            abuse_established,
            prohibited,
        })
    }

    /// Assess if conduct constitutes abuse
    fn assess_abuse(
        &self,
        abuse_type: &AbuseType,
        market: &RelevantMarket,
    ) -> Result<LegalResult<bool>, CompetitionError> {
        match abuse_type {
            AbuseType::Exploitative(exploitative) => self.assess_exploitative(exploitative),
            AbuseType::Exclusionary(exclusionary) => self.assess_exclusionary(exclusionary, market),
        }
    }

    /// Assess exploitative abuse (Article 102(a)-(b))
    fn assess_exploitative(
        &self,
        abuse: &ExploitativeAbuse,
    ) -> Result<LegalResult<bool>, CompetitionError> {
        match abuse {
            ExploitativeAbuse::UnfairPricing {
                price: _,
                competitive_price: _,
                excessive_percentage,
            } => {
                // United Brands test: Price is excessive if significantly above competitive level
                if *excessive_percentage > 0.20 {
                    // >20% above competitive price
                    Ok(LegalResult::Deterministic(true))
                } else {
                    Ok(LegalResult::JudicialDiscretion {
                        issue: format!(
                            "Pricing {:.1}% above competitive level - requires assessment of cost structure and economic value",
                            excessive_percentage * 100.0
                        ),
                        context_id: uuid::Uuid::new_v4(),
                        narrative_hint: Some(
                            "United Brands test: Price is excessive if it has no reasonable relation \
                             to economic value. Requires comparison with competitive prices, costs, \
                             and profit margins.".to_string()
                        ),
                    })
                }
            }
            ExploitativeAbuse::LimitingProduction { .. } => {
                // Limiting production to consumer prejudice (Article 102(b))
                Ok(LegalResult::Deterministic(true))
            }
        }
    }

    /// Assess exclusionary abuse (Article 102(c)-(d))
    fn assess_exclusionary(
        &self,
        abuse: &ExclusionaryAbuse,
        _market: &RelevantMarket,
    ) -> Result<LegalResult<bool>, CompetitionError> {
        match abuse {
            ExclusionaryAbuse::PredatoryPricing {
                price,
                average_variable_cost,
            } => {
                // AKZO test:
                // - Below AVC: Presumed predatory
                // - Between AVC and ATC: Predatory if intent to eliminate competitor
                if price < average_variable_cost {
                    Ok(LegalResult::Deterministic(true)) // Presumed predatory
                } else {
                    Ok(LegalResult::JudicialDiscretion {
                        issue: "Pricing above average variable cost but potentially predatory - requires intent analysis".to_string(),
                        context_id: uuid::Uuid::new_v4(),
                        narrative_hint: Some(
                            "AKZO test: Pricing between AVC and ATC is abusive only if intended to \
                             eliminate competitors. Requires evidence of strategic intent and ability \
                             to recoup losses.".to_string()
                        ),
                    })
                }
            }
            ExclusionaryAbuse::RefusalToDeal {
                essential_facility, ..
            } => {
                if *essential_facility {
                    // Essential facility doctrine: Refusal to supply essential input is abusive
                    Ok(LegalResult::Deterministic(true))
                } else {
                    Ok(LegalResult::JudicialDiscretion {
                        issue: "Refusal to deal with non-essential facility - requires justification analysis".to_string(),
                        context_id: uuid::Uuid::new_v4(),
                        narrative_hint: Some(
                            "Bronner test: Refusal to supply is abusive if: (1) refusal likely to \
                             eliminate competition, (2) refusal not objectively justified, (3) product/service \
                             indispensable for carrying on business.".to_string()
                        ),
                    })
                }
            }
            ExclusionaryAbuse::Tying { .. } => {
                // Microsoft test: Tying is abusive if products are separate, dominant in tying product,
                // no choice, forecloses competition in tied market
                Ok(LegalResult::Deterministic(true))
            }
            ExclusionaryAbuse::ExclusiveDealing {
                duration_months,
                market_foreclosure_percentage,
            } => {
                // Long-duration exclusive dealing with significant foreclosure
                if *duration_months > 24 && *market_foreclosure_percentage > 0.30 {
                    Ok(LegalResult::Deterministic(true))
                } else {
                    Ok(LegalResult::JudicialDiscretion {
                        issue: format!(
                            "Exclusive dealing {} months, {:.1}% foreclosure - requires market analysis",
                            duration_months,
                            market_foreclosure_percentage * 100.0
                        ),
                        context_id: uuid::Uuid::new_v4(),
                        narrative_hint: Some(
                            "Assess cumulative effect on market foreclosure and impact on consumer welfare.".to_string()
                        ),
                    })
                }
            }
            ExclusionaryAbuse::MarginSqueeze {
                wholesale_price,
                retail_price,
                downstream_competitor_costs,
            } => {
                // Margin squeeze test: Can as-efficient competitor operate profitably?
                let competitor_margin =
                    retail_price - wholesale_price - downstream_competitor_costs;
                if competitor_margin < 0.0 {
                    Ok(LegalResult::Deterministic(true)) // Negative margin = squeeze
                } else {
                    Ok(LegalResult::Deterministic(false)) // As-efficient competitor can compete
                }
            }
            ExclusionaryAbuse::Discrimination { .. } => {
                // Article 102(c): Applying dissimilar conditions placing parties at competitive disadvantage
                Ok(LegalResult::Deterministic(true))
            }
        }
    }
}

impl Default for Article102Conduct {
    fn default() -> Self {
        Self::new()
    }
}

/// Article 102 validation result
#[derive(Debug, Clone)]
pub struct Article102Validation {
    /// Dominance assessment
    pub dominance: DominanceAssessment,

    /// Whether abuse is established
    pub abuse_established: LegalResult<bool>,

    /// Final determination: prohibited by Article 102?
    pub prohibited: LegalResult<bool>,
}

impl Article102Validation {
    /// Check if conduct is prohibited
    pub fn is_prohibited(&self) -> bool {
        matches!(self.prohibited, LegalResult::Deterministic(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::competition::types::GeographicMarket;

    #[test]
    fn test_dominance_assessment() {
        let assessment = DominanceAssessment::assess(0.65, vec![]);
        assert!(assessment.is_dominant);
        assert!(assessment.market_share > 0.50);
    }

    #[test]
    fn test_predatory_pricing_below_avc() {
        let market = RelevantMarket {
            product_market: "Widgets".to_string(),
            geographic_market: GeographicMarket::EuWide,
            market_share: 0.55,
        };

        let conduct = Article102Conduct::new()
            .with_undertaking("Dominant Corp")
            .with_relevant_market(market)
            .with_abuse(AbuseType::Exclusionary(
                ExclusionaryAbuse::PredatoryPricing {
                    price: 5.0,
                    average_variable_cost: 8.0, // Pricing below AVC
                },
            ));

        let result = conduct.validate().unwrap();
        assert!(result.is_prohibited());
    }

    #[test]
    fn test_essential_facility_refusal() {
        let market = RelevantMarket {
            product_market: "Port infrastructure".to_string(),
            geographic_market: GeographicMarket::NationalMarket(
                super::super::types::MemberState::Spain,
            ),
            market_share: 0.80,
        };

        let conduct = Article102Conduct::new()
            .with_undertaking("Port Authority")
            .with_relevant_market(market)
            .with_abuse(AbuseType::Exclusionary(ExclusionaryAbuse::RefusalToDeal {
                customer: "Shipping Co".to_string(),
                essential_facility: true,
            }));

        let result = conduct.validate().unwrap();
        assert!(result.is_prohibited());
    }

    #[test]
    fn test_no_dominance_error() {
        let market = RelevantMarket {
            product_market: "Smartphones".to_string(),
            geographic_market: GeographicMarket::EuWide,
            market_share: 0.25, // Below threshold
        };

        let conduct = Article102Conduct::new()
            .with_undertaking("Small Player")
            .with_relevant_market(market)
            .with_abuse(AbuseType::Exploitative(ExploitativeAbuse::UnfairPricing {
                price: 100.0,
                competitive_price: 80.0,
                excessive_percentage: 0.25,
            }));

        let result = conduct.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompetitionError::NoDominantPosition { .. }
        ));
    }

    #[test]
    fn test_margin_squeeze() {
        let market = RelevantMarket {
            product_market: "Telecommunications".to_string(),
            geographic_market: GeographicMarket::EuWide,
            market_share: 0.70,
        };

        let conduct = Article102Conduct::new()
            .with_undertaking("Telecom Incumbent")
            .with_relevant_market(market)
            .with_abuse(AbuseType::Exclusionary(ExclusionaryAbuse::MarginSqueeze {
                wholesale_price: 40.0,
                retail_price: 50.0,
                downstream_competitor_costs: 15.0, // Competitor margin: 50-40-15 = -5 (negative!)
            }));

        let result = conduct.validate().unwrap();
        assert!(result.is_prohibited());
    }
}
