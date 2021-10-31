use pokedex::context::PokedexClientContext;

use crate::{
    engine::{gui::MessageBox, util::Completable, EngineContext},
    game::battle_glue::{BattleId, BattleTrainerEntry},
};

pub mod managers;

pub mod closers;
pub mod transitions;

pub(crate) trait BattleTransition: Completable {
    fn update(&mut self, ctx: &mut EngineContext, delta: f32);

    fn draw(&self, ctx: &mut EngineContext);

    // fn render_below_player(&self);
}

pub(crate) trait BattleCloser: Completable {
    fn spawn<'d>(
        &mut self,
        ctx: &PokedexClientContext<'d>,
        player: &BattleId,
        player_name: &str,
        winner: Option<&BattleId>,
        trainer_entry: Option<&BattleTrainerEntry>,
        text: &mut MessageBox,
    );

    fn update(&mut self, ctx: &mut EngineContext, delta: f32, text: &mut MessageBox);

    fn draw(&self, ctx: &mut EngineContext);

    fn draw_battle(&self, ctx: &mut EngineContext);

    fn world_active(&self) -> bool;
}
