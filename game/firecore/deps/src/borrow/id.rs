use serde::{de::DeserializeOwned, Deserialize, Serialize};
use core::fmt::Display;

pub enum StaticRef<V: 'static + Identifiable> {
    Init(&'static V),
    Uninit(V::Id),
}

impl<V: 'static + Identifiable> Default for StaticRef<V> {
    fn default() -> Self {
        Self::Uninit(V::UNKNOWN)
    }
}

pub trait Identifiable {

    type Id: DeserializeOwned + Serialize + Display + Clone + Copy;

    const UNKNOWN: Self::Id;

    fn id(&self) -> &Self::Id;
    
    fn get(id: &Self::Id) -> StaticRef<Self> where Self: Sized {
        match Self::try_get(id) {
            Some(this) => StaticRef::Init(this),
            None => StaticRef::Uninit(*id),
        }
    }

    fn try_get(id: &Self::Id) -> Option<&'static Self> where Self: Sized;

    fn unknown() -> Option<&'static Self> where Self: Sized {
        Self::try_get(&Self::UNKNOWN)
    }

}

impl<V: 'static + Identifiable> StaticRef<V> {

    pub fn value(&self) -> &'static V {
        match self {
            StaticRef::Init(value) => value,
            StaticRef::Uninit(id) => match V::get(&id) {
                StaticRef::Init(value) => value,
                StaticRef::Uninit(..) => match V::unknown() {
                    Some(unknown) => unknown,
                    None => panic!("Could not get reference for {} with id {}", {
                        let v = std::any::type_name::<V>();
                        v.split("::").last().unwrap_or(v)
                    }, id),
                },
            },
        }
    }

    // pub fn try_value()

    pub fn id(&self) -> &V::Id {
        match self {
            StaticRef::Init(v) => v.id(),
            StaticRef::Uninit(id) => id,
        }
    }
}

impl<V: 'static + Identifiable> Serialize for StaticRef<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.id().serialize(serializer)
    }
}

impl<'de, V: 'static + Identifiable> Deserialize<'de> for StaticRef<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        V::Id::deserialize(deserializer).map(|id| V::get(&id)).map_err(|err| serde::de::Error::custom(format_args!("pokemon de err, {}", err)))
    }
}

impl<V: 'static + Identifiable> Clone for StaticRef<V> {
    fn clone(&self) -> Self {
        match self {
            Self::Init(v) => Self::Init(v),
            _ => *self,
        }
    }
}

impl<V: 'static + Identifiable> Copy for StaticRef<V> {}

impl<V: 'static + Identifiable> Display for StaticRef<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.id(), f)
    }
}

impl<V: 'static + Identifiable> core::fmt::Debug for StaticRef<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(self.id(), f)
    }
}