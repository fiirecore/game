use firecore_world::npc::trainer::Trainer;
use macroquad::rand::gen_range;
use parking_lot::Mutex;
use firecore_world::{BattleType, BattleScreenTransitions};
use firecore_pokedex::PokemonId;
use firecore_pokedex::pokemon::data::StatSet;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_world::wild::table::WildPokemonTable;
use firecore_world::npc::trainer::TrainerData;
use firecore_world::BattleData;

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
            pokemon: smallvec::smallvec![firecore_pokedex::pokemon::instance::PokemonInstance::generate(gen_range(0, firecore_pokedex::POKEDEX.len()) as PokemonId + 1, 1, 100, Some(StatSet::iv_random()))],
        },
        trainer_data: None,
    });
}

pub fn wild_battle(table: &WildPokemonTable) {
    *BATTLE_DATA.lock() = Some(BattleData {
        battle_type: BattleType::Wild,
        party: PokemonParty {
            pokemon: smallvec::smallvec![table.generate()],
        },
        trainer_data: None,
    });
}

pub fn trainer_battle(trainer: &Trainer, name: &str, npc_type: &String) {
    *BATTLE_DATA.lock() = Some(BattleData {
        battle_type: trainer.trainer_type.battle_type(),
        party: trainer.party.clone(),
        trainer_data: Some(TrainerData {
            name: trainer.trainer_type.to_string().to_string() + " " + name,
            npc_type: npc_type.clone(),
            transition: trainer.battle_transition.unwrap_or(BattleScreenTransitions::Trainer),        
        }),
    });     
}