use std::borrow::Cow;

use pokedex::{
    types::PokemonType,
    texture::{
        pokemon_texture,
        PokemonTexture::Icon,
    },
    pokemon::{
        instance::PokemonInstance,
    },
};

use crate::tetra::graphics::{Color, Texture};

#[derive(Clone)]
pub struct PokemonDisplay {

    pub instance: Cow<'static, PokemonInstance>,
    pub icon: Texture,
    pub name: String,
    pub level: String,
    pub health: (String, f32),

}

pub struct PokemonSummaryDisplay {

    pub types: Vec<PokemonTypeDisplay>, // To - do: non-vec
    pub item: String,

}



const PLACEHOLDER: Color = Color::rgb(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0);

const NORMAL: Color = Color::rgb(168.0 / 255.0, 168.0 / 255.0, 120.0 / 255.0);
const WATER: Color = Color::rgb(104.0 / 255.0, 144.0 / 255.0, 240.0 / 255.0);
const GRASS: Color = Color::rgb(120.0 / 255.0, 200.0 / 255.0, 80.0 / 255.0);
const FIGHTING: Color = Color::rgb(232.0 / 255.0, 48.0 / 255.0, 0.0);
const POISON_UPPER: Color = Color::rgb(248.0 / 255.0, 88.0 / 255.0, 136.0 / 255.0);
const POISON_LOWER: Color = Color::rgb(160.0 / 255.0, 64.0 / 255.0, 160.0 / 255.0);
const FLYING_UPPER: Color = Color::rgb(152.0 / 255.0, 216.0 / 255.0, 216.0 / 255.0);

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
            health: (format!("{}/{}", instance.hp(), instance.max_hp()), super::health::HealthBar::width(instance.hp(), instance.max_hp())),
            icon: pokemon_texture(instance.pokemon.id(), Icon).clone(),
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