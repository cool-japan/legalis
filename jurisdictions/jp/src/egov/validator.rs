//! e-Gov Application Validator
//!
//! Provides validation logic for e-Gov electronic filing applications,
//! checking required fields, attachments, and business rules.

use crate::egov::{
    error::{EgovError, Result},
    types::{ApplicationStatus, EgovApplication, ValidationReport},
};

/// Validate an e-Gov application before submission
pub fn validate_application(app: &EgovApplication) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // Check application metadata
    validate_metadata(app, &mut report);

    // Check required fields
    validate_required_fields(app, &mut report);

    // Check attachments
    validate_attachments(app, &mut report);

    // Check form data integrity
    validate_form_data(app, &mut report);

    Ok(report)
}

/// Validate application metadata
fn validate_metadata(app: &EgovApplication, report: &mut ValidationReport) {
    // Check application ID format
    if app.metadata.application_id.is_empty() {
        report.add_error("Application ID is required (申請IDが必要です)");
    } else if !is_valid_application_id(&app.metadata.application_id) {
        report.add_error(format!(
            "Invalid application ID format: {} (申請IDの形式が無効です)",
            app.metadata.application_id
        ));
    }

    // Check applicant name
    if app.metadata.applicant_name.is_empty() {
        report.add_error("Applicant name is required (申請者名が必要です)");
    }

    // Check application type
    if app.metadata.application_type.is_empty() {
        report.add_error("Application type is required (申請種別が必要です)");
    }

    // Check submission status consistency
    if app.metadata.is_submitted() && app.metadata.submission_date.is_none() {
        report.add_error(
            "Submission date is missing for submitted application (提出日が設定されていません)",
        );
    }

    // Warn if application is in terminal state
    if app.metadata.is_terminal() {
        report.add_warning(format!(
            "Application is in terminal state: {:?} (最終状態です)",
            app.metadata.status
        ));
    }
}

/// Validate required fields based on application type
fn validate_required_fields(app: &EgovApplication, report: &mut ValidationReport) {
    let required_fields = get_required_fields(&app.metadata.application_type);

    for field in required_fields {
        if !app.form_data.contains_key(field) {
            report.add_error(format!(
                "Required field '{}' is missing (必須項目が欠落)",
                field
            ));
        } else if let Some(value) = app.form_data.get(field) {
            // Check if value is empty for text fields
            if let Some(text) = value.as_text() {
                if text.trim().is_empty() {
                    report.add_error(format!(
                        "Required field '{}' is empty (必須項目が空)",
                        field
                    ));
                }
            }
        }
    }
}

/// Validate attachments
fn validate_attachments(app: &EgovApplication, report: &mut ValidationReport) {
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
    const ALLOWED_TYPES: &[&str] = &[
        "application/pdf",
        "image/jpeg",
        "image/png",
        "application/msword",
        "application/vnd.openxmlformats-officedocument",
    ];

    for attachment in &app.attachments {
        // Check file size
        if !attachment.is_size_valid(MAX_FILE_SIZE) {
            report.add_error(format!(
                "Attachment '{}' exceeds size limit of {} bytes (ファイルサイズが制限を超過)",
                attachment.filename, MAX_FILE_SIZE
            ));
        }

        // Check file type
        if !attachment.is_type_allowed(ALLOWED_TYPES) {
            report.add_error(format!(
                "Attachment '{}' has unsupported file type: {} (サポートされていないファイル形式)",
                attachment.filename, attachment.mime_type
            ));
        }

        // Check if required attachment has content
        if attachment.required && attachment.content.is_none() {
            report.add_error(format!(
                "Required attachment '{}' has no content (必須添付ファイルの内容がありません)",
                attachment.filename
            ));
        }

        // Warn if attachment is large (>5MB)
        if attachment.size_bytes > 5 * 1024 * 1024 {
            report.add_warning(format!(
                "Attachment '{}' is large ({} bytes), consider compression (ファイルサイズが大きい)",
                attachment.filename, attachment.size_bytes
            ));
        }
    }

    // Check if all required attachments are present
    if !app.has_required_attachments() {
        report.add_error("Some required attachments are missing content (必須添付ファイルが不足)");
    }
}

/// Validate form data integrity
fn validate_form_data(app: &EgovApplication, report: &mut ValidationReport) {
    for (key, value) in &app.form_data {
        // Validate date fields
        if let Some(date) = value.as_date() {
            let today = chrono::Utc::now().date_naive();
            if date > today {
                report.add_warning(format!(
                    "Field '{}' has future date: {} (未来の日付)",
                    key, date
                ));
            }
        }

        // Validate number fields
        if let Some(num) = value.as_number() {
            if num < 0 {
                report.add_warning(format!(
                    "Field '{}' has negative value: {} (負の値)",
                    key, num
                ));
            }
        }

        // Validate nested objects
        if let Some(obj) = value.as_object() {
            if obj.is_empty() {
                report.add_warning(format!(
                    "Field '{}' is an empty object (空のオブジェクト)",
                    key
                ));
            }
        }

        // Validate lists
        if let Some(list) = value.as_list() {
            if list.is_empty() {
                report.add_warning(format!("Field '{}' is an empty list (空のリスト)", key));
            }
        }
    }
}

/// Check if application ID format is valid
fn is_valid_application_id(id: &str) -> bool {
    // Application ID should be alphanumeric with hyphens, length 5-50
    !id.is_empty()
        && id.len() >= 5
        && id.len() <= 50
        && id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

/// Get required fields for application type
fn get_required_fields(application_type: &str) -> Vec<&'static str> {
    match application_type {
        "administrative_procedure" => vec!["procedure_type", "agency", "applicant_name"],
        "construction_license" => vec!["license_type", "business_name", "capital"],
        "environmental_notification" => vec!["facility_name", "facility_type", "location"],
        "personal_info_filing" => vec!["business_name", "data_types", "purposes"],
        _ => vec![], // Default: no required form fields (metadata fields are validated separately)
    }
}

/// Validate status transition
pub fn validate_status_transition(
    current: ApplicationStatus,
    target: ApplicationStatus,
) -> Result<()> {
    if !current.can_transition_to(target) {
        return Err(EgovError::InvalidStatusTransition {
            from: current,
            to: target,
        });
    }
    Ok(())
}

/// Quick validation check (returns true if valid, false otherwise)
pub fn quick_validate(app: &EgovApplication) -> bool {
    validate_application(app)
        .map(|report| report.is_valid())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::egov::types::{Attachment, EgovApplication, EgovFieldValue, GovernmentAgency};

    fn create_test_application() -> EgovApplication {
        EgovApplication::new(
            "APP-TEST-001",
            "test_application",
            "Test Applicant",
            GovernmentAgency::DigitalAgency,
        )
    }

    #[test]
    fn test_validate_valid_application() {
        let app = create_test_application();
        let report = validate_application(&app).unwrap();
        // Should have no errors for basic application
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_validate_empty_applicant_name() {
        let mut app = create_test_application();
        app.metadata.applicant_name = String::new();

        let report = validate_application(&app).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors.iter().any(|e| e.contains("Applicant name")));
    }

    #[test]
    fn test_validate_invalid_application_id() {
        let mut app = create_test_application();
        app.metadata.application_id = "AB".to_string(); // Too short

        let report = validate_application(&app).unwrap();
        assert!(!report.is_valid());
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.contains("Invalid application ID"))
        );
    }

    #[test]
    fn test_is_valid_application_id() {
        assert!(is_valid_application_id("APP-001"));
        assert!(is_valid_application_id("TEST_APPLICATION_123"));
        assert!(!is_valid_application_id("AB")); // Too short
        assert!(!is_valid_application_id("APP 001")); // Contains space
        assert!(!is_valid_application_id("")); // Empty
    }

    #[test]
    fn test_validate_required_fields() {
        let mut app = create_test_application();
        app.metadata.application_type = "administrative_procedure".to_string();

        // Missing required fields
        let report = validate_application(&app).unwrap();
        assert!(!report.is_valid());

        // Add required fields
        app.add_field(
            "procedure_type",
            EgovFieldValue::Text("Application".to_string()),
        );
        app.add_field(
            "agency",
            EgovFieldValue::Text("MinistryOfJustice".to_string()),
        );
        app.add_field("applicant_name", EgovFieldValue::Text("Test".to_string()));

        let report = validate_application(&app).unwrap();
        // Should pass required field validation
        assert!(
            report
                .errors
                .iter()
                .all(|e| !e.contains("Required field") || e.contains("is empty"))
        );
    }

    #[test]
    fn test_validate_attachments_size() {
        let mut app = create_test_application();

        let large_attachment = Attachment::new(
            "ATT-001",
            "large.pdf",
            "application/pdf",
            20 * 1024 * 1024, // 20MB
        );
        app.add_attachment(large_attachment);

        let report = validate_application(&app).unwrap();
        assert!(!report.is_valid());
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.contains("exceeds size limit"))
        );
    }

    #[test]
    fn test_validate_attachments_type() {
        let mut app = create_test_application();

        let invalid_attachment =
            Attachment::new("ATT-001", "script.exe", "application/x-executable", 1000);
        app.add_attachment(invalid_attachment);

        let report = validate_application(&app).unwrap();
        assert!(!report.is_valid());
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.contains("unsupported file type"))
        );
    }

    #[test]
    fn test_validate_required_attachment_missing_content() {
        let mut app = create_test_application();

        let required_attachment =
            Attachment::new("ATT-001", "required.pdf", "application/pdf", 1000).required();
        app.add_attachment(required_attachment);

        let report = validate_application(&app).unwrap();
        assert!(!report.is_valid());
        assert!(report.errors.iter().any(|e| e.contains("has no content")));
    }

    #[test]
    fn test_validate_status_transition_valid() {
        assert!(
            validate_status_transition(ApplicationStatus::Draft, ApplicationStatus::Submitted)
                .is_ok()
        );
        assert!(
            validate_status_transition(
                ApplicationStatus::Submitted,
                ApplicationStatus::UnderReview
            )
            .is_ok()
        );
    }

    #[test]
    fn test_validate_status_transition_invalid() {
        let result =
            validate_status_transition(ApplicationStatus::Draft, ApplicationStatus::Approved);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EgovError::InvalidStatusTransition { .. }
        ));
    }

    #[test]
    fn test_quick_validate() {
        let app = create_test_application();
        assert!(quick_validate(&app));

        let mut invalid_app = create_test_application();
        invalid_app.metadata.applicant_name = String::new();
        assert!(!quick_validate(&invalid_app));
    }
}
