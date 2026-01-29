//! Contracts (Contratos) - Articles 421-853
//!
//! Contract law including general provisions and specific contract types.

use crate::common::BrazilianCurrency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Contract (contrato) - Arts. 421-480
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contract {
    /// Contract parties
    pub partes: Vec<String>,
    /// Contract type
    pub tipo: ContractType,
    /// Object/subject matter
    pub objeto: String,
    /// Contract value
    pub valor: Option<BrazilianCurrency>,
    /// Execution date
    pub data_execucao: Option<NaiveDate>,
    /// Duration
    pub duracao: Option<ContractDuration>,
    /// Whether contract is adhesion contract
    pub contrato_adesao: bool,
}

/// Contract types (specific contracts)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractType {
    /// Sale (compra e venda) - Arts. 481-532
    Sale {
        /// Seller
        vendedor: String,
        /// Buyer
        comprador: String,
        /// Object sold
        coisa: String,
    },
    /// Lease (locação) - Arts. 565-578
    Lease {
        /// Lessor (locador)
        locador: String,
        /// Lessee (locatário)
        locatario: String,
        /// Property
        imovel: String,
    },
    /// Loan (empréstimo) - Arts. 586-592
    Loan {
        /// Lender (mutuante)
        mutuante: String,
        /// Borrower (mutuário)
        mutuario: String,
        /// Amount
        valor: BrazilianCurrency,
    },
    /// Service provision (prestação de serviços) - Arts. 593-609
    ServiceProvision {
        /// Service provider
        prestador: String,
        /// Service recipient
        tomador: String,
        /// Service description
        servico: String,
    },
    /// Partnership (sociedade) - Arts. 981-985
    Partnership {
        /// Partners
        socios: Vec<String>,
        /// Business purpose
        objeto_social: String,
    },
    /// Mandate (mandato) - Arts. 653-692
    Mandate {
        /// Mandator (mandante)
        mandante: String,
        /// Mandatory (mandatário)
        mandatario: String,
        /// Powers granted
        poderes: String,
    },
    /// Insurance (seguro) - Arts. 757-802
    Insurance {
        /// Insurer (segurador)
        segurador: String,
        /// Insured (segurado)
        segurado: String,
        /// Risk covered
        risco: String,
    },
    /// Deposit (depósito) - Arts. 627-652
    Deposit {
        /// Depositary
        depositario: String,
        /// Depositor
        depositante: String,
        /// Deposited object
        coisa_depositada: String,
    },
    /// Commission (comissão) - Arts. 693-709
    Commission {
        /// Commissioner
        comissario: String,
        /// Principal
        comitente: String,
    },
    /// Other contract types
    Other { descricao: String },
}

/// Contract duration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractDuration {
    /// Determined term
    Determined {
        /// Start date
        inicio: NaiveDate,
        /// End date
        fim: NaiveDate,
    },
    /// Indeterminate term
    Indeterminate {
        /// Start date
        inicio: NaiveDate,
    },
}

impl Contract {
    /// Create a new contract
    pub fn new(tipo: ContractType) -> Self {
        Self {
            partes: Vec::new(),
            tipo,
            objeto: String::new(),
            valor: None,
            data_execucao: None,
            duracao: None,
            contrato_adesao: false,
        }
    }

    /// Check if contract requires good faith (Art. 422)
    /// All contracts require good faith
    pub fn requires_good_faith(&self) -> bool {
        true
    }

    /// Check if contract serves social function (Art. 421)
    /// All contracts must serve social function
    pub fn serves_social_function(&self) -> bool {
        true
    }

    /// Check if adhesion contract requires special interpretation (Art. 423)
    /// Ambiguous clauses interpreted in favor of adherent
    pub fn requires_favorable_interpretation(&self) -> bool {
        self.contrato_adesao
    }

    /// Mark as adhesion contract
    pub fn as_adhesion(mut self) -> Self {
        self.contrato_adesao = true;
        self
    }

    /// Add party to contract
    pub fn add_party(mut self, parte: impl Into<String>) -> Self {
        self.partes.push(parte.into());
        self
    }

    /// Set contract value
    pub fn with_value(mut self, valor: BrazilianCurrency) -> Self {
        self.valor = Some(valor);
        self
    }
}

/// Contract defects (vícios do contrato)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractDefect {
    /// Lesion (lesão) - Art. 157
    /// Excessive advantage due to need or inexperience
    Lesion {
        /// Description of advantage
        vantagem_excessiva: String,
    },
    /// Onerous disproportion (onerosidade excessiva) - Art. 478
    /// Supervening extraordinary fact
    OnerousDisproportion {
        /// Description of supervening fact
        fato_superveniente: String,
    },
    /// State of danger (estado de perigo) - Art. 156
    StateOfDanger {
        /// Description of danger
        perigo: String,
    },
}

/// Contract termination (extinção do contrato)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractTermination {
    /// Fulfillment (adimplemento)
    Fulfillment {
        /// Fulfillment date
        data: NaiveDate,
    },
    /// Mutual rescission (distrato) - Art. 472
    MutualRescission {
        /// Rescission date
        data: NaiveDate,
    },
    /// Unilateral rescission (resilição unilateral) - Art. 473
    UnilateralRescission {
        /// Notice given
        notificacao: bool,
    },
    /// Resolution for breach (resolução por inadimplemento) - Art. 475
    ResolutionForBreach {
        /// Breaching party
        inadimplente: String,
    },
    /// Resolution for onerous disproportion (resolução por onerosidade) - Art. 478
    ResolutionForDisproportion {
        /// Supervening fact
        fato: String,
    },
}

/// Contractual liability (responsabilidade contratual)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractualLiability {
    /// Breaching party
    pub inadimplente: String,
    /// Type of breach
    pub tipo_violacao: BreachType,
    /// Damages amount
    pub danos: Option<BrazilianCurrency>,
    /// Penalty clause (cláusula penal) amount - Art. 408
    pub clausula_penal: Option<BrazilianCurrency>,
}

/// Types of contractual breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Total breach (inadimplemento total)
    Total,
    /// Partial breach (inadimplemento parcial)
    Partial,
    /// Delay (mora)
    Delay,
    /// Defective performance (adimplemento defeituoso)
    DefectivePerformance,
}

/// Contract errors
#[derive(Debug, Clone, Error)]
pub enum ContractError {
    /// Contract defect
    #[error("Vício do contrato: {defect:?}")]
    ContractDefect { defect: ContractDefect },

    /// Breach of contract
    #[error("Inadimplemento contratual (Art. 389): {breach_type:?}")]
    Breach { breach_type: BreachType },

    /// Invalid termination
    #[error("Extinção inválida: {reason}")]
    InvalidTermination { reason: String },

    /// Bad faith violation (Art. 422)
    #[error("Violação da boa-fé objetiva (Art. 422): {description}")]
    BadFaithViolation { description: String },

    /// Social function violation (Art. 421)
    #[error("Violação da função social (Art. 421): {description}")]
    SocialFunctionViolation { description: String },

    /// Excessive penalty clause (Art. 412)
    #[error("Cláusula penal excessiva (Art. 412)")]
    ExcessivePenalty,

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for contract operations
pub type ContractResult<T> = Result<T, ContractError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sale_contract() {
        let sale = ContractType::Sale {
            vendedor: "Vendedor A".to_string(),
            comprador: "Comprador B".to_string(),
            coisa: "Imóvel".to_string(),
        };
        let contract = Contract::new(sale).with_value(BrazilianCurrency::from_reais(500000));
        assert!(contract.requires_good_faith());
    }

    #[test]
    fn test_adhesion_contract() {
        let contract = Contract::new(ContractType::Other {
            descricao: "Serviço".to_string(),
        })
        .as_adhesion();
        assert!(contract.requires_favorable_interpretation());
    }

    #[test]
    fn test_contract_termination() {
        let termination = ContractTermination::Fulfillment {
            data: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
        };
        assert!(matches!(
            termination,
            ContractTermination::Fulfillment { .. }
        ));
    }

    #[test]
    fn test_breach_liability() {
        let liability = ContractualLiability {
            inadimplente: "Parte A".to_string(),
            tipo_violacao: BreachType::Total,
            danos: Some(BrazilianCurrency::from_reais(100000)),
            clausula_penal: Some(BrazilianCurrency::from_reais(10000)),
        };
        assert_eq!(liability.tipo_violacao, BreachType::Total);
    }
}
