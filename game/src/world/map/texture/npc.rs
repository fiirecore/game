use crate::{
    deps::{
        str::TinyStr16,
        hash::HashMap,
    },
    util::{
        TILE_SIZE, 
        Direction,
    },
    graphics::{byte_texture, position, RED, draw_rectangle},
    tetra::{
        Context,
        math::Vec2,
        graphics::{
            Texture,
            Rectangle,
        }
    },
};

use worldlib::{
    serialized::SerializedNpcType,
    character::npc::{
        Npc,
        npc_type::NpcTypeId,
    },
};

use crate::world::{
    npc::npc_type,
    RenderCoords,
};

pub type NpcTextures = HashMap<TinyStr16, Texture>;
pub type TrainerTextures = NpcTextures;

static mut TRAINER_TEXTURES: Option<TrainerTextures> = None;

#[derive(Default)]
pub struct NpcTextureManager {

    pub npcs: NpcTextures,
    // pub trainer: TrainerTextures,

}

impl NpcTextureManager {

    pub fn trainer_texture(npc_type: &NpcTypeId) -> &'static Texture {
        unsafe { TRAINER_TEXTURES.as_ref().expect("Could not get trainer textures! (Not initialized)").get(npc_type).unwrap_or_else(|| panic!("Could not get trainer texture for Npc Type {}", npc_type)) }
    }

    pub fn with_capacity(&mut self, capacity: usize) {
        self.npcs.reserve(capacity);
        unsafe { TRAINER_TEXTURES = Some(HashMap::with_capacity(capacity)); }
    }

    pub fn add_npc_type(&mut self, ctx: &mut Context, npc_type: &SerializedNpcType) {
        self.npcs.insert(npc_type.config.identifier, byte_texture(ctx, &npc_type.texture));
        if let Some(texture) = &npc_type.battle_texture {
            unsafe {
                TRAINER_TEXTURES.as_mut().unwrap().insert(npc_type.config.identifier, byte_texture(ctx, texture));
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, npc: &Npc, screen: &RenderCoords) {
        let x = ((npc.character.position.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x + npc.character.position.offset.x;
        let y = ((npc.character.position.coords.y - 1 + screen.offset.y) << 4) as f32 - screen.focus.y + npc.character.position.offset.y;
        
        if let Some(texture) = self.npcs.get(&npc.npc_type) {
            let params = if npc.character.position.direction == Direction::Right {
                position(x + 32.0, y).scale(Vec2::new(-1.0, 1.0))
            } else {
                position(x, y)
            };
            texture.draw_region(
                ctx,
                Rectangle::new(
                    current_texture_pos(npc),
                    0.0,
                    16.0,
                    32.0,
                ),
                params
            );
        } else {
            draw_rectangle(ctx, x, y + TILE_SIZE, TILE_SIZE, TILE_SIZE * 2.0, RED);
        }
    }

}

pub fn current_texture_pos(npc: &Npc) -> f32 {
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