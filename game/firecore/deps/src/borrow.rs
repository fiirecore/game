use std::borrow::Cow;

mod id;
pub use id::*;

#[derive(Debug)]
pub enum BorrowableMut<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}

impl<'a, T> BorrowableMut<'a, T> {

    pub fn value(&self) -> &T {
        match self {
            Self::Owned(instance) => instance,
            Self::Borrowed(instance) => &**instance,
        }
    }

    pub fn value_mut(&mut self) -> &mut T {
        match self {
            Self::Owned(instance) => instance,
            Self::Borrowed(instance) => instance,
        }
    }
    
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

impl<'a, T: Clone> Clone for BorrowableMut<'a, T> {
    fn clone(&self) -> Self {
        Self::Owned(self.cloned())
    }
}