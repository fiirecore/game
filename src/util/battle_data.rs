use firecore_world::character::npc::NPC;
use firecore_world::character::npc::npc_type::NPCType;
use macroquad::rand::gen_range;
use parking_lot::Mutex;
use firecore_pokedex::PokemonId;
use firecore_pokedex::pokemon::data::StatSet;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_world::map::wild::table::WildPokemonTable;

lazy_static::lazy_static! {
	pub static ref BATTLE_DATA: Mutex<Option<BattleData>> = Mutex::new(None);
}

// #[derive(Default)]
// pub struct BattleDataContainer {
//     pub data: Option<BattleData>
// }

#[derive(Default)]
pub struct BattleData {

    pub party: PokemonParty,
    pub trainer_data: Option<TrainerData>,

}

pub struct TrainerData {

    pub npc_name: String,
    pub npc_data: dashmap::mapref::one::Ref<'static, String, NPCType>,

}

pub fn random_wild_battle() {
    *BATTLE_DATA.lock() = Some(BattleData {
        party: PokemonParty {
            pokemon: smallvec::smallvec![firecore_pokedex::pokemon::instance::PokemonInstance::generate(gen_range(0, firecore_pokedex::POKEDEX.len()) as PokemonId + 1, 1, 100, Some(StatSet::iv_random()))],
        },
        trainer_data: None,
    });
}

pub fn wild_battle(table: &WildPokemonTable) {
    *BATTLE_DATA.lock() = Some(BattleData {
        party: PokemonParty {
            pokemon: smallvec::smallvec![table.generate()],
        },
        trainer_data: None,
    });
}

pub fn trainer_battle(npc: &NPC) {
    if let Some(trainer) = npc.trainer.as_ref() {
        *BATTLE_DATA.lock() = Some( BattleData {
            party: trainer.party.clone(),
            trainer_data: Some(TrainerData {
                npc_name: npc.identifier.name.clone(),
                npc_data: crate::world::npc::NPC_TYPES.get(&npc.identifier.npc_type).unwrap(),
            })
        });
        macroquad::prelude::info!("Trainer battle with {}", npc.identifier.name);
    }
}