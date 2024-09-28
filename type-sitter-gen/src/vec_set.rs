use std::ops::Deref;
use serde::Deserialize;

/// Sorted set backed by a vector. Insertion and retrieval via binary search.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct VecSet<T>(Vec<T>);

impl<T> VecSet<T> {
    /// Create a new empty set.
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T: Ord> VecSet<T> {
    /// Insert an element into the set.
    ///
    /// Returns `true` if the element was inserted, `false` if it was already present.
    pub fn insert(&mut self, value: T) -> bool {
        match self.0.binary_search(&value) {
            Ok(_) => false,
            Err(i) => {
                self.0.insert(i, value);
                true
            }
        }
    }

    /// Check if the set contains the given value.
    pub fn contains(&self, value: &T) -> bool {
        self.0.binary_search(value).is_ok()
    }

    /// Get the index of the given value in the set.
    pub fn index_of(&self, value: &T) -> Option<usize> {
        self.0.binary_search(value).ok()
    }

    /// Get the elements as a slice.
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
}

impl<T> Default for VecSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for VecSet<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

impl<'a, T> IntoIterator for &'a VecSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<T> IntoIterator for VecSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Ord> FromIterator<T> for VecSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut this = Self::new();
        this.extend(iter);
        this
    }
}

impl<T: Ord> Extend<T> for VecSet<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(value);
        }
    }
}
