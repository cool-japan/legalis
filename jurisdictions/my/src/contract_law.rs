//! Contracts Act 1950
//!
//! Malaysian contract law, based on the Indian Contract Act 1872.
//!
//! # Key Provisions
//!
//! - **Section 2(h)**: Definition of contract
//! - **Section 10**: What agreements are contracts
//! - **Section 11**: Who are competent to contract
//! - **Section 13-22**: Free consent
//! - **Section 23**: Lawful consideration and object
//! - **Section 73-75**: Damages for breach of contract
//!
//! # Formation Requirements
//!
//! 1. Offer (Section 2(a))
//! 2. Acceptance (Section 2(b))
//! 3. Consideration (Section 2(d))
//! 4. Intention to create legal relations
//! 5. Capacity to contract (Section 11)
//! 6. Free consent (Section 13)
//! 7. Lawful object (Section 23)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Contract error types.
#[derive(Debug, Error)]
pub enum ContractError {
    /// Contract is void.
    #[error("Contract is void: {reason}")]
    VoidContract { reason: String },

    /// Contract is voidable.
    #[error("Contract is voidable: {reason}")]
    VoidableContract { reason: String },

    /// Invalid consideration.
    #[error("Invalid consideration: {reason}")]
    InvalidConsideration { reason: String },

    /// Lack of capacity.
    #[error("Party lacks capacity to contract: {party}")]
    LackOfCapacity { party: String },

    /// Consent not free (coercion, undue influence, fraud, misrepresentation, mistake).
    #[error("Consent not free: {vitiation_type}")]
    ConsentNotFree { vitiation_type: String },

    /// Unlawful object.
    #[error("Contract object is unlawful: {reason}")]
    UnlawfulObject { reason: String },
}

/// Result type for contract operations.
pub type Result<T> = std::result::Result<T, ContractError>;

/// Contract type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale of goods.
    SaleOfGoods,
    /// Service agreement.
    ServiceAgreement,
    /// Employment contract.
    Employment,
    /// Lease/rental agreement.
    Lease,
    /// Loan agreement.
    Loan,
    /// Partnership agreement.
    Partnership,
    /// Agency agreement.
    Agency,
    /// General contract.
    General,
}

/// Party to a contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Party {
    /// Party identifier.
    pub id: Uuid,
    /// Party name.
    pub name: String,
    /// IC/registration number.
    pub identification: String,
    /// Whether party has capacity to contract.
    pub has_capacity: bool,
    /// Party type (individual, company, etc.).
    pub party_type: PartyType,
}

/// Party type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyType {
    /// Individual person.
    Individual,
    /// Company (Sdn Bhd, Bhd, etc.).
    Company,
    /// Partnership.
    Partnership,
    /// Government entity.
    Government,
    /// Other legal entity.
    Other,
}

impl Party {
    /// Creates a new party.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        identification: impl Into<String>,
        party_type: PartyType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            identification: identification.into(),
            has_capacity: true,
            party_type,
        }
    }

    /// Sets capacity to contract.
    #[must_use]
    pub fn with_capacity(mut self, has_capacity: bool) -> Self {
        self.has_capacity = has_capacity;
        self
    }
}

/// Consideration for the contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consideration {
    /// Description of consideration.
    pub description: String,
    /// Monetary value in sen (if applicable).
    pub value_sen: Option<i64>,
    /// Whether consideration is lawful.
    pub lawful: bool,
    /// Whether consideration is adequate.
    pub adequate: bool,
}

impl Consideration {
    /// Creates a new consideration.
    #[must_use]
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            value_sen: None,
            lawful: true,
            adequate: true,
        }
    }

    /// Sets the monetary value.
    #[must_use]
    pub fn with_value_sen(mut self, value_sen: i64) -> Self {
        self.value_sen = Some(value_sen);
        self
    }

    /// Sets whether consideration is lawful.
    #[must_use]
    pub fn with_lawful(mut self, lawful: bool) -> Self {
        self.lawful = lawful;
        self
    }
}

/// Contract under Contracts Act 1950.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    /// Contract identifier.
    pub id: Uuid,
    /// Contract type.
    pub contract_type: ContractType,
    /// Parties to the contract.
    pub parties: Vec<Party>,
    /// Consideration.
    pub consideration: Consideration,
    /// Contract terms.
    pub terms: Vec<String>,
    /// Date of contract formation.
    pub date: DateTime<Utc>,
    /// Whether consent is free.
    pub free_consent: bool,
    /// Whether contract has lawful object.
    pub lawful_object: bool,
    /// Contract status.
    pub status: ContractStatus,
}

/// Contract status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractStatus {
    /// Valid and enforceable.
    Valid,
    /// Void ab initio.
    Void,
    /// Voidable at the option of a party.
    Voidable,
    /// Discharged (completed, rescinded, etc.).
    Discharged,
    /// Breached.
    Breached,
}

impl Contract {
    /// Creates a contract builder.
    #[must_use]
    pub fn builder() -> ContractBuilder {
        ContractBuilder::default()
    }

    /// Validates the contract under Contracts Act 1950.
    pub fn validate(&self) -> Result<ValidationReport> {
        validate_contract(self)
    }
}

/// Contract builder.
#[derive(Debug, Clone, Default)]
pub struct ContractBuilder {
    contract_type: Option<ContractType>,
    parties: Vec<Party>,
    consideration: Option<Consideration>,
    terms: Vec<String>,
    free_consent: bool,
    lawful_object: bool,
}

impl ContractBuilder {
    /// Sets the contract type.
    #[must_use]
    pub fn contract_type(mut self, contract_type: ContractType) -> Self {
        self.contract_type = Some(contract_type);
        self
    }

    /// Adds a party.
    #[must_use]
    pub fn add_party(mut self, party: Party) -> Self {
        self.parties.push(party);
        self
    }

    /// Sets the consideration.
    #[must_use]
    pub fn consideration(mut self, consideration: Consideration) -> Self {
        self.consideration = Some(consideration);
        self
    }

    /// Adds a term.
    #[must_use]
    pub fn add_term(mut self, term: impl Into<String>) -> Self {
        self.terms.push(term.into());
        self
    }

    /// Sets whether consent is free.
    #[must_use]
    pub fn free_consent(mut self, free_consent: bool) -> Self {
        self.free_consent = free_consent;
        self
    }

    /// Sets whether object is lawful.
    #[must_use]
    pub fn lawful_object(mut self, lawful_object: bool) -> Self {
        self.lawful_object = lawful_object;
        self
    }

    /// Builds the contract.
    pub fn build(self) -> Result<Contract> {
        let contract_type = self
            .contract_type
            .ok_or_else(|| ContractError::VoidContract {
                reason: "Contract type not specified".to_string(),
            })?;

        let consideration =
            self.consideration
                .ok_or_else(|| ContractError::InvalidConsideration {
                    reason: "No consideration provided".to_string(),
                })?;

        if self.parties.len() < 2 {
            return Err(ContractError::VoidContract {
                reason: "Contract requires at least two parties".to_string(),
            });
        }

        Ok(Contract {
            id: Uuid::new_v4(),
            contract_type,
            parties: self.parties,
            consideration,
            terms: self.terms,
            date: Utc::now(),
            free_consent: self.free_consent,
            lawful_object: self.lawful_object,
            status: ContractStatus::Valid,
        })
    }
}

/// Contract validation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Whether contract is valid.
    pub valid: bool,
    /// Issues found.
    pub issues: Vec<String>,
    /// Contract status.
    pub status: ContractStatus,
}

/// Validates a contract under Contracts Act 1950, Section 10.
pub fn validate_contract(contract: &Contract) -> Result<ValidationReport> {
    let mut issues = Vec::new();
    let mut status = ContractStatus::Valid;

    // Check capacity (Section 11)
    for party in &contract.parties {
        if !party.has_capacity {
            issues.push(format!("Party '{}' lacks capacity to contract", party.name));
            status = ContractStatus::Void;
        }
    }

    // Check consideration (Section 2(d))
    if !contract.consideration.lawful {
        issues.push("Consideration is unlawful".to_string());
        status = ContractStatus::Void;
    }

    // Check free consent (Section 13)
    if !contract.free_consent {
        issues.push("Consent is not free (may be vitiated by coercion, undue influence, fraud, misrepresentation, or mistake)".to_string());
        status = ContractStatus::Voidable;
    }

    // Check lawful object (Section 23)
    if !contract.lawful_object {
        issues.push("Contract object is unlawful or opposed to public policy".to_string());
        status = ContractStatus::Void;
    }

    // Check if at least 2 parties
    if contract.parties.len() < 2 {
        issues.push("Contract requires at least two parties".to_string());
        status = ContractStatus::Void;
    }

    let valid = issues.is_empty();

    Ok(ValidationReport {
        valid,
        issues,
        status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_contract() {
        let party1 = Party::new("Ahmad bin Ali", "850123-01-5678", PartyType::Individual);
        let party2 = Party::new("Tech Sdn Bhd", "201601012345", PartyType::Company);

        let consideration =
            Consideration::new("Software development services").with_value_sen(1000000);

        let contract = Contract::builder()
            .contract_type(ContractType::ServiceAgreement)
            .add_party(party1)
            .add_party(party2)
            .consideration(consideration)
            .free_consent(true)
            .lawful_object(true)
            .build()
            .expect("Valid contract");

        let report = contract.validate().expect("Validation succeeds");
        assert!(report.valid);
        assert_eq!(report.status, ContractStatus::Valid);
    }

    #[test]
    fn test_void_contract_lack_of_capacity() {
        let party1 =
            Party::new("Minor", "050123-01-5678", PartyType::Individual).with_capacity(false);
        let party2 = Party::new("Tech Sdn Bhd", "201601012345", PartyType::Company);

        let consideration = Consideration::new("Services");

        let contract = Contract::builder()
            .contract_type(ContractType::ServiceAgreement)
            .add_party(party1)
            .add_party(party2)
            .consideration(consideration)
            .free_consent(true)
            .lawful_object(true)
            .build()
            .expect("Contract built");

        let report = contract.validate().expect("Validation succeeds");
        assert!(!report.valid);
        assert_eq!(report.status, ContractStatus::Void);
    }

    #[test]
    fn test_voidable_contract_no_free_consent() {
        let party1 = Party::new("Ahmad bin Ali", "850123-01-5678", PartyType::Individual);
        let party2 = Party::new("Tech Sdn Bhd", "201601012345", PartyType::Company);

        let consideration = Consideration::new("Services");

        let contract = Contract::builder()
            .contract_type(ContractType::ServiceAgreement)
            .add_party(party1)
            .add_party(party2)
            .consideration(consideration)
            .free_consent(false) // Consent vitiated
            .lawful_object(true)
            .build()
            .expect("Contract built");

        let report = contract.validate().expect("Validation succeeds");
        assert!(!report.valid);
        assert_eq!(report.status, ContractStatus::Voidable);
    }
}
