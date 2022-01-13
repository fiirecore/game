use fiirengine::{error::ImageError, graphics::Color, Context};

pub extern crate firecore_font_builder as font;

pub use font::FontId;

use crate::EngineContext;

pub use firecore_text::*;

pub fn insert_font(
    ctx: &mut Context, 
    eng: &mut EngineContext,
    font_sheet: &font::FontSheet<Vec<u8>>,
) -> Result<(), ImageError> {
    eng.text.add_font_sheet(ctx, font_sheet)
}

pub struct TextColor;

impl TextColor {
    pub const BLACK: Color = Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0);
    pub const WHITE: Color = Color::rgb(240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0);
}


