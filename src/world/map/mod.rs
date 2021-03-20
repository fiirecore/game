use crate::audio::{play_music_named, play_music};
use crate::data::player::list::PlayerSaves;
use firecore_world::character::Character;
use firecore_world::character::npc::NPCId;
use firecore_world::map::manager::test_move_code;
use firecore_world::script::world::WorldScript;
use macroquad::prelude::KeyCode;
use macroquad::prelude::collections::storage::{get, get_mut};
use macroquad::prelude::debug;
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::warn;

use firecore_util::text::MessageSet;
use firecore_util::text::TextColor;
use firecore_world::map::World;
use firecore_util::Destination;
use firecore_world::character::player::PlayerCharacter;
use firecore_world::map::WorldMap;
use firecore_world::script::world::Condition;
use firecore_world::script::world::WorldActionKind;
use firecore_world::map::wild::WildEntry;

use firecore_util::Entity;
use firecore_input::{self as input, Control};

use firecore_util::Completable;

use super::npc::WorldNpc;
use super::gui::map_window_manager::MapWindowManager;
use super::NpcTextures;
use super::TileTextures;
use super::RenderCoords;
use super::GameWorld;

pub mod manager;
pub mod set;
pub mod chunk;

pub trait GameWorldMap {

    fn tile_row(&self, x: isize, offset: isize) -> u16;

}

impl GameWorldMap for WorldMap {

    fn tile_row(&self, x: isize, offset: isize) -> u16 {
        self.tile_map[x as usize + offset as usize]
    }

}

impl GameWorld for WorldMap {

    fn on_tile(&mut self, player: &mut PlayerCharacter) {
        let tile_id = self.tile(player.position.local.coords);

        if let Some(wild) = &self.wild {
            if let Some(tiles) = &wild.tiles {
                tiles.iter().for_each(|tile| {
                    if tile_id.eq(tile) {
                        try_wild_battle(wild);
                    }
                });
            } else {
                try_wild_battle(wild);
            }            
        }

        // look for player

        for (npc_index, npc) in self.npcs.iter_mut() {
            if npc.trainer.is_some() {
                if let Some(saves) = get::<PlayerSaves>() {
                    if !saves.get().has_battled(&self.name, &npc.identifier.name) {
                        if npc.find_player(player.position.local.coords, player) {
                            self.npc_active = Some(*npc_index);
                        }
                    }
                }
            }            
        }
        if let Some(mut saves) = get_mut::<PlayerSaves>() {
            let player_data = saves.get_mut();
            for script in self.scripts.iter_mut() {

                if script.test_pos(&player.position.local.coords) {
                    let mut break_script = false;
                    for condition in &script.conditions {
                        match condition {
                            Condition::Scripts(scripts) => {
                                for script_condition in scripts {
                                    if player_data.world_status.ran_scripts.contains(&script_condition.identifier).ne(&script_condition.happened) {
                                        break_script = true;
                                    }  
                                }                          
                            },
                            Condition::PlayerHasPokemon(is_true) => {
                                if player_data.party.pokemon.is_empty().eq(is_true) {
                                    break_script = true;
                                }
                            }
                        }
                        if break_script {
                            break;
                        }
                    }
                    if !break_script {
                        debug!("Attempting to spawn script \"{}\"!", script.identifier);
                        script.spawn();
                    }                
                }
            }
        }
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, window_manager: &mut MapWindowManager) {

        for script in self.scripts.iter_mut() {

            if script.is_alive() {
                let mut pop = false;
                match script.actions_clone.front() {
                    Some(action) => {
                        match action {
                            WorldActionKind::Wait(time) => {
                                if script.timer.is_alive() {
                                    script.timer.update(delta);
                                    if script.timer.is_finished() {
                                        script.timer.despawn();
                                        pop = true;
                                    }
                                } else {
                                    script.timer.spawn();
                                    script.timer.set_target(*time);
                                    script.timer.update(delta);
                                }
                            },
                            WorldActionKind::PlayMusic(music) => {
                                play_music_named(&music);
                                // if let Err(err) = play_music_named(&music) {
                                //     warn!("Could not play music named {} for script {} with error {}", music, script.identifier, err);
                                // }
                                pop = true;
                            },
                            WorldActionKind::PlayMapMusic => {
                                play_music(self.music);
                                // if let Err(err) = play_music_id(self.music) {
                                    // warn!("Could not play music id {:x} for script {} with error {}", self.music, script.identifier, err);
                                // }
                                pop = true;
                            },
                            WorldActionKind::PlaySound(sound) => {
                                if let Err(err) = firecore_audio::play_sound(sound.clone()) {
                                    warn!("Could not play sound {:?} for script {} with error {}", sound, script.identifier, err);
                                }
                                pop = true;
                            }
                            WorldActionKind::PlayerFreezeInput => {
                                player.input_frozen = true;
                                player.stop_move();
                                pop = true;
                            },
                            WorldActionKind::PlayerUnfreezeInput => {
                                player.input_frozen = false;
                                pop = true;
                            }
                            WorldActionKind::PlayerUnfreeze => {
                                player.properties.frozen = false;
                                pop = true;
                            }
                            WorldActionKind::PlayerLook(direction) => {
                                player.position.local.direction = *direction;
                                pop = true;
                            }
                            WorldActionKind::PlayerMove(destination) => {
                                if player.properties.destination.is_some() {
                                    if player.should_move_to_destination() {
                                        player.move_to_destination(delta);
                                    } else {
                                        player.properties.destination = None;
                                        pop = true;
                                    }
                                } else {
                                    player.properties.destination = Some(*destination);
                                }
                            }
                            WorldActionKind::PlayerGivePokemon(instance) => {
                                if let Some(mut saves) = get_mut::<PlayerSaves>() {
                                    if saves.get().party.pokemon.len() < 6 {
                                        saves.get_mut().party.pokemon.push(instance.clone());
                                    } else {
                                        warn!("Could not add pokemon #{} to player party because it is full", instance.id);
                                    }
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCSpawn(npc) => {
                                self.npcs.insert(npc.identifier.index, npc.clone());
                                pop = true;
                                info!("Script spawned NPC {} with id {}", &npc.identifier.name, npc.identifier.index);
                            }
                            WorldActionKind::NPCLook(id, direction) => {
                                if let Some(npc) = self.npcs.get_mut(id) {
                                    npc.position.direction = *direction;
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCMove( id, pos ) => {
                                if let Some(npc) = self.npcs.get_mut(id) {
                                    if npc.properties.character.destination.is_some() {
                                        if npc.should_move_to_destination() {
                                            npc.move_to_destination(delta);
                                        } else {
                                            npc.properties.character.destination = None;
                                            pop = true;
                                        }
                                    } else {
                                        npc.walk_to(&pos.coords);
                                    }
                                } else {
                                    warn!("NPC script tried to move an unknown NPC (with id {})", id);
                                    pop = true;
                                }
                            },
                            WorldActionKind::NPCLeadPlayer( id, pos ) => {
                                if let Some(npc) = self.npcs.get_mut(id) {
                                    if npc.properties.character.destination.is_some() {
                                        if npc.should_move_to_destination() {
                                            npc.move_to_destination(delta);
                                        } else {
                                            npc.properties.character.destination = None;
                                            if player.properties.destination.is_none() {
                                                pop = true;
                                            }
                                        }
                                    } else {
                                        if npc.position.coords != pos.coords {
                                            npc.walk_to(&pos.coords);
                                        }
                                    }
                                    if player.properties.destination.is_some() {
                                        if player.should_move_to_destination() {
                                            player.move_to_destination(delta);
                                        } else {
                                            player.properties.destination = None;
                                            if npc.properties.character.destination.is_none() {
                                                pop = true;
                                            }
                                        }
                                    } else {
                                        if player.position.local.coords.ne(&pos.coords) {
                                            player.properties.destination = Some(Destination::next_to(&player.position.local, &pos.coords));
                                        }
                                    }
                                } else {
                                    warn!("NPC script tried to lead player with an unknown NPC (with id {})", id);
                                    pop = true;
                                }
                            }
                            WorldActionKind::NPCMoveToPlayer(id) => {
                                if let Some(npc) = self.npcs.get_mut(id) {
                                    if npc.properties.character.destination.is_some() {
                                        if npc.should_move_to_destination() {
                                            npc.move_to_destination(delta);
                                        } else {
                                            npc.properties.character.destination = None;
                                            pop = true;
                                        }
                                    } else {
                                        npc.walk_next_to(&player.position.local.coords)
                                    }
                                } else {
                                    warn!("NPC script tried to move to player with an unknown NPC (with id {})", id);
                                    pop = true;
                                }
                            }
                            WorldActionKind::NPCRespawn(id) => {
                                // match self.despawned_npcs.remove(id) {
                                //     Some(npc) => {
                                //         // self.npcs.insert(*id, npc);
                                //         info!("Script Respawned NPC with id {}", id);
                                //     }
                                //     None => {
                                //         warn!("Script attempted to respawn npc that doesn't exist!");
                                //     }
                                // }
                                pop = true;
                            }
                            WorldActionKind::NPCDespawn(id) => {
                                match self.npcs.remove(id) {
                                    Some(npc) => {
                                        // self.despawned_npcs.insert(*id, npc);
                                        info!("Script Despawned NPC with id {}", id);
                                    }
                                    None => {
                                        warn!("Script attempted to despawn npc that doesn't exist!");
                                    }
                                }                                
                                pop = true;
                            }
                            WorldActionKind::NPCInteract(id) => {
                                if let Some(npc) = self.npcs.get_mut(id) {
                                    if npc.interact_from(&player.position.local) {
                                        self.npc_active = Some(*id);
                                    }
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCBattle(id) => {
                                if let Some(npc) = self.npcs.get(id) {
                                    if npc.trainer.is_some() {
                                        crate::util::battle_data::trainer_battle(&npc);
                                    }
                                }
                                pop = true;
                            }
                            WorldActionKind::DisplayText(message_set) => {
                                if window_manager.is_alive() {
                                    if window_manager.is_finished() {
                                        window_manager.despawn();
                                        pop = true;
                                    } else {
                                        window_manager.update(delta);
                                    }
                                } else {
                                    window_manager.spawn();
                                    window_manager.set_text(message_set.clone());
                                }
                            }

                            WorldActionKind::Warp(_, _) => {},
                        }
                    }
                    None => {
                        despawn_script(script);
                    }
                }
                if pop {
                    script.actions_clone.pop_front();
                }
            }
        }

        

        if window_manager.is_alive() {
            if window_manager.is_finished() {
                if let Some(npc) = if let Some(index) = self.npc_active.take() {
                    self.npcs.get_mut(&index)
                } else {
                    None
                } {
                    super::npc::try_battle(&self.name, npc);
                }
                window_manager.despawn();
            } else {
                window_manager.update(delta);
            }
        } else {
            if let Some(npc) = if let Some(index) = self.npc_active {
                self.npcs.get_mut(&index)
            } else {
                None
            } {
                if npc.should_move_to_destination() {
                    npc.move_to_destination(delta) 
                } else {
                    window_manager.spawn();
                    npc.properties.character.destination = None;

                    let mut message_ran = false;

                    if let Some(message_set) = npc.properties.message.as_ref() {
                        window_manager.set_text(message_set.clone());
                        message_ran = true;
                    }
                    
                    if let Some(mut saves) = get_mut::<PlayerSaves>() {
                        if !saves.get_mut().world_status.get_or_create_map_data(&self.name).battled.contains(&npc.identifier.name) {
                            if let Some(trainer) = npc.trainer.as_ref() {

                                // Spawn text window

                                let message_set = MessageSet::new(
                                    1, 
                                    TextColor::Blue, 
                                    trainer.encounter_message.clone()
                                );
                                window_manager.set_text(message_set);
                                message_ran = true;

                                // Play Trainer music

                                if let Some(npc_type) = super::npc::NPC_TYPES.get(&npc.identifier.npc_type) {
                                    if let Some(trainer) = npc_type.trainer.as_ref() {
                                        if let Err(err) = if let Some(playing_music) = firecore_audio::get_current_music() {
                                            if playing_music != firecore_audio::get_music_id(&trainer.encounter_music).unwrap() {
                                                firecore_audio::play_music_named(&trainer.encounter_music)
                                            } else {
                                                Ok(())
                                            }
                                        } else {
                                            firecore_audio::play_music_named(&trainer.encounter_music)
                                        } {
                                            warn!("Could not play music named {} with error {}", self.name, err);
                                        }
                                    }
                                }
                            }   
                        }
                    }

                    if !message_ran {
                        window_manager.despawn();
                        self.npc_active = None;
                    } else {
                        window_manager.on_start();
                    }

                    player.position.local.direction = npc.position.direction.inverse();
                    if player.is_frozen() {
                        player.unfreeze();
                    }

                }
            } 
        }
        // self.script_manager.update(delta, player);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        for yy in screen.top..screen.bottom {
            let y = yy - screen.tile_offset.y;
            let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
            
            let row_offset = y * self.width as isize;
            
            for xx in screen.left..screen.right {
                let x = xx - screen.tile_offset.x;
                let render_x = (xx << 4) as f32 - screen.focus.x;

                if !(x < 0 || y < 0 || y >= self.height as isize || x >= self.width as isize) {
                    tile_textures.render_tile(&self.tile_row(x, row_offset), render_x, render_y);
                } else if border {
                    if x % 2 == 0 {
                        if y % 2 == 0 {
                            tile_textures.render_tile(&self.border_blocks[0], render_x, render_y);
                        } else {
                            tile_textures.render_tile(&self.border_blocks[2], render_x, render_y);
                        }
                    } else {
                        if y % 2 == 0 {
                            tile_textures.render_tile(&self.border_blocks[1], render_x, render_y);
                        } else {
                            tile_textures.render_tile(&self.border_blocks[3], render_x, render_y);
                        }
                    }
                }
            }
        }
        for npc in self.npcs.values() {
            npc.render(npc_textures, &screen);
        }
        // self.script_manager.render(tile_textures, npc_textures, &screen);
    }

    fn input(&mut self, _delta: f32, player: &mut PlayerCharacter) {

        if crate::debug() {
            if is_key_pressed(KeyCode::F7) {
                player.freeze();
                player.unfreeze();
                player.properties.noclip = true;
                info!("Unfroze player!");
            }
            // if is_key_pressed(KeyCode::F7) {
            //     info!("There are {} scripts in this map.", self.scripts.len());
            // }
            if is_key_pressed(KeyCode::F8) {
                for npc in self.npcs.values() {
                    info!("NPC {} is at {}, {}; looking {:?}", &npc.identifier.name, &npc.position.coords.x, &npc.position.coords.y, &npc.position.direction);
                }
            }
        }

        if input::pressed(Control::A) {
            for (npc_index, npc) in self.npcs.iter_mut() {
                if npc.interact_from(&player.position.local) {
                    self.npc_active = Some(*npc_index);
                }
            }
        }
    }


}

fn try_wild_battle(wild: &WildEntry) {
    if macroquad::rand::gen_range(0, 255) < wild.table.encounter_rate() {
        crate::util::battle_data::wild_battle(&wild.table);
    }
}

pub fn despawn_script(script: &mut WorldScript) {
    if let Some(mut saves) = get_mut::<PlayerSaves>() {
        saves.get_mut().world_status.ran_scripts.insert(script.identifier.clone());
    }
    script.despawn();
}