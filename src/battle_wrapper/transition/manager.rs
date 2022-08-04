use crate::engine::{
    graphics::{Draw, Graphics},
    music::play_music,
    App, Plugins,
};

use battlecli::battle::data::BattleType;

use super::{
    transitions::{BattleTransitions, FlashBattleTransition, TrainerBattleTransition},
    BattleTransition,
};

pub struct BattleScreenTransitionManager {
    pub current: BattleTransitions,

    flash: FlashBattleTransition,
    trainer: TrainerBattleTransition,
}

impl BattleScreenTransitionManager {
    pub fn new(gfx: &mut Graphics) -> Result<Self, String> {
        Ok(Self {
            current: BattleTransitions::default(),

            flash: FlashBattleTransition::default(),
            trainer: TrainerBattleTransition::new(gfx)?,
        })
    }

    pub fn begin(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        battle_type: BattleType,
    ) {
        self.get_mut().reset();
        self.play_music(app, plugins, battle_type);
    }

    pub fn update(&mut self, delta: f32) -> bool {
        self.get_mut().update(delta)
    }

    pub fn draw(&self, draw: &mut Draw) {
        self.get().draw(draw);
    }

    fn play_music(&mut self, app: &mut App, plugins: &mut Plugins, battle_type: BattleType) {
        match battle_type {
            BattleType::Wild => play_music(app, plugins, &"battle_wild".parse().unwrap()),
            BattleType::Trainer => play_music(app, plugins, &"battle_trainer".parse().unwrap()),
            BattleType::GymLeader => play_music(app, plugins, &"battle_gym".parse().unwrap()),
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
