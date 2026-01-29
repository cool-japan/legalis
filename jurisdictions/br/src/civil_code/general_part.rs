//! General Part (Parte Geral) - Articles 1-232
//!
//! Covers persons, property classification, and legal acts.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Natural person (pessoa natural) - Arts. 1-39
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalPerson {
    /// Full name
    pub nome_completo: String,
    /// Age in years
    pub idade: u8,
    /// Whether person is emancipated
    pub emancipado: bool,
    /// CPF (optional)
    pub cpf: Option<String>,
    /// Domicile (domicílio)
    pub domicilio: Option<String>,
}

impl NaturalPerson {
    /// Create a new natural person
    pub fn new(nome: impl Into<String>, idade: u8) -> Self {
        Self {
            nome_completo: nome.into(),
            idade,
            emancipado: false,
            cpf: None,
            domicilio: None,
        }
    }

    /// Check if person has full legal capacity (Art. 5)
    /// Fully capable: >= 18 years OR emancipated
    pub fn is_fully_capable(&self) -> bool {
        self.idade >= 18 || self.emancipado
    }

    /// Check if person is relatively incapable (Art. 4)
    /// Ages 16-17 (unless emancipated)
    pub fn is_relatively_incapable(&self) -> bool {
        self.idade >= 16 && self.idade < 18 && !self.emancipado
    }

    /// Check if person is absolutely incapable (Art. 3 - revoked by Law 13.146/2015)
    /// Now: only minors under 16 (mental incapacity removed)
    pub fn is_absolutely_incapable(&self) -> bool {
        self.idade < 16
    }

    /// Set emancipation status (Art. 5, sole paragraph)
    pub fn with_emancipation(mut self) -> Self {
        self.emancipado = true;
        self
    }

    /// Set CPF
    pub fn with_cpf(mut self, cpf: impl Into<String>) -> Self {
        self.cpf = Some(cpf.into());
        self
    }

    /// Set domicile
    pub fn with_domicile(mut self, domicilio: impl Into<String>) -> Self {
        self.domicilio = Some(domicilio.into());
        self
    }
}

/// Legal person (pessoa jurídica) - Arts. 40-69
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalPerson {
    /// Corporate name (razão social)
    pub razao_social: String,
    /// Trade name (nome fantasia)
    pub nome_fantasia: Option<String>,
    /// CNPJ
    pub cnpj: String,
    /// Type of legal person
    pub tipo: LegalPersonType,
    /// Headquarters location
    pub sede: String,
}

/// Types of legal persons (Art. 44)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalPersonType {
    /// Private associations (associações)
    Association,
    /// Foundations (fundações)
    Foundation,
    /// Business corporations (sociedades empresárias)
    BusinessCorporation,
    /// Simple partnerships (sociedades simples)
    SimplePartnership,
    /// Political parties (partidos políticos)
    PoliticalParty,
    /// Religious organizations (organizações religiosas)
    ReligiousOrganization,
    /// Public law entities (direito público)
    PublicLaw,
}

impl LegalPerson {
    /// Create a new legal person
    pub fn new(
        razao_social: impl Into<String>,
        cnpj: impl Into<String>,
        tipo: LegalPersonType,
    ) -> Self {
        Self {
            razao_social: razao_social.into(),
            nome_fantasia: None,
            cnpj: cnpj.into(),
            tipo,
            sede: String::new(),
        }
    }

    /// Check if has legal personality (Art. 45)
    /// Legal personality begins with registration
    pub fn has_legal_personality(&self) -> bool {
        !self.cnpj.is_empty()
    }

    /// Check if is for-profit (Art. 44, II)
    pub fn is_for_profit(&self) -> bool {
        matches!(
            self.tipo,
            LegalPersonType::BusinessCorporation | LegalPersonType::SimplePartnership
        )
    }
}

/// Property classification (Arts. 79-103)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Immovable property (imóveis) - Art. 79
    Immovable,
    /// Movable property (móveis) - Art. 82
    Movable,
    /// Fungible goods (fungíveis) - Art. 85
    Fungible,
    /// Non-fungible goods (infungíveis) - Art. 85
    NonFungible,
    /// Consumable goods (consumíveis) - Art. 86
    Consumable,
    /// Non-consumable goods (inconsumíveis) - Art. 86
    NonConsumable,
    /// Divisible goods (divisíveis) - Art. 87
    Divisible,
    /// Indivisible goods (indivisíveis) - Art. 87
    Indivisible,
}

/// Legal act (negócio jurídico) - Arts. 104-184
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalAct {
    /// Description of the act
    pub descricao: String,
    /// Whether agent is capable
    pub agente_capaz: bool,
    /// Whether object is licit
    pub objeto_licito: bool,
    /// Whether form is valid
    pub forma_valida: bool,
    /// Type of legal act
    pub tipo: LegalActType,
}

/// Types of legal acts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalActType {
    /// Unilateral act
    Unilateral,
    /// Bilateral/contract
    Bilateral,
    /// Multilateral
    Multilateral,
}

impl LegalAct {
    /// Create a new legal act
    pub fn new(descricao: impl Into<String>, tipo: LegalActType) -> Self {
        Self {
            descricao: descricao.into(),
            agente_capaz: true,
            objeto_licito: true,
            forma_valida: true,
            tipo,
        }
    }

    /// Check validity requirements (Art. 104)
    /// Requires: capable agent, licit object, prescribed/non-prohibited form
    pub fn is_valid(&self) -> Result<(), CivilCodeError> {
        if !self.agente_capaz {
            return Err(CivilCodeError::IncapableAgent);
        }
        if !self.objeto_licito {
            return Err(CivilCodeError::IllicitObject);
        }
        if !self.forma_valida {
            return Err(CivilCodeError::InvalidForm);
        }
        Ok(())
    }

    /// Check if act is null (Art. 166)
    pub fn is_null(&self) -> bool {
        !self.agente_capaz || !self.objeto_licito || !self.forma_valida
    }
}

/// Defects of legal acts (vícios) - Arts. 138-165
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalActDefect {
    /// Error (erro) - Art. 138
    Error { description: String },
    /// Fraud (dolo) - Art. 145
    Fraud { description: String },
    /// Duress (coação) - Art. 151
    Duress { description: String },
    /// Simulated act (simulação) - Art. 167
    Simulation { description: String },
    /// Fraud against creditors (fraude contra credores) - Art. 158
    FraudAgainstCreditors { description: String },
}

/// Civil Code errors
#[derive(Debug, Clone, Error)]
pub enum CivilCodeError {
    /// Incapable agent
    #[error("Agente incapaz (Art. 104, I)")]
    IncapableAgent,

    /// Illicit object
    #[error("Objeto ilícito (Art. 104, II)")]
    IllicitObject,

    /// Invalid form
    #[error("Forma inválida (Art. 104, III)")]
    InvalidForm,

    /// Null act (Art. 166)
    #[error("Ato nulo (Art. 166): {reason}")]
    NullAct { reason: String },

    /// Voidable act (Art. 171)
    #[error("Ato anulável (Art. 171): {reason}")]
    VoidableAct { reason: String },

    /// Legal act defect
    #[error("Vício do negócio jurídico: {defect:?}")]
    ActDefect { defect: LegalActDefect },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for civil code operations
pub type CivilCodeResult<T> = Result<T, CivilCodeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_person_capacity() {
        let adult = NaturalPerson::new("João Silva", 25);
        assert!(adult.is_fully_capable());
        assert!(!adult.is_relatively_incapable());

        let minor = NaturalPerson::new("Maria Santos", 17);
        assert!(minor.is_relatively_incapable());
        assert!(!minor.is_fully_capable());

        let child = NaturalPerson::new("Pedro Oliveira", 10);
        assert!(child.is_absolutely_incapable());
    }

    #[test]
    fn test_emancipation() {
        let emancipated = NaturalPerson::new("Ana Costa", 16).with_emancipation();
        assert!(emancipated.is_fully_capable());
        assert!(!emancipated.is_relatively_incapable());
    }

    #[test]
    fn test_legal_person() {
        let company = LegalPerson::new(
            "ACME Ltda",
            "12345678000190",
            LegalPersonType::BusinessCorporation,
        );
        assert!(company.has_legal_personality());
        assert!(company.is_for_profit());
    }

    #[test]
    fn test_legal_act_validity() {
        let valid_act = LegalAct::new("Compra e venda", LegalActType::Bilateral);
        assert!(valid_act.is_valid().is_ok());

        let mut invalid_act = valid_act.clone();
        invalid_act.agente_capaz = false;
        assert!(invalid_act.is_valid().is_err());
        assert!(invalid_act.is_null());
    }
}
