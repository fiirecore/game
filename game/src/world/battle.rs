use crate::{
    deps::{
        str::TinyStr16,
        hash::HashSet,
    },
    pokedex::{
        moves::target::Team,
        pokemon::{
            dex::pokedex_len,
            instance::PokemonInstance,
            party::PersistentParty,
            stat::StatSet,
        },
    },
    storage::{data, player::PlayerSave},
    battle_glue::{BattleEntry, BattleEntryRef, BattleTrainerEntry},
    log::warn,
};

use worldlib::{
    map::{
        MapIdentifier,
        wild::{WildEntry, WILD_RANDOM},
    },
    character::npc::{NPC, NPCId},
};

use crate::world::{
    npc::npc_type,
    map::{
        manager::WorldManager,
        texture::npc::NPCTextureManager,
    },
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
            WILD_RANDOM.gen_range(0, pokedex_len()) + 1, 
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
                let npc_type = npc_type(&npc.npc_type);
                if let Some(trainer_type) = npc_type.trainer.as_ref() {
                    *battle = Some(
                        BattleEntry {
                            party: trainer.party.iter().cloned().map(|instance| instance).collect(),
                            trainer: Some(
                                BattleTrainerEntry {
                                    prefix: trainer_type.name.clone(),
                                    name: npc.name.clone(),
                                    transition: trainer.battle_transition,
                                    texture: NPCTextureManager::trainer_texture(&npc.npc_type).clone(),
                                    gym_badge: trainer_type.badge,
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

pub fn update_world(world_manager: &mut WorldManager, player: &mut PlayerSave, winner: Team, trainer: bool) {
    if let Some(world) = unsafe{WORLD_TRAINER_DATA.take()} {
        match winner {
            Team::Player => {
                if trainer {
                    let battled = &mut player.world.get_map(&world.map).battled;
                    battled.insert(world.id);
                    for npc in world.disable_others {
                        battled.insert(npc);
                    }
                }                	
            }
            Team::Opponent => {
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