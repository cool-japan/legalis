//! Intellectual Property Law

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Intellectual property types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IPType {
    /// Patent (Patente)
    Patent(Patent),
    /// Trademark (Marca)
    Trademark(Trademark),
    /// Copyright (Derecho de autor)
    Copyright(Copyright),
    /// Industrial design (Diseño industrial)
    IndustrialDesign(IndustrialDesign),
}

/// Patent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Patent {
    /// Patent title
    pub titulo: String,
    /// Inventor(s)
    pub inventores: Vec<String>,
    /// Application number
    pub numero_solicitud: String,
    /// Filing date
    pub fecha_solicitud: DateTime<Utc>,
    /// Patent type
    pub tipo: PatentType,
}

/// Patent type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatentType {
    /// Invention patent (20 years)
    Invention,
    /// Utility model (10 years)
    UtilityModel,
}

/// Trademark
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trademark {
    /// Trademark name
    pub nombre: String,
    /// Owner
    pub titular: String,
    /// Registration number
    pub numero_registro: Option<String>,
    /// Nice classification
    pub clase_nice: Vec<u8>,
    /// Trademark type
    pub tipo: TrademarkType,
}

/// Trademark type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrademarkType {
    /// Nominative
    Nominativa,
    /// Figurative
    Figurativa,
    /// Mixed
    Mixta,
    /// Three-dimensional
    Tridimensional,
}

/// Copyright
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Copyright {
    /// Work title
    pub titulo: String,
    /// Author(s)
    pub autores: Vec<String>,
    /// Work type
    pub tipo_obra: WorkType,
    /// Registration number
    pub numero_registro: Option<String>,
}

/// Work type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkType {
    /// Literary
    Literary,
    /// Musical
    Musical,
    /// Artistic
    Artistic,
    /// Software
    Software,
    /// Database
    Database,
}

/// Industrial design
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndustrialDesign {
    /// Design title
    pub titulo: String,
    /// Designer(s)
    pub disenadores: Vec<String>,
    /// Application number
    pub numero_solicitud: String,
}

/// IP protection duration (in years)
pub mod protection_duration {
    /// Patent protection (Article 23 LPI)
    pub const PATENT_INVENTION: u8 = 20;
    pub const PATENT_UTILITY_MODEL: u8 = 10;

    /// Trademark protection (renewable every 10 years)
    pub const TRADEMARK: u8 = 10;

    /// Copyright protection (life + 100 years in Mexico)
    pub const COPYRIGHT_AFTER_DEATH: u8 = 100;

    /// Industrial design
    pub const INDUSTRIAL_DESIGN: u8 = 15;
}

/// IP errors
#[derive(Debug, Error)]
pub enum IPError {
    #[error("Invalid IP registration: {0}")]
    InvalidRegistration(String),
    #[error("Prior art exists: {0}")]
    PriorArt(String),
}

impl Patent {
    /// Get protection duration in years
    pub fn protection_years(&self) -> u8 {
        match self.tipo {
            PatentType::Invention => protection_duration::PATENT_INVENTION,
            PatentType::UtilityModel => protection_duration::PATENT_UTILITY_MODEL,
        }
    }
}

impl Trademark {
    /// Check if trademark is renewable
    pub fn is_renewable(&self) -> bool {
        true // Trademarks are renewable indefinitely
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patent_protection_duration() {
        let patent = Patent {
            titulo: "Nueva invención".to_string(),
            inventores: vec!["Juan Pérez".to_string()],
            numero_solicitud: "MX/a/2024/000001".to_string(),
            fecha_solicitud: Utc::now(),
            tipo: PatentType::Invention,
        };

        assert_eq!(patent.protection_years(), 20);
    }

    #[test]
    fn test_trademark_renewable() {
        let trademark = Trademark {
            nombre: "MARCA".to_string(),
            titular: "Empresa SA".to_string(),
            numero_registro: Some("123456".to_string()),
            clase_nice: vec![35],
            tipo: TrademarkType::Nominativa,
        };

        assert!(trademark.is_renewable());
    }
}
