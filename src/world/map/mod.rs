use crate::battle::data::BattleData;
use crate::util::{play_music_named, play_music};
use crate::data::player::list::PlayerSaves;
use firecore_util::text::Message;
use firecore_world::character::Character;
use firecore_world::script::world::WorldScript;
use macroquad::prelude::KeyCode;
use macroquad::prelude::collections::storage::{get, get_mut};
use macroquad::prelude::{info, warn, is_key_pressed};

use firecore_util::text::TextColor;
use firecore_world::map::World;
use firecore_util::Destination;
use firecore_world::character::player::PlayerCharacter;
use firecore_world::map::WorldMap;
use firecore_world::script::world::Condition;
use firecore_world::script::world::WorldActionKind;
use firecore_world::map::wild::WildEntry;

use firecore_util::Entity;
use firecore_input::{pressed, Control};

use firecore_util::Completable;

use super::NPCTypes;
use super::npc::WorldNpc;
use super::gui::text_window::TextWindow;
use super::{GameWorld, TileTextures, NpcTextures, GuiTextures, RenderCoords};

pub mod manager;
pub mod set;
pub mod chunk;

impl GameWorld for WorldMap {

    fn on_start(&self, music: bool) {
        if music {
            if firecore_audio::get_current_music().map(|current| current != self.music).unwrap_or(true) {
                play_music(self.music);
            }
        }
    }

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        if let Some(tile_id) = self.tile(player.position.local.coords) {

            if let Some(wild) = &self.wild {
                if let Some(tiles) = wild.tiles.as_ref() {
                    for tile in tiles.iter() {
                        if tile_id.eq(tile) {
                            try_wild_battle(battle_data, wild);
                            break;
                        }
                    }
                } else {
                    try_wild_battle(battle_data, wild);
                }            
            }
    
            // look for player
    
            for (npc_index, npc) in self.npcs.iter_mut() {
                if npc.trainer.is_some() {
                    if let Some(saves) = get::<PlayerSaves>() {
                        if !saves.get().has_battled(&self.name, &npc.identifier.index) && saves.get().party.iter().filter(|pokemon| pokemon.current_hp.map(|hp| hp != 0).unwrap_or(true)).next().is_some() {
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
    
                    if !script.is_alive() && script.in_location(&player.position.local.coords) {
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
                                    if player_data.party.is_empty().eq(is_true) {
                                        break_script = true;
                                    }
                                }
                                _ => {
                                    break_script = true;
                                }
                            }
                            if break_script {
                                break;
                            }
                        }
                        if !break_script {
                            // debug!("Attempting to spawn script \"{}\"!", script.identifier);
                            script.spawn();
                        }                
                    }
                }
            }

        }
    }

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, text_window: &mut TextWindow, npc_types: &NPCTypes) {

        for script in self.scripts.iter_mut() {

            if script.is_alive() {
                let mut pop = false;
                match script.actions.front() {
                    Some(action) => {
                        match action {
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
                            WorldActionKind::PlayerGivePokemon(saved) => {
                                if let Some(mut saves) = get_mut::<PlayerSaves>() {
                                    if saves.get().party.len() < 6 {
                                        saves.get_mut().party.push(saved.clone());
                                    } else {
                                        warn!("Could not add pokemon #{} to player party because it is full", saved.id);
                                    }
                                }
                                pop = true;
                            }
                            WorldActionKind::PlayerHealPokemon => {
                                if let Some(mut saves) = get_mut::<PlayerSaves>() {
                                    for pokemon in saves.get_mut().party.iter_mut() {
                                        pokemon.current_hp = None;
                                        if let Some(moves) = pokemon.moves.as_mut() {
                                            for saved_move in moves {
                                                saved_move.pp = None;
                                            }
                                        }
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
                                        crate::battle::data::trainer_battle(battle_data, &npc, npc_types);
                                    }
                                }
                                pop = true;
                            }

                            WorldActionKind::Wait(time) => {
                                if script.timer.is_alive() {
                                    script.timer.update(delta);
                                    if script.timer.is_finished() {
                                        script.timer.despawn();
                                        pop = true;
                                    }
                                } else {
                                    script.timer.hard_reset();
                                    script.timer.spawn();
                                    script.timer.set_target(*time);
                                    script.timer.update(delta);
                                }
                            },

                            WorldActionKind::Info(string) => {
                                info!("{}: {}", script.identifier, string);
                                pop = true;
                            }

                            WorldActionKind::DisplayText(messages) => {
                                if display_text(delta, text_window, messages) {
                                    pop = true;
                                }
                            },

                            WorldActionKind::Conditional { messages, end_messages, unfreeze } => {

                                /*
                                * 0 = first message (default)
                                * 1 = end message
                                * 2 or 3 = yes/no option and cursor pos
                                */

                                if script.option == 0 {
                                    if text_window.is_alive() {
                                        if text_window.is_finished() {
                                            script.option = 2;
                                        } else {
                                            text_window.update(delta);
                                        }
                                    } else {
                                        text_window.spawn();
                                        text_window.set_text(messages.clone());
                                    }
                                } else if script.option == 1 {

                                    if end_messages.is_some() {

                                        if text_window.is_finished() {
                                            text_window.despawn();
                                            if *unfreeze {
                                                player.unfreeze();
                                            }
                                            script.option = 0;
                                            despawn_script(script);
                                        } else {
                                            text_window.update(delta);
                                        }

                                    } else {
                                        if *unfreeze {
                                            player.unfreeze();
                                        }
                                        script.option = 0;
                                        despawn_script(script);
                                    }
                                } else {
                                    if pressed(Control::A) {
                                        if script.option == 2 {
                                            script.option = 0;
                                            text_window.despawn();
                                            pop = true;
                                        } else if script.option == 3 {

                                            script.option = 1;
                                            if let Some(end_messages) = end_messages {
                                                text_window.reset_text();
                                                text_window.set_text(end_messages.clone());
                                            }

                                        }
                                    } else if pressed(Control::B) {

                                        script.option = 1;
                                        if let Some(end_messages) = end_messages {
                                            text_window.reset_text();
                                            text_window.set_text(end_messages.clone());
                                        }

                                    }
                                    if pressed(Control::Up) && script.option == 3 {
                                        script.option = 2;
                                    }
                                    if pressed(Control::Down) && script.option == 2 {
                                        script.option = 3;
                                    }
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
                    script.actions.pop_front();
                }
            }
        }

        // Npc window manager code

        if let Some(npc) = if let Some(index) = self.npc_active {
            self.npcs.get_mut(&index)
        } else {
            None
        } {
            if text_window.is_alive() {
                if text_window.is_finished() {
                    {
                        self.npc_active = None;
                        super::npc::try_battle(battle_data, &self.name, npc, npc_types);
                    }
                    text_window.despawn();
                } else {
                    text_window.update(delta);
                }
            } else {
                if npc.should_move_to_destination() {
                    npc.move_to_destination(delta) 
                } else {
                    text_window.spawn();
                    npc.properties.character.destination = None;
    
                    let mut message_ran = false;
    
                    if let Some(messages) = npc.properties.message.as_ref() {
                        text_window.set_text(messages.clone());
                        message_ran = true;
                    }
                    
                    if let Some(mut saves) = get_mut::<PlayerSaves>() {
                        if !saves.get_mut().world_status.get_or_create_map_data(&self.name).battled.contains(&npc.identifier.index) {
                            if let Some(trainer) = npc.trainer.as_ref() {
    
                                if !trainer.dont_battle_on_interact {

                                    // Spawn text window
                                        
                                    let messages = trainer.encounter_message.iter().map(|message| {
                                        Message::new(
                                            message.clone(),
                                            TextColor::Blue,
                                            None,
                                        )
                                    }).collect();
                                    text_window.set_text(messages);
                                    message_ran = true;

                                    // Play Trainer music

                                    if let Some(npc_type) = npc_types.get(&npc.identifier.npc_type) {
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
                    }
    
                    if !message_ran {
                        text_window.despawn();
                        self.npc_active = None;
                    } else {
                        if let Some(saves) = get::<PlayerSaves>() {
                            text_window.on_start(saves.get());
                        }
                    }
    
                    player.position.local.direction = npc.position.direction.inverse();
                    if player.is_frozen() {
                        player.unfreeze();
                    }
                } 
            }
        }
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, npc_types: &NPCTypes, gui_textures: &GuiTextures, screen: RenderCoords, border: bool) {
        for yy in screen.top..screen.bottom {
            let y = yy - screen.offset.y;
            let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
            
            for xx in screen.left..screen.right {
                let x = xx - screen.offset.x;
                let render_x = (xx << 4) as f32 - screen.focus.x;

                if !(x < 0 || y < 0 || y >= self.height as isize || x >= self.width as isize) {
                    tile_textures.render_tile(&self.tile_or_panic(x as usize, y as usize), render_x, render_y);
                } else if border {
                    if x % 2 == 0 {
                        if y % 2 == 0 {
                            tile_textures.render_tile(&self.border.tiles[0], render_x, render_y);
                        } else {
                            tile_textures.render_tile(&self.border.tiles[2], render_x, render_y);
                        }
                    } else {
                        if y % 2 == 0 {
                            tile_textures.render_tile(&self.border.tiles[1], render_x, render_y);
                        } else {
                            tile_textures.render_tile(&self.border.tiles[3], render_x, render_y);
                        }
                    }
                }
            }
        }
        for npc in self.npcs.values() {
            npc.render(npc_textures, npc_types, &screen);
        }
        for script in self.scripts.iter() {
            if script.is_alive() {
                if let Some(action) = script.actions.front() {
                    match action {
                        WorldActionKind::Conditional{ .. } => {
                            if let Some(texture) = gui_textures.get(&0) {
                                if script.option > 1 {
                                    crate::util::graphics::draw(*texture, 162.0, 66.0);
                                    crate::util::graphics::draw_cursor(170.0, 77.0 + (script.option - 2) as f32 * 16.0);
                                }                                
                            }
                        }
                        _ => (),
                    }                    
                }
            }            
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

        if pressed(Control::A) {
            for (npc_index, npc) in self.npcs.iter_mut() {
                if npc.interact_from(&player.position.local) {
                    self.npc_active = Some(*npc_index);
                }
            }
            for script in self.scripts.iter_mut() {
                if !script.is_alive() {
                    if script.in_location(&player.position.local.coords) {
                        let mut spawn = false;
                        for condition in script.conditions.iter() {
                            match condition {
                                Condition::Activate(direction) => {
                                    if player.position.local.direction.eq(direction) {
                                        spawn = true;
                                    }
                                }
                                _ => (),
                            }
                        }
                        if spawn {
                            script.spawn();
                        }
                    }
                }                
            }
        }
    }


}

// #[deprecated(note = "move this function")]
fn try_wild_battle(battle_data: &mut Option<BattleData>, wild: &WildEntry) { // move
    if wild.table.try_encounter() {
        crate::battle::data::wild_battle(battle_data, &wild.table);
    }
}

pub fn despawn_script(script: &mut WorldScript) {
    if let Some(mut saves) = get_mut::<PlayerSaves>() {
        saves.get_mut().world_status.ran_scripts.insert(script.identifier.clone());
    }
    script.despawn();
}

fn display_text(delta: f32, text_window: &mut TextWindow, messages: &Vec<Message>) -> bool {
    if text_window.is_alive() {
        if text_window.is_finished() {
            text_window.despawn();
            return true;
        } else {
            text_window.update(delta);
        }
    } else {
        text_window.spawn();
        text_window.set_text(messages.clone());
        if let Some(saves) = get::<PlayerSaves>() {
            text_window.on_start(saves.get());
        }
        
    }
    false
}