use oorandom::Rand32;

use crate::io::data::pokemon::StatSet;
use crate::io::data::pokemon::pokemon_party::PokemonParty;
use crate::io::data::pokemon::saved_pokemon::SavedPokemon;
use crate::world::npc::NPC;
use crate::world::pokemon::wild_pokemon_table::WildPokemonTable;

use super::battle_info::BattleType;


#[derive(Clone)]
pub struct BattleContext {

    pub battle: bool,
    pub battle_data: Option<BattleData>,

}

impl BattleContext {

    pub fn empty() -> Self {

        Self {

            battle: false,
            battle_data: None,

        }

    }

    pub fn reset(&mut self) {
        self.battle = false;
    }

    pub fn random_wild_battle(&mut self, random: &mut Rand32) {
        self.battle = true;

        let id = random.rand_range(0..crate::game::pokedex::pokedex::LENGTH as u32) as usize + 1;
        let ivs = StatSet::iv_random(random);

        self.battle_data = Some(BattleData {
            battle_type: BattleType::Wild,
            party: PokemonParty {
                pokemon: vec![SavedPokemon::generate(random, id, 1, 100, Some(ivs), None)],
            },
            trainer_data: None,
        });
    }

    pub fn wild_battle(&mut self, random: &mut Rand32, table: &Box<dyn WildPokemonTable>) {
        self.battle = true;
        self.battle_data = Some(BattleData {
            battle_type: BattleType::Wild,
            party: PokemonParty {
                pokemon: vec![table.generate(random)],
            },
            trainer_data: None,
        });
    }

    pub fn trainer_battle(&mut self, npc: &NPC) {
        if let Some(trainer) = &npc.trainer {
            self.battle = true;
            let mut name = trainer.trainer_type.to_string().to_string();
            name.push(' ');
            name.push_str(npc.identifier.name.as_str());
            self.battle_data = Some(BattleData {
                battle_type: trainer.trainer_type.battle_type(),
                party: trainer.party.clone(),
                trainer_data: Some(TrainerData {
                    name: name,
                    sprite_id: npc.identifier.sprite,
                }),
            });
        }        
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