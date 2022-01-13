use enum_map::{enum_map, EnumMap};

use crate::pokedex::{item::ItemId, pokemon::PokemonId};

use engine::{
    error::ImageError,
    graphics::Texture,
    utils::HashMap,
    Context,
};

pub type TrainerGroupTextures = HashMap<crate::TrainerGroupId, Texture>;
pub type ItemTextures = HashMap<ItemId, Texture>;

pub use firecore_pokedex_engine_builder::pokemon::PokemonTexture;

pub struct PokemonTextures(HashMap<PokemonId, EnumMap<PokemonTexture, Texture>>);

impl PokemonTextures {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    pub fn insert(
        &mut self,
        ctx: &mut Context,
        id: PokemonId,
        textures: EnumMap<PokemonTexture, Vec<u8>>,
    ) -> Result<(), ImageError> {
        self.0.insert(
            id,
            enum_map! {
                PokemonTexture::Front => Texture::new(ctx, &textures[PokemonTexture::Front])?,
                PokemonTexture::Back => Texture::new(ctx, &textures[PokemonTexture::Back])?,
                PokemonTexture::Icon => Texture::new(ctx, &textures[PokemonTexture::Icon])?,
            },
        );
        Ok(())
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> Option<&Texture> {
        self.0.get(id).map(|m| &m[side])
    }
}
