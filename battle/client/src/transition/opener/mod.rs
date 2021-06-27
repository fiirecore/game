use game::{
    graphics::{byte_texture, draw_rectangle, position},
    pokedex::trainer::TrainerData,
    tetra::{
        graphics::{Color, Rectangle, Texture},
        Context,
    },
    util::{Completable, Reset, Timer, WIDTH},
};

use crate::ui::view::ActiveRenderer;

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

pub(crate) trait BattleOpener: Completable {
    fn spawn(&mut self, opponent: Option<&TrainerData>);

    fn update(&mut self, delta: f32);

    fn draw_below_panel(
        &self,
        ctx: &mut Context,
        player: &ActiveRenderer,
        opponent: &ActiveRenderer,
    );

    fn draw(&self, ctx: &mut Context);

    fn offset(&self) -> f32;
}

static mut PLAYER: Option<Texture> = None;

pub fn player_texture(ctx: &mut Context) -> &Texture {
    unsafe {
        PLAYER.get_or_insert(byte_texture(
            ctx,
            include_bytes!("../../../assets/player.png"),
        ))
    }
}

pub struct DefaultBattleOpener {
    start_timer: Timer,

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

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            start_timer: Timer::new(true, 0.5),
            rect_size: Self::RECT_SIZE,
            shrink_by: Self::SHRINK_BY_DEF,
            offset: Self::OFFSET,
            player: player_texture(ctx).clone(),
        }
    }
}

impl BattleOpener for DefaultBattleOpener {
    fn spawn(&mut self, _: Option<&TrainerData>) {}

    fn update(&mut self, delta: f32) {
        if self.start_timer.finished() {
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
        } else {
            self.start_timer.update(delta);
        }
    }

    fn draw_below_panel(
        &self,
        ctx: &mut Context,
        _player: &ActiveRenderer,
        _opponent: &ActiveRenderer,
    ) {
        self.player.draw_region(
            ctx,
            Rectangle::new(0.0, 0.0, 64.0, 64.0),
            position(41.0 + self.offset, 49.0),
        )
    }

    fn draw(&self, ctx: &mut Context) {
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

    fn offset(&self) -> f32 {
        self.offset
    }
}

impl Reset for DefaultBattleOpener {
    fn reset(&mut self) {
        self.offset = Self::OFFSET;
        self.rect_size = Self::RECT_SIZE;
        self.shrink_by = Self::SHRINK_BY_DEF;
        self.start_timer.hard_reset();
    }
}

impl Completable for DefaultBattleOpener {
    fn finished(&self) -> bool {
        self.offset <= 0.0
    }
}
