//! # Environmental Law - Direito Ambiental
//!
//! Brazilian environmental legislation including crimes and liability.
//!
//! ## Overview
//!
//! | Law | Description | Year |
//! |-----|-------------|------|
//! | Lei 6.938/1981 | National Environmental Policy | 1981 |
//! | Lei 9.605/1998 | Environmental Crimes Law | 1998 |
//! | Lei 12.651/2012 | New Forest Code | 2012 |
//! | CF/88, Art. 225 | Constitutional environmental right | 1988 |
//!
//! ## Key Principles
//!
//! | Principle | Description |
//! |-----------|-------------|
//! | Polluter pays | Polluter must prevent and repair damage |
//! | Precaution | Prevention of environmental harm |
//! | Sustainable development | Balance between development and environment |
//! | Strict liability | Liability regardless of fault (objective) |

use crate::common::BrazilianCurrency;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Environmental license (licença ambiental)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentalLicense {
    /// License type
    pub tipo: LicenseType,
    /// License holder
    pub titular: String,
    /// Activity description
    pub atividade: String,
    /// Issue date
    pub data_emissao: NaiveDate,
    /// Expiration date
    pub data_validade: Option<NaiveDate>,
    /// Whether license is valid
    pub valida: bool,
}

/// License types (Resolution CONAMA 237/1997)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseType {
    /// Preliminary license (Licença Prévia - LP)
    /// Approves location and feasibility
    Preliminary,
    /// Installation license (Licença de Instalação - LI)
    /// Authorizes construction
    Installation,
    /// Operating license (Licença de Operação - LO)
    /// Authorizes operation
    Operating,
}

impl EnvironmentalLicense {
    /// Create a new environmental license
    pub fn new(
        tipo: LicenseType,
        titular: impl Into<String>,
        atividade: impl Into<String>,
        data_emissao: NaiveDate,
    ) -> Self {
        Self {
            tipo,
            titular: titular.into(),
            atividade: atividade.into(),
            data_emissao,
            data_validade: None,
            valida: true,
        }
    }

    /// Check if license is still valid
    pub fn is_valid(&self, reference_date: NaiveDate) -> bool {
        if !self.valida {
            return false;
        }

        if let Some(validade) = self.data_validade {
            reference_date <= validade
        } else {
            true
        }
    }
}

/// Environmental crime (Lei 9.605/1998)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalCrime {
    /// Crime type
    pub tipo: CrimeType,
    /// Date of occurrence
    pub data: NaiveDate,
    /// Perpetrator
    pub autor: String,
    /// Description
    pub descricao: String,
    /// Damage assessment
    pub dano: Option<EnvironmentalDamage>,
}

/// Environmental crime types (Lei 9.605/1998)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrimeType {
    /// Crimes against fauna (Arts. 29-37)
    AgainstFauna,
    /// Crimes against flora (Arts. 38-53)
    AgainstFlora,
    /// Pollution crimes (Arts. 54-61)
    Pollution,
    /// Crimes against urban planning (Arts. 62-65)
    AgainstUrbanPlanning,
    /// Administrative crimes (Arts. 66-69)
    Administrative,
}

impl CrimeType {
    /// Get penalty range in months
    pub fn penalty_range(&self) -> (u8, u8) {
        match self {
            Self::AgainstFauna => (6, 12),         // 6 months to 1 year
            Self::AgainstFlora => (1, 4),          // 1 month to 4 years (varies)
            Self::Pollution => (1, 4),             // 1 month to 4 years
            Self::AgainstUrbanPlanning => (6, 12), // 6 months to 1 year
            Self::Administrative => (1, 3),        // 1 month to 3 years
        }
    }
}

/// Environmental damage (dano ambiental)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalDamage {
    /// Affected area in hectares
    pub area_afetada_hectares: Option<f64>,
    /// Number of animals affected
    pub animais_afetados: Option<u32>,
    /// Estimated repair cost
    pub custo_reparacao: Option<BrazilianCurrency>,
    /// Whether damage is reversible
    pub reversivel: bool,
}

/// Environmental liability (Lei 6.938/1981, Art. 14, §1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalLiability {
    /// Liable party
    pub responsavel: String,
    /// Liability type
    pub tipo: LiabilityType,
    /// Damage caused
    pub dano: EnvironmentalDamage,
    /// Obligation imposed
    pub obrigacao: LiabilityObligation,
}

/// Liability types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiabilityType {
    /// Civil liability (strict/objective)
    Civil,
    /// Criminal liability
    Criminal,
    /// Administrative liability
    Administrative,
}

/// Liability obligations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiabilityObligation {
    /// Restore environment (recuperar)
    Restoration {
        /// Restoration plan
        plano: String,
        /// Deadline in days
        prazo_dias: u32,
    },
    /// Compensate for damage (compensar)
    Compensation {
        /// Compensation amount
        valor: BrazilianCurrency,
    },
    /// Fine (multa)
    Fine {
        /// Fine amount
        valor: BrazilianCurrency,
    },
}

/// Forest reserve requirements (Lei 12.651/2012 - New Forest Code)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForestReserve {
    /// Property area in hectares
    pub area_total_hectares: f64,
    /// Biome type
    pub bioma: Biome,
    /// Required legal reserve percentage
    pub percentual_reserva_legal: u8,
    /// Required APP (Permanent Preservation Area) in hectares
    pub app_hectares: f64,
}

/// Brazilian biomes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Biome {
    /// Amazon (80% reserve)
    Amazonia,
    /// Cerrado in Legal Amazon (35% reserve)
    CerradoAmazonia,
    /// Other areas (20% reserve)
    Outros,
    /// Atlantic Forest (20% reserve)
    MataAtlantica,
    /// Pantanal (20% reserve)
    Pantanal,
    /// Pampa (20% reserve)
    Pampa,
}

impl ForestReserve {
    /// Get required legal reserve percentage by biome (Art. 12)
    pub fn required_reserve_percentage(&self) -> u8 {
        match self.bioma {
            Biome::Amazonia => 80,
            Biome::CerradoAmazonia => 35,
            _ => 20,
        }
    }

    /// Calculate required reserve area in hectares
    pub fn required_reserve_hectares(&self) -> f64 {
        self.area_total_hectares * (self.percentual_reserva_legal as f64 / 100.0)
    }

    /// Check if property complies with forest code
    pub fn is_compliant(&self, actual_reserve_hectares: f64) -> bool {
        actual_reserve_hectares >= self.required_reserve_hectares()
    }
}

/// Environmental errors
#[derive(Debug, Clone, Error)]
pub enum EnvironmentalError {
    /// Operating without license
    #[error("Operação sem licença ambiental (Lei 6.938/1981, Art. 10)")]
    NoLicense,

    /// Expired license
    #[error("Licença ambiental expirada: {expiration}")]
    ExpiredLicense { expiration: NaiveDate },

    /// Environmental crime
    #[error("Crime ambiental (Lei 9.605/1998): {crime_type:?}")]
    Crime { crime_type: CrimeType },

    /// Forest code violation
    #[error("Violação do Código Florestal (Lei 12.651/2012): {description}")]
    ForestCodeViolation { description: String },

    /// Insufficient legal reserve
    #[error("Reserva legal insuficiente: requerido {required}ha, atual {actual}ha")]
    InsufficientReserve { required: f64, actual: f64 },

    /// Environmental damage
    #[error("Dano ambiental detectado: {description}")]
    EnvironmentalDamage { description: String },

    /// Validation error
    #[error("Erro de validação: {message}")]
    ValidationError { message: String },
}

/// Result type for environmental operations
pub type EnvironmentalResult<T> = Result<T, EnvironmentalError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environmental_license() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let license = EnvironmentalLicense::new(
            LicenseType::Operating,
            "ACME Ltda.",
            "Indústria química",
            date,
        );
        assert!(license.is_valid(date));
    }

    #[test]
    fn test_crime_penalties() {
        let (min, max) = CrimeType::AgainstFauna.penalty_range();
        assert_eq!(min, 6);
        assert_eq!(max, 12);
    }

    #[test]
    fn test_forest_reserve_amazon() {
        let reserve = ForestReserve {
            area_total_hectares: 100.0,
            bioma: Biome::Amazonia,
            percentual_reserva_legal: 80,
            app_hectares: 10.0,
        };

        assert_eq!(reserve.required_reserve_percentage(), 80);
        assert_eq!(reserve.required_reserve_hectares(), 80.0);
        assert!(reserve.is_compliant(85.0));
        assert!(!reserve.is_compliant(70.0));
    }

    #[test]
    fn test_forest_reserve_cerrado() {
        let reserve = ForestReserve {
            area_total_hectares: 100.0,
            bioma: Biome::CerradoAmazonia,
            percentual_reserva_legal: 35,
            app_hectares: 5.0,
        };

        assert_eq!(reserve.required_reserve_percentage(), 35);
        assert_eq!(reserve.required_reserve_hectares(), 35.0);
    }
}
