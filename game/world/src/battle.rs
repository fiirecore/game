use firecore_game::data::player::PlayerSave;
use game::{
    util::{
        smallvec::smallvec,
        hash::HashSet,
    },
    pokedex::{
        pokedex,
        pokemon::{
            PokemonId,
            instance::PokemonInstance,
            data::StatSet,
            GeneratePokemon,
            party::PokemonParty,
        }
    },
    data::{get, player::PlayerSaves},
    battle::{BattleData, TrainerData, BattlePokemonParty, BattleWinner},
    macroquad::prelude::warn
};

use world::{
    map::wild::{
        WildEntry,
        GenerateWild,
    },
    character::npc::{NPC, NPCId},
};

pub static mut WORLD_TRAINER_DATA: Option<WorldTrainerData> = None;

pub struct WorldTrainerData {
    id: NPCId,
    disable_others: HashSet<NPCId>,
    map: String,
}

pub fn random_wild_battle(battle_data: &mut Option<BattleData>) {
    *battle_data = Some(BattleData {
        party: smallvec![PokemonInstance::generate(
            world::map::wild::WILD_RANDOM.gen_range(0..pokedex().len() as u32) as PokemonId + 1, 
            1, 
            100, 
            Some(StatSet::random())
        )],
        trainer: None,
    });
}

pub fn wild_battle(battle_data: &mut Option<BattleData>, wild: &WildEntry) {
    if wild.table.try_encounter() {
        *battle_data = Some(BattleData {
            party: smallvec![wild.table.generate()],
            trainer: None,
        });
    }
}

pub fn trainer_battle(battle_data: &mut Option<BattleData>, map_name: &String, npc_index: NPCId, npc: &NPC) {
    if let Some(trainer) = npc.trainer.as_ref() {
        if let Some(saves) = get::<PlayerSaves>() {
            let save = saves.get();
            if let Some(map) = save.world_status.map_data.get(map_name) {
                if !map.battled.contains(&npc_index) {
                    let npc_type = crate::npc::npc_type(&npc.properties.npc_type);
                    *battle_data = Some(
                        BattleData {
                            party: to_battle_party(&trainer.party),
                            trainer: Some(
                                TrainerData {
                                    name: npc.name.clone(),
                                    npc_type: npc_type.map(|npc_type| npc_type.trainer.as_ref().map(|trainer| trainer.name.clone())).flatten().unwrap_or(String::from("Trainer")),
                                    texture: game::textures::trainer_texture(&npc.properties.npc_type),
                                    worth: trainer.worth,
                                    battle_type: npc_type.map(|npc_type| npc_type.trainer.as_ref().map(|trainer| trainer.battle_type)).flatten().unwrap_or_default(),
                                    victory_message: trainer.victory_message.clone(),
                                }
                            )
                        }
                    );
                    unsafe {
                        WORLD_TRAINER_DATA = Some(
                            WorldTrainerData {
                                id: npc_index,
                                disable_others: trainer.disable_others.clone(),
                                map: map_name.clone(),
                            }
                        )
                    }
                }
            }
        }
    }
}

pub fn to_battle_party(party: &PokemonParty) -> BattlePokemonParty {
    let mut battle_party = BattlePokemonParty::new();
    for pokemon in party {
        if let Some(pokemon) = PokemonInstance::new(pokemon) {
            battle_party.push(pokemon)
        } else {
            warn!("Could not create battle pokemon from ID {}", pokemon.id);
        }
    }
    battle_party
}

pub fn update_world(player: &mut PlayerSave, winner: BattleWinner, trainer: bool) {
    if let Some(world) = unsafe{WORLD_TRAINER_DATA.take()} {
        match winner {
            BattleWinner::Player => {
                if trainer {
                    let battled = &mut player.world_status.get_or_create_map_data(&world.map).battled;
                    battled.insert(world.id);
                    for npc in world.disable_others {
                        battled.insert(npc);
                    }
                }                	
            }
            BattleWinner::Opponent => {
    
            }
        }
    }    
}