//! Format conversion helpers for legal data interchange.
//!
//! This module provides conversion utilities to/from common legal data formats:
//! - JSON-LD (Linked Data format for semantic web integration)
//! - XML (eXtensible Markup Language for legacy systems)
//!
//! # JSON-LD Support
//!
//! JSON-LD is a lightweight Linked Data format that makes it easy to integrate
//! legal data with semantic web technologies. Each statute can be represented
//! with proper @context and @type annotations.
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_core::formats::JsonLdConverter;
//!
//! let statute = Statute::new(
//!     "tax-credit-2025",
//!     "Low Income Tax Credit",
//!     Effect::new(EffectType::Grant, "Tax credit of $1000")
//! )
//! .with_precondition(Condition::Income {
//!     operator: ComparisonOp::LessThan,
//!     value: 50000
//! })
//! .with_jurisdiction("US");
//!
//! let json_ld = JsonLdConverter::to_json_ld(&statute).unwrap();
//! println!("{}", json_ld);
//!
//! // Round-trip conversion
//! let parsed = JsonLdConverter::from_json_ld(&json_ld).unwrap();
//! assert_eq!(statute.id, parsed.id);
//! ```

#[cfg(feature = "serde")]
use serde_json::{Value, json};

use crate::{Condition, Effect, EffectType, Statute, TemporalValidity};

/// Errors that can occur during format conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    /// JSON parsing or serialization failed
    JsonError(String),
    /// XML parsing or serialization failed
    XmlError(String),
    /// Required field is missing
    MissingField(String),
    /// Invalid value for a field
    InvalidValue(String),
    /// Unsupported format version
    UnsupportedVersion(String),
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            FormatError::XmlError(msg) => write!(f, "XML error: {}", msg),
            FormatError::MissingField(field) => write!(f, "Missing required field: {}", field),
            FormatError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            FormatError::UnsupportedVersion(ver) => write!(f, "Unsupported version: {}", ver),
        }
    }
}

impl std::error::Error for FormatError {}

/// JSON-LD converter for legal data structures.
///
/// Provides conversion to/from JSON-LD format with proper semantic annotations.
#[cfg(feature = "serde")]
pub struct JsonLdConverter;

#[cfg(feature = "serde")]
impl JsonLdConverter {
    /// Convert a statute to JSON-LD format.
    ///
    /// The resulting JSON-LD includes:
    /// - @context with legal vocabulary URIs
    /// - @type indicating this is a legal statute
    /// - All statute fields with semantic annotations
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_core::formats::JsonLdConverter;
    ///
    /// let statute = Statute::new(
    ///     "example",
    ///     "Example Statute",
    ///     Effect::new(EffectType::Grant, "Example grant")
    /// );
    ///
    /// let json_ld = JsonLdConverter::to_json_ld(&statute).unwrap();
    /// assert!(json_ld.contains("@context"));
    /// assert!(json_ld.contains("@type"));
    /// ```
    pub fn to_json_ld(statute: &Statute) -> Result<String, FormatError> {
        let mut obj = json!({
            "@context": {
                "@vocab": "http://schema.org/",
                "legalis": "http://legalis.org/vocab#",
                "statute": "legalis:Statute",
                "effect": "legalis:Effect",
                "condition": "legalis:Condition"
            },
            "@type": "statute",
            "legalis:id": statute.id,
            "legalis:title": statute.title,
            "legalis:effect": {
                "@type": "effect",
                "legalis:effectType": format!("{:?}", statute.effect.effect_type),
                "legalis:description": statute.effect.description
            }
        });

        let map = obj.as_object_mut().unwrap();

        // Add optional fields
        if !statute.preconditions.is_empty() {
            let conditions: Vec<Value> = statute
                .preconditions
                .iter()
                .map(Self::condition_to_json)
                .collect();
            map.insert("legalis:preconditions".to_string(), json!(conditions));
        }

        if let Some(ref discretion_logic) = statute.discretion_logic {
            map.insert("legalis:discretion".to_string(), json!(discretion_logic));
        }

        if let Some(ref jurisdiction) = statute.jurisdiction {
            map.insert("legalis:jurisdiction".to_string(), json!(jurisdiction));
        }

        if statute.version > 0 {
            map.insert("legalis:version".to_string(), json!(statute.version));
        }

        map.insert(
            "legalis:temporalValidity".to_string(),
            Self::temporal_to_json(&statute.temporal_validity),
        );

        serde_json::to_string_pretty(&obj).map_err(|e| FormatError::JsonError(e.to_string()))
    }

    /// Parse a statute from JSON-LD format.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::formats::JsonLdConverter;
    ///
    /// let json_ld = r#"{
    ///     "@context": {"legalis": "http://legalis.org/vocab#"},
    ///     "@type": "statute",
    ///     "legalis:id": "example",
    ///     "legalis:title": "Example",
    ///     "legalis:effect": {
    ///         "legalis:effectType": "Grant",
    ///         "legalis:description": "Example grant"
    ///     }
    /// }"#;
    ///
    /// let statute = JsonLdConverter::from_json_ld(json_ld).unwrap();
    /// assert_eq!(statute.id, "example");
    /// ```
    pub fn from_json_ld(json_ld: &str) -> Result<Statute, FormatError> {
        let value: Value =
            serde_json::from_str(json_ld).map_err(|e| FormatError::JsonError(e.to_string()))?;

        let obj = value
            .as_object()
            .ok_or_else(|| FormatError::InvalidValue("Expected object".to_string()))?;

        // Extract required fields
        let id = obj
            .get("legalis:id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FormatError::MissingField("legalis:id".to_string()))?;

        let title = obj
            .get("legalis:title")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FormatError::MissingField("legalis:title".to_string()))?;

        let effect_obj = obj
            .get("legalis:effect")
            .and_then(|v| v.as_object())
            .ok_or_else(|| FormatError::MissingField("legalis:effect".to_string()))?;

        let effect_type_str = effect_obj
            .get("legalis:effectType")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FormatError::MissingField("legalis:effectType".to_string()))?;

        let effect_description = effect_obj
            .get("legalis:description")
            .and_then(|v| v.as_str())
            .ok_or_else(|| FormatError::MissingField("legalis:description".to_string()))?;

        let effect_type = match effect_type_str {
            "Grant" => EffectType::Grant,
            "Revoke" => EffectType::Revoke,
            "Obligation" => EffectType::Obligation,
            "Prohibition" => EffectType::Prohibition,
            "MonetaryTransfer" => EffectType::MonetaryTransfer,
            "StatusChange" => EffectType::StatusChange,
            "Custom" => EffectType::Custom,
            _ => {
                return Err(FormatError::InvalidValue(format!(
                    "Unknown effect type: {}",
                    effect_type_str
                )));
            }
        };

        let effect = Effect::new(effect_type, effect_description);
        let mut statute = Statute::new(id, title, effect);

        // Add optional fields
        if let Some(jurisdiction) = obj.get("legalis:jurisdiction").and_then(|v| v.as_str()) {
            statute = statute.with_jurisdiction(jurisdiction);
        }

        if let Some(version) = obj.get("legalis:version").and_then(|v| v.as_u64()) {
            statute = statute.with_version(version as u32);
        }

        if let Some(discretion) = obj.get("legalis:discretion").and_then(|v| v.as_str()) {
            statute = statute.with_discretion(discretion);
        }

        Ok(statute)
    }

    fn condition_to_json(condition: &Condition) -> Value {
        match condition {
            Condition::Age { operator, value } => json!({
                "@type": "condition",
                "legalis:conditionType": "Age",
                "legalis:operator": format!("{:?}", operator),
                "legalis:value": value
            }),
            Condition::Income { operator, value } => json!({
                "@type": "condition",
                "legalis:conditionType": "Income",
                "legalis:operator": format!("{:?}", operator),
                "legalis:value": value
            }),
            Condition::And(left, right) => json!({
                "@type": "condition",
                "legalis:conditionType": "And",
                "legalis:left": Self::condition_to_json(left),
                "legalis:right": Self::condition_to_json(right)
            }),
            Condition::Or(left, right) => json!({
                "@type": "condition",
                "legalis:conditionType": "Or",
                "legalis:left": Self::condition_to_json(left),
                "legalis:right": Self::condition_to_json(right)
            }),
            Condition::Not(inner) => json!({
                "@type": "condition",
                "legalis:conditionType": "Not",
                "legalis:inner": Self::condition_to_json(inner)
            }),
            _ => json!({
                "@type": "condition",
                "legalis:conditionType": "Custom",
                "legalis:description": format!("{}", condition)
            }),
        }
    }

    fn temporal_to_json(temporal: &TemporalValidity) -> Value {
        let mut obj = json!({});
        let map = obj.as_object_mut().unwrap();

        if let Some(ref effective) = temporal.effective_date {
            map.insert(
                "legalis:effectiveDate".to_string(),
                json!(effective.to_string()),
            );
        }

        if let Some(ref expiry) = temporal.expiry_date {
            map.insert("legalis:expiryDate".to_string(), json!(expiry.to_string()));
        }

        if let Some(ref enacted) = temporal.enacted_at {
            map.insert("legalis:enactedAt".to_string(), json!(enacted.to_rfc3339()));
        }

        if let Some(ref amended) = temporal.amended_at {
            map.insert("legalis:amendedAt".to_string(), json!(amended.to_rfc3339()));
        }

        obj
    }
}

/// XML converter for legal data structures.
///
/// Provides conversion to/from XML format for interoperability with
/// legacy legal information systems.
pub struct XmlConverter;

impl XmlConverter {
    /// Convert a statute to XML format.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_core::formats::XmlConverter;
    ///
    /// let statute = Statute::new(
    ///     "example",
    ///     "Example Statute",
    ///     Effect::new(EffectType::Grant, "Example grant")
    /// );
    ///
    /// let xml = XmlConverter::to_xml(&statute).unwrap();
    /// assert!(xml.contains("<statute"));
    /// assert!(xml.contains("</statute>"));
    /// ```
    pub fn to_xml(statute: &Statute) -> Result<String, FormatError> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<statute");

        if let Some(ref jurisdiction) = statute.jurisdiction {
            xml.push_str(&format!(
                " jurisdiction=\"{}\"",
                Self::escape_xml(jurisdiction)
            ));
        }

        if statute.version > 0 {
            xml.push_str(&format!(" version=\"{}\"", statute.version));
        }

        xml.push_str(">\n");

        // ID and Title
        xml.push_str(&format!("  <id>{}</id>\n", Self::escape_xml(&statute.id)));
        xml.push_str(&format!(
            "  <title>{}</title>\n",
            Self::escape_xml(&statute.title)
        ));

        // Effect
        xml.push_str("  <effect>\n");
        xml.push_str(&format!(
            "    <type>{:?}</type>\n",
            statute.effect.effect_type
        ));
        xml.push_str(&format!(
            "    <description>{}</description>\n",
            Self::escape_xml(&statute.effect.description)
        ));

        if !statute.effect.parameters.is_empty() {
            xml.push_str("    <parameters>\n");
            for (key, value) in &statute.effect.parameters {
                xml.push_str(&format!(
                    "      <parameter name=\"{}\">{}</parameter>\n",
                    Self::escape_xml(key),
                    Self::escape_xml(value)
                ));
            }
            xml.push_str("    </parameters>\n");
        }

        xml.push_str("  </effect>\n");

        // Preconditions
        if !statute.preconditions.is_empty() {
            xml.push_str("  <preconditions>\n");
            for condition in &statute.preconditions {
                xml.push_str(&Self::condition_to_xml(condition, 4));
            }
            xml.push_str("  </preconditions>\n");
        }

        // Discretion
        if let Some(ref discretion_logic) = statute.discretion_logic {
            xml.push_str(&format!(
                "  <discretion>{}</discretion>\n",
                Self::escape_xml(discretion_logic)
            ));
        }

        // Temporal Validity
        let temporal = &statute.temporal_validity;
        xml.push_str("  <temporalValidity>\n");

        if let Some(ref effective) = temporal.effective_date {
            xml.push_str(&format!(
                "    <effectiveDate>{}</effectiveDate>\n",
                effective
            ));
        }

        if let Some(ref expiry) = temporal.expiry_date {
            xml.push_str(&format!("    <expiryDate>{}</expiryDate>\n", expiry));
        }

        if let Some(ref enacted) = temporal.enacted_at {
            xml.push_str(&format!(
                "    <enactedAt>{}</enactedAt>\n",
                enacted.to_rfc3339()
            ));
        }

        if let Some(ref amended) = temporal.amended_at {
            xml.push_str(&format!(
                "    <amendedAt>{}</amendedAt>\n",
                amended.to_rfc3339()
            ));
        }

        xml.push_str("  </temporalValidity>\n");

        xml.push_str("</statute>");

        Ok(xml)
    }

    fn condition_to_xml(condition: &Condition, indent: usize) -> String {
        let spaces = " ".repeat(indent);
        match condition {
            Condition::Age { operator, value } => {
                format!(
                    "{}<condition type=\"Age\" operator=\"{:?}\" value=\"{}\"/>\n",
                    spaces, operator, value
                )
            }
            Condition::Income { operator, value } => {
                format!(
                    "{}<condition type=\"Income\" operator=\"{:?}\" value=\"{}\"/>\n",
                    spaces, operator, value
                )
            }
            _ => {
                format!(
                    "{}<condition type=\"Custom\">{}</condition>\n",
                    spaces,
                    Self::escape_xml(&format!("{}", condition))
                )
            }
        }
    }

    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    /// Parse a statute from XML format (basic implementation).
    ///
    /// Note: This is a simplified parser. For production use, consider using
    /// a proper XML parsing library like `quick-xml` or `roxmltree`.
    pub fn from_xml(_xml: &str) -> Result<Statute, FormatError> {
        // Simplified implementation - would need proper XML parser for full support
        Err(FormatError::XmlError("XML parsing not yet fully implemented. Use a proper XML library like quick-xml for production.".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn test_json_ld_roundtrip_basic() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test grant"),
        );

        let json_ld = JsonLdConverter::to_json_ld(&statute).unwrap();
        let parsed = JsonLdConverter::from_json_ld(&json_ld).unwrap();

        assert_eq!(statute.id, parsed.id);
        assert_eq!(statute.title, parsed.title);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_json_ld_with_jurisdiction() {
        let statute = Statute::new(
            "test-2",
            "Test Statute",
            Effect::new(EffectType::Prohibition, "Test prohibition"),
        )
        .with_jurisdiction("US-CA")
        .with_version(2);

        let json_ld = JsonLdConverter::to_json_ld(&statute).unwrap();
        let parsed = JsonLdConverter::from_json_ld(&json_ld).unwrap();

        assert_eq!(parsed.jurisdiction, Some("US-CA".to_string()));
        assert_eq!(parsed.version, 2);
    }

    #[test]
    fn test_xml_basic() {
        let statute = Statute::new(
            "test-3",
            "Test Statute",
            Effect::new(EffectType::Obligation, "Test obligation"),
        );

        let xml = XmlConverter::to_xml(&statute).unwrap();
        assert!(xml.contains("<statute"));
        assert!(xml.contains("<id>test-3</id>"));
        assert!(xml.contains("<title>Test Statute</title>"));
        assert!(xml.contains("</statute>"));
    }

    #[test]
    fn test_xml_escape() {
        let statute = Statute::new(
            "test-4",
            "Test <Statute> & \"Quote\"",
            Effect::new(EffectType::Grant, "Test 'grant'"),
        );

        let xml = XmlConverter::to_xml(&statute).unwrap();
        assert!(xml.contains("&lt;Statute&gt;"));
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&quot;"));
        assert!(xml.contains("&apos;"));
    }
}
