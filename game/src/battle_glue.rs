use deps::str::TinyStr8;
use pokedex::pokemon::party::PokemonParty;

use deps::tetra::graphics::Texture;

/***********************/

pub type BattleEntryRef<'a> = &'a mut Option<BattleEntry>;

pub struct BattleEntry {
    pub size: usize,
    pub party: PokemonParty,
    pub trainer: Option<BattleTrainerEntry>,
}

pub struct BattleTrainerEntry {
    pub prefix: String,
    pub name: String,
    pub transition: TinyStr8,
    pub texture: Texture,
    pub gym_badge: Option<deps::str::TinyStr16>,
    pub victory_message: Vec<Vec<String>>,
    pub worth: u16,
}