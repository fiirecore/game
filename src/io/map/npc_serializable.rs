use serde_derive::Deserialize;
use crate::io::data::player_data::SavedPokemon;

#[derive(Deserialize)]
pub struct JsonNPC {

    pub identifier: NPCIdentifier,
    pub location: NPCLocation,    
    pub trainer: Option<NPCTrainer>,

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

#[derive(Deserialize)]
pub struct NPCTrainer {

    pub pokemon: Vec<SavedPokemon>,

}