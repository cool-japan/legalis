//! Company Law (Ley General de Sociedades Mercantiles - LGSM)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Company types (Tipos de sociedades)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompanyType {
    /// Stock corporation (Sociedad Anónima - SA)
    SA(StockCorporation),
    /// Limited liability company (Sociedad de Responsabilidad Limitada - SRL)
    SRL(LimitedLiabilityCompany),
    /// Variable capital company (de Capital Variable - CV)
    CV(VariableCapitalCompany),
    /// Partnership (Sociedad en Nombre Colectivo)
    Partnership,
    /// Limited partnership (Sociedad en Comandita)
    LimitedPartnership,
}

/// Stock corporation (SA)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StockCorporation {
    /// Company name (Denominación social)
    pub denominacion: String,
    /// Corporate purpose (Objeto social)
    pub objeto_social: String,
    /// Share capital
    pub capital_social: i64,
    /// Number of shareholders
    pub num_accionistas: u32,
    /// Board of directors
    pub consejo_administracion: Vec<String>,
}

/// Limited liability company (SRL)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitedLiabilityCompany {
    /// Company name
    pub razon_social: String,
    /// Corporate purpose
    pub objeto_social: String,
    /// Capital
    pub capital_social: i64,
    /// Number of partners (max 50)
    pub num_socios: u32,
    /// Managers
    pub gerentes: Vec<String>,
}

/// Variable capital company (CV)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableCapitalCompany {
    /// Base company type
    pub tipo_base: String, // "SA" or "SRL"
    /// Minimum capital
    pub capital_minimo: i64,
    /// Maximum capital
    pub capital_maximo: i64,
}

/// Corporate governance requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceRequirements {
    /// General assembly required
    pub asamblea_general: bool,
    /// Board of directors required
    pub consejo_administracion: bool,
    /// Statutory auditor required
    pub comisario: bool,
}

/// Company errors
#[derive(Debug, Error)]
pub enum CompanyError {
    #[error("Invalid company structure: {0}")]
    InvalidStructure(String),
    #[error("Insufficient capital: minimum {0}")]
    InsufficientCapital(i64),
    #[error("Governance violation: {0}")]
    GovernanceViolation(String),
}

impl StockCorporation {
    /// Minimum capital for SA (Article 89)
    pub const MINIMUM_CAPITAL: i64 = 5_000_000; // 50,000 pesos (in cents)

    /// Create new SA
    pub fn new(
        denominacion: String,
        objeto_social: String,
        capital_social: i64,
        num_accionistas: u32,
    ) -> Result<Self, CompanyError> {
        if capital_social < Self::MINIMUM_CAPITAL {
            return Err(CompanyError::InsufficientCapital(Self::MINIMUM_CAPITAL));
        }

        if num_accionistas < 2 {
            return Err(CompanyError::InvalidStructure(
                "SA requires at least 2 shareholders".to_string(),
            ));
        }

        Ok(Self {
            denominacion,
            objeto_social,
            capital_social,
            num_accionistas,
            consejo_administracion: Vec::new(),
        })
    }

    /// Validate SA structure
    pub fn validate(&self) -> Result<(), CompanyError> {
        if self.capital_social < Self::MINIMUM_CAPITAL {
            return Err(CompanyError::InsufficientCapital(Self::MINIMUM_CAPITAL));
        }

        if self.num_accionistas < 2 {
            return Err(CompanyError::InvalidStructure(
                "SA requires at least 2 shareholders".to_string(),
            ));
        }

        Ok(())
    }
}

impl LimitedLiabilityCompany {
    /// Maximum number of partners (Article 58)
    pub const MAX_PARTNERS: u32 = 50;

    /// Create new SRL
    pub fn new(
        razon_social: String,
        objeto_social: String,
        capital_social: i64,
        num_socios: u32,
    ) -> Result<Self, CompanyError> {
        if num_socios < 2 {
            return Err(CompanyError::InvalidStructure(
                "SRL requires at least 2 partners".to_string(),
            ));
        }

        if num_socios > Self::MAX_PARTNERS {
            return Err(CompanyError::InvalidStructure(format!(
                "SRL cannot exceed {} partners",
                Self::MAX_PARTNERS
            )));
        }

        Ok(Self {
            razon_social,
            objeto_social,
            capital_social,
            num_socios,
            gerentes: Vec::new(),
        })
    }

    /// Validate SRL structure
    pub fn validate(&self) -> Result<(), CompanyError> {
        if self.num_socios < 2 {
            return Err(CompanyError::InvalidStructure(
                "SRL requires at least 2 partners".to_string(),
            ));
        }

        if self.num_socios > Self::MAX_PARTNERS {
            return Err(CompanyError::InvalidStructure(format!(
                "SRL cannot exceed {} partners",
                Self::MAX_PARTNERS
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sa() {
        let sa = StockCorporation::new(
            "Empresa SA de CV".to_string(),
            "Comercio".to_string(),
            10_000_000,
            5,
        );

        assert!(sa.is_ok());
        assert!(sa.unwrap().validate().is_ok());
    }

    #[test]
    fn test_create_srl() {
        let srl = LimitedLiabilityCompany::new(
            "Empresa SRL".to_string(),
            "Servicios".to_string(),
            5_000_000,
            3,
        );

        assert!(srl.is_ok());
        assert!(srl.unwrap().validate().is_ok());
    }

    #[test]
    fn test_srl_max_partners() {
        let srl = LimitedLiabilityCompany::new(
            "Empresa SRL".to_string(),
            "Servicios".to_string(),
            5_000_000,
            51,
        );

        assert!(srl.is_err());
    }
}
