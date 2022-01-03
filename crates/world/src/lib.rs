
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

    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

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

    #[derive(Default, Debug, Clone)]
    #[repr(transparent)]
    pub struct Wait(Arc<AtomicBool>);

    impl Wait {
        pub fn update(&self) {
            self.0.store(true, Ordering::Relaxed)
        }

        pub fn get(&self) -> bool {
            self.0.load(Ordering::Relaxed)
        }
    }

    impl<T: WaitableAction> Sender<T> {
        pub fn send_polling(&self, msg: impl Into<T>) -> Option<Wait> {
            let mut msg = msg.into();
            if msg.waitable() {
                let waiter = Wait::default();
                msg.give(waiter.clone());
                self.send(msg);
                return Some(waiter);
            } else {
                self.send(msg);
                None
            }
        }
    }

    pub trait WaitableAction {
        fn waitable(&self) -> bool;

        fn give(&mut self, waiter: Wait);
    }
}

pub mod actions {

    use crate::{
        character::npc::group::MessageColor,
        events::{Wait, WaitableAction},
        map::{battle::BattleEntry, MusicId},
        positions::Coordinate,
    };

    #[derive(Debug, Clone)]
    pub struct WorldAction {
        pub action: WorldActions,
        pub waiter: Option<Wait>,
    }

    #[derive(Debug, Clone)]
    pub enum WorldActions {
        PlayMusic(MusicId),
        BeginWarpTransition(Coordinate),
        PlayerJump,
        Message(Vec<Vec<String>>, MessageColor),
        BreakObject(Coordinate),

        /// Should freeze player and start battle
        Battle(BattleEntry),
        OnTile,
        // Command(PlayerActions),
    }

    impl WaitableAction for WorldAction {
        fn waitable(&self) -> bool {
            matches!(self.action, WorldActions::Message(..))
        }

        fn give(&mut self, waiter: Wait) {
            self.waiter = Some(waiter);
        }
    }

    impl From<WorldActions> for WorldAction {
        fn from(action: WorldActions) -> Self {
            Self {
                action,
                waiter: None,
            }
        }
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
