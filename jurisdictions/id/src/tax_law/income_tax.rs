//! Income Tax (PPh - Pajak Penghasilan)
//!
//! ## Overview
//!
//! Income tax in Indonesia is governed by:
//! - UU No. 7/1983 as last amended by UU No. 7/2021 (UU HPP)
//! - Progressive rates for individuals (5% to 35%)
//! - Flat rate for corporations (22%)
//!
//! ## Types of Income Tax (PPh Pasal)
//!
//! - **PPh Pasal 21**: Withholding tax on employment income
//! - **PPh Pasal 22**: Collection tax on imports and certain transactions
//! - **PPh Pasal 23**: Withholding tax on dividends, interest, royalties, services
//! - **PPh Pasal 24**: Foreign tax credit
//! - **PPh Pasal 25**: Monthly installments
//! - **PPh Pasal 26**: Withholding tax on foreign entities
//! - **PPh Pasal 29**: Annual tax reconciliation
//! - **PPh Pasal 4(2)**: Final tax on certain income

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Tax subject type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TaxSubject {
    /// Individual resident taxpayer (WPOP dalam negeri)
    IndividualResident,
    /// Individual non-resident taxpayer (WPOP luar negeri)
    IndividualNonResident,
    /// Corporate taxpayer (Badan dalam negeri)
    CorporateResident,
    /// Corporate non-resident taxpayer (Badan luar negeri)
    CorporateNonResident,
    /// Permanent establishment (BUT - Bentuk Usaha Tetap)
    PermanentEstablishment,
}

impl TaxSubject {
    /// Get subject type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::IndividualResident => "Wajib Pajak Orang Pribadi Dalam Negeri",
            Self::IndividualNonResident => "Wajib Pajak Orang Pribadi Luar Negeri",
            Self::CorporateResident => "Wajib Pajak Badan Dalam Negeri",
            Self::CorporateNonResident => "Wajib Pajak Badan Luar Negeri",
            Self::PermanentEstablishment => "Bentuk Usaha Tetap (BUT)",
        }
    }

    /// Get subject type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::IndividualResident => "Individual Resident Taxpayer",
            Self::IndividualNonResident => "Individual Non-Resident Taxpayer",
            Self::CorporateResident => "Corporate Resident Taxpayer",
            Self::CorporateNonResident => "Corporate Non-Resident Taxpayer",
            Self::PermanentEstablishment => "Permanent Establishment",
        }
    }

    /// Check if subject is resident
    pub fn is_resident(&self) -> bool {
        matches!(
            self,
            Self::IndividualResident | Self::CorporateResident | Self::PermanentEstablishment
        )
    }
}

/// Individual income tax brackets (UU HPP 2021) - Progressive rates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomeTaxBracket {
    /// Lower bound (inclusive)
    pub lower_bound: i64,
    /// Upper bound (exclusive, None for top bracket)
    pub upper_bound: Option<i64>,
    /// Tax rate as percentage
    pub rate_percent: f64,
}

impl IncomeTaxBracket {
    /// Get individual income tax brackets for 2022+ (UU HPP)
    pub fn individual_brackets_2022() -> Vec<Self> {
        vec![
            Self {
                lower_bound: 0,
                upper_bound: Some(60_000_000),
                rate_percent: 5.0,
            },
            Self {
                lower_bound: 60_000_000,
                upper_bound: Some(250_000_000),
                rate_percent: 15.0,
            },
            Self {
                lower_bound: 250_000_000,
                upper_bound: Some(500_000_000),
                rate_percent: 25.0,
            },
            Self {
                lower_bound: 500_000_000,
                upper_bound: Some(5_000_000_000),
                rate_percent: 30.0,
            },
            Self {
                lower_bound: 5_000_000_000,
                upper_bound: None,
                rate_percent: 35.0,
            },
        ]
    }

    /// Calculate tax for given taxable income using progressive brackets
    pub fn calculate_progressive_tax(taxable_income: i64) -> i64 {
        let brackets = Self::individual_brackets_2022();
        let mut total_tax = 0i64;
        let mut remaining_income = taxable_income;

        for bracket in brackets {
            if remaining_income <= 0 {
                break;
            }

            let bracket_size = match bracket.upper_bound {
                Some(upper) => {
                    if remaining_income > (upper - bracket.lower_bound) {
                        upper - bracket.lower_bound
                    } else {
                        remaining_income
                    }
                }
                None => remaining_income,
            };

            let tax_in_bracket =
                (bracket_size as f64 * bracket.rate_percent / 100.0).round() as i64;
            total_tax += tax_in_bracket;
            remaining_income -= bracket_size;
        }

        total_tax
    }
}

/// Corporate income tax rate
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CorporateTaxRate {
    /// Standard rate 22% (2022+)
    Standard22Percent,
    /// Listed company rate 20% (if ≥40% shares publicly traded)
    Listed20Percent,
    /// SME rate (reduced 50% for turnover < Rp 4.8 billion on first Rp 50 billion)
    SmeReducedRate,
}

impl CorporateTaxRate {
    /// Get rate as decimal
    pub fn rate_decimal(&self) -> f64 {
        match self {
            Self::Standard22Percent => 0.22,
            Self::Listed20Percent => 0.20,
            Self::SmeReducedRate => 0.11, // 50% x 22%
        }
    }

    /// Get rate as percentage
    pub fn rate_percent(&self) -> f64 {
        self.rate_decimal() * 100.0
    }
}

/// Non-taxable income (PTKP - Penghasilan Tidak Kena Pajak)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PtkpStatus {
    /// Single taxpayer (TK/0)
    Single,
    /// Married taxpayer (K/0)
    Married,
    /// Married with 1 dependent (K/1)
    MarriedOneDependentOne,
    /// Married with 2 dependents (K/2)
    MarriedWithTwoDependents,
    /// Married with 3 dependents (K/3)
    MarriedWithThreeDependents,
}

impl PtkpStatus {
    /// Get PTKP amount per year (2022+)
    pub fn annual_amount(&self) -> i64 {
        match self {
            Self::Single => 54_000_000,
            Self::Married => 58_500_000,
            Self::MarriedOneDependentOne => 63_000_000,
            Self::MarriedWithTwoDependents => 67_500_000,
            Self::MarriedWithThreeDependents => 72_000_000,
        }
    }

    /// Get status code
    pub fn status_code(&self) -> &str {
        match self {
            Self::Single => "TK/0",
            Self::Married => "K/0",
            Self::MarriedOneDependentOne => "K/1",
            Self::MarriedWithTwoDependents => "K/2",
            Self::MarriedWithThreeDependents => "K/3",
        }
    }
}

/// PPh Pasal 21 - Withholding tax on employment income
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pph21 {
    /// Employee name
    pub employee_name: String,
    /// Employee NPWP
    pub employee_npwp: Option<String>,
    /// Tax period (month)
    pub period: NaiveDate,
    /// Gross salary
    pub gross_salary: i64,
    /// Allowances
    pub allowances: i64,
    /// Position allowance (5% of gross, max Rp 500k/month, Rp 6M/year)
    pub position_allowance: i64,
    /// Pension contribution (deductible)
    pub pension_contribution: i64,
    /// PTKP status
    pub ptkp_status: PtkpStatus,
    /// Monthly tax withheld
    pub monthly_tax_withheld: i64,
}

impl Pph21 {
    /// Calculate position allowance (5% of gross, max Rp 500k/month)
    pub fn calculate_position_allowance(gross_salary: i64) -> i64 {
        let calculated = (gross_salary as f64 * 0.05).round() as i64;
        calculated.min(500_000)
    }

    /// Calculate annual taxable income
    pub fn calculate_annual_taxable_income(&self) -> i64 {
        let annual_gross = (self.gross_salary + self.allowances) * 12;
        let annual_position_allowance = self.position_allowance * 12;
        let annual_pension = self.pension_contribution * 12;

        let net_income = annual_gross - annual_position_allowance - annual_pension;
        let taxable_income = net_income - self.ptkp_status.annual_amount();

        taxable_income.max(0)
    }

    /// Calculate annual income tax
    pub fn calculate_annual_tax(&self) -> i64 {
        let taxable_income = self.calculate_annual_taxable_income();
        IncomeTaxBracket::calculate_progressive_tax(taxable_income)
    }

    /// Calculate monthly tax withholding
    pub fn calculate_monthly_withholding(&self) -> i64 {
        let annual_tax = self.calculate_annual_tax();
        annual_tax / 12
    }
}

/// PPh Pasal 23 - Withholding tax on dividends, interest, royalties, services
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pph23Type {
    /// Dividends (15%)
    Dividends,
    /// Interest (15%)
    Interest,
    /// Royalties (15%)
    Royalties,
    /// Rent (2%)
    Rent,
    /// Services (2%)
    Services,
}

impl Pph23Type {
    /// Get withholding rate as percentage
    pub fn rate_percent(&self) -> f64 {
        match self {
            Self::Dividends => 15.0,
            Self::Interest => 15.0,
            Self::Royalties => 15.0,
            Self::Rent => 2.0,
            Self::Services => 2.0,
        }
    }

    /// Get rate as decimal
    pub fn rate_decimal(&self) -> f64 {
        self.rate_percent() / 100.0
    }

    /// Get type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Dividends => "Dividen",
            Self::Interest => "Bunga",
            Self::Royalties => "Royalti",
            Self::Rent => "Sewa",
            Self::Services => "Jasa",
        }
    }

    /// Get type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Dividends => "Dividends",
            Self::Interest => "Interest",
            Self::Royalties => "Royalties",
            Self::Rent => "Rent",
            Self::Services => "Services",
        }
    }
}

/// PPh Pasal 26 - Withholding tax on foreign entities (20% or treaty rate)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pph26 {
    /// Payment type
    pub payment_type: Pph26PaymentType,
    /// Gross amount
    pub gross_amount: i64,
    /// Withholding rate (20% or treaty rate)
    pub rate_percent: f64,
    /// Tax withheld
    pub tax_withheld: i64,
    /// Recipient country
    pub recipient_country: String,
    /// Whether tax treaty applies
    pub tax_treaty_applies: bool,
}

/// PPh Pasal 26 payment types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pph26PaymentType {
    /// Dividends
    Dividends,
    /// Interest
    Interest,
    /// Royalties
    Royalties,
    /// Services
    Services,
    /// Branch profit remittance
    BranchProfit,
}

impl Pph26PaymentType {
    /// Get standard rate (20%) before treaty
    pub fn standard_rate_percent() -> f64 {
        20.0
    }

    /// Get type name in Indonesian
    pub fn name_id(&self) -> &str {
        match self {
            Self::Dividends => "Dividen",
            Self::Interest => "Bunga",
            Self::Royalties => "Royalti",
            Self::Services => "Jasa",
            Self::BranchProfit => "Laba Cabang",
        }
    }

    /// Get type name in English
    pub fn name_en(&self) -> &str {
        match self {
            Self::Dividends => "Dividends",
            Self::Interest => "Interest",
            Self::Royalties => "Royalties",
            Self::Services => "Services",
            Self::BranchProfit => "Branch Profit",
        }
    }
}

/// PPh Pasal 4(2) - Final tax on certain income
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Pph4ayat2Type {
    /// Interest on deposits (20%)
    DepositInterest,
    /// Interest on bonds (10% or 15%)
    BondInterest { rate_percent: u8 },
    /// Lottery winnings (25%)
    LotteryWinnings,
    /// Construction services (2%-4%)
    ConstructionServices { rate_percent: u8 },
    /// Property rental (10%)
    PropertyRental,
    /// Transfer of land and buildings (2.5%)
    PropertyTransfer,
}

impl Pph4ayat2Type {
    /// Get rate as percentage
    pub fn rate_percent(&self) -> f64 {
        match self {
            Self::DepositInterest => 20.0,
            Self::BondInterest { rate_percent } => *rate_percent as f64,
            Self::LotteryWinnings => 25.0,
            Self::ConstructionServices { rate_percent } => *rate_percent as f64,
            Self::PropertyRental => 10.0,
            Self::PropertyTransfer => 2.5,
        }
    }

    /// Get rate as decimal
    pub fn rate_decimal(&self) -> f64 {
        self.rate_percent() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax_subject() {
        let resident = TaxSubject::IndividualResident;
        assert!(resident.is_resident());

        let non_resident = TaxSubject::IndividualNonResident;
        assert!(!non_resident.is_resident());
    }

    #[test]
    fn test_income_tax_brackets() {
        // Test Rp 50 million (should be in first bracket at 5%)
        let tax = IncomeTaxBracket::calculate_progressive_tax(50_000_000);
        assert_eq!(tax, 2_500_000); // 5% of 50M

        // Test Rp 100 million (spans first two brackets)
        let tax = IncomeTaxBracket::calculate_progressive_tax(100_000_000);
        // First 60M at 5% = 3M
        // Next 40M at 15% = 6M
        // Total = 9M
        assert_eq!(tax, 9_000_000);
    }

    #[test]
    fn test_ptkp_status() {
        let single = PtkpStatus::Single;
        assert_eq!(single.annual_amount(), 54_000_000);
        assert_eq!(single.status_code(), "TK/0");

        let married_k3 = PtkpStatus::MarriedWithThreeDependents;
        assert_eq!(married_k3.annual_amount(), 72_000_000);
        assert_eq!(married_k3.status_code(), "K/3");
    }

    #[test]
    fn test_position_allowance() {
        let gross = 10_000_000;
        let allowance = Pph21::calculate_position_allowance(gross);
        assert_eq!(allowance, 500_000); // 5% = 500k, capped at 500k

        let gross_small = 5_000_000;
        let allowance_small = Pph21::calculate_position_allowance(gross_small);
        assert_eq!(allowance_small, 250_000); // 5% = 250k, under cap
    }

    #[test]
    fn test_corporate_tax_rate() {
        let standard = CorporateTaxRate::Standard22Percent;
        assert_eq!(standard.rate_percent(), 22.0);

        let listed = CorporateTaxRate::Listed20Percent;
        assert_eq!(listed.rate_percent(), 20.0);

        let sme = CorporateTaxRate::SmeReducedRate;
        assert_eq!(sme.rate_percent(), 11.0);
    }

    #[test]
    fn test_pph23_rates() {
        let dividends = Pph23Type::Dividends;
        assert_eq!(dividends.rate_percent(), 15.0);

        let services = Pph23Type::Services;
        assert_eq!(services.rate_percent(), 2.0);
    }

    #[test]
    fn test_pph26_standard_rate() {
        assert_eq!(Pph26PaymentType::standard_rate_percent(), 20.0);
    }

    #[test]
    fn test_pph4ayat2_rates() {
        let deposit = Pph4ayat2Type::DepositInterest;
        assert_eq!(deposit.rate_percent(), 20.0);

        let property_transfer = Pph4ayat2Type::PropertyTransfer;
        assert_eq!(property_transfer.rate_percent(), 2.5);
    }
}
