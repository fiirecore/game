use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

use pokedex::pokemon::owned::SavedPokemon;

use crate::{positions::{Location, Position}, state::WorldState, map::warp::WarpDestination};

use super::{
    npc::{trainer::NpcTrainer, NpcId},
    trainer::Trainer,
    Character,
};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub location: Location,
    pub character: Character,
    pub trainer: Trainer,

    pub pc: Vec<SavedPokemon>,
    pub world: WorldState,

    pub rival: String,
    pub input_frozen: bool,
    pub ignore: bool,
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

    pub fn new(name: impl Into<String>, rival: impl Into<String>, spawn: (Location, Position)) -> Self {
        Self {
            location: spawn.0,
            character: Character {
                name: name.into(),
                position: spawn.1,
                ..Default::default()
            },
            trainer: Default::default(),
            pc: Default::default(),
            world: Default::default(),
            rival: rival.into(),
            input_frozen: false,
            ignore: false,
        }
    }

    pub fn warp(&mut self, destination: WarpDestination) {
        self.position.from_destination(destination.position);
        self.pathing.clear();
        self.location = destination.location;
        self.position.elevation = 0;
    }

    pub fn find_battle(
        &mut self,
        map: &Location,
        id: &NpcId,
        trainer: &NpcTrainer,
        character: &mut Character,
    ) -> bool {
        if self.world.npc.active.is_none()
            && !self.world.battle.battled(map, id)
            && trainer.find_character(character, &mut self.character)
        {
            self.world.npc.active = Some(*id);
            true
        } else {
            false
        }
    }

    /// does not cover cases where pokemon cannot be sent to pc
    pub fn give_pokemon(&mut self, pokemon: SavedPokemon) {
        match self.trainer.party.is_full() {
            true => self.pc.push(pokemon),
            false => self.trainer.party.push(pokemon),
        }
    }
}
