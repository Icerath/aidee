use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
};

use crate::{
    Id,
    slice::{IdSlice, IdSliceIndex},
};
use alloc::vec::Vec;

pub struct IdVec<K: Id, V> {
    pub raw: Vec<V>,
    _marker: crate::Boo<K>,
}

impl<K: Id, V> Default for IdVec<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Id, V> IdVec<K, V> {
    #[must_use]
    pub const fn new() -> Self {
        Self::from_vec(Vec::new())
    }
    #[must_use]
    pub const fn from_vec(vec: Vec<V>) -> Self {
        Self { raw: vec, _marker: PhantomData }
    }
    #[must_use]
    pub fn repeat(value: V, len: K) -> Self
    where
        V: Clone,
    {
        Self::from_vec(alloc::vec![value; len.index()])
    }
    #[must_use]
    pub fn repeat_with(mut f: impl FnMut(K) -> V, len: K) -> Self {
        (0..len.index()).map(|i| f(K::from_index(i))).collect()
    }
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_vec(Vec::with_capacity(capacity))
    }
    pub fn reserve(&mut self, additional: usize) {
        self.raw.reserve(additional);
    }
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.raw.capacity()
    }
    pub fn extend<I: IntoIterator<Item = V>>(&mut self, iter: I) -> Range<K> {
        let start_len = self.len_id();
        self.raw.extend(iter);
        let end_len = self.len_id();
        start_len..end_len
    }
    pub fn push(&mut self, value: V) -> K {
        let id = self.len_id();
        self.raw.push(value);
        id
    }
    pub fn push_with<F: FnOnce(K) -> V>(&mut self, f: F) -> K {
        let id = self.len_id();
        self.raw.push(f(id));
        id
    }
    pub fn pop(&mut self) -> Option<V> {
        self.raw.pop()
    }
    pub fn retain(&mut self, mut f: impl FnMut(K, &mut V) -> bool) {
        let mut id = K::from_index(0);
        self.raw.retain_mut(|value| f(id.incr(), value));
    }
    pub fn clear(&mut self) {
        self.raw.clear();
    }
    #[must_use]
    pub const fn as_slice(&self) -> &IdSlice<K, V> {
        IdSlice::from_slice(self.raw.as_slice())
    }
    #[must_use]
    pub const fn as_mut_slice(&mut self) -> &mut IdSlice<K, V> {
        IdSlice::from_mut_slice(self.raw.as_mut_slice())
    }
}

impl<K: Id, V> From<Vec<V>> for IdVec<K, V> {
    fn from(vec: Vec<V>) -> Self {
        Self::from_vec(vec)
    }
}

impl<K: Id, V> Extend<V> for IdVec<K, V> {
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        self.raw.extend(iter);
    }
}

impl<K: Id, V> FromIterator<V> for IdVec<K, V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<K: Id, V> Deref for IdVec<K, V> {
    type Target = IdSlice<K, V>;
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<K: Id, V> DerefMut for IdVec<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<K: Id, V, I: IdSliceIndex<K, V>> Index<I> for IdVec<K, V> {
    type Output = I::Output;
    #[track_caller]
    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<K: Id, V, I: IdSliceIndex<K, V>> IndexMut<I> for IdVec<K, V> {
    #[track_caller]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}

impl<K: Id, V: fmt::Debug> fmt::Debug for IdVec<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.raw, f)
    }
}

impl<K: Id, V: PartialEq> PartialEq for IdVec<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<K: Id, V: Eq> Eq for IdVec<K, V> {}

impl<K: Id, V: Clone> Clone for IdVec<K, V> {
    fn clone(&self) -> Self {
        Self::from_vec(self.raw.clone())
    }
    fn clone_from(&mut self, source: &Self) {
        self.raw.clone_from(&source.raw);
    }
}
