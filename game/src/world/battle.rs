use crate::{
    util::Location,
    pokedex::{
        moves::target::Team,
        pokemon::{
            dex::pokedex_len,
            instance::PokemonInstance,
            party::PokemonParty,
            stat::StatSet,
        },
    },
    storage::{data, player::PlayerSave},
    battle_glue::{BattleEntry, BattleEntryRef, BattleTrainerEntry},
};

use worldlib::{
    map::{
        wild::{WildEntry, WILD_RANDOM},
        manager::{TrainerEntry, TrainerEntryRef},
    },
    character::npc::{Npc, NpcId},
};

use crate::world::{
    npc::npc_type,
    map::{
        manager::WorldManager,
        texture::npc::NpcTextureManager,
    },
};

pub fn random_wild_battle(battle: &mut Option<BattleEntry>) {
    let mut party = PokemonParty::new();
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
    let mut party = PokemonParty::new();
    party.push(wild.generate());
    *battle = Some(BattleEntry {
        party,
        trainer: None,
        size: 1,
    });
}

pub fn trainer_battle(battle: BattleEntryRef, world: TrainerEntryRef, npc: &Npc, map_id: &Location, npc_id: &NpcId) {
    if let Some(trainer) = npc.trainer.as_ref() {
        let save = data();
        if let Some(map) = save.world.map.get(&map_id.index) {
            if !map.battled.contains(npc_id) {
                let npc_type = npc_type(&npc.npc_type);
                if let Some(trainer_type) = npc_type.trainer.as_ref() {
                    *battle = Some(
                        BattleEntry {
                            party: trainer.party.clone(),
                            trainer: Some(
                                BattleTrainerEntry {
                                    prefix: trainer_type.name.clone(),
                                    name: npc.name.clone(),
                                    transition: trainer.battle_transition,
                                    texture: NpcTextureManager::trainer_texture(&npc.npc_type).clone(),
                                    gym_badge: trainer_type.badge,
                                    victory_message: trainer.victory_message.clone(),
                                    worth: trainer.worth,
                                }
                            ),
                            size: 1,
                        }
                    );
                    *world = Some(
                        TrainerEntry {
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

pub fn update_world(world_manager: &mut WorldManager, player: &mut PlayerSave, winner: Team, trainer: bool) {
    if let Some(world) = world_manager.map_manager.data.battling.take() {
        match winner {
            Team::Player => {
                if trainer {
                    let battled = &mut player.world.get_map(&world.map).battled;
                    battled.insert(world.id);
                    battled.extend(world.disable_others);
                }                	
            }
            Team::Opponent => {
                player.location = player.world.heal.0;
                world_manager.map_manager.data.player.character.position = player.world.heal.1;
                world_manager.map_manager.data.current = Some(player.location);
            }
        }
    }    
}