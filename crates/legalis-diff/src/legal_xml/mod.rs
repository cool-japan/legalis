//! Legal XML interchange formats (v0.5.9).
//!
//! This module implements serialization and parsing for three real
//! legal-informatics XML standards, modelling the core element structure of
//! each rather than a lossy approximation:
//!
//! - [`akoma_ntoso`] — **Akoma Ntoso** (OASIS *LegalDocML*): the
//!   `akomaNtoso → act → meta`/`body → section → article` document hierarchy,
//!   with FRBR-style metadata. See [`akoma_ntoso::AkomaNtosoDocument`].
//! - [`legalruleml`] — **OASIS LegalRuleML**: legal *rules* expressed as
//!   facts, prescriptive statements and deontic operators (obligation /
//!   permission / prohibition). See [`legalruleml::LegalRuleMlDocument`].
//! - [`metalex`] — **CEN MetaLex**: the CEN metalex bibliographic /
//!   fragment interchange structure (`bibliographicWork → ... → fragment`).
//!   See [`metalex::MetalexDocument`].
//!
//! Each format provides:
//!
//! - a typed document model that mirrors the standard's element vocabulary;
//! - `to_xml` / `from_xml` for the format itself (roundtrip-stable);
//! - conversion to and from this crate's [`crate::StatuteDiff`] /
//!   `legalis_core::Statute` where it is meaningful, so a statute or a diff can
//!   be exported to and re-imported from any of the three standards.
//!
//! Parsing uses the workspace `quick-xml` reader (correct entity / attribute
//! handling); emission uses a small, dependency-free indented writer in
//! [`writer`] so the produced documents are human-readable.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::legal_xml::akoma_ntoso::AkomaNtosoDocument;
//!
//! let statute = Statute::new(
//!     "act-42",
//!     "Senior Tax Credit Act",
//!     Effect::new(EffectType::Grant, "Tax credit granted"),
//! )
//! .with_precondition(Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 65,
//! });
//!
//! // Export to Akoma Ntoso and read it back.
//! let doc = AkomaNtosoDocument::from_statute(&statute);
//! let xml = doc.to_xml().unwrap();
//! assert!(xml.contains("<akomaNtoso"));
//!
//! let parsed = AkomaNtosoDocument::from_xml(&xml).unwrap();
//! let restored = parsed.to_statute().unwrap();
//! assert_eq!(restored.id, statute.id);
//! assert_eq!(restored.title, statute.title);
//! ```

pub mod akoma_ntoso;
pub mod legalruleml;
pub mod metalex;
pub(crate) mod writer;
pub(crate) mod xml_util;

pub use akoma_ntoso::{AknArticle, AknBody, AknMeta, AknSection, AkomaNtosoDocument};
pub use legalruleml::{DeonticKind, LegalRule, LegalRuleMlDocument, RuleAtom, RuleStatement};
pub use metalex::{MetalexDocument, MetalexExpression, MetalexFragment, MetalexWork};

use crate::DiffError;

/// Convenience constructor for an XML (de)serialization error.
pub(crate) fn xml_error(context: &str, detail: impl std::fmt::Display) -> DiffError {
    DiffError::SerializationError(format!("{context}: {detail}"))
}
