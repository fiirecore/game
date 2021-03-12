use macroquad::rand::gen_range;
use parking_lot::Mutex;
use firecore_world::{BattleType, BattleScreenTransitions};
use firecore_pokedex::PokemonId;
use firecore_pokedex::pokemon::data::StatSet;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_world::npc::NPC;
use firecore_world::pokemon::wild_pokemon_table::WildPokemonTable;

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
            pokemon: firecore_pokedex::smallvec![firecore_pokedex::pokemon::instance::PokemonInstance::generate(gen_range(0, firecore_pokedex::POKEDEX.len()) as PokemonId + 1, 1, 100, Some(StatSet::iv_random()))],
        },
        trainer_data: None,
    });
}

pub fn wild_battle(table: &WildPokemonTable) {
    *BATTLE_DATA.lock() = Some(BattleData {
        battle_type: BattleType::Wild,
        party: PokemonParty {
            pokemon: firecore_pokedex::smallvec![table.generate()],
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
                name: trainer.trainer_type.to_string().to_string() + " " + &npc.identifier.name,
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