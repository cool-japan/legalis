//! Federal Civil Code - Property (Bienes)
//!
//! Covers property rights, ownership, and possession
//! (Código Civil Federal, Libro Segundo)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Property types (Tipos de bienes)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Immovable property (Bienes inmuebles) - Real estate
    Immovable(ImmovableProperty),
    /// Movable property (Bienes muebles) - Personal property
    Movable(MovableProperty),
}

/// Immovable property (Real estate)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImmovableProperty {
    /// Property description
    pub descripcion: String,
    /// Cadastral key (Clave catastral)
    pub clave_catastral: Option<String>,
    /// Public registry folio
    pub folio_real: Option<String>,
}

/// Movable property (Personal property)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovableProperty {
    /// Property description
    pub descripcion: String,
    /// Serial number or identifier
    pub numero_serie: Option<String>,
}

/// Ownership types (Tipos de propiedad)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipType {
    /// Full ownership (Propiedad plena)
    Full,
    /// Bare ownership (Nuda propiedad)
    Bare,
    /// Usufruct (Usufructo)
    Usufruct,
    /// Joint ownership (Copropiedad)
    Joint,
}

/// Property rights (Derechos reales)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyRight {
    /// Type of right
    pub tipo_derecho: RightType,
    /// Property subject to the right
    pub bien: PropertyType,
    /// Owner or holder of the right
    pub titular: String,
}

/// Types of property rights
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightType {
    /// Ownership (Propiedad)
    Ownership,
    /// Possession (Posesión)
    Possession,
    /// Usufruct (Usufructo)
    Usufruct,
    /// Easement (Servidumbre)
    Easement,
    /// Mortgage (Hipoteca)
    Mortgage,
    /// Pledge (Prenda)
    Pledge,
}

/// Property errors
#[derive(Debug, Error)]
pub enum PropertyError {
    #[error("Invalid property data: {0}")]
    InvalidData(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Property right conflict: {0}")]
    RightConflict(String),
}

impl ImmovableProperty {
    /// Create new immovable property
    pub fn new(descripcion: String) -> Self {
        Self {
            descripcion,
            clave_catastral: None,
            folio_real: None,
        }
    }

    /// Validate immovable property
    pub fn validate(&self) -> Result<(), PropertyError> {
        if self.descripcion.is_empty() {
            return Err(PropertyError::MissingField("descripcion".to_string()));
        }
        Ok(())
    }
}

impl MovableProperty {
    /// Create new movable property
    pub fn new(descripcion: String) -> Self {
        Self {
            descripcion,
            numero_serie: None,
        }
    }

    /// Validate movable property
    pub fn validate(&self) -> Result<(), PropertyError> {
        if self.descripcion.is_empty() {
            return Err(PropertyError::MissingField("descripcion".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immovable_property() {
        let property = ImmovableProperty::new("Casa en CDMX".to_string());
        assert!(property.validate().is_ok());
    }

    #[test]
    fn test_movable_property() {
        let property = MovableProperty::new("Vehículo".to_string());
        assert!(property.validate().is_ok());
    }
}
