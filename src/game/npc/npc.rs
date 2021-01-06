use crate::engine::game_context::GameContext;
use crate::entity::util::direction::Direction;
use crate::io::data::trainer::Trainer;

pub struct NPC {

    // x and y local to map

    pub x: isize,
    pub y: isize,

    pub direction: Direction,
    pub sprite: u8,

    pub trainer: Option<Trainer>,

}

impl NPC {

    pub fn interact(&mut self, direction: Direction, context: &mut GameContext) {
        self.direction = direction.inverse();
        self.test_trainer(context);
    }

    fn test_trainer(&self, context: &mut GameContext) {
        if let Some(trainer) = &self.trainer {
            context.battle_context.trainer_battle(trainer);
        }
    }

}