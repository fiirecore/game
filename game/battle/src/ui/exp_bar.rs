use game::{
    pokedex::pokemon::{
        Experience,
        instance::PokemonInstance,
    },
    macroquad::prelude::{Vec2, Color, color_u8, draw_rectangle},
    gui::ProgressBar,
};

pub struct ExperienceBar {
    bar: ProgressBar,
}

impl ExperienceBar {

    pub const WIDTH: f32 = 64.0;
    pub const COLOR: Color = color_u8!(64, 200, 248, 255); // To - do: correct texture for exp bar

    pub fn new() -> Self {
        Self {
            bar: ProgressBar::new(Self::WIDTH),
        }
    }

    pub const fn with_size(width: f32) -> Self {
        Self {
            bar: ProgressBar::new(width),
        }
    }

    pub fn width(current: Experience, max: Experience) -> f32 {
        current as f32 * Self::WIDTH / max as f32
    }

    pub fn update_exp(&mut self, pokemon: &PokemonInstance, reset: bool) {
        self.bar.resize(Self::width(pokemon.data.experience, pokemon.pokemon.training.growth_rate.max_exp(pokemon.data.level)), reset);
	}

    pub fn update(&mut self, delta: f32) {
        self.bar.update(delta);
    }

    pub fn render(&self, origin: Vec2) {
        draw_rectangle(origin.x, origin.y, self.bar.width().abs() % Self::WIDTH, 2.0, Self::COLOR);
    }

    pub fn moving(&self) -> bool {
        self.bar.moving()
    }

}