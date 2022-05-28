use std::{ops::Deref, cell::RefCell};

use pokengine::{
    engine::{
        graphics::{Color, Draw, DrawExt, DrawImages, DrawParams, Texture},
        math::{Rect, Vec2},
        utils::HashMap,
    },
    pokedex::{
        item::Item,
        moves::Move,
        pokemon::{Pokemon, PokemonId},
    },
    texture::PokemonTexture,
    PokedexClientData,
};

use crate::{
    context::BattleGuiData,
    players::{GuiLocalPlayer, GuiRemotePlayers},
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

pub struct PokemonRenderer<D: Deref<Target = PokedexClientData>> {
    dexengine: D,
    texture: Texture,
    sprites: RefCell<HashMap<(bool, usize), PokemonSprite>>,
}

#[derive(Clone)]
pub struct PokemonSprite {
    pub current: Option<Texture>,
    pub spawner: Spawner,
    pub faint: Faint,
    pub flicker: Flicker,
}

impl PokemonSprite {
    const SIZE: f32 = 64.0;

    pub fn new(texture: &Texture) -> Self {
        Self {
            spawner: Spawner::new(texture.clone(), None),
            faint: Faint::default(),
            flicker: Flicker::default(),
            current: None,
        }
    }

    pub fn reset(&mut self) {
        self.faint = Faint::default();
        self.flicker = Flicker::default();
        self.spawner.spawning = SpawnerState::None;
    }

    pub fn flicker(&mut self) {
        self.flicker.remaining = Flicker::TIMES;
        self.flicker.accumulator = 0.0;
    }

    pub fn faint(&mut self) {
        self.faint.fainting = true;
        self.faint.remaining = self
            .current
            .as_ref()
            .map(|t| t.height())
            .unwrap_or(Self::SIZE);
    }
}


const fn local(local: bool) -> PokemonTexture {
    match local {
        true => PokemonTexture::Back,
        false => PokemonTexture::Front,
    }
}

impl<D: Deref<Target = PokedexClientData>> PokemonRenderer<D> {
    pub fn new(dexengine: D, ctx: &BattleGuiData) -> Self {
        Self {
            dexengine,
            texture: ctx.pokeball.clone(),
            sprites: Default::default(),
        }
    }


    fn position(index: BattleGuiPositionIndex) -> Vec2 {
        let offset = (index.size - 1) as f32 * 32.0 - index.index as f32 * 64.0;
        match index.position {
            BattleGuiPosition::Top => Vec2::new(144.0 - offset, 74.0),
            BattleGuiPosition::Bottom => Vec2::new(40.0 - offset, 113.0),
        }
    }

    pub fn faint(&mut self, position: (bool, usize)) {
        if let Some(sprite) = self.sprites.borrow_mut().get_mut(&position) {
            sprite.faint();
        }
    }

    pub fn flicker(&mut self, position: (bool, usize)) {
        if let Some(sprite) = self.sprites.borrow_mut().get_mut(&position) {
            sprite.flicker();
        }
    }

    pub fn draw_local<
        ID,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    >(
        &self,
        draw: &mut Draw,
        local: &GuiLocalPlayer<ID, P, M, I>,
        offset: Vec2,
        color: Color,
    ) {
        for (index, pokemon) in local.player.active_iter() {
            self.draw(
                &pokemon.pokemon.id,
                draw,
                offset,
                color,
                (true, index),
                local.player.active.len(),
            );
        }
    }

    pub fn draw_remotes<ID, P: Deref<Target = Pokemon> + Clone>(
        &self,
        draw: &mut Draw,
        remotes: &GuiRemotePlayers<ID, P>,
        offset: Vec2,
        color: Color,
    ) {
        if let Some((.., remote)) = remotes.players.get_index(remotes.current) {
            for (active, pokemon) in remote.active_iter() {
                if let Some(pokemon) = pokemon.as_ref() {
                    self.draw(
                        &pokemon.pokemon.id,
                        draw,
                        offset,
                        color,
                        (false, active),
                        remote.active.len(),
                    );
                }
            }
        }
    }

    pub fn draw(
        &self,
        pokemon: &PokemonId,
        draw: &mut Draw,
        offset: Vec2,
        color: Color,
        position: (bool, usize),
        active: usize,
    ) {
        if position.1 >= active {
            return;
        }
        let side = local(position.0);
        if let Some(texture) = self.dexengine.pokemon_textures.get(pokemon, side) {
            let pos = Self::position(BattleGuiPositionIndex {
                position: side.into(),
                index: position.1,
                size: active,
            });
            let pos = pos + offset;
            let mut sprites = self.sprites.borrow_mut();
            if !sprites.contains_key(&position) {
                sprites
                    .insert(position, PokemonSprite::new(&self.texture));
            }
            let sprite = sprites.get_mut(&position).unwrap();
            if Some(texture.id()) != sprite.current.as_ref().map(|t| t.id()) {
                sprite.current = Some(texture.clone());
            }
            if sprite.spawner.spawning() {
                sprite.spawner.draw(draw, pos, texture);
            } else if sprite.flicker.accumulator < Flicker::HALF {
                if sprite.faint.fainting {
                    if sprite.faint.remaining > 0.0 {
                        draw.texture(
                            texture,
                            pos.x,
                            pos.y - sprite.faint.remaining,
                            DrawParams {
                                source: Some(Rect {
                                    x: 0.0,
                                    y: 0.0,
                                    width: texture.width(),
                                    height: sprite.faint.remaining,
                                }),
                                color,
                                ..Default::default()
                            },
                        );
                    }
                } else {
                    draw.image(texture)
                        .position(
                            pos.x, //+ self.moves.pokemon_x(),
                            pos.y - texture.height(),
                        )
                        .color(color);
                }
            }
        }
    }
}
