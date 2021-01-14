#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct BattleInfo {

    pub battle_type: BattleType,

}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum BattleType {

    Wild,
    Trainer,
    GymLeader,

}