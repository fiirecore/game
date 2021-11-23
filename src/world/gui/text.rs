use crate::engine::{graphics::Texture, gui::MessageBox, math::{Vec2, vec2}, util::Entity, Context};

pub struct TextWindow {
    background: Texture,
    pub text: MessageBox,
}

impl TextWindow {
    const ORIGIN: Vec2 = vec2(6.0, 116.0);
    const TEXT_OFFSET: Vec2 = vec2(11.0, 5.0);

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            background: Texture::new(
                ctx,
                include_bytes!("../../../assets/world/textures/gui/message.png"),
            ).unwrap(),
            text: MessageBox::new(Self::ORIGIN + Self::TEXT_OFFSET, 1),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.text.alive() {
            self.background
                .draw(ctx, Self::ORIGIN.x, Self::ORIGIN.y, Default::default());
            self.text.draw(ctx);
        }
    }
}
