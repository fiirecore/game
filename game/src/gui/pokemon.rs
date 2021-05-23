use std::borrow::Cow;

use pokedex::{
    texture::{
        pokemon_texture,
        PokemonTexture::Icon,
    },
    pokemon::{
        types::PokemonType,
        instance::PokemonInstance,
    },
};

use macroquad::prelude::{Color, color_u8, Texture2D};

#[derive(Clone)]
pub struct PokemonDisplay {

    pub instance: Cow<'static, PokemonInstance>,
    pub icon: Texture2D,
    pub name: String,
    pub level: String,
    pub health: (String, f32),

}

pub struct PokemonSummaryDisplay {

    pub types: Vec<PokemonTypeDisplay>, // To - do: non-vec
    pub item: String,

}



const PLACEHOLDER: Color = color_u8!(30, 30, 30, 255);

const NORMAL: Color = color_u8!(168, 168, 120, 255);
const WATER: Color = color_u8!(104, 144, 240, 255);
const GRASS: Color = color_u8!(120, 200, 80, 255);
const FIGHTING: Color = color_u8!(232, 48, 0, 255);
const POISON_UPPER: Color = color_u8!(248, 88, 136, 255);
const POISON_LOWER: Color = color_u8!(160, 64, 160, 255);
const FLYING_UPPER: Color = color_u8!(152, 216, 216, 255);

pub fn pokemon_type_color(pokemon_type: PokemonType) -> (Color, Color) {
	match pokemon_type {
	    PokemonType::Normal => (NORMAL, NORMAL),
	    // PokemonType::Fire => {}
	    PokemonType::Water => (WATER, WATER),
	    // PokemonType::Electric => {}
	    PokemonType::Grass => (GRASS, GRASS),
	    // PokemonType::Ice => {}
	    PokemonType::Fighting => (FIGHTING, FIGHTING),
	    PokemonType::Poison => (POISON_UPPER, POISON_LOWER),
	    // PokemonType::Ground => {}
	    PokemonType::Flying => (FLYING_UPPER, NORMAL),
	    // PokemonType::Psychic => {}
	    // PokemonType::Bug => {}
	    // PokemonType::Rock => {}
	    // PokemonType::Ghost => {}
	    // PokemonType::Dragon => {}
	    // PokemonType::Dark => {}
	    // PokemonType::Steel => {}
	    // PokemonType::Fairy => {}
		_ => (PLACEHOLDER, PLACEHOLDER),
	}
}

pub struct PokemonTypeDisplay {
    pub name: String,
    pub upper: Color,
    pub lower: Color,
}

impl PokemonDisplay {
    pub const ICON_TICK: f32 = 0.15;

    pub fn new(instance: Cow<'static, PokemonInstance>) -> Self {
        Self {
            name: instance.name().to_string(),
            level: format!("Lv{}", instance.level),
            health: (format!("{}/{}", instance.current_hp, instance.base.hp), super::health::HealthBar::width(instance.current_hp, instance.base.hp)),
            icon: pokemon_texture(instance.pokemon.id(), Icon),
            instance,
        }
    }

}

impl PokemonTypeDisplay {

    pub fn new(pokemon_type: PokemonType) -> Self {
        let (upper, lower) = pokemon_type_color(pokemon_type);
        Self {
            name: format!("{:?}", pokemon_type),
            upper,
            lower,
        }
    }

}