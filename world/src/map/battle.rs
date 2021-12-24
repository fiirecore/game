use firecore_pokedex::pokemon::{owned::SavedPokemon, party::Party};
use serde::{Deserialize, Serialize};

use crate::{
    character::npc::{Npc, NpcId},
    positions::Location,
};

use super::manager::state::WorldBattleState;

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
                    trainer: Some(TrainerEntry {
                        id: *id,
                        location: *map,
                    }),
                });
            }
        }
        None
    }
}
