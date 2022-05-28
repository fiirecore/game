use crate::engine::graphics::Draw;

pub mod managers;
pub mod transitions;

pub(crate) trait BattleTransition {
    fn update(&mut self, delta: f32);

    fn draw(&self, ctx: &mut Draw);

    fn reset(&mut self);

    fn finished(&self) -> bool;

    // fn render_below_player(&self);
}
