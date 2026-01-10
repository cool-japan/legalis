//! Personal Data Protection Act 2012 - Validation Logic

use super::error::{PdpaError, Result};
use super::types::*;

/// Validates consent record (s. 13)
pub fn validate_consent(record: &ConsentRecord) -> Result<()> {
    if !record.is_valid {
        return Err(PdpaError::ConsentWithdrawn);
    }

    if record.data_categories.is_empty() {
        return Err(PdpaError::ValidationError {
            message: "No data categories specified in consent".to_string(),
        });
    }

    Ok(())
}

/// Validates purpose limitation compliance (s. 18)
pub fn validate_purpose_limitation(
    _original_purpose: PurposeOfCollection,
    _current_purpose: PurposeOfCollection,
) -> Result<()> {
    // Simplified: In real implementation, check if purposes are compatible
    Ok(())
}

/// Validates data breach notification (s. 26B/26C)
pub fn validate_breach_notification(breach: &DataBreachNotification) -> Result<()> {
    if breach.is_notifiable && breach.pdpc_notification_date.is_none() {
        return Err(PdpaError::ValidationError {
            message: "Notifiable breach must be reported to PDPC".to_string(),
        });
    }

    if breach.pdpc_notification_date.is_some() && !breach.is_timely_notification() {
        return Err(PdpaError::LateBreachNotification);
    }

    Ok(())
}

/// Validates DNC compliance (Part IX)
pub fn validate_dnc_compliance(
    phone: &str,
    dnc_type: DncType,
    registry: &DncRegistry,
) -> Result<()> {
    if registry.is_opted_out_from(dnc_type) {
        return Err(PdpaError::DncViolation {
            phone: phone.to_string(),
            dnc_type: format!("{:?}", dnc_type),
        });
    }

    Ok(())
}

/// Validates cross-border data transfer
pub fn validate_cross_border_transfer(transfer: &DataTransfer) -> Result<()> {
    if transfer.legal_basis == TransferLegalBasis::ComparableProtection {
        // Check if destination has adequate protection
        // Simplified: assume certain countries are adequate
        let adequate_countries = ["Switzerland", "Canada", "Japan", "EU"];
        if !adequate_countries.contains(&transfer.destination_country.as_str()) {
            return Err(PdpaError::InadequateTransferProtection {
                country: transfer.destination_country.clone(),
            });
        }
    }

    Ok(())
}

/// Assesses whether DPO appointment is recommended
pub fn validate_dpo_requirement(org: &PdpaOrganisation) -> bool {
    // DPO recommended (not mandatory) for organizations handling large amounts of data
    // Simplified assessment
    org.organisation_type == OrganisationType::Private && org.last_dpia_date.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_consent_valid() {
        let mut consent = ConsentRecord::new(
            "c1",
            "user@test.com",
            PurposeOfCollection::Marketing,
            ConsentMethod::Electronic,
        );
        consent.add_data_category(PersonalDataCategory::Email);

        assert!(validate_consent(&consent).is_ok());
    }

    #[test]
    fn test_validate_consent_withdrawn() {
        let mut consent = ConsentRecord::new(
            "c2",
            "user@test.com",
            PurposeOfCollection::Marketing,
            ConsentMethod::Electronic,
        );
        consent.withdraw(None);

        match validate_consent(&consent) {
            Err(PdpaError::ConsentWithdrawn) => {}
            _ => panic!("Expected ConsentWithdrawn error"),
        }
    }

    #[test]
    fn test_dnc_validation() {
        let mut registry = DncRegistry::new("+6598765432");
        registry.opt_out(vec![DncType::VoiceCall]);

        match validate_dnc_compliance("+6598765432", DncType::VoiceCall, &registry) {
            Err(PdpaError::DncViolation { .. }) => {}
            _ => panic!("Expected DncViolation error"),
        }
    }

    #[test]
    fn test_breach_notification_timely() {
        let mut breach =
            DataBreachNotification::new("b1", BreachType::UnauthorizedAccess, 1000, "Test breach");
        breach.notify_pdpc();

        assert!(validate_breach_notification(&breach).is_ok());
    }
}
