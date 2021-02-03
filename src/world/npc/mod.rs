use crate::io::data::Direction;
use crate::io::data::Position;
use crate::util::graphics::Texture;
use crate::util::graphics::texture::debug_texture;
use macroquad::prelude::warn;
use serde::Deserialize;
use self::trainer::Trainer;

pub mod trainer;

#[derive(Deserialize)]
pub struct NPC {

    pub identifier: NPCIdentifier,
    pub position: Position, // Home position
    #[serde(skip)]
    pub offset: Option<(isize, isize)>, // Offset from home position, see if changing the struct to something that uses variables better would help
    // pub movement: Option<MovementType>,
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
                self.offset = Some((x, y + 1));
            }
            Direction::Down => {
                self.offset = Some((x, y - 1));
            }
            Direction::Left => {
                self.offset = Some((x + 1, y));
            }
            Direction::Right => {
                self.offset = Some((x - 1, y));
            }
        }
    }

    pub fn should_move(&self) -> bool {
        if let Some(offset) = self.offset {
            self.position.x != offset.0 || self.position.y != offset.1
        } else {
            false
        }
    }

    pub fn interact(&mut self, direction: Option<Direction>, x: isize, y: isize) {
        if let Some(direction) = direction {
            self.position.direction = direction.inverse();
        }
        if self.trainer.is_some() {
            macroquad::prelude::info!("Trainer battle with {}", &self.identifier.name);
            // if !trainer.battled {
            //     trainer.battled = true;
                self.walk_to(x, y);
            //}
        }
    }

    pub fn after_interact(&mut self) {
        self.trainer.as_mut().unwrap().tracking_length = None;
        crate::util::battle_data::trainer_battle(&self);
    }

    pub fn battle_sprite(id: u8) -> Texture {
        match crate::io::ASSET_DIR.get_file(std::path::PathBuf::from("world/textures/npcs/").join(id.to_string()).join("battle.png")) {
            Some(file) => {
                return crate::util::graphics::texture::byte_texture(file.contents());
            }
            None => {
                warn!("Could not find file of battle sprite #{}", id);
                return debug_texture();
            }
        }
    }

}