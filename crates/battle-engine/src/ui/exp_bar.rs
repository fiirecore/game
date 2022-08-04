use core::ops::Deref;

use pokengine::{
    engine::{
        graphics::Color,
        gui::ProgressBar,
        math::Vec2,
        notan::draw::{Draw, DrawShapes},
    },
    pokedex::{
        item::Item,
        moves::Move,
        pokemon::{owned::OwnedPokemon, Experience, Level, Pokemon},
    },
};

#[derive(Clone, Copy)]
pub struct ExperienceBar {
    bar: ProgressBar,
    remaining: (Level, f32),
}

impl ExperienceBar {
    pub const WIDTH: f32 = 64.0;
    pub const COLOR: Color = Color::new(64.0 / 255.0, 200.0 / 255.0, 248.0 / 255.0, 1.0); // To - do: correct texture for exp bar
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

    pub fn update_exp(&mut self, previous: Level, pokemon: &OwnedPokemon, reset: bool) {
        let width = Self::width(
            pokemon.experience,
            pokemon.pokemon.training.growth.max_exp(pokemon.level),
        );
        self.remaining = (pokemon.level.saturating_sub(previous), width);
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
            self.bar
                .resize_with_gap(self.remaining.1, -self.remaining.1);
            self.remaining = Default::default();
            true
        } else {
            false
        }
    }

    pub fn draw(&self, draw: &mut Draw, origin: Vec2) {
        draw.rect(
            (origin.x, origin.y),
            (self.bar.width().abs() % Self::WIDTH, 2.0),
        )
        .color(Self::COLOR);
    }

    pub fn moving(&self) -> bool {
        self.bar.moving() || self.remaining.0 != 0
    }
}

impl Default for ExperienceBar {
    fn default() -> Self {
        Self::new()
    }
}
