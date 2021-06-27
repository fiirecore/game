use deps::tetra::{Context, graphics::Texture, math::Vec2};

use crate::graphics::{byte_texture, position};
use crate::{CRY_ID, play_sound, audio::sound::Sound};
use pokedex::pokemon::PokemonId;

pub struct Spawner {
    pub spawning: SpawnerState,
    pub texture: Texture,
    pub id: Option<PokemonId>,
    pub x: f32,
}

#[derive(PartialEq)]
pub enum SpawnerState {
    None,
    Start,
    Throwing,
    Spawning,
}

static mut POKEBALL: Option<Texture> = None;

impl Spawner {

    const LEN: f32 = 20.0;
    const ORIGIN: f32 = 0.0;
    const OFFSET: f32 = -5.0;
    const PARABOLA_ORIGIN: f32 = (Self::LEN / 3.0);

    pub fn new(ctx: &mut Context, id: Option<PokemonId>) -> Self {
        Self {
            spawning: SpawnerState::None,
            x: 0.0,
            id: id,
            texture: unsafe { POKEBALL.get_or_insert(byte_texture(ctx, include_bytes!("../../../../../../assets/battle/thrown_pokeball.png"))).clone() }
        }
    }

    fn f(x: f32) -> f32 {
        0.5 * (x - Self::PARABOLA_ORIGIN).powi(2) - 50.0
    }

    pub fn update(&mut self, ctx: &Context, delta: f32) {
        match self.spawning {
            SpawnerState::Start => {
                self.x = Self::ORIGIN;
                self.spawning = SpawnerState::Throwing;
                self.update(ctx, delta);
            }
            SpawnerState::Throwing => {
                self.x += delta * 20.0;
                if self.x > Self::LEN {
                    if let Some(id) = self.id {
                        play_sound(ctx, &Sound::variant(CRY_ID, Some(id)));
                    }
                    self.spawning = SpawnerState::Spawning;
                    self.x = 0.0;
                }
            }
            SpawnerState::Spawning => {
                self.x += delta * 1.5;
                if self.x > 1.0 {
                    self.spawning = SpawnerState::None;
                }
            }
            SpawnerState::None => (),
        }
    }

    pub fn draw(&self, ctx: &mut Context, origin: Vec2<f32>, texture: &Texture) {
        match self.spawning {
            SpawnerState::Throwing => self.texture.draw(
                ctx, 
                position(origin.x + self.x + Self::OFFSET, origin.y + Self::f(self.x)).origin(Vec2::new(6.0, 6.0)).rotation(self.x)
            ),
            SpawnerState::Spawning => {
                let h = texture.height() as f32;
                let scale = self.x * h;
                let mut y = origin.y - scale * 1.5;
                let max = origin.y - h;
                if y < max {
                    y = max;
                }
                texture.draw(ctx, position(origin.x, y)); // change color of texture when spawning here
            },
            SpawnerState::None | SpawnerState::Start => (),
        }
    }

    pub fn spawning(&self) -> bool {
        self.spawning != SpawnerState::None
    }

}