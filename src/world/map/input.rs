use crate::engine::{
    input::controls::{down, Control},
    Context,
};
use worldlib::{
    character::{player::PlayerCharacter, Movement},
    positions::Direction,
};

#[derive(Default)]
pub struct PlayerInput {
    wait: f32,
    // queue: ArrayVec<[Direction; 4]>,
    first_direction: Direction,
}

impl PlayerInput {
    const MOVE_WAIT: f32 = 0.12;

    pub fn update(
        &mut self,
        player: &mut PlayerCharacter,
        ctx: &mut Context,
        delta: f32,
    ) -> Option<Direction> {
        if !player.moving() && !player.frozen() && !player.input_frozen {
            match down(ctx, Control::B) {
                true => {
                    if player.movement == Movement::Walking {
                        player.movement = Movement::Running;
                    }
                }
                false => {
                    if player.movement == Movement::Running {
                        player.movement = Movement::Walking;
                    }
                }
            }

            if down(ctx, Self::keybind(self.first_direction)) {
                if self.wait > Self::MOVE_WAIT {
                    return Some(self.first_direction);
                } else {
                    self.wait += delta;
                }
            } else {
                let mut movdir: Option<Direction> = None;
                for direction in &Direction::DIRECTIONS {
                    let direction = *direction;
                    if down(ctx, Self::keybind(direction)) {
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
                    player.position.direction = direction;
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
