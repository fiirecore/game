use dashmap::DashMap;
use firecore_util::Direction;
use firecore_world::character::npc::NPC;
use firecore_world::character::npc::NPCType;

use crate::io::data::player::PLAYER_DATA;

use super::NpcTextures;
use super::RenderCoords;

lazy_static::lazy_static! {
    pub static ref NPC_TYPES: DashMap<String, NPCType> = DashMap::new();
}

pub trait WorldNpc {

    // fn interact(&mut self, direction: Option<Direction>, map_name: &String, player: &mut Player);

    // fn after_interact(&mut self, map_name: &String);

    fn render(&self, npc_textures: &NpcTextures, screen: &RenderCoords);

    fn current_texture_pos(&self) -> f32;

}

pub fn has_battled(map_name: &String, npc: &NPC) -> bool {
    npc.trainer.is_some() && !PLAYER_DATA.read().as_ref().map(|data| data.has_battled(map_name, &npc.identifier.name)).unwrap_or(true)
}

pub fn try_battle(map_name: &String, npc: &NPC) {
    if let Some(player_data) = PLAYER_DATA.write().as_mut() {
        player_data.world_status.get_or_create_map_data(map_name).battle(npc);
    } else {
        macroquad::prelude::warn!("Could not get player data!");
    }
}

impl WorldNpc for NPC {

    fn render(&self, npc_textures: &NpcTextures, screen: &RenderCoords) {
        let x = ((self.position.coords.x + screen.x_tile_offset) << 4) as f32 - screen.focus.x + self.position.offset.x;
        let y = ((self.position.coords.y - 1 + screen.y_tile_offset) << 4) as f32 - screen.focus.y + self.position.offset.y;
        
        if let Some(texture) = npc_textures.get(&self.identifier.npc_type) {
            macroquad::prelude::draw_texture_ex(*texture, x, y, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
                source: Some(macroquad::prelude::Rect::new(
                    self.current_texture_pos(),
                    0.0,
                    16.0,
                    32.0,
                )),
                flip_x: self.position.direction == Direction::Right,
                ..Default::default()
            })
        } else {
            crate::util::graphics::draw_rect([1.0, 0.0, 0.0, 1.0], x, y + crate::util::TILE_SIZE as f32, 16, 16);
        }
    }

    fn current_texture_pos(&self) -> f32 {
        if let Some(npc_type) = NPC_TYPES.get(&self.identifier.npc_type) {
            let index = (
                if self.position.offset.x != 0.0 {
                    self.position.offset.x
                } else {
                    self.position.offset.y
                }.abs() as usize >> 3
            ) + self.properties.character.sprite_index as usize;

            let sprite_indexes = firecore_world::character::sprite::get_indexes(npc_type.sprite_indexes);
            
            (match self.position.direction {
                Direction::Down => sprite_indexes.down[index],
                Direction::Up => sprite_indexes.up[index],
                _ => sprite_indexes.side[index]
            } << 4) as f32
        } else {
            0.0
        }
        
    }

}