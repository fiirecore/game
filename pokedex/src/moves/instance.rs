use deps::vec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::moves::{MoveRef, PP};

pub type MoveInstanceSet = ArrayVec<[MoveInstance; 4]>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MoveInstance {
    #[serde(rename = "move")]
    pub move_ref: MoveRef,
    pub pp: PP,
}

impl MoveInstance {
    pub fn new(move_ref: MoveRef) -> Self {
        Self {
            pp: move_ref.pp,
            move_ref,
        }
    }

    pub fn get(&self) -> Option<MoveRef> {
        (self.pp != 0).then(|| self.move_ref)
    }

    pub fn decrement(&mut self) {
        self.pp = self.pp.saturating_sub(1);
    }

    pub fn empty(&self) -> bool {
        self.pp == 0
    }

    pub fn restore(&mut self) {
        self.pp = self.move_ref.pp;
    }
}
