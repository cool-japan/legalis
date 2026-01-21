//! Exclusive Dealing Analysis (CCA s.47)
//!
//! This module implements analysis of exclusive dealing conduct under section 47
//! of the Competition and Consumer Act 2010.
//!
//! ## Types of Exclusive Dealing
//!
//! Section 47 covers several forms of exclusive dealing:
//!
//! ### Third Line Forcing (s.47(6)-(7))
//!
//! Supplying goods/services on condition that acquirer:
//! - Acquire goods/services from a third party
//! - NOT acquire goods/services from a competitor of third party
//!
//! **Note:** Third line forcing was per se illegal until 2017. Now subject
//! to SLC test like other exclusive dealing.
//!
//! ### Exclusive Supply (s.47(2))
//!
//! Supplier conditions:
//! - Acquirer not deal with competitor of supplier
//! - Acquirer not resupply in particular place/manner
//!
//! ### Exclusive Acquisition (s.47(3)-(5))
//!
//! Acquirer conditions:
//! - Supplier not supply to others
//! - Supplier not supply particular goods/services to others
//!
//! ## SLC Test
//!
//! All exclusive dealing now requires substantial lessening of competition.
//! Factors include:
//! - Duration of arrangement
//! - Market coverage (% of market foreclosed)
//! - Barriers to alternative suppliers/customers
//! - Intent of parties
//!
//! ## Notification Regime
//!
//! Parties can notify exclusive dealing to ACCC:
//! - ACCC has 14 days to request conference
//! - If no objection within period, conduct deemed notified
//! - ACCC can revoke if circumstances change
//!
//! ## Leading Cases
//!
//! - Castlemaine Tooheys v Williams & Hodgson (1986) - Exclusive supply
//! - ACCC v Simply No-Knead (1996) - Franchise exclusive dealing
//! - ACCC v Cement Australia (2013) - Acquisition exclusive dealing

use serde::{Deserialize, Serialize};

use super::types::{MarketShare, RelevantMarket, Undertaking};

/// Type of exclusive dealing conduct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExclusiveDealingType {
    /// Third line forcing - supply conditional on acquiring from third party
    ThirdLineForcing {
        /// The third party whose goods/services must be acquired
        third_party: String,
        /// The goods/services that must be acquired
        required_goods_services: String,
    },
    /// Exclusive supply - conditions on acquirer
    ExclusiveSupply {
        /// Condition type
        condition: ExclusiveSupplyCondition,
    },
    /// Exclusive acquisition - conditions on supplier
    ExclusiveAcquisition {
        /// Condition type
        condition: ExclusiveAcquisitionCondition,
    },
    /// Full requirements contract
    FullRequirements {
        /// Duration in months
        duration_months: u32,
        /// Percentage of requirements
        requirements_percentage: f64,
    },
    /// Non-compete clause
    NonCompete {
        /// Duration in months
        duration_months: u32,
        /// Geographic scope
        geographic_scope: String,
        /// Products covered
        products_covered: Vec<String>,
    },
}

/// Exclusive supply condition types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExclusiveSupplyCondition {
    /// Not deal with competitor
    NotDealWithCompetitor {
        /// Competitor name (if specific)
        competitor: Option<String>,
    },
    /// Not resupply in particular place
    NotResupplyInPlace {
        /// Place restriction
        place: String,
    },
    /// Not resupply in particular manner
    NotResupplyManner {
        /// Manner restriction
        manner: String,
    },
}

/// Exclusive acquisition condition types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExclusiveAcquisitionCondition {
    /// Supplier not supply to others
    NotSupplyOthers,
    /// Supplier not supply particular goods/services to others
    NotSupplyParticularGoods {
        /// Goods/services restricted
        goods_services: Vec<String>,
    },
}

/// Exclusive dealing arrangement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExclusiveDealingArrangement {
    /// Type of exclusive dealing
    pub dealing_type: ExclusiveDealingType,
    /// Supplier undertaking
    pub supplier: Undertaking,
    /// Acquirer undertaking
    pub acquirer: Undertaking,
    /// Third party (if third line forcing)
    pub third_party: Option<Undertaking>,
    /// Relevant market
    pub market: Option<RelevantMarket>,
    /// Duration of arrangement (months)
    pub duration_months: Option<u32>,
    /// Market foreclosure percentage (0.0 to 1.0)
    pub market_foreclosure: Option<f64>,
    /// Whether arrangement is notified
    pub notified: bool,
    /// ACCC notification number (if notified)
    pub notification_number: Option<String>,
}

impl ExclusiveDealingArrangement {
    /// Create new third line forcing arrangement
    pub fn third_line_forcing(
        supplier: Undertaking,
        acquirer: Undertaking,
        third_party: Undertaking,
        required_goods: impl Into<String>,
    ) -> Self {
        Self {
            dealing_type: ExclusiveDealingType::ThirdLineForcing {
                third_party: third_party.name.clone(),
                required_goods_services: required_goods.into(),
            },
            supplier,
            acquirer,
            third_party: Some(third_party),
            market: None,
            duration_months: None,
            market_foreclosure: None,
            notified: false,
            notification_number: None,
        }
    }

    /// Create new exclusive supply arrangement
    pub fn exclusive_supply(
        supplier: Undertaking,
        acquirer: Undertaking,
        condition: ExclusiveSupplyCondition,
    ) -> Self {
        Self {
            dealing_type: ExclusiveDealingType::ExclusiveSupply { condition },
            supplier,
            acquirer,
            third_party: None,
            market: None,
            duration_months: None,
            market_foreclosure: None,
            notified: false,
            notification_number: None,
        }
    }

    /// Set market
    pub fn with_market(mut self, market: RelevantMarket) -> Self {
        self.market = Some(market);
        self
    }

    /// Set duration
    pub fn with_duration(mut self, months: u32) -> Self {
        self.duration_months = Some(months);
        self
    }

    /// Set market foreclosure
    pub fn with_foreclosure(mut self, percentage: f64) -> Self {
        self.market_foreclosure = Some(percentage);
        self
    }

    /// Mark as notified
    pub fn notified_to_accc(mut self, notification_number: impl Into<String>) -> Self {
        self.notified = true;
        self.notification_number = Some(notification_number.into());
        self
    }
}

/// Exclusive dealing analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExclusiveDealingResult {
    /// Whether conduct constitutes exclusive dealing
    pub is_exclusive_dealing: bool,
    /// Type of exclusive dealing
    pub dealing_type: Option<ExclusiveDealingType>,
    /// Whether conduct likely to SLC
    pub likely_slc: bool,
    /// Contravention likely
    pub contravention_likely: bool,
    /// Notification status
    pub notification_status: NotificationStatus,
    /// Defences available
    pub defences: Vec<ExclusiveDealingDefence>,
    /// Reasoning
    pub reasoning: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Notification status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationStatus {
    /// Not notified
    NotNotified,
    /// Notification pending
    NotificationPending {
        /// Notification number
        notification_number: String,
        /// Days remaining in assessment period
        days_remaining: u32,
    },
    /// Notification allowed (no ACCC objection)
    NotificationAllowed {
        /// Notification number
        notification_number: String,
    },
    /// Notification revoked
    NotificationRevoked {
        /// Notification number
        notification_number: String,
        /// Reason for revocation
        reason: String,
    },
    /// ACCC objected
    AcccObjected {
        /// Notification number
        notification_number: String,
        /// Objection reason
        reason: String,
    },
}

/// Defences to exclusive dealing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExclusiveDealingDefence {
    /// No SLC
    NoSlc { reason: String },
    /// Notification in place
    Notification { notification_number: String },
    /// Authorisation granted
    Authorisation {
        determination_number: String,
        public_benefit: String,
    },
    /// Conduct not exclusive dealing
    NotExclusiveDealing { reason: String },
}

/// Exclusive dealing analyzer
pub struct ExclusiveDealingAnalyzer;

impl ExclusiveDealingAnalyzer {
    /// Analyze exclusive dealing arrangement
    pub fn analyze(arrangement: &ExclusiveDealingArrangement) -> ExclusiveDealingResult {
        let is_exclusive = Self::is_exclusive_dealing(arrangement);
        let likely_slc = is_exclusive && Self::assess_slc(arrangement);
        let contravention = likely_slc && !arrangement.notified;
        let notification_status = Self::determine_notification_status(arrangement);
        let defences = Self::identify_defences(arrangement, likely_slc);
        let reasoning = Self::build_reasoning(arrangement, is_exclusive, likely_slc);
        let recommendations = Self::generate_recommendations(arrangement, likely_slc);

        ExclusiveDealingResult {
            is_exclusive_dealing: is_exclusive,
            dealing_type: if is_exclusive {
                Some(arrangement.dealing_type.clone())
            } else {
                None
            },
            likely_slc,
            contravention_likely: contravention,
            notification_status,
            defences,
            reasoning,
            recommendations,
        }
    }

    /// Check if arrangement constitutes exclusive dealing
    fn is_exclusive_dealing(arrangement: &ExclusiveDealingArrangement) -> bool {
        // Check for condition element
        match &arrangement.dealing_type {
            ExclusiveDealingType::ThirdLineForcing { .. } => true,
            ExclusiveDealingType::ExclusiveSupply { .. } => true,
            ExclusiveDealingType::ExclusiveAcquisition { .. } => true,
            ExclusiveDealingType::FullRequirements {
                requirements_percentage,
                ..
            } => {
                // Full requirements > 80% typically considered exclusive
                *requirements_percentage >= 0.80
            }
            ExclusiveDealingType::NonCompete { .. } => true,
        }
    }

    /// Assess likelihood of SLC
    fn assess_slc(arrangement: &ExclusiveDealingArrangement) -> bool {
        // Duration factor
        let duration_concern = arrangement.duration_months.map(|d| d > 24).unwrap_or(false);

        // Foreclosure factor
        let foreclosure_concern = arrangement
            .market_foreclosure
            .map(|f| f > 0.20)
            .unwrap_or(false);

        // Market power factor (from supplier)
        let supplier_power = arrangement
            .supplier
            .market_share
            .as_ref()
            .map(|s| s.indicates_substantial_power())
            .unwrap_or(false);

        // Cumulative effect consideration
        duration_concern || foreclosure_concern || supplier_power
    }

    /// Determine notification status
    fn determine_notification_status(
        arrangement: &ExclusiveDealingArrangement,
    ) -> NotificationStatus {
        if arrangement.notified {
            if let Some(ref number) = arrangement.notification_number {
                NotificationStatus::NotificationAllowed {
                    notification_number: number.clone(),
                }
            } else {
                NotificationStatus::NotNotified
            }
        } else {
            NotificationStatus::NotNotified
        }
    }

    /// Identify defences
    fn identify_defences(
        arrangement: &ExclusiveDealingArrangement,
        likely_slc: bool,
    ) -> Vec<ExclusiveDealingDefence> {
        let mut defences = Vec::new();

        if !likely_slc {
            defences.push(ExclusiveDealingDefence::NoSlc {
                reason: "Conduct not likely to substantially lessen competition".into(),
            });
        }

        if arrangement.notified
            && let Some(ref number) = arrangement.notification_number
        {
            defences.push(ExclusiveDealingDefence::Notification {
                notification_number: number.clone(),
            });
        }

        defences
    }

    /// Build reasoning
    fn build_reasoning(
        arrangement: &ExclusiveDealingArrangement,
        is_exclusive: bool,
        likely_slc: bool,
    ) -> String {
        let mut parts = Vec::new();

        parts.push("Exclusive dealing analysis under CCA s.47".into());

        if is_exclusive {
            let dealing_desc = match &arrangement.dealing_type {
                ExclusiveDealingType::ThirdLineForcing { .. } => "Third line forcing (s.47(6)-(7))",
                ExclusiveDealingType::ExclusiveSupply { .. } => "Exclusive supply (s.47(2))",
                ExclusiveDealingType::ExclusiveAcquisition { .. } => {
                    "Exclusive acquisition (s.47(3)-(5))"
                }
                ExclusiveDealingType::FullRequirements { .. } => "Full requirements contract",
                ExclusiveDealingType::NonCompete { .. } => "Non-compete clause",
            };
            parts.push(format!("Conduct type: {}", dealing_desc));

            if likely_slc {
                parts.push("Conduct likely to substantially lessen competition".into());

                if let Some(foreclosure) = arrangement.market_foreclosure {
                    parts.push(format!("Market foreclosure: {:.1}%", foreclosure * 100.0));
                }

                if let Some(duration) = arrangement.duration_months {
                    parts.push(format!("Duration: {} months", duration));
                }
            } else {
                parts.push("Conduct not likely to substantially lessen competition".into());
            }
        } else {
            parts.push("Conduct does not constitute exclusive dealing".into());
        }

        parts.join(". ")
    }

    /// Generate recommendations
    fn generate_recommendations(
        arrangement: &ExclusiveDealingArrangement,
        likely_slc: bool,
    ) -> Vec<String> {
        let mut recs = Vec::new();

        if likely_slc {
            if !arrangement.notified {
                recs.push("Consider lodging notification with ACCC".into());
            }
            recs.push("Review arrangement for SLC-reducing modifications".into());
            recs.push("Consider shorter duration or lower exclusivity".into());
        } else {
            recs.push("Maintain documentation of competitive effects".into());
            recs.push("Monitor for cumulative market effects".into());
        }

        recs
    }

    /// Calculate market foreclosure
    pub fn calculate_foreclosure(supplier_share: &MarketShare, exclusivity_percentage: f64) -> f64 {
        supplier_share
            .primary_share()
            .map(|s| s * exclusivity_percentage)
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_line_forcing() {
        let supplier = Undertaking::new("Franchisor");
        let acquirer = Undertaking::new("Franchisee");
        let third_party = Undertaking::new("Approved Supplier");

        let arrangement = ExclusiveDealingArrangement::third_line_forcing(
            supplier,
            acquirer,
            third_party,
            "Coffee beans",
        );

        let result = ExclusiveDealingAnalyzer::analyze(&arrangement);

        assert!(result.is_exclusive_dealing);
        assert!(matches!(
            result.dealing_type,
            Some(ExclusiveDealingType::ThirdLineForcing { .. })
        ));
    }

    #[test]
    fn test_exclusive_supply() {
        let supplier =
            Undertaking::new("Supplier Co").with_market_share(MarketShare::from_revenue(0.35));
        let acquirer = Undertaking::new("Distributor");

        let arrangement = ExclusiveDealingArrangement::exclusive_supply(
            supplier,
            acquirer,
            ExclusiveSupplyCondition::NotDealWithCompetitor { competitor: None },
        )
        .with_duration(36)
        .with_foreclosure(0.25);

        let result = ExclusiveDealingAnalyzer::analyze(&arrangement);

        assert!(result.is_exclusive_dealing);
        assert!(result.likely_slc); // Due to duration and foreclosure
    }

    #[test]
    fn test_notified_arrangement() {
        let supplier = Undertaking::new("Supplier");
        let acquirer = Undertaking::new("Acquirer");

        let arrangement = ExclusiveDealingArrangement::exclusive_supply(
            supplier,
            acquirer,
            ExclusiveSupplyCondition::NotDealWithCompetitor { competitor: None },
        )
        .notified_to_accc("N12345");

        let result = ExclusiveDealingAnalyzer::analyze(&arrangement);

        assert!(matches!(
            result.notification_status,
            NotificationStatus::NotificationAllowed { .. }
        ));
    }

    #[test]
    fn test_foreclosure_calculation() {
        let share = MarketShare::from_revenue(0.40);
        let exclusivity = 0.75;

        let foreclosure = ExclusiveDealingAnalyzer::calculate_foreclosure(&share, exclusivity);

        assert!((foreclosure - 0.30).abs() < 0.01);
    }

    #[test]
    fn test_full_requirements() {
        let arrangement = ExclusiveDealingArrangement {
            dealing_type: ExclusiveDealingType::FullRequirements {
                duration_months: 24,
                requirements_percentage: 0.90,
            },
            supplier: Undertaking::new("Supplier"),
            acquirer: Undertaking::new("Acquirer"),
            third_party: None,
            market: None,
            duration_months: Some(24),
            market_foreclosure: None,
            notified: false,
            notification_number: None,
        };

        let result = ExclusiveDealingAnalyzer::analyze(&arrangement);

        assert!(result.is_exclusive_dealing);
    }
}
