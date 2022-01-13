
pub extern crate firecore_pokedex as pokedex;

pub mod character;
pub mod map;
pub mod positions;
pub mod script;
pub mod state;

pub mod serialized;

pub const TILE_SIZE: f32 = 16.0;

pub mod events {

    use crate::positions::Direction;

    #[derive(Debug, Clone, Copy)]
    pub enum InputEvent {
        Move(Direction),
        Interact,
    }

    pub use crossbeam_channel::Receiver;

    pub fn split<T>() -> (Sender<T>, Receiver<T>) {
        let (x, y) = crossbeam_channel::unbounded();
        (Sender(x), y)
    }

    #[derive(Clone)]
    pub struct Sender<T>(crossbeam_channel::Sender<T>);

    impl<T> Sender<T> {
        pub fn send(&self, msg: impl Into<T>) {
            if let Err(err) = self.0.try_send(msg.into()) {}
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

}

pub mod actions {

    use crate::{
        map::{battle::BattleEntry, MusicId},
        positions::Coordinate,
    };

    #[derive(Debug, Clone)]
    pub enum WorldActions {
        PlayMusic(MusicId),
        BeginWarpTransition(Coordinate),
        PlayerJump,
        BreakObject(Coordinate),

        /// Should freeze player and start battle
        Battle(BattleEntry),
        OnTile,
        // Command(PlayerActions),
    }

    // #[derive(Debug, PartialEq, Eq, Hash)]
    // pub enum WorldEvents {
    //     TextActive,
    //     TextFinished,
    //     WarpFinished,
    // }

    // #[derive(Debug, Clone)]
    // pub enum PlayerActions {
    //     Wild(Option<bool>),
    //     NoClip(Option<bool>),
    //     Unfreeze,
    //     DebugDraw,
    // }
}
