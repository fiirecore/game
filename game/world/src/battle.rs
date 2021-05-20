use firecore_game::deps::tinystr::TinyStr16;
use firecore_game::storage::player::PlayerSave;
use firecore_world_lib::map::MapIdentifier;
use game::{
    deps::hash::HashSet,
    pokedex::{
        pokemon::{
            pokedex_len,
            instance::PokemonInstance,
            party::PersistentParty,
            stat::StatSet,
        },
    },
    storage::data,
    battle::{BattleEntry, BattleEntryRef, BattleTrainerEntry, BattleTeam},
    macroquad::prelude::warn
};

use world::{
    map::wild::WildEntry,
    character::npc::{NPC, NPCId},
};

#[deprecated]
pub static mut WORLD_TRAINER_DATA: Option<WorldTrainerData> = None;

pub struct WorldTrainerData {
    id: NPCId,
    disable_others: HashSet<NPCId>,
    map: TinyStr16,
}

pub fn random_wild_battle(battle: &mut Option<BattleEntry>) {
    let mut party = PersistentParty::new();
    let size = 2;
    for _ in 0..size {
        party.push(PokemonInstance::generate(
            world::map::wild::WILD_RANDOM.gen_range(0, pokedex_len()) + 1, 
            1, 
            100, 
            Some(StatSet::random())
        ));
    }    
    *battle = Some(BattleEntry {
        party,
        trainer: None,
        size,
    });
}

pub fn wild_battle(battle: BattleEntryRef, wild: &WildEntry) {
    let mut party = PersistentParty::new();
    party.push(wild.generate());
    *battle = Some(BattleEntry {
        party,
        trainer: None,
        size: 1,
    });
}

pub fn trainer_battle(battle: BattleEntryRef, npc: &NPC, map_id: &MapIdentifier, npc_id: &NPCId) {
    if let Some(trainer) = npc.trainer.as_ref() {
        let save = data();
        if let Some(map) = save.world.map.get(map_id) {
            if !map.battled.contains(npc_id) {
                let npc_type = crate::npc::npc_type(&npc.npc_type);
                if let Some(trainer_type) = npc_type.trainer.as_ref() {
                    *battle = Some(
                        BattleEntry {
                            party: trainer.party.iter().cloned().map(|instance| instance).collect(),
                            trainer: Some(
                                BattleTrainerEntry {
                                    prefix: trainer_type.name.clone(),
                                    name: npc.name.clone(),
                                    transition: trainer.battle_transition,
                                    texture: crate::map::texture::npc::NPCTextureManager::trainer_texture(&npc.npc_type),
                                    is_gym_leader: trainer_type.gym_leader,
                                    victory_message: trainer.victory_message.clone(),
                                    worth: trainer.worth,
                                }
                            ),
                            size: 1,
                        }
                    );
                    unsafe {
                        WORLD_TRAINER_DATA = Some(
                            WorldTrainerData {
                                id: *npc_id,
                                disable_others: trainer.disable.clone(),
                                map: *map_id,
                            }
                        )
                    }
                }
            }
        }
    }
}

pub fn update_world(world_manager: &mut crate::map::manager::WorldManager, player: &mut PlayerSave, winner: BattleTeam, trainer: bool) {
    if let Some(world) = unsafe{WORLD_TRAINER_DATA.take()} {
        match winner {
            BattleTeam::Player => {
                if trainer {
                    let battled = &mut player.world.get_map(&world.map).battled;
                    battled.insert(world.id);
                    for npc in world.disable_others {
                        battled.insert(npc);
                    }
                }                	
            }
            BattleTeam::Opponent => {
                player.location = player.world.heal.0;
                world_manager.map_manager.player.character.position = player.world.heal.1;
                world_manager.map_manager.chunk_active = if let Some(map) = player.location.map {
                    world_manager.map_manager.map_set_manager.current = Some(map);
                    if let Some(set) = world_manager.map_manager.map_set_manager.set_mut() {
                        set.current = Some(player.location.index);
                    } else {
                        warn!("Could not warp to map index {} under {}", player.location.index, map);
                    }
                    false
                } else {
                    world_manager.map_manager.chunk_map.current = Some(player.location.index);
                    true
                }
            }
        }
    }    
}