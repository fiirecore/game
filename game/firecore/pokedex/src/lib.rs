extern crate firecore_dependencies as deps;

use std::borrow::Cow;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod pokemon;
pub mod moves;
pub mod item;

#[derive(Debug)]
pub enum Ref<V: 'static + Identifiable>{
    Init(&'static V),
    Uninit(V::Id),
}

pub trait Identifiable {

    type Id: DeserializeOwned + Serialize + core::fmt::Display + core::fmt::Debug + Clone + Copy;

    fn id(&self) -> &Self::Id;
    
    fn get(id: &Self::Id) -> Ref<Self> where Self: Sized {
        match Self::try_get(id) {
            Some(this) => Ref::Init(this),
            None => Ref::Uninit(*id),
        }
    }

    fn try_get(id: &Self::Id) -> Option<&'static Self> where Self: Sized;

}

impl<V: 'static + Identifiable> Ref<V> {
    pub fn value(self) -> &'static V {
        match self {
            Ref::Init(value) => value,
            Ref::Uninit(id) => match V::get(&id) {
                Ref::Init(value) => value,
                Ref::Uninit(id) => panic!("Could not get reference for {} with id {}", std::any::type_name::<V::Id>(), id),
            },
        }
    }
    pub fn id(&self) -> &V::Id {
        match self {
            Ref::Init(v) => v.id(),
            Ref::Uninit(id) => id,
        }
    }
}

impl<V: 'static + Identifiable> Serialize for Ref<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.id().serialize(serializer)
    }
}

impl<'de, V: 'static + Identifiable> Deserialize<'de> for Ref<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        V::Id::deserialize(deserializer).map(|id| V::get(&id)).map_err(|err| serde::de::Error::custom(format_args!("pokemon de err, {}", err)))
    }
}

impl<V: 'static + Identifiable> Clone for Ref<V> {
    fn clone(&self) -> Self {
        match self {
            Self::Init(v) => Self::Init(v),
            _ => self.clone(),
        }
    }
}

impl<V: 'static + Identifiable> Copy for Ref<V> {}


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