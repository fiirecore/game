use game::{
    deps::{
        hash::HashMap,
        tinystr::TinyStr16,
    },
    util::{
        TILE_SIZE, 
        Direction,
    },
    graphics::byte_texture,
    macroquad::prelude::{
        Texture2D,
        draw_texture_ex,
        WHITE,
        DrawTextureParams,
        Rect,
        draw_rectangle,
        RED,
    },
};

use world::{
    serialized::SerializedNPCType,
    character::npc::{
        NPC,
        npc_type::NPCTypeId,
    },
};

use crate::npc::npc_type;

pub type NpcTextures = HashMap<TinyStr16, Texture2D>;
pub type TrainerTextures = NpcTextures;

static mut TRAINER_TEXTURES: Option<TrainerTextures> = None;

#[derive(Default)]
pub struct NPCTextureManager {

    pub npcs: NpcTextures,
    // pub trainer: TrainerTextures,

}

impl NPCTextureManager {

    pub fn trainer_texture(npc_type: &NPCTypeId) -> Texture2D {
        unsafe { *TRAINER_TEXTURES.as_ref().expect("Could not get trainer textures! (Not initialized)").get(npc_type).unwrap_or_else(|| panic!("Could not get trainer texture for NPC Type {}", npc_type)) }
    }

    pub fn with_capacity(&mut self, capacity: usize) {
        self.npcs.reserve(capacity);
        unsafe { TRAINER_TEXTURES = Some(HashMap::with_capacity(capacity)); }
    }

    pub fn add_npc_type(&mut self, npc_type: &SerializedNPCType) {
        self.npcs.insert(npc_type.config.identifier, byte_texture(&npc_type.texture));
        if let Some(texture) = &npc_type.battle_texture {
            unsafe {
                TRAINER_TEXTURES.as_mut().unwrap().insert(npc_type.config.identifier, byte_texture(texture));
            }
        }
    }

    pub fn render(&self, npc: &NPC, screen: &crate::RenderCoords) {
        let x = ((npc.character.position.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x + npc.character.position.offset.x;
        let y = ((npc.character.position.coords.y - 1 + screen.offset.y) << 4) as f32 - screen.focus.y + npc.character.position.offset.y;
        
        if let Some(texture) = self.npcs.get(&npc.npc_type) {
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

}

pub fn current_texture_pos(npc: &NPC) -> f32 {
    let index = (
        if npc.character.position.offset.x != 0.0 {
            npc.character.position.offset.x
        } else {
            npc.character.position.offset.y
        }.abs() as usize >> 3
    ) + npc.character.sprite_index as usize;

    let npc_type = npc_type(&npc.npc_type);
    
    (match npc.character.position.direction {
        Direction::Down => npc_type.sprite.down[index],
        Direction::Up => npc_type.sprite.up[index],
        _ => npc_type.sprite.side[index]
    } << 4) as f32
    
}