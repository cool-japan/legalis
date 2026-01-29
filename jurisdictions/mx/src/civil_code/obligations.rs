//! Federal Civil Code - Obligations (Obligaciones)
//!
//! Covers obligations, performance, and breach
//! (Código Civil Federal, Libro Cuarto, Primera Parte)

use crate::common::MexicanCurrency;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Obligation types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObligationType {
    /// Give something (Dar)
    Give(GiveObligation),
    /// Do something (Hacer)
    Do(DoObligation),
    /// Not do something (No hacer)
    NotDo(NotDoObligation),
}

/// Give obligation (Obligación de dar)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiveObligation {
    /// Description of what must be given
    pub objeto: String,
    /// Monetary value if applicable
    pub valor: Option<MexicanCurrency>,
}

/// Do obligation (Obligación de hacer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoObligation {
    /// Description of the action
    pub descripcion: String,
    /// Deadline for performance
    pub plazo: Option<DateTime<Utc>>,
}

/// Not do obligation (Obligación de no hacer)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotDoObligation {
    /// Description of what must not be done
    pub descripcion: String,
    /// Duration of the prohibition
    pub duracion: Option<DateTime<Utc>>,
}

/// Obligation source (Fuentes de las obligaciones)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationSource {
    /// Contract (Contrato)
    Contract,
    /// Unilateral declaration (Declaración unilateral)
    UnilateralDeclaration,
    /// Tort (Acto ilícito)
    Tort,
    /// Unjust enrichment (Enriquecimiento sin causa)
    UnjustEnrichment,
    /// Management of another's affairs (Gestión de negocios)
    ManagementOfAffairs,
    /// Law (Ley)
    Law,
}

/// Performance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceStatus {
    /// Pending
    Pending,
    /// Performed
    Performed,
    /// Breached
    Breached,
    /// Impossible
    Impossible,
}

/// Breach of obligation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Breach {
    /// Description of the breach
    pub descripcion: String,
    /// Date of breach
    pub fecha_incumplimiento: DateTime<Utc>,
    /// Damages claimed
    pub danos_reclamados: Option<MexicanCurrency>,
    /// Type of breach
    pub tipo: BreachType,
}

/// Types of breach
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Total breach (Incumplimiento total)
    Total,
    /// Partial breach (Incumplimiento parcial)
    Partial,
    /// Delayed performance (Mora)
    Delay,
}

/// Obligation errors
#[derive(Debug, Error)]
pub enum ObligationError {
    #[error("Invalid obligation: {0}")]
    Invalid(String),
    #[error("Performance not possible: {0}")]
    ImpossiblePerformance(String),
    #[error("Breach of obligation: {0}")]
    Breach(String),
}

impl GiveObligation {
    /// Create new give obligation
    pub fn new(objeto: String, valor: Option<MexicanCurrency>) -> Self {
        Self { objeto, valor }
    }

    /// Validate give obligation
    pub fn validate(&self) -> Result<(), ObligationError> {
        if self.objeto.is_empty() {
            return Err(ObligationError::Invalid(
                "objeto cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

impl DoObligation {
    /// Create new do obligation
    pub fn new(descripcion: String, plazo: Option<DateTime<Utc>>) -> Self {
        Self { descripcion, plazo }
    }

    /// Check if obligation is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(plazo) = self.plazo {
            plazo < Utc::now()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_give_obligation() {
        let obligation = GiveObligation::new(
            "Entregar mercancía".to_string(),
            Some(MexicanCurrency::from_pesos(1000)),
        );
        assert!(obligation.validate().is_ok());
    }

    #[test]
    fn test_do_obligation() {
        let obligation = DoObligation::new("Prestar servicio".to_string(), None);
        assert!(!obligation.is_overdue());
    }
}
