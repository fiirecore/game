use macroquad::prelude::Texture2D;
use crate::util::graphics::byte_texture;

pub mod start_menu;
pub mod text_window;

type GuiTextures = Vec<Texture2D>;

static mut GUI_TEXTURES: Option<GuiTextures> = None;

pub fn load() {
    let mut gui_textures = GuiTextures::new();
    gui_textures.push(byte_texture(include_bytes!("../../../build/assets/condition.png")));
}

pub fn gui_texture(id: usize) -> Option<Texture2D> {
    unsafe{GUI_TEXTURES.as_ref()}.expect("Could not get GUI textures!").get(id).map(|texture| *texture)
}