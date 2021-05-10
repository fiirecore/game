use serde::{Deserialize, Serialize};
use firecore_util::{Direction, Coordinate, Position, Destination};

use self::destination::DestinationPath;

pub mod movement;
pub mod destination;
pub mod npc;
pub mod sprite;
pub mod player;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {

    pub position: Position,

    #[serde(default = "default_speed")]
    pub speed: f32,

    #[serde(skip)]
    pub sprite_index: u8,

    #[serde(skip)]
    pub moving: bool,

    #[serde(default)]
    pub move_type: MoveType,

    #[serde(skip)]
    pub frozen: bool,

    #[serde(skip)]
    pub noclip: bool,

    #[serde(skip)]
    pub destination: Option<DestinationPath>,

}

impl Character {

    pub const fn new(position: Position) -> Self {
        Self {
            position,
            // base_speed: default_speed(),
            speed: default_speed(),
            sprite_index: 0,
            moving: false,
            move_type: default_move_type(),
            frozen: false,
            noclip: false,
            destination: None,
        }
    }

    pub fn update_sprite(&mut self) {
        self.sprite_index = if self.sprite_index == 0 {
            2
        } else {
            0
        }
    }

    pub fn on_try_move(&mut self, direction: Direction) {
        self.position.direction = direction;
        // self.update_sprite();
    }

    pub fn stop_move(&mut self) {
        self.moving = false;
        self.position.offset.reset();
        // self.reset_speed();
    }

    pub fn freeze(&mut self) {
        if !self.noclip {
            self.frozen = true;
            self.stop_move();
        }
    }

    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }

    pub fn is_frozen(&self) -> bool {
        self.frozen && !self.noclip
    }

    pub fn move_to(&mut self, destination: Destination) {
        self.destination = Some(crate::character::DestinationPath::new_path(self.position.coords, destination));
    }

    pub fn move_to_destination(&mut self, delta: f32) -> bool {
        if let Some(destination) = self.destination.as_mut() {
            if destination.started {
                if self.position.offset.update(delta * self.speed * match self.move_type {
                    MoveType::Walking => 1.0,
                    _ => 2.0,
                }, &self.position.direction) {
                    self.position.coords += self.position.direction.tile_offset();
                    if let Some(direction) = destination.queued_movements.pop_front() {
                        self.position.direction = direction;
                    } else if let Some(direction) = destination.final_direction {
                        self.destination = None;
                        self.position.direction = direction;
                        return true;
                    };
                    self.update_sprite();
                }
            } else if let Some(direction) = destination.queued_movements.pop_front() {
                destination.started = true;
                self.moving = true;
                self.position.direction = direction;
                self.move_to_destination(delta);
            } else if destination.queued_movements.is_empty() {
                if let Some(direction) = destination.final_direction {
                    self.position.direction = direction;
                }
                self.moving = false;
                self.destination = None;
            }
        }
        false
    }

    pub fn speed(&self) -> f32 {
        self.speed * match self.move_type {
            MoveType::Walking => 1.0,
            _ => 2.0,
        } * if self.noclip {
            3.0
        } else {
            1.0
        }
    }

    // pub fn reset_speed(&mut self) {
    //     self.speed = self.base_speed;
    // }

    pub fn go_to(&mut self, to: Coordinate) {
        self.move_to(Destination::to(&self.position, to));
    }

    pub fn go_next_to(&mut self, to: Coordinate) {
        self.move_to(Destination::next_to(&self.position, to));
    }

}

impl Default for Character {
    fn default() -> Self {
        Self {
            position: Position::default(),
            // base_speed: default_speed(),
            speed: default_speed(),
            sprite_index: 0,
            moving: false,
            move_type: MoveType::default(),
            frozen: false,
            noclip: false,
            destination: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum MoveType {

    Walking,
    Running,
    Swimming,

}

impl Default for MoveType {
    fn default() -> Self {
        default_move_type()
    }
}

const fn default_speed() -> f32 {
    1.0
}

const fn default_move_type() -> MoveType {
    MoveType::Walking
}