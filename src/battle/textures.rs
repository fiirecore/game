use ahash::AHashMap;
use firecore_util::TinyStr16;
use macroquad::prelude::Texture2D;

pub type TrainerSprites = AHashMap<TinyStr16, Texture2D>;

pub static mut TRAINER_SPRITES: Option<TrainerSprites> = None;

pub fn trainer_texture(npc_type: &TinyStr16) -> Texture2D {
    unsafe{TRAINER_SPRITES.as_ref()}.expect("Could not get trainer sprites!").get(npc_type).map(|texture| *texture).unwrap_or(crate::util::graphics::debug_texture())
}