// use game::pokedex::moves::battle_script::BattleActionScript;
use game::{
    util::Reset,
    pokedex::{
        pokemon::instance::PokemonInstance,
        texture::{PokemonTexture, pokemon_texture},
    },
    macroquad::prelude::{Texture2D, Vec2, draw_texture_ex, DrawTextureParams, Rect, WHITE},
};

use crate::gui::{BattleGuiPosition, BattleGuiPositionIndex};

pub struct ActivePokemonRenderer {

    pub texture: Option<Texture2D>,
    side: PokemonTexture,

    pub pos: Vec2,
    missing: f32,

    flicker: Flicker,

}

#[derive(Default)]
struct Flicker {
    remaining: u8,
    accumulator: f32,
}

impl Flicker {
    const LENGTH: f32 = 0.20;
    const TIMES: u8 = 4;
}

impl ActivePokemonRenderer {

    const MISSING: f32 = 0.0;
    const SIZE: f32 = 64.0;

    pub fn new(index: BattleGuiPositionIndex, side: PokemonTexture) -> Self {
        Self {
            texture: None,
            side,
            pos: Self::position(index),
            missing: Self::MISSING,
            flicker: Flicker::default(),
        }
    }

    pub fn with(index: BattleGuiPositionIndex, pokemon: &PokemonInstance, side: PokemonTexture) -> Self {
        Self {
            texture: Some(pokemon_texture(&pokemon.pokemon.data.id, side)),
            ..Self::new(index, side)
        }
    }

    pub fn update_pokemon(&mut self, pokemon: Option<&PokemonInstance>) {
        self.texture = pokemon.map(|pokemon| pokemon_texture(&pokemon.pokemon.data.id, self.side));
        self.reset();
    }

    fn position(index: BattleGuiPositionIndex) -> Vec2 {
        match index.position {
            BattleGuiPosition::Top => Vec2::new(144.0 - (index.size - 1) as f32 * 32.0 + index.index as f32 * 64.0, 74.0),
            BattleGuiPosition::Bottom => Vec2::new(40.0 - (index.size - 1) as f32 * 32.0 + index.index as f32 * 64.0, 113.0),
        }
    }

    pub fn update_faint(&mut self, delta: f32) {
        if self.flicker.remaining == 0 {
            self.missing += delta * 128.0;
        }
    }

    pub fn faint_finished(&self) -> bool {
        self.missing >= Self::SIZE
    }

    pub fn update_flicker(&mut self, delta: f32) {
        if self.flicker.remaining != 0 {
            self.flicker.accumulator += delta;
            if self.flicker.accumulator > Flicker::LENGTH {
                self.flicker.accumulator -= Flicker::LENGTH;
                self.flicker.remaining -= 1;
            }
            if self.flicker.remaining == 0 {
                self.flicker.accumulator = 0.0;
            }
        }
    }

    // pub fn update_script(&mut self, delta: f32) {
    //     if let Some(script) = self.script.as_mut() {
    //         let mut pop = false;
    //         for action in &script.actions {
    //             match action {
    //                 BattleActionActions::MoveAndReturn(pos) => {
    //                     if !self.leaving {
    //                         self.offset.x += delta * 120.0;
    //                         if self.offset.x > *pos {
    //                             self.offset.x = *pos;
    //                             self.leaving = true;
    //                         }
    //                     } else {
    //                         self.offset.x -= delta * 120.0;
    //                         if self.offset.x < 0.0 {
    //                             self.offset.x = 0.0;
    //                             self.leaving = false;
    //                             pop = true;
    //                         }
    //                     }
    //                 }
    //                 BattleActionActions::SpawnTexture => {
    //                     println!("Spawn texture");
    //                     self.texture_active = true;
    //                     pop = true;
    //                 }
    //                 BattleActionActions::Wait(time) => {
    //                     self.tex_accumulator += delta;
    //                     if self.tex_accumulator > *time {
    //                         self.tex_accumulator = 0.0;
    //                         pop = true;
    //                     }
    //                 }
    //                 // BattleActionActions::MoveTexture(x, y, speed) => {
    //                 //     self.texture_offset.x = (*x) * delta * (*speed);
    //                 //     self.texture_offset.y = (*y) * delta * (*speed);
    //                 //     if self.texture_offset.x.eq(x) && self.texture_offset.y.eq(y) {
    //                 //         pop = true;
    //                 //     }
    //                 // }
    //                 BattleActionActions::DespawnTexture => {
    //                     println!("Despawn texture with {:?}", script.texture);
    //                     self.texture_active = false;
    //                     pop = true;
    //                 }
    //             }
    //         }
    //         if pop {
    //             script.actions.pop_front();
    //         }
    //         if script.actions.is_empty() {
    //             self.script = None;
    //         }
    //     }
    // }

    // pub fn is_finished(&self) -> bool {
    //     // self.script.as_ref().map(|script| script.actions.is_empty()).unwrap_or_default()
    // }

    pub fn flicker(&mut self) {
        self.flicker.remaining = Flicker::TIMES;
        self.flicker.accumulator = 0.0;
    }

    pub fn render(&self, offset: Vec2) {
        if let Some(texture) = self.texture {
            if self.missing < Self::SIZE && self.flicker.accumulator < Flicker::LENGTH / 2.0 {
                draw_texture_ex(
                    texture,
                    self.pos.x + offset.x,
                    self.pos.y - texture.height() + offset.y + self.missing,
                    WHITE,
                    DrawTextureParams {
                        source: if self.missing > 0.0 {
                            Some(
                                Rect::new(0.0, 0.0, texture.width(), Self::SIZE - self.missing)
                            )
                        } else {
                            None
                        },
                        ..Default::default()
                    }
                );
            }
        }
    }

}

impl Reset for ActivePokemonRenderer {
    fn reset(&mut self) {
        self.missing = Self::MISSING;
    }
}