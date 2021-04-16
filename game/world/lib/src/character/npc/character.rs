use firecore_util::{Direction, Coordinate, Destination};

use crate::character::Character;

use super::NPC;

impl NPC {

    pub fn go_to(&mut self, to: Coordinate) {
        self.move_to(Destination::to(&self.position, to));
    }

    pub fn go_next_to(&mut self, to: Coordinate) {
        self.move_to(Destination::next_to(&self.position, to));
    }

}

impl Character for NPC {

    fn update_sprite(&mut self) {
        if self.properties.character.sprite_index == 0 {
            self.properties.character.sprite_index = 2;
        } else {
            self.properties.character.sprite_index = 0;
        }
    }

    fn on_try_move(&mut self, direction: Direction) {
        self.position.direction = direction;
        self.update_sprite();
    }

    fn stop_move(&mut self) {
        self.properties.character.moving = false;
		self.properties.character.running = false;
		self.properties.character.reset_speed();
    }

    fn freeze(&mut self) {
        self.properties.character.frozen = true;
        self.stop_move();
    }

    fn unfreeze(&mut self) {
        self.properties.character.frozen = false;
    }

    fn is_frozen(&self) -> bool {
        self.properties.character.frozen
    }

    fn move_to(&mut self, destination: Destination) {
        self.properties.character.destination = Some(crate::character::DestinationPath::new_path(self.position.coords, destination));
    }

    // fn should_move_to_destination(&self) -> bool {
    //     // self.properties.character.destination.as_ref().map(|path| path.paths.len() <= path.current).unwrap_or(false)
    //     // if let Some(destination) = self.properties.character.destination.as_ref() {
    //     //     self.position.coords != destination.coords
    //     // } else {
    //     //     false
    //     // }
    //     self.properties.character.destination.is_some()
    //     // if let Some(movements) = self.properties.character.destination.as_ref() {
    //     //     if movements.queued_movements.is_empty() {
    //     //         let movements = ();
    //     //         self.properties.character.destination = None;
    //     //         false
    //     //     } else {
    //     //         true
    //     //     }
    //     // } else {
    //     //     false
    //     // }
    // }

    fn move_to_destination(&mut self, delta: f32) -> bool {
        if let Some(destination) = self.properties.character.destination.as_mut() {
            if destination.started {
                if let Some(offset) = self.position.offset.update(delta * self.properties.character.base_speed, &self.position.direction) {
                    self.position.coords += offset;
                    if let Some(direction) = destination.queued_movements.pop_front() {
                        self.position.direction = direction;
                    } else if let Some(direction) = destination.final_direction {
                        let destination = ();
                        self.properties.character.destination = None;
                        self.position.direction = direction;
                        return true;
                    };
                    let destination = ();
                    self.update_sprite();
                }
            } else if let Some(direction) = destination.queued_movements.pop_front() {
                destination.started = true;
                self.position.direction = direction;
                self.move_to_destination(delta);
            }
        }
        false
        // if let Some(destination) = self.properties.character.destination {

        //     if self.position.coords.y == destination.coords.y {
        //         self.position.direction = if self.position.coords.x < destination.coords.x {
        //             Direction::Right
        //         } else {
        //             Direction::Left
        //         };
        //     } else if self.position.coords.x == destination.coords.x {
        //         self.position.direction = if self.position.coords.y < destination.coords.y {
        //             Direction::Down
        //         } else {
        //             Direction::Up
        //         };
        //     }

        //     if let Some(offset) = self.position.offset.update(delta * self.properties.character.speed, &self.position.direction) {
        //         self.position.coords += offset;
        //         self.update_sprite();
        //         if self.position.coords == destination.coords {
        //             if let Some(direction) = destination.direction {
        //                 self.position.direction = direction;
        //             }
        //         }             
        //     }
            
        // }
    
    }

}