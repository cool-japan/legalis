//! Multi-format document views.
//!
//! [`MultiFormatView`] projects a single canonical legal document into several
//! *simultaneous* format views (e.g. Catala + L4 + Akoma Ntoso) that are kept
//! mutually consistent: every view is derived from the same canonical source, so
//! any edit applied through the view manager is reflected, after a single
//! refresh, in *all* views at once.
//!
//! Each view is refreshed incrementally: a per-view cache keyed by region
//! fingerprint means an edit that touches one region only re-exports that region
//! in each view, not the whole document. This makes maintaining many views cheap
//! even for large documents. Views can be added or removed at runtime.
//!
//! Consistency property (verified by tests): for every registered view, the
//! materialised view text equals a from-scratch export of the canonical
//! document into that view's format, after any sequence of edits.

use super::live_translate::LiveTranslator;
use super::{CanonicalDocument, ChangeKind, DocumentChange, RegionDelta};
use crate::{ConversionReport, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use std::collections::BTreeMap;

/// A single rendered view in one target format.
#[derive(Debug, Clone)]
pub struct FormatView {
    /// The format this view renders to.
    pub format: LegalFormat,
    /// The current rendered text.
    pub text: String,
    /// The conversion report for the most recent render.
    pub report: ConversionReport,
}

/// The result of refreshing all views after an edit.
#[derive(Debug, Clone)]
pub struct ViewRefresh {
    /// The delta that triggered the refresh.
    pub delta: RegionDelta,
    /// Total region (re-)renders performed across all views.
    pub rerenders: usize,
}

/// Manages a canonical document and a set of simultaneous format views.
pub struct MultiFormatView {
    /// Canonical source of truth (format-neutral).
    document: CanonicalDocument,
    /// One incremental translator per registered view, keyed by format.
    ///
    /// The canonical document is exported "from itself" — the source format of
    /// each translator is irrelevant for export, so we use the view format as
    /// both sides and never call its importer.
    views: BTreeMap<LegalFormat, LiveTranslator>,
    /// Source format label for reporting (the canonical document's logical
    /// origin format). Defaults to [`LegalFormat::Legalis`].
    source_format: LegalFormat,
}

impl MultiFormatView {
    /// Creates an empty multi-view manager with the given canonical origin
    /// format label (used only for reporting).
    pub fn new(source_format: LegalFormat) -> Self {
        Self {
            document: CanonicalDocument::new(),
            views: BTreeMap::new(),
            source_format,
        }
    }

    /// Creates a multi-view manager seeded from a slice of statutes.
    ///
    /// # Errors
    /// Returns an error if any statute cannot be fingerprinted.
    pub fn from_statutes(source_format: LegalFormat, statutes: &[Statute]) -> InteropResult<Self> {
        let mut mv = Self::new(source_format);
        mv.document = CanonicalDocument::from_statutes(statutes)?;
        Ok(mv)
    }

    /// Creates a multi-view manager seeded by importing `source` in `format`.
    ///
    /// # Errors
    /// Returns an error if the source cannot be imported.
    pub fn from_source(format: LegalFormat, source: &str) -> InteropResult<Self> {
        let mut converter = LegalConverter::new();
        let (statutes, _report) = converter.import(source, format)?;
        Self::from_statutes(format, &statutes)
    }

    /// Read-only access to the canonical document.
    pub fn document(&self) -> &CanonicalDocument {
        &self.document
    }

    /// Number of registered views.
    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    /// The set of registered view formats, in deterministic order.
    pub fn formats(&self) -> Vec<LegalFormat> {
        self.views.keys().copied().collect()
    }

    /// Registers a new view in `format` and renders it immediately.
    ///
    /// If a view for that format already exists it is re-rendered. Returns the
    /// freshly-rendered view.
    ///
    /// # Errors
    /// Returns an error if rendering fails.
    pub fn add_view(&mut self, format: LegalFormat) -> InteropResult<FormatView> {
        let mut translator = LiveTranslator::new(self.source_format, format);
        // Seed the translator's document from the canonical document.
        for region in self.document.regions() {
            translator
                .document_seed(region.statute.clone())
                .map_err(|e| crate::InteropError::ConversionError(e.to_string()))?;
        }
        let pass = translator.translate_all()?;
        let view = FormatView {
            format,
            text: pass.output,
            report: pass.report,
        };
        self.views.insert(format, translator);
        Ok(view)
    }

    /// Removes a view; returns `true` if it was present.
    pub fn remove_view(&mut self, format: LegalFormat) -> bool {
        self.views.remove(&format).is_some()
    }

    /// Returns the current rendered text and report for a view, if registered.
    ///
    /// # Errors
    /// Returns an error if a (defensive) re-render is required and fails.
    pub fn view(&mut self, format: LegalFormat) -> InteropResult<Option<FormatView>> {
        let Some(translator) = self.views.get_mut(&format) else {
            return Ok(None);
        };
        let pass = translator.translate_all()?;
        Ok(Some(FormatView {
            format,
            text: pass.output,
            report: pass.report,
        }))
    }

    /// Materialises all registered views in deterministic format order.
    ///
    /// # Errors
    /// Returns an error if any view fails to render.
    pub fn all_views(&mut self) -> InteropResult<Vec<FormatView>> {
        let formats: Vec<LegalFormat> = self.views.keys().copied().collect();
        let mut out = Vec::with_capacity(formats.len());
        for format in formats {
            if let Some(translator) = self.views.get_mut(&format) {
                let pass = translator.translate_all()?;
                out.push(FormatView {
                    format,
                    text: pass.output,
                    report: pass.report,
                });
            }
        }
        Ok(out)
    }

    /// Applies an edit to the canonical document and refreshes every view
    /// incrementally (only changed regions are re-rendered in each view).
    ///
    /// # Errors
    /// Returns an error if the change or any re-render fails.
    pub fn apply_change(&mut self, change: DocumentChange) -> InteropResult<ViewRefresh> {
        self.apply_changes(std::slice::from_ref(&change))
    }

    /// Applies a batch of edits to the canonical document and refreshes all
    /// views, returning the propagated delta and total re-render count.
    ///
    /// # Errors
    /// Returns an error if any change or re-render fails.
    pub fn apply_changes(&mut self, changes: &[DocumentChange]) -> InteropResult<ViewRefresh> {
        let previous = self.document.clone();
        for change in changes {
            change.apply_to(&mut self.document)?;
        }
        let delta = self.document.delta_from(&previous);

        let mut rerenders = 0;
        let formats: Vec<LegalFormat> = self.views.keys().copied().collect();
        for format in formats {
            if let Some(translator) = self.views.get_mut(&format) {
                // Mirror the same edits into each view's translator document.
                let pass = translator.apply_changes(changes)?;
                rerenders += pass.stats.retranslated;
            }
        }
        Ok(ViewRefresh { delta, rerenders })
    }

    /// Convenience: upsert a statute into the canonical document and refresh.
    ///
    /// # Errors
    /// Returns an error if the change or refresh fails.
    pub fn upsert(&mut self, statute: Statute) -> InteropResult<ViewRefresh> {
        self.apply_change(DocumentChange::update(statute))
    }

    /// Convenience: remove a region by id and refresh.
    ///
    /// # Errors
    /// Returns an error if the refresh fails.
    pub fn remove_region(&mut self, id: impl Into<String>) -> InteropResult<ViewRefresh> {
        self.apply_change(DocumentChange::remove(id))
    }

    /// Classifies what a single canonical upsert would do without applying it.
    ///
    /// # Errors
    /// Returns an error if the statute cannot be fingerprinted.
    pub fn classify_upsert(&self, statute: &Statute) -> InteropResult<ChangeKind> {
        let mut probe = self.document.clone();
        probe.upsert(statute.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType, Statute};

    fn statute(id: &str, title: &str, desc: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, desc))
    }

    fn make_view() -> MultiFormatView {
        MultiFormatView::from_statutes(
            LegalFormat::Legalis,
            &[statute("a", "A", "x"), statute("b", "B", "y")],
        )
        .expect("make view")
    }

    #[test]
    fn add_views_renders_each_format() {
        let mut mv = make_view();
        let v_l4 = mv.add_view(LegalFormat::L4).expect("l4");
        let v_catala = mv.add_view(LegalFormat::Catala).expect("catala");
        assert_eq!(v_l4.format, LegalFormat::L4);
        assert_eq!(v_catala.format, LegalFormat::Catala);
        assert!(!v_l4.text.is_empty());
        assert!(!v_catala.text.is_empty());
        assert_eq!(mv.view_count(), 2);
    }

    #[test]
    fn views_stay_consistent_with_canonical_after_edit() {
        // Consistency property: every view equals a from-scratch export of the
        // canonical document into that format.
        let mut mv = make_view();
        mv.add_view(LegalFormat::L4).expect("l4");
        mv.add_view(LegalFormat::Catala).expect("catala");

        // Edit: change region "a" and add region "c".
        mv.apply_changes(&[
            DocumentChange::update(statute("a", "A", "changed")),
            DocumentChange::append(statute("c", "C", "z")),
        ])
        .expect("edit");

        let canonical_statutes = mv.document().statutes();
        for format in [LegalFormat::L4, LegalFormat::Catala] {
            let view = mv.view(format).expect("view").expect("present");
            // Compare against a fresh full export of the canonical document.
            let mut translator = LiveTranslator::new(LegalFormat::Legalis, format);
            for s in &canonical_statutes {
                translator.document_seed(s.clone()).expect("seed");
            }
            let fresh = translator.translate_all().expect("fresh").output;
            assert_eq!(view.text, fresh, "view {format:?} inconsistent");
        }
    }

    #[test]
    fn edit_one_region_rerenders_minimally_per_view() {
        let mut mv = make_view(); // 2 regions
        mv.add_view(LegalFormat::L4).expect("l4");
        mv.add_view(LegalFormat::Catala).expect("catala");

        // Edit a single region -> 1 re-render per view -> 2 total.
        let refresh = mv
            .apply_change(DocumentChange::update(statute("a", "A", "changed")))
            .expect("edit");
        assert_eq!(refresh.delta.updated, vec!["a".to_string()]);
        assert_eq!(refresh.rerenders, 2, "one re-render per view");
    }

    #[test]
    fn remove_view_then_absent() {
        let mut mv = make_view();
        mv.add_view(LegalFormat::L4).expect("l4");
        assert!(mv.remove_view(LegalFormat::L4));
        assert!(!mv.remove_view(LegalFormat::L4));
        assert!(mv.view(LegalFormat::L4).expect("view").is_none());
    }

    #[test]
    fn add_view_after_edits_matches_others() {
        // A view added late must immediately match the canonical state.
        let mut mv = make_view();
        mv.add_view(LegalFormat::L4).expect("l4");
        mv.apply_change(DocumentChange::append(statute("c", "C", "z")))
            .expect("edit");

        let late = mv.add_view(LegalFormat::Catala).expect("late catala");
        // Build the expected fresh export.
        let mut translator = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::Catala);
        for s in &mv.document().statutes() {
            translator.document_seed(s.clone()).expect("seed");
        }
        let expected = translator.translate_all().expect("fresh").output;
        assert_eq!(late.text, expected);
    }

    #[test]
    fn classify_upsert_predicts_change_kind() {
        let mv = make_view();
        assert_eq!(
            mv.classify_upsert(&statute("a", "A", "x")).expect("noop"),
            ChangeKind::Unchanged
        );
        assert_eq!(
            mv.classify_upsert(&statute("a", "A", "y")).expect("upd"),
            ChangeKind::Updated
        );
        assert_eq!(
            mv.classify_upsert(&statute("z", "Z", "x")).expect("ins"),
            ChangeKind::Inserted
        );
    }
}
