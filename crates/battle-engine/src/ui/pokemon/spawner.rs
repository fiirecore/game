use pokedex::{
    engine::{
        sound::play_sound,
        graphics::{DrawParams, Texture},
        math::Vec2,
        Context, EngineContext,
    },
    pokemon::PokemonId,
    CRY_ID,
};

use crate::context::BattleGuiData;

#[derive(Clone)]
pub struct Spawner {
    pub spawning: SpawnerState,
    pub texture: Texture,
    pub id: Option<PokemonId>,
    pub x: f32,
}

#[derive(PartialEq, Clone)]
pub enum SpawnerState {
    None,
    Start,
    Throwing,
    Spawning,
}

impl Spawner {
    const LEN: f32 = 20.0;
    const ORIGIN: f32 = 0.0;
    const OFFSET: f32 = -5.0;
    const PARABOLA_ORIGIN: f32 = (Self::LEN / 3.0);

    pub fn new(ctx: &BattleGuiData, id: Option<PokemonId>) -> Self {
        Self {
            spawning: SpawnerState::None,
            x: 0.0,
            id,
            texture: ctx.pokeball.clone(),
        }
    }

    fn f(x: f32) -> f32 {
        0.5 * (x - Self::PARABOLA_ORIGIN).powi(2) - 50.0
    }

    pub fn update(&mut self, ctx: &mut Context, eng: &mut EngineContext, delta: f32) {
        match self.spawning {
            SpawnerState::Start => {
                self.x = Self::ORIGIN;
                self.spawning = SpawnerState::Throwing;
                self.update(ctx, eng, delta);
            }
            SpawnerState::Throwing => {
                self.x += delta * 20.0;
                if self.x > Self::LEN {
                    if let Some(id) = self.id {
                        play_sound(ctx, eng, &CRY_ID, Some(id));
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

    pub fn draw(&self, ctx: &mut Context, origin: Vec2, texture: &Texture) {
        match self.spawning {
            SpawnerState::Throwing => {
                self.texture.draw(
                    ctx,
                    origin.x + self.x + Self::OFFSET,
                    origin.y + Self::f(self.x),
                    DrawParams {
                        rotation: self.x,
                        ..Default::default()
                    },
                );
            }
            SpawnerState::Spawning => {
                let h = texture.height() as f32;
                let scale = self.x * h;
                let mut y = origin.y - scale * 1.5;
                let max = origin.y - h;
                if y < max {
                    y = max;
                }
                texture.draw(ctx, origin.x, y, DrawParams::default()); // change color of texture when spawning here
            }
            SpawnerState::None | SpawnerState::Start => (),
        }
    }

    pub fn spawning(&self) -> bool {
        self.spawning != SpawnerState::None
    }
}
