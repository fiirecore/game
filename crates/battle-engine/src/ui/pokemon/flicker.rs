use pokengine::engine::{graphics::{Draw, Texture, DrawImages, Color}, math::Vec2, App};

#[derive(Clone)]
pub struct Flicker {
    pub remaining: u8,
    pub accumulator: f32,
}

impl Flicker {
    pub const HALF: f32 = Self::LENGTH / 2.0;
    pub const LENGTH: f32 = 0.20;
    pub const TIMES: u8 = 4;

    pub fn new() -> Self {
        Self {
            remaining: Flicker::TIMES,
            accumulator: Default::default(),
        }
    }

    pub fn update(&mut self, app: &mut App) -> bool {
        if self.remaining > 0 {
            self.accumulator += app.timer.delta_f32();
            if self.accumulator > Self::LENGTH {
                self.accumulator -= Self::LENGTH;
                self.remaining -= 1;
            }
            if self.remaining == 0 {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, draw: &mut Draw, texture: &Texture, pos: Vec2, color: Color) {
        if self.accumulator < Flicker::HALF {
            draw.image(texture)
                .position(
                    pos.x, //+ self.moves.pokemon_x(),
                    pos.y - texture.height(),
                )
                .color(color);
        }
    }

}
