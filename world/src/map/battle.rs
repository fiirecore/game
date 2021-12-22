use firecore_pokedex::pokemon::{owned::SavedPokemon, party::Party};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    character::npc::{Npc, NpcId},
    positions::Location,
};

use super::{manager::state::WorldBattleState, wild::WildEntry};

#[derive(Debug, Clone)]
pub struct BattleEntry {
    pub party: Party<SavedPokemon>,
    pub active: usize,
    pub trainer: Option<TrainerEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainerEntry {
    pub id: NpcId,
    pub location: Location,
}

impl BattleEntry {
    pub fn wild(random: &mut impl Rng, wild: &WildEntry) -> Self {
        let mut party = Party::new();
        party.push(wild.generate(random));
        Self {
            party,
            active: 1,
            trainer: None,
        }
    }

    pub fn trainer(
        world: &mut WorldBattleState,
        map: &Location,
        id: &NpcId,
        npc: &Npc,
    ) -> Option<Self> {
        if let Some(trainer) = npc.trainer.as_ref() {
            if !world.battled(map, id) {
                return Some(BattleEntry {
                    party: trainer.party.clone(),
                    active: 1,
                    trainer:  Some(TrainerEntry {
                        id: *id,
                        location: *map,
                    }),
                });
            }
        }
        None
    }
}
