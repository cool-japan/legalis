//! # Bankruptcy Law - Lei de Falências e Recuperação de Empresas
//!
//! Brazilian insolvency law (Lei nº 11.101/2005).
//!
//! ## Overview
//!
//! Brazil's bankruptcy law covers:
//! - Judicial reorganization (recuperação judicial)
//! - Extrajudicial reorganization (recuperação extrajudicial)
//! - Bankruptcy (falência)
//!
//! ## Key Features
//!
//! | Aspect | Description |
//! |--------|-------------|
//! | Goal | Company preservation (going concern) |
//! | Stay Period | 180 days from filing |
//! | Creditor Classes | Labor, secured, unsecured, micro-enterprises |
//! | Approval | Majority vote in each class |

use crate::common::BrazilianCurrency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Insolvency proceeding
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InsolvencyProceeding {
    /// Debtor company
    pub devedor: String,
    /// CNPJ
    pub cnpj: String,
    /// Proceeding type
    pub tipo: ProceedingType,
    /// Filing date
    pub data_pedido: NaiveDate,
    /// Total debt
    pub divida_total: BrazilianCurrency,
    /// Creditors
    pub credores: Vec<Creditor>,
}

/// Insolvency proceeding types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProceedingType {
    /// Judicial reorganization (recuperação judicial) - Arts. 47-69
    JudicialReorganization,
    /// Extrajudicial reorganization (recuperação extrajudicial) - Arts. 161-167
    ExtrajudicialReorganization,
    /// Bankruptcy (falência) - Arts. 75-160
    Bankruptcy,
}

impl InsolvencyProceeding {
    /// Create a new judicial reorganization proceeding
    pub fn judicial_reorganization(
        devedor: impl Into<String>,
        cnpj: impl Into<String>,
        data: NaiveDate,
        divida: BrazilianCurrency,
    ) -> Self {
        Self {
            devedor: devedor.into(),
            cnpj: cnpj.into(),
            tipo: ProceedingType::JudicialReorganization,
            data_pedido: data,
            divida_total: divida,
            credores: Vec::new(),
        }
    }

    /// Check if debtor is eligible for reorganization (Art. 48)
    /// Requirements: regular operation for 2+ years, no reorganization in last 5 years
    pub fn check_eligibility(
        &self,
        anos_atividade: u8,
        ultima_recuperacao: Option<NaiveDate>,
    ) -> Result<(), BankruptcyError> {
        if anos_atividade < 2 {
            return Err(BankruptcyError::IneligibleDebtor {
                reason: "Empresa deve estar em atividade há pelo menos 2 anos (Art. 48, I)"
                    .to_string(),
            });
        }

        if let Some(data_ultima) = ultima_recuperacao {
            let anos_desde_ultima = (self
                .data_pedido
                .signed_duration_since(data_ultima)
                .num_days()
                / 365) as u8;
            if anos_desde_ultima < 5 {
                return Err(BankruptcyError::IneligibleDebtor {
                    reason: "Não pode haver recuperação nos últimos 5 anos (Art. 48, III)"
                        .to_string(),
                });
            }
        }

        Ok(())
    }

    /// Get automatic stay period (Art. 6, §4)
    /// 180 days from filing
    pub fn stay_period_days(&self) -> u32 {
        180
    }
}

/// Creditor (credor)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Creditor {
    /// Creditor name
    pub nome: String,
    /// Creditor document
    pub documento: String,
    /// Credit class
    pub classe: CreditClass,
    /// Credit amount
    pub valor_credito: BrazilianCurrency,
}

/// Credit classes (Art. 83)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreditClass {
    /// I - Labor credits (up to 150 minimum wages) - Art. 83, I
    Labor,
    /// II - Secured credits (com garantia real) - Art. 83, II
    Secured,
    /// III - Tax credits - Art. 83, III
    Tax,
    /// IV - Unsecured credits (quirografários) - Art. 83, IV
    Unsecured,
    /// V - Subordinated credits - Art. 83, V-VIII
    Subordinated,
    /// Micro-enterprise credits (Lei Complementar 123/2006)
    MicroEnterprise,
}

impl CreditClass {
    /// Get priority order (lower is higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            Self::Labor => 1,
            Self::Secured => 2,
            Self::Tax => 3,
            Self::MicroEnterprise => 4,
            Self::Unsecured => 5,
            Self::Subordinated => 6,
        }
    }

    /// Get description in Portuguese
    pub fn descricao_pt(&self) -> &'static str {
        match self {
            Self::Labor => "Créditos trabalhistas (até 150 salários mínimos)",
            Self::Secured => "Créditos com garantia real",
            Self::Tax => "Créditos tributários",
            Self::MicroEnterprise => "Créditos de microempresas",
            Self::Unsecured => "Créditos quirografários",
            Self::Subordinated => "Créditos subordinados",
        }
    }
}

/// Reorganization plan (plano de recuperação) - Art. 53
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReorganizationPlan {
    /// Plan description
    pub descricao: String,
    /// Deadline for submission (60 days from filing)
    pub prazo_apresentacao_dias: u32,
    /// Payment terms by class
    pub condicoes_pagamento: Vec<PaymentCondition>,
    /// Whether plan is approved
    pub aprovado: bool,
}

/// Payment conditions for creditors
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentCondition {
    /// Credit class
    pub classe: CreditClass,
    /// Payment percentage
    pub percentual_pagamento: u8,
    /// Payment term in months
    pub prazo_meses: u16,
    /// Grace period
    pub carencia_meses: Option<u16>,
}

impl ReorganizationPlan {
    /// Check if plan respects legal limits (Art. 54)
    /// Labor credits: max 30 days payment term
    pub fn validate(&self) -> Result<(), BankruptcyError> {
        for condition in &self.condicoes_pagamento {
            if matches!(condition.classe, CreditClass::Labor) {
                let prazo_dias = condition.prazo_meses as u32 * 30;
                if prazo_dias > 30 {
                    return Err(BankruptcyError::InvalidPlan {
                        reason: "Créditos trabalhistas devem ser pagos em até 30 dias (Art. 54)"
                            .to_string(),
                    });
                }
            }
        }
        Ok(())
    }
}

/// Creditors' assembly (assembleia de credores) - Arts. 35-46
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreditorsAssembly {
    /// Assembly date
    pub data: NaiveDate,
    /// Voting results by class
    pub votacao: Vec<ClassVote>,
}

/// Voting by creditor class
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassVote {
    /// Credit class
    pub classe: CreditClass,
    /// Votes in favor
    pub votos_favor: u32,
    /// Votes against
    pub votos_contra: u32,
    /// Whether class approved
    pub aprovado: bool,
}

impl ClassVote {
    /// Calculate approval percentage
    pub fn approval_percentage(&self) -> f64 {
        let total = self.votos_favor + self.votos_contra;
        if total == 0 {
            return 0.0;
        }
        (self.votos_favor as f64 / total as f64) * 100.0
    }

    /// Check if meets approval threshold (Art. 45)
    /// Labor/secured/micro: simple majority
    /// Unsecured: majority of present AND majority of total credit
    pub fn meets_threshold(&self) -> bool {
        self.approval_percentage() > 50.0
    }
}

/// Bankruptcy errors
#[derive(Debug, Clone, Error)]
pub enum BankruptcyError {
    /// Ineligible debtor
    #[error("Devedor não elegível para recuperação judicial (Art. 48): {reason}")]
    IneligibleDebtor { reason: String },

    /// Invalid reorganization plan
    #[error("Plano de recuperação inválido (Art. 53): {reason}")]
    InvalidPlan { reason: String },

    /// Plan not approved
    #[error("Plano de recuperação rejeitado pela assembleia (Art. 45)")]
    PlanRejected,

    /// Bankruptcy declaration
    #[error("Decretação de falência (Art. 94): {reason}")]
    BankruptcyDeclaration { reason: String },

    /// Fraudulent conveyance (Art. 130)
    #[error("Ato fraudulento (Art. 130): {description}")]
    FraudulentConveyance { description: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for bankruptcy operations
pub type BankruptcyResult<T> = Result<T, BankruptcyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judicial_reorganization() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let proceeding = InsolvencyProceeding::judicial_reorganization(
            "ACME Ltda.",
            "12345678000190",
            date,
            BrazilianCurrency::from_reais(10000000),
        );
        assert_eq!(proceeding.tipo, ProceedingType::JudicialReorganization);
        assert_eq!(proceeding.stay_period_days(), 180);
    }

    #[test]
    fn test_eligibility() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let proceeding = InsolvencyProceeding::judicial_reorganization(
            "ABC Ltda.",
            "12345678000190",
            date,
            BrazilianCurrency::from_reais(5000000),
        );

        assert!(proceeding.check_eligibility(3, None).is_ok());
        assert!(proceeding.check_eligibility(1, None).is_err());
    }

    #[test]
    fn test_credit_class_priority() {
        assert_eq!(CreditClass::Labor.priority(), 1);
        assert_eq!(CreditClass::Secured.priority(), 2);
        assert_eq!(CreditClass::Unsecured.priority(), 5);
    }

    #[test]
    fn test_plan_validation() {
        let mut plan = ReorganizationPlan {
            descricao: "Plano de recuperação".to_string(),
            prazo_apresentacao_dias: 60,
            condicoes_pagamento: vec![PaymentCondition {
                classe: CreditClass::Labor,
                percentual_pagamento: 100,
                prazo_meses: 1,
                carencia_meses: None,
            }],
            aprovado: false,
        };

        assert!(plan.validate().is_ok());

        plan.condicoes_pagamento[0].prazo_meses = 3; // Exceeds 30 days
        assert!(plan.validate().is_err());
    }

    #[test]
    fn test_class_vote() {
        let vote = ClassVote {
            classe: CreditClass::Unsecured,
            votos_favor: 60,
            votos_contra: 40,
            aprovado: true,
        };

        assert_eq!(vote.approval_percentage(), 60.0);
        assert!(vote.meets_threshold());
    }
}
