use firecore_util::battle::BattleType;
use firecore_world::{
    map::wild::{
        GenerateWild,
        table::WildPokemonTable,
    },
    character::npc::{
        NPC, 
        npc_type::NPCType
    }
};

use firecore_pokedex::pokemon::{
    PokemonId,
    party::PokemonParty,
    instance::PokemonInstance,
    data::StatSet,
    random::RandomSet,
    generate::GeneratePokemon,
};

use macroquad::prelude::warn;
use macroquad::rand::gen_range;
use smallvec::{smallvec, SmallVec};

use crate::world::NPCTypes;

pub static mut BATTLE_DATA: Option<BattleData> = None;

pub type BattlePokemonParty = SmallVec<[PokemonInstance; 6]>;

pub struct BattleData {

    pub party: BattlePokemonParty,
    pub trainer: Option<TrainerData>,

}

impl BattleData {

    pub fn get_type(&self) -> BattleType {
        self.trainer.as_ref().map(|data| data.npc_type.trainer.as_ref().unwrap().battle_type).unwrap_or_default()
    }

}

pub struct TrainerData {

    pub name: String,
    pub npc_type: NPCType,
    pub npc_type_id: String,

}

pub fn random_wild_battle() {
    unsafe {
        BATTLE_DATA = Some(BattleData {
            party: smallvec![PokemonInstance::generate(
                gen_range(0, firecore_pokedex::pokedex().len()) as PokemonId + 1, 
                1, 
                100, 
                Some(StatSet::random())
            )],
            trainer: None,
        });
    }
}

pub fn wild_battle(table: &WildPokemonTable) {
    unsafe {
        BATTLE_DATA = Some(BattleData {
            party: smallvec![table.generate()],
            trainer: None,
        });
    }
}

pub fn trainer_battle(npc: &NPC, npc_types: &NPCTypes) {
    if let Some(trainer) = npc.trainer.as_ref() {
        unsafe {
            BATTLE_DATA = Some( BattleData {
                party: to_battle_party(&trainer.party),
                trainer: Some(TrainerData {
                    name: npc.identifier.name.clone(),
                    npc_type_id: npc.identifier.npc_type.clone(),
                    npc_type: npc_types.get(&npc.identifier.npc_type).map(|npc_type| npc_type.clone()).unwrap(),
                })
            });
            macroquad::prelude::info!("Trainer battle with {}", npc.identifier.name);
        }
    }
}

pub fn to_battle_party(party: &PokemonParty) -> BattlePokemonParty {
    let mut battle_party = BattlePokemonParty::new();
    for pokemon in party {
        if let Some(pokemon) = PokemonInstance::new(pokemon) {
            battle_party.push(pokemon)
        } else {
            warn!("Could not create battle pokemon from ID {}", pokemon.id);
        }
    }
    battle_party
}