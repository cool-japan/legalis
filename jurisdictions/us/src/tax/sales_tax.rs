//! State Sales Tax Analysis
//!
//! This module tracks sales tax rates and nexus requirements across US states.
//!
//! # No Sales Tax States
//!
//! **5 states** have no state-level sales tax:
//! - Alaska (AK) - but localities may impose
//! - Delaware (DE)
//! - Montana (MT)
//! - New Hampshire (NH)
//! - Oregon (OR)
//!
//! # Post-Wayfair Era
//!
//! *South Dakota v. Wayfair, Inc.*, 138 S. Ct. 2080 (2018) eliminated the
//! physical presence requirement for sales tax nexus. States can now require
//! remote sellers to collect sales tax based on **economic nexus**.
//!
//! ## Typical Economic Nexus Threshold
//! - $100,000 in sales OR
//! - 200 transactions in the previous 12 months

use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// Sales tax information for a state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SalesTaxInfo {
    /// State identifier
    pub state_id: StateId,
    /// State sales tax rate (decimal, e.g., 0.0625 for 6.25%)
    pub state_rate: f64,
    /// Average local sales tax rate
    pub avg_local_rate: Option<f64>,
    /// Maximum combined rate (state + local)
    pub max_combined_rate: Option<f64>,
    /// Local sales taxes allowed
    pub local_taxes_allowed: bool,
    /// Economic nexus threshold (sales)
    pub economic_nexus_threshold: Option<u64>,
    /// Transaction count threshold
    pub transaction_threshold: Option<u32>,
}

/// Type of nexus (connection to state)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NexusType {
    /// Physical presence (office, warehouse, employees)
    PhysicalPresence,
    /// Economic nexus (sales/transaction threshold)
    EconomicNexus {
        /// Sales threshold
        sales_threshold: u64,
        /// Transaction count threshold
        transaction_count: Option<u32>,
    },
    /// Affiliate nexus (related entities in state)
    AffiliateNexus,
    /// Click-through nexus (in-state referrals)
    ClickThroughNexus,
}

/// Sales tax nexus determination
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SalesTaxNexus {
    /// State identifier
    pub state_id: StateId,
    /// Types of nexus recognized
    pub nexus_types: Vec<NexusType>,
    /// Marketplace facilitator law (Amazon must collect)
    pub marketplace_facilitator_law: bool,
}

/// Check if state has sales tax
pub fn has_sales_tax(state_code: &str) -> bool {
    !matches!(state_code, "AK" | "DE" | "MT" | "NH" | "OR")
}

/// Get state sales tax rate
pub fn state_sales_tax_rate(state_code: &str) -> SalesTaxInfo {
    let state_id = StateId::from_code(state_code);

    match state_code {
        // No sales tax states (5)
        "AK" => SalesTaxInfo {
            state_id,
            state_rate: 0.0,
            avg_local_rate: Some(0.0176), // Local taxes allowed
            max_combined_rate: Some(0.0750),
            local_taxes_allowed: true,
            economic_nexus_threshold: None,
            transaction_threshold: None,
        },
        "DE" | "MT" | "NH" | "OR" => SalesTaxInfo {
            state_id,
            state_rate: 0.0,
            avg_local_rate: None,
            max_combined_rate: None,
            local_taxes_allowed: false,
            economic_nexus_threshold: None,
            transaction_threshold: None,
        },

        // Highest combined rates
        "CA" => SalesTaxInfo {
            state_id,
            state_rate: 0.0725, // 7.25%
            avg_local_rate: Some(0.0157),
            max_combined_rate: Some(0.1025), // Up to 10.25% in some cities
            local_taxes_allowed: true,
            economic_nexus_threshold: Some(500_000),
            transaction_threshold: None,
        },
        "LA" => SalesTaxInfo {
            state_id,
            state_rate: 0.0445, // 4.45%
            avg_local_rate: Some(0.0510),
            max_combined_rate: Some(0.1155), // Highest average combined in US
            local_taxes_allowed: true,
            economic_nexus_threshold: Some(100_000),
            transaction_threshold: Some(200),
        },
        "TN" => SalesTaxInfo {
            state_id,
            state_rate: 0.0700, // 7.00%
            avg_local_rate: Some(0.0275),
            max_combined_rate: Some(0.0975),
            local_taxes_allowed: true,
            economic_nexus_threshold: Some(100_000),
            transaction_threshold: None,
        },
        "AR" => SalesTaxInfo {
            state_id,
            state_rate: 0.0650, // 6.50%
            avg_local_rate: Some(0.0295),
            max_combined_rate: Some(0.1150),
            local_taxes_allowed: true,
            economic_nexus_threshold: Some(100_000),
            transaction_threshold: Some(200),
        },
        "AL" => SalesTaxInfo {
            state_id,
            state_rate: 0.0400, // 4.00%
            avg_local_rate: Some(0.0541),
            max_combined_rate: Some(0.1300), // Highest in some localities
            local_taxes_allowed: true,
            economic_nexus_threshold: Some(250_000),
            transaction_threshold: None,
        },

        // Texas (no income tax, relies on sales tax)
        "TX" => SalesTaxInfo {
            state_id,
            state_rate: 0.0625, // 6.25%
            avg_local_rate: Some(0.0195),
            max_combined_rate: Some(0.0825),
            local_taxes_allowed: true,
            economic_nexus_threshold: Some(500_000),
            transaction_threshold: None,
        },

        // Other states (typical pattern)
        _ => {
            let (state_rate, local_allowed, economic_threshold) = match state_code {
                "AZ" => (0.0560, true, Some(100_000)),
                "CO" => (0.0290, true, Some(100_000)), // Lowest state rate
                "CT" => (0.0635, false, Some(100_000)),
                "DC" => (0.0600, false, Some(100_000)),
                "FL" => (0.0600, true, Some(100_000)),
                "GA" => (0.0400, true, Some(100_000)),
                "HI" => (0.0400, false, Some(100_000)),
                "ID" => (0.0600, true, Some(100_000)),
                "IL" => (0.0625, true, Some(100_000)),
                "IN" => (0.0700, false, Some(100_000)),
                "IA" => (0.0600, true, Some(100_000)),
                "KS" => (0.0650, true, Some(100_000)),
                "KY" => (0.0600, false, Some(100_000)),
                "ME" => (0.0550, false, Some(100_000)),
                "MD" => (0.0600, false, Some(100_000)),
                "MA" => (0.0625, false, Some(100_000)),
                "MI" => (0.0600, false, Some(100_000)),
                "MN" => (0.0688, true, Some(100_000)),
                "MS" => (0.0700, true, Some(250_000)),
                "MO" => (0.0423, true, Some(100_000)),
                "NE" => (0.0550, true, Some(100_000)),
                "NV" => (0.0685, true, Some(100_000)),
                "NJ" => (0.0663, false, Some(100_000)),
                "NM" => (0.0513, true, Some(100_000)),
                "NY" => (0.0400, true, Some(500_000)),
                "NC" => (0.0475, true, Some(100_000)),
                "ND" => (0.0500, true, Some(100_000)),
                "OH" => (0.0575, true, Some(100_000)),
                "OK" => (0.0450, true, Some(100_000)),
                "PA" => (0.0600, true, Some(100_000)),
                "RI" => (0.0700, false, Some(100_000)),
                "SC" => (0.0600, true, Some(100_000)),
                "SD" => (0.0450, true, Some(100_000)), // Origin of Wayfair case
                "UT" => (0.0485, true, Some(100_000)),
                "VT" => (0.0600, true, Some(100_000)),
                "VA" => (0.0530, true, Some(100_000)),
                "WA" => (0.0650, true, Some(100_000)),
                "WV" => (0.0600, true, Some(100_000)),
                "WI" => (0.0500, true, Some(100_000)),
                "WY" => (0.0400, true, Some(100_000)),
                _ => (0.0500, false, Some(100_000)),
            };

            SalesTaxInfo {
                state_id,
                state_rate,
                avg_local_rate: if local_allowed { Some(0.02) } else { None },
                max_combined_rate: if local_allowed {
                    Some(state_rate + 0.05)
                } else {
                    None
                },
                local_taxes_allowed: local_allowed,
                economic_nexus_threshold: economic_threshold,
                transaction_threshold: if economic_threshold.is_some() {
                    Some(200)
                } else {
                    None
                },
            }
        }
    }
}

/// Determine if seller has nexus in state (post-Wayfair)
pub fn post_wayfair_nexus(state_code: &str) -> SalesTaxNexus {
    let state_id = StateId::from_code(state_code);

    // Most states adopted $100k/200 transactions after Wayfair
    let economic_threshold = match state_code {
        "CA" | "TX" | "NY" => 500_000, // Higher thresholds
        "AL" | "MS" => 250_000,
        _ => 100_000,
    };

    SalesTaxNexus {
        state_id,
        nexus_types: vec![
            NexusType::PhysicalPresence,
            NexusType::EconomicNexus {
                sales_threshold: economic_threshold,
                transaction_count: Some(200),
            },
        ],
        marketplace_facilitator_law: true, // All states with sales tax have this
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_sales_tax_states() {
        assert!(!has_sales_tax("AK"));
        assert!(!has_sales_tax("DE"));
        assert!(!has_sales_tax("MT"));
        assert!(!has_sales_tax("NH"));
        assert!(!has_sales_tax("OR"));
    }

    #[test]
    fn test_has_sales_tax() {
        assert!(has_sales_tax("CA"));
        assert!(has_sales_tax("TX"));
        assert!(has_sales_tax("NY"));
        assert!(has_sales_tax("FL"));
    }

    #[test]
    fn test_california_highest_state_rate() {
        let ca = state_sales_tax_rate("CA");
        assert_eq!(ca.state_rate, 0.0725); // 7.25%

        // CA should have one of highest state rates
        let co = state_sales_tax_rate("CO");
        assert!(ca.state_rate > co.state_rate); // CO has lowest
    }

    #[test]
    fn test_colorado_lowest_state_rate() {
        let co = state_sales_tax_rate("CO");
        assert_eq!(co.state_rate, 0.0290); // 2.9% - lowest state rate
    }

    #[test]
    fn test_local_taxes_allowed() {
        let ca = state_sales_tax_rate("CA");
        assert!(ca.local_taxes_allowed);

        let la = state_sales_tax_rate("LA");
        assert!(la.local_taxes_allowed);

        let ma = state_sales_tax_rate("MA");
        assert!(!ma.local_taxes_allowed); // MA doesn't allow local sales tax
    }

    #[test]
    fn test_economic_nexus_thresholds() {
        let ca = state_sales_tax_rate("CA");
        assert_eq!(ca.economic_nexus_threshold, Some(500_000)); // CA: higher threshold

        let tx = state_sales_tax_rate("TX");
        assert_eq!(tx.economic_nexus_threshold, Some(500_000));

        let sd = state_sales_tax_rate("SD");
        assert_eq!(sd.economic_nexus_threshold, Some(100_000)); // SD v. Wayfair origin
    }

    #[test]
    fn test_south_dakota_wayfair() {
        // South Dakota v. Wayfair established economic nexus
        let sd_nexus = post_wayfair_nexus("SD");

        assert!(sd_nexus.nexus_types.iter().any(|nt| matches!(
            nt,
            NexusType::EconomicNexus {
                sales_threshold: 100_000,
                ..
            }
        )));

        assert!(sd_nexus.marketplace_facilitator_law);
    }

    #[test]
    fn test_marketplace_facilitator_laws() {
        // All states with sales tax have marketplace facilitator laws
        let states_with_tax = vec!["CA", "TX", "NY", "FL", "IL", "PA"];

        for state in states_with_tax {
            let nexus = post_wayfair_nexus(state);
            assert!(
                nexus.marketplace_facilitator_law,
                "{} should have marketplace facilitator law",
                state
            );
        }
    }

    #[test]
    fn test_physical_presence_still_creates_nexus() {
        let ny_nexus = post_wayfair_nexus("NY");

        assert!(ny_nexus.nexus_types.contains(&NexusType::PhysicalPresence));
    }

    #[test]
    fn test_louisiana_highest_combined_rate() {
        let la = state_sales_tax_rate("LA");
        // Louisiana has highest average combined rate in US
        assert!(la.max_combined_rate.unwrap() > 0.11); // Over 11%
    }

    #[test]
    fn test_alaska_local_only() {
        let ak = state_sales_tax_rate("AK");
        assert_eq!(ak.state_rate, 0.0); // No state sales tax
        assert!(ak.local_taxes_allowed); // But localities can impose
        assert!(ak.avg_local_rate.is_some());
    }
}
