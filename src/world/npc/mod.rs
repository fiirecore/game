use std::io::Read;
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
        let mut path = "textures/npcs/".to_string();
        path.push_str(&id.to_string());
        path.push_str("/battle.png");
        let mut archive = crate::io::map::WORLD_ARCHIVE.lock();
        let mut bytes: Vec<u8> = Vec::new();
        match archive.by_name(&path) {
            Ok(mut zipfile) => {
                match zipfile.read_to_end(&mut bytes) {
                    Ok(_) => {}
                    Err(err) => {
                        warn!("Could not read battle sprite #{} to bytes with error {}", id, err);
                        return debug_texture();
                    }
                }
            }
            Err(err) => {
                warn!("Could not get battle texture for sprite id #{} with error {}", id, err);
                return debug_texture();
            }
        }
        crate::util::texture::byte_texture(bytes.as_slice())
    }

}