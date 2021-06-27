use std::{borrow::Cow, ops::{Deref, DerefMut}};

mod id;
pub use id::*;

#[derive(Debug)]
pub enum BorrowableMut<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}

impl<'a, T> BorrowableMut<'a, T> {
    
    pub fn as_ref(&'a self) -> Cow<'a, T> where T: ToOwned {
        match self {
            Self::Borrowed(t) => Cow::Borrowed(t),
            Self::Owned(t) => Cow::Owned(t.to_owned()),
        }
    }

    pub fn cloned(&self) -> T where T: Clone {
        match self {
            Self::Owned(t) => t,
            Self::Borrowed(t) => (*t),
        }.clone()
    }

    pub fn owned(self) -> T where T: Clone {
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