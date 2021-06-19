use crate::{
    graphics::{byte_texture, position},
    tetra::{
        Context,
        graphics::Texture,
    }
};

static mut PANEL: Option<Texture> = None;

pub fn panel_texture(ctx: &mut Context) -> &Texture {
	unsafe { PANEL.get_or_insert(byte_texture(ctx, include_bytes!("../../../assets/battle/gui/panel.png"))) }
}

pub struct BattleBackground {

	background: Texture,
	ground: Texture,
	pub panel: Texture,

}

impl BattleBackground {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            background: byte_texture(ctx, include_bytes!("../../../assets/battle/background.png")),
            ground: byte_texture(ctx, include_bytes!("../../../assets/battle/ground.png")),
            panel: panel_texture(ctx).clone(),
        }

    }

    pub fn draw(&self, ctx: &mut Context, offset: f32) {
        self.background.draw(ctx, position(0.0, 1.0));
        self.ground.draw(ctx, position(113.0 - offset, 50.0));
		self.ground.draw(ctx, position(offset, 103.0));
    }

}