use serde::{Deserialize, Serialize};

use deps::vec::ArrayVec;

use crate::{pokemon::party::Party, trainer::TrainerData};

pub mod knowable;
pub mod battle;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleParty<ID, A, P> {
    pub id: ID,
    pub trainer: Option<TrainerData>,
    pub active: Vec<A>,
    pub pokemon: Party<P>,
}

impl<ID: Default, A, P> Default for BattleParty<ID, A, P> {
    fn default() -> Self {
        Self {
            id: Default::default(),
            trainer: Default::default(),
            active: Default::default(),
            pokemon: ArrayVec::default(),
        }
    }
}

impl<ID, A, P> BattleParty<ID, A, P> {
    pub fn default_with_id(id: ID) -> Self {
        Self {
            id,
            trainer: Default::default(),
            active: Default::default(),
            pokemon: ArrayVec::default(),
        }
    }
}