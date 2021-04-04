use firecore_util::{Entity, Completable, Direction, Destination, text::{Message, TextColor}};

use firecore_world::{
    character::{
        Character,
        movement::MovementType,
        npc::{NPC, NPCId},
        player::PlayerCharacter,
    },
    map::{
        World,
        WorldMap,
        warp::WarpDestination,
        manager::can_move,
    },
    script::world::{WorldScript, Condition, WorldActionKind},
};

use firecore_input::{pressed, Control};

use firecore_data::{get, get_mut,
    player::{
        PlayerSave,
        PlayerSaves,
    }
};

use macroquad::{
    prelude::{
        KeyCode, info, warn, is_key_pressed
    },
    rand::Random,
};

use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crate::battle::data::{BattleData, wild_battle};
use crate::util::{play_music_named, play_music};

use super::gui::text_window::TextWindow;
use super::{GameWorld, TileTextures, NpcTextures, RenderCoords};

pub mod manager;
pub mod set;
pub mod chunk;

pub static NPC_RANDOM: Random = Random::new();
pub static WILD_ENCOUNTERS: AtomicBool = AtomicBool::new(true);

const NPC_MOVE_CHANCE: f32 = 1.0 / 12.0;
// const NPC_MOVE_TICK: f32 = 0.5;

// impl GameWorldMap for WorldMap {
//     // fn despawn_npc(&mut self, index: u8) {
//     //     if let Some(npc) = self.npcs.remove(&index) {
//     //         self.despawned_npcs.insert(index, npc);
//     //     }
//     // }
// }

impl GameWorld for WorldMap {

    fn on_start(&mut self, music: bool) {

        self.npc_manager.timer.spawn();
        // self.npc_timer.set_target(NPC_MOVE_TICK);

        if let Some(saves) = get::<PlayerSaves>() {
            if let Some(data) = saves.get().world_status.map_data.get(&self.name) {
                for (index, state) in data.npcs.iter() {
                    if let Some(npc) = self.npc_manager.npcs.get_mut(index) {
                        npc.alive = *state;
                    }
                }
            }
        }

        if music {
            if firecore_audio::get_current_music().map(|current| current != self.music).unwrap_or(true) {
                play_music(self.music);
            }
        }

    }

    fn on_tile(&mut self, battle_data: &mut Option<BattleData>, player: &mut PlayerCharacter) {
        if let Some(tile_id) = self.tile(player.position.local.coords) {

            if WILD_ENCOUNTERS.load(Relaxed) {
                if let Some(wild) = &self.wild {
                    if let Some(tiles) = wild.tiles.as_ref() {
                        for tile in tiles.iter() {
                            if tile_id.eq(tile) {
                                wild_battle(battle_data, wild);
                                break;
                            }
                        }
                    } else {
                        wild_battle(battle_data, wild);
                    }            
                }
            }            
    
            // look for player
            if let Some(saves) = get::<PlayerSaves>() {
                let save = saves.get();
                for (index, npc) in self.npc_manager.npcs.iter_mut().filter(|(_, npc)| npc.is_alive() && npc.trainer.is_some()) {
                    find_battle(save, &self.name, index, npc, &mut self.npc_manager.active, player);
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

    fn update(&mut self, delta: f32, player: &mut PlayerCharacter, battle_data: &mut Option<BattleData>, warp: &mut Option<(WarpDestination, bool)>, text_window: &mut TextWindow) {

        // Move NPCs
        self.npc_manager.do_move(delta);

        if self.npc_manager.timer.is_finished() {
            self.npc_manager.timer.soft_reset();
            if let Some(saves) = get::<PlayerSaves>() {
                let save = saves.get();
                for (index, npc) in self.npc_manager.npcs.iter_mut() {
                    if npc.is_alive() && !npc.should_move_to_destination() {
                        if NPC_RANDOM.gen_float() < NPC_MOVE_CHANCE {
                            match npc.properties.movement {
                                MovementType::Still => (),
                                MovementType::LookAround => {
                                    npc.position.direction = firecore_util::Direction::DIRECTIONS[NPC_RANDOM.gen_range(0..4) as usize];
                                    find_battle(save, &self.name, index, npc, &mut self.npc_manager.active, player);
                                },
                                MovementType::WalkUpAndDown(steps) => {
                                    let origin = npc.properties.origin.get_or_insert(npc.position.coords);
                                    let direction = 
                                        if npc.position.coords.y <= origin.y - steps {
                                            Direction::Down
                                        } else if npc.position.coords.y >= origin.y + steps {
                                            Direction::Up
                                        } else 
                                    if NPC_RANDOM.gen_range(0..2) == 0 {//rand::thread_rng().gen_bool(0.5) {
                                        Direction::Down
                                    } else {
                                        Direction::Up
                                    };
                                    let coords = npc.position.coords.in_direction(direction);
                                    if can_move(firecore_world::map::tile_walkable(coords, &self.movements, self.width)) {
                                        npc.position.direction = direction;
                                        if !find_battle(save, &self.name, index, npc, &mut self.npc_manager.active, player) {
                                            if coords.y != player.position.local.coords.y {
                                                npc.go_to(coords);
                                            }                                        
                                        }                                    
                                    }
                                },
                            }
                        }  
                    }            
                }
            }
        } else {
            self.npc_manager.timer.update(delta);
        }

        

        // Update scripts

        for script in self.scripts.iter_mut().filter(|script| script.is_alive()) {
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
                                player.move_to(*destination);
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




                        WorldActionKind::NPCAdd(npc) => {
                            if self.npc_manager.npcs.insert(npc.identifier.index, npc.clone()).is_some() {
                                warn!("Replaced NPC with id {}!", npc.identifier.index);
                            }
                            pop = true;
                        }

                        WorldActionKind::NPCRemove(id) => {
                            if self.npc_manager.npcs.remove(id).is_none() {
                                warn!("Could not remove NPC with id {}!", id);
                            }
                            pop = true;
                        }

                        WorldActionKind::NPCSpawn(id) => {
                            if let Some(npc) = self.npc_manager.get_mut(id) {
                                npc.spawn();
                            } else {
                                warn!("Could not spawn NPC with id {}!", id);
                            }
                            pop = true;
                        }
                        WorldActionKind::NPCDespawn(id) => {
                            if let Some(npc) = self.npc_manager.get_mut(id) {
                                npc.despawn();
                            } else {
                                warn!("Could not despawn NPC with id {}!", id);
                            }
                            pop = true;
                        }


                        WorldActionKind::NPCLook(id, direction) => {
                            if let Some(npc) = self.npc_manager.get_mut(id) {
                                npc.position.direction = *direction;                                   
                            }
                            pop = true;
                        }
                        WorldActionKind::NPCMove( id, pos ) => {
                            if let Some(npc) = self.npc_manager.get_mut(id) {
                                if npc.properties.character.destination.is_some() {
                                    if npc.should_move_to_destination() {
                                        npc.move_to_destination(delta);
                                    } else {
                                        npc.properties.character.destination = None;
                                        pop = true;
                                    }
                                } else {
                                    npc.go_to(pos.coords);
                                }
                            } else {
                                warn!("NPC script tried to move an unknown NPC (with id {})", id);
                                pop = true;
                            }
                        },
                        WorldActionKind::NPCLeadPlayer( id, pos ) => {
                            if let Some(npc) = self.npc_manager.get_mut(id) {
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
                                        npc.go_to(pos.coords);
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
                                        player.move_to(Destination::next_to(&player.position.local, pos.coords));
                                    }
                                }
                            } else {
                                warn!("NPC script tried to lead player with an unknown NPC (with id {})", id);
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCMoveToPlayer(id) => {
                            if let Some(npc) = self.npc_manager.get_mut(id) {
                                if npc.properties.character.destination.is_some() {
                                    if npc.should_move_to_destination() {
                                        npc.move_to_destination(delta);
                                    } else {
                                        npc.properties.character.destination = None;
                                        pop = true;
                                    }
                                } else {
                                    npc.go_next_to(player.position.local.coords)
                                }
                            } else {
                                warn!("NPC script tried to move to player with an unknown NPC (with id {})", id);
                                pop = true;
                            }
                        }
                        WorldActionKind::NPCInteract(id) => {
                            if let Some(npc) = self.npc_manager.get_mut(id) {
                                if npc.interact_from(&player.position.local) {
                                    self.npc_manager.active = Some(*id);
                                }
                            }
                            pop = true;
                        }
                        WorldActionKind::NPCBattle(id) => {
                            if let Some(npc) = self.npc_manager.get(id) {
                                if npc.trainer.is_some() {
                                    crate::battle::data::trainer_battle(battle_data, &self.name, &npc);
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

                        WorldActionKind::Warp(destination, change_music) => {
                            *warp = Some((destination.clone(), *change_music));
                            despawn_script(script);
                        },
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

        // Npc window manager code

        if let Some(npc) = 
        
        if let Some(active) = self.npc_manager.active {
            if let Some(npc) = self.npc_manager.npcs.get_mut(&active) {
                Some(npc)
            } else {
                None
            }
        } else {
            None
        }
        
        {
            if text_window.is_alive() {
                if text_window.is_finished() {
                    {
                        self.npc_manager.active = None;
                        crate::battle::data::trainer_battle(battle_data, &self.name, npc);
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
    
                                if trainer.battle_on_interact {

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

                                    if let Some(npc_type) = super::npc::npc_type(&npc.identifier.npc_type) {
                                        if let Some(trainer) = npc_type.trainer.as_ref() {
                                            if let Some(encounter_music) = trainer.music.as_ref() {
                                                if let Err(err) = if let Some(playing_music) = firecore_audio::get_current_music() {
                                                    if playing_music != firecore_audio::get_music_id(encounter_music).unwrap() {
                                                        firecore_audio::play_music_named(encounter_music)
                                                    } else {
                                                        Ok(())
                                                    }
                                                } else {
                                                    firecore_audio::play_music_named(encounter_music)
                                                } {
                                                    warn!("Could not play music named {} with error {}", self.name, err);
                                                }
                                            }
                                        }
                                    }

                                }
                                
                            }   
                        }
                    }
    
                    if !message_ran {
                        text_window.despawn();
                        self.npc_manager.active = None;
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

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
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
        for npc in self.npc_manager.npcs.values() {
            if npc.is_alive() {
                super::npc::render(npc, npc_textures, &screen);
            }
        }
        for script in self.scripts.iter() {
            if script.is_alive() {
                if let Some(action) = script.actions.front() {
                    match action {
                        WorldActionKind::Conditional{ .. } => {
                            if let Some(texture) = super::gui::gui_texture(0) {
                                if script.option > 1 {
                                    crate::util::graphics::draw(texture, 162.0, 66.0);
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
                for (index, npc) in self.npc_manager.npcs.iter() {
                    info!("NPC {} (id: {}), {} is at {}, {}; looking {:?}", &npc.identifier.name, index, if npc.is_alive() {""} else {" (despawned)"}, &npc.position.coords.x, &npc.position.coords.y, &npc.position.direction);
                }
            }
            if is_key_pressed(KeyCode::F9) {
                let wild = !WILD_ENCOUNTERS.load(Relaxed);
                WILD_ENCOUNTERS.store(wild, Relaxed);
                info!("Wild Encounters: {}", wild);
            }
            if is_key_pressed(KeyCode::H) {
				if let Some(mut saves) = get_mut::<PlayerSaves>() {
					saves.get_mut().party.iter_mut().for_each(|pokemon| {
                        pokemon.current_hp = None;
                        pokemon.moves.as_mut().map(
                            | moves | 
                            moves.iter_mut().for_each(
                                | pmove | 
                                pmove.pp = None
                            )
                        );
                    });
				}
			}
        }

        if pressed(Control::A) {
            for (npc_index, npc) in self.npc_manager.npcs.iter_mut() {
                if npc.is_alive() {
                    if npc.interact_from(&player.position.local) {
                        self.npc_manager.active = Some(*npc_index);
                    }
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

fn find_battle(save: &PlayerSave, map_name: &String, index: &NPCId, npc: &mut NPC, active: &mut Option<NPCId>, player: &mut PlayerCharacter) -> bool {
    if !save.has_battled(map_name, index) {
        if npc.find_character(player.position.local.coords, player) {
            *active = Some(*index);
            true
        } else {
            false
        }                                          
    } else {
        false
    }
}