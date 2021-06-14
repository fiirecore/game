use serde::{de::DeserializeOwned, Deserialize, Serialize};
use core::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub type StaticRef<V> = IdentifiableRef<'static, V>;

pub enum IdentifiableRef<'a, V: Identifiable<'a>> {
    Init(&'a V),
    Uninit(V::Id),
}

impl<'a, V: Identifiable<'a>> Default for IdentifiableRef<'a, V> {
    fn default() -> Self {
        Self::Uninit(V::UNKNOWN)
    }
}

pub trait Identifiable<'a> {

    type Id: DeserializeOwned + Serialize + Display + Clone + Copy;

    const UNKNOWN: Self::Id;

    fn id(&self) -> &Self::Id;
    
    fn get(id: &Self::Id) -> IdentifiableRef<'a, Self> where Self: Sized {
        match Self::try_get(id) {
            Some(this) => IdentifiableRef::Init(this),
            None => IdentifiableRef::Uninit(*id),
        }
    }

    fn try_get(id: &Self::Id) -> Option<&'a Self> where Self: Sized;

    fn unknown() -> Option<&'a Self> where Self: Sized {
        Self::try_get(&Self::UNKNOWN)
    }

}

impl<'a, V: Identifiable<'a>> IdentifiableRef<'a, V> {

    pub fn value(&self) -> &'a V {
        match self {
            IdentifiableRef::Init(value) => value,
            IdentifiableRef::Uninit(id) => match V::get(&id) {
                IdentifiableRef::Init(value) => value,
                IdentifiableRef::Uninit(..) => match V::unknown() {
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
            IdentifiableRef::Init(v) => v.id(),
            IdentifiableRef::Uninit(id) => id,
        }
    }
}

impl<'a, V: Identifiable<'a>> Serialize for IdentifiableRef<'a, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        self.id().serialize(serializer)
    }
}

impl<'a, 'de, V: Identifiable<'a>> Deserialize<'de> for IdentifiableRef<'a, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        V::Id::deserialize(deserializer).map(|id| V::get(&id)).map_err(|err| serde::de::Error::custom(format_args!("pokemon de err, {}", err)))
    }
}

impl<'a, V: Identifiable<'a>> Clone for IdentifiableRef<'a, V> {
    fn clone(&self) -> Self {
        match self {
            Self::Init(v) => Self::Init(v),
            _ => *self,
        }
    }
}

impl<'a, V: Identifiable<'a>> Copy for IdentifiableRef<'a, V> {}

impl<'a, V: Identifiable<'a>> Display for IdentifiableRef<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.id(), f)
    }
}

impl<'a, V: Identifiable<'a>> Debug for IdentifiableRef<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.id(), f)
    }
}