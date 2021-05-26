use deps::hash::HashMap;
use super::{Move, MoveId};

pub type Movedex = HashMap<MoveId, Move>;

pub(crate) static mut MOVEDEX: Option<Movedex> = None;

pub fn set(dex: Movedex) {
    unsafe { MOVEDEX = Some(dex) }
}