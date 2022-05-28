use std::ops::Deref;

use battlecli::{
    battle::{
        item::engine::ItemEngine,
        moves::engine::MoveEngine,
        pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
        prelude::{Battle, BattleAi},
    },
    BattleTrainer,
};

use rand::{prelude::SmallRng, Rng};

use worldcli::worldlib::map::battle::{BattleId, TrainerEntry};

mod manager;

pub use manager::*;

pub struct GameBattleWrapper<
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    battle: Battle<BattleId, BattleTrainer, P, M, I>,
    ai: BattleAi<BattleId, BattleTrainer, SmallRng, P, M, I>,
    trainer: Option<TrainerEntry>,
}

impl<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > GameBattleWrapper<P, M, I>
{
    pub fn update<'d>(
        &mut self,
        random: &mut (impl Rng + Clone + 'static),
        engine: &mut (impl MoveEngine + ItemEngine),
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
    ) {
        self.battle.update(random, engine, movedex, itemdex);
        self.ai.update(pokedex, movedex, itemdex);
    }
}
