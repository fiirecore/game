use game::{
	util::{Entity, Reset},
	pokedex::pokemon::instance::PokemonInstance,
};

pub mod player;
pub mod opponent;

pub mod party;

const OFFSET: f32 = 24.0 * 5.0;

const HEALTH_Y_OFFSET: f32 = 17.0;

pub trait PokemonGui: Entity {

	fn update(&mut self, delta: f32);

	fn render(&self);

	fn update_gui(&mut self, pokemon: &PokemonInstance, reset: bool);

	fn offset(&mut self, delta: f32) -> bool;

}

pub struct PokemonGuiOffset {
	x: f32,
}

impl PokemonGuiOffset {

	pub const fn new() -> Self {
		Self {
			x: OFFSET,
		}
	}

	pub fn update(&mut self, delta: f32) -> bool {
		self.x -= delta * 240.0;
		if self.x < 0.0 {
			self.x = 0.0;
			true
		} else {
			false
		}
	}

}

impl Reset for PokemonGuiOffset {
    fn reset(&mut self) {
        self.x = OFFSET;
    }
}