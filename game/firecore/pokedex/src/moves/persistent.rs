use serde::{Deserialize, Serialize};
use super::script::MoveAction;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersistentMove {

    pub remaining: Option<u8>,

    pub actions: Vec<MoveAction>,

}