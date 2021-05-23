use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {

    Paralysis,
    Poison,
    Sleep,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct StatusEffect {

    pub status: Status,
    pub remaining: Option<u8>, // create type alias "Turns"

}