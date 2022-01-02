use hashbrown::HashMap;

use worldlib::{
    character::{
        npc::{
            group::{NpcGroup, NpcGroupId},
            Npc,
        },
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

pub struct NpcTextures(HashMap<NpcGroupId, NpcTexture>);

struct NpcTexture {
    pub texture: Texture,
    pub data: SpriteData,
    // pub trainer: NpcGroupTextures,
}

struct SpriteData {
    pub width: f32,
    pub up: [u8; 4],
    pub down: [u8; 4],
    pub side: [u8; 4],
}

impl SpriteData {

    pub const fn still(width: f32) -> Self {
        Self {
            width,
            up: [1; 4],
            down: [0; 4],
            side: [2; 4],
        }
    }

    pub const fn walk(width: f32) -> Self {
        Self {
            width,
            up: [1, 5, 1, 6],
            down: [0, 3, 0, 4],
            side: [2, 7, 2, 8],
        }
    }
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
            let w = texture.width() as u16;
            let data = if w < 96 {
                SpriteData::still(16.0)
            } else {
                match w > 280 {
                    true => SpriteData::walk(32.0),
                    false => SpriteData::walk(16.0),
                }
            };
            npcs.insert(npc, NpcTexture { texture, data });
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
        let x = ((npc.character.position.coords.x + 1 + screen.offset.x) << 4) as f32
            - screen.focus.x
            + npc.character.offset.x
            - self.data.width;

        let y = ((npc.character.position.coords.y + screen.offset.y) << 4) as f32 - screen.focus.y
            + npc.character.offset.y
            - (self.texture.height() - worldlib::TILE_SIZE);

        self.texture.draw(
            ctx,
            x,
            y,
            DrawParams {
                flip_x: npc.character.position.direction == Direction::Right,
                source: Some(Rectangle::new(
                    self.current_texture_pos(npc),
                    0.0,
                    self.data.width,
                    self.texture.height(),
                )),
                ..Default::default()
            },
        );
    }

    pub fn current_texture_pos(&self, npc: &Npc) -> f32 {
        let index =
            (npc.character.offset.offset().abs() as usize >> 3) + npc.character.sprite as usize;

        (match npc.character.position.direction {
            Direction::Down => self.data.down[index],
            Direction::Up => self.data.up[index],
            _ => self.data.side[index],
        } << 4) as f32
    }
}
