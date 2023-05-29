use crate::engine::controls::{down, pressed, Control};
use pokengine::engine::notan::prelude::{App, Plugins};
use worldlib::{
    audio::MusicId,
    character::{player::PlayerCharacter, Activity, CharacterState},
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

    pub const CYCLING_MUSIC: MusicId = unsafe {
        MusicId::from_bytes_unchecked([
            0x63, 0x79, 0x63, 0x6C, 0x69, 0x6E, 0x67, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    };

    const MOVE_WAIT: f32 = 0.12;

    pub fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        player: &mut PlayerCharacter,
        delta: f32,
    ) -> Option<Direction> {
        if !player.character.moving() && !player.character.input_lock.active() {
            fn try_run(app: &App, plugins: &Plugins, player: &mut PlayerCharacter) {
                match down(app, plugins, Control::B) {
                    true => {
                        if player.character.activity == Activity::Walking
                            && player.character.capabilities.contains(&CharacterState::RUN)
                        {
                            player.character.activity = Activity::Running;
                        }
                    }
                    false => {
                        if player.character.activity == Activity::Running {
                            player.character.activity = Activity::Walking;
                        }
                    }
                }
            }

            match pressed(app, plugins, Control::Select) {
                true => match matches!(player.character.activity, Activity::Cycling) {
                    false => {
                        if player
                            .character
                            .capabilities
                            .contains(&CharacterState::CYCLE)
                        {
                            player.character.activity = Activity::Cycling;
                            pokengine::engine::music::play_music(app, plugins, &Self::CYCLING_MUSIC);
                        }
                    }
                    true => {
                        player.character.activity = Activity::Walking;
                        try_run(app, plugins, player);
                    }
                },
                false => try_run(app, plugins, player),
            }

            if down(app, plugins, Self::keybind(self.first_direction)) {
                if self.wait > Self::MOVE_WAIT {
                    return Some(self.first_direction);
                } else {
                    self.wait += delta;
                }
            } else {
                let mut movdir: Option<Direction> = None;
                for direction in Direction::iter() {
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
