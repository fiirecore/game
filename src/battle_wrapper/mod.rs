use std::ops::Deref;

use battlecli::{
    battle::{
        engine::BattleEngine,
        pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
        prelude::{Battle, BattleAi},
    },
    BattleTrainer,
};

use rand::{prelude::SmallRng, Rng};

use worldcli::worldlib::map::battle::{BattleId, TrainerEntry};

mod manager;

pub use manager::*;

pub struct GameBattleWrapper {
    battle: Battle<BattleId, BattleTrainer>,
    ai: BattleAi<BattleId, SmallRng, BattleTrainer>,
    trainer: Option<TrainerEntry>,
}

impl GameBattleWrapper {
    pub fn update<'d>(
        &mut self,
        random: &mut (impl Rng + Clone + 'static),
        engine: &mut impl BattleEngine,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) {
        self.battle.update(random, engine, movedex, itemdex);
        self.ai.update(pokedex, movedex, itemdex);
    }
}
