use crate::pokengine::PokedexClientData;

use crate::engine::{utils::Completable, Context, EngineContext};

use firecore_battle_engine::ui::text::BattleText;
use worldcli::battle::*;

pub mod managers;

pub mod closers;
pub mod transitions;

pub(crate) trait BattleTransition: Completable {
    fn update(&mut self, ctx: &mut Context, delta: f32);

    fn draw(&self, ctx: &mut Context);

    // fn render_below_player(&self);
}

pub(crate) trait BattleCloser: Completable {
    fn spawn<'d>(
        &mut self,
        ctx: &PokedexClientData,
        player: &BattleId,
        player_name: &str,
        winner: Option<&BattleId>,
        trainer_entry: Option<&BattleTrainerEntry>,
        text: &mut BattleText,
    );

    fn update(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
        text: &mut BattleText,
    );

    fn draw(&self, ctx: &mut Context);

    fn draw_battle(&self, ctx: &mut Context);

    fn world_active(&self) -> bool;
}
