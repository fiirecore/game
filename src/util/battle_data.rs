use macroquad::rand::gen_range;
use parking_lot::Mutex;
use crate::battle::battle_info::BattleType;
use crate::battle::transitions::managers::battle_screen_transition_manager::BattleScreenTransitions;
use firecore_pokedex::PokemonId;
use firecore_pokedex::data::StatSet;
use firecore_pokedex::party::PokemonParty;
use crate::world::npc::NPC;
use crate::world::pokemon::wild_pokemon_table::WildPokemonTable;

lazy_static::lazy_static! {
	pub static ref BATTLE_DATA: Mutex<Option<BattleData>> = Mutex::new(None);
}

// #[derive(Default)]
// pub struct BattleDataContainer {
//     pub data: Option<BattleData>
// }

pub fn random_wild_battle() {
    *BATTLE_DATA.lock() = Some(BattleData {
        battle_type: BattleType::Wild,
        party: PokemonParty {
            pokemon: vec![firecore_pokedex::instance::PokemonInstance::generate(gen_range(0, firecore_pokedex::POKEDEX.len()) as PokemonId + 1, 1, 100, Some(StatSet::iv_random()))],
        },
        trainer_data: None,
    });
}

pub fn wild_battle(table: &WildPokemonTable) {
    *BATTLE_DATA.lock() = Some(BattleData {
        battle_type: BattleType::Wild,
        party: PokemonParty {
            pokemon: vec![table.generate()],
        },
        trainer_data: None,
    });
}

pub fn trainer_battle(npc: &NPC) {
    if let Some(trainer) = &npc.trainer {
        *BATTLE_DATA.lock() = Some(BattleData {
            battle_type: trainer.trainer_type.battle_type(),
            party: trainer.party.clone(),
            trainer_data: Some(TrainerData {
                name: trainer.trainer_type.to_string().to_string() + " " + npc.identifier.name.as_str(),
                npc_type: npc.identifier.npc_type.clone(),
                transition: trainer.battle_transition.unwrap_or(BattleScreenTransitions::Trainer),        
            }),
        });
    }        
}

#[derive(Default)]
pub struct BattleData {

    pub battle_type: BattleType,
    pub party: PokemonParty,
    pub trainer_data: Option<TrainerData>,

}

//#[derive(Clone)]
pub struct TrainerData {

    pub name: String,
    pub npc_type: String,
    pub transition: BattleScreenTransitions,

}