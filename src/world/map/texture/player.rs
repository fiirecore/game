use deps::hash::HashMap;
use engine::{
    graphics::{byte_texture, position},
    tetra::{
        graphics::{Color, Rectangle, Texture},
        math::Vec2,
        Context,
    },
    util::{HEIGHT, WIDTH},
};

use worldlib::{
    character::{Character, MoveType},
    positions::Direction,
    TILE_SIZE,
};

const SCREEN_X: f32 = (WIDTH as isize >> 1) as f32;
const SCREEN_Y: f32 = ((HEIGHT as isize - TILE_SIZE as isize) >> 1) as f32 - TILE_SIZE;

pub mod bush;

pub struct PlayerTexture {
    pub draw: bool,

    pub textures: HashMap<MoveType, CharacterTexture>,

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
            MoveType::Walking,
            byte_texture(
                ctx,
                include_bytes!("../../../../assets/world/player/walking.png"),
            )
            .into(),
        );
        textures.insert(
            MoveType::Running,
            byte_texture(
                ctx,
                include_bytes!("../../../../assets/world/player/running.png"),
            )
            .into(),
        );
        textures.insert(
            MoveType::Swimming,
            CharacterTexture {
                idle: Some(0.5),
                texture: byte_texture(
                    ctx,
                    include_bytes!("../../../../assets/world/player/surfing.png"),
                ),
            },
        );

        Self {
            draw: true,
            textures,
            bush: bush::PlayerBushTexture::default(),
            accumulator: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32, character: &mut Character) {
        self.bush.update(delta);
        if !character.moving {
            if let Some(texture) = self.textures.get(&character.move_type) {
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
        if self.draw {
            if let Some(texture) = self.textures.get(&character.move_type) {
                let (x, width) = current_texture(character);
                let params = if character.position.direction == Direction::Right {
                    position(SCREEN_X + width / 2.0, SCREEN_Y).scale(Vec2::new(-1.0, 1.0))
                } else {
                    position(SCREEN_X - width / 2.0, SCREEN_Y)
                };
                texture.texture.draw_region(
                    ctx,
                    Rectangle::new(
                        x,
                        0.0,
                        width,
                        if !self.bush.in_bush
                            || (character.moving && character.position.direction.vertical())
                        {
                            32.0
                        } else {
                            26.0
                        },
                    ),
                    params.color(color),
                );
            }
        }
    }
}

fn current_texture(character: &Character) -> (f32, f32) {
    // x, width
    let (indexes, width) = player_texture_index(character);
    (
        (*indexes
            .get(
                (
                    if if character.position.offset.x != 0.0 {
                        character.position.offset.x
                    } else {
                        character.position.offset.y
                    }
                    .abs()
                        < 8.0
                        && character.moving
                    {
                        1
                    } else {
                        0
                    }
                    //.abs() as usize >> 3
                ) + character.sprite_index as usize,
            )
            .unwrap_or(&3) as f32)
            * width,
        width,
    )
}

pub const fn player_texture_index(character: &Character) -> ([u8; 4], f32) {
    match character.move_type {
        MoveType::Walking => (
            match character.position.direction {
                Direction::Up => [1, 5, 1, 6],
                Direction::Down => [0, 3, 0, 4],
                _ => [2, 7, 2, 8],
            },
            16.0,
        ),
        MoveType::Running => (
            match character.position.direction {
                Direction::Up => [6, 7, 6, 8],
                Direction::Down => [3, 4, 3, 5],
                _ => [9, 10, 9, 11],
            },
            16.0,
        ),
        MoveType::Swimming => (
            match character.position.direction {
                Direction::Up => [2, 2, 3, 3],
                Direction::Down => [0, 0, 1, 1],
                _ => [4, 4, 5, 5],
            },
            32.0,
        ),
    }
}
