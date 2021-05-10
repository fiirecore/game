use serde::{Deserialize, Serialize};

mod condition;
pub use condition::*;
mod action;
pub use action::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MoveScript {

    // pub conditions: Vec<MoveCondition>,

    pub action: MoveAction,

}