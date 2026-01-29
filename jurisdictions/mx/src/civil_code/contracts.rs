//! Federal Civil Code - Contracts (Contratos)
//!
//! Covers contract formation, validity, and enforcement
//! (Código Civil Federal, Libro Cuarto, Segunda Parte)

use crate::common::MexicanCurrency;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Contract structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    /// Contract parties
    pub partes: Vec<Party>,
    /// Contract object
    pub objeto: String,
    /// Contract type
    pub tipo: ContractType,
    /// Formation date
    pub fecha_celebracion: DateTime<Utc>,
    /// Contract terms
    pub terminos: Vec<Term>,
    /// Contract value
    pub valor: Option<MexicanCurrency>,
}

/// Contract party
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Party {
    /// Party name
    pub nombre: String,
    /// Party role
    pub rol: PartyRole,
    /// Legal capacity
    pub capacidad: bool,
}

/// Party role in contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartyRole {
    /// Buyer (Comprador)
    Buyer,
    /// Seller (Vendedor)
    Seller,
    /// Lender (Mutuante)
    Lender,
    /// Borrower (Mutuatario)
    Borrower,
    /// Lessor (Arrendador)
    Lessor,
    /// Lessee (Arrendatario)
    Lessee,
    /// Service provider (Prestador)
    ServiceProvider,
    /// Client (Cliente)
    Client,
}

/// Contract types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale (Compraventa)
    Sale,
    /// Lease (Arrendamiento)
    Lease,
    /// Loan (Mutuo)
    Loan,
    /// Service (Prestación de servicios)
    Service,
    /// Partnership (Asociación)
    Partnership,
    /// Mandate (Mandato)
    Mandate,
    /// Deposit (Depósito)
    Deposit,
}

/// Contract term
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Term {
    /// Term description
    pub descripcion: String,
    /// Whether the term is essential
    pub esencial: bool,
}

/// Contract validity requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidityRequirements {
    /// Consent (Consentimiento)
    pub consentimiento: bool,
    /// Lawful object (Objeto lícito)
    pub objeto_licito: bool,
    /// Consideration (Causa)
    pub causa: bool,
    /// Legal form if required (Forma legal)
    pub forma_legal: bool,
}

/// Contract defects (Vicios del consentimiento)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentDefect {
    /// Error (Error)
    Error,
    /// Fraud (Dolo)
    Fraud,
    /// Violence (Violencia)
    Violence,
    /// Undue influence (Lesión)
    UndueInfluence,
}

/// Contract errors
#[derive(Debug, Error)]
pub enum ContractError {
    #[error("Invalid contract: {0}")]
    Invalid(String),
    #[error("Missing required element: {0}")]
    MissingElement(String),
    #[error("Consent defect: {0:?}")]
    ConsentDefect(ConsentDefect),
    #[error("Illegal object: {0}")]
    IllegalObject(String),
}

impl Contract {
    /// Create new contract
    pub fn new(
        partes: Vec<Party>,
        objeto: String,
        tipo: ContractType,
        fecha_celebracion: DateTime<Utc>,
    ) -> Self {
        Self {
            partes,
            objeto,
            tipo,
            fecha_celebracion,
            terminos: Vec::new(),
            valor: None,
        }
    }

    /// Add contract term
    pub fn add_term(&mut self, term: Term) {
        self.terminos.push(term);
    }

    /// Validate contract
    pub fn validate(&self) -> Result<(), ContractError> {
        // Must have at least 2 parties
        if self.partes.len() < 2 {
            return Err(ContractError::MissingElement(
                "at least 2 parties required".to_string(),
            ));
        }

        // All parties must have legal capacity
        for party in &self.partes {
            if !party.capacidad {
                return Err(ContractError::Invalid(format!(
                    "party {} lacks legal capacity",
                    party.nombre
                )));
            }
        }

        // Must have valid object
        if self.objeto.is_empty() {
            return Err(ContractError::MissingElement("objeto".to_string()));
        }

        Ok(())
    }

    /// Check if contract is bilateral
    pub fn is_bilateral(&self) -> bool {
        self.partes.len() == 2
    }

    /// Check if contract is onerous
    pub fn is_onerous(&self) -> bool {
        self.valor.is_some()
    }
}

impl ValidityRequirements {
    /// Check if all requirements are met
    pub fn is_valid(&self) -> bool {
        self.consentimiento && self.objeto_licito && self.causa && self.forma_legal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_creation() {
        let parties = vec![
            Party {
                nombre: "Comprador".to_string(),
                rol: PartyRole::Buyer,
                capacidad: true,
            },
            Party {
                nombre: "Vendedor".to_string(),
                rol: PartyRole::Seller,
                capacidad: true,
            },
        ];

        let contract = Contract::new(
            parties,
            "Venta de inmueble".to_string(),
            ContractType::Sale,
            Utc::now(),
        );

        assert!(contract.validate().is_ok());
        assert!(contract.is_bilateral());
    }

    #[test]
    fn test_validity_requirements() {
        let requirements = ValidityRequirements {
            consentimiento: true,
            objeto_licito: true,
            causa: true,
            forma_legal: true,
        };

        assert!(requirements.is_valid());
    }
}
