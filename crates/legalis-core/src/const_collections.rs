//! Const generic collections for performance-critical legal operations.
//!
//! This module provides array-based collections using const generics for
//! stack-allocated, cache-friendly data structures in performance-critical
//! code paths.
//!
//! # Benefits
//!
//! - **Stack allocation**: No heap allocations for small collections
//! - **Cache locality**: Better CPU cache utilization
//! - **Compile-time size**: Size known at compile time
//! - **Zero-cost abstractions**: No runtime overhead
//!
//! # Examples
//!
//! ```
//! use legalis_core::const_collections::{ConditionSet, StatuteArray};
//! use legalis_core::{Condition, ComparisonOp, Statute, Effect, EffectType};
//!
//! // Fixed-size condition set (stack allocated)
//! let mut conditions = ConditionSet::<3>::new();
//! conditions.push(Condition::Age {
//!     operator: ComparisonOp::GreaterOrEqual,
//!     value: 18
//! }).unwrap();
//!
//! assert_eq!(conditions.len(), 1);
//! assert_eq!(conditions.capacity(), 3);
//! ```

use crate::{Condition, Statute, ValidationError};
use std::fmt;

/// Error types for const collections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionError {
    /// Collection is full (capacity exceeded)
    CapacityExceeded,
    /// Index out of bounds
    IndexOutOfBounds,
    /// Item not found
    NotFound,
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityExceeded => write!(f, "Collection capacity exceeded"),
            Self::IndexOutOfBounds => write!(f, "Index out of bounds"),
            Self::NotFound => write!(f, "Item not found"),
        }
    }
}

impl std::error::Error for CollectionError {}

/// Fixed-size array of conditions (stack allocated).
///
/// Useful for statutes with a known small number of preconditions.
///
/// # Examples
///
/// ```
/// use legalis_core::const_collections::ConditionSet;
/// use legalis_core::{Condition, ComparisonOp};
///
/// let mut conditions = ConditionSet::<5>::new();
///
/// conditions.push(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 21
/// }).unwrap();
///
/// conditions.push(Condition::Income {
///     operator: ComparisonOp::LessThan,
///     value: 50000
/// }).unwrap();
///
/// assert_eq!(conditions.len(), 2);
/// assert!(!conditions.is_full());
/// ```
#[derive(Debug, Clone)]
pub struct ConditionSet<const N: usize> {
    items: [Option<Condition>; N],
    len: usize,
}

impl<const N: usize> ConditionSet<N> {
    /// Create a new empty condition set.
    pub fn new() -> Self {
        Self {
            items: [const { None }; N],
            len: 0,
        }
    }

    /// Add a condition to the set.
    pub fn push(&mut self, condition: Condition) -> Result<(), CollectionError> {
        if self.len >= N {
            return Err(CollectionError::CapacityExceeded);
        }
        self.items[self.len] = Some(condition);
        self.len += 1;
        Ok(())
    }

    /// Get a condition by index.
    pub fn get(&self, index: usize) -> Option<&Condition> {
        if index < self.len {
            self.items[index].as_ref()
        } else {
            None
        }
    }

    /// Get a mutable reference to a condition.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Condition> {
        if index < self.len {
            self.items[index].as_mut()
        } else {
            None
        }
    }

    /// Remove the last condition.
    pub fn pop(&mut self) -> Option<Condition> {
        if self.len > 0 {
            self.len -= 1;
            self.items[self.len].take()
        } else {
            None
        }
    }

    /// Get the number of conditions.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if the set is full.
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Get the capacity.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Get remaining capacity.
    pub fn remaining_capacity(&self) -> usize {
        N - self.len
    }

    /// Clear all conditions.
    pub fn clear(&mut self) {
        for item in &mut self.items[..self.len] {
            *item = None;
        }
        self.len = 0;
    }

    /// Iterate over conditions.
    pub fn iter(&self) -> impl Iterator<Item = &Condition> {
        self.items[..self.len].iter().filter_map(|opt| opt.as_ref())
    }

    /// Convert to a Vec.
    pub fn to_vec(&self) -> Vec<Condition> {
        self.iter().cloned().collect()
    }
}

impl<const N: usize> Default for ConditionSet<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed-size array of statutes (stack allocated).
///
/// # Examples
///
/// ```
/// use legalis_core::const_collections::StatuteArray;
/// use legalis_core::{Statute, Effect, EffectType};
///
/// let mut statutes = StatuteArray::<3>::new();
///
/// statutes.push(Statute::new(
///     "s1",
///     "Statute 1",
///     Effect::new(EffectType::Grant, "Grant 1")
/// )).unwrap();
///
/// assert_eq!(statutes.len(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct StatuteArray<const N: usize> {
    items: [Option<Statute>; N],
    len: usize,
}

impl<const N: usize> StatuteArray<N> {
    /// Create a new empty statute array.
    pub fn new() -> Self {
        Self {
            items: [const { None }; N],
            len: 0,
        }
    }

    /// Add a statute to the array.
    pub fn push(&mut self, statute: Statute) -> Result<(), CollectionError> {
        if self.len >= N {
            return Err(CollectionError::CapacityExceeded);
        }
        self.items[self.len] = Some(statute);
        self.len += 1;
        Ok(())
    }

    /// Get a statute by index.
    pub fn get(&self, index: usize) -> Option<&Statute> {
        if index < self.len {
            self.items[index].as_ref()
        } else {
            None
        }
    }

    /// Get a mutable reference to a statute.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Statute> {
        if index < self.len {
            self.items[index].as_mut()
        } else {
            None
        }
    }

    /// Find a statute by ID.
    pub fn find_by_id(&self, id: &str) -> Option<&Statute> {
        self.iter().find(|s| s.id == id)
    }

    /// Remove the last statute.
    pub fn pop(&mut self) -> Option<Statute> {
        if self.len > 0 {
            self.len -= 1;
            self.items[self.len].take()
        } else {
            None
        }
    }

    /// Get the number of statutes.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Check if the array is full.
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Get the capacity.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Clear all statutes.
    pub fn clear(&mut self) {
        for item in &mut self.items[..self.len] {
            *item = None;
        }
        self.len = 0;
    }

    /// Iterate over statutes.
    pub fn iter(&self) -> impl Iterator<Item = &Statute> {
        self.items[..self.len].iter().filter_map(|opt| opt.as_ref())
    }

    /// Validate all statutes.
    pub fn validate_all(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        for statute in self.iter() {
            errors.extend(statute.validate());
        }
        errors
    }

    /// Convert to a Vec.
    pub fn to_vec(&self) -> Vec<Statute> {
        self.iter().cloned().collect()
    }
}

impl<const N: usize> Default for StatuteArray<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixed-size lookup table for fast ID-based access.
///
/// Uses const generics to provide a stack-allocated hash table
/// for small collections of statutes.
///
/// # Examples
///
/// ```
/// use legalis_core::const_collections::FastLookup;
/// use legalis_core::{Statute, Effect, EffectType};
///
/// let mut lookup = FastLookup::<5>::new();
///
/// let statute = Statute::new("tax-1", "Tax Law", Effect::grant("Tax credit"));
/// lookup.insert(statute.clone()).unwrap();
///
/// assert!(lookup.contains("tax-1"));
/// assert_eq!(lookup.get("tax-1").unwrap().id, "tax-1");
/// ```
#[derive(Debug, Clone)]
pub struct FastLookup<const N: usize> {
    items: [(Option<String>, Option<Statute>); N],
}

impl<const N: usize> FastLookup<N> {
    /// Create a new empty lookup table.
    pub fn new() -> Self {
        Self {
            items: [const { (None, None) }; N],
        }
    }

    /// Insert a statute.
    pub fn insert(&mut self, statute: Statute) -> Result<(), CollectionError> {
        let id = statute.id.clone();
        let hash = Self::hash(&id) % N;

        // Linear probing
        for i in 0..N {
            let index = (hash + i) % N;
            if self.items[index].0.is_none() {
                self.items[index] = (Some(id), Some(statute));
                return Ok(());
            }
        }

        Err(CollectionError::CapacityExceeded)
    }

    /// Get a statute by ID.
    pub fn get(&self, id: &str) -> Option<&Statute> {
        let hash = Self::hash(id) % N;

        for i in 0..N {
            let index = (hash + i) % N;
            if let Some(ref stored_id) = self.items[index].0 {
                if stored_id == id {
                    return self.items[index].1.as_ref();
                }
            } else {
                return None;
            }
        }

        None
    }

    /// Check if an ID exists.
    pub fn contains(&self, id: &str) -> bool {
        self.get(id).is_some()
    }

    /// Remove a statute by ID.
    pub fn remove(&mut self, id: &str) -> Option<Statute> {
        let hash = Self::hash(id) % N;

        for i in 0..N {
            let index = (hash + i) % N;
            if let Some(ref stored_id) = self.items[index].0 {
                if stored_id == id {
                    let statute = self.items[index].1.take();
                    self.items[index].0 = None;
                    return statute;
                }
            } else {
                return None;
            }
        }

        None
    }

    /// Simple hash function for strings.
    fn hash(s: &str) -> usize {
        let mut hash = 0usize;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as usize);
        }
        hash
    }

    /// Get the capacity.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Count non-empty slots.
    pub fn len(&self) -> usize {
        self.items.iter().filter(|(id, _)| id.is_some()).count()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        for item in &mut self.items {
            *item = (None, None);
        }
    }
}

impl<const N: usize> Default for FastLookup<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ComparisonOp, Effect};

    #[test]
    fn test_condition_set_basic() {
        let mut set = ConditionSet::<3>::new();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
        assert_eq!(set.capacity(), 3);

        let cond = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        set.push(cond.clone()).unwrap();

        assert_eq!(set.len(), 1);
        assert!(!set.is_empty());
        assert!(!set.is_full());
    }

    #[test]
    fn test_condition_set_capacity() {
        let mut set = ConditionSet::<2>::new();

        set.push(Condition::age(ComparisonOp::GreaterOrEqual, 18))
            .unwrap();
        set.push(Condition::income(ComparisonOp::LessThan, 50000))
            .unwrap();

        assert!(set.is_full());

        let result = set.push(Condition::age(ComparisonOp::GreaterOrEqual, 21));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CollectionError::CapacityExceeded);
    }

    #[test]
    fn test_condition_set_pop() {
        let mut set = ConditionSet::<3>::new();
        set.push(Condition::age(ComparisonOp::GreaterOrEqual, 18))
            .unwrap();
        set.push(Condition::income(ComparisonOp::LessThan, 50000))
            .unwrap();

        assert_eq!(set.len(), 2);

        let popped = set.pop().unwrap();
        assert_eq!(set.len(), 1);

        // Check that we popped the income condition
        if let Condition::Income { .. } = popped {
            // Expected
        } else {
            panic!("Expected Income condition");
        }
    }

    #[test]
    fn test_statute_array_basic() {
        let mut array = StatuteArray::<3>::new();
        assert_eq!(array.len(), 0);

        let statute = Statute::new("s1", "Statute 1", Effect::grant("Grant"));
        array.push(statute).unwrap();

        assert_eq!(array.len(), 1);
        assert!(!array.is_full());
    }

    #[test]
    fn test_statute_array_find() {
        let mut array = StatuteArray::<3>::new();

        let s1 = Statute::new("tax-1", "Tax Law 1", Effect::grant("Credit"));
        let s2 = Statute::new("tax-2", "Tax Law 2", Effect::grant("Deduction"));

        array.push(s1).unwrap();
        array.push(s2).unwrap();

        assert!(array.find_by_id("tax-1").is_some());
        assert!(array.find_by_id("tax-2").is_some());
        assert!(array.find_by_id("tax-3").is_none());
    }

    #[test]
    fn test_fast_lookup_basic() {
        let mut lookup = FastLookup::<5>::new();

        let statute = Statute::new("test", "Test", Effect::grant("Grant"));
        lookup.insert(statute).unwrap();

        assert!(lookup.contains("test"));
        assert!(lookup.get("test").is_some());
        assert_eq!(lookup.len(), 1);
    }

    #[test]
    fn test_fast_lookup_multiple() {
        let mut lookup = FastLookup::<10>::new();

        for i in 0..5 {
            let id = format!("statute-{}", i);
            let statute = Statute::new(&id, format!("Statute {}", i), Effect::grant("Grant"));
            lookup.insert(statute).unwrap();
        }

        assert_eq!(lookup.len(), 5);
        assert!(lookup.contains("statute-0"));
        assert!(lookup.contains("statute-4"));
        assert!(!lookup.contains("statute-5"));
    }

    #[test]
    fn test_fast_lookup_remove() {
        let mut lookup = FastLookup::<5>::new();

        let statute = Statute::new("remove-me", "Test", Effect::grant("Grant"));
        lookup.insert(statute).unwrap();

        assert!(lookup.contains("remove-me"));

        let removed = lookup.remove("remove-me");
        assert!(removed.is_some());
        assert!(!lookup.contains("remove-me"));
        assert_eq!(lookup.len(), 0);
    }
}
