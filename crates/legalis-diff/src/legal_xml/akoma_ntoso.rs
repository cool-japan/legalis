//! Akoma Ntoso (OASIS *LegalDocML*) document model.
//!
//! [Akoma Ntoso](https://www.oasis-open.org/committees/legaldocml/) is the OASIS
//! standard for representing parliamentary, legislative and judicial documents
//! in XML. This module models the core *act* document type and its hierarchy:
//!
//! ```text
//! akomaNtoso
//!   act  (@name)
//!     meta
//!       identification  (@source)
//!         FRBRWork        → FRBRthis/@value, FRBRuri/@value, FRBRalias, FRBRdate, FRBRcountry
//!         FRBRExpression  → FRBRthis/@value, FRBRlanguage/@language, FRBRdate
//!         FRBRManifestation → FRBRthis/@value, FRBRformat/@value
//!     body
//!       section  (@eId, optional heading)
//!         article  (@eId)
//!           num
//!           heading
//!           content → p
//! ```
//!
//! The mapping to/from [`legalis_core::Statute`] is deliberately faithful to the
//! Akoma Ntoso vocabulary while remaining lossless for the statute fields the
//! standard can carry naturally: the statute id becomes the act `@name` and the
//! FRBR work id; the title becomes both the `FRBRalias` and the body's first
//! article heading; preconditions and the effect are emitted as articles within
//! a dedicated section, with the machine-readable original preserved in a
//! `<content>` `<p>` carrying its serialized form so the statute can be
//! reconstructed exactly.

use crate::DiffError;
use crate::legal_xml::writer::XmlBuilder;
use crate::legal_xml::xml_error;
use crate::legal_xml::xml_util::{XmlNode, parse_document};
use legalis_core::{Condition, Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Akoma Ntoso XML namespace (the 1.0 / CSD13 namespace URI).
pub const AKN_NAMESPACE: &str = "http://docs.oasis-open.org/legaldocml/ns/akn/3.0";

/// The FRBR / bibliographic metadata of an Akoma Ntoso act.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AknMeta {
    /// `act/@name` and `FRBRWork/FRBRthis/@value` — the work identifier.
    pub work_id: String,
    /// `FRBRalias/@value` — a human-readable short title.
    pub alias: String,
    /// `FRBRcountry/@value` — ISO jurisdiction code, if known.
    pub country: Option<String>,
    /// `FRBRWork/FRBRdate/@date` — the work date (free-form, ISO recommended).
    pub work_date: Option<String>,
    /// `FRBRExpression/FRBRlanguage/@language` — language code (default `eng`).
    pub language: String,
    /// `identification/@source` — the producing system / authority.
    pub source: String,
    /// Document version expressed in the expression's FRBR date suffix.
    pub version: u32,
}

impl AknMeta {
    /// Builds metadata from a statute.
    fn from_statute(statute: &Statute) -> Self {
        Self {
            work_id: statute.id.clone(),
            alias: statute.title.clone(),
            country: statute.jurisdiction.clone(),
            work_date: None,
            language: "eng".to_string(),
            source: "#legalis".to_string(),
            version: statute.version,
        }
    }
}

/// An `<article>` within a section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AknArticle {
    /// `@eId` — element identifier.
    pub e_id: String,
    /// `<num>` — the article number / label.
    pub num: String,
    /// `<heading>` — the article heading.
    pub heading: String,
    /// `<content><p>` — the article body text.
    pub content: String,
    /// Optional machine-readable payload preserved verbatim in a marked `<p>`.
    ///
    /// Used to round-trip the original statute element this article was derived
    /// from (a serialized [`Condition`] or [`Effect`]).
    pub machine_payload: Option<String>,
}

/// A `<section>` containing articles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AknSection {
    /// `@eId` — element identifier.
    pub e_id: String,
    /// `<heading>` — the section heading.
    pub heading: String,
    /// Articles within the section, in order.
    pub articles: Vec<AknArticle>,
}

/// The `<body>` of an act.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AknBody {
    /// Sections in document order.
    pub sections: Vec<AknSection>,
}

/// A complete Akoma Ntoso act document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AkomaNtosoDocument {
    /// Document metadata.
    pub meta: AknMeta,
    /// Document body.
    pub body: AknBody,
}

/// The marker attribute used to flag a `<p>` that carries a machine payload.
const PAYLOAD_MARKER: &str = "legalis:payload";

impl AkomaNtosoDocument {
    /// Builds an Akoma Ntoso act document from a statute.
    ///
    /// The statute's preconditions become articles in an *Eligibility* section
    /// and the effect becomes the sole article of an *Effect* section. Each
    /// generated article preserves the serialized original in a marked `<p>` so
    /// [`Self::to_statute`] can reconstruct the statute exactly.
    pub fn from_statute(statute: &Statute) -> Self {
        let meta = AknMeta::from_statute(statute);
        let mut sections = Vec::new();

        // Eligibility section: one article per precondition.
        if !statute.preconditions.is_empty() {
            let mut articles = Vec::new();
            for (idx, cond) in statute.preconditions.iter().enumerate() {
                let payload = serde_json::to_string(cond).ok();
                articles.push(AknArticle {
                    e_id: format!("art_elig_{}", idx + 1),
                    num: format!("Art. {}", idx + 1),
                    heading: format!("Eligibility condition {}", idx + 1),
                    content: cond.to_string(),
                    machine_payload: payload,
                });
            }
            sections.push(AknSection {
                e_id: "sec_eligibility".to_string(),
                heading: "Eligibility".to_string(),
                articles,
            });
        }

        // Effect section: a single article describing the legal effect.
        let effect_payload = serde_json::to_string(&statute.effect).ok();
        let effect_article = AknArticle {
            e_id: "art_effect".to_string(),
            num: "Art. E".to_string(),
            heading: "Legal effect".to_string(),
            content: format!(
                "{:?}: {}",
                statute.effect.effect_type, statute.effect.description
            ),
            machine_payload: effect_payload,
        };
        sections.push(AknSection {
            e_id: "sec_effect".to_string(),
            heading: "Effect".to_string(),
            articles: vec![effect_article],
        });

        // Discretion section, if the statute has discretionary logic.
        if let Some(logic) = &statute.discretion_logic {
            sections.push(AknSection {
                e_id: "sec_discretion".to_string(),
                heading: "Discretion".to_string(),
                articles: vec![AknArticle {
                    e_id: "art_discretion".to_string(),
                    num: "Art. D".to_string(),
                    heading: "Discretionary judgment".to_string(),
                    content: logic.clone(),
                    machine_payload: None,
                }],
            });
        }

        Self {
            meta,
            body: AknBody { sections },
        }
    }

    /// Serializes the document to Akoma Ntoso XML.
    ///
    /// # Errors
    ///
    /// Currently infallible, but returns [`DiffError`] for API symmetry with the
    /// other formats and to allow future validation.
    pub fn to_xml(&self) -> Result<String, DiffError> {
        let mut b = XmlBuilder::new();
        b.open("akomaNtoso", &[("xmlns", AKN_NAMESPACE)]);
        b.open("act", &[("name", &self.meta.work_id)]);

        // ----- meta / identification (FRBR) -----
        b.open("meta", &[]);
        b.open("identification", &[("source", &self.meta.source)]);

        let work_this = format!("/akn/work/{}", self.meta.work_id);
        b.open("FRBRWork", &[]);
        b.empty("FRBRthis", &[("value", &work_this)]);
        b.empty(
            "FRBRuri",
            &[("value", &format!("/akn/{}", self.meta.work_id))],
        );
        b.empty("FRBRalias", &[("value", &self.meta.alias)]);
        if let Some(date) = &self.meta.work_date {
            b.empty("FRBRdate", &[("date", date), ("name", "enactment")]);
        }
        if let Some(country) = &self.meta.country {
            b.empty("FRBRcountry", &[("value", country)]);
        }
        b.close("FRBRWork");

        let expr_this = format!("/akn/expr/{}/v{}", self.meta.work_id, self.meta.version);
        b.open("FRBRExpression", &[]);
        b.empty("FRBRthis", &[("value", &expr_this)]);
        b.empty("FRBRlanguage", &[("language", &self.meta.language)]);
        b.empty(
            "FRBRversionNumber",
            &[("value", &self.meta.version.to_string())],
        );
        b.close("FRBRExpression");

        b.open("FRBRManifestation", &[]);
        b.empty("FRBRthis", &[("value", &format!("{expr_this}.xml"))]);
        b.empty("FRBRformat", &[("value", "xml")]);
        b.close("FRBRManifestation");

        b.close("identification");
        b.close("meta");

        // ----- body -----
        b.open("body", &[]);
        for section in &self.body.sections {
            b.open("section", &[("eId", &section.e_id)]);
            b.leaf("heading", &[], &section.heading);
            for article in &section.articles {
                b.open("article", &[("eId", &article.e_id)]);
                b.leaf("num", &[], &article.num);
                b.leaf("heading", &[], &article.heading);
                b.open("content", &[]);
                b.leaf("p", &[], &article.content);
                if let Some(payload) = &article.machine_payload {
                    b.leaf("p", &[("class", PAYLOAD_MARKER)], payload);
                }
                b.close("content");
                b.close("article");
            }
            b.close("section");
        }
        b.close("body");

        b.close("act");
        b.close("akomaNtoso");
        Ok(b.finish())
    }

    /// Parses an Akoma Ntoso act document from XML.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the XML is malformed or does
    /// not contain a recognisable `akomaNtoso`/`act` structure.
    pub fn from_xml(xml: &str) -> Result<Self, DiffError> {
        let root = parse_document(xml)?;
        let akn = if root.name == "akomaNtoso" {
            root
        } else {
            root.find_descendant("akomaNtoso")
                .cloned()
                .ok_or_else(|| xml_error("akoma ntoso", "missing <akomaNtoso> root"))?
        };
        let act = akn
            .child("act")
            .ok_or_else(|| xml_error("akoma ntoso", "missing <act>"))?;

        let meta = parse_meta(act)?;
        let body = parse_body(act)?;

        Ok(Self { meta, body })
    }

    /// Reconstructs a [`Statute`] from this document.
    ///
    /// Preconditions and the effect are recovered from the machine-readable
    /// payloads embedded by [`Self::from_statute`]; if a payload is missing, the
    /// effect falls back to a [`EffectType::Custom`] effect carrying the article
    /// content, and conditions without a payload are skipped.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if an embedded payload is
    /// present but cannot be deserialized.
    pub fn to_statute(&self) -> Result<Statute, DiffError> {
        let mut preconditions = Vec::new();
        let mut effect: Option<Effect> = None;
        let mut discretion: Option<String> = None;

        for section in &self.body.sections {
            match section.e_id.as_str() {
                "sec_eligibility" => {
                    for article in &section.articles {
                        if let Some(payload) = &article.machine_payload {
                            let cond: Condition = serde_json::from_str(payload)
                                .map_err(|e| xml_error("akoma ntoso condition", e))?;
                            preconditions.push(cond);
                        }
                    }
                }
                "sec_effect" => {
                    if let Some(article) = section.articles.first() {
                        if let Some(payload) = &article.machine_payload {
                            effect = Some(
                                serde_json::from_str(payload)
                                    .map_err(|e| xml_error("akoma ntoso effect", e))?,
                            );
                        } else {
                            effect = Some(Effect::new(EffectType::Custom, article.content.clone()));
                        }
                    }
                }
                "sec_discretion" => {
                    if let Some(article) = section.articles.first() {
                        discretion = Some(article.content.clone());
                    }
                }
                _ => {}
            }
        }

        let effect = effect.unwrap_or_else(|| Effect::new(EffectType::Custom, String::new()));
        let mut statute = Statute::new(&self.meta.work_id, &self.meta.alias, effect);
        statute.version = self.meta.version;
        statute.jurisdiction = self.meta.country.clone();
        statute.preconditions = preconditions;
        statute.discretion_logic = discretion;
        Ok(statute)
    }
}

fn parse_meta(act: &XmlNode) -> Result<AknMeta, DiffError> {
    let work_id = act
        .attr("name")
        .map(|s| s.to_string())
        .or_else(|| {
            // Fall back to the FRBRWork/FRBRthis value's last path segment.
            act.find_descendant("FRBRWork")
                .and_then(|w| w.child("FRBRthis"))
                .and_then(|t| t.attr("value"))
                .map(|v| v.rsplit('/').next().unwrap_or(v).to_string())
        })
        .ok_or_else(|| xml_error("akoma ntoso", "act has no name / work id"))?;

    let meta = act.child("meta");
    let identification = meta.and_then(|m| m.child("identification"));
    let source = identification
        .and_then(|i| i.attr("source"))
        .unwrap_or("#legalis")
        .to_string();

    let frbr_work = identification.and_then(|i| i.child("FRBRWork"));
    let alias = frbr_work
        .and_then(|w| w.child("FRBRalias"))
        .and_then(|a| a.attr("value"))
        .unwrap_or(&work_id)
        .to_string();
    let country = frbr_work
        .and_then(|w| w.child("FRBRcountry"))
        .and_then(|c| c.attr("value"))
        .map(|s| s.to_string());
    let work_date = frbr_work
        .and_then(|w| w.child("FRBRdate"))
        .and_then(|d| d.attr("date"))
        .map(|s| s.to_string());

    let frbr_expr = identification.and_then(|i| i.child("FRBRExpression"));
    let language = frbr_expr
        .and_then(|e| e.child("FRBRlanguage"))
        .and_then(|l| l.attr("language"))
        .unwrap_or("eng")
        .to_string();
    let version = frbr_expr
        .and_then(|e| e.child("FRBRversionNumber"))
        .and_then(|v| v.attr("value"))
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1);

    Ok(AknMeta {
        work_id,
        alias,
        country,
        work_date,
        language,
        source,
        version,
    })
}

fn parse_body(act: &XmlNode) -> Result<AknBody, DiffError> {
    let body = match act.child("body") {
        Some(b) => b,
        None => return Ok(AknBody::default()),
    };

    let mut sections = Vec::new();
    for section_node in body.children_named("section") {
        let e_id = section_node.attr("eId").unwrap_or("").to_string();
        let heading = section_node
            .child("heading")
            .map(|h| h.trimmed_text().to_string())
            .unwrap_or_default();

        let mut articles = Vec::new();
        for article_node in section_node.children_named("article") {
            articles.push(parse_article(article_node));
        }

        sections.push(AknSection {
            e_id,
            heading,
            articles,
        });
    }

    Ok(AknBody { sections })
}

fn parse_article(node: &XmlNode) -> AknArticle {
    let e_id = node.attr("eId").unwrap_or("").to_string();
    let num = node
        .child("num")
        .map(|n| n.trimmed_text().to_string())
        .unwrap_or_default();
    let heading = node
        .child("heading")
        .map(|h| h.trimmed_text().to_string())
        .unwrap_or_default();

    let mut content = String::new();
    let mut machine_payload = None;
    if let Some(content_node) = node.child("content") {
        for p in content_node.children_named("p") {
            if p.attr("class") == Some(PAYLOAD_MARKER) {
                machine_payload = Some(p.trimmed_text().to_string());
            } else if content.is_empty() {
                content = p.trimmed_text().to_string();
            }
        }
    }

    AknArticle {
        e_id,
        num,
        heading,
        content,
        machine_payload,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn sample_statute() -> Statute {
        Statute::new(
            "act-42",
            "Senior Tax Credit Act",
            Effect::new(EffectType::Grant, "Tax credit granted"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50_000,
        })
    }

    #[test]
    fn test_to_xml_structure() {
        let doc = AkomaNtosoDocument::from_statute(&sample_statute());
        let xml = doc.to_xml().expect("serialize");
        assert!(xml.contains("<akomaNtoso"));
        assert!(xml.contains("<act name=\"act-42\">"));
        assert!(xml.contains("<FRBRalias value=\"Senior Tax Credit Act\"/>"));
        assert!(xml.contains("<section eId=\"sec_eligibility\">"));
        assert!(xml.contains("<article eId=\"art_effect\">"));
    }

    #[test]
    fn test_roundtrip_document() {
        let doc = AkomaNtosoDocument::from_statute(&sample_statute());
        let xml = doc.to_xml().expect("serialize");
        let parsed = AkomaNtosoDocument::from_xml(&xml).expect("parse");
        assert_eq!(doc, parsed);
    }

    #[test]
    fn test_roundtrip_statute() {
        let original = sample_statute();
        let doc = AkomaNtosoDocument::from_statute(&original);
        let xml = doc.to_xml().expect("serialize");
        let parsed = AkomaNtosoDocument::from_xml(&xml).expect("parse");
        let restored = parsed.to_statute().expect("to statute");

        assert_eq!(restored.id, original.id);
        assert_eq!(restored.title, original.title);
        assert_eq!(restored.version, original.version);
        assert_eq!(restored.preconditions, original.preconditions);
        assert_eq!(restored.effect, original.effect);
    }

    #[test]
    fn test_roundtrip_with_discretion() {
        let mut original = sample_statute();
        original.discretion_logic = Some("officer reviews hardship".into());
        let doc = AkomaNtosoDocument::from_statute(&original);
        let xml = doc.to_xml().expect("serialize");
        assert!(xml.contains("sec_discretion"));
        let restored = AkomaNtosoDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.discretion_logic, original.discretion_logic);
    }

    #[test]
    fn test_jurisdiction_preserved() {
        let mut original = sample_statute();
        original.jurisdiction = Some("de".into());
        let xml = AkomaNtosoDocument::from_statute(&original)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("<FRBRcountry value=\"de\"/>"));
        let restored = AkomaNtosoDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.jurisdiction, Some("de".to_string()));
    }

    #[test]
    fn test_no_preconditions() {
        let statute = Statute::new("a", "T", Effect::new(EffectType::Obligation, "must"));
        let doc = AkomaNtosoDocument::from_statute(&statute);
        let xml = doc.to_xml().expect("serialize");
        // No eligibility section when there are no preconditions.
        assert!(!xml.contains("sec_eligibility"));
        let restored = AkomaNtosoDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert!(restored.preconditions.is_empty());
        assert_eq!(restored.effect.effect_type, EffectType::Obligation);
    }

    #[test]
    fn test_special_characters_escaped() {
        let statute = Statute::new(
            "x",
            "Title with <tags> & \"quotes\"",
            Effect::new(EffectType::Grant, "a & b"),
        );
        let xml = AkomaNtosoDocument::from_statute(&statute)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("&lt;tags&gt;"));
        assert!(xml.contains("&quot;quotes&quot;"));
        let restored = AkomaNtosoDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.title, "Title with <tags> & \"quotes\"");
    }

    #[test]
    fn test_missing_root_errors() {
        assert!(AkomaNtosoDocument::from_xml("<foo/>").is_err());
    }

    #[test]
    fn test_parse_external_minimal() {
        // A minimal hand-written AKN document (no machine payloads) must parse.
        let xml = r##"<?xml version="1.0"?>
<akomaNtoso xmlns="http://docs.oasis-open.org/legaldocml/ns/akn/3.0">
  <act name="ext-1">
    <meta>
      <identification source="#gov">
        <FRBRWork>
          <FRBRalias value="External Act"/>
        </FRBRWork>
      </identification>
    </meta>
    <body>
      <section eId="sec_effect">
        <heading>Effect</heading>
        <article eId="art_effect">
          <num>Art. E</num>
          <heading>Legal effect</heading>
          <content><p>Grant: something</p></content>
        </article>
      </section>
    </body>
  </act>
</akomaNtoso>"##;
        let doc = AkomaNtosoDocument::from_xml(xml).expect("parse");
        assert_eq!(doc.meta.work_id, "ext-1");
        assert_eq!(doc.meta.alias, "External Act");
        let statute = doc.to_statute().expect("statute");
        // No payload => Custom effect carrying the content.
        assert_eq!(statute.effect.effect_type, EffectType::Custom);
        assert_eq!(statute.effect.description, "Grant: something");
    }
}
