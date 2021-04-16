use firecore_game::macroquad::prelude::Texture2D;
use firecore_game::graphics::byte_texture;
use firecore_game::util::hash::HashMap;

pub mod start_menu;
pub mod text_window;

#[derive(PartialEq, Eq, Hash)]
pub enum GuiTexture {
    Condition,
}

type GuiTextures = HashMap<GuiTexture, Texture2D>;

static mut GUI_TEXTURES: Option<GuiTextures> = None;

pub fn load() {
    let mut gui_textures = GuiTextures::new();
    gui_textures.insert(GuiTexture::Condition, byte_texture(include_bytes!("../../assets/gui/world/condition.png")));
    unsafe { GUI_TEXTURES = Some(gui_textures); }
}

pub fn gui_texture(id: &GuiTexture) -> Option<Texture2D> {
    unsafe{GUI_TEXTURES.as_ref()}.expect("Could not get GUI textures!").get(id).map(|texture| *texture)
}