
use deps::hash::HashMap;

use pokedex::{
    pokemon::PokemonId,
    item::ItemId,
};

use macroquad::prelude::Texture2D;

use crate::serialize::SerializedPokemon;

pub static mut POKEMON_TEXTURES: Option<PokemonTextures> = None;

pub static mut ITEM_TEXTURES: Option<ItemTextures> = None;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PokemonTexture {
    Front,
    Back,
    Icon,
}

// impl PokemonTexture {
//     pub const fn path(self) -> &'static str {
//         match self {
//             PokemonTexture::Front => "front",
//             PokemonTexture::Back => "back",
//             PokemonTexture::Icon => "icon",
//         }
//     }
// }

pub fn pokemon_texture(id: &PokemonId, side: PokemonTexture) -> Texture2D {
	unsafe{POKEMON_TEXTURES.as_ref()}.expect("Could not get pokemon textures!").get(&id, side)
}

pub fn item_texture(id: &ItemId) -> Option<Texture2D> {
    unsafe{ITEM_TEXTURES.as_ref()}.expect("Could not get item textures!").get(id).map(|texture| *texture)
}


pub struct PokemonTextures {

    pub front: HashMap<PokemonId, Texture2D>,
    pub back: HashMap<PokemonId, Texture2D>,
    pub icon: HashMap<PokemonId, Texture2D>,

}

impl PokemonTextures {

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            front: HashMap::with_capacity(capacity),
            back: HashMap::with_capacity(capacity),
            icon: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, pokemon: &SerializedPokemon) {
        self.front.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.front_png));
		self.back.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.back_png));
		self.icon.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.icon_png));
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> Texture2D {
        match side {
            PokemonTexture::Front => self.front.get(id),
            PokemonTexture::Back => self.back.get(id),
            PokemonTexture::Icon => self.icon.get(id),
        }.map(|texture| *texture).unwrap_or_else(|| panic!("Could not get texture for pokemon with ID {}", id))
    }

}

pub type ItemTextures = HashMap<ItemId, Texture2D>;

#[inline]
fn byte_texture(bytes: &[u8]) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, None);
    texture.set_filter(macroquad::prelude::FilterMode::Nearest);
    texture
}