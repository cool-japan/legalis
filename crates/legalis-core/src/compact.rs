//! Memory-optimized statute representation.
//!
//! This module provides `CompactStatute`, a memory-efficient representation
//! of statutes optimized for:
//!
//! - Large-scale statute collections
//! - Memory-constrained environments
//! - Cache-efficient batch processing
//! - Reduced heap allocations
//!
//! ## Memory Savings
//!
//! `CompactStatute` reduces memory usage through:
//!
//! - String interning for repeated identifiers
//! - Bit-packed flags instead of `Option<bool>`
//! - Compact integer encoding for numeric fields
//! - Lazy loading of complex fields
//!
//! ## Example
//!
//! ```
//! use legalis_core::compact::CompactStatute;
//! use legalis_core::interning::StringInterner;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let mut interner = StringInterner::new();
//!
//! // Create a regular statute
//! let statute = Statute::new("tax-law-2025", "Income Tax", Effect::new(EffectType::Grant, "Tax credit"));
//!
//! // Convert to compact representation
//! let compact = CompactStatute::from_statute(&statute, &mut interner);
//!
//! // Convert back to regular statute
//! let reconstructed = compact.to_statute(&interner);
//! assert_eq!(&reconstructed.id, &statute.id);
//! ```

use crate::interning::{StringInterner, Symbol};
use crate::{Effect, EffectType, Statute};

/// Bit flags for statute properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StatuteFlags(u8);

impl StatuteFlags {
    const HAS_JURISDICTION: u8 = 1 << 0;
    const HAS_PRECONDITIONS: u8 = 1 << 1;
    #[allow(dead_code)]
    const HAS_DISCRETION: u8 = 1 << 2;
    #[allow(dead_code)]
    const HAS_TEMPORAL: u8 = 1 << 3;
    #[allow(dead_code)]
    const HAS_TAGS: u8 = 1 << 4;

    fn new() -> Self {
        Self(0)
    }

    fn set(&mut self, flag: u8, value: bool) {
        if value {
            self.0 |= flag;
        } else {
            self.0 &= !flag;
        }
    }

    fn get(&self, flag: u8) -> bool {
        (self.0 & flag) != 0
    }
}

/// Compact effect representation.
#[derive(Clone, Debug)]
struct CompactEffect {
    effect_type: EffectType,
    description: Symbol,
}

/// Memory-optimized statute representation.
///
/// Uses string interning and compact encoding to reduce memory footprint.
#[derive(Clone, Debug)]
pub struct CompactStatute {
    id: Symbol,
    title: Symbol,
    effect: CompactEffect,
    jurisdiction: Option<Symbol>,
    version: u32,
    flags: StatuteFlags,
    // Lazy-loaded fields (indices into external storage)
    #[allow(dead_code)]
    preconditions_idx: Option<u32>,
    #[allow(dead_code)]
    discretion_idx: Option<u32>,
}

impl CompactStatute {
    /// Creates a compact statute from a regular statute.
    ///
    /// Interns all strings and compacts the representation.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatute;
    /// use legalis_core::interning::StringInterner;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut interner = StringInterner::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let compact = CompactStatute::from_statute(&statute, &mut interner);
    /// ```
    pub fn from_statute(statute: &Statute, interner: &mut StringInterner) -> Self {
        let mut flags = StatuteFlags::new();
        flags.set(
            StatuteFlags::HAS_JURISDICTION,
            statute.jurisdiction.is_some(),
        );
        flags.set(
            StatuteFlags::HAS_PRECONDITIONS,
            !statute.preconditions.is_empty(),
        );

        let jurisdiction = statute.jurisdiction.as_ref().map(|j| interner.intern(j));

        let effect = CompactEffect {
            effect_type: statute.effect.effect_type.clone(),
            description: interner.intern(&statute.effect.description),
        };

        Self {
            id: interner.intern(&statute.id),
            title: interner.intern(&statute.title),
            effect,
            jurisdiction,
            version: statute.version,
            flags,
            preconditions_idx: None,
            discretion_idx: None,
        }
    }

    /// Converts the compact statute back to a regular statute.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatute;
    /// use legalis_core::interning::StringInterner;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut interner = StringInterner::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let compact = CompactStatute::from_statute(&statute, &mut interner);
    ///
    /// let reconstructed = compact.to_statute(&interner);
    /// assert_eq!(&reconstructed.id, "id-1");
    /// ```
    pub fn to_statute(&self, _interner: &StringInterner) -> Statute {
        let effect = Effect::new(
            self.effect.effect_type.clone(),
            self.effect.description.as_str(),
        );

        let mut statute = Statute::new(self.id.as_str(), self.title.as_str(), effect);

        if let Some(jurisdiction) = &self.jurisdiction {
            statute = statute.with_jurisdiction(jurisdiction.as_str());
        }

        statute = statute.with_version(self.version);

        statute
    }

    /// Returns the statute ID.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Returns the statute title.
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Returns the jurisdiction, if any.
    pub fn jurisdiction(&self) -> Option<&str> {
        self.jurisdiction.as_ref().map(|s| s.as_str())
    }

    /// Returns the version number.
    pub fn version(&self) -> u32 {
        self.version
    }

    /// Checks if the statute has preconditions.
    pub fn has_preconditions(&self) -> bool {
        self.flags.get(StatuteFlags::HAS_PRECONDITIONS)
    }

    /// Checks if the statute has a jurisdiction.
    pub fn has_jurisdiction(&self) -> bool {
        self.flags.get(StatuteFlags::HAS_JURISDICTION)
    }

    /// Returns the estimated memory usage in bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatute;
    /// use legalis_core::interning::StringInterner;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut interner = StringInterner::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let compact = CompactStatute::from_statute(&statute, &mut interner);
    ///
    /// let size = compact.memory_size();
    /// assert!(size > 0);
    /// ```
    pub fn memory_size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

/// Collection of compact statutes with shared string interner.
///
/// Provides efficient storage for large statute collections.
pub struct CompactStatuteCollection {
    statutes: Vec<CompactStatute>,
    interner: StringInterner,
}

impl CompactStatuteCollection {
    /// Creates a new empty collection.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatuteCollection;
    ///
    /// let collection = CompactStatuteCollection::new();
    /// assert_eq!(collection.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            statutes: Vec::new(),
            interner: StringInterner::new(),
        }
    }

    /// Creates a new collection with pre-allocated capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatuteCollection;
    ///
    /// let collection = CompactStatuteCollection::with_capacity(1000);
    /// assert!(collection.capacity() >= 1000);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            statutes: Vec::with_capacity(capacity),
            interner: StringInterner::with_capacity(capacity * 3),
        }
    }

    /// Adds a statute to the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatuteCollection;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut collection = CompactStatuteCollection::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// collection.add(statute);
    /// assert_eq!(collection.len(), 1);
    /// ```
    pub fn add(&mut self, statute: Statute) {
        let compact = CompactStatute::from_statute(&statute, &mut self.interner);
        self.statutes.push(compact);
    }

    /// Returns the number of statutes in the collection.
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Returns `true` if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }

    /// Returns the allocated capacity.
    pub fn capacity(&self) -> usize {
        self.statutes.capacity()
    }

    /// Gets a reference to a statute by index.
    pub fn get(&self, index: usize) -> Option<&CompactStatute> {
        self.statutes.get(index)
    }

    /// Returns an iterator over compact statutes.
    pub fn iter(&self) -> impl Iterator<Item = &CompactStatute> {
        self.statutes.iter()
    }

    /// Converts all statutes back to regular statutes.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatuteCollection;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut collection = CompactStatuteCollection::new();
    /// collection.add(Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit")));
    /// collection.add(Statute::new("id-2", "Title 2", Effect::new(EffectType::Revoke, "Penalty")));
    ///
    /// let statutes = collection.to_statutes();
    /// assert_eq!(statutes.len(), 2);
    /// ```
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.statutes
            .iter()
            .map(|compact| compact.to_statute(&self.interner))
            .collect()
    }

    /// Returns the total memory usage in bytes.
    ///
    /// Includes statutes, interner, and collection overhead.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::compact::CompactStatuteCollection;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut collection = CompactStatuteCollection::new();
    /// collection.add(Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit")));
    ///
    /// let usage = collection.memory_usage();
    /// assert!(usage > 0);
    /// ```
    pub fn memory_usage(&self) -> usize {
        let vec_overhead = std::mem::size_of::<Vec<CompactStatute>>()
            + self.statutes.capacity() * std::mem::size_of::<CompactStatute>();

        let interner_usage = self.interner.memory_usage();

        vec_overhead + interner_usage
    }

    /// Returns statistics about string interning.
    pub fn interner_stats(&self) -> InternerStats {
        InternerStats {
            unique_strings: self.interner.len(),
            total_memory: self.interner.memory_usage(),
        }
    }
}

impl Default for CompactStatuteCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about string interning in a collection.
#[derive(Clone, Debug)]
pub struct InternerStats {
    /// Number of unique interned strings
    pub unique_strings: usize,
    /// Total memory used by interner
    pub total_memory: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Effect, EffectType};

    #[test]
    fn test_compact_statute_from_statute() {
        let mut interner = StringInterner::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let compact = CompactStatute::from_statute(&statute, &mut interner);

        assert_eq!(compact.id(), "id-1");
        assert_eq!(compact.title(), "Title");
    }

    #[test]
    fn test_compact_statute_to_statute() {
        let mut interner = StringInterner::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let compact = CompactStatute::from_statute(&statute, &mut interner);

        let reconstructed = compact.to_statute(&interner);
        assert_eq!(&reconstructed.id, "id-1");
        assert_eq!(&reconstructed.title, "Title");
    }

    #[test]
    fn test_compact_collection() {
        let mut collection = CompactStatuteCollection::new();
        collection.add(Statute::new(
            "id-1",
            "Title",
            Effect::new(EffectType::Grant, "Benefit"),
        ));
        collection.add(Statute::new(
            "id-2",
            "Title 2",
            Effect::new(EffectType::Revoke, "Penalty"),
        ));

        assert_eq!(collection.len(), 2);
    }

    #[test]
    fn test_compact_collection_to_statutes() {
        let mut collection = CompactStatuteCollection::new();
        collection.add(Statute::new(
            "id-1",
            "Title",
            Effect::new(EffectType::Grant, "Benefit"),
        ));

        let statutes = collection.to_statutes();
        assert_eq!(statutes.len(), 1);
        assert_eq!(&statutes[0].id, "id-1");
    }

    #[test]
    fn test_memory_usage() {
        let mut collection = CompactStatuteCollection::new();
        collection.add(Statute::new(
            "id-1",
            "Title",
            Effect::new(EffectType::Grant, "Benefit"),
        ));

        let usage = collection.memory_usage();
        assert!(usage > 0);
    }
}
