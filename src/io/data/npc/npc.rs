use serde::Deserialize;

use crate::io::data::npc::trainer::Trainer;

#[derive(Deserialize)]
pub struct JsonNPC {

    pub identifier: NPCIdentifier,
    pub location: NPCLocation,    
    pub trainer: Option<Trainer>,

}

#[derive(Debug, Deserialize)]
pub struct NPCIdentifier {

    pub name: String,
    pub sprite: u8,

}

#[derive(Debug, Deserialize)]
pub struct NPCLocation {

    pub x: isize,
    pub y: isize,
    pub direction: u8,

}