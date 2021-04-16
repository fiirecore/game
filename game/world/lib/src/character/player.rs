use firecore_util::Destination;
use firecore_util::Direction;
use firecore_util::GlobalPosition;

use super::Character;
use super::CharacterProperties;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct PlayerCharacter {

	pub position: GlobalPosition,
	pub properties: CharacterProperties,

	pub input_frozen: bool,

}

impl PlayerCharacter {

    pub fn do_move(&mut self, delta: f32) -> bool {
        if !self.position.local.offset.is_none() {
            if let Some(offset) = self.position.local.offset.update(delta * self.properties.speed, &self.position.local.direction) {
                self.position.local.coords += offset;
                self.update_sprite();
                true
            } else {
                false
            }
        } else {
            false
        }        
    }

    pub fn freeze_input(&mut self) {
        self.input_frozen = true;
        self.stop_move();
    }

}

impl Character for PlayerCharacter {

    fn update_sprite(&mut self) {
        if self.properties.sprite_index == 0 {
			self.properties.sprite_index = 2;
		} else {
			self.properties.sprite_index = 0;
		}
    }

    fn on_try_move(&mut self, direction: Direction) {
        self.position.local.direction = direction;
    }

    fn stop_move(&mut self) {
        self.properties.moving = false;
		self.properties.running = false;
		self.position.local.offset.reset();
		self.properties.reset_speed();
    }

    fn freeze(&mut self) {
		self.properties.frozen = true;
        self.stop_move();
    }

    fn unfreeze(&mut self) {
        self.input_frozen = false;
        self.properties.frozen = false;
    }

    fn is_frozen(&self) -> bool {
        self.properties.frozen || self.input_frozen
    }

    fn move_to(&mut self, destination: Destination) {
        let mut new_dest = super::DestinationPath::new_path(self.position.local.coords, destination);
        match self.properties.destination.as_mut() {
            Some(destination) => {
                destination.queued_movements.append(&mut new_dest.queued_movements);
                destination.final_direction = new_dest.final_direction;
            }
            None => self.properties.destination = Some(new_dest),
        }
    }

    fn move_to_destination(&mut self, delta: f32) -> bool {
        if let Some(destination) = self.properties.destination.as_mut() {
            if destination.started {
                if let Some(offset) = self.position.local.offset.update(delta * self.properties.base_speed, &self.position.local.direction) {
                    self.position.local.coords += offset;
                    if let Some(direction) = destination.queued_movements.pop_front() {
                        self.position.local.direction = direction;
                    } else if let Some(direction) = destination.final_direction {
                        let destination = ();
                        self.properties.moving = false;
                        self.properties.destination = None;
                        self.position.local.direction = direction;
                        return true;
                    };
                    let destination = ();
                    self.update_sprite();
                }
            } else if let Some(direction) = destination.queued_movements.pop_front() {
                self.properties.moving = true;
                destination.started = true;
                self.position.local.direction = direction;
                self.move_to_destination(delta);
            }
        }
        false
    }
}