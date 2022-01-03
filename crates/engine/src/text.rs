use fiirengine::{error::ImageError, graphics::Color, Context};

pub extern crate firecore_font_builder as font;

pub use font::FontId;

use crate::EngineContext;

pub fn insert_font(
    ctx: &mut Context, 
    eng: &mut EngineContext,
    font_sheet: &font::FontSheet<Vec<u8>>,
) -> Result<(), ImageError> {
    eng.text.add_font_sheet(ctx, font_sheet)
}

impl MessagePage {
    pub const BLACK: Color = Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
    pub const WHITE: Color = Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0);
}

#[derive(Default, Debug, Clone)]
pub struct MessagePage {
    pub lines: Vec<String>,
    // #[serde(default)]
    pub wait: Option<f32>,
    // #[serde]
    pub color: Color,
}
