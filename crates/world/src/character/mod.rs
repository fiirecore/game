use crate::positions::{Destination, Direction, Path, PixelOffset, Position};
use serde::{Deserialize, Serialize};

pub mod message;
pub mod npc;
pub mod player;
pub mod trainer;
// pub mod pathfind;

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub name: String,

    pub position: Position,

    #[serde(skip)]
    pub offset: PixelOffset,

    #[serde(default)]
    pub movement: Movement,

    #[serde(default)]
    pub frozen: bool,

    #[serde(skip)]
    pub sprite: u8,

    #[serde(skip)]
    pub noclip: bool,

    #[serde(skip)]
    pub hidden: bool,

    #[serde(skip)]
    pub pathing: Path,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Movement {
    Walking,
    Running,
    Swimming,
}

impl Character {
    pub fn new<S: Into<String>>(name: S, position: Position) -> Self {
        Self {
            name: name.into(),
            position,
            ..Default::default()
        }
    }

    pub fn moving(&self) -> bool {
        !self.pathing.queue.is_empty() || !self.offset.is_zero()
    }

    pub fn update_sprite(&mut self) {
        self.sprite = if self.sprite == 0 { 2 } else { 0 }
    }

    pub fn on_try_move(&mut self, direction: Direction) {
        self.position.direction = direction;
    }

    pub fn stop_move(&mut self) {
        self.offset.reset();
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

    pub fn frozen(&self) -> bool {
        self.frozen && !self.noclip
    }

    pub fn pathfind(&mut self, destination: Destination) {
        self.pathing.extend(&self.position, destination);
        // match pathfind {
        //     true => {
        //         if let Some(path) = pathfind::pathfind(&self.position, destination, player, world) {
        //             self.pathing += path;
        //         }
        //     }
        //     false => ,
        // }
    }

    pub fn do_move(&mut self, delta: f32) -> bool {
        if !self.frozen() {
            match self.offset.is_zero() {
                true => {
                    match self.pathing.queue.pop() {
                        Some(direction) => {
                            self.position.direction = direction;
                            self.offset = direction.pixel_offset(self.speed() * 60.0 * delta);
                        }
                        None => {
                            if let Some(direction) = self.pathing.turn.take() {
                                self.position.direction = direction;
                            }
                        }
                    }
                    false
                }
                false => {
                    if self
                        .offset
                        .update(&self.position.direction, delta * self.speed() * 60.0)
                    {
                        self.position.coords += self.position.direction.tile_offset();
                        self.update_sprite();
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    pub fn sees(&self, sight: u8, position: &Position) -> bool {
        let tracker = sight as i32;
        if position.elevation != self.position.elevation && self.position.elevation != 0 {
            return false;
        }
        match self.position.direction {
            Direction::Up => {
                if self.position.coords.x == position.coords.x
                    && self.position.coords.y > position.coords.y
                    && self.position.coords.y - tracker <= position.coords.y
                {
                    return true;
                }
            }
            Direction::Down => {
                if self.position.coords.x == position.coords.x
                    && self.position.coords.y < position.coords.y
                    && self.position.coords.y + tracker >= position.coords.y
                {
                    return true;
                }
            }
            Direction::Left => {
                if self.position.coords.y == position.coords.y
                    && self.position.coords.x > position.coords.x
                    && self.position.coords.x - tracker <= position.coords.x
                {
                    return true;
                }
            }
            Direction::Right => {
                if self.position.coords.y == position.coords.y
                    && self.position.coords.x < position.coords.x
                    && self.position.coords.x + tracker >= position.coords.x
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn speed(&self) -> f32 {
        match self.movement {
            Movement::Walking => 1.0,
            Movement::Running | Movement::Swimming => 2.0,
        }
    }
}

impl Default for Movement {
    fn default() -> Self {
        Movement::Walking
    }
}
