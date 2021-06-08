use crate::{
    util::Entity,
    gui::TextDisplay, 
    tetra::Context,
    battle_glue::BattleTrainerEntry,
};

use crate::battle::{
    BattleType,
    client_state::TransitionState,
    gui::BattlePlayerGui,
    ui::{
        view::ActiveRenderer,
        transitions::{
            BattleIntroduction,
            introductions::{
                Introductions,
                basic::BasicBattleIntroduction, 
                trainer::TrainerBattleIntroduction
            },
        }
    },
};

pub struct BattleIntroductionManager {

    pub state: TransitionState,
    current: Introductions,

    basic: BasicBattleIntroduction,
    trainer: TrainerBattleIntroduction,

}

impl BattleIntroductionManager {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            state: TransitionState::default(),
            current: Introductions::default(),

            basic: BasicBattleIntroduction::new(ctx),
            trainer: TrainerBattleIntroduction::new(ctx),
        }
    }

    pub fn begin(&mut self, battle_type: BattleType, trainer: Option<&BattleTrainerEntry>, player: &mut BattlePlayerGui) {
        self.state = TransitionState::Run;
        match battle_type {
            BattleType::Wild => self.current = Introductions::Basic,
            _ => self.current = Introductions::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(battle_type, trainer, &player.player.party, &player.opponent.party, &mut player.gui.text);
        player.gui.text.spawn();
    }

    pub fn end(&mut self, text: &mut TextDisplay) {
        text.clear();
        self.state = TransitionState::Begin;
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut BattlePlayerGui) {
        let current = self.get_mut();
        current.update(ctx, delta, &mut player.player, &mut player.opponent, &mut player.gui.text);
        if current.finished() {
            self.state = TransitionState::End;
        }
    }

    pub fn draw(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer) {
        self.get().draw(ctx, player, opponent);
    }

    fn get(&self) -> &dyn BattleIntroduction {
        match self.current {
            Introductions::Basic => &self.basic,
            Introductions::Trainer => &self.trainer,
        }
    }

    fn get_mut(&mut self) -> &mut dyn BattleIntroduction {
        match self.current {
            Introductions::Basic => &mut self.basic,
            Introductions::Trainer => &mut self.trainer,
        }
    }

}