use firecore_audio::play_music_named;
use macroquad::prelude::KeyCode;
use macroquad::prelude::debug;
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::warn;

use firecore_audio::play_music_id;
use firecore_util::text::MessageSet;
use firecore_util::text::TextColor;
use firecore_world::World;
use firecore_world::character::movement::Destination;
use firecore_world::map::WorldMap;
use firecore_world::script::world::Condition;
use firecore_world::script::world::WorldActionKind;
use firecore_world::wild::WildEntry;

use firecore_util::Entity;
use firecore_util::Direction;
use firecore_input::{self as input, Control};

use crate::io::data::player::PLAYER_DATA;
use crate::util::Completable;

use super::npc::WorldNpc;
use super::gui::map_window_manager::MapWindowManager;
use super::NpcTextures;
use super::TileTextures;
use super::npc::on_sight;
use super::player::Player;
use super::RenderCoords;
use super::GameWorld;

pub mod manager;
pub mod set;
pub mod chunk;

pub trait GameWorldMap {

    fn tile_row(&self, x: isize, offset: isize) -> u16;

    fn after_interact(&mut self, index: usize);

}

impl GameWorldMap for WorldMap {

    fn tile_row(&self, x: isize, offset: isize) -> u16 {
        self.tile_map[x as usize + offset as usize]
    }

    fn after_interact(&mut self, index: usize) {
        self.npcs[index].after_interact(&self.name);
    }

}

impl GameWorld for WorldMap {

    fn on_tile(&mut self, player: &mut Player) {
        let tile_id = self.tile(&player.position.local.coords);

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

        for npc in self.npcs.iter_mut().enumerate() {
            let (npc_index, npc) = npc;
            if let Some(trainer) = &npc.trainer {
                if let Some(data) = PLAYER_DATA.write().as_mut() {
                    if !data.world_status.get_or_create_map_data(&self.name).battled.contains(&npc.identifier.name) {
                        if let Some(tracker) = trainer.tracking_length {
                            let tracker = tracker as isize;
                            match npc.position.direction {
                                Direction::Up => {
                                    if player.position.local.coords.x == npc.position.coords.x {
                                        if player.position.local.coords.y < npc.position.coords.y && player.position.local.coords.y >= npc.position.coords.y - tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            on_sight(npc, None, player);
                                            
                                            //npc.movement.walk_to_player();
                                        }
                                    }
                                }
                                Direction::Down => {
                                    if player.position.local.coords.x == npc.position.coords.x {
                                        if player.position.local.coords.y > npc.position.coords.y && player.position.local.coords.y <= npc.position.coords.y + tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            on_sight(npc, None, player);
                                        }
                                    }
                                }
                                Direction::Left => {
                                    if player.position.local.coords.y == npc.position.coords.y {
                                        if player.position.local.coords.x < npc.position.coords.x && player.position.local.coords.x >= npc.position.coords.x - tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            on_sight(npc, None, player);
                                        }
                                    }
                                }
                                Direction::Right => {
                                    if player.position.local.coords.y == npc.position.coords.y {
                                        if player.position.local.coords.x > npc.position.coords.x && player.position.local.coords.x <= npc.position.coords.x + tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            on_sight(npc, None, player);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }            
        }
        if let Some(player_data) = PLAYER_DATA.write().as_mut() {
            for script in self.scripts.iter_mut() {

                if script.location.as_ref().map(|location| location.in_bounds(&player.position.local.coords)).unwrap_or(true) {
                    let mut run = script.conditions.len() == 0;
                    for condition in &script.conditions {
                        match condition {
                            Condition::WorldEvent { id, happened, activate} => {
                                if player_data.world_status.completed_events.contains(id).eq(happened) {
                                    run = true;
                                    if true.eq(activate) {
                                        player_data.world_status.completed_events.insert(id.clone());
                                    }
                                }                                
                            },
                            Condition::PlayerPokemonAny(pokemon_ids) => {
                                for pokemon in &player_data.party.pokemon {
                                    if run {
                                        break;
                                    }
                                    for id in pokemon_ids {
                                        if pokemon.id.eq(id) {
                                            run = true;
                                            break;
                                        }
                                    }
                                }
                            },
                            Condition::PlayerPokemonAll(pokemon_ids) => {
                                // let mut has 
                                // for id in pokemon_ids {
                                //     let mut has = true;
                                //     for pokemon in &player_data.party.pokemon {
                                //         if pokemon.id.eq(id) {
                                //             continue;
                                //         }
                                //     }
                                // }
                            }
                        }
                    }
                    if run {
                        debug!("Attempting to spawn script \"{}\"!", script.identifier);
                        script.spawn();
                    }                    
                }
            }
        }

    }

    fn update(&mut self, delta: f32, player: &mut Player, window_manager: &mut MapWindowManager) {
        
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
                                pop = true;
                            },
                            WorldActionKind::PlayMapMusic => {
                                play_music_id(self.music);
                                pop = true;
                            },
                            WorldActionKind::PlaySound(sound) => {
                                // firecore_audio::play_sound(*sound);
                                pop = true;
                            }
                            WorldActionKind::PlayerFreeze => {
                                player.freeze();
                                pop = true;
                                info!("Script froze player!");
                            },
                            WorldActionKind::PlayerUnfreeze => {
                                player.frozen = false;
                                pop = true;
                                info!("Script unfroze player!");
                            }
                            WorldActionKind::PlayerLook(direction) => {
                                player.position.local.direction = *direction;
                                pop = true;
                            }
                            WorldActionKind::PlayerMove(pos) => {
                                if player.properties.destination.is_some() {
                                    if player.should_move_to() {
                                        player.move_to_destination(delta);
                                    } else {
                                        player.properties.destination = None;
                                        pop = true;
                                    }
                                } else {
                                    player.properties.destination = Some(Destination::to(&player.position.local, pos));
                                }
                            }
                            WorldActionKind::PlayerGivePokemon(instance) => {
                                if let Some(player_data) = PLAYER_DATA.write().as_mut() {
                                    if player_data.party.pokemon.len() < 6 {
                                        player_data.party.pokemon.push(instance.clone());
                                    } else {
                                        warn!("Could not add pokemon #{} to player party because it is full", instance.id);
                                    }
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCSpawn { id, npc } => {
                                self.script_npcs.insert(*id, npc.clone());
                                pop = true;
                                info!("Script spawned NPC {} with id {}", &npc.identifier.name, id);
                            }
                            WorldActionKind::NPCLook(id, direction) => {
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    npc.position.direction = *direction;
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCMove( id, pos ) => {
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    if npc.properties.destination.is_some() {
                                        if npc.should_move() {
                                            npc.do_move(delta);
                                        } else {
                                            npc.properties.destination = None;
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
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    if npc.properties.destination.is_some() {
                                        if npc.should_move() {
                                            npc.do_move(delta);
                                        } else {
                                            npc.properties.destination = None;
                                            if player.properties.destination.is_none() {
                                                pop = true;
                                            }
                                        }
                                    } else {
                                        if npc.position.coords.ne(&pos.coords) {
                                            npc.walk_to(&pos.coords);
                                        }
                                    }
                                    if player.properties.destination.is_some() {
                                        if player.should_move_to() {
                                            player.move_to_destination(delta);
                                        } else {
                                            player.properties.destination = None;
                                            if npc.properties.destination.is_none() {
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
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    if npc.properties.destination.is_some() {
                                        if npc.should_move() {
                                            npc.do_move(delta);
                                        } else {
                                            npc.properties.destination = None;
                                            pop = true;
                                        }
                                    } else {
                                        npc.walk_next_to(&player.position.local.coords);
                                    }
                                } else {
                                    warn!("NPC script tried to move to player with an unknown NPC (with id {})", id);
                                    pop = true;
                                }
                            }
                            WorldActionKind::NPCDespawn(id) => {
                                if self.script_npcs.remove(id).is_none() {
                                    warn!("Script attempted to despawn npc that doesn't exist!");
                                }
                                info!("Script Despawned NPC with id {}", id);
                                pop = true;
                            }
                            WorldActionKind::NPCInteract(id) => {
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    self.npc_active = Some(self.npcs.len() + (*id) as usize);
                                    on_sight(npc, None, player);
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCBattle(id) => {
                                if let Some(npc) = self.script_npcs.get(id) {
                                    if let Some(trainer) = npc.trainer.as_ref() {
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

                            // WorldActionKind::Battle(battle_data) => {
                            //     *crate::util::battle_data::BATTLE_DATA.lock() = Some(battle_data.clone());
                            //     pop = true;
                            // }

                            WorldActionKind::Warp(_) => {},

                        }
                    }
                    None => {
                        script.despawn();
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
                    if let Some(npc) = self.npcs.get_mut(index) {
                        Some(npc)
                    } else {
                        self.script_npcs.get_mut(&((index - self.npcs.len()) as u8))
                    }
                } else {
                    None
                } {
                    npc.after_interact(&self.name);
                }
                window_manager.despawn();
            } else {
                window_manager.update(delta);
            }
        } else {
            if let Some(npc) = if let Some(index) = self.npc_active {
                if let Some(npc) = self.npcs.get_mut(index) {
                    Some(npc)
                } else {
                    self.script_npcs.get_mut(&((index - self.npcs.len()) as u8))
                }
            } else {
                None
            } {
                if npc.should_move() {
                    npc.do_move(delta) 
                } else {
                    warn!("npc has message: {}", npc.message.is_some());
                    window_manager.spawn();
                    npc.properties.destination = None;

                    let hash = window_manager.text_hash();

                    if let Some(message_set) = npc.message.as_ref() {
                        window_manager.set_text(message_set.clone());
                    }
                    
                    if let Some(data) = PLAYER_DATA.write().as_mut() {
                        if !data.world_status.get_or_create_map_data(&self.name).battled.contains(&npc.identifier.name) {
                            if let Some(trainer) = npc.trainer.as_ref() {

                                // Spawn text window

                                let message_set = MessageSet::new(
                                    1, 
                                    TextColor::Blue, 
                                    trainer.encounter_message.clone()
                                );
                                window_manager.set_text(message_set);

                                // Play Trainer music

                                if let Some(npc_type) = super::npc::NPC_TYPES.get(&npc.identifier.npc_type) {
                                    if let Some(trainer) = npc_type.trainer.as_ref() {
                                        if let Some(playing_music) = firecore_audio::get_current_music() {
                                            if playing_music != firecore_audio::get_music_id(&trainer.encounter_music).unwrap() {
                                                play_music_named(&trainer.encounter_music);
                                            }
                                        } else {
                                            play_music_named(&trainer.encounter_music);
                                        }
                                    }
                                }
                            }   
                        }
                    }

                    if hash == window_manager.text_hash() {
                        window_manager.despawn();
                    }

                    player.position.local.direction = npc.position.direction.inverse();
                    if player.frozen {
                        // self.player.move_update(0.0);
                        player.frozen = false;
                    }

                }
            } 
        }
        // self.script_manager.update(delta, player);
    }

    fn render(&self, tile_textures: &TileTextures, npc_textures: &NpcTextures, screen: RenderCoords, border: bool) {
        for yy in screen.top..screen.bottom {
            let y = yy - screen.y_tile_offset;
            let render_y = (yy << 4) as f32 - screen.focus.y; // old = y_tile w/ offset - player x pixel
            
            let row_offset = y * self.width as isize;
            
            for xx in screen.left..screen.right {
                let x = xx - screen.x_tile_offset;
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
        for npc in &self.npcs {
            npc.render(npc_textures, &screen);
        }
        for npc in self.script_npcs.values() {
            npc.render(npc_textures, &screen);
        }
        // self.script_manager.render(tile_textures, npc_textures, &screen);
    }

    fn input(&mut self, _delta: f32, player: &mut Player) {

        if cfg!(debug_assertions) {
            if is_key_pressed(KeyCode::F7) {
                info!("There are {} scripts in this map.", self.scripts.len());
            }
            if is_key_pressed(KeyCode::F8) {
                for npc in &self.npcs {
                    info!("NPC {} is at {}, {}; looking {:?}", &npc.identifier.name, &npc.position.coords.x, &npc.position.coords.y, &npc.position.direction);
                }
            }
        }

        if input::pressed(Control::A) {
            let len = self.npcs.len();
            for npc_index in 0..(len + self.script_npcs.len()) {
                // info!("npc index: {}, len: {}", npc_index, len);
                let npc = if npc_index < len {
                    self.npcs.get_mut(npc_index)
                } else {
                    self.script_npcs.values_mut().nth(npc_index - len)
                }.unwrap();
                if player.position.local.coords.x == npc.position.coords.x {
                    match player.position.local.direction {
                        Direction::Up => {
                            if player.position.local.coords.y - 1 == npc.position.coords.y {
                                self.npc_active = Some(npc_index);
                                on_sight(npc, Some(player.position.local.direction), player);
                            }
                        },
                        Direction::Down => {
                            if player.position.local.coords.y + 1 == npc.position.coords.y {
                                self.npc_active = Some(npc_index);
                                on_sight(npc, Some(player.position.local.direction), player);
                            }
                        },
                        _ => (),
                    }
                } else if player.position.local.coords.y == npc.position.coords.y {
                    match player.position.local.direction {
                        Direction::Right => {
                            if player.position.local.coords.x + 1 == npc.position.coords.x {
                                self.npc_active = Some(npc_index);
                                on_sight(npc, Some(player.position.local.direction), player);
                            }
                        },
                        Direction::Left => {
                            if player.position.local.coords.x - 1 == npc.position.coords.x {
                                self.npc_active = Some(npc_index);
                                on_sight(npc, Some(player.position.local.direction), player);
                            }
                        },
                        _ => (),
                    }
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