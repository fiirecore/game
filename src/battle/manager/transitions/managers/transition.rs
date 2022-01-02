use crate::{
    battle::TransitionState,
    engine::{audio::play_music, Context, EngineContext},
    game::battle_glue::BattleTrainerEntry,
};

use battlelib::data::BattleType;

use crate::battle::manager::transitions::{
    transitions::{BattleTransitions, FlashBattleTransition, TrainerBattleTransition},
    BattleTransition,
};

pub struct BattleScreenTransitionManager {
    pub state: TransitionState,
    current: BattleTransitions,

    flash: FlashBattleTransition,
    trainer: TrainerBattleTransition,
}

impl BattleScreenTransitionManager {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            state: TransitionState::default(),
            current: BattleTransitions::default(),

            flash: FlashBattleTransition::default(),
            trainer: TrainerBattleTransition::new(ctx),
        }
    }

    pub fn begin(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        battle_type: BattleType,
        trainer: &Option<BattleTrainerEntry>,
    ) {
        self.play_music(ctx, eng, battle_type);
        match trainer {
            Some(trainer) => self.current = BattleTransitions::from(trainer.transition),
            None => self.current = BattleTransitions::default(),
        }
        self.get_mut().reset();
        self.state = TransitionState::Run;
    }

    pub fn end(&mut self) {
        self.state = TransitionState::Begin;
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32) {
        let current = self.get_mut();
        current.update(ctx, delta);
        if current.finished() {
            self.state = TransitionState::End;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.get().draw(ctx);
    }

    fn play_music(&mut self, ctx: &mut Context, eng: &mut EngineContext, battle_type: BattleType) {
        match battle_type {
            BattleType::Wild => play_music(ctx, eng, &"battle_wild".parse().unwrap()),
            BattleType::Trainer => play_music(ctx, eng, &"battle_trainer".parse().unwrap()),
            BattleType::GymLeader => play_music(ctx, eng, &"battle_gym".parse().unwrap()),
        }
    }

    fn get(&self) -> &dyn BattleTransition {
        match self.current {
            BattleTransitions::Flash => &self.flash,
            BattleTransitions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleTransition {
        match self.current {
            BattleTransitions::Flash => &mut self.flash,
            BattleTransitions::Trainer => &mut self.trainer,
        }
    }
}
