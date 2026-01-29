//! Company Law of the Russian Federation.
//!
//! This module covers:
//! - Federal Law No. 14-FZ on Limited Liability Companies (LLC/ООО)
//! - Federal Law No. 208-FZ on Joint Stock Companies (JSC/АО)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to Company Law operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CompanyError {
    /// Invalid company structure
    #[error("Invalid company structure: {0}")]
    InvalidStructure(String),

    /// Invalid capital
    #[error("Invalid capital: {0}")]
    InvalidCapital(String),

    /// Invalid governance
    #[error("Invalid governance: {0}")]
    InvalidGovernance(String),

    /// Validation failed
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

/// Types of companies in Russia
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    /// Limited Liability Company (ООО)
    LLC,
    /// Joint Stock Company (АО)
    JSC,
    /// Public Joint Stock Company (ПАО)
    PublicJSC,
}

/// Limited Liability Company (LLC / ООО) representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitedLiabilityCompany {
    /// Company name (must end with "ООО" or equivalent)
    pub name: String,
    /// Registration number (OGRN)
    pub ogrn: String,
    /// Tax identification number (INN)
    pub inn: String,
    /// Authorized capital
    pub authorized_capital: crate::common::Currency,
    /// Founders and their contributions
    pub founders: Vec<FounderContribution>,
    /// Legal address
    pub legal_address: String,
    /// Governance structure
    pub governance: GovernanceStructure,
}

impl LimitedLiabilityCompany {
    /// Creates a new LLC
    pub fn new(
        name: impl Into<String>,
        ogrn: impl Into<String>,
        inn: impl Into<String>,
        authorized_capital: crate::common::Currency,
        legal_address: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            ogrn: ogrn.into(),
            inn: inn.into(),
            authorized_capital,
            founders: Vec::new(),
            legal_address: legal_address.into(),
            governance: GovernanceStructure::default(),
        }
    }

    /// Adds a founder
    pub fn add_founder(mut self, founder: FounderContribution) -> Self {
        self.founders.push(founder);
        self
    }

    /// Sets governance structure
    pub fn with_governance(mut self, governance: GovernanceStructure) -> Self {
        self.governance = governance;
        self
    }

    /// Validates the LLC according to Federal Law 14-FZ
    pub fn validate(&self) -> Result<(), CompanyError> {
        // Minimum authorized capital: 10,000 RUB (Article 14)
        let min_capital = crate::common::Currency::from_rubles(10_000);
        if self.authorized_capital.kopecks < min_capital.kopecks {
            return Err(CompanyError::InvalidCapital(
                "LLC authorized capital must be at least 10,000 RUB".to_string(),
            ));
        }

        // Maximum number of founders: 50 (Article 7)
        if self.founders.len() > 50 {
            return Err(CompanyError::InvalidStructure(
                "LLC cannot have more than 50 founders".to_string(),
            ));
        }

        // Minimum number of founders: 1
        if self.founders.is_empty() {
            return Err(CompanyError::InvalidStructure(
                "LLC must have at least one founder".to_string(),
            ));
        }

        // Verify total contributions equal authorized capital
        let total_contributions: i64 = self
            .founders
            .iter()
            .map(|f| f.contribution_amount.kopecks)
            .sum();

        if total_contributions != self.authorized_capital.kopecks {
            return Err(CompanyError::InvalidCapital(
                "Total founder contributions must equal authorized capital".to_string(),
            ));
        }

        Ok(())
    }
}

/// Joint Stock Company (JSC / АО) representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointStockCompany {
    /// Company name (must end with "АО" or "ПАО")
    pub name: String,
    /// Registration number (OGRN)
    pub ogrn: String,
    /// Tax identification number (INN)
    pub inn: String,
    /// Authorized capital (in shares)
    pub authorized_capital: crate::common::Currency,
    /// Number of shares
    pub total_shares: u64,
    /// Nominal value per share
    pub nominal_value_per_share: crate::common::Currency,
    /// Is public (ПАО) or non-public (АО)
    pub is_public: bool,
    /// Legal address
    pub legal_address: String,
    /// Shareholders
    pub shareholders: Vec<ShareholderRights>,
}

impl JointStockCompany {
    /// Creates a new JSC
    pub fn new(
        name: impl Into<String>,
        ogrn: impl Into<String>,
        inn: impl Into<String>,
        total_shares: u64,
        nominal_value_per_share: crate::common::Currency,
        is_public: bool,
        legal_address: impl Into<String>,
    ) -> Self {
        let authorized_capital = crate::common::Currency {
            kopecks: (total_shares as i64) * nominal_value_per_share.kopecks,
        };

        Self {
            name: name.into(),
            ogrn: ogrn.into(),
            inn: inn.into(),
            authorized_capital,
            total_shares,
            nominal_value_per_share,
            is_public,
            legal_address: legal_address.into(),
            shareholders: Vec::new(),
        }
    }

    /// Adds a shareholder
    pub fn add_shareholder(mut self, shareholder: ShareholderRights) -> Self {
        self.shareholders.push(shareholder);
        self
    }

    /// Validates the JSC according to Federal Law 208-FZ
    pub fn validate(&self) -> Result<(), CompanyError> {
        // Minimum authorized capital for public JSC: 100,000 RUB
        // Minimum authorized capital for non-public JSC: 10,000 RUB (Article 26)
        let min_capital = if self.is_public {
            crate::common::Currency::from_rubles(100_000)
        } else {
            crate::common::Currency::from_rubles(10_000)
        };

        if self.authorized_capital.kopecks < min_capital.kopecks {
            return Err(CompanyError::InvalidCapital(format!(
                "JSC authorized capital must be at least {} RUB",
                min_capital.rubles()
            )));
        }

        // Verify shares calculation
        let calculated_capital = crate::common::Currency {
            kopecks: (self.total_shares as i64) * self.nominal_value_per_share.kopecks,
        };

        if calculated_capital.kopecks != self.authorized_capital.kopecks {
            return Err(CompanyError::InvalidCapital(
                "Authorized capital must equal total shares × nominal value".to_string(),
            ));
        }

        Ok(())
    }
}

/// Founder contribution in LLC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderContribution {
    /// Founder name
    pub founder_name: String,
    /// Contribution amount
    pub contribution_amount: crate::common::Currency,
    /// Share percentage
    pub share_percentage: f64,
}

impl FounderContribution {
    /// Creates a new founder contribution
    pub fn new(
        founder_name: impl Into<String>,
        contribution_amount: crate::common::Currency,
        share_percentage: f64,
    ) -> Self {
        Self {
            founder_name: founder_name.into(),
            contribution_amount,
            share_percentage,
        }
    }
}

/// Shareholder rights in JSC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderRights {
    /// Shareholder name
    pub shareholder_name: String,
    /// Number of shares owned
    pub shares_owned: u64,
    /// Can vote in general meeting
    pub voting_rights: bool,
    /// Can receive dividends
    pub dividend_rights: bool,
}

/// Governance structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceStructure {
    /// Has general meeting
    pub general_meeting: bool,
    /// Has board of directors
    pub board_of_directors: bool,
    /// Has audit commission
    pub audit_commission: bool,
    /// Executive body (director, CEO)
    pub executive_body: String,
}

impl Default for GovernanceStructure {
    fn default() -> Self {
        Self {
            general_meeting: true,
            board_of_directors: false,
            audit_commission: false,
            executive_body: "Director".to_string(),
        }
    }
}

/// Quick validation for LLC
pub fn quick_validate_llc(llc: &LimitedLiabilityCompany) -> Result<(), CompanyError> {
    llc.validate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llc_creation() {
        let llc = LimitedLiabilityCompany::new(
            "ООО Компания",
            "1234567890123",
            "1234567890",
            crate::common::Currency::from_rubles(100_000),
            "Москва, ул. Ленина, 1",
        )
        .add_founder(FounderContribution::new(
            "Иванов И.И.",
            crate::common::Currency::from_rubles(60_000),
            60.0,
        ))
        .add_founder(FounderContribution::new(
            "Петров П.П.",
            crate::common::Currency::from_rubles(40_000),
            40.0,
        ));

        assert!(llc.validate().is_ok());
    }

    #[test]
    fn test_llc_minimum_capital() {
        let llc = LimitedLiabilityCompany::new(
            "ООО Малая",
            "1234567890123",
            "1234567890",
            crate::common::Currency::from_rubles(5_000),
            "Москва",
        )
        .add_founder(FounderContribution::new(
            "Иванов И.И.",
            crate::common::Currency::from_rubles(5_000),
            100.0,
        ));

        assert!(llc.validate().is_err());
    }

    #[test]
    fn test_jsc_creation() {
        let jsc = JointStockCompany::new(
            "АО Компания",
            "1234567890123",
            "1234567890",
            1000,
            crate::common::Currency::from_rubles(100),
            false,
            "Москва",
        );

        assert!(jsc.validate().is_ok());
    }

    #[test]
    fn test_public_jsc_minimum_capital() {
        let jsc = JointStockCompany::new(
            "ПАО Компания",
            "1234567890123",
            "1234567890",
            500,
            crate::common::Currency::from_rubles(100),
            true,
            "Москва",
        );

        assert!(jsc.validate().is_err()); // Only 50,000 RUB, need 100,000
    }
}
