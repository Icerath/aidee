use crate::Id;
use core::{
    fmt,
    ops::{Index, IndexMut, Range},
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[repr(transparent)]
pub struct IdSlice<K: Id, V> {
    _marker: crate::Boo<K>,
    pub raw: [V],
}

impl<K: Id, V> IdSlice<K, V> {
    #[must_use]
    pub const fn from_slice(slice: &[V]) -> &Self {
        unsafe { core::mem::transmute(slice) }
    }
    #[must_use]
    pub const fn from_mut_slice(slice: &mut [V]) -> &mut Self {
        unsafe { core::mem::transmute(slice) }
    }
    #[cfg(feature = "alloc")]
    #[must_use]
    pub const fn from_boxed_slice(slice: Box<[V]>) -> Box<Self> {
        unsafe { core::mem::transmute(slice) }
    }
    #[must_use]
    pub fn get<I: IdSliceIndex<K, V>>(&self, index: I) -> Option<&I::Output> {
        index.get(self)
    }
    #[must_use]
    pub fn get_mut<I: IdSliceIndex<K, V>>(&mut self, index: I) -> Option<&mut I::Output> {
        index.get_mut(self)
    }
    #[must_use]
    pub fn len_id(&self) -> K {
        K::from_index(self.raw.len())
    }
    #[must_use]
    pub const fn len(&self) -> usize {
        self.raw.len()
    }
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
    pub fn ids(&self) -> impl Iterator<Item = K> + use<K, V> {
        (0..self.raw.len()).map(K::from_index)
    }
    pub fn values(&self) -> core::slice::Iter<'_, V> {
        self.raw.iter()
    }
    pub fn values_mut(&mut self) -> core::slice::IterMut<'_, V> {
        self.raw.iter_mut()
    }
    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        self.raw.iter().enumerate().map(|(i, v)| (K::from_index(i), v))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (K, &mut V)> {
        self.raw.iter_mut().enumerate().map(|(i, v)| (K::from_index(i), v))
    }
}

impl<K: Id, V, I: IdSliceIndex<K, V>> Index<I> for IdSlice<K, V> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

impl<K: Id, V, I: IdSliceIndex<K, V>> IndexMut<I> for IdSlice<K, V> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        index.index_mut(self)
    }
}

#[cfg(feature = "alloc")]
impl<K: Id, V> From<Box<[V]>> for Box<IdSlice<K, V>> {
    fn from(slice: Box<[V]>) -> Self {
        IdSlice::from_boxed_slice(slice)
    }
}

impl<K: Id, V: fmt::Debug> fmt::Debug for IdSlice<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.raw, f)
    }
}

pub trait IdSliceIndex<K: Id, V> {
    type Output: ?Sized;
    fn get(self, slice: &IdSlice<K, V>) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut IdSlice<K, V>) -> Option<&mut Self::Output>;
    fn index(self, slice: &IdSlice<K, V>) -> &Self::Output;
    fn index_mut(self, slice: &mut IdSlice<K, V>) -> &mut Self::Output;
}

impl<K: Id, V> IdSliceIndex<K, V> for K {
    type Output = V;
    fn get(self, slice: &IdSlice<K, V>) -> Option<&V> {
        slice.raw.get(self.index())
    }
    fn get_mut(self, slice: &mut IdSlice<K, V>) -> Option<&mut V> {
        slice.raw.get_mut(self.index())
    }
    #[track_caller]
    fn index(self, slice: &IdSlice<K, V>) -> &V {
        &slice.raw[self.index()]
    }
    #[track_caller]
    fn index_mut(self, slice: &mut IdSlice<K, V>) -> &mut V {
        &mut slice.raw[self.index()]
    }
}

impl<K: Id, V> IdSliceIndex<K, V> for Range<K> {
    type Output = [V];
    fn get(self, slice: &IdSlice<K, V>) -> Option<&[V]> {
        slice.raw.get(self.start.index()..self.end.index())
    }
    fn get_mut(self, slice: &mut IdSlice<K, V>) -> Option<&mut Self::Output> {
        slice.raw.get_mut(self.start.index()..self.end.index())
    }
    #[track_caller]
    fn index(self, slice: &IdSlice<K, V>) -> &Self::Output {
        &slice.raw[self.start.index()..self.end.index()]
    }
    #[track_caller]
    fn index_mut(self, slice: &mut IdSlice<K, V>) -> &mut Self::Output {
        &mut slice.raw[self.start.index()..self.end.index()]
    }
}
