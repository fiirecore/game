use crate::{
    util::Reset,
    pokedex::{
        pokemon::instance::PokemonInstance,
        texture::{PokemonTexture, pokemon_texture},
    },
    macroquad::prelude::{Texture2D, Vec2, draw_texture_ex, Color, DrawTextureParams, Rect},
    graphics::{byte_texture, DRAW_COLOR},
};

use crate::battle::ui::{BattleGuiPosition, BattleGuiPositionIndex};

pub mod status;
pub mod bounce;

pub struct PokemonRenderer {

    pub texture: Option<Texture2D>,
    side: PokemonTexture,

    pub pos: Vec2,

    pub spawner: Spawner,
    pub faint: Faint,
    pub flicker: Flicker,

}

pub struct Spawner {
    spawning: SpawnerState,
    texture: Texture2D,
    x: f32,
}

#[derive(PartialEq)]
enum SpawnerState {
    None,
    Start,
    Throwing,
    Spawning,
}

impl Default for Spawner {
    fn default() -> Self {
        Self {
            spawning: SpawnerState::None,
            x: 0.0,
            texture: unsafe { *POKEBALL.get_or_insert(byte_texture(include_bytes!("../../../../assets/battle/thrown_pokeball.png"))) }
        }
    }
}

static mut POKEBALL: Option<Texture2D> = None;

impl Spawner {

    const LEN: f32 = 20.0;
    const ORIGIN: f32 = 0.0;
    const OFFSET: f32 = -5.0;
    const PARABOLA_ORIGIN: f32 = (Self::LEN / 3.0);

    fn f(x: f32) -> f32 {
        0.5 * (x - Self::PARABOLA_ORIGIN).powi(2) - 50.0
    }

    pub fn update(&mut self, delta: f32) {
        match self.spawning {
            SpawnerState::Start => {
                self.x = Self::ORIGIN;
                self.spawning = SpawnerState::Throwing;
                self.update(delta);
            }
            SpawnerState::Throwing => {
                self.x += delta * 20.0;
                if self.x > Self::LEN {
                    self.spawning = SpawnerState::Spawning;
                    self.x = 0.0;
                }
            }
            SpawnerState::Spawning => {
                self.x += delta * 1.5;
                if self.x > 1.0 {
                    self.spawning = SpawnerState::None;
                }
            }
            SpawnerState::None => (),
        }
    }

    fn render(&self, origin: Vec2, texture: Texture2D) {
        match self.spawning {
            SpawnerState::Throwing => 
            draw_texture_ex(
                self.texture,
                origin.x + self.x + Self::OFFSET,
                origin.y + Self::f(self.x),
                DRAW_COLOR,
                DrawTextureParams {
                    source: Some(Rect { x: 0.0, y: 0.0, w: 12.0, h: 12.0 }),
                    rotation: self.x,
                    ..Default::default()
                }
            ),
            SpawnerState::Spawning => {
                let scale = self.x * texture.height();
                let mut y = origin.y - scale * 1.5;
                let max = origin.y - texture.height();
                if y < max {
                    y = max;
                }
                draw_texture_ex(
                    texture, 
                    origin.x, 
                    y,
                    DRAW_COLOR,
                    // Color { r: self.x, g: self.x, b: self.x, a: 1.0 }, 
                    DrawTextureParams::default()
                )
            },
            SpawnerState::None | SpawnerState::Start => (),
        }
    }

    pub fn spawning(&self) -> bool {
        self.spawning != SpawnerState::None
    }

}

#[derive(Default)]
pub struct Flicker {
    remaining: u8,
    accumulator: f32,
}

impl Flicker {

    const HALF: f32 = Self::LENGTH / 2.0;
    const LENGTH: f32 = 0.20;
    const TIMES: u8 = 4;

    pub fn update(&mut self, delta: f32) {
        if self.remaining != 0 {
            self.accumulator += delta;
            if self.accumulator > Self::LENGTH {
                self.accumulator -= Self::LENGTH;
                self.remaining -= 1;
            }
            if self.remaining == 0 {
                self.accumulator = 0.0;
            }
        }
    }

    pub fn flickering(&self) -> bool {
        self.remaining != 0
    }

}

#[derive(Default)]
pub struct Faint {
    fainting: bool,
    remaining: f32,
}

impl Faint {

    pub fn update(&mut self, delta: f32) {
        if self.fainting {
            self.remaining -= delta * 128.0;
            if self.remaining < 0.0 {
                self.remaining = 0.0;
            }
        }
    }

    pub fn fainting(&self) -> bool {
        self.fainting && self.remaining != 0.0
    }

}

impl PokemonRenderer {

    pub fn new(index: BattleGuiPositionIndex, side: PokemonTexture) -> Self {
        Self {
            texture: None,
            side,
            pos: Self::position(index),
            spawner: Spawner::default(),
            faint: Faint::default(),
            flicker: Flicker::default(),
        }
    }

    pub fn with(index: BattleGuiPositionIndex, pokemon: &PokemonInstance, side: PokemonTexture) -> Self {
        Self {
            texture: Some(pokemon_texture(&pokemon.pokemon.id(), side)),
            ..Self::new(index, side)
        }
    }

    fn position(index: BattleGuiPositionIndex) -> Vec2 {
        match index.position {
            BattleGuiPosition::Top => Vec2::new(144.0 - (index.size - 1) as f32 * 32.0 + index.index as f32 * 64.0, 74.0),
            BattleGuiPosition::Bottom => Vec2::new(40.0 - (index.size - 1) as f32 * 32.0 + index.index as f32 * 64.0, 113.0),
        }
    }

    pub fn new_pokemon(&mut self, pokemon: Option<&PokemonInstance>) {
        self.texture = pokemon.map(|pokemon| pokemon_texture(pokemon.pokemon.id(), self.side));
        self.reset();
    }

    pub fn spawn(&mut self) {
        self.spawner.spawning = SpawnerState::Start;
        self.spawner.x = 0.0;
    }

    pub fn faint(&mut self) {
        if let Some(texture) =self.texture.as_ref() {
            self.faint.fainting = true;
            self.faint.remaining = texture.height();
        }
    }

    pub fn flicker(&mut self) {
        self.flicker.remaining = Flicker::TIMES;
        self.flicker.accumulator = 0.0;
    }

    pub fn render(&self, offset: Vec2, color: Color) {
        if let Some(texture) = self.texture {
            let pos = self.pos + offset;
            if self.spawner.spawning() {
                self.spawner.render(pos, texture);
            } else if self.flicker.accumulator < Flicker::HALF {
                if self.faint.fainting {
                    if self.faint.remaining > 0.0 {
                        draw_texture_ex(
                            texture,
                            pos.x,
                            pos.y - self.faint.remaining,
                            color,
                            DrawTextureParams {
                                source: Some(
                                    Rect::new(0.0, 0.0, texture.width(), self.faint.remaining)
                                ),
                                ..Default::default()
                            }
                        );
                    }
                } else {
                    draw_texture_ex(texture, pos.x, pos.y - texture.height(), color, DrawTextureParams::default());
                }
            }
        }
    }

}

impl Reset for PokemonRenderer {
    fn reset(&mut self) {
        self.faint = Faint::default();
        self.flicker = Flicker::default();
        self.spawner.spawning = SpawnerState::None;
    }
}