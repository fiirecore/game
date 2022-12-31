use battle::prelude::EndMessage;

use crate::action::MoveQueue;

// pub type BattleClientState = Option<BattlePlayerState>;



impl<ID> BattleClientState<ID> {
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
