//! State Corporate Tax Analysis
//!
//! This module tracks corporate income tax rates, apportionment formulas,
//! and tax haven status across US jurisdictions.
//!
//! # Corporate Tax Havens
//!
//! Three states are known as **corporate tax havens**:
//! - **Delaware**: Most US corporations incorporate here (business-friendly courts)
//! - **Nevada**: No corporate income tax, strong privacy protections
//! - **Wyoming**: No corporate income tax, low fees, privacy
//!
//! # No Corporate Income Tax
//!
//! **6 states** have no corporate income tax:
//! - Nevada (NV)
//! - South Dakota (SD)
//! - Wyoming (WY)
//! - Washington (WA) - has B&O tax instead
//! - Texas (TX) - has franchise tax instead
//! - Ohio (OH) - has CAT (Commercial Activity Tax) instead

use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// Corporate tax information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateTaxInfo {
    /// State identifier
    pub state_id: StateId,
    /// Corporate income tax rate (decimal)
    pub tax_rate: f64,
    /// Alternative minimum tax rate
    pub alt_minimum_tax: Option<f64>,
    /// Apportionment formula used
    pub apportionment: ApportionmentFormula,
    /// Combined reporting required
    pub combined_reporting: bool,
    /// Tax haven status
    pub tax_haven_status: TaxHavenStatus,
    /// Alternative taxes (if no corporate income tax)
    pub alternative_taxes: Vec<String>,
}

/// Apportionment formula for multi-state corporations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApportionmentFormula {
    /// Single-factor: 100% sales (most common modern approach)
    SingleFactorSales,
    /// Three-factor: property, payroll, sales (traditional)
    ThreeFactorEqual,
    /// Three-factor with double-weighted sales
    ThreeFactorDoubleWeightedSales,
    /// Custom formula
    Custom {
        /// Property weight
        property: u8,
        /// Payroll weight
        payroll: u8,
        /// Sales weight
        sales: u8,
    },
    /// Not applicable (no corporate income tax)
    NotApplicable,
}

/// Tax haven status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaxHavenStatus {
    /// Major tax haven (Delaware, Nevada, Wyoming)
    MajorTaxHaven {
        /// Reasons for tax haven status
        reasons: Vec<String>,
    },
    /// Minor tax haven (favorable but not primary)
    MinorTaxHaven,
    /// Not a tax haven
    None,
}

/// Get corporate tax rate for a state
pub fn corporate_tax_rate(state_code: &str) -> CorporateTaxInfo {
    let state_id = StateId::from_code(state_code);

    match state_code {
        // Major tax havens
        "DE" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0885, // 8.85%
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::SingleFactorSales,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::MajorTaxHaven {
                reasons: vec![
                    "Court of Chancery (business expertise)".to_string(),
                    "Flexible corporate laws".to_string(),
                    "Strong precedent library".to_string(),
                    "Privacy protections".to_string(),
                    "Over 60% of Fortune 500 incorporated here".to_string(),
                ],
            },
            alternative_taxes: vec![],
        },

        "NV" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0, // No corporate income tax
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::NotApplicable,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::MajorTaxHaven {
                reasons: vec![
                    "No corporate income tax".to_string(),
                    "No franchise tax".to_string(),
                    "Strong privacy protections".to_string(),
                    "No information sharing agreements".to_string(),
                ],
            },
            alternative_taxes: vec!["Modified Business Tax on payroll".to_string()],
        },

        "WY" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0, // No corporate income tax
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::NotApplicable,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::MajorTaxHaven {
                reasons: vec![
                    "No corporate income tax".to_string(),
                    "Low annual fees".to_string(),
                    "Strong privacy protections".to_string(),
                    "LLC-friendly laws".to_string(),
                ],
            },
            alternative_taxes: vec![],
        },

        // Other no-corporate-tax states
        "SD" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0,
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::NotApplicable,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::MinorTaxHaven,
            alternative_taxes: vec![],
        },

        "TX" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0, // No corporate income tax
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::NotApplicable,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::None,
            alternative_taxes: vec!["Franchise Tax (0.375% retail, 0.75% other)".to_string()],
        },

        "WA" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0, // No corporate income tax
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::NotApplicable,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::None,
            alternative_taxes: vec!["Business & Occupation (B&O) Tax (0.13% - 3.3%)".to_string()],
        },

        "OH" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0, // Eliminated corporate income tax in 2014
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::NotApplicable,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::None,
            alternative_taxes: vec!["Commercial Activity Tax (CAT) on gross receipts".to_string()],
        },

        // Highest rates
        "NJ" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.1150, // 11.5% - highest in US
            alt_minimum_tax: Some(0.0090),
            apportionment: ApportionmentFormula::SingleFactorSales,
            combined_reporting: true,
            tax_haven_status: TaxHavenStatus::None,
            alternative_taxes: vec![],
        },

        "PA" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0899, // 8.99%
            alt_minimum_tax: None,
            apportionment: ApportionmentFormula::SingleFactorSales,
            combined_reporting: false,
            tax_haven_status: TaxHavenStatus::None,
            alternative_taxes: vec![],
        },

        "CA" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0884,              // 8.84%
            alt_minimum_tax: Some(0.0088), // AMT
            apportionment: ApportionmentFormula::SingleFactorSales,
            combined_reporting: true,
            tax_haven_status: TaxHavenStatus::None,
            alternative_taxes: vec![],
        },

        "NY" => CorporateTaxInfo {
            state_id,
            tax_rate: 0.0625,              // 6.25% base rate (can be higher in NYC)
            alt_minimum_tax: Some(0.0015), // Fixed dollar minimum
            apportionment: ApportionmentFormula::SingleFactorSales,
            combined_reporting: true,
            tax_haven_status: TaxHavenStatus::None,
            alternative_taxes: vec!["NYC: additional 8.85% corporate tax".to_string()],
        },

        // Other states (typical)
        _ => {
            let (rate, combined, apportionment) = match state_code {
                "AL" => (
                    0.0650,
                    true,
                    ApportionmentFormula::ThreeFactorDoubleWeightedSales,
                ),
                "AK" => (0.0920, false, ApportionmentFormula::ThreeFactorEqual),
                "AZ" => (0.0490, false, ApportionmentFormula::SingleFactorSales),
                "AR" => (0.0540, true, ApportionmentFormula::SingleFactorSales),
                "CO" => (0.0446, true, ApportionmentFormula::SingleFactorSales),
                "CT" => (0.0750, true, ApportionmentFormula::SingleFactorSales),
                "DC" => (0.0875, true, ApportionmentFormula::SingleFactorSales),
                "FL" => (0.0550, false, ApportionmentFormula::SingleFactorSales),
                "GA" => (0.0575, false, ApportionmentFormula::SingleFactorSales),
                "HI" => (0.0640, false, ApportionmentFormula::ThreeFactorEqual),
                "ID" => (0.0600, false, ApportionmentFormula::SingleFactorSales),
                "IL" => (0.0950, true, ApportionmentFormula::SingleFactorSales),
                "IN" => (0.0490, false, ApportionmentFormula::SingleFactorSales),
                "IA" => (0.0550, true, ApportionmentFormula::SingleFactorSales),
                "KS" => (0.0400, true, ApportionmentFormula::ThreeFactorEqual),
                "KY" => (0.0450, true, ApportionmentFormula::SingleFactorSales),
                "LA" => (0.0450, false, ApportionmentFormula::SingleFactorSales),
                "ME" => (0.0889, true, ApportionmentFormula::SingleFactorSales),
                "MD" => (0.0825, true, ApportionmentFormula::SingleFactorSales),
                "MA" => (0.0800, true, ApportionmentFormula::SingleFactorSales),
                "MI" => (0.0600, false, ApportionmentFormula::SingleFactorSales),
                "MN" => (0.0990, true, ApportionmentFormula::SingleFactorSales),
                "MS" => (0.0500, false, ApportionmentFormula::ThreeFactorEqual),
                "MO" => (0.0400, false, ApportionmentFormula::SingleFactorSales),
                "MT" => (0.0675, false, ApportionmentFormula::ThreeFactorEqual),
                "NE" => (0.0540, false, ApportionmentFormula::ThreeFactorEqual),
                "NH" => (
                    0.0765,
                    false,
                    ApportionmentFormula::ThreeFactorDoubleWeightedSales,
                ),
                "NM" => (0.0490, true, ApportionmentFormula::SingleFactorSales),
                "NC" => (0.0250, false, ApportionmentFormula::SingleFactorSales),
                "ND" => (0.0189, false, ApportionmentFormula::ThreeFactorEqual),
                "OK" => (0.0400, false, ApportionmentFormula::SingleFactorSales),
                "OR" => (0.0675, false, ApportionmentFormula::SingleFactorSales),
                "RI" => (0.0700, false, ApportionmentFormula::SingleFactorSales),
                "SC" => (0.0500, false, ApportionmentFormula::SingleFactorSales),
                "TN" => (0.0650, false, ApportionmentFormula::SingleFactorSales),
                "UT" => (0.0485, false, ApportionmentFormula::SingleFactorSales),
                "VT" => (0.0600, false, ApportionmentFormula::SingleFactorSales),
                "VA" => (0.0600, true, ApportionmentFormula::SingleFactorSales),
                "WV" => (0.0650, true, ApportionmentFormula::SingleFactorSales),
                "WI" => (0.0790, true, ApportionmentFormula::SingleFactorSales),
                _ => (0.0500, false, ApportionmentFormula::ThreeFactorEqual),
            };

            CorporateTaxInfo {
                state_id,
                tax_rate: rate,
                alt_minimum_tax: None,
                apportionment,
                combined_reporting: combined,
                tax_haven_status: TaxHavenStatus::None,
                alternative_taxes: vec![],
            }
        }
    }
}

/// Check if state is a corporate tax haven
pub fn is_tax_haven(state_code: &str) -> bool {
    matches!(
        corporate_tax_rate(state_code).tax_haven_status,
        TaxHavenStatus::MajorTaxHaven { .. }
    )
}

/// Get apportionment formula for a state
pub fn apportionment_formula(state_code: &str) -> ApportionmentFormula {
    corporate_tax_rate(state_code).apportionment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delaware_tax_haven() {
        assert!(is_tax_haven("DE"));

        let de = corporate_tax_rate("DE");
        assert!(matches!(
            de.tax_haven_status,
            TaxHavenStatus::MajorTaxHaven { .. }
        ));

        if let TaxHavenStatus::MajorTaxHaven { reasons } = de.tax_haven_status {
            assert!(reasons.iter().any(|r| r.contains("Court of Chancery")));
            assert!(reasons.iter().any(|r| r.contains("Fortune 500")));
        }
    }

    #[test]
    fn test_nevada_wyoming_tax_havens() {
        assert!(is_tax_haven("NV"));
        assert!(is_tax_haven("WY"));

        let nv = corporate_tax_rate("NV");
        assert_eq!(nv.tax_rate, 0.0); // No corporate income tax

        let wy = corporate_tax_rate("WY");
        assert_eq!(wy.tax_rate, 0.0); // No corporate income tax
    }

    #[test]
    fn test_no_corporate_tax_states() {
        let no_tax_states = vec!["NV", "SD", "WY", "WA", "TX", "OH"];

        for state in no_tax_states {
            let info = corporate_tax_rate(state);
            assert_eq!(
                info.tax_rate, 0.0,
                "{} should have 0% corporate income tax",
                state
            );
        }
    }

    #[test]
    fn test_new_jersey_highest_rate() {
        let nj = corporate_tax_rate("NJ");
        assert_eq!(nj.tax_rate, 0.1150); // 11.5% - highest in US

        // Compare with other states
        let ca = corporate_tax_rate("CA");
        assert!(nj.tax_rate > ca.tax_rate);
    }

    #[test]
    fn test_single_factor_sales_common() {
        // Most states use single-factor sales apportionment
        let single_factor_states = vec!["CA", "NY", "TX", "FL", "IL", "PA"];

        for state in single_factor_states {
            let formula = apportionment_formula(state);
            assert!(
                matches!(
                    formula,
                    ApportionmentFormula::SingleFactorSales | ApportionmentFormula::NotApplicable
                ),
                "{} apportionment formula mismatch",
                state
            );
        }
    }

    #[test]
    fn test_combined_reporting_states() {
        // States with combined reporting
        let combined_states = vec!["CA", "NY", "IL", "MA"];

        for state in combined_states {
            let info = corporate_tax_rate(state);
            assert!(
                info.combined_reporting,
                "{} should require combined reporting",
                state
            );
        }
    }

    #[test]
    fn test_texas_franchise_tax() {
        let tx = corporate_tax_rate("TX");
        assert_eq!(tx.tax_rate, 0.0); // No corporate income tax
        assert!(!tx.alternative_taxes.is_empty()); // Has franchise tax
        assert!(tx.alternative_taxes[0].contains("Franchise"));
    }

    #[test]
    fn test_washington_bo_tax() {
        let wa = corporate_tax_rate("WA");
        assert_eq!(wa.tax_rate, 0.0); // No corporate income tax
        assert!(wa.alternative_taxes.iter().any(|t| t.contains("B&O")));
    }

    #[test]
    fn test_ohio_eliminated_corporate_tax() {
        let oh = corporate_tax_rate("OH");
        assert_eq!(oh.tax_rate, 0.0); // Eliminated in 2014
        assert!(oh.alternative_taxes.iter().any(|t| t.contains("CAT")));
    }

    #[test]
    fn test_delaware_not_zero_rate() {
        // Delaware is tax haven but still has corporate tax
        let de = corporate_tax_rate("DE");
        assert!(de.tax_rate > 0.0); // 8.85%
        assert!(is_tax_haven("DE")); // But still tax haven for other reasons
    }

    #[test]
    fn test_count_no_corporate_tax() {
        let all_states = vec![
            "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN",
            "IA", "KS", "KY", "LA", "ME", "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV",
            "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA", "RI", "SC", "SD", "TN",
            "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY", "DC",
        ];

        let no_tax_count = all_states
            .iter()
            .filter(|&&state| corporate_tax_rate(state).tax_rate == 0.0)
            .count();

        assert!(no_tax_count >= 6); // At least 6 states with no corporate income tax
    }
}
