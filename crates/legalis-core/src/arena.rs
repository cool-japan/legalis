//! Arena allocator for bulk statute operations.
//!
//! This module provides an arena allocator optimized for scenarios where many
//! statutes are created in bulk and then deallocated together. This pattern
//! is common when:
//!
//! - Loading large statute collections from disk
//! - Batch processing legal rules
//! - Temporary evaluation contexts
//! - Import/export operations
//!
//! ## Benefits
//!
//! - **Fast allocation**: O(1) bump pointer allocation
//! - **Reduced fragmentation**: Contiguous memory layout
//! - **Batch deallocation**: Free all at once when arena is dropped
//! - **Cache locality**: Better CPU cache performance
//!
//! ## Example
//!
//! ```
//! use legalis_core::arena::StatuteArena;
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let mut arena = StatuteArena::new();
//!
//! // Allocate many statutes efficiently
//! for i in 0..1000 {
//!     let statute = Statute::new(
//!         format!("statute-{}", i),
//!         format!("Title {}", i),
//!         Effect::new(EffectType::Grant, "Benefit")
//!     );
//!     arena.alloc(statute);
//! }
//!
//! // Access statutes
//! assert_eq!(arena.len(), 1000);
//!
//! // All statutes are freed when arena is dropped
//! ```

use crate::Statute;
use std::cell::RefCell;

/// Arena allocator for bulk statute operations.
///
/// Provides efficient allocation and deallocation of statutes in bulk.
/// All statutes are freed when the arena is dropped.
pub struct StatuteArena {
    statutes: RefCell<Vec<Statute>>,
}

impl StatuteArena {
    /// Creates a new empty arena.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    ///
    /// let arena = StatuteArena::new();
    /// assert_eq!(arena.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            statutes: RefCell::new(Vec::new()),
        }
    }

    /// Creates a new arena with pre-allocated capacity.
    ///
    /// This can improve performance when the number of statutes is known in advance.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    ///
    /// let arena = StatuteArena::with_capacity(1000);
    /// assert_eq!(arena.capacity(), 1000);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            statutes: RefCell::new(Vec::with_capacity(capacity)),
        }
    }

    /// Allocates a statute in the arena.
    ///
    /// Returns the index where the statute was stored.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut arena = StatuteArena::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let idx = arena.alloc(statute);
    /// assert_eq!(idx, 0);
    /// ```
    pub fn alloc(&mut self, statute: Statute) -> usize {
        let mut statutes = self.statutes.borrow_mut();
        let idx = statutes.len();
        statutes.push(statute);
        idx
    }

    /// Gets a reference to a statute by index.
    ///
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut arena = StatuteArena::new();
    /// let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let idx = arena.alloc(statute);
    ///
    /// let retrieved = arena.get(idx).unwrap();
    /// assert_eq!(&retrieved.id, "id-1");
    /// ```
    pub fn get(&self, index: usize) -> Option<std::cell::Ref<'_, Statute>> {
        let statutes = self.statutes.borrow();
        if index < statutes.len() {
            Some(std::cell::Ref::map(statutes, |s| &s[index]))
        } else {
            None
        }
    }

    /// Gets a mutable reference to a statute by index.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Statute> {
        self.statutes.get_mut().get_mut(index)
    }

    /// Returns the number of statutes in the arena.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut arena = StatuteArena::new();
    /// assert_eq!(arena.len(), 0);
    ///
    /// arena.alloc(Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit")));
    /// assert_eq!(arena.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.statutes.borrow().len()
    }

    /// Returns `true` if the arena is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    ///
    /// let arena = StatuteArena::new();
    /// assert!(arena.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.statutes.borrow().is_empty()
    }

    /// Returns the allocated capacity of the arena.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    ///
    /// let arena = StatuteArena::with_capacity(100);
    /// assert!(arena.capacity() >= 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.statutes.borrow().capacity()
    }

    /// Clears the arena, removing all statutes.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::arena::StatuteArena;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut arena = StatuteArena::new();
    /// arena.alloc(Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit")));
    /// assert_eq!(arena.len(), 1);
    ///
    /// arena.clear();
    /// assert_eq!(arena.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.statutes.borrow_mut().clear();
    }

    /// Returns an iterator over all statutes in the arena.
    pub fn iter(&self) -> impl Iterator<Item = std::cell::Ref<'_, Statute>> {
        (0..self.len()).filter_map(move |i| self.get(i))
    }

    /// Shrinks the arena's capacity to fit the current number of statutes.
    pub fn shrink_to_fit(&mut self) {
        self.statutes.borrow_mut().shrink_to_fit();
    }

    /// Reserves capacity for at least `additional` more statutes.
    pub fn reserve(&mut self, additional: usize) {
        self.statutes.borrow_mut().reserve(additional);
    }
}

impl Default for StatuteArena {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Effect, EffectType};

    #[test]
    fn test_arena_new() {
        let arena = StatuteArena::new();
        assert_eq!(arena.len(), 0);
        assert!(arena.is_empty());
    }

    #[test]
    fn test_arena_with_capacity() {
        let arena = StatuteArena::with_capacity(100);
        assert!(arena.capacity() >= 100);
        assert_eq!(arena.len(), 0);
    }

    #[test]
    fn test_arena_alloc() {
        let mut arena = StatuteArena::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let idx = arena.alloc(statute);
        assert_eq!(idx, 0);
        assert_eq!(arena.len(), 1);
    }

    #[test]
    fn test_arena_get() {
        let mut arena = StatuteArena::new();
        let statute = Statute::new("id-1", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let idx = arena.alloc(statute);

        let retrieved = arena.get(idx).unwrap();
        assert_eq!(&retrieved.id, "id-1");
    }

    #[test]
    fn test_arena_clear() {
        let mut arena = StatuteArena::new();
        arena.alloc(Statute::new(
            "id-1",
            "Title",
            Effect::new(EffectType::Grant, "Benefit"),
        ));
        arena.alloc(Statute::new(
            "id-2",
            "Title 2",
            Effect::new(EffectType::Revoke, "Penalty"),
        ));
        assert_eq!(arena.len(), 2);

        arena.clear();
        assert_eq!(arena.len(), 0);
        assert!(arena.is_empty());
    }

    #[test]
    fn test_arena_bulk_allocation() {
        let mut arena = StatuteArena::with_capacity(1000);

        for i in 0..1000 {
            let statute = Statute::new(
                format!("statute-{}", i),
                format!("Title {}", i),
                Effect::new(EffectType::Grant, "Benefit"),
            );
            arena.alloc(statute);
        }

        assert_eq!(arena.len(), 1000);
    }
}
