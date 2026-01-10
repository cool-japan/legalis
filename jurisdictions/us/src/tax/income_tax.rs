//! State Income Tax Analysis
//!
//! This module tracks state income tax structures across all US jurisdictions,
//! enabling analysis of tax competition, revenue policy, and interstate tax
//! burden comparison.
//!
//! # Overview
//!
//! State income tax is a major source of state revenue, accounting for approximately
//! 37% of state tax collections nationwide. However, **interstate competition** for
//! residents and businesses has led to significant variation in state tax policy.
//!
//! As of 2024, states follow three distinct approaches:
//! - **No Income Tax** (9 states): Attract residents/businesses with zero income tax
//! - **Flat Tax** (9 states): Simple administration, predictable rates
//! - **Progressive Tax** (33 states): Higher rates on higher incomes, more revenue
//!
//! ## Tax Competition and Migration
//!
//! The "tax competition" phenomenon describes how states compete for residents and
//! businesses through favorable tax policies. This creates interstate dynamics:
//!
//! **Supply-Side Arguments** (pro-no-tax):
//! - Lower taxes attract high-income earners and businesses
//! - Economic growth compensates for lost tax revenue
//! - Examples: TX, FL have strong population growth, no income tax
//!
//! **Demand-Side Arguments** (pro-progressive-tax):
//! - Tax revenue funds services (education, infrastructure, healthcare)
//! - Quality services attract skilled workers and families
//! - Examples: CA, NY maintain high services despite high taxes
//!
//! **Empirical Evidence** (mixed):
//! - Migration from high-tax to low-tax states exists but is modest
//! - Other factors (jobs, weather, family) dominate migration decisions
//! - Corporations show more tax sensitivity than individuals
//!
//! ## Why 9 States Have No Income Tax
//!
//! No-income-tax states compensate through alternative revenue sources:
//!
//! **Alaska**: Oil revenue (Alaska Permanent Fund) + federal transfers
//! - Petroleum production taxes provide 70%+ of state revenue
//! - Residents receive annual dividend from permanent fund (~$1,600)
//!
//! **Florida**: Sales tax (6% state + local) + tourism revenue
//! - 108 million tourists annually generate sales tax
//! - No corporate income tax attracts retirees and snowbirds
//!
//! **Nevada**: Gaming tax + sales tax (6.85% state)
//! - Casino tax provides 15-20% of state revenue
//! - Low business costs attract corporations (no franchise tax)
//!
//! **South Dakota**: Sales tax (4.5% state) + tourism (Mount Rushmore)
//! - Low-cost government operations
//! - Trust-friendly laws attract financial services
//!
//! **Tennessee**: Eliminated income tax in 2021 (previously 6% on dividends/interest)
//! - Sales tax (7% state, highest combined rates in US)
//! - Manufacturing and distribution centers
//!
//! **Texas**: Sales tax (6.25% state) + property tax + oil/gas severance taxes
//! - Property taxes among highest in US (compensates for no income tax)
//! - Large economy (2nd in US) provides scale
//!
//! **Washington**: Sales tax (6.5% state) + B&O tax (business & occupation)
//! - High sales tax rates (combined up to 10.4% in Seattle)
//! - Major corporations (Microsoft, Amazon, Boeing) provide economic base
//!
//! **Wyoming**: Mineral severance taxes (coal, oil, gas) + tourism
//! - Natural resource extraction provides revenue
//! - Low population (578k) enables low-cost government
//!
//! **New Hampshire**: Eliminated in 2025 (was 5% on dividends/interest only)
//! - Property taxes + meals/rooms taxes
//! - "Live Free or Die" libertarian tradition
//!
//! # Progressive vs. Flat Tax Debate
//!
//! ## Progressive Tax (33 states)
//!
//! **Definition**: Tax rate increases as income increases (marginal rates)
//!
//! **Advantages**:
//! - **Vertical Equity**: Those with greater ability to pay contribute more
//! - **Revenue Stability**: High earners provide stable tax base
//! - **Automatic Stabilizer**: Tax collections fall during recessions (cushions economy)
//! - **Revenue Capacity**: Can raise more revenue from same tax base
//!
//! **Disadvantages**:
//! - **Complexity**: Multiple brackets increase compliance costs
//! - **Marginal Rate Confusion**: Taxpayers misunderstand marginal vs. effective rates
//! - **Bracket Creep**: Inflation pushes people into higher brackets (without indexing)
//! - **Migration Risk**: High earners may relocate to low-tax states
//!
//! **Highest Rates** (2024):
//! - California: 13.3% (highest in US) on income >$1M
//! - Hawaii: 11% on income >$200k
//! - New York: 10.9% on income >$25M (NYC residents pay additional 3.876% local)
//! - New Jersey: 10.75% on income >$1M
//! - Washington DC: 10.75% on income >$1M
//!
//! ## Flat Tax (9 states)
//!
//! **Definition**: Single rate applies to all taxable income
//!
//! **Advantages**:
//! - **Simplicity**: Easy to calculate, low compliance costs
//! - **Transparency**: No confusion about marginal vs. effective rates
//! - **Economic Neutrality**: Doesn't distort work/investment decisions as much
//! - **Political Appeal**: Perceived as "fair" (everyone pays same rate)
//!
//! **Disadvantages**:
//! - **Regressive Impact**: Same rate is larger burden on low-income earners
//! - **Revenue Limitations**: Can't easily raise revenue without broad tax increase
//! - **Limited Redistribution**: Less progressive than graduated rates
//!
//! **Flat Tax States** (2024):
//! - Colorado: 4.40%
//! - Illinois: 4.95%
//! - Indiana: 3.15% (lowest flat rate) + county taxes
//! - Kentucky: 4.50%
//! - Massachusetts: 5.00%
//! - Michigan: 4.25%
//! - North Carolina: 4.75%
//! - Pennsylvania: 3.07% (lowest) + local taxes
//! - Utah: 4.85%
//!
//! ## Trend: Flat Tax Movement
//!
//! Several states have moved from progressive to flat tax in recent decades:
//! - **North Carolina** (2014): Reduced from 7.75% progressive to 4.75% flat
//! - **Arizona** (2021): Moving to 2.5% flat by 2024
//! - **Iowa** (2023): Transitioning to 3.9% flat by 2026
//! - **Mississippi** (2022): Moving to 4% flat by 2026
//!
//! This "flat tax movement" reflects:
//! - Republican control of state governments
//! - Supply-side economic theory ("lower taxes → growth")
//! - Tax competition with neighboring states
//! - Simplification arguments
//!
//! # Local Income Taxes
//!
//! Some states allow **local governments** to impose additional income taxes:
//!
//! **Major Cities with Local Income Tax**:
//! - **New York City**: 3.078%-3.876% (on top of NYS 4%-10.9%)
//! - **Philadelphia**: 3.79% (residents) / 3.44% (non-residents)
//! - **Detroit**: 2.4% (residents) / 1.2% (non-residents)
//! - **Columbus, OH**: 2.5%
//! - **Kansas City, MO**: 1%
//!
//! **States Allowing Local Income Tax**:
//! - **Indiana**: All 92 counties impose county income tax (0.25%-3.38%)
//! - **Ohio**: ~600 municipalities impose local income tax
//! - **Maryland**: All 23 counties + Baltimore City impose local tax (2.25%-3.20%)
//! - **Pennsylvania**: 2,500+ municipalities impose local tax
//! - **Kentucky**: Some cities (Louisville, Lexington) impose occupational tax
//!
//! Combined state + local rates can exceed 14% (NYC: 10.9% state + 3.876% city = 14.776%)

use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// Type of income tax structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IncomeTaxType {
    /// No state income tax
    None,
    /// Flat tax rate
    Flat {
        /// Tax rate (as decimal, e.g., 0.0495 for 4.95%)
        rate: f64,
    },
    /// Progressive tax with brackets
    Progressive {
        /// Tax brackets (income threshold, rate)
        brackets: Vec<TaxBracket>,
        /// Top marginal rate
        top_rate: f64,
    },
}

/// Tax bracket for progressive income tax
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxBracket {
    /// Income threshold for this bracket
    pub threshold: u64,
    /// Tax rate for income above threshold (decimal)
    pub rate: f64,
}

/// State income tax structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomeTaxStructure {
    /// State identifier
    pub state_id: StateId,
    /// Tax structure type
    pub tax_type: IncomeTaxType,
    /// Local income taxes exist (e.g., NYC, Philadelphia)
    pub has_local_income_tax: bool,
    /// Notable exemptions or deductions
    pub notable_features: Vec<String>,
}

/// Check if state has income tax
///
/// # Example
/// ```
/// use legalis_us::tax::income_tax::has_state_income_tax;
///
/// assert!(!has_state_income_tax("TX")); // Texas: no income tax
/// assert!(!has_state_income_tax("FL")); // Florida: no income tax
/// assert!(has_state_income_tax("CA"));  // California: progressive
/// assert!(has_state_income_tax("CO"));  // Colorado: flat tax
/// ```
pub fn has_state_income_tax(state_code: &str) -> bool {
    !matches!(
        state_code,
        "AK" | "FL" | "NV" | "SD" | "TN" | "TX" | "WA" | "WY" | "NH"
    )
}

/// Get list of states with no income tax
pub fn no_income_tax_states() -> Vec<&'static str> {
    vec!["AK", "FL", "NV", "SD", "TN", "TX", "WA", "WY", "NH"]
}

/// Get income tax structure for a state
pub fn income_tax_structure(state_code: &str) -> IncomeTaxStructure {
    let state_id = StateId::from_code(state_code);

    match state_code {
        // No income tax states (9)
        "AK" | "FL" | "NV" | "SD" | "TN" | "TX" | "WA" | "WY" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::None,
            has_local_income_tax: false,
            notable_features: vec![],
        },

        "NH" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::None,
            has_local_income_tax: false,
            notable_features: vec![
                "Formerly taxed dividends/interest at 5%, eliminated 2025".to_string(),
            ],
        },

        // Flat tax states (9)
        "CO" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0440 }, // 4.40%
            has_local_income_tax: false,
            notable_features: vec![],
        },
        "IL" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0495 }, // 4.95%
            has_local_income_tax: false,
            notable_features: vec![],
        },
        "IN" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0315 }, // 3.15%
            has_local_income_tax: true,                     // County income taxes
            notable_features: vec!["County income taxes vary".to_string()],
        },
        "KY" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0450 }, // 4.50%
            has_local_income_tax: true,                     // Some cities
            notable_features: vec![],
        },
        "MA" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0500 }, // 5.00%
            has_local_income_tax: false,
            notable_features: vec![],
        },
        "MI" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0425 }, // 4.25%
            has_local_income_tax: true,                     // Detroit and others
            notable_features: vec![],
        },
        "NC" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0475 }, // 4.75%
            has_local_income_tax: false,
            notable_features: vec![],
        },
        "PA" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0307 }, // 3.07%
            has_local_income_tax: true,                     // Philadelphia, Pittsburgh
            notable_features: vec!["Philadelphia: 3.79% local tax".to_string()],
        },
        "UT" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Flat { rate: 0.0485 }, // 4.85%
            has_local_income_tax: false,
            notable_features: vec![],
        },

        // Progressive tax states (California - highest rates)
        "CA" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Progressive {
                brackets: vec![
                    TaxBracket {
                        threshold: 0,
                        rate: 0.0100,
                    },
                    TaxBracket {
                        threshold: 10_412,
                        rate: 0.0200,
                    },
                    TaxBracket {
                        threshold: 24_684,
                        rate: 0.0400,
                    },
                    TaxBracket {
                        threshold: 38_959,
                        rate: 0.0600,
                    },
                    TaxBracket {
                        threshold: 54_081,
                        rate: 0.0800,
                    },
                    TaxBracket {
                        threshold: 68_350,
                        rate: 0.0930,
                    },
                    TaxBracket {
                        threshold: 349_137,
                        rate: 0.1030,
                    },
                    TaxBracket {
                        threshold: 418_961,
                        rate: 0.1130,
                    },
                    TaxBracket {
                        threshold: 698_271,
                        rate: 0.1230,
                    },
                    TaxBracket {
                        threshold: 1_000_000,
                        rate: 0.1330,
                    },
                ],
                top_rate: 0.1330, // 13.3% - highest in US
            },
            has_local_income_tax: false,
            notable_features: vec!["Highest top rate in US (13.3%)".to_string()],
        },

        // New York
        "NY" => IncomeTaxStructure {
            state_id,
            tax_type: IncomeTaxType::Progressive {
                brackets: vec![
                    TaxBracket {
                        threshold: 0,
                        rate: 0.0400,
                    },
                    TaxBracket {
                        threshold: 8_500,
                        rate: 0.0450,
                    },
                    TaxBracket {
                        threshold: 11_700,
                        rate: 0.0525,
                    },
                    TaxBracket {
                        threshold: 13_900,
                        rate: 0.0585,
                    },
                    TaxBracket {
                        threshold: 80_650,
                        rate: 0.0625,
                    },
                    TaxBracket {
                        threshold: 215_400,
                        rate: 0.0685,
                    },
                    TaxBracket {
                        threshold: 1_077_550,
                        rate: 0.0965,
                    },
                    TaxBracket {
                        threshold: 5_000_000,
                        rate: 0.1030,
                    },
                    TaxBracket {
                        threshold: 25_000_000,
                        rate: 0.1090,
                    },
                ],
                top_rate: 0.1090, // 10.9%
            },
            has_local_income_tax: true, // NYC: up to 3.876%
            notable_features: vec!["NYC adds up to 3.876% local tax".to_string()],
        },

        // Other progressive states (simplified - most common pattern)
        _ => {
            // Default progressive structure for remaining states
            let (brackets, top_rate) = match state_code {
                "HI" => (
                    vec![
                        TaxBracket {
                            threshold: 0,
                            rate: 0.0140,
                        },
                        TaxBracket {
                            threshold: 2_400,
                            rate: 0.0320,
                        },
                        TaxBracket {
                            threshold: 4_800,
                            rate: 0.0550,
                        },
                        TaxBracket {
                            threshold: 9_600,
                            rate: 0.0640,
                        },
                        TaxBracket {
                            threshold: 14_400,
                            rate: 0.0680,
                        },
                        TaxBracket {
                            threshold: 19_200,
                            rate: 0.0720,
                        },
                        TaxBracket {
                            threshold: 24_000,
                            rate: 0.0760,
                        },
                        TaxBracket {
                            threshold: 36_000,
                            rate: 0.0790,
                        },
                        TaxBracket {
                            threshold: 48_000,
                            rate: 0.0825,
                        },
                        TaxBracket {
                            threshold: 150_000,
                            rate: 0.0900,
                        },
                        TaxBracket {
                            threshold: 175_000,
                            rate: 0.1000,
                        },
                        TaxBracket {
                            threshold: 200_000,
                            rate: 0.1100,
                        },
                    ],
                    0.1100,
                ),

                // Most states have 3-5 brackets with top rates 4-7%
                _ => (
                    vec![
                        TaxBracket {
                            threshold: 0,
                            rate: 0.0200,
                        },
                        TaxBracket {
                            threshold: 10_000,
                            rate: 0.0400,
                        },
                        TaxBracket {
                            threshold: 25_000,
                            rate: 0.0500,
                        },
                        TaxBracket {
                            threshold: 50_000,
                            rate: 0.0575,
                        },
                    ],
                    0.0575,
                ),
            };

            IncomeTaxStructure {
                state_id,
                tax_type: IncomeTaxType::Progressive { brackets, top_rate },
                has_local_income_tax: false,
                notable_features: vec![],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_income_tax_states() {
        // 9 states with no income tax
        assert!(!has_state_income_tax("TX"));
        assert!(!has_state_income_tax("FL"));
        assert!(!has_state_income_tax("WA"));
        assert!(!has_state_income_tax("NV"));
        assert!(!has_state_income_tax("WY"));
        assert!(!has_state_income_tax("SD"));
        assert!(!has_state_income_tax("AK"));
        assert!(!has_state_income_tax("TN"));
        assert!(!has_state_income_tax("NH"));
    }

    #[test]
    fn test_has_income_tax_states() {
        // States with income tax
        assert!(has_state_income_tax("CA"));
        assert!(has_state_income_tax("NY"));
        assert!(has_state_income_tax("IL"));
        assert!(has_state_income_tax("PA"));
    }

    #[test]
    fn test_no_income_tax_states_list() {
        let no_tax = no_income_tax_states();
        assert_eq!(no_tax.len(), 9);
        assert!(no_tax.contains(&"TX"));
        assert!(no_tax.contains(&"FL"));
    }

    #[test]
    fn test_california_progressive_structure() {
        let ca = income_tax_structure("CA");
        assert!(matches!(ca.tax_type, IncomeTaxType::Progressive { .. }));

        if let IncomeTaxType::Progressive {
            top_rate, brackets, ..
        } = ca.tax_type
        {
            assert_eq!(top_rate, 0.1330); // 13.3%
            assert!(!brackets.is_empty());
        }
    }

    #[test]
    fn test_flat_tax_states() {
        let il = income_tax_structure("IL");
        assert!(matches!(il.tax_type, IncomeTaxType::Flat { rate: 0.0495 }));

        let co = income_tax_structure("CO");
        assert!(matches!(co.tax_type, IncomeTaxType::Flat { rate: 0.0440 }));

        let pa = income_tax_structure("PA");
        assert!(matches!(pa.tax_type, IncomeTaxType::Flat { rate: 0.0307 }));
    }

    #[test]
    fn test_local_income_tax_states() {
        let ny = income_tax_structure("NY");
        assert!(ny.has_local_income_tax); // NYC

        let pa = income_tax_structure("PA");
        assert!(pa.has_local_income_tax); // Philadelphia

        let tx = income_tax_structure("TX");
        assert!(!tx.has_local_income_tax); // No income tax at all
    }

    #[test]
    fn test_california_highest_rate() {
        let ca = income_tax_structure("CA");

        if let IncomeTaxType::Progressive { top_rate, .. } = ca.tax_type {
            // California has highest state income tax rate in US
            assert_eq!(top_rate, 0.1330);

            // Compare with other high-tax states
            let ny = income_tax_structure("NY");
            if let IncomeTaxType::Progressive {
                top_rate: ny_rate, ..
            } = ny.tax_type
            {
                assert!(top_rate > ny_rate);
            }
        }
    }

    #[test]
    fn test_texas_no_income_tax() {
        let tx = income_tax_structure("TX");
        assert!(matches!(tx.tax_type, IncomeTaxType::None));
        assert!(!tx.has_local_income_tax);
    }

    #[test]
    fn test_count_flat_vs_progressive() {
        let all_states = vec![
            "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN",
            "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV",
            "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN",
            "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY", "DC",
        ];

        let mut no_tax_count = 0;
        let mut flat_count = 0;
        let mut progressive_count = 0;

        for state in all_states {
            let structure = income_tax_structure(state);
            match structure.tax_type {
                IncomeTaxType::None => no_tax_count += 1,
                IncomeTaxType::Flat { .. } => flat_count += 1,
                IncomeTaxType::Progressive { .. } => progressive_count += 1,
            }
        }

        assert_eq!(no_tax_count, 9); // 9 no-tax states
        assert_eq!(flat_count, 9); // 9 flat-tax states
        assert!(progressive_count >= 30); // Majority progressive
    }
}
