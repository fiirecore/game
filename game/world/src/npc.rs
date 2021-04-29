use game::deps::{
    tinystr::TinyStr16,
    hash::HashMap,
};

use game::util::{
    TILE_SIZE, 
    Direction,
};

use firecore_world_lib::character::npc::{NPC, npc_type::NPCType};

use game::macroquad::prelude::{
    draw_texture_ex,
    WHITE,
    DrawTextureParams,
    Rect,
    draw_rectangle,
    RED,
};

use super::{NpcTextures, RenderCoords};


pub type NPCTypes = HashMap<TinyStr16, NPCType>;

pub static mut NPC_TYPES: Option<NPCTypes> = None;

pub fn npc_type(id: &TinyStr16) -> Option<&NPCType> {
    unsafe{NPC_TYPES.as_ref()}.expect("Could not get NPC types!").get(id)
}

pub fn render(npc: &NPC, npc_textures: &NpcTextures, screen: &RenderCoords) {
    let x = ((npc.character.position.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x + npc.character.position.offset.x;
    let y = ((npc.character.position.coords.y - 1 + screen.offset.y) << 4) as f32 - screen.focus.y + npc.character.position.offset.y;
    
    if let Some(texture) = npc_textures.get(&npc.npc_type) {
        draw_texture_ex(*texture, x, y, WHITE, DrawTextureParams {
            source: Some(Rect::new(
                current_texture_pos(npc),
                0.0,
                16.0,
                32.0,
            )),
            flip_x: npc.character.position.direction == Direction::Right,
            ..Default::default()
        })
    } else {
        draw_rectangle(x, y + TILE_SIZE, TILE_SIZE, TILE_SIZE * 2.0, RED);
    }
}

pub fn current_texture_pos(npc: &NPC) -> f32 {
    if let Some(npc_type) = npc_type(&npc.npc_type) {
        let index = (
            if npc.character.position.offset.x != 0.0 {
                npc.character.position.offset.x
            } else {
                npc.character.position.offset.y
            }.abs() as usize >> 3
        ) + npc.character.sprite_index as usize;
        
        (match npc.character.position.direction {
            Direction::Down => npc_type.sprite.down[index],
            Direction::Up => npc_type.sprite.up[index],
            _ => npc_type.sprite.side[index]
        } << 4) as f32
    } else {
        0.0
    }
    
}