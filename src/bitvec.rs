use core::{marker::PhantomData, ops::Index};

use crate::Id;

use bitvec::vec::BitVec;

pub struct IdBitVec<K: Id> {
    raw: BitVec,
    _marker: crate::Boo<K>,
}

impl<K: Id> Default for IdBitVec<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Id> IdBitVec<K> {
    pub const fn new() -> Self {
        Self::from_bitvec(BitVec::EMPTY)
    }
    const fn from_bitvec(bitvec: BitVec) -> Self {
        Self { raw: bitvec, _marker: PhantomData }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from_bitvec(BitVec::with_capacity(capacity))
    }
    pub fn reserve(&mut self, additional: usize) {
        self.raw.reserve(additional);
    }
    pub fn push(&mut self, value: bool) -> K {
        let id = self.len_id();
        self.raw.push(value);
        id
    }
    pub fn set(&mut self, id: K, value: bool) {
        self.raw.set(id.index(), value);
    }
    pub fn replace(&mut self, id: K, value: bool) -> bool {
        self.raw.replace(id.index(), value)
    }
    pub fn swap(&mut self, lhs: K, rhs: K) {
        self.raw.swap(lhs.index(), rhs.index());
    }
    pub fn reverse(&mut self) {
        self.raw.reverse();
    }
    pub fn retain<F: FnMut(K, bool) -> bool>(&mut self, mut f: F) {
        self.raw.retain(|index, &bit| f(K::from_index(index), bit));
    }
    pub fn clear(&mut self) {
        self.raw.clear();
    }
    pub fn len_id(&self) -> K {
        K::from_index(self.raw.len())
    }
    pub fn len(&self) -> usize {
        self.raw.len()
    }
    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
    pub fn ids(&self) -> impl Iterator<Item = K> + use<K> {
        (0..self.len()).map(K::from_index)
    }
    pub fn values(&self) -> impl Iterator<Item = bool> {
        self.raw.iter().by_vals()
    }
    pub fn iter(&self) -> impl Iterator<Item = (K, bool)> {
        self.raw.iter().by_vals().enumerate().map(|(index, b)| (K::from_index(index), b))
    }
}

impl<K: Id> Index<K> for IdBitVec<K> {
    type Output = bool;

    #[track_caller]
    fn index(&self, index: K) -> &Self::Output {
        &self.raw[index.index()]
    }
}

impl<K: Id> FromIterator<bool> for IdBitVec<K> {
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        Self::from_bitvec(BitVec::from_iter(iter))
    }
}

impl<K: Id> Extend<bool> for IdBitVec<K> {
    fn extend<T: IntoIterator<Item = bool>>(&mut self, iter: T) {
        self.raw.extend(iter);
    }
}
