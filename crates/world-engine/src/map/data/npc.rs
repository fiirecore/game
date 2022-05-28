use pokengine::engine::{
    graphics::{Color, Draw, DrawExt, DrawParams, Graphics, Texture},
    math::Rect,
};
use worldlib::{
    character::npc::{
        group::{NpcGroup, NpcGroupId},
        Npc,
    },
    positions::Direction,
};

use crate::engine::utils::HashMap;

use crate::map::RenderCoords;

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
    pub fn new(gfx: &mut Graphics, textures: HashMap<NpcGroupId, Vec<u8>>) -> Result<Self, String> {
        let mut npcs = HashMap::with_capacity(textures.len());
        // let trainer = NpcGroupTextures::new(Default::default());
        for (npc, data) in textures {
            let texture = gfx.create_texture().from_image(&data).build()?;
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
            //     gfx.create_texture().from_image(&npc.texture).unwrap(),
            // );
        }

        Ok(Self(
            npcs,
            // trainer,
        ))
    }

    pub fn draw(&self, draw: &mut Draw, npc: &Npc, screen: &RenderCoords, color: Color) {
        self.0
            .get(&npc.group)
            .unwrap_or_else(|| {
                self.0
                    .get(&NpcGroup::PLACEHOLDER)
                    .unwrap_or_else(|| panic!("Cannot get placeholder NPC texture!"))
            })
            .draw(draw, npc, screen, color);
    }
}

impl NpcTexture {
    pub fn draw(&self, draw: &mut Draw, npc: &Npc, screen: &RenderCoords, color: Color) {
        let x = ((npc.character.position.coords.x + 1 + screen.offset.x) << 4) as f32
            - screen.focus.x
            + npc.character.offset.x
            - self.data.width;

        let y = ((npc.character.position.coords.y + screen.offset.y) << 4) as f32 - screen.focus.y
            + npc.character.offset.y
            - (self.texture.height() - worldlib::TILE_SIZE);
        draw.texture(
            &self.texture,
            x,
            y,
            DrawParams {
                source: Some(Rect {
                    x: self.current_texture_pos(npc),
                    y: 1.0,
                    width: self.data.width,
                    height: self.texture.height(),
                }),
                flip_x: npc.character.position.direction == Direction::Right,
                color,
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
