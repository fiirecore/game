use firecore_util::battle::BattleType;

use firecore_pokedex::pokemon::{
    saved::SavedPokemonParty,
    instance::{
        PokemonInstance,
        PokemonInstanceParty,
    },
};

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

// pub fn random_wild_battle(battle_data: &mut Option<BattleData>) {
//     *battle_data = Some(BattleData {
//         party: smallvec![PokemonInstance::generate(
//             BATTLE_RANDOM.gen_range(0..firecore_pokedex::pokedex().len() as u32) as PokemonId + 1, 
//             1, 
//             100, 
//             Some(StatSet::random())
//         )],
//         trainer: None,
//     });
// }

// pub fn wild_battle(battle_data: &mut Option<BattleData>, wild: &WildEntry) {
//     if wild.table.try_encounter() {
//         *battle_data = Some(BattleData {
//             party: smallvec![wild.table.generate()],
//             trainer: None,
//         });
//     }
// }

// pub fn trainer_battle(battle_data: &mut Option<BattleData>, map_name: &String, npc: &NPC) {
//     if let Some(trainer) = npc.trainer.as_ref() {
//         if let Some(saves) = get::<PlayerSaves>() {
//             let save = saves.get();
//             if let Some(map) = save.world_status.map_data.get(map_name) {
//                 if !map.battled.contains(&npc.identifier.index) {
//                     *battle_data = Some(
//                         BattleData {
//                             party: to_battle_party(&trainer.party),
//                             trainer: Some(
//                                 TrainerData {
//                                     identifier: npc.identifier.clone(),
//                                     npc_type: crate::npc::npc_type(&npc.identifier.npc_type).map(|npc_type| npc_type.trainer.as_ref().map(|trainer| trainer.name.clone())).flatten().unwrap_or(String::from("Trainer")),
//                                     texture: crate::textures::trainer_texture(&npc.identifier.npc_type),
//                                     victory_message: trainer.victory_message.clone(),
//                                     disable_others: trainer.disable_others.clone(),
//                                     worth: trainer.worth,
//                                     map: map_name.clone(),
//                                 }
//                             )
//                         }
//                     );
//                     // macroquad::prelude::info!("Trainer battle with {}", npc.identifier.name);
//                 }
//             }
//         }
//     }
// }

#[deprecated]
pub fn to_battle_party(party: &SavedPokemonParty) -> PokemonInstanceParty {
    party.iter().map(|pokemon| PokemonInstance::new(pokemon)).flatten().collect()
}