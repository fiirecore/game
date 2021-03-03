#[derive(Clone, Copy)]
pub struct BattleInfo {

    pub battle_type: BattleType,

}

#[derive(Clone, Copy, PartialEq)]
pub enum BattleType {

    Wild,
    Trainer,
    GymLeader,

}

impl Default for BattleType {
    fn default() -> Self {
        Self::Wild
    }
}