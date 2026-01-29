//! Islamic Finance (Islamic Banking Act 1983)
//!
//! Syariah-compliant financial products and services in Malaysia.
//!
//! # Key Principles
//!
//! - **No Riba (Interest)**: Prohibition of interest
//! - **No Gharar (Uncertainty)**: Prohibition of excessive uncertainty
//! - **No Maysir (Gambling)**: Prohibition of gambling/speculation
//! - **Asset-backed**: Transactions must be backed by real assets
//! - **Profit-sharing**: Risk and profit sharing encouraged
//!
//! # Common Islamic Finance Contracts
//!
//! - **Murabahah**: Cost-plus financing
//! - **Ijarah**: Leasing
//! - **Musharakah**: Partnership/joint venture
//! - **Mudarabah**: Profit-sharing investment
//! - **Wakalah**: Agency
//! - **Takaful**: Islamic insurance

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Islamic finance error types.
#[derive(Debug, Error)]
pub enum IslamicFinanceError {
    /// Riba (interest) detected.
    #[error("Riba (interest) is prohibited in Islamic finance")]
    RibaDetected,

    /// Gharar (excessive uncertainty) detected.
    #[error("Gharar (excessive uncertainty) detected: {description}")]
    GhararDetected { description: String },

    /// Maysir (gambling) detected.
    #[error("Maysir (gambling/speculation) is prohibited")]
    MaysirDetected,

    /// Not Syariah-compliant.
    #[error("Product/contract is not Syariah-compliant: {reason}")]
    NotSyariahCompliant { reason: String },
}

/// Result type for Islamic finance operations.
pub type Result<T> = std::result::Result<T, IslamicFinanceError>;

/// Type of Islamic finance product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IslamicFinanceType {
    /// Murabahah (cost-plus financing).
    Murabahah,
    /// Ijarah (leasing).
    Ijarah,
    /// Musharakah (partnership).
    Musharakah,
    /// Mudarabah (profit-sharing).
    Mudarabah,
    /// Wakalah (agency).
    Wakalah,
    /// Takaful (Islamic insurance).
    Takaful,
    /// Sukuk (Islamic bond).
    Sukuk,
}

impl IslamicFinanceType {
    /// Returns a description of the Islamic finance type.
    #[must_use]
    pub fn description(self) -> &'static str {
        match self {
            Self::Murabahah => {
                "Cost-plus financing - Bank purchases asset and sells to customer at cost + profit margin"
            }
            Self::Ijarah => "Leasing - Bank owns asset and leases it to customer",
            Self::Musharakah => "Partnership - Joint venture with profit/loss sharing",
            Self::Mudarabah => {
                "Profit-sharing - One party provides capital, other provides expertise"
            }
            Self::Wakalah => "Agency - Agent acts on behalf of principal",
            Self::Takaful => "Islamic insurance - Cooperative risk-sharing",
            Self::Sukuk => "Islamic bond - Asset-backed securities",
        }
    }
}

/// Islamic finance product.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IslamicFinanceProduct {
    /// Product ID.
    pub id: Uuid,
    /// Product name.
    pub name: String,
    /// Product type.
    pub product_type: IslamicFinanceType,
    /// Principal amount in sen.
    pub principal_sen: i64,
    /// Profit/rental amount in sen (not interest).
    pub profit_sen: i64,
    /// Underlying asset description.
    pub underlying_asset: String,
    /// Whether product has been certified Syariah-compliant.
    pub syariah_certified: bool,
    /// Syariah advisor/committee approval.
    pub syariah_advisor: Option<String>,
}

impl IslamicFinanceProduct {
    /// Creates a new Islamic finance product.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        product_type: IslamicFinanceType,
        principal_sen: i64,
        profit_sen: i64,
        underlying_asset: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            product_type,
            principal_sen,
            profit_sen,
            underlying_asset: underlying_asset.into(),
            syariah_certified: false,
            syariah_advisor: None,
        }
    }

    /// Sets Syariah certification.
    #[must_use]
    pub fn with_syariah_certification(mut self, advisor: impl Into<String>) -> Self {
        self.syariah_certified = true;
        self.syariah_advisor = Some(advisor.into());
        self
    }

    /// Validates Syariah compliance.
    pub fn validate(&self) -> Result<ComplianceReport> {
        validate_shariah_compliance_product(self)
    }
}

/// Islamic contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IslamicContract {
    /// Contract ID.
    pub id: Uuid,
    /// Contract type.
    pub contract_type: IslamicFinanceType,
    /// Parties to the contract.
    pub parties: Vec<String>,
    /// Contract terms.
    pub terms: Vec<String>,
    /// Date of contract.
    pub date: DateTime<Utc>,
    /// Whether contract is Syariah-compliant.
    pub syariah_compliant: bool,
}

impl IslamicContract {
    /// Creates a new Islamic contract.
    #[must_use]
    pub fn new(contract_type: IslamicFinanceType, parties: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            contract_type,
            parties,
            terms: Vec::new(),
            date: Utc::now(),
            syariah_compliant: false,
        }
    }

    /// Adds a term to the contract.
    #[must_use]
    pub fn add_term(mut self, term: impl Into<String>) -> Self {
        self.terms.push(term.into());
        self
    }

    /// Sets Syariah compliance.
    #[must_use]
    pub fn with_syariah_compliance(mut self, compliant: bool) -> Self {
        self.syariah_compliant = compliant;
        self
    }
}

/// Syariah compliance report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Whether product is Syariah-compliant.
    pub compliant: bool,
    /// Issues found.
    pub issues: Vec<String>,
    /// Recommendations.
    pub recommendations: Vec<String>,
}

/// Validates Syariah compliance of an Islamic finance product.
pub fn validate_shariah_compliance_product(
    product: &IslamicFinanceProduct,
) -> Result<ComplianceReport> {
    let mut issues = Vec::new();
    let mut recommendations = Vec::new();

    // Check if product has underlying asset (no asset = potential riba)
    if product.underlying_asset.is_empty() {
        issues.push("Islamic finance product must have an underlying asset".to_string());
    }

    // Check for Syariah certification
    if !product.syariah_certified {
        recommendations
            .push("Product should be certified by a Syariah advisor or committee".to_string());
    }

    // Check for profit ratio reasonableness
    if product.profit_sen > product.principal_sen {
        issues.push(
            "Profit margin appears excessive and may indicate gharar (uncertainty)".to_string(),
        );
    }

    let compliant = issues.is_empty();

    Ok(ComplianceReport {
        compliant,
        issues,
        recommendations,
    })
}

/// Validates Syariah compliance of a contract or transaction.
pub fn validate_shariah_compliance(contract: &IslamicContract) -> Result<()> {
    // Check for prohibited elements
    for term in &contract.terms {
        let term_lower = term.to_lowercase();

        // Check for riba (interest)
        if term_lower.contains("interest") || term_lower.contains("riba") {
            return Err(IslamicFinanceError::RibaDetected);
        }

        // Check for gharar (excessive uncertainty)
        if term_lower.contains("gambling") || term_lower.contains("speculation") {
            return Err(IslamicFinanceError::MaysirDetected);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_murabahah_product() {
        let product = IslamicFinanceProduct::new(
            "Home Financing",
            IslamicFinanceType::Murabahah,
            50000000, // RM 500,000 principal
            5000000,  // RM 50,000 profit
            "Residential property at Kuala Lumpur",
        )
        .with_syariah_certification("Syariah Advisory Council");

        let report = product.validate().expect("Validation succeeds");
        assert!(report.compliant);
    }

    #[test]
    fn test_invalid_product_no_asset() {
        let product = IslamicFinanceProduct::new(
            "Invalid Product",
            IslamicFinanceType::Murabahah,
            10000000,
            1000000,
            "", // No underlying asset
        );

        let report = product.validate().expect("Validation succeeds");
        assert!(!report.compliant);
    }

    #[test]
    fn test_islamic_contract() {
        let contract = IslamicContract::new(
            IslamicFinanceType::Ijarah,
            vec!["Bank Islam".to_string(), "Ahmad bin Ali".to_string()],
        )
        .add_term("Monthly rental of RM 2,000")
        .add_term("Lease period: 5 years")
        .with_syariah_compliance(true);

        assert_eq!(contract.contract_type, IslamicFinanceType::Ijarah);
        assert!(contract.syariah_compliant);
    }

    #[test]
    fn test_riba_detection() {
        let contract = IslamicContract::new(
            IslamicFinanceType::Murabahah,
            vec!["Bank".to_string(), "Customer".to_string()],
        )
        .add_term("Interest rate of 5% per annum"); // Contains "interest"

        let result = validate_shariah_compliance(&contract);
        assert!(result.is_err());
    }
}
