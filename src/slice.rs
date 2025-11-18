use crate::Id;
use core::{
    fmt,
    ops::{Index, IndexMut},
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[repr(transparent)]
pub struct IdSlice<K: Id, V> {
    _marker: crate::Boo<K>,
    pub raw: [V],
}

impl<K: Id, V> IdSlice<K, V> {
    pub const fn from_slice(slice: &[V]) -> &Self {
        unsafe { core::mem::transmute(slice) }
    }
    pub const fn from_mut_slice(slice: &mut [V]) -> &mut Self {
        unsafe { core::mem::transmute(slice) }
    }
    #[cfg(feature = "alloc")]
    pub const fn from_boxed_slice(slice: Box<[V]>) -> Box<Self> {
        unsafe { core::mem::transmute(slice) }
    }
    pub fn get(&self, key: K) -> Option<&V> {
        self.raw.get(key.index())
    }
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.raw.get_mut(key.index())
    }
    pub const fn len(&self) -> usize {
        self.raw.len()
    }
    pub const fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }
    pub fn ids(&self) -> impl Iterator<Item = K> {
        (0..self.raw.len()).map(|index| K::from_index(index))
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
    pub fn len_id(&self) -> K {
        K::from_index(self.raw.len())
    }
}

impl<K: Id, V> Index<K> for IdSlice<K, V> {
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        &self.raw[index.index()]
    }
}

impl<K: Id, V> IndexMut<K> for IdSlice<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.raw[index.index()]
    }
}

impl<'a, K: Id, V> From<&'a [V]> for &'a IdSlice<K, V> {
    fn from(slice: &'a [V]) -> Self {
        IdSlice::from_slice(slice)
    }
}

impl<'a, K: Id, V> From<&'a mut [V]> for &'a mut IdSlice<K, V> {
    fn from(slice: &'a mut [V]) -> Self {
        IdSlice::from_mut_slice(slice)
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
