//! A collection of types that use newtype integer

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod slice;

#[cfg(feature = "alloc")]
pub mod vec;

use core::marker::PhantomData;

pub use aidee_derive::Id;
pub use slice::IdSlice;

#[cfg(feature = "alloc")]
pub use vec::IdVec;

// we don't store a `K` anywhere, instead we store an object that consumes `K`, fn(&K) is most appropriate for this.
type Boo<K> = PhantomData<fn(&K)>;

pub trait Id: Copy {
    fn from_index(index: usize) -> Self;
    fn index(self) -> usize;
    fn incr(&mut self) -> Self {
        let id = *self;
        *self = Self::from_index(self.index() + 1);
        id
    }
}

impl Id for usize {
    fn from_index(index: usize) -> Self {
        index
    }
    fn index(self) -> usize {
        self
    }
}

impl Id for u32 {
    fn from_index(index: usize) -> Self {
        index.try_into().unwrap()
    }
    fn index(self) -> usize {
        self.try_into().unwrap()
    }
}

impl Id for u16 {
    fn from_index(index: usize) -> Self {
        index.try_into().unwrap()
    }
    fn index(self) -> usize {
        self.into()
    }
}

impl Id for u8 {
    fn from_index(index: usize) -> Self {
        index.try_into().unwrap()
    }
    fn index(self) -> usize {
        self.into()
    }
}
