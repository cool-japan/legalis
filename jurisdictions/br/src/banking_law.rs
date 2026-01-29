//! # Banking Law - Direito Bancário
//!
//! Brazilian banking and financial system regulations.
//!
//! ## Overview
//!
//! | Authority | Responsibility | Law |
//! |-----------|---------------|-----|
//! | CMN | National Monetary Council (policy) | Lei 4.595/1964 |
//! | BCB | Central Bank (supervision) | Lei 4.595/1964 |
//! | CVM | Securities Commission (capital markets) | Lei 6.385/1976 |
//!
//! ## Key Legislation
//!
//! | Law | Description | Year |
//! |-----|-------------|------|
//! | Lei 4.595/1964 | Financial System Law | 1964 |
//! | Lei 4.728/1965 | Capital Market Law | 1965 |
//! | Lei 6.024/1974 | Banking Intervention/Liquidation | 1974 |
//! | Lei Complementar 105/2001 | Banking Secrecy | 2001 |

use crate::common::BrazilianCurrency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Financial institution (instituição financeira)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinancialInstitution {
    /// Institution name
    pub nome: String,
    /// CNPJ
    pub cnpj: String,
    /// Institution type
    pub tipo: InstitutionType,
    /// BCB authorization number
    pub autorizacao_bcb: String,
    /// Whether authorized to operate
    pub autorizado: bool,
}

/// Financial institution types (Lei 4.595/1964, Art. 17)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstitutionType {
    /// Commercial bank (banco comercial)
    CommercialBank,
    /// Investment bank (banco de investimento)
    InvestmentBank,
    /// Development bank (banco de desenvolvimento)
    DevelopmentBank,
    /// Multiple bank (banco múltiplo)
    MultipleBank,
    /// Savings bank (caixa econômica)
    SavingsBank,
    /// Credit cooperative (cooperativa de crédito)
    CreditCooperative,
    /// Financing company (sociedade de crédito, financiamento e investimento)
    FinancingCompany,
    /// Securities dealer (sociedade corretora)
    SecuritiesDealer,
    /// Securities distributor (sociedade distribuidora)
    SecuritiesDistributor,
}

impl FinancialInstitution {
    /// Create a new financial institution
    pub fn new(
        nome: impl Into<String>,
        cnpj: impl Into<String>,
        tipo: InstitutionType,
        autorizacao: impl Into<String>,
    ) -> Self {
        Self {
            nome: nome.into(),
            cnpj: cnpj.into(),
            tipo,
            autorizacao_bcb: autorizacao.into(),
            autorizado: true,
        }
    }

    /// Check if institution can accept deposits (Art. 17)
    /// Only banks and caixas econômicas
    pub fn can_accept_deposits(&self) -> bool {
        matches!(
            self.tipo,
            InstitutionType::CommercialBank
                | InstitutionType::MultipleBank
                | InstitutionType::SavingsBank
        )
    }

    /// Check if institution can grant loans
    pub fn can_grant_loans(&self) -> bool {
        matches!(
            self.tipo,
            InstitutionType::CommercialBank
                | InstitutionType::InvestmentBank
                | InstitutionType::DevelopmentBank
                | InstitutionType::MultipleBank
                | InstitutionType::FinancingCompany
        )
    }
}

/// Banking operation (operação bancária)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankingOperation {
    /// Operation type
    pub tipo: OperationType,
    /// Financial institution
    pub instituicao: String,
    /// Client
    pub cliente: String,
    /// Operation value
    pub valor: BrazilianCurrency,
    /// Operation date
    pub data: NaiveDate,
    /// Interest rate (annual percentage)
    pub taxa_juros_anual: Option<f64>,
}

/// Banking operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    /// Deposit (depósito)
    Deposit,
    /// Loan (empréstimo)
    Loan,
    /// Financing (financiamento)
    Financing,
    /// Investment (investimento)
    Investment,
    /// Foreign exchange (câmbio)
    ForeignExchange,
    /// Credit card (cartão de crédito)
    CreditCard,
}

impl BankingOperation {
    /// Create a new loan operation
    pub fn loan(
        instituicao: impl Into<String>,
        cliente: impl Into<String>,
        valor: BrazilianCurrency,
        data: NaiveDate,
        taxa_anual: f64,
    ) -> Self {
        Self {
            tipo: OperationType::Loan,
            instituicao: instituicao.into(),
            cliente: cliente.into(),
            valor,
            data,
            taxa_juros_anual: Some(taxa_anual),
        }
    }

    /// Check if interest rate is within legal limits
    /// Usury rate check (Lei 1.521/1951 - economy crimes)
    pub fn check_usury(&self, market_rate: f64) -> Result<(), BankingError> {
        if let Some(taxa) = self.taxa_juros_anual {
            // Brazilian case law considers rates significantly above market as usury
            // Simplified check: rate > market_rate * 2
            if taxa > market_rate * 2.0 {
                return Err(BankingError::UsuraryRate {
                    charged_rate: taxa,
                    market_rate,
                });
            }
        }
        Ok(())
    }
}

/// Banking secrecy (sigilo bancário) - LC 105/2001
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BankingSecrecy {
    /// Account holder
    pub titular: String,
    /// Institution
    pub instituicao: String,
    /// Whether secrecy can be broken
    pub quebra_permitida: bool,
    /// Reason for breaking secrecy
    pub motivo_quebra: Option<SecrecyBreakReason>,
}

/// Reasons for breaking banking secrecy (Art. 1, §4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecrecyBreakReason {
    /// Judicial order (ordem judicial)
    JudicialOrder,
    /// Tax investigation (fiscalização tributária)
    TaxInvestigation,
    /// Parliamentary inquiry (CPI)
    ParliamentaryInquiry,
    /// Central Bank supervision (supervisão do BCB)
    CentralBankSupervision,
}

/// Capital adequacy (adequação de capital) - Basel Accords
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapitalAdequacy {
    /// Tier 1 capital (Nível I)
    pub nivel_i: BrazilianCurrency,
    /// Tier 2 capital (Nível II)
    pub nivel_ii: BrazilianCurrency,
    /// Risk-weighted assets (ativos ponderados pelo risco)
    pub apr: BrazilianCurrency,
}

impl CapitalAdequacy {
    /// Calculate Basel ratio (capital / risk-weighted assets)
    /// Minimum required: 11% (BCB Resolution 4.193/2013)
    pub fn calculate_basel_ratio(&self) -> f64 {
        let total_capital = self.nivel_i.centavos + self.nivel_ii.centavos;
        let apr = self.apr.centavos;

        if apr == 0 {
            return 0.0;
        }

        (total_capital as f64 / apr as f64) * 100.0
    }

    /// Check if meets minimum capital requirement
    pub fn meets_requirement(&self) -> bool {
        self.calculate_basel_ratio() >= 11.0
    }
}

/// Banking intervention (intervenção bancária) - Lei 6.024/1974
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BankingIntervention {
    /// Intervened institution
    pub instituicao: String,
    /// Intervention type
    pub tipo: InterventionType,
    /// Start date
    pub data_inicio: NaiveDate,
    /// Intervener/liquidator
    pub interventor: String,
}

/// Intervention types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterventionType {
    /// Temporary administration (administração especial temporária - RAET)
    TemporaryAdministration,
    /// Intervention (intervenção)
    Intervention,
    /// Extrajudicial liquidation (liquidação extrajudicial)
    ExtrajudicialLiquidation,
}

/// Banking errors
#[derive(Debug, Clone, Error)]
pub enum BankingError {
    /// Unauthorized operation
    #[error("Instituição não autorizada pelo BCB (Lei 4.595/1964, Art. 10)")]
    UnauthorizedInstitution,

    /// Usurious interest rate
    #[error("Taxa de juros abusiva: {charged_rate}% (mercado: {market_rate}%)")]
    UsuraryRate { charged_rate: f64, market_rate: f64 },

    /// Insufficient capital
    #[error("Capital insuficiente: {ratio}% (mínimo 11%)")]
    InsufficientCapital { ratio: f64 },

    /// Banking secrecy violation
    #[error("Violação de sigilo bancário (LC 105/2001): {description}")]
    SecrecyViolation { description: String },

    /// Illegal operation
    #[error("Operação ilegal: {reason}")]
    IllegalOperation { reason: String },

    /// Intervention required
    #[error("Intervenção necessária (Lei 6.024/1974): {reason}")]
    InterventionRequired { reason: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for banking operations
pub type BankingResult<T> = Result<T, BankingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_financial_institution() {
        let bank = FinancialInstitution::new(
            "Banco ACME S.A.",
            "12345678000190",
            InstitutionType::CommercialBank,
            "BCB-12345",
        );
        assert!(bank.can_accept_deposits());
        assert!(bank.can_grant_loans());
    }

    #[test]
    fn test_loan_operation() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let loan = BankingOperation::loan(
            "Banco XYZ",
            "Cliente A",
            BrazilianCurrency::from_reais(100000),
            date,
            15.0,
        );
        assert_eq!(loan.tipo, OperationType::Loan);
        assert!(loan.check_usury(10.0).is_ok()); // 15% < 20% (2x market)
    }

    #[test]
    fn test_usury_detection() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let loan = BankingOperation::loan(
            "Banco ABC",
            "Cliente B",
            BrazilianCurrency::from_reais(50000),
            date,
            50.0, // 50% rate
        );
        assert!(loan.check_usury(10.0).is_err()); // 50% > 20% (2x market)
    }

    #[test]
    fn test_capital_adequacy() {
        let capital = CapitalAdequacy {
            nivel_i: BrazilianCurrency::from_reais(110000),
            nivel_ii: BrazilianCurrency::from_reais(10000),
            apr: BrazilianCurrency::from_reais(1000000),
        };
        let ratio = capital.calculate_basel_ratio();
        assert!((ratio - 12.0).abs() < 0.1); // Should be 12%
        assert!(capital.meets_requirement());
    }

    #[test]
    fn test_insufficient_capital() {
        let capital = CapitalAdequacy {
            nivel_i: BrazilianCurrency::from_reais(80000),
            nivel_ii: BrazilianCurrency::from_reais(10000),
            apr: BrazilianCurrency::from_reais(1000000),
        };
        let ratio = capital.calculate_basel_ratio();
        assert!((ratio - 9.0).abs() < 0.1); // Should be 9%
        assert!(!capital.meets_requirement()); // Below 11% minimum
    }
}
