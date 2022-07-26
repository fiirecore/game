use crate::engine::controls::{down, Control};
use pokengine::engine::notan::prelude::{App, Plugins};
use worldlib::{
    character::{player::PlayerCharacter, Activity},
    positions::Direction,
};

#[derive(Default)]
pub struct PlayerInput {
    wait: f32,
    // queue: ArrayVec<[Direction; 4]>,
    first_direction: Direction,
}

impl PlayerInput {
    // pub const INPUT_LOCK: CharacterFlag = unsafe { CharacterFlag::from_bytes_unchecked(500186508905u64.to_ne_bytes()) };
    pub const INPUT_LOCK_VAL: i8 = 0;

    const MOVE_WAIT: f32 = 0.12;

    pub fn update(
        &mut self,
        app: &App,
        plugins: &Plugins,
        player: &mut PlayerCharacter,
        delta: f32,
    ) -> Option<Direction> {
        if !player.character.moving() && !player.character.input_lock.active() {
            match down(app, plugins, Control::B) {
                true => {
                    if player.character.activity == Activity::Walking {
                        player.character.activity = Activity::Running;
                    }
                }
                false => {
                    if player.character.activity == Activity::Running {
                        player.character.activity = Activity::Walking;
                    }
                }
            }

            if down(app, plugins, Self::keybind(self.first_direction)) {
                if self.wait > Self::MOVE_WAIT {
                    return Some(self.first_direction);
                } else {
                    self.wait += delta;
                }
            } else {
                let mut movdir: Option<Direction> = None;
                for direction in &[
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    let direction = *direction;
                    if down(app, plugins, Self::keybind(direction)) {
                        movdir = if let Some(dir) = movdir {
                            if dir.inverse() == direction {
                                None
                            } else {
                                Some(direction)
                            }
                        } else {
                            Some(direction)
                        };
                    }
                }
                if let Some(direction) = movdir {
                    player.character.position.direction = direction;
                    self.first_direction = direction;
                } else {
                    self.wait = 0.0;
                }
            }
        }
        None
    }

    pub fn keybind(direction: Direction) -> Control {
        match direction {
            Direction::Up => Control::Up,
            Direction::Down => Control::Down,
            Direction::Left => Control::Left,
            Direction::Right => Control::Right,
        }
    }
}
