use game::{
    pokedex::pokemon::instance::PokemonInstance,
    macroquad::prelude::{Vec2, Color, color_u8, draw_rectangle},
};

pub struct ExperienceBar {

    pub origin: Vec2,

    width: f32,
	gap: f32,

}

impl ExperienceBar {

    pub const WIDTH: f32 = 64.0;

    pub const COLOR: Color = color_u8!(64, 200, 248, 255); // To - do: correct texture for exp bar

    pub fn new(origin: Vec2) -> Self {
        Self {
            origin,
            width: Self::WIDTH,
            gap: 0.0,
        }
    }

    pub fn update_exp(&mut self, pokemon: &PokemonInstance, reset: bool) {
		let new = pokemon.data.experience as f32 * Self::WIDTH / pokemon.pokemon.training.growth_rate.max_exp(pokemon.data.level) as f32;

		self.gap = if reset {
			0.0
		} else {
			new - self.width
		};

		self.width = new;
	}

    pub fn update(&mut self, delta: f32) {
        if self.gap > 0.0 {
            self.gap -= 60.0 * delta;
            if self.gap < 0.0 {
                self.gap = 0.0;
            }
        }
    }

    pub fn render(&self) {
        draw_rectangle(self.origin.x, self.origin.y, (self.width - self.gap).abs() % Self::WIDTH, 2.0, Self::COLOR);
    }

    pub fn render_position(&self, offset: Vec2) {
        draw_rectangle(self.origin.x + offset.x, self.origin.y + offset.y, (self.width - self.gap).abs() % Self::WIDTH, 2.0, Self::COLOR);
    }

}