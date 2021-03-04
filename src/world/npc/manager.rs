use crate::world::npc::NPC;

#[derive(Default)]
pub struct MapNpcManager {

    pub npcs: Vec<NPC>,
    
    // #[serde(skip)]
    pub npc_active: Option<usize>,

}