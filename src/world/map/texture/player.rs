use crate::engine::{
    graphics::{Color, DrawParams, Texture},
    math::{Rectangle, Vec2},
    util::{HEIGHT, WIDTH},
    Context,
};
use std::collections::HashMap;

use worldlib::{
    character::{Character, Movement},
    positions::Direction,
    TILE_SIZE,
};

const SCREEN_X: f32 = (WIDTH as isize >> 1) as f32;
const SCREEN_Y: f32 = ((HEIGHT as isize - TILE_SIZE as isize) >> 1) as f32 - TILE_SIZE;

pub mod bush;

pub struct PlayerTexture {
    pub textures: HashMap<Movement, CharacterTexture>,

    pub bush: bush::PlayerBushTexture,

    accumulator: f32,
}

pub struct CharacterTexture {
    pub idle: Option<f32>,
    pub texture: Texture,
}

impl From<Texture> for CharacterTexture {
    fn from(texture: Texture) -> Self {
        Self {
            idle: None,
            texture,
        }
    }
}

impl PlayerTexture {
    pub fn new(ctx: &mut Context) -> Self {
        bush::new(ctx);
        let mut textures = HashMap::with_capacity(3);
        textures.insert(
            Movement::Walking,
            Texture::new(
                ctx,
                include_bytes!("../../../../assets/world/textures/player/walking.png"),
            ).unwrap()
            .into(),
        );
        textures.insert(
            Movement::Running,
            Texture::new(
                ctx,
                include_bytes!("../../../../assets/world/textures/player/running.png"),
            ).unwrap()
            .into(),
        );
        textures.insert(
            Movement::Swimming,
            CharacterTexture {
                idle: Some(0.5),
                texture: Texture::new(
                    ctx,
                    include_bytes!("../../../../assets/world/textures/player/surfing.png"),
                ).unwrap(),
            },
        );

        Self {
            textures,
            bush: bush::PlayerBushTexture::default(),
            accumulator: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32, character: &mut Character) {
        self.bush.update(delta);
        if character.offset.is_zero() {
            if let Some(texture) = self.textures.get(&character.movement) {
                if let Some(idle) = texture.idle {
                    self.accumulator += delta;
                    if self.accumulator > idle {
                        self.accumulator -= idle;
                        character.update_sprite();
                    }
                }
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, character: &Character, color: Color) {
        if !character.hidden {
            if let Some(texture) = self.textures.get(&character.movement) {
                let (x, width) = current_texture(character);
                texture.texture.draw(
                    ctx,
                    SCREEN_X - width / 2.0,
                    SCREEN_Y,
                    DrawParams {
                        source: Some(Rectangle::new(
                            x,
                            0.0,
                            width,
                            if !self.bush.in_bush
                                || (character.offset.is_zero()
                                    && character.position.direction.vertical())
                            {
                                32.0
                            } else {
                                26.0
                            },
                        )),
                        flip_x: character.position.direction == Direction::Right,
                        color,
                        ..Default::default()
                    },
                );
            }
        }
    }
}

fn current_texture(character: &Character) -> (f32, f32) {
    const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;
    // x, width
    let (indexes, width) = player_texture_index(character);
    (
        (*indexes
            .get(
                (
                    if character.offset.offset().abs() < HALF_TILE_SIZE
                        && !character.offset.is_zero()
                    {
                        1
                    } else {
                        0
                    }
                    //.abs() as usize >> 3
                ) + character.sprite as usize,
            )
            .unwrap_or(&3) as f32)
            * width,
        width,
    )
}

pub const fn player_texture_index(character: &Character) -> (&[u8; 4], f32) {
    match character.movement {
        Movement::Walking => (
            match character.position.direction {
                Direction::Up => &[1, 5, 1, 6],
                Direction::Down => &[0, 3, 0, 4],
                _ => &[2, 7, 2, 8],
            },
            16.0,
        ),
        Movement::Running => (
            match character.position.direction {
                Direction::Up => &[6, 7, 6, 8],
                Direction::Down => &[3, 4, 3, 5],
                _ => &[9, 10, 9, 11],
            },
            16.0,
        ),
        Movement::Swimming => (
            match character.position.direction {
                Direction::Up => &[2, 2, 3, 3],
                Direction::Down => &[0, 0, 1, 1],
                _ => &[4, 4, 5, 5],
            },
            32.0,
        ),
    }
}
