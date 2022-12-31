use pokengine::{
    engine::{
        graphics::{Color, Texture},
        math::Vec2,
        notan::{
            draw::{Draw, DrawImages, DrawTransform},
            prelude::{App, Plugins},
        },
        sound::{play_sound, SoundVariant},
    },
    pokedex::pokemon::PokemonId,
    CRY_ID,
};

#[derive(Clone)]
pub struct Spawner {
    spawning: SpawnerState,
    pokeball: Texture,
    id: Option<PokemonId>,
    x: f32,
}

#[derive(PartialEq, Clone)]
pub enum SpawnerState {
    Start,
    Throwing,
    Spawning,
    End,
}

impl Spawner {
    const LEN: f32 = 20.0;
    const ORIGIN: f32 = 0.0;
    const OFFSET: f32 = -5.0;
    const PARABOLA_ORIGIN: f32 = (Self::LEN / 3.0);

    pub fn new(pokeball: Texture, id: Option<PokemonId>) -> Self {
        Self {
            spawning: SpawnerState::Start,
            x: 0.0,
            id,
            pokeball,
        }
    }

    fn f(x: f32) -> f32 {
        0.5 * (x - Self::PARABOLA_ORIGIN).powi(2) - 50.0
    }

    pub fn spawn(&mut self) {
        self.spawning = SpawnerState::Start;
        self.x = 0.0;
    }

    pub fn update(&mut self, app: &mut App, plugins: &mut Plugins) -> bool {
        let delta = app.timer.delta_f32();
        match self.spawning {
            SpawnerState::Start => {
                self.x = Self::ORIGIN;
                self.spawning = SpawnerState::Throwing;
                self.update(app, plugins);
            }
            SpawnerState::Throwing => {
                self.x += delta * 20.0;
                if self.x > Self::LEN {
                    if let Some(pokemon) = self.id {
                        play_sound(app, plugins, CRY_ID, SoundVariant::Num(pokemon as _));
                    }
                    self.spawning = SpawnerState::Spawning;
                    self.x = 0.0;
                }
            }
            SpawnerState::Spawning => {
                self.x += delta * 1.5;
                if self.x > 1.0 {
                    self.spawning = SpawnerState::End;
                }
            }
            SpawnerState::End => (),
        }
        matches!(self.spawning, SpawnerState::End)
    }

    pub fn draw(&self, draw: &mut Draw, texture: &Texture, origin: Vec2, color: Color) {
        match self.spawning {
            SpawnerState::Throwing => {
                draw.image(&self.pokeball)
                    .position(origin.x + self.x + Self::OFFSET, origin.y + Self::f(self.x))
                    .rotate(self.x);
            }
            SpawnerState::Spawning => {
                let h = texture.height() as f32;
                let scale = self.x * h;
                let mut y = origin.y - scale * 1.5;
                let max = origin.y - h;
                if y < max {
                    y = max;
                }
                draw.image(texture).position(origin.x, y); // change color of texture when spawning here
                                                           // texture.draw(ctx, , DrawParams::default());
            }
            SpawnerState::Start => (),
            SpawnerState::End => {
                draw.image(texture).position(origin.x, origin.y);
            }
        }
    }
}
