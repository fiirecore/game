use crate::io::data::Direction;
use crate::io::data::Position;
use crate::util::texture::Texture;
use crate::util::texture::debug_texture;
use macroquad::prelude::warn;
use serde::Deserialize;
use self::trainer::Trainer;

pub mod trainer;

#[derive(Deserialize)]
pub struct NPC {

    pub identifier: NPCIdentifier,
    pub position: Position,
    pub movement: Option<MovementType>,
    pub trainer: Option<Trainer>,

}

#[derive(Debug, Deserialize)]
pub struct NPCIdentifier {

    pub name: String,
    pub sprite: u8,

}

#[derive(Clone, Debug, Deserialize)]
pub enum MovementType {

    Still,

}

impl NPC {

    pub fn interact(&mut self, direction: Direction) {
        self.position.direction = direction.inverse();
        if self.trainer.is_some() {
            macroquad::prelude::info!("Trainer battle with {}", &self.identifier.name);
            crate::util::battle_data::trainer_battle(&self);
        }
    }

    pub fn battle_sprite(id: u8) -> Texture {
        match crate::io::ASSET_DIR.get_file(std::path::PathBuf::from("world/textures/npcs/").join(id.to_string()).join("battle.png")) {
            Some(file) => {
                return crate::util::texture::byte_texture(file.contents());
            }
            None => {
                warn!("Could not find file of battle sprite #{}", id);
                return debug_texture();
            }
        }
    }

}