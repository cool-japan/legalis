//! Live (delta-driven) document format translation.
//!
//! [`LiveTranslator`] keeps a canonical document and a per-region translation
//! cache. When the document is edited via [`LiveTranslator::apply_change`] (or a
//! batch of changes), only the regions that actually changed are re-translated;
//! untouched regions are served from the cache. This gives near-constant work
//! per keystroke-sized edit regardless of document size, which is what makes the
//! translator suitable for "live" / interactive editing without any networking.
//!
//! The translator exports each region independently and concatenates the
//! per-region outputs in document order. This *segmented* output is what enables
//! the incremental behaviour: a change to region `k` only invalidates segment
//! `k`. For formats whose serialization is a simple concatenation of
//! per-statute renderings (the common case for the DSL-style targets), the
//! concatenated output equals a full export; for envelope formats it remains a
//! faithful, region-addressable rendering.

use super::{
    CanonicalDocument, DocumentChange, FormatPair, RegionDelta, RegionFingerprint, RegionId,
};
use crate::{ConversionReport, InteropResult, LegalConverter, LegalFormat};
use legalis_core::Statute;
use std::collections::HashMap;

/// A cached translation of a single region.
#[derive(Debug, Clone)]
struct RegionTranslation {
    /// Fingerprint of the source statute this translation was produced from.
    fingerprint: RegionFingerprint,
    /// Rendered output for this region in the target format.
    output: String,
    /// Per-region conversion report.
    report: ConversionReport,
}

/// Statistics describing the work done by the most recent translation pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TranslationStats {
    /// Number of regions (re-)translated this pass.
    pub retranslated: usize,
    /// Number of regions served from cache (skipped).
    pub cached: usize,
    /// Number of regions removed (cache entries evicted).
    pub removed: usize,
}

impl TranslationStats {
    /// Total regions considered (retranslated + cached).
    pub fn total(&self) -> usize {
        self.retranslated + self.cached
    }
}

/// Result of a live translation pass.
#[derive(Debug, Clone)]
pub struct LiveTranslation {
    /// Full concatenated output in the target format.
    pub output: String,
    /// Aggregate conversion report across all regions.
    pub report: ConversionReport,
    /// Region delta that drove this pass.
    pub delta: RegionDelta,
    /// Work statistics (cache hits/misses).
    pub stats: TranslationStats,
}

/// An incremental, delta-driven translator between two legal formats.
pub struct LiveTranslator {
    pair: FormatPair,
    converter: LegalConverter,
    document: CanonicalDocument,
    /// Per-region translation cache keyed by region id.
    cache: HashMap<RegionId, RegionTranslation>,
    /// Separator inserted between region outputs (defaults to a blank line).
    separator: String,
}

impl LiveTranslator {
    /// Creates a live translator for the given format pair, starting empty.
    pub fn new(from: LegalFormat, to: LegalFormat) -> Self {
        Self {
            pair: FormatPair::new(from, to),
            converter: LegalConverter::new(),
            document: CanonicalDocument::new(),
            cache: HashMap::new(),
            separator: "\n\n".to_string(),
        }
    }

    /// Creates a live translator seeded from an existing source document.
    ///
    /// The source is imported once into the canonical model; subsequent edits go
    /// through [`Self::apply_change`].
    ///
    /// # Errors
    /// Returns an error if the source cannot be imported.
    pub fn from_source(from: LegalFormat, to: LegalFormat, source: &str) -> InteropResult<Self> {
        let mut translator = Self::new(from, to);
        let mut converter = LegalConverter::new();
        let (statutes, _report) = converter.import(source, from)?;
        translator.document = CanonicalDocument::from_statutes(&statutes)?;
        Ok(translator)
    }

    /// Sets the separator string placed between per-region outputs.
    pub fn with_separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }

    /// The format pair this translator handles.
    pub fn format_pair(&self) -> FormatPair {
        self.pair
    }

    /// Read-only access to the current canonical document.
    pub fn document(&self) -> &CanonicalDocument {
        &self.document
    }

    /// Seeds the canonical document with a statute *without* re-translating.
    ///
    /// Intended for building up an initial document before the first
    /// [`Self::translate_all`] call (used by [`crate::realtime::views`] to mirror
    /// a canonical document into per-view translators).
    ///
    /// # Errors
    /// Returns an error if the statute cannot be fingerprinted.
    pub fn document_seed(&mut self, statute: Statute) -> InteropResult<()> {
        self.document.upsert(statute)?;
        Ok(())
    }

    /// Number of cached region translations.
    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    /// Translates a single statute into the target format, returning its output
    /// and report. Shared by the initial and incremental passes.
    fn translate_region(&mut self, statute: &Statute) -> InteropResult<(String, ConversionReport)> {
        let slice = std::slice::from_ref(statute);
        self.converter.export(slice, self.pair.to)
    }

    /// Performs a full translation of the current document, populating the cache.
    ///
    /// This is the cold-start path; afterwards prefer [`Self::apply_change`].
    ///
    /// # Errors
    /// Returns an error if any region fails to translate.
    pub fn translate_all(&mut self) -> InteropResult<LiveTranslation> {
        let region_ids: Vec<RegionId> = self
            .document
            .regions()
            .iter()
            .map(|r| r.id.clone())
            .collect();
        let mut stats = TranslationStats::default();

        // Evict cache entries for regions that no longer exist.
        let live: std::collections::HashSet<&RegionId> = region_ids.iter().collect();
        let before = self.cache.len();
        self.cache.retain(|id, _| live.contains(id));
        stats.removed = before.saturating_sub(self.cache.len());

        for id in &region_ids {
            let (statute, fp) = match self.document.region(id) {
                Some(region) => (region.statute.clone(), region.fingerprint),
                None => continue,
            };
            let fresh = self
                .cache
                .get(id)
                .map(|c| c.fingerprint == fp)
                .unwrap_or(false);
            if fresh {
                stats.cached += 1;
                continue;
            }
            let (output, report) = self.translate_region(&statute)?;
            self.cache.insert(
                id.clone(),
                RegionTranslation {
                    fingerprint: fp,
                    output,
                    report,
                },
            );
            stats.retranslated += 1;
        }

        let (output, report) = self.assemble(&region_ids)?;
        Ok(LiveTranslation {
            output,
            report,
            delta: RegionDelta {
                added: region_ids,
                ..Default::default()
            },
            stats,
        })
    }

    /// Applies a single edit and incrementally re-translates only the affected
    /// region(s).
    ///
    /// # Errors
    /// Returns an error if the change or a re-translation fails.
    pub fn apply_change(&mut self, change: DocumentChange) -> InteropResult<LiveTranslation> {
        self.apply_changes(std::slice::from_ref(&change))
    }

    /// Applies a batch of edits and re-translates only the regions that changed
    /// across the whole batch.
    ///
    /// The previous document snapshot is diffed against the post-edit document,
    /// so redundant edits (e.g. update then revert) correctly result in no
    /// re-translation work.
    ///
    /// # Errors
    /// Returns an error if any change or re-translation fails.
    pub fn apply_changes(&mut self, changes: &[DocumentChange]) -> InteropResult<LiveTranslation> {
        let previous = self.document.clone();
        for change in changes {
            change.apply_to(&mut self.document)?;
        }
        let delta = self.document.delta_from(&previous);

        let mut stats = TranslationStats::default();

        // Evict cache entries for removed regions.
        for id in &delta.removed {
            if self.cache.remove(id).is_some() {
                stats.removed += 1;
            }
        }

        // Re-translate only the touched regions (added + content-updated).
        for id in delta.touched() {
            let (statute, fp) = match self.document.region(&id) {
                Some(region) => (region.statute.clone(), region.fingerprint),
                None => continue,
            };
            // Skip if cache already matches this fingerprint (revert case).
            if self
                .cache
                .get(&id)
                .map(|c| c.fingerprint == fp)
                .unwrap_or(false)
            {
                stats.cached += 1;
                continue;
            }
            let (output, report) = self.translate_region(&statute)?;
            self.cache.insert(
                id.clone(),
                RegionTranslation {
                    fingerprint: fp,
                    output,
                    report,
                },
            );
            stats.retranslated += 1;
        }

        // Count untouched regions as cache hits for visibility.
        let touched: std::collections::HashSet<RegionId> = delta.touched().into_iter().collect();
        for region in self.document.regions() {
            if !touched.contains(&region.id) {
                stats.cached += 1;
            }
        }

        let region_ids: Vec<RegionId> = self
            .document
            .regions()
            .iter()
            .map(|r| r.id.clone())
            .collect();
        let (output, report) = self.assemble(&region_ids)?;
        Ok(LiveTranslation {
            output,
            report,
            delta,
            stats,
        })
    }

    /// Assembles the full output from the per-region cache, in document order.
    ///
    /// Any region missing from the cache is translated on demand (defensive: the
    /// public entry points keep the cache complete, but this keeps `assemble`
    /// total).
    ///
    /// # Errors
    /// Returns an error if an on-demand translation fails.
    fn assemble(&mut self, region_ids: &[RegionId]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(self.pair.from, self.pair.to);
        report.statutes_converted = 0;
        report.confidence = 1.0;
        let mut segments: Vec<String> = Vec::with_capacity(region_ids.len());

        for id in region_ids {
            let cached = match self.cache.get(id) {
                Some(c) => c.clone(),
                None => {
                    let statute = match self.document.region(id) {
                        Some(region) => region.statute.clone(),
                        None => continue,
                    };
                    let fp = self.document.fingerprint_of(id).unwrap_or([0u8; 32]);
                    let (output, rep) = self.translate_region(&statute)?;
                    let entry = RegionTranslation {
                        fingerprint: fp,
                        output,
                        report: rep,
                    };
                    self.cache.insert(id.clone(), entry.clone());
                    entry
                }
            };
            segments.push(cached.output.clone());
            report.statutes_converted += cached.report.statutes_converted.max(1);
            report
                .unsupported_features
                .extend(cached.report.unsupported_features.clone());
            report.warnings.extend(cached.report.warnings.clone());
            report.confidence = (report.confidence * cached.report.confidence).max(0.0);
        }

        Ok((segments.join(&self.separator), report))
    }

    /// Clears the cache and the document, returning to the cold-start state.
    pub fn reset(&mut self) {
        self.cache.clear();
        self.document = CanonicalDocument::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::realtime::DocumentChange;
    use legalis_core::{Effect, EffectType, Statute};

    fn statute(id: &str, title: &str, desc: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, desc))
    }

    #[test]
    fn cold_start_translates_everything() {
        let mut t = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::L4);
        for s in [statute("a", "A", "x"), statute("b", "B", "y")] {
            t.document_seed(s).expect("seed");
        }
        let pass = t.translate_all().expect("translate all");
        assert_eq!(pass.stats.retranslated, 2);
        assert_eq!(pass.stats.cached, 0);
        assert!(!pass.output.is_empty());
        assert_eq!(t.cache_len(), 2);
    }

    #[test]
    fn editing_one_region_retranslates_only_that_region() {
        let mut t = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::L4);
        for s in [
            statute("a", "A", "x"),
            statute("b", "B", "y"),
            statute("c", "C", "z"),
        ] {
            t.document_seed(s).expect("seed");
        }
        let _ = t.translate_all().expect("seed");

        // Edit only region "b".
        let pass = t
            .apply_change(DocumentChange::update(statute("b", "B", "changed")))
            .expect("apply");
        assert_eq!(pass.stats.retranslated, 1, "only one region re-translated");
        assert_eq!(pass.delta.updated, vec!["b".to_string()]);
        assert_eq!(pass.stats.cached, 2, "two regions served from cache");
    }

    #[test]
    fn revert_edit_does_no_retranslation() {
        let mut t = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::L4);
        t.document_seed(statute("a", "A", "x")).expect("seed");
        let _ = t.translate_all().expect("seed");

        // Update then revert in the same batch -> net no content change.
        let pass = t
            .apply_changes(&[
                DocumentChange::update(statute("a", "A", "y")),
                DocumentChange::update(statute("a", "A", "x")),
            ])
            .expect("batch");
        assert_eq!(pass.stats.retranslated, 0, "net no change => no work");
        assert!(pass.delta.is_empty());
    }

    #[test]
    fn removing_region_evicts_cache_and_shrinks_output() {
        let mut t = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::L4);
        for s in [statute("a", "A", "x"), statute("b", "B", "y")] {
            t.document_seed(s).expect("seed");
        }
        let full = t.translate_all().expect("seed");
        let full_len = full.output.len();

        let pass = t.apply_change(DocumentChange::remove("b")).expect("remove");
        assert_eq!(pass.stats.removed, 1);
        assert_eq!(pass.delta.removed, vec!["b".to_string()]);
        assert!(pass.output.len() < full_len);
        assert_eq!(t.cache_len(), 1);
    }

    #[test]
    fn incremental_output_equals_full_retranslation() {
        // Property: after a sequence of edits, the incrementally maintained
        // output must equal a from-scratch translation of the same document.
        let mut t = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::L4);
        for s in [statute("a", "A", "x"), statute("b", "B", "y")] {
            t.document_seed(s).expect("seed");
        }
        let _ = t.translate_all().expect("seed");
        t.apply_change(DocumentChange::append(statute("c", "C", "z")))
            .expect("add c");
        let incremental = t
            .apply_change(DocumentChange::update(statute("a", "A", "x2")))
            .expect("edit a");

        // Build a fresh translator with the same final document.
        let mut fresh = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::L4);
        for region in t.document().regions() {
            fresh.document_seed(region.statute.clone()).expect("seed");
        }
        let from_scratch = fresh.translate_all().expect("fresh");
        assert_eq!(incremental.output, from_scratch.output);
    }

    #[test]
    fn seeded_document_translates_to_target_format() {
        let mut t = LiveTranslator::new(LegalFormat::Legalis, LegalFormat::Catala);
        t.document_seed(statute("a", "A", "x")).expect("seed");
        let pass = t.translate_all().expect("translate");
        assert!(!pass.output.is_empty());
        assert_eq!(pass.stats.retranslated, 1);
    }

    #[test]
    fn from_source_imports_and_seeds() {
        // Build a source via the converter, then seed a translator from it. The
        // exact re-imported statute count depends on the source format's
        // round-trip fidelity, so we assert the document is non-empty and that
        // every imported region is translated (not a fixed count).
        let mut c = LegalConverter::new();
        let (source, _r) = c
            .export(
                &[statute("a", "A", "x"), statute("b", "B", "y")],
                LegalFormat::L4,
            )
            .expect("export l4");
        let mut t = LiveTranslator::from_source(LegalFormat::L4, LegalFormat::Catala, &source)
            .expect("from_source");
        let imported = t.document().len();
        assert!(imported >= 1, "import produced at least one region");
        let pass = t.translate_all().expect("translate");
        assert!(!pass.output.is_empty());
        assert_eq!(pass.stats.retranslated, imported);
    }
}
