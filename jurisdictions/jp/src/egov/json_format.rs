//! e-Gov JSON Format Handler
//!
//! Provides modern JSON format parsing and serialization for e-Gov electronic filing,
//! as well as conversion between XML and JSON formats.

use crate::egov::{
    error::{EgovError, Result},
    types::{
        ApplicationMetadata, ApplicationStatus, Attachment, EgovApplication, EgovFieldValue,
        GovernmentAgency,
    },
    xml_parser::EgovXmlParser,
};
use serde_json::{Map, Value, json};
use std::collections::HashMap;

/// e-Gov JSON formatter for parsing and serializing applications
pub struct EgovJsonFormatter {
    /// Whether to pretty-print JSON output
    pretty_print: bool,
}

impl EgovJsonFormatter {
    /// Create new JSON formatter
    pub fn new() -> Self {
        Self {
            pretty_print: false,
        }
    }

    /// Enable pretty printing
    pub fn with_pretty_print(mut self, pretty: bool) -> Self {
        self.pretty_print = pretty;
        self
    }

    /// Parse e-Gov JSON format into EgovApplication
    pub fn parse_application(&self, json: &str) -> Result<EgovApplication> {
        let value: Value = serde_json::from_str(json)
            .map_err(|e| EgovError::JsonParse(format!("Failed to parse JSON: {}", e)))?;

        let obj = value
            .as_object()
            .ok_or_else(|| EgovError::JsonParse("Root element must be an object".to_string()))?;

        // Parse metadata
        let metadata_value =
            obj.get("metadata")
                .ok_or_else(|| EgovError::MissingRequiredField {
                    field: "metadata".to_string(),
                })?;

        let metadata = self.parse_metadata(metadata_value)?;

        // Parse form_data
        let form_data = if let Some(form_data_value) = obj.get("form_data") {
            self.parse_form_data(form_data_value)?
        } else {
            HashMap::new()
        };

        // Parse attachments
        let attachments = if let Some(attachments_value) = obj.get("attachments") {
            self.parse_attachments(attachments_value)?
        } else {
            Vec::new()
        };

        Ok(EgovApplication {
            metadata,
            form_data,
            attachments,
        })
    }

    /// Serialize EgovApplication to JSON format
    pub fn serialize_application(&self, app: &EgovApplication) -> Result<String> {
        let json_value = json!({
            "metadata": self.metadata_to_json(&app.metadata),
            "form_data": self.form_data_to_json(&app.form_data),
            "attachments": self.attachments_to_json(&app.attachments),
        });

        let result = if self.pretty_print {
            serde_json::to_string_pretty(&json_value)
        } else {
            serde_json::to_string(&json_value)
        };

        result.map_err(|e| EgovError::JsonSerialize(format!("Failed to serialize JSON: {}", e)))
    }

    /// Convert XML format to JSON format
    pub fn xml_to_json(&self, xml: &str) -> Result<String> {
        let parser = EgovXmlParser::new();
        let app = parser.parse_application(xml)?;
        self.serialize_application(&app)
    }

    /// Convert JSON format to XML format
    pub fn json_to_xml(&self, json: &str) -> Result<String> {
        let app = self.parse_application(json)?;
        let parser = EgovXmlParser::new();
        parser.serialize_application(&app)
    }

    fn parse_metadata(&self, value: &Value) -> Result<ApplicationMetadata> {
        let obj = value
            .as_object()
            .ok_or_else(|| EgovError::JsonParse("metadata must be an object".to_string()))?;

        let application_id = obj
            .get("application_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EgovError::MissingRequiredField {
                field: "application_id".to_string(),
            })?
            .to_string();

        let application_type = obj
            .get("application_type")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let applicant_name = obj
            .get("applicant_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| EgovError::MissingRequiredField {
                field: "applicant_name".to_string(),
            })?
            .to_string();

        let submission_date = obj
            .get("submission_date")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

        let status = obj
            .get("status")
            .and_then(|v| v.as_str())
            .map(|s| self.parse_status(s))
            .transpose()?
            .unwrap_or(ApplicationStatus::Draft);

        let agency = obj
            .get("agency")
            .and_then(|v| v.as_str())
            .map(|s| self.parse_agency(s))
            .transpose()?
            .unwrap_or(GovernmentAgency::DigitalAgency);

        let created_date = obj
            .get("created_date")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::Utc::now().date_naive());

        let updated_date = obj
            .get("updated_date")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
            .unwrap_or(created_date);

        let notes = obj.get("notes").and_then(|v| v.as_str()).map(String::from);

        Ok(ApplicationMetadata {
            application_id,
            application_type,
            applicant_name,
            submission_date,
            status,
            agency,
            created_date,
            updated_date,
            notes,
        })
    }

    fn parse_form_data(&self, value: &Value) -> Result<HashMap<String, EgovFieldValue>> {
        let obj = value
            .as_object()
            .ok_or_else(|| EgovError::JsonParse("form_data must be an object".to_string()))?;

        let mut form_data = HashMap::new();

        for (key, field_value) in obj {
            let parsed_value = Self::parse_field_value(field_value)?;
            form_data.insert(key.clone(), parsed_value);
        }

        Ok(form_data)
    }

    fn parse_field_value(value: &Value) -> Result<EgovFieldValue> {
        match value {
            Value::String(s) => {
                // Try to parse as date
                if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                    Ok(EgovFieldValue::Date(date))
                } else {
                    Ok(EgovFieldValue::Text(s.clone()))
                }
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(EgovFieldValue::Number(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(EgovFieldValue::Decimal(f))
                } else {
                    Err(EgovError::JsonParse("Invalid number format".to_string()))
                }
            }
            Value::Bool(b) => Ok(EgovFieldValue::Boolean(*b)),
            Value::Array(arr) => {
                let mut list = Vec::new();
                for item in arr {
                    list.push(Self::parse_field_value(item)?);
                }
                Ok(EgovFieldValue::List(list))
            }
            Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), Self::parse_field_value(v)?);
                }
                Ok(EgovFieldValue::Object(map))
            }
            Value::Null => Ok(EgovFieldValue::Text(String::new())),
        }
    }

    fn parse_attachments(&self, value: &Value) -> Result<Vec<Attachment>> {
        let arr = value
            .as_array()
            .ok_or_else(|| EgovError::JsonParse("attachments must be an array".to_string()))?;

        let mut attachments = Vec::new();

        for item in arr {
            let obj = item
                .as_object()
                .ok_or_else(|| EgovError::JsonParse("attachment must be an object".to_string()))?;

            let id = obj
                .get("id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| EgovError::JsonParse("attachment missing id".to_string()))?
                .to_string();

            let filename = obj
                .get("filename")
                .and_then(|v| v.as_str())
                .ok_or_else(|| EgovError::JsonParse("attachment missing filename".to_string()))?
                .to_string();

            let mime_type = obj
                .get("mime_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| EgovError::JsonParse("attachment missing mime_type".to_string()))?
                .to_string();

            let size_bytes = obj
                .get("size_bytes")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| EgovError::JsonParse("attachment missing size_bytes".to_string()))?
                as usize;

            let mut attachment = Attachment::new(id, filename, mime_type, size_bytes);

            if let Some(description) = obj.get("description").and_then(|v| v.as_str()) {
                attachment = attachment.with_description(description);
            }

            if let Some(required) = obj.get("required").and_then(|v| v.as_bool())
                && required
            {
                attachment = attachment.required();
            }

            attachments.push(attachment);
        }

        Ok(attachments)
    }

    fn metadata_to_json(&self, metadata: &ApplicationMetadata) -> Value {
        let mut obj = Map::new();

        obj.insert(
            "application_id".to_string(),
            Value::String(metadata.application_id.clone()),
        );
        obj.insert(
            "application_type".to_string(),
            Value::String(metadata.application_type.clone()),
        );
        obj.insert(
            "applicant_name".to_string(),
            Value::String(metadata.applicant_name.clone()),
        );

        if let Some(date) = metadata.submission_date {
            obj.insert(
                "submission_date".to_string(),
                Value::String(date.format("%Y-%m-%d").to_string()),
            );
        }

        obj.insert(
            "status".to_string(),
            Value::String(metadata.status.display_en().to_string()),
        );
        obj.insert(
            "agency".to_string(),
            Value::String(metadata.agency.name_en().to_string()),
        );
        obj.insert(
            "created_date".to_string(),
            Value::String(metadata.created_date.format("%Y-%m-%d").to_string()),
        );
        obj.insert(
            "updated_date".to_string(),
            Value::String(metadata.updated_date.format("%Y-%m-%d").to_string()),
        );

        if let Some(notes) = &metadata.notes {
            obj.insert("notes".to_string(), Value::String(notes.clone()));
        }

        Value::Object(obj)
    }

    fn form_data_to_json(&self, form_data: &HashMap<String, EgovFieldValue>) -> Value {
        let mut obj = Map::new();

        for (key, value) in form_data {
            obj.insert(key.clone(), Self::field_value_to_json(value));
        }

        Value::Object(obj)
    }

    fn field_value_to_json(value: &EgovFieldValue) -> Value {
        match value {
            EgovFieldValue::Text(s) => Value::String(s.clone()),
            EgovFieldValue::Number(n) => json!(*n),
            EgovFieldValue::Decimal(d) => json!(*d),
            EgovFieldValue::Date(d) => Value::String(d.format("%Y-%m-%d").to_string()),
            EgovFieldValue::Boolean(b) => Value::Bool(*b),
            EgovFieldValue::List(list) => {
                let arr: Vec<Value> = list.iter().map(Self::field_value_to_json).collect();
                Value::Array(arr)
            }
            EgovFieldValue::Object(obj) => {
                let mut map = Map::new();
                for (k, v) in obj {
                    map.insert(k.clone(), Self::field_value_to_json(v));
                }
                Value::Object(map)
            }
        }
    }

    fn attachments_to_json(&self, attachments: &[Attachment]) -> Value {
        let arr: Vec<Value> = attachments
            .iter()
            .map(|attachment| {
                let mut obj = Map::new();
                obj.insert("id".to_string(), Value::String(attachment.id.clone()));
                obj.insert(
                    "filename".to_string(),
                    Value::String(attachment.filename.clone()),
                );
                obj.insert(
                    "mime_type".to_string(),
                    Value::String(attachment.mime_type.clone()),
                );
                obj.insert("size_bytes".to_string(), json!(attachment.size_bytes));

                if let Some(description) = &attachment.description {
                    obj.insert(
                        "description".to_string(),
                        Value::String(description.clone()),
                    );
                }

                obj.insert("required".to_string(), Value::Bool(attachment.required));

                Value::Object(obj)
            })
            .collect();

        Value::Array(arr)
    }

    fn parse_status(&self, s: &str) -> Result<ApplicationStatus> {
        match s {
            "Draft" => Ok(ApplicationStatus::Draft),
            "Submitted" => Ok(ApplicationStatus::Submitted),
            "UnderReview" | "Under Review" => Ok(ApplicationStatus::UnderReview),
            "Accepted" => Ok(ApplicationStatus::Accepted),
            "Rejected" => Ok(ApplicationStatus::Rejected),
            "RequiresRevision" | "Requires Revision" => Ok(ApplicationStatus::RequiresRevision),
            "Approved" => Ok(ApplicationStatus::Approved),
            "Withdrawn" => Ok(ApplicationStatus::Withdrawn),
            _ => Err(EgovError::JsonParse(format!(
                "Unknown application status: {}",
                s
            ))),
        }
    }

    fn parse_agency(&self, s: &str) -> Result<GovernmentAgency> {
        match s {
            "Digital Agency" | "DigitalAgency" | "00100" => Ok(GovernmentAgency::DigitalAgency),
            "Ministry of Justice" | "MinistryOfJustice" | "00200" => {
                Ok(GovernmentAgency::MinistryOfJustice)
            }
            "Ministry of Land, Infrastructure, Transport and Tourism"
            | "MinistryOfLand"
            | "00800" => Ok(GovernmentAgency::MinistryOfLand),
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
            _ => Err(EgovError::JsonParse(format!(
                "Unknown government agency: {}",
                s
            ))),
        }
    }
}

impl Default for EgovJsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_json() -> String {
        r#"{
  "metadata": {
    "application_id": "APP-TEST-001",
    "application_type": "test_application",
    "applicant_name": "Test Applicant",
    "status": "Draft",
    "agency": "Digital Agency",
    "created_date": "2026-01-09",
    "updated_date": "2026-01-09"
  },
  "form_data": {
    "test_field": "Test Value",
    "test_number": 42
  },
  "attachments": [
    {
      "id": "ATT-001",
      "filename": "test.pdf",
      "mime_type": "application/pdf",
      "size_bytes": 1000,
      "required": false
    }
  ]
}"#
        .to_string()
    }

    #[test]
    fn test_parse_valid_json() {
        let formatter = EgovJsonFormatter::new();
        let json = create_test_json();
        let result = formatter.parse_application(&json);

        assert!(result.is_ok());
        let app = result.unwrap();
        assert_eq!(app.metadata.application_id, "APP-TEST-001");
        assert_eq!(app.metadata.applicant_name, "Test Applicant");
        assert_eq!(app.metadata.status, ApplicationStatus::Draft);
    }

    #[test]
    fn test_parse_form_data() {
        let formatter = EgovJsonFormatter::new();
        let json = create_test_json();
        let app = formatter.parse_application(&json).unwrap();

        assert_eq!(app.form_data.len(), 2);
        assert!(app.form_data.contains_key("test_field"));
        assert!(app.form_data.contains_key("test_number"));
    }

    #[test]
    fn test_parse_attachments() {
        let formatter = EgovJsonFormatter::new();
        let json = create_test_json();
        let app = formatter.parse_application(&json).unwrap();

        assert_eq!(app.attachments.len(), 1);
        assert_eq!(app.attachments[0].id, "ATT-001");
        assert_eq!(app.attachments[0].filename, "test.pdf");
    }

    #[test]
    fn test_serialize_application() {
        let formatter = EgovJsonFormatter::new();
        let mut app = EgovApplication::new(
            "APP-TEST-002",
            "test_application",
            "Test Applicant",
            GovernmentAgency::DigitalAgency,
        );

        app.add_field("test_field", EgovFieldValue::Text("Test Value".to_string()));
        app.add_field("test_number", EgovFieldValue::Number(42));

        let json = formatter.serialize_application(&app);
        assert!(json.is_ok());

        let json_string = json.unwrap();
        assert!(json_string.contains("APP-TEST-002"));
        assert!(json_string.contains("Test Applicant"));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let formatter = EgovJsonFormatter::new();
        let json = create_test_json();

        // Parse JSON
        let app1 = formatter.parse_application(&json).unwrap();

        // Serialize to JSON
        let json2 = formatter.serialize_application(&app1).unwrap();

        // Parse again
        let app2 = formatter.parse_application(&json2).unwrap();

        // Compare
        assert_eq!(app1.metadata.application_id, app2.metadata.application_id);
        assert_eq!(app1.metadata.applicant_name, app2.metadata.applicant_name);
        assert_eq!(app1.metadata.status, app2.metadata.status);
    }

    #[test]
    fn test_xml_to_json_conversion() {
        let formatter = EgovJsonFormatter::new();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<application>
  <metadata>
    <application_id>APP-TEST-003</application_id>
    <application_type>test</application_type>
    <applicant_name>Test User</applicant_name>
    <status>Draft</status>
    <agency>Digital Agency</agency>
    <created_date>2026-01-09</created_date>
    <updated_date>2026-01-09</updated_date>
  </metadata>
  <form_data></form_data>
  <attachments></attachments>
</application>"#;

        let result = formatter.xml_to_json(xml);
        assert!(result.is_ok());

        let json = result.unwrap();
        assert!(json.contains("APP-TEST-003"));
        assert!(json.contains("Test User"));
    }

    #[test]
    fn test_json_to_xml_conversion() {
        let formatter = EgovJsonFormatter::new();
        let json = create_test_json();

        let result = formatter.json_to_xml(&json);
        assert!(result.is_ok());

        let xml = result.unwrap();
        assert!(xml.contains("<application_id>APP-TEST-001</application_id>"));
        assert!(xml.contains("<applicant_name>Test Applicant</applicant_name>"));
    }

    #[test]
    fn test_pretty_print() {
        let formatter = EgovJsonFormatter::new().with_pretty_print(true);
        let app = EgovApplication::new(
            "APP-TEST-004",
            "test",
            "Test User",
            GovernmentAgency::MinistryOfJustice,
        );

        let json = formatter.serialize_application(&app).unwrap();
        assert!(json.contains('\n')); // Pretty printed JSON has newlines
    }

    #[test]
    fn test_parse_missing_metadata() {
        let formatter = EgovJsonFormatter::new();
        let json = r#"{"form_data": {}, "attachments": []}"#;

        let result = formatter.parse_application(json);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EgovError::MissingRequiredField { .. }
        ));
    }
}
