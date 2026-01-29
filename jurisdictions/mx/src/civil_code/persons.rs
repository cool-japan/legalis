//! Federal Civil Code - Persons (Personas)
//!
//! Covers legal personality, capacity, and individual rights
//! (Código Civil Federal, Libro Primero)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Legal person types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonType {
    /// Natural person (Persona física)
    Natural(NaturalPerson),
    /// Juridical person (Persona moral/jurídica)
    Juridical(JuridicalPerson),
}

/// Natural person (Persona física)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NaturalPerson {
    /// Full name
    pub nombre_completo: String,
    /// RFC (Tax ID)
    pub rfc: Option<String>,
    /// CURP (Population Registry ID)
    pub curp: Option<String>,
    /// Legal capacity
    pub capacidad: LegalCapacity,
}

/// Juridical person (Persona moral)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JuridicalPerson {
    /// Corporate name (Razón social)
    pub razon_social: String,
    /// RFC (Tax ID)
    pub rfc: String,
    /// Legal entity type
    pub tipo_entidad: EntityType,
}

/// Legal capacity (Capacidad jurídica)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalCapacity {
    /// Full capacity (Capacidad plena) - 18+ years
    Full,
    /// Limited capacity (Capacidad limitada) - Minors
    Limited,
    /// Incapacitated (Incapaz) - By court order
    Incapacitated,
}

/// Entity types for juridical persons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    /// Public corporation
    PublicCorporation,
    /// Private corporation
    PrivateCorporation,
    /// Association
    Association,
    /// Foundation
    Foundation,
}

/// Person validation errors
#[derive(Debug, Error)]
pub enum PersonError {
    #[error("Invalid person data: {0}")]
    InvalidData(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Legal capacity restriction: {0}")]
    CapacityRestriction(String),
}

impl NaturalPerson {
    /// Create a new natural person with full capacity
    pub fn new(nombre_completo: String, rfc: Option<String>, curp: Option<String>) -> Self {
        Self {
            nombre_completo,
            rfc,
            curp,
            capacidad: LegalCapacity::Full,
        }
    }

    /// Check if person has full legal capacity
    pub fn has_full_capacity(&self) -> bool {
        matches!(self.capacidad, LegalCapacity::Full)
    }

    /// Validate natural person data
    pub fn validate(&self) -> Result<(), PersonError> {
        if self.nombre_completo.is_empty() {
            return Err(PersonError::MissingField("nombre_completo".to_string()));
        }

        Ok(())
    }
}

impl JuridicalPerson {
    /// Create a new juridical person
    pub fn new(razon_social: String, rfc: String, tipo_entidad: EntityType) -> Self {
        Self {
            razon_social,
            rfc,
            tipo_entidad,
        }
    }

    /// Validate juridical person data
    pub fn validate(&self) -> Result<(), PersonError> {
        if self.razon_social.is_empty() {
            return Err(PersonError::MissingField("razon_social".to_string()));
        }

        if self.rfc.is_empty() {
            return Err(PersonError::MissingField("rfc".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_person() {
        let person = NaturalPerson::new(
            "Juan Pérez".to_string(),
            Some("PEXJ800101XXX".to_string()),
            Some("PEXJ800101HDFRXN00".to_string()),
        );
        assert!(person.has_full_capacity());
        assert!(person.validate().is_ok());
    }

    #[test]
    fn test_juridical_person() {
        let person = JuridicalPerson::new(
            "Empresa SA de CV".to_string(),
            "EMP010101XXX".to_string(),
            EntityType::PrivateCorporation,
        );
        assert!(person.validate().is_ok());
    }
}
