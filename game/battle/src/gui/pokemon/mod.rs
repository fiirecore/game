use game::{
	util::Entity,
	pokedex::pokemon::instance::PokemonInstance,
};

pub mod player;
pub mod opponent;

pub mod party;

const OFFSET: f32 = 24.0 * 5.0;

const HEALTH_X_OFFSET: f32 = 48.0;
const HEALTH_Y_OFFSET: f32 = 17.0;

pub trait PokemonGui: Entity {

	fn reset(&mut self);

	fn update(&mut self, delta: f32);

	fn render(&self);

	fn update_gui(&mut self, pokemon: &PokemonInstance, new_pokemon: bool);

	fn update_position(&mut self, x: f32, y: f32);

	fn offset_position(&mut self, x: f32, y: f32);

}