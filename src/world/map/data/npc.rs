use hashbrown::HashMap;

use worldlib::{
    character::{
        npc::{
            group::{NpcGroup, NpcGroupId},
            Npc,
        },
        sprite::SpriteIndices,
    },
    positions::Direction,
};

use crate::world::RenderCoords;

use crate::engine::{
    error::ImageError,
    graphics::{DrawParams, Texture},
    math::Rectangle,
    Context,
};

pub struct NpcTextures(pub HashMap<NpcGroupId, NpcTexture>);

pub struct NpcTexture {
    pub texture: Texture,
    pub indices: SpriteIndices,
    // pub trainer: NpcGroupTextures,
}

impl NpcTextures {
    pub fn new(
        ctx: &mut Context,
        textures: HashMap<NpcGroupId, Vec<u8>>,
    ) -> Result<Self, ImageError> {
        let mut npcs = HashMap::with_capacity(textures.len());
        // let trainer = NpcGroupTextures::new(Default::default());
        for (npc, data) in textures {
            let texture = Texture::new(ctx, &data)?;
            let indices = if (texture.width() as u16) < 96 {
                SpriteIndices::STILL
            } else {
                SpriteIndices::WALK
            };
            npcs.insert(npc, NpcTexture { texture, indices });
            // trainer.insert(
            //     npc.config.identifier,
            //     Texture::new(ctx, &npc.texture).unwrap(),
            // );
        }

        Ok(Self(
            npcs,
            // trainer,
        ))
    }

    pub fn draw(&self, ctx: &mut Context, npc: &Npc, screen: &RenderCoords) {
        self.0
            .get(&npc.group)
            .unwrap_or_else(|| {
                self.0
                    .get(&NpcGroup::PLACEHOLDER)
                    .unwrap_or_else(|| panic!("Cannot get placeholder NPC texture!"))
            })
            .draw(ctx, npc, screen);
    }
}

impl NpcTexture {
    pub fn draw(&self, ctx: &mut Context, npc: &Npc, screen: &RenderCoords) {
        let x = ((npc.character.position.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x
            + npc.character.offset.x;
        let y = ((npc.character.position.coords.y - 1 + screen.offset.y) << 4) as f32
            - screen.focus.y
            + npc.character.offset.y;

        // {
        self.texture.draw(
            ctx,
            x,
            y,
            DrawParams {
                flip_x: npc.character.position.direction == Direction::Right,
                source: Some(Rectangle::new(
                    self.current_texture_pos(npc),
                    0.0,
                    16.0,
                    32.0,
                )),
                ..Default::default()
            },
        );
        // } else {
        //     // draw_rectangle(
        //     //     ctx,
        //     //     x,
        //     //     y + TILE_SIZE,
        //     //     TILE_SIZE,
        //     //     TILE_SIZE * 2.0,
        //     //     Color::rgb(1.0, 0.0, 0.0),
        //     // );
        // }
    }

    pub fn current_texture_pos(&self, npc: &Npc) -> f32 {
        let index =
            (npc.character.offset.offset().abs() as usize >> 3) + npc.character.sprite as usize;

        (match npc.character.position.direction {
            Direction::Down => self.indices.down[index],
            Direction::Up => self.indices.up[index],
            _ => self.indices.side[index],
        } << 4) as f32
    }
}
