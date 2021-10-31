use pokedex::{
    pokemon::{
        owned::{OwnedPokemon, SavedPokemon},
        party::Party,
        stat::StatSet,
    },
    Dex,
};
use rand::Rng;
use saves::PlayerData;
use worldlib::{
    character::npc::{trainer::TrainerDisable, Npc, NpcId},
    map::{
        manager::{TrainerEntry, TrainerEntryRef},
        wild::WildEntry,
    },
    positions::Location,
    TrainerId,
};

use crate::game::battle_glue::{BattleEntry, BattleEntryRef, BattleId, BattleTrainerEntry};
use crate::world::{map::manager::WorldManager, npc::npc_type};

pub const DEFAULT_RANDOM_BATTLE_SIZE: usize = 2;

pub fn random_wild_battle(
    random: &mut impl Rng,
    pokedex: u16,
    battle: &mut Option<BattleEntry>,
    size: usize,
) {
    let mut party = Party::new();
    for _ in 0..size {
        let id = random.gen_range(0..pokedex) + 1;
        let ivs = StatSet::random_iv(random);
        let level = random.gen_range(1..=100);
        party.push(SavedPokemon::generate(random, id, level, None, Some(ivs)));
    }
    *battle = Some(BattleEntry {
        party,
        trainer: None,
        active: 1,
        id: BattleId(None),
        name: None,
    });
}

pub fn wild_battle(random: &mut impl Rng, battle: BattleEntryRef, wild: &WildEntry) {
    let mut party = Party::new();
    party.push(wild.generate(random));
    *battle = Some(BattleEntry {
        party,
        trainer: None,
        active: 1,
        id: BattleId(None),
        name: None,
    });
}

pub fn trainer_battle(
    save: &PlayerData,
    battle: BattleEntryRef,
    world: TrainerEntryRef,
    npc: &Npc,
    map_id: &Location,
    npc_id: &NpcId,
) {
    if let Some(trainer) = npc.trainer.as_ref() {
        if let Some(map) = save.world.map.get(&map_id.index) {
            if !map.battled.contains(npc_id) {
                let npc_type = npc_type(&npc.type_id);
                if let Some(trainer_type) = npc_type.trainer.as_ref() {
                    *battle = Some(BattleEntry {
                        id: BattleId(Some(npc_id.as_str().parse().unwrap())),
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
                    *world = Some(TrainerEntry {
                        id: *npc_id,
                        disable_others: trainer.disable.clone(),
                        map: *map_id,
                    })
                }
            }
        }
    }
}

impl WorldManager {
    pub fn update_world(&mut self, player: &mut PlayerData, winner: TrainerId, trainer: bool) {
        if let Some(entry) = self.world.battling.take() {
            if winner == player.id {
                if trainer {
                    let battled = &mut player.world.get_map(&entry.map).battled;
                    match entry.disable_others {
                        TrainerDisable::DisableSelf => {
                            battled.insert(entry.id);
                        }
                        TrainerDisable::Many(others) => {
                            battled.insert(entry.id);
                            battled.extend(others);
                        }
                        TrainerDisable::None => (),
                    }
                }
            } else {
                player.location = player.world.heal.0;
                self.world.player.position = player.world.heal.1;
                self.world.location = Some(player.location);
                player.party.iter_mut().for_each(|o |o.heal(None, None));
            }
        }
    }
}
