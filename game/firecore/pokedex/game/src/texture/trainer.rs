use deps::{hash::HashMap, tetra::graphics::Texture};
use pokedex::trainer::TrainerId;

pub type TrainerTextures = HashMap<TrainerId, Texture>;

pub static mut TRAINER_TEXTURES: Option<TrainerTextures> = None;

pub fn trainer_texture(npc_type: &TrainerId) -> &'static Texture {
    unsafe { TRAINER_TEXTURES.as_ref().expect("Could not get trainer textures! (Not initialized)").get(npc_type).unwrap_or_else(|| panic!("Could not get trainer texture for Npc Type {}", npc_type)) }
}

pub fn set_trainer_textures(textures: TrainerTextures) {
    unsafe { TRAINER_TEXTURES = Some(textures) }
}