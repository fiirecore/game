use crate::engine::graphics::Draw;

pub mod manager;
pub mod transitions;

pub(crate) trait BattleTransition {
    fn update(&mut self, delta: f32) -> bool;

    fn draw(&self, ctx: &mut Draw);

    fn reset(&mut self);

    // fn render_below_player(&self);
}
