use std::collections::VecDeque;

use firecore_util::{Direction, Destination};
use serde::{Deserialize, Serialize};
use util::Coordinate;

pub mod movement;
pub mod npc;
pub mod sprite;
pub mod player;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterProperties {

    #[serde(default = "default_speed")]
    pub base_speed: f32,

    #[serde(skip, default = "default_speed")]
    pub speed: f32,

    #[serde(skip)]
    pub sprite_index: u8,

    #[serde(skip)]
    pub moving: bool,

    #[serde(skip)]
    pub running: bool,

    #[serde(skip)]
    pub frozen: bool,

    #[serde(skip)]
    pub noclip: bool,

    // #[deprecated]
    #[serde(skip)]
    pub destination: Option<DestinationPath>,

    // #[serde(skip)]
    // pub destination: Option<DestinationPath>,

}

#[derive(Debug, Clone)]
pub struct DestinationPath {
    pub started: bool,
    pub queued_movements: VecDeque<Direction>,
    pub final_direction: Option<Direction>,
}

impl DestinationPath {
    pub fn new_path(origin: Coordinate, destination: Destination) -> Self {
        let xlen = destination.coords.x - origin.x;
        let xdir = if xlen.is_negative() {
            Direction::Left
        } else {
            Direction::Right
        };
        let mut vec = vec![xdir; xlen.abs() as usize];

        let ylen = destination.coords.y - origin.y;
        let ydir = if ylen.is_negative() {
            Direction::Up
        } else {
            Direction::Down
        };
        vec.append(&mut vec![ydir; ylen.abs() as usize]);
        let vec = VecDeque::from(vec);
        Self {
            started: false,
            queued_movements: vec,
            final_direction: destination.direction,
        }
    }
}

pub trait Character {

    fn update_sprite(&mut self);


    fn on_try_move(&mut self, direction: Direction);

    fn stop_move(&mut self);
    

    fn freeze(&mut self);

    fn unfreeze(&mut self);

    fn is_frozen(&self) -> bool;


    fn move_to(&mut self, destination: Destination);

    // fn should_move_to_destination(&self) -> bool;

    fn move_to_destination(&mut self, delta: f32) -> bool;

}

impl CharacterProperties {

    pub fn reset_speed(&mut self) {
        self.speed = self.base_speed;
    }

}

impl Default for CharacterProperties {
    fn default() -> Self {
        Self {
            base_speed: default_speed(),
            speed: default_speed(),
            sprite_index: 0,
            moving: false,
            running: false,
            frozen: false,
            noclip: false,
            destination: None,
        }
    }
}

const fn default_speed() -> f32 {
    1.0
}