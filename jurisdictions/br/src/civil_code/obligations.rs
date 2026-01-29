//! Obligations (Direito das Obrigações) - Articles 233-420
//!
//! Law of obligations including sources, types, and modes of extinction.

use crate::common::BrazilianCurrency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Obligation (obrigação) - Arts. 233-285
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Obligation {
    /// Creditor (credor)
    pub credor: String,
    /// Debtor (devedor)
    pub devedor: String,
    /// Type of obligation
    pub tipo: ObligationType,
    /// Object/subject matter (objeto)
    pub objeto: String,
    /// Due date (vencimento)
    pub vencimento: Option<NaiveDate>,
    /// Whether obligation is divisible
    pub divisivel: bool,
}

/// Types of obligations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationType {
    /// To give (dar) - Art. 233
    ToGive {
        /// What to give
        coisa: String,
        /// Certain or uncertain
        certa: bool,
    },
    /// To do (fazer) - Art. 247
    ToDo {
        /// Action to perform
        acao: String,
        /// Whether fungible (can be done by another)
        fungivel: bool,
    },
    /// Not to do (não fazer) - Art. 250
    NotToDo {
        /// Action to abstain from
        abstencao: String,
    },
    /// Alternative obligation (alternativa) - Art. 252
    Alternative {
        /// List of alternative performances
        alternativas: Vec<String>,
    },
    /// Facultative obligation (facultativa) - Art. 252
    Facultative {
        /// Principal object
        principal: String,
        /// Substitute object
        substituto: String,
    },
}

impl Obligation {
    /// Create a new obligation to give
    pub fn to_give(
        credor: impl Into<String>,
        devedor: impl Into<String>,
        coisa: impl Into<String>,
        certa: bool,
    ) -> Self {
        Self {
            credor: credor.into(),
            devedor: devedor.into(),
            tipo: ObligationType::ToGive {
                coisa: coisa.into(),
                certa,
            },
            objeto: String::new(),
            vencimento: None,
            divisivel: true,
        }
    }

    /// Create a new obligation to do
    pub fn to_do(
        credor: impl Into<String>,
        devedor: impl Into<String>,
        acao: impl Into<String>,
        fungivel: bool,
    ) -> Self {
        Self {
            credor: credor.into(),
            devedor: devedor.into(),
            tipo: ObligationType::ToDo {
                acao: acao.into(),
                fungivel,
            },
            objeto: String::new(),
            vencimento: None,
            divisivel: fungivel,
        }
    }

    /// Check if obligation is due (vencida)
    pub fn is_due(&self, reference_date: NaiveDate) -> bool {
        self.vencimento
            .is_some_and(|vencimento| reference_date >= vencimento)
    }

    /// Check if debtor is in default (mora) - Art. 394
    pub fn is_in_default(&self, reference_date: NaiveDate) -> bool {
        self.is_due(reference_date)
    }
}

/// Transmission of obligations (transmissão) - Arts. 286-303
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationTransmission {
    /// Assignment of credit (cessão de crédito) - Art. 286
    CreditAssignment {
        /// Original creditor (cedente)
        cedente: String,
        /// New creditor (cessionário)
        cessionario: String,
        /// Debtor (cedido)
        cedido: String,
        /// Whether debtor consented
        consentimento_devedor: bool,
    },
    /// Assumption of debt (assunção de dívida) - Art. 299
    DebtAssumption {
        /// Original debtor
        devedor_original: String,
        /// New debtor
        devedor_novo: String,
        /// Whether creditor consented (required)
        consentimento_credor: bool,
    },
}

/// Modes of extinction of obligations (extinção) - Arts. 304-388
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationExtinction {
    /// Payment (pagamento) - Art. 304
    Payment {
        /// Amount paid
        valor: BrazilianCurrency,
        /// Payment date
        data: NaiveDate,
        /// Receipt issued
        recibo: bool,
    },
    /// Set-off (compensação) - Art. 368
    SetOff {
        /// Amount offset
        valor: BrazilianCurrency,
        /// Description
        descricao: String,
    },
    /// Novation (novação) - Art. 360
    Novation {
        /// New obligation description
        nova_obrigacao: String,
    },
    /// Confusion (confusão) - Art. 381
    Confusion {
        /// When creditor and debtor become same person
        descricao: String,
    },
    /// Remission (remissão) - Art. 385
    Remission {
        /// Forgiveness of debt
        perdoada: bool,
    },
}

/// Default (inadimplemento) - Arts. 389-420
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Default {
    /// Type of default
    pub tipo: DefaultType,
    /// Date of default
    pub data: NaiveDate,
    /// Losses and damages (perdas e danos)
    pub perdas_danos: Option<BrazilianCurrency>,
    /// Whether debtor is liable
    pub devedor_responsavel: bool,
}

/// Types of default
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefaultType {
    /// Debtor's default (mora do devedor) - Art. 394
    DebtorDefault,
    /// Creditor's default (mora do credor) - Art. 400
    CreditorDefault,
    /// Impossibility of performance (impossibilidade) - Art. 248
    Impossibility,
    /// Breach of positive obligation (descumprimento de obrigação de fazer)
    PositiveBreach,
    /// Breach of negative obligation (descumprimento de obrigação de não fazer)
    NegativeBreach,
}

impl Default {
    /// Create a new debtor default
    pub fn debtor_default(data: NaiveDate) -> Self {
        Self {
            tipo: DefaultType::DebtorDefault,
            data,
            perdas_danos: None,
            devedor_responsavel: true,
        }
    }

    /// Calculate damages (Art. 402)
    /// Includes actual losses and lost profits (lucros cessantes)
    pub fn with_damages(mut self, valor: BrazilianCurrency) -> Self {
        self.perdas_danos = Some(valor);
        self
    }

    /// Check if default is excused (force majeure - Art. 393)
    pub fn is_excused(&self) -> bool {
        !self.devedor_responsavel
    }
}

/// Obligations errors
#[derive(Debug, Clone, Error)]
pub enum ObligationsError {
    /// Invalid obligation
    #[error("Obrigação inválida: {reason}")]
    InvalidObligation { reason: String },

    /// Default occurred
    #[error("Inadimplemento (Art. 389): {tipo:?}")]
    Default { tipo: DefaultType },

    /// Indivisible obligation division attempted
    #[error("Tentativa de divisão de obrigação indivisível (Art. 258)")]
    IndivisibleDivision,

    /// Invalid transmission
    #[error("Transmissão inválida: {reason}")]
    InvalidTransmission { reason: String },

    /// Payment error
    #[error("Erro no pagamento: {reason}")]
    PaymentError { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for obligations operations
pub type ObligationsResult<T> = Result<T, ObligationsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obligation_to_give() {
        let obligation = Obligation::to_give("Credor A", "Devedor B", "Imóvel", true);
        assert!(matches!(obligation.tipo, ObligationType::ToGive { .. }));
    }

    #[test]
    fn test_obligation_due() {
        let mut obligation = Obligation::to_give("A", "B", "Coisa", true);
        let past = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        obligation.vencimento = Some(past);

        let today = NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date");
        assert!(obligation.is_due(today));
        assert!(obligation.is_in_default(today));
    }

    #[test]
    fn test_default_with_damages() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let default =
            Default::debtor_default(date).with_damages(BrazilianCurrency::from_reais(10000));

        assert_eq!(default.tipo, DefaultType::DebtorDefault);
        assert!(default.perdas_danos.is_some());
    }

    #[test]
    fn test_payment_extinction() {
        let extinction = ObligationExtinction::Payment {
            valor: BrazilianCurrency::from_reais(5000),
            data: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            recibo: true,
        };
        assert!(matches!(extinction, ObligationExtinction::Payment { .. }));
    }
}
