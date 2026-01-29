//! Integration tests for data protection law

use legalis_mx::data_protection::*;

#[test]
fn test_data_processing_with_consent() {
    let processing = PersonalDataProcessing {
        responsable: "Empresa SA de CV".to_string(),
        titulares: vec![DataSubject {
            nombre: "Juan Pérez".to_string(),
            categorias_datos: vec![DataCategory::Identification, DataCategory::Contact],
        }],
        finalidad: vec!["Prestación de servicios".to_string()],
        base_legal: LegalBasis::Consent,
        consentimiento: true,
    };

    assert!(processing.validate().is_ok());
}

#[test]
fn test_sensitive_data_requires_consent() {
    let processing = PersonalDataProcessing {
        responsable: "Hospital".to_string(),
        titulares: vec![DataSubject {
            nombre: "Paciente".to_string(),
            categorias_datos: vec![DataCategory::Health],
        }],
        finalidad: vec!["Atención médica".to_string()],
        base_legal: LegalBasis::Consent,
        consentimiento: false, // Missing consent
    };

    assert!(processing.validate().is_err());
}

#[test]
fn test_privacy_notice_validation() {
    let notice = PrivacyNotice {
        identidad_responsable: "Empresa SA de CV".to_string(),
        finalidades: vec!["Prestación de servicios".to_string()],
        datos_recabados: vec![DataCategory::Identification],
        transferencias: vec![],
        medios_arco: "privacidad@empresa.com".to_string(),
    };

    assert!(notice.validate().is_ok());
}

#[test]
fn test_privacy_notice_missing_controller() {
    let notice = PrivacyNotice {
        identidad_responsable: "".to_string(), // Missing
        finalidades: vec!["Test".to_string()],
        datos_recabados: vec![DataCategory::Identification],
        transferencias: vec![],
        medios_arco: "test@test.com".to_string(),
    };

    assert!(notice.validate().is_err());
}
