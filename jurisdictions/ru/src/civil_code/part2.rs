//! Civil Code Part 2: Obligations and Contracts (1996).
//!
//! Federal Law No. 14-FZ of January 26, 1996
//!
//! This part covers:
//! - General provisions on obligations (Articles 307-419)
//! - Individual types of obligations (Articles 420-1109)
//! - Sale, lease, loans, services, etc.

use serde::{Deserialize, Serialize};

use super::CivilCodeError;

/// Types of contracts under Russian Civil Code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale and purchase (купля-продажа)
    SaleAndPurchase,
    /// Supply (поставка)
    Supply,
    /// Retail sale (розничная купля-продажа)
    RetailSale,
    /// Lease (аренда)
    Lease,
    /// Loan (заем)
    Loan,
    /// Credit (кредит)
    Credit,
    /// Bank account (банковский счет)
    BankAccount,
    /// Services (возмездное оказание услуг)
    Services,
    /// Work contract (подряд)
    WorkContract,
    /// Agency (поручение)
    Agency,
    /// Commission (комиссия)
    Commission,
    /// Trust management (доверительное управление)
    TrustManagement,
    /// Insurance (страхование)
    Insurance,
    /// Partnership agreement (договор простого товарищества)
    Partnership,
    /// Gift (дарение)
    Gift,
    /// Donation (пожертвование)
    Donation,
}

/// Types of obligations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationType {
    /// To transfer property
    TransferProperty,
    /// To perform work
    PerformWork,
    /// To provide service
    ProvideService,
    /// To pay money
    PayMoney,
    /// To refrain from action
    RefrainFromAction,
    /// Mixed obligation
    Mixed,
}

/// Contract party
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractParty {
    /// Party name
    pub name: String,
    /// Identification (INN, OGRN, or passport)
    pub identification: String,
    /// Legal address
    pub address: String,
    /// Is legal entity
    pub is_legal_entity: bool,
}

/// Contract representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    /// Type of contract
    pub contract_type: ContractType,
    /// First party (typically seller, lessor, creditor)
    pub party1: ContractParty,
    /// Second party (typically buyer, lessee, borrower)
    pub party2: ContractParty,
    /// Contract date
    pub date: chrono::NaiveDate,
    /// Subject matter
    pub subject: String,
    /// Price or consideration
    pub price: Option<crate::common::Currency>,
    /// Obligations
    pub obligations: Vec<ObligationType>,
    /// Is in written form
    pub written_form: bool,
}

impl Contract {
    /// Creates a new contract
    pub fn new(
        contract_type: ContractType,
        party1: ContractParty,
        party2: ContractParty,
        date: chrono::NaiveDate,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            contract_type,
            party1,
            party2,
            date,
            subject: subject.into(),
            price: None,
            obligations: Vec::new(),
            written_form: false,
        }
    }

    /// Sets the price
    pub fn with_price(mut self, price: crate::common::Currency) -> Self {
        self.price = Some(price);
        self
    }

    /// Adds an obligation
    pub fn add_obligation(mut self, obligation: ObligationType) -> Self {
        self.obligations.push(obligation);
        self
    }

    /// Sets written form requirement
    pub fn with_written_form(mut self, written: bool) -> Self {
        self.written_form = written;
        self
    }

    /// Validates the contract according to Civil Code requirements
    pub fn validate(&self) -> Result<(), CivilCodeError> {
        validate_contract(&self.contract_type)?;

        // Certain contracts require written form
        if self.requires_written_form() && !self.written_form {
            return Err(CivilCodeError::InvalidContract(
                "This contract type requires written form".to_string(),
            ));
        }

        // Sale and purchase requires price
        if matches!(
            self.contract_type,
            ContractType::SaleAndPurchase | ContractType::Supply | ContractType::RetailSale
        ) && self.price.is_none()
        {
            return Err(CivilCodeError::InvalidContract(
                "Sale contract requires price specification".to_string(),
            ));
        }

        Ok(())
    }

    /// Checks if this contract type requires written form
    pub fn requires_written_form(&self) -> bool {
        matches!(
            self.contract_type,
            ContractType::Loan
                | ContractType::Credit
                | ContractType::Lease
                | ContractType::Insurance
                | ContractType::TrustManagement
        )
    }
}

/// Article 307: Concept of obligation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article307 {
    /// Creditor (кредитор)
    pub creditor: String,
    /// Debtor (должник)
    pub debtor: String,
    /// Type of obligation
    pub obligation_type: ObligationType,
    /// Description
    pub description: String,
}

/// Article 420: Concept of contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article420 {
    /// Agreement between two or more persons
    pub parties_count: usize,
    /// Establishes, changes, or terminates civil rights and obligations
    pub purpose: ContractPurpose,
}

/// Purpose of contract under Article 420
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractPurpose {
    /// Establish rights and obligations
    Establish,
    /// Change rights and obligations
    Change,
    /// Terminate rights and obligations
    Terminate,
}

/// Validates contract type
pub fn validate_contract(contract_type: &ContractType) -> Result<(), CivilCodeError> {
    // All contract types defined in the enum are valid
    // Additional validation would check specific requirements per contract type
    match contract_type {
        ContractType::Gift | ContractType::Donation => {
            // Gift contracts have special rules about consideration
            Ok(())
        }
        _ => Ok(()),
    }
}

/// Article 432: Basic provisions on contract conclusion
pub fn validate_essential_terms(
    contract_type: &ContractType,
    has_subject: bool,
    has_price: bool,
) -> Result<(), CivilCodeError> {
    // Subject is essential for all contracts
    if !has_subject {
        return Err(CivilCodeError::InvalidContract(
            "Contract must have a subject matter".to_string(),
        ));
    }

    // Price is essential for sale contracts
    if matches!(
        contract_type,
        ContractType::SaleAndPurchase | ContractType::Supply | ContractType::RetailSale
    ) && !has_price
    {
        return Err(CivilCodeError::InvalidContract(
            "Sale contract requires price as essential term".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_creation() {
        let party1 = ContractParty {
            name: "ООО Продавец".to_string(),
            identification: "1234567890".to_string(),
            address: "Москва".to_string(),
            is_legal_entity: true,
        };

        let party2 = ContractParty {
            name: "ООО Покупатель".to_string(),
            identification: "0987654321".to_string(),
            address: "Санкт-Петербург".to_string(),
            is_legal_entity: true,
        };

        let contract = Contract::new(
            ContractType::SaleAndPurchase,
            party1,
            party2,
            chrono::NaiveDate::from_ymd_opt(2024, 1, 15).expect("Valid date"),
            "Sale of goods",
        )
        .with_price(crate::common::Currency::from_rubles(100000))
        .with_written_form(true);

        assert!(contract.validate().is_ok());
    }

    #[test]
    fn test_written_form_requirement() {
        let party1 = ContractParty {
            name: "Иванов И.И.".to_string(),
            identification: "1234567890".to_string(),
            address: "Москва".to_string(),
            is_legal_entity: false,
        };

        let party2 = ContractParty {
            name: "Петров П.П.".to_string(),
            identification: "0987654321".to_string(),
            address: "Москва".to_string(),
            is_legal_entity: false,
        };

        // Loan requires written form
        let loan = Contract::new(
            ContractType::Loan,
            party1,
            party2,
            chrono::NaiveDate::from_ymd_opt(2024, 1, 15).expect("Valid date"),
            "Loan agreement",
        );

        assert!(loan.validate().is_err());

        let loan_written = loan.with_written_form(true);
        assert!(loan_written.validate().is_ok());
    }

    #[test]
    fn test_essential_terms() {
        // Sale without price should fail
        assert!(validate_essential_terms(&ContractType::SaleAndPurchase, true, false).is_err());

        // Sale with price and subject should succeed
        assert!(validate_essential_terms(&ContractType::SaleAndPurchase, true, true).is_ok());

        // Gift doesn't require price
        assert!(validate_essential_terms(&ContractType::Gift, true, false).is_ok());
    }
}
