use crate::engine::game_context::GameContext;
use crate::io::data::Direction;
use crate::io::data::Position;
use crate::util::file_util::asset_as_pathbuf;
use crate::util::texture_util::texture64_from_path;

use opengl_graphics::Texture;
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

    pub fn interact(&mut self, direction: Direction, context: &mut GameContext) {
        self.position.direction = direction.inverse();
        if self.trainer.is_some() {
            context.battle_context.trainer_battle(&self);
        }
    }

    pub fn battle_sprite(id: u8) -> Texture {
        texture64_from_path(asset_as_pathbuf("worlds/firered/textures/npcs").join(id.to_string()).join("battle.png"))
    }

}