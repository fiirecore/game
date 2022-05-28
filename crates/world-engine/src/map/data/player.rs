use crate::engine::{
    graphics::{Color, Draw, DrawExt, DrawParams, DrawShapes, Graphics, Texture},
    math::Rect,
    utils::HashMap,
};

use worldlib::{
    character::{action::ActionQueue, player::PlayerCharacter, Character, MovementType},
    positions::Direction,
    serialized::SerializedPlayerTexture,
    TILE_SIZE,
};

fn screen_x(x: f32) -> f32 {
    x / 2.0
}

fn screen_y(y: f32) -> f32 {
    (y - TILE_SIZE as f32) / 2.0 - TILE_SIZE as f32
}

pub mod bush;

pub struct PlayerTexture {
    pub textures: HashMap<MovementType, CharacterTexture>,

    pub bush: bush::PlayerBushTexture,

    accumulator: f32,
    jumping: bool,
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
    pub fn new(gfx: &mut Graphics, player: SerializedPlayerTexture) -> Result<Self, String> {
        let mut textures = HashMap::with_capacity(3);
        textures.insert(
            MovementType::Walking,
            gfx.create_texture()
                .from_image(&player[MovementType::Walking])
                .build()?
                .into(),
        );
        textures.insert(
            MovementType::Running,
            gfx.create_texture()
                .from_image(&player[MovementType::Running])
                .build()?
                .into(),
        );
        textures.insert(
            MovementType::Swimming,
            CharacterTexture {
                idle: Some(0.5),
                texture: gfx
                    .create_texture()
                    .from_image(&player[MovementType::Swimming])
                    .build()?,
            },
        );

        Ok(Self {
            textures,
            bush: bush::PlayerBushTexture::new(gfx)?,
            accumulator: 0.0,
            jumping: false,
        })
    }

    pub fn jump<P, B: Default>(&mut self, player: &mut PlayerCharacter<P, B>) {
        player.character.sprite = 0;
        player.character.noclip = true;
        player.character.input_lock.increment();
        for _ in 0..2 {
            player
                .character
                .actions
                .queue
                .insert(0, ActionQueue::Move(player.character.position.direction));
        }
        self.accumulator = 0.0;
        self.jumping = true;
    }

    pub fn update<P, B: Default>(&mut self, delta: f32, player: &mut PlayerCharacter<P, B>) {
        self.bush.update(delta);
        match self.jumping {
            false => {
                if player.character.offset.is_zero() {
                    if let Some(texture) = self.textures.get(&player.character.movement) {
                        if let Some(idle) = texture.idle {
                            self.accumulator += delta;
                            if self.accumulator > idle {
                                self.accumulator -= idle;
                                player.character.update_sprite();
                            }
                        }
                    }
                }
            }
            true => {
                if !player.character.moving() {
                    player.character.input_lock.decrement();
                    player.character.noclip = false;
                    self.jumping = false;
                    self.accumulator = 0.0;
                }
            }
        }
    }

    pub fn draw(&self, draw: &mut Draw, character: &Character, color: Color) {
        if !character.hidden {
            if let Some(texture) = self.textures.get(&character.movement) {
                let screen_x = screen_x(draw.width());
                let screen_y = screen_y(draw.height());
                if self.jumping {
                    draw.circle(TILE_SIZE / 2.0)
                        .position(screen_x, screen_y + 24.0)
                        .color(Color::BLACK);
                }
                let (x, width) = current_texture(character);

                let px = screen_x - width / 2.0;

                let y = match self.jumping {
                    true => {
                        let negative = matches!(character.position.direction, Direction::Up);
                        let o = character.offset.offset() / 4.0;
                        match character.actions.queue.is_empty() {
                            false => match negative {
                                true => screen_y + o,
                                false => screen_y - o,
                            },
                            true => match negative {
                                true => screen_y + (TILE_SIZE / 4.0) - o,
                                false => screen_y - (TILE_SIZE / 4.0) + o,
                            },
                        }
                    }
                    false => screen_y,
                };

                draw.texture(
                    &texture.texture,
                    px,
                    y,
                    DrawParams {
                        source: Some(Rect {
                            x,
                            y: 0.0,
                            width,
                            height: if !self.bush.in_bush
                                || (character.offset.is_zero()
                                    && character.position.direction.vertical())
                            {
                                32.0
                            } else {
                                26.0
                            },
                        }),
                        flip_x: character.position.direction == Direction::Right,
                        color,
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
        MovementType::Walking => (
            match character.position.direction {
                Direction::Up => &[1, 5, 1, 6],
                Direction::Down => &[0, 3, 0, 4],
                _ => &[2, 7, 2, 8],
            },
            16.0,
        ),
        MovementType::Running => (
            match character.position.direction {
                Direction::Up => &[6, 7, 6, 8],
                Direction::Down => &[3, 4, 3, 5],
                _ => &[9, 10, 9, 11],
            },
            16.0,
        ),
        MovementType::Swimming => (
            match character.position.direction {
                Direction::Up => &[2, 2, 3, 3],
                Direction::Down => &[0, 0, 1, 1],
                _ => &[4, 4, 5, 5],
            },
            32.0,
        ),
    }
}
