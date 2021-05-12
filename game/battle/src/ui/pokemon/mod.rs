use game::{
    util::Reset,
    pokedex::{
        pokemon::instance::PokemonInstance,
        texture::{PokemonTexture, pokemon_texture},
    },
    macroquad::prelude::{Texture2D, Vec2, draw_texture_ex, DrawTextureParams, Rect, WHITE},
    graphics::draw_bottom,
};

use crate::ui::{BattleGuiPosition, BattleGuiPositionIndex};

pub mod status;
pub mod bounce;

pub struct PokemonRenderer {

    pub texture: Option<Texture2D>,
    side: PokemonTexture,

    pub pos: Vec2,

    pub faint: Faint,
    pub flicker: Flicker,

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
            faint: Faint::default(),
            flicker: Flicker::default(),
        }
    }

    pub fn with(index: BattleGuiPositionIndex, pokemon: &PokemonInstance, side: PokemonTexture) -> Self {
        Self {
            texture: Some(pokemon_texture(&pokemon.pokemon.data.id, side)),
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
        self.texture = pokemon.map(|pokemon| pokemon_texture(&pokemon.pokemon.data.id, self.side));
        self.reset();
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

    pub fn render(&self, offset: Vec2) {
        if let Some(texture) = self.texture {
            if self.flicker.accumulator < Flicker::HALF {
                let pos = self.pos + offset;
                if self.faint.fainting {
                    if self.faint.remaining > 0.0 {
                        draw_texture_ex(
                            texture,
                            pos.x,
                            pos.y - self.faint.remaining,
                            WHITE,
                            DrawTextureParams {
                                source: Some(
                                    Rect::new(0.0, 0.0, texture.width(), self.faint.remaining)
                                ),
                                ..Default::default()
                            }
                        );
                    }
                } else {
                    draw_bottom(texture, pos.x, pos.y);
                }
            }
        }
    }

}

impl Reset for PokemonRenderer {
    fn reset(&mut self) {
        self.faint = Faint::default();
        self.flicker = Flicker::default();
    }
}