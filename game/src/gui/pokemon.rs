use crate::textures::pokemon_texture;
use pokedex::pokemon::texture::PokemonTexture::Icon;
use firecore_pokedex::pokemon::instance::PokemonInstance;
use firecore_pokedex::pokemon::saved::SavedPokemon;
use firecore_pokedex::pokemon::types::PokemonType;
use macroquad::prelude::Color;
use macroquad::prelude::Texture2D;

// pub enum PokemonContainer {
//     Saved(&'static SavedPokemon),
//     Instance(& PokemonInstance),
// }

#[derive(Clone)]
pub struct PokemonDisplay {

    pub instance: PokemonInstance,
    pub icon: Texture2D,
    pub name: String,
    pub level: String,
    pub health: (String, f32),

}

pub struct PokemonSummaryDisplay {

    pub types: Vec<PokemonTypeDisplay>, // To - do: non-vec
    pub item: String,

}

pub struct PokemonTypeDisplay {
    pub name: String,
    pub upper: Color,
    pub lower: Color,
}

impl PokemonDisplay {
    pub const ICON_TICK: f32 = 0.15;

    pub fn new_saved(saved: &SavedPokemon) -> Option<Self> {
        PokemonInstance::new(saved).map(|instance| {
            Self::new(instance)
        })
    }

    pub fn new(instance: PokemonInstance) -> Self {
        Self {
            name: instance.name(),
            level: format!("Lv{}", instance.data.level),
            health: (format!("{}/{}", instance.current_hp, instance.base.hp), super::health::HealthBar::get_hp_width(instance.current_hp, instance.base.hp)),
            icon: pokemon_texture(&instance.pokemon.data.id, Icon),
            instance,
        }
    }

}

impl PokemonTypeDisplay {

    pub fn new(pokemon_type: PokemonType) -> Self {
        let colors = crate::graphics::pokemon_type_color(pokemon_type);
        Self {
            name: format!("{:?}", pokemon_type),
            upper: colors.0,
            lower: colors.1,
        }
    }

}