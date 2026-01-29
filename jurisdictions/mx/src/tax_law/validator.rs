//! Tax law validation

use super::types::Taxpayer;
use crate::common::validate_rfc;
use thiserror::Error;

/// Tax validation error
#[derive(Debug, Error)]
pub enum TaxValidationError {
    #[error("Invalid RFC: {0}")]
    InvalidRFC(String),
    #[error("Invalid taxpayer data: {0}")]
    InvalidTaxpayer(String),
}

/// Validate taxpayer
pub fn validate_taxpayer(taxpayer: &Taxpayer) -> Result<(), TaxValidationError> {
    // Validate RFC
    if let Err(e) = validate_rfc(&taxpayer.rfc) {
        return Err(TaxValidationError::InvalidRFC(e.to_string()));
    }

    // Validate name
    if taxpayer.nombre.is_empty() {
        return Err(TaxValidationError::InvalidTaxpayer(
            "name required".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tax_law::types::TaxpayerType;

    #[test]
    fn test_validate_taxpayer() {
        let taxpayer = Taxpayer::new(
            "XAXX010101000".to_string(),
            "Test Company".to_string(),
            TaxpayerType::Corporation,
        );

        assert!(validate_taxpayer(&taxpayer).is_ok());
    }
}
