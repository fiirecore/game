use engine::{
    graphics::{draw_rectangle, position, RED},
    tetra::{
        graphics::{Rectangle, Texture},
        math::Vec2,
        Context,
    },
};

use deps::{hash::HashMap, str::TinyStr16};

use worldlib::{character::npc::Npc, positions::Direction, TILE_SIZE};

use crate::world::{npc::npc_type, RenderCoords};

pub type NpcTextures = HashMap<TinyStr16, Texture>;

#[derive(Default)]
pub struct NpcTextureManager {
    pub npcs: NpcTextures,
    // pub trainer: TrainerTextures,
}

impl NpcTextureManager {
    pub fn set(&mut self, npcs: NpcTextures) {
        self.npcs = npcs;
    }

    pub fn draw(&self, ctx: &mut Context, npc: &Npc, screen: &RenderCoords) {
        let x = ((npc.character.position.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x
            + npc.character.offset.x;
        let y = ((npc.character.position.coords.y - 1 + screen.offset.y) << 4) as f32
            - screen.focus.y
            + npc.character.offset.y;

        if let Some(texture) = self.npcs.get(&npc.type_id) {
            let params = if npc.character.position.direction == Direction::Right {
                position(x + 16.0, y).scale(Vec2::new(-1.0, 1.0))
            } else {
                position(x, y)
            };
            texture.draw_region(
                ctx,
                Rectangle::new(current_texture_pos(npc), 0.0, 16.0, 32.0),
                params,
            );
        } else {
            draw_rectangle(ctx, x, y + TILE_SIZE, TILE_SIZE, TILE_SIZE * 2.0, RED);
        }
    }
}

pub fn current_texture_pos(npc: &Npc) -> f32 {
    let index = (npc.character.offset.offset().abs() as usize >> 3) + npc.character.sprite as usize;

    let npc_type = npc_type(&npc.type_id);

    (match npc.character.position.direction {
        Direction::Down => npc_type.sprite.down[index],
        Direction::Up => npc_type.sprite.up[index],
        _ => npc_type.sprite.side[index],
    } << 4) as f32
}
