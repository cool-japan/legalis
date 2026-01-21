//! Core tax types

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Australian Financial Year
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FinancialYear {
    /// 2023-24 financial year (1 July 2023 - 30 June 2024)
    FY2023_24,
    /// 2024-25 financial year (1 July 2024 - 30 June 2025)
    FY2024_25,
    /// 2025-26 financial year (1 July 2025 - 30 June 2026)
    FY2025_26,
    /// Other financial year
    Other {
        /// Start year
        start_year: u16,
    },
}

impl FinancialYear {
    /// Get the start date of the financial year
    pub fn start_date(&self) -> NaiveDate {
        let year = match self {
            FinancialYear::FY2023_24 => 2023,
            FinancialYear::FY2024_25 => 2024,
            FinancialYear::FY2025_26 => 2025,
            FinancialYear::Other { start_year } => *start_year as i32,
        };
        NaiveDate::from_ymd_opt(year, 7, 1).unwrap_or_default()
    }

    /// Get the end date of the financial year
    pub fn end_date(&self) -> NaiveDate {
        let year = match self {
            FinancialYear::FY2023_24 => 2024,
            FinancialYear::FY2024_25 => 2025,
            FinancialYear::FY2025_26 => 2026,
            FinancialYear::Other { start_year } => (*start_year + 1) as i32,
        };
        NaiveDate::from_ymd_opt(year, 6, 30).unwrap_or_default()
    }

    /// Get the label for the financial year
    pub fn label(&self) -> String {
        match self {
            FinancialYear::FY2023_24 => "2023-24".to_string(),
            FinancialYear::FY2024_25 => "2024-25".to_string(),
            FinancialYear::FY2025_26 => "2025-26".to_string(),
            FinancialYear::Other { start_year } => format!("{}-{}", start_year, start_year + 1),
        }
    }
}

/// Tax File Number (TFN)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaxFileNumber {
    /// The 9-digit TFN
    number: String,
}

impl TaxFileNumber {
    /// Create a new TFN (validates format)
    pub fn new(number: impl Into<String>) -> Option<Self> {
        let number = number.into().replace([' ', '-'], "");
        if number.len() == 9 && number.chars().all(|c| c.is_ascii_digit()) {
            // In reality, TFN has a check digit algorithm
            Some(Self { number })
        } else {
            None
        }
    }

    /// Get the TFN as a formatted string
    pub fn formatted(&self) -> String {
        format!(
            "{} {} {}",
            &self.number[0..3],
            &self.number[3..6],
            &self.number[6..9]
        )
    }
}

/// Australian Business Number (ABN)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Abn {
    /// The 11-digit ABN
    number: String,
}

impl Abn {
    /// Create a new ABN (validates format and check digit)
    pub fn new(number: impl Into<String>) -> Option<Self> {
        let number = number.into().replace([' ', '-'], "");
        if number.len() == 11
            && number.chars().all(|c| c.is_ascii_digit())
            && Self::validate_checksum(&number)
        {
            Some(Self { number })
        } else {
            None
        }
    }

    /// Validate ABN checksum
    fn validate_checksum(abn: &str) -> bool {
        let weights = [10, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
        let digits: Vec<u32> = abn.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() != 11 {
            return false;
        }

        // Subtract 1 from first digit
        let mut adjusted = digits.clone();
        adjusted[0] = adjusted[0].saturating_sub(1);

        // Calculate weighted sum
        let sum: u32 = adjusted
            .iter()
            .zip(weights.iter())
            .map(|(d, w)| d * w)
            .sum();

        sum.is_multiple_of(89)
    }

    /// Get the ABN as a formatted string
    pub fn formatted(&self) -> String {
        format!(
            "{} {} {} {}",
            &self.number[0..2],
            &self.number[2..5],
            &self.number[5..8],
            &self.number[8..11]
        )
    }
}

/// Entity type for tax purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    /// Individual/sole trader
    Individual,
    /// Company
    Company,
    /// Partnership
    Partnership,
    /// Trust
    Trust,
    /// Self-managed super fund (SMSF)
    Smsf,
    /// Superannuation fund
    SuperFund,
    /// Government entity
    Government,
    /// Non-profit organization
    NonProfit,
}

impl EntityType {
    /// Get the company tax rate for this entity type (2024-25)
    pub fn company_tax_rate(&self) -> Option<f64> {
        match self {
            EntityType::Company => Some(0.30), // Default to 30%
            EntityType::SuperFund | EntityType::Smsf => Some(0.15),
            EntityType::NonProfit => Some(0.0), // Tax exempt
            _ => None,
        }
    }

    /// Whether this entity is required to have an ABN
    pub fn requires_abn(&self) -> bool {
        matches!(
            self,
            EntityType::Company
                | EntityType::Partnership
                | EntityType::Trust
                | EntityType::Smsf
                | EntityType::SuperFund
        )
    }
}

/// Tax payer details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxPayer {
    /// Name
    pub name: String,
    /// Entity type
    pub entity_type: EntityType,
    /// TFN
    pub tfn: Option<TaxFileNumber>,
    /// ABN (if applicable)
    pub abn: Option<Abn>,
    /// Registered for GST
    pub gst_registered: bool,
    /// GST registration details
    pub gst_registration: Option<GstRegistration>,
    /// Resident for tax purposes
    pub is_resident: bool,
}

/// GST registration details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GstRegistration {
    /// Registration date
    pub registration_date: NaiveDate,
    /// Accounting method
    pub accounting_method: GstAccountingMethod,
    /// BAS reporting period
    pub reporting_period: BasReportingPeriod,
    /// Annual turnover threshold met
    pub turnover_above_threshold: bool,
    /// Voluntarily registered
    pub voluntary_registration: bool,
}

/// GST accounting method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GstAccountingMethod {
    /// Cash basis (recognize on receipt/payment)
    Cash,
    /// Accruals basis (recognize on invoice)
    Accruals,
}

/// BAS reporting period
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BasReportingPeriod {
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Annually (small business)
    Annually,
}

impl BasReportingPeriod {
    /// Get the number of periods per year
    pub fn periods_per_year(&self) -> u8 {
        match self {
            BasReportingPeriod::Monthly => 12,
            BasReportingPeriod::Quarterly => 4,
            BasReportingPeriod::Annually => 1,
        }
    }
}

/// Tax agent details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxAgent {
    /// Agent name
    pub name: String,
    /// Tax agent number
    pub agent_number: String,
    /// Registered with Tax Practitioners Board
    pub tpb_registered: bool,
    /// Registration expiry
    pub registration_expiry: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_year_dates() {
        let fy = FinancialYear::FY2024_25;
        assert_eq!(
            fy.start_date(),
            NaiveDate::from_ymd_opt(2024, 7, 1).unwrap()
        );
        assert_eq!(fy.end_date(), NaiveDate::from_ymd_opt(2025, 6, 30).unwrap());
        assert_eq!(fy.label(), "2024-25");
    }

    #[test]
    fn test_tfn_creation() {
        let tfn = TaxFileNumber::new("123456789");
        assert!(tfn.is_some());
        assert_eq!(tfn.unwrap().formatted(), "123 456 789");

        let invalid = TaxFileNumber::new("12345");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_abn_creation_and_validation() {
        // Example valid ABN: 51 824 753 556
        let abn = Abn::new("51824753556");
        assert!(abn.is_some());

        // Invalid checksum
        let invalid = Abn::new("12345678901");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_entity_type_tax_rate() {
        assert_eq!(EntityType::Company.company_tax_rate(), Some(0.30));
        assert_eq!(EntityType::SuperFund.company_tax_rate(), Some(0.15));
        assert_eq!(EntityType::Individual.company_tax_rate(), None);
    }

    #[test]
    fn test_bas_reporting_periods() {
        assert_eq!(BasReportingPeriod::Monthly.periods_per_year(), 12);
        assert_eq!(BasReportingPeriod::Quarterly.periods_per_year(), 4);
        assert_eq!(BasReportingPeriod::Annually.periods_per_year(), 1);
    }
}
