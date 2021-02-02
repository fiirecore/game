#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BattleInfo {

    pub battle_type: BattleType,

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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