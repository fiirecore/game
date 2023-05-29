use crate::{text::font::FontId, EngineContext};

use fiirengine::{graphics::DrawParams, Context};

pub use font::Font;

pub(crate) mod font;
pub(crate) mod renderer;

pub fn font<'c>(eng: &'c EngineContext, font: &FontId) -> Option<&'c Font> {
    eng.text.get(font)
}

pub fn draw_text_left(
    ctx: &mut Context,
    eng: &EngineContext,
    font: &FontId,
    text: &str,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    eng.text.draw_text_left(ctx, font, text, x, y, params)
}

pub fn draw_text_right(
    ctx: &mut Context,
    eng: &EngineContext,
    font: &FontId,
    text: &str,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    eng.text.draw_text_right(ctx, font, text, x, y, params)
}

pub fn draw_text_center(
    ctx: &mut Context,
    eng: &EngineContext,
    font: &FontId,
    text: &str,
    center_vertical: bool,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    eng.text
        .draw_text_center(ctx, font, text, center_vertical, x, y, params)
}

pub fn draw_button_for_text(
    ctx: &mut Context,
    eng: &EngineContext,
    font: &FontId,
    text: &str,
    x: f32,
    y: f32,
    params: DrawParams,
) {
    eng.text.draw_button_for_text(ctx, font, text, x, y, params)
}

pub fn draw_cursor(ctx: &mut Context, eng: &EngineContext, x: f32, y: f32, params: DrawParams) {
    eng.text.draw_cursor(ctx, x, y, params)
}

pub fn text_len(eng: &EngineContext, font: &FontId, text: &str) -> f32 {
    eng.text.text_len(font, text)
}
