use firecore_pokedex::pokemon::PokemonId;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::Texture2D;
use ahash::AHashMap as HashMap;

use super::graphics::texture::debug_texture;

#[derive(Default)]
pub struct PokemonTextures {

    pub front: HashMap<PokemonId, Texture2D>,
    pub back: HashMap<PokemonId, Texture2D>,
    pub icon: HashMap<PokemonId, Texture2D>,

}

impl PokemonTextures {

    pub fn pokemon_texture(&self, id: &PokemonId, side: PokemonTexture) -> Texture2D {
        match side {
            PokemonTexture::Front => self.front.get(id).map(|tex| *tex),
            PokemonTexture::Back => self.back.get(id).map(|tex| *tex),
            PokemonTexture::Icon => self.icon.get(id).map(|tex| *tex),
        }.unwrap_or(debug_texture())
    }

}