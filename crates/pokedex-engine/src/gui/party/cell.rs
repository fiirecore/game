use core::ops::Deref;
use engine::graphics::Texture;

use crate::pokedex::{
    pokemon::{Nature, stat::StatType, Pokemon},
    Dex,
};
use tinystr::TinyStr16;

use crate::{
    get::GetPokemonData,
    gui::{health::HealthBar, SizedStr},
    texture::PokemonTexture,
    PokedexClientData,
};

use super::PartyError;

pub struct PartyCell {
    pub icon: Texture,
    pub name: TinyStr16,
    pub level: SizedStr<4>,
    pub health: CellHealth,
}

impl PartyCell {
    pub const ICON_TICK: f32 = 0.15;

    pub fn new<'d, P: Deref<Target = Pokemon>, I: GetPokemonData>(
        ctx: &PokedexClientData,
        pokedex: &'d dyn Dex<'d, Pokemon, P>,
        instance: &I,
    ) -> Result<Self, PartyError> {
        let pokemon = pokedex
            .try_get(instance.pokemon_id())
            .ok_or(PartyError::MissingPokemon)?;
        Ok(Self {
            icon: ctx
                .pokemon_textures
                .get(&pokemon.id, PokemonTexture::Icon)
                .cloned()
                .ok_or(PartyError::MissingTexture)?,
            name: instance
                .name()
                .unwrap_or_else(|| pokemon.name.as_str())
                .parse()
                .map_err(|err| PartyError::TinyStr("PartyCell.name", err))?,
            level: SizedStr::new(instance.level() as u16)?,
            health: CellHealth::new(&pokemon, instance)?,
        })
    }
}

#[derive(Clone)]
pub struct CellHealth {
    pub current: SizedStr<4>,
    pub maximum: SizedStr<4>,
    pub percent: f32,
}

impl CellHealth {
    pub fn new<I: GetPokemonData>(
        pokemon: &impl Deref<Target = Pokemon>,
        instance: &I,
    ) -> Result<Self, PartyError> {
        let max = pokemon.stat(
            instance.ivs(),
            instance.evs(),
            instance.level(),
            instance.nature().unwrap_or(Nature::Hardy),
            StatType::Health,
        );
        let hp = instance.hp().unwrap_or(max);
        Ok(Self {
            current: SizedStr::new(hp)?,
            maximum: SizedStr::new(max)?,
            percent: (hp as f32 / max as f32) * HealthBar::WIDTH,
        })
        // instance
    }
}
