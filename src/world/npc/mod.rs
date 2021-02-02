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
    //pub movement: Option<MovementType>,
    pub trainer: Option<Trainer>,

}

#[derive(Debug, Deserialize)]
pub struct NPCIdentifier {

    pub name: String,
    pub sprite: u8,

}

// #[derive(Clone, Debug, Deserialize)]
// pub enum MovementType {

//     Still,

// }

impl NPC {

    pub fn walk_to(&mut self, x: isize, y: isize) {
        match self.position.direction {
            Direction::Up => {
                self.position.y = y + 1;
            }
            Direction::Down => {
                self.position.y = y - 1;
            }
            Direction::Left => {
                self.position.x = x + 1;
            }
            Direction::Right => {
                self.position.x = x - 1;
            }
        }
    }

    pub fn interact(&mut self, direction: Option<Direction>, x: isize, y: isize) {
        if let Some(direction) = direction {
            self.position.direction = direction.inverse();
        }
        if let Some(trainer) = &mut self.trainer {
            macroquad::prelude::info!("Trainer battle with {}", &self.identifier.name);
            if !trainer.battled {
                trainer.battled = true;
                self.walk_to(x, y);
                crate::util::battle_data::trainer_battle(&self);
            }
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