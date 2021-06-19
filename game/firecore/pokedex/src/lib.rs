extern crate firecore_dependencies as deps;

use deps::{
    random::{Random, RandomState, GLOBAL_STATE},
    borrow::Identifiable,
    hash::HashMap,
};

pub mod pokemon;
pub mod moves;
pub mod item;

pub mod types;

pub mod trainer;

pub(crate) static RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));

pub trait Dex<'a> {

    type DexType: Identifiable<'a> + 'a;

    fn dex() -> &'a mut Option<HashMap<<<Self as Dex<'a>>::DexType as Identifiable<'a>>::Id, Self::DexType>>;

    fn set(dex: HashMap<<<Self as Dex<'a>>::DexType as Identifiable<'a>>::Id, Self::DexType>) {
        *Self::dex() = Some(dex)
    }

    fn get(id: &<<Self as Dex<'a>>::DexType as Identifiable<'a>>::Id) -> &'a Self::DexType {
        Self::try_get(id).unwrap()
    }

    fn try_get(id: &<<Self as Dex<'a>>::DexType as Identifiable<'a>>::Id) -> Option<&'a Self::DexType> {
        Self::dex().as_ref().map(|dex| dex.get(id)).flatten()
    }

    fn len() -> usize {
        Self::dex().as_ref().map(|dex| dex.len()).unwrap_or_default()
    }

    fn with_capacity(capacity: usize) -> HashMap<<<Self as Dex<'a>>::DexType as Identifiable<'a>>::Id, Self::DexType> {
        HashMap::with_capacity(capacity)
    }

}