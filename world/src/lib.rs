extern crate firecore_pokedex as pokedex;

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

    pub use crossbeam_channel::Receiver;

    pub fn split<T>() -> (Sender<T>, Receiver<T>) {
        let (x, y) = crossbeam_channel::unbounded();
        (Sender(x), y)
    }

    #[derive(Clone)]
    pub struct Sender<T>(crossbeam_channel::Sender<T>);

    impl<T> Sender<T> {
        pub fn send(&self, msg: T) {
            if let Err(err) = self.0.try_send(msg) {}
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }
}

pub mod actions {
    use firecore_pokedex::item::SavedItemStack;
    use tinystr::TinyStr16;

    use crate::{character::npc::NpcId, map::battle::BattleEntry, positions::Coordinate};

    #[derive(Debug, Clone)]
    pub enum WorldActions {
        PlayMusic(TinyStr16),
        BeginWarpTransition(Coordinate),
        PlayerJump,
        Message(Option<(NpcId, bool)>, Vec<Vec<String>>, bool),
        Battle(BattleEntry),
        // GivePokemon(SavedPokemon),
        #[deprecated]
        GiveItem(SavedItemStack),
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
