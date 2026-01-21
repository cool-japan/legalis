//! LGPD Validation Functions

use super::error::{LgpdError, LgpdResult};
use super::types::*;
use chrono::Utc;

/// Validate processing has legal basis (Art. 7)
pub fn validate_legal_basis(processing: &PersonalDataProcessing) -> LgpdResult<()> {
    if processing.purposes.is_empty() {
        return Err(LgpdError::NoLegalBasis {
            description: "Nenhuma finalidade definida para o tratamento".to_string(),
        });
    }

    // Sensitive data requires specific legal bases (Art. 11)
    if processing.has_sensitive_data {
        let valid_bases = processing.purposes.iter().all(|p| {
            matches!(
                p.legal_basis,
                LegalBasis::Consent
                    | LegalBasis::LegalObligation
                    | LegalBasis::HealthProtection
                    | LegalBasis::Research
                    | LegalBasis::LegalProceedings
            )
        });

        if !valid_bases {
            return Err(LgpdError::SensitiveDataViolation {
                description: "Base legal inadequada para dados sensíveis".to_string(),
            });
        }
    }

    Ok(())
}

/// Validate consent requirements (Art. 8)
pub fn validate_consent(
    consent: &ConsentRecord,
    is_specific: bool,
    is_informed: bool,
    is_unambiguous: bool,
) -> LgpdResult<()> {
    if !consent.is_valid() {
        return Err(LgpdError::InvalidConsent {
            reason: "Consentimento revogado ou inativo".to_string(),
        });
    }

    if !is_specific {
        return Err(LgpdError::InvalidConsent {
            reason: "Consentimento não é específico para a finalidade".to_string(),
        });
    }

    if !is_informed {
        return Err(LgpdError::InvalidConsent {
            reason: "Titular não foi adequadamente informado".to_string(),
        });
    }

    if !is_unambiguous {
        return Err(LgpdError::InvalidConsent {
            reason: "Manifestação de consentimento não inequívoca".to_string(),
        });
    }

    Ok(())
}

/// Validate children's data processing (Art. 14)
pub fn validate_childrens_data(
    has_parental_consent: bool,
    is_in_childs_best_interest: bool,
) -> LgpdResult<()> {
    if !has_parental_consent {
        return Err(LgpdError::ChildrensDataViolation {
            description: "Tratamento de dados de criança sem consentimento parental".to_string(),
        });
    }

    if !is_in_childs_best_interest {
        return Err(LgpdError::ChildrensDataViolation {
            description: "Tratamento não atende ao melhor interesse da criança".to_string(),
        });
    }

    Ok(())
}

/// Validate international transfer (Art. 33)
pub fn validate_international_transfer(
    destination_country: &str,
    has_adequacy_decision: bool,
    has_standard_clauses: bool,
    has_binding_rules: bool,
    has_specific_consent: bool,
) -> LgpdResult<()> {
    // Transfer allowed if any condition is met
    let is_valid =
        has_adequacy_decision || has_standard_clauses || has_binding_rules || has_specific_consent;

    if !is_valid {
        return Err(LgpdError::InvalidInternationalTransfer {
            destination: destination_country.to_string(),
        });
    }

    Ok(())
}

/// Validate security incident notification (Art. 48)
pub fn validate_incident_notification(incident: &SecurityIncident) -> LgpdResult<()> {
    if !incident.requires_anpd_notification() {
        return Ok(()); // Low risk, no notification required
    }

    // Calculate time since incident
    let hours_since_detection = (Utc::now() - incident.data_deteccao).num_hours();

    // ANPD expects notification in "reasonable time" - typically 72 hours
    if !incident.anpd_notified && hours_since_detection > 72 {
        return Err(LgpdError::IncidentNotReported {
            description: format!(
                "Incidente detectado há {} horas sem comunicação à ANPD",
                hours_since_detection
            ),
        });
    }

    // Data subjects must be notified for high risk
    if incident.requires_subject_notification() && !incident.subjects_notified {
        return Err(LgpdError::IncidentNotReported {
            description: "Titulares não foram comunicados sobre incidente de alto risco"
                .to_string(),
        });
    }

    Ok(())
}

/// Validate data minimization (Art. 6, III)
pub fn validate_data_minimization(
    data_categories: &[DataCategory],
    processing_purpose: &str,
) -> LgpdResult<()> {
    // Check for potentially excessive collection
    let sensitive_count = data_categories.iter().filter(|c| c.is_sensitive()).count();

    if sensitive_count > 3 {
        return Err(LgpdError::ExcessiveDataCollection {
            description: format!(
                "Coleta de {} categorias de dados sensíveis para '{}'",
                sensitive_count, processing_purpose
            ),
        });
    }

    Ok(())
}

/// Validate data subject request response
pub fn validate_dsr_response(
    request_date: chrono::DateTime<Utc>,
    response_date: Option<chrono::DateTime<Utc>>,
    right: DataSubjectRight,
) -> LgpdResult<()> {
    let deadline_days = right.response_deadline_days() as i64;

    match response_date {
        Some(response) => {
            let response_days = (response - request_date).num_days();
            if response_days > deadline_days {
                return Err(LgpdError::RightDenied {
                    right: format!(
                        "{} - resposta em {} dias (prazo: {} dias)",
                        right.descricao_pt(),
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
                Err(LgpdError::RightDenied {
                    right: format!(
                        "{} - {} dias sem resposta (prazo: {} dias)",
                        right.descricao_pt(),
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

/// Validate retention period (Art. 16)
pub fn validate_retention(
    processing: &PersonalDataProcessing,
    purpose_fulfilled: bool,
    has_legal_retention_requirement: bool,
) -> LgpdResult<()> {
    if purpose_fulfilled && !has_legal_retention_requirement {
        return Err(LgpdError::RetentionViolation {
            description: "Finalidade cumprida, dados devem ser eliminados".to_string(),
        });
    }

    // Check excessive retention
    if let Some(months) = processing.retention_months
        && months > 120
        && !has_legal_retention_requirement
    {
        // 10 years
        return Err(LgpdError::RetentionViolation {
            description: format!("Período de retenção de {} meses pode ser excessivo", months),
        });
    }

    Ok(())
}

/// Validate automated decision-making (Art. 20)
pub fn validate_automated_decision(
    is_automated: bool,
    human_review_available: bool,
    transparency_provided: bool,
) -> LgpdResult<()> {
    if !is_automated {
        return Ok(());
    }

    if !human_review_available {
        return Err(LgpdError::AutomatedDecisionNoReview);
    }

    if !transparency_provided {
        return Err(LgpdError::RightDenied {
            right: "Informação sobre critérios de decisão automatizada".to_string(),
        });
    }

    Ok(())
}

/// Comprehensive LGPD compliance check
pub fn validate_lgpd_compliance(
    has_dpo: bool,
    has_privacy_policy: bool,
    has_data_mapping: bool,
    has_consent_mechanism: bool,
    has_dsr_process: bool,
    has_incident_plan: bool,
) -> LgpdCompliance {
    let mut compliance = LgpdCompliance {
        compliant: true,
        has_dpo,
        has_privacy_policy,
        has_data_mapping,
        has_consent_mechanism,
        has_dsr_process,
        has_incident_plan,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check DPO (Art. 41)
    if !has_dpo {
        compliance.compliant = false;
        compliance
            .issues
            .push("Encarregado (DPO) não nomeado".to_string());
        compliance
            .recommendations
            .push("Nomear DPO e divulgar contato publicamente".to_string());
    }

    // Check privacy policy
    if !has_privacy_policy {
        compliance.compliant = false;
        compliance
            .issues
            .push("Política de privacidade ausente".to_string());
        compliance
            .recommendations
            .push("Elaborar política de privacidade clara e acessível".to_string());
    }

    // Check data mapping (ROPA)
    if !has_data_mapping {
        compliance.compliant = false;
        compliance
            .issues
            .push("Inventário de dados pessoais ausente".to_string());
        compliance
            .recommendations
            .push("Mapear todos os tratamentos de dados pessoais (Art. 37)".to_string());
    }

    // Check consent mechanism
    if !has_consent_mechanism {
        compliance.compliant = false;
        compliance
            .issues
            .push("Mecanismo de consentimento inadequado".to_string());
        compliance
            .recommendations
            .push("Implementar coleta de consentimento livre, informado e específico".to_string());
    }

    // Check DSR process
    if !has_dsr_process {
        compliance.compliant = false;
        compliance
            .issues
            .push("Processo de atendimento a direitos do titular ausente".to_string());
        compliance
            .recommendations
            .push("Implementar canal de atendimento a direitos do titular (Art. 18)".to_string());
    }

    // Check incident response
    if !has_incident_plan {
        compliance.compliant = false;
        compliance
            .issues
            .push("Plano de resposta a incidentes ausente".to_string());
        compliance
            .recommendations
            .push("Elaborar plano de resposta a incidentes de segurança (Art. 48)".to_string());
    }

    compliance
}

/// Get LGPD compliance checklist
pub fn get_lgpd_checklist() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Encarregado (DPO) nomeado", "Art. 41"),
        ("Política de privacidade", "Art. 9"),
        ("Inventário de dados (ROPA)", "Art. 37"),
        ("Base legal documentada", "Art. 7"),
        ("Consentimento específico", "Art. 8"),
        ("Direitos do titular", "Art. 18"),
        ("Segurança da informação", "Art. 46"),
        ("Plano de incidentes", "Art. 48"),
        ("Transferência internacional", "Art. 33"),
        ("Avaliação de impacto (RIPD)", "Art. 38"),
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
                "Marketing",
                "Marketing",
                LegalBasis::Consent,
            )],
            data_subjects: vec!["Clientes".to_string()],
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
            titular_id: "123".to_string(),
            data_consentimento: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            finalidade: "Marketing".to_string(),
            texto_consentimento: "Aceito".to_string(),
            metodo: "Click".to_string(),
            ativo: true,
            data_revogacao: None,
        };

        assert!(validate_consent(&consent, true, true, true).is_ok());
    }

    #[test]
    fn test_validate_consent_revoked() {
        let consent = ConsentRecord {
            titular_id: "123".to_string(),
            data_consentimento: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            finalidade: "Marketing".to_string(),
            texto_consentimento: "Aceito".to_string(),
            metodo: "Click".to_string(),
            ativo: false,
            data_revogacao: Some(NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date")),
        };

        assert!(validate_consent(&consent, true, true, true).is_err());
    }

    #[test]
    fn test_validate_childrens_data() {
        assert!(validate_childrens_data(true, true).is_ok());
        assert!(validate_childrens_data(false, true).is_err());
        assert!(validate_childrens_data(true, false).is_err());
    }

    #[test]
    fn test_validate_international_transfer_ok() {
        assert!(validate_international_transfer("EU", true, false, false, false).is_ok());
        assert!(validate_international_transfer("US", false, true, false, false).is_ok());
    }

    #[test]
    fn test_validate_international_transfer_fail() {
        assert!(validate_international_transfer("XX", false, false, false, false).is_err());
    }

    #[test]
    fn test_validate_data_minimization() {
        let categories = vec![DataCategory::Contact, DataCategory::Identification];
        assert!(validate_data_minimization(&categories, "Newsletter").is_ok());

        let excessive = vec![
            DataCategory::Health,
            DataCategory::Biometric,
            DataCategory::Genetic,
            DataCategory::Religious,
        ];
        assert!(validate_data_minimization(&excessive, "Newsletter").is_err());
    }

    #[test]
    fn test_validate_automated_decision() {
        assert!(validate_automated_decision(false, false, false).is_ok());
        assert!(validate_automated_decision(true, true, true).is_ok());
        assert!(validate_automated_decision(true, false, true).is_err());
    }

    #[test]
    fn test_compliance_check_full() {
        let compliance = validate_lgpd_compliance(true, true, true, true, true, true);
        assert!(compliance.compliant);
        assert!(compliance.issues.is_empty());
    }

    #[test]
    fn test_compliance_check_missing() {
        let compliance = validate_lgpd_compliance(false, true, true, true, true, true);
        assert!(!compliance.compliant);
        assert!(!compliance.issues.is_empty());
    }

    #[test]
    fn test_lgpd_checklist() {
        let checklist = get_lgpd_checklist();
        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|(item, _)| item.contains("DPO")));
    }
}
