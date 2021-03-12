use firecore_audio::play_music;
use firecore_util::text::MessageSet;
use firecore_util::text::TextColor;
use firecore_world::World;
use firecore_world::map::WorldMap;
use firecore_world::script::Condition;
use firecore_world::script::WorldActionKind;
use macroquad::prelude::KeyCode;
use macroquad::prelude::debug;
use macroquad::prelude::info;
use macroquad::prelude::is_key_pressed;
use macroquad::prelude::warn;

use crate::util::Completable;
use firecore_util::Entity;
use firecore_util::Direction;
use firecore_input::{self as input, Control};
use super::WorldNpc;
use super::gui::map_window_manager::MapWindowManager;
use super::NpcTextures;
use super::TileTextures;
use firecore_world::pokemon::WildEntry;
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
        let tile_id = self.tile(player.position.local.coords.x, player.position.local.coords.y);

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
                if let Some(mut data) = macroquad::prelude::collections::storage::get_mut::<crate::io::data::player::PlayerData>() {
                    if !std::ops::DerefMut::deref_mut(&mut data).world_status.get_or_create_map_data(&self.name).battled.contains(&npc.identifier.name) {
                        if let Some(tracker) = trainer.tracking_length {
                            let tracker = tracker as isize;
                            match npc.position.direction {
                                Direction::Up => {
                                    if player.position.local.coords.x == npc.position.coords.x {
                                        if player.position.local.coords.y < npc.position.coords.y && player.position.local.coords.y >= npc.position.coords.y - tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                            
                                            //npc.movement.walk_to_player();
                                        }
                                    }
                                }
                                Direction::Down => {
                                    if player.position.local.coords.x == npc.position.coords.x {
                                        if player.position.local.coords.y > npc.position.coords.y && player.position.local.coords.y <= npc.position.coords.y + tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                        }
                                    }
                                }
                                Direction::Left => {
                                    if player.position.local.coords.y == npc.position.coords.y {
                                        if player.position.local.coords.x < npc.position.coords.x && player.position.local.coords.x >= npc.position.coords.x - tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                        }
                                    }
                                }
                                Direction::Right => {
                                    if player.position.local.coords.y == npc.position.coords.y {
                                        if player.position.local.coords.x > npc.position.coords.x && player.position.local.coords.x <= npc.position.coords.x + tracker {
                                            // info!("NPC {} Found player at tile {}, {}", npc.identifier.name, player.position.local.coords.x, player.position.local.coords.y);
                                            self.npc_active = Some(npc_index);
                                            npc.interact(None, player);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }            
        }
        if let Some(mut player_data) = macroquad::prelude::collections::storage::get_mut::<crate::io::data::player::PlayerData>() {
            for script in self.scripts.iter_mut() {
                if script.condition.location.in_bounds(&player.position.local.coords) {
                    if let Some(condition) = script.condition.event.as_ref() {
                        match condition {
                            Condition::WorldEvent { id, happened, activate} => {
                                if player_data.world_status.completed_events.contains(id).ne(happened) {
                                    // debug!("Script cannot run because event has happened");
                                    return;
                                } else if *activate {
                                    player_data.world_status.completed_events.insert(id.clone());
                                }
                            }
                        }
                    }
                    debug!("Attempting to spawn script!");
                    script.spawn();
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
                            // crate::experimental::script::ActionKind::MovePlayer { pos } => {
                            //     pop = true;
                            // }
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
                            }
                            WorldActionKind::FreezePlayer => {
                                player.freeze();
                                pop = true;
                                info!("Script froze player!");
                            },
                            WorldActionKind::UnfreezePlayer => {
                                player.frozen = false;
                                pop = true;
                                info!("Script unfroze player!");
                            }
                            WorldActionKind::NPCSpawn { id, npc } => {
                                self.script_npcs.insert(*id, npc.clone());
                                pop = true;
                                info!("Script spawned NPC {} with id {}", &npc.identifier.name, id);
                            }
                            WorldActionKind::NPCLook {id, direction} => {
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    npc.position.direction = *direction;
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCMove { id, pos } => {
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    if npc.offset.is_some() {
                                        if npc.should_move() {
                                            npc.do_move(delta);
                                        } else {
                                            pop = true;
                                        }
                                    } else {
                                        npc.walk_to(pos);
                                    }
                                } else {
                                    warn!("NPC script tried to move an unknown NPC (with id {})", id);
                                    pop = true;
                                }
                            }
                            WorldActionKind::NPCMoveToPlayer ( id ) => {
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    if npc.offset.is_some() {
                                        if npc.should_move() {
                                            npc.do_move(delta);
                                        } else {
                                            pop = true;
                                        }
                                    } else {
                                        npc.walk_next_to(&player.position.local.coords);
                                    }
                                }
                            }
                            WorldActionKind::NPCDespawn (id) => {
                                if self.script_npcs.remove(id).is_none() {
                                    warn!("Script attempted to despawn npc that doesn't exist!");
                                }
                                info!("Script Despawned NPC with id {}", id);
                                pop = true;
                            }
                            WorldActionKind::NPCInteract(id) => {
                                if let Some(npc) = self.script_npcs.get_mut(id) {
                                    self.npc_active = Some(self.npcs.len() + (*id) as usize);
                                    npc.interact(None, player);
                                }
                                pop = true;
                            }
                            WorldActionKind::NPCBattle(id) => {
                                if let Some(npc) = self.script_npcs.get(id) {
                                    if let Some(trainer) = npc.trainer.as_ref() {
                                        crate::util::battle_data::trainer_battle(trainer, &npc.identifier.name, &npc.identifier.npc_type);
                                    }
                                }
                                pop = true;
                            }
                            WorldActionKind::DisplayText { message_set } => {
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

                            WorldActionKind::Battle(battle_data) => {
                                *crate::util::battle_data::BATTLE_DATA.lock() = Some(battle_data.clone());
                                pop = true;
                            }
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
                    window_manager.spawn();
                    npc.offset = None;

                    if let Some(trainer) = npc.trainer.as_ref() {
                        if let Some(mut data) = macroquad::prelude::collections::storage::get_mut::<crate::io::data::player::PlayerData>() {
                            if !data.world_status.get_or_create_map_data(&self.name).battled.contains(&npc.identifier.name) {

                                // Spawn text window

                                let message_set = MessageSet::new(
                                    1, 
                                    TextColor::Blue, 
                                    trainer.encounter_message.clone()
                                );
                                window_manager.set_text(message_set);

                                // Play Trainer music

                                if let Some(music) = trainer.encounter_music {
                                    if let Some(playing_music) = firecore_audio::get_music_playing() {
                                        if playing_music != music {
                                            play_music(music);
                                        }
                                    } else {
                                        play_music(music);
                                    }
                                }
                            }   
                        }
                    } else {
                        window_manager.set_text(npc.message_set.clone());
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
            if is_key_pressed(KeyCode::F4) {
                for npc in &self.npcs {
                    info!("NPC {} is at {}, {}; looking {:?}", &npc.identifier.name, &npc.position.coords.x, &npc.position.coords.y, &npc.position.direction);
                }
            }
            if is_key_pressed(KeyCode::F7) {
                info!("There are {} scripts in this map.", self.scripts.len());
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
                                npc.interact(Some(player.position.local.direction), player);
                            }
                        },
                        Direction::Down => {
                            if player.position.local.coords.y + 1 == npc.position.coords.y {
                                self.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player);
                            }
                        },
                        _ => (),
                    }
                } else if player.position.local.coords.y == npc.position.coords.y {
                    match player.position.local.direction {
                        Direction::Right => {
                            if player.position.local.coords.x + 1 == npc.position.coords.x {
                                self.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player);
                            }
                        },
                        Direction::Left => {
                            if player.position.local.coords.x - 1 == npc.position.coords.x {
                                self.npc_active = Some(npc_index);
                                npc.interact(Some(player.position.local.direction), player);
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