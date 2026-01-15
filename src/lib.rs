//! A collection of types that use newtype integer

#![cfg_attr(feature = "nightly", feature(new_range_api))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod option;
pub mod slice;

#[cfg(feature = "alloc")]
pub mod bitvec;
#[cfg(feature = "alloc")]
pub mod vec;

use core::marker::PhantomData;

pub use aidee_derive::Id;
pub use slice::IdSlice;

#[cfg(feature = "alloc")]
pub use bitvec::IdBitVec;
#[cfg(feature = "alloc")]
pub use vec::IdVec;

// we don't store a `K` anywhere, instead we store an object that consumes `K`, fn(&K) is most appropriate for this.
type Boo<K> = PhantomData<fn(&K)>;

pub trait Id: Copy {
    const INVALID_REPR: Self;
    fn from_index(index: usize) -> Self;
    fn index(self) -> usize;
    #[expect(clippy::return_self_not_must_use, reason = "I'm just not sure yet")]
    fn incr(&mut self) -> Self {
        let id = *self;
        *self = Self::from_index(self.index() + 1);
        id
    }
}

impl Id for usize {
    const INVALID_REPR: Self = Self::MAX;
    fn from_index(index: usize) -> Self {
        index
    }
    fn index(self) -> usize {
        self
    }
}

impl Id for u32 {
    const INVALID_REPR: Self = Self::MAX;

    #[expect(clippy::cast_possible_truncation)]
    fn from_index(index: usize) -> Self {
        debug_assert!(Self::try_from(index).is_ok());
        index as _
    }
    fn index(self) -> usize {
        self as _
    }
}

impl Id for u16 {
    const INVALID_REPR: Self = Self::MAX;
    #[expect(clippy::cast_possible_truncation)]
    fn from_index(index: usize) -> Self {
        debug_assert!(Self::try_from(index).is_ok());
        index as _
    }
    fn index(self) -> usize {
        self.into()
    }
}

impl Id for u8 {
    const INVALID_REPR: Self = Self::MAX;

    #[expect(clippy::cast_possible_truncation)]
    fn from_index(index: usize) -> Self {
        debug_assert!(Self::try_from(index).is_ok());
        index as _
    }
    fn index(self) -> usize {
        self.into()
    }
}
