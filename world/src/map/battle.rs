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
    // pub trainer: Option<TrainerEntry>,
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
            // trainer: None,
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
                world.battling = Some(TrainerEntry {
                    id: *id,
                    location: *map,
                });
                return Some(BattleEntry {
                    party: trainer.party.clone(),
                    // trainer: Some(BattleTrainerEntry {
                    //     transition: trainer.battle_transition,
                    //     sprite: data.trainer.get(&npc.type_id).clone(),
                    //     gym_badge: trainer_type.badge,
                    //     victory_message: trainer.victory_message.clone(),
                    //     worth: trainer.worth,
                    // }),
                    active: 1,
                });
            }
        }
        None
    }
}
