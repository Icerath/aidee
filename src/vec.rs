use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
};

use crate::{Id, slice::IdSlice};
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
    pub const fn new() -> Self {
        Self::from_vec(Vec::new())
    }
    pub const fn from_vec(vec: Vec<V>) -> Self {
        Self { raw: vec, _marker: PhantomData }
    }
    pub const fn capacity(&self) -> usize {
        self.raw.capacity()
    }
    pub fn push(&mut self, value: V) -> K {
        let id = self.len_id();
        self.raw.push(value);
        id
    }
    pub fn pop(&mut self) -> Option<V> {
        self.raw.pop()
    }
    pub fn retain(&mut self, mut f: impl FnMut(K, &mut V) -> bool) {
        let mut id = K::from_index(0);
        self.raw.retain_mut(|value| f(id.incr(), value));
    }
    pub const fn as_slice(&self) -> &IdSlice<K, V> {
        IdSlice::from_slice(self.raw.as_slice())
    }
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

impl<K: Id, V> Index<K> for IdVec<K, V> {
    type Output = V;
    #[track_caller]
    fn index(&self, index: K) -> &Self::Output {
        &self.raw[index.index()]
    }
}

impl<K: Id, V> IndexMut<K> for IdVec<K, V> {
    #[track_caller]
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.raw[index.index()]
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
}
