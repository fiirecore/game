use serde_derive::Deserialize;

use crate::io::data::trainer::Trainer;


#[derive(Deserialize)]
pub struct JsonNPC {

    pub identifier: NPCIdentifier,
    pub location: NPCLocation,    
    pub trainer: Option<Trainer>,

}

#[derive(Debug, Deserialize)]
pub struct NPCIdentifier {

    pub npc_type: String,
    pub sprite: u8,

}

#[derive(Debug, Deserialize)]
pub struct NPCLocation {

    pub x: isize,
    pub y: isize,
    pub direction: u8,

}