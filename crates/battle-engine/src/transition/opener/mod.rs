use core::ops::Deref;
use pokedex::{engine::utils::HashMap, item::Item, moves::Move, pokemon::Pokemon};

use pokedex::{
    engine::{
        graphics::{draw_rectangle, Color, DrawParams, Texture},
        math::Rectangle,
        utils::{Completable, Reset, WIDTH},
        Context,
    },
    PokedexClientData,
};

use crate::{
    context::BattleGuiData,
    ui::view::{ActivePokemonRenderer, GuiLocalPlayer, GuiRemotePlayer},
};

mod manager;

mod trainer;
mod wild;

pub use manager::BattleOpenerManager;
pub use trainer::TrainerBattleOpener;
pub use wild::WildBattleOpener;

pub enum Openers {
    Wild,
    Trainer,
}

impl Default for Openers {
    fn default() -> Self {
        Self::Wild
    }
}

pub(crate) trait BattleOpener<
    ID,
    P: Deref<Target = Pokemon>,
    M: Deref<Target = Move>,
    I: Deref<Target = Item>,
>: Completable
{
    #[allow(unused_variables)]
    fn spawn(
        &mut self,
        ctx: &PokedexClientData,
        local: &GuiLocalPlayer<ID, P, M, I>,
        opponents: &HashMap<ID, GuiRemotePlayer<ID, P>>,
    ) {
    }

    fn update(&mut self, delta: f32);

    // fn end(&mut self) {}

    fn draw_below_panel(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    );

    fn draw(&self, ctx: &mut Context);

    fn offset(&self) -> f32;
}

pub struct DefaultBattleOpener {
    wait: f32,

    offset: f32,

    rect_size: f32,
    shrink_by: f32,

    player: Texture,
}

impl DefaultBattleOpener {
    const RECT_SIZE: f32 = 80.0;
    const SHRINK_BY_DEF: f32 = 1.0;
    const SHRINK_BY_FAST: f32 = 4.0;
    const OFFSET: f32 = 153.0 * 2.0;
    const WAIT: f32 = 0.5;

    pub fn new(ctx: &BattleGuiData) -> Self {
        Self {
            wait: Self::WAIT,
            rect_size: Self::RECT_SIZE,
            shrink_by: Self::SHRINK_BY_DEF,
            offset: Self::OFFSET,
            player: ctx.player.clone(),
        }
    }
}

impl DefaultBattleOpener {

    pub fn update(&mut self, delta: f32) {
        match self.wait < 0.0 {
            false => self.wait -= delta,
            true => {
                if self.offset > 0.0 {
                    self.offset -= 120.0 * delta;
                    if self.offset < 0.0 {
                        self.offset = 0.0;
                    }
                }
                if self.rect_size > 0.0 {
                    if self.rect_size > 0.0 {
                        self.rect_size -= self.shrink_by * 60.0 * delta;
                        if self.rect_size < 0.0 {
                            self.rect_size = 0.0;
                        }
                    } else {
                        self.rect_size = 0.0;
                    }
                    if self.rect_size <= 58.0 && self.shrink_by != Self::SHRINK_BY_FAST {
                        self.shrink_by = Self::SHRINK_BY_FAST;
                    }
                }
            }
        }
    }

    pub fn draw_below_panel(
        &self,
        ctx: &mut Context,
        _player: &[ActivePokemonRenderer],
        _opponent: &[ActivePokemonRenderer],
    ) {
        self.player.draw(
            ctx,
            41.0 + self.offset,
            49.0,
            DrawParams::source(Rectangle::new(0.0, 0.0, 64.0, 64.0)),
        )
    }

    pub fn draw(&self, ctx: &mut Context) {
        draw_rectangle(ctx, 0.0, 0.0, WIDTH, self.rect_size, Color::BLACK);
        draw_rectangle(
            ctx,
            0.0,
            160.0 - self.rect_size,
            WIDTH,
            self.rect_size,
            Color::BLACK,
        );
    }

    pub fn offset(&self) -> f32 {
        self.offset
    }
}

impl Reset for DefaultBattleOpener {
    fn reset(&mut self) {
        self.offset = Self::OFFSET;
        self.rect_size = Self::RECT_SIZE;
        self.shrink_by = Self::SHRINK_BY_DEF;
        self.wait = Self::WAIT;
    }
}

impl Completable for DefaultBattleOpener {
    fn finished(&self) -> bool {
        self.offset <= 0.0
    }
}
