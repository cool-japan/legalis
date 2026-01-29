//! # Limited Liability Companies - Sociedade Limitada
//!
//! Brazilian limited liability companies (Arts. 1052-1087, Civil Code).

use crate::common::BrazilianCurrency;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Limited liability company (Ltda.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitedLiabilityCompany {
    /// Company name (must end with "Ltda." or "Limitada")
    pub nome_empresarial: String,
    /// CNPJ
    pub cnpj: String,
    /// Share capital (capital social)
    pub capital_social: BrazilianCurrency,
    /// Partners (sócios)
    pub socios: Vec<Partner>,
    /// Whether capital is fully paid
    pub capital_integralizado: bool,
}

/// Partner (sócio)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Partner {
    /// Partner name
    pub nome: String,
    /// Partner document (CPF or CNPJ)
    pub documento: String,
    /// Quota holdings
    pub quotas: u64,
    /// Capital contribution
    pub contribuicao: BrazilianCurrency,
    /// Whether contribution is paid
    pub integralizado: bool,
}

impl Partner {
    /// Create a new partner
    pub fn new(
        nome: impl Into<String>,
        documento: impl Into<String>,
        quotas: u64,
        contribuicao: BrazilianCurrency,
    ) -> Self {
        Self {
            nome: nome.into(),
            documento: documento.into(),
            quotas,
            contribuicao,
            integralizado: false,
        }
    }

    /// Mark contribution as paid
    pub fn mark_paid(mut self) -> Self {
        self.integralizado = true;
        self
    }

    /// Calculate ownership percentage
    pub fn ownership_percentage(&self, total_quotas: u64) -> f64 {
        if total_quotas == 0 {
            return 0.0;
        }
        (self.quotas as f64 / total_quotas as f64) * 100.0
    }
}

impl LimitedLiabilityCompany {
    /// Create a new limited liability company
    pub fn new(
        nome: impl Into<String>,
        cnpj: impl Into<String>,
        capital: BrazilianCurrency,
    ) -> Result<Self, LimitedLiabilityError> {
        let nome = nome.into();

        // Name must end with "Ltda." or "Limitada" (Art. 1158)
        if !nome.ends_with("Ltda.") && !nome.ends_with("Limitada") {
            return Err(LimitedLiabilityError::InvalidName {
                reason: "Nome deve terminar com 'Ltda.' ou 'Limitada'".to_string(),
            });
        }

        Ok(Self {
            nome_empresarial: nome,
            cnpj: cnpj.into(),
            capital_social: capital,
            socios: Vec::new(),
            capital_integralizado: false,
        })
    }

    /// Add partner
    pub fn add_partner(mut self, partner: Partner) -> Self {
        self.socios.push(partner);
        self
    }

    /// Calculate total quotas
    pub fn total_quotas(&self) -> u64 {
        self.socios.iter().map(|s| s.quotas).sum()
    }

    /// Calculate total contributions
    pub fn total_contributions(&self) -> BrazilianCurrency {
        let total: i64 = self.socios.iter().map(|s| s.contribuicao.centavos).sum();
        BrazilianCurrency::from_centavos(total)
    }

    /// Check if capital is fully subscribed
    pub fn is_capital_subscribed(&self) -> bool {
        self.total_contributions().centavos >= self.capital_social.centavos
    }

    /// Check if all contributions are paid (Art. 1052)
    pub fn are_contributions_paid(&self) -> bool {
        self.socios.iter().all(|s| s.integralizado)
    }
}

/// Partner liability (Arts. 1052-1053)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartnerLiability {
    /// Partner
    pub socio: String,
    /// Whether liable for unpaid capital
    pub responsavel_capital: bool,
    /// Whether solidarily liable (before capital payment)
    pub responsabilidade_solidaria: bool,
}

impl PartnerLiability {
    /// Calculate partner liability (Art. 1052)
    /// Partners are liable up to the total unpaid capital
    pub fn calculate_liability(
        partner: &Partner,
        total_capital: BrazilianCurrency,
        paid_capital: BrazilianCurrency,
    ) -> Self {
        let unpaid = total_capital.centavos > paid_capital.centavos;

        Self {
            socio: partner.nome.clone(),
            responsavel_capital: !partner.integralizado,
            responsabilidade_solidaria: unpaid,
        }
    }
}

/// Partner withdrawal (retirada de sócio) - Arts. 1029-1032
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartnerWithdrawal {
    /// Withdrawing partner
    pub socio: String,
    /// Withdrawal reason
    pub motivo: WithdrawalReason,
    /// Reimbursement value (haveres)
    pub valor_reembolso: BrazilianCurrency,
}

/// Withdrawal reasons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WithdrawalReason {
    /// By right (de pleno direito) - Art. 1029
    ByRight,
    /// Judicial withdrawal (via judicial)
    Judicial,
    /// Expulsion (exclusão)
    Expulsion,
    /// Death
    Death,
}

/// Management structure (Arts. 1060-1065)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Management {
    /// Managers (administradores)
    pub administradores: Vec<Manager>,
    /// Whether requires unanimity for certain acts
    pub unanimidade: bool,
}

/// Manager (administrador)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manager {
    /// Manager name
    pub nome: String,
    /// Whether is also a partner
    pub socio: bool,
    /// Management powers
    pub poderes: Vec<String>,
}

/// Limited liability errors
#[derive(Debug, Clone, Error)]
pub enum LimitedLiabilityError {
    /// Invalid company name
    #[error("Nome empresarial inválido (Art. 1158): {reason}")]
    InvalidName { reason: String },

    /// Capital not subscribed
    #[error("Capital social não subscrito (Art. 1052)")]
    CapitalNotSubscribed,

    /// Contributions not paid
    #[error("Quotas não integralizadas (Art. 1052): {partner}")]
    ContributionsNotPaid { partner: String },

    /// Invalid partner structure
    #[error("Estrutura societária inválida: {reason}")]
    InvalidPartnerStructure { reason: String },

    /// Withdrawal error
    #[error("Erro na retirada de sócio (Arts. 1029-1032): {reason}")]
    WithdrawalError { reason: String },

    /// Management error
    #[error("Erro na administração (Arts. 1060-1065): {reason}")]
    ManagementError { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for limited liability operations
pub type LimitedLiabilityResult<T> = Result<T, LimitedLiabilityError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_creation() {
        let company = LimitedLiabilityCompany::new(
            "ABC Ltda.",
            "12345678000190",
            BrazilianCurrency::from_reais(100000),
        )
        .expect("valid company");
        assert_eq!(company.nome_empresarial, "ABC Ltda.");
    }

    #[test]
    fn test_invalid_name() {
        let result = LimitedLiabilityCompany::new(
            "ABC", // Missing "Ltda."
            "12345678000190",
            BrazilianCurrency::from_reais(100000),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_partner_ownership() {
        let partner = Partner::new(
            "João Silva",
            "12345678909",
            40,
            BrazilianCurrency::from_reais(40000),
        );
        let ownership = partner.ownership_percentage(100);
        assert_eq!(ownership, 40.0);
    }

    #[test]
    fn test_capital_subscription() {
        let company = LimitedLiabilityCompany::new(
            "XYZ Ltda.",
            "12345678000190",
            BrazilianCurrency::from_reais(100000),
        )
        .expect("valid company")
        .add_partner(
            Partner::new(
                "Sócio A",
                "11111111111",
                50,
                BrazilianCurrency::from_reais(50000),
            )
            .mark_paid(),
        )
        .add_partner(
            Partner::new(
                "Sócio B",
                "22222222222",
                50,
                BrazilianCurrency::from_reais(50000),
            )
            .mark_paid(),
        );

        assert!(company.is_capital_subscribed());
        assert!(company.are_contributions_paid());
    }
}
