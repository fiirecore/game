use macroquad::rand::gen_range;
use parking_lot::Mutex;

use crate::battle::battle_info::BattleType;
use crate::io::data::pokemon::StatSet;
use crate::io::data::pokemon::pokemon_party::PokemonParty;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;
use crate::world::npc::NPC;
use crate::world::pokemon::wild_pokemon_table::WildPokemonTable;

lazy_static::lazy_static! {
	pub static ref BATTLE_DATA: Mutex<Option<BattleData>> = Mutex::new(None);
}

pub fn random_wild_battle() {
    *BATTLE_DATA.lock() = Some(BattleData {
        battle_type: BattleType::Wild,
        party: PokemonParty {
            pokemon: vec![SavedPokemon::generate(gen_range(0, crate::game::pokedex::pokedex::LENGTH) + 1, 1, 100, Some(StatSet::iv_random()), None)],
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
        let mut name = trainer.trainer_type.to_string().to_string();
        name.push(' ');
        name.push_str(npc.identifier.name.as_str());
        *BATTLE_DATA.lock() = Some(BattleData {
            battle_type: trainer.trainer_type.battle_type(),
            party: trainer.party.clone(),
            trainer_data: Some(TrainerData {
                name: name,
                sprite_id: npc.identifier.sprite,
            }),
        });
    }        
}

#[derive(Clone)]
pub struct BattleData {

    pub battle_type: BattleType,
    pub party: PokemonParty,
    pub trainer_data: Option<TrainerData>,

}

impl Default for BattleData {
    fn default() -> Self {
        Self {
            battle_type: BattleType::Wild,
            party: PokemonParty {
                pokemon: Vec::with_capacity(0),
            },
            trainer_data: None,
        }
    }
}

#[derive(Clone)]
pub struct TrainerData {

    pub name: String,
    pub sprite_id: u8,

}