use core::ops::Deref;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use pokedex::engine::{
    utils::{Completable, Reset},
    Context,
    {
        graphics::{Color, Texture},
        math::Vec2,
    },
};

use crate::{context::BattleGuiData, ui::view::ActivePokemonRenderer};

use super::{BattleOpener, DefaultBattleOpener};

pub struct WildBattleOpener {
    opener: DefaultBattleOpener,

    grass: Texture,
    offset: Vec2,
}

impl WildBattleOpener {
    const LIGHTGRAY: Color = Color::rgb(0.78, 0.78, 0.78);
    const GRASS_WIDTH: f32 = 128.0;
    const GRASS_HEIGHT: f32 = 47.0;
    pub fn new(ctx: &mut Context, gui: &BattleGuiData) -> Self {
        Self {
            opener: DefaultBattleOpener::new(gui),
            grass: Texture::new(ctx, include_bytes!("../../../assets/grass.png")).unwrap(),
            offset: Vec2::new(Self::GRASS_WIDTH, Self::GRASS_HEIGHT),
        }
    }
}

impl<ID, P: Deref<Target = Pokemon>, M: Deref<Target = Move>, I: Deref<Target = Item>>
    BattleOpener<ID, P, M, I> for WildBattleOpener
{
    fn update(&mut self, delta: f32) {
        self.opener.update(delta);
        if self.offset.y > 0.0 {
            self.offset.x -= 360.0 * delta;
            if self.offset.x < 0.0 {
                self.offset.x += Self::GRASS_WIDTH;
            }
            if self.opener.offset() <= 130.0 {
                self.offset.y -= 60.0 * delta;
            }
        }
    }

    fn offset(&self) -> f32 {
        self.opener.offset
    }

    fn draw_below_panel(
        &self,
        ctx: &mut Context,
        player: &[ActivePokemonRenderer],
        opponent: &[ActivePokemonRenderer],
    ) {
        for active in opponent.iter() {
            active
                .pokemon
                .draw(ctx, Vec2::new(-self.opener.offset, 0.0), Self::LIGHTGRAY);
        }
        self.opener.draw_below_panel(ctx, player, opponent);
        if self.offset.y > 0.0 {
            let y = 114.0 - self.offset.y;
            self.grass.draw(
                ctx,
                self.offset.x - Self::GRASS_WIDTH,
                y,
                Default::default(),
            );
            self.grass.draw(ctx, self.offset.x, y, Default::default());
            self.grass.draw(
                ctx,
                self.offset.x + Self::GRASS_WIDTH,
                y,
                Default::default(),
            );
        }
    }

    fn draw(&self, ctx: &mut Context) {
        self.opener.draw(ctx);
    }
}

impl Reset for WildBattleOpener {
    fn reset(&mut self) {
        self.offset = Vec2::new(Self::GRASS_WIDTH, Self::GRASS_HEIGHT);
        self.opener.reset();
    }
}
impl Completable for WildBattleOpener {
    fn finished(&self) -> bool {
        self.opener.finished() && self.offset.y <= 0.0
    }
}
