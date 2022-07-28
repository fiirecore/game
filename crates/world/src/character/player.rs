use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use pokedex::pokemon::owned::SavedPokemon;

use crate::{map::battle::BattleEntry, positions::Location};

use super::{
    npc::{
        trainer::{BadgeId, NpcTrainer},
        NpcId,
    },
    CharacterState,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerCharacter {
    pub name: String,
    pub character: CharacterState,

    /// Player State
    #[serde(default)]
    pub battle: GlobalBattleState,
    #[serde(default)]
    pub badges: HashSet<BadgeId>,

    pub cooldown: f32,
    pub rival: String,
}

pub type Battled = HashSet<NpcId>;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalBattleState {
    pub battled: HashMap<Location, Battled>,
    pub battling: Option<BattleEntry<SavedPokemon>>,
}

impl PlayerCharacter {
    pub fn new(name: impl Into<String>, rival: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            character: CharacterState::default(),
            battle: Default::default(),
            badges: Default::default(),
            cooldown: Default::default(),
            rival: rival.into(),
        }
    }

    pub fn find_battle(
        &mut self,
        map: &Location,
        id: &NpcId,
        trainer: &NpcTrainer,
        character: &mut CharacterState,
    ) -> bool {
        if !self.battle.battled(map, id) && trainer.find_character(character, &mut self.character) {
            self.character.locked.increment();
            true
        } else {
            false
        }
    }

    pub fn update(
        &mut self,
        message: &mut crate::state::map::MapMessage,
        delta: f32,
    ) -> Option<super::DoMoveResult> {
        let i = match self.character.do_move(delta) {
            Some(action) => match action {
                super::DoMoveResult::Interact => {
                    if !self.character.input_lock.active() {
                        Some(super::DoMoveResult::Interact)
                    } else {
                        None
                    }
                }
                other => Some(other),
            },
            None => None,
        };

        if let text::MessageStates::Finished(cooldown) = message {
            *cooldown -= delta;
            if *cooldown <= 0.0 {
                *message = text::MessageStates::None;
                self.character.input_lock.decrement();
            }
        }

        i
    }
}

impl GlobalBattleState {
    pub fn insert(&mut self, location: &Location, npc: NpcId) {
        if let Some(battled) = self.battled.get_mut(location) {
            battled.insert(npc);
        } else {
            let mut battled = HashSet::with_capacity(1);
            battled.insert(npc);
            self.battled.insert(*location, battled);
        }
    }

    pub fn battled(&self, map: &Location, npc: &NpcId) -> bool {
        self.battled
            .get(map)
            .map(|battled| battled.contains(npc))
            .unwrap_or_default()
    }
}

// impl SavedPlayerCharacter {
//     pub fn init<
//     >(
//         self,
//         random: &mut impl rand::Rng,
//         pokedex: &impl Dex<Pokemon, Output = P>,
//         movedex: &impl Dex<Move, Output = M>,
//         itemdex: &impl Dex<Item, Output = I>,
//     ) -> Option<InitPlayerCharacter> {
//         Some(PlayerCharacter {
//             name: self.name,
//             character: self.character,
//             trainer: self.trainer.init(random, pokedex, movedex, itemdex)?,
//             battle: self.battle,
//             badges: self.badges,
//             cooldown: self.cooldown,
//             rival: self.rival,
//         })
//     }
// }

// impl<
//     > InitPlayerCharacter
// {
//     pub fn uninit(self) -> SavedPlayerCharacter {
//         SavedPlayerCharacter {
//             name: self.name,
//             character: self.character,
//             trainer: self.trainer.uninit(),
//             battle: self.battle,
//             badges: self.badges,
//             cooldown: self.cooldown,
//             rival: self.rival,
//         }
//     }
// }

impl Default for PlayerCharacter {
    fn default() -> Self {
        Self {
            name: "Red".into(),
            character: Default::default(),
            battle: Default::default(),
            badges: Default::default(),
            cooldown: Default::default(),
            rival: "Blue".into(),
        }
    }
}
