use serde::{Deserialize, Serialize};

use crate::script::WorldScriptingEngine;

pub mod map;

// pub type SavedWorldState<R, S> = WorldState<R, S, SavedPokemon, SavedBag>;
// pub type InitWorldState<R, S, P, M, I> = WorldState<R, S, OwnedPokemon<P, M, I>, Bag<I>>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WorldState<S: WorldScriptingEngine> {
    /// Map State
    #[serde(default)]
    pub map: map::MapState,

    /// Script state
    #[serde(default)]
    pub scripts: S::State,
}

impl<S: WorldScriptingEngine> WorldState<S> {
    pub fn new(name: impl Into<String>, rival: impl Into<String>) -> Self {
        Self {
            map: map::MapState::new(name, rival),
            scripts: Default::default(),
        }
    }
}
