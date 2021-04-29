use deps::{
    hash::HashMap,
    tinystr::TinyStr16,
};
use firecore_pokedex::pokemon::{
    PokemonId,
    texture::PokemonTexture,
};
use macroquad::prelude::Texture2D;

pub type TrainerSprites = HashMap<TinyStr16, Texture2D>;
pub type ItemTextures = HashMap<TinyStr16, Texture2D>;

pub static mut TRAINER_SPRITES: Option<TrainerSprites> = None;

pub fn trainer_texture(npc_type: &TinyStr16) -> Texture2D {
    unsafe{TRAINER_SPRITES.as_ref()}.expect("Could not get trainer sprites!").get(npc_type).map(|texture| *texture).unwrap_or(crate::graphics::debug_texture())
}

pub static mut ITEM_TEXTURES: Option<TrainerSprites> = None;

pub fn item_texture(id: &TinyStr16) -> Option<Texture2D> {
    unsafe{ITEM_TEXTURES.as_ref()}.expect("Could not get item textures!").get(id).map(|texture| *texture)
}

pub static mut POKEMON_TEXTURES: Option<PokemonTextures> = None;

pub fn pokemon_texture(id: &PokemonId, side: PokemonTexture) -> Texture2D {
	unsafe{POKEMON_TEXTURES.as_ref()}.expect("Could not get pokemon textures!").get(&id, side)
}

#[derive(Default)]
pub struct PokemonTextures {

    pub front: HashMap<PokemonId, Texture2D>,
    pub back: HashMap<PokemonId, Texture2D>,
    pub icon: HashMap<PokemonId, Texture2D>,

}

impl PokemonTextures {

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> Texture2D {
        match side {
            PokemonTexture::Front => self.front.get(id),
            PokemonTexture::Back => self.back.get(id),
            PokemonTexture::Icon => self.icon.get(id),
        }.map(|texture| *texture).unwrap_or(crate::graphics::debug_texture())
    }

}