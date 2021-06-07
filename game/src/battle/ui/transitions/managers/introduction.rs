use crate::{
    util::Entity,
    gui::TextDisplay, 
    tetra::Context,
};

use crate::battle::{
    BattleData,
    BattleType,
    state::TransitionState,
    pokemon::{
        view::{
            BattlePartyKnown, 
            BattlePartyUnknown,
            gui::{ActivePokemonParty, ActiveRenderer}
        },
    },
    ui::transitions::{
        BattleIntroduction,
        introductions::{
            Introductions,
            basic::BasicBattleIntroduction, 
            trainer::TrainerBattleIntroduction
        },
    }
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

    pub fn begin(&mut self, data: &BattleData, player: &BattlePartyKnown, opponent: &BattlePartyUnknown, text: &mut TextDisplay) {
        self.state = TransitionState::Run;
        match data.battle_type {
            BattleType::Wild => self.current = Introductions::Basic,
            _ => self.current = Introductions::Trainer,
        }
        let current = self.get_mut();
        current.reset();
        current.spawn(data, player, opponent, text);
        text.spawn();
    }

    pub fn end(&mut self, text: &mut TextDisplay) {
        text.clear();
        self.state = TransitionState::Begin;
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut ActivePokemonParty<BattlePartyKnown>, opponent: &mut ActivePokemonParty<BattlePartyUnknown>, text: &mut TextDisplay) {
        let current = self.get_mut();
        current.update(ctx, delta, player, opponent, text);
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