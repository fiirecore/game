use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

mod id;
pub use id::*;

#[derive(Debug)]
pub enum BorrowableMut<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}

impl<'a, T: ToOwned> BorrowableMut<'a, T> {
    pub fn as_ref(&'a self) -> Cow<'a, T> {
        match self {
            Self::Borrowed(t) => Cow::Borrowed(t),
            Self::Owned(t) => Cow::Owned(t.to_owned()),
        }
    }
}

impl<'a, T: Clone> BorrowableMut<'a, T> {
    pub fn cloned(&self) -> T where {
        match self {
            Self::Owned(t) => t,
            Self::Borrowed(t) => (*t),
        }
        .clone()
    }

    pub fn owned(self) -> T where {
        match self {
            BorrowableMut::Owned(t) => t,
            BorrowableMut::Borrowed(t) => t.clone(),
        }
    }
}

impl<'a, T> Deref for BorrowableMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(instance) => instance,
            Self::Borrowed(instance) => &**instance,
        }
    }
}

impl<'a, T> DerefMut for BorrowableMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Owned(instance) => instance,
            Self::Borrowed(instance) => *instance,
        }
    }
}

impl<'a, T: Clone> Clone for BorrowableMut<'a, T> {
    fn clone(&self) -> Self {
        Self::Owned(self.cloned())
    }
}

impl<'de, 'a, T: Deserialize<'de>> Deserialize<'de> for BorrowableMut<'a, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(BorrowableMut::Owned)
    }
}

impl<'a, T: Serialize> Serialize for BorrowableMut<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BorrowableMut::Owned(t) => t,
            BorrowableMut::Borrowed(t) => &**t,
        }
        .serialize(serializer)
    }
}
