use crate::{
    storage_player::PlayerId,
    pokedex::{
        types::Effective,
        pokemon::{Level, Experience, stat::StatType},
        moves::{
            MoveRef,
            target::MoveTargetInstance,
        },
        item::ItemRef,
    },
};

use super::view::UnknownPokemon;

#[derive(Debug, Clone)]
pub enum BattleMove {

    Move(usize, Vec<MoveTargetInstance>),
    UseItem(ItemRef, MoveTargetInstance),
    Switch(usize),

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActivePokemonIndex {
    pub team: PlayerId,
    pub index: usize,
}

// impl ActivePokemonIndex {

//     pub fn into_other(self) -> Self {
//         Self {
//             team: self.team.other(),
//             index: self.index,
//         }
//     }

// }

impl core::fmt::Display for ActivePokemonIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} #{}", self.team, self.index)
    }
}

#[derive(Debug)]
pub struct BattleActionInstance {
    pub pokemon: ActivePokemonIndex,
    pub action: BattleMove,
}

// #[deprecated]
// #[derive(Debug)]
// pub enum BattleAction {
//     Pokemon(BattleMove),
//     Faint(Option<MoveTargetInstance>), // user that made target faint
//     Catch(MoveTargetInstance),
//     GainExp(Level, Experience),
//     LevelUp(Level, Option<Vec<MoveRef>>),
//     // Wait(f32),
// }

#[derive(Debug, Clone)]
pub struct BattleClientActionInstance {
    pub pokemon: ActivePokemonIndex,
    pub action: BattleClientAction,
}

// impl BattleClientActionInstance {
//     pub fn into_other(self) -> Self {
//         Self {
//             pokemon: self.pokemon.into_other(),
//             action: self.action.into_other(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub enum BattleClientAction {

    Move(MoveRef, Vec<(MoveTargetInstance, Vec<BattleClientMove>)>),
    Switch(usize, Option<UnknownPokemon>),
    UseItem(ItemRef, MoveTargetInstance),

    // maybe move these

    Faint,
    Catch(usize),
    GainExp(Level, Experience),
    LevelUp(Level, Option<Vec<MoveRef>>),
}

impl BattleClientAction {
    pub fn requires_user(&self) -> bool {
        match self {
            BattleClientAction::Faint => false,
            _ => true,
        }
    }
}

// impl BattleClientAction {
//     pub fn into_other(self) -> Self {
//         match self {
//             BattleClientAction::Move(pokemon_move, targets) => Self::Move(pokemon_move, targets.into_iter().map(|(t, mut m)| {
//                 m.iter_mut().for_each(|m| m.as_other());
//                 (t, m)
//             }).collect()),
//             _ => self
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BattleClientMove {
    Miss,
    TargetHP(f32),
    UserHP(f32), // dont heal the target
    Effective(Effective),
    StatStage(StatType, i8),
    Faint(ActivePokemonIndex), // target that is fainting
    Fail,
}

// impl BattleClientMove {
//     fn as_other(&mut self) {
//         match self {
//             BattleClientMove::Faint(i) => *i = i.into_other(),
//             _ => (),
//         }
//     }
// }