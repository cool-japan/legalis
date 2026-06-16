//! CEN MetaLex legal-document interchange model.
//!
//! [CEN MetaLex](http://www.metalex.eu/) is the European (CEN Workshop
//! Agreement) standard for the interchange of legal sources. It is an
//! abstraction layer above national legislative XML formats, built on the FRBR
//! work/expression/manifestation model and a generic fragment hierarchy. This
//! module models the interchange structure:
//!
//! ```text
//! metalex (@xmlns)
//!   bibliographicWork (@id, @name)
//!     workIdentifier               (the canonical work IRI)
//!     realizedBy → expressionRef   (link to the expression)
//!   bibliographicExpression (@id, @name, @language)
//!     realizes → workRef
//!     content
//!       fragment (@id, @type, @name)   (recursively nestable)
//!         content (text)
//!         fragment*                    (sub-fragments)
//! ```
//!
//! When mapping a [`legalis_core::Statute`], the work captures the statute's
//! identity, the expression captures language/version, and the statute's
//! provisions become typed fragments under the expression's content: an
//! `eligibility` fragment with one `condition` sub-fragment per precondition and
//! an `effect` fragment. As with the other formats, the machine-readable
//! original of each provision is preserved in the fragment `@data` attribute so
//! the statute can be reconstructed exactly.

use crate::DiffError;
use crate::legal_xml::writer::XmlBuilder;
use crate::legal_xml::xml_error;
use crate::legal_xml::xml_util::{XmlNode, parse_document};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// CEN MetaLex namespace (the CEN MetaLex content-set namespace).
pub const METALEX_NAMESPACE: &str = "http://www.metalex.eu/schema/1.0/metalex-cen-rev1";

/// A document fragment in the MetaLex hierarchy (recursively nestable).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetalexFragment {
    /// `fragment/@id` — the fragment identifier.
    pub id: String,
    /// `fragment/@type` — the fragment's structural type (e.g. `eligibility`).
    pub fragment_type: String,
    /// `fragment/@name` — a human-readable name.
    pub name: String,
    /// `content` text of the fragment.
    pub content: String,
    /// `fragment/@data` — verbatim machine-readable payload, if any.
    pub data: Option<String>,
    /// Nested sub-fragments, in order.
    pub children: Vec<MetalexFragment>,
}

impl MetalexFragment {
    /// Creates a leaf fragment.
    fn leaf(
        id: impl Into<String>,
        fragment_type: impl Into<String>,
        name: impl Into<String>,
        content: impl Into<String>,
        data: Option<String>,
    ) -> Self {
        Self {
            id: id.into(),
            fragment_type: fragment_type.into(),
            name: name.into(),
            content: content.into(),
            data,
            children: Vec::new(),
        }
    }
}

/// The FRBR *work* level of a MetaLex document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetalexWork {
    /// `bibliographicWork/@id`.
    pub id: String,
    /// `bibliographicWork/@name`.
    pub name: String,
    /// The canonical work IRI emitted as `workIdentifier`.
    pub identifier: String,
    /// Jurisdiction code, emitted as a `jurisdiction` element when present.
    pub jurisdiction: Option<String>,
}

/// The FRBR *expression* level of a MetaLex document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetalexExpression {
    /// `bibliographicExpression/@id`.
    pub id: String,
    /// `bibliographicExpression/@name`.
    pub name: String,
    /// `bibliographicExpression/@language`.
    pub language: String,
    /// The expression's version.
    pub version: u32,
    /// Top-level content fragments.
    pub fragments: Vec<MetalexFragment>,
}

/// A complete CEN MetaLex interchange document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetalexDocument {
    /// The work level.
    pub work: MetalexWork,
    /// The expression level.
    pub expression: MetalexExpression,
}

impl MetalexDocument {
    /// Builds a MetaLex document from a statute.
    pub fn from_statute(statute: &Statute) -> Self {
        let work = MetalexWork {
            id: format!("{}_work", statute.id),
            name: statute.title.clone(),
            identifier: format!("/metalex/{}", statute.id),
            jurisdiction: statute.jurisdiction.clone(),
        };

        let mut fragments = Vec::new();

        // Eligibility fragment with one condition sub-fragment per precondition.
        if !statute.preconditions.is_empty() {
            let condition_fragments = statute
                .preconditions
                .iter()
                .enumerate()
                .map(|(idx, cond)| {
                    MetalexFragment::leaf(
                        format!("{}_cond_{}", statute.id, idx),
                        "condition",
                        format!("Condition {}", idx + 1),
                        cond.to_string(),
                        serde_json::to_string(cond).ok(),
                    )
                })
                .collect();
            fragments.push(MetalexFragment {
                id: format!("{}_eligibility", statute.id),
                fragment_type: "eligibility".to_string(),
                name: "Eligibility".to_string(),
                content: String::new(),
                data: None,
                children: condition_fragments,
            });
        }

        // Effect fragment.
        fragments.push(MetalexFragment::leaf(
            format!("{}_effect", statute.id),
            "effect",
            "Effect",
            format!(
                "{:?}: {}",
                statute.effect.effect_type, statute.effect.description
            ),
            serde_json::to_string(&statute.effect).ok(),
        ));

        // Discretion fragment, if present.
        if let Some(logic) = &statute.discretion_logic {
            fragments.push(MetalexFragment::leaf(
                format!("{}_discretion", statute.id),
                "discretion",
                "Discretion",
                logic.clone(),
                None,
            ));
        }

        let expression = MetalexExpression {
            id: format!("{}_expr_v{}", statute.id, statute.version),
            name: statute.title.clone(),
            language: "eng".to_string(),
            version: statute.version,
            fragments,
        };

        Self { work, expression }
    }

    /// Serializes the document to CEN MetaLex XML.
    ///
    /// # Errors
    ///
    /// Infallible at present; returns [`DiffError`] for API symmetry.
    pub fn to_xml(&self) -> Result<String, DiffError> {
        let mut b = XmlBuilder::new();
        b.open("metalex", &[("xmlns", METALEX_NAMESPACE)]);

        // ----- work -----
        b.open(
            "bibliographicWork",
            &[("id", &self.work.id), ("name", &self.work.name)],
        );
        b.leaf("workIdentifier", &[], &self.work.identifier);
        if let Some(j) = &self.work.jurisdiction {
            b.leaf("jurisdiction", &[], j);
        }
        b.empty("realizedBy", &[("idref", &self.expression.id)]);
        b.close("bibliographicWork");

        // ----- expression -----
        b.open(
            "bibliographicExpression",
            &[
                ("id", &self.expression.id),
                ("name", &self.expression.name),
                ("language", &self.expression.language),
                ("version", &self.expression.version.to_string()),
            ],
        );
        b.empty("realizes", &[("idref", &self.work.id)]);
        b.open("content", &[]);
        for fragment in &self.expression.fragments {
            write_fragment(&mut b, fragment);
        }
        b.close("content");
        b.close("bibliographicExpression");

        b.close("metalex");
        Ok(b.finish())
    }

    /// Parses a CEN MetaLex document from XML.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the XML is malformed or lacks
    /// a recognisable MetaLex structure.
    pub fn from_xml(xml: &str) -> Result<Self, DiffError> {
        let root = parse_document(xml)?;
        let metalex = if root.name == "metalex" {
            root
        } else {
            root.find_descendant("metalex")
                .cloned()
                .ok_or_else(|| xml_error("metalex", "missing <metalex> root"))?
        };

        let work_node = metalex
            .child("bibliographicWork")
            .ok_or_else(|| xml_error("metalex", "missing <bibliographicWork>"))?;
        let work = MetalexWork {
            id: work_node.attr("id").unwrap_or("").to_string(),
            name: work_node.attr("name").unwrap_or("").to_string(),
            identifier: work_node
                .child("workIdentifier")
                .map(|n| n.trimmed_text().to_string())
                .unwrap_or_default(),
            jurisdiction: work_node
                .child("jurisdiction")
                .map(|n| n.trimmed_text().to_string()),
        };

        let expr_node = metalex
            .child("bibliographicExpression")
            .ok_or_else(|| xml_error("metalex", "missing <bibliographicExpression>"))?;
        let fragments = expr_node
            .child("content")
            .map(|c| c.children_named("fragment").map(parse_fragment).collect())
            .unwrap_or_default();
        let expression = MetalexExpression {
            id: expr_node.attr("id").unwrap_or("").to_string(),
            name: expr_node.attr("name").unwrap_or("").to_string(),
            language: expr_node.attr("language").unwrap_or("eng").to_string(),
            version: expr_node
                .attr("version")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1),
            fragments,
        };

        Ok(Self { work, expression })
    }

    /// Reconstructs a [`Statute`] from this document.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if an embedded fragment payload
    /// cannot be deserialized.
    pub fn to_statute(&self) -> Result<Statute, DiffError> {
        let mut preconditions = Vec::new();
        let mut effect: Option<Effect> = None;
        let mut discretion: Option<String> = None;

        for fragment in &self.expression.fragments {
            match fragment.fragment_type.as_str() {
                "eligibility" => {
                    for cond_frag in &fragment.children {
                        if let Some(data) = &cond_frag.data {
                            let cond: Condition = serde_json::from_str(data)
                                .map_err(|e| xml_error("metalex condition", e))?;
                            preconditions.push(cond);
                        }
                    }
                }
                "effect" => {
                    effect = Some(match &fragment.data {
                        Some(data) => serde_json::from_str(data)
                            .map_err(|e| xml_error("metalex effect", e))?,
                        None => Effect::new(EffectType::Custom, fragment.content.clone()),
                    });
                }
                "discretion" => {
                    discretion = Some(fragment.content.clone());
                }
                _ => {}
            }
        }

        // The statute id is the work id without the `_work` suffix.
        let id = self
            .work
            .id
            .strip_suffix("_work")
            .unwrap_or(&self.work.id)
            .to_string();
        let effect = effect.unwrap_or_else(|| Effect::new(EffectType::Custom, String::new()));
        let mut statute = Statute::new(&id, &self.work.name, effect);
        statute.version = self.expression.version;
        statute.jurisdiction = self.work.jurisdiction.clone();
        statute.preconditions = preconditions;
        statute.discretion_logic = discretion;
        Ok(statute)
    }
}

/// Recursively emits a `<fragment>` and its children.
fn write_fragment(b: &mut XmlBuilder, fragment: &MetalexFragment) {
    let mut attrs: Vec<(&str, &str)> = vec![
        ("id", fragment.id.as_str()),
        ("type", fragment.fragment_type.as_str()),
        ("name", fragment.name.as_str()),
    ];
    if let Some(data) = &fragment.data {
        attrs.push(("data", data.as_str()));
    }
    b.open("fragment", &attrs);
    if !fragment.content.is_empty() {
        b.leaf("content", &[], &fragment.content);
    }
    for child in &fragment.children {
        write_fragment(b, child);
    }
    b.close("fragment");
}

/// Recursively parses a `<fragment>` node.
fn parse_fragment(node: &XmlNode) -> MetalexFragment {
    let content = node
        .children_named("content")
        .next()
        .map(|c| c.trimmed_text().to_string())
        .unwrap_or_default();
    let children = node
        .children_named("fragment")
        .map(parse_fragment)
        .collect();
    MetalexFragment {
        id: node.attr("id").unwrap_or("").to_string(),
        fragment_type: node.attr("type").unwrap_or("").to_string(),
        name: node.attr("name").unwrap_or("").to_string(),
        content,
        data: node.attr("data").map(|s| s.to_string()),
        children,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn sample_statute() -> Statute {
        Statute::new(
            "ml-1",
            "Residency Benefit",
            Effect::new(EffectType::Grant, "benefit granted"),
        )
        .with_precondition(Condition::ResidencyDuration {
            operator: ComparisonOp::GreaterOrEqual,
            months: 12,
        })
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_to_xml_structure() {
        let doc = MetalexDocument::from_statute(&sample_statute());
        let xml = doc.to_xml().expect("serialize");
        assert!(xml.contains("<metalex"));
        assert!(xml.contains("<bibliographicWork id=\"ml-1_work\""));
        assert!(xml.contains("<bibliographicExpression"));
        assert!(xml.contains("type=\"eligibility\""));
        assert!(xml.contains("type=\"condition\""));
        assert!(xml.contains("type=\"effect\""));
    }

    #[test]
    fn test_roundtrip_document() {
        let doc = MetalexDocument::from_statute(&sample_statute());
        let xml = doc.to_xml().expect("serialize");
        let parsed = MetalexDocument::from_xml(&xml).expect("parse");
        assert_eq!(doc, parsed);
    }

    #[test]
    fn test_roundtrip_statute() {
        let original = sample_statute();
        let doc = MetalexDocument::from_statute(&original);
        let xml = doc.to_xml().expect("serialize");
        let restored = MetalexDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.title, original.title);
        assert_eq!(restored.version, original.version);
        assert_eq!(restored.preconditions, original.preconditions);
        assert_eq!(restored.effect, original.effect);
    }

    #[test]
    fn test_nested_fragments() {
        let doc = MetalexDocument::from_statute(&sample_statute());
        // The eligibility fragment should contain two condition sub-fragments.
        let elig = doc
            .expression
            .fragments
            .iter()
            .find(|f| f.fragment_type == "eligibility")
            .expect("eligibility fragment");
        assert_eq!(elig.children.len(), 2);
    }

    #[test]
    fn test_jurisdiction_roundtrip() {
        let mut original = sample_statute();
        original.jurisdiction = Some("nl".into());
        let xml = MetalexDocument::from_statute(&original)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("<jurisdiction>nl</jurisdiction>"));
        let restored = MetalexDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.jurisdiction, Some("nl".to_string()));
    }

    #[test]
    fn test_discretion_roundtrip() {
        let mut original = sample_statute();
        original.discretion_logic = Some("case officer assessment".into());
        let xml = MetalexDocument::from_statute(&original)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("type=\"discretion\""));
        let restored = MetalexDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.discretion_logic, original.discretion_logic);
    }

    #[test]
    fn test_no_preconditions() {
        let statute = Statute::new("e", "Empty", Effect::new(EffectType::Revoke, "revoked"));
        let xml = MetalexDocument::from_statute(&statute)
            .to_xml()
            .expect("serialize");
        assert!(!xml.contains("type=\"eligibility\""));
        let restored = MetalexDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert!(restored.preconditions.is_empty());
        assert_eq!(restored.effect.effect_type, EffectType::Revoke);
    }

    #[test]
    fn test_special_characters() {
        let statute = Statute::new(
            "sp&1",
            "Tax < 5% & \"low\"",
            Effect::new(EffectType::Grant, "a > b"),
        );
        let xml = MetalexDocument::from_statute(&statute)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("&amp;"));
        let restored = MetalexDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.title, "Tax < 5% & \"low\"");
    }

    #[test]
    fn test_missing_root_errors() {
        assert!(MetalexDocument::from_xml("<other/>").is_err());
    }

    #[test]
    fn test_data_attribute_carries_payload() {
        let xml = MetalexDocument::from_statute(&sample_statute())
            .to_xml()
            .expect("serialize");
        // The effect fragment carries a JSON payload in its @data attribute.
        assert!(xml.contains("data=\""));
    }
}
