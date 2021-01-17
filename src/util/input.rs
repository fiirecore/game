use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Control {

    A/*(u8)*/,
    B/*(u8)*/,
    Up/*(u8)*/,
    Down/*(u8)*/,
    Left/*(u8)*/,
    Right/*(u8)*/,
    Start/*(u8)*/,
    Select/*(u8)*/,
    Escape/*(u8)*/,

}