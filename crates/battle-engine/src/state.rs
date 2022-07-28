use std::ops::Deref;

use battle::prelude::EndMessage;
use pokengine::pokedex::moves::Move;

use crate::action::MoveQueue;

#[derive(Debug)]
pub enum BattlePlayerState<ID> {
    WaitToStart,
    Opening,
    // Introduction(TransitionState),
    WaitToSelect,
    Select,
    Moving(MoveQueue<ID>),
    Lose(EndMessage),
    Closing(Option<ID>),
    End(Option<ID>),
}

impl<ID> BattlePlayerState<ID> {
    // #[cfg(debug_assertions)]
    // pub fn name(&self) -> &str {
    //     match self {
    //         BattlePlayerState::WaitToStart => "wait to start",
    //         BattlePlayerState::Opening(_) => "opening",
    //         BattlePlayerState::Introduction(_) => "introduction",
    //         BattlePlayerState::WaitToSelect => "wait to select",
    //         BattlePlayerState::Select => "select",
    //         BattlePlayerState::Moving(_) => "moving",
    //         BattlePlayerState::Lose(..) => "lose",
    //         BattlePlayerState::End(_) => "end",
    //         BattlePlayerState::Closing(..) => "closing",
    //     }
    // }
}
