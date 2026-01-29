//! # Corporations Law - Lei das S.A. (Lei nº 6.404/1976)
//!
//! Brazilian corporations law for joint-stock companies (sociedades anônimas).

use crate::common::BrazilianCurrency;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Corporation (S.A. - Sociedade Anônima)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Corporation {
    /// Corporate name (denominação social)
    pub denominacao: String,
    /// CNPJ
    pub cnpj: String,
    /// Corporation type
    pub tipo: CorporationType,
    /// Share capital (capital social)
    pub capital_social: BrazilianCurrency,
    /// Number of shares
    pub numero_acoes: u64,
    /// Whether publicly traded
    pub capital_aberto: bool,
}

/// Corporation types (Art. 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorporationType {
    /// Publicly-held (companhia aberta)
    PubliclyHeld,
    /// Closely-held (companhia fechada)
    ClosedCorporation,
}

impl Corporation {
    /// Create a new corporation
    pub fn new(
        denominacao: impl Into<String>,
        cnpj: impl Into<String>,
        tipo: CorporationType,
        capital: BrazilianCurrency,
    ) -> Self {
        Self {
            denominacao: denominacao.into(),
            cnpj: cnpj.into(),
            tipo,
            capital_social: capital,
            numero_acoes: 0,
            capital_aberto: matches!(tipo, CorporationType::PubliclyHeld),
        }
    }

    /// Check minimum capital requirement
    /// Publicly-held: higher requirements
    /// Closed: no minimum (but must be subscribed)
    pub fn meets_capital_requirement(&self) -> bool {
        self.capital_social.centavos > 0
    }

    /// Calculate par value per share
    pub fn par_value_per_share(&self) -> Option<BrazilianCurrency> {
        if self.numero_acoes == 0 {
            return None;
        }
        Some(BrazilianCurrency::from_centavos(
            self.capital_social.centavos / self.numero_acoes as i64,
        ))
    }
}

/// Share types (Arts. 15-20)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Share {
    /// Share type
    pub tipo: ShareType,
    /// Class (A, B, etc.)
    pub classe: Option<String>,
    /// Par value
    pub valor_nominal: Option<BrazilianCurrency>,
    /// Whether has voting rights
    pub voto: bool,
    /// Dividend preference
    pub preferencia_dividendo: Option<u8>,
}

/// Share types (Art. 15)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareType {
    /// Common shares (ações ordinárias)
    /// Always have voting rights
    Common,
    /// Preferred shares (ações preferenciais)
    /// May have restricted voting rights, dividend preferences
    Preferred,
}

impl Share {
    /// Create common share
    pub fn common() -> Self {
        Self {
            tipo: ShareType::Common,
            classe: None,
            valor_nominal: None,
            voto: true,
            preferencia_dividendo: None,
        }
    }

    /// Create preferred share
    pub fn preferred(dividend_preference: u8) -> Self {
        Self {
            tipo: ShareType::Preferred,
            classe: None,
            valor_nominal: None,
            voto: false,
            preferencia_dividendo: Some(dividend_preference),
        }
    }
}

/// Corporate governance structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorporateGovernance {
    /// General meeting (assembleia geral)
    pub assembleia_geral: bool,
    /// Board of directors (conselho de administração)
    /// Mandatory for publicly-held companies
    pub conselho_administracao: bool,
    /// Executive board (diretoria)
    /// Mandatory
    pub diretoria: bool,
    /// Fiscal council (conselho fiscal)
    /// Not permanent, installed when requested
    pub conselho_fiscal: bool,
}

impl CorporateGovernance {
    /// Create governance for publicly-held company
    pub fn publicly_held() -> Self {
        Self {
            assembleia_geral: true,
            conselho_administracao: true, // Mandatory (Art. 138, §2)
            diretoria: true,
            conselho_fiscal: false,
        }
    }

    /// Create governance for closed corporation
    pub fn closed() -> Self {
        Self {
            assembleia_geral: true,
            conselho_administracao: false, // Optional
            diretoria: true,
            conselho_fiscal: false,
        }
    }
}

/// Dividend distribution (Arts. 201-205)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dividend {
    /// Net profit
    pub lucro_liquido: BrazilianCurrency,
    /// Mandatory dividend percentage (minimum 25% by law)
    pub percentual_obrigatorio: u8,
    /// Total dividend to distribute
    pub dividendo_total: BrazilianCurrency,
}

impl Dividend {
    /// Calculate mandatory dividend (Art. 202)
    /// Minimum 25% of adjusted net profit unless bylaws specify otherwise
    pub fn calculate_mandatory(lucro_liquido: BrazilianCurrency, percentual: u8) -> Self {
        let percentual = if percentual < 25 { 25 } else { percentual };

        let dividendo = (lucro_liquido.centavos * percentual as i64) / 100;

        Self {
            lucro_liquido,
            percentual_obrigatorio: percentual,
            dividendo_total: BrazilianCurrency::from_centavos(dividendo),
        }
    }
}

/// Shareholder rights (Arts. 109-120)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShareholderRight {
    /// Participate in profits (Art. 109, I)
    ProfitParticipation,
    /// Participate in liquidation (Art. 109, II)
    LiquidationParticipation,
    /// Inspect corporate books (Art. 109, III)
    InspectionRight,
    /// Preferential subscription (Art. 109, IV)
    PreferentialSubscription,
    /// Withdrawal right (Art. 109, V and 137)
    WithdrawalRight,
    /// Vote in general meeting (Art. 110)
    VotingRight,
}

/// Corporations law errors
#[derive(Debug, Clone, Error)]
pub enum CorporationsError {
    /// Insufficient capital
    #[error("Capital social insuficiente (Art. 82): {capital:?}")]
    InsufficientCapital { capital: BrazilianCurrency },

    /// Invalid governance structure
    #[error("Estrutura de governança inválida (Arts. 138-161): {reason}")]
    InvalidGovernance { reason: String },

    /// Dividend distribution error
    #[error("Erro na distribuição de dividendos (Arts. 201-205): {reason}")]
    DividendError { reason: String },

    /// Shareholder rights violation
    #[error("Violação de direitos dos acionistas (Arts. 109-120): {right:?}")]
    ShareholderRightsViolation { right: ShareholderRight },

    /// Invalid share structure
    #[error("Estrutura acionária inválida (Arts. 15-20): {reason}")]
    InvalidShareStructure { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for corporations operations
pub type CorporationsResult<T> = Result<T, CorporationsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corporation_creation() {
        let corp = Corporation::new(
            "ACME S.A.",
            "12345678000190",
            CorporationType::ClosedCorporation,
            BrazilianCurrency::from_reais(1000000),
        );
        assert!(corp.meets_capital_requirement());
        assert!(!corp.capital_aberto);
    }

    #[test]
    fn test_share_types() {
        let common = Share::common();
        assert!(common.voto);

        let preferred = Share::preferred(10);
        assert!(!preferred.voto);
        assert_eq!(preferred.preferencia_dividendo, Some(10));
    }

    #[test]
    fn test_mandatory_dividend() {
        let dividend = Dividend::calculate_mandatory(BrazilianCurrency::from_reais(1000000), 25);
        assert_eq!(dividend.dividendo_total.reais(), 250000); // 25% of 1M
    }

    #[test]
    fn test_governance_structures() {
        let publicly_held = CorporateGovernance::publicly_held();
        assert!(publicly_held.conselho_administracao); // Mandatory

        let closed = CorporateGovernance::closed();
        assert!(!closed.conselho_administracao); // Optional
    }
}
