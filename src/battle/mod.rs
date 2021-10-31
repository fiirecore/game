use battlelib::prelude::{
    Battle, BattleAi, BattleData, BattleType, DefaultMoveEngine, PlayerData, PlayerSettings,
};
use firecore_battle::endpoint::MpscEndpoint;
use pokedex::{
    item::Item,
    moves::Move,
    pokemon::{owned::OwnedPokemon, Pokemon},
    Dex, Uninitializable,
};
use rand::{prelude::SmallRng, Rng, SeedableRng};
use saves::PlayerData as PD;

use crate::game::battle_glue::{BattleEntry, BattleId, BattleTrainerEntry};

mod manager;

pub use manager::*;

pub struct GameBattleWrapper<'d> {
    pub battle: Option<Battle<'d, BattleId>>,
    pub ai: Option<BattleAi<'d, SmallRng, BattleId>>,
    pub trainer: Option<BattleTrainerEntry>,
    pub engine: DefaultMoveEngine,
    pub seed: u64,
}

impl<'d> GameBattleWrapper<'d> {

    pub fn new() -> Self {

        let mut engine = DefaultMoveEngine::new::<BattleId, SmallRng>();

        let (bmoves, scripts) = bincode::deserialize(include_bytes!("../../build/data/battle.bin")).unwrap();

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

    pub fn battle(
        &mut self,
        random: &mut impl Rng,
        pokedex: &'d dyn Dex<'d, Pokemon, &'d Pokemon>,
        movedex: &'d dyn Dex<'d, Move, &'d Move>,
        itemdex: &'d dyn Dex<'d, Item, &'d Item>,
        player: &PD,
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

        let ai = BattleAi::new(SmallRng::seed_from_u64(random.next_u64()), entry.active, p);

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
}

impl GameBattleWrapper<'_> {
    pub fn update_data(&mut self, winner: &BattleId, player: &mut PD) -> bool {
        let trainer = self.trainer.is_some();

        if &BattleId(Some(player.id)) == winner {
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
