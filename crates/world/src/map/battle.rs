use serde::{Deserialize, Serialize};

use pokedex::{
    item::bag::SavedBag,
    pokemon::{owned::SavedPokemon, party::Party},
};

use crate::{
    character::npc::{Npc, NpcId},
    positions::Location,
    state::WorldBattleState,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleEntry {
    pub party: Party<SavedPokemon>,
    pub active: usize,
    pub trainer: Option<TrainerEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainerEntry {
    pub id: NpcId,
    pub location: Location,
    pub bag: SavedBag,
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
                        bag: trainer.bag.clone(),
                    }),
                });
            }
        }
        None
    }
}
