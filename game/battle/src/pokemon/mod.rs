use game::{
    pokedex::pokemon::instance::PokemonInstance,
    macroquad::prelude::Texture2D
};

mod party;
mod renderer;
mod move_status;

pub use party::BattleParty;
pub use renderer::ActivePokemonRenderer;
pub use move_status::BattleMoveStatus;


pub struct BattlePokemon {

    pub pokemon: PokemonInstance,
    pub texture: Texture2D,

}