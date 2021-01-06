use crate::engine::game_context::GameContext;
use crate::entity::entities::player::RUN_SPEED;
use crate::entity::entity::Entity;
use crate::entity::util::direction::Direction;
use crate::gui::gui::Activatable;

use crate::util::map_traits::MapManager;
use crate::util::map_util::GameMap;
use super::world_manager::WorldManager;

impl WorldManager {

    pub fn input(&mut self, context: &mut GameContext) {

        if context.fkeys[0] == 1 {
			context.battle_context.random_wild_battle(&mut context.random);
        }

        if context.keys[6] == 1 {
            self.player_gui.toggle();
        }
        if self.player_gui.in_focus() {
            
            self.player_gui.input(context);

        } else {
    
            if context.fkeys[1] == 1 {
                self.player.noclip = !self.player.noclip;
                if self.player.noclip {
                    self.player.speed *= 4;
                } else {
                    self.player.speed /= 4;
                }
                context.app_console.add_line(String::from("Noclip toggled!"));
            }
    
            if context.fkeys[3] == 1 {
                let mut pos = String::from("X: ");
                pos.push_str(self.player.coords.x.to_string().as_str());
                pos.push_str(", Y: ");
                pos.push_str(self.player.coords.y.to_string().as_str());
                context.app_console.add_line(pos);
                if self.world_map_manager.is_alive() {
                    let mut pos_map = String::from("Local X: ");
                    let map = self.world_map_manager.get_current_world().get_current_piece();
                    pos_map.push_str((self.player.coords.x - map.x).to_string().as_str());
                    pos_map.push_str(", Local Y: ");
                    pos_map.push_str((self.player.coords.y - map.y).to_string().as_str());
                    context.app_console.add_line(pos_map);
                }
                
            }
    
            if context.fkeys[4] == 1 {
                if let Some(tile_id) = self.world_map_manager.get_tile_id(self.player.coords.x, self.player.coords.y) {
                    context.app_console.add_line(tile_id.to_string());
                }
            }

            self.player.reset_speed();
            if context.keys[1] == 1 || context.keys[1] == 2 {
                self.player.running = true;
                if self.player.noclip {
                    self.player.speed = RUN_SPEED * 2;
                } else {
                    self.player.speed = RUN_SPEED;
                }
            }
    
            if !self.player.moving {
    
                let mut x_offset: i8 = 0;
                let mut y_offset: i8 = 0;
                let mut jump = false;
    
                if context.keys[2] == 1 || context.keys[2] == 2 {
                    // Up
                    //self.player.dir_changed = true;
                    self.player.direction = Direction::Up;
                    y_offset -= 1;
                    self.player.on_try_move(self.player.direction);
                }
                if context.keys[3] == 1 || context.keys[3] == 2 {
                    //self.player.dir_changed = true;
                    self.player.direction = Direction::Down;
                    y_offset += 1;
                    if let Some(tile_id) = self.world_map_manager.get_tile_id(self.player.coords.x, self.player.coords.y + y_offset as isize) {
                        if tile_id == 135 || tile_id == 176 || tile_id == 177 || tile_id == 143 || tile_id == 184 || tile_id == 185 || tile_id == 217 || tile_id == 1234 {
                            jump = true;
                        }
                    }
                    self.player.on_try_move(self.player.direction);
                }
                if context.keys[4] == 1 || context.keys[4] == 2 {
                    // Left
                    //self.player.dir_changed = true;
                    self.player.direction = Direction::Left;
                    x_offset -= 1;
                    if let Some(tile_id) = self.world_map_manager.get_tile_id(self.player.coords.x + x_offset as isize, self.player.coords.y) {
                        if tile_id == 133 {
                            jump = true;
                        }
                    }
                    self.player.on_try_move(self.player.direction);
                }
                if context.keys[5] == 1 || context.keys[5] == 2 {
                    // Right
                    //self.player.dir_changed = true;
                    self.player.direction = Direction::Right;
                    x_offset += 1;
                    if let Some(tile_id) = self.world_map_manager.get_tile_id(self.player.coords.x + x_offset as isize, self.player.coords.y) {
                        if tile_id == 134 {
                            jump = true;
                        }
                    }
                    self.player.on_try_move(self.player.direction);
                }
    
                if x_offset != 0 && y_offset != 0 {
                    y_offset = 0;
                }
    
                if self.world_map_manager.is_alive() {
                    self.world_map_manager.input(context, &self.player);
                } else {
                    self.warp_map_manager.input(context, &self.player);
                }
                
                
    
                if x_offset != 0 || y_offset != 0 {
    
                    let code;
    
                    if self.world_map_manager.is_alive() {
                        
                        let cw = self.world_map_manager.get_current_world_mut();
    
                        code = cw.walkable(self.player.coords.x + x_offset as isize, self.player.coords.y + y_offset as isize);
                        if let Some(entry) = cw.check_warp(self.player.coords.x + x_offset as isize, self.player.coords.y + y_offset as isize) {
                            self.world_warp(context, entry);
                            return;
                        }
                    } else {
    
                        let cms = self.warp_map_manager.current_map_set();
                        
                        code = cms.walkable(self.player.coords.x + x_offset as isize, self.player.coords.y + y_offset as isize);
                        if let Some(entry) = cms.current_map().check_warp(self.player.coords.x + x_offset as isize, self.player.coords.y + y_offset as isize) {
                            self.warp_warp(context, entry);
                            return;
                        }
                    }
                    if code == 0x0C || self.player.noclip || code == 0x00 || code == 0x04 || jump {
                        self.player.moving();
                    }
    
                }
            }

        }
        
    }

    pub(crate) fn player_movement(&mut self, context: &mut GameContext) {
        if self.player.moving {
            if (self.player.direction) == Direction::Up {
                self.player.coords.y_offset -= self.player.speed as i8;
                if self.player.coords.y_offset <= -16 {
                    self.player.coords.y -= 1;
                    self.player.coords.y_offset = 0;
                    self.stop_player(context);
                }
            }

            if (self.player.direction) == Direction::Down {
                self.player.coords.y_offset += self.player.speed as i8;
                if self.player.coords.y_offset >= 16 {
                    self.player.coords.y += 1;
                    self.player.coords.y_offset = 0;
                    self.stop_player(context);
                }
            }
            if (self.player.direction) == Direction::Left {
                self.player.coords.x_offset -= self.player.speed as i8;
                if self.player.coords.x_offset <= -16 {
                    self.player.coords.x -= 1;
                    self.player.coords.x_offset = 0;
                    self.stop_player(context);
                }
            }

            if (self.player.direction) == Direction::Right {
                self.player.coords.x_offset += self.player.speed as i8;
                if self.player.coords.x_offset >= 16 {
                    self.player.coords.x += 1;
                    self.player.coords.x_offset = 0;
                    self.stop_player(context);
                }
            }
            self.player.move_update();
        }
    }

    fn stop_player(&mut self, context: &mut GameContext) {
        self.player.moving = false;
        self.player.on_stopped_moving();
        if self.world_map_manager.is_alive() {
            self.world_map_manager.on_tile(context, &self.player);
        } else {
            self.warp_map_manager.on_tile(context, &self.player);
        }
        
        //self.on_finished_moving(context);
        
    }

}