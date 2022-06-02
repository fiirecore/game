use std::ops::Deref;

use battle::pokemon::PokemonView;
use pokengine::{
    engine::{
        graphics::{Color, Draw, DrawImages, Texture},
        math::Vec2,
        utils::HashMap,
        App, Plugins,
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

use self::{faint::Faint, flicker::Flicker, spawner::Spawner};

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
    pokeball: Texture,
    actions: HashMap<(bool, usize), Vec<PokemonAction>>,
    current: Option<(bool, usize)>,
}

enum PokemonAction {
    Spawn(Spawner),
    Flicker(Flicker),
    Faint(Faint),
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
            pokeball: ctx.pokeball.clone(),
            actions: Default::default(),
            current: None,
        }
    }

    fn position(index: BattleGuiPositionIndex) -> Vec2 {
        let offset = (index.size - 1) as f32 * 32.0 - index.index as f32 * 64.0;
        match index.position {
            BattleGuiPosition::Top => Vec2::new(144.0 - offset, 74.0),
            BattleGuiPosition::Bottom => Vec2::new(40.0 - offset, 113.0),
        }
    }

    fn insert(&mut self, position: (bool, usize), item: PokemonAction) {
        match self.actions.get_mut(&position) {
            Some(actions) => actions.push(item),
            None => {
                self.actions.insert(position, vec![item]);
            }
        }
    }

    pub fn spawn_pokemon(&mut self, position: (bool, usize), pokemon: &PokemonId) {
        self.insert(
            position,
            PokemonAction::Spawn(Spawner::new(self.pokeball.clone(), Some(*pokemon))),
        );
    }

    pub fn faint(&mut self, position: (bool, usize), pokemon: Option<&PokemonId>) {
        self.insert(
            position,
            PokemonAction::Faint(Faint::new(pokemon.and_then(|pokemon| {
                self.dexengine
                    .pokemon_textures
                    .get(pokemon, local(position.0))
            }))),
        );
    }

    pub fn flicker(&mut self, position: (bool, usize)) {
        self.insert(position, PokemonAction::Flicker(Flicker::new()));
    }

    pub fn update(&mut self, app: &mut App, plugins: &mut Plugins) {
        match &self.current {
            Some(current) => match self.actions.get_mut(current) {
                Some(actions) => match actions.first_mut() {
                    Some(action) => {
                        if match action {
                            PokemonAction::Spawn(spawner) => spawner.update(app, plugins),
                            PokemonAction::Flicker(flicker) => flicker.update(app),
                            PokemonAction::Faint(faint) => faint.update(app),
                        } {
                            actions.remove(0);
                        }
                    }
                    None => self.current = None,
                },
                None => {
                    log::warn!(
                        "Could not get pokemon renderer for {} pokemon #{}",
                        match current.0 {
                            true => "local",
                            false => "remote",
                        },
                        current.1
                    );
                    self.current = None;
                }
            },
            None => {
                self.resume();
            }
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
                (true, index),
                local.player.active.len(),
                &pokemon.pokemon.id,
                pokemon.fainted(),
                draw,
                offset,
                color,
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
                        (false, active),
                        remote.active.len(),
                        &pokemon.pokemon.id,
                        pokemon.fainted(),
                        draw,
                        offset,
                        color,
                    );
                }
            }
        }
    }

    pub fn draw(
        &self,
        position: (bool, usize),
        active: usize,
        pokemon: &PokemonId,
        fainted: bool,
        draw: &mut Draw,
        offset: Vec2,
        color: Color,
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
            let current = self
                .current
                .as_ref()
                .filter(|current| *current == &position)
                .and_then(|current| self.actions.get(current));
            match current.and_then(|v| v.first()) {
                Some(action) => match action {
                    PokemonAction::Spawn(spawner) => spawner.draw(draw, texture, pos, color),
                    PokemonAction::Flicker(flicker) => flicker.draw(draw, texture, pos, color),
                    PokemonAction::Faint(faint) => faint.draw(draw, texture, pos, color),
                },
                None => {
                    if !fainted || current.map(|v| !v.is_empty()).unwrap_or_default() {
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

    pub fn resume(&mut self) {
        self.current = self
            .actions
            .iter()
            .find(|(.., v)| !v.is_empty())
            .map(|(id, ..)| *id);
    }

    pub fn should_resume(&self) -> bool {
        self.current.is_none()
    }

    pub fn finished(&self) -> bool {
        self.actions.values().all(|v| v.is_empty())
    }

    pub fn reset(&mut self) {
        self.actions.clear();
    }
}
