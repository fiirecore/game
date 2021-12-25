use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use crate::{positions::Location, map::manager::state::WorldMapState};

use super::{Character, npc::{NpcId, Npc}, trainer::Trainer};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub location: Location,
    pub character: Character,
    pub trainer: Trainer,
    pub rival: String,
    pub input_frozen: bool,
    pub ignore: bool,
    
    pub world: WorldMapState,
}

impl Deref for PlayerCharacter {
    type Target = Character;

    fn deref(&self) -> &Self::Target {
        &self.character
    }
}

impl DerefMut for PlayerCharacter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.character
    }
}

impl PlayerCharacter {

    pub fn find_battle(&mut self, map: &Location, id: &NpcId, npc: &mut Npc) -> bool {
        if self.world.npc.active.is_none()
            && !self.world.battle.battled(map, id)
            && npc.find_character(&mut self.character)
        {
            self.world.npc.active = Some(*id);
            true
        } else {
            false
        }
    }

}