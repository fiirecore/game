use std::ops::Deref;

use crate::pokedex::{
    item::Item,
    moves::Move,
    pokemon::{owned::OwnedPokemon, Pokemon},
    Dex, Uninitializable,
};
use battlelib::prelude::{
    Battle, BattleAi, BattleData, BattleType, DefaultMoveEngine, PlayerData, PlayerSettings,
};
use firecore_battle::endpoint::MpscEndpoint;
use rand::{prelude::SmallRng, Rng, SeedableRng};
use worldlib::character::player::PlayerCharacter;
use crate::saves::OwnedPlayer;

use crate::game::battle_glue::{BattleEntry, BattleId, BattleTrainerEntry};

mod manager;

pub use manager::*;

pub struct GameBattleWrapper<
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    pub battle: Option<Battle<BattleId, P, M, I>>,
    pub ai: Option<BattleAi<SmallRng, BattleId, P, M, I>>,
    pub trainer: Option<BattleTrainerEntry>,
    pub engine: DefaultMoveEngine,
    pub seed: u64,
}

impl<
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > GameBattleWrapper<P, M, I>
{
    pub fn new() -> Self {
        let mut engine = DefaultMoveEngine::new::<BattleId, SmallRng>();

        let (bmoves, scripts) =
            bincode::deserialize(include_bytes!("../../build/data/battle.bin")).unwrap();

        engine.moves = bmoves;

        engine.scripting.scripts = scripts;

        Self {
            battle: None,
            ai: None,
            trainer: None,
            engine,
            seed: 0,
        }
    }

    pub fn battle<'d>(
        &mut self,
        random: &mut impl Rng,
        pokedex: &'d dyn Dex<'d, Pokemon, P>,
        movedex: &'d dyn Dex<'d, Move, M>,
        itemdex: &'d dyn Dex<'d, Item, I>,
        player: &OwnedPlayer<P, M, I>,
        endpoint: &MpscEndpoint<BattleId>,
        entry: BattleEntry,
    ) {
        let player = PlayerData {
            id: BattleId(Some(player.id)),
            name: Some(player.name.clone()),
            party: player
                .party
                .iter()
                .cloned()
                .map(OwnedPokemon::uninit)
                .collect(),
            settings: PlayerSettings { gains_exp: true },
            endpoint: Box::new(endpoint.clone()),
        };

        let p = entry
            .party
            .iter()
            .cloned()
            .flat_map(|o| o.init(random, pokedex, movedex, itemdex))
            .collect();

        let ai = BattleAi::new(
            BattleId::default(),
            SmallRng::seed_from_u64(random.next_u64()),
            entry.active,
            p,
        );

        let ai_player = PlayerData {
            id: entry.id,
            name: entry.name,
            party: entry.party,
            settings: PlayerSettings { gains_exp: false },
            endpoint: Box::new(ai.endpoint().clone()),
        };

        self.battle = Some(Battle::new(
            BattleData {
                type_: entry
                    .trainer
                    .as_ref()
                    .map(|trainer| {
                        if trainer.gym_badge.is_some() {
                            BattleType::GymLeader
                        } else {
                            BattleType::Trainer
                        }
                    })
                    .unwrap_or(BattleType::Wild),
            },
            random,
            entry.active,
            pokedex,
            movedex,
            itemdex,
            std::iter::once(player).chain(std::iter::once(ai_player)),
        ));
        self.trainer = entry.trainer;
        self.ai = Some(ai);
    }

    pub fn update_data(&mut self, winner: bool, player: &mut PlayerCharacter) -> bool {
        let trainer = self.trainer.is_some();

        if winner {
            if let Some(trainer) = self.trainer.take() {
                player.worth += trainer.worth as u32;
                if let Some(badge) = trainer.gym_badge {
                    player.world.badges.insert(badge);
                }
            }
        }

        trainer
    }
}
