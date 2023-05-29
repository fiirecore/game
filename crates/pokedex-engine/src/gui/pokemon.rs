use crate::pokedex::types::PokemonType;

use engine::graphics::Color;

const PLACEHOLDER: Color = Color::new(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0, 1.0);

const NORMAL: Color = Color::new(168.0 / 255.0, 168.0 / 255.0, 120.0 / 255.0, 1.0);
const WATER: Color = Color::new(104.0 / 255.0, 144.0 / 255.0, 240.0 / 255.0, 1.0);
const GRASS: Color = Color::new(120.0 / 255.0, 200.0 / 255.0, 80.0 / 255.0, 1.0);
const FIGHTING: Color = Color::new(232.0 / 255.0, 48.0 / 255.0, 0.0, 1.0);
const POISON_UPPER: Color = Color::new(248.0 / 255.0, 88.0 / 255.0, 136.0 / 255.0, 1.0);
const POISON_LOWER: Color = Color::new(160.0 / 255.0, 64.0 / 255.0, 160.0 / 255.0, 1.0);
const FLYING_UPPER: Color = Color::new(152.0 / 255.0, 216.0 / 255.0, 216.0 / 255.0, 1.0);

pub struct PokemonTypeDisplay {
    pub name: &'static str,
    pub upper: Color,
    pub lower: Color,
}

impl PokemonTypeDisplay {
    pub fn new(pokemon_type: PokemonType) -> Self {
        let (upper, lower) = Self::color(pokemon_type);
        Self {
            name: Self::name(pokemon_type),
            upper,
            lower,
        }
    }

    pub fn name(pokemon_type: PokemonType) -> &'static str {
        match pokemon_type {
            PokemonType::Unknown => "Unknown",
            PokemonType::Normal => "Normal",
            PokemonType::Fire => "Fire",
            PokemonType::Water => "Water",
            PokemonType::Electric => "Electric",
            PokemonType::Grass => "Grass",
            PokemonType::Ice => "Ice",
            PokemonType::Fighting => "Fighting",
            PokemonType::Poison => "Poison",
            PokemonType::Ground => "Ground",
            PokemonType::Flying => "Flying",
            PokemonType::Psychic => "Psychic",
            PokemonType::Bug => "Bug",
            PokemonType::Rock => "Rock",
            PokemonType::Ghost => "Ghost",
            PokemonType::Dragon => "Dragon",
            PokemonType::Dark => "Dark",
            PokemonType::Steel => "Steel",
            PokemonType::Fairy => "Fairy",
        }
    }

    pub fn color(pokemon_type: PokemonType) -> (Color, Color) {
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
}
