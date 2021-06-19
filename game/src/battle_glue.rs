use deps::str::{TinyStr8, TinyStr16};
use pokedex::{
    trainer::TrainerData,
    pokemon::party::PokemonParty,
};

use deps::tetra::graphics::Texture;

/***********************/

pub type BattleEntryRef<'a> = &'a mut Option<BattleEntry>;

pub struct BattleEntry {
    pub size: usize,
    pub party: PokemonParty,
    pub trainer: Option<BattleTrainerEntry>,
    pub trainer_data: Option<TrainerData>,
}

pub struct BattleTrainerEntry {
    pub id: TinyStr16,
    pub transition: TinyStr8,
    pub texture: Texture,
    pub gym_badge: Option<TinyStr16>,
    pub victory_message: Vec<Vec<String>>,
    pub worth: u16,
}