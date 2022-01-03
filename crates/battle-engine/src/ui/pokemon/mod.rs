use pokedex::{
    engine::{
        graphics::{Color, DrawParams, Texture},
        math::{Rectangle, Vec2},
        utils::Reset,
        Context,
    },
    pokemon::PokemonId,
    texture::PokemonTexture,
    PokedexClientData,
};

use crate::{
    context::BattleGuiData,
    ui::{BattleGuiPosition, BattleGuiPositionIndex},
};

use self::{
    faint::Faint,
    flicker::Flicker,
    spawner::{Spawner, SpawnerState},
};

// mod moves;
mod status;

// pub use moves::*;
pub use status::*;
pub mod bounce;

pub mod faint;
pub mod flicker;
pub mod spawner;

#[derive(Clone)]
pub struct PokemonRenderer {
    // pub moves: MoveRenderer,
    pub pokemon: Option<Texture>,
    side: PokemonTexture,

    pub pos: Vec2,

    pub spawner: Spawner,
    pub faint: Faint,
    pub flicker: Flicker,
}

impl PokemonRenderer {
    pub fn new(ctx: &BattleGuiData, index: BattleGuiPositionIndex, side: PokemonTexture) -> Self {
        Self {
            // moves: MoveRenderer::new(index.position),
            pokemon: None,
            side,
            pos: Self::position(index),
            spawner: Spawner::new(ctx, None),
            faint: Faint::default(),
            flicker: Flicker::default(),
        }
    }

    pub fn with<'d>(
        ctx: &BattleGuiData,
        data: &PokedexClientData,
        index: BattleGuiPositionIndex,
        pokemon: Option<PokemonId>,
        side: PokemonTexture,
    ) -> Self {
        Self {
            pokemon: pokemon
                .map(|pokemon| data.pokemon_textures.get(&pokemon, side))
                .flatten()
                .cloned(),
            spawner: Spawner::new(ctx, pokemon),
            ..Self::new(ctx, index, side)
        }
    }

    fn position(index: BattleGuiPositionIndex) -> Vec2 {
        let offset = (index.size - 1) as f32 * 32.0 - index.index as f32 * 64.0;
        match index.position {
            BattleGuiPosition::Top => Vec2::new(144.0 - offset, 74.0),
            BattleGuiPosition::Bottom => Vec2::new(40.0 - offset, 113.0),
        }
    }

    pub fn new_pokemon<'d>(&mut self, data: &PokedexClientData, pokemon: Option<PokemonId>) {
        self.spawner.id = pokemon;
        self.pokemon = pokemon
            .map(|pokemon| data.pokemon_textures.get(&pokemon, self.side))
            .flatten()
            .cloned();
        self.reset();
    }

    pub fn spawn(&mut self) {
        self.spawner.spawning = SpawnerState::Start;
        self.spawner.x = 0.0;
    }

    pub fn faint(&mut self) {
        if let Some(texture) = self.pokemon.as_ref() {
            self.faint.fainting = true;
            self.faint.remaining = texture.height() as f32;
        }
    }

    pub fn flicker(&mut self) {
        self.flicker.remaining = Flicker::TIMES;
        self.flicker.accumulator = 0.0;
    }

    pub fn draw(&self, ctx: &mut Context, offset: Vec2, color: Color) {
        if let Some(texture) = &self.pokemon {
            let pos = self.pos + offset;
            if self.spawner.spawning() {
                self.spawner.draw(ctx, pos, texture);
            } else if self.flicker.accumulator < Flicker::HALF {
                if self.faint.fainting {
                    if self.faint.remaining > 0.0 {
                        texture.draw(
                            ctx,
                            pos.x,
                            pos.y - self.faint.remaining,
                            DrawParams {
                                source: Some(Rectangle::new(
                                    0.0,
                                    0.0,
                                    texture.width() as f32,
                                    self.faint.remaining,
                                )),
                                color,
                                ..Default::default()
                            },
                        );
                    }
                } else {
                    texture.draw(
                        ctx,
                        pos.x, //+ self.moves.pokemon_x(),
                        pos.y - texture.height() as f32,
                        DrawParams::color(color),
                    );
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
