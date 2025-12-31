//! String interning for repeated identifiers.
//!
//! This module provides string interning to reduce memory usage when dealing
//! with large statute collections that contain many repeated strings such as:
//!
//! - Statute IDs with common prefixes
//! - Jurisdiction codes
//! - Category/tag names
//! - Common attribute names
//!
//! ## Benefits
//!
//! - **Memory savings**: Single copy of each unique string
//! - **Fast equality checks**: O(1) pointer comparison
//! - **Cache-friendly**: Reduced memory footprint improves cache locality
//! - **Thread-safe**: Can be shared across threads with Arc
//!
//! ## Example
//!
//! ```
//! use legalis_core::interning::StringInterner;
//!
//! let mut interner = StringInterner::new();
//!
//! // Intern strings
//! let s1 = interner.intern("US");
//! let s2 = interner.intern("US");
//! let s3 = interner.intern("UK");
//!
//! // Same string returns same symbol
//! assert_eq!(s1, s2);
//! assert_ne!(s1, s3);
//!
//! // Statistics
//! assert_eq!(interner.len(), 2); // Only 2 unique strings
//! ```

use std::collections::HashMap;
use std::sync::Arc;

/// An interned string symbol.
///
/// Symbols are cheap to copy and compare (they're just Arc pointers).
/// Equality checks are O(1) pointer comparisons.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(Arc<str>);

impl Symbol {
    /// Returns the string value of this symbol.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// let sym = interner.intern("hello");
    /// assert_eq!(sym.as_str(), "hello");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// String interner for deduplicating repeated strings.
///
/// Provides efficient storage and comparison of strings by maintaining
/// a single copy of each unique string value.
pub struct StringInterner {
    map: HashMap<Arc<str>, Symbol>,
}

impl StringInterner {
    /// Creates a new empty string interner.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let interner = StringInterner::new();
    /// assert_eq!(interner.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Creates a new string interner with pre-allocated capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let interner = StringInterner::with_capacity(1000);
    /// assert!(interner.capacity() >= 1000);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }

    /// Interns a string, returning a symbol.
    ///
    /// If the string was already interned, returns the existing symbol.
    /// Otherwise, creates a new symbol and stores the string.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// let s1 = interner.intern("hello");
    /// let s2 = interner.intern("hello");
    ///
    /// // Same string, same symbol
    /// assert_eq!(s1, s2);
    /// assert_eq!(interner.len(), 1);
    /// ```
    pub fn intern(&mut self, s: &str) -> Symbol {
        if let Some(symbol) = self.map.get(s) {
            symbol.clone()
        } else {
            let arc: Arc<str> = Arc::from(s);
            let symbol = Symbol(arc.clone());
            self.map.insert(arc, symbol.clone());
            symbol
        }
    }

    /// Returns the number of unique interned strings.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// interner.intern("foo");
    /// interner.intern("bar");
    /// interner.intern("foo"); // Duplicate
    ///
    /// assert_eq!(interner.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if no strings have been interned.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let interner = StringInterner::new();
    /// assert!(interner.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the allocated capacity.
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// Clears the interner, removing all interned strings.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// interner.intern("foo");
    /// assert_eq!(interner.len(), 1);
    ///
    /// interner.clear();
    /// assert_eq!(interner.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Shrinks the interner's capacity to fit the current number of strings.
    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit();
    }

    /// Checks if a string has been interned.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// interner.intern("foo");
    ///
    /// assert!(interner.contains("foo"));
    /// assert!(!interner.contains("bar"));
    /// ```
    pub fn contains(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }

    /// Returns the estimated memory usage in bytes.
    ///
    /// This includes the map overhead and all interned strings.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::StringInterner;
    ///
    /// let mut interner = StringInterner::new();
    /// interner.intern("hello");
    /// interner.intern("world");
    ///
    /// let usage = interner.memory_usage();
    /// assert!(usage > 0);
    /// ```
    pub fn memory_usage(&self) -> usize {
        let map_overhead = std::mem::size_of::<HashMap<Arc<str>, Symbol>>()
            + self.map.capacity() * std::mem::size_of::<(Arc<str>, Symbol)>();

        let strings_size: usize = self.map.keys().map(|s| s.len()).sum();

        map_overhead + strings_size
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}

/// A symbol table for common legal string constants.
///
/// Pre-interns commonly used strings to avoid repeated interning overhead.
pub struct LegalSymbols {
    interner: StringInterner,
    // Common jurisdictions
    pub us: Symbol,
    pub uk: Symbol,
    pub eu: Symbol,
    pub ca: Symbol,
    // Common effect types (as strings)
    pub grant: Symbol,
    pub revoke: Symbol,
    pub obligation: Symbol,
    pub prohibition: Symbol,
}

impl LegalSymbols {
    /// Creates a new symbol table with common legal constants pre-interned.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::interning::LegalSymbols;
    ///
    /// let symbols = LegalSymbols::new();
    /// assert_eq!(symbols.us.as_str(), "US");
    /// assert_eq!(symbols.grant.as_str(), "Grant");
    /// ```
    pub fn new() -> Self {
        let mut interner = StringInterner::with_capacity(50);

        Self {
            us: interner.intern("US"),
            uk: interner.intern("UK"),
            eu: interner.intern("EU"),
            ca: interner.intern("CA"),
            grant: interner.intern("Grant"),
            revoke: interner.intern("Revoke"),
            obligation: interner.intern("Obligation"),
            prohibition: interner.intern("Prohibition"),
            interner,
        }
    }

    /// Gets the underlying interner for custom string interning.
    pub fn interner_mut(&mut self) -> &mut StringInterner {
        &mut self.interner
    }

    /// Gets a reference to the underlying interner.
    pub fn interner(&self) -> &StringInterner {
        &self.interner
    }
}

impl Default for LegalSymbols {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interner_new() {
        let interner = StringInterner::new();
        assert_eq!(interner.len(), 0);
        assert!(interner.is_empty());
    }

    #[test]
    fn test_interner_intern() {
        let mut interner = StringInterner::new();
        let s1 = interner.intern("hello");
        let s2 = interner.intern("hello");
        let s3 = interner.intern("world");

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn test_symbol_as_str() {
        let mut interner = StringInterner::new();
        let sym = interner.intern("test");
        assert_eq!(sym.as_str(), "test");
    }

    #[test]
    fn test_interner_clear() {
        let mut interner = StringInterner::new();
        interner.intern("foo");
        interner.intern("bar");
        assert_eq!(interner.len(), 2);

        interner.clear();
        assert_eq!(interner.len(), 0);
        assert!(interner.is_empty());
    }

    #[test]
    fn test_interner_contains() {
        let mut interner = StringInterner::new();
        interner.intern("foo");

        assert!(interner.contains("foo"));
        assert!(!interner.contains("bar"));
    }

    #[test]
    fn test_legal_symbols() {
        let symbols = LegalSymbols::new();
        assert_eq!(symbols.us.as_str(), "US");
        assert_eq!(symbols.uk.as_str(), "UK");
        assert_eq!(symbols.grant.as_str(), "Grant");
        assert_eq!(symbols.obligation.as_str(), "Obligation");
    }

    #[test]
    fn test_memory_usage() {
        let mut interner = StringInterner::new();
        interner.intern("hello");
        interner.intern("world");

        let usage = interner.memory_usage();
        assert!(usage > 0);
    }
}
