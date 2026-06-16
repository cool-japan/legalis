//! OASIS LegalRuleML document model.
//!
//! [LegalRuleML](https://www.oasis-open.org/committees/legalruleml/) is the
//! OASIS standard for modelling the *normative* content of legal texts as rules.
//! This module models the core rule vocabulary:
//!
//! ```text
//! lrml:LegalRuleML
//!   lrml:Statements
//!     lrml:PrescriptiveStatement (@key)
//!       ruleml:Rule (@key)
//!         ruleml:if   → ruleml:And → ruleml:Atom*   (the conditions)
//!         ruleml:then → lrml:Obligation | lrml:Permission | lrml:Prohibition
//!                         → ruleml:Atom               (the deontic conclusion)
//!     lrml:ConstitutiveStatement (@key)            (asserted facts)
//!       ruleml:Atom
//! ```
//!
//! A LegalRuleML *atom* is a predicate applied to terms; here each atom is
//! `ruleml:Atom → ruleml:Rel (the predicate)` plus zero or more
//! `ruleml:Ind`/`ruleml:Var` terms. The mapping from a [`legalis_core::Statute`]
//! turns each precondition into a condition atom in the rule body and the
//! statute's [`legalis_core::EffectType`] selects the deontic operator wrapping
//! the conclusion atom:
//!
//! | `EffectType`        | Deontic operator      |
//! |---------------------|-----------------------|
//! | `Obligation`        | `lrml:Obligation`     |
//! | `Prohibition`       | `lrml:Prohibition`    |
//! | `Grant`             | `lrml:Permission`     |
//! | everything else     | `lrml:Permission`     |

use crate::DiffError;
use crate::legal_xml::writer::XmlBuilder;
use crate::legal_xml::xml_error;
use crate::legal_xml::xml_util::{XmlNode, parse_document};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// LegalRuleML namespace.
pub const LRML_NAMESPACE: &str = "http://docs.oasis-open.org/legalruleml/ns/v1.0/";
/// RuleML namespace (LegalRuleML reuses RuleML for the logical layer).
pub const RULEML_NAMESPACE: &str = "http://ruleml.org/spec";

/// The deontic modality of a prescriptive statement's conclusion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeonticKind {
    /// `lrml:Obligation` — the conclusion is required.
    Obligation,
    /// `lrml:Permission` — the conclusion is allowed.
    Permission,
    /// `lrml:Prohibition` — the conclusion is forbidden.
    Prohibition,
}

impl DeonticKind {
    /// The LegalRuleML element local name for this modality.
    pub fn element_name(&self) -> &'static str {
        match self {
            Self::Obligation => "Obligation",
            Self::Permission => "Permission",
            Self::Prohibition => "Prohibition",
        }
    }

    /// Parses a modality from a LegalRuleML element local name.
    fn from_element(name: &str) -> Option<Self> {
        match name {
            "Obligation" => Some(Self::Obligation),
            "Permission" => Some(Self::Permission),
            "Prohibition" => Some(Self::Prohibition),
            _ => None,
        }
    }

    /// Maps a statute effect type to the corresponding deontic modality.
    pub fn from_effect_type(effect_type: &EffectType) -> Self {
        match effect_type {
            EffectType::Obligation => Self::Obligation,
            EffectType::Prohibition => Self::Prohibition,
            _ => Self::Permission,
        }
    }
}

/// A RuleML atom: a predicate (`ruleml:Rel`) applied to individual terms
/// (`ruleml:Ind`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuleAtom {
    /// The relation / predicate name (`ruleml:Rel`).
    pub predicate: String,
    /// Individual constant terms (`ruleml:Ind`), in order.
    pub terms: Vec<String>,
}

impl RuleAtom {
    /// Creates an atom from a predicate and its terms.
    pub fn new(predicate: impl Into<String>, terms: Vec<String>) -> Self {
        Self {
            predicate: predicate.into(),
            terms,
        }
    }
}

/// A single legal rule: a body (conjunction of condition atoms) implying a
/// deontic conclusion atom.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LegalRule {
    /// `ruleml:Rule/@key` — the rule identifier.
    pub key: String,
    /// Conjoined condition atoms forming `ruleml:if`.
    pub conditions: Vec<RuleAtom>,
    /// The deontic modality wrapping the conclusion.
    pub deontic: DeonticKind,
    /// The conclusion atom inside `ruleml:then`.
    pub conclusion: RuleAtom,
}

/// A LegalRuleML statement: either a prescriptive rule or an asserted fact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleStatement {
    /// `lrml:PrescriptiveStatement` wrapping a [`LegalRule`].
    Prescriptive(LegalRule),
    /// `lrml:ConstitutiveStatement` asserting a bare fact atom, with its key.
    Constitutive {
        /// The statement key.
        key: String,
        /// The asserted fact.
        fact: RuleAtom,
    },
}

/// A complete LegalRuleML document.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct LegalRuleMlDocument {
    /// All statements, in document order.
    pub statements: Vec<RuleStatement>,
}

impl LegalRuleMlDocument {
    /// Builds a LegalRuleML document from a statute.
    ///
    /// The statute becomes a single prescriptive statement: each precondition is
    /// a condition atom (`precondition(<id>, <serialized condition>)`) and the
    /// effect becomes the deontic conclusion atom
    /// (`effect(<id>, <description>)`), with the deontic operator chosen from the
    /// effect type. A constitutive statement records the statute's identity.
    pub fn from_statute(statute: &Statute) -> Self {
        let mut statements = Vec::new();

        // Constitutive: assert the statute exists with its title.
        statements.push(RuleStatement::Constitutive {
            key: format!("fact_{}", statute.id),
            fact: RuleAtom::new("statute", vec![statute.id.clone(), statute.title.clone()]),
        });

        // Prescriptive: the rule itself.
        let conditions: Vec<RuleAtom> = statute
            .preconditions
            .iter()
            .enumerate()
            .map(|(idx, cond)| {
                let serialized = serde_json::to_string(cond).unwrap_or_default();
                RuleAtom::new(
                    "precondition",
                    vec![format!("{}#{}", statute.id, idx), serialized],
                )
            })
            .collect();

        let effect_serialized = serde_json::to_string(&statute.effect).unwrap_or_default();
        let conclusion = RuleAtom::new("effect", vec![statute.id.clone(), effect_serialized]);

        statements.push(RuleStatement::Prescriptive(LegalRule {
            key: format!("rule_{}", statute.id),
            conditions,
            deontic: DeonticKind::from_effect_type(&statute.effect.effect_type),
            conclusion,
        }));

        Self { statements }
    }

    /// Serializes the document to LegalRuleML XML.
    ///
    /// # Errors
    ///
    /// Infallible at present; returns [`DiffError`] for API symmetry.
    pub fn to_xml(&self) -> Result<String, DiffError> {
        let mut b = XmlBuilder::new();
        b.open(
            "lrml:LegalRuleML",
            &[
                ("xmlns:lrml", LRML_NAMESPACE),
                ("xmlns:ruleml", RULEML_NAMESPACE),
            ],
        );
        b.open("lrml:Statements", &[]);

        for statement in &self.statements {
            match statement {
                RuleStatement::Constitutive { key, fact } => {
                    b.open("lrml:ConstitutiveStatement", &[("key", key)]);
                    write_atom(&mut b, fact);
                    b.close("lrml:ConstitutiveStatement");
                }
                RuleStatement::Prescriptive(rule) => {
                    b.open("lrml:PrescriptiveStatement", &[("key", &rule.key)]);
                    b.open("ruleml:Rule", &[("key", &rule.key)]);

                    // if → And → atoms
                    b.open("ruleml:if", &[]);
                    b.open("ruleml:And", &[]);
                    for atom in &rule.conditions {
                        write_atom(&mut b, atom);
                    }
                    b.close("ruleml:And");
                    b.close("ruleml:if");

                    // then → deontic → atom
                    b.open("ruleml:then", &[]);
                    let deontic_el = format!("lrml:{}", rule.deontic.element_name());
                    b.open(&deontic_el, &[]);
                    write_atom(&mut b, &rule.conclusion);
                    b.close(&deontic_el);
                    b.close("ruleml:then");

                    b.close("ruleml:Rule");
                    b.close("lrml:PrescriptiveStatement");
                }
            }
        }

        b.close("lrml:Statements");
        b.close("lrml:LegalRuleML");
        Ok(b.finish())
    }

    /// Parses a LegalRuleML document from XML.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if the XML is malformed or lacks
    /// a recognisable LegalRuleML structure.
    pub fn from_xml(xml: &str) -> Result<Self, DiffError> {
        let root = parse_document(xml)?;
        let lrml = if root.name == "LegalRuleML" {
            root
        } else {
            root.find_descendant("LegalRuleML")
                .cloned()
                .ok_or_else(|| xml_error("legalruleml", "missing <LegalRuleML> root"))?
        };
        let statements_node = lrml
            .child("Statements")
            .ok_or_else(|| xml_error("legalruleml", "missing <Statements>"))?;

        let mut statements = Vec::new();
        for child in &statements_node.children {
            match child.name.as_str() {
                "ConstitutiveStatement" => {
                    let key = child.attr("key").unwrap_or("").to_string();
                    let fact = child
                        .child("Atom")
                        .map(parse_atom)
                        .ok_or_else(|| xml_error("legalruleml", "constitutive without atom"))?;
                    statements.push(RuleStatement::Constitutive { key, fact });
                }
                "PrescriptiveStatement" => {
                    statements.push(RuleStatement::Prescriptive(parse_rule(child)?));
                }
                _ => {}
            }
        }

        Ok(Self { statements })
    }

    /// Reconstructs a [`Statute`] from the first prescriptive statement.
    ///
    /// Preconditions and effect are recovered from the serialized payloads in the
    /// rule atoms produced by [`Self::from_statute`]. The statute id/title are
    /// taken from the constitutive statement when present, otherwise from the
    /// rule's conclusion subject.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if no prescriptive statement is
    /// present or an embedded payload cannot be deserialized.
    pub fn to_statute(&self) -> Result<Statute, DiffError> {
        let rule = self
            .statements
            .iter()
            .find_map(|s| match s {
                RuleStatement::Prescriptive(r) => Some(r),
                _ => None,
            })
            .ok_or_else(|| xml_error("legalruleml", "no prescriptive statement"))?;

        // id / title from the constitutive fact if available.
        let (id, title) = self
            .statements
            .iter()
            .find_map(|s| match s {
                RuleStatement::Constitutive { fact, .. } if fact.predicate == "statute" => {
                    let id = fact.terms.first().cloned().unwrap_or_default();
                    let title = fact.terms.get(1).cloned().unwrap_or_default();
                    Some((id, title))
                }
                _ => None,
            })
            .unwrap_or_else(|| {
                let id = rule.conclusion.terms.first().cloned().unwrap_or_default();
                (id.clone(), id)
            });

        // Effect from the conclusion's serialized payload.
        let effect = match rule.conclusion.terms.get(1) {
            Some(payload) if !payload.is_empty() => {
                serde_json::from_str(payload).map_err(|e| xml_error("legalruleml effect", e))?
            }
            _ => Effect::new(EffectType::Custom, String::new()),
        };

        // Preconditions from condition atoms' serialized payloads.
        let mut preconditions = Vec::new();
        for atom in &rule.conditions {
            if let Some(payload) = atom.terms.get(1).filter(|p| !p.is_empty()) {
                let cond = serde_json::from_str(payload)
                    .map_err(|e| xml_error("legalruleml condition", e))?;
                preconditions.push(cond);
            }
        }

        let mut statute = Statute::new(&id, &title, effect);
        statute.preconditions = preconditions;
        Ok(statute)
    }
}

/// Emits a `ruleml:Atom` from a [`RuleAtom`].
fn write_atom(b: &mut XmlBuilder, atom: &RuleAtom) {
    b.open("ruleml:Atom", &[]);
    b.leaf("ruleml:Rel", &[], &atom.predicate);
    for term in &atom.terms {
        b.leaf("ruleml:Ind", &[], term);
    }
    b.close("ruleml:Atom");
}

/// Parses a `ruleml:Atom` node into a [`RuleAtom`].
fn parse_atom(node: &XmlNode) -> RuleAtom {
    let predicate = node
        .child("Rel")
        .map(|r| r.trimmed_text().to_string())
        .unwrap_or_default();
    let terms = node
        .children_named("Ind")
        .map(|i| i.trimmed_text().to_string())
        .collect();
    RuleAtom { predicate, terms }
}

/// Parses a `lrml:PrescriptiveStatement` into a [`LegalRule`].
fn parse_rule(statement: &XmlNode) -> Result<LegalRule, DiffError> {
    let rule_node = statement
        .child("Rule")
        .ok_or_else(|| xml_error("legalruleml", "prescriptive without <Rule>"))?;
    let key = rule_node
        .attr("key")
        .or_else(|| statement.attr("key"))
        .unwrap_or("")
        .to_string();

    // Conditions: if → And → Atom*
    let conditions = rule_node
        .child("if")
        .and_then(|i| i.child("And"))
        .map(|and| and.children_named("Atom").map(parse_atom).collect())
        .unwrap_or_default();

    // Conclusion: then → (deontic) → Atom
    let then_node = rule_node
        .child("then")
        .ok_or_else(|| xml_error("legalruleml", "rule without <then>"))?;
    let deontic_node = then_node
        .children
        .iter()
        .find(|c| DeonticKind::from_element(&c.name).is_some())
        .ok_or_else(|| xml_error("legalruleml", "no deontic operator in <then>"))?;
    let deontic = DeonticKind::from_element(&deontic_node.name)
        .ok_or_else(|| xml_error("legalruleml", "unknown deontic operator"))?;
    let conclusion = deontic_node
        .child("Atom")
        .map(parse_atom)
        .ok_or_else(|| xml_error("legalruleml", "deontic without atom"))?;

    Ok(LegalRule {
        key,
        conditions,
        deontic,
        conclusion,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn obligation_statute() -> Statute {
        Statute::new(
            "ob-1",
            "Filing Obligation",
            Effect::new(EffectType::Obligation, "must file annual return"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::GreaterOrEqual,
            value: 10_000,
        })
    }

    #[test]
    fn test_to_xml_has_deontic() {
        let doc = LegalRuleMlDocument::from_statute(&obligation_statute());
        let xml = doc.to_xml().expect("serialize");
        assert!(xml.contains("<lrml:LegalRuleML"));
        assert!(xml.contains("<lrml:PrescriptiveStatement"));
        assert!(xml.contains("<lrml:Obligation>"));
        assert!(xml.contains("<ruleml:Atom>"));
    }

    #[test]
    fn test_deontic_mapping() {
        assert_eq!(
            DeonticKind::from_effect_type(&EffectType::Obligation),
            DeonticKind::Obligation
        );
        assert_eq!(
            DeonticKind::from_effect_type(&EffectType::Prohibition),
            DeonticKind::Prohibition
        );
        assert_eq!(
            DeonticKind::from_effect_type(&EffectType::Grant),
            DeonticKind::Permission
        );
        assert_eq!(
            DeonticKind::from_effect_type(&EffectType::MonetaryTransfer),
            DeonticKind::Permission
        );
    }

    #[test]
    fn test_roundtrip_document() {
        let doc = LegalRuleMlDocument::from_statute(&obligation_statute());
        let xml = doc.to_xml().expect("serialize");
        let parsed = LegalRuleMlDocument::from_xml(&xml).expect("parse");
        assert_eq!(doc, parsed);
    }

    #[test]
    fn test_roundtrip_statute() {
        let original = obligation_statute();
        let doc = LegalRuleMlDocument::from_statute(&original);
        let xml = doc.to_xml().expect("serialize");
        let restored = LegalRuleMlDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.title, original.title);
        assert_eq!(restored.preconditions, original.preconditions);
        assert_eq!(restored.effect, original.effect);
    }

    #[test]
    fn test_prohibition_roundtrip() {
        let original = Statute::new(
            "pr-1",
            "No Smoking",
            Effect::new(EffectType::Prohibition, "smoking forbidden"),
        );
        let xml = LegalRuleMlDocument::from_statute(&original)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("<lrml:Prohibition>"));
        let restored = LegalRuleMlDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.effect.effect_type, EffectType::Prohibition);
    }

    #[test]
    fn test_permission_for_grant() {
        let original = Statute::new(
            "gr-1",
            "Allowance",
            Effect::new(EffectType::Grant, "allowance granted"),
        );
        let xml = LegalRuleMlDocument::from_statute(&original)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("<lrml:Permission>"));
    }

    #[test]
    fn test_constitutive_statement_present() {
        let doc = LegalRuleMlDocument::from_statute(&obligation_statute());
        let has_constitutive = doc
            .statements
            .iter()
            .any(|s| matches!(s, RuleStatement::Constitutive { .. }));
        assert!(has_constitutive);
    }

    #[test]
    fn test_missing_prescriptive_errors() {
        let xml = r#"<lrml:LegalRuleML xmlns:lrml="x"><lrml:Statements></lrml:Statements></lrml:LegalRuleML>"#;
        let doc = LegalRuleMlDocument::from_xml(xml).expect("parse");
        assert!(doc.to_statute().is_err());
    }

    #[test]
    fn test_element_name() {
        assert_eq!(DeonticKind::Obligation.element_name(), "Obligation");
        assert_eq!(DeonticKind::Permission.element_name(), "Permission");
        assert_eq!(DeonticKind::Prohibition.element_name(), "Prohibition");
    }

    #[test]
    fn test_special_chars_in_atoms() {
        let original = Statute::new(
            "s&1",
            "A & B <law>",
            Effect::new(EffectType::Obligation, "x < y & z"),
        );
        let xml = LegalRuleMlDocument::from_statute(&original)
            .to_xml()
            .expect("serialize");
        assert!(xml.contains("&amp;"));
        let restored = LegalRuleMlDocument::from_xml(&xml)
            .expect("parse")
            .to_statute()
            .expect("statute");
        assert_eq!(restored.id, "s&1");
        assert_eq!(restored.title, "A & B <law>");
    }
}
