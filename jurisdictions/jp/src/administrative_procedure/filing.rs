//! e-Gov Filing Integration
//!
//! Convert administrative procedures to e-Gov application format for electronic submission.

use crate::administrative_procedure::{
    error::{AdministrativeError, Result},
    types::AdministrativeProcedure,
};
use crate::egov::{EgovApplication, EgovFieldValue, EgovXmlParser};

/// e-Gov filing service for administrative procedures
pub struct AdministrativeFilingService {
    xml_parser: EgovXmlParser,
}

impl AdministrativeFilingService {
    /// Create new filing service
    pub fn new() -> Self {
        Self {
            xml_parser: EgovXmlParser::new(),
        }
    }

    /// Prepare e-Gov application from administrative procedure
    pub fn prepare_filing(&self, procedure: &AdministrativeProcedure) -> Result<EgovApplication> {
        let mut app = EgovApplication::new(
            &procedure.procedure_id,
            "administrative_procedure",
            &procedure.applicant.name,
            procedure.agency,
        );

        // Add procedure type
        app.add_field(
            "procedure_type",
            EgovFieldValue::Text(procedure.procedure_type.name_en().to_string()),
        );

        // Add applicant type
        app.add_field(
            "applicant_type",
            EgovFieldValue::Text(format!("{:?}", procedure.applicant.applicant_type)),
        );

        // Add contact information
        app.add_field(
            "address",
            EgovFieldValue::Text(procedure.applicant.contact.address.clone()),
        );

        if let Some(phone) = &procedure.applicant.contact.phone {
            app.add_field("phone", EgovFieldValue::Text(phone.clone()));
        }

        if let Some(email) = &procedure.applicant.contact.email {
            app.add_field("email", EgovFieldValue::Text(email.clone()));
        }

        // Add identification if available
        if let Some(id) = &procedure.applicant.identification {
            app.add_field("id_type", EgovFieldValue::Text(id.id_type.clone()));
            app.add_field("id_number", EgovFieldValue::Text(id.id_number.clone()));
        }

        // Add submission date
        app.add_field(
            "submission_date",
            EgovFieldValue::Date(procedure.submission_date),
        );

        // Add processing period if set
        if let Some(period) = procedure.processing_period_days {
            app.add_field(
                "processing_period_days",
                EgovFieldValue::Number(period as i64),
            );
        }

        // Add document count
        app.add_field(
            "document_count",
            EgovFieldValue::Number(procedure.documents.len() as i64),
        );

        // Add electronic signature status
        app.add_field(
            "electronically_signed",
            EgovFieldValue::Boolean(procedure.is_electronically_signed()),
        );

        // Add reason statement status
        app.add_field(
            "has_reason_statement",
            EgovFieldValue::Boolean(procedure.has_reason_statement()),
        );

        // Add notes if present
        if let Some(notes) = &procedure.notes {
            app.add_field("notes", EgovFieldValue::Text(notes.clone()));
        }

        Ok(app)
    }

    /// Export administrative procedure as XML
    pub fn export_xml(&self, procedure: &AdministrativeProcedure) -> Result<String> {
        let app = self.prepare_filing(procedure)?;
        self.xml_parser
            .serialize_application(&app)
            .map_err(|e| AdministrativeError::FilingError(e.to_string()))
    }

    /// Export administrative procedure as JSON
    pub fn export_json(&self, procedure: &AdministrativeProcedure) -> Result<String> {
        let app = self.prepare_filing(procedure)?;
        let json_formatter = crate::egov::EgovJsonFormatter::new().with_pretty_print(true);
        json_formatter
            .serialize_application(&app)
            .map_err(|e| AdministrativeError::FilingError(e.to_string()))
    }
}

impl Default for AdministrativeFilingService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::administrative_procedure::types::{Applicant, ProcedureType};
    use crate::egov::GovernmentAgency;

    #[test]
    fn test_prepare_filing() {
        let applicant = Applicant::individual("田中太郎", "東京都渋谷区")
            .with_phone("03-1234-5678")
            .with_email("tanaka@example.com");

        let procedure = AdministrativeProcedure::new(
            "PROC-TEST-001",
            ProcedureType::Application,
            GovernmentAgency::DigitalAgency,
            applicant,
        );

        let service = AdministrativeFilingService::new();
        let app = service.prepare_filing(&procedure).unwrap();

        assert_eq!(app.metadata.application_id, "PROC-TEST-001");
        assert_eq!(app.metadata.applicant_name, "田中太郎");
        assert!(app.form_data.contains_key("procedure_type"));
        assert!(app.form_data.contains_key("address"));
    }

    #[test]
    fn test_export_xml() {
        let applicant = Applicant::individual("Test User", "Tokyo");
        let procedure = AdministrativeProcedure::new(
            "PROC-XML-001",
            ProcedureType::Notification,
            GovernmentAgency::MinistryOfJustice,
            applicant,
        );

        let service = AdministrativeFilingService::new();
        let xml = service.export_xml(&procedure);

        assert!(xml.is_ok());
        let xml_string = xml.unwrap();
        assert!(xml_string.contains("<application_id>PROC-XML-001</application_id>"));
        assert!(xml_string.contains("Test User"));
    }

    #[test]
    fn test_export_json() {
        let applicant = Applicant::individual("Test User", "Tokyo");
        let procedure = AdministrativeProcedure::new(
            "PROC-JSON-001",
            ProcedureType::Application,
            GovernmentAgency::DigitalAgency,
            applicant,
        );

        let service = AdministrativeFilingService::new();
        let json = service.export_json(&procedure);

        assert!(json.is_ok());
        let json_string = json.unwrap();
        assert!(json_string.contains("PROC-JSON-001"));
        assert!(json_string.contains("Test User"));
    }

    #[test]
    fn test_filing_with_processing_period() {
        let applicant = Applicant::individual("Test User", "Tokyo");
        let mut procedure = AdministrativeProcedure::new(
            "PROC-PERIOD-001",
            ProcedureType::Application,
            GovernmentAgency::MinistryOfEnvironment,
            applicant,
        );

        procedure.set_processing_period(30);

        let service = AdministrativeFilingService::new();
        let app = service.prepare_filing(&procedure).unwrap();

        assert!(app.form_data.contains_key("processing_period_days"));
        if let Some(EgovFieldValue::Number(period)) = app.form_data.get("processing_period_days") {
            assert_eq!(*period, 30);
        } else {
            panic!("Expected processing_period_days to be a Number");
        }
    }
}
