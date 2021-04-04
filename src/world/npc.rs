use ahash::AHashMap as HashMap;
use firecore_util::Direction;
use firecore_util::TinyStr16;
use firecore_world::character::npc::NPC;
use firecore_world::character::npc::npc_type::NPCType;

use super::NpcTextures;
use super::RenderCoords;

pub type NPCTypes = HashMap<TinyStr16, NPCType>;

pub static mut NPC_TYPES: Option<NPCTypes> = None;

pub fn npc_type(id: &TinyStr16) -> Option<&NPCType> {
    unsafe{NPC_TYPES.as_ref()}.expect("Could not get NPC types!").get(id)
}

pub fn render(npc: &NPC, npc_textures: &NpcTextures, screen: &RenderCoords) {
    let x = ((npc.position.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x + npc.position.offset.x;
    let y = ((npc.position.coords.y - 1 + screen.offset.y) << 4) as f32 - screen.focus.y + npc.position.offset.y;
    
    if let Some(texture) = npc_textures.get(&npc.identifier.npc_type) {
        macroquad::prelude::draw_texture_ex(*texture, x, y, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
            source: Some(macroquad::prelude::Rect::new(
                current_texture_pos(npc),
                0.0,
                16.0,
                32.0,
            )),
            flip_x: npc.position.direction == Direction::Right,
            ..Default::default()
        })
    } else {
        crate::util::graphics::draw_rect([1.0, 0.0, 0.0, 1.0], x, y + crate::util::TILE_SIZE as f32, 16.0, 16.0);
    }
}

pub fn current_texture_pos(npc: &NPC) -> f32 {
    if let Some(npc_type) = npc_type(&npc.identifier.npc_type) {
        let index = (
            if npc.position.offset.x != 0.0 {
                npc.position.offset.x
            } else {
                npc.position.offset.y
            }.abs() as usize >> 3
        ) + npc.properties.character.sprite_index as usize;
        
        (match npc.position.direction {
            Direction::Down => npc_type.sprite.down[index],
            Direction::Up => npc_type.sprite.up[index],
            _ => npc_type.sprite.side[index]
        } << 4) as f32
    } else {
        0.0
    }
    
}