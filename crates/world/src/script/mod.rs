use core::ops::Deref;

use firecore_pokedex::trainer::InitTrainer;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use crate::{map::data::WorldMapData, state::map::MapState, random::WorldRandoms};

pub mod default;

pub trait WorldScriptingEngine {
    type State: Default + Clone + std::fmt::Debug + serde::Serialize + serde::de::DeserializeOwned;

    type Error;

    fn on_tile(&self);

    fn update<
        R: rand::Rng,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &self,
        data: &WorldMapData,
        map: &mut MapState,
        trainer: &mut InitTrainer<P, M, I>,
        randoms: &mut WorldRandoms<R>,
        state: &mut Self::State,
    ) where
        Self: Sized;
}
