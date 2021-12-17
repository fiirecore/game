use std::ops::Deref;

use crate::pokedex::pokemon::{owned::SavedPokemon, party::Party, stat::StatSet};
use firecore_battle::pokedex::{
    item::Item,
    moves::Move,
    pokemon::{owned::OwnedPokemon, Pokemon},
};
use rand::Rng;
use worldlib::{
    character::{
        npc::{trainer::TrainerDisable, Npc, NpcId},
        player::PlayerCharacter,
    },
    map::{
        manager::state::{
            default_heal_loc, TrainerEntry, TrainerEntryRef, WorldBattleState, WorldMapState,
        },
        wild::WildEntry,
    },
    positions::Location,
};

use crate::game::battle_glue::{BattleEntry, BattleId, BattleTrainerEntry};
use crate::world::manager::WorldManager;

use super::npc::NpcTypes;

pub const DEFAULT_RANDOM_BATTLE_SIZE: usize = 2;

pub fn random_wild_battle(random: &mut impl Rng, pokedex: u16, size: usize) -> BattleEntry {
    let mut party = Party::new();
    for _ in 0..size {
        let id = random.gen_range(0..pokedex) + 1;
        let ivs = StatSet::random_iv(random);
        let level = random.gen_range(1..=100);
        party.push(SavedPokemon::generate(random, id, level, None, Some(ivs)));
    }
    BattleEntry {
        party,
        trainer: None,
        active: 1,
        id: BattleId(None),
        name: None,
    }
}

pub fn wild_battle(random: &mut impl Rng, wild: &WildEntry) -> BattleEntry {
    let mut party = Party::new();
    party.push(wild.generate(random));
    BattleEntry {
        party,
        trainer: None,
        active: 1,
        id: BattleId(None),
        name: None,
    }
}

pub fn trainer_battle(
    npc_types: &NpcTypes,
    world: &mut WorldBattleState,
    map: &Location,
    id: &NpcId,
    npc: &Npc,
) -> Option<BattleEntry> {
    if let Some(trainer) = npc.trainer.as_ref() {
        if !world.battled(map, id) {
            if let Some(npc_type) = npc_types.get(&npc.type_id) {
                if let Some(trainer_type) = npc_type.trainer.as_ref() {
                    world.battling = Some(TrainerEntry {
                        id: *id,
                        disable_others: trainer.disable.clone(),
                        map: *map,
                    });
                    return Some(BattleEntry {
                        id: BattleId(Some(id.as_str().parse().unwrap())),
                        name: Some(format!("{} {}", trainer_type.name, npc.name)),
                        party: trainer.party.clone(),
                        trainer: Some(BattleTrainerEntry {
                            transition: trainer.battle_transition,
                            // texture: TrainerTextures::get(&npc.type_id).clone(),
                            gym_badge: trainer_type.badge,
                            victory_message: trainer.victory_message.clone(),
                            worth: trainer.worth,
                        }),
                        active: 1,
                    });
                }
            }
        }
    }
    None
}

impl WorldManager {
    pub fn update_world<
        P: Deref<Target = Pokemon>,
        M: Deref<Target = Move>,
        I: Deref<Target = Item>,
    >(
        &mut self,
        player: &mut PlayerCharacter,
        party: &mut [OwnedPokemon<P, M, I>],
        winner: bool,
        trainer: bool,
    ) {
        if let Some(entry) = player.world.battle.battling.take() {
            if winner {
                if trainer {
                    match entry.disable_others {
                        TrainerDisable::DisableSelf => {
                            player.world.battle.insert(&entry.map, entry.id);
                        }
                        TrainerDisable::Many(others) => {
                            player.world.battle.insert(&entry.map, entry.id);
                            player.world.battle.battled.get_mut(&entry.map).unwrap().extend(others);
                        }
                        TrainerDisable::None => (),
                    }
                }
            } else {
                let loc = player.world.heal.unwrap_or_else(default_heal_loc);
                player.location = loc.0;
                player.position = loc.1;
                player.location = player.location;
                party.iter_mut().for_each(|o| o.heal(None, None));
            }
        }
    }
}
