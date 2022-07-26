use pokengine::engine::{
    graphics::{Color, Draw, DrawExt, DrawParams, Graphics, Texture},
    math::Rect,
};
use worldlib::{
    character::{CharacterGroupId, CharacterState},
    positions::Direction,
};

use crate::{engine::utils::HashMap, map::CharacterCamera};

pub struct NpcTextures(HashMap<CharacterGroupId, NpcTexture>);

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
        gfx: &mut Graphics,
        textures: HashMap<CharacterGroupId, Vec<u8>>,
    ) -> Result<Self, String> {
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

    pub fn draw(
        &self,
        draw: &mut Draw,
        character: &CharacterState,
        // offset: Option<&PixelOffset>,
        camera: &CharacterCamera,
        color: Color,
    ) {
        self.0
            .get(&character.group)
            .unwrap_or_else(|| {
                self.0
                    .get(&CharacterState::PLACEHOLDER)
                    .unwrap_or_else(|| panic!("Cannot get placeholder NPC texture!"))
            })
            .draw(draw, character, camera, color);
    }
}

impl NpcTexture {
    pub fn draw(
        &self,
        draw: &mut Draw,
        character: &CharacterState,
        camera: &CharacterCamera,
        color: Color,
    ) {
        if !character.hidden {
            let x = ((character.position.coords.x + 1 + camera.offset.x) << 4) as f32
                - camera.focus.x
                + character.offset.x
                - self.data.width;

            let y = ((character.position.coords.y + camera.offset.y) << 4) as f32 - camera.focus.y
                + character.offset.y
                - (self.texture.height() - worldlib::TILE_SIZE);
            draw.texture(
                &self.texture,
                x,
                y,
                DrawParams {
                    source: Some(Rect {
                        x: self.current_texture_pos(character),
                        y: 1.0,
                        width: self.data.width,
                        height: self.texture.height(),
                    }),
                    flip_x: character.position.direction == Direction::Right,
                    color,
                },
            );
        }
    }

    pub fn current_texture_pos(&self, character: &CharacterState) -> f32 {
        let index = (character.offset.offset().abs() as usize >> 3) + character.sprite as usize;

        (match character.position.direction {
            Direction::Down => self.data.down[index],
            Direction::Up => self.data.up[index],
            _ => self.data.side[index],
        } << 4) as f32
    }
}
