use game::{
    pokedex::pokemon::{
        Level,
        Experience,
        instance::PokemonInstance,
    },
    macroquad::prelude::{Vec2, Color, color_u8, draw_rectangle},
    gui::ProgressBar,
};

pub struct ExperienceBar {
    bar: ProgressBar,
    remaining: (Level, f32),
}

impl ExperienceBar {

    pub const WIDTH: f32 = 64.0;
    pub const COLOR: Color = color_u8!(64, 200, 248, 255); // To - do: correct texture for exp bar
    const DEF_REM: (Level, f32) = (0, 0.0);

    pub const fn new() -> Self {
        Self {
            bar: ProgressBar::new(Self::WIDTH),
            remaining: Self::DEF_REM,
        }
    }

    pub const fn with_size(width: f32) -> Self {
        Self {
            bar: ProgressBar::new(width),
            remaining: Self::DEF_REM,
        }
    }

    pub fn width(current: Experience, max: Experience) -> f32 {
        (current as f32 * Self::WIDTH / max as f32).clamp(0.0, Self::WIDTH)
    }

    pub fn update_exp(&mut self, previous: Level, pokemon: &PokemonInstance, reset: bool) {
        let width = Self::width(pokemon.data.experience, pokemon.pokemon.value().training.growth_rate.max_exp(pokemon.data.level));
        self.remaining = (pokemon.data.level.saturating_sub(previous), width);
        if self.remaining.0 != 0 {
            self.bar.resize(Self::WIDTH, false);
        } else {
            self.bar.resize(width, reset);
        }
	}

    pub fn update(&mut self, delta: f32) -> bool {
        if self.bar.moving() {
            self.bar.update(delta);
            false
        } else if self.remaining.0 > 1 {
            self.remaining.0 -= 1;
            self.bar.resize_with_gap(Self::WIDTH, -Self::WIDTH);
            true
        } else if self.remaining.0 == 1 {
            self.bar.resize_with_gap(self.remaining.1, -self.remaining.1);
            self.remaining = Default::default();
            true
        } else {
            false
        }
    }

    pub fn render(&self, origin: Vec2) {
        draw_rectangle(origin.x, origin.y, self.bar.width().abs() % Self::WIDTH, 2.0, Self::COLOR);
    }

    pub fn moving(&self) -> bool {
        self.bar.moving() || self.remaining.0 != 0
    }

}