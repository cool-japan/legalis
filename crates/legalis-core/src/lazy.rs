//! Lazy loading for statute preconditions and effects.
//!
//! This module provides lazy evaluation of statute components to improve
//! memory usage and startup time when working with large statute collections.
//!
//! ## Use Cases
//!
//! - **Large statute databases**: Load metadata quickly, defer details
//! - **Search and filter**: Find statutes without loading full content
//! - **Memory-constrained systems**: Load components on-demand
//! - **Network-backed storage**: Fetch data only when needed
//!
//! ## Benefits
//!
//! - **Faster startup**: Load statute metadata first, details later
//! - **Lower memory**: Only loaded components consume memory
//! - **Network efficiency**: Fetch only what's needed
//! - **Cache-friendly**: Load hot data, leave cold data unloaded
//!
//! ## Example
//!
//! ```
//! use legalis_core::lazy::{LazyStatute, StatuteLoader};
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use std::sync::Arc;
//!
//! // Create a simple loader
//! let mut loader = StatuteLoader::new();
//!
//! // Register a statute
//! let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"))
//!     .with_precondition(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 });
//! loader.register("id-1", statute);
//!
//! // Create lazy statute
//! let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
//!
//! // Preconditions loaded on first access
//! let preconditions = lazy.preconditions();
//! assert!(preconditions.is_some());
//! ```

use crate::{Condition, Effect, Statute};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for loading statute components on-demand.
///
/// Implement this trait to provide custom loading logic, such as
/// loading from a database, network, or file system.
pub trait Loader: Send + Sync {
    /// Loads the full statute by ID.
    fn load_statute(&self, id: &str) -> Option<Statute>;

    /// Loads preconditions for a statute.
    fn load_preconditions(&self, id: &str) -> Option<Vec<Condition>>;

    /// Loads the effect for a statute.
    fn load_effect(&self, id: &str) -> Option<Effect>;
}

/// In-memory statute loader for testing and simple use cases.
pub struct StatuteLoader {
    statutes: HashMap<String, Statute>,
}

impl StatuteLoader {
    /// Creates a new empty loader.
    pub fn new() -> Self {
        Self {
            statutes: HashMap::new(),
        }
    }

    /// Registers a statute in the loader.
    pub fn register(&mut self, id: impl Into<String>, statute: Statute) {
        self.statutes.insert(id.into(), statute);
    }

    /// Removes a statute from the loader.
    pub fn unregister(&mut self, id: &str) -> Option<Statute> {
        self.statutes.remove(id)
    }

    /// Returns the number of registered statutes.
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Returns `true` if no statutes are registered.
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }
}

impl Default for StatuteLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl Loader for StatuteLoader {
    fn load_statute(&self, id: &str) -> Option<Statute> {
        self.statutes.get(id).cloned()
    }

    fn load_preconditions(&self, id: &str) -> Option<Vec<Condition>> {
        self.statutes.get(id).map(|s| s.preconditions.clone())
    }

    fn load_effect(&self, id: &str) -> Option<Effect> {
        self.statutes.get(id).map(|s| s.effect.clone())
    }
}

/// Lazy-loading statute wrapper.
///
/// Loads statute components on-demand to reduce memory usage
/// and improve startup time.
pub struct LazyStatute {
    id: String,
    title: String,
    loader: Arc<dyn Loader>,
    // Cached components
    preconditions: RefCell<Option<Option<Vec<Condition>>>>,
    effect: RefCell<Option<Option<Effect>>>,
    full_statute: RefCell<Option<Option<Statute>>>,
}

impl LazyStatute {
    /// Creates a new lazy statute.
    ///
    /// Only the ID and title are stored initially. Other components
    /// are loaded on first access.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::lazy::{LazyStatute, StatuteLoader};
    /// use std::sync::Arc;
    ///
    /// let loader = StatuteLoader::new();
    /// let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
    /// assert_eq!(lazy.id(), "id-1");
    /// ```
    pub fn new(id: impl Into<String>, title: impl Into<String>, loader: Arc<dyn Loader>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            loader,
            preconditions: RefCell::new(None),
            effect: RefCell::new(None),
            full_statute: RefCell::new(None),
        }
    }

    /// Returns the statute ID.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the statute title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Loads and returns the preconditions, if any.
    ///
    /// Caches the result for subsequent calls.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::lazy::{LazyStatute, StatuteLoader};
    /// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
    /// use std::sync::Arc;
    ///
    /// let mut loader = StatuteLoader::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"))
    ///     .with_precondition(Condition::Age { operator: ComparisonOp::GreaterOrEqual, value: 18 });
    /// loader.register("id-1", statute);
    ///
    /// let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
    /// let preconditions = lazy.preconditions();
    /// assert!(preconditions.is_some());
    /// ```
    pub fn preconditions(&self) -> Option<Vec<Condition>> {
        let mut cache = self.preconditions.borrow_mut();
        if cache.is_none() {
            *cache = Some(self.loader.load_preconditions(&self.id));
        }
        cache.as_ref().and_then(|opt| opt.clone())
    }

    /// Loads and returns the effect.
    ///
    /// Caches the result for subsequent calls.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::lazy::{LazyStatute, StatuteLoader};
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use std::sync::Arc;
    ///
    /// let mut loader = StatuteLoader::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// loader.register("id-1", statute);
    ///
    /// let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
    /// let effect = lazy.effect();
    /// assert!(effect.is_some());
    /// ```
    pub fn effect(&self) -> Option<Effect> {
        let mut cache = self.effect.borrow_mut();
        if cache.is_none() {
            *cache = Some(self.loader.load_effect(&self.id));
        }
        cache.as_ref().and_then(|opt| opt.clone())
    }

    /// Loads and returns the full statute.
    ///
    /// Caches the result for subsequent calls.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::lazy::{LazyStatute, StatuteLoader};
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use std::sync::Arc;
    ///
    /// let mut loader = StatuteLoader::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// loader.register("id-1", statute);
    ///
    /// let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
    /// let full = lazy.full_statute();
    /// assert!(full.is_some());
    /// ```
    pub fn full_statute(&self) -> Option<Statute> {
        let mut cache = self.full_statute.borrow_mut();
        if cache.is_none() {
            *cache = Some(self.loader.load_statute(&self.id));
        }
        cache.as_ref().and_then(|opt| opt.clone())
    }

    /// Checks if preconditions have been loaded.
    pub fn are_preconditions_loaded(&self) -> bool {
        self.preconditions.borrow().is_some()
    }

    /// Checks if the effect has been loaded.
    pub fn is_effect_loaded(&self) -> bool {
        self.effect.borrow().is_some()
    }

    /// Checks if the full statute has been loaded.
    pub fn is_full_statute_loaded(&self) -> bool {
        self.full_statute.borrow().is_some()
    }

    /// Clears all cached data, forcing reload on next access.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::lazy::{LazyStatute, StatuteLoader};
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use std::sync::Arc;
    ///
    /// let mut loader = StatuteLoader::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// loader.register("id-1", statute);
    ///
    /// let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
    /// let _ = lazy.effect(); // Load
    /// assert!(lazy.is_effect_loaded());
    ///
    /// lazy.clear_cache();
    /// assert!(!lazy.is_effect_loaded());
    /// ```
    pub fn clear_cache(&self) {
        *self.preconditions.borrow_mut() = None;
        *self.effect.borrow_mut() = None;
        *self.full_statute.borrow_mut() = None;
    }
}

/// Collection of lazy statutes with shared loader.
pub struct LazyStatuteCollection {
    statutes: Vec<LazyStatute>,
    loader: Arc<dyn Loader>,
}

impl LazyStatuteCollection {
    /// Creates a new collection with the given loader.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::lazy::{LazyStatuteCollection, StatuteLoader};
    /// use std::sync::Arc;
    ///
    /// let loader = StatuteLoader::new();
    /// let collection = LazyStatuteCollection::new(Arc::new(loader));
    /// assert_eq!(collection.len(), 0);
    /// ```
    pub fn new(loader: Arc<dyn Loader>) -> Self {
        Self {
            statutes: Vec::new(),
            loader,
        }
    }

    /// Adds a lazy statute to the collection.
    pub fn add(&mut self, id: impl Into<String>, title: impl Into<String>) {
        let lazy = LazyStatute::new(id, title, self.loader.clone());
        self.statutes.push(lazy);
    }

    /// Returns the number of statutes in the collection.
    pub fn len(&self) -> usize {
        self.statutes.len()
    }

    /// Returns `true` if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }

    /// Gets a reference to a lazy statute by index.
    pub fn get(&self, index: usize) -> Option<&LazyStatute> {
        self.statutes.get(index)
    }

    /// Returns an iterator over lazy statutes.
    pub fn iter(&self) -> impl Iterator<Item = &LazyStatute> {
        self.statutes.iter()
    }

    /// Clears all caches in the collection.
    pub fn clear_caches(&self) {
        for statute in &self.statutes {
            statute.clear_cache();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_statute_loader() {
        let mut loader = StatuteLoader::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        loader.register("id-1", statute.clone());

        assert_eq!(loader.len(), 1);
        assert!(loader.load_statute("id-1").is_some());
    }

    #[test]
    fn test_lazy_statute_id() {
        let loader = StatuteLoader::new();
        let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
        assert_eq!(lazy.id(), "id-1");
        assert_eq!(lazy.title(), "Title");
    }

    #[test]
    fn test_lazy_statute_preconditions() {
        let mut loader = StatuteLoader::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });
        loader.register("id-1", statute);

        let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
        assert!(!lazy.are_preconditions_loaded());

        let preconditions = lazy.preconditions();
        assert!(preconditions.is_some());
        assert!(lazy.are_preconditions_loaded());
    }

    #[test]
    fn test_lazy_statute_effect() {
        let mut loader = StatuteLoader::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        loader.register("id-1", statute);

        let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
        assert!(!lazy.is_effect_loaded());

        let effect = lazy.effect();
        assert!(effect.is_some());
        assert!(lazy.is_effect_loaded());
    }

    #[test]
    fn test_lazy_statute_clear_cache() {
        let mut loader = StatuteLoader::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        loader.register("id-1", statute);

        let lazy = LazyStatute::new("id-1", "Title", Arc::new(loader));
        let _ = lazy.effect();
        assert!(lazy.is_effect_loaded());

        lazy.clear_cache();
        assert!(!lazy.is_effect_loaded());
    }

    #[test]
    fn test_lazy_collection() {
        let loader = StatuteLoader::new();
        let mut collection = LazyStatuteCollection::new(Arc::new(loader));

        collection.add("id-1", "Title 1");
        collection.add("id-2", "Title 2");

        assert_eq!(collection.len(), 2);
    }
}
