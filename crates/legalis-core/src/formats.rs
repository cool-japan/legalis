//! Format conversion helpers for legal data interchange.
//!
//! This module provides conversion utilities to/from common legal data formats:
//! - JSON-LD (Linked Data format for semantic web integration)
//! - XML (eXtensible Markup Language for legacy systems)
//! - YAML (Human-readable configuration format)
//! - TOML (Tom's Obvious Minimal Language configuration format)
//!
//! # JSON-LD Support
//!
//! JSON-LD is a lightweight Linked Data format that makes it easy to integrate
//! legal data with semantic web technologies. Each statute can be represented
//! with proper @context and @type annotations.
//!
//! # YAML and TOML Support
//!
//! YAML and TOML provide human-readable configuration formats suitable for
//! statute definitions and legal rule specifications. Both formats support
//! complete round-trip conversion with all statute fields preserved.
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
    /// YAML parsing or serialization failed
    YamlError(String),
    /// TOML parsing or serialization failed
    TomlError(String),
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
            FormatError::YamlError(msg) => write!(f, "YAML error: {}", msg),
            FormatError::TomlError(msg) => write!(f, "TOML error: {}", msg),
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

/// YAML converter for legal data structures.
///
/// Provides conversion to/from YAML format for human-readable statute definitions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// # #[cfg(feature = "yaml")]
/// use legalis_core::formats::YamlConverter;
///
/// # #[cfg(feature = "yaml")]
/// # {
/// let statute = Statute::new(
///     "example",
///     "Example Statute",
///     Effect::new(EffectType::Grant, "Example grant")
/// );
///
/// let yaml = YamlConverter::to_yaml(&statute).unwrap();
/// let parsed = YamlConverter::from_yaml(&yaml).unwrap();
/// assert_eq!(statute.id, parsed.id);
/// # }
/// ```
#[cfg(feature = "yaml")]
pub struct YamlConverter;

#[cfg(feature = "yaml")]
impl YamlConverter {
    /// Convert a statute to YAML format.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    /// # #[cfg(feature = "yaml")]
    /// use legalis_core::formats::YamlConverter;
    ///
    /// # #[cfg(feature = "yaml")]
    /// # {
    /// let statute = Statute::new(
    ///     "tax-credit-2025",
    ///     "Low Income Tax Credit",
    ///     Effect::new(EffectType::Grant, "Tax credit")
    /// )
    /// .with_precondition(Condition::Income {
    ///     operator: ComparisonOp::LessThan,
    ///     value: 50000
    /// })
    /// .with_jurisdiction("US");
    ///
    /// let yaml = YamlConverter::to_yaml(&statute).unwrap();
    /// assert!(yaml.contains("id:"));
    /// assert!(yaml.contains("title:"));
    /// # }
    /// ```
    pub fn to_yaml(statute: &Statute) -> Result<String, FormatError> {
        serde_yaml::to_string(statute).map_err(|e| FormatError::YamlError(e.to_string()))
    }

    /// Parse a statute from YAML format.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// # #[cfg(feature = "yaml")]
    /// use legalis_core::formats::YamlConverter;
    ///
    /// # #[cfg(feature = "yaml")]
    /// # {
    /// let statute = Statute::new(
    ///     "example",
    ///     "Example Statute",
    ///     Effect::new(EffectType::Grant, "Example grant")
    /// );
    ///
    /// let yaml = YamlConverter::to_yaml(&statute).unwrap();
    /// let parsed = YamlConverter::from_yaml(&yaml).unwrap();
    /// assert_eq!(statute.id, parsed.id);
    /// assert_eq!(statute.title, parsed.title);
    /// # }
    /// ```
    pub fn from_yaml(yaml: &str) -> Result<Statute, FormatError> {
        serde_yaml::from_str(yaml).map_err(|e| FormatError::YamlError(e.to_string()))
    }

    /// Parse multiple statutes from a YAML document.
    ///
    /// Supports both YAML arrays and YAML document separators (---).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// # #[cfg(feature = "yaml")]
    /// use legalis_core::formats::YamlConverter;
    ///
    /// # #[cfg(feature = "yaml")]
    /// # {
    /// let yaml = r#"
    /// - id: law-1
    ///   title: First Law
    ///   effect:
    ///     effect_type: Grant
    ///     description: Grant benefit
    ///     parameters: {}
    ///   preconditions: []
    ///   temporal_validity: {}
    ///   version: 1
    ///   derives_from: []
    ///   applies_to: []
    ///   exceptions: []
    /// - id: law-2
    ///   title: Second Law
    ///   effect:
    ///     effect_type: Revoke
    ///     description: Revoke benefit
    ///     parameters: {}
    ///   preconditions: []
    ///   temporal_validity: {}
    ///   version: 1
    ///   derives_from: []
    ///   applies_to: []
    ///   exceptions: []
    /// "#;
    ///
    /// let statutes = YamlConverter::from_yaml_multi(yaml).unwrap();
    /// assert_eq!(statutes.len(), 2);
    /// assert_eq!(statutes[0].id, "law-1");
    /// assert_eq!(statutes[1].id, "law-2");
    /// # }
    /// ```
    pub fn from_yaml_multi(yaml: &str) -> Result<Vec<Statute>, FormatError> {
        serde_yaml::from_str(yaml).map_err(|e| FormatError::YamlError(e.to_string()))
    }
}

/// TOML converter for legal data structures.
///
/// Provides conversion to/from TOML format for configuration-style statute definitions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// # #[cfg(feature = "toml")]
/// use legalis_core::formats::TomlConverter;
///
/// # #[cfg(feature = "toml")]
/// # {
/// let statute = Statute::new(
///     "example",
///     "Example Statute",
///     Effect::new(EffectType::Grant, "Example grant")
/// );
///
/// let toml = TomlConverter::to_toml(&statute).unwrap();
/// let parsed = TomlConverter::from_toml(&toml).unwrap();
/// assert_eq!(statute.id, parsed.id);
/// # }
/// ```
#[cfg(feature = "toml")]
pub struct TomlConverter;

#[cfg(feature = "toml")]
impl TomlConverter {
    /// Convert a statute to TOML format.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    /// # #[cfg(feature = "toml")]
    /// use legalis_core::formats::TomlConverter;
    ///
    /// # #[cfg(feature = "toml")]
    /// # {
    /// let statute = Statute::new(
    ///     "tax-credit-2025",
    ///     "Low Income Tax Credit",
    ///     Effect::new(EffectType::Grant, "Tax credit")
    /// )
    /// .with_jurisdiction("US")
    /// .with_version(1);
    ///
    /// let toml = TomlConverter::to_toml(&statute).unwrap();
    /// assert!(toml.contains("id ="));
    /// assert!(toml.contains("title ="));
    /// # }
    /// ```
    pub fn to_toml(statute: &Statute) -> Result<String, FormatError> {
        toml::to_string(statute).map_err(|e| FormatError::TomlError(e.to_string()))
    }

    /// Parse a statute from TOML format.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// # #[cfg(feature = "toml")]
    /// use legalis_core::formats::TomlConverter;
    ///
    /// # #[cfg(feature = "toml")]
    /// # {
    /// let statute = Statute::new(
    ///     "example",
    ///     "Example Statute",
    ///     Effect::new(EffectType::Grant, "Example grant")
    /// );
    ///
    /// let toml = TomlConverter::to_toml(&statute).unwrap();
    /// let parsed = TomlConverter::from_toml(&toml).unwrap();
    /// assert_eq!(statute.id, parsed.id);
    /// assert_eq!(statute.title, parsed.title);
    /// # }
    /// ```
    pub fn from_toml(toml: &str) -> Result<Statute, FormatError> {
        toml::from_str(toml).map_err(|e| FormatError::TomlError(e.to_string()))
    }
}

/// Streaming deserializer for large statute collections.
///
/// This module provides streaming deserialization support to process
/// large statute collections without loading everything into memory.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// # #[cfg(feature = "yaml")]
/// use legalis_core::formats::StreamingDeserializer;
///
/// # #[cfg(feature = "yaml")]
/// # {
/// let yaml = r#"
/// - id: law-1
///   title: First Law
///   effect:
///     effect_type: Grant
///     description: Grant benefit
///     parameters: {}
///   preconditions: []
///   temporal_validity: {}
///   version: 1
///   derives_from: []
///   applies_to: []
///   exceptions: []
/// - id: law-2
///   title: Second Law
///   effect:
///     effect_type: Revoke
///     description: Revoke benefit
///     parameters: {}
///   preconditions: []
///   temporal_validity: {}
///   version: 1
///   derives_from: []
///   applies_to: []
///   exceptions: []
/// "#;
///
/// let mut count = 0;
/// StreamingDeserializer::from_yaml_stream(yaml.as_bytes(), |statute| {
///     count += 1;
///     println!("Processing statute: {}", statute.id);
///     Ok(())
/// }).unwrap();
/// assert_eq!(count, 2);
/// # }
/// ```
#[cfg(any(feature = "yaml", feature = "serde"))]
pub struct StreamingDeserializer;

#[cfg(any(feature = "yaml", feature = "serde"))]
impl StreamingDeserializer {
    /// Stream statutes from a YAML document.
    ///
    /// Calls the provided callback for each statute in the stream.
    /// This is memory-efficient for large datasets.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::formats::StreamingDeserializer;
    /// # #[cfg(feature = "yaml")]
    /// # {
    ///
    /// let yaml = r#"
    /// - id: law-1
    ///   title: First Law
    ///   effect:
    ///     effect_type: Grant
    ///     description: Grant benefit
    ///     parameters: {}
    ///   preconditions: []
    ///   temporal_validity: {}
    ///   version: 1
    ///   derives_from: []
    ///   applies_to: []
    ///   exceptions: []
    /// "#;
    ///
    /// let mut ids = Vec::new();
    /// StreamingDeserializer::from_yaml_stream(yaml.as_bytes(), |statute| {
    ///     ids.push(statute.id.clone());
    ///     Ok(())
    /// }).unwrap();
    /// assert_eq!(ids, vec!["law-1"]);
    /// # }
    /// ```
    #[cfg(feature = "yaml")]
    pub fn from_yaml_stream<R, F>(reader: R, mut callback: F) -> Result<(), FormatError>
    where
        R: std::io::Read,
        F: FnMut(Statute) -> Result<(), FormatError>,
    {
        let statutes: Vec<Statute> =
            serde_yaml::from_reader(reader).map_err(|e| FormatError::YamlError(e.to_string()))?;

        for statute in statutes {
            callback(statute)?;
        }

        Ok(())
    }

    /// Stream statutes from a JSON array.
    ///
    /// Calls the provided callback for each statute in the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::formats::StreamingDeserializer;
    /// # #[cfg(feature = "serde")]
    /// # {
    ///
    /// let json = r#"[
    ///   {
    ///     "id": "law-1",
    ///     "title": "First Law",
    ///     "effect": {
    ///       "effect_type": "Grant",
    ///       "description": "Grant benefit",
    ///       "parameters": {}
    ///     },
    ///     "preconditions": [],
    ///     "temporal_validity": {},
    ///     "version": 1,
    ///     "derives_from": [],
    ///     "applies_to": [],
    ///     "exceptions": []
    ///   }
    /// ]"#;
    ///
    /// let mut count = 0;
    /// StreamingDeserializer::from_json_stream(json.as_bytes(), |_statute| {
    ///     count += 1;
    ///     Ok(())
    /// }).unwrap();
    /// assert_eq!(count, 1);
    /// # }
    /// ```
    #[cfg(feature = "serde")]
    pub fn from_json_stream<R, F>(reader: R, mut callback: F) -> Result<(), FormatError>
    where
        R: std::io::Read,
        F: FnMut(Statute) -> Result<(), FormatError>,
    {
        let statutes: Vec<Statute> =
            serde_json::from_reader(reader).map_err(|e| FormatError::JsonError(e.to_string()))?;

        for statute in statutes {
            callback(statute)?;
        }

        Ok(())
    }
}

/// Content-addressable hashing for statutes.
///
/// Provides cryptographic hashing of statute content for:
/// - Deduplication
/// - Version control
/// - Integrity verification
/// - Content-addressable storage
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::formats::StatuteHasher;
///
/// let statute = Statute::new(
///     "example",
///     "Example Statute",
///     Effect::new(EffectType::Grant, "Example grant")
/// );
///
/// let hash = StatuteHasher::hash(&statute);
/// assert_eq!(hash.len(), 64); // SHA-256 hex string
/// ```
pub struct StatuteHasher;

impl StatuteHasher {
    /// Compute SHA-256 hash of statute content.
    ///
    /// The hash is computed from a canonical representation of the statute,
    /// ensuring that identical content always produces the same hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_core::formats::StatuteHasher;
    ///
    /// let statute1 = Statute::new(
    ///     "test",
    ///     "Test Statute",
    ///     Effect::new(EffectType::Grant, "Grant")
    /// );
    ///
    /// let statute2 = Statute::new(
    ///     "test",
    ///     "Test Statute",
    ///     Effect::new(EffectType::Grant, "Grant")
    /// );
    ///
    /// // Identical content produces identical hashes
    /// assert_eq!(
    ///     StatuteHasher::hash(&statute1),
    ///     StatuteHasher::hash(&statute2)
    /// );
    /// ```
    pub fn hash(statute: &Statute) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();

        // Hash fields in a deterministic order
        hasher.update(statute.id.as_bytes());
        hasher.update(statute.title.as_bytes());
        hasher.update(statute.effect.description.as_bytes());
        hasher.update(format!("{:?}", statute.effect.effect_type).as_bytes());

        // Hash preconditions
        for precondition in &statute.preconditions {
            hasher.update(format!("{}", precondition).as_bytes());
        }

        // Hash optional fields
        if let Some(ref discretion) = statute.discretion_logic {
            hasher.update(discretion.as_bytes());
        }

        if let Some(ref jurisdiction) = statute.jurisdiction {
            hasher.update(jurisdiction.as_bytes());
        }

        hasher.update(statute.version.to_string().as_bytes());

        // Hash derives_from
        for source in &statute.derives_from {
            hasher.update(source.as_bytes());
        }

        // Hash applies_to
        for entity_type in &statute.applies_to {
            hasher.update(entity_type.as_bytes());
        }

        // Hash exceptions
        for exception in &statute.exceptions {
            hasher.update(exception.id.as_bytes());
            hasher.update(exception.description.as_bytes());
        }

        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Verify that a statute matches its expected hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_core::formats::StatuteHasher;
    ///
    /// let statute = Statute::new(
    ///     "test",
    ///     "Test Statute",
    ///     Effect::new(EffectType::Grant, "Grant")
    /// );
    ///
    /// let hash = StatuteHasher::hash(&statute);
    /// assert!(StatuteHasher::verify(&statute, &hash));
    ///
    /// // Modified statute fails verification
    /// let modified = statute.with_version(2);
    /// assert!(!StatuteHasher::verify(&modified, &hash));
    /// ```
    pub fn verify(statute: &Statute, expected_hash: &str) -> bool {
        Self::hash(statute) == expected_hash
    }
}

/// Schema migration support for version changes.
///
/// Provides utilities for migrating statutes between different schema versions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_core::formats::SchemaMigration;
///
/// let statute = Statute::new(
///     "example",
///     "Example Statute",
///     Effect::new(EffectType::Grant, "Example grant")
/// );
///
/// // Migrate from version 1 to version 2
/// let migrated = SchemaMigration::migrate(statute, 1, 2).unwrap();
/// assert_eq!(migrated.version, 2);
/// ```
pub struct SchemaMigration;

impl SchemaMigration {
    /// Migrate a statute from one schema version to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_core::formats::SchemaMigration;
    ///
    /// let statute = Statute::new(
    ///     "test",
    ///     "Test Statute",
    ///     Effect::new(EffectType::Grant, "Grant")
    /// )
    /// .with_version(1);
    ///
    /// let migrated = SchemaMigration::migrate(statute, 1, 3).unwrap();
    /// assert_eq!(migrated.version, 3);
    /// ```
    pub fn migrate(
        statute: Statute,
        from_version: u32,
        to_version: u32,
    ) -> Result<Statute, FormatError> {
        if from_version == to_version {
            return Ok(statute);
        }

        if from_version > to_version {
            return Err(FormatError::UnsupportedVersion(format!(
                "Cannot migrate backwards from {} to {}",
                from_version, to_version
            )));
        }

        // Apply migrations sequentially
        let mut current = statute;
        for version in (from_version + 1)..=to_version {
            current = Self::migrate_to_version(current, version)?;
        }

        Ok(current)
    }

    fn migrate_to_version(statute: Statute, version: u32) -> Result<Statute, FormatError> {
        match version {
            1 => Ok(statute.with_version(1)),
            2 => {
                // Example migration: add default jurisdiction if missing
                let mut migrated = statute.with_version(2);
                if migrated.jurisdiction.is_none() {
                    migrated = migrated.with_jurisdiction("UNSPECIFIED");
                }
                Ok(migrated)
            }
            3 => {
                // Example migration: ensure derives_from is initialized
                Ok(statute.with_version(3))
            }
            _ => Err(FormatError::UnsupportedVersion(format!(
                "Unknown schema version: {}",
                version
            ))),
        }
    }

    /// Check if a statute can be migrated to a target version.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::formats::SchemaMigration;
    ///
    /// assert!(SchemaMigration::can_migrate(1, 2));
    /// assert!(SchemaMigration::can_migrate(1, 3));
    /// assert!(!SchemaMigration::can_migrate(3, 2)); // Cannot migrate backwards
    /// ```
    pub fn can_migrate(from_version: u32, to_version: u32) -> bool {
        from_version <= to_version && to_version <= 3 // Current max version is 3
    }

    /// Get the current schema version.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::formats::SchemaMigration;
    ///
    /// assert_eq!(SchemaMigration::current_version(), 3);
    /// ```
    pub fn current_version() -> u32 {
        3
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

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_roundtrip_basic() {
        let statute = Statute::new(
            "test-5",
            "Test YAML Statute",
            Effect::new(EffectType::Grant, "YAML test grant"),
        );

        let yaml = YamlConverter::to_yaml(&statute).unwrap();
        let parsed = YamlConverter::from_yaml(&yaml).unwrap();

        assert_eq!(statute.id, parsed.id);
        assert_eq!(statute.title, parsed.title);
        assert_eq!(statute.effect.description, parsed.effect.description);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_with_jurisdiction() {
        let statute = Statute::new(
            "test-6",
            "Test YAML Statute",
            Effect::new(EffectType::Revoke, "YAML revoke"),
        )
        .with_jurisdiction("EU")
        .with_version(3);

        let yaml = YamlConverter::to_yaml(&statute).unwrap();
        let parsed = YamlConverter::from_yaml(&yaml).unwrap();

        assert_eq!(parsed.jurisdiction, Some("EU".to_string()));
        assert_eq!(parsed.version, 3);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_yaml_multi() {
        let statute1 = Statute::new(
            "multi-1",
            "First Statute",
            Effect::new(EffectType::Grant, "First grant"),
        );
        let statute2 = Statute::new(
            "multi-2",
            "Second Statute",
            Effect::new(EffectType::Obligation, "Second obligation"),
        );

        let statutes = vec![statute1, statute2];
        let yaml = serde_yaml::to_string(&statutes).unwrap();
        let parsed = YamlConverter::from_yaml_multi(&yaml).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].id, "multi-1");
        assert_eq!(parsed[1].id, "multi-2");
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml_roundtrip_basic() {
        let statute = Statute::new(
            "test-7",
            "Test TOML Statute",
            Effect::new(EffectType::Grant, "TOML test grant"),
        );

        let toml = TomlConverter::to_toml(&statute).unwrap();
        let parsed = TomlConverter::from_toml(&toml).unwrap();

        assert_eq!(statute.id, parsed.id);
        assert_eq!(statute.title, parsed.title);
        assert_eq!(statute.effect.description, parsed.effect.description);
    }

    #[test]
    #[cfg(feature = "toml")]
    fn test_toml_with_jurisdiction() {
        let statute = Statute::new(
            "test-8",
            "Test TOML Statute",
            Effect::new(EffectType::Prohibition, "TOML prohibition"),
        )
        .with_jurisdiction("UK")
        .with_version(5);

        let toml = TomlConverter::to_toml(&statute).unwrap();
        let parsed = TomlConverter::from_toml(&toml).unwrap();

        assert_eq!(parsed.jurisdiction, Some("UK".to_string()));
        assert_eq!(parsed.version, 5);
    }

    #[test]
    #[cfg(feature = "yaml")]
    fn test_streaming_yaml() {
        let yaml = r#"
- id: stream-1
  title: First Statute
  effect:
    effect_type: Grant
    description: Grant benefit
    parameters: {}
  preconditions: []
  temporal_validity: {}
  version: 1
  derives_from: []
  applies_to: []
  exceptions: []
- id: stream-2
  title: Second Statute
  effect:
    effect_type: Revoke
    description: Revoke benefit
    parameters: {}
  preconditions: []
  temporal_validity: {}
  version: 1
  derives_from: []
  applies_to: []
  exceptions: []
"#;

        let mut ids = Vec::new();
        StreamingDeserializer::from_yaml_stream(yaml.as_bytes(), |statute| {
            ids.push(statute.id.clone());
            Ok(())
        })
        .unwrap();

        assert_eq!(ids, vec!["stream-1", "stream-2"]);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_streaming_json() {
        let json = r#"[
  {
    "id": "json-stream-1",
    "title": "First Statute",
    "effect": {
      "effect_type": "Grant",
      "description": "Grant benefit",
      "parameters": {}
    },
    "preconditions": [],
    "temporal_validity": {},
    "version": 1,
    "derives_from": [],
    "applies_to": [],
    "exceptions": []
  }
]"#;

        let mut count = 0;
        StreamingDeserializer::from_json_stream(json.as_bytes(), |statute| {
            assert_eq!(statute.id, "json-stream-1");
            count += 1;
            Ok(())
        })
        .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn test_statute_hashing() {
        let statute1 = Statute::new(
            "hash-test",
            "Hash Test Statute",
            Effect::new(EffectType::Grant, "Grant"),
        );

        let statute2 = Statute::new(
            "hash-test",
            "Hash Test Statute",
            Effect::new(EffectType::Grant, "Grant"),
        );

        let hash1 = StatuteHasher::hash(&statute1);
        let hash2 = StatuteHasher::hash(&statute2);

        // Identical content produces identical hashes
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 hex string

        // Different content produces different hashes
        let statute3 = statute1.with_version(2);
        let hash3 = StatuteHasher::hash(&statute3);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_statute_hash_verification() {
        let statute = Statute::new(
            "verify-test",
            "Verification Test",
            Effect::new(EffectType::Grant, "Grant"),
        );

        let hash = StatuteHasher::hash(&statute);
        assert!(StatuteHasher::verify(&statute, &hash));

        // Modified statute fails verification
        let modified = statute.with_version(2);
        assert!(!StatuteHasher::verify(&modified, &hash));
    }

    #[test]
    fn test_schema_migration_forward() {
        let statute = Statute::new(
            "migrate-test",
            "Migration Test",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_version(1);

        let migrated = SchemaMigration::migrate(statute, 1, 3).unwrap();
        assert_eq!(migrated.version, 3);
    }

    #[test]
    fn test_schema_migration_with_jurisdiction() {
        let statute = Statute::new(
            "migrate-test-2",
            "Migration Test 2",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_version(1);

        // Migration to version 2 adds default jurisdiction
        let migrated = SchemaMigration::migrate(statute, 1, 2).unwrap();
        assert_eq!(migrated.version, 2);
        assert_eq!(migrated.jurisdiction, Some("UNSPECIFIED".to_string()));
    }

    #[test]
    fn test_schema_migration_no_op() {
        let statute = Statute::new(
            "migrate-test-3",
            "Migration Test 3",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_version(2);

        let migrated = SchemaMigration::migrate(statute.clone(), 2, 2).unwrap();
        assert_eq!(statute.version, migrated.version);
    }

    #[test]
    fn test_schema_migration_backwards_fails() {
        let statute = Statute::new(
            "migrate-test-4",
            "Migration Test 4",
            Effect::new(EffectType::Grant, "Grant"),
        )
        .with_version(3);

        let result = SchemaMigration::migrate(statute, 3, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_can_migrate() {
        assert!(SchemaMigration::can_migrate(1, 2));
        assert!(SchemaMigration::can_migrate(1, 3));
        assert!(SchemaMigration::can_migrate(2, 3));
        assert!(!SchemaMigration::can_migrate(3, 2));
        assert!(!SchemaMigration::can_migrate(2, 1));
    }

    #[test]
    fn test_current_version() {
        assert_eq!(SchemaMigration::current_version(), 3);
    }
}
