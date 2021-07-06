use engine::{
    util::Entity,
    gui::TextDisplay,
    text::TextColor,
    graphics::{byte_texture, position},
    tetra::{
        Context,
        math::Vec2,
        graphics::Texture,
    },
};

pub struct TextWindow {

    background: Texture,
    pub text: TextDisplay,

}

impl TextWindow {

    const ORIGIN: Vec2<f32> = Vec2::new(6.0, 116.0);
    const TEXT_OFFSET: Vec2<f32> = Vec2::new(11.0, 5.0);

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            background: byte_texture(ctx, include_bytes!("../../../assets/world/gui/message.png")),
            text: TextDisplay::new(Self::ORIGIN + Self::TEXT_OFFSET, 1, TextColor::Black, 5),
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.text.alive() {
            self.background.draw(ctx, position(Self::ORIGIN.x, Self::ORIGIN.y));
            self.text.draw(ctx);
        }
    }
}