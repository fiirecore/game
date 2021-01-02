use crate::entity::entity::Ticking;

pub trait BattleTransitionManager: Ticking {

    fn is_finished(&self) -> bool;

}