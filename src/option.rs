use core::fmt;

use crate::Id;

/// Temporary type until there is support for custom niches
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdOption<K: Id>(K);

impl<K: Id> IdOption<K> {
    pub fn new(id: K) -> Self {
        Self(id)
    }
    pub const NONE: Self = Self(K::INVALID_REPR);

    pub fn get(self) -> Option<K> {
        if self.0.index() == K::INVALID_REPR.index() { None } else { Some(self.0) }
    }
    pub fn is_none(&self) -> bool {
        self.0.index() == K::INVALID_REPR.index()
    }
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
    pub fn into_option(self) -> Option<K> {
        self.into()
    }
}

impl<K: Id> From<K> for IdOption<K> {
    fn from(value: K) -> Self {
        Self(value)
    }
}

impl<K: Id> From<Option<K>> for IdOption<K> {
    fn from(value: Option<K>) -> Self {
        match value {
            Some(id) => Self(id),
            None => Self::NONE,
        }
    }
}

impl<K: Id> From<IdOption<K>> for Option<K> {
    fn from(value: IdOption<K>) -> Self {
        if value.is_some() { Some(value.0) } else { None }
    }
}

impl<K: fmt::Debug + Id> fmt::Debug for IdOption<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.into_option().fmt(f)
    }
}
