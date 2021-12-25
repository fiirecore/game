pub extern crate firecore_pokedex as pokedex;

pub mod character;
pub mod map;
pub mod positions;
pub mod script;

pub mod serialized;

pub const TILE_SIZE: f32 = 16.0;

pub(crate) const fn default_true() -> bool {
    true
}

pub mod events {

    use std::{cell::Cell, rc::Rc};

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

    impl<T: WaitableAction> Sender<T> {
        pub fn send_polling(&self, msg: impl Into<T>) -> Option<Rc<Cell<bool>>> {
            let mut msg = msg.into();
            if msg.waitable() {
                let waiter = Rc::new(Cell::new(false));
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

        fn give(&mut self, waiter: Rc<Cell<bool>>);
    }
}

pub mod actions {
    use std::{cell::Cell, rc::Rc};

    use firecore_pokedex::item::SavedItemStack;
    use tinystr::TinyStr16;

    use crate::{
        character::npc::group::MessageColor, events::WaitableAction, map::battle::BattleEntry,
        positions::Coordinate,
    };

    #[derive(Debug, Clone)]
    pub struct WorldAction {
        pub action: WorldActions,
        pub receiver: Option<Rc<Cell<bool>>>,
    }

    #[derive(Debug, Clone)]
    pub enum WorldActions {
        PlayMusic(TinyStr16),
        BeginWarpTransition(Coordinate),
        PlayerJump,
        Message(Vec<Vec<String>>, MessageColor),

        /// Should freeze player and start battle
        Battle(BattleEntry),
        // GivePokemon(SavedPokemon),
        #[deprecated]
        GiveItem(SavedItemStack),
        OnTile,
        // Command(PlayerActions),
    }

    impl WaitableAction for WorldAction {
        fn waitable(&self) -> bool {
            matches!(self.action, WorldActions::Message(..))
        }

        fn give(&mut self, waiter: Rc<Cell<bool>>) {
            self.receiver = Some(waiter);
        }
    }

    impl From<WorldActions> for WorldAction {
        fn from(action: WorldActions) -> Self {
            Self {
                action,
                receiver: None,
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
