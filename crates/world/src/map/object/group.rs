use serde::{Deserialize, Serialize};

use pokedex::moves::MoveId;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ObjectGroup {
    pub solid: bool,
    pub destroy: Option<ObjectDestroy>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ObjectDestroy {
    pub regeneratable: bool,
    pub method: DestroyMethod,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DestroyMethod {
    Move(MoveId),
}
