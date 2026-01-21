//! PDPA Validation Functions

use super::error::{PdpaError, PdpaResult};
use super::types::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Validate processing has legal basis (Section 24)
pub fn validate_legal_basis(processing: &PersonalDataProcessing) -> PdpaResult<()> {
    if processing.purposes.is_empty() {
        return Err(PdpaError::NoLegalBasis {
            description: "ไม่ได้กำหนดวัตถุประสงค์ในการประมวลผล".to_string(),
        });
    }

    // Sensitive data requires explicit consent or specific legal bases (Section 26)
    if processing.contains_sensitive_data() {
        let valid_bases = processing.purposes.iter().all(|p| {
            matches!(
                p.legal_basis,
                LegalBasis::Consent | LegalBasis::LegalObligation | LegalBasis::VitalInterests
            )
        });

        if !valid_bases {
            return Err(PdpaError::SensitiveDataViolation {
                description: "ฐานทางกฎหมายไม่เหมาะสมสำหรับข้อมูลอ่อนไหว".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate consent requirements (Section 19)
pub fn validate_consent(
    consent: &ConsentRecord,
    is_specific: bool,
    is_informed: bool,
    is_freely_given: bool,
) -> PdpaResult<()> {
    if !consent.is_valid() {
        return Err(PdpaError::InvalidConsent {
            reason: "ความยินยอมถูกถอนหรือไม่ได้ใช้งาน".to_string(),
        });
    }

    if !is_specific {
        return Err(PdpaError::InvalidConsent {
            reason: "ความยินยอมไม่เฉพาะเจาะจงกับวัตถุประสงค์".to_string(),
        });
    }

    if !is_informed {
        return Err(PdpaError::InvalidConsent {
            reason: "เจ้าของข้อมูลไม่ได้รับข้อมูลอย่างเพียงพอ".to_string(),
        });
    }

    if !is_freely_given {
        return Err(PdpaError::InvalidConsent {
            reason: "ความยินยอมไม่ได้ให้โดยเสรี".to_string(),
        });
    }

    Ok(())
}

/// Validate security incident notification (Section 37)
pub fn validate_incident_notification(incident: &SecurityIncident) -> PdpaResult<()> {
    if !incident.requires_pdpc_notification() {
        return Ok(());
    }

    // 72-hour notification deadline
    if !incident.pdpc_notified && incident.notification_deadline_passed() {
        return Err(PdpaError::BreachNotReported {
            description: format!(
                "เหตุการณ์ตรวจพบเมื่อ {} ชั่วโมงที่แล้วโดยไม่ได้แจ้ง สคส.",
                incident.hours_since_detection()
            ),
        });
    }

    // Subject notification for high risk
    if incident.requires_subject_notification() && !incident.subjects_notified {
        return Err(PdpaError::BreachNotReported {
            description: "ไม่ได้แจ้งเจ้าของข้อมูลที่ได้รับผลกระทบจากเหตุการณ์ความเสี่ยงสูง".to_string(),
        });
    }

    Ok(())
}

/// Validate cross-border transfer (Section 28)
pub fn validate_cross_border_transfer(
    destination_country: &str,
    has_adequate_protection: bool,
    has_consent: bool,
    has_contract: bool,
    has_binding_rules: bool,
) -> PdpaResult<()> {
    let is_valid = has_adequate_protection || has_consent || has_contract || has_binding_rules;

    if !is_valid {
        return Err(PdpaError::InvalidCrossBorderTransfer {
            destination: destination_country.to_string(),
        });
    }

    Ok(())
}

/// Validate data subject request response
pub fn validate_dsr_response(
    request_date: chrono::DateTime<Utc>,
    response_date: Option<chrono::DateTime<Utc>>,
    right: DataSubjectRight,
) -> PdpaResult<()> {
    let deadline_days = right.response_deadline_days() as i64;

    match response_date {
        Some(response) => {
            let response_days = (response - request_date).num_days();
            if response_days > deadline_days {
                return Err(PdpaError::RightDenied {
                    right: format!(
                        "{} - ตอบกลับใน {} วัน (กำหนด: {} วัน)",
                        right.name_th(),
                        response_days,
                        deadline_days
                    ),
                });
            }
            Ok(())
        }
        None => {
            let days_elapsed = (Utc::now() - request_date).num_days();
            if days_elapsed > deadline_days {
                Err(PdpaError::RightDenied {
                    right: format!(
                        "{} - {} วันโดยไม่มีการตอบกลับ (กำหนด: {} วัน)",
                        right.name_th(),
                        days_elapsed,
                        deadline_days
                    ),
                })
            } else {
                Ok(())
            }
        }
    }
}

/// Validate data minimization (Section 22)
pub fn validate_data_minimization(
    data_categories: &[DataCategory],
    processing_purpose: &str,
) -> PdpaResult<()> {
    let sensitive_count = data_categories.iter().filter(|c| c.is_sensitive()).count();

    if sensitive_count > 3 {
        return Err(PdpaError::ExcessiveDataCollection {
            description: format!(
                "เก็บข้อมูลอ่อนไหว {} หมวดหมู่สำหรับ '{}'",
                sensitive_count, processing_purpose
            ),
        });
    }

    Ok(())
}

/// PDPA compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdpaCompliance {
    /// Overall compliance status
    pub compliant: bool,

    /// Whether DPO is appointed
    pub has_dpo: bool,

    /// Whether privacy policy exists
    pub has_privacy_policy: bool,

    /// Whether ROPA (Record of Processing Activities) exists
    pub has_ropa: bool,

    /// Whether consent mechanism is implemented
    pub has_consent_mechanism: bool,

    /// Whether DSR process exists
    pub has_dsr_process: bool,

    /// Whether incident response plan exists
    pub has_incident_plan: bool,

    /// List of compliance issues
    pub issues: Vec<String>,

    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Comprehensive PDPA compliance check
pub fn validate_pdpa_compliance(
    has_dpo: bool,
    has_privacy_policy: bool,
    has_ropa: bool,
    has_consent_mechanism: bool,
    has_dsr_process: bool,
    has_incident_plan: bool,
) -> PdpaCompliance {
    let mut compliance = PdpaCompliance {
        compliant: true,
        has_dpo,
        has_privacy_policy,
        has_ropa,
        has_consent_mechanism,
        has_dsr_process,
        has_incident_plan,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check DPO (Section 41)
    if !has_dpo {
        compliance.compliant = false;
        compliance
            .issues
            .push("ไม่มีเจ้าหน้าที่คุ้มครองข้อมูลส่วนบุคคล (DPO)".to_string());
        compliance
            .recommendations
            .push("แต่งตั้ง DPO และเผยแพร่ข้อมูลติดต่อ".to_string());
    }

    // Check privacy policy (Section 23)
    if !has_privacy_policy {
        compliance.compliant = false;
        compliance.issues.push("ไม่มีนโยบายความเป็นส่วนตัว".to_string());
        compliance
            .recommendations
            .push("จัดทำนโยบายความเป็นส่วนตัวที่ชัดเจนและเข้าถึงได้".to_string());
    }

    // Check ROPA (Section 39)
    if !has_ropa {
        compliance.compliant = false;
        compliance
            .issues
            .push("ไม่มีบันทึกกิจกรรมการประมวลผล (ROPA)".to_string());
        compliance
            .recommendations
            .push("จัดทำบันทึกกิจกรรมการประมวลผลข้อมูลทั้งหมด".to_string());
    }

    // Check consent mechanism (Section 19)
    if !has_consent_mechanism {
        compliance.compliant = false;
        compliance
            .issues
            .push("ระบบจัดการความยินยอมไม่เพียงพอ".to_string());
        compliance
            .recommendations
            .push("พัฒนาระบบขอความยินยอมที่ชัดเจน เฉพาะเจาะจง และให้โดยเสรี".to_string());
    }

    // Check DSR process (Sections 30-36)
    if !has_dsr_process {
        compliance.compliant = false;
        compliance
            .issues
            .push("ไม่มีกระบวนการรองรับสิทธิของเจ้าของข้อมูล".to_string());
        compliance
            .recommendations
            .push("จัดทำช่องทางรับคำขอใช้สิทธิของเจ้าของข้อมูล".to_string());
    }

    // Check incident response (Section 37)
    if !has_incident_plan {
        compliance.compliant = false;
        compliance
            .issues
            .push("ไม่มีแผนรับมือเหตุการณ์ละเมิดข้อมูล".to_string());
        compliance
            .recommendations
            .push("จัดทำแผนรับมือเหตุการณ์ละเมิดข้อมูลพร้อมขั้นตอนแจ้ง สคส.".to_string());
    }

    compliance
}

/// Get PDPA compliance checklist
pub fn get_pdpa_checklist() -> Vec<(&'static str, &'static str, u32)> {
    vec![
        ("แต่งตั้ง DPO", "DPO Appointed", 41),
        ("นโยบายความเป็นส่วนตัว", "Privacy Policy", 23),
        (
            "บันทึกกิจกรรมการประมวลผล (ROPA)",
            "Record of Processing Activities",
            39,
        ),
        ("ฐานทางกฎหมายในการประมวลผล", "Legal Basis Documented", 24),
        ("ระบบจัดการความยินยอม", "Consent Management", 19),
        ("สิทธิของเจ้าของข้อมูล", "Data Subject Rights", 30),
        ("มาตรการรักษาความปลอดภัย", "Security Measures", 37),
        ("แผนรับมือเหตุการณ์", "Incident Response Plan", 37),
        (
            "การโอนข้อมูลข้ามประเทศ",
            "Cross-Border Transfer Safeguards",
            28,
        ),
        (
            "การประเมินผลกระทบ (DPIA)",
            "Data Protection Impact Assessment",
            38,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_validate_legal_basis_ok() {
        let processing = PersonalDataProcessing {
            id: "1".to_string(),
            data_categories: vec![DataCategory::Contact],
            purposes: vec![ProcessingPurpose::new(
                "การตลาด",
                "Marketing",
                LegalBasis::Consent,
            )],
            data_subjects: vec!["ลูกค้า".to_string()],
            retention_months: Some(24),
            international_transfer: false,
            transfer_countries: vec![],
            has_sensitive_data: false,
            automated_decision: false,
        };

        assert!(validate_legal_basis(&processing).is_ok());
    }

    #[test]
    fn test_validate_legal_basis_no_purpose() {
        let processing = PersonalDataProcessing {
            id: "1".to_string(),
            data_categories: vec![DataCategory::Contact],
            purposes: vec![],
            data_subjects: vec![],
            retention_months: None,
            international_transfer: false,
            transfer_countries: vec![],
            has_sensitive_data: false,
            automated_decision: false,
        };

        assert!(validate_legal_basis(&processing).is_err());
    }

    #[test]
    fn test_validate_consent_valid() {
        let consent = ConsentRecord {
            subject_id: "123".to_string(),
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            purpose_th: "การตลาด".to_string(),
            purpose_en: "Marketing".to_string(),
            consent_text: "ยอมรับ".to_string(),
            method: "Click".to_string(),
            active: true,
            withdrawal_date: None,
        };

        assert!(validate_consent(&consent, true, true, true).is_ok());
    }

    #[test]
    fn test_validate_consent_withdrawn() {
        let consent = ConsentRecord {
            subject_id: "123".to_string(),
            consent_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            purpose_th: "การตลาด".to_string(),
            purpose_en: "Marketing".to_string(),
            consent_text: "ยอมรับ".to_string(),
            method: "Click".to_string(),
            active: false,
            withdrawal_date: Some(NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date")),
        };

        assert!(validate_consent(&consent, true, true, true).is_err());
    }

    #[test]
    fn test_validate_cross_border_transfer_ok() {
        assert!(validate_cross_border_transfer("EU", true, false, false, false).is_ok());
        assert!(validate_cross_border_transfer("US", false, true, false, false).is_ok());
    }

    #[test]
    fn test_validate_cross_border_transfer_fail() {
        assert!(validate_cross_border_transfer("XX", false, false, false, false).is_err());
    }

    #[test]
    fn test_validate_data_minimization() {
        let categories = vec![DataCategory::Contact, DataCategory::Identification];
        assert!(validate_data_minimization(&categories, "Newsletter").is_ok());

        let excessive = vec![
            DataCategory::Health,
            DataCategory::Biometric,
            DataCategory::Genetic,
            DataCategory::Criminal,
        ];
        assert!(validate_data_minimization(&excessive, "Newsletter").is_err());
    }

    #[test]
    fn test_compliance_check_full() {
        let compliance = validate_pdpa_compliance(true, true, true, true, true, true);
        assert!(compliance.compliant);
        assert!(compliance.issues.is_empty());
    }

    #[test]
    fn test_compliance_check_missing() {
        let compliance = validate_pdpa_compliance(false, true, true, true, true, true);
        assert!(!compliance.compliant);
        assert!(!compliance.issues.is_empty());
    }

    #[test]
    fn test_pdpa_checklist() {
        let checklist = get_pdpa_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(th, _, _)| th.contains("DPO")));
    }
}
