use firecore_data::{get, player::PlayerSaves};
use firecore_util::battle::BattleType;
use firecore_world::character::npc::NPCId;
use firecore_world::character::npc::NPCIdentifier;
use firecore_world::map::wild::WildEntry;
use firecore_world::{
    map::wild::{
        GenerateWild,
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
use crate::battle::BATTLE_RANDOM;
use smallvec::{smallvec, SmallVec};

use crate::world::NPCTypes;

pub type BattlePokemonParty = SmallVec<[PokemonInstance; 6]>;

pub struct BattleData {

    pub party: BattlePokemonParty,
    pub trainer: Option<TrainerData>,

}

pub struct TrainerData {

    pub identifier: NPCIdentifier,
    pub npc_type: NPCType,
    pub victory_message: Vec<Vec<String>>,
    pub disable_others: ahash::AHashSet<NPCId>,
    pub worth: u16,
    pub map: String,

}

impl BattleData {

    pub fn get_type(&self) -> BattleType {
        self.trainer.as_ref().map(|data| data.npc_type.trainer.as_ref().unwrap().battle_type).unwrap_or_default()
    }

}

pub fn random_wild_battle(battle_data: &mut Option<BattleData>) {
    *battle_data = Some(BattleData {
        party: smallvec![PokemonInstance::generate(
            BATTLE_RANDOM.gen_range(0..firecore_pokedex::pokedex().len() as u32) as PokemonId + 1, 
            1, 
            100, 
            Some(StatSet::random())
        )],
        trainer: None,
    });
}

pub fn wild_battle(battle_data: &mut Option<BattleData>, wild: &WildEntry) {
    if wild.table.try_encounter() {
        *battle_data = Some(BattleData {
            party: smallvec![wild.table.generate()],
            trainer: None,
        });
    }
}

pub fn trainer_battle(battle_data: &mut Option<BattleData>, map_name: &String, npc: &NPC, npc_types: &NPCTypes) {
    if let Some(trainer) = npc.trainer.as_ref() {
        if let Some(saves) = get::<PlayerSaves>() {
            let save = saves.get();
            if let Some(map) = save.world_status.map_data.get(map_name) {
                if !map.battled.contains(&npc.identifier.index) {
                    *battle_data = Some(
                        BattleData {
                            party: to_battle_party(&trainer.party),
                            trainer: Some(
                                TrainerData {
                                    identifier: npc.identifier.clone(),
                                    npc_type: npc_types.get(&npc.identifier.npc_type).map(|npc_type| npc_type.clone()).unwrap(),
                                    victory_message: trainer.victory_message.clone(),
                                    disable_others: trainer.disable_others.clone(),
                                    worth: trainer.worth,
                                    map: map_name.clone(),
                                }
                            )
                        }
                    );
                    // macroquad::prelude::info!("Trainer battle with {}", npc.identifier.name);
                }
            }
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