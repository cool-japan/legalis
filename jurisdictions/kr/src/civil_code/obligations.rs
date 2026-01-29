//! Obligations (채권법)
//!
//! # 채권법 / Law of Obligations
//!
//! Articles 373-766 (제373조 - 제766조)
//!
//! Covers:
//! - General provisions on obligations
//! - Contracts
//! - Management of affairs without mandate
//! - Unjust enrichment
//! - Torts

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Obligations errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ObligationsError {
    /// Invalid contract
    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    /// Breach of contract
    #[error("Breach of contract: {0}")]
    BreachOfContract(String),

    /// Tort error
    #[error("Tort error: {0}")]
    TortError(String),
}

/// Result type for obligations operations
pub type ObligationsResult<T> = Result<T, ObligationsError>;

/// Contract type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale (매매)
    Sale,
    /// Lease (임대차)
    Lease,
    /// Gift (증여)
    Gift,
    /// Loan (소비대차)
    Loan,
    /// Employment (고용)
    Employment,
    /// Service (도급)
    Service,
}

/// Contract
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    /// Contract type
    pub contract_type: ContractType,
    /// Party A
    pub party_a: String,
    /// Party B
    pub party_b: String,
    /// Contract date
    pub contract_date: NaiveDate,
    /// Subject matter
    pub subject_matter: String,
    /// Consideration (if applicable)
    pub consideration: Option<KrwAmount>,
}

impl Contract {
    /// Create new contract
    pub fn new(
        contract_type: ContractType,
        party_a: impl Into<String>,
        party_b: impl Into<String>,
        contract_date: NaiveDate,
        subject_matter: impl Into<String>,
    ) -> Self {
        Self {
            contract_type,
            party_a: party_a.into(),
            party_b: party_b.into(),
            contract_date,
            subject_matter: subject_matter.into(),
            consideration: None,
        }
    }

    /// Set consideration
    pub fn with_consideration(mut self, amount: KrwAmount) -> Self {
        self.consideration = Some(amount);
        self
    }
}

/// Validate contract formation
pub fn validate_contract(contract: &Contract) -> ObligationsResult<()> {
    if contract.party_a.is_empty() {
        return Err(ObligationsError::InvalidContract(
            "Party A cannot be empty".to_string(),
        ));
    }

    if contract.party_b.is_empty() {
        return Err(ObligationsError::InvalidContract(
            "Party B cannot be empty".to_string(),
        ));
    }

    if contract.subject_matter.is_empty() {
        return Err(ObligationsError::InvalidContract(
            "Subject matter cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Tort (불법행위)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tort {
    /// Tortfeasor (가해자)
    pub tortfeasor: String,
    /// Victim (피해자)
    pub victim: String,
    /// Date of tort
    pub date: NaiveDate,
    /// Description
    pub description: String,
    /// Damages claimed
    pub damages: Option<KrwAmount>,
}

impl Tort {
    /// Create new tort
    pub fn new(
        tortfeasor: impl Into<String>,
        victim: impl Into<String>,
        date: NaiveDate,
        description: impl Into<String>,
    ) -> Self {
        Self {
            tortfeasor: tortfeasor.into(),
            victim: victim.into(),
            date,
            description: description.into(),
            damages: None,
        }
    }

    /// Set damages
    pub fn with_damages(mut self, damages: KrwAmount) -> Self {
        self.damages = Some(damages);
        self
    }
}

/// Unjust enrichment (부당이득)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnjustEnrichment {
    /// Enriched party (수익자)
    pub enriched_party: String,
    /// Impoverished party (손실자)
    pub impoverished_party: String,
    /// Enrichment amount
    pub enrichment: KrwAmount,
    /// Date
    pub date: NaiveDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let contract = Contract::new(ContractType::Sale, "김철수", "박영희", date, "아파트")
                .with_consideration(KrwAmount::from_eok(5.0));

            assert_eq!(contract.contract_type, ContractType::Sale);
            assert!(contract.consideration.is_some());
        }
    }

    #[test]
    fn test_validate_contract() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let contract = Contract::new(ContractType::Sale, "김철수", "박영희", date, "아파트");

            let result = validate_contract(&contract);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_tort_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let tort = Tort::new("김철수", "박영희", date, "교통사고")
                .with_damages(KrwAmount::from_man(1_000.0));

            assert_eq!(tort.tortfeasor, "김철수");
            assert!(tort.damages.is_some());
        }
    }
}
