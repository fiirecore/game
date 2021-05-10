use firecore_game::deps::tinystr::TinyStr16;
use firecore_game::storage::player::PlayerSave;
use firecore_world_lib::map::MapIdentifier;
use game::{
    deps::{
        vec::ArrayVec,
        hash::HashSet,
    },
    pokedex::{
        pokemon::{
            PokemonId,
            pokedex,
            instance::{
                PokemonInstance,
                PokemonInstanceParty
            },
            data::StatSet,
            saved::SavedPokemonParty,
        },
    },
    storage::{get, player::PlayerSaves},
    battle::{BattleData, TrainerData, BattleTeam},
    macroquad::prelude::warn
};

use world::{
    map::wild::WildEntry,
    character::npc::{NPC, NPCId},
};

pub static mut WORLD_TRAINER_DATA: Option<WorldTrainerData> = None;

pub struct WorldTrainerData {
    id: NPCId,
    disable_others: HashSet<NPCId>,
    map: TinyStr16,
}

pub fn random_wild_battle(battle_data: &mut Option<BattleData>) {
    let mut party = PokemonInstanceParty::new();
    let size = 2;
    for _ in 0..size {
        party.push(PokemonInstance::generate(
            world::map::wild::WILD_RANDOM.gen_range(0, pokedex().len()) as PokemonId + 1, 
            1, 
            100, 
            Some(StatSet::random())
        ));
    }    
    *battle_data = Some(BattleData {
        party,
        trainer: None,
        size,
    });
}

pub fn wild_battle(battle_data: &mut Option<BattleData>, wild: &WildEntry) {
    let mut party = ArrayVec::new();
    party.push(wild.generate());
    *battle_data = Some(BattleData {
        party,
        trainer: None,
        size: 1,
    });
}

pub fn trainer_battle(battle_data: &mut Option<BattleData>, map_id: &MapIdentifier, npc_id: &NPCId, npc: &NPC) {
    if let Some(trainer) = npc.trainer.as_ref() {
        if let Some(saves) = get::<PlayerSaves>() {
            let save = saves.get();
            if let Some(map) = save.world.map.get(map_id) {
                if !map.battled.contains(npc_id) {
                    let npc_type = crate::npc::npc_type(&npc.npc_type);
                    *battle_data = Some(
                        BattleData {
                            party: to_battle_party(&trainer.party),
                            trainer: Some(
                                TrainerData {
                                    name: npc.name.clone(),
                                    npc_type: npc_type.map(|npc_type| npc_type.trainer.as_ref().map(|trainer| trainer.name.clone())).flatten().unwrap_or(String::from("Trainer")),
                                    texture: *crate::map::texture::npc::NPCTextureManager::trainer_textures().get(&npc.npc_type).unwrap(),
                                    worth: trainer.worth,
                                    battle_type: npc_type.map(|npc_type| npc_type.trainer.as_ref().map(|trainer| trainer.battle_type)).flatten().unwrap_or_default(),
                                    victory_message: trainer.victory_message.clone(),
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

pub fn to_battle_party(party: &SavedPokemonParty) -> PokemonInstanceParty {
    let mut battle_party = PokemonInstanceParty::new();
    for pokemon in party {
        if let Some(pokemon) = PokemonInstance::new(pokemon) {
            battle_party.push(pokemon)
        } else {
            warn!("Could not create battle pokemon from ID {}", pokemon.id);
        }
    }
    battle_party
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