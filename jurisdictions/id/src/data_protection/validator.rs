//! Validation functions for UU PDP compliance

use super::error::{PdpError, PdpResult};
use super::types::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Validate legal basis for data processing - Pasal 20
pub fn validate_legal_basis(
    legal_basis: &LegalBasis,
    data_category: &DataCategory,
    has_consent: bool,
) -> PdpResult<()> {
    // For specific/sensitive data, explicit consent is required - Pasal 25
    if let DataCategory::Specific(specific_type) = data_category
        && (!matches!(legal_basis, LegalBasis::Consent) || !has_consent)
    {
        return Err(PdpError::SpecificDataRequiresExplicitConsent {
            data_type: format!("{:?}", specific_type),
        });
    }

    // Validate that consent-based processing actually has consent
    if matches!(legal_basis, LegalBasis::Consent) && !has_consent {
        return Err(PdpError::InvalidConsent {
            description: "Persetujuan diperlukan tetapi tidak diberikan".to_string(),
        });
    }

    Ok(())
}

/// Validate consent record - Pasal 20-22
pub fn validate_consent(consent: &ConsentRecord) -> PdpResult<()> {
    // Check if consent is in Indonesian - Pasal 21(3)
    if consent.language.to_lowercase() != "indonesian"
        && consent.language.to_lowercase() != "bahasa indonesia"
        && consent.language.to_lowercase() != "id"
    {
        return Err(PdpError::ConsentNotInIndonesian);
    }

    // Check if consent is explicit for specific data - Pasal 25
    if consent.is_for_specific_data && !consent.is_explicit {
        return Err(PdpError::InvalidConsent {
            description: "Data Pribadi Spesifik memerlukan persetujuan eksplisit".to_string(),
        });
    }

    // Check consent expiry
    if let Some(expiry) = consent.expiry_date
        && Utc::now() > expiry
    {
        return Err(PdpError::InvalidConsent {
            description: "Persetujuan telah kadaluarsa".to_string(),
        });
    }

    // Check if withdrawn
    if consent.withdrawal_date.is_some() {
        return Err(PdpError::InvalidConsent {
            description: "Persetujuan telah ditarik".to_string(),
        });
    }

    // Check purposes are specified
    if consent.purposes.is_empty() {
        return Err(PdpError::PurposeNotSpecified);
    }

    Ok(())
}

/// Validate data retention period - Pasal 44
pub fn validate_data_retention(
    actual_retention_days: u32,
    legal_retention_limit: u32,
    purpose_fulfilled: bool,
) -> PdpResult<()> {
    // Data must be deleted when purpose is fulfilled or retention limit reached
    if purpose_fulfilled || actual_retention_days > legal_retention_limit {
        return Err(PdpError::DataRetentionViolation {
            retained_days: actual_retention_days,
            limit_days: legal_retention_limit,
        });
    }
    Ok(())
}

/// Validate cross-border data transfer - Pasal 55-56
pub fn validate_cross_border_transfer(
    destination_country: &str,
    has_adequacy: bool,
    has_contractual_safeguards: bool,
    has_explicit_consent: bool,
) -> PdpResult<()> {
    // Transfer allowed if destination has equivalent protection (Pasal 56)
    if has_adequacy {
        return Ok(());
    }

    // Or if appropriate safeguards exist
    if has_contractual_safeguards {
        return Ok(());
    }

    // Or with explicit consent for specific circumstances
    if has_explicit_consent {
        return Ok(());
    }

    Err(PdpError::CrossBorderTransferViolation {
        destination: destination_country.to_string(),
        reason: "Tidak ada perlindungan yang setara atau safeguard kontraktual".to_string(),
    })
}

/// Validate security incident notification - Pasal 46
pub fn validate_incident_notification(incident: &SecurityIncident) -> PdpResult<()> {
    let hours_elapsed = incident.hours_since_discovery();

    // Must notify within 3x24 hours (72 hours) - Pasal 46
    if hours_elapsed > 72 && !incident.notified_authority {
        return Err(PdpError::BreachNotificationOverdue { hours_elapsed });
    }

    Ok(())
}

/// PDP compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdpCompliance {
    /// Overall compliance status
    pub compliant: bool,
    /// Legal basis valid
    pub legal_basis_valid: bool,
    /// Consent valid
    pub consent_valid: bool,
    /// Data retention compliant
    pub retention_compliant: bool,
    /// Cross-border transfer compliant
    pub cross_border_compliant: bool,
    /// Security measures adequate
    pub security_adequate: bool,
    /// DPO appointed (if required)
    pub dpo_appointed: bool,
    /// DPIA conducted (if required)
    pub dpia_conducted: bool,
    /// List of compliance issues
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Comprehensive PDP compliance check
pub fn validate_pdp_compliance(processing: &PersonalDataProcessing) -> PdpCompliance {
    let mut compliance = PdpCompliance {
        compliant: true,
        legal_basis_valid: true,
        consent_valid: true,
        retention_compliant: true,
        cross_border_compliant: true,
        security_adequate: true,
        dpo_appointed: processing.has_dpo,
        dpia_conducted: processing.requires_dpia,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check legal bases
    if processing.legal_bases.is_empty() {
        compliance.compliant = false;
        compliance.legal_basis_valid = false;
        compliance
            .issues
            .push("Tidak ada dasar hukum pemrosesan (UU PDP Pasal 20)".to_string());
        compliance
            .recommendations
            .push("Tentukan dasar hukum yang valid untuk setiap pemrosesan".to_string());
    }

    // Check DPIA requirement
    if processing.should_require_dpia() && !processing.requires_dpia {
        compliance.compliant = false;
        compliance.dpia_conducted = false;
        compliance
            .issues
            .push("DPIA diperlukan untuk pemrosesan berisiko tinggi (UU PDP Pasal 34)".to_string());
        compliance
            .recommendations
            .push("Lakukan Analisis Dampak Pelindungan Data Pribadi".to_string());
    }

    // Check DPO requirement for specific data processing
    let has_specific_data = processing
        .data_categories
        .iter()
        .any(|c| matches!(c, DataCategory::Specific(_)));

    if has_specific_data && !processing.has_dpo {
        compliance.compliant = false;
        compliance.dpo_appointed = false;
        compliance.issues.push(
            "DPO diperlukan untuk pemrosesan Data Pribadi Spesifik (UU PDP Pasal 53)".to_string(),
        );
        compliance
            .recommendations
            .push("Tunjuk Pejabat Pelindung Data Pribadi".to_string());
    }

    // Check cross-border transfers
    if processing.involves_cross_border && processing.transfer_destinations.is_empty() {
        compliance.compliant = false;
        compliance.cross_border_compliant = false;
        compliance.issues.push(
            "Transfer lintas batas tanpa dokumentasi negara tujuan (UU PDP Pasal 55-56)"
                .to_string(),
        );
        compliance
            .recommendations
            .push("Dokumentasikan negara tujuan dan pastikan perlindungan yang setara".to_string());
    }

    // Check profiling
    if processing.uses_profiling || processing.uses_automated_decisions {
        compliance.recommendations.push(
            "Pastikan Subjek Data diinformasikan tentang pembuatan profil (UU PDP Pasal 12)"
                .to_string(),
        );
    }

    compliance
}

/// Get PDP compliance checklist
pub fn get_pdp_checklist() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Dasar hukum pemrosesan ditentukan",
            "Legal basis for processing established",
            "Pasal 20",
        ),
        (
            "Persetujuan dalam Bahasa Indonesia",
            "Consent in Indonesian language",
            "Pasal 21(3)",
        ),
        (
            "Tujuan pemrosesan jelas dan terbatas",
            "Processing purposes clear and limited",
            "Pasal 21",
        ),
        (
            "Persetujuan eksplisit untuk Data Spesifik",
            "Explicit consent for Specific Data",
            "Pasal 25",
        ),
        (
            "Persetujuan orang tua untuk data anak",
            "Parental consent for children's data",
            "Pasal 25(2)",
        ),
        (
            "Hak Subjek Data dapat dilaksanakan",
            "Data Subject rights exercisable",
            "Pasal 5-13",
        ),
        (
            "DPIA untuk pemrosesan berisiko tinggi",
            "DPIA for high-risk processing",
            "Pasal 34",
        ),
        (
            "Langkah keamanan teknis dan organisasi",
            "Technical and organizational security measures",
            "Pasal 35",
        ),
        (
            "Perjanjian pemrosesan data dengan prosesor",
            "Data processing agreement with processor",
            "Pasal 39",
        ),
        (
            "Notifikasi pelanggaran 3x24 jam",
            "Breach notification within 3x24 hours",
            "Pasal 46",
        ),
        (
            "DPO ditunjuk (jika diperlukan)",
            "DPO appointed (if required)",
            "Pasal 53",
        ),
        (
            "Transfer lintas batas sesuai ketentuan",
            "Cross-border transfer compliant",
            "Pasal 55-56",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_validate_legal_basis_general_data() {
        let result = validate_legal_basis(&LegalBasis::Consent, &DataCategory::General, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_legal_basis_specific_data_without_consent() {
        let result = validate_legal_basis(
            &LegalBasis::LegitimateInterests,
            &DataCategory::Specific(SpecificDataType::Health),
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_consent_valid() {
        let consent = ConsentRecord {
            id: "C001".to_string(),
            data_subject_id: "DS001".to_string(),
            data_controller_id: "DC001".to_string(),
            purposes: vec![ProcessingPurpose::ServiceDelivery],
            data_categories: vec![DataCategory::General],
            consent_date: Utc::now(),
            expiry_date: Some(Utc::now() + Duration::days(365)),
            is_explicit: true,
            is_for_specific_data: false,
            consent_method: ConsentMethod::Electronic,
            is_withdrawable: true,
            language: "Bahasa Indonesia".to_string(),
            withdrawal_date: None,
        };
        assert!(validate_consent(&consent).is_ok());
    }

    #[test]
    fn test_validate_consent_wrong_language() {
        let consent = ConsentRecord {
            id: "C001".to_string(),
            data_subject_id: "DS001".to_string(),
            data_controller_id: "DC001".to_string(),
            purposes: vec![ProcessingPurpose::ServiceDelivery],
            data_categories: vec![DataCategory::General],
            consent_date: Utc::now(),
            expiry_date: None,
            is_explicit: true,
            is_for_specific_data: false,
            consent_method: ConsentMethod::Electronic,
            is_withdrawable: true,
            language: "English".to_string(),
            withdrawal_date: None,
        };
        let result = validate_consent(&consent);
        assert!(matches!(result, Err(PdpError::ConsentNotInIndonesian)));
    }

    #[test]
    fn test_validate_cross_border_with_adequacy() {
        let result = validate_cross_border_transfer("Singapore", true, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_cross_border_without_protection() {
        let result = validate_cross_border_transfer("Unknown Country", false, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_incident_notification_timely() {
        let incident = SecurityIncident {
            id: "INC-001".to_string(),
            discovery_time: Utc::now() - Duration::hours(24),
            incident_time: None,
            affected_data_categories: vec![DataCategory::General],
            affected_subjects_count: 100,
            description: "Minor breach".to_string(),
            notified_authority: false,
            authority_notification_time: None,
            notified_subjects: false,
            subject_notification_time: None,
            remedial_actions: vec![],
            risk_level: RiskLevel::Low,
        };
        assert!(validate_incident_notification(&incident).is_ok());
    }

    #[test]
    fn test_pdp_compliance_check() {
        let processing = PersonalDataProcessing {
            controller_name: "PT Example".to_string(),
            processor_name: None,
            purposes: vec![ProcessingPurpose::ServiceDelivery],
            data_categories: vec![DataCategory::General],
            legal_bases: vec![LegalBasis::Consent],
            involves_cross_border: false,
            transfer_destinations: vec![],
            retention_period_days: 365,
            uses_automated_decisions: false,
            uses_profiling: false,
            requires_dpia: false,
            has_dpo: false,
        };

        let compliance = validate_pdp_compliance(&processing);
        assert!(compliance.compliant);
        assert!(compliance.issues.is_empty());
    }

    #[test]
    fn test_pdp_checklist() {
        let checklist = get_pdp_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(id, _, _)| id.contains("DPIA")));
    }
}
