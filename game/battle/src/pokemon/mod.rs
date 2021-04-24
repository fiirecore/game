use game::{
    pokedex::pokemon::instance::PokemonInstance,
    macroquad::prelude::Texture2D
};

mod party;
mod renderer;
mod move_status;

pub use party::*;
pub use renderer::*;
pub use move_status::*;


pub struct BattlePokemon {

    pub pokemon: PokemonInstance,
    pub texture: Texture2D,

}