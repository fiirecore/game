use firecore_util::Entity;
use firecore_pokedex::pokemon::battle::BattlePokemon;

pub mod player;
pub mod opponent;

const OFFSET: f32 = 24.0 * 5.0;

const HEALTH_X_OFFSET: f32 = 48.0;
const HEALTH_Y_OFFSET: f32 = 17.0;

pub trait PokemonGui: Entity { // To-do: sort out trait or have it extend something

	fn reset(&mut self);

	fn update(&mut self, delta: f32);

	fn render(&self);

	fn update_gui(&mut self, pokemon: &BattlePokemon, new_pokemon: bool);

	fn update_hp(&mut self, new_pokemon: bool, current_hp: u16, max_hp: u16);

	fn update_position(&mut self, x: f32, y: f32);

	fn offset_position(&mut self, x: f32, y: f32);

}