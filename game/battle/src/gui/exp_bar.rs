use game::{
    pokedex::pokemon::instance::PokemonInstance,
    macroquad::prelude::{Vec2, Color, color_u8, draw_rectangle},
};

pub struct ExperienceBar {

    pub panel: Vec2,
    pos: Vec2,

    exp_width: f32,
	exp_gap: f32,

}

impl ExperienceBar {

    const EXP_COLOR: Color = color_u8!(64, 200, 248, 255);
    const WIDTH: f32 = 64.0;

    pub fn new(pos: Vec2, panel: Vec2,) -> Self {
        Self {
            panel,
            pos,
            exp_width: Self::WIDTH,
            exp_gap: 0.0,
        }
    }

    pub fn update_exp(&mut self, pokemon: &PokemonInstance, new_pokemon: bool) {
		let new = pokemon.data.experience as f32 * Self::WIDTH / pokemon.pokemon.training.growth_rate.level_exp(pokemon.data.level) as f32;

		self.exp_gap = if new_pokemon {
			0.0
		} else {
			new - self.exp_width
		};

		self.exp_width = new;
	}

    pub fn update(&mut self, delta: f32) {
        if self.exp_gap > 0.0 {
            self.exp_gap -= 60.0 * delta;
            if self.exp_gap < 0.0 {
                self.exp_gap = 0.0;
            }
        }
    }

    pub fn render(&self, y_offset: f32) {
        draw_rectangle(self.panel.x + self.pos.x, self.panel.y + self.pos.y + y_offset, (self.exp_width - self.exp_gap).abs() % Self::WIDTH, 2.0, Self::EXP_COLOR);
    }

}