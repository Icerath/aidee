use crate::Id;

/// Temporary type until there is support for custom niches
pub struct IdOption<K: Id>(K);

impl<K: Id> IdOption<K> {
    pub fn new(id: K) -> Self {
        Self(id)
    }
    pub const NONE: Self = Self(K::INVALID_REPR);
}

impl<K: Id + PartialEq> IdOption<K> {
    pub fn get(self) -> Option<K> {
        if self.0 == K::INVALID_REPR { None } else { Some(self.0) }
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
