//! Article 101 TFEU - Anti-competitive agreements
//!
//! Article 101(1): Prohibits agreements between undertakings, decisions by associations,
//! and concerted practices which may affect trade between Member States and which have
//! as their object or effect the prevention, restriction or distortion of competition.
//!
//! Article 101(2): Such agreements are automatically void.
//!
//! Article 101(3): Provides exemptions if all four criteria are met:
//! 1. Improves production/distribution OR promotes technical/economic progress
//! 2. Allows consumers a fair share of resulting benefit
//! 3. Restrictions are indispensable to achieving objectives
//! 4. Does not eliminate competition for substantial part of products

use super::error::CompetitionError;
use super::types::MemberState;
use legalis_core::LegalResult;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Types of concerted practices under Article 101(1)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConcertedPractice {
    /// Price fixing (Article 101(1)(a) - hardcore restriction)
    PriceFixing {
        agreed_minimum_price: Option<f64>,
        market_share_combined: f64,
    },

    /// Market sharing (Article 101(1)(c) - hardcore restriction)
    MarketSharing {
        allocation_type: MarketAllocation,
        market_share_combined: f64,
    },

    /// Limiting production/technical development (Article 101(1)(b))
    LimitingProduction {
        description: String,
        market_share_combined: f64,
    },

    /// Applying dissimilar conditions to equivalent transactions (Article 101(1)(d))
    Discrimination { description: String },

    /// Tying arrangements (Article 101(1)(e))
    Tying {
        main_product: String,
        tied_product: String,
    },

    /// Information exchange (may restrict competition)
    InformationExchange {
        information_type: String,
        strategic: bool, // Strategic info (prices, costs, customers) vs non-strategic
    },
}

/// Market allocation methods
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketAllocation {
    Geographic(Vec<String>),
    CustomerAllocation(Vec<String>),
    ProductAllocation(Vec<String>),
}

/// Article 101(3) exemption criteria
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Article101Exemption {
    /// Criterion 1: Improves production/distribution OR promotes technical/economic progress
    pub efficiency_gains: Option<String>,

    /// Criterion 2: Allows consumers fair share of benefit
    pub consumer_benefit: Option<String>,

    /// Criterion 3: Restrictions are indispensable
    pub indispensable: bool,

    /// Criterion 4: Does not eliminate competition
    pub competition_remains: bool,
}

impl Article101Exemption {
    pub fn new() -> Self {
        Self {
            efficiency_gains: None,
            consumer_benefit: None,
            indispensable: false,
            competition_remains: false,
        }
    }

    pub fn with_efficiency_gains(mut self, gains: impl Into<String>) -> Self {
        self.efficiency_gains = Some(gains.into());
        self
    }

    pub fn with_consumer_benefit(mut self, benefit: impl Into<String>) -> Self {
        self.consumer_benefit = Some(benefit.into());
        self
    }

    pub fn with_indispensable(mut self, value: bool) -> Self {
        self.indispensable = value;
        self
    }

    pub fn with_competition_remains(mut self, value: bool) -> Self {
        self.competition_remains = value;
        self
    }

    /// Validate if all four Article 101(3) criteria are met
    pub fn validate(&self) -> Result<LegalResult<bool>, CompetitionError> {
        // All four criteria must be satisfied
        if self.efficiency_gains.is_none() {
            return Err(CompetitionError::exemption_not_met(
                "No efficiency gains or technical progress demonstrated (criterion 1)",
            ));
        }

        if self.consumer_benefit.is_none() {
            return Err(CompetitionError::exemption_not_met(
                "No consumer benefit demonstrated (criterion 2)",
            ));
        }

        if !self.indispensable {
            return Err(CompetitionError::exemption_not_met(
                "Restrictions not indispensable (criterion 3)",
            ));
        }

        if !self.competition_remains {
            return Err(CompetitionError::exemption_not_met(
                "Eliminates competition (criterion 4)",
            ));
        }

        Ok(LegalResult::Deterministic(true))
    }
}

impl Default for Article101Exemption {
    fn default() -> Self {
        Self::new()
    }
}

/// Article 101 agreement validation
#[derive(Debug, Clone)]
pub struct Article101Agreement {
    pub parties: Vec<String>,
    pub practice: Option<ConcertedPractice>,
    pub affected_member_states: Vec<MemberState>,
    pub exemption_claim: Option<Article101Exemption>,
}

impl Article101Agreement {
    pub fn new() -> Self {
        Self {
            parties: Vec::new(),
            practice: None,
            affected_member_states: Vec::new(),
            exemption_claim: None,
        }
    }

    pub fn with_parties(mut self, parties: Vec<impl Into<String>>) -> Self {
        self.parties = parties.into_iter().map(|p| p.into()).collect();
        self
    }

    pub fn with_practice(mut self, practice: ConcertedPractice) -> Self {
        self.practice = Some(practice);
        self
    }

    pub fn with_affected_member_states(mut self, states: Vec<MemberState>) -> Self {
        self.affected_member_states = states;
        self
    }

    pub fn with_exemption_claim(mut self, exemption: Article101Exemption) -> Self {
        self.exemption_claim = Some(exemption);
        self
    }

    /// Validate Article 101 compliance
    pub fn validate(&self) -> Result<Article101Validation, CompetitionError> {
        // Check required fields
        if self.parties.len() < 2 {
            return Err(CompetitionError::missing_field(
                "At least 2 parties required for agreement",
            ));
        }

        let practice = self
            .practice
            .as_ref()
            .ok_or_else(|| CompetitionError::missing_field("practice"))?;

        // Element 1: Agreement between undertakings
        let agreement_established = LegalResult::Deterministic(true);

        // Element 2: Appreciable effect on competition (de minimis test)
        let appreciable_effect = self.check_appreciable_effect(practice)?;

        // Element 3: Effect on trade between Member States
        if self.affected_member_states.is_empty() {
            return Err(CompetitionError::InsufficientCrossBorderEffect);
        }
        let affects_interstate_trade = LegalResult::Deterministic(true);

        // Check if hardcore restriction (always prohibited)
        let is_hardcore = self.is_hardcore_restriction(practice);

        // Check exemption if claimed
        let exemption_valid = if let Some(ref exemption) = self.exemption_claim {
            if is_hardcore {
                // Hardcore restrictions rarely qualify for exemption
                LegalResult::JudicialDiscretion {
                    issue: "Article 101(3) exemption claimed for hardcore restriction (price fixing/market sharing)".to_string(),
                    context_id: uuid::Uuid::new_v4(),
                    narrative_hint: Some(
                        "Hardcore restrictions under Article 101(1)(a)-(c) are presumed anti-competitive. \
                         Exemption requires exceptional circumstances and substantial evidence of efficiencies \
                         that outweigh competition concerns.".to_string()
                    ),
                }
            } else {
                exemption.validate()?
            }
        } else {
            LegalResult::Deterministic(false)
        };

        // Determine prohibition status
        let is_appreciable = matches!(appreciable_effect, LegalResult::Deterministic(true));
        let is_exempt = matches!(exemption_valid, LegalResult::Deterministic(true));

        let prohibited = if !is_appreciable || is_exempt {
            // Not prohibited if: (1) de minimis applies, or (2) exemption applies
            LegalResult::Deterministic(false)
        } else {
            // Prohibited by Article 101(1)
            LegalResult::Deterministic(true)
        };

        Ok(Article101Validation {
            agreement_established,
            appreciable_effect,
            affects_interstate_trade,
            is_hardcore_restriction: is_hardcore,
            exemption_valid,
            prohibited,
        })
    }

    /// Check if agreement has appreciable effect on competition (de minimis test)
    ///
    /// De minimis Notice 2014: Agreement does not appreciably restrict competition if:
    /// - Horizontal agreements: combined market share ≤ 10%
    /// - Vertical agreements: market share of each party ≤ 15%
    /// - Hardcore restrictions: no de minimis threshold applies
    fn check_appreciable_effect(
        &self,
        practice: &ConcertedPractice,
    ) -> Result<LegalResult<bool>, CompetitionError> {
        let market_share = match practice {
            ConcertedPractice::PriceFixing {
                market_share_combined,
                ..
            }
            | ConcertedPractice::MarketSharing {
                market_share_combined,
                ..
            }
            | ConcertedPractice::LimitingProduction {
                market_share_combined,
                ..
            } => *market_share_combined,
            _ => 0.5, // Assume appreciable for qualitative restrictions
        };

        if market_share <= 0.10 {
            // Below de minimis threshold (10% for horizontal)
            Ok(LegalResult::Deterministic(false))
        } else {
            Ok(LegalResult::Deterministic(true))
        }
    }

    /// Check if practice is hardcore restriction
    fn is_hardcore_restriction(&self, practice: &ConcertedPractice) -> bool {
        matches!(
            practice,
            ConcertedPractice::PriceFixing { .. } | ConcertedPractice::MarketSharing { .. }
        )
    }
}

impl Default for Article101Agreement {
    fn default() -> Self {
        Self::new()
    }
}

/// Article 101 validation result
#[derive(Debug, Clone)]
pub struct Article101Validation {
    /// Element 1: Agreement/concerted practice established
    pub agreement_established: LegalResult<bool>,

    /// Element 2: Appreciable effect on competition (de minimis test)
    pub appreciable_effect: LegalResult<bool>,

    /// Element 3: Affects trade between Member States
    pub affects_interstate_trade: LegalResult<bool>,

    /// Whether practice is hardcore restriction (price fixing, market sharing)
    pub is_hardcore_restriction: bool,

    /// Article 101(3) exemption valid (if claimed)
    pub exemption_valid: LegalResult<bool>,

    /// Final determination: prohibited by Article 101(1)?
    pub prohibited: LegalResult<bool>,
}

impl Article101Validation {
    /// Check if agreement is prohibited
    pub fn is_prohibited(&self) -> bool {
        matches!(self.prohibited, LegalResult::Deterministic(true))
    }

    /// Check if de minimis applies (no appreciable effect)
    pub fn is_de_minimis(&self) -> bool {
        matches!(self.appreciable_effect, LegalResult::Deterministic(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_fixing_hardcore() {
        let agreement = Article101Agreement::new()
            .with_parties(vec!["Company A", "Company B"])
            .with_practice(ConcertedPractice::PriceFixing {
                agreed_minimum_price: Some(100.0),
                market_share_combined: 0.35,
            })
            .with_affected_member_states(vec![MemberState::Germany, MemberState::France]);

        let result = agreement.validate().unwrap();
        assert!(result.is_prohibited());
        assert!(result.is_hardcore_restriction);
    }

    #[test]
    fn test_de_minimis_not_prohibited() {
        let agreement = Article101Agreement::new()
            .with_parties(vec!["Small Co A", "Small Co B"])
            .with_practice(ConcertedPractice::PriceFixing {
                agreed_minimum_price: Some(50.0),
                market_share_combined: 0.08, // Below 10% threshold
            })
            .with_affected_member_states(vec![MemberState::Netherlands]);

        let result = agreement.validate().unwrap();
        assert!(!result.is_prohibited());
        assert!(result.is_de_minimis());
    }

    #[test]
    fn test_exemption_all_criteria_met() {
        let exemption = Article101Exemption::new()
            .with_efficiency_gains("Joint R&D reduces costs by 30%")
            .with_consumer_benefit("Lower prices passed to consumers")
            .with_indispensable(true)
            .with_competition_remains(true);

        let result = exemption.validate().unwrap();
        assert!(matches!(result, LegalResult::Deterministic(true)));
    }

    #[test]
    fn test_exemption_missing_consumer_benefit() {
        let exemption = Article101Exemption::new()
            .with_efficiency_gains("Cost savings")
            .with_indispensable(true)
            .with_competition_remains(true);
        // Missing consumer_benefit

        let result = exemption.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CompetitionError::ExemptionNotMet { .. }
        ));
    }

    #[test]
    fn test_market_sharing_hardcore() {
        let agreement = Article101Agreement::new()
            .with_parties(vec!["Company X", "Company Y"])
            .with_practice(ConcertedPractice::MarketSharing {
                allocation_type: MarketAllocation::Geographic(vec![
                    "Northern Europe".to_string(),
                    "Southern Europe".to_string(),
                ]),
                market_share_combined: 0.60,
            })
            .with_affected_member_states(vec![
                MemberState::Sweden,
                MemberState::Spain,
                MemberState::Italy,
            ]);

        let result = agreement.validate().unwrap();
        assert!(result.is_prohibited());
        assert!(result.is_hardcore_restriction);
    }
}
