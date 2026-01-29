//! Federal Criminal Code (Código Penal Federal)
//!
//! Covers criminal offenses and penalties

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Criminal offense (Delito)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriminalOffense {
    /// Offense name
    pub nombre: String,
    /// Offense classification
    pub clasificacion: OffenseClassification,
    /// Penalty range
    pub pena: PenaltyRange,
    /// Article reference
    pub articulo: u32,
}

/// Offense classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenseClassification {
    /// Against life and bodily integrity (Contra la vida y la integridad corporal)
    AgainstLife,
    /// Against personal freedom (Contra la libertad personal)
    AgainstFreedom,
    /// Against property (Contra el patrimonio)
    AgainstProperty,
    /// Against public faith (Contra la fe pública)
    AgainstPublicFaith,
    /// Against public administration (Contra la administración pública)
    AgainstPublicAdministration,
    /// Against public health (Contra la salud pública)
    AgainstPublicHealth,
}

/// Penalty range (Penalidad)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PenaltyRange {
    /// Minimum prison term (months)
    pub prision_minima_meses: u32,
    /// Maximum prison term (months)
    pub prision_maxima_meses: u32,
    /// Fine in UMA units
    pub multa_uma: Option<(u32, u32)>,
}

/// Criminal responsibility (Responsabilidad penal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriminalResponsibility {
    /// Full responsibility
    Full,
    /// Diminished responsibility
    Diminished,
    /// No responsibility
    None,
}

/// Criminal errors
#[derive(Debug, Error)]
pub enum CriminalError {
    #[error("Invalid offense data: {0}")]
    InvalidData(String),
}

impl CriminalOffense {
    /// Create new criminal offense
    pub fn new(
        nombre: String,
        clasificacion: OffenseClassification,
        pena: PenaltyRange,
        articulo: u32,
    ) -> Self {
        Self {
            nombre,
            clasificacion,
            pena,
            articulo,
        }
    }
}

impl PenaltyRange {
    /// Calculate average prison term in months
    pub fn average_prison_months(&self) -> u32 {
        (self.prision_minima_meses + self.prision_maxima_meses) / 2
    }

    /// Check if offense is serious (grave)
    pub fn is_serious(&self) -> bool {
        self.prision_maxima_meses >= 60 // 5 years or more
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penalty_range() {
        let penalty = PenaltyRange {
            prision_minima_meses: 12,
            prision_maxima_meses: 60,
            multa_uma: Some((100, 500)),
        };

        assert_eq!(penalty.average_prison_months(), 36);
        assert!(penalty.is_serious());
    }
}
