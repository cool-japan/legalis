//! e-Gov XML Parser
//!
//! Provides XML parsing and serialization for e-Gov electronic filing format.
//! Supports legacy e-Gov XML format used by Japanese government agencies.

use crate::egov::{
    error::{EgovError, Result},
    types::{
        ApplicationMetadata, ApplicationStatus, Attachment, EgovApplication, EgovFieldValue,
        GovernmentAgency,
    },
};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Reader, Writer};
use std::collections::HashMap;
use std::io::Cursor;

/// e-Gov XML parser for parsing and serializing applications
pub struct EgovXmlParser {
    /// Whether to validate against schema (not implemented yet)
    validate_schema: bool,
}

impl EgovXmlParser {
    /// Create new XML parser
    pub fn new() -> Self {
        Self {
            validate_schema: false,
        }
    }

    /// Create new XML parser with schema validation
    pub fn with_schema_validation(mut self, validate: bool) -> Self {
        self.validate_schema = validate;
        self
    }

    /// Parse e-Gov XML format into EgovApplication
    pub fn parse_application(&self, xml: &str) -> Result<EgovApplication> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut application_id = String::new();
        let mut application_type = String::new();
        let mut applicant_name = String::new();
        let mut submission_date = None;
        let mut status = ApplicationStatus::Draft;
        let mut agency = GovernmentAgency::DigitalAgency;
        let mut created_date = chrono::Utc::now().date_naive();
        let mut updated_date = created_date;
        let mut notes = None;
        let mut form_data = HashMap::new();
        let mut attachments = Vec::new();

        let mut buf = Vec::new();
        let mut current_element = String::new();
        let mut in_metadata = false;
        let mut in_form_data = false;
        let mut in_attachments = false;
        let mut current_field_key = String::new();
        let mut current_attachment_id = String::new();
        let mut current_attachment_filename = String::new();
        let mut current_attachment_mime = String::new();
        let mut current_attachment_size = 0;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    current_element = name.clone();

                    match name.as_str() {
                        "metadata" => in_metadata = true,
                        "form_data" => in_form_data = true,
                        "attachments" => in_attachments = true,
                        "field" => {
                            if in_form_data {
                                // Get key attribute
                                for attr in e.attributes().flatten() {
                                    if attr.key.as_ref() == b"key" {
                                        current_field_key =
                                            String::from_utf8_lossy(&attr.value).to_string();
                                    }
                                }
                            }
                        }
                        "attachment" => {
                            if in_attachments {
                                current_attachment_id.clear();
                                current_attachment_filename.clear();
                                current_attachment_mime.clear();
                                current_attachment_size = 0;
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    let text = std::str::from_utf8(e.as_ref())
                        .map_err(|e| EgovError::XmlParse(format!("Invalid UTF-8: {}", e)))?;

                    if in_metadata {
                        match current_element.as_str() {
                            "application_id" => application_id = text.to_string(),
                            "application_type" => application_type = text.to_string(),
                            "applicant_name" => applicant_name = text.to_string(),
                            "submission_date" => {
                                submission_date =
                                    chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d").ok();
                            }
                            "status" => {
                                status = parse_application_status(text)?;
                            }
                            "agency" => {
                                agency = parse_government_agency(text)?;
                            }
                            "created_date" => {
                                created_date = chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d")
                                    .map_err(|e| {
                                        EgovError::XmlParse(format!("Invalid created_date: {}", e))
                                    })?;
                            }
                            "updated_date" => {
                                updated_date = chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d")
                                    .map_err(|e| {
                                        EgovError::XmlParse(format!("Invalid updated_date: {}", e))
                                    })?;
                            }
                            "notes" => notes = Some(text.to_string()),
                            _ => {}
                        }
                    } else if in_form_data && !current_field_key.is_empty() {
                        match current_element.as_str() {
                            "text" => {
                                form_data.insert(
                                    current_field_key.clone(),
                                    EgovFieldValue::Text(text.to_string()),
                                );
                            }
                            "number" => {
                                if let Ok(num) = text.parse::<i64>() {
                                    form_data.insert(
                                        current_field_key.clone(),
                                        EgovFieldValue::Number(num),
                                    );
                                }
                            }
                            "decimal" => {
                                if let Ok(num) = text.parse::<f64>() {
                                    form_data.insert(
                                        current_field_key.clone(),
                                        EgovFieldValue::Decimal(num),
                                    );
                                }
                            }
                            "date" => {
                                if let Ok(date) =
                                    chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d")
                                {
                                    form_data.insert(
                                        current_field_key.clone(),
                                        EgovFieldValue::Date(date),
                                    );
                                }
                            }
                            "boolean" => {
                                if let Ok(b) = text.parse::<bool>() {
                                    form_data.insert(
                                        current_field_key.clone(),
                                        EgovFieldValue::Boolean(b),
                                    );
                                }
                            }
                            _ => {}
                        }
                    } else if in_attachments {
                        match current_element.as_str() {
                            "id" => current_attachment_id = text.to_string(),
                            "filename" => current_attachment_filename = text.to_string(),
                            "mime_type" => current_attachment_mime = text.to_string(),
                            "size_bytes" => {
                                current_attachment_size = text.parse().unwrap_or(0);
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match name.as_str() {
                        "metadata" => in_metadata = false,
                        "form_data" => in_form_data = false,
                        "attachments" => in_attachments = false,
                        "field" => {
                            current_field_key.clear();
                        }
                        "attachment" => {
                            if !current_attachment_id.is_empty() {
                                attachments.push(Attachment::new(
                                    current_attachment_id.clone(),
                                    current_attachment_filename.clone(),
                                    current_attachment_mime.clone(),
                                    current_attachment_size,
                                ));
                            }
                        }
                        _ => {}
                    }
                    current_element.clear();
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(EgovError::XmlParse(format!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    )));
                }
                _ => {}
            }
            buf.clear();
        }

        // Validate required fields
        if application_id.is_empty() {
            return Err(EgovError::MissingRequiredField {
                field: "application_id".to_string(),
            });
        }
        if applicant_name.is_empty() {
            return Err(EgovError::MissingRequiredField {
                field: "applicant_name".to_string(),
            });
        }

        let metadata = ApplicationMetadata {
            application_id,
            application_type,
            applicant_name,
            submission_date,
            status,
            agency,
            created_date,
            updated_date,
            notes,
        };

        Ok(EgovApplication {
            metadata,
            form_data,
            attachments,
        })
    }

    /// Serialize EgovApplication to XML format
    pub fn serialize_application(&self, app: &EgovApplication) -> Result<String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        // Write XML declaration
        writer
            .write_event(Event::Decl(quick_xml::events::BytesDecl::new(
                "1.0",
                Some("UTF-8"),
                None,
            )))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to write declaration: {}", e)))?;

        // Start application element
        writer
            .write_event(Event::Start(BytesStart::new("application")))
            .map_err(|e| {
                EgovError::XmlSerialize(format!("Failed to write application element: {}", e))
            })?;

        // Write metadata
        self.write_metadata(&mut writer, &app.metadata)?;

        // Write form_data
        self.write_form_data(&mut writer, &app.form_data)?;

        // Write attachments
        self.write_attachments(&mut writer, &app.attachments)?;

        // End application element
        writer
            .write_event(Event::End(BytesEnd::new("application")))
            .map_err(|e| {
                EgovError::XmlSerialize(format!("Failed to close application element: {}", e))
            })?;

        let result = writer.into_inner().into_inner();
        String::from_utf8(result)
            .map_err(|e| EgovError::XmlSerialize(format!("Invalid UTF-8: {}", e)))
    }

    fn write_metadata(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        metadata: &ApplicationMetadata,
    ) -> Result<()> {
        writer
            .write_event(Event::Start(BytesStart::new("metadata")))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to write metadata: {}", e)))?;

        self.write_element(writer, "application_id", &metadata.application_id)?;
        self.write_element(writer, "application_type", &metadata.application_type)?;
        self.write_element(writer, "applicant_name", &metadata.applicant_name)?;

        if let Some(date) = metadata.submission_date {
            self.write_element(
                writer,
                "submission_date",
                &date.format("%Y-%m-%d").to_string(),
            )?;
        }

        self.write_element(writer, "status", metadata.status.display_en())?;
        self.write_element(writer, "agency", metadata.agency.name_en())?;
        self.write_element(
            writer,
            "created_date",
            &metadata.created_date.format("%Y-%m-%d").to_string(),
        )?;
        self.write_element(
            writer,
            "updated_date",
            &metadata.updated_date.format("%Y-%m-%d").to_string(),
        )?;

        if let Some(notes) = &metadata.notes {
            self.write_element(writer, "notes", notes)?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("metadata")))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to close metadata: {}", e)))?;

        Ok(())
    }

    fn write_form_data(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        form_data: &HashMap<String, EgovFieldValue>,
    ) -> Result<()> {
        writer
            .write_event(Event::Start(BytesStart::new("form_data")))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to write form_data: {}", e)))?;

        for (key, value) in form_data {
            let mut field_elem = BytesStart::new("field");
            field_elem.push_attribute(("key", key.as_str()));

            writer
                .write_event(Event::Start(field_elem))
                .map_err(|e| EgovError::XmlSerialize(format!("Failed to write field: {}", e)))?;

            match value {
                EgovFieldValue::Text(text) => {
                    self.write_element(writer, "text", text)?;
                }
                EgovFieldValue::Number(num) => {
                    self.write_element(writer, "number", &num.to_string())?;
                }
                EgovFieldValue::Decimal(dec) => {
                    self.write_element(writer, "decimal", &dec.to_string())?;
                }
                EgovFieldValue::Date(date) => {
                    self.write_element(writer, "date", &date.format("%Y-%m-%d").to_string())?;
                }
                EgovFieldValue::Boolean(b) => {
                    self.write_element(writer, "boolean", &b.to_string())?;
                }
                EgovFieldValue::List(_) => {
                    // Simplified: skip lists for now
                }
                EgovFieldValue::Object(_) => {
                    // Simplified: skip objects for now
                }
            }

            writer
                .write_event(Event::End(BytesEnd::new("field")))
                .map_err(|e| EgovError::XmlSerialize(format!("Failed to close field: {}", e)))?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("form_data")))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to close form_data: {}", e)))?;

        Ok(())
    }

    fn write_attachments(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        attachments: &[Attachment],
    ) -> Result<()> {
        writer
            .write_event(Event::Start(BytesStart::new("attachments")))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to write attachments: {}", e)))?;

        for attachment in attachments {
            writer
                .write_event(Event::Start(BytesStart::new("attachment")))
                .map_err(|e| {
                    EgovError::XmlSerialize(format!("Failed to write attachment: {}", e))
                })?;

            self.write_element(writer, "id", &attachment.id)?;
            self.write_element(writer, "filename", &attachment.filename)?;
            self.write_element(writer, "mime_type", &attachment.mime_type)?;
            self.write_element(writer, "size_bytes", &attachment.size_bytes.to_string())?;

            if let Some(description) = &attachment.description {
                self.write_element(writer, "description", description)?;
            }

            self.write_element(writer, "required", &attachment.required.to_string())?;

            writer
                .write_event(Event::End(BytesEnd::new("attachment")))
                .map_err(|e| {
                    EgovError::XmlSerialize(format!("Failed to close attachment: {}", e))
                })?;
        }

        writer
            .write_event(Event::End(BytesEnd::new("attachments")))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to close attachments: {}", e)))?;

        Ok(())
    }

    fn write_element(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        tag: &str,
        content: &str,
    ) -> Result<()> {
        writer
            .write_event(Event::Start(BytesStart::new(tag)))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to write <{}>: {}", tag, e)))?;

        writer
            .write_event(Event::Text(BytesText::new(content)))
            .map_err(|e| {
                EgovError::XmlSerialize(format!("Failed to write text for <{}>: {}", tag, e))
            })?;

        writer
            .write_event(Event::End(BytesEnd::new(tag)))
            .map_err(|e| EgovError::XmlSerialize(format!("Failed to close <{}>: {}", tag, e)))?;

        Ok(())
    }
}

impl Default for EgovXmlParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse application status from string
fn parse_application_status(s: &str) -> Result<ApplicationStatus> {
    match s {
        "Draft" => Ok(ApplicationStatus::Draft),
        "Submitted" => Ok(ApplicationStatus::Submitted),
        "UnderReview" | "Under Review" => Ok(ApplicationStatus::UnderReview),
        "Accepted" => Ok(ApplicationStatus::Accepted),
        "Rejected" => Ok(ApplicationStatus::Rejected),
        "RequiresRevision" | "Requires Revision" => Ok(ApplicationStatus::RequiresRevision),
        "Approved" => Ok(ApplicationStatus::Approved),
        "Withdrawn" => Ok(ApplicationStatus::Withdrawn),
        _ => Err(EgovError::XmlParse(format!(
            "Unknown application status: {}",
            s
        ))),
    }
}

/// Parse government agency from string
fn parse_government_agency(s: &str) -> Result<GovernmentAgency> {
    match s {
        "Digital Agency" | "DigitalAgency" | "00100" => Ok(GovernmentAgency::DigitalAgency),
        "Ministry of Justice" | "MinistryOfJustice" | "00200" => {
            Ok(GovernmentAgency::MinistryOfJustice)
        }
        "Ministry of Land, Infrastructure, Transport and Tourism" | "MinistryOfLand" | "00800" => {
            Ok(GovernmentAgency::MinistryOfLand)
        }
        "Ministry of the Environment" | "MinistryOfEnvironment" | "01300" => {
            Ok(GovernmentAgency::MinistryOfEnvironment)
        }
        "Ministry of Economy, Trade and Industry" | "MinistryOfEconomy" | "00900" => {
            Ok(GovernmentAgency::MinistryOfEconomy)
        }
        "Personal Information Protection Commission" | "PersonalInfoCommission" | "01800" => {
            Ok(GovernmentAgency::PersonalInfoCommission)
        }
        "Ministry of Health, Labour and Welfare" | "MinistryOfHealthLabour" | "00500" => {
            Ok(GovernmentAgency::MinistryOfHealthLabour)
        }
        "Consumer Affairs Agency" | "ConsumerAffairsAgency" | "01700" => {
            Ok(GovernmentAgency::ConsumerAffairsAgency)
        }
        _ => Err(EgovError::XmlParse(format!(
            "Unknown government agency: {}",
            s
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_xml() -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<application>
  <metadata>
    <application_id>APP-TEST-001</application_id>
    <application_type>test_application</application_type>
    <applicant_name>Test Applicant</applicant_name>
    <status>Draft</status>
    <agency>Digital Agency</agency>
    <created_date>2026-01-09</created_date>
    <updated_date>2026-01-09</updated_date>
  </metadata>
  <form_data>
    <field key="test_field">
      <text>Test Value</text>
    </field>
    <field key="test_number">
      <number>42</number>
    </field>
  </form_data>
  <attachments>
    <attachment>
      <id>ATT-001</id>
      <filename>test.pdf</filename>
      <mime_type>application/pdf</mime_type>
      <size_bytes>1000</size_bytes>
      <required>false</required>
    </attachment>
  </attachments>
</application>"#
            .to_string()
    }

    #[test]
    fn test_parse_valid_xml() {
        let parser = EgovXmlParser::new();
        let xml = create_test_xml();
        let result = parser.parse_application(&xml);

        assert!(result.is_ok());
        let app = result.unwrap();
        assert_eq!(app.metadata.application_id, "APP-TEST-001");
        assert_eq!(app.metadata.applicant_name, "Test Applicant");
        assert_eq!(app.metadata.status, ApplicationStatus::Draft);
    }

    #[test]
    fn test_parse_form_data() {
        let parser = EgovXmlParser::new();
        let xml = create_test_xml();
        let app = parser.parse_application(&xml).unwrap();

        assert_eq!(app.form_data.len(), 2);
        assert!(app.form_data.contains_key("test_field"));
        assert!(app.form_data.contains_key("test_number"));

        if let Some(EgovFieldValue::Text(text)) = app.form_data.get("test_field") {
            assert_eq!(text, "Test Value");
        } else {
            panic!("Expected text field");
        }

        if let Some(EgovFieldValue::Number(num)) = app.form_data.get("test_number") {
            assert_eq!(*num, 42);
        } else {
            panic!("Expected number field");
        }
    }

    #[test]
    fn test_parse_attachments() {
        let parser = EgovXmlParser::new();
        let xml = create_test_xml();
        let app = parser.parse_application(&xml).unwrap();

        assert_eq!(app.attachments.len(), 1);
        assert_eq!(app.attachments[0].id, "ATT-001");
        assert_eq!(app.attachments[0].filename, "test.pdf");
        assert_eq!(app.attachments[0].mime_type, "application/pdf");
        assert_eq!(app.attachments[0].size_bytes, 1000);
    }

    #[test]
    fn test_parse_missing_required_field() {
        let parser = EgovXmlParser::new();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<application>
  <metadata>
    <application_type>test</application_type>
  </metadata>
  <form_data></form_data>
  <attachments></attachments>
</application>"#;

        let result = parser.parse_application(xml);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EgovError::MissingRequiredField { .. }
        ));
    }

    #[test]
    fn test_serialize_application() {
        let parser = EgovXmlParser::new();
        let mut app = EgovApplication::new(
            "APP-TEST-002",
            "test_application",
            "Test Applicant",
            GovernmentAgency::DigitalAgency,
        );

        app.add_field("test_field", EgovFieldValue::Text("Test Value".to_string()));
        app.add_field("test_number", EgovFieldValue::Number(42));

        let xml = parser.serialize_application(&app);
        assert!(xml.is_ok());

        let xml_string = xml.unwrap();
        assert!(xml_string.contains("<application_id>APP-TEST-002</application_id>"));
        assert!(xml_string.contains("<applicant_name>Test Applicant</applicant_name>"));
        assert!(xml_string.contains("test_field"));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let parser = EgovXmlParser::new();
        let xml = create_test_xml();

        // Parse XML
        let app1 = parser.parse_application(&xml).unwrap();

        // Serialize to XML
        let xml2 = parser.serialize_application(&app1).unwrap();

        // Parse again
        let app2 = parser.parse_application(&xml2).unwrap();

        // Compare
        assert_eq!(app1.metadata.application_id, app2.metadata.application_id);
        assert_eq!(app1.metadata.applicant_name, app2.metadata.applicant_name);
        assert_eq!(app1.metadata.status, app2.metadata.status);
    }

    #[test]
    fn test_parse_application_status() {
        assert_eq!(
            parse_application_status("Draft").unwrap(),
            ApplicationStatus::Draft
        );
        assert_eq!(
            parse_application_status("Submitted").unwrap(),
            ApplicationStatus::Submitted
        );
        assert_eq!(
            parse_application_status("Under Review").unwrap(),
            ApplicationStatus::UnderReview
        );
        assert!(parse_application_status("Invalid").is_err());
    }

    #[test]
    fn test_parse_government_agency() {
        assert_eq!(
            parse_government_agency("Digital Agency").unwrap(),
            GovernmentAgency::DigitalAgency
        );
        assert_eq!(
            parse_government_agency("00200").unwrap(),
            GovernmentAgency::MinistryOfJustice
        );
        assert!(parse_government_agency("Invalid").is_err());
    }

    #[test]
    fn test_parse_malformed_xml() {
        let parser = EgovXmlParser::new();
        let xml = "<application><metadata><unclosed>";

        let result = parser.parse_application(xml);
        assert!(result.is_err());
        // XML may parse successfully but fail validation for missing required fields
        // Accept either XmlParse or MissingRequiredField errors
        match result.unwrap_err() {
            EgovError::XmlParse(_) | EgovError::MissingRequiredField { .. } => {}
            _ => panic!("Expected XmlParse or MissingRequiredField error"),
        }
    }

    #[test]
    fn test_serialize_with_attachments() {
        let parser = EgovXmlParser::new();
        let mut app = EgovApplication::new(
            "APP-TEST-003",
            "test",
            "Test User",
            GovernmentAgency::MinistryOfJustice,
        );

        let attachment = Attachment::new("ATT-001", "document.pdf", "application/pdf", 2000)
            .with_description("Test document");
        app.add_attachment(attachment);

        let xml = parser.serialize_application(&app).unwrap();
        assert!(xml.contains("<attachments>"));
        assert!(xml.contains("<filename>document.pdf</filename>"));
        assert!(xml.contains("<description>Test document</description>"));
    }
}
