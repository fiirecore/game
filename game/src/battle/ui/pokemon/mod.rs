use crate::{
    util::Reset,
    pokedex::{
        pokemon::PokemonId,
        texture::{PokemonTexture, pokemon_texture},
    },
    graphics::{byte_texture, position},
    tetra::{
        Context,
        math::Vec2,
        graphics::{
            Color,
            Texture,
            Rectangle,
        }
    },
};

use crate::battle::ui::{BattleGuiPosition, BattleGuiPositionIndex};

pub mod status;
pub mod bounce;

pub struct PokemonRenderer {

    pub texture: Option<Texture>,
    side: PokemonTexture,

    pub pos: Vec2<f32>,

    pub spawner: Spawner,
    pub faint: Faint,
    pub flicker: Flicker,

}

pub struct Spawner {
    spawning: SpawnerState,
    texture: Texture,
    x: f32,
}

#[derive(PartialEq)]
enum SpawnerState {
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

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            spawning: SpawnerState::None,
            x: 0.0,
            texture: unsafe { POKEBALL.get_or_insert(byte_texture(ctx, include_bytes!("../../../../assets/battle/thrown_pokeball.png"))).clone() }
        }
    }

    fn f(x: f32) -> f32 {
        0.5 * (x - Self::PARABOLA_ORIGIN).powi(2) - 50.0
    }

    pub fn update(&mut self, delta: f32) {
        match self.spawning {
            SpawnerState::Start => {
                self.x = Self::ORIGIN;
                self.spawning = SpawnerState::Throwing;
                self.update(delta);
            }
            SpawnerState::Throwing => {
                self.x += delta * 20.0;
                if self.x > Self::LEN {
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

    fn draw(&self, ctx: &mut Context, origin: Vec2<f32>, texture: &Texture) {
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

#[derive(Default)]
pub struct Flicker {
    remaining: u8,
    accumulator: f32,
}

impl Flicker {

    const HALF: f32 = Self::LENGTH / 2.0;
    const LENGTH: f32 = 0.20;
    const TIMES: u8 = 4;

    pub fn update(&mut self, delta: f32) {
        if self.remaining != 0 {
            self.accumulator += delta;
            if self.accumulator > Self::LENGTH {
                self.accumulator -= Self::LENGTH;
                self.remaining -= 1;
            }
            if self.remaining == 0 {
                self.accumulator = 0.0;
            }
        }
    }

    pub fn flickering(&self) -> bool {
        self.remaining != 0
    }

}

#[derive(Default)]
pub struct Faint {
    fainting: bool,
    remaining: f32,
}

impl Faint {

    pub fn update(&mut self, delta: f32) {
        if self.fainting {
            self.remaining -= delta * 128.0;
            if self.remaining < 0.0 {
                self.remaining = 0.0;
            }
        }
    }

    pub fn fainting(&self) -> bool {
        self.fainting && self.remaining != 0.0
    }

}

impl PokemonRenderer {

    pub fn new(ctx: &mut Context, index: BattleGuiPositionIndex, side: PokemonTexture) -> Self {
        Self {
            texture: None,
            side,
            pos: Self::position(index),
            spawner: Spawner::new(ctx),
            faint: Faint::default(),
            flicker: Flicker::default(),
        }
    }

    pub fn with(ctx: &mut Context, index: BattleGuiPositionIndex, pokemon: Option<&PokemonId>, side: PokemonTexture) -> Self {
        Self {
            texture: pokemon.map(|pokemon| pokemon_texture(pokemon, side).clone()),
            ..Self::new(ctx, index, side)
        }
    }

    fn position(index: BattleGuiPositionIndex) -> Vec2<f32> {
        let offset = (index.size - 1) as f32 * 32.0 - index.index as f32 * 64.0;
        match index.position {
            BattleGuiPosition::Top => Vec2::new(144.0 - offset, 74.0),
            BattleGuiPosition::Bottom => Vec2::new(40.0 - offset, 113.0),
        }
    }

    pub fn new_pokemon(&mut self, pokemon: Option<&PokemonId>) {
        self.texture = pokemon.map(|pokemon| pokemon_texture(pokemon, self.side)).cloned();
        self.reset();
    }

    pub fn spawn(&mut self) {
        self.spawner.spawning = SpawnerState::Start;
        self.spawner.x = 0.0;
    }

    pub fn faint(&mut self) {
        if let Some(texture) =self.texture.as_ref() {
            self.faint.fainting = true;
            self.faint.remaining = texture.height() as f32;
        }
    }

    pub fn flicker(&mut self) {
        self.flicker.remaining = Flicker::TIMES;
        self.flicker.accumulator = 0.0;
    }

    pub fn draw(&self, ctx: &mut Context, offset: Vec2<f32>, color: Color) {
        if let Some(texture) = &self.texture {
            let pos = self.pos + offset;
            if self.spawner.spawning() {
                self.spawner.draw(ctx, pos, texture);
            } else if self.flicker.accumulator < Flicker::HALF {
                if self.faint.fainting {
                    if self.faint.remaining > 0.0 {
                        texture.draw_region(
                            ctx,
                            Rectangle::new(0.0, 0.0, texture.width() as f32, self.faint.remaining),
                            position(pos.x, pos.y - self.faint.remaining).color(color),
                        );
                    }
                } else {
                    texture.draw(ctx, position(pos.x, pos.y - texture.height() as f32).color(color));
                }
            }
        }
    }

}

impl Reset for PokemonRenderer {
    fn reset(&mut self) {
        self.faint = Faint::default();
        self.flicker = Flicker::default();
        self.spawner.spawning = SpawnerState::None;
    }
}