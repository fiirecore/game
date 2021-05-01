use firecore_util::battle::BattleType;

use firecore_pokedex::pokemon::instance::PokemonInstanceParty;

use macroquad::prelude::Texture2D;

/***********************/

pub struct BattleData {

    pub party: PokemonInstanceParty,
    pub trainer: Option<TrainerData>,

}

pub struct TrainerData {


    pub name: String,
    pub npc_type: String,
    pub battle_type: BattleType,
    pub texture: Texture2D,
    pub victory_message: Vec<Vec<String>>,
    pub worth: u16,

    // use this stuff in world crate

    // pub index: NPCId,
    // pub disable_others: HashSet<NPCId>,
    // pub map: String,

}

#[derive(Debug, Clone, Copy)]
pub enum BattleWinner {

	Player,
	Opponent,

}

impl BattleData {

    pub fn get_type(&self) -> BattleType {
        self.trainer.as_ref().map(|data| data.battle_type).unwrap_or(BattleType::Wild)
    }

}

// #[deprecated]
// pub fn to_battle_party(party: &SavedPokemonParty) -> PokemonInstanceParty {
//     party.iter().map(|pokemon| PokemonInstance::new(pokemon)).flatten().collect()
// }